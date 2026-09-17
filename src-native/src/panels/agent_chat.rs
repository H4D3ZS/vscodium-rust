use crate::app_state::{ChatMessage, FocusedPanel, HadesNativeState, RightSidebarTab};
use crate::ui::icons::{icon_12, icon_14, ui_icon, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;

pub fn render_agent_chat_panel(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    crate::panels::render_right_sidebar_panel(state, cx)
}

pub fn render_agent_chat_body(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;
    let is_focused = state.focused_panel == FocusedPanel::Composer;
    let streaming_text = state.current_streaming_text();
    let streaming_thoughts = state.current_streaming_thoughts();

    div()
        .flex()
        .flex_col()
        .size_full()
        .bg(theme.bg_sidebar)
        // Main Content Area
        .child(
            div()
                .id("agent_chat_messages_scroll")
                .flex()
                .flex_col()
                .flex_1()
                .overflow_y_scroll()
                .p_2p5()
                .gap_2p5()
                .children(
                    (state.chat_messages.is_empty())
                        .then(|| render_fresh_agent_view(state, cx).into_any_element()),
                )
                .children((!state.chat_messages.is_empty()).then(|| {
                    div()
                        .flex()
                        .flex_col()
                        .flex_1()
                        .gap_3()
                        .children(
                            state
                                .chat_messages
                                .iter()
                                .enumerate()
                                .map(|(idx, msg)| render_chat_message(msg, idx, state, theme, cx)),
                        )
                        .into_any_element()
                }))
                // Live streaming response bubble while agent is actively generating
                .children((state.is_agent_thinking).then(|| {
                    let (live_tokens, live_tps, live_elapsed, live_mtp) = state.current_live_metrics();
                    let tps_label = if live_tps > 0.0 {
                        format!("{:.1} tok/s", live_tps)
                    } else {
                        "Decoding...".to_string()
                    };
                    div()
                        .flex()
                        .flex_col()
                        .w_full()
                        .gap_1p5()
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .justify_between()
                                .text_xs()
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_1p5()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.accent)
                                        .child(ui_icon(IconName::Bot, 13.0, theme.accent))
                                        .child("Agent Executing (Auto-Approve)"),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap_1()
                                                .px_2()
                                                .py_0p5()
                                                .rounded(px(4.0))
                                                .bg(rgba(0x38bdf814))
                                                .border_1()
                                                .border_color(rgba(0x38bdf833))
                                                .text_color(theme.accent)
                                                .child(ui_icon(IconName::Zap, 10.0, theme.accent))
                                                .child(format!(
                                                    "{} · {} tok{}{}",
                                                    tps_label,
                                                    live_tokens,
                                                    if let Some(mtp) = live_mtp { format!(" · MTP {:.0}%", mtp) } else { String::new() },
                                                    if live_elapsed > 0.0 { format!(" ({:.1}s)", live_elapsed) } else { String::new() }
                                                ))
                                        )
                                        .child(
                                            div().text_color(theme.status_green).child("● Live Actions"),
                                        ),
                                ),
                        )
                        // Live Streaming Thoughts / Reasoning
                        .children((!streaming_thoughts.trim().is_empty()).then(|| {
                            render_composer_thinking_block(
                                &streaming_thoughts,
                                None,
                                true,
                                state.live_thought_expanded,
                                None,
                                theme,
                                cx,
                            ).into_any_element()
                        }))
                        // Live Streaming Body
                        .child(
                            div()
                                .p_2p5()
                                .rounded(px(6.0))
                                .bg(theme.bg_card)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .text_xs()
                                .text_color(theme.text_primary)
                                .child(if streaming_text.is_empty() {
                                    div()
                                        .child("Waiting for model output...")
                                        .into_any_element()
                                } else {
                                    crate::ui::markdown::render_markdown(&streaming_text, theme)
                                        .into_any_element()
                                }),
                        )
                })),
        )
        // Bottom: Cursor AI Composer Card
        .child(
            div()
                .flex()
                .flex_col()
                .p_2p5()
                .bg(theme.bg_sidebar)
                .border_t_1()
                .border_color(theme.border_subtle)
                .gap_1p5()
                // Floating Model Picker Dropdown
                .children((state.model_picker_open).then(|| {
                    render_model_picker(state, cx).into_any_element()
                }))
                // Floating Mode Switcher Dropdown
                .children((state.mode_picker_open).then(|| {
                    render_mode_switcher(state, cx).into_any_element()
                }))
                // @mention autocomplete popup
                .children(
                    (state
                        .composer_input
                        .split_whitespace()
                        .last()
                        .map(|w| w.starts_with('@'))
                        .unwrap_or(false))
                    .then(|| {
                        let query = state
                            .composer_input
                            .split_whitespace()
                            .last()
                            .unwrap_or("@")
                            .trim_start_matches('@');
                        let matching_files: Vec<_> = state
                            .file_tree
                            .iter()
                            .filter(|f| {
                                !f.is_dir
                                    && (query.is_empty()
                                        || f.name.to_lowercase().contains(&query.to_lowercase()))
                            })
                            .take(5)
                            .collect();
                        div()
                            .flex()
                            .flex_col()
                            .p_1p5()
                            .rounded(px(6.0))
                            .bg(theme.bg_raised)
                            .border_1()
                            .border_color(theme.border_focus)
                            .gap_1()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .px_1p5()
                                    .text_xs()
                                    .text_color(theme.text_subtle)
                                    .child("Mention file context:"),
                            )
                            .children(matching_files.into_iter().map(|f| {
                                let name = f.name.clone();
                                let name_click = f.name.clone();
                                let path = f.path.clone();
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .justify_between()
                                    .px_2()
                                    .py_1()
                                    .rounded(px(3.0))
                                    .cursor_pointer()
                                    .hover(|s| s.bg(theme.bg_hover))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |this, _event, _window, cx| {
                                            if let Some(pos) = this.state.composer_input.rfind('@')
                                            {
                                                this.state.composer_input.truncate(pos);
                                                this.state
                                                    .composer_input
                                                    .push_str(&format!("@{} ", name_click));
                                            }
                                            cx.notify();
                                        }),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_1p5()
                                            .child(icon_12(IconName::FileCode, theme.accent))
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(theme.text_primary)
                                                    .child(name),
                                            ),
                                    )
                                    .child(
                                        div().text_xs().text_color(theme.text_subtle).child(path),
                                    )
                            }))
                    }),
                )
                // Main Chat Input Box
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .rounded(px(8.0))
                        .bg(rgb(0x161719))
                        .border_1()
                        .border_color(if is_focused {
                            theme.border_focus
                        } else {
                            rgba(0xffffff14)
                        })
                        .p_2p5()
                        .gap_2()
                        .cursor_pointer()
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, window, cx| {
                                this.state.focused_panel = FocusedPanel::Composer;
                                window.focus(&this.focus_handle, cx);
                                cx.notify();
                            }),
                        )
                        // Context Chips: + Codebase, + Docs, + Web, @ Files (Cursor IDE signature)
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1p5()
                                .mb_1()
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .px_2()
                                        .py_0p5()
                                        .rounded_full()
                                        .bg(theme.bg_raised)
                                        .border_1()
                                        .border_color(theme.border_subtle)
                                        .text_xs()
                                        .text_color(theme.text_subtle)
                                        .cursor_pointer()
                                        .hover(|s| {
                                            s.bg(theme.bg_hover)
                                                .text_color(theme.text_primary)
                                                .border_color(theme.accent)
                                        })
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _event, window, cx| {
                                                this.state.composer_input.push_str("@Codebase ");
                                                this.state.focused_panel = FocusedPanel::Composer;
                                                window.focus(&this.focus_handle, cx);
                                                cx.notify();
                                            }),
                                        )
                                        .child(ui_icon(IconName::Sparkles, 10.0, theme.accent))
                                        .child("+ Codebase"),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .px_2()
                                        .py_0p5()
                                        .rounded_full()
                                        .bg(theme.bg_raised)
                                        .border_1()
                                        .border_color(theme.border_subtle)
                                        .text_xs()
                                        .text_color(theme.text_subtle)
                                        .cursor_pointer()
                                        .hover(|s| {
                                            s.bg(theme.bg_hover)
                                                .text_color(theme.text_primary)
                                                .border_color(theme.accent)
                                        })
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _event, window, cx| {
                                                this.state.composer_input.push_str("@Docs ");
                                                this.state.focused_panel = FocusedPanel::Composer;
                                                window.focus(&this.focus_handle, cx);
                                                cx.notify();
                                            }),
                                        )
                                        .child(ui_icon(IconName::FileText, 10.0, theme.accent))
                                        .child("+ Docs"),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .px_2()
                                        .py_0p5()
                                        .rounded_full()
                                        .bg(theme.bg_raised)
                                        .border_1()
                                        .border_color(theme.border_subtle)
                                        .text_xs()
                                        .text_color(theme.text_subtle)
                                        .cursor_pointer()
                                        .hover(|s| {
                                            s.bg(theme.bg_hover)
                                                .text_color(theme.text_primary)
                                                .border_color(theme.accent)
                                        })
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _event, window, cx| {
                                                this.state.composer_input.push_str("@Web ");
                                                this.state.focused_panel = FocusedPanel::Composer;
                                                window.focus(&this.focus_handle, cx);
                                                cx.notify();
                                            }),
                                        )
                                        .child(ui_icon(IconName::Globe, 10.0, theme.accent))
                                        .child("+ Web"),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .px_2()
                                        .py_0p5()
                                        .rounded_full()
                                        .bg(theme.bg_raised)
                                        .border_1()
                                        .border_color(theme.border_subtle)
                                        .text_xs()
                                        .text_color(theme.text_subtle)
                                        .cursor_pointer()
                                        .hover(|s| {
                                            s.bg(theme.bg_hover)
                                                .text_color(theme.text_primary)
                                                .border_color(theme.accent)
                                        })
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _event, window, cx| {
                                                this.state.composer_input.push('@');
                                                this.state.focused_panel = FocusedPanel::Composer;
                                                window.focus(&this.focus_handle, cx);
                                                cx.notify();
                                            }),
                                        )
                                        .child(ui_icon(IconName::FileCode, 10.0, theme.text_muted))
                                        .child("@ Files"),
                                ),
                        )
                        // Input Area
                        .child(
                            div()
                                .min_h(px(52.0))
                                .w_full()
                                .cursor_text()
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, window, cx| {
                                        this.state.focused_panel = FocusedPanel::Composer;
                                        window.focus(&this.focus_handle, cx);
                                        cx.notify();
                                    }),
                                )
                                .text_xs()
                                .text_color(if state.composer_input.is_empty() {
                                    theme.text_subtle
                                } else {
                                    theme.text_primary
                                })
                                .child(if state.composer_input.is_empty() {
                                    div()
                                        .flex()
                                        .items_center()
                                        .child(
                                            div()
                                                .text_color(theme.text_subtle)
                                                .child("Ask anything...  (type @ to mention a file, / for commands)")
                                        )
                                        .children(is_focused.then(|| {
                                            div()
                                                .w(px(2.0))
                                                .h(px(14.0))
                                                .bg(theme.accent)
                                                .ml_0p5()
                                        }))
                                        .into_any_element()
                                } else {
                                    div()
                                        .flex()
                                        .flex_wrap()
                                        .items_center()
                                        .child(div().child(state.composer_input.clone()))
                                        .children(is_focused.then(|| {
                                            div()
                                                .w(px(2.0))
                                                .h(px(14.0))
                                                .bg(theme.accent)
                                                .ml_0p5()
                                        }))
                                        .into_any_element()
                                }),
                        )
                        // Input Toolbar
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .justify_between()
                                .pt_1p5()
                                .border_t_1()
                                .border_color(theme.border_subtle)
                                // Left Icons & Dropdowns
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_1p5()
                                        // Paperclip / Attach
                                        .child(
                                            div()
                                                .cursor_pointer()
                                                .hover(|s| s.text_color(theme.text_primary))
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(|this, _event, window, cx| {
                                                        this.state.composer_input.push_str("@Files ");
                                                        this.state.focused_panel = FocusedPanel::Composer;
                                                        window.focus(&this.focus_handle, cx);
                                                        cx.notify();
                                                    }),
                                                )
                                                .child(ui_icon(
                                                    IconName::Paperclip,
                                                    12.0,
                                                    theme.text_subtle,
                                                )),
                                        )
                                        // Globe (Web Search)
                                        .child(
                                            div()
                                                .cursor_pointer()
                                                .hover(|s| s.text_color(theme.text_primary))
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(|this, _event, window, cx| {
                                                        this.state.composer_input.push_str("@Web ");
                                                        this.state.focused_panel = FocusedPanel::Composer;
                                                        window.focus(&this.focus_handle, cx);
                                                        cx.notify();
                                                    }),
                                                )
                                                .child(ui_icon(
                                                    IconName::Globe,
                                                    12.0,
                                                    theme.text_subtle,
                                                )),
                                        )
                                        // Mode Pill: Ask ▾
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap_1()
                                                .px_1p5()
                                                .py_0p5()
                                                .rounded(px(4.0))
                                                .bg(theme.bg_raised)
                                                .border_1()
                                                .border_color(theme.border_subtle)
                                                .text_xs()
                                                .text_color(match state.agent_mode.as_str() {
                                                    "Agent" => rgb(0x4ade80),
                                                    "Edit" => rgb(0xf59e0b),
                                                    "Plan" => rgb(0xa855f7),
                                                    _ => rgb(0x38bdf8),
                                                })
                                                .cursor_pointer()
                                                .hover(|s| s.bg(theme.bg_hover))
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(|this, _event, _window, cx| {
                                                        this.state.toggle_mode_picker();
                                                        cx.notify();
                                                    }),
                                                )
                                                .child(ui_icon(
                                                    IconName::MessageSquare,
                                                    10.0,
                                                    match state.agent_mode.as_str() {
                                                        "Agent" => rgb(0x4ade80),
                                                        "Edit" => rgb(0xf59e0b),
                                                        "Plan" => rgb(0xa855f7),
                                                        _ => rgb(0x38bdf8),
                                                    },
                                                ))
                                                .child(state.agent_mode.clone())
                                                .child(icon_12(
                                                    IconName::ChevronDown,
                                                    theme.text_subtle,
                                                )),
                                        )
                                        // Model Pill: Qwen3.8-27B-GGUF-IQ3_... ▾
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap_1()
                                                .px_1p5()
                                                .py_0p5()
                                                .rounded(px(4.0))
                                                .bg(theme.bg_raised)
                                                .border_1()
                                                .border_color(theme.border_subtle)
                                                .text_xs()
                                                .text_color(theme.text_muted)
                                                .cursor_pointer()
                                                .hover(|s| s.bg(theme.bg_hover))
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(|this, _event, _window, cx| {
                                                        this.state.toggle_model_picker();
                                                        cx.notify();
                                                    }),
                                                )
                                                .child(div()
                                                    .w(px(6.0))
                                                    .h(px(6.0))
                                                    .rounded_full()
                                                    .bg(if state.agent_model.contains("GGUF")
                                                        || state.agent_model.contains("Escha")
                                                        || state.agent_model.starts_with("Qwen3.")
                                                    {
                                                        rgb(0x22c55e) // Green for local
                                                    } else if state.agent_model.contains("ModelScope") {
                                                        rgb(0xf97316) // Orange for ModelScope
                                                    } else {
                                                        rgb(0x38bdf8) // Blue for Cloud BYOK
                                                    })
                                                )
                                                .child({
                                                    let m = &state.agent_model;
                                                    let display_name = if m.starts_with("Qwen3.8") {
                                                        "Qwen 3.8 27B"
                                                    } else if m.starts_with("Qwen3.5") {
                                                        "Qwen 3.5 4B"
                                                    } else if m.starts_with("Escha") {
                                                        "Escha 35B"
                                                    } else if m.starts_with("Qwen 2.5 Coder") {
                                                        "Qwen 2.5 Coder"
                                                    } else if m.starts_with("Qwen Max") {
                                                        "Qwen Max"
                                                    } else if m.starts_with("Qwen Plus") {
                                                        "Qwen Plus"
                                                    } else if m.starts_with("DeepSeek V3 (ModelScope)") {
                                                        "DeepSeek V3 (MS)"
                                                    } else if m.starts_with("Claude") {
                                                        "Claude 3.5"
                                                    } else if m.starts_with("Gemini") {
                                                        "Gemini 1.5"
                                                    } else if m.starts_with("GPT") {
                                                        "GPT-4o"
                                                    } else if m.len() > 18 {
                                                        &m[..18]
                                                    } else {
                                                        m.as_str()
                                                    };
                                                    format!("{} ▾", display_name)
                                                }),
                                        )
                                        // Qwen Ambassador Pill Badge in Top Bar
                                        .children(state.agent_model.contains("ModelScope").then(|| {
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap_1()
                                                .px_2()
                                                .py_0p5()
                                                .rounded(px(4.0))
                                                .bg(rgba(0xf9731618))
                                                .border_1()
                                                .border_color(rgba(0xf9731644))
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .font_weight(FontWeight::BOLD)
                                                        .text_color(rgb(0xf97316))
                                                        .child("✦ Qwen Ambassador")
                                                )
                                        }))
                                        // Think Toggle
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap_1()
                                                .px_1p5()
                                                .py_0p5()
                                                .rounded(px(4.0))
                                                .bg(if state.reasoning_active {
                                                    rgba(0xa855f725)
                                                } else {
                                                    theme.bg_raised
                                                })
                                                .border_1()
                                                .border_color(if state.reasoning_active {
                                                    rgb(0xa855f7)
                                                } else {
                                                    theme.border_subtle
                                                })
                                                .text_xs()
                                                .text_color(if state.reasoning_active {
                                                    rgb(0xc084fc)
                                                } else {
                                                    theme.text_muted
                                                })
                                                .cursor_pointer()
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(|this, _event, _window, cx| {
                                                        this.state.reasoning_active =
                                                            !this.state.reasoning_active;
                                                        this.state.toast_manager.push_info(
                                                            if this.state.reasoning_active {
                                                                "Thinking Process: ON"
                                                            } else {
                                                                "Thinking Process: OFF"
                                                            },
                                                        );
                                                        cx.notify();
                                                    }),
                                                )
                                                .child(ui_icon(
                                                    IconName::Brain,
                                                    10.0,
                                                    if state.reasoning_active {
                                                        rgb(0xc084fc)
                                                    } else {
                                                        theme.text_muted
                                                    },
                                                ))
                                                .child("Think"),
                                        ),
                                )
                                // Right: Send / Stop Button with flex_shrink_0
                                .child(
                                    div()
                                        .flex()
                                        .flex_shrink_0()
                                        .items_center()
                                        .gap_1()
                                        .px_3()
                                        .py_1()
                                        .rounded(px(6.0))
                                        .bg(if state.is_agent_thinking {
                                            rgb(0xef4444)
                                        } else if state.composer_input.trim().is_empty() {
                                            theme.bg_raised
                                        } else {
                                            rgb(0x388bfd)
                                        })
                                        .text_color(if state.is_agent_thinking || !state.composer_input.trim().is_empty() {
                                            rgb(0xffffff)
                                        } else {
                                            theme.text_subtle
                                        })
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .cursor_pointer()
                                        .hover(|s| s.opacity(0.85))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _event, window, cx| {
                                                if this.state.is_agent_thinking {
                                                    this.state.cancel_ai_turn();
                                                } else if !this.state.composer_input.trim().is_empty() {
                                                    let prompt = this.state.composer_input.clone();
                                                    this.state.send_ai_prompt(&prompt);
                                                    window.focus(&this.focus_handle, cx);
                                                }
                                                cx.notify();
                                            }),
                                        )
                                        .child(ui_icon(
                                            if state.is_agent_thinking {
                                                IconName::Square
                                            } else {
                                                IconName::Send
                                            },
                                            12.0,
                                            if state.is_agent_thinking || !state.composer_input.trim().is_empty() {
                                                rgb(0xffffff)
                                            } else {
                                                theme.text_subtle
                                            },
                                        ))
                                        .child(if state.is_agent_thinking { "Stop" } else { "Send" }),
                                ),
                        ),
                )
                // Footer Status / Policy Toggles
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .px_1()
                        .pt_1()
                        .text_xs()
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_2()
                                // Auto-Approve / Review Edits Mode Toggle
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .px_1p5()
                                        .py_0p5()
                                        .rounded(px(4.0))
                                        .bg(if state.is_yolo_mode { rgba(0x22c55e14) } else { theme.bg_editor })
                                        .border_1()
                                        .border_color(if state.is_yolo_mode { rgba(0x22c55e33) } else { theme.border_subtle })
                                        .cursor_pointer()
                                        .hover(|s| s.bg(theme.bg_hover))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _event, _window, cx| {
                                                this.state.is_yolo_mode = !this.state.is_yolo_mode;
                                                this.state.toast_manager.push_info(if this.state.is_yolo_mode {
                                                    "Auto-Approve Mode: ON"
                                                } else {
                                                    "Review Edits Mode: Active"
                                                });
                                                cx.notify();
                                            }),
                                        )
                                        .child(ui_icon(if state.is_yolo_mode { IconName::Check } else { IconName::Eye }, 11.0, if state.is_yolo_mode { rgb(0x22c55e) } else { theme.text_muted }))
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::MEDIUM)
                                                .text_color(if state.is_yolo_mode { rgb(0x22c55e) } else { theme.text_muted })
                                                .child(if state.is_yolo_mode { "Auto-Approve" } else { "Review Edits" }),
                                        ),
                                )
                                // Clear chat button
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .px_1p5()
                                        .py_0p5()
                                        .rounded(px(4.0))
                                        .cursor_pointer()
                                        .hover(|s| s.bg(theme.bg_hover))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _event, _window, cx| {
                                                this.state.clear_chat();
                                                cx.notify();
                                            }),
                                        )
                                        .child(ui_icon(IconName::Trash, 11.0, theme.text_muted))
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.text_muted)
                                                .child("Clear"),
                                        ),
                                )
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_subtle)
                                .child("Local Engine"),
                        ),
                ),
        )
}

