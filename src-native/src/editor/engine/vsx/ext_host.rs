// Ext-Host Sidecar Bridge: spawn and manage the Node.js Extension Host process.
// Communicates via JSON-line IPC (stdin/stdout) with src-tauri/ext-host/index.js.
// Graceful fallback: if `node` is not available, all methods return None/empty.

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

use serde_json::{json, Value};

use crate::editor::engine::completion::{CompletionItem, CompletionKind};
use crate::editor::engine::decorations::{DiagnosticMarker, DiagnosticSeverity};
use crate::editor::engine::vsx::languages::{DefinitionLocation, HoverInfo};

/// A registered provider advertisement from the ext-host.
#[derive(Clone, Debug)]
pub struct ExtHostProvider {
    pub id: String,
    pub kind: String,
    pub selector: Value,
}

/// Outgoing request awaiting a correlated response.
struct PendingRequest {
    tx: std::sync::mpsc::Sender<Value>,
}

/// The Node.js Extension Host sidecar bridge.
pub struct ExtHostBridge {
    process: Option<Child>,
    stdin_writer: Arc<Mutex<Option<Box<dyn Write + Send>>>>,
    request_counter: std::sync::atomic::AtomicU64,
    pending_requests: Arc<Mutex<HashMap<String, PendingRequest>>>,
    /// Diagnostics pushed by extension providers (uri → markers).
    pub diagnostics: Arc<RwLock<HashMap<PathBuf, Vec<DiagnosticMarker>>>>,
    /// Provider registrations advertised by extensions.
    pub providers: Arc<RwLock<Vec<ExtHostProvider>>>,
    /// Notifications emitted by extensions (level, message).
    pub notifications: Arc<Mutex<Vec<(String, String)>>>,
    /// IDs of extensions that have been activated.
    pub activated_extensions: Arc<RwLock<Vec<String>>>,
    pub is_connected: bool,
}

impl std::fmt::Debug for ExtHostBridge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExtHostBridge")
            .field("is_connected", &self.is_connected)
            .finish()
    }
}

impl Clone for ExtHostBridge {
    fn clone(&self) -> Self {
        Self {
            process: None, // child process is not cloneable
            stdin_writer: self.stdin_writer.clone(),
            request_counter: std::sync::atomic::AtomicU64::new(
                self.request_counter
                    .load(std::sync::atomic::Ordering::SeqCst),
            ),
            pending_requests: self.pending_requests.clone(),
            diagnostics: self.diagnostics.clone(),
            providers: self.providers.clone(),
            notifications: self.notifications.clone(),
            activated_extensions: self.activated_extensions.clone(),
            is_connected: self.is_connected,
        }
    }
}

impl ExtHostBridge {
    /// Create a disconnected (no-op) bridge for when Node.js is absent.
    pub fn disconnected() -> Self {
        Self {
            process: None,
            stdin_writer: Arc::new(Mutex::new(None)),
            request_counter: std::sync::atomic::AtomicU64::new(1),
            pending_requests: Arc::new(Mutex::new(HashMap::new())),
            diagnostics: Arc::new(RwLock::new(HashMap::new())),
            providers: Arc::new(RwLock::new(Vec::new())),
            notifications: Arc::new(Mutex::new(Vec::new())),
            activated_extensions: Arc::new(RwLock::new(Vec::new())),
            is_connected: false,
        }
    }

