//! HADES optional modules — install from GitHub catalog without bloating the core IDE.

use crate::ide_shell;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

pub const DEFAULT_CATALOG_URL: &str =
    "https://raw.githubusercontent.com/H4D3ZS/vscodium-rust-ide-saas/mac_dev/catalog/hades-modules.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleCategory {
    pub id: String,
    pub label: String,
    #[serde(default)]
    pub icon: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogModule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub kind: String,
    #[serde(default)]
    pub component_id: Option<String>,
    #[serde(default)]
    pub repo: Option<String>,
    #[serde(default)]
    pub branch: Option<String>,
    #[serde(default)]
    pub install_script: Option<String>,
    #[serde(default)]
    pub launch_command: Option<String>,
    #[serde(default)]
    pub launch_url: Option<String>,
    #[serde(default)]
    pub size_mb: u32,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleCatalog {
    pub version: u32,
    #[serde(default)]
    pub updated: String,
    #[serde(default)]
    pub catalog_url: String,
    #[serde(default)]
    pub publisher: String,
    #[serde(default)]
    pub categories: Vec<ModuleCategory>,
    pub modules: Vec<CatalogModule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledModule {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub category: String,
    pub enabled: bool,
    pub installed_at: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub install_dir: Option<String>,
    #[serde(default)]
    pub component_id: Option<String>,
    #[serde(default)]
    pub launch_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct ModuleRegistry {
    version: u32,
    catalog_url: String,
    modules: Vec<InstalledModule>,
}

pub fn modules_home() -> PathBuf {
    ide_shell::hades_home().join("modules")
}

fn installed_root() -> PathBuf {
    modules_home().join("installed")
}

fn registry_path() -> PathBuf {
    modules_home().join("registry.json")
}

fn bundled_catalog_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../catalog/hades-modules.json")
}

fn load_registry() -> ModuleRegistry {
    let path = registry_path();
    if !path.is_file() {
        return ModuleRegistry {
            version: 1,
            catalog_url: DEFAULT_CATALOG_URL.to_string(),
            ..Default::default()
        };
    }
    fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn save_registry(reg: &ModuleRegistry) -> Result<(), String> {
    fs::create_dir_all(modules_home()).map_err(|e| e.to_string())?;
    let data = serde_json::to_string_pretty(reg).map_err(|e| e.to_string())?;
    fs::write(registry_path(), data).map_err(|e| e.to_string())
}

fn now_iso() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_else(|_| "0".into())
}

pub fn load_bundled_catalog() -> Result<ModuleCatalog, String> {
    let path = bundled_catalog_path();
    let raw = fs::read_to_string(&path).map_err(|e| format!("bundled catalog: {e}"))?;
    serde_json::from_str(&raw).map_err(|e| e.to_string())
}

pub async fn fetch_catalog(url: Option<String>) -> Result<ModuleCatalog, String> {
    let url = url
        .filter(|u| !u.trim().is_empty())
        .unwrap_or_else(|| {
            load_registry()
                .catalog_url
                .trim()
                .to_string()
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
        });
    let url = if url.is_empty() {
        DEFAULT_CATALOG_URL.to_string()
    } else {
        url
    };

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .user_agent("HADES-IDE-ModuleInstaller/1.0")
        .build()
        .map_err(|e| e.to_string())?;

    match client.get(&url).send().await {
        Ok(resp) if resp.status().is_success() => {
            let body = resp.text().await.map_err(|e| e.to_string())?;
            let catalog: ModuleCatalog = serde_json::from_str(&body).map_err(|e| e.to_string())?;
            let mut reg = load_registry();
            reg.catalog_url = url;
            let _ = save_registry(&reg);
            Ok(catalog)
        }
        Ok(resp) => {
            let status = resp.status();
            load_bundled_catalog().map_err(|e| {
                format!("catalog fetch HTTP {status} and bundled fallback failed: {e}")
            })
        }
        Err(_) => load_bundled_catalog(),
    }
}

pub fn list_installed() -> Result<Vec<InstalledModule>, String> {
    Ok(load_registry().modules)
}

pub fn is_installed(id: &str) -> bool {
    load_registry()
        .modules
        .iter()
        .any(|m| m.id == id)
}

fn find_catalog_entry<'a>(catalog: &'a ModuleCatalog, id: &str) -> Option<&'a CatalogModule> {
    catalog.modules.iter().find(|m| m.id == id)
}