fn render_fresh_agent_view(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;

    div()
        .flex()
        .flex_col()
        .w_full()
        .gap_3()
        .p_1()
        // Clean Hero Card
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1p5()
                .p_3()
                .rounded(px(8.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_subtle)
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .child(ui_icon(IconName::Sparkles, 15.0, theme.accent))
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_primary)
                                        .child("AI Pair Programmer"),
                                ),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .px_1p5()
                                .py_0p5()
                                .rounded(px(3.0))
                                .bg(rgba(0x22c55e14))
                                .child(div().w(px(5.0)).h(px(5.0)).rounded_full().bg(rgb(0x22c55e)))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(rgb(0x22c55e))
                                        .child("Connected"),
                                ),
                        ),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .child("Ask questions about your codebase, request edits, or run workflows."),
                ),
        )
        // Quick Actions Section
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(theme.text_subtle)
                .child("SUGGESTED WORKFLOWS"),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1p5()
                .child(render_quick_action_row(
                    "Audit codebase for security & bug findings",
                    IconName::Shield,
                    rgb(0xfbbf24),
                    "Audit the active workspace for potential bugs, security issues, and dead code. Provide actionable findings.",
                    theme,
                    cx,
                ))
                .child(render_quick_action_row(
                    "Diagnose & fix compiler or runtime errors",
                    IconName::Wrench,
                    rgb(0xf43f5e),
                    "Inspect compiler diagnostics or build logs, identify root causes, and propose fixes.",
                    theme,
                    cx,
                ))
                .child(render_quick_action_row(
                    "Build & verify project binaries and tests",
                    IconName::Box,
                    rgb(0x818cf8),
                    "Run cargo check and verify all modules build and tests pass cleanly.",
                    theme,
                    cx,
                ))
                .child(render_quick_action_row(
                    "Generate semantic git commit from diff",
                    IconName::GitCommitHorizontal,
                    rgb(0x34d399),
                    "Inspect git status and diff, stage modified files, and draft a clear semantic commit message.",
                    theme,
                    cx,
                ))
        )
}

