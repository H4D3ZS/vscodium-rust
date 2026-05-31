//! Snapshot/undo system — tracks file changes per tool call within a session.
//!
//! Each time a tool is about to write a file, it should call `snapshot_before`
//! with the `tool_use_id` provided by the API.  The original content (or `None`
//! if the file did not yet exist) is stored in memory.
//!
//! The `/undo <tool_use_id>` command then calls `revert` to restore those files
//! to their pre-tool state.
//!
//! This is entirely in-process — no git required.

use std::collections::HashMap;

// ---------------------------------------------------------------------------
// SnapshotManager
// ---------------------------------------------------------------------------

/// Tracks file-content snapshots keyed by the `tool_use_id` that caused each
/// write, enabling per-tool-call undo within a session.
///
/// `None` as the stored content means the file did not exist before the tool
/// call (so reverting deletes it).
pub struct SnapshotManager {
    /// tool_use_id -> Vec<(absolute_file_path, content_before_write)>
    ///
    /// A single tool call can write multiple files (e.g. `BatchEdit`), so the
    /// value is a `Vec`.  Files are stored in the order `snapshot_before` was
    /// called so that reverting happens in reverse order (last-written first).
    snapshots: HashMap<String, Vec<(String, Option<String>)>>,
}

impl SnapshotManager {
    /// Create a new, empty `SnapshotManager`.
    pub fn new() -> Self {
        Self {
            snapshots: HashMap::new(),
        }
    }

    /// Read and store the current content of `path` before a write.
    ///
    /// Call this once for every file that a tool is about to overwrite, *before*
    /// the write happens.  Multiple calls with the same `tool_use_id` accumulate
    /// entries for that tool call.
    ///
    /// If the file does not exist the snapshot stores `None` (reverting will
    /// delete the file).  If the file exists but cannot be read the error is
    /// returned immediately and nothing is stored.
    pub fn snapshot_before(&mut self, tool_use_id: &str, path: &str) -> std::io::Result<()> {
        let content = if std::path::Path::new(path).exists() {
            Some(std::fs::read_to_string(path)?)
        } else {
            None
        };
        self.snapshots
            .entry(tool_use_id.to_string())
            .or_default()
            .push((path.to_string(), content));
        Ok(())
    }

    /// Revert all file changes that were made by `tool_use_id`.
    ///
    /// Files are reverted in *reverse* snapshot order (the last file written is
    /// restored first) to keep things consistent when one tool writes the same
    /// file twice.
    ///
    /// Returns `(files_reverted, errors)`.  Errors are human-readable strings;
    /// the caller decides whether to surface them.
    pub fn revert(&self, tool_use_id: &str) -> (Vec<String>, Vec<String>) {
        let mut reverted = Vec::new();
        let mut errors = Vec::new();

        let entries = match self.snapshots.get(tool_use_id) {
            Some(e) => e,
            None => return (reverted, errors),
        };

        // Revert in reverse order.
        for (path, original_content) in entries.iter().rev() {
            match original_content {
                None => {
                    // File did not exist before — delete it if it does now.
                    if std::path::Path::new(path).exists() {
                        if let Err(e) = std::fs::remove_file(path) {
                            errors.push(format!("Failed to delete {}: {}", path, e));
                        } else {
                            reverted.push(path.clone());
                        }
                    } else {
                        // Already gone — count as reverted.
                        reverted.push(path.clone());
                    }
                }
                Some(content) => {
                    if let Err(e) = std::fs::write(path, content) {
                        errors.push(format!("Failed to restore {}: {}", path, e));
                    } else {
                        reverted.push(path.clone());
                    }
                }
            }
        }

        (reverted, errors)
    }

    /// Revert ALL file changes recorded in this session, across all tool calls.
    ///
    /// Returns `(files_reverted, errors)`.
    pub fn revert_all(&self) -> (Vec<String>, Vec<String>) {
        let mut all_reverted = Vec::new();
        let mut all_errors = Vec::new();

        // Collect tool_use_ids first so we can borrow `self.snapshots` cleanly.
        let ids: Vec<String> = self.snapshots.keys().cloned().collect();
        for id in ids {
            let (mut r, mut e) = self.revert(&id);
            all_reverted.append(&mut r);
            all_errors.append(&mut e);
        }

        (all_reverted, all_errors)
    }

