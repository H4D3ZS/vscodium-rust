use std::process::{Child, Command, Stdio};
use std::io::{Write, BufRead, BufReader};
use serde_json::{Value, json};
use std::thread;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager};

pub struct LspClient {
    child: Option<Child>,
    writer: Option<Box<dyn Write + Send>>,
}

impl LspClient {
    pub fn new() -> Self {
        Self {
            child: None,
            writer: None,
        }
    }

    pub fn start(&mut self, command: &str, app_handle: AppHandle) -> std::io::Result<()> {
        let mut child = Command::new(command)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdin = child.stdin.take().expect("Failed to open stdin");
        let stdout = child.stdout.take().expect("Failed to open stdout");
        
        self.writer = Some(Box::new(stdin));
        self.child = Some(child);

        // Background thread to read stdout
        thread::spawn(move || {
            let mut reader = BufReader::new(stdout);
            loop {
                let mut line = String::new();
                // Read headers
                loop {
                    let mut header = String::new();
                    if reader.read_line(&mut header).is_err() || header.trim().is_empty() {
                        break;
                    }
                    if header.starts_with("Content-Length: ") {
                        line = header.replace("Content-Length: ", "").trim().to_string();
                    }
                }

                if let Ok(len) = line.parse::<usize>() {
                    let mut buffer = vec![0u8; len];
                    if reader.read_exact(&mut buffer).is_ok() {
                        if let Ok(msg) = serde_json::from_slice::<Value>(&buffer) {
                             // Emit to frontend or handle internal state
                             app_handle.emit_all("lsp-msg", msg).unwrap();
                        }
                    }
                }
            }
        });

        println!("LSP server {} started", command);
        Ok(())
    }

    pub fn send_request(&mut self, id: i32, method: &str, params: Value) -> std::io::Result<()> {
        if let Some(ref mut writer) = self.writer {
            let request = json!({
                "jsonrpc": "2.0",
                "id": id,
                "method": method,
                "params": params
            });
            let content = serde_json::to_string(&request)?;
            let payload = format!("Content-Length: {}\r\n\r\n{}", content.len(), content);
            writer.write_all(payload.as_bytes())?;
            writer.flush()?;
        }
        Ok(())
    }

    pub fn stop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
        }
    }
}
