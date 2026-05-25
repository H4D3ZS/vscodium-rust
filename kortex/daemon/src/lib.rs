pub mod gc;
pub mod gist;
pub mod vfs_state;
pub mod crypto;
pub mod symlink;
pub mod watcher;
pub mod chain;
pub mod watermark;
pub mod neural_math;
pub mod visual_encoder;

use std::path::PathBuf;
use crate::gc::MemoryGarbageCollector;

pub struct CognitiveKernel {
pub runtime_state: vfs_state::VfsState,
pub security_layer: crypto::SecurityLayer,
pub shadow_watcher: watcher::ShadowWatcher,
pub chain_vault: chain::QuantumChain,
pub safety_watchdog: watermark::SoftBindingWatchdog,
}

impl CognitiveKernel {
pub fn new() -> Self {
Self {
runtime_state: vfs_state::VfsState::new(),
security_layer: crypto::SecurityLayer::new(),
shadow_watcher: watcher::ShadowWatcher::new(),
chain_vault: chain::QuantumChain::new(),
safety_watchdog: watermark::SoftBindingWatchdog::new(),
}
}

pub async fn spawn_background_infrastructure(&self) {
let gc = self.runtime_state.gc.clone();
let _gc_handle = tokio::spawn(async move {
MemoryGarbageCollector::spawn_consolidation_loop(gc).await;
});
}

pub async fn mount(&mut self, path: PathBuf) -> Result<(), Box<dyn Send + Sync>> {
self.runtime_state.mount(path).await
}
}

impl Default for CognitiveKernel {
fn default() -> Self {
Self::new()
}
}
