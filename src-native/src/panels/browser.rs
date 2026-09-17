use crate::app_state::HadesNativeState;
use crate::panels::traits::WorkbenchPanel;
use crate::ui::icons::{icon_12, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use vscode_rust_app::browser::BrowserState as BackendBrowserState;

#[derive(Clone)]
pub struct BrowserPanelState {
    pub url: String,
    pub mode: String,
    pub headless: bool,
    pub external_active: bool,
    pub status: Arc<Mutex<String>>,
    pub navigating: Arc<AtomicBool>,
    pub backend: Arc<BackendBrowserState>,
}

impl Default for BrowserPanelState {
    fn default() -> Self {
        Self {
            url: String::new(),
            mode: "vision".to_string(),
            headless: false,
            external_active: false,
            status: Arc::new(Mutex::new(String::new())),
            navigating: Arc::new(AtomicBool::new(false)),
            backend: Arc::new(BackendBrowserState::new()),
        }
    }
}

impl BrowserPanelState {
    pub fn status_text(&self) -> String {
        self.status.lock().map(|s| s.clone()).unwrap_or_default()
    }

    pub fn is_navigating(&self) -> bool {
        self.navigating.load(Ordering::SeqCst)
    }

    pub fn navigate(&self, url: &str) {
        let normalized = normalize_url(url);
        if normalized.is_empty() || self.is_navigating() {
            return;
        }
        if let Ok(mut s) = self.status.lock() {
            *s = format!("Navigating to {normalized}…");
        }
        self.navigating.store(true, Ordering::SeqCst);
        let backend = self.backend.clone();
        let status = self.status.clone();
        let navigating = self.navigating.clone();
        std::thread::Builder::new()
            .name("browser-navigate".to_string())
            .spawn(move || {
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build();
                let message = match rt {
                    Ok(rt) => {
                        let result = rt.block_on(async {
                            backend
                                .cmd(
                                    "navigate",
                                    serde_json::json!({ "url": normalized.clone() }),
                                    60,
                                )
                                .await
                        });
                        match result {
                            Ok(r) => {
                                let http = r
                                    .get("status")
                                    .map(|s| s.to_string())
                                    .unwrap_or_else(|| "?".to_string());
                                format!("Navigated to {normalized} (HTTP {http})")
                            }
                            Err(e) => format!(
                                "Navigate failed: {e}. Dev: pip install playwright invisible_playwright | Release: browser-agent.exe in binaries/"
                            ),
                        }
                    }
                    Err(e) => format!("Runtime unavailable: {e}"),
                };
                if let Ok(mut s) = status.lock() {
                    *s = message;
                }
                navigating.store(false, Ordering::SeqCst);
            })
            .ok();
    }

    pub fn open_external(&mut self) {
        self.external_active = true;
        if let Ok(mut s) = self.status.lock() {
            *s = "Launching external browser… first run may download Firefox.".to_string();
        }
        let backend = self.backend.clone();
        let headless = self.headless;
        let status = self.status.clone();
        std::thread::Builder::new()
            .name("browser-open".to_string())
            .spawn(move || {
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build();
                let message = match rt {
                    Ok(rt) => {
                        let result = rt.block_on(async { backend.ensure_started_with(headless).await });
                        match result {
                            Ok(()) => format!(
                                "Browser launching ({}).",
                                if headless {
                                    "hidden stealth desktop"
                                } else {
                                    "visible window"
                                }
                            ),
                            Err(e) => format!(
                                "{e}. Release: browser-agent.exe should ship in binaries/ · Dev: pip install playwright invisible_playwright"
                            ),
                        }
                    }
                    Err(e) => format!("Runtime unavailable: {e}"),
                };
                if let Ok(mut s) = status.lock() {
                    *s = message;
                }
            })
            .ok();
    }

    pub fn close_external(&mut self) {
        self.external_active = false;
        let backend = self.backend.clone();
        let status = self.status.clone();
        std::thread::Builder::new()
            .name("browser-close".to_string())
            .spawn(move || {
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build();
                let message = match rt {
                    Ok(rt) => {
                        let _ = rt.block_on(async { backend.stop_sidecar().await });
                        "Browser closed".to_string()
                    }
                    Err(e) => format!("Runtime unavailable: {e}"),
                };
                if let Ok(mut s) = status.lock() {
                    *s = message;
                }
            })
            .ok();
    }

    pub fn set_mode(&mut self, mode: &str) {
        self.mode = mode.to_string();
    }
}

pub fn normalize_url(url: &str) -> String {
    let trimmed = url.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        trimmed.to_string()
    } else if trimmed.starts_with("localhost") || trimmed.starts_with("127.0.0.1") {
        format!("http://{trimmed}")
    } else {
        format!("https://{trimmed}")
    }
}

pub struct BrowserPanel;

impl WorkbenchPanel for BrowserPanel {
    fn id(&self) -> &'static str {
        "browser"
    }

    fn title(&self) -> &'static str {
        "External Browser"
    }

    fn icon(&self) -> IconName {
        IconName::Globe
    }

    fn render(&self, state: &HadesNativeState, cx: &mut Context<HadesAppView>) -> AnyElement {
        render_browser_panel(state, cx).into_any_element()
    }
}

const QUICK_PORTS: [(&str, u16); 5] = [
    ("Vite", 5174),
    ("Astro", 4321),
    ("Next / CRA", 3000),
    ("Dev", 8080),
    ("Live Server", 5500),
];

