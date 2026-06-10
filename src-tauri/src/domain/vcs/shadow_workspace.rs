use anyhow::{anyhow, Result};
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowWorkspace {
    root_path: PathBuf,
    cache_path: PathBuf,
}

impl ShadowWorkspace {
    pub fn new(root_path: PathBuf) -> Self {
        let cache_path = root_path.join(".hades_cache");
        if !cache_path.exists() {
            let _ = fs::create_dir_all(&cache_path);
        }
        
        Self {
            root_path,
            cache_path,
        }
    }

    /// Mirrors a file from the main VFS to the shadow workspace
    pub fn mirror_file(&self, relative_path: &Path) -> Result<PathBuf> {
        let source = self.root_path.join(relative_path);
        let target = self.cache_path.join(relative_path);
        
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        
        fs::copy(&source, &target)?;
        Ok(target)
    }

    /// Creates a checkpoint (backup) of a shadow file
    pub fn create_checkpoint(&self, relative_path: &Path) -> Result<PathBuf> {
        let shadow_file = self.cache_path.join(relative_path);
        if !shadow_file.exists() {
            return Err(anyhow!("Shadow file does not exist: {:?}", relative_path));
        }
        
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let checkpoint_name = format!("{}.{}.bak", relative_path.file_name().unwrap().to_str().unwrap(), timestamp);
        let checkpoint_path = self.cache_path.join("checkpoints").join(checkpoint_name);
        
        if let Some(parent) = checkpoint_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        fs::copy(&shadow_file, &checkpoint_path)?;
        Ok(checkpoint_path)
    }

    /// Reverts a shadow file to the last known checkpoint
    pub fn revert_to_last_checkpoint(&self, relative_path: &Path) -> Result<()> {
        let checkpoint_dir = self.cache_path.join("checkpoints");
        if !checkpoint_dir.exists() {
            return Err(anyhow!("No checkpoints found."));
        }
        
        let prefix = relative_path.file_name().unwrap().to_str().unwrap();
        let mut entries: Vec<_> = fs::read_dir(checkpoint_dir)?
            .filter_map(|res| res.ok())
            .filter(|e| e.file_name().to_str().map(|s| s.starts_with(prefix)).unwrap_or(false))
            .collect();
            
        entries.sort_by_key(|e| e.metadata().unwrap().modified().unwrap());
        
        if let Some(last_entry) = entries.last() {
            let shadow_target = self.cache_path.join(relative_path);
            fs::copy(last_entry.path(), shadow_target)?;
            Ok(())
        } else {
            Err(anyhow!("No checkpoint found for {:?}", relative_path))
        }
    }

    /// Commits the shadow file to the actual filesystem
    pub fn commit_shadow(&self, relative_path: &Path) -> Result<()> {
        let shadow_source = self.cache_path.join(relative_path);
        let final_target = self.root_path.join(relative_path);
        
        if !shadow_source.exists() {
            return Err(anyhow!("No shadow file found to commit."));
        }
        
        fs::copy(shadow_source, final_target)?;
        Ok(())
    }
}
