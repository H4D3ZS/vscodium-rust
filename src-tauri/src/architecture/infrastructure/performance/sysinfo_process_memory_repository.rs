use crate::architecture::domain::performance::{
    ProcessMemoryLineItem, ProcessMemoryRepository, ProcessMemorySnapshot,
};
use sysinfo::{Pid, System};

/// Reads process memory via `sysinfo` and sums the full descendant tree.
///
/// **Why a dedicated adapter?** Keeps Win32/sysinfo quirks out of domain and
/// Tauri commands so you can unit-test tree-walk logic or swap to WMI later.
pub struct SysinfoProcessMemoryRepository {
    sys: std::sync::Mutex<System>,
}

impl SysinfoProcessMemoryRepository {
    pub fn new() -> Self {
        Self {
            sys: std::sync::Mutex::new(System::new()),
        }
    }

    /// Collect root PID + every descendant (BFS on parent links).
    fn collect_process_tree(sys: &System, root: Pid) -> Vec<Pid> {
        let mut owned = vec![root];
        let mut frontier = vec![root];
        while let Some(parent) = frontier.pop() {
            for (pid, process) in sys.processes() {
                if process.parent() == Some(parent) && !owned.contains(pid) {
                    owned.push(*pid);
                    frontier.push(*pid);
                }
            }
        }
        owned
    }

    #[cfg(windows)]
    fn private_bytes_mb(pid: u32) -> f64 {
        use std::mem::MaybeUninit;
        use windows::Win32::Foundation::CloseHandle;
        use windows::Win32::System::ProcessStatus::{
            GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS, PROCESS_MEMORY_COUNTERS_EX,
        };
        use windows::Win32::System::Threading::{
            OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ,
        };

        unsafe {
            let Ok(handle) = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid)
            else {
                return 0.0;
            };
            let mut counters = MaybeUninit::<PROCESS_MEMORY_COUNTERS_EX>::uninit();
            let base = counters.as_mut_ptr() as *mut PROCESS_MEMORY_COUNTERS;
            let ok = GetProcessMemoryInfo(
                handle,
                base,
                std::mem::size_of::<PROCESS_MEMORY_COUNTERS_EX>() as u32,
            );
            let _ = CloseHandle(handle);
            if ok.is_err() {
                return 0.0;
            }
            let c = counters.assume_init();
            (c.PrivateUsage as f64) / (1024.0 * 1024.0)
        }
    }

    #[cfg(not(windows))]
    fn private_bytes_mb(_pid: u32) -> f64 {
        0.0
    }
}

impl ProcessMemoryRepository for SysinfoProcessMemoryRepository {
    fn capture_snapshot(&self, root_pid: u32) -> Option<ProcessMemorySnapshot> {
        let mut sys = self.sys.lock().ok()?;
        sys.refresh_processes();
        sys.refresh_memory();

        let root = Pid::from(root_pid as usize);
        let tree = Self::collect_process_tree(&sys, root);

        let mut breakdown = Vec::new();
        let mut host_ws = 0.0;
        let mut child_ws = 0.0;
        let mut total_private = 0.0;
        let mut cpu = 0.0f32;

        for pid in &tree {
            let Some(process) = sys.process(*pid) else {
                continue;
            };
            let ws_mb = process.memory() as f64 / (1024.0 * 1024.0);
            let pid_u32 = pid.as_u32();
            let private_mb = Self::private_bytes_mb(pid_u32);
            let private_mb = if private_mb > 0.0 { private_mb } else { ws_mb };

            if *pid == root {
                host_ws = ws_mb;
                cpu = process.cpu_usage();
            } else {
                child_ws += ws_mb;
            }
            total_private += private_mb;

            breakdown.push(ProcessMemoryLineItem {
                name: process.name().to_string(),
                pid: pid_u32,
                working_set_mb: (ws_mb * 10.0).round() / 10.0,
                private_mb: (private_mb * 10.0).round() / 10.0,
            });
        }

        breakdown.sort_by(|a, b| {
            b.working_set_mb
                .partial_cmp(&a.working_set_mb)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let child_count = tree.len().saturating_sub(1) as u32;
        let total_ws = host_ws + child_ws;
        let total_private = if total_private > 0.0 {
            total_private
        } else {
            total_ws
        };

        Some(ProcessMemorySnapshot {
            host_working_set_mb: (host_ws * 10.0).round() / 10.0,
            child_working_set_mb: (child_ws * 10.0).round() / 10.0,
            total_working_set_mb: (total_ws * 10.0).round() / 10.0,
            total_private_mb: (total_private * 10.0).round() / 10.0,
            child_process_count: child_count,
            cpu_usage_percent: cpu,
            system_total_ram_gb: sys.total_memory() / 1024 / 1024 / 1024,
            system_available_ram_gb: sys.available_memory() / 1024 / 1024 / 1024,
            breakdown,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_includes_host() {
        let repo = SysinfoProcessMemoryRepository::new();
        let snap = repo
            .capture_snapshot(std::process::id())
            .expect("snapshot");
        assert!(snap.host_working_set_mb > 0.0);
        assert!(snap.total_working_set_mb >= snap.host_working_set_mb);
    }
}
