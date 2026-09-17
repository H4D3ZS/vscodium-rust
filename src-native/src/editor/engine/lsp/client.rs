// Pure Rust Language Server Protocol (LSP) Client.
// Communicates with language servers (rust-analyzer, pyright, etc.) over JSON-RPC stdin/stdout.

use anyhow::{anyhow, Result};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

use crate::editor::engine::completion::{CompletionItem, CompletionKind};
use crate::editor::engine::decorations::{DiagnosticMarker, DiagnosticSeverity};
use crate::editor::engine::vsx::languages::{DefinitionLocation, HoverInfo};

pub struct NativeLspClient {
    pub command: String,
    process: Option<Child>,
    stdin_writer: Arc<Mutex<Option<Box<dyn Write + Send>>>>,
    request_counter: AtomicI64,
    pending_requests: Arc<Mutex<HashMap<i64, std::sync::mpsc::Sender<Value>>>>,
    diagnostics: Arc<RwLock<HashMap<PathBuf, Vec<DiagnosticMarker>>>>,
    pub is_initialized: bool,
}

impl std::fmt::Debug for NativeLspClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NativeLspClient")
            .field("command", &self.command)
            .field("is_initialized", &self.is_initialized)
            .finish()
    }
}

impl NativeLspClient {
    pub fn new(command: &str, args: &[String], workspace_root: &Path) -> Result<Self> {
        let mut cmd = Command::new(command);
        if !args.is_empty() {
            cmd.args(args);
        }
        cmd.current_dir(workspace_root);
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::null());

        let mut child = cmd
            .spawn()
            .map_err(|e| anyhow!("Failed to spawn LSP server '{}': {e}", command))?;

        let stdin = child.stdin.take().expect("Failed to open LSP stdin");
        let stdout = child.stdout.take().expect("Failed to open LSP stdout");

        let stdin_writer = Arc::new(Mutex::new(Some(Box::new(stdin) as Box<dyn Write + Send>)));
        let pending_requests: Arc<Mutex<HashMap<i64, std::sync::mpsc::Sender<Value>>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let diagnostics = Arc::new(RwLock::new(HashMap::new()));

        let pending_clone = pending_requests.clone();
        let diags_clone = diagnostics.clone();