fn render_quick_action_row(
    title: &'static str,
    icon: IconName,
    icon_color: Rgba,
    prompt: &'static str,
    theme: &crate::theme::Theme,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .p_2p5()
        .rounded(px(6.0))
        .bg(theme.bg_editor)
        .border_1()
        .border_color(theme.border_subtle)
        .cursor_pointer()
        .hover(|s| s.bg(theme.bg_hover).border_color(theme.border_focus))
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this, _event, window, cx| {
                this.state.send_ai_prompt(prompt);
                window.focus(&this.focus_handle, cx);
                cx.notify();
            }),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(ui_icon(icon, 13.0, icon_color))
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_primary)
                        .child(title),
                ),
        )
        .child(ui_icon(IconName::ArrowRight, 10.0, theme.text_muted))
}

/// Collapsible Thinking Block directly matching ComposerThinkingBlock.tsx
fn render_composer_thinking_block(
    thoughts: &str,
    duration_ms: Option<u64>,
    is_streaming: bool,
    is_expanded: bool,
    msg_idx: Option<usize>,
    theme: &crate::theme::Theme,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let label = if is_streaming {
        "Thinking...".to_string()
    } else if let Some(ms) = duration_ms {
        format!("Thought for {:.1}s", ms as f64 / 1000.0)
    } else {
        "Thought process".to_string()
    };

    div()
        .flex()
        .flex_col()
        .w_full()
        .rounded(px(6.0))
        .bg(rgb(0x0e1117))
        .border_1()
        .border_color(if is_streaming { rgba(0xa855f755) } else { theme.border_subtle })
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .px_2p5()
                .py_1p5()
                .cursor_pointer()
                .hover(|s| s.bg(rgba(0xffffff08)))
                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                    if let Some(idx) = msg_idx {
                        this.state.toggle_message_thoughts(idx);
                    } else if is_streaming {
                        this.state.toggle_live_thought();
                    }
                    cx.notify();
                }))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .w(px(7.0))
                                .h(px(7.0))
                                .rounded_full()
                                .bg(if is_streaming { rgb(0xc084fc) } else { rgb(0x4ade80) })
                        )
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(if is_streaming { rgb(0xc084fc) } else { theme.text_muted })
                                .child(label)
                        )
                )
                .child(
                    ui_icon(
                        if is_expanded { IconName::ChevronUp } else { IconName::ChevronDown },
                        11.0,
                        theme.text_subtle,
                    )
                )
        )
        .children(is_expanded.then(|| {
            div()
                .px_3()
                .py_2()
                .border_t_1()
                .border_color(theme.border_subtle)
                .text_xs()
                .text_color(theme.text_subtle)
                .child(thoughts.trim().to_string())
        }))
}

