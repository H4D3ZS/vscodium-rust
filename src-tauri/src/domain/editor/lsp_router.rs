//! Multi-LSP router — per-file server resolution and client pool.

use crate::lsp::{DiagnosticsMap, LspClient};
use crate::lsp_bundle::{self, BundledLspId, ResolvedLaunch};
use crate::lsp_store::{self, UserLspRecord};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};

pub struct LspRouter {
    clients: HashMap<String, Arc<tokio::sync::Mutex<LspClient>>>,
    document_servers: HashMap<String, String>,
    active_server_id: Option<String>,
    diagnostics: DiagnosticsMap,
    workspace_root_uri: Option<String>,
}

#[derive(Debug, Clone)]
pub struct WorkspaceCandidate {
    pub id: String,
    pub label: String,
    pub stacks: Vec<String>,
    pub score: u8,
    pub reason: String,
    pub source: String,
}

impl LspRouter {
    pub fn new(diagnostics: DiagnosticsMap) -> Self {
        Self {
            clients: HashMap::new(),
            document_servers: HashMap::new(),
            active_server_id: None,
            diagnostics,
            workspace_root_uri: None,
        }
    }

    pub fn active_server_id(&self) -> Option<&str> {
        self.active_server_id.as_deref()
    }

    pub async fn running_server_ids(&self) -> Vec<String> {
        let mut out = Vec::new();
        for (id, client) in &self.clients {
            if client.lock().await.is_running() {
                out.push(id.clone());
            }
        }
        out
    }

    pub async fn client_for_uri(&self, uri: &str) -> Option<Arc<tokio::sync::Mutex<LspClient>>> {
        if let Some(id) = self.document_servers.get(uri) {
            return self.clients.get(id).cloned();
        }
        self.active_server_id
            .as_ref()
            .and_then(|id| self.clients.get(id))
            .cloned()
    }

    fn path_to_uri(path: &str) -> String {
        let normalized = path.replace('\\', "/");
        if normalized.starts_with('/') {
            format!("file://{normalized}")
        } else {
            format!("file:///{normalized}")
        }
    }

    pub async fn ensure_server(
        &mut self,
        server_id: &str,
        launch: &ResolvedLaunch,
        workspace_root: &str,
        app: AppHandle,
    ) -> Result<Arc<tokio::sync::Mutex<LspClient>>, String> {
        if let Some(existing) = self.clients.get(server_id) {
            if existing.lock().await.is_running() {
                return Ok(existing.clone());
            }
        }

        let client = Arc::new(tokio::sync::Mutex::new(LspClient::with_diagnostics(
            self.diagnostics.clone(),
        )));
        client
            .lock()
            .await
            .start(&launch.command, &launch.args, app.clone())
            .map_err(|e| e.to_string())?;

        tokio::time::sleep(std::time::Duration::from_millis(400)).await;

        client
            .lock()
            .await
            .send_initialized()
            .map_err(|e| e.to_string())?;

        let root_uri = Self::path_to_uri(workspace_root);
        client
            .lock()
            .await
            .set_workspace_root(&root_uri)
            .map_err(|e| e.to_string())?;

        self.clients.insert(server_id.to_string(), client.clone());
        self.workspace_root_uri = Some(root_uri);
        let _ = app.emit(
            "lsp-server-started",
            json!({ "id": server_id, "command": launch.command }),
        );
        Ok(client)
    }

    pub async fn ensure_for_file(
        &mut self,
        file_path: &Path,
        workspace_root: &Path,
        config_dir: &Path,
        language_id: &str,
        version: i32,
        text: &str,
        app: AppHandle,
    ) -> Result<Value, String> {
        let server_id = resolve_server_for_file(file_path, workspace_root, config_dir)
            .ok_or_else(|| format!("No language server for {}", file_path.display()))?;
        let launch = lsp_store::resolve_launch_by_server_id(
            &server_id,
            config_dir,
            Some(workspace_root),
        )
        .ok_or_else(|| format!("Could not launch language server: {server_id}"))?;

        let root_str = workspace_root.to_string_lossy();
        let client = self
            .ensure_server(&server_id, &launch, &root_str, app.clone())
            .await?;

        let uri = Self::path_to_uri(&file_path.to_string_lossy());
        self.document_servers
            .insert(uri.clone(), server_id.clone());
        self.active_server_id = Some(server_id.clone());

        client
            .lock()
            .await
            .did_open(&uri, language_id, version, text)
            .map_err(|e| e.to_string())?;

        Ok(json!({
            "status": "ready",
            "serverId": server_id,
            "uri": uri,
            "source": launch.source,
        }))
    }

