use crate::app_state::{HadesNativeState, RightSidebarTab};
use crate::ui::icons::{icon_12, icon_14, ui_icon, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;
use std::sync::Mutex;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StudioSubTab {
    Dashboard,
    Agents,
    WebAgent,
    Specs,
    Tasks,
    Steering,
    Rules,
    Session,
}

impl StudioSubTab {
    fn label(&self) -> &'static str {
        match self {
            StudioSubTab::Dashboard => "Dashboard",
            StudioSubTab::Agents => "Agents",
            StudioSubTab::WebAgent => "Web Agent",
            StudioSubTab::Specs => "Specs",
            StudioSubTab::Tasks => "Tasks",
            StudioSubTab::Steering => "Steering",
            StudioSubTab::Rules => "Rules",
            StudioSubTab::Session => "Session",
        }
    }

    fn icon(&self) -> IconName {
        match self {
            StudioSubTab::Dashboard => IconName::LayoutDashboard,
            StudioSubTab::Agents => IconName::Cpu,
            StudioSubTab::WebAgent => IconName::Globe,
            StudioSubTab::Specs => IconName::Book,
            StudioSubTab::Tasks => IconName::ListChecks,
            StudioSubTab::Steering => IconName::Sparkles,
            StudioSubTab::Rules => IconName::Scale,
            StudioSubTab::Session => IconName::Notebook,
        }
    }

    const ALL: [StudioSubTab; 8] = [
        StudioSubTab::Dashboard,
        StudioSubTab::Agents,
        StudioSubTab::WebAgent,
        StudioSubTab::Specs,
        StudioSubTab::Tasks,
        StudioSubTab::Steering,
        StudioSubTab::Rules,
        StudioSubTab::Session,
    ];
}

static STUDIO_SUB_TAB: std::sync::OnceLock<Mutex<StudioSubTab>> = std::sync::OnceLock::new();

fn studio_sub_tab() -> StudioSubTab {
    *STUDIO_SUB_TAB
        .get_or_init(|| Mutex::new(StudioSubTab::Dashboard))
        .lock()
        .unwrap()
}

fn set_studio_sub_tab(tab: StudioSubTab) {
    if let Ok(mut slot) = STUDIO_SUB_TAB
        .get_or_init(|| Mutex::new(StudioSubTab::Dashboard))
        .lock()
    {
        *slot = tab;
    }
}