/// Floating Model Picker Dropdown matching ModelPicker.tsx
fn render_model_picker(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;

    let local_models = [
        (
            "Qwen3.8-27B-Uncensored-Cyber-IQ4_XS-imatrix-fromq8.gguf",
            "Qwen 3.8 27B Cyber Agentic",
            "Uncensored Agentic Local LLM · imatrix IQ4_XS · Active in VRAM",
            "Priority Local",
            rgb(0x22c55e),
            true,
        ),
        (
            "Qwen3.5-4B-GGUF-Q5_K_M",
            "Qwen 3.5 4B",
            "Ultra-Fast Lightweight Local LLM · 32k Context",
            "Fast Local",
            rgb(0x34d399),
            true,
        ),
        (
            "Escha-W2-35B-A3B-ROCmFP2-Qwen3.6-35B-A3B-Escha-W2-ROCmFP2.gguf",
            "Escha 35B Deep Reasoning",
            "High-Param Deep Thinking · 64k Context · ROCm/Vulkan",
            "Reasoning Local",
            rgb(0xfbbf24),
            true,
        ),
        (
            "Qwen3.8-35B-A3B-Q4_K_M.gguf",
            "Qwen 3.8 35B-A3B Distill",
            "35B MoE Distill (3B Active) · High Efficiency Local Coding",
            "Distill Local",
            rgb(0xa855f7),
            true,
        ),
        (
            "Qwen3-Embedding-0.6B-GGUF",
            "Qwen3 Embedding 0.6B",
            "Code & Context Vector Indexing · 100% Offline",
            "Embed Local",
            rgb(0x38bdf8),
            true,
        ),
    ];

    let modelscope_models = [
        (
            "Qwen 2.5 Coder (ModelScope)",
            "Qwen 2.5 Coder 32B",
            "Alibaba ModelScope · 32B Specialist Coding Agent",
            "ModelScope API",
            rgb(0xf97316),
            !state.modelscope_api_key.is_empty(),
        ),
        (
            "Qwen Max (ModelScope)",
            "Qwen Max",
            "Alibaba ModelScope · Flagship Deep Reasoning",
            "ModelScope API",
            rgb(0xf97316),
            !state.modelscope_api_key.is_empty(),
        ),
        (
            "Qwen Plus (ModelScope)",
            "Qwen Plus",
            "Alibaba ModelScope · Balanced Fast Coding",
            "ModelScope API",
            rgb(0xf97316),
            !state.modelscope_api_key.is_empty(),
        ),
        (
            "DeepSeek V3 (ModelScope)",
            "DeepSeek V3 (ModelScope)",
            "Alibaba ModelScope · High Speed V3 Gateway",
            "ModelScope API",
            rgb(0xf97316),
            !state.modelscope_api_key.is_empty(),
        ),
    ];

    let cloud_byok_models = [
        (
            "Claude 3.5 Sonnet",
            "Claude 3.5 Sonnet",
            "Anthropic API · Extended Thinking & Planning",
            "BYOK Cloud",
            rgb(0x60a5fa),
            !state.anthropic_api_key.is_empty(),
        ),
        (
            "Gemini 1.5 Pro",
            "Gemini 1.5 Pro",
            "Google API · 2M Long-Horizon Context",
            "BYOK Cloud",
            rgb(0x60a5fa),
            !state.gemini_api_key.is_empty(),
        ),
        (
            "GPT-4o",
            "GPT-4o",
            "OpenAI API · Multimodal Reasoning",
            "BYOK Cloud",
            rgb(0x60a5fa),
            !state.openai_api_key.is_empty(),
        ),
        (
            "DeepSeek V3",
            "DeepSeek V3 (Official)",
            "DeepSeek API · Official DeepSeek-Chat",
            "BYOK Cloud",
            rgb(0x60a5fa),
            !state.deepseek_api_key.is_empty(),
        ),
    ];

    let render_card = |id: &'static str, name: &'static str, desc: &'static str, badge: &'static str, badge_color: Rgba, is_ready: bool, cx: &mut Context<HadesAppView>| {
        let is_active = state.agent_model == id;
        let target_id = id.to_string();
        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .px_2p5()
            .py_1p5()
            .rounded(px(5.0))
            .cursor_pointer()
            .bg(if is_active { rgba(0x388bfd18) } else { theme.bg_raised })
            .border_1()
            .border_color(if is_active { theme.accent } else { theme.border_subtle })
            .hover(|s| s.bg(theme.bg_hover))
            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                this.state.set_agent_model(&target_id);
                cx.notify();
            }))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_0p5()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1p5()
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(if is_active { theme.accent } else { theme.text_primary })
                                    .child(name)
                            )
                            .child(
                                div()
                                    .px_1p5()
                                    .py_0p5()
                                    .rounded(px(3.0))
                                    .bg(rgba(0xffffff0a))
                                    .text_xs()
                                    .text_color(badge_color)
                                    .child(badge)
                            )
                            .child(
                                div()
                                    .px_1p5()
                                    .py_0p5()
                                    .rounded(px(3.0))
                                    .bg(if is_ready { rgba(0x22c55e15) } else { rgba(0xf59e0b15) })
                                    .text_xs()
                                    .text_color(if is_ready { rgb(0x4ade80) } else { rgb(0xfbbf24) })
                                    .child(if is_ready { "● Ready" } else { "Key Needed" })
                            )
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_subtle)
                            .child(desc)
                    )
            )
            .child(
                div()
                    .child(if is_active {
                        ui_icon(IconName::Check, 14.0, theme.accent).into_any_element()
                    } else {
                        div().into_any_element()
                    })
            )
    };

    let section_title = |title: &'static str, icon: IconName, color: Rgba| {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap_1p5()
            .pt_1p5()
            .pb_0p5()
            .child(ui_icon(icon, 11.0, color))
            .child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::BOLD)
                    .text_color(color)
                    .child(title)
            )
    };

    div()
        .id("explore_repo_results_scroll")
        .flex()
        .flex_col()
        .w_full()
        .max_h(px(460.0))
        .overflow_y_scroll()
        .rounded(px(6.0))
        .bg(theme.bg_raised)
        .border_1()
        .border_color(theme.border_subtle)
        .p_2p5()
        .gap_2()
        // Header
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .pb_1p5()
                .border_b_1()
                .border_color(theme.border_subtle)
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1p5()
                        .child(ui_icon(IconName::Cpu, 12.0, theme.accent))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child("SELECT INFERENCE MODEL")
                        )
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .px_1p5()
                                .py_0p5()
                                .rounded(px(3.0))
                                .bg(rgba(0x22c55e15))
                                .text_xs()
                                .text_color(rgb(0x4ade80))
                                .child("● Lemonade :13305 / :8001")
                        )
                        .child(
                            div()
                                .cursor_pointer()
                                .hover(|s| s.text_color(theme.text_primary))
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                    this.state.model_picker_open = false;
                                    cx.notify();
                                }))
                                .child(ui_icon(IconName::X, 11.0, theme.text_subtle))
                        )
                )
        )
        // Group 1: Local Models (Primary)
        .child(section_title("LOCAL MODELS (EMBEDDED INFERENCE)", IconName::Cpu, rgb(0x4ade80)))
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .children(local_models.into_iter().map(|(id, name, desc, badge, badge_color, is_ready)| {
                    render_card(id, name, desc, badge, badge_color, is_ready, cx)
                }))
        )
        // Group 2: ModelScope (Alibaba Qwen Ambassador)
        .child(section_title("MODELSCOPE (ALIBABA QWEN AMBASSADOR)", IconName::Globe, rgb(0xf97316)))
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .children(modelscope_models.into_iter().map(|(id, name, desc, badge, badge_color, is_ready)| {
                    render_card(id, name, desc, badge, badge_color, is_ready, cx)
                }))
        )
        // Group 3: Cloud Inference (BYOK / BYOB)
        .child(section_title("CLOUD INFERENCE (BYOK · BRING YOUR OWN KEY)", IconName::Key, rgb(0x60a5fa)))
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .children(cloud_byok_models.into_iter().map(|(id, name, desc, badge, badge_color, is_ready)| {
                    render_card(id, name, desc, badge, badge_color, is_ready, cx)
                }))
        )
}