        // Spawn background reader thread to process JSON-RPC responses & diagnostics
        thread::spawn(move || {
            let mut reader = BufReader::new(stdout);
            loop {
                let mut content_len: Option<usize> = None;
                loop {
                    let mut header = String::new();
                    match reader.read_line(&mut header) {
                        Ok(0) | Err(_) => return, // EOF or pipe closed
                        _ => {}
                    }
                    let trimmed = header.trim();
                    if trimmed.is_empty() {
                        break;
                    }
                    if trimmed.starts_with("Content-Length:") {
                        if let Ok(n) = trimmed["Content-Length:".len()..].trim().parse::<usize>() {
                            content_len = Some(n);
                        }
                    }
                }

                let len = match content_len {
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

                // Dispatch responses to pending requests
                if let Some(id) = msg.get("id").and_then(|v| v.as_i64()) {
                    if let Ok(mut pending) = pending_clone.lock() {
                        if let Some(sender) = pending.remove(&id) {
                            let _ = sender.send(msg);
                            continue;
                        }
                    }
                }

                // Dispatch incoming server notifications
                if msg.get("method").and_then(|m| m.as_str())
                    == Some("textDocument/publishDiagnostics")
                {
                    if let Some(params) = msg.get("params") {
                        if let Some(uri) = params.get("uri").and_then(|u| u.as_str()) {
                            let path = uri_to_path(uri);
                            let mut markers = Vec::new();
                            if let Some(diag_array) =
                                params.get("diagnostics").and_then(|d| d.as_array())
                            {
                                for d in diag_array {
                                    let message = d
                                        .get("message")
                                        .and_then(|m| m.as_str())
                                        .unwrap_or("")
                                        .to_string();
                                    let severity = match d.get("severity").and_then(|s| s.as_i64())
                                    {
                                        Some(1) => DiagnosticSeverity::Error,
                                        Some(2) => DiagnosticSeverity::Warning,
                                        Some(3) => DiagnosticSeverity::Information,
                                        _ => DiagnosticSeverity::Hint,
                                    };
                                    let range = d.get("range").cloned().unwrap_or(json!({}));
                                    let start = range.get("start").cloned().unwrap_or(json!({}));
                                    let line =
                                        start.get("line").and_then(|l| l.as_u64()).unwrap_or(0)
                                            as usize;
                                    let col = start
                                        .get("character")
                                        .and_then(|c| c.as_u64())
                                        .unwrap_or(0)
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

                            if let Ok(mut store) = diags_clone.write() {
                                store.insert(path, markers);
                            }
                        }
                    }
                }
            }
        });

        let client = Self {
            command: command.to_string(),
            process: Some(child),
            stdin_writer: stdin_writer.clone(),
            request_counter: AtomicI64::new(2),
            pending_requests,
            diagnostics,
            is_initialized: true,
        };

        // Asynchronously send initialize and initialized handshake on background thread
        // so the UI main thread is never blocked waiting for rust-analyzer or pylsp startup.
        let root_buf = workspace_root.to_path_buf();
        thread::spawn(move || {
            let uri = path_to_uri(&root_buf);
            let params = json!({
                "processId": std::process::id(),
                "rootUri": uri,
                "capabilities": {
                    "textDocument": {
                        "hover": { "contentFormat": ["markdown", "plaintext"] },
                        "definition": { "linkSupport": true },
                        "completion": {
                            "completionItem": {
                                "snippetSupport": true,
                                "documentationFormat": ["markdown", "plaintext"]
                            }
                        },
                        "publishDiagnostics": { "relatedInformation": true }
                    },
                    "workspace": {
                        "workspaceFolders": true
                    }
                }
            });

            let init_msg = json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "initialize",
                "params": params
            });
            if let Ok(content) = serde_json::to_string(&init_msg) {
                let frame = format!("Content-Length: {}\r\n\r\n{}", content.len(), content);
                if let Ok(mut lock) = stdin_writer.lock() {
                    if let Some(writer) = lock.as_mut() {
                        let _ = writer.write_all(frame.as_bytes());
                        let _ = writer.flush();
                    }
                }
            }

            let notif_msg = json!({
                "jsonrpc": "2.0",
                "method": "initialized",
                "params": {}
            });
            if let Ok(content) = serde_json::to_string(&notif_msg) {
                let frame = format!("Content-Length: {}\r\n\r\n{}", content.len(), content);
                if let Ok(mut lock) = stdin_writer.lock() {
                    if let Some(writer) = lock.as_mut() {
                        let _ = writer.write_all(frame.as_bytes());
                        let _ = writer.flush();
                    }
                }
            }
        });

        Ok(client)
    }

    /// Perform standard LSP initialize & initialized handshake
    pub fn initialize(&mut self, workspace_root: &Path) -> Result<Value> {
        let uri = path_to_uri(workspace_root);
        let params = json!({
            "processId": std::process::id(),
            "rootUri": uri,
            "capabilities": {
                "textDocument": {
                    "hover": { "contentFormat": ["markdown", "plaintext"] },
                    "definition": { "linkSupport": true },
                    "completion": {
                        "completionItem": {
                            "snippetSupport": true,
                            "documentationFormat": ["markdown", "plaintext"]
                        }
                    },
                    "publishDiagnostics": { "relatedInformation": true }
                },
                "workspace": {
                    "workspaceFolders": true
                }
            }
        });

        let res = self.send_request("initialize", params)?;
        self.send_notification("initialized", json!({}))?;
        self.is_initialized = true;
        Ok(res)
    }

    pub fn did_open(&self, path: &Path, language_id: &str, text: &str) -> Result<()> {
        let uri = path_to_uri(path);
        let params = json!({
            "textDocument": {
                "uri": uri,
                "languageId": language_id,
                "version": 1,
                "text": text,
            }
        });
        self.send_notification("textDocument/didOpen", params)
    }

