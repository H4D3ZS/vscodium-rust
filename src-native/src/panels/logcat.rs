use crate::app_state::HadesNativeState;
use crate::panels::traits::BottomPanelTab;
use crate::ui::icons::{icon_12, icon_14, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;
use std::io::BufRead;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LogcatEntry {
    pub raw: String,
    pub level: Option<String>,
    pub tag: Option<String>,
    pub message: String,
}

pub fn parse_logcat_line(raw: &str) -> LogcatEntry {
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

fn android_sdk_path() -> String {
    for var in ["ANDROID_HOME", "ANDROID_SDK_ROOT"] {
        if let Ok(val) = std::env::var(var) {
            if PathBuf::from(&val).exists() {
                return val;
            }
        }
    }
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .unwrap_or_else(|_| "C:\\Users\\Default".to_string());
    let locations = [
        format!("{home}\\AppData\\Local\\Android\\Sdk"),
        format!("{home}\\Android\\Sdk"),
        format!("{home}/Library/Android/sdk"),
        "C:\\Android\\Sdk".to_string(),
        "C:\\Program Files\\Android\\Sdk".to_string(),
    ];
    for loc in &locations {
        if PathBuf::from(loc).exists() {
            return loc.clone();
        }
    }
    String::new()
}

fn adb_path() -> PathBuf {
    let sdk = android_sdk_path();
    if !sdk.is_empty() {
        let root = PathBuf::from(&sdk);
        let candidates = [
            root.join("platform-tools").join("adb.exe"),
            root.join("platform-tools").join("adb"),
            root.join("adb.exe"),
            root.join("adb"),
        ];
        for p in &candidates {
            if p.exists() {
                return p.clone();
            }
        }
    }
    PathBuf::from("adb")
}

fn adb_cmd() -> Command {
    let mut cmd = Command::new(adb_path());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x0800_0000);
    }
    cmd
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LogcatDevice {
    pub id: String,
    pub state: String,
}

pub fn list_adb_devices() -> Vec<LogcatDevice> {
    let output = adb_cmd().arg("devices").output();
    let mut devices = Vec::new();
    if let Ok(out) = output {
        let text = String::from_utf8_lossy(&out.stdout);
        for line in text.lines().skip(1) {
            let mut parts = line.split_whitespace();
            if let (Some(id), Some(state)) = (parts.next(), parts.next()) {
                if id != "List" && !id.is_empty() {
                    devices.push(LogcatDevice {
                        id: id.to_string(),
                        state: state.to_string(),
                    });
                }
            }
        }
    }
    devices
}

#[derive(Debug)]
pub struct LogcatSession {
    pub child: Arc<Mutex<Option<Child>>>,
    pub buffer: Arc<Mutex<Vec<LogcatEntry>>>,
    pub stopped: Arc<AtomicBool>,
}

impl LogcatSession {
    pub fn spawn(device: Option<&str>, filter: Option<&str>) -> Result<Self, String> {
        let mut cmd = adb_cmd();
        if let Some(serial) = device.filter(|s| !s.is_empty()) {
            cmd.args(["-s", serial]);
        }
        cmd.arg("logcat").arg("-v").arg("threadtime");
        if let Some(f) = filter.filter(|s| !s.is_empty()) {
            cmd.arg(f);
        }
        let mut child = cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("adb logcat failed: {e}"))?;
        let stdout = child.stdout.take().ok_or("logcat stdout unavailable")?;
        let child = Arc::new(Mutex::new(Some(child)));
        let buffer = Arc::new(Mutex::new(Vec::<LogcatEntry>::new()));
        let stopped = Arc::new(AtomicBool::new(false));

        let buffer_clone = buffer.clone();
        let stopped_clone = stopped.clone();
        std::thread::Builder::new()
            .name("logcat-reader".to_string())
            .spawn(move || {
                let reader = std::io::BufReader::new(stdout);
                for line in reader.lines().map_while(Result::ok) {
                    if let Ok(mut buf) = buffer_clone.lock() {
                        buf.push(parse_logcat_line(&line));
                        if buf.len() > 2000 {
                            let excess = buf.len() - 2000;
                            buf.drain(0..excess);
                        }
                    }
                }
                stopped_clone.store(true, Ordering::SeqCst);
            })
            .map_err(|e| format!("Failed to spawn logcat reader: {e}"))?;

        Ok(Self {
            child,
            buffer,
            stopped,
        })
    }

    pub fn poll_entries(&self) -> Vec<LogcatEntry> {
        if let Ok(mut buf) = self.buffer.lock() {
            std::mem::take(&mut *buf)
        } else {
            Vec::new()
        }
    }

    pub fn stop(&self) {
        if let Ok(mut guard) = self.child.lock() {
            if let Some(mut child) = guard.take() {
                let _ = child.kill();
                let _ = child.wait();
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct LogcatState {
    pub entries: Vec<LogcatEntry>,
    pub filter: String,
    pub running: bool,
    pub devices: Vec<LogcatDevice>,
    pub device: String,
    pub session: Option<Arc<LogcatSession>>,
    pub devices_loaded: bool,
}

impl Default for LogcatState {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            filter: String::new(),
            running: false,
            devices: Vec::new(),
            device: String::new(),
            session: None,
            devices_loaded: false,
        }
    }
}

fn filter_matches(filter: &str, entry: &LogcatEntry) -> bool {
    let f = filter.trim();
    if f.is_empty() {
        return true;
    }
    if f.len() == 1 && "EWIVD".contains(f) {
        return entry.level.as_deref() == Some(f);
    }
    let lower = f.to_lowercase();
    entry.raw.to_lowercase().contains(&lower)
        || entry.tag.as_deref().map(|t| t.contains(f)).unwrap_or(false)
}

impl LogcatState {
    pub fn refresh_devices(&mut self) {
        self.devices = list_adb_devices();
        if self.device.is_empty() {
            if let Some(first) = self.devices.first() {
                self.device = first.id.clone();
            }
        }
        self.devices_loaded = true;
    }

    pub fn start(&mut self) -> Result<(), String> {
        if let Some(existing) = &self.session {
            existing.stop();
        }
        let device = self.device.clone();
        let filter = self.filter.trim().to_string();
        match LogcatSession::spawn(Some(&device), Some(&filter)) {
            Ok(session) => {
                self.session = Some(Arc::new(session));
                self.running = true;
                self.entries.clear();
                Ok(())
            }
            Err(e) => {
                self.running = false;
                Err(e)
            }
        }
    }

    pub fn stop(&mut self) {
        if let Some(existing) = &self.session {
            existing.stop();
        }
        self.session = None;
        self.running = false;
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn sync(&mut self) {
        if let Some(session) = &self.session {
            let fresh = session.poll_entries();
            if !fresh.is_empty() {
                self.entries.extend(fresh);
                if self.entries.len() > 2000 {
                    let excess = self.entries.len() - 2000;
                    self.entries.drain(0..excess);
                }
            }
            if session.stopped.load(Ordering::SeqCst) {
                self.session = None;
                self.running = false;
            }
        }
    }
}

pub struct LogcatPanel;

impl BottomPanelTab for LogcatPanel {
    fn id(&self) -> &'static str {
        "logcat"
    }

    fn title(&self) -> &'static str {
        "LOGCAT"
    }

    fn render(&self, state: &HadesNativeState, cx: &mut Context<HadesAppView>) -> AnyElement {
        let theme = &state.theme;
        let logcat = &state.logcat;
        let device_label = if logcat.device.is_empty() {
            "Default device".to_string()
        } else {
            match logcat.devices.iter().find(|d| d.id == logcat.device) {
                Some(d) => format!("{} ({})", d.id, d.state),
                None => logcat.device.clone(),
            }
        };
        let filter_active = !logcat.filter.trim().is_empty();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.bg_editor)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_2()
                    .flex_wrap()
                    .h(px(32.0))
                    .px_3()
                    .bg(theme.bg_titlebar)
                    .border_b_1()
                    .border_color(theme.border_subtle)
                    .child(
                        div().flex().flex_row().items_center().gap_2().child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1p5()
                                .px_2()
                                .py_0p5()
                                .rounded(px(4.0))
                                .bg(theme.bg_raised)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        let devices = &this.state.logcat.devices;
                                        if devices.is_empty() {
                                            this.state.logcat.device.clear();
                                        } else {
                                            let next = devices
                                                .iter()
                                                .position(|d| d.id == this.state.logcat.device)
                                                .map(|i| {
                                                    devices[(i + 1) % devices.len()].id.clone()
                                                })
                                                .unwrap_or_else(|| devices[0].id.clone());
                                            this.state.logcat.device = next;
                                        }
                                        cx.notify();
                                    }),
                                )
                                .child(icon_12(IconName::Radio, theme.text_muted))
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_primary)
                                        .child(device_label),
                                ),
                        ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_1()
                            .child(
                                div()
                                    .px_2()
                                    .py_0p5()
                                    .rounded(px(4.0))
                                    .text_xs()
                                    .cursor_pointer()
                                    .bg(if filter_active {
                                        theme.bg_raised
                                    } else {
                                        theme.bg_hover
                                    })
                                    .text_color(if filter_active {
                                        theme.text_muted
                                    } else {
                                        theme.text_primary
                                    })
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _event, _window, cx| {
                                            this.state.logcat.filter.clear();
                                            cx.notify();
                                        }),
                                    )
                                    .child("All"),
                            )
                            .child(
                                div()
                                    .px_2()
                                    .py_0p5()
                                    .rounded(px(4.0))
                                    .text_xs()
                                    .cursor_pointer()
                                    .bg(if logcat.filter == "E" {
                                        theme.status_red.opacity(0.25)
                                    } else {
                                        theme.bg_raised
                                    })
                                    .text_color(if logcat.filter == "E" {
                                        theme.status_red
                                    } else {
                                        theme.text_muted
                                    })
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _event, _window, cx| {
                                            this.state.logcat.filter = "E".to_string();
                                            cx.notify();
                                        }),
                                    )
                                    .child("E"),
                            )
                            .child(
                                div()
                                    .px_2()
                                    .py_0p5()
                                    .rounded(px(4.0))
                                    .text_xs()
                                    .cursor_pointer()
                                    .bg(if logcat.filter == "W" {
                                        theme.status_yellow.opacity(0.25)
                                    } else {
                                        theme.bg_raised
                                    })
                                    .text_color(if logcat.filter == "W" {
                                        theme.status_yellow
                                    } else {
                                        theme.text_muted
                                    })
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _event, _window, cx| {
                                            this.state.logcat.filter = "W".to_string();
                                            cx.notify();
                                        }),
                                    )
                                    .child("W"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2()
                            .child(if logcat.running {
                                div()
                                    .px_2()
                                    .py_1()
                                    .rounded(px(4.0))
                                    .bg(theme.bg_raised)
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .cursor_pointer()
                                    .hover(|s| s.bg(theme.bg_hover))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _event, _window, cx| {
                                            this.state.logcat.stop();
                                            this.state.toast_manager.push_info("Logcat stopped");
                                            cx.notify();
                                        }),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .gap_1()
                                            .child(icon_12(IconName::Square, theme.status_red))
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(theme.text_primary)
                                                    .child("Stop"),
                                            ),
                                    )
                                    .into_any_element()
                            } else {
                                div()
                                    .px_2()
                                    .py_1()
                                    .rounded(px(4.0))
                                    .bg(theme.accent)
                                    .cursor_pointer()
                                    .hover(|s| s.opacity(0.85))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _event, _window, cx| {
                                            if !this.state.logcat.devices_loaded {
                                                this.state.logcat.refresh_devices();
                                            }
                                            match this.state.logcat.start() {
                                                Ok(()) => {
                                                    this.state
                                                        .toast_manager
                                                        .push_success("Started adb logcat");
                                                }
                                                Err(e) => {
                                                    this.state.toast_manager.push_error(&e);
                                                }
                                            }
                                            cx.notify();
                                        }),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .gap_1()
                                            .child(icon_12(IconName::Play, theme.text_primary))
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(theme.text_primary)
                                                    .child("Start Logcat"),
                                            ),
                                    )
                                    .into_any_element()
                            })
                            .child(
                                div()
                                    .p_1()
                                    .rounded(px(3.0))
                                    .cursor_pointer()
                                    .hover(|s| s.bg(theme.bg_hover))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _event, _window, cx| {
                                            this.state.logcat.clear();
                                            cx.notify();
                                        }),
                                    )
                                    .child(icon_14(IconName::Trash, theme.text_muted)),
                            )
                            .child(
                                div()
                                    .p_1()
                                    .rounded(px(3.0))
                                    .cursor_pointer()
                                    .hover(|s| s.bg(theme.bg_hover))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _event, _window, cx| {
                                            this.state.logcat.refresh_devices();
                                            this.state
                                                .toast_manager
                                                .push_info("Refreshed adb devices");
                                            cx.notify();
                                        }),
                                    )
                                    .child(icon_14(IconName::RefreshCw, theme.text_muted)),
                            ),
                    ),
            )
            .child(
                div()
                    .id("logcat_scroll")
                    .flex()
                    .flex_col()
                    .flex_1()
                    .overflow_y_scroll()
                    .p_2()
                    .gap_0p5()
                    .children(
                        logcat
                            .entries
                            .iter()
                            .filter(|e| filter_matches(&logcat.filter, e))
                            .map(|entry| {
                                let (color, opacity) = match entry.level.as_deref() {
                                    Some("E") => (theme.status_red, 1.0),
                                    Some("W") => (theme.status_yellow, 1.0),
                                    _ => (theme.text_primary, 0.85),
                                };
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_start()
                                    .text_xs()
                                    .font_family("Consolas")
                                    .text_color(color)
                                    .opacity(opacity)
                                    .child(div().flex_1().child(entry.raw.clone()))
                                    .into_any_element()
                            }),
                    ),
            )
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn filter_level_and_substring() {
        let err = parse_logcat_line("03-15 10:12:01.234  1  2 E MyTag: boom");
        let info = parse_logcat_line("03-15 10:12:01.234  1  2 I MyTag: hello world");
        assert!(filter_matches("E", &err));
        assert!(!filter_matches("E", &info));
        assert!(filter_matches("hello", &info));
        assert!(!filter_matches("hello", &err));
        assert!(filter_matches("", &info));
    }
}