/// Floating Mode Switcher Dropdown matching ModeSwitcher.tsx
fn render_mode_switcher(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;
    div()
        .flex()
        .flex_col()
        .w_full()
        .rounded(px(6.0))
        .bg(theme.bg_raised)
        .border_1()
        .border_color(theme.border_subtle)
        .p_2p5()
        .gap_2()
        // Header
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .pb_1p5()
                .border_b_1()
                .border_color(theme.border_subtle)
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1p5()
                        .child(ui_icon(IconName::Settings2, 12.0, theme.accent))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child("AGENT OPERATING MODE")
                        )
                )
                .child(
                    div()
                        .cursor_pointer()
                        .hover(|s| s.text_color(theme.text_primary))
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                            this.state.mode_picker_open = false;
                            cx.notify();
                        }))
                        .child(ui_icon(IconName::X, 11.0, theme.text_subtle))
                )
        )
        // Modes List matching ModeSwitcher.tsx
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .children([
                    ("Ask", "Ask", "Ask questions, discuss architecture & explain code", rgb(0x34d399)),
                    ("Agent", "Agent", "Full autonomous agent with live tools and file edits", rgb(0x388bfd)),
                    ("Plan", "Plan", "Generate architectural plans and task lists first", rgb(0xf59e0b)),
                    ("Security", "Security", "Offensive security research, MobHunt SAST & exploit PoCs", rgb(0xef4444)),
                    ("Fast", "Fast", "Direct single-shot answers with zero subagent overhead", rgb(0xa855f7)),
                    ("Verify", "Verify", "Run tests, diagnostics, and code verification", rgb(0x06b6d4)),
                    ("Autonomous", "Autonomous", "Full autonomous loop with persistent task memory", rgb(0x818cf8)),
                ].into_iter().map(|(id, label, desc, color)| {
                    let is_active = state.agent_mode == id;
                    let target_mode = id.to_string();
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .px_2p5()
                        .py_1p5()
                        .rounded(px(5.0))
                        .cursor_pointer()
                        .bg(if is_active { rgba(0x388bfd18) } else { theme.bg_raised })
                        .border_1()
                        .border_color(if is_active { color } else { theme.border_subtle })
                        .hover(|s| s.bg(theme.bg_hover))
                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                            this.state.set_agent_mode(&target_mode);
                            cx.notify();
                        }))
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_0p5()
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_1p5()
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(color)
                                                .child(label)
                                        )
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_subtle)
                                        .child(desc)
                                )
                        )
                        .child(
                            div()
                                .child(if is_active {
                                    ui_icon(IconName::Check, 14.0, color).into_any_element()
                                } else {
                                    div().into_any_element()
                                })
                        )
                }))
        )
}

