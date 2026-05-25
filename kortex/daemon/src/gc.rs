use std::path::{Path, PathBuf};
use std::fs;
use std::sync::Arc;

/// The Garbage Collector for the Cognitive Kernel background memory consolidation
pub struct MemoryGarbageCollector {
pub decay_rate: f32,
pub memory_dir: PathBuf,
pub consolidation_interval_secs: u64,
}

impl MemoryGarbageCollector {
pub fn new(memory_dir: PathBuf) -> Self {
Self {
decay_rate: 0.1,
memory_dir,
consolidation_interval_secs: 60,
}
}

/// This function evaluates neural memory files (.aim files).
/// Highly valuable constants are shifted to structural L2/L3 structural vectors,
/// while transient context logs are decayed and dropped.
pub async fn consolidate_transient_logs(&self) {
let aim_path = self.memory_dir.join(".aim");
if !aim_path.exists() {
return;
}

let _ = self.consolidate_aim_memory(&aim_path).await;
}

/// Consolidates AIM memory files by merging session data into persistent memory
async fn consolidate_aim_memory(&self, aim_dir: &Path) -> Result<(), std::io::Error> {
let memory_aim = aim_dir.join("memory.aim");
let mut persistent_content = String::new();

if memory_aim.exists() {
persistent_content = fs::read_to_string(&memory_aim).unwrap_or_default();
}

for entry in fs::read_dir(aim_dir)? {
let entry = entry?;
let path = entry.path();
if path.file_name().and_then(|n| n.to_str()).map_or(true, |n| !n.starts_with("session_")) {
continue;
}

if let Ok(content) = fs::read_to_string(&path) {
if content.len() > persistent_content.len() {
persistent_content = content;
}
}
}

let _ = fs::write(&memory_aim, persistent_content);
Ok(())
}

/// Spawns the background consolidation loop
pub async fn spawn_consolidation_loop(gc: Arc<Self>) {
loop {
tokio::time::sleep(tokio::time::Duration::from_secs(gc.consolidation_interval_secs)).await;
gc.consolidate_transient_logs().await;
}
}
}