    pub fn did_change(&self, path: &Path, version: i32, text: &str) -> Result<()> {
        let uri = path_to_uri(path);
        let params = json!({
            "textDocument": {
                "uri": uri,
                "version": version,
            },
            "contentChanges": [
                { "text": text }
            ]
        });
        self.send_notification("textDocument/didChange", params)
    }

    pub fn request_hover(&self, path: &Path, row: usize, col: usize) -> Option<HoverInfo> {
        let uri = path_to_uri(path);
        let params = json!({
            "textDocument": { "uri": uri },
            "position": { "line": row, "character": col }
        });

        let resp = self
            .send_request_timeout("textDocument/hover", params, Duration::from_millis(800))
            .ok()?;
        let result = resp.get("result")?;
        if result.is_null() {
            return None;
        }

        let contents = result.get("contents")?;
        let docs = if let Some(s) = contents.as_str() {
            s.to_string()
        } else if let Some(obj) = contents.as_object() {
            obj.get("value")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string()
        } else if let Some(arr) = contents.as_array() {
            arr.iter()
                .filter_map(|v| {
                    v.as_str()
                        .or_else(|| v.get("value").and_then(|s| s.as_str()))
                })
                .collect::<Vec<_>>()
                .join("\n\n")
        } else {
            String::new()
        };

        if docs.is_empty() {
            return None;
        }

        Some(HoverInfo {
            title: "LSP Hover".to_string(),
            signature: String::new(),
            docs,
            start_col: col,
            end_col: col + 1,
        })
    }

    pub fn request_definition(
        &self,
        path: &Path,
        row: usize,
        col: usize,
    ) -> Option<DefinitionLocation> {
        let uri = path_to_uri(path);
        let params = json!({
            "textDocument": { "uri": uri },
            "position": { "line": row, "character": col }
        });

        let resp = self
            .send_request_timeout(
                "textDocument/definition",
                params,
                Duration::from_millis(1000),
            )
            .ok()?;
        let result = resp.get("result")?;

        let loc = if let Some(arr) = result.as_array() {
            arr.first()?
        } else if result.is_object() {
            result
        } else {
            return None;
        };

        let target_uri = loc.get("uri").or_else(|| loc.get("targetUri"))?.as_str()?;
        let range = loc
            .get("range")
            .or_else(|| loc.get("targetSelectionRange"))?;
        let start = range.get("start")?;
        let def_row = start.get("line")?.as_u64()? as usize;
        let def_col = start.get("character")?.as_u64()? as usize;

        Some(DefinitionLocation {
            file_path: uri_to_path(target_uri),
            row: def_row,
            col: def_col,
            preview: String::new(),
        })
    }

    /// Request all references to symbol at (row, col) across workspace
    pub fn request_references(
        &self,
        path: &Path,
        row: usize,
        col: usize,
        include_declaration: bool,
    ) -> Vec<DefinitionLocation> {
        let uri = path_to_uri(path);
        let params = json!({
            "textDocument": { "uri": uri },
            "position": { "line": row, "character": col },
            "context": { "includeDeclaration": include_declaration }
        });

        let resp = match self.send_request_timeout(
            "textDocument/references",
            params,
            Duration::from_millis(1500),
        ) {
            Ok(v) => v,
            Err(_) => return Vec::new(),
        };

        let result = match resp.get("result") {
            Some(v) if v.is_array() => v.as_array().unwrap(),
            _ => return Vec::new(),
        };

        result
            .iter()
            .filter_map(|loc| {
                let target_uri = loc.get("uri")?.as_str()?;
                let range = loc.get("range")?;
                let start = range.get("start")?;
                let ref_row = start.get("line")?.as_u64()? as usize;
                let ref_col = start.get("character")?.as_u64()? as usize;

                Some(DefinitionLocation {
                    file_path: uri_to_path(target_uri),
                    row: ref_row,
                    col: ref_col,
                    preview: String::new(),
                })
            })
            .collect()
    }