fn render_edit_message_box(
    idx: usize,
    state: &HadesNativeState,
    theme: &crate::theme::Theme,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let edit_input = state.edit_msg_input.clone();
    div()
        .flex()
        .flex_col()
        .w_full()
        .gap_2()
        .child(
            div()
                .p_2p5()
                .rounded(px(6.0))
                .bg(theme.bg_input)
                .border_1()
                .border_color(theme.accent)
                .text_xs()
                .text_color(theme.text_primary)
                .child(if edit_input.is_empty() {
                    div().text_color(theme.text_subtle).child("Type your prompt... (Press Enter to save & resend, Esc to cancel)")
                } else {
                    div().child(format!("{}▏", edit_input))
                })
        )
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_subtle)
                        .child("Editing message · Enter to resend · Esc to cancel")
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .px_2p5()
                                .py_1()
                                .rounded(px(4.0))
                                .bg(theme.bg_raised)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .text_xs()
                                .text_color(theme.text_muted)
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                    this.state.cancel_edit_message();
                                    cx.notify();
                                }))
                                .child("Cancel")
                        )
                        .child(
                            div()
                                .px_3()
                                .py_1()
                                .rounded(px(4.0))
                                .bg(theme.accent)
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0x000000))
                                .cursor_pointer()
                                .hover(|s| s.opacity(0.9))
                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                    this.state.save_and_resend_message(idx);
                                    cx.notify();
                                }))
                                .child("Save & Resend")
                        )
                )
        )
}

