//! RAM-keyed runtime profile ("potato mode").
//!
//! Machines with little RAM (4–8GB) get a *lite* profile that skips or defers
//! every non-essential background subsystem at boot: Kairos suggestion ticks,
//! the MCP listener, eager vision init, eager full-repo indexing, and
//! workspace-level LSP auto-start. All of those remain available on demand —
//! lite mode changes *when* they start, never whether the user can use them.
//!
//! Override detection with `HADES_LITE=1` (force lite) or `HADES_LITE=0`
//! (force full), regardless of detected RAM.

use serde::Serialize;
use std::sync::OnceLock;

/// Total-RAM threshold below which the lite profile activates.
/// 9 GB deliberately captures both 4GB and 8GB machines.
pub const LITE_RAM_THRESHOLD_GB: f64 = 9.0;

#[derive(Debug, Clone, Serialize)]
pub struct SystemProfile {
    pub total_ram_gb: f64,
    pub lite_mode: bool,
    /// "env" when forced via HADES_LITE, otherwise "detected".
    pub source: &'static str,
    pub threshold_gb: f64,
}

static PROFILE: OnceLock<SystemProfile> = OnceLock::new();

pub fn get() -> &'static SystemProfile {
    PROFILE.get_or_init(|| {
        let total_ram_gb = detect_total_ram_gb();
        let (lite_mode, source) = match std::env::var("HADES_LITE").ok().as_deref() {
            Some("1") | Some("true") => (true, "env"),
            Some("0") | Some("false") => (false, "env"),
            _ => (
                total_ram_gb > 0.0 && total_ram_gb < LITE_RAM_THRESHOLD_GB,
                "detected",
            ),
        };
        SystemProfile {
            total_ram_gb,
            lite_mode,
            source,
            threshold_gb: LITE_RAM_THRESHOLD_GB,
        }
    })
}

pub fn is_lite() -> bool {
    get().lite_mode
}

fn detect_total_ram_gb() -> f64 {
    let mut sys = sysinfo::System::new();
    sys.refresh_memory();
    sys.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0)
}

/// Frontend gate: panels and bootstraps query this once and cache it.
#[tauri::command]
pub fn get_system_profile() -> SystemProfile {
    get().clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_some_ram() {
        let gb = detect_total_ram_gb();
        assert!(gb > 0.5, "expected to detect host RAM, got {gb}");
    }

    #[test]
    fn profile_is_stable() {
        let a = get();
        let b = get();
        assert_eq!(a.lite_mode, b.lite_mode);
        assert_eq!(a.threshold_gb, LITE_RAM_THRESHOLD_GB);
    }
}
