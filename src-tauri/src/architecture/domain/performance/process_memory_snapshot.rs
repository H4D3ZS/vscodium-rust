use serde::Serialize;

/// One line in the memory breakdown tooltip (status bar / diagnostics).
#[derive(Serialize, Clone, Debug)]
pub struct ProcessMemoryLineItem {
    /// OS process name, e.g. `vscode-rust-app`, `msedgewebview2`.
    pub name: String,
    pub pid: u32,
    /// Resident set / working set in megabytes.
    pub working_set_mb: f64,
    /// Windows private bytes when available; otherwise mirrors working set.
    pub private_mb: f64,
}

/// Accurate memory picture for the IDE process **tree**.
///
/// **Why not a single `memory_mb` field?**
/// Tauri hosts the UI in WebView2 child processes. Measuring only the Rust
/// host (~300 MB) under-reports what Task Manager shows when you expand the
/// app group (~800 MB–1 GB with chat, Monaco, emulators).
#[derive(Serialize, Clone, Debug)]
pub struct ProcessMemorySnapshot {
    /// Rust/Tauri host working set only.
    pub host_working_set_mb: f64,
    /// Sum of descendant processes (WebView2 renderer, GPU, utility).
    pub child_working_set_mb: f64,
    /// `host + children` — primary number shown in the status bar.
    pub total_working_set_mb: f64,
    /// Sum of private bytes (Windows); closer to Task Manager Details view.
    pub total_private_mb: f64,
    pub child_process_count: u32,
    pub cpu_usage_percent: f32,
    pub system_total_ram_gb: u64,
    pub system_available_ram_gb: u64,
    pub breakdown: Vec<ProcessMemoryLineItem>,
}

impl ProcessMemorySnapshot {
    /// Back-compat for older frontend code expecting `memory_mb`.
    pub fn legacy_memory_mb(&self) -> u64 {
        self.total_working_set_mb.round() as u64
    }
}
