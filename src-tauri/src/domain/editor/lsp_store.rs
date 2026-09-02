//! User LSP store — install, enable/disable, uninstall custom language servers.
//!
//! Registry: `{HADES_HOME}/lsp/user/registry.json`
//! Installs: `{HADES_HOME}/lsp/user/installed/{id}/`

use crate::ide_shell;
use crate::lsp_bundle::ResolvedLaunch;
use crate::lsp_catalog::{find_preset, lsp_presets, preset_file_extensions, LspPreset};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLspRecord {
    pub id: String,
    pub name: String,
    pub languages: Vec<String>,
    pub command: String,
    pub args: Vec<String>,
    pub enabled: bool,
    pub source: String,
    pub install_source: String,
    pub installed_at: String,
    pub install_dir: Option<String>,
    #[serde(default)]
    pub file_extensions: Vec<String>,
    pub preset_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct LspUserRegistry {
    version: u32,
    servers: Vec<UserLspRecord>,
}

pub fn user_lsp_home() -> PathBuf {
    ide_shell::hades_home().join("lsp").join("user")
}

fn installed_root() -> PathBuf {
    user_lsp_home().join("installed")
}

fn registry_path() -> PathBuf {
    user_lsp_home().join("registry.json")
}

fn load_registry() -> LspUserRegistry {
    let path = registry_path();
    if !path.is_file() {
        return LspUserRegistry {
            version: 1,
            ..Default::default()
        };
    }
    fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn save_registry(reg: &LspUserRegistry) -> Result<(), String> {
    fs::create_dir_all(user_lsp_home()).map_err(|e| e.to_string())?;
    let data = serde_json::to_string_pretty(reg).map_err(|e| e.to_string())?;
    fs::write(registry_path(), data).map_err(|e| e.to_string())
}

fn slugify(raw: &str) -> String {
    raw.trim()
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

fn bundled_node_exe(config_dir: &Path) -> Option<PathBuf> {
    let mut search = crate::lsp_bundle::installer_lsp_roots();
    search.push(config_dir.join("lsp"));
    for root in search {
        let cand = root.join("typescript-language-server");
        if let Ok(found) = glob::glob(&format!("{}/**/node.exe", cand.display())) {
            if let Some(Ok(p)) = found.into_iter().next() {
                return Some(p);
            }
        }
    }
    which::which("node").ok()
}

fn npm_cmd_for(node: &Path) -> PathBuf {
    let dir = node.parent().unwrap_or(node);
    let cmd = dir.join("npm.cmd");
    if cmd.is_file() {
        cmd
    } else {
        dir.join("npm.exe")
    }
}

fn find_npm_bin(install_dir: &Path, package_hint: &str) -> Option<PathBuf> {
    let bin_dir = install_dir.join("node_modules").join(".bin");
    if !bin_dir.is_dir() {
        return None;
    }
    let hint = package_hint.to_lowercase();
    let mut fallback: Option<PathBuf> = None;
    for entry in fs::read_dir(&bin_dir).ok()?.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_lowercase();
        if name.contains(&hint) {
            return Some(path);
        }
        if name.ends_with(".cmd") && fallback.is_none() {
            fallback = Some(path);
        }
    }
    fallback
}

pub fn list_user_servers() -> Vec<UserLspRecord> {
    load_registry().servers
}

pub fn find_user_server(id: &str) -> Option<UserLspRecord> {
    let q = slugify(id);
    load_registry()
        .servers
        .into_iter()
        .find(|s| s.id == q || s.preset_id.as_deref() == Some(id))
}

pub fn resolve_user_launch(rec: &UserLspRecord) -> Option<ResolvedLaunch> {
    if !rec.enabled {
        return None;
    }
    let cmd_path = PathBuf::from(&rec.command);
    if !cmd_path.is_file() && which::which(&rec.command).is_err() {
        return None;
    }
    let (command, args) = if cmd_path.extension().and_then(|e| e.to_str()) == Some("cmd") {
        (
            "cmd".to_string(),
            std::iter::once("/C".to_string())
                .chain(std::iter::once(rec.command.clone()))
                .chain(rec.args.clone())
                .collect(),
        )
    } else {
        (rec.command.clone(), rec.args.clone())
    };
    Some(ResolvedLaunch {
        id: rec.id.clone(),
        command,
        args,
        source: rec.source.clone(),
    })
}

pub fn resolve_launch_by_server_id(
    server_id: &str,
    config_dir: &Path,
    workspace_root: Option<&Path>,
) -> Option<ResolvedLaunch> {
    if let Some(rec) = find_user_server(server_id) {
        if let Some(launch) = resolve_user_launch(&rec) {
            return Some(launch);
        }
    }
    crate::lsp_bundle::bundled_id_from_str(server_id)
        .and_then(|id| crate::lsp_bundle::resolve_launch(id, config_dir, workspace_root))
}

pub fn install_from_path(
    name: String,
    command: String,
    args: Vec<String>,
    languages: Vec<String>,
    file_extensions: Vec<String>,
    id_hint: Option<&str>,
) -> Result<UserLspRecord, String> {
    let cmd_path = PathBuf::from(&command);
    if !cmd_path.is_file() && which::which(&command).is_err() {
        return Err(format!("Command not found: {command}"));
    }
    let id = slugify(id_hint.as_deref().unwrap_or(&name));
    if id.is_empty() {
        return Err("Invalid server id".into());
    }
    let record = UserLspRecord {
        id: id.clone(),
        name,
        languages,
        command,
        args,
        enabled: true,
        source: "user-path".into(),
        install_source: "manual".into(),
        installed_at: chrono::Utc::now().to_rfc3339(),
        install_dir: None,
        file_extensions,
        preset_id: None,
    };
    let mut reg = load_registry();
    reg.servers.retain(|s| s.id != id);
    reg.servers.push(record.clone());
    reg.version = 1;
    save_registry(&reg)?;
    Ok(record)
}

pub fn install_from_preset(
    preset_id: &str,
    config_dir: &Path,
) -> Result<UserLspRecord, String> {
    let preset = find_preset(preset_id).ok_or_else(|| format!("Unknown preset: {preset_id}"))?;
    match preset.install_kind.as_str() {
        "path" => install_preset_from_path(&preset),
        "npm" => install_preset_from_npm(&preset, config_dir),
        other => Err(format!("Unsupported install kind: {other}")),
    }
}

fn install_preset_from_path(preset: &LspPreset) -> Result<UserLspRecord, String> {
    for cmd in &preset.path_commands {
        if let Ok(found) = which::which(cmd) {
            let id = slugify(&preset.id);
            let record = UserLspRecord {
                id: id.clone(),
                name: preset.name.clone(),
                languages: preset.languages.clone(),
                command: found.to_string_lossy().to_string(),
                args: preset.default_args.clone(),
                enabled: true,
                source: "user-import".into(),
                install_source: format!("preset:{}", preset.id),
                installed_at: chrono::Utc::now().to_rfc3339(),
                install_dir: None,
                file_extensions: preset_file_extensions(&preset.id),
                preset_id: Some(preset.id.clone()),
            };
            let mut reg = load_registry();
            reg.servers.retain(|s| s.id != id);
            reg.servers.push(record.clone());
            reg.version = 1;
            save_registry(&reg)?;
            return Ok(record);
        }
    }
    Err(format!(
        "None of {:?} found on PATH. Install the tool globally, then retry.",
        preset.path_commands
    ))
}

fn install_preset_from_npm(preset: &LspPreset, config_dir: &Path) -> Result<UserLspRecord, String> {
    let package = preset
        .npm_package
        .as_ref()
        .ok_or("Preset has no npm package")?;
    install_from_npm_package(
        package,
        preset.name.clone(),
        preset.languages.clone(),
        preset_file_extensions(&preset.id),
        preset.default_args.clone(),
        Some(preset.id.as_str()),
        Some(preset.id.clone()),
        config_dir,
    )
}

pub fn install_from_npm_package(
    package: &str,
    name: String,
    languages: Vec<String>,
    file_extensions: Vec<String>,
    args: Vec<String>,
    id_hint: Option<&str>,
    preset_id: Option<String>,
    config_dir: &Path,
) -> Result<UserLspRecord, String> {
    let node = bundled_node_exe(config_dir)
        .ok_or("Node.js not found. Run scripts/fetch-lsp-binaries.ps1 first.")?;
    let npm = npm_cmd_for(&node);
    let id = slugify(id_hint.unwrap_or(package));
    let dest = installed_root().join(&id);
    if dest.exists() {
        fs::remove_dir_all(&dest).map_err(|e| e.to_string())?;
    }
    fs::create_dir_all(&dest).map_err(|e| e.to_string())?;

    let status = Command::new(&npm)
        .args(["install", package, "--no-save", "--prefix", &dest.to_string_lossy()])
        .status()
        .map_err(|e| format!("npm install failed: {e}"))?;
    if !status.success() {
        return Err(format!("npm install {package} failed"));
    }

    let bin_hint = package.split('/').next_back().unwrap_or(package);
    let bin = find_npm_bin(&dest, bin_hint).ok_or_else(|| {
        format!("Could not find binary for {package} in {}", dest.display())
    })?;

    let record = UserLspRecord {
        id: id.clone(),
        name,
        languages,
        command: bin.to_string_lossy().to_string(),
        args,
        enabled: true,
        source: "user-npm".into(),
        install_source: format!("npm:{package}"),
        installed_at: chrono::Utc::now().to_rfc3339(),
        install_dir: Some(dest.to_string_lossy().to_string()),
        file_extensions,
        preset_id,
    };

    let mut reg = load_registry();
    reg.servers.retain(|s| s.id != id);
    reg.servers.push(record.clone());
    reg.version = 1;
    save_registry(&reg)?;
    Ok(record)
}

pub fn scan_path_imports() -> Vec<Value> {
    let mut out = Vec::new();
    for preset in lsp_presets() {
        if preset.install_kind != "path" {
            continue;
        }
        for cmd in &preset.path_commands {
            if let Ok(found) = which::which(cmd) {
                out.push(json!({
                    "presetId": preset.id,
                    "name": preset.name,
                    "command": found.to_string_lossy(),
                    "languages": preset.languages,
                    "alreadyInstalled": find_user_server(&preset.id).is_some(),
                }));
                break;
            }
        }
    }
    out
}

pub fn set_enabled(id: &str, enabled: bool) -> Result<UserLspRecord, String> {
    let id = slugify(id);
    let mut reg = load_registry();
    let rec = reg
        .servers
        .iter_mut()
        .find(|s| s.id == id)
        .ok_or_else(|| format!("Server not found: {id}"))?;
    rec.enabled = enabled;
    let out = rec.clone();
    save_registry(&reg)?;
    Ok(out)
}

pub fn uninstall(id: &str) -> Result<(), String> {
    let id = slugify(id);
    let mut reg = load_registry();
    let removed = reg.servers.iter().find(|s| s.id == id).cloned();
    reg.servers.retain(|s| s.id != id);
    save_registry(&reg)?;
    if let Some(rec) = removed {
        if let Some(dir) = rec.install_dir {
            let p = PathBuf::from(dir);
            if p.starts_with(&installed_root()) {
                let _ = fs::remove_dir_all(p);
            }
        }
    }
    Ok(())
}

#[tauri::command]
pub fn lsp_store_status() -> Result<Value, String> {
    Ok(json!({
        "storeDir": user_lsp_home().to_string_lossy(),
        "installedCount": list_user_servers().len(),
        "registryPath": registry_path().to_string_lossy(),
    }))
}

#[tauri::command]
pub fn lsp_store_list() -> Result<Value, String> {
    Ok(json!({ "servers": list_user_servers() }))
}

#[tauri::command]
pub fn lsp_store_catalog() -> Result<Value, String> {
    Ok(json!({ "presets": lsp_presets() }))
}

#[tauri::command]
pub fn lsp_store_scan_path() -> Result<Value, String> {
    Ok(json!({ "imports": scan_path_imports() }))
}

#[tauri::command]
pub fn lsp_store_install_preset(
    preset_id: String,
    state: tauri::State<'_, std::sync::Arc<crate::EditorState>>,
) -> Result<Value, String> {
    let record = install_from_preset(&preset_id, &state.config_dir)?;
    Ok(json!({ "ok": true, "server": record }))
}

#[tauri::command]
pub fn lsp_store_install_path(
    name: String,
    command: String,
    args: Option<Vec<String>>,
    languages: Option<Vec<String>>,
    file_extensions: Option<Vec<String>>,
    id: Option<String>,
) -> Result<Value, String> {
    let langs = languages.unwrap_or_else(|| vec!["unknown".into()]);
    let exts = file_extensions.unwrap_or_else(|| langs.clone());
    let record = install_from_path(
        name,
        command,
        args.unwrap_or_else(|| vec!["--stdio".into()]),
        langs,
        exts,
        id.as_deref(),
    )?;
    Ok(json!({ "ok": true, "server": record }))
}

#[tauri::command]
pub fn lsp_store_install_npm(
    package: String,
    name: Option<String>,
    languages: Option<Vec<String>>,
    file_extensions: Option<Vec<String>>,
    args: Option<Vec<String>>,
    id: Option<String>,
    state: tauri::State<'_, std::sync::Arc<crate::EditorState>>,
) -> Result<Value, String> {
    let display = name.unwrap_or_else(|| package.clone());
    let langs = languages.unwrap_or_else(|| vec!["unknown".into()]);
    let exts = file_extensions.unwrap_or_else(|| langs.clone());
    let record = install_from_npm_package(
        &package,
        display,
        langs,
        exts,
        args.unwrap_or_else(|| vec!["--stdio".into()]),
        id.as_deref().or(Some(package.as_str())),
        None,
        &state.config_dir,
    )?;
    Ok(json!({ "ok": true, "server": record }))
}

#[tauri::command]
pub fn lsp_store_set_enabled(id: String, enabled: bool) -> Result<Value, String> {
    let record = set_enabled(&id, enabled)?;
    Ok(json!({ "ok": true, "server": record }))
}

#[tauri::command]
pub fn lsp_store_uninstall(id: String) -> Result<Value, String> {
    uninstall(&id)?;
    Ok(json!({ "ok": true, "id": slugify(&id) }))
}
