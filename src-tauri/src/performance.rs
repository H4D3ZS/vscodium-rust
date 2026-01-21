use sysinfo::{Pid, System};
use std::sync::Mutex;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct ProcessStats {
    pub memory_mb: u64,
    pub cpu_usage: f32,
}

pub struct PerformanceMonitor {
    sys: Mutex<System>,
    pid: Pid,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        let pid = Pid::from(std::process::id() as usize);
        Self {
            sys: Mutex::new(sys),
            pid,
        }
    }

    pub fn get_stats(&self) -> Option<ProcessStats> {
        let mut sys = self.sys.lock().unwrap();
        // Refresh process specific info. In sysinfo 0.30+, refresh_process operates on System.
        sys.refresh_process(self.pid);
        
        if let Some(process) = sys.process(self.pid) {
            let memory_mb = process.memory() / 1024 / 1024; // Bytes -> MB
            let cpu_usage = process.cpu_usage();
            Some(ProcessStats {
                memory_mb,
                cpu_usage,
            })
        } else {
            None
        }
    }
}
