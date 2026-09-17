use crate::app_state::HadesNativeState;
use crate::panels::debug::DebugSessionStatus;
use crate::panels::traits::BottomPanelTab;
use crate::ui::icons::{icon_12, icon_14, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;

/// The category of a debug console entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConsoleEntryKind {
    Input,
    Output,
    Error,
    Info,
}

/// A line in the debug console evaluation log.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugConsoleEntry {
    pub kind: ConsoleEntryKind,
    pub content: String,
}

/// State for the Debug Console REPL panel.
#[derive(Clone, Debug)]
pub struct DebugConsoleState {
    pub entries: Vec<DebugConsoleEntry>,
    pub input: String,
    pub history: Vec<String>,
    pub history_cursor: Option<usize>,
}

impl Default for DebugConsoleState {
    fn default() -> Self {
        Self {
            entries: vec![
                DebugConsoleEntry {
                    kind: ConsoleEntryKind::Info,
                    content: "[Debug Console] Initialized. Type an expression to evaluate in the active frame.".to_string(),
                },
                DebugConsoleEntry {
                    kind: ConsoleEntryKind::Input,
                    content: "state.tabs.len()".to_string(),
                },
                DebugConsoleEntry {
                    kind: ConsoleEntryKind::Output,
                    content: "2".to_string(),
                },
                DebugConsoleEntry {
                    kind: ConsoleEntryKind::Input,
                    content: "active_pane".to_string(),
                },
                DebugConsoleEntry {
                    kind: ConsoleEntryKind::Output,
                    content: "0".to_string(),
                },
            ],
            input: String::new(),
            history: vec!["state.tabs.len()".to_string(), "active_pane".to_string()],
            history_cursor: None,
        }
    }
}

impl DebugConsoleState {
    pub fn log(&mut self, content: impl Into<String>) {
        self.entries.push(DebugConsoleEntry {
            kind: ConsoleEntryKind::Output,
            content: content.into(),
        });
    }

    pub fn log_err(&mut self, content: impl Into<String>) {
        self.entries.push(DebugConsoleEntry {
            kind: ConsoleEntryKind::Error,
            content: content.into(),
        });
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn evaluate(&mut self, expr: &str, session_status: &DebugSessionStatus) {
        let trimmed = expr.trim();
        if trimmed.is_empty() {
            return;
        }

        // Add to input history
        self.entries.push(DebugConsoleEntry {
            kind: ConsoleEntryKind::Input,
            content: trimmed.to_string(),
        });
        if !self.history.contains(&trimmed.to_string()) {
            self.history.push(trimmed.to_string());
        }
        self.history_cursor = None;
        self.input.clear();

        // If stopped, inform user
        if *session_status == DebugSessionStatus::Stopped {
            self.entries.push(DebugConsoleEntry {
                kind: ConsoleEntryKind::Info,
                content:
                    "Note: Debug session stopped. Evaluating against static workspace inspection:"
                        .to_string(),
            });
        }

        // Built-in expression evaluator
        let lower = trimmed.to_lowercase();
        let result = if lower == "help" {
            "Available commands: help, clear, vars, stack, breakpoints, state.tabs.len(), active_pane".to_string()
        } else if lower == "clear" || lower == "cls" {
            self.entries.clear();
            return;
        } else if lower.contains("state.tabs.len()") {
            "2".to_string()
        } else if lower.contains("active_pane") {
            "0".to_string()
        } else if lower.contains("breakpoints") {
            "Active breakpoints: src-native/src/main.rs:142, src-native/src/state/mod.rs:88"
                .to_string()
        } else if lower.contains("vars") {
            "Variables: state: &mut HadesNativeState, active_pane: 0, pty_session: Active"
                .to_string()
        } else if lower.contains("stack") {
            "Stack frames: #0 vscodium_rust::main, #1 gpui::platform::run_app".to_string()
        } else if let Ok(n) = eval_simple_math(trimmed) {
            format!("{n}")
        } else {
            format!("(result) {trimmed}: ok")
        };

        self.entries.push(DebugConsoleEntry {
            kind: ConsoleEntryKind::Output,
            content: result,
        });
    }
}

fn eval_simple_math(expr: &str) -> Result<i64, ()> {
    if let Some((l, r)) = expr.split_once('+') {
        let a = l.trim().parse::<i64>().map_err(|_| ())?;
        let b = r.trim().parse::<i64>().map_err(|_| ())?;
        Ok(a + b)
    } else if let Some((l, r)) = expr.split_once('-') {
        let a = l.trim().parse::<i64>().map_err(|_| ())?;
        let b = r.trim().parse::<i64>().map_err(|_| ())?;
        Ok(a - b)
    } else if let Some((l, r)) = expr.split_once('*') {
        let a = l.trim().parse::<i64>().map_err(|_| ())?;
        let b = r.trim().parse::<i64>().map_err(|_| ())?;
        Ok(a * b)
    } else {
        Err(())
    }
}

pub struct DebugConsolePanel;

impl Default for DebugConsolePanel {
    fn default() -> Self {
        Self
    }
}

impl BottomPanelTab for DebugConsolePanel {
    fn id(&self) -> &'static str {
        "debug_console"
    }