pub fn render_right_sidebar_panel(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> AnyElement {
    let theme = &state.theme;
    let active_tab = state.active_right_tab;

    div()
        .flex()
        .flex_col()
        .size_full()
        .bg(theme.bg_sidebar)
        // ── Header Row 1: Session Tabs (Chat 1 x, +, History shortcut, Close) ──
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .h(px(32.0))
                .px_2()
                .bg(theme.bg_titlebar)
                .border_b_1()
                .border_color(theme.border_subtle)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1p5()
                        .children(state.agent_threads.iter().enumerate().map(|(idx, thread)| {
                            let is_current = idx == state.active_thread_idx;
                            let title = thread.title.clone();
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1p5()
                                .px_2p5()
                                .py_0p5()
                                .rounded(px(4.0))
                                .cursor_pointer()
                                .bg(if is_current { theme.bg_raised } else { theme.bg_titlebar })
                                .border_1()
                                .border_color(if is_current { theme.border_focus } else { theme.border_subtle })
                                .text_xs()
                                .text_color(if is_current { theme.text_primary } else { theme.text_muted })
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                    this.state.select_agent_thread(idx);
                                    this.state.active_right_tab = RightSidebarTab::AgentChat;
                                    cx.notify();
                                }))
                                .child(div().child(title))
                                .child(
                                    div()
                                        .p_0p5()
                                        .rounded(px(2.0))
                                        .hover(|s| s.bg(theme.bg_hover))
                                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                            if this.state.agent_threads.len() > 1 {
                                                this.state.agent_threads.remove(idx);
                                                if this.state.active_thread_idx >= this.state.agent_threads.len() {
                                                    this.state.active_thread_idx = this.state.agent_threads.len() - 1;
                                                }
                                            } else {
                                                this.state.clear_chat();
                                            }
                                            cx.notify();
                                        }))
                                        .child(icon_12(IconName::X, theme.text_subtle))
                                )
                        }))
                        .children((state.agent_threads.is_empty()).then(|| {
                            div().text_xs().text_color(theme.text_subtle).px_2().child("New Chat")
                        }))
                        .child(
                            div()
                                .p_1()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                    this.state.create_agent_thread();
                                    this.state.active_right_tab = RightSidebarTab::AgentChat;
                                    cx.notify();
                                }))
                                .child(icon_14(IconName::Plus, theme.text_subtle))
                        )
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1p5()
                        .child(
                            div()
                                .p_1()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                    this.state.active_right_tab = RightSidebarTab::History;
                                    cx.notify();
                                }))
                                .child(icon_14(IconName::RefreshCw, theme.text_subtle))
                        )
                        .child(
                            div()
                                .p_1()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                    this.state.right_sidebar_open = false;
                                    cx.notify();
                                }))
                                .child(icon_12(IconName::X, theme.text_subtle))
                        )
                )
        )
        // ── Header Row 2: Sub-navigation (CHAT, STUDIO, DEVICES, KORTEX, HISTORY) + Right Utilities ──
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .h(px(28.0))
                .px_2()
                .bg(theme.bg_sidebar)
                .border_b_1()
                .border_color(theme.border_subtle)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_3()
                        .children([
                            (RightSidebarTab::AgentChat, "CHAT"),
                            (RightSidebarTab::AgentStudio, "STUDIO"),
                            (RightSidebarTab::Mobile, "DEVICES"),
                            (RightSidebarTab::Kortex, "KORTEX"),
                            (RightSidebarTab::VectorSearch, "VECTOR"),
                            (RightSidebarTab::History, "HISTORY"),
                        ].into_iter().map(|(tab, label)| {
                            let is_sel = if tab == RightSidebarTab::Mobile {
                                state.iphone_preview_open
                            } else {
                                active_tab == tab
                            };
                            div()
                                .cursor_pointer()
                                .text_xs()
                                .font_weight(if is_sel { FontWeight::BOLD } else { FontWeight::NORMAL })
                                .text_color(if is_sel { theme.text_primary } else { theme.text_subtle })
                                .border_b_2()
                                .border_color(if is_sel { theme.accent } else { theme.bg_sidebar })
                                .pb_0p5()
                                .hover(|s| s.text_color(theme.text_primary))
                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                    if tab == RightSidebarTab::Mobile {
                                        this.state.iphone_preview_open = !this.state.iphone_preview_open;
                                    } else {
                                        this.state.active_right_tab = tab;
                                    }
                                    cx.notify();
                                }))
                                .child(label)
                        }))
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        // AIRI eye toggle
                        .child(
                            div()
                                .cursor_pointer()
                                .hover(|s| s.text_color(theme.text_primary))
                                .child(ui_icon(IconName::Eye, 12.0, theme.text_subtle))
                        )
                        // Agent MCP Menu
                        .child(
                            div()
                                .cursor_pointer()
                                .hover(|s| s.text_color(theme.text_primary))
                                .child(ui_icon(IconName::Cpu, 12.0, theme.text_subtle))
                        )
                        // Settings Gear
                        .child(
                            div()
                                .cursor_pointer()
                                .hover(|s| s.text_color(theme.text_primary))
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                    this.state.set_activity(crate::app_state::ActivityTab::Settings);
                                    this.state.left_sidebar_open = true;
                                    cx.notify();
                                }))
                                .child(ui_icon(IconName::Settings, 12.0, theme.text_subtle))
                        )
                        // Command Help Question mark (blue)
                        .child(
                            div()
                                .cursor_pointer()
                                .hover(|s| s.opacity(0.8))
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                    this.state.toast_manager.push_info("Command Reference: /doctor, /help, /tools, /diff, /commit, /compact");
                                    cx.notify();
                                }))
                                .child(ui_icon(IconName::Info, 12.0, rgb(0x3b82f6)))
                        )
                        // Pop-out / Toggle iPhone Preview Side-by-Side
                        .child(
                            div()
                                .cursor_pointer()
                                .hover(|s| s.text_color(theme.text_primary))
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                    this.state.iphone_preview_open = !this.state.iphone_preview_open;
                                    cx.notify();
                                }))
                                .child(ui_icon(
                                    IconName::Smartphone,
                                    12.0,
                                    if state.iphone_preview_open {
                                        theme.accent
                                    } else {
                                        theme.text_subtle
                                    },
                                ))
                        )
                        // Close sidebar
                        .child(
                            div()
                                .cursor_pointer()
                                .hover(|s| s.text_color(theme.text_primary))
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                    this.state.right_sidebar_open = false;
                                    cx.notify();
                                }))
                                .child(ui_icon(IconName::X, 12.0, theme.text_subtle))
                        )
                )
        )
        // ── Body: Switch between Chat, Studio, Devices, Kortex, and History ──
        .child(
            div()
                .flex()
                .flex_col()
                .flex_1()
                .overflow_hidden()
                .child(match active_tab {
                    RightSidebarTab::AgentChat => crate::panels::agent_chat::render_agent_chat_body(state, cx).into_any_element(),
                    RightSidebarTab::AgentStudio => render_agent_studio_workflows(state, cx).into_any_element(),
                    RightSidebarTab::Mobile => crate::panels::render_mobile_panel(state, cx).into_any_element(),
                    RightSidebarTab::Kortex => crate::panels::render_kortex_panel(state, cx).into_any_element(),
                    RightSidebarTab::VectorSearch => crate::panels::vector_search::render_vector_search_panel(state, cx).into_any_element(),
                    RightSidebarTab::History => render_history_panel(state, cx).into_any_element(),
                    RightSidebarTab::Specs => crate::panels::render_specs_panel(state, cx).into_any_element(),
                    RightSidebarTab::Rules => crate::panels::render_rules_panel(state, cx).into_any_element(),
                })
        )
        .into_any_element()
}

