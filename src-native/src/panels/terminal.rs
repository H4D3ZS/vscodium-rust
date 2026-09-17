use crate::app_state::{FocusedPanel, HadesNativeState};
use crate::ui::icons::{icon_12, icon_14, ui_icon, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;

pub fn render_terminal_panel(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;
    let is_focused = state.focused_panel == FocusedPanel::Terminal;

    let active_sess_title = state
        .terminal_sessions
        .get(state.active_terminal_idx)
        .map(|s| s.title.clone())
        .unwrap_or_else(|| "1: pwsh".to_string());

    let cwd_display = state
        .workspace_root
        .file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_else(|| "vscodium-rust".to_string());

    div()
        .flex()
        .flex_col()
        .size_full()
        .bg(theme.bg_sidebar)
        .cursor_pointer()
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(|this, _event, _window, cx| {
                this.state.focused_panel = FocusedPanel::Terminal;
                cx.notify();
            }),
        )
        // Terminal Subheader Toolbar (VS Code Terminal Parity)
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .h(px(28.0))
                .px_2()
                .bg(theme.bg_editor)
                .border_b_1()
                .border_color(theme.border_subtle)
                // Left: Active Terminal Session Indicator & CWD
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .child(ui_icon(IconName::Terminal, 12.0, theme.accent))
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1p5()
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(theme.text_primary)
                                        .child(active_sess_title),
                                )
                                .child(div().w(px(6.0)).h(px(6.0)).rounded_full().bg(
                                    if state.pty.is_some() {
                                        theme.status_green
                                    } else {
                                        theme.text_muted
                                    },
                                )),
                        )
                        .child(
                            div()
                                .px_1p5()
                                .py_0p5()
                                .rounded(px(3.0))
                                .bg(theme.bg_hover)
                                .text_xs()
                                .text_color(theme.text_subtle)
                                .child(cwd_display),
                        ),
                )
                // Right controls: PTY Multi-Session pills, Add Session (+), Shell Picker, Split Terminal, Kill Session, Clear
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1p5()
                        // Multi-session tabs
                        .children(
                            state
                                .terminal_sessions
                                .iter()
                                .enumerate()
                                .map(|(idx, sess)| {
                                    let is_current = idx == state.active_terminal_idx;
                                    let title = sess.title.clone();
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_1()
                                        .px_2()
                                        .py_0p5()
                                        .rounded(px(3.0))
                                        .cursor_pointer()
                                        .bg(if is_current {
                                            theme.bg_raised
                                        } else {
                                            theme.bg_titlebar
                                        })
                                        .border_1()
                                        .border_color(if is_current {
                                            theme.accent
                                        } else {
                                            theme.border_subtle
                                        })
                                        .hover(|s| s.bg(theme.bg_hover))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _event, _window, cx| {
                                                this.state.select_terminal(idx);
                                                cx.notify();
                                            }),
                                        )
                                        .child(ui_icon(
                                            IconName::Terminal,
                                            10.0,
                                            if is_current {
                                                theme.status_green
                                            } else {
                                                theme.text_muted
                                            },
                                        ))
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(if is_current {
                                                    FontWeight::MEDIUM
                                                } else {
                                                    FontWeight::NORMAL
                                                })
                                                .text_color(if is_current {
                                                    theme.text_primary
                                                } else {
                                                    theme.text_muted
                                                })
                                                .child(title),
                                        )
                                }),
                        )
                        // Add New Session (+)
                        .child(
                            div()
                                .p_1()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.state.spawn_terminal(None);
                                        cx.notify();
                                    }),
                                )
                                .child(icon_12(IconName::Plus, theme.text_subtle)),
                        )
                        // Shell picker dropdown
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .px_1p5()
                                .py_0p5()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .text_xs()
                                .text_color(theme.text_muted)
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        let next_shell =
                                            if cfg!(windows) { "cmd.exe" } else { "bash" };
                                        this.state.spawn_terminal(Some(next_shell));
                                        this.state
                                            .toast_manager
                                            .push_info("Spawned new shell session");
                                        cx.notify();
                                    }),
                                )
                                .child(ui_icon(IconName::Terminal, 10.0, theme.accent))
                                .child("Shell")
                                .child(icon_12(IconName::ChevronDown, theme.text_subtle)),
                        )
                        // Split Terminal Action
                        .child(
                            div()
                                .p_1()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.state.spawn_terminal(None);
                                        this.state
                                            .toast_manager
                                            .push_info("Split terminal created");
                                        cx.notify();
                                    }),
                                )
                                .child(icon_12(IconName::PanelRight, theme.text_subtle)),
                        )
                        // Clear Action
                        .child(
                            div()
                                .p_1()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.state.clear_terminal();
                                        cx.notify();
                                    }),
                                )
                                .child(icon_14(IconName::Trash, theme.text_subtle)),
                        )
                        // Kill Session Action
                        .child(
                            div()
                                .p_1()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.state.kill_active_terminal();
                                        cx.notify();
                                    }),
                                )
                                .child(icon_14(IconName::X, theme.text_subtle)),
                        ),
                ),
        )
        // Terminal Content (Output stream + input prompt)
        .child(render_terminal_content(state, theme, is_focused, cx))
}