    /// Request outline / document symbols for a file
    pub fn request_document_symbols(&self, path: &Path) -> Vec<DocumentSymbol> {
        let uri = path_to_uri(path);
        let params = json!({
            "textDocument": { "uri": uri }
        });

        let resp = match self.send_request_timeout(
            "textDocument/documentSymbol",
            params,
            Duration::from_millis(1200),
        ) {
            Ok(v) => v,
            Err(_) => return Vec::new(),
        };

        let result = match resp.get("result") {
            Some(v) if v.is_array() => v,
            _ => return Vec::new(),
        };

        parse_document_symbols(result)
    }

    pub fn request_completions(&self, path: &Path, row: usize, col: usize) -> Vec<CompletionItem> {
        let uri = path_to_uri(path);
        let params = json!({
            "textDocument": { "uri": uri },
            "position": { "line": row, "character": col }
        });

        let resp = match self.send_request_timeout(
            "textDocument/completion",
            params,
            Duration::from_millis(600),
        ) {
            Ok(v) => v,
            Err(_) => return Vec::new(),
        };

        let result = match resp.get("result") {
            Some(v) => v,
            None => return Vec::new(),
        };

        let items = if let Some(arr) = result.as_array() {
            arr.clone()
        } else if let Some(items_arr) = result.get("items").and_then(|i| i.as_array()) {
            items_arr.clone()
        } else {
            Vec::new()
        };

        items
            .into_iter()
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
            .collect()
    }

    /// Request document formatting from LSP server
    pub fn request_formatting(
        &self,
        path: &Path,
        tab_size: usize,
        insert_spaces: bool,
    ) -> Result<Vec<LspTextEdit>> {
        let uri = path_to_uri(path);
        let params = json!({
            "textDocument": { "uri": uri },
            "options": {
                "tabSize": tab_size,
                "insertSpaces": insert_spaces
            }
        });

        let resp = self.send_request_timeout(
            "textDocument/formatting",
            params,
            Duration::from_millis(2000),
        )?;
        let result = resp.get("result").unwrap_or(&Value::Null);
        if result.is_null() {
            return Ok(Vec::new());
        }

        Ok(parse_text_edits(result))
    }

    /// Request code actions for a given range and current diagnostics
    pub fn request_code_actions(
        &self,
        path: &Path,
        start_row: usize,
        start_col: usize,
        end_row: usize,
        end_col: usize,
    ) -> Vec<LspCodeAction> {
        let uri = path_to_uri(path);
        let current_diags = self.get_diagnostics(path);
        let diags_json: Vec<Value> = current_diags
            .iter()
            .map(|d| {
                json!({
                    "range": {
                        "start": { "line": d.row, "character": d.start_col },
                        "end": { "line": d.row, "character": d.end_col }
                    },
                    "message": d.message,
                    "severity": match d.severity {
                        DiagnosticSeverity::Error => 1,
                        DiagnosticSeverity::Warning => 2,
                        DiagnosticSeverity::Information => 3,
                        DiagnosticSeverity::Hint => 4,
                    }
                })
            })
            .collect();

        let params = json!({
            "textDocument": { "uri": uri },
            "range": {
                "start": { "line": start_row, "character": start_col },
                "end": { "line": end_row, "character": end_col }
            },
            "context": {
                "diagnostics": diags_json
            }
        });

        let resp = match self.send_request_timeout(
            "textDocument/codeAction",
            params,
            Duration::from_millis(1200),
        ) {
            Ok(v) => v,
            Err(_) => return Vec::new(),
        };

        let result = match resp.get("result") {
            Some(v) if v.is_array() => v.as_array().unwrap(),
            _ => return Vec::new(),
        };

        result
            .iter()
            .filter_map(|item| {
                let title = item.get("title")?.as_str()?.to_string();
                let kind = item
                    .get("kind")
                    .and_then(|k| k.as_str())
                    .map(|s| s.to_string());
                let is_preferred = item
                    .get("isPreferred")
                    .and_then(|p| p.as_bool())
                    .unwrap_or(false);
                let edit = item.get("edit").map(parse_workspace_edit);

                Some(LspCodeAction {
                    title,
                    kind,
                    is_preferred,
                    edit,
                })
            })
            .collect()
    }

