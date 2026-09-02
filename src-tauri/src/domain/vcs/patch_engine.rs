use anyhow::{anyhow, Result};
use serde_json::json;
use ropey::Rope;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchBlock {
    pub search: String,
    pub replace: String,
}

pub struct PatchEngine {
    // Keeps track of uncommitted changes for "shadow document" editing
    shadow_buffers: HashMap<PathBuf, Rope>,
    pub shadow_workspace: Arc<crate::shadow_workspace::ShadowWorkspace>,
    editor_state: std::sync::RwLock<std::sync::Weak<crate::EditorState>>,
}

impl PatchEngine {
    pub fn new(shadow_workspace: Arc<crate::shadow_workspace::ShadowWorkspace>) -> Self {
        Self {
            shadow_buffers: HashMap::new(),
            shadow_workspace,
            editor_state: std::sync::RwLock::new(std::sync::Weak::new()),
        }
    }

    pub fn set_editor_state(&self, weak: std::sync::Weak<crate::EditorState>) {
        if let Ok(mut g) = self.editor_state.write() {
            *g = weak;
        }
    }

    /// Parse content for SEARCH/REPLACE blocks.
    /// Supports the "Hades-Kortex" miracle format:
    /// <<<< SEARCH
    /// [existing code]
    /// ====
    /// [new code]
    /// >>>>
    pub fn parse_search_replace(content: &str) -> Vec<PatchBlock> {
        let mut patches = Vec::new();
        let mut lines = content.lines().peekable();

        while let Some(line) = lines.next() {
            let trimmed = line.trim();
            if trimmed == "<<<<<<< ORIGINAL"
                || trimmed == "<<<<<<< SEARCH"
                || trimmed == "<<<< SEARCH"
            {
                let mut search_lines = Vec::new();
                let mut replace_lines = Vec::new();
                let mut in_replace = false;

                while let Some(inner_line) = lines.next() {
                    let inner_trimmed = inner_line.trim();
                    if inner_trimmed == "=======" || inner_trimmed == "====" {
                        in_replace = true;
                        continue;
                    }
                    if inner_trimmed == ">>>>>>> REPLACE"
                        || inner_trimmed == ">>>>>>> UPDATED"
                        || inner_trimmed == ">>>>"
                    {
                        break;
                    }

                    if in_replace {
                        replace_lines.push(inner_line);
                    } else {
                        search_lines.push(inner_line);
                    }
                }

                if !search_lines.is_empty() || !replace_lines.is_empty() {
                    patches.push(PatchBlock {
                        search: search_lines.join("\n"),
                        replace: replace_lines.join("\n"),
                    });
                }
            }
        }
        patches
    }

    /// Apply a set of patches to a file's content.
    /// Uses a "Shadow Document" approach: first application is to a buffer.
    pub async fn apply_patches(&mut self, path: &Path, content: &str, patches: &[PatchBlock]) -> Result<String> {
        let mut rope = Rope::from_str(content);
        
        for patch in patches {
            let search_str = patch.search.trim();
            let replace_str = patch.replace.trim();

            if search_str.is_empty() {
                continue;
            }

            let full_text = rope.to_string();
            
            // 1. Exact match check (The "Zero-Loss" rule)
            if let Some(pos) = full_text.find(search_str) {
                let start_char = rope.byte_to_char(pos);
                let end_char = rope.byte_to_char(pos + search_str.len());
                
                rope.remove(start_char..end_char);
                rope.insert(start_char, replace_str);
            } else {
                // 2. Fuzzy match fallback (The "Claude Code" logic)
                // If exact match fails, normalize whitespace to check if the intent exists
                let normalized_content = full_text.split_whitespace().collect::<Vec<_>>().join(" ");
                let normalized_search = search_str.split_whitespace().collect::<Vec<_>>().join(" ");

                if normalized_content.contains(&normalized_search) {
                    return Err(anyhow!("Exact match failed for {:?}, but the block exists with different whitespace. Refusing to patch to avoid data loss. Please re-read the file.", path));
                }
                
                return Err(anyhow!("SEARCH block not found in file: {}. Check context and try again.", path.display()));
            }
        }

        let new_content = rope.to_string();
        // Cap shadow buffers at 50 entries to prevent unbounded growth
        const MAX_SHADOW_BUFFERS: usize = 50;
        if self.shadow_buffers.len() >= MAX_SHADOW_BUFFERS {
            // Remove the oldest entry (first key)
            if let Some(oldest) = self.shadow_buffers.keys().next().cloned() {
                self.shadow_buffers.remove(&oldest);
            }
        }
        self.shadow_buffers.insert(path.to_path_buf(), rope);
        
        // Mirror to physical shadow workspace
        let shadow_path = self.shadow_workspace.mirror_file(path)?;
        std::fs::write(&shadow_path, &new_content)?;
        
        // Emit update event for real-time frontend streaming
        if let Some(es) = self.editor_state.read().ok().and_then(|w| w.upgrade()) {
            es.emit("shadow-file-updated", json!({
                "path": path,
                "content": new_content,
                "diff": self.get_diff(path, content)?
            }));
        }
        
        Ok(new_content)
    }

    pub fn get_diff(&self, path: &Path, original_content: &str) -> Result<String> {
        let shadow = self.shadow_buffers.get(path)
            .ok_or_else(|| anyhow!("No shadow buffer for {:?}", path))?;
            
        let new_content = shadow.to_string();
        let patch = diffy::create_patch(original_content, &new_content);
        Ok(format!("{}", patch))
    }

    pub fn commit_shadow(&mut self, path: &Path) -> Result<()> {
        if let Some(rope) = self.shadow_buffers.remove(path) {
            std::fs::write(path, rope.to_string())?;
            Ok(())
        } else {
            Err(anyhow!("No uncommitted changes for {:?}", path))
        }
    }

    pub fn discard_shadow(&mut self, path: &Path) {
        self.shadow_buffers.remove(path);
    }

    pub fn set_shadow_buffer(&mut self, path: &Path, content: &str) -> Result<()> {
        let rope = Rope::from_str(content);
        self.shadow_buffers.insert(path.to_path_buf(), rope);
        
        let shadow_path = self.shadow_workspace.mirror_file(path)?;
        std::fs::write(&shadow_path, content)?;
        
        Ok(())
    }
}
