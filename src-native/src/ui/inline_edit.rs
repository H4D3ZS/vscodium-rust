// Inline Edit Overlay (Ctrl+K) for Native GPUI.
//
// Floating AI instruction overlay positioned over the editor.
// Allows prompting the AI model to modify, generate, or refactor selected code in-place.

use crate::theme::Theme;
use crate::ui::icons::{icon_12, ui_icon, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;

#[derive(Clone, Debug)]
pub struct InlineEditState {
    pub is_open: bool,
    pub prompt: String,
    pub position_top: f32,
    pub position_left: f32,
    pub target_file: String,
    pub line_number: usize,
    pub selected_text: String,
    pub is_generating: bool,
}

impl Default for InlineEditState {
    fn default() -> Self {
        Self {
            is_open: false,
            prompt: String::new(),
            position_top: 80.0,
            position_left: 120.0,
            target_file: String::new(),
            line_number: 1,
            selected_text: String::new(),
            is_generating: false,
        }
    }
}

impl InlineEditState {
    pub fn open(
        &mut self,
        top: f32,
        left: f32,
        target_file: String,
        line: usize,
        selected_text: String,
    ) {
        self.is_open = true;
        self.prompt.clear();
        self.position_top = top.max(40.0);
        self.position_left = left.max(60.0);
        self.target_file = target_file;
        self.line_number = line;
        self.selected_text = selected_text;
        self.is_generating = false;
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.prompt.clear();
        self.is_generating = false;
    }
}

/// Render the floating inline edit overlay.
pub fn render_inline_edit_overlay(
    inline_state: &InlineEditState,
    theme: &Theme,
    cx: &mut Context<HadesAppView>,
) -> Option<impl IntoElement> {
    if !inline_state.is_open {
        return None;
    }

    let top = inline_state.position_top;
    let left = inline_state.position_left;
    let has_prompt = !inline_state.prompt.trim().is_empty();
    let prompt_display = if inline_state.prompt.is_empty() {
        "Edit with AI... (e.g. 'add error handling')".to_string()
    } else {
        inline_state.prompt.clone()
    };

    Some(
        div()
            .absolute()
            .top(px(top))
            .left(px(left))
            .w(px(380.0))
            .rounded(px(8.0))
            .bg(theme.bg_composer)
            .border_1()
            .border_color(theme.accent)
            .shadow_lg()
            .p_3()
            .flex()
            .flex_col()
            .gap_2()
            // Header: Bot Icon + "Inline Edit" + Close
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .w(px(22.0))
                                    .h(px(22.0))
                                    .rounded(px(4.0))
                                    .bg(theme.accent)
                                    .child(icon_12(IconName::Sparkles, rgb(0xffffff)))
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .child("Inline Edit (Ctrl+K)")
                            )
                            .children((!inline_state.target_file.is_empty()).then(|| {
                                div()
                                    .text_xs()
                                    .text_color(theme.text_subtle)
                                    .child(format!(":{}", inline_state.line_number))
                            }))
                    )
                    .child(
                        div()
                            .p_1()
                            .rounded(px(3.0))
                            .cursor_pointer()
                            .hover(|s| s.bg(theme.bg_hover))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.state.inline_edit.close();
                                cx.notify();
                            }))
                            .child(icon_12(IconName::X, theme.text_muted))
                    )
            )
            // Input Box
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .w_full()
                    .min_h(px(34.0))
                    .px_2p5()
                    .py_1p5()
                    .rounded(px(5.0))
                    .bg(theme.bg_input)
                    .border_1()
                    .border_color(theme.border_focus)
                    .child(
                        div()
                            .flex_1()
                            .text_xs()
                            .text_color(if inline_state.prompt.is_empty() {
                                theme.text_subtle
                            } else {
                                theme.text_primary
                            })
                            .child(prompt_display)
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_1p5()
                            .children(inline_state.is_generating.then(|| {
                                div()
                                    .text_xs()
                                    .text_color(theme.accent)
                                    .child("Generating...")
                            }))
                            .child(
                                div()
                                    .p_1()
                                    .rounded(px(3.0))
                                    .cursor_pointer()
                                    .hover(|s| s.bg(theme.bg_hover))
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                        if !this.state.inline_edit.prompt.trim().is_empty() {
                                            let prompt = this.state.inline_edit.prompt.clone();
                                            let context = if !this.state.inline_edit.selected_text.is_empty() {
                                                format!("In file {}, at line {}:\n```\n{}\n```\nInstruction: {}",
                                                    this.state.inline_edit.target_file,
                                                    this.state.inline_edit.line_number,
                                                    this.state.inline_edit.selected_text,
                                                    prompt
                                                )
                                            } else {
                                                format!("In file {}: {}", this.state.inline_edit.target_file, prompt)
                                            };
                                            this.state.send_ai_prompt(&context);
                                            this.state.inline_edit.is_generating = true;
                                            this.state.toast_manager.push_info("AI Edit dispatched");
                                            this.state.inline_edit.close();
                                            cx.notify();
                                        }
                                    }))
                                    .child(ui_icon(
                                        IconName::Send,
                                        12.0,
                                        if has_prompt { theme.accent } else { theme.text_subtle },
                                    ))
                            )
                    )
            )
            // Footer: Hints
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .text_xs()
                    .text_color(theme.text_subtle)
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_1()
                            .child("Press")
                            .child(
                                div()
                                    .px_1()
                                    .py_0p5()
                                    .rounded(px(2.0))
                                    .bg(theme.bg_raised)
                                    .text_color(theme.text_muted)
                                    .child("Enter")
                            )
                            .child("to generate,")
                            .child(
                                div()
                                    .px_1()
                                    .py_0p5()
                                    .rounded(px(2.0))
                                    .bg(theme.bg_raised)
                                    .text_color(theme.text_muted)
                                    .child("Esc")
                            )
                            .child("to cancel")
                    )
                    .child(
                        div()
                            .text_color(theme.accent)
                            .child("Auto-Apply")
                    )
            )
    )
}
