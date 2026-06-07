//! HADES IDE shell resolver — our own Git Bash / PortableGit bundle (not Hermes subprocess).
//!
//! Bundle layout (same as Git for Windows portable):
//!   `{HADES_HOME}/git/bin/bash.exe` + `usr/bin/` on PATH for grep, sed, ssh, etc.
//!
//! On first launch, copies bundled PortableGit from installer resources to HADES_HOME.

use serde_json::{json, Value};
use std::path::{Path, PathBuf};

pub fn hades_home() -> PathBuf {
    if let Ok(h) = std::env::var("HADES_HOME") {
        if !h.trim().is_empty() {
            return PathBuf::from(h.trim());
        }
    }
    #[cfg(windows)]
    {
        if let Ok(local) = std::env::var("LOCALAPPDATA") {
            return PathBuf::from(local).join("HADES");
        }
    }
    if let Ok(home) = std::env::var("USERPROFILE") {
        return PathBuf::from(home).join(".hades");
    }
    if let Ok(home) = std::env::var("HOME") {
        return PathBuf::from(home).join(".hades");
    }
    PathBuf::from(".hades")
}

fn portable_git_root() -> PathBuf {
    if let Ok(p) = std::env::var("HADES_PORTABLE_GIT_DIR") {
        if !p.trim().is_empty() {
            return PathBuf::from(p.trim());
        }
    }
    hades_home().join("git")
}

/// Bundled PortableGit shipped inside the IDE installer (see `bundles/portable-git`).
pub fn installer_git_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            roots.push(dir.join("bundles").join("portable-git"));
            roots.push(dir.join("resources").join("bundles").join("portable-git"));
        }
    }
    if let Ok(manifest) = std::env::var("CARGO_MANIFEST_DIR") {
        roots.push(PathBuf::from(manifest).join("bundles").join("portable-git"));
    }
    roots
}

fn repo_bundles_git() -> Option<PathBuf> {
    let mut roots: Vec<PathBuf> = installer_git_roots();
    if let Ok(manifest) = std::env::var("CARGO_MANIFEST_DIR") {
        if let Some(root) = PathBuf::from(manifest).parent() {
            roots.push(root.to_path_buf());
        }
    }
    if let Ok(cwd) = std::env::current_dir() {
        roots.push(cwd.clone());
        if let Some(p) = cwd.parent() {
            roots.push(p.to_path_buf());
        }
    }
    for root in roots {
        let cand = root.join("bundles").join("portable-git");
        if cand.is_dir() && cand.join("bin").join("bash.exe").is_file() {
            return Some(cand);
        }
        if root.ends_with("portable-git") && root.join("bin").join("bash.exe").is_file() {
            return Some(root);
        }
        let cand2 = root.join("src-tauri").join("bundles").join("portable-git");
        if cand2.is_dir() && cand2.join("bin").join("bash.exe").is_file() {
            return Some(cand2);
        }
    }
    None
}

fn file_exists(p: &Path) -> bool {
    p.is_file()
}