    /// Request rename of symbol at (row, col) to new_name across workspace
    pub fn request_rename(
        &self,
        path: &Path,
        row: usize,
        col: usize,
        new_name: &str,
    ) -> Option<WorkspaceEdit> {
        let uri = path_to_uri(path);
        let params = json!({
            "textDocument": { "uri": uri },
            "position": { "line": row, "character": col },
            "newName": new_name
        });

        let resp = self
            .send_request_timeout("textDocument/rename", params, Duration::from_millis(2000))
            .ok()?;
        let result = resp.get("result")?;
        if result.is_null() {
            return None;
        }

        Some(parse_workspace_edit(result))
    }

    pub fn get_diagnostics(&self, path: &Path) -> Vec<DiagnosticMarker> {
        if let Ok(store) = self.diagnostics.read() {
            store.get(path).cloned().unwrap_or_default()
        } else {
            Vec::new()
        }
    }

    pub fn get_all_diagnostics(&self) -> HashMap<PathBuf, Vec<DiagnosticMarker>> {
        if let Ok(store) = self.diagnostics.read() {
            store.clone()
        } else {
            HashMap::new()
        }
    }

    fn send_request(&self, method: &str, params: Value) -> Result<Value> {
        self.send_request_timeout(method, params, Duration::from_secs(5))
    }

    fn send_request_timeout(
        &self,
        method: &str,
        params: Value,
        timeout: Duration,
    ) -> Result<Value> {
        let id = self.request_counter.fetch_add(1, Ordering::SeqCst);
        let msg = json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params
        });

        let (tx, rx) = std::sync::mpsc::channel();
        if let Ok(mut pending) = self.pending_requests.lock() {
            pending.insert(id, tx);
        }

        self.write_frame(&msg)?;

        rx.recv_timeout(timeout)
            .map_err(|_| anyhow!("LSP request '{}' (id {}) timed out", method, id))
    }

    fn send_notification(&self, method: &str, params: Value) -> Result<()> {
        let msg = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params
        });
        self.write_frame(&msg)
    }

    fn write_frame(&self, msg: &Value) -> Result<()> {
        let content = serde_json::to_string(msg)?;
        let frame = format!("Content-Length: {}\r\n\r\n{}", content.len(), content);

        if let Ok(mut lock) = self.stdin_writer.lock() {
            if let Some(writer) = lock.as_mut() {
                writer.write_all(frame.as_bytes())?;
                writer.flush()?;
                return Ok(());
            }
        }

        Err(anyhow!("LSP server stdin not available"))
    }
}

impl Drop for NativeLspClient {
    fn drop(&mut self) {
        if let Some(mut child) = self.process.take() {
            let _ = child.kill();
        }
    }
}

pub fn path_to_uri(path: &Path) -> String {
    let normalized = path.to_string_lossy().replace('\\', "/");
    if normalized.starts_with('/') {
        format!("file://{normalized}")
    } else {
        format!("file:///{normalized}")
    }
}