    pub async fn did_open(
        &mut self,
        uri: &str,
        language_id: &str,
        version: i32,
        text: &str,
    ) -> Result<(), String> {
        if let Some(client) = self.client_for_uri(uri).await {
            client
                .lock()
                .await
                .did_open(uri, language_id, version, text)
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub async fn did_change(
        &mut self,
        uri: &str,
        version: i32,
        text: &str,
    ) -> Result<(), String> {
        if let Some(client) = self.client_for_uri(uri).await {
            client
                .lock()
                .await
                .did_change(uri, version, text)
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub async fn did_save(&mut self, uri: &str) -> Result<(), String> {
        if let Some(client) = self.client_for_uri(uri).await {
            client.lock().await.did_save(uri).map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub async fn set_workspace(&mut self, root_uri: &str) -> Result<(), String> {
        self.workspace_root_uri = Some(root_uri.to_string());
        for client in self.clients.values() {
            client
                .lock()
                .await
                .set_workspace_root(root_uri)
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub async fn change_workspace_folders(
        &mut self,
        folders: Vec<(String, String)>,
        removed: &[String],
    ) -> Result<(), String> {
        let refs: Vec<(&str, &str)> = folders
            .iter()
            .map(|(u, n)| (u.as_str(), n.as_str()))
            .collect();
        for client in self.clients.values() {
            if client.lock().await.is_running() {
                client
                    .lock()
                    .await
                    .sync_workspace_folders(&refs, removed)
                    .map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    pub async fn request_with_response(
        &mut self,
        uri: &str,
        method: &str,
        params: Value,
    ) -> Result<Value, String> {
        let client = self
            .client_for_uri(uri)
            .await
            .ok_or_else(|| "No LSP client for this document".to_string())?;
        let mut guard = client.lock().await;
        guard.request_with_response(method, params).await
    }

    pub async fn is_any_running(&self) -> bool {
        for client in self.clients.values() {
            if client.lock().await.is_running() {
                return true;
            }
        }
        false
    }

    pub async fn stop_all(&mut self) {
        for client in self.clients.values() {
            client.lock().await.stop();
        }
        self.clients.clear();
        self.document_servers.clear();
        self.active_server_id = None;
    }

    pub async fn stop_server(&mut self, server_id: &str) {
        if let Some(client) = self.clients.remove(server_id) {
            client.lock().await.stop();
        }
        self.document_servers
            .retain(|_, id| id != server_id);
        if self.active_server_id.as_deref() == Some(server_id) {
            self.active_server_id = self.document_servers.values().next().cloned();
        }
    }

    pub fn pool_status(&self) -> Value {
        json!({
            "active": self.active_server_id,
            "documents": self.document_servers.len(),
            "clients": self.clients.len(),
        })
    }
}

pub fn bundled_id_for_extension(ext: &str) -> Option<BundledLspId> {
    match ext.to_lowercase().as_str() {
        "rs" => Some(BundledLspId::RustAnalyzer),
        "ts" | "tsx" | "mts" | "cts" => Some(BundledLspId::TypeScript),
        "js" | "jsx" | "mjs" | "cjs" => Some(BundledLspId::TypeScript),
        "py" | "pyi" => Some(BundledLspId::Python),
        "go" => Some(BundledLspId::Go),
        "kt" | "kts" => Some(BundledLspId::Kotlin),
        "java" => Some(BundledLspId::Java),
        "dart" => Some(BundledLspId::Dart),
        "swift" => Some(BundledLspId::Swift),
        "c" | "cpp" | "cc" | "cxx" | "h" | "hpp" | "hxx" => Some(BundledLspId::Cpp),
        "cs" => Some(BundledLspId::CSharp),
        "rb" => Some(BundledLspId::Ruby),
        "php" => Some(BundledLspId::Php),
        "lua" => Some(BundledLspId::Lua),
        "zig" => Some(BundledLspId::Zig),
        "sh" | "bash" | "zsh" => Some(BundledLspId::Bash),
        "html" | "htm" | "css" | "scss" | "less" | "json" | "jsonc" => Some(BundledLspId::WebMarkup),
        "ex" | "exs" => Some(BundledLspId::Elixir),
        "r" => Some(BundledLspId::R),
        _ => None,
    }
}

fn language_matches_file(lang: &str, ext: &str, fname: &str) -> bool {
    let lang = lang.to_lowercase();
    match lang.as_str() {
        "yaml" | "yml" => ext == "yaml" || ext == "yml",
        "prisma" => ext == "prisma",
        "svelte" => ext == "svelte",
        "vue" => ext == "vue",
        "graphql" | "gql" => ext == "graphql" || ext == "gql",
        "terraform" | "hcl" | "tf" => ext == "tf" || ext == "hcl",
        "markdown" | "md" => ext == "md" || ext == "markdown",
        "latex" | "tex" => ext == "tex" || ext == "latex",
        "dockerfile" => fname == "dockerfile" || ext == "dockerfile",
        "tailwindcss" | "css" => ext == "css" || ext == "html" || ext == "htm",
        "javascript" | "typescript" => {
            matches!(ext, "js" | "jsx" | "mjs" | "cjs" | "ts" | "tsx" | "mts" | "cts")
        }
        "python" => ext == "py" || ext == "pyi",
        "ruby" => ext == "rb",
        "php" => ext == "php",
        "haskell" => ext == "hs" || ext == "lhs",
        "csharp" => ext == "cs",
        "shellscript" | "bash" => ext == "sh" || ext == "bash" || ext == "zsh",
        other => ext == other || ext == lang,
    }
}

pub fn user_server_for_file(path: &Path) -> Option<UserLspRecord> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    let fname = path
        .file_name()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    for rec in lsp_store::list_user_servers() {
        if !rec.enabled {
            continue;
        }
        if rec
            .file_extensions
            .iter()
            .any(|e| e.eq_ignore_ascii_case(&ext))
        {
            return Some(rec);
        }
        if rec
            .languages
            .iter()
            .any(|lang| language_matches_file(lang, &ext, &fname))
        {
            return Some(rec);
        }
    }
    None
}

pub fn resolve_server_for_file(
    file_path: &Path,
    workspace_root: &Path,
    config_dir: &Path,
) -> Option<String> {
    if let Some(rec) = user_server_for_file(file_path) {
        if lsp_store::resolve_user_launch(&rec).is_some() {
            return Some(rec.id);
        }
    }

    if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
        if let Some(id) = bundled_id_for_extension(ext) {
            if lsp_bundle::resolve_launch(id, config_dir, Some(workspace_root)).is_some() {
                return Some(id.as_str().to_string());
            }
        }
    }

    let fname = file_path
        .file_name()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    if fname == "dockerfile" {
        if let Some(rec) = lsp_store::find_user_server("dockerfile-language-server") {
            if rec.enabled && lsp_store::resolve_user_launch(&rec).is_some() {
                return Some(rec.id);
            }
        }
    }

    lsp_bundle::workspace_lsp_id(workspace_root).map(|id| id.as_str().to_string())
}

pub fn user_workspace_candidates(root: &Path) -> Vec<(String, String, u8, String)> {
    let mut out = Vec::new();
    for rec in lsp_store::list_user_servers() {
        if !rec.enabled || lsp_store::resolve_user_launch(&rec).is_none() {
            continue;
        }
        let preset = rec.preset_id.as_deref().unwrap_or("");
        match preset {
            "prisma-language-server" if root.join("schema.prisma").is_file() => {
                out.push((rec.id.clone(), rec.name.clone(), 97, "schema.prisma".into()));
            }
            "svelte-language-server" if lsp_bundle::glob_exists(root, "*.svelte") => {
                out.push((rec.id.clone(), rec.name.clone(), 86, "Svelte components".into()));
            }
            "vue-language-server" if lsp_bundle::glob_exists(root, "*.vue") => {
                out.push((rec.id.clone(), rec.name.clone(), 86, "Vue single-file components".into()));
            }
            "terraform-ls" if lsp_bundle::glob_exists(root, "*.tf") => {
                out.push((rec.id.clone(), rec.name.clone(), 88, "Terraform (.tf)".into()));
            }
            "tailwindcss-language-server" if lsp_bundle::has_web_marker(root) => {
                out.push((rec.id.clone(), rec.name.clone(), 79, "Tailwind CSS project".into()));
            }
            "graphql-language-service" if lsp_bundle::glob_exists(root, "*.graphql") => {
                out.push((rec.id.clone(), rec.name.clone(), 84, "GraphQL schemas".into()));
            }
            _ => {}
        }
    }
    out
}

pub fn all_workspace_candidates(root: &Path, config_dir: &Path) -> Vec<WorkspaceCandidate> {
    let mut out: Vec<WorkspaceCandidate> = lsp_bundle::workspace_lsp_candidates(root)
        .into_iter()
        .map(|(id, score, reason)| WorkspaceCandidate {
            id: id.as_str().to_string(),
            label: id.label().to_string(),
            stacks: id.stacks().iter().map(|s| s.to_string()).collect(),
            score,
            reason: reason.to_string(),
            source: "bundled".into(),
        })
        .collect();

    for (id, name, score, reason) in user_workspace_candidates(root) {
        out.push(WorkspaceCandidate {
            id: id.clone(),
            label: name,
            stacks: vec!["User installed".into()],
            score,
            reason,
            source: "user".into(),
        });
    }

    out.sort_by(|a, b| b.score.cmp(&a.score));
    out.dedup_by(|a, b| a.id == b.id);
    out
}

pub fn detect_workspace_lsp_json(root: &Path, config_dir: &Path) -> Value {
    let candidates: Vec<Value> = all_workspace_candidates(root, config_dir)
        .into_iter()
        .map(|c| {
            let installed = lsp_store::resolve_launch_by_server_id(&c.id, config_dir, Some(root))
                .is_some();
            json!({
                "id": c.id,
                "label": c.label,
                "stacks": c.stacks,
                "score": c.score,
                "reason": c.reason,
                "source": c.source,
                "installed": installed,
            })
        })
        .collect();
    let primary = candidates.first().cloned();
    json!({ "primary": primary, "candidates": candidates })
}
