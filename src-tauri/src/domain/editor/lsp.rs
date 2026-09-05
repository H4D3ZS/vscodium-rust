use std::collections::HashMap;
use std::io::{Write, BufRead, BufReader, Read};
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use std::thread;
use serde_json::{Value, json};
#[cfg(feature = "tauri")]
use tauri::{AppHandle, Emitter};
use tokio::sync::RwLock;

pub type DiagnosticsMap = Arc<RwLock<HashMap<String, Vec<Value>>>>;
pub type PendingRequests = Arc<std::sync::Mutex<HashMap<i32, tokio::sync::oneshot::Sender<Value>>>>;

pub struct LspClient {
    child: Option<Child>,
    writer: Option<Box<dyn Write + Send>>,
    request_id: i32,
    pub diagnostics: DiagnosticsMap,
    pub pending_requests: PendingRequests,
}

impl LspClient {
    pub fn new() -> Self {
        Self::with_diagnostics(Arc::new(RwLock::new(HashMap::new())))
    }

    /// Create an LspClient that shares the given DiagnosticsMap so callers
    /// can read diagnostics without going through LspClient directly.
    pub fn with_diagnostics(diagnostics: DiagnosticsMap) -> Self {
        Self {
            child: None,
            writer: None,
            request_id: 1,
            diagnostics,
            pending_requests: Arc::new(std::sync::Mutex::new(HashMap::new())),
        }
    }

    /// Start an LSP server process, perform `initialize` handshake, and
    /// spawn a background reader that stores `publishDiagnostics` notifications.
    #[cfg(feature = "tauri")]
    pub fn start(&mut self, command: &str, args: &[String], app_handle: AppHandle) -> std::io::Result<()> {
        use crate::process_ext::CommandExtHidden;

        let mut cmd = Command::new(command);
        if !args.is_empty() {
            cmd.args(args);
        }
        let mut child = cmd
            .hidden()
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?;

        let stdin = child.stdin.take().expect("LSP stdin");
        let stdout = child.stdout.take().expect("LSP stdout");

        self.writer = Some(Box::new(stdin));
        self.child = Some(child);

        let diagnostics = self.diagnostics.clone();
        let pending_requests = self.pending_requests.clone();

        thread::spawn(move || {
            let mut reader = BufReader::new(stdout);
            loop {
                // Read headers
                let mut content_length: Option<usize> = None;
                loop {
                    let mut header = String::new();
                    match reader.read_line(&mut header) {
                        Ok(0) | Err(_) => return, // EOF or error — exit thread
                        _ => {}
                    }
                    let trimmed = header.trim();
                    if trimmed.is_empty() {
                        break; // blank line = end of headers
                    }
                    if trimmed.starts_with("Content-Length:") {
                        if let Ok(n) = trimmed["Content-Length:".len()..].trim().parse::<usize>() {
                            content_length = Some(n);
                        }
                    }
                }

                let len = match content_length {
                    Some(l) => l,
                    None => continue,
                };

                let mut buf = vec![0u8; len];
                if reader.read_exact(&mut buf).is_err() {
                    return;
                }

                let msg: Value = match serde_json::from_slice(&buf) {
                    Ok(v) => v,
                    Err(_) => continue,
                };

                // Handle request responses (has numeric id + result/error, no method)
                let is_response = msg.get("id").and_then(|v| v.as_i64()).is_some()
                    && msg.get("method").is_none()
                    && (msg.get("result").is_some() || msg.get("error").is_some());
                if is_response {
                    let id = msg["id"].as_i64().unwrap_or(0) as i32;
                    if let Ok(mut map) = pending_requests.lock() {
                        if let Some(tx) = map.remove(&id) {
                            let _ = tx.send(msg.clone());
                            continue; // Don't broadcast responses as general lsp-msg
                        }
                    }
                }

                // Handle publishDiagnostics notification — store server-side
                if msg["method"] == "textDocument/publishDiagnostics" {
                    if let Some(params) = msg["params"].as_object() {
                        let uri = params.get("uri")
                            .and_then(|u| u.as_str())
                            .unwrap_or("")
                            .to_string();
                        let items = params.get("diagnostics")
                            .and_then(|d| d.as_array())
                            .cloned()
                            .unwrap_or_default();

                        // Store diagnostics — non-async: use blocking_write from sync thread
                        {
                            let diags = diagnostics.clone();
                            // Use std::thread-safe tokio handle for blocking write
                            let rt = tokio::runtime::Handle::try_current();
                            if let Ok(handle) = rt {
                                handle.block_on(async {
                                    let mut map = diags.write().await;
                                    if items.is_empty() {
                                        map.remove(&uri);
                                    } else {
                                        map.insert(uri.clone(), items.clone());
                                    }
                                });
                            } else {
                                // Fallback: fire-and-forget via blocking thread
                                let diags2 = diagnostics.clone();
                                let uri2 = uri.clone();
                                let items2 = items.clone();
                                std::thread::spawn(move || {
                                    let rt2 = tokio::runtime::Builder::new_current_thread()
                                        .enable_all().build().expect("rt");
                                    rt2.block_on(async move {
                                        let mut map = diags2.write().await;
                                        if items2.is_empty() { map.remove(&uri2); }
                                        else { map.insert(uri2, items2); }
                                    });
                                });
                            }
                        }

                        // Also emit to frontend for Monaco marker integration
                        let _ = app_handle.emit("lsp-diagnostics", json!({
                            "uri": uri,
                            "diagnostics": items
                        }));
                    }
                }

                // Emit all other messages to frontend (completions, hover, etc.)
                let _ = app_handle.emit("lsp-msg", &msg);
            }
        });

        // Send initialize request
        self.request_id += 1;
        let id = self.request_id;
        let _ = self.send_request(id, "initialize", json!({
            "processId": std::process::id(),
            "rootUri": null,
            "capabilities": {
                "textDocument": {
                    "publishDiagnostics": {
                        "relatedInformation": true,
                        "versionSupport": false,
                        "codeDescriptionSupport": true,
                        "dataSupport": true
                    },
                    "completion": {
                        "completionItem": {
                            "snippetSupport": true,
                            "documentationFormat": ["markdown", "plaintext"]
                        }
                    },
                    "hover": {
                        "contentFormat": ["markdown", "plaintext"]
                    },
                    "definition": {},
                    "references": {},
                    "documentSymbol": { "hierarchicalDocumentSymbolSupport": true },
                    "codeAction": {}
                },
                "workspace": {
                    "applyEdit": true,
                    "workspaceEdit": { "documentChanges": true }
                }
            },
            "clientInfo": { "name": "Hades-Kortex", "version": "1.0" }
        }));

        println!("[LSP] Server '{}' started (id={})", command, id);
        Ok(())
    }

