use crate::app_state::{ActivityTab, HadesNativeState};
use crate::ui::icons::{icon_20, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;

pub fn render_activity_bar(
    state: &HadesNativeState,
    registry: &crate::panels::PanelRegistry,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;
    let active = state.active_activity;

    let tabs = [
        (
            ActivityTab::Explorer,
            IconName::Files,
            "Explorer (Ctrl+Shift+E)",
        ),
        (
            ActivityTab::Search,
            IconName::Search,
            "Search (Ctrl+Shift+F)",
        ),
        (
            ActivityTab::SourceControl,
            IconName::GitBranch,
            "Source Control (Ctrl+Shift+G)",
        ),
        (
            ActivityTab::ApexSecurity,
            IconName::Shield,
            "Security Suite — DAST, RE, Vulnerability Audit, Mobile Pentest",
        ),
        (
            ActivityTab::Canvases,
            IconName::LayoutDashboard,
            "Canvases — interactive agent dashboards",
        ),
        (
            ActivityTab::Debug,
            IconName::Bug,
            "Run & Debug (Ctrl+Shift+D)",
        ),
        (
            ActivityTab::Extensions,
            IconName::Blocks,
            "Extensions (Ctrl+Shift+X)",
        ),
        (
            ActivityTab::ExternalBrowser,
            IconName::Globe,
            "External Browser (Ctrl+Shift+U)",
        ),
        (
            ActivityTab::PyTorchStudio,
            IconName::Flame,
            "PyTorch ML Studio",
        ),
    ];

    div()
        .flex()
        .flex_col()
        .justify_between()
        .w(px(48.0))
        .h_full()
        .bg(theme.bg_activity_bar)
        .border_r_1()
        .border_color(theme.border_subtle)
        .py_1()
        // Top tab icons
        .child(
            div()
                .flex()
                .flex_col()
                .items_center()
                .w_full()

                .children(tabs.into_iter().map(|(tab, icon_name, _tooltip)| {
                    let is_selected = active == tab && state.left_sidebar_open;
                    let icon_color = if is_selected {
                        theme.text_primary
                    } else {
                        theme.text_subtle
                    };
                    let badge = registry.find_primary(tab.id()).and_then(|p| p.badge(state));

                    div()
                        .relative()
                        .flex()
                        .items_center()
                        .justify_center()
                        .w_full()
                        .h(px(46.0))
                        .cursor_pointer()
                        .hover(|s| {
                            s.bg(if is_selected {
                                theme.bg_active
                            } else {
                                theme.bg_hover
                            })
                        })
                        // Active indicator line on the left edge (Cursor IDE style with rounded cap)
                        .children(is_selected.then(|| {
                            div()
                                .absolute()
                                .left_0()
                                .top(px(9.0))
                                .bottom(px(9.0))
                                .w(px(2.5))
                                .rounded_r(px(2.0))
                                .bg(theme.accent)
                        }))
                        // Optional badge pill
                        .children(badge.map(|count| {
                            div()
                                .absolute()
                                .top(px(5.0))
                                .right(px(6.0))
                                .px_1()
                                .rounded_full()
                                .bg(theme.accent)
                                .text_color(rgb(0xffffff))
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .child(count.to_string())
                        }))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _event, _window, cx| {
                                if this.state.active_activity == tab && this.state.left_sidebar_open
                                {
                                    this.state.left_sidebar_open = false;
                                } else {
                                    this.state.set_activity(tab);
                                    this.state.left_sidebar_open = true;
                                }
                                cx.notify();
                            }),
                        )
                        .child(icon_20(icon_name, icon_color))
                })),
        )
        // Bottom utility icons: Account Profile, Settings, and Green Remote Dev indicator (exact TS parity)
        .child(
            div()
                .flex()
                .flex_col()
                .items_center()
                .w_full()
                .gap_1()
                // Account Profile
                .child(
                    div()
                        .relative()
                        .flex()
                        .items_center()
                        .justify_center()
                        .w_full()
                        .h(px(40.0))
                        .cursor_pointer()
                        .hover(|s| s.bg(theme.bg_hover))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                this.state
                                    .toast_manager
                                    .push_info("Account: Local Developer Profile Active");
                                cx.notify();
                            }),
                        )
                        .child(icon_20(IconName::User, theme.text_subtle)),
                )
                // Settings Gear
                .child({
                    let is_settings = state
                        .active_tab()
                        .map(|t| t.is_settings || t.path == "hades://settings")
                        .unwrap_or(false);
                    let icon_color = if is_settings {
                        theme.accent
                    } else {
                        theme.text_subtle
                    };

                    div()
                        .relative()
                        .flex()
                        .items_center()
                        .justify_center()
                        .w_full()
                        .h(px(40.0))
                        .cursor_pointer()
                        .hover(|s| {
                            s.bg(if is_settings {
                                theme.bg_active
                            } else {
                                theme.bg_hover
                            })
                        })
                        .children(is_settings.then(|| {
                            div()
                                .absolute()
                                .left_0()
                                .top(px(8.0))
                                .bottom(px(8.0))
                                .w(px(2.5))
                                .rounded_r(px(2.0))
                                .bg(theme.accent)
                        }))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                this.state.open_settings_tab();
                                cx.notify();
                            }),
                        )
                        .child(icon_20(IconName::Settings, icon_color))
                })
                // Green Remote Indicator Badge (VS Code Parity)
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .w_full()
                        .h(px(28.0))
                        .bg(rgb(0x16825d))
                        .hover(|s| s.bg(rgb(0x137050)))
                        .cursor_pointer()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xffffff))
                        .child("><")
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                this.state
                                    .toast_manager
                                    .push_info("Environment: Local Native Direct3D 12 Engine");
                                cx.notify();
                            }),
                        ),
                ),
        )
}
