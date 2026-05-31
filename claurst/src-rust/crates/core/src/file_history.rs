//! Per-session file modification history.
//! Mirrors src/utils/fileHistory.ts (1,115 lines).
//!
//! Tracks which files were modified by tool calls in the current session,
//! enabling the /rewind command to restore files to earlier states.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// Record of a single file modification in a session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileHistoryEntry {
    /// Absolute path to the file.
    pub path: PathBuf,
    /// SHA-256 hex of file content BEFORE the modification.
    pub before_hash: String,
    /// SHA-256 hex of file content AFTER the modification.
    pub after_hash: String,
    /// UTF-8 text snapshot before the modification when available.
    pub before_text: Option<String>,
    /// UTF-8 text snapshot after the modification when available.
    pub after_text: Option<String>,
    /// Whether either side of the change was non-UTF-8.
    #[serde(default)]
    pub binary: bool,
    /// Conversation turn index at which this modification happened.
    pub turn_index: usize,
    /// Unix timestamp (ms) of the modification.
    pub timestamp_ms: u64,
    /// Tool that made the change ("FileEdit", "FileWrite", etc.).
    pub tool_name: String,
}

// ---------------------------------------------------------------------------
// FileHistory
// ---------------------------------------------------------------------------

/// In-memory file modification tracker for a single session.
#[derive(Debug, Default)]
pub struct FileHistory {
    /// All recorded modifications, in chronological order.
    entries: Vec<FileHistoryEntry>,
    /// Path → all entry indices for that path.
    by_path: HashMap<PathBuf, Vec<usize>>,
}

/// Squashed before/after snapshot for one file within a single turn.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnFileSnapshot {
    pub path: PathBuf,
    pub before_text: Option<String>,
    pub after_text: Option<String>,
    pub binary: bool,
    pub turn_index: usize,
}

impl FileHistory {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record that `tool_name` modified `path` from `before_content` to `after_content`.
    pub fn record_modification(
        &mut self,
        path: PathBuf,
        before_content: &[u8],
        after_content: &[u8],
        turn_index: usize,
        tool_name: &str,
    ) {
        let before_hash = sha256_hex(before_content);
        let after_hash = sha256_hex(after_content);
        let timestamp_ms = current_time_ms();
        let before_text = String::from_utf8(before_content.to_vec()).ok();
        let after_text = String::from_utf8(after_content.to_vec()).ok();
        let binary = before_text.is_none() || after_text.is_none();

        let idx = self.entries.len();
        self.entries.push(FileHistoryEntry {
            path: path.clone(),
            before_hash,
            after_hash,
            before_text,
            after_text,
            binary,
            turn_index,
            timestamp_ms,
            tool_name: tool_name.to_string(),
        });
        self.by_path.entry(path).or_default().push(idx);
    }

    /// Return all recorded modifications for `path`, in chronological order.
    pub fn get_file_history(&self, path: &Path) -> Vec<&FileHistoryEntry> {
        match self.by_path.get(path) {
            Some(indices) => indices.iter().map(|&i| &self.entries[i]).collect(),
            None => Vec::new(),
        }
    }

    /// Return all recorded modifications for `turn_index`, in chronological order.
    pub fn get_entries_for_turn(&self, turn_index: usize) -> Vec<&FileHistoryEntry> {
        self.entries
            .iter()
            .filter(|e| e.turn_index == turn_index)
            .collect()
    }

    pub fn latest_turn_index(&self) -> Option<usize> {
        self.entries.iter().map(|entry| entry.turn_index).max()
    }

    /// Return all files that were modified at or after `turn_index`.
    pub fn get_files_changed_since(&self, turn_index: usize) -> Vec<PathBuf> {
        let mut paths: Vec<PathBuf> = self
            .entries
            .iter()
            .filter(|e| e.turn_index >= turn_index)
            .map(|e| e.path.clone())
            .collect();
        paths.sort();
        paths.dedup();
        paths
    }

    /// Attempt to rewind a file to its state at the beginning of `turn_index`.
    ///
    /// Finds the most recent entry for `path` with `turn_index < rewind_to`.
    /// Returns the content to restore, or `None` if no earlier state is known.
    pub fn state_at_turn(&self, path: &Path, rewind_to: usize) -> Option<String> {
        let indices = self.by_path.get(path)?;
        // Find the earliest modification at or after rewind_to.
        let first_after: Option<&FileHistoryEntry> = indices
            .iter()
            .filter_map(|&i| self.entries.get(i))
            .filter(|e| e.turn_index >= rewind_to)
            .min_by_key(|e| e.turn_index);

        first_after.and_then(|e| e.before_text.clone())
    }

