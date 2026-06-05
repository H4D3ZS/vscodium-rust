//! Bundled language servers — shipped in `binaries/lsp/` or auto-downloaded to
//! `{app_config}/lsp/`. Users never run `rustup component add` manually.

use std::fs::{self, File};
use std::io::{copy, Cursor};
use std::path::{Path, PathBuf};
use zip::ZipArchive;

/// Known IDE-managed language servers.
#[derive(Debug, Clone, Copy)]
pub enum BundledLspId {
    RustAnalyzer,
    TypeScript,
    Python,
    Go,
}

impl BundledLspId {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RustAnalyzer => "rust-analyzer",
            Self::TypeScript => "typescript-language-server",
            Self::Python => "pyright",
            Self::Go => "gopls",
        }
    }

    pub fn exe_leaf(self) -> &'static str {
        match self {
            Self::RustAnalyzer => "rust-analyzer.exe",
            Self::TypeScript => "typescript-language-server.cmd",
            Self::Python => "pyright-langserver.cmd",
            Self::Go => "gopls.exe",
        }
    }
}

/// Search roots next to the IDE exe (installer layout).
pub fn installer_lsp_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            roots.push(dir.join("binaries").join("lsp"));
            roots.push(dir.join("resources").join("binaries").join("lsp"));
            roots.push(dir.join("lsp"));
        }
    }
    // Dev: src-tauri/binaries/lsp relative to manifest dir
    if let Ok(manifest) = std::env::var("CARGO_MANIFEST_DIR") {
        roots.push(PathBuf::from(manifest).join("binaries").join("lsp"));
    }
    roots
}

fn cache_dir(config_dir: &Path) -> PathBuf {
    config_dir.join("lsp")
}

/// Resolve a bundled / cached server executable (no download).
pub fn resolve_bundled_exe(id: BundledLspId, config_dir: &Path) -> Option<PathBuf> {
    let leaf = id.exe_leaf();
    for root in installer_lsp_roots() {
        let direct = root.join(leaf);
        if direct.is_file() {
            return Some(direct);
        }
        let nested = root.join(id.as_str()).join(leaf);
        if nested.is_file() {
            return Some(nested);
        }
    }
    let cached = cache_dir(config_dir).join(id.as_str()).join(leaf);
    if cached.is_file() {
        return Some(cached);
    }
    // TypeScript / Python wrappers may live as .cmd next to node.exe
    if matches!(id, BundledLspId::TypeScript | BundledLspId::Python) {
        for root in installer_lsp_roots() {
            let wrapper = root.join(id.as_str()).join(leaf);
            if wrapper.is_file() {
                return Some(wrapper);
            }
        }
    }
    None
}

/// Launch spec: command + args for `LspClient::start`.
pub struct ResolvedLaunch {
    pub id: String,
    pub command: String,
    pub args: Vec<String>,
    pub source: String,
}

pub fn resolve_launch(id: BundledLspId, config_dir: &Path) -> Option<ResolvedLaunch> {
    if let Some(exe) = resolve_bundled_exe(id, config_dir) {
        let (command, args) = launch_for_path(id, &exe)?;
        return Some(ResolvedLaunch {
            id: id.as_str().to_string(),
            command,
            args,
            source: "bundled".into(),
        });
    }
    // PATH fallback (dev machines with global installs)
    if let Ok(cmd) = which::which(id.exe_leaf().trim_end_matches(".exe").trim_end_matches(".cmd")) {
        let p = cmd.to_string_lossy().to_string();
        let (command, args) = launch_for_path(id, Path::new(&p))?;
        return Some(ResolvedLaunch {
            id: id.as_str().to_string(),
            command,
            args,
            source: "path".into(),
        });
    }
    None
}

fn launch_for_path(id: BundledLspId, path: &Path) -> Option<(String, Vec<String>)> {
    match id {
        BundledLspId::RustAnalyzer | BundledLspId::Go => {
            Some((path.to_string_lossy().to_string(), vec![]))
        }
        BundledLspId::TypeScript => {
            // Wrapper .cmd or node + cli.mjs
            if path.extension().and_then(|e| e.to_str()) == Some("cmd") {
                Some(("cmd".into(), vec!["/C".into(), path.to_string_lossy().to_string(), "--stdio".into()]))
            } else {
                Some((path.to_string_lossy().to_string(), vec!["--stdio".into()]))
            }
        }
        BundledLspId::Python => {
            if path.extension().and_then(|e| e.to_str()) == Some("cmd") {
                Some(("cmd".into(), vec!["/C".into(), path.to_string_lossy().to_string(), "--stdio".into()]))
            } else {
                Some((path.to_string_lossy().to_string(), vec!["--stdio".into()]))
            }
        }
    }
}

fn workspace_has(root: &Path, rel: &str) -> bool {
    root.join(rel).exists()
}

/// Which bundled server does this workspace need?
pub fn workspace_lsp_id(root: &Path) -> Option<BundledLspId> {
    if workspace_has(root, "Cargo.toml") {
        return Some(BundledLspId::RustAnalyzer);
    }
    if workspace_has(root, "package.json")
        || workspace_has(root, "tsconfig.json")
        || workspace_has(root, "jsconfig.json")
    {
        return Some(BundledLspId::TypeScript);
    }
    if workspace_has(root, "pyproject.toml")
        || workspace_has(root, "requirements.txt")
        || workspace_has(root, "setup.py")
    {
        return Some(BundledLspId::Python);
    }
    if workspace_has(root, "go.mod") {
        return Some(BundledLspId::Go);
    }
    None
}