fn render_history_panel(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;

    div()
        .flex()
        .flex_col()
        .size_full()
        .p_3()
        .gap_3()
        .overflow_hidden()
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_0p5()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child("CONVERSATION HISTORY"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_subtle)
                                .child("Click any session to restore chat in panel."),
                        ),
                )
                .child(
                    div()
                        .px_2()
                        .py_1()
                        .rounded(px(4.0))
                        .cursor_pointer()
                        .bg(theme.bg_raised)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .hover(|s| s.bg(theme.bg_hover))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                this.state.create_agent_thread();
                                this.state.active_right_tab = RightSidebarTab::AgentChat;
                                cx.notify();
                            }),
                        )
                        .text_xs()
                        .text_color(theme.text_primary)
                        .child("Archive current"),
                ),
        )
        // Sessions list
        .child(
            div()
                .id("agent_studio_history_sessions_scroll")
                .flex()
                .flex_col()
                .flex_1()
                .gap_2()
                .overflow_y_scroll()
                .children(state.agent_threads.iter().enumerate().map(|(idx, thread)| {
                    let is_current = idx == state.active_thread_idx;
                    div()
                        .flex()
                        .flex_col()
                        .p_2p5()
                        .rounded(px(8.0))
                        .cursor_pointer()
                        .bg(if is_current {
                            rgba(0x007acc18)
                        } else {
                            theme.bg_raised
                        })
                        .border_1()
                        .border_color(if is_current {
                            rgba(0x007acc55)
                        } else {
                            theme.border_subtle
                        })
                        .hover(|s| s.bg(theme.bg_hover))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _event, _window, cx| {
                                this.state.select_agent_thread(idx);
                                this.state.active_right_tab = RightSidebarTab::AgentChat;
                                cx.notify();
                            }),
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
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_primary)
                                        .child(thread.title.clone()),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_1p5()
                                        .children(is_current.then(|| {
                                            div()
                                                .px_1p5()
                                                .py_0p5()
                                                .rounded(px(3.0))
                                                .bg(rgba(0x007acc33))
                                                .text_xs()
                                                .text_color(rgb(0x7ec8ff))
                                                .child("Live")
                                        }))
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.text_subtle)
                                                .child(format!("{} msgs", thread.messages.len())),
                                        ),
                                ),
                        )
                        .child(
                            div().text_xs().text_color(theme.text_muted).mt_1().child(
                                thread
                                    .messages
                                    .last()
                                    .map(|m| {
                                        if m.text.len() > 70 {
                                            format!("{}...", &m.text[..70])
                                        } else {
                                            m.text.clone()
                                        }
                                    })
                                    .unwrap_or_else(|| {
                                        "Fresh session ready for prompt".to_string()
                                    }),
                            ),
                        )
                })),
        )
}