    /// Number of entries recorded.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// All entries (for persistence / serialisation).
    pub fn entries(&self) -> &[FileHistoryEntry] {
        &self.entries
    }

    /// Return squashed snapshots for every file changed in `turn_index`.
    pub fn snapshots_for_turn(&self, turn_index: usize) -> Vec<TurnFileSnapshot> {
        let mut snapshots_by_path: HashMap<PathBuf, TurnFileSnapshot> = HashMap::new();

        for entry in self.entries.iter().filter(|e| e.turn_index == turn_index) {
            snapshots_by_path
                .entry(entry.path.clone())
                .and_modify(|snapshot| {
                    snapshot.after_text = entry.after_text.clone();
                    snapshot.binary |= entry.binary;
                })
                .or_insert_with(|| TurnFileSnapshot {
                    path: entry.path.clone(),
                    before_text: entry.before_text.clone(),
                    after_text: entry.after_text.clone(),
                    binary: entry.binary,
                    turn_index,
                });
        }

        let mut snapshots: Vec<TurnFileSnapshot> = snapshots_by_path.into_values().collect();
        snapshots.sort_by(|a, b| a.path.cmp(&b.path));
        snapshots
    }

    pub fn from_entries(entries: Vec<FileHistoryEntry>) -> Self {
        let mut by_path: HashMap<PathBuf, Vec<usize>> = HashMap::new();
        for (idx, entry) in entries.iter().enumerate() {
            by_path.entry(entry.path.clone()).or_default().push(idx);
        }
        Self { entries, by_path }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn sha256_hex(data: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    h.update(data);
    hex::encode(h.finalize())
}

fn current_time_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_and_retrieve() {
        let mut fh = FileHistory::new();
        let path = PathBuf::from("/foo/bar.rs");
        fh.record_modification(path.clone(), b"old", b"new", 1, "FileEdit");
        assert_eq!(fh.len(), 1);
        let history = fh.get_file_history(&path);
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].tool_name, "FileEdit");
        assert_eq!(history[0].turn_index, 1);
    }

    #[test]
    fn files_changed_since() {
        let mut fh = FileHistory::new();
        let a = PathBuf::from("/a.rs");
        let b = PathBuf::from("/b.rs");
        fh.record_modification(a.clone(), b"", b"x", 0, "FileWrite");
        fh.record_modification(b.clone(), b"", b"y", 3, "FileEdit");
        let changed = fh.get_files_changed_since(2);
        assert_eq!(changed, vec![b.clone()]);
    }

    #[test]
    fn state_at_turn_none_if_no_history() {
        let fh = FileHistory::new();
        assert!(fh.state_at_turn(Path::new("/x.rs"), 0).is_none());
    }

    #[test]
    fn state_at_turn_returns_text_snapshot() {
        let mut fh = FileHistory::new();
        let path = PathBuf::from("/x.rs");
        fh.record_modification(path.clone(), b"before", b"after", 2, "FileEdit");
        assert_eq!(fh.state_at_turn(&path, 2).as_deref(), Some("before"));
    }

    #[test]
    fn entries_for_turn_filters_exact_turn() {
        let mut fh = FileHistory::new();
        fh.record_modification(PathBuf::from("/a.rs"), b"a", b"b", 1, "FileEdit");
        fh.record_modification(PathBuf::from("/b.rs"), b"c", b"d", 2, "FileWrite");
        let entries = fh.get_entries_for_turn(2);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].path, PathBuf::from("/b.rs"));
    }

    #[test]
    fn snapshots_for_turn_squash_multiple_edits() {
        let mut fh = FileHistory::new();
        let path = PathBuf::from("/repeat.rs");
        fh.record_modification(path.clone(), b"one", b"two", 4, "FileEdit");
        fh.record_modification(path.clone(), b"two", b"three", 4, "FileEdit");

        let snapshots = fh.snapshots_for_turn(4);
        assert_eq!(snapshots.len(), 1);
        assert_eq!(snapshots[0].before_text.as_deref(), Some("one"));
        assert_eq!(snapshots[0].after_text.as_deref(), Some("three"));
    }

    #[test]
    fn latest_turn_index_returns_most_recent_change() {
        let mut fh = FileHistory::new();
        fh.record_modification(PathBuf::from("/a.rs"), b"a", b"b", 2, "FileEdit");
        fh.record_modification(PathBuf::from("/b.rs"), b"b", b"c", 7, "FileWrite");
        assert_eq!(fh.latest_turn_index(), Some(7));
    }
}
