use crate::app_state::HadesNativeState;
use crate::panels::traits::AuxiliaryTab;
use crate::ui::icons::{icon_12, icon_14, ui_icon, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;

pub struct SpecsPanel;

impl AuxiliaryTab for SpecsPanel {
    fn id(&self) -> &'static str {
        "specs"
    }

    fn title(&self) -> &'static str {
        "Specs"
    }

    fn icon(&self) -> IconName {
        IconName::ListOrdered
    }

    fn render(&self, state: &HadesNativeState, cx: &mut Context<HadesAppView>) -> AnyElement {
        render_specs_panel(state, cx).into_any_element()
    }
}

pub fn render_specs_panel(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;

    div()
        .flex()
        .flex_col()
        .size_full()
        .bg(theme.bg_sidebar)
        // Specs Header
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
                                .child(ui_icon(IconName::ListOrdered, 14.0, theme.accent))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_primary)
                                        .child("SPECS TO CODE ENGINE")
                                )
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.accent)
                                .font_weight(FontWeight::MEDIUM)
                                .child("EARS Notation")
                        )
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_subtle)
                        .child("Autonomous architectural synthesis & task decomposition")
                )
        )
        // Specs Prompt Input & Generator
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
                                .child("FEATURE SPECIFICATION PROMPT")
                        )
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .p_2()
                        .rounded(px(6.0))
                        .bg(theme.bg_input)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_subtle)
                                .child("e.g. Implement full native GPUI workspace migration with 120 FPS D3D12")
                        )
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .flex()
                                .flex_1()
                                .items_center()
                                .justify_center()
                                .gap_1p5()
                                .py_1p5()
                                .rounded(px(4.0))
                                .bg(theme.accent)
                                .cursor_pointer()
                                .hover(|s| s.opacity(0.9))
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                    this.state.send_ai_prompt("Transform requirements into EARS specification and discrete task list.");
                                    cx.notify();
                                }))
                                .child(icon_12(IconName::Sparkles, theme.text_on_accent))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_on_accent)
                                        .child("Synthesize Specs (EARS)")
                                )
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .px_3()
                                .py_1p5()
                                .rounded(px(4.0))
                                .bg(theme.bg_raised)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                    this.execute_terminal_command("cargo check --manifest-path src-native/Cargo.toml");
                                    cx.notify();
                                }))
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_primary)
                                        .child("Scaffold")
                                )
                        )
                )
        )
        // Active Project Requirements & Tasks List
        .child(
            div()
                .id("specs_panel_scroll")
                .flex()
                .flex_col()
                .flex_1()
                .overflow_y_scroll()
                .p_3()
                .gap_3()
                .children(state.specs_projects.iter().enumerate().map(|(p_idx, project)| {
                    let is_active = state.active_specs_project_idx == Some(p_idx);
                    div()
                        .flex()
                        .flex_col()
                        .gap_2()
                        .p_2p5()
                        .rounded(px(6.0))
                        .bg(if is_active { theme.bg_raised } else { theme.bg_surface })
                        .border_1()
                        .border_color(if is_active { theme.border_focus } else { theme.border_subtle })
                        // Project Title Header
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
                                        .child(icon_14(IconName::Folder, theme.accent))
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(theme.text_primary)
                                                .child(project.name.clone())
                                        )
                                )
                                .child(
                                    div()
                                        .px_2()
                                        .py_0p5()
                                        .rounded(px(3.0))
                                        .bg(theme.bg_hover)
                                        .text_xs()
                                        .text_color(theme.status_green)
                                        .child("Approved")
                                )
                        )
                        // EARS Specs preview
                        .child(
                            div()
                                .p_2()
                                .rounded(px(4.0))
                                .bg(theme.bg_input)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .text_xs()
                                .text_color(theme.text_muted)
                                .font_family("Consolas")
                                .child(project.specs.clone())
                        )
                        // Discrete Tasks Breakdown
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(theme.text_muted)
                                .child(format!("DISCRETE TASKS ({} total)", project.tasks.len()))
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_1p5()
                                .children(project.tasks.iter().enumerate().map(|(t_idx, task)| {
                                    let is_done = task.status == "done";
                                    let is_in_progress = task.status == "in_progress";

                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .justify_between()
                                        .p_2()
                                        .rounded(px(4.0))
                                        .bg(theme.bg_app)
                                        .border_1()
                                        .border_color(theme.border_subtle)
                                        .hover(|s| s.border_color(theme.border_focus))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .gap_2()
                                                .child(
                                                    div()
                                                        .w(px(14.0))
                                                        .h(px(14.0))
                                                        .rounded(px(3.0))
                                                        .border_1()
                                                        .border_color(if is_done { theme.status_green } else if is_in_progress { theme.status_yellow } else { theme.border_focus })
                                                        .bg(if is_done { theme.status_green } else { theme.bg_input })
                                                        .flex()
                                                        .items_center()
                                                        .justify_center()
                                                        .cursor_pointer()
                                                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                                            if let Some(proj) = this.state.specs_projects.get_mut(p_idx) {
                                                                if let Some(t) = proj.tasks.get_mut(t_idx) {
                                                                    t.status = if t.status == "done" {
                                                                        "todo".to_string()
                                                                    } else {
                                                                        "done".to_string()
                                                                    };
                                                                }
                                                            }
                                                            cx.notify();
                                                        }))
                                                        .child(if is_done {
                                                            icon_12(IconName::Check, theme.text_on_accent)
                                                        } else {
                                                            icon_12(IconName::Check, gpui_kit::gpui::rgba(0x00000000))
                                                        })
                                                )
                                                .child(
                                                    div()
                                                        .flex()
                                                        .flex_col()
                                                        .child(
                                                            div()
                                                                .text_xs()
                                                                .font_weight(FontWeight::MEDIUM)
                                                                .text_color(if is_done { theme.text_subtle } else { theme.text_primary })
                                                                .child(task.title.clone())
                                                        )
                                                        .child(
                                                            div()
                                                                .text_xs()
                                                                .text_color(theme.text_subtle)
                                                                .child(task.details.clone())
                                                        )
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
                                                        .px_1p5()
                                                        .py_0p5()
                                                        .rounded(px(3.0))
                                                        .bg(theme.bg_raised)
                                                        .text_xs()
                                                        .text_color(if task.complexity == "Hard" {
                                                            theme.status_red
                                                        } else if task.complexity == "Medium" {
                                                            theme.status_yellow
                                                        } else {
                                                            theme.status_green
                                                        })
                                                        .child(task.complexity.clone())
                                                )
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(theme.text_subtle)
                                                        .child(format!("{}h", task.estimated_hours))
                                                )
                                        )
                                }))
                        )
                }))
        )
}