fn render_agent_studio_workflows(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;
    let active = studio_sub_tab();

    div()
        .flex()
        .flex_col()
        .size_full()
        .bg(theme.bg_sidebar)
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap_1p5()
                .flex_wrap()
                .px_2()
                .py_1p5()
                .bg(theme.bg_titlebar)
                .border_b_1()
                .border_color(theme.border_subtle)
                .children(StudioSubTab::ALL.iter().map(|tab| {
                    let is_sel = active == *tab;
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .px_2()
                        .py_0p5()
                        .rounded(px(4.0))
                        .cursor_pointer()
                        .bg(if is_sel {
                            theme.bg_raised
                        } else {
                            theme.bg_sidebar
                        })
                        .border_1()
                        .border_color(if is_sel {
                            theme.accent
                        } else {
                            theme.border_subtle
                        })
                        .text_xs()
                        .font_weight(if is_sel {
                            FontWeight::BOLD
                        } else {
                            FontWeight::NORMAL
                        })
                        .text_color(if is_sel {
                            theme.text_primary
                        } else {
                            theme.text_muted
                        })
                        .hover(|s| s.bg(theme.bg_hover))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener({
                                let tab = *tab;
                                move |_this, _event, _window, cx| {
                                    set_studio_sub_tab(tab);
                                    cx.notify();
                                }
                            }),
                        )
                        .child(ui_icon(
                            tab.icon(),
                            11.0,
                            if is_sel {
                                theme.accent
                            } else {
                                theme.text_muted
                            },
                        ))
                        .child(tab.label())
                })),
        )
        .child(
            div()
                .id("agent_studio_workflows_scroll")
                .flex()
                .flex_col()
                .flex_1()
                .overflow_y_scroll()
                .p_3()
                .gap_3()
                .child(match active {
                    StudioSubTab::Dashboard => {
                        render_studio_dashboard(state, cx).into_any_element()
                    }
                    StudioSubTab::Agents => render_studio_agents(state, cx).into_any_element(),
                    StudioSubTab::WebAgent => render_studio_web_agent(state, cx).into_any_element(),
                    StudioSubTab::Specs => {
                        crate::panels::render_specs_panel(state, cx).into_any_element()
                    }
                    StudioSubTab::Tasks => render_studio_tasks(state, cx).into_any_element(),
                    StudioSubTab::Steering => render_studio_steering(state, cx).into_any_element(),
                    StudioSubTab::Rules => {
                        crate::panels::render_rules_panel(state, cx).into_any_element()
                    }
                    StudioSubTab::Session => render_studio_session(state, cx).into_any_element(),
                }),
        )
        .into_any_element()
}