fn copy_dir_all(src: &Path, dst: &Path) -> Result<(), String> {
    std::fs::create_dir_all(dst).map_err(|e| e.to_string())?;
    for entry in walkdir::WalkDir::new(src)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let rel = entry.path().strip_prefix(src).map_err(|e| e.to_string())?;
        if rel.as_os_str().is_empty() {
            continue;
        }
        let target = dst.join(rel);
        if entry.file_type().is_dir() {
            std::fs::create_dir_all(&target).map_err(|e| e.to_string())?;
        } else {
            if let Some(parent) = target.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            std::fs::copy(entry.path(), &target).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

/// Copy bundled PortableGit into `%LOCALAPPDATA%\\HADES\\git` if missing.
pub fn ensure_portable_git_installed() -> Result<bool, String> {
    let dest = portable_git_root();
    let bash = dest.join("bin").join("bash.exe");
    if bash.is_file() {
        return Ok(false);
    }
    let Some(src) = repo_bundles_git() else {
        return Ok(false);
    };
    std::fs::create_dir_all(hades_home()).map_err(|e| e.to_string())?;
    if dest.exists() {
        let _ = std::fs::remove_dir_all(&dest);
    }
    copy_dir_all(&src, &dest)?;
    Ok(true)
}

/// Resolve Git Bash executable for agent + IDE terminal.
pub fn resolve_git_bash_exe() -> Option<PathBuf> {
    if let Ok(p) = std::env::var("HADES_GIT_BASH_PATH") {
        let pb = PathBuf::from(p.trim());
        if file_exists(&pb) {
            return Some(pb);
        }
    }

    let mut roots: Vec<PathBuf> = vec![portable_git_root()];
    if let Some(r) = repo_bundles_git() {
        roots.push(r);
    }
    for root in roots {
        for sub in ["bin/bash.exe", "usr/bin/bash.exe", "bin/sh.exe", "usr/bin/sh.exe"] {
            let cand = root.join(sub);
            if file_exists(&cand) {
                return Some(cand);
            }
        }
    }

    for path in [
        r"C:\Program Files\Git\bin\bash.exe",
        r"C:\Program Files (x86)\Git\bin\bash.exe",
        r"C:\Program Files\Git\usr\bin\bash.exe",
    ] {
        let cand = PathBuf::from(path);
        if file_exists(&cand) {
            return Some(cand);
        }
    }

    if let Ok(path) = which::which("bash.exe") {
        return Some(path);
    }
    if let Ok(path) = which::which("bash") {
        return Some(path);
    }
    None
}

pub fn resolve_sh_exe() -> Option<PathBuf> {
    if let Some(bash) = resolve_git_bash_exe() {
        return Some(bash);
    }
    for path in [
        r"C:\Program Files\Git\bin\sh.exe",
        r"C:\Program Files (x86)\Git\bin\sh.exe",
        r"C:\Program Files\Git\usr\bin\sh.exe",
    ] {
        let cand = PathBuf::from(path);
        if file_exists(&cand) {
            return Some(cand);
        }
    }
    which::which("sh.exe").ok().or_else(|| which::which("sh").ok())
}

/// Extra PATH dirs so `grep`, `git`, `curl`, etc. work in Git Bash sessions.
pub fn git_bash_path_extensions() -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = Vec::new();
    let mut roots: Vec<PathBuf> = vec![portable_git_root()];
    if let Some(r) = repo_bundles_git() {
        roots.push(r);
    }
    if let Some(bash) = resolve_git_bash_exe() {
        if let Some(git_root) = bash.parent().and_then(|p| p.parent()) {
            roots.push(git_root.to_path_buf());
        }
    }
    for root in roots {
        for sub in ["bin", "usr/bin", "cmd", "mingw64/bin"] {
            let d = root.join(sub);
            if d.is_dir() {
                dirs.push(d);
            }
        }
    }
    dirs.sort();
    dirs.dedup();
    dirs
}

pub fn augmented_path_for_git_bash() -> Option<String> {
    let extras: Vec<String> = git_bash_path_extensions()
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();
    if extras.is_empty() {
        return None;
    }
    let base = std::env::var("PATH").unwrap_or_default();
    Some(format!("{};{}", extras.join(";"), base))
}

#[tauri::command]
pub fn ide_shell_status() -> Result<Value, String> {
    let git_bash = resolve_git_bash_exe();
    let sh = resolve_sh_exe();
    let home = hades_home();
    let bundle = portable_git_root();
    let bundled = repo_bundles_git();
    Ok(json!({
        "hadesHome": home.to_string_lossy(),
        "portableGitDir": bundle.to_string_lossy(),
        "gitBash": git_bash.as_ref().map(|p| p.to_string_lossy().to_string()),
        "sh": sh.as_ref().map(|p| p.to_string_lossy().to_string()),
        "ready": git_bash.is_some(),
        "bundledInInstaller": bundled.is_some(),
        "bundledSource": bundled.as_ref().map(|p| p.to_string_lossy().to_string()),
        "pathExtensions": git_bash_path_extensions().iter().map(|p| p.to_string_lossy().to_string()).collect::<Vec<_>>(),
        "installHint": "PortableGit ships with the IDE installer and auto-copies to %LOCALAPPDATA%\\HADES\\git on first launch."
    }))
}

#[tauri::command]
pub fn ide_git_bash_path() -> Result<Value, String> {
    Ok(json!({
        "path": resolve_git_bash_exe().map(|p| p.to_string_lossy().to_string()),
        "hadesHome": hades_home().to_string_lossy(),
    }))
}

#[tauri::command]
pub fn ide_ensure_portable_git() -> Result<Value, String> {
    let installed = ensure_portable_git_installed()?;
    Ok(json!({
        "installed": installed,
        "ready": resolve_git_bash_exe().is_some(),
        "gitBash": resolve_git_bash_exe().map(|p| p.to_string_lossy().to_string()),
    }))
}
