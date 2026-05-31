use std::path::{Path, PathBuf};
use std::fs;
use std::io::Write;
use anyhow::Result;

#[derive(Clone)]
pub struct VfsBridge {
    project_root: PathBuf,
}

impl VfsBridge {
    pub fn new(project_root: PathBuf) -> Self {
        Self { project_root }
    }

    /// Fetches the raw content of a file from disk (L2) to resolve a Page-Fault.
    pub fn fetch_raw(&self, relative_path: &Path) -> Result<String> {
        let full_path = self.project_root.join(relative_path);
        self.assert_within_root(&full_path)?;
        Ok(fs::read_to_string(full_path)?)
    }

    /// Atomic copy-on-write write: write to a .tmp sibling then rename.
    /// Crash-safe: either the old content survives or the new content is visible.
    /// Never leaves a half-written file.
    pub fn write_atomic(&self, relative_path: &Path, content: &str) -> Result<()> {
        let full_path = self.project_root.join(relative_path);
        self.assert_within_root(&full_path)?;

        // Ensure parent directory exists.
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Write to a sibling temp file so the final rename is atomic.
        let tmp_path = full_path.with_extension("tmp_aim_write");
        {
            let mut f = fs::File::create(&tmp_path)?;
            f.write_all(content.as_bytes())?;
            f.flush()?;
            f.sync_all()?; // fsync before rename to guarantee durability
        }
        fs::rename(&tmp_path, &full_path)?;
        Ok(())
    }

    /// Copy-on-write patch: read → apply diffy search/replace → atomic write.
    /// Returns the new content on success.
    pub fn apply_patch(&self, relative_path: &Path, search: &str, replace: &str) -> Result<String> {
        let original = self.fetch_raw(relative_path)?;
        if !original.contains(search) {
            return Err(anyhow::anyhow!(
                "Search string not found in {}",
                relative_path.display()
            ));
        }
        // Only replace the first occurrence for surgical precision.
        let patched = original.replacen(search, replace, 1);
        self.write_atomic(relative_path, &patched)?;
        Ok(patched)
    }

    /// Verifies if the cached neural content matches the current disk state.
    pub fn verify_integrity(&self, relative_path: &Path, cached_content: &str) -> bool {
        match self.fetch_raw(relative_path) {
            Ok(disk_content) => disk_content == cached_content,
            Err(_) => false,
        }
    }

    /// List all files under the project root with given extensions (for dep graph).
    pub fn list_files(&self, extensions: &[&str]) -> Vec<PathBuf> {
        let mut out = Vec::new();
        self.walk_dir(&self.project_root, extensions, &mut out);
        out
    }

    fn walk_dir(&self, dir: &Path, exts: &[&str], out: &mut Vec<PathBuf>) {
        let Ok(entries) = fs::read_dir(dir) else { return };
        for entry in entries.flatten() {
            let path = entry.path();
            // Skip hidden dirs and common noise dirs
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name.starts_with('.') || matches!(name, "node_modules" | "target" | "dist" | ".git") {
                continue;
            }
            if path.is_dir() {
                self.walk_dir(&path, exts, out);
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if exts.contains(&ext) {
                    out.push(path);
                }
            }
        }
    }

    fn assert_within_root(&self, full_path: &Path) -> Result<()> {
        // Use starts_with on the raw path first (fast path) and canonicalize
        // only when the fast check fails (symlink traversal defense).
        if full_path.starts_with(&self.project_root) {
            return Ok(());
        }
        let canonical = full_path
            .canonicalize()
            .unwrap_or_else(|_| full_path.to_path_buf());
        let root = self.project_root
            .canonicalize()
            .unwrap_or_else(|_| self.project_root.clone());
        if canonical.starts_with(&root) {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Access denied: path escapes project root"))
        }
    }
}
