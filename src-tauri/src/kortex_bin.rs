//! Kortex binary resolver — finds bundled kortex executables (aim-index,
//! aim-vfs) using the same pattern as the iOS tools resolver in
//! `iphone_device.rs`. Order: KORTEX_BIN env → exe/binaries/kortex →
//! CARGO_MANIFEST_DIR/binaries/kortex → PATH.

use std::path::PathBuf;

/// Directories where kortex binaries may be shipped alongside the IDE.
fn kortex_bin_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            roots.push(dir.join("binaries").join("kortex"));
            roots.push(dir.join("resources").join("binaries").join("kortex"));
        }
    }
    if let Ok(manifest) = std::env::var("CARGO_MANIFEST_DIR") {
        roots.push(PathBuf::from(manifest).join("binaries").join("kortex"));
    }
    roots
}

/// Find a kortex binary by name across the bundle roots. Adds `.exe` on Windows.
pub fn find_kortex_tool(names: &[&str]) -> Option<PathBuf> {
    let exe_name = |n: &str| if cfg!(windows) { format!("{n}.exe") } else { n.to_string() };

    // 1. KORTEX_BIN env override.
    if let Ok(p) = std::env::var("KORTEX_BIN") {
        for n in names {
            let cand = PathBuf::from(&p).join(exe_name(n));
            if cand.is_file() { return Some(cand); }
        }
    }

    // 2. Bundle roots.
    for root in kortex_bin_roots() {
        for n in names {
            let cand = root.join(exe_name(n));
            if cand.is_file() { return Some(cand); }
        }
    }

    // 3. PATH.
    for n in names {
        if let Ok(p) = which::which(exe_name(n)) {
            return Some(p);
        }
    }

    None
}

/// Resolve the aim-index binary (catalog builder).
pub fn resolve_aim_index() -> Option<PathBuf> {
    find_kortex_tool(&["aim-index", "aim-index.exe"])
}

/// Resolve the aim-vfs binary (virtual filesystem daemon).
pub fn resolve_aim_vfs() -> Option<PathBuf> {
    find_kortex_tool(&["aim-vfs", "aim-vfs.exe"])
}