fn mirror_base() -> String {
    std::env::var("LSP_BUNDLE_MIRROR")
        .or_else(|_| std::env::var("CYBERIFRIT_LSP_MIRROR"))
        .unwrap_or_else(|_| "https://github.com/rust-lang/rust-analyzer/releases/download".to_string())
}

/// Download rust-analyzer Windows x64 into cache if missing.
pub async fn ensure_rust_analyzer(config_dir: &Path) -> Result<PathBuf, String> {
    if let Some(p) = resolve_bundled_exe(BundledLspId::RustAnalyzer, config_dir) {
        return Ok(p);
    }
    let dest_dir = cache_dir(config_dir).join("rust-analyzer");
    let dest_exe = dest_dir.join("rust-analyzer.exe");
    if dest_exe.is_file() {
        return Ok(dest_exe);
    }
    fs::create_dir_all(&dest_dir).map_err(|e| e.to_string())?;

    // Latest release tag via GitHub API
    let client = reqwest::Client::builder()
        .user_agent("vscodium-rust-ide/1.0")
        .build()
        .map_err(|e| e.to_string())?;
    let release: serde_json::Value = client
        .get("https://api.github.com/repos/rust-lang/rust-analyzer/releases/latest")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;

    let tag = release["tag_name"].as_str().unwrap_or("nightly");
    let asset_url = release["assets"]
        .as_array()
        .and_then(|a| {
            a.iter().find(|x| {
                x["name"]
                    .as_str()
                    .map(|n| n.contains("x86_64-pc-windows-msvc") && n.ends_with(".zip"))
                    .unwrap_or(false)
            })
        })
        .and_then(|x| x["browser_download_url"].as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("{}/{}/rust-analyzer-x86_64-pc-windows-msvc.zip", mirror_base(), tag));

    download_zip_exe(&client, &asset_url, &dest_dir, "rust-analyzer.exe").await?;
    Ok(dest_exe)
}

/// Download gopls Windows x64 if missing.
pub async fn ensure_gopls(config_dir: &Path) -> Result<PathBuf, String> {
    if let Some(p) = resolve_bundled_exe(BundledLspId::Go, config_dir) {
        return Ok(p);
    }
    let dest_dir = cache_dir(config_dir).join("gopls");
    let dest_exe = dest_dir.join("gopls.exe");
    if dest_exe.is_file() {
        return Ok(dest_exe);
    }
    fs::create_dir_all(&dest_dir).map_err(|e| e.to_string())?;

    let client = reqwest::Client::builder()
        .user_agent("vscodium-rust-ide/1.0")
        .build()
        .map_err(|e| e.to_string())?;

    // golang/tools gopls release — pin a stable asset pattern
    let url = "https://github.com/golang/tools/releases/download/gopls/v0.18.0/gopls-v0.18.0-windows-amd64.zip";
    download_zip_exe(&client, url, &dest_dir, "gopls.exe").await?;
    Ok(dest_exe)
}

async fn download_zip_exe(
    client: &reqwest::Client,
    url: &str,
    dest_dir: &Path,
    exe_name: &str,
) -> Result<(), String> {
    let bytes = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("download {url}: {e}"))?
        .bytes()
        .await
        .map_err(|e| e.to_string())?;

    let reader = Cursor::new(bytes);
    let mut archive = ZipArchive::new(reader).map_err(|e| format!("zip: {e}"))?;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
        let name = file.name().to_string();
        if name.ends_with(exe_name) || name.contains(exe_name) {
            let out = dest_dir.join(exe_name);
            let mut outfile = File::create(&out).map_err(|e| e.to_string())?;
            copy(&mut file, &mut outfile).map_err(|e| e.to_string())?;
            return Ok(());
        }
    }
    Err(format!("{exe_name} not found in archive from {url}"))
}

/// Ensure the IDE-managed server for this workspace is present (download if needed).
pub async fn ensure_workspace_lsp(root: &Path, config_dir: &Path) -> Result<ResolvedLaunch, String> {
    let id = workspace_lsp_id(root).ok_or("No language server mapping for this workspace type")?;

    match id {
        BundledLspId::RustAnalyzer => {
            let _ = ensure_rust_analyzer(config_dir).await?;
        }
        BundledLspId::Go => {
            let _ = ensure_gopls(config_dir).await?;
        }
        BundledLspId::TypeScript | BundledLspId::Python => {
            if resolve_bundled_exe(id, config_dir).is_none() {
                return Err(format!(
                    "{} is not bundled yet. Run scripts/fetch-lsp-binaries.ps1 before release, or set LSP_BUNDLE_MIRROR.",
                    id.as_str()
                ));
            }
        }
    }

    resolve_launch(id, config_dir).ok_or_else(|| format!("Failed to resolve {}", id.as_str()))
}

pub fn bundle_status(config_dir: &Path) -> serde_json::Value {
    let ids = [
        BundledLspId::RustAnalyzer,
        BundledLspId::TypeScript,
        BundledLspId::Python,
        BundledLspId::Go,
    ];
    let mut servers = Vec::new();
    for id in ids {
        let resolved = resolve_bundled_exe(id, config_dir);
        servers.push(serde_json::json!({
            "id": id.as_str(),
            "installed": resolved.is_some(),
            "path": resolved.map(|p| p.to_string_lossy().to_string()),
        }));
    }
    serde_json::json!({ "servers": servers, "managed": true })
}
