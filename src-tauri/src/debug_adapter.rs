use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::thread;

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

    pub fn start_session(&mut self, adapter_path: &str, app_handle: AppHandle) -> Result<(), String> {
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

        let app_handle_clone = app_handle.clone();
        thread::spawn(move || {
            read_dap_messages(BufReader::new(stdout), app_handle_clone);
        });

        let app_handle_err = app_handle.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines().flatten() {
                let _ = app_handle_err.emit("debug-log", line);
            }
        });

        self.active_session = Some(DebugSession { child, stdin });
        Ok(())
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

fn write_dap_message(stdin: &mut ChildStdin, body: &str) -> std::io::Result<()> {
    let header = format!("Content-Length: {}\r\n\r\n", body.as_bytes().len());
    stdin.write_all(header.as_bytes())?;
    stdin.write_all(body.as_bytes())?;
    stdin.flush()
}

/// Read DAP responses/events from adapter stdout (Content-Length framed).
fn read_dap_messages(mut reader: BufReader<impl Read>, app: AppHandle) {
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
        let _ = app.emit("dap-message", body);
    }
}