fn render_studio_dashboard(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;
    let total_messages: usize = state.agent_threads.iter().map(|t| t.messages.len()).sum();

    div()
        .flex()
        .flex_col()
        .gap_3()
        .child(
            div()
                .flex()
                .flex_row()
                .gap_2()
                .flex_wrap()
                .child(studio_stat_card(theme, "Runtime", &state.agent_model))
                .child(studio_stat_card(theme, "Mode", &state.agent_mode))
                .child(studio_stat_card(theme, "Threads", &state.agent_threads.len().to_string()))
                .child(studio_stat_card(theme, "Messages", &total_messages.to_string()))
        )
        .child(
            div()
                .flex()
                .flex_row()
                .gap_2()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .px_2()
                        .py_1()
                        .rounded(px(4.0))
                        .bg(if state.is_yolo_mode { rgba(0x22c55e18) } else { theme.bg_raised })
                        .border_1()
                        .border_color(if state.is_yolo_mode { rgba(0x22c55e44) } else { theme.border_subtle })
                        .text_xs()
                        .text_color(if state.is_yolo_mode { rgb(0x4ade80) } else { theme.text_muted })
                        .child("Auto-Approve Edits")
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .px_2()
                        .py_1()
                        .rounded(px(4.0))
                        .bg(if state.is_continuous_mode { rgba(0x388bfd18) } else { theme.bg_raised })
                        .border_1()
                        .border_color(if state.is_continuous_mode { rgba(0x388bfd44) } else { theme.border_subtle })
                        .text_xs()
                        .text_color(if state.is_continuous_mode { theme.accent } else { theme.text_muted })
                        .child("Continuous Loop")
                )
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .children([
                    ("Deep Research", "Multi-step web extraction, documentation synthesis, and architecture planning", IconName::Search),
                    ("Refactor Engine", "AST-aware semantic code transformation across multi-file dependencies", IconName::FileCode),
                    ("Browser Automation", "Automated browser validation, screenshots, and visual regression tests", IconName::Globe),
                ].into_iter().map(|(name, desc, icon)| {
                    div()
                        .flex()
                        .flex_col()
                        .p_2p5()
                        .rounded(px(6.0))
                        .bg(theme.bg_card)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .gap_1()
                        .hover(|s| s.border_color(theme.border_focus))
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
                                        .gap_2()
                                        .child(ui_icon(icon, 13.0, theme.accent))
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(theme.text_primary)
                                                .child(name)
                                        )
                                )
                                .child(
                                    div()
                                        .px_2()
                                        .py_0p5()
                                        .rounded(px(3.0))
                                        .bg(theme.accent)
                                        .text_xs()
                                        .text_color(theme.text_on_accent)
                                        .cursor_pointer()
                                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                            this.state.send_ai_prompt(&format!("Launch workflow: {name}"));
                                            cx.notify();
                                        }))
                                        .child("Run")
                                )
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child(desc)
                        )
                }))
        )
}

fn studio_stat_card(theme: &crate::theme::Theme, label: &str, value: &str) -> impl IntoElement {
    let label = label.to_string();
    let value = value.to_string();
    div()
        .flex()
        .flex_col()
        .gap_0p5()
        .px_2p5()
        .py_1p5()
        .rounded(px(6.0))
        .bg(theme.bg_card)
        .border_1()
        .border_color(theme.border_subtle)
        .flex_grow(1.0)
        .child(div().text_xs().text_color(theme.text_muted).child(label))
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::BOLD)
                .truncate()
                .text_color(theme.text_primary)
                .child(value),
        )
}

