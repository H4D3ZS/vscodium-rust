use crate::gc::MemoryGarbageCollector;
use crate::gist::GistInjector;

pub struct VfsState {
    pub injector: GistInjector,
    pub gc: MemoryGarbageCollector,
}

impl VfsState {
    pub fn new() -> Self {
        Self {
            injector: GistInjector::new(),
            gc: MemoryGarbageCollector::new(),
        }
    }
}

impl Default for VfsState {
    fn default() -> Self {
        Self::new()
    }
}
