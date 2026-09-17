use crate::app_state::HadesNativeState;
use crate::ui::icons::{icon_12, ui_icon, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputBoxMode {
    NewFile,
    NewFolder,
    Rename,
    Custom,
}

#[derive(Clone, Debug)]
pub struct InputBoxState {
    pub is_open: bool,
    pub mode: InputBoxMode,
    pub title: String,
    pub prompt: String,
    pub placeholder: String,
    pub value: String,
    pub target_dir: Option<PathBuf>,
    pub target_path: Option<PathBuf>,
    pub error: Option<String>,
}

impl Default for InputBoxState {
    fn default() -> Self {
        Self {
            is_open: false,
            mode: InputBoxMode::NewFile,
            title: String::new(),
            prompt: String::new(),
            placeholder: String::new(),
            value: String::new(),
            target_dir: None,
            target_path: None,
            error: None,
        }
    }
}

impl InputBoxState {
    pub fn open_new_file(&mut self, target_dir: Option<PathBuf>) {
        self.is_open = true;
        self.mode = InputBoxMode::NewFile;
        self.title = "New File".to_string();
        self.prompt = "Enter file name or relative path:".to_string();
        self.placeholder = "e.g. src/utils.rs or component.rs".to_string();
        self.value.clear();
        self.target_dir = target_dir;
        self.target_path = None;
        self.error = None;
    }

    pub fn open_new_folder(&mut self, target_dir: Option<PathBuf>) {
        self.is_open = true;
        self.mode = InputBoxMode::NewFolder;
        self.title = "New Folder".to_string();
        self.prompt = "Enter folder name or relative path:".to_string();
        self.placeholder = "e.g. components or tests/unit".to_string();
        self.value.clear();
        self.target_dir = target_dir;
        self.target_path = None;
        self.error = None;
    }

    pub fn open_rename(&mut self, target_path: PathBuf) {
        self.is_open = true;
        self.mode = InputBoxMode::Rename;
        self.title = "Rename".to_string();
        let initial_name = target_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        self.prompt = format!("Enter new name for `{initial_name}`:");
        self.placeholder = "New name...".to_string();
        self.value = initial_name;
        self.target_dir = target_path.parent().map(|p| p.to_path_buf());
        self.target_path = Some(target_path);
        self.error = None;
    }

    pub fn open_custom(&mut self, title: &str, prompt: &str, placeholder: &str, initial: &str) {
        self.is_open = true;
        self.mode = InputBoxMode::Custom;
        self.title = title.to_string();
        self.prompt = prompt.to_string();
        self.placeholder = placeholder.to_string();
        self.value = initial.to_string();
        self.target_dir = None;
        self.target_path = None;
        self.error = None;
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.value.clear();
        self.target_dir = None;
        self.target_path = None;
        self.error = None;
    }

    pub fn validate(&mut self) {
        let trimmed = self.value.trim();
        if trimmed.is_empty() {
            self.error = None;
            return;
        }

        const INVALID_CHARS: &[char] = &['<', '>', ':', '"', '|', '?', '*'];
        for c in INVALID_CHARS {
            if trimmed.contains(*c) {
                self.error = Some(format!("Filename cannot contain '{c}'"));
                return;
            }
        }

        self.error = None;
    }
}

pub fn render_input_box_overlay(
    state: &HadesNativeState,
    input: &InputBoxState,
    cx: &mut Context<HadesAppView>,
) -> Option<AnyElement> {
    if !input.is_open {
        return None;
    }

    let theme = &state.theme;
    let mode_icon = match input.mode {
        InputBoxMode::NewFile => IconName::FilePlus,
        InputBoxMode::NewFolder => IconName::FolderPlus,
        InputBoxMode::Rename => IconName::FileCode,
        InputBoxMode::Custom => IconName::Sparkles,
    };

    Some(
        div()
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .bg(rgba(0x00000088))
            .flex()
            .justify_center()
            .items_start()
            .pt(px(60.0))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _event, _window, cx| {
                    this.input_box.close();
                    cx.notify();
                }),
            )
            .child(
                div()
                    .w(px(520.0))
                    .bg(theme.bg_raised)
                    .border_1()
                    .border_color(theme.border_focus)
                    .rounded(px(8.0))
                    .shadow_lg()
                    .flex()
                    .flex_col()
                    .overflow_hidden()
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _event, window, cx| {
                            window.focus(&this.focus_handle, cx);
                            cx.notify();
                        }),
                    )
                    // Header with Icon & Title
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .px_3()
                            .py_2()
                            .bg(theme.bg_titlebar)
                            .border_b_1()
                            .border_color(theme.border_subtle)
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_2()
                                    .child(ui_icon(mode_icon, 14.0, theme.accent))
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(theme.text_primary)
                                            .child(input.title.clone()),
                                    ),
                            )
                            .child(
                                div()
                                    .p_0p5()
                                    .rounded(px(3.0))
                                    .cursor_pointer()
                                    .hover(|s| s.bg(theme.bg_hover))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _event, _window, cx| {
                                            this.input_box.close();
                                            cx.notify();
                                        }),
                                    )
                                    .child(icon_12(IconName::X, theme.text_muted)),
                            ),
                    )
                    // Body container
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .p_3()
                            .gap_2()
                            // Prompt label
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child(input.prompt.clone()),
                            )
                            // Input field box
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .h(px(34.0))
                                    .px_2p5()
                                    .bg(theme.bg_editor)
                                    .border_1()
                                    .cursor_text()
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _event, window, cx| {
                                            window.focus(&this.focus_handle, cx);
                                            cx.notify();
                                        }),
                                    )
                                    .border_color(if input.error.is_some() {
                                        rgb(0xe57373)
                                    } else {
                                        theme.accent
                                    })
                                    .rounded(px(4.0))
                                    .child(if input.value.is_empty() {
                                        div()
                                            .text_sm()
                                            .text_color(theme.text_subtle)
                                            .child(input.placeholder.clone())
                                    } else {
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(theme.text_primary)
                                                    .child(input.value.clone()),
                                            )
                                            .child(
                                                div()
                                                    .w(px(2.0))
                                                    .h(px(16.0))
                                                    .bg(theme.accent)
                                                    .ml_0p5(),
                                            )
                                    }),
                            )
                            // Optional validation error badge
                            .children(input.error.as_ref().map(|err| {
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_1p5()
                                    .text_xs()
                                    .text_color(rgb(0xe57373))
                                    .child(icon_12(IconName::TriangleAlert, rgb(0xe57373)))
                                    .child(err.clone())
                            })),
                    )
                    // Footer toolbar with shortcut hints
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_end()
                            .gap_2()
                            .px_3()
                            .py_1p5()
                            .bg(theme.bg_titlebar)
                            .border_t_1()
                            .border_color(theme.border_subtle)
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_1()
                                    .child(
                                        div()
                                            .px_1p5()
                                            .py_0p5()
                                            .rounded(px(3.0))
                                            .bg(theme.bg_hover)
                                            .text_xs()
                                            .text_color(theme.text_muted)
                                            .child("Enter"),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.text_subtle)
                                            .child("Confirm"),
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
                                            .px_1p5()
                                            .py_0p5()
                                            .rounded(px(3.0))
                                            .bg(theme.bg_hover)
                                            .text_xs()
                                            .text_color(theme.text_muted)
                                            .child("Esc"),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.text_subtle)
                                            .child("Cancel"),
                                    ),
                            ),
                    ),
            )
            .into_any_element(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::prelude::v1::test;

    #[test]
    fn test_input_box_lifecycle_and_validation() {
        let mut input = InputBoxState::default();
        assert!(!input.is_open);

        input.open_new_file(None);
        assert!(input.is_open);
        assert_eq!(input.mode, InputBoxMode::NewFile);
        assert_eq!(input.title, "New File");

        // Validate invalid character
        input.value = "my_file:bad.rs".to_string();
        input.validate();
        assert!(input.error.is_some());

        // Validate valid character
        input.value = "src/valid_file.rs".to_string();
        input.validate();
        assert!(input.error.is_none());

        input.close();
        assert!(!input.is_open);
        assert!(input.value.is_empty());
    }

    #[test]
    fn test_input_box_open_modes() {
        let mut input = InputBoxState::default();

        input.open_new_folder(Some(PathBuf::from("src")));
        assert!(input.is_open);
        assert_eq!(input.mode, InputBoxMode::NewFolder);
        assert_eq!(input.target_dir, Some(PathBuf::from("src")));

        let target_file = PathBuf::from("src/main.rs");
        input.open_rename(target_file.clone());
        assert!(input.is_open);
        assert_eq!(input.mode, InputBoxMode::Rename);
        assert_eq!(input.value, "main.rs");
        assert_eq!(input.target_path, Some(target_file));
    }
}