    /// Attempt to spawn the ext-host Node.js process.
    /// Returns a connected bridge, or a disconnected one if Node.js / ext-host is unavailable.
    pub fn spawn(workspace_root: &Path) -> Self {
        // 1. Check if `node` is available on PATH (fast in-memory check without spawning processes)
        let node_available = std::env::var_os("PATH")
            .map(|path_var| {
                let extensions: &[&str] = if cfg!(windows) {
                    &[".exe", ".cmd", ""]
                } else {
                    &[""]
                };
                for dir in std::env::split_paths(&path_var) {
                    for ext in extensions {
                        if dir.join(format!("node{ext}")).is_file() {
                            return true;
                        }
                    }
                }
                false
            })
            .unwrap_or(false);

        if !node_available {
            eprintln!("[ext-host] Node.js not found on PATH – running in pure Rust mode");
            return Self::disconnected();
        }

        // 2. Locate ext-host/index.js relative to workspace or known paths
        let ext_host_js = find_ext_host_script(workspace_root);
        let script_path = match ext_host_js {
            Some(p) => p,
            None => {
                eprintln!("[ext-host] ext-host/index.js not found – running in pure Rust mode");
                return Self::disconnected();
            }
        };

        // 3. Spawn the process
        let mut cmd = Command::new("node");
        cmd.arg(&script_path);
        cmd.current_dir(workspace_root);
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let mut child = match cmd.spawn() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("[ext-host] Failed to spawn Node.js ext-host: {e}");
                return Self::disconnected();
            }
        };

        let stdin = child.stdin.take().expect("ext-host stdin");
        let stdout = child.stdout.take().expect("ext-host stdout");

        let stdin_writer: Arc<Mutex<Option<Box<dyn Write + Send>>>> =
            Arc::new(Mutex::new(Some(Box::new(stdin))));
        let pending_requests: Arc<Mutex<HashMap<String, PendingRequest>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let diagnostics: Arc<RwLock<HashMap<PathBuf, Vec<DiagnosticMarker>>>> =
            Arc::new(RwLock::new(HashMap::new()));
        let providers: Arc<RwLock<Vec<ExtHostProvider>>> = Arc::new(RwLock::new(Vec::new()));
        let notifications: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(Vec::new()));
        let activated_extensions: Arc<RwLock<Vec<String>>> = Arc::new(RwLock::new(Vec::new()));

        // 4. Background reader thread: parse JSON lines from ext-host stdout
        let pend_clone = pending_requests.clone();
        let diag_clone = diagnostics.clone();
        let prov_clone = providers.clone();
        let notif_clone = notifications.clone();
        let act_clone = activated_extensions.clone();

        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line_result in reader.lines() {
                let line = match line_result {
                    Ok(l) => l,
                    Err(_) => break, // pipe closed
                };

                if line.trim().is_empty() {
                    continue;
                }

                let msg: Value = match serde_json::from_str(&line) {
                    Ok(v) => v,
                    Err(_) => continue,
                };

                // Check for correlated response
                if let Some(req_id) = msg.get("_reqId").and_then(|v| v.as_str()) {
                    if let Ok(mut pending) = pend_clone.lock() {
                        if let Some(pr) = pending.remove(req_id) {
                            let _ = pr.tx.send(msg.clone());
                            continue;
                        }
                    }
                }

                // Dispatch by message type
                let msg_type = msg.get("type").and_then(|t| t.as_str()).unwrap_or("");
                match msg_type {
                    "providerRegistered" => {
                        let provider = ExtHostProvider {
                            id: msg
                                .get("id")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string(),
                            kind: msg
                                .get("kind")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string(),
                            selector: msg.get("selector").cloned().unwrap_or(json!(null)),
                        };
                        if let Ok(mut prov) = prov_clone.write() {
                            prov.push(provider);
                        }
                    }
                    "extensionActivated" => {
                        if let Some(id) = msg.get("id").and_then(|v| v.as_str()) {
                            if let Ok(mut act) = act_clone.write() {
                                act.push(id.to_string());
                            }
                        }
                    }
                    "notification" => {
                        let level = msg
                            .get("level")
                            .and_then(|v| v.as_str())
                            .unwrap_or("info")
                            .to_string();
                        let message = msg
                            .get("message")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();
                        if let Ok(mut notif) = notif_clone.lock() {
                            notif.push((level, message));
                        }
                    }
                    "diagnostics" => {
                        let uri = msg.get("uri").and_then(|u| u.as_str()).unwrap_or("");
                        let path = uri_to_path_local(uri);
                        let mut markers = Vec::new();
                        if let Some(diag_array) = msg.get("diagnostics").and_then(|d| d.as_array())
                        {
                            for d in diag_array {
                                let message = d
                                    .get("message")
                                    .and_then(|m| m.as_str())
                                    .unwrap_or("")
                                    .to_string();
                                let severity = match d.get("severity").and_then(|s| s.as_i64()) {
                                    Some(0) => DiagnosticSeverity::Error,
                                    Some(1) => DiagnosticSeverity::Warning,
                                    Some(2) => DiagnosticSeverity::Information,
                                    _ => DiagnosticSeverity::Hint,
                                };
                                let range = d.get("range").cloned().unwrap_or(json!({}));
                                let start = range.get("start").cloned().unwrap_or(json!({}));
                                let line = start.get("line").and_then(|l| l.as_u64()).unwrap_or(0)
                                    as usize;
                                let col =
                                    start.get("character").and_then(|c| c.as_u64()).unwrap_or(0)
                                        as usize;
                                let end_char = range
                                    .get("end")
                                    .and_then(|e| e.get("character"))
                                    .and_then(|c| c.as_u64())
                                    .unwrap_or(col as u64 + 1)
                                    as usize;

                                markers.push(DiagnosticMarker {
                                    row: line,
                                    start_col: col,
                                    end_col: end_char.max(col + 1),
                                    message,
                                    severity,
                                });
                            }
                        }
                        if let Ok(mut store) = diag_clone.write() {
                            store.insert(path, markers);
                        }
                    }
                    "providerResult" => {
                        // Correlated by reqId field
                        if let Some(req_id) = msg.get("reqId").and_then(|v| v.as_str()) {
                            if let Ok(mut pending) = pend_clone.lock() {
                                if let Some(pr) = pending.remove(req_id) {
                                    let _ = pr.tx.send(msg.clone());
                                }
                            }
                        }
                    }
                    _ => {
                        // Unknown message type — ignore
                    }
                }
            }
        });

        Self {
            process: Some(child),
            stdin_writer,
            request_counter: std::sync::atomic::AtomicU64::new(1),
            pending_requests,
            diagnostics,
            providers,
            notifications,
            activated_extensions,
            is_connected: true,
        }
    }

    // ── IPC Send Methods ──────────────────────────────────────────────────────

    fn send_json(&self, msg: &Value) -> bool {
        if let Ok(mut lock) = self.stdin_writer.lock() {
            if let Some(writer) = lock.as_mut() {
                let line = serde_json::to_string(msg).unwrap_or_default();
                if writer.write_all(line.as_bytes()).is_ok()
                    && writer.write_all(b"\n").is_ok()
                    && writer.flush().is_ok()
                {
                    return true;
                }
            }
        }
        false
    }

    fn next_req_id(&self) -> String {
        let n = self
            .request_counter
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        format!("native_req_{n}")
    }

    fn send_request(&self, msg: Value, timeout: Duration) -> Option<Value> {
        if !self.is_connected {
            return None;
        }

        let req_id = self.next_req_id();
        let (tx, rx) = std::sync::mpsc::channel();

        if let Ok(mut pending) = self.pending_requests.lock() {
            pending.insert(req_id.clone(), PendingRequest { tx });
        }

        let mut full_msg = msg;
        full_msg["reqId"] = json!(req_id);
        self.send_json(&full_msg);

        rx.recv_timeout(timeout).ok()
    }

    // ── Extension Lifecycle ───────────────────────────────────────────────────

    /// Send bootstrap message with extension metadata for loading.
    pub fn bootstrap(&self, extensions: Vec<Value>) {
        if !self.is_connected {
            return;
        }
        let msg = json!({
            "type": "bootstrap",
            "extensions": extensions,
        });
        self.send_json(&msg);
    }

    /// Request activation of a specific extension by ID.
    pub fn activate_extension(&self, extension_id: &str) {
        if !self.is_connected {
            return;
        }
        let msg = json!({
            "type": "activateExtension",
            "id": extension_id,
        });
        self.send_json(&msg);
    }

    /// Sync workspace folders with the ext-host.
    pub fn sync_workspace_folders(&self, folders: &[PathBuf]) {
        if !self.is_connected {
            return;
        }
        let folder_vals: Vec<Value> = folders
            .iter()
            .map(|f| {
                json!({
                    "path": f.to_string_lossy(),
                    "uri": format!("file:///{}", f.to_string_lossy().replace('\\', "/")),
                    "name": f.file_name().unwrap_or_default().to_string_lossy(),
                })
            })
            .collect();

        let msg = json!({
            "type": "syncWorkspaceFolders",
            "folders": folder_vals,
        });
        self.send_json(&msg);
    }

    // ── Document Synchronization ──────────────────────────────────────────────

    pub fn document_opened(&self, path: &Path, language_id: &str, content: &str) {
        if !self.is_connected {
            return;
        }
        let uri = path_to_uri_local(path);
        let msg = json!({
            "type": "documentOpened",
            "uri": uri,
            "languageId": language_id,
            "content": content,
            "version": 1,
        });
        self.send_json(&msg);
    }

    pub fn document_changed(&self, path: &Path, content: &str, version: i32) {
        if !self.is_connected {
            return;
        }
        let uri = path_to_uri_local(path);
        let msg = json!({
            "type": "documentChanged",
            "uri": uri,
            "content": content,
            "version": version,
        });
        self.send_json(&msg);
    }

    pub fn document_saved(&self, path: &Path) {
        if !self.is_connected {
            return;
        }
        let uri = path_to_uri_local(path);
        let msg = json!({
            "type": "documentSaved",
            "uri": uri,
        });
        self.send_json(&msg);
    }

    pub fn document_closed(&self, path: &Path) {
        if !self.is_connected {
            return;
        }
        let uri = path_to_uri_local(path);
        let msg = json!({
            "type": "documentClosed",
            "uri": uri,
        });
        self.send_json(&msg);
    }

    // ── Provider Requests (request-response) ──────────────────────────────────

    /// Request completions from ext-host JavaScript extension providers.
    pub fn request_completions(
        &self,
        path: &Path,
        language_id: &str,
        text: &str,
        line: usize,
        character: usize,
    ) -> Vec<CompletionItem> {
        if !self.is_connected {
            return Vec::new();
        }

        let msg = json!({
            "type": "provideCompletions",
            "uri": path_to_uri_local(path),
            "languageId": language_id,
            "text": text,
            "line": line,
            "character": character,
        });

        let resp = match self.send_request(msg, Duration::from_millis(800)) {
            Some(v) => v,
            None => return Vec::new(),
        };

        let items = resp.get("items").and_then(|i| i.as_array());
        match items {
            Some(arr) => arr
                .iter()
                .map(|item| {
                    let label = item
                        .get("label")
                        .and_then(|l| l.as_str())
                        .unwrap_or("")
                        .to_string();
                    let detail = item
                        .get("detail")
                        .and_then(|d| d.as_str())
                        .unwrap_or("")
                        .to_string();
                    let insert_text = item
                        .get("insertText")
                        .and_then(|t| t.as_str())
                        .unwrap_or(&label)
                        .to_string();

                    let kind = match item.get("kind").and_then(|k| k.as_i64()) {
                        Some(2) | Some(3) => CompletionKind::Function,
                        Some(7) | Some(22) => CompletionKind::Struct,
                        Some(14) => CompletionKind::Keyword,
                        Some(15) => CompletionKind::Snippet,
                        _ => CompletionKind::Variable,
                    };

                    CompletionItem {
                        label,
                        kind,
                        detail,
                        insert_text,
                    }
                })
                .collect(),
            None => Vec::new(),
        }
    }

    /// Request hover from ext-host JavaScript extension providers.
    pub fn request_hover(
        &self,
        path: &Path,
        language_id: &str,
        text: &str,
        line: usize,
        character: usize,
    ) -> Option<HoverInfo> {
        if !self.is_connected {
            return None;
        }

        let msg = json!({
            "type": "provideHover",
            "uri": path_to_uri_local(path),
            "languageId": language_id,
            "text": text,
            "line": line,
            "character": character,
        });

        let resp = self.send_request(msg, Duration::from_millis(800))?;
        let result = resp.get("result")?;
        if result.is_null() {
            return None;
        }

        // Extract hover contents from VS Code MarkdownString or plain string
        let contents = result.get("contents")?;
        let docs = if let Some(arr) = contents.as_array() {
            arr.iter()
                .filter_map(|v| {
                    v.as_str().map(|s| s.to_string()).or_else(|| {
                        v.get("value")
                            .and_then(|s| s.as_str())
                            .map(|s| s.to_string())
                    })
                })
                .collect::<Vec<_>>()
                .join("\n\n")
        } else if let Some(obj) = contents.as_object() {
            obj.get("value")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string()
        } else if let Some(s) = contents.as_str() {
            s.to_string()
        } else {
            return None;
        };

        if docs.is_empty() {
            return None;
        }

        Some(HoverInfo {
            title: "Extension Hover".to_string(),
            signature: String::new(),
            docs,
            start_col: character,
            end_col: character + 1,
        })
    }

    /// Request go-to-definition from ext-host JavaScript extension providers.
    pub fn request_definition(
        &self,
        path: &Path,
        language_id: &str,
        text: &str,
        line: usize,
        character: usize,
    ) -> Option<DefinitionLocation> {
        if !self.is_connected {
            return None;
        }

        let msg = json!({
            "type": "provideDefinition",
            "uri": path_to_uri_local(path),
            "languageId": language_id,
            "text": text,
            "line": line,
            "character": character,
        });

        let resp = self.send_request(msg, Duration::from_millis(1000))?;
        let result = resp.get("result")?;

        let loc = if let Some(arr) = result.as_array() {
            arr.first()?
        } else if result.is_object() {
            result
        } else {
            return None;
        };

        let target_uri = loc.get("uri").and_then(|u| u.as_str())?;
        let range = loc.get("range")?;
        let start = range.get("start")?;
        let def_row = start.get("line")?.as_u64()? as usize;
        let def_col = start.get("character")?.as_u64()? as usize;

        Some(DefinitionLocation {
            file_path: uri_to_path_local(target_uri),
            row: def_row,
            col: def_col,
            preview: String::new(),
        })
    }

    /// Get diagnostics for a specific file from ext-host providers.
    pub fn get_diagnostics(&self, path: &Path) -> Vec<DiagnosticMarker> {
        if let Ok(store) = self.diagnostics.read() {
            store.get(path).cloned().unwrap_or_default()
        } else {
            Vec::new()
        }
    }

    /// Drain collected notifications from extensions.
    pub fn drain_notifications(&self) -> Vec<(String, String)> {
        if let Ok(mut notif) = self.notifications.lock() {
            notif.drain(..).collect()
        } else {
            Vec::new()
        }
    }

    /// Check if any extension providers of a given kind are registered.
    pub fn has_provider(&self, kind: &str) -> bool {
        if let Ok(prov) = self.providers.read() {
            prov.iter().any(|p| p.kind == kind)
        } else {
            false
        }
    }
}