fn render_studio_agents(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;

    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.text_primary)
                        .child("PARALLEL BACKGROUND AGENTS"),
                )
                .child(
                    div()
                        .px_2()
                        .py_0p5()
                        .rounded(px(4.0))
                        .bg(theme.accent)
                        .cursor_pointer()
                        .hover(|s| s.opacity(0.85))
                        .text_xs()
                        .text_color(theme.text_on_accent)
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                this.state.create_agent_thread();
                                this.state
                                    .toast_manager
                                    .push_success("Spawned background agent thread");
                                cx.notify();
                            }),
                        )
                        .child("Spawn"),
                ),
        )
        .children(state.agent_threads.iter().enumerate().map(|(idx, t)| {
            let active = idx == state.active_thread_idx;
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap_2()
                .p_2p5()
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(if active {
                    theme.accent
                } else {
                    theme.border_subtle
                })
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_0p5()
                        .flex_1()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child(t.title.clone()),
                        )
                        .child(div().text_xs().text_color(theme.text_muted).child(format!(
                            "{} · {} msgs",
                            t.created_at,
                            t.messages.len()
                        ))),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .px_1p5()
                        .py_0p5()
                        .rounded_full()
                        .bg(if active {
                            theme.status_green.opacity(0.2)
                        } else {
                            theme.bg_raised
                        })
                        .text_xs()
                        .text_color(if active {
                            theme.status_green
                        } else {
                            theme.text_muted
                        })
                        .child(if active { "live" } else { "idle" }),
                )
                .child(
                    div()
                        .px_2()
                        .py_0p5()
                        .rounded(px(3.0))
                        .bg(theme.bg_raised)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .cursor_pointer()
                        .hover(|s| s.bg(theme.bg_hover))
                        .text_xs()
                        .text_color(theme.text_primary)
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _event, _window, cx| {
                                this.state.select_agent_thread(idx);
                                this.state.active_right_tab = RightSidebarTab::AgentChat;
                                cx.notify();
                            }),
                        )
                        .child("Open"),
                )
                .into_any_element()
        }))
}

fn render_studio_web_agent(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;
    const PIPELINE: [&str; 8] = [
        "Search",
        "Open browser",
        "Navigate",
        "Scrape",
        "Audit",
        "Terminal",
        "Summarize",
        "findings.md",
    ];

    div()
        .flex()
        .flex_col()
        .gap_3()
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
                        .child("Full Web Mission"),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .child("invisible_playwright stealth Firefox · search · scrape · audit · terminal · findings"),
                )
                .child(
                    div()
                        .px_3()
                        .py_1p5()
                        .rounded(px(4.0))
                        .bg(theme.accent)
                        .cursor_pointer()
                        .hover(|s| s.opacity(0.85))
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.text_on_accent)
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                            this.state.send_ai_prompt("Run full web mission: research the open target and write findings.md");
                            cx.notify();
                        }))
                        .child("Run full web mission"),
                )
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
                        .child("Pipeline"),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .gap_1()
                        .children(PIPELINE.iter().enumerate().map(|(i, step)| {
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .px_1p5()
                                .py_0p5()
                                .rounded(px(4.0))
                                .bg(theme.bg_raised)
                                .text_xs()
                                .text_color(theme.text_primary)
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.accent)
                                        .child(format!("{}", i + 1)),
                                )
                                .child(step.to_string())
                        }))
                )
        )
}

fn render_studio_tasks(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;
    let mut tasks: Vec<(&str, &crate::state::specs::SpecsTask)> = Vec::new();
    for project in &state.specs_projects {
        for task in &project.tasks {
            tasks.push((project.name.as_str(), task));
        }
    }

    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::BOLD)
                .text_color(theme.text_primary)
                .child("EXECUTABLE TASKS"),
        )
        .children(tasks.iter().map(|(project, task)| {
            let done = task.status == "done";
            div()
                .flex()
                .flex_col()
                .gap_1()
                .p_2p5()
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_subtle)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .px_1p5()
                                .py_0p5()
                                .rounded(px(3.0))
                                .bg(if done {
                                    theme.status_green.opacity(0.2)
                                } else {
                                    theme.bg_raised
                                })
                                .text_xs()
                                .text_color(if done {
                                    theme.status_green
                                } else {
                                    theme.text_muted
                                })
                                .child(task.status.clone()),
                        )
                        .child(
                            div()
                                .flex_1()
                                .text_xs()
                                .truncate()
                                .text_color(if done {
                                    theme.text_muted
                                } else {
                                    theme.text_primary
                                })
                                .child(task.title.clone()),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child(format!("{} · {}h", task.complexity, task.estimated_hours)),
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
                                .flex_1()
                                .text_xs()
                                .text_color(theme.text_subtle)
                                .child(project.to_string()),
                        )
                        .child(
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded(px(3.0))
                                .bg(theme.accent)
                                .cursor_pointer()
                                .hover(|s| s.opacity(0.85))
                                .text_xs()
                                .text_color(theme.text_on_accent)
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener({
                                        let title = task.title.clone();
                                        move |this, _event, _window, cx| {
                                            this.state
                                                .send_ai_prompt(&format!("Execute task: {title}"));
                                            cx.notify();
                                        }
                                    }),
                                )
                                .child("Execute with AI"),
                        ),
                )
                .into_any_element()
        }))
}

