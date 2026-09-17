use crate::app_state::HadesNativeState;
use crate::ui::icons::{get_app_icon, icon_12, icon_14, ui_icon, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;

pub fn render_titlebar(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;

    div()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .h(px(36.0))
        .w_full()
        .bg(theme.bg_titlebar)
        .border_b_1()
        .border_color(theme.border_subtle)
        .px_3()
        .text_xs()
        .text_color(theme.text_muted)
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(|_this, event: &MouseDownEvent, window, _cx| {
                if event.click_count == 2 {
                    window.zoom_window();
                } else {
                    window.start_window_move();
                }
            }),
        )
        // Left: Vscodium Icon & Standard IDE Menu Items
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap_2()
                // App Logo Icon
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .w(px(20.0))
                        .h(px(20.0))
                        .child(
                            img(get_app_icon())
                                .w(px(18.0))
                                .h(px(18.0)),
                        ),
                )
                // Menus
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_0p5()
                        .child(
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded(px(4.0))
                                .hover(|s| s.bg(theme.bg_hover).text_color(theme.text_primary))
                                .cursor_pointer()
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, event: &MouseDownEvent, _window, cx| {
                                        let x: f32 = event.position.x.into();
                                        let y: f32 = (event.position.y + px(18.0)).into();
                                        this.state.context_menu.show(
                                            x,
                                            y,
                                            vec![
                                                crate::ui::context_menu::ContextMenuItem::action(
                                                    "New File",
                                                )
                                                .with_shortcut("Ctrl+N"),
                                                crate::ui::context_menu::ContextMenuItem::separator(),
                                                crate::ui::context_menu::ContextMenuItem::action(
                                                    "Open File...",
                                                )
                                                .with_shortcut("Ctrl+O"),
                                                crate::ui::context_menu::ContextMenuItem::action(
                                                    "Open Folder...",
                                                )
                                                .with_shortcut("Ctrl+K Ctrl+O"),
                                                crate::ui::context_menu::ContextMenuItem::separator(),
                                                crate::ui::context_menu::ContextMenuItem::action(
                                                    "Save",
                                                )
                                                .with_shortcut("Ctrl+S"),
                                                crate::ui::context_menu::ContextMenuItem::action(
                                                    "Save As...",
                                                )
                                                .with_shortcut("Ctrl+Shift+S"),
                                                crate::ui::context_menu::ContextMenuItem::action(
                                                    "Save All",
                                                )
                                                .with_shortcut("Ctrl+K S"),
                                                crate::ui::context_menu::ContextMenuItem::separator(),
                                                crate::ui::context_menu::ContextMenuItem::action(
                                                    "Close Tab",
                                                )
                                                .with_shortcut("Ctrl+W"),
                                                crate::ui::context_menu::ContextMenuItem::action(
                                                    "Close All Tabs",
                                                ),
                                                crate::ui::context_menu::ContextMenuItem::separator(),
                                                crate::ui::context_menu::ContextMenuItem::action(
                                                    "Exit",
                                                )
                                                .with_shortcut("Alt+F4"),
                                            ],
                                        );
                                        cx.notify();
                                    }),
                                )
                                .child("File"),
                        )
                        .child(
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded(px(4.0))
                                .hover(|s| s.bg(theme.bg_hover).text_color(theme.text_primary))
                                .cursor_pointer()
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.state.toggle_find_bar();
                                        cx.notify();
                                    }),
                                )
                                .child("Edit"),
                        )
                        .child(
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded(px(4.0))
                                .hover(|s| s.bg(theme.bg_hover).text_color(theme.text_primary))
                                .cursor_pointer()
                                .child("Selection"),
                        )
                        .child(
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded(px(4.0))
                                .hover(|s| s.bg(theme.bg_hover).text_color(theme.text_primary))
                                .cursor_pointer()
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, event: &MouseDownEvent, _window, cx| {
                                        let x: f32 = event.position.x.into();
                                        let y: f32 = (event.position.y + px(18.0)).into();
                                        this.state.context_menu.show(
                                            x,
                                            y,
                                            vec![
                                                crate::ui::context_menu::ContextMenuItem::action(
                                                    "Toggle Left Sidebar",
                                                )
                                                .with_shortcut("Ctrl+B"),
                                                crate::ui::context_menu::ContextMenuItem::action(
                                                    "Toggle Bottom Terminal",
                                                )
                                                .with_shortcut("Ctrl+J"),
                                                crate::ui::context_menu::ContextMenuItem::action(
                                                    "Toggle Agent Studio",
                                                )
                                                .with_shortcut("Ctrl+L"),
                                                crate::ui::context_menu::ContextMenuItem::separator(),
                                                crate::ui::context_menu::ContextMenuItem::action(
                                                    "Zoom In",
                                                )
                                                .with_shortcut("Ctrl+="),
                                                crate::ui::context_menu::ContextMenuItem::action(
                                                    "Zoom Out",
                                                )
                                                .with_shortcut("Ctrl+-"),
                                                crate::ui::context_menu::ContextMenuItem::action(
                                                    "Reset Zoom",
                                                )
                                                .with_shortcut("Ctrl+0"),
                                            ],
                                        );
                                        cx.notify();
                                    }),
                                )
                                .child("View"),
                        )
                        .child(
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded(px(4.0))
                                .hover(|s| s.bg(theme.bg_hover).text_color(theme.text_primary))
                                .cursor_pointer()
                                .child("Go"),
                        )
                        .child(
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded(px(4.0))
                                .hover(|s| s.bg(theme.bg_hover).text_color(theme.text_primary))
                                .cursor_pointer()
                                .child("Run"),
                        )
                        .child(
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded(px(4.0))
                                .hover(|s| s.bg(theme.bg_hover).text_color(theme.text_primary))
                                .cursor_pointer()
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.state.toggle_bottom_panel();
                                        cx.notify();
                                    }),
                                )
                                .child("Terminal"),
                        )
                        .child(
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded(px(4.0))
                                .hover(|s| s.bg(theme.bg_hover).text_color(theme.text_primary))
                                .cursor_pointer()
                                .child("Help"),
                        ),
                ),
        )
        // Draggable spacer between Left menus and Center pill
        .child(
            div()
                .flex_1()
                .h_full()
                .cursor_move()
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|_this, event: &MouseDownEvent, window, _cx| {
                        if event.click_count == 2 {
                            window.zoom_window();
                        } else {
                            window.start_window_move();
                        }
                    }),
                ),
        )
        // Center: Quick Open Pill with "vscodium-rust" (Signature Cursor Style)
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_center()
                .w(px(440.0))
                .h(px(26.0))
                .bg(theme.pill_bg)
                .border_1()
                .border_color(theme.pill_border)
                .rounded(px(6.0))
                .px_3()
                .cursor_pointer()
                .hover(|s| {
                    s.border_color(theme.ring_focus)
                        .bg(theme.pill_hover_bg)
                })
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _event, _window, cx| {
                        this.quick_open.open(&this.state, &this.commands);
                        cx.notify();
                    }),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .w_full()
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_2()
                                .child(ui_icon(IconName::Search, 12.0, theme.text_muted))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(theme.text_primary)
                                        .child("vscodium-rust"),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_subtle)
                                        .child("— Search files, symbols, commands"),
                                ),
                        )
                        .child(
                            div()
                                .px_1p5()
                                .py_0p5()
                                .rounded(px(3.0))
                                .bg(rgba(0xffffff14))
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child("Ctrl+P"),
                        ),
                ),
        )
        // Draggable spacer between Center pill and Right controls
        .child(
            div()
                .flex_1()
                .h_full()
                .cursor_move()
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|_this, event: &MouseDownEvent, window, _cx| {
                        if event.click_count == 2 {
                            window.zoom_window();
                        } else {
                            window.start_window_move();
                        }
                    }),
                ),
        )
        // Right: Utility Icons, Layout Toggles, Chat Pill & Window Controls
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap_2()
                // Settings / Manage Gear Icon (exact TypeScript TitleBar parity)
                .child(
                    div()
                        .p_1()
                        .rounded(px(3.0))
                        .cursor_pointer()
                        .hover(|s| s.bg(theme.bg_hover))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                this.state
                                    .set_activity(crate::app_state::ActivityTab::Settings);
                                this.state.left_sidebar_open = true;
                                cx.notify();
                            }),
                        )
                        .child(icon_14(IconName::Settings, theme.text_muted)),
                )
                // Globe icon
                .child(
                    div()
                        .p_1()
                        .rounded(px(3.0))
                        .cursor_pointer()
                        .hover(|s| s.bg(theme.bg_hover))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                this.state
                                    .set_activity(crate::app_state::ActivityTab::ExternalBrowser);
                                this.state.left_sidebar_open = true;
                                cx.notify();
                            }),
                        )
                        .child(icon_14(IconName::Globe, theme.text_muted)),
                )
                // Panel Layout Toggles
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1()
                        // Toggle Left Sidebar
                        .child(
                            div()
                                .p_1()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .bg(if state.left_sidebar_open {
                                    theme.bg_hover
                                } else {
                                    theme.bg_titlebar
                                })
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.state.left_sidebar_open =
                                            !this.state.left_sidebar_open;
                                        cx.notify();
                                    }),
                                )
                                .child(icon_14(
                                    IconName::PanelLeft,
                                    if state.left_sidebar_open {
                                        theme.text_primary
                                    } else {
                                        theme.text_subtle
                                    },
                                )),
                        )
                        // Toggle Bottom Panel
                        .child(
                            div()
                                .p_1()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .bg(if state.bottom_panel_open {
                                    theme.bg_hover
                                } else {
                                    theme.bg_titlebar
                                })
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.state.bottom_panel_open =
                                            !this.state.bottom_panel_open;
                                        cx.notify();
                                    }),
                                )
                                .child(icon_14(
                                    IconName::PanelBottom,
                                    if state.bottom_panel_open {
                                        theme.text_primary
                                    } else {
                                        theme.text_subtle
                                    },
                                )),
                        )
                        // Toggle Right AI Sidebar (Col 1)
                        .child(
                            div()
                                .p_1()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .bg(if state.right_sidebar_open {
                                    theme.bg_hover
                                } else {
                                    theme.bg_titlebar
                                })
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.state.right_sidebar_open =
                                            !this.state.right_sidebar_open;
                                        cx.notify();
                                    }),
                                )
                                .child(icon_14(
                                    IconName::PanelRight,
                                    if state.right_sidebar_open {
                                        theme.text_primary
                                    } else {
                                        theme.text_subtle
                                    },
                                )),
                        )
                        // Toggle iPhone Live Preview (Col 2)
                        .child(
                            div()
                                .p_1()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .bg(if state.iphone_preview_open {
                                    theme.bg_hover
                                } else {
                                    theme.bg_titlebar
                                })
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.state.iphone_preview_open =
                                            !this.state.iphone_preview_open;
                                        cx.notify();
                                    }),
                                )
                                .child(icon_14(
                                    IconName::Smartphone,
                                    if state.iphone_preview_open {
                                        theme.accent
                                    } else {
                                        theme.text_subtle
                                    },
                                )),
                        )
                        // Toggle Both Side-by-Side (Dual View)
                        .child(
                            div()
                                .p_1()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .bg(if state.right_sidebar_open && state.iphone_preview_open {
                                    theme.bg_hover
                                } else {
                                    theme.bg_titlebar
                                })
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.state.toggle_dual_sidebar();
                                        cx.notify();
                                    }),
                                )
                                .child(icon_14(
                                    IconName::Columns2,
                                    if state.right_sidebar_open && state.iphone_preview_open {
                                        theme.accent
                                    } else {
                                        theme.text_subtle
                                    },
                                )),
                        ),
                )
                // Security / APEX Shield Icon
                .child(
                    div()
                        .p_1()
                        .rounded(px(3.0))
                        .cursor_pointer()
                        .hover(|s| s.bg(theme.bg_hover))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                this.state
                                    .set_activity(crate::app_state::ActivityTab::ApexSecurity);
                                this.state.left_sidebar_open = true;
                                cx.notify();
                            }),
                        )
                        .child(icon_14(IconName::Shield, theme.text_muted)),
                )
                // Chat Pill with "+"
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1p5()
                        .px_2p5()
                        .py_0p5()
                        .rounded(px(5.0))
                        .bg(theme.bg_raised)
                        .border_1()
                        .border_color(theme.pill_border)
                        .cursor_pointer()
                        .hover(|s| {
                            s.bg(theme.pill_hover_bg)
                                .border_color(theme.ring_focus)
                        })
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                this.state.right_sidebar_open = true;
                                this.state.create_agent_thread();
                                cx.notify();
                            }),
                        )
                        .child(icon_12(IconName::Bot, theme.accent))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.text_primary)
                                .child("AI Agent"),
                        )
                        .child(icon_12(IconName::Plus, theme.text_muted)),
                )
                // Window Control Buttons (Minimize, Maximize, Close)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .ml_1()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .w(px(28.0))
                                .h(px(26.0))
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|_this, _event, window, _cx| {
                                        window.minimize_window();
                                    }),
                                )
                                .child(icon_12(IconName::Minus, theme.text_muted)),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .w(px(28.0))
                                .h(px(26.0))
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|_this, _event, window, _cx| {
                                        window.zoom_window();
                                    }),
                                )
                                .child(icon_12(IconName::Square, theme.text_muted)),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .w(px(28.0))
                                .h(px(26.0))
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(rgb(0xe81123)))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|_this, _event, _window, _cx| {
                                        std::process::exit(0);
                                    }),
                                )
                                .child(icon_12(IconName::X, theme.text_muted)),
                        ),
                ),
        )
}