impl Drop for ExtHostBridge {
    fn drop(&mut self) {
        // Close stdin to signal the Node.js process to exit
        if let Ok(mut lock) = self.stdin_writer.lock() {
            *lock = None;
        }
        if let Some(mut child) = self.process.take() {
            let _ = child.kill();
        }
    }
}

// ── Utility Functions ─────────────────────────────────────────────────────────

fn path_to_uri_local(path: &Path) -> String {
    let normalized = path.to_string_lossy().replace('\\', "/");
    if normalized.starts_with('/') {
        format!("file://{normalized}")
    } else {
        format!("file:///{normalized}")
    }
}

fn uri_to_path_local(uri: &str) -> PathBuf {
    let stripped = uri
        .trim_start_matches("file:///")
        .trim_start_matches("file://");
    PathBuf::from(stripped.replace('/', "\\"))
}

/// Search for ext-host/index.js relative to common project structures.
fn find_ext_host_script(workspace_root: &Path) -> Option<PathBuf> {
    // Direct path: workspace_root/src-tauri/ext-host/index.js
    let direct = workspace_root
        .join("src-tauri")
        .join("ext-host")
        .join("index.js");
    if direct.exists() {
        return Some(direct);
    }

    // Walk up from workspace root
    let mut candidate = workspace_root.to_path_buf();
    for _ in 0..5 {
        let attempt = candidate
            .join("src-tauri")
            .join("ext-host")
            .join("index.js");
        if attempt.exists() {
            return Some(attempt);
        }
        if let Some(parent) = candidate.parent() {
            candidate = parent.to_path_buf();
        } else {
            break;
        }
    }

    // Check relative to executable
    if let Ok(exe_path) = std::env::current_exe() {
        let mut exe_dir = exe_path;
        for _ in 0..5 {
            let attempt = exe_dir.join("src-tauri").join("ext-host").join("index.js");
            if attempt.exists() {
                return Some(attempt);
            }
            if let Some(parent) = exe_dir.parent() {
                exe_dir = parent.to_path_buf();
            } else {
                break;
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disconnected_bridge_is_noop() {
        let bridge = ExtHostBridge::disconnected();
        assert!(!bridge.is_connected);
        assert!(bridge.get_diagnostics(Path::new("test.rs")).is_empty());
        assert!(bridge.drain_notifications().is_empty());
        assert!(!bridge.has_provider("completion"));
    }

    #[test]
    fn test_path_uri_roundtrip() {
        let path = PathBuf::from(r"C:\Users\test\project\main.rs");
        let uri = path_to_uri_local(&path);
        assert!(uri.starts_with("file:///"));
        let roundtrip = uri_to_path_local(&uri);
        assert_eq!(roundtrip.to_string_lossy(), path.to_string_lossy());
    }

    #[test]
    fn test_disconnected_completions_empty() {
        let bridge = ExtHostBridge::disconnected();
        let items = bridge.request_completions(Path::new("main.rs"), "rust", "fn main() {}", 0, 3);
        assert!(items.is_empty());
    }

    #[test]
    fn test_disconnected_hover_none() {
        let bridge = ExtHostBridge::disconnected();
        let hover = bridge.request_hover(Path::new("main.rs"), "rust", "fn main() {}", 0, 3);
        assert!(hover.is_none());
    }

    #[test]
    fn test_disconnected_definition_none() {
        let bridge = ExtHostBridge::disconnected();
        let def = bridge.request_definition(Path::new("main.rs"), "rust", "fn main() {}", 0, 3);
        assert!(def.is_none());
    }

    #[test]
    fn test_find_ext_host_script() {
        // This should find the actual ext-host/index.js in the workspace
        let workspace = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        if workspace.exists() {
            let found = find_ext_host_script(&workspace);
            assert!(
                found.is_some(),
                "Should find ext-host/index.js in workspace"
            );
        }
    }
}
