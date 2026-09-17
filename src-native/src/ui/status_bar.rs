use crate::app_state::HadesNativeState;
use crate::ui::icons::{icon_12, ui_icon, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;

pub fn render_status_bar(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;

    div()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .h(px(22.0))
        .w_full()
        .bg(theme.bg_status_bar)
        .border_t_1()
        .border_color(theme.border_subtle)
        .px_2()
        .text_xs()
        .text_color(theme.text_muted)
        // Left Section
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap_2()
                // Remote Indicator Badge (>< icon on green square)
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .w(px(28.0))
                        .h(px(22.0))
                        .bg(rgb(0x16a34a))
                        .cursor_pointer()
                        .hover(|s| s.bg(rgb(0x15803d)))
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                            this.state.toast_manager.push_info("Remote: Local WSL / Container environment ready");
                            cx.notify();
                        }))
                        .child(
                            div()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xffffff))
                                .child("><")
                        )
                )
                // Workspace Name
                .child(
                    div()
                        .text_color(theme.text_primary)
                        .font_weight(FontWeight::MEDIUM)
                        .child("vscodium-rust")
                )
                // Git Branch
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .cursor_pointer()
                        .px_1p5()
                        .py_0p5()
                        .rounded(px(3.0))
                        .hover(|s| s.bg(theme.bg_hover))
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                            this.state.set_activity(crate::app_state::ActivityTab::SourceControl);
                            this.state.left_sidebar_open = true;
                            cx.notify();
                        }))
                        .child(icon_12(IconName::GitBranch, theme.text_muted))
                        .child(
                            div()
                                .text_color(theme.text_primary)
                                .child("main")
                        )
                )
                // Synchronize Changes (sync icon)
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .cursor_pointer()
                        .px_1p5()
                        .py_0p5()
                        .rounded(px(3.0))
                        .hover(|s| s.bg(theme.bg_hover))
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                            this.state.toast_manager.push_info("No upstream changes to synchronize");
                            cx.notify();
                        }))
                        .child(icon_12(IconName::RefreshCw, theme.text_muted))
                )
                // Cursor AI Agent Status Pill
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1p5()
                        .cursor_pointer()
                        .px_2()
                        .py_0p5()
                        .rounded(px(4.0))
                        .bg(rgba(0x22c55e14))
                        .border_1()
                        .border_color(rgba(0x22c55e30))
                        .hover(|s| s.bg(rgba(0x22c55e24)))
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                            this.state.right_sidebar_open = !this.state.right_sidebar_open;
                            cx.notify();
                        }))
                        .child(
                            div()
                                .w(px(6.0))
                                .h(px(6.0))
                                .rounded_full()
                                .bg(if state.is_agent_thinking { rgb(0x38bdf8) } else { rgb(0x22c55e) })
                        )
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(if state.is_agent_thinking { theme.accent } else { rgb(0x22c55e) })
                                .child(if state.is_agent_thinking { "✦ Agent Thinking..." } else { "✦ AI Agent: Ready" })
                        )
                )
                // Problems / Diagnostics (Clickable: opens bottom panel PROBLEMS tab)
                .child({
                    let err_count = state.active_tab().map(|t| {
                        t.model.diagnostics.iter().filter(|d| matches!(d.severity, crate::editor::engine::decorations::DiagnosticSeverity::Error)).count()
                    }).unwrap_or(0);
                    let warn_count = state.active_tab().map(|t| {
                        t.model.diagnostics.iter().filter(|d| matches!(d.severity, crate::editor::engine::decorations::DiagnosticSeverity::Warning)).count()
                    }).unwrap_or(0);
                    div()
                        .flex()
                        .items_center()
                        .gap_1p5()
                        .cursor_pointer()
                        .px_1p5()
                        .py_0p5()
                        .rounded(px(3.0))
                        .hover(|s| s.bg(theme.bg_hover))
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                            this.state.set_bottom_tab(crate::app_state::BottomPanelTab::Problems);
                            cx.notify();
                        }))
                        .child(ui_icon(IconName::CircleX, 11.0, if err_count > 0 { theme.status_red } else { theme.text_subtle }))
                        .child(div().child(err_count.to_string()))
                        .child(ui_icon(IconName::TriangleAlert, 11.0, if warn_count > 0 { theme.status_yellow } else { theme.text_subtle }))
                        .child(div().child(warn_count.to_string()))
                })
                // LSP Status
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
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                            this.state.toast_manager.push_info("Language Server Protocol: rust-analyzer active");
                            cx.notify();
                        }))
                        .child(ui_icon(IconName::Radio, 10.0, theme.text_subtle))
                        .child(div().child("LSP"))
                )
                // Security / Trust
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .cursor_pointer()
                        .px_1p5()
                        .py_0p5()
                        .rounded(px(3.0))
                        .hover(|s| s.bg(theme.bg_hover))
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                            this.state.workspace_trusted = !this.state.workspace_trusted;
                            let msg = if this.state.workspace_trusted {
                                "Workspace set to Trusted"
                            } else {
                                "Workspace set to Restricted Mode"
                            };
                            this.state.toast_manager.push_info(msg);
                            cx.notify();
                        }))
                        .child(ui_icon(
                            IconName::Shield,
                            10.0,
                            if state.workspace_trusted { theme.status_green } else { theme.status_yellow },
                        ))
                        .child(
                            div()
                                .text_color(if state.workspace_trusted { theme.status_green } else { theme.status_yellow })
                                .child(if state.workspace_trusted { "Trusted" } else { "Restricted" })
                        )
                )
                // BugBot
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .cursor_pointer()
                        .px_1p5()
                        .py_0p5()
                        .rounded(px(3.0))
                        .hover(|s| s.bg(theme.bg_hover).text_color(theme.text_primary))
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                            this.state.send_ai_prompt("/review the current changes");
                            cx.notify();
                        }))
                        .child(ui_icon(IconName::Bot, 10.0, theme.text_subtle))
                        .child(div().child("BugBot"))
                )
                // Indexed Status
                .child({
                    let chunks = state.kortex_chunks_indexed;
                    let indexing = state.kortex_indexing_status.to_lowercase().contains("indexing");
                    let display = if indexing {
                        "Indexing…".to_string()
                    } else if chunks >= 1000 {
                        format!("Indexed {:.1}k", chunks as f32 / 1000.0)
                    } else {
                        format!("Indexed {chunks}")
                    };
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .cursor_pointer()
                        .px_1p5()
                        .py_0p5()
                        .rounded(px(3.0))
                        .hover(|s| s.bg(theme.bg_hover).text_color(theme.text_primary))
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                            this.state.active_right_tab =
                                crate::domain::layout::RightSidebarTab::VectorSearch;
                            this.state.right_sidebar_open = true;
                            cx.notify();
                        }))
                        .child(ui_icon(
                            IconName::Folder,
                            10.0,
                            if indexing { rgb(0x00c6ff) } else { rgb(0x4ade80) },
                        ))
                        .child(
                            div()
                                .text_color(if indexing { rgb(0x00c6ff) } else { theme.text_muted })
                                .child(display),
                        )
                })
                // Model Pill: with Sparkles & live connected status dot (toggles Inference Health Dashboard)
                .child({
                    let active_name = {
                        let raw = state.inference_health.active_model_name();
                        if raw.starts_with("Qwen3.8") || raw.contains("Qwen3.8") {
                            "Qwen 3.8 27B".to_string()
                        } else if raw.starts_with("Qwen3.5") || raw.contains("Qwen3.5") {
                            "Qwen 3.5 4B".to_string()
                        } else if raw.starts_with("Escha") || raw.contains("Escha") {
                            "Escha 35B".to_string()
                        } else if raw.starts_with("Qwen3-Embedding") {
                            "Qwen3 Embed".to_string()
                        } else if raw.starts_with("Qwen 2.5 Coder") || raw.contains("Qwen2.5-Coder") {
                            "Qwen 2.5 Coder".to_string()
                        } else if raw.len() > 24 {
                            format!("{}…", &raw[..22])
                        } else {
                            raw
                        }
                    };
                    let dot_color = match state.inference_health.status {
                        crate::ui::InferenceStatus::Connected => rgb(0x10b981),
                        crate::ui::InferenceStatus::Checking => rgb(0xf59e0b),
                        crate::ui::InferenceStatus::Disconnected => rgb(0xf43f5e),
                    };
                    div()
                        .flex()
                        .items_center()
                        .gap_1p5()
                        .cursor_pointer()
                        .px_1p5()
                        .py_0p5()
                        .rounded(px(3.0))
                        .hover(|s| s.bg(theme.bg_hover))
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                            this.state.toggle_inference_health();
                            cx.notify();
                        }))
                        .child(ui_icon(IconName::Sparkles, 11.0, theme.accent))
                        .child(
                            div()
                                .text_color(theme.text_primary)
                                .child(active_name)
                        )
                        .child(
                            div()
                                .w(px(6.0))
                                .h(px(6.0))
                                .rounded_full()
                                .bg(dot_color)
                        )
                })
        )
        // Right Section
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap_2()
                // Sign In
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .cursor_pointer()
                        .px_1p5()
                        .py_0p5()
                        .rounded(px(3.0))
                        .hover(|s| s.bg(theme.bg_hover).text_color(theme.text_primary))
                        .child(ui_icon(IconName::User, 11.0, theme.text_muted))
                        .child(div().child("Sign in"))
                )
                // Token Budget progress bar (exact TypeScript StatusBar parity)
                .children((!state.chat_messages.is_empty()).then(|| {
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1()
                        .child(ui_icon(IconName::Code, 10.0, theme.text_muted))
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child("22/16k")
                        )
                        .child(
                            div()
                                .w(px(36.0))
                                .h(px(3.0))
                                .rounded(px(2.0))
                                .bg(rgba(0xffffff26))
                                .child(
                                    div()
                                        .w(px(14.0))
                                        .h_full()
                                        .rounded(px(2.0))
                                        .bg(rgb(0x4ade80))
                                )
                        )
                }))
                // Line & Column indicator (interactive: click opens Go to Line)
                .child({
                    let (row, col, sel_info) = state.active_tab().map(|t| {
                        let r = t.cursor_row + 1;
                        let c = t.cursor_col + 1;
                        let sec_len = t.model.cursors.secondary.len();
                        let sel = if sec_len > 0 {
                            format!(" ({} carets)", sec_len + 1)
                        } else if !t.model.cursors.primary.is_empty() {
                            " (selected)".to_string()
                        } else {
                            String::new()
                        };
                        (r, c, sel)
                    }).unwrap_or((1, 1, String::new()));

                    div()
                        .cursor_pointer()
                        .px_1p5()
                        .py_0p5()
                        .rounded(px(3.0))
                        .hover(|s| s.bg(theme.bg_hover).text_color(theme.text_primary))
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                            let cmds = this.commands.clone();
                            this.quick_open.open_line(&this.state, &cmds);
                            cx.notify();
                        }))
                        .child(format!("Ln {row}, Col {col}{sel_info}"))
                })
                // Indentation (interactive: click toggles 2 / 4 spaces)
                .child({
                    let tab_size = state.active_tab().map(|t| t.model.tab_size).unwrap_or(4);
                    div()
                        .cursor_pointer()
                        .px_1p5()
                        .py_0p5()
                        .rounded(px(3.0))
                        .hover(|s| s.bg(theme.bg_hover).text_color(theme.text_primary))
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                            if let Some(tab) = this.state.active_tab_mut() {
                                tab.model.tab_size = if tab.model.tab_size == 4 { 2 } else { 4 };
                                let new_size = tab.model.tab_size;
                                this.state.toast_manager.push_info(&format!("Indentation changed to Spaces: {new_size}"));
                            }
                            cx.notify();
                        }))
                        .child(format!("Spaces: {tab_size}"))
                })
                // File Encoding (interactive: click notifies UTF-8)
                .child(
                    div()
                        .cursor_pointer()
                        .px_1p5()
                        .py_0p5()
                        .rounded(px(3.0))
                        .hover(|s| s.bg(theme.bg_hover).text_color(theme.text_primary))
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                            this.state.toast_manager.push_info("File encoding: UTF-8 (Western/Unicode)");
                            cx.notify();
                        }))
                        .child("UTF-8")
                )
                // Line Endings sequence (interactive: CRLF / LF)
                .child(
                    div()
                        .cursor_pointer()
                        .px_1p5()
                        .py_0p5()
                        .rounded(px(3.0))
                        .hover(|s| s.bg(theme.bg_hover).text_color(theme.text_primary))
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                            this.state.toast_manager.push_info("End of Line Sequence: CRLF");
                            cx.notify();
                        }))
                        .child("CRLF")
                )
                // Language Mode (interactive: click opens Command Palette / language mode)
                .child({
                    let lang_mode: String = state.active_tab().map(|t| {
                        match t.model.language.as_str() {
                            "rust" => "Rust".to_string(),
                            "typescript" => "TypeScript".to_string(),
                            "javascript" => "JavaScript".to_string(),
                            "python" => "Python".to_string(),
                            "json" => "JSON".to_string(),
                            "toml" => "TOML".to_string(),
                            "markdown" => "Markdown".to_string(),
                            "bash" => "Shell Script".to_string(),
                            "html" => "HTML".to_string(),
                            "css" => "CSS".to_string(),
                            other => other.to_string(),
                        }
                    }).unwrap_or_else(|| "Plain Text".to_string());

                    div()
                        .cursor_pointer()
                        .px_1p5()
                        .py_0p5()
                        .rounded(px(3.0))
                        .hover(|s| s.bg(theme.bg_hover).text_color(theme.text_primary))
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                            let cmds = this.commands.clone();
                            this.quick_open.open_commands(&this.state, &cmds);
                            cx.notify();
                        }))
                        .child(lang_mode)
                })
                // Notifications indicator
                .child(
                    div()
                        .cursor_pointer()
                        .px_1p5()
                        .py_0p5()
                        .rounded(px(3.0))
                        .hover(|s| s.bg(theme.bg_hover).text_color(theme.text_primary))
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                            this.state.toast_manager.push_info("Notifications: No unread notifications");
                            cx.notify();
                        }))
                        .child(ui_icon(IconName::Info, 11.0, theme.text_muted))
                )
                // Devices
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .cursor_pointer()
                        .px_1p5()
                        .py_0p5()
                        .rounded(px(3.0))
                        .hover(|s| s.bg(theme.bg_hover).text_color(theme.text_primary))
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                            this.state.toast_manager.push_info("Connected Devices: Windows x86_64 Desktop Host");
                            cx.notify();
                        }))
                        .child(ui_icon(IconName::Smartphone, 11.0, theme.text_muted))
                        .child(div().child("Devices"))
                )
        )
}
