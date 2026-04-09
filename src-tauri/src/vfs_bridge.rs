use std::path::{Path, PathBuf};
use std::fs;
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
        
        // Security check: ensure path is within project root
        if !full_path.canonicalize()?.starts_with(&self.project_root.canonicalize()?) {
            return Err(anyhow::anyhow!("Access denied: Path outside project root"));
        }

        let content = fs::read_to_string(full_path)?;
        Ok(content)
    }

    /// Verifies if the cached neural content matches the current disk state.
    pub fn verify_integrity(&self, relative_path: &Path, cached_content: &str) -> bool {
        match self.fetch_raw(relative_path) {
            Ok(disk_content) => disk_content == cached_content,
            Err(_) => false,
        }
    }
}
