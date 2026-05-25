use crate::gc::MemoryGarbageCollector;
use crate::gist::GistInjector;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub struct VfsState {
pub injector: GistInjector,
pub gc: Arc<MemoryGarbageCollector>,
pub mounted_path: Option<PathBuf>,
}

impl VfsState {
pub fn new() -> Self {
let memory_dir = PathBuf::from(".");
Self {
injector: GistInjector::new(),
gc: Arc::new(MemoryGarbageCollector::new(memory_dir.clone())),
mounted_path: None,
}
}

pub fn with_memory_dir(memory_dir: PathBuf) -> Self {
Self {
injector: GistInjector::new(),
gc: Arc::new(MemoryGarbageCollector::new(memory_dir.clone())),
mounted_path: None,
}
}

/// Mount the VFS at the given path and initialize memory structures
pub async fn mount(&mut self, path: PathBuf) -> Result<(), Box<dyn Send + Sync>> {
self.mounted_path = Some(path.clone());
let aim_dir = path.join(".aim");

if !aim_dir.exists() {
let _ = std::fs::create_dir_all(&aim_dir);
}

let memory_aim = aim_dir.join("memory.aim");
if !memory_aim.exists() {
let _ = std::fs::write(&memory_aim, r#"{"kortex":{"entities":{},"session_messages":[]}}"#.as_bytes());
}

let gc = self.gc.clone();
tokio::spawn(async move {
MemoryGarbageCollector::spawn_consolidation_loop(gc).await;
});

Ok(())
}

/// Verify file integrity against the mounted VFS
pub fn verify_integrity(&self, relative_path: &Path, cached_content: &str) -> bool {
if let Some(ref mounted) = self.mounted_path {
let full_path = mounted.join(relative_path);
if let Ok(disk_content) = std::fs::read_to_string(full_path) {
return disk_content == cached_content;
}
}
false
}
}

impl Default for VfsState {
fn default() -> Self {
Self::new()
}
}
