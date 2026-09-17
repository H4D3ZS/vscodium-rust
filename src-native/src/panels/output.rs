use crate::app_state::HadesNativeState;
use crate::panels::traits::BottomPanelTab;
use crate::ui::icons::{icon_12, icon_14, ui_icon, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct OutputState {
    pub active_channel: String,
    pub channels: Vec<String>,
    pub channel_buffers: HashMap<String, Vec<String>>,
    pub auto_scroll: bool,
    pub channel_dropdown_open: bool,
}

impl Default for OutputState {
    fn default() -> Self {
        let channels = vec![
            "Tasks".to_string(),
            "Rust Analyzer".to_string(),
            "Ext-Host Bridge".to_string(),
            "Direct3D 12 Engine".to_string(),
            "Git / SCM".to_string(),
            "AI Agent".to_string(),
        ];
        let mut channel_buffers = HashMap::new();
        channel_buffers.insert(
            "Tasks".to_string(),
            vec![
                "[Tasks] Initialized zero-latency in-process task scheduler.".to_string(),
                "[Tasks] Ready for build, test, and package tasks.".to_string(),
            ],
        );
        channel_buffers.insert(
            "Rust Analyzer".to_string(),
            vec![
                "[Rust Analyzer] Workspace symbols indexed across workspace crates.".to_string(),
                "[Rust Analyzer] Semantic tokens & inlay hints stream active.".to_string(),
            ],
        );
        channel_buffers.insert(
            "Ext-Host Bridge".to_string(),
            vec![
                "[Ext-Host] IPC channel established over NamedPipe.".to_string(),
                "[Ext-Host] 12 extension contributions loaded successfully.".to_string(),
            ],
        );
        channel_buffers.insert(
            "Direct3D 12 Engine".to_string(),
            vec![
                "[Direct3D 12] Swapchain initialized: DXGI_FORMAT_B8G8R8A8_UNORM, 120Hz refresh."
                    .to_string(),
                "[Direct3D 12] Tear-free flip-discard swap presentation running at zero latency."
                    .to_string(),
            ],
        );
        channel_buffers.insert(
            "Git / SCM".to_string(),
            vec!["[Git] Repository status synchronized with local working tree.".to_string()],
        );
        channel_buffers.insert(
            "AI Agent".to_string(),
            vec![
                "[AI Agent] Session model initialized with context-aware code intelligence."
                    .to_string(),
            ],
        );

        Self {
            active_channel: "Tasks".to_string(),
            channels,
            channel_buffers,
            auto_scroll: true,
            channel_dropdown_open: false,
        }
    }
}

impl OutputState {
    pub fn log(&mut self, channel: &str, line: impl Into<String>) {
        let entry = self.channel_buffers.entry(channel.to_string()).or_default();
        entry.push(line.into());
        if entry.len() > 10_000 {
            entry.drain(0..1_000);
        }
    }

    pub fn clear_active(&mut self) {
        if let Some(buf) = self.channel_buffers.get_mut(&self.active_channel) {
            buf.clear();
        }
    }

    pub fn active_lines(&self) -> &[String] {
        self.channel_buffers
            .get(&self.active_channel)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }
}

pub struct OutputPanel;

impl Default for OutputPanel {
    fn default() -> Self {
        Self
    }
}

impl BottomPanelTab for OutputPanel {
    fn id(&self) -> &'static str {
        "output"
    }

    fn title(&self) -> &'static str {
        "OUTPUT"
    }

    fn render(&self, state: &HadesNativeState, cx: &mut Context<HadesAppView>) -> AnyElement {
        let theme = &state.theme;
        let active_ch = &state.output.active_channel;
        let is_dropdown_open = state.output.channel_dropdown_open;
        let auto_scroll = state.output.auto_scroll;
        let lines = state.output.active_lines();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.bg_editor)
            // Channel Header Bar
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .h(px(28.0))
                    .px_3()
                    .bg(theme.bg_card)
                    .border_b_1()
                    .border_color(theme.border_subtle)
                    .child(
                        div()
                            .relative()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_subtle)
                                    .child("Channel:"),
                            )
                            // Channel Picker Button
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_1p5()
                                    .px_2()
                                    .py_0p5()
                                    .rounded_sm()
                                    .bg(theme.bg_raised)
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .cursor_pointer()
                                    .hover(|s| s.bg(theme.bg_hover))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _event, _window, cx| {
                                            this.state.output.channel_dropdown_open =
                                                !this.state.output.channel_dropdown_open;
                                            cx.notify();
                                        }),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(theme.text_primary)
                                            .child(active_ch.clone()),
                                    )
                                    .child(icon_12(IconName::ChevronDown, theme.text_subtle)),
                            )
                            // Dropdown Menu
                            .children(is_dropdown_open.then(|| {
                                div()
                                    .absolute()
                                    .top(px(26.0))
                                    .left(px(55.0))
                                    .w(px(180.0))
                                    .p_1()
                                    .rounded(px(4.0))
                                    .bg(theme.bg_raised)
                                    .border_1()
                                    .border_color(theme.border_focus)
                                    .shadow_md()
                                    .children(state.output.channels.iter().map(|ch| {
                                        let ch_name = ch.clone();
                                        let is_sel = &ch_name == active_ch;
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .justify_between()
                                            .px_2()
                                            .py_1()
                                            .rounded_sm()
                                            .cursor_pointer()
                                            .bg(if is_sel {
                                                theme.accent.opacity(0.18)
                                            } else {
                                                theme.bg_raised
                                            })
                                            .hover(|s| s.bg(theme.bg_hover))
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(move |this, _e, _w, cx| {
                                                    this.state.output.active_channel =
                                                        ch_name.clone();
                                                    this.state.output.channel_dropdown_open = false;
                                                    cx.notify();
                                                }),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(if is_sel {
                                                        theme.accent
                                                    } else {
                                                        theme.text_primary
                                                    })
                                                    .font_weight(if is_sel {
                                                        FontWeight::BOLD
                                                    } else {
                                                        FontWeight::NORMAL
                                                    })
                                                    .child(ch.clone()),
                                            )
                                            .children(
                                                is_sel.then(|| {
                                                    icon_12(IconName::Check, theme.accent)
                                                }),
                                            )
                                    }))
                            })),
                    )
                    // Toolbar Right Controls: Auto-Scroll, Copy, Clear
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2()
                            // Auto-scroll toggle
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_1()
                                    .px_1p5()
                                    .py_0p5()
                                    .rounded_sm()
                                    .cursor_pointer()
                                    .bg(if auto_scroll {
                                        theme.accent.opacity(0.15)
                                    } else {
                                        theme.bg_card
                                    })
                                    .border_1()
                                    .border_color(if auto_scroll {
                                        theme.accent.opacity(0.4)
                                    } else {
                                        theme.border_subtle
                                    })
                                    .hover(|s| s.bg(theme.bg_hover))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _e, _w, cx| {
                                            this.state.output.auto_scroll =
                                                !this.state.output.auto_scroll;
                                            cx.notify();
                                        }),
                                    )
                                    .child(ui_icon(
                                        IconName::ChevronsDown,
                                        11.0,
                                        if auto_scroll {
                                            theme.accent
                                        } else {
                                            theme.text_subtle
                                        },
                                    ))
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(if auto_scroll {
                                                theme.accent
                                            } else {
                                                theme.text_subtle
                                            })
                                            .child("Scroll Lock"),
                                    ),
                            )
                            // Copy Output Button
                            .child(
                                div()
                                    .p_1()
                                    .rounded_sm()
                                    .cursor_pointer()
                                    .hover(|s| s.bg(theme.bg_hover))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _e, _w, cx| {
                                            let text = this.state.output.active_lines().join("\n");
                                            cx.write_to_clipboard(ClipboardItem::new_string(text));
                                            this.state
                                                .toast_manager
                                                .push_info("Output copied to clipboard");
                                            cx.notify();
                                        }),
                                    )
                                    .child(icon_14(IconName::Copy, theme.text_muted)),
                            )
                            // Clear Button
                            .child(
                                div()
                                    .p_1()
                                    .rounded_sm()
                                    .cursor_pointer()
                                    .hover(|s| s.bg(theme.bg_hover))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _e, _w, cx| {
                                            this.state.output.clear_active();
                                            cx.notify();
                                        }),
                                    )
                                    .child(icon_14(IconName::Trash, theme.text_muted)),
                            ),
                    ),
            )
            // Log Lines Scroll Area
            .child(
                div()
                    .id("output_lines_scroll")
                    .flex()
                    .flex_col()
                    .flex_1()
                    .overflow_y_scroll()
                    .p_2()
                    .font_family("Cascadia Code, Consolas, monospace")
                    .text_xs()
                    .children(if lines.is_empty() {
                        vec![div()
                            .py_4()
                            .flex()
                            .justify_center()
                            .text_color(theme.text_subtle)
                            .child("No output has been recorded for this channel.")
                            .into_any_element()]
                    } else {
                        lines
                            .iter()
                            .map(|line| {
                                let text_col = if line.contains("error")
                                    || line.contains("Error")
                                    || line.contains("FAIL")
                                {
                                    theme.status_red
                                } else if line.contains("warn") || line.contains("Warning") {
                                    theme.status_yellow
                                } else if line.contains("ready")
                                    || line.contains("Active")
                                    || line.contains("PASS")
                                    || line.contains("120Hz")
                                {
                                    theme.status_green
                                } else {
                                    theme.text_primary
                                };

                                div()
                                    .py_0p5()
                                    .text_color(text_col)
                                    .child(line.clone())
                                    .into_any_element()
                            })
                            .collect()
                    }),
            )
            .into_any_element()
    }
}