fn module_install_dir(id: &str) -> PathBuf {
    installed_root().join(id)
}

fn run_shell_in(dir: &Path, script: &str) -> Result<(), String> {
    if script.trim().is_empty() {
        return Ok(());
    }
    #[cfg(windows)]
    {
        let status = Command::new("cmd")
            .args(["/C", script])
            .current_dir(dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .status()
            .map_err(|e| format!("install script: {e}"))?;
        if !status.success() {
            return Err(format!("install script failed (exit {status})"));
        }
    }
    #[cfg(not(windows))]
    {
        let status = Command::new("sh")
            .args(["-lc", script])
            .current_dir(dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .status()
            .map_err(|e| format!("install script: {e}"))?;
        if !status.success() {
            return Err(format!("install script failed (exit {status})"));
        }
    }
    Ok(())
}

fn git_shallow_clone(repo: &str, branch: &str, dest: &Path) -> Result<(), String> {
    if dest.exists() {
        fs::remove_dir_all(dest).map_err(|e| e.to_string())?;
    }
    fs::create_dir_all(dest.parent().unwrap_or(dest)).map_err(|e| e.to_string())?;
    let url = if repo.starts_with("http") {
        repo.to_string()
    } else {
        format!("https://github.com/{repo}.git")
    };
    let status = Command::new("git")
        .args([
            "clone",
            "--depth",
            "1",
            "--branch",
            branch,
            &url,
            &dest.to_string_lossy(),
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .status()
        .map_err(|e| format!("git clone: {e} — install Git and retry"))?;
    if !status.success() {
        return Err(format!("git clone failed for {repo} (exit {status})"));
    }
    Ok(())
}

pub async fn install_module(id: String, catalog_url: Option<String>) -> Result<InstalledModule, String> {
    let catalog = fetch_catalog(catalog_url).await?;
    let entry = find_catalog_entry(&catalog, &id)
        .ok_or_else(|| format!("Module '{id}' not found in catalog"))?;

    if is_installed(&id) {
        return Err(format!("Module '{id}' is already installed"));
    }

    let install_dir = match entry.kind.as_str() {
        "component" => None,
        "git" => {
            let repo = entry
                .repo
                .as_deref()
                .ok_or_else(|| format!("Module '{id}' missing repo"))?;
            let branch = entry.branch.as_deref().unwrap_or("main");
            let dir = module_install_dir(&id);
            git_shallow_clone(repo, branch, &dir)?;
            if let Some(script) = entry.install_script.as_deref() {
                run_shell_in(&dir, script)?;
            }
            Some(dir.to_string_lossy().to_string())
        }
        other => return Err(format!("Unsupported module kind: {other}")),
    };

    let installed = InstalledModule {
        id: entry.id.clone(),
        name: entry.name.clone(),
        kind: entry.kind.clone(),
        category: entry.category.clone(),
        enabled: true,
        installed_at: now_iso(),
        version: "1.0.0".into(),
        install_dir: install_dir.clone(),
        component_id: entry.component_id.clone(),
        launch_url: entry.launch_url.clone(),
    };

    let mut reg = load_registry();
    reg.modules.push(installed.clone());
    save_registry(&reg)?;

    Ok(installed)
}

pub fn uninstall_module(id: String) -> Result<(), String> {
    let mut reg = load_registry();
    let idx = reg
        .modules
        .iter()
        .position(|m| m.id == id)
        .ok_or_else(|| format!("Module '{id}' is not installed"))?;
    let removed = reg.modules.remove(idx);
    save_registry(&reg)?;

    if let Some(dir) = removed.install_dir {
        let path = PathBuf::from(dir);
        if path.starts_with(&installed_root()) && path.exists() {
            let _ = fs::remove_dir_all(path);
        }
    }
    Ok(())
}

pub fn set_module_enabled(id: String, enabled: bool) -> Result<InstalledModule, String> {
    let mut reg = load_registry();
    let m = reg
        .modules
        .iter_mut()
        .find(|m| m.id == id)
        .ok_or_else(|| format!("Module '{id}' is not installed"))?;
    m.enabled = enabled;
    let out = m.clone();
    save_registry(&reg)?;
    Ok(out)
}

pub fn launch_module(id: String) -> Result<Value, String> {
    let reg = load_registry();
    let m = reg
        .modules
        .iter()
        .find(|m| m.id == id)
        .ok_or_else(|| format!("Module '{id}' is not installed"))?;

    if m.kind == "component" {
        return Ok(json!({
            "ok": true,
            "mode": "component",
            "component_id": m.component_id,
            "launch_url": m.launch_url,
        }));
    }

    let dir = m
        .install_dir
        .as_ref()
        .map(PathBuf::from)
        .ok_or_else(|| "Module has no install directory".to_string())?;
    if !dir.is_dir() {
        return Err(format!("Install dir missing: {}", dir.display()));
    }

    let catalog = load_bundled_catalog().ok();
    let launch_cmd = catalog
        .as_ref()
        .and_then(|c| find_catalog_entry(c, &id))
        .and_then(|e| e.launch_command.clone())
        .unwrap_or_else(|| "npm start".to_string());

    #[cfg(windows)]
    let child = Command::new("cmd")
        .args(["/C", "start", "", "cmd", "/C", &launch_cmd])
        .current_dir(&dir)
        .spawn()
        .map_err(|e| format!("launch: {e}"))?;

    #[cfg(not(windows))]
    let child = Command::new("sh")
        .args(["-lc", &launch_cmd])
        .current_dir(&dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("launch: {e}"))?;

    Ok(json!({
        "ok": true,
        "mode": "process",
        "pid": child.id(),
        "launch_url": m.launch_url,
        "install_dir": dir.to_string_lossy(),
    }))
}

pub fn merge_catalog_with_installed(catalog: &ModuleCatalog) -> Value {
    let reg = load_registry();
    let installed_ids: std::collections::HashSet<_> =
        reg.modules.iter().map(|m| m.id.as_str()).collect();

    let modules: Vec<Value> = catalog
        .modules
        .iter()
        .map(|m| {
            let inst = reg.modules.iter().find(|i| i.id == m.id);
            json!({
                "id": m.id,
                "name": m.name,
                "description": m.description,
                "category": m.category,
                "kind": m.kind,
                "component_id": m.component_id,
                "repo": m.repo,
                "size_mb": m.size_mb,
                "tags": m.tags,
                "installed": installed_ids.contains(m.id.as_str()),
                "enabled": inst.map(|i| i.enabled).unwrap_or(false),
                "installed_at": inst.map(|i| i.installed_at.clone()).unwrap_or_default(),
                "launch_url": m.launch_url,
            })
        })
        .collect();

    json!({
        "version": catalog.version,
        "updated": catalog.updated,
        "publisher": catalog.publisher,
        "catalog_url": catalog.catalog_url,
        "categories": catalog.categories,
        "modules": modules,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundled_catalog_parses() {
        let cat = load_bundled_catalog().expect("catalog");
        assert!(!cat.modules.is_empty());
        assert!(cat.modules.iter().any(|m| m.id == "flutter-sentinel"));
        assert!(cat.modules.iter().any(|m| m.id == "moxy-dast"));
        assert!(cat.modules.iter().any(|m| m.id == "chunk-secret-scanner"));
        assert!(cat.modules.iter().any(|m| m.id == "dissectx-pro"));
    }
}
