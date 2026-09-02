//! Global registry of active foreground `run_command` child processes.
//!
//! Foreground shell commands (scans, builds) are spawned by the agent tool
//! layer (`AiTools::run_command`) and waited on with a blocking `child.wait()`.
//! The agent loop's stop signal lives on `Sentient` and is only checked *between*
//! tools — it cannot interrupt a command already running. This registry bridges
//! that gap: `run_command` registers each child's PID while it runs, and
//! `Sentient::stop()` calls `kill_all()` to terminate the whole process tree so
//! the Stop button actually stops a long-running scan.

use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};

use crate::process_ext::hidden_command;

fn registry() -> &'static Mutex<HashSet<u32>> {
    static REG: OnceLock<Mutex<HashSet<u32>>> = OnceLock::new();
    REG.get_or_init(|| Mutex::new(HashSet::new()))
}

/// Record a spawned foreground child PID as killable by the stop signal.
pub fn register(pid: u32) {
    if let Ok(mut set) = registry().lock() {
        set.insert(pid);
    }
}

/// Drop a PID once its command has exited normally.
pub fn unregister(pid: u32) {
    if let Ok(mut set) = registry().lock() {
        set.remove(&pid);
    }
}

/// True when at least one foreground command is currently tracked.
pub fn has_active() -> bool {
    registry().lock().map(|s| !s.is_empty()).unwrap_or(false)
}

/// Kill every tracked foreground process (and its child tree) and clear the set.
/// Called from `Sentient::stop()`. Best-effort — a PID that already exited is a
/// no-op. On Unix this relies on each child having been spawned into its own
/// process group (see `run_command`), so the negative-PID signal reaches the
/// whole group (e.g. `sh -c "nmap ..."` and the nmap it forked).
pub fn kill_all() {
    let pids: Vec<u32> = match registry().lock() {
        Ok(mut set) => set.drain().collect(),
        Err(_) => return,
    };
    for pid in pids {
        kill_tree(pid);
    }
}

fn kill_tree(pid: u32) {
    #[cfg(target_os = "windows")]
    {
        // /T kills the process tree, /F forces it.
        let _ = hidden_command("taskkill")
            .args(["/F", "/T", "/PID", &pid.to_string()])
            .status();
    }
    #[cfg(not(target_os = "windows"))]
    {
        // Negative PID targets the process group (child was spawned with
        // process_group(0), so pgid == pid). SIGKILL to be sure a wedged scan dies.
        let _ = hidden_command("kill")
            .args(["-KILL", &format!("-{pid}")])
            .status();
    }
}