fn render_terminal_content(
    state: &HadesNativeState,
    theme: &crate::theme::Theme,
    is_focused: bool,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .flex_1()
        .overflow_hidden()
        // Terminal Output Stream
        .child(
            div()
                .id("terminal_output_scroll")
                .flex()
                .flex_col()
                .flex_1()
                .overflow_y_scroll()
                .p_3()
                .gap_0p5()
                .font_family("Cascadia Code, Consolas, monospace")
                .text_xs()
                .cursor_text()
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _event, _window, cx| {
                        this.state.focused_panel = crate::app_state::FocusedPanel::Terminal;
                        cx.notify();
                    }),
                )
                .on_mouse_down(
                    MouseButton::Right,
                    cx.listener(|this, event: &MouseDownEvent, _window, cx| {
                        let x: f32 = event.position.x.into();
                        let y: f32 = event.position.y.into();
                        this.state.context_menu.show(
                            x,
                            y,
                            crate::ui::context_menu::terminal_context_menu(),
                        );
                        cx.notify();
                    }),
                )
                .children(state.terminal_lines.iter().map(|line| {
                    let segments = crate::terminal_ansi::parse_ansi_line(line);
                    if segments.is_empty() {
                        div().h(px(14.0)).into_any_element()
                    } else {
                        div()
                            .flex()
                            .flex_row()
                            .flex_wrap()
                            .children(segments.into_iter().map(|seg| {
                                let mut el = div().child(seg.text);
                                if let Some(fg) = seg.style.fg {
                                    el = el.text_color(fg);
                                } else {
                                    el = el.text_color(theme.text_primary);
                                }
                                if let Some(bg) = seg.style.bg {
                                    el = el.bg(bg);
                                }
                                if seg.style.bold {
                                    el = el.font_weight(FontWeight::BOLD);
                                }
                                el
                            }))
                            .into_any_element()
                    }
                })),
        )
        // Terminal Input Prompt
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .h(px(28.0))
                .px_3()
                .bg(theme.bg_sidebar)
                .border_t_1()
                .border_color(theme.border_subtle)
                .font_family("Cascadia Code, Consolas, monospace")
                .text_xs()
                .cursor_text()
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _event, _window, cx| {
                        this.state.focused_panel = crate::app_state::FocusedPanel::Terminal;
                        cx.notify();
                    }),
                )
                .child(
                    div()
                        .text_color(theme.accent)
                        .font_weight(FontWeight::BOLD)
                        .child(if state.terminal_active_prompt.is_empty() {
                            format!("PS {}> ", state.workspace_root.display())
                        } else {
                            state.terminal_active_prompt.clone()
                        }),
                )
                .child(
                    div()
                        .text_color(theme.text_primary)
                        .child(state.terminal_input.clone()),
                )
                .children(is_focused.then(|| div().w(px(7.0)).h(px(14.0)).bg(theme.accent))),
        )
}

pub struct TerminalPanel;

impl crate::panels::traits::BottomPanelTab for TerminalPanel {
    fn id(&self) -> &'static str {
        "terminal"
    }

    fn title(&self) -> &'static str {
        "TERMINAL"
    }

    fn render(&self, state: &HadesNativeState, cx: &mut Context<HadesAppView>) -> AnyElement {
        render_terminal_panel(state, cx).into_any_element()
    }
}

impl crate::panels::traits::WorkbenchPanel for TerminalPanel {
    fn id(&self) -> &'static str {
        "workbench.view.terminal"
    }

    fn title(&self) -> &'static str {
        "TERMINAL"
    }

    fn icon(&self) -> IconName {
        IconName::SquareTerminal
    }

    fn render(&self, state: &HadesNativeState, cx: &mut Context<HadesAppView>) -> AnyElement {
        render_terminal_panel(state, cx).into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::prelude::v1::test;

    #[test]
    fn test_terminal_panel_metadata() {
        let panel = TerminalPanel;
        assert_eq!(
            crate::panels::traits::BottomPanelTab::id(&panel),
            "terminal"
        );
        assert_eq!(
            crate::panels::traits::BottomPanelTab::title(&panel),
            "TERMINAL"
        );
        assert_eq!(
            crate::panels::traits::WorkbenchPanel::id(&panel),
            "workbench.view.terminal"
        );
        assert_eq!(
            crate::panels::traits::WorkbenchPanel::title(&panel),
            "TERMINAL"
        );
        assert_eq!(
            crate::panels::traits::WorkbenchPanel::icon(&panel),
            IconName::SquareTerminal
        );
    }
}
