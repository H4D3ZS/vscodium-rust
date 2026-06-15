use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use serde_json::Value;
use tauri::{AppHandle, Emitter};

pub struct DebugSession {
    pub child: Child,
    pub stdin: ChildStdin,
}

pub struct DebugManager {
    pub active_session: Option<DebugSession>,
}

impl DebugManager {
    pub fn new() -> Self {
        Self { active_session: None }
    }

    pub fn start_session(&mut self, adapter_path: &str, app_handle: AppHandle) -> Result<mpsc::Receiver<()>, String> {
        use crate::process_ext::CommandExtHidden;
        let mut child = Command::new(adapter_path)
            .hidden()
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| e.to_string())?;

        let stdin = child.stdin.take().expect("Failed to open stdin");
        let stdout = child.stdout.take().expect("Failed to open stdout");
        let stderr = child.stderr.take().expect("Failed to open stderr");

        let (init_tx, init_rx) = mpsc::channel();

        let app_handle_clone = app_handle.clone();
        thread::spawn(move || {
            read_dap_messages(BufReader::new(stdout), app_handle_clone, Some(init_tx));
        });

        let app_handle_err = app_handle.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines().flatten() {
                let _ = app_handle_err.emit("debug-log", line);
            }
        });

        self.active_session = Some(DebugSession { child, stdin });
        Ok(init_rx)
    }

    /// Wait up to `timeout` for the debug adapter to respond to `initialize`.
    pub fn wait_for_initialize(&self, init_rx: mpsc::Receiver<()>, timeout: Duration) -> Result<(), String> {
        init_rx
            .recv_timeout(timeout)
            .map_err(|_| "Debug adapter did not respond to initialize (timeout)".into())
    }

    /// Send a DAP message using Content-Length framing (VS Code standard).
    pub fn send_message(&mut self, msg: String) -> Result<(), String> {
        if let Some(session) = &mut self.active_session {
            write_dap_message(&mut session.stdin, &msg).map_err(|e| e.to_string())
        } else {
            Err("No active debug session".into())
        }
    }

    pub fn stop_session(&mut self) -> Result<(), String> {
        if let Some(mut session) = self.active_session.take() {
            session.child.kill().map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}

/// Format a DAP body with Content-Length header (for tests and transport).
pub fn format_dap_frame(body: &str) -> String {
    format!("Content-Length: {}\r\n\r\n{}", body.as_bytes().len(), body)
}

fn write_dap_message(stdin: &mut ChildStdin, body: &str) -> std::io::Result<()> {
    stdin.write_all(format_dap_frame(body).as_bytes())?;
    stdin.flush()
}

fn signal_initialize_ready(init_tx: &Option<mpsc::Sender<()>>, body: &str) {
    let Some(tx) = init_tx else { return };
    let Ok(v) = serde_json::from_str::<Value>(body) else { return };
    let is_init_response = v.get("type").and_then(|t| t.as_str()) == Some("response")
        && v.get("command").and_then(|c| c.as_str()) == Some("initialize");
    if is_init_response {
        let _ = tx.send(());
    }
}

/// Read DAP responses/events from adapter stdout (Content-Length framed).
fn read_dap_messages(
    mut reader: BufReader<impl Read>,
    app: AppHandle,
    init_tx: Option<mpsc::Sender<()>>,
) {
    loop {
        let mut content_length: Option<usize> = None;
        loop {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) => return,
                Ok(_) => {}
                Err(_) => return,
            }
            let trimmed = line.trim_end_matches(['\r', '\n']);
            if trimmed.is_empty() {
                break;
            }
            if let Some(rest) = trimmed.strip_prefix("Content-Length:") {
                content_length = rest.trim().parse().ok();
            }
        }

        let len = content_length.unwrap_or(0);
        if len == 0 {
            continue;
        }

        let mut buf = vec![0u8; len];
        if reader.read_exact(&mut buf).is_err() {
            break;
        }
        let body = String::from_utf8_lossy(&buf).to_string();
        signal_initialize_ready(&init_tx, &body);
        let _ = app.emit("dap-message", body);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dap_frame_includes_content_length() {
        let body = r#"{"type":"request","command":"initialize","seq":1}"#;
        let frame = format_dap_frame(body);
        assert!(frame.starts_with("Content-Length:"));
        assert!(frame.contains(body));
        assert_eq!(
            frame.split("\r\n\r\n").nth(1).unwrap_or(""),
            body
        );
    }

    #[test]
    fn initialize_response_signals_ready() {
        let (tx, rx) = mpsc::channel();
        let body = r#"{"type":"response","command":"initialize","request_seq":1,"success":true,"seq":2}"#;
        signal_initialize_ready(&Some(tx), body);
        assert!(rx.try_recv().is_ok());
    }
}
