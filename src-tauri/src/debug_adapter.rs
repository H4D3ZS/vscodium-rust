use std::process::{Command, Child, Stdio};
use std::io::{BufReader, BufRead, Write};
use std::thread;
use serde::{Serialize, Deserialize};
use tauri::AppHandle;
use tauri::Emitter;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Breakpoint {
    pub id: Option<usize>,
    pub line: usize,
    pub path: String,
    pub verified: bool,
}

pub struct DebugSession {
    pub child: Child,
    pub stdin: std::process::ChildStdin,
    pub breakpoints: Vec<Breakpoint>,
}

pub struct DebugManager {
    pub active_session: Option<DebugSession>,
}

impl DebugManager {
    pub fn new() -> Self {
        Self {
            active_session: None,
        }
    }

    pub fn start_session(&mut self, adapter_path: &str, app_handle: AppHandle) -> Result<(), String> {
        let mut child = Command::new(adapter_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| e.to_string())?;

        let stdin = child.stdin.take().expect("Failed to open stdin");
        let stdout = child.stdout.take().expect("Failed to open stdout");
        let stderr = child.stderr.take().expect("Failed to open stderr");

        // Simple thread to read DAP messages from stdout
        let app_handle_clone = app_handle.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(l) = line {
                    // Forward DAP message to frontend
                    app_handle_clone.emit("dap-message", l).unwrap();
                }
            }
        });

        // Forward stderr to logs
        let app_handle_err = app_handle.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                if let Ok(l) = line {
                    app_handle_err.emit("debug-log", l).unwrap();
                }
            }
        });

        self.active_session = Some(DebugSession {
            child,
            stdin,
            breakpoints: Vec::new(),
        });

        Ok(())
    }

    pub fn send_message(&mut self, msg: String) -> Result<(), String> {
        if let Some(session) = &mut self.active_session {
            session.stdin.write_all(msg.as_bytes()).map_err(|e| e.to_string())?;
            session.stdin.write_all(b"\n").map_err(|e| e.to_string())?;
            session.stdin.flush().map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err("No active debug session".into())
        }
    }

    pub fn stop_session(&mut self) -> Result<(), String> {
        if let Some(mut session) = self.active_session.take() {
            session.child.kill().map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Ok(())
        }
    }
}
