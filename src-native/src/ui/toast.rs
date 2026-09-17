// Toast Notification System for Native GPUI.
//
// Floating bottom-right notification cards with auto-dismiss.
// Supports info, warning, error, and success severity levels.
// Used by file save, git commit, agent actions, build results.

use crate::theme::Theme;
use crate::ui::icons::{icon_14, IconName};
use gpui_kit::gpui::*;

/// Severity level of a toast notification.
#[derive(Clone, Debug, PartialEq)]
pub enum ToastSeverity {
    Info,
    Success,
    Warning,
    Error,
}

/// A single toast notification.
#[derive(Clone, Debug)]
pub struct Toast {
    pub id: u64,
    pub message: String,
    pub severity: ToastSeverity,
    pub created_at: std::time::Instant,
    pub duration_ms: u64,
}

impl Toast {
    pub fn info(message: &str) -> Self {
        Self {
            id: rand_id(),
            message: message.to_string(),
            severity: ToastSeverity::Info,
            created_at: std::time::Instant::now(),
            duration_ms: 4000,
        }
    }

    pub fn success(message: &str) -> Self {
        Self {
            id: rand_id(),
            message: message.to_string(),
            severity: ToastSeverity::Success,
            created_at: std::time::Instant::now(),
            duration_ms: 3000,
        }
    }

    pub fn warning(message: &str) -> Self {
        Self {
            id: rand_id(),
            message: message.to_string(),
            severity: ToastSeverity::Warning,
            created_at: std::time::Instant::now(),
            duration_ms: 5000,
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            id: rand_id(),
            message: message.to_string(),
            severity: ToastSeverity::Error,
            created_at: std::time::Instant::now(),
            duration_ms: 6000,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed().as_millis() as u64 >= self.duration_ms
    }
}

fn rand_id() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64
}

/// Toast manager state.
#[derive(Clone, Debug, Default)]
pub struct ToastManager {
    pub toasts: Vec<Toast>,
}

impl ToastManager {
    pub fn push(&mut self, toast: Toast) {
        // Limit to 5 visible toasts max
        if self.toasts.len() >= 5 {
            self.toasts.remove(0);
        }
        self.toasts.push(toast);
    }

    pub fn push_info(&mut self, msg: &str) {
        self.push(Toast::info(msg));
    }

    pub fn push_success(&mut self, msg: &str) {
        self.push(Toast::success(msg));
    }

    pub fn push_warning(&mut self, msg: &str) {
        self.push(Toast::warning(msg));
    }

    pub fn push_error(&mut self, msg: &str) {
        self.push(Toast::error(msg));
    }

    /// Remove expired toasts. Call this in the render loop.
    pub fn gc(&mut self) {
        self.toasts.retain(|t| !t.is_expired());
    }

    pub fn dismiss(&mut self, id: u64) {
        self.toasts.retain(|t| t.id != id);
    }

    pub fn clear(&mut self) {
        self.toasts.clear();
    }
}

/// Render the toast notification overlay (positioned absolute bottom-right).
pub fn render_toast_overlay(manager: &ToastManager, theme: &Theme) -> Option<impl IntoElement> {
    if manager.toasts.is_empty() {
        return None;
    }

    Some(
        div()
            .absolute()
            .bottom(px(44.0)) // Above status bar
            .right(px(16.0))
            .flex()
            .flex_col()
            .gap_2()
            .w(px(340.0))
            .children(manager.toasts.iter().rev().map(|toast| {
                let (icon, icon_color, border_color) = match toast.severity {
                    ToastSeverity::Info => (IconName::Info, theme.accent, theme.accent),
                    ToastSeverity::Success => {
                        (IconName::Check, theme.status_green, theme.status_green)
                    }
                    ToastSeverity::Warning => (
                        IconName::TriangleAlert,
                        theme.status_yellow,
                        theme.status_yellow,
                    ),
                    ToastSeverity::Error => (IconName::X, theme.status_red, theme.status_red),
                };

                // Calculate opacity for fade-out effect (last 500ms)
                let elapsed = toast.created_at.elapsed().as_millis() as f32;
                let remaining = toast.duration_ms as f32 - elapsed;
                let opacity_val = if remaining < 500.0 {
                    (remaining / 500.0).clamp(0.0, 1.0)
                } else {
                    1.0
                };

                div()
                    .flex()
                    .flex_row()
                    .items_start()
                    .gap_2()
                    .p_3()
                    .rounded(px(8.0))
                    .bg(theme.bg_raised)
                    .border_1()
                    .border_color(border_color)
                    .opacity(opacity_val)
                    // Icon
                    .child(div().mt_0p5().child(icon_14(icon, icon_color)))
                    // Message
                    .child(
                        div()
                            .flex_1()
                            .text_xs()
                            .text_color(theme.text_primary)
                            .child(toast.message.clone()),
                    )
                    // Close X
                    .child(
                        div()
                            .p_0p5()
                            .rounded(px(3.0))
                            .cursor_pointer()
                            .hover(|s| s.bg(theme.bg_hover))
                            .child(icon_14(IconName::X, theme.text_subtle)),
                    )
            })),
    )
}