pub fn uri_to_path(uri: &str) -> PathBuf {
    let stripped = uri
        .trim_start_matches("file:///")
        .trim_start_matches("file://");
    PathBuf::from(stripped.replace('/', "\\"))
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LspPosition {
    pub line: usize,
    pub character: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LspRange {
    pub start: LspPosition,
    pub end: LspPosition,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LspTextEdit {
    pub range: LspRange,
    pub new_text: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LspCodeAction {
    pub title: String,
    pub kind: Option<String>,
    pub is_preferred: bool,
    pub edit: Option<WorkspaceEdit>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct WorkspaceEdit {
    pub changes: HashMap<PathBuf, Vec<LspTextEdit>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SymbolKind {
    File = 1,
    Module = 2,
    Namespace = 3,
    Package = 4,
    Class = 5,
    Method = 6,
    Property = 7,
    Field = 8,
    Constructor = 9,
    Enum = 10,
    Interface = 11,
    Function = 12,
    Variable = 13,
    Constant = 14,
    String = 15,
    Number = 16,
    Boolean = 17,
    Array = 18,
    Object = 19,
    Key = 20,
    Null = 21,
    EnumMember = 22,
    Struct = 23,
    Event = 24,
    Operator = 25,
    TypeParameter = 26,
}

impl SymbolKind {
    pub fn from_u64(val: u64) -> Self {
        match val {
            1 => SymbolKind::File,
            2 => SymbolKind::Module,
            3 => SymbolKind::Namespace,
            4 => SymbolKind::Package,
            5 => SymbolKind::Class,
            6 => SymbolKind::Method,
            7 => SymbolKind::Property,
            8 => SymbolKind::Field,
            9 => SymbolKind::Constructor,
            10 => SymbolKind::Enum,
            11 => SymbolKind::Interface,
            12 => SymbolKind::Function,
            13 => SymbolKind::Variable,
            14 => SymbolKind::Constant,
            15 => SymbolKind::String,
            16 => SymbolKind::Number,
            17 => SymbolKind::Boolean,
            18 => SymbolKind::Array,
            19 => SymbolKind::Object,
            20 => SymbolKind::Key,
            21 => SymbolKind::Null,
            22 => SymbolKind::EnumMember,
            23 => SymbolKind::Struct,
            24 => SymbolKind::Event,
            25 => SymbolKind::Operator,
            26 => SymbolKind::TypeParameter,
            _ => SymbolKind::Function,
        }
    }

    pub fn badge(&self) -> &'static str {
        match self {
            SymbolKind::Function | SymbolKind::Method | SymbolKind::Constructor => "fn",
            SymbolKind::Struct | SymbolKind::Class => "struct",
            SymbolKind::Enum => "enum",
            SymbolKind::Interface => "trait",
            SymbolKind::Module | SymbolKind::Namespace | SymbolKind::Package => "mod",
            SymbolKind::Constant => "const",
            SymbolKind::Variable | SymbolKind::Property | SymbolKind::Field => "var",
            _ => "sym",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DocumentSymbol {
    pub name: String,
    pub detail: Option<String>,
    pub kind: SymbolKind,
    pub range: LspRange,
    pub selection_range: LspRange,
    pub children: Vec<DocumentSymbol>,
}

pub fn parse_document_symbols(val: &Value) -> Vec<DocumentSymbol> {
    let arr = match val.as_array() {
        Some(a) => a,
        None => return Vec::new(),
    };

    arr.iter()
        .filter_map(|item| {
            let name = item.get("name")?.as_str()?.to_string();
            let kind_num = item.get("kind")?.as_u64()?;
            let kind = SymbolKind::from_u64(kind_num);
            let detail = item
                .get("detail")
                .and_then(|d| d.as_str())
                .map(|s| s.to_string());

            let range_val = item
                .get("range")
                .or_else(|| item.get("location").and_then(|l| l.get("range")))?;
            let start = range_val.get("start")?;
            let end = range_val.get("end")?;
            let range = LspRange {
                start: LspPosition {
                    line: start.get("line")?.as_u64()? as usize,
                    character: start.get("character")?.as_u64()? as usize,
                },
                end: LspPosition {
                    line: end.get("line")?.as_u64()? as usize,
                    character: end.get("character")?.as_u64()? as usize,
                },
            };

            let sel_range = if let Some(sr) = item.get("selectionRange") {
                if let (Some(s), Some(e)) = (sr.get("start"), sr.get("end")) {
                    LspRange {
                        start: LspPosition {
                            line: s.get("line")?.as_u64()? as usize,
                            character: s.get("character")?.as_u64()? as usize,
                        },
                        end: LspPosition {
                            line: e.get("line")?.as_u64()? as usize,
                            character: e.get("character")?.as_u64()? as usize,
                        },
                    }
                } else {
                    range.clone()
                }
            } else {
                range.clone()
            };

            let children = item
                .get("children")
                .map(parse_document_symbols)
                .unwrap_or_default();

            Some(DocumentSymbol {
                name,
                detail,
                kind,
                range,
                selection_range: sel_range,
                children,
            })
        })
        .collect()
}

pub fn parse_text_edits(val: &Value) -> Vec<LspTextEdit> {
    let arr = match val.as_array() {
        Some(a) => a,
        None => return Vec::new(),
    };

    arr.iter()
        .filter_map(|item| {
            let range = item.get("range")?;
            let start = range.get("start")?;
            let end = range.get("end")?;
            let start_line = start.get("line")?.as_u64()? as usize;
            let start_char = start.get("character")?.as_u64()? as usize;
            let end_line = end.get("line")?.as_u64()? as usize;
            let end_char = end.get("character")?.as_u64()? as usize;
            let new_text = item.get("newText")?.as_str()?.to_string();

            Some(LspTextEdit {
                range: LspRange {
                    start: LspPosition {
                        line: start_line,
                        character: start_char,
                    },
                    end: LspPosition {
                        line: end_line,
                        character: end_char,
                    },
                },
                new_text,
            })
        })
        .collect()
}

pub fn parse_workspace_edit(val: &Value) -> WorkspaceEdit {
    let mut changes = HashMap::new();

    if let Some(changes_obj) = val.get("changes").and_then(|c| c.as_object()) {
        for (uri_str, edits_val) in changes_obj {
            let path = uri_to_path(uri_str);
            let edits = parse_text_edits(edits_val);
            changes.insert(path, edits);
        }
    }

    if let Some(doc_changes) = val.get("documentChanges").and_then(|dc| dc.as_array()) {
        for item in doc_changes {
            if let (Some(td), Some(edits_val)) = (item.get("textDocument"), item.get("edits")) {
                if let Some(uri_str) = td.get("uri").and_then(|u| u.as_str()) {
                    let path = uri_to_path(uri_str);
                    let edits = parse_text_edits(edits_val);
                    changes.entry(path).or_insert_with(Vec::new).extend(edits);
                }
            }
        }
    }

    WorkspaceEdit { changes }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uri_path_conversion() {
        let path = PathBuf::from(r"C:\Users\test\project\main.rs");
        let uri = path_to_uri(&path);
        assert!(uri.starts_with("file:///"));

        let roundtrip = uri_to_path(&uri);
        assert_eq!(roundtrip.to_string_lossy(), path.to_string_lossy());
    }

    #[test]
    fn test_frame_formatting() {
        let msg = json!({ "jsonrpc": "2.0", "method": "test" });
        let content = serde_json::to_string(&msg).unwrap();
        let frame = format!("Content-Length: {}\r\n\r\n{}", content.len(), content);
        assert!(frame.starts_with("Content-Length: "));
        assert!(frame.contains("\r\n\r\n"));
    }

    #[test]
    fn test_parse_text_edits() {
        let val = json!([
            {
                "range": {
                    "start": { "line": 0, "character": 0 },
                    "end": { "line": 0, "character": 5 }
                },
                "newText": "hello"
            }
        ]);
        let edits = parse_text_edits(&val);
        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].new_text, "hello");
        assert_eq!(edits[0].range.start.line, 0);
        assert_eq!(edits[0].range.end.character, 5);
    }

    #[test]
    fn test_parse_workspace_edit() {
        let val = json!({
            "changes": {
                "file:///C:/project/src/main.rs": [
                    {
                        "range": {
                            "start": { "line": 1, "character": 4 },
                            "end": { "line": 1, "character": 10 }
                        },
                        "newText": "renamed_fn"
                    }
                ]
            }
        });
        let ws_edit = parse_workspace_edit(&val);
        assert_eq!(ws_edit.changes.len(), 1);
        let path = PathBuf::from(r"C:\project\src\main.rs");
        let edits = ws_edit.changes.get(&path).expect("edits for path");
        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].new_text, "renamed_fn");
    }
}