pub fn render_browser_panel(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> AnyElement {
    let theme = &state.theme;
    let browser = &state.browser;
    let status_text = browser.status_text();
    let is_navigating = browser.is_navigating();

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
                .h(px(48.0))
                .px_2p5()
                .bg(theme.bg_titlebar)
                .border_b_1()
                .border_color(theme.border_subtle)
                .child(icon_12(IconName::Globe, theme.text_muted))
                .child(
                    div()
                        .flex_1()
                        .px_2()
                        .py_1()
                        .rounded(px(4.0))
                        .bg(theme.bg_input)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .text_xs()
                        .truncate()
                        .text_color(if browser.url.is_empty() {
                            theme.text_muted
                        } else {
                            theme.text_primary
                        })
                        .child(if browser.url.is_empty() {
                            "Enter a URL and press Navigate — the external stealth Firefox drives the page".to_string()
                        } else {
                            browser.url.clone()
                        }),
                )
                .child(if browser.mode == "live" {
                    div()
                        .px_2()
                        .py_0p5()
                        .rounded(px(4.0))
                        .bg(theme.bg_raised)
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.text_primary)
                        .cursor_pointer()
                        .hover(|s| s.bg(theme.bg_hover))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                this.state.browser.set_mode("vision");
                                cx.notify();
                            }),
                        )
                        .child("Vision")
                        .into_any_element()
                } else {
                    div()
                        .px_2()
                        .py_0p5()
                        .rounded(px(4.0))
                        .bg(theme.accent)
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.text_on_accent)
                        .cursor_pointer()
                        .hover(|s| s.opacity(0.85))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                this.state.browser.set_mode("live");
                                cx.notify();
                            }),
                        )
                        .child("Vision")
                        .into_any_element()
                })
                .child(if browser.external_active {
                    div()
                        .px_2p5()
                        .py_1()
                        .rounded(px(4.0))
                        .bg(theme.bg_raised)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .cursor_pointer()
                        .hover(|s| s.bg(theme.bg_hover))
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.text_primary)
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                this.state.browser.close_external();
                                this.state.toast_manager.push_info("Closing external browser");
                                cx.notify();
                            }),
                        )
                        .child("Close External")
                        .into_any_element()
                } else {
                    div()
                        .px_2p5()
                        .py_1()
                        .rounded(px(4.0))
                        .bg(theme.accent)
                        .cursor_pointer()
                        .hover(|s| s.opacity(0.85))
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.text_on_accent)
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                this.state.browser.open_external();
                                cx.notify();
                            }),
                        )
                        .child("Open External")
                        .into_any_element()
                }),
        )
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap_2()
                .h(px(32.0))
                .px_2p5()
                .bg(theme.bg_titlebar)
                .border_b_1()
                .border_color(theme.border_subtle)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1p5()
                        .px_2()
                        .py_0p5()
                        .rounded(px(4.0))
                        .bg(theme.accent)
                        .cursor_pointer()
                        .hover(|s| s.opacity(0.85))
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.text_on_accent)
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                let url = this.state.browser.url.clone();
                                this.state.browser.navigate(&url);
                                cx.notify();
                            }),
                        )
                        .child("Navigate"),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1p5()
                        .children(QUICK_PORTS.iter().map(|(label, port)| {
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded(px(4.0))
                                .bg(theme.bg_raised)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .text_xs()
                                .text_color(theme.text_primary)
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener({
                                        let port = *port;
                                        move |this, _event, _window, cx| {
                                            let url = format!("http://localhost:{port}");
                                            this.state.browser.url = url.clone();
                                            this.state.browser.navigate(&url);
                                            cx.notify();
                                        }
                                    }),
                                )
                                .child(format!("{label} :{port}"))
                                .into_any_element()
                        })),
                ),
        )
        .child(
            div()
                .id("browser_body_scroll")
                .flex()
                .flex_col()
                .flex_1()
                .overflow_y_scroll()
                .p_3()
                .gap_2p5()
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .px_2p5()
                        .py_1p5()
                        .rounded(px(6.0))
                        .bg(theme.bg_card)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .text_xs()
                        .text_color(if is_navigating {
                            theme.status_yellow
                        } else if status_text.is_empty() {
                            theme.text_muted
                        } else {
                            theme.text_primary
                        })
                        .child(if is_navigating {
                            "Working…".to_string()
                        } else if status_text.is_empty() {
                            "Idle. Open the external browser or navigate to a URL to begin.".to_string()
                        } else {
                            status_text
                        }),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .p_2p5()
                        .rounded(px(6.0))
                        .bg(theme.bg_card)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .gap_1p5()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child("Stealth Firefox (external OS window)"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child(
                                    "Not the in-IDE preview: a real stealth Firefox the agent drives (open, navigate, click, type, screenshot). Hidden mode runs an invisible desktop — mirror it through the Vision panel.",
                                ),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .p_2p5()
                        .rounded(px(6.0))
                        .bg(theme.bg_card)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .gap_1p5()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child("Browser JavaScript execution"),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_2()
                                .children(["Visible window", "Hidden stealth"].iter().map(|m| {
                                    let selected = (browser.headless && *m == "Hidden stealth")
                                        || (!browser.headless && *m == "Visible window");
                                    div()
                                        .px_2()
                                        .py_0p5()
                                        .rounded(px(4.0))
                                        .cursor_pointer()
                                        .bg(if selected {
                                            theme.bg_raised
                                        } else {
                                            theme.bg_card
                                        })
                                        .border_1()
                                        .border_color(theme.border_subtle)
                                        .text_xs()
                                        .text_color(if selected {
                                            theme.text_primary
                                        } else {
                                            theme.text_muted
                                        })
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener({
                                                let mode = m.to_string();
                                                move |this, _event, _window, cx| {
                                                    this.state.browser.headless =
                                                        mode == "Hidden stealth";
                                                    cx.notify();
                                                }
                                            }),
                                        )
                                        .child(m.to_string())
                                        .into_any_element()
                                })),
                        ),
                ),
        )
        .into_any_element()
}