    /// List all tool_use_ids that made changes, paired with the file paths they
    /// modified.  Ordered by insertion order where possible.
    pub fn list_changes(&self) -> Vec<(String, Vec<String>)> {
        self.snapshots
            .iter()
            .map(|(id, entries)| {
                let paths = entries.iter().map(|(p, _)| p.clone()).collect();
                (id.clone(), paths)
            })
            .collect()
    }
}

impl Default for SnapshotManager {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_snapshot_existing_file_and_revert() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.txt");
        fs::write(&path, "original content").unwrap();

        let path_str = path.to_str().unwrap();
        let mut mgr = SnapshotManager::new();

        mgr.snapshot_before("tool-1", path_str).unwrap();

        // Overwrite the file.
        fs::write(&path, "modified content").unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), "modified content");

        // Revert.
        let (reverted, errors) = mgr.revert("tool-1");
        assert!(errors.is_empty(), "unexpected errors: {:?}", errors);
        assert_eq!(reverted, vec![path_str.to_string()]);
        assert_eq!(fs::read_to_string(&path).unwrap(), "original content");
    }

    #[test]
    fn test_snapshot_new_file_and_revert() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("new_file.txt");
        let path_str = path.to_str().unwrap();

        let mut mgr = SnapshotManager::new();
        // File does not exist yet.
        mgr.snapshot_before("tool-2", path_str).unwrap();

        // Create the file.
        fs::write(&path, "brand new").unwrap();
        assert!(path.exists());

        // Revert should delete it.
        let (reverted, errors) = mgr.revert("tool-2");
        assert!(errors.is_empty(), "unexpected errors: {:?}", errors);
        assert_eq!(reverted, vec![path_str.to_string()]);
        assert!(!path.exists(), "file should have been deleted");
    }

    #[test]
    fn test_revert_unknown_id_is_noop() {
        let mgr = SnapshotManager::new();
        let (reverted, errors) = mgr.revert("nonexistent-id");
        assert!(reverted.is_empty());
        assert!(errors.is_empty());
    }

    #[test]
    fn test_list_changes() {
        let dir = tempdir().unwrap();
        let p1 = dir.path().join("a.txt");
        let p2 = dir.path().join("b.txt");
        fs::write(&p1, "a").unwrap();
        fs::write(&p2, "b").unwrap();

        let mut mgr = SnapshotManager::new();
        mgr.snapshot_before("tool-a", p1.to_str().unwrap()).unwrap();
        mgr.snapshot_before("tool-b", p2.to_str().unwrap()).unwrap();

        let changes = mgr.list_changes();
        assert_eq!(changes.len(), 2);
        // Both tool ids are present.
        let ids: Vec<&str> = changes.iter().map(|(id, _)| id.as_str()).collect();
        assert!(ids.contains(&"tool-a"));
        assert!(ids.contains(&"tool-b"));
    }

    #[test]
    fn test_revert_all() {
        let dir = tempdir().unwrap();
        let p1 = dir.path().join("x.txt");
        let p2 = dir.path().join("y.txt");
        fs::write(&p1, "x original").unwrap();
        fs::write(&p2, "y original").unwrap();

        let mut mgr = SnapshotManager::new();
        mgr.snapshot_before("tx", p1.to_str().unwrap()).unwrap();
        mgr.snapshot_before("ty", p2.to_str().unwrap()).unwrap();

        fs::write(&p1, "x modified").unwrap();
        fs::write(&p2, "y modified").unwrap();

        let (reverted, errors) = mgr.revert_all();
        assert!(errors.is_empty());
        assert_eq!(reverted.len(), 2);
        assert_eq!(fs::read_to_string(&p1).unwrap(), "x original");
        assert_eq!(fs::read_to_string(&p2).unwrap(), "y original");
    }
}
