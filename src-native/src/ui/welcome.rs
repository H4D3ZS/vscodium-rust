use crate::app_state::{FocusedPanel, HadesNativeState};
use crate::ui::components::drop_shadow;
use crate::ui::icons::{get_app_icon, icon_12, icon_14, ui_icon, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;

pub fn render_branded_welcome_screen(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;

    div()
        .id("welcome_screen_scroll")
        .flex()
        .flex_col()
        .items_center()
        .justify_start()
        .size_full()
        .bg(theme.bg_editor)
        .overflow_y_scroll()
        .p_8()
        .child(
            div()
                .flex()
                .flex_col()
                .items_center()
                .max_w(px(880.0))
                .w_full()
                .gap_6()
                // ── Header: Logo, Title, Version Pill, Tagline ──
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap_3()
                        // Authentic App Logo Badge
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .w(px(64.0))
                                .h(px(64.0))
                                .rounded_full()
                                .bg(rgb(0x18181b))
                                .border_2()
                                .border_color(theme.border_subtle)
                                .shadow(vec![
                                    BoxShadow::new(px(0.), px(6.), theme.shadow_strong.into())
                                        .blur_radius(px(18.)),
                                    BoxShadow::new(px(0.), px(0.), rgb(0x007acc).into())
                                        .blur_radius(px(16.)),
                                ])
                                .child(
                                    img(get_app_icon())
                                        .w(px(48.0))
                                        .h(px(48.0)),
                                ),
                        )
                        // Title
                        .child(
                            div()
                                .text_xl()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xffffff))
                                .child("VSCODIUM-RUST IDE")
                                          // Subtitle Tagline
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child("High-Performance GPU-Accelerated Code Editor & Native AI Development Environment")
                        )
                        // Quick Action Link (exact TypeScript UI parity)
                        .child(
                            div()
                                .mt_1()
                                .cursor_pointer()
                                .text_xs()
                                .text_color(rgb(0x38bdf8))
                                .hover(|s| s.underline())
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _e, _w, cx| {
                                    this.state.right_sidebar_open = true;
                                    this.state.active_right_tab = crate::app_state::RightSidebarTab::AgentChat;
                                    this.state.focused_panel = crate::app_state::FocusedPanel::Composer;
                                    cx.notify();
                                }))
                                .child("Code with Agent · Ctrl+L")
                        )
                )
                // ── AI Architecture Pillars (2x2 Grid) ──
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_2p5()
                        .w_full()
                        // Row 1
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .gap_2p5()
                                .w_full()
                                .child(
                                    div()
                                        .flex_1()
                                        .p_3()
                                        .rounded(px(8.0))
                                        .bg(theme.bg_glass_card)
                                        .border_1()
                                        .border_color(theme.border_glass)
                                        .shadow(drop_shadow(theme.shadow_soft))
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap_1p5()
                                                .mb_1()
                                                .child(ui_icon(IconName::Cpu, 14.0, theme.accent))
                                                .child(div().text_xs().font_weight(FontWeight::BOLD).text_color(theme.text_primary).child("Local-first"))
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.text_muted)
                                                .child("Local models, your API keys, your checkpoints — privacy by default.")
                                        )
                                )
                                .child(
                                    div()
                                        .flex_1()
                                        .p_3()
                                        .rounded(px(8.0))
                                        .bg(theme.bg_glass_card)
                                        .border_1()
                                        .border_color(theme.border_glass)
                                        .shadow(drop_shadow(theme.shadow_soft))
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap_1p5()
                                                .mb_1()
                                                .child(ui_icon(IconName::Flame, 14.0, rgb(0xf59e0b)))
                                                .child(div().text_xs().font_weight(FontWeight::BOLD).text_color(theme.text_primary).child("PyTorch ML Studio"))
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.text_muted)
                                                .child("Datasets, training runs, hyperparameters, and GPU resource monitoring.")
                                        )
                                )
                        )
                        // Row 2
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .gap_2p5()
                                .w_full()
                                .child(
                                    div()
                                        .flex_1()
                                        .p_3()
                                        .rounded(px(8.0))
                                        .bg(theme.bg_glass_card)
                                        .border_1()
                                        .border_color(theme.border_glass)
                                        .shadow(drop_shadow(theme.shadow_soft))
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap_1p5()
                                                .mb_1()
                                                .child(ui_icon(IconName::Bot, 14.0, rgb(0x38bdf8)))
                                                .child(div().text_xs().font_weight(FontWeight::BOLD).text_color(theme.text_primary).child("Agent-native"))
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.text_muted)
                                                .child("42+ tools, shadow VFS verification, MCP servers, and skills.")
                                        )
                                )
                                .child(
                                    div()
                                        .flex_1()
                                        .p_3()
                                        .rounded(px(8.0))
                                        .bg(theme.bg_glass_card)
                                        .border_1()
                                        .border_color(theme.border_glass)
                                        .shadow(drop_shadow(theme.shadow_soft))
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap_1p5()
                                                .mb_1()
                                                .child(ui_icon(IconName::Terminal, 14.0, rgb(0x4ade80)))
                                                .child(div().text_xs().font_weight(FontWeight::BOLD).text_color(theme.text_primary).child("Pure Rust GPUI"))
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.text_muted)
                                                .child("Native GPU rendering, zero webview latency, memory under 90 MB.")
                                        )
                                )
                        )
                )
            )
                // ── Two Columns: Get Started vs Recent Workspaces ──
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .gap_6()
                        .w_full()
                        // Left Column: Get Started
                        .child(
                            div()
                                .flex_1()
                                .flex()
                                .flex_col()
                                .gap_2()
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_muted)
                                        .mb_1()
                                        .child("GET STARTED")
                                )
                                .child(
                                    render_welcome_action_row(
                                        IconName::Plus,
                                        theme.accent,
                                        "New File...",
                                        "Create file in active workspace",
                                        "Ctrl+N",
                                        theme,
                                        cx.listener(|this, _event, window, cx| {
                                            let id = chrono::Local::now().timestamp_subsec_millis();
                                            let filename = format!("untitled_{id}.rs");
                                            this.state.create_new_file(&filename);
                                            let full = this.state.workspace_root.join(&filename);
                                            this.open_file(&full.to_string_lossy(), window, cx);
                                            cx.notify();
                                        })
                                    )
                                )
                                .child(
                                    render_welcome_action_row(
                                        IconName::Folder,
                                        rgb(0xf59e0b),
                                        "Open Folder...",
                                        "Open folder from filesystem",
                                        "Ctrl+K Ctrl+O",
                                        theme,
                                        cx.listener(|this, _event, window, cx| {
                                            this.prompt_open_folder(window, cx);
                                        })
                                    )
                                )
                                .child(
                                    render_welcome_action_row(
                                        IconName::File,
                                        rgb(0x38bdf8),
                                        "Open File...",
                                        "Open individual file in editor",
                                        "Ctrl+O",
                                        theme,
                                        cx.listener(|this, _event, window, cx| {
                                            this.prompt_open_file(window, cx);
                                        })
                                    )
                                )
                                .child(
                                    render_welcome_action_row(
                                        IconName::Sparkles,
                                        rgb(0xec4899),
                                        "New AI Project...",
                                        "Specs-to-Code Autonomous Pipeline",
                                        "Specs",
                                        theme,
                                        cx.listener(|this, _event, _window, cx| {
                                            this.state.active_right_tab = crate::app_state::RightSidebarTab::Specs;
                                            this.state.right_sidebar_open = true;
                                            cx.notify();
                                        })
                                    )
                                )
                                .child(
                                    render_welcome_action_row(
                                        IconName::Cpu,
                                        rgb(0xa78bfa),
                                        "PyTorch ML Studio",
                                        "Train, fine-tune, and monitor models",
                                        "ML Studio",
                                        theme,
                                        cx.listener(|this, _event, _window, cx| {
                                            this.state.set_activity(crate::app_state::ActivityTab::Settings);
                                            this.state.left_sidebar_open = true;
                                            this.state.toast_manager.push_info("Opening PyTorch Studio...");
                                            cx.notify();
                                        })
                                    )
                                )
                                .child(
                                    render_welcome_action_row(
                                        IconName::Bot,
                                        rgb(0x38bdf8),
                                        "Code with AI Agent",
                                        "Local Qwen 3.8 27B · Multi-File Agentic Coding",
                                        "Ctrl+L",
                                        theme,
                                        cx.listener(|this, _event, _window, cx| {
                                            this.state.right_sidebar_open = true;
                                            this.state.active_right_tab = crate::app_state::RightSidebarTab::AgentChat;
                                            this.state.focused_panel = FocusedPanel::Composer;
                                            cx.notify();
                                        })
                                    )
                                )
                        )
                        // Right Column: Recent Workspaces & Project Navigation
                        .child(
                            div()
                                .flex_1()
                                .flex()
                                .flex_col()
                                .gap_2()
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_muted)
                                        .mb_1()
                                        .child("RECENT WORKSPACES")
                                )
                                .children(if state.recent_workspaces.is_empty() {
                                    vec![
                                        render_welcome_action_row(
                                            IconName::Folder,
                                            rgb(0x94a3b8),
                                            "No recent workspaces",
                                            "Click 'Open Folder...' to open a workspace",
                                            "Start",
                                            theme,
                                            cx.listener(|this, _event, window, cx| {
                                                this.prompt_open_folder(window, cx);
                                            })
                                        ).into_any_element(),
                                        render_welcome_action_row(
                                            IconName::Terminal,
                                            rgb(0x81c784),
                                            "Integrated Terminal",
                                            "Direct3D 12 Windows ConPTY",
                                            "Ctrl+`",
                                            theme,
                                            cx.listener(|this, _event, _window, cx| {
                                                this.state.toggle_bottom_panel();
                                                cx.notify();
                                            })
                                        ).into_any_element(),
                                    ]
                                } else {
                                    state.recent_workspaces.iter().take(5).map(|ws_path| {
                                        let p_clone = ws_path.clone();
                                        let name = p_clone
                                            .file_name()
                                            .map(|s| s.to_string_lossy().to_string())
                                            .unwrap_or_else(|| p_clone.to_string_lossy().to_string());
                                        let display_path = p_clone.to_string_lossy().to_string();
                                        render_welcome_action_row(
                                            IconName::Folder,
                                            rgb(0xf59e0b),
                                            &name,
                                            &display_path,
                                            "Workspace",
                                            theme,
                                            cx.listener(move |this, _event, window, cx| {
                                                this.open_workspace(p_clone.clone(), window, cx);
                                            })
                                        ).into_any_element()
                                    }).collect()
                                })
                        )
                )
                // ── Footer: Startup Checkbox & Shortcuts ──
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .w_full()
                        .pt_4()
                        .border_t_1()
                        .border_color(theme.border_subtle)
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .child(
                                    div()
                                        .w(px(14.0))
                                        .h(px(14.0))
                                        .rounded(px(3.0))
                                        .bg(theme.accent)
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .child(icon_12(IconName::Check, rgb(0x000000)))
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .child("Show welcome page on startup")
                                )
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_4()
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_subtle)
                                        .child("Quick Open: Ctrl+P")
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_subtle)
                                        .child("Command Palette: Ctrl+Shift+P")
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_subtle)
                                        .child("Keybindings: Ctrl+K Ctrl+S")
                                )
                        )
                )
        )
}

fn render_welcome_action_row(
    icon: IconName,
    icon_color: impl Into<Rgba>,
    title: &str,
    desc: &str,
    badge: &str,
    theme: &crate::theme::Theme,
    handler: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let title_owned = title.to_string();
    let desc_owned = desc.to_string();
    let badge_owned = badge.to_string();

    div()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .px_3()
        .py_2()
        .rounded(px(6.0))
        .bg(theme.bg_glass_card)
        .border_1()
        .border_color(theme.border_glass)
        .shadow(drop_shadow(theme.shadow_soft))
        .cursor_pointer()
        .hover(|s| {
            s.bg(theme.bg_hover)
                .border_color(theme.ring_focus)
        })
        .on_mouse_down(MouseButton::Left, handler)
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap_2p5()
                .child(ui_icon(icon, 16.0, icon_color.into()))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.text_primary)
                                .child(title_owned),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_subtle)
                                .child(desc_owned),
                        ),
                ),
        )
        .child(
            div()
                .px_1p5()
                .py_0p5()
                .rounded(px(3.0))
                .bg(theme.bg_app)
                .border_1()
                .border_color(theme.border_subtle)
                .text_xs()
                .text_color(theme.text_subtle)
                .child(badge_owned),
        )
}
