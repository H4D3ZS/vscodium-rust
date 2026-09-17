use crate::app_state::HadesNativeState;
use crate::panels::traits::AuxiliaryTab;
use crate::ui::icons::{icon_12, ui_icon, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;

pub struct RulesPanel;

impl AuxiliaryTab for RulesPanel {
    fn id(&self) -> &'static str {
        "rules"
    }

    fn title(&self) -> &'static str {
        "Rules"
    }

    fn icon(&self) -> IconName {
        IconName::BookOpen
    }

    fn render(&self, state: &HadesNativeState, cx: &mut Context<HadesAppView>) -> AnyElement {
        render_rules_panel(state, cx).into_any_element()
    }
}

pub fn render_rules_panel(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;

    div()
        .flex()
        .flex_col()
        .size_full()
        .bg(theme.bg_sidebar)
        // Header
        .child(
            div()
                .flex()
                .flex_col()
                .p_3()
                .border_b_1()
                .border_color(theme.border_subtle)
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
                                .gap_2()
                                .child(ui_icon(IconName::BookOpen, 14.0, theme.accent))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_primary)
                                        .child("STEERING RULES & HOOKS"),
                                ),
                        )
                        .child(
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded(px(3.0))
                                .bg(theme.bg_raised)
                                .text_xs()
                                .text_color(theme.status_green)
                                .child(".cursorrules Active"),
                        ),
                )
                .child(
                    div().text_xs().text_color(theme.text_subtle).child(
                        "Autonomous steering directives, system prompts, and file save hooks",
                    ),
                ),
        )
        // Global Steering Rules
        .child(
            div()
                .flex()
                .flex_col()
                .p_3()
                .gap_2()
                .border_b_1()
                .border_color(theme.border_subtle)
                .bg(theme.bg_surface)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.text_muted)
                                .child("GLOBAL STEERING DIRECTIVES"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.accent)
                                .child(".cursorrules"),
                        ),
                )
                .child(
                    div()
                        .p_2()
                        .rounded(px(6.0))
                        .bg(theme.bg_input)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .text_xs()
                        .text_color(theme.text_primary)
                        .font_family("Consolas")
                        .child(state.global_rules.clone()),
                )
                .child(
                    div().flex().flex_row().justify_end().child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1p5()
                            .px_2p5()
                            .py_1()
                            .rounded(px(4.0))
                            .bg(theme.bg_raised)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .cursor_pointer()
                            .hover(|s| s.bg(theme.bg_hover))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _event, _window, cx| {
                                    this.state.status_message =
                                        "Steering rules synced to .cursorrules".to_string();
                                    cx.notify();
                                }),
                            )
                            .child(icon_12(IconName::Check, theme.status_green))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_primary)
                                    .child("Sync to Disk"),
                            ),
                    ),
                ),
        )
        // Automated Agent Hooks
        .child(
            div()
                .id("rules_panel_scroll")
                .flex()
                .flex_col()
                .flex_1()
                .overflow_y_scroll()
                .p_3()
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
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.text_muted)
                                .child("AUTOMATED AGENT SAVE HOOKS"),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .px_2()
                                .py_0p5()
                                .rounded(px(3.0))
                                .bg(theme.accent)
                                .cursor_pointer()
                                .hover(|s| s.opacity(0.9))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.state.add_hook(
                                            "src/**/*.rs",
                                            "Check memory safety and invariants",
                                        );
                                        cx.notify();
                                    }),
                                )
                                .child(icon_12(IconName::Plus, theme.text_on_accent))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_on_accent)
                                        .child("Add Hook"),
                                ),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_2()
                        .children(state.agent_hooks.iter().map(|hook| {
                            let hook_id = hook.id.clone();
                            let hook_id_del = hook.id.clone();
                            let is_enabled = hook.enabled;

                            div()
                                .flex()
                                .flex_col()
                                .p_2p5()
                                .rounded(px(6.0))
                                .bg(theme.bg_raised)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .gap_2()
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .justify_between()
                                        .child(
                                            div()
                                                .px_2()
                                                .py_0p5()
                                                .rounded(px(3.0))
                                                .bg(theme.bg_app)
                                                .border_1()
                                                .border_color(theme.border_subtle)
                                                .text_xs()
                                                .font_family("Consolas")
                                                .text_color(theme.accent)
                                                .child(hook.pattern.clone()),
                                        )
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
                                                        .bg(if is_enabled {
                                                            theme.bg_hover
                                                        } else {
                                                            theme.bg_input
                                                        })
                                                        .text_xs()
                                                        .cursor_pointer()
                                                        .text_color(if is_enabled {
                                                            theme.status_green
                                                        } else {
                                                            theme.text_subtle
                                                        })
                                                        .on_mouse_down(
                                                            MouseButton::Left,
                                                            cx.listener(
                                                                move |this, _event, _window, cx| {
                                                                    this.state
                                                                        .toggle_hook(&hook_id);
                                                                    cx.notify();
                                                                },
                                                            ),
                                                        )
                                                        .child(if is_enabled {
                                                            "Enabled"
                                                        } else {
                                                            "Disabled"
                                                        }),
                                                )
                                                .child(
                                                    div()
                                                        .p_1()
                                                        .rounded(px(3.0))
                                                        .cursor_pointer()
                                                        .hover(|s| s.bg(theme.bg_hover))
                                                        .on_mouse_down(
                                                            MouseButton::Left,
                                                            cx.listener(
                                                                move |this, _event, _window, cx| {
                                                                    this.state
                                                                        .delete_hook(&hook_id_del);
                                                                    cx.notify();
                                                                },
                                                            ),
                                                        )
                                                        .child(icon_12(
                                                            IconName::X,
                                                            theme.status_red,
                                                        )),
                                                ),
                                        ),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .child(hook.prompt.clone()),
                                )
                        })),
                ),
        )
}
