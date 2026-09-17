use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct NativePtySession {
    pub id: String,
    pub title: String,
    pub writer: Arc<Mutex<Box<dyn Write + Send>>>,
    pub lines: Arc<Mutex<Vec<String>>>,
    pub pending: Arc<Mutex<String>>,
    pub active_prompt: Arc<Mutex<String>>,
}

impl NativePtySession {
    pub fn spawn(cwd: &Path) -> Result<Self, String> {
        let shell = if cfg!(windows) {
            "powershell.exe"
        } else {
            "bash"
        };
        Self::spawn_with_shell(cwd, shell, "1: pwsh")
    }

    pub fn spawn_with_shell(cwd: &Path, shell_cmd: &str, title: &str) -> Result<Self, String> {
        let pty_system = native_pty_system();
        let pair = pty_system
            .openpty(PtySize {
                rows: 28,
                cols: 100,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| format!("Failed to open PTY: {e}"))?;

        let mut cmd = CommandBuilder::new(shell_cmd);
        cmd.cwd(cwd.as_os_str().to_owned());

        let _child = pair
            .slave
            .spawn_command(cmd)
            .map_err(|e| format!("Failed to spawn shell '{shell_cmd}': {e}"))?;

        let writer = pair
            .master
            .take_writer()
            .map_err(|e| format!("Failed to take writer: {e}"))?;
        let mut reader = pair
            .master
            .try_clone_reader()
            .map_err(|e| format!("Failed to clone reader: {e}"))?;

        let lines = Arc::new(Mutex::new(Vec::<String>::new()));
        let pending = Arc::new(Mutex::new(String::new()));
        let active_prompt = Arc::new(Mutex::new(String::new()));

        let lines_clone = lines.clone();
        let pending_clone = pending.clone();
        let active_prompt_clone = active_prompt.clone();

        // Dedicated PTY background reader thread
        thread::Builder::new()
            .name(format!("pty-reader-{title}"))
            .spawn(move || {
                let mut buf = [0u8; 4096];
                let mut line_accumulator = String::new();

                loop {
                    match reader.read(&mut buf) {
                        Ok(n) if n > 0 => {
                            let text = String::from_utf8_lossy(&buf[..n]);
                            if let Ok(mut pend) = pending_clone.lock() {
                                pend.push_str(&text);
                            }

                            for c in text.chars() {
                                if c == '\n' {
                                    let line_to_push = std::mem::take(&mut line_accumulator);
                                    if let Ok(mut l) = lines_clone.lock() {
                                        l.push(line_to_push);
                                        if l.len() > 3000 {
                                            l.drain(0..300);
                                        }
                                    }
                                    if let Ok(mut ap) = active_prompt_clone.lock() {
                                        ap.clear();
                                    }
                                } else if c != '\r' {
                                    line_accumulator.push(c);
                                    if let Ok(mut ap) = active_prompt_clone.lock() {
                                        *ap = line_accumulator.clone();
                                    }
                                }
                            }
                        }
                        _ => break,
                    }
                }
            })
            .map_err(|e| format!("Failed to spawn reader thread: {e}"))?;

        Ok(Self {
            id: title.to_string(),
            title: title.to_string(),
            writer: Arc::new(Mutex::new(writer)),
            lines,
            pending,
            active_prompt,
        })
    }

    /// Write raw bytes directly to the PTY stream (for interactive keystrokes)
    pub fn write_raw(&self, data: &[u8]) -> Result<(), String> {
        let mut w = self
            .writer
            .lock()
            .map_err(|e| format!("Poisoned writer lock: {e}"))?;
        w.write_all(data)
            .map_err(|e| format!("Write failed: {e}"))?;
        w.flush().map_err(|e| format!("Flush failed: {e}"))?;
        Ok(())
    }

    /// Write a string slice directly to the PTY without adding newline
    pub fn write_input(&self, data: &str) -> Result<(), String> {
        self.write_raw(data.as_bytes())
    }

    /// Send a complete command line with CRLF to the PTY
    pub fn send_line(&self, line: &str) -> Result<(), String> {
        let mut payload = line.to_string();
        if !payload.ends_with('\n') {
            payload.push_str("\r\n");
        }
        self.write_raw(payload.as_bytes())
    }

    /// Send Ctrl+C interrupt (0x03) to cancel running command
    pub fn send_interrupt(&self) -> Result<(), String> {
        self.write_raw(&[0x03])
    }

    pub fn clear(&self) {
        if let Ok(mut l) = self.lines.lock() {
            l.clear();
        }
        if let Ok(mut ap) = self.active_prompt.lock() {
            ap.clear();
        }
    }

    pub fn poll_lines(&self) -> Vec<String> {
        if let Ok(l) = self.lines.lock() {
            l.clone()
        } else {
            Vec::new()
        }
    }

    pub fn get_active_prompt(&self) -> String {
        if let Ok(ap) = self.active_prompt.lock() {
            ap.clone()
        } else {
            String::new()
        }
    }
}

#[allow(dead_code)]
fn strip_ansi_codes(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_escape = false;

    for c in s.chars() {
        if c == '\x1b' {
            in_escape = true;
        } else if in_escape {
            if c.is_ascii_alphabetic() {
                in_escape = false;
            }
        } else {
            out.push(c);
        }
    }

    out
}