    /// Send `initialized` notification after receiving `initialize` response.
    pub fn send_initialized(&mut self) -> std::io::Result<()> {
        self.send_notification("initialized", json!({}))
    }

    /// Notify the LSP that a file was opened.
    pub fn did_open(&mut self, uri: &str, language_id: &str, version: i32, text: &str) -> std::io::Result<()> {
        self.send_notification("textDocument/didOpen", json!({
            "textDocument": {
                "uri": uri,
                "languageId": language_id,
                "version": version,
                "text": text
            }
        }))
    }

    /// Notify the LSP that a file changed.
    pub fn did_change(&mut self, uri: &str, version: i32, text: &str) -> std::io::Result<()> {
        self.send_notification("textDocument/didChange", json!({
            "textDocument": { "uri": uri, "version": version },
            "contentChanges": [{ "text": text }]
        }))
    }

    /// Notify the LSP that a file was saved.
    pub fn did_save(&mut self, uri: &str) -> std::io::Result<()> {
        self.send_notification("textDocument/didSave", json!({
            "textDocument": { "uri": uri }
        }))
    }

    /// Set the workspace root after LSP is initialized.
    pub fn set_workspace_root(&mut self, root_uri: &str) -> std::io::Result<()> {
        self.sync_workspace_folders(&[(root_uri, "project")], &[])
    }

    /// Multi-root workspace: replace folder list on the LSP client.
    pub fn sync_workspace_folders(
        &mut self,
        folders: &[(&str, &str)],
        removed: &[String],
    ) -> std::io::Result<()> {
        let added: Vec<Value> = folders
            .iter()
            .map(|(uri, name)| json!({ "uri": uri, "name": name }))
            .collect();
        let removed_json: Vec<Value> = removed.iter().map(|u| json!({ "uri": u })).collect();
        self.send_notification(
            "workspace/didChangeWorkspaceFolders",
            json!({ "event": { "added": added, "removed": removed_json } }),
        )
    }

    pub fn send_request(&mut self, id: i32, method: &str, params: Value) -> std::io::Result<()> {
        self.write_message(json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params
        }))
    }

    pub fn send_notification(&mut self, method: &str, params: Value) -> std::io::Result<()> {
        self.write_message(json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params
        }))
    }

    fn write_message(&mut self, msg: Value) -> std::io::Result<()> {
        if let Some(ref mut writer) = self.writer {
            let content = serde_json::to_string(&msg)?;
            let payload = format!("Content-Length: {}\r\n\r\n{}", content.len(), content);
            writer.write_all(payload.as_bytes())?;
            writer.flush()?;
        }
        Ok(())
    }

    /// Send a request and wait for the LSP server response (async, with 3s timeout).
    /// Returns the full JSON message (including `result` or `error` key).
    pub async fn request_with_response(&mut self, method: &str, params: Value) -> Result<Value, String> {
        if self.writer.is_none() {
            return Err("LSP not running".to_string());
        }
        self.request_id += 1;
        let id = self.request_id;

        let (tx, rx) = tokio::sync::oneshot::channel::<Value>();
        {
            let mut map = self.pending_requests.lock().map_err(|e| e.to_string())?;
            map.insert(id, tx);
        }

        self.send_request(id, method, params).map_err(|e| {
            // Clean up pending entry on send failure
            if let Ok(mut map) = self.pending_requests.lock() { map.remove(&id); }
            e.to_string()
        })?;

        match tokio::time::timeout(std::time::Duration::from_millis(3000), rx).await {
            Ok(Ok(msg)) => Ok(msg),
            Ok(Err(_)) => Err("LSP channel dropped".to_string()),
            Err(_) => {
                if let Ok(mut map) = self.pending_requests.lock() { map.remove(&id); }
                Err("LSP request timed out".to_string())
            }
        }
    }

    pub fn stop(&mut self) {
        let _ = self.send_notification("exit", json!({}));
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
        }
        self.writer = None;
    }

    pub fn is_running(&self) -> bool {
        self.child.is_some()
    }
}