fn render_studio_steering(
    state: &HadesNativeState,
    _cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;

    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::BOLD)
                .text_color(theme.text_primary)
                .child("GLOBAL STEERING RULES"),
        )
        .child(
            div()
                .p_2()
                .rounded(px(6.0))
                .bg(rgb(0x0e1014))
                .font_family("Cascadia Code, Consolas, monospace")
                .text_xs()
                .text_color(theme.text_muted)
                .child(state.global_rules.clone()),
        )
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::BOLD)
                .text_color(theme.text_primary)
                .child("AUTOMATED AGENT HOOKS"),
        )
        .children(state.agent_hooks.iter().map(|hook| {
            div()
                .flex()
                .flex_col()
                .gap_0p5()
                .p_2p5()
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_subtle)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1p5()
                        .child(
                            div()
                                .px_1p5()
                                .py_0p5()
                                .rounded(px(3.0))
                                .bg(theme.bg_raised)
                                .text_xs()
                                .font_family("Cascadia Code, Consolas, monospace")
                                .text_color(theme.text_primary)
                                .child(hook.pattern.clone()),
                        )
                        .child(
                            div()
                                .px_1p5()
                                .py_0p5()
                                .rounded_full()
                                .bg(if hook.enabled {
                                    theme.status_green.opacity(0.2)
                                } else {
                                    theme.bg_raised
                                })
                                .text_xs()
                                .text_color(if hook.enabled {
                                    theme.status_green
                                } else {
                                    theme.text_muted
                                })
                                .child(if hook.enabled { "enabled" } else { "disabled" }),
                        ),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .child(hook.prompt.clone()),
                )
                .into_any_element()
        }))
        .child(
            div()
                .text_xs()
                .text_color(theme.text_muted)
                .child("Manage steering files in .agent/steering/ and hooks in .agent/hooks."),
        )
}

fn render_studio_session(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;
    const FILES: [&str; 3] = ["task_plan.md", "findings.md", "progress.md"];

    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::BOLD)
                .text_color(theme.text_primary)
                .child("SESSION ARTIFACTS"),
        )
        .children(FILES.iter().map(|f| {
            let path = state.workspace_root.join(".agent").join(f);
            let exists = path.exists();
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap_2()
                .p_2p5()
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_subtle)
                .child(ui_icon(IconName::FileText, 13.0, theme.text_muted))
                .child(
                    div()
                        .flex_1()
                        .text_xs()
                        .font_family("Cascadia Code, Consolas, monospace")
                        .text_color(theme.text_primary)
                        .child(f.to_string()),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(if exists {
                            theme.status_green
                        } else {
                            theme.text_muted
                        })
                        .child(if exists { "exists" } else { "missing" }),
                )
                .child(
                    div()
                        .px_2()
                        .py_0p5()
                        .rounded(px(3.0))
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
                                let path = path.to_string_lossy().to_string();
                                move |this, _event, _window, cx| {
                                    this.state.open_file(&path);
                                    cx.notify();
                                }
                            }),
                        )
                        .child("Open"),
                )
                .into_any_element()
        }))
}

pub struct AgentStudioTab;

impl crate::panels::traits::AuxiliaryTab for AgentStudioTab {
    fn id(&self) -> &'static str {
        "studio"
    }

    fn title(&self) -> &'static str {
        "STUDIO"
    }

    fn icon(&self) -> IconName {
        IconName::Brain
    }

    fn render(&self, state: &HadesNativeState, cx: &mut Context<HadesAppView>) -> AnyElement {
        render_right_sidebar_panel(state, cx).into_any_element()
    }
}