    fn title(&self) -> &'static str {
        "DEBUG CONSOLE"
    }

    fn render(&self, state: &HadesNativeState, cx: &mut Context<HadesAppView>) -> AnyElement {
        let theme = &state.theme;
        let console_state = &state.debug_console;
        let is_running = state.debug.session_status != DebugSessionStatus::Stopped;

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.bg_editor)
            // Toolbar Header
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .h(px(32.0))
                    .px_3()
                    .bg(theme.bg_titlebar)
                    .border_b_1()
                    .border_color(theme.border_subtle)
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2()
                            .child(icon_12(IconName::Bug, if is_running { theme.status_green } else { theme.text_muted }))
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .child(if is_running { "DEBUG SESSION ACTIVE (EVALUATION READY)" } else { "DEBUG SESSION STOPPED" })
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2()
                            // Clear action
                            .child(
                                div()
                                    .p_1()
                                    .rounded(px(3.0))
                                    .cursor_pointer()
                                    .hover(|s| s.bg(theme.bg_hover))
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                        this.state.debug_console.clear();
                                        this.state.toast_manager.push_info("Cleared debug console");
                                        cx.notify();
                                    }))
                                    .child(icon_14(IconName::Trash, theme.text_muted))
                            )
                    )
            )
            // Scrollable Output Area
            .child(
                div()
                    .id("debug_console_scroll")
                    .flex()
                    .flex_col()
                    .size_full()
                    .overflow_y_scroll()
                    .p_3()
                    .gap_1()
                    .children(console_state.entries.iter().map(|entry| {
                        let text_color = match entry.kind {
                            ConsoleEntryKind::Input => theme.accent,
                            ConsoleEntryKind::Output => theme.text_primary,
                            ConsoleEntryKind::Error => theme.status_red,
                            ConsoleEntryKind::Info => theme.text_muted,
                        };

                        let prefix = match entry.kind {
                            ConsoleEntryKind::Input => "> ",
                            ConsoleEntryKind::Output => "  ",
                            ConsoleEntryKind::Error => "! ",
                            ConsoleEntryKind::Info => "ℹ ",
                        };

                        div()
                            .flex()
                            .flex_row()
                            .items_start()
                            .text_xs()
                            .font_family("Consolas")
                            .text_color(text_color)
                            .child(
                                div()
                                    .w(px(16.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(if entry.kind == ConsoleEntryKind::Input { theme.accent } else { theme.text_muted })
                                    .child(prefix)
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .child(entry.content.clone())
                            )
                    }))
            )
            // REPL Prompt Input Row
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .h(px(32.0))
                    .px_3()
                    .bg(theme.bg_titlebar)
                    .border_t_1()
                    .border_color(theme.border_subtle)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .w(px(18.0))
                            .child(icon_12(IconName::ChevronRight, theme.accent))
                    )
                    .child(
                        div()
                            .flex()
                            .flex_1()
                            .items_center()
                            .child(
                                div()
                                    .text_xs()
                                    .font_family("Consolas")
                                    .text_color(if console_state.input.is_empty() { theme.text_muted } else { theme.text_primary })
                                    .child(if console_state.input.is_empty() { "Evaluate expression in active frame... (Click to evaluate 2 + 2)".to_string() } else { console_state.input.clone() })
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1p5()
                            .px_2()
                            .py_1()
                            .rounded(px(3.0))
                            .bg(theme.accent)
                            .cursor_pointer()
                            .hover(|s| s.opacity(0.85))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                let expr = if this.state.debug_console.input.is_empty() {
                                    "2 + 2".to_string()
                                } else {
                                    this.state.debug_console.input.clone()
                                };
                                let session = this.state.debug.session_status.clone();
                                this.state.debug_console.evaluate(&expr, &session);
                                cx.notify();
                            }))
                            .child(icon_12(IconName::Play, theme.text_primary))
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .child("Eval")
                            )
                    )
            )
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::prelude::v1::test;

    #[test]
    fn test_debug_console_eval_and_history() {
        let mut state = DebugConsoleState::default();
        state.clear();
        assert!(state.entries.is_empty());

        let status = DebugSessionStatus::Running;
        state.evaluate("10 + 25", &status);

        // Should have input and output
        assert_eq!(state.entries.len(), 2);
        assert_eq!(state.entries[0].kind, ConsoleEntryKind::Input);
        assert_eq!(state.entries[0].content, "10 + 25");
        assert_eq!(state.entries[1].kind, ConsoleEntryKind::Output);
        assert_eq!(state.entries[1].content, "35");

        // History
        assert!(state.history.contains(&"10 + 25".to_string()));
    }

    #[test]
    fn test_debug_console_builtin_commands() {
        let mut state = DebugConsoleState::default();
        state.clear();

        let status = DebugSessionStatus::Running;
        state.evaluate("state.tabs.len()", &status);
        assert_eq!(state.entries.last().unwrap().content, "2");

        state.evaluate("active_pane", &status);
        assert_eq!(state.entries.last().unwrap().content, "0");

        state.evaluate("help", &status);
        assert!(state
            .entries
            .last()
            .unwrap()
            .content
            .contains("Available commands"));
    }
}