fn render_chat_message(
    msg: &ChatMessage,
    idx: usize,
    state: &HadesNativeState,
    theme: &crate::theme::Theme,
    cx: &mut Context<HadesAppView>,
) -> AnyElement {
    let model_name = &state.agent_model;
    let is_copied = state.copied_msg_idx == Some(idx);
    let is_modelscope = model_name.contains("ModelScope");
    let display_model = if is_modelscope {
        if model_name.contains("Coder") {
            "Qwen 2.5 Coder · ModelScope".to_string()
        } else if model_name.contains("Max") {
            "Qwen Max · ModelScope".to_string()
        } else if model_name.contains("Plus") {
            "Qwen Plus · ModelScope".to_string()
        } else if model_name.contains("DeepSeek") {
            "DeepSeek V3 · ModelScope".to_string()
        } else {
            "ModelScope".to_string()
        }
    } else if model_name.starts_with("Qwen3.8") {
        "Qwen 3.8 · Lemonade".to_string()
    } else if model_name.starts_with("Qwen3.5") {
        "Qwen 3.5 · Lemonade".to_string()
    } else if model_name.starts_with("Escha") {
        "Escha 35B · Lemonade".to_string()
    } else if model_name.len() > 24 {
        model_name[..24].to_string()
    } else {
        model_name.clone()
    };

    div()
        .flex()
        .flex_col()
        .w_full()
        .gap_2()
        // Message Header matching ChatMessage.tsx
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .text_xs()
                .child(if msg.is_user {
                    div()
                        .flex()
                        .items_center()
                        .gap_1p5()
                        .px_2()
                        .py_0p5()
                        .rounded(px(4.0))
                        .bg(theme.bg_raised)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .child(ui_icon(IconName::User, 11.0, theme.text_muted))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child("YOU"),
                        )
                        .into_any_element()
                } else {
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1p5()
                                .px_2()
                                .py_0p5()
                                .rounded(px(4.0))
                                .bg(if is_modelscope { rgba(0xf9731618) } else { rgba(0x38bdf815) })
                                .border_1()
                                .border_color(if is_modelscope { rgba(0xf9731644) } else { rgba(0x38bdf833) })
                                .child(
                                    div()
                                        .text_color(if is_modelscope { rgb(0xf97316) } else { theme.accent })
                                        .font_weight(FontWeight::BOLD)
                                        .child("✦"),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(if is_modelscope { rgb(0xf97316) } else { theme.accent })
                                        .child("Agent"),
                                ),
                        )
                        .child(
                            div()
                                .px_1p5()
                                .py_0p5()
                                .rounded(px(3.0))
                                .bg(theme.bg_raised)
                                .text_xs()
                                .text_color(if is_modelscope { rgb(0xf97316) } else { theme.text_subtle })
                                .child(display_model.clone()),
                        )
                        .children((is_modelscope).then(|| {
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .px_2()
                                .py_0p5()
                                .rounded(px(4.0))
                                .bg(rgba(0xf9731620))
                                .border_1()
                                .border_color(rgba(0xf9731655))
                                .child(
                                    div()
                                        .text_color(rgb(0xf97316))
                                        .font_weight(FontWeight::BOLD)
                                        .child("✦")
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0xf97316))
                                        .child("Qwen Ambassador")
                                )
                        }))
                        .into_any_element()
                })
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(div().text_xs().text_color(theme.text_subtle).child(msg.timestamp.clone()))
                        .children((msg.is_user).then(|| {
                            let text_copy = msg.text.clone();
                            let is_this_copied = is_copied;
                            div()
                                .flex()
                                .items_center()
                                .gap_1p5()
                                .child(
                                    // Edit button
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .px_1p5()
                                        .py_0p5()
                                        .rounded(px(3.0))
                                        .bg(theme.bg_raised)
                                        .border_1()
                                        .border_color(theme.border_subtle)
                                        .cursor_pointer()
                                        .hover(|s| s.bg(theme.bg_hover))
                                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                            this.state.start_edit_message(idx);
                                            this.state.focused_panel = FocusedPanel::Composer;
                                            cx.notify();
                                        }))
                                        .child(ui_icon(IconName::Replace, 10.0, theme.text_subtle))
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.text_subtle)
                                                .child("Edit")
                                        )
                                )
                                .child(
                                    // Copy button
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .px_1p5()
                                        .py_0p5()
                                        .rounded(px(3.0))
                                        .bg(theme.bg_raised)
                                        .border_1()
                                        .border_color(theme.border_subtle)
                                        .cursor_pointer()
                                        .hover(|s| s.bg(theme.bg_hover))
                                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                            cx.write_to_clipboard(ClipboardItem::new_string(text_copy.clone()));
                                            this.state.copied_msg_idx = Some(idx);
                                            this.state.toast_manager.push_info("Copied prompt to clipboard");
                                            cx.notify();
                                        }))
                                        .child(ui_icon(
                                            if is_this_copied { IconName::Check } else { IconName::Copy },
                                            10.0,
                                            if is_this_copied { rgb(0x4ade80) } else { theme.text_subtle },
                                        ))
                                )
                        }))
                        .children((!msg.is_user).then(|| {
                            let text = msg.text.clone();
                            let text_for_report = msg.text.clone();
                            let model_for_report = display_model.clone();
                            let is_ms = is_modelscope;
                            div()
                                .flex()
                                .items_center()
                                .gap_1p5()
                                .child(
                                    // Copy button
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .px_1p5()
                                        .py_0p5()
                                        .rounded(px(3.0))
                                        .bg(theme.bg_raised)
                                        .border_1()
                                        .border_color(theme.border_subtle)
                                        .cursor_pointer()
                                        .hover(|s| s.bg(theme.bg_hover))
                                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                            cx.write_to_clipboard(ClipboardItem::new_string(text.clone()));
                                            this.state.copied_msg_idx = Some(idx);
                                            this.state.toast_manager.push_info("Copied response to clipboard");
                                            cx.notify();
                                        }))
                                        .child(ui_icon(
                                            if is_copied { IconName::Check } else { IconName::Copy },
                                            10.0,
                                            if is_copied { rgb(0x4ade80) } else { theme.text_subtle },
                                        ))
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(if is_copied { rgb(0x4ade80) } else { theme.text_subtle })
                                                .child(if is_copied { "Copied" } else { "Copy" })
                                        )
                                )
                                .child(
                                    // Export Qwen Ambassador Report button
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .px_1p5()
                                        .py_0p5()
                                        .rounded(px(3.0))
                                        .bg(if is_ms { rgba(0xf9731615) } else { theme.bg_raised })
                                        .border_1()
                                        .border_color(if is_ms { rgba(0xf9731644) } else { theme.border_subtle })
                                        .cursor_pointer()
                                        .hover(|s| s.bg(theme.bg_hover))
                                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                            let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                                            let report = format!(
                                                "# Security & Reverse Engineering Report\n\n\
                                                > **AI Engine:** Alibaba ModelScope (`{}`)\n\
                                                > **Affiliation:** Official Qwen Ambassador Verified Analysis\n\
                                                > **Date:** {}\n\
                                                > **Platform:** VSCodium-Rust Reverse Engineering Suite (IDA Pro MCP)\n\n\
                                                ---\n\n\
                                                {}\n\n\
                                                ---\n\
                                                *Report generated natively via VSCodium-Rust AI Autonomous Agent · ModelScope Qwen Ambassador Pipeline.*\n",
                                                model_for_report,
                                                now,
                                                text_for_report
                                            );
                                            cx.write_to_clipboard(ClipboardItem::new_string(report.clone()));
                                            let reports_dir = this.state.workspace_root.join("reports");
                                            let _ = std::fs::create_dir_all(&reports_dir);
                                            let file_stamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
                                            let report_file = reports_dir.join(format!("qwen_ambassador_report_{}.md", file_stamp));
                                            let _ = std::fs::write(&report_file, &report);
                                            this.state.toast_manager.push_info("Exported Qwen Ambassador Report to clipboard & reports/");
                                            cx.notify();
                                        }))
                                        .child(ui_icon(IconName::FileText, 10.0, if is_ms { rgb(0xf97316) } else { theme.text_subtle }))
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(if is_ms { FontWeight::BOLD } else { FontWeight::NORMAL })
                                                .text_color(if is_ms { rgb(0xf97316) } else { theme.text_subtle })
                                                .child(if is_ms { "✦ Export Report" } else { "Export Report" })
                                        )
                                )
                        }))
                ),
        )
        // Attached Context Chips (e.g. @Codebase, @File, @Web)
        .children((!msg.context.is_empty()).then(|| {
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .gap_1p5()
                .children(msg.context.iter().map(|ctx| {
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .px_2()
                        .py_0p5()
                        .rounded(px(4.0))
                        .bg(theme.bg_raised)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .text_xs()
                        .text_color(theme.accent)
                        .child(ui_icon(IconName::FileText, 11.0, theme.accent))
                        .child(format!("@{}: {}", ctx.kind, ctx.name))
                }))
        }))
        // Checkpoint Card
        .children(msg.checkpoint_id.as_ref().map(|cp_id| {
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .px_2p5()
                .py_1p5()
                .rounded(px(4.0))
                .bg(rgba(0x38bdf810))
                .border_1()
                .border_color(rgba(0x38bdf830))
                .text_xs()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1p5()
                        .text_color(theme.accent)
                        .child(ui_icon(IconName::GitBranch, 12.0, theme.accent))
                        .child(format!(
                            "Checkpoint: {}",
                            msg.checkpoint_description.as_deref().unwrap_or(cp_id)
                        )),
                )
                .child(
                    div()
                        .px_2()
                        .py_0p5()
                        .rounded(px(3.0))
                        .bg(theme.accent)
                        .text_color(rgb(0x000000))
                        .font_weight(FontWeight::BOLD)
                        .cursor_pointer()
                        .child("↺ Restore"),
                )
        }))
        // Collapsible Thoughts / Reasoning Block matching ComposerThinkingBlock.tsx
        .children(msg.thoughts.as_ref().map(|thoughts| {
            let is_expanded = state.expanded_thoughts.contains(&idx);
            render_composer_thinking_block(
                thoughts,
                msg.thought_duration_ms,
                false,
                is_expanded,
                Some(idx),
                theme,
                cx,
            ).into_any_element()
        }))
        // Tool Execution Cards
        .children(msg.tool_calls.iter().map(|tc| {
            div()
                .flex()
                .flex_col()
                .p_2()
                .rounded(px(4.0))
                .bg(theme.bg_raised)
                .border_1()
                .border_color(theme.border_subtle)
                .text_xs()
                .gap_1()
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1p5()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.status_green)
                                .child(ui_icon(IconName::Terminal, 12.0, theme.status_green))
                                .child(format!("Executed: {}", tc.name)),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .child(icon_12(IconName::Check, theme.status_green))
                                .child(div().text_color(theme.status_green).child("Done")),
                        ),
                )
                .children(
                    tc.output
                        .as_ref()
                        .map(|out| div().text_color(theme.text_muted).child(out.clone())),
                )
        }))
        // Message Content Body matching MessageBody.tsx & ChatMessage.tsx
        .child(
            div()
                .px_3()
                .py_2p5()
                .rounded(px(8.0))
                .bg(if msg.is_user {
                    rgb(0x161719)
                } else {
                    theme.bg_card
                })
                .border_1()
                .border_color(if msg.is_user {
                    rgba(0xffffff14)
                } else {
                    theme.border_subtle
                })
                .text_xs()
                .text_color(if msg.is_user {
                    rgb(0xe6e7e9)
                } else {
                    theme.text_primary
                })
                .child(if msg.is_user {
                    if state.editing_msg_idx == Some(idx) {
                        render_edit_message_box(idx, state, theme, cx).into_any_element()
                    } else {
                        div().child(msg.text.clone()).into_any_element()
                    }
                } else {
                    crate::ui::markdown::render_markdown(&msg.text, theme).into_any_element()
                }),
        )
        // Token Metrics Badge (Cursor & Antigravity Style) for Assistant messages
        .children((!msg.is_user && msg.token_metrics.is_some()).then(|| {
            let m = msg.token_metrics.as_ref().unwrap();
            let mut parts = Vec::new();
            if let Some(tps) = m.predicted_per_second {
                let n_str = m.predicted_tokens.map(|n| format!("{} tokens", n)).unwrap_or_default();
                let time_str = m.predicted_ms.map(|ms| format!("({:.1}s)", ms / 1000.0)).unwrap_or_default();
                parts.push(format!("{:.1} tok/s · {} {}", tps, n_str, time_str));
            }
            if let Some(prompt_tps) = m.prompt_per_second {
                let p_time = m.prompt_ms.map(|ms| format!("({:.0}ms)", ms)).unwrap_or_default();
                parts.push(format!("Prefill: {:.1} tok/s {}", prompt_tps, p_time));
            }
            if let (Some(acc), Some(d)) = (m.draft_accepted, m.draft_tokens) {
                let rate = m.mtp_acceptance_rate.unwrap_or_else(|| if d > 0 { (acc as f64 / d as f64) * 100.0 } else { 0.0 });
                parts.push(format!("MTP: {:.1}% ({}/{} accepted)", rate, acc, d));
            }
            if let Some(cache_n) = m.cache_n {
                if cache_n > 0 {
                    parts.push(format!("KV Cache: {} hits", cache_n));
                }
            }
            let text_summary = parts.join(" · ");

            div()
                .flex()
                .flex_row()
                .items_center()
                .gap_1p5()
                .px_2()
                .py_1()
                .rounded(px(4.0))
                .bg(rgba(0x38bdf80f))
                .border_1()
                .border_color(rgba(0x38bdf826))
                .text_xs()
                .text_color(rgba(0x94a3b8ff))
                .child(ui_icon(IconName::Zap, 11.0, theme.accent))
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_subtle)
                        .child(text_summary)
                )
        }))
        // Interactive Plan Proceed Card (Antigravity-style) matching PlanProceedCard.tsx
        .children({
            let is_plan = !msg.is_user && (
                msg.text.contains("Implementation Plan") 
                || msg.text.contains("Plan:") 
                || msg.text.contains("## Proposed Changes")
                || msg.artifacts.iter().any(|a| a.kind == "plan")
            );
            is_plan.then(|| {
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .p_2p5()
                    .rounded(px(6.0))
                    .bg(rgba(0x10b98115))
                    .border_1()
                    .border_color(rgba(0x10b98144))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(ui_icon(IconName::Sparkles, 14.0, rgb(0x10b981)))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0x10b981))
                                            .child("Implementation Plan Ready")
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.text_subtle)
                                            .child("Review steps above and approve execution.")
                                    )
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .px_3()
                                    .py_1()
                                    .rounded(px(4.0))
                                    .bg(rgb(0x10b981))
                                    .text_color(rgb(0x000000))
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .cursor_pointer()
                                    .hover(|s| s.opacity(0.9))
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                        this.state.send_ai_prompt("Proceed with the implementation plan.");
                                        cx.notify();
                                    }))
                                    .child("Proceed")
                            )
                    )
            })
        })
        .into_any_element()
}

pub struct ComposerTab;

impl crate::panels::traits::AuxiliaryTab for ComposerTab {
    fn id(&self) -> &'static str {
        "composer"
    }

    fn title(&self) -> &'static str {
        "COMPOSER"
    }

    fn icon(&self) -> IconName {
        IconName::Sparkles
    }

    fn render(&self, state: &HadesNativeState, cx: &mut Context<HadesAppView>) -> AnyElement {
        render_agent_chat_panel(state, cx).into_any_element()
    }
}

pub struct AgentChatPanel;

impl crate::panels::traits::WorkbenchPanel for AgentChatPanel {
    fn id(&self) -> &'static str {
        "aichat"
    }

    fn title(&self) -> &'static str {
        "AI AGENT CHAT"
    }

    fn icon(&self) -> IconName {
        IconName::Sparkles
    }

    fn render(&self, state: &HadesNativeState, cx: &mut Context<HadesAppView>) -> AnyElement {
        render_agent_chat_panel(state, cx).into_any_element()
    }
}
