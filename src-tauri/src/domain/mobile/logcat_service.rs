use std::io::{BufRead, BufReader};
use std::process::{Child, Stdio};
use std::sync::{Arc, Mutex};

use tauri::{AppHandle, Emitter};

use crate::android_sdk;
use crate::architecture::domain::logcat::LogcatEntry;

pub struct LogcatService {
    child: Mutex<Option<Child>>,
}

impl LogcatService {
    pub fn new() -> Self {
        Self {
            child: Mutex::new(None),
        }
    }

    pub fn is_running(&self) -> bool {
        self.child
            .lock()
            .ok()
            .map(|g| g.is_some())
            .unwrap_or(false)
    }

    pub fn start(
        &self,
        app: AppHandle,
        device: Option<String>,
        filter: Option<String>,
    ) -> Result<(), String> {
        self.stop()?;
        let mut cmd = android_sdk::get_adb_cmd();
        if let Some(serial) = device.as_deref() {
            cmd.args(["-s", serial]);
        }
        cmd.arg("logcat").arg("-v").arg("threadtime");
        if let Some(f) = filter.as_deref().filter(|s| !s.is_empty()) {
            cmd.arg(f);
        }
        let mut child = cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("adb logcat failed: {e}"))?;
        let stdout = child.stdout.take().ok_or("logcat stdout unavailable")?;
        if let Ok(mut guard) = self.child.lock() {
            *guard = Some(child);
        }
        let app_clone = app.clone();
        std::thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines().flatten() {
                let entry = parse_logcat_line(&line);
                let _ = app_clone.emit("logcat-line", entry);
            }
            let _ = app_clone.emit("logcat-stopped", ());
        });
        Ok(())
    }

    pub fn stop(&self) -> Result<(), String> {
        if let Ok(mut guard) = self.child.lock() {
            if let Some(mut child) = guard.take() {
                let _ = child.kill();
            }
        }
        Ok(())
    }
}

impl Default for LogcatService {
    fn default() -> Self {
        Self::new()
    }
}

fn parse_logcat_line(raw: &str) -> LogcatEntry {
    // threadtime: MM-DD HH:MM:SS.mmm  PID  TID LEVEL TAG: message
    let parts: Vec<&str> = raw.split_whitespace().collect();
    let (level, tag, message) = if parts.len() >= 6 {
        (
            Some(parts[4].to_string()),
            parts.get(5).map(|t| t.trim_end_matches(':').to_string()),
            raw.splitn(2, ": ").nth(1).unwrap_or(raw).to_string(),
        )
    } else {
        (None, None, raw.to_string())
    };
    LogcatEntry {
        raw: raw.to_string(),
        level,
        tag,
        message,
    }
}

pub type SharedLogcatService = Arc<LogcatService>;

#[cfg(test)]
mod tests {
    use super::parse_logcat_line;

    #[test]
    fn parse_threadtime_line() {
        let raw = "03-15 10:12:01.234  1234  5678 E MyTag: Something failed";
        let entry = parse_logcat_line(raw);
        assert_eq!(entry.level.as_deref(), Some("E"));
        assert_eq!(entry.tag.as_deref(), Some("MyTag"));
        assert!(entry.message.contains("Something failed"));
    }

    #[test]
    fn parse_short_line_fallback() {
        let raw = "garbled line without structure";
        let entry = parse_logcat_line(raw);
        assert_eq!(entry.raw, raw);
        assert!(entry.level.is_none());
    }
}
