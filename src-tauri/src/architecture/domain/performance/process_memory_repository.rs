use super::ProcessMemorySnapshot;

/// Port (DDD): how the domain reads live process memory.
///
/// Infrastructure provides `SysinfoProcessMemoryRepository`.
/// Application code depends on this trait, not on sysinfo/Win32.
pub trait ProcessMemoryRepository: Send + Sync {
    fn capture_snapshot(&self, root_pid: u32) -> Option<ProcessMemorySnapshot>;
}
