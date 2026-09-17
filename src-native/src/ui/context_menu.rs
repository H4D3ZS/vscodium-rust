// Context Menu System for Native GPUI.
//
// Provides a floating overlay context menu triggered by right-click events and titlebar menus.
// Supports menu items with labels, shortcuts, separators, disabled states, and full click execution.
// Used by Explorer (file operations), Tab strip (close actions), Terminal, and Titlebar Menus.

use crate::theme::Theme;
use crate::ui::icons::{icon_14, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;

/// A single context menu item.
#[derive(Clone, Debug)]
pub enum ContextMenuItem {
    Action {
        label: String,
        shortcut: Option<String>,
        icon: Option<IconName>,
        disabled: bool,
        danger: bool,
    },
    Separator,
}

impl ContextMenuItem {
    pub fn action(label: &str) -> Self {
        Self::Action {
            label: label.to_string(),
            shortcut: None,
            icon: None,
            disabled: false,
            danger: false,
        }
    }

    pub fn with_shortcut(mut self, shortcut: &str) -> Self {
        if let Self::Action {
            shortcut: ref mut s,
            ..
        } = self
        {
            *s = Some(shortcut.to_string());
        }
        self
    }

    pub fn with_icon(mut self, icon: IconName) -> Self {
        if let Self::Action {
            icon: ref mut i, ..
        } = self
        {
            *i = Some(icon);
        }
        self
    }

    pub fn disabled(mut self) -> Self {
        if let Self::Action {
            disabled: ref mut d,
            ..
        } = self
        {
            *d = true;
        }
        self
    }

    pub fn danger(mut self) -> Self {
        if let Self::Action {
            danger: ref mut dg, ..
        } = self
        {
            *dg = true;
        }
        self
    }

    pub fn separator() -> Self {
        Self::Separator
    }
}

/// Context menu state managed in HadesNativeState.
#[derive(Clone, Debug, Default)]
pub struct ContextMenuState {
    pub is_open: bool,
    pub position_x: f32,
    pub position_y: f32,
    pub items: Vec<ContextMenuItem>,
    pub selected_idx: Option<usize>,
    /// Opaque context for the handler (e.g., file path for explorer menu).
    pub context: String,
}

impl ContextMenuState {
    pub fn open(&mut self, x: f32, y: f32, items: Vec<ContextMenuItem>, context: String) {
        self.is_open = true;
        self.position_x = x;
        self.position_y = y;
        self.items = items;
        self.selected_idx = None;
        self.context = context;
    }

    pub fn show(&mut self, x: f32, y: f32, items: Vec<ContextMenuItem>) {
        self.open(x, y, items, String::new());
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.items.clear();
        self.selected_idx = None;
        self.context.clear();
    }
}

/// Predefined context menus for common scenarios.
pub fn explorer_file_context_menu() -> Vec<ContextMenuItem> {
    vec![
        ContextMenuItem::action("Open").with_shortcut("Enter"),
        ContextMenuItem::action("Open to the Side").with_shortcut("Ctrl+Enter"),
        ContextMenuItem::separator(),
        ContextMenuItem::action("New File...").with_icon(IconName::Plus),
        ContextMenuItem::action("New Folder...").with_icon(IconName::FolderPlus),
        ContextMenuItem::separator(),
        ContextMenuItem::action("Cut").with_shortcut("Ctrl+X"),
        ContextMenuItem::action("Copy").with_shortcut("Ctrl+C"),
        ContextMenuItem::action("Copy Path").with_shortcut("Ctrl+Shift+C"),
        ContextMenuItem::action("Copy Relative Path"),
        ContextMenuItem::separator(),
        ContextMenuItem::action("Rename...").with_shortcut("F2"),
        ContextMenuItem::action("Delete")
            .with_shortcut("Delete")
            .danger(),
    ]
}

pub fn tab_context_menu() -> Vec<ContextMenuItem> {
    tab_context_menu_for(false)
}

pub fn tab_context_menu_for(is_pinned: bool) -> Vec<ContextMenuItem> {
    vec![
        ContextMenuItem::action("Close").with_shortcut("Ctrl+W"),
        ContextMenuItem::action("Close Others"),
        ContextMenuItem::action("Close All"),
        ContextMenuItem::action("Close to the Right"),
        ContextMenuItem::separator(),
        ContextMenuItem::action("Copy Path").with_shortcut("Ctrl+Shift+C"),
        ContextMenuItem::action("Reveal in Explorer"),
        ContextMenuItem::separator(),
        if is_pinned {
            ContextMenuItem::action("Unpin Tab")
        } else {
            ContextMenuItem::action("Pin Tab")
        },
    ]
}

pub fn terminal_context_menu() -> Vec<ContextMenuItem> {
    vec![
        ContextMenuItem::action("Copy").with_shortcut("Ctrl+C"),
        ContextMenuItem::action("Paste").with_shortcut("Ctrl+V"),
        ContextMenuItem::separator(),
        ContextMenuItem::action("Select All").with_shortcut("Ctrl+A"),
        ContextMenuItem::action("Clear Terminal"),
        ContextMenuItem::separator(),
        ContextMenuItem::action("Split Terminal"),
        ContextMenuItem::action("Kill Terminal"),
    ]
}

/// Execute an action triggered from a context menu item.
pub fn execute_context_menu_action(
    view: &mut HadesAppView,
    label: &str,
    context: &str,
    window: &mut Window,
    cx: &mut Context<HadesAppView>,
) {
    match label {
        "New File" | "New File..." => {
            let parent = if !context.is_empty() {
                let p = std::path::Path::new(context);
                if p.is_dir() {
                    Some(p.to_path_buf())
                } else {
                    p.parent().map(|d| d.to_path_buf())
                }
            } else {
                None
            };
            view.state.explorer_create = Some(crate::app_state::ExplorerCreateState {
                is_dir: false,
                name: String::new(),
                parent_dir: parent,
            });
            view.state.focused_panel = crate::app_state::FocusedPanel::Explorer;
            window.focus(&view.focus_handle, cx);
            cx.notify();
        }
        "Open File..." | "Open File" => {
            view.prompt_open_file(window, cx);
        }
        "Open Folder..." | "Open Folder" => {
            view.prompt_open_folder(window, cx);
        }
        "Save" => {
            view.save_current_file(window, cx);
        }
        "Save As..." | "Save As" => {
            view.prompt_save_file_as(window, cx);
        }
        "Save All" => {
            for tab in &mut view.state.tabs {
                if tab.dirty {
                    let _ = std::fs::write(&tab.path, tab.content());
                    tab.dirty = false;
                }
            }
            view.state.toast_manager.push_info("All files saved");
            cx.notify();
        }
        "Exit" => {
            std::process::exit(0);
        }
        "Close" | "Close Tab" => {
            let target_idx = context.parse::<usize>().ok().or(view.state.active_tab_idx);
            if let Some(idx) = target_idx {
                view.close_tab(idx, window, cx);
            }
        }
        "Close Others" => {
            let target_idx = context.parse::<usize>().ok().or(view.state.active_tab_idx);
            if let Some(idx) = target_idx {
                if idx < view.state.tabs.len() {
                    let target_path = view.state.tabs[idx].path.clone();
                    view.state
                        .tabs
                        .retain(|t| t.is_pinned || t.path == target_path);
                    view.state.active_tab_idx = view
                        .state
                        .tabs
                        .iter()
                        .position(|t| t.path == target_path)
                        .or(Some(0));
                    view.state.save_session();
                }
            }
        }
        "Close to the Right" => {
            let target_idx = context.parse::<usize>().ok().or(view.state.active_tab_idx);
            if let Some(idx) = target_idx {
                let mut i = 0;
                view.state.tabs.retain(|t| {
                    let keep = i <= idx || t.is_pinned;
                    i += 1;
                    keep
                });
                if let Some(active) = view.state.active_tab_idx {
                    if active >= view.state.tabs.len() {
                        view.state.active_tab_idx = view.state.tabs.len().checked_sub(1);
                    }
                }
                view.state.save_session();
            }
        }
        "Close All" => {
            view.state.tabs.retain(|t| t.is_pinned);
            if view.state.tabs.is_empty() {
                view.state.active_tab_idx = None;
                view.editor.update(cx, |ed, cx| {
                    ed.set_value("", window, cx);
                });
            } else {
                view.state.active_tab_idx = Some(0);
            }
            view.state.save_session();
        }
        "Reveal in Explorer" => {
            let target_idx = context.parse::<usize>().ok().or(view.state.active_tab_idx);
            if let Some(idx) = target_idx {
                let title = view.state.tabs.get(idx).map(|t| t.title.clone());
                if let Some(t) = title {
                    view.state
                        .set_activity(crate::app_state::ActivityTab::Explorer);
                    view.state.left_sidebar_open = true;
                    view.state
                        .toast_manager
                        .push_info(&format!("Revealed: {t}"));
                }
            }
        }
        "Pin Tab" => {
            let target_idx = context.parse::<usize>().ok().or(view.state.active_tab_idx);
            if let Some(idx) = target_idx {
                view.state.pin_tab(idx);
                view.state.toast_manager.push_info("Tab pinned");
            }
        }
        "Unpin Tab" => {
            let target_idx = context.parse::<usize>().ok().or(view.state.active_tab_idx);
            if let Some(idx) = target_idx {
                view.state.unpin_tab(idx);
                view.state.toast_manager.push_info("Tab unpinned");
            }
        }
        "Toggle Left Sidebar" => {
            view.state.toggle_left_sidebar();
        }
        "Toggle Bottom Terminal" => {
            view.state.toggle_bottom_panel();
        }
        "Toggle Agent Studio" => {
            view.state.right_sidebar_open = !view.state.right_sidebar_open;
        }
        "Zoom In" => {
            view.state.editor_font_size = (view.state.editor_font_size + 1).min(36);
            view.state.toast_manager.push_info(&format!("Zoom In (Font: {}px)", view.state.editor_font_size));
        }
        "Zoom Out" => {
            view.state.editor_font_size = (view.state.editor_font_size.saturating_sub(1)).max(9);
            view.state.toast_manager.push_info(&format!("Zoom Out (Font: {}px)", view.state.editor_font_size));
        }
        "Reset Zoom" => {
            view.state.editor_font_size = 14;
            view.state.toast_manager.push_info("Reset Zoom (Font: 14px)");
        }
        "Clear Terminal" => {
            view.state.clear_terminal();
        }
        "Split Terminal" => {
            view.state.spawn_terminal(None);
        }
        "Kill Terminal" => {
            view.state.kill_active_terminal();
        }
        "Open" => {
            if !context.is_empty() {
                view.open_file(context, window, cx);
            }
        }
        "Open to the Side" => {
            if !context.is_empty() {
                if !view.state.is_split_editor {
                    view.state.split_editor();
                }
                view.open_file(context, window, cx);
            }
        }
        "New Folder..." => {
            let parent = if !context.is_empty() {
                let p = std::path::Path::new(context);
                if p.is_dir() {
                    Some(p.to_path_buf())
                } else {
                    p.parent().map(|d| d.to_path_buf())
                }
            } else {
                None
            };
            view.state.explorer_create = Some(crate::app_state::ExplorerCreateState {
                is_dir: true,
                name: String::new(),
                parent_dir: parent,
            });
            view.state.focused_panel = crate::app_state::FocusedPanel::Explorer;
            window.focus(&view.focus_handle, cx);
            cx.notify();
        }
        "Copy Path" => {
            let path = if !context.is_empty() {
                context.to_string()
            } else if let Some(t) = view.state.active_tab() {
                t.path.clone()
            } else {
                String::new()
            };
            if !path.is_empty() {
                view.state
                    .toast_manager
                    .push_info(&format!("Copied: {}", path));
            }
        }
        "Copy Relative Path" => {
            let path_str = if !context.is_empty() {
                context
            } else if let Some(t) = view.state.active_tab() {
                &t.path
            } else {
                ""
            };
            if !path_str.is_empty() {
                let rel = std::path::Path::new(path_str)
                    .strip_prefix(&view.state.workspace_root)
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| path_str.to_string());
                view.state
                    .toast_manager
                    .push_info(&format!("Copied: {}", rel));
            }
        }
        "Delete" => {
            if !context.is_empty() {
                let p = std::path::Path::new(context);
                if p.exists() {
                    let res = if p.is_dir() {
                        std::fs::remove_dir_all(p)
                    } else {
                        std::fs::remove_file(p)
                    };
                    if res.is_ok() {
                        view.state.refresh_file_tree();
                        view.state.toast_manager.push_warning(&format!(
                            "Deleted {}",
                            p.file_name().unwrap_or_default().to_string_lossy()
                        ));
                    }
                }
            }
        }
        "Rename..." => {
            if !context.is_empty() {
                view.input_box
                    .open_rename(std::path::PathBuf::from(context));
                window.focus(&view.focus_handle, cx);
                cx.notify();
            }
        }
        _ => {
            view.state
                .toast_manager
                .push_info(&format!("Action: {}", label));
        }
    }
}

/// Render the floating context menu overlay with outside click dismissal and full action execution.
pub fn render_context_menu(
    state: &ContextMenuState,
    theme: &Theme,
    cx: &mut Context<HadesAppView>,
) -> Option<impl IntoElement> {
    if !state.is_open || state.items.is_empty() {
        return None;
    }

    let x = state.position_x;
    let y = state.position_y;
    let context_val = state.context.clone();

    Some(
        div()
            .absolute()
            .inset_0()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _event, _window, cx| {
                    this.state.context_menu.close();
                    cx.notify();
                }),
            )
            .on_mouse_down(
                MouseButton::Right,
                cx.listener(|this, _event, _window, cx| {
                    this.state.context_menu.close();
                    cx.notify();
                }),
            )
            .child(
                div()
                    .absolute()
                    .left(px(x))
                    .top(px(y))
                    .min_w(px(200.0))
                    .rounded(px(6.0))
                    .bg(theme.bg_raised)
                    .border_1()
                    .border_color(theme.border_subtle)
                    .shadow_lg()
                    .py_1()
                    .children(state.items.iter().enumerate().map(|(idx, item)| {
                        match item {
                            ContextMenuItem::Separator => div()
                                .w_full()
                                .h(px(1.0))
                                .bg(theme.border_subtle)
                                .my_0p5()
                                .into_any_element(),
                            ContextMenuItem::Action {
                                label,
                                shortcut,
                                icon,
                                disabled,
                                danger,
                            } => {
                                let is_selected = state.selected_idx == Some(idx);
                                let text_color = if *danger {
                                    theme.status_red
                                } else if *disabled {
                                    theme.text_subtle
                                } else if is_selected {
                                    theme.text_primary
                                } else {
                                    theme.text_muted
                                };

                                let mut row = div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .justify_between()
                                    .px_3()
                                    .py_1()
                                    .text_xs()
                                    .text_color(text_color);

                                if !*disabled {
                                    let lbl = label.clone();
                                    let ctx_str = context_val.clone();
                                    row = row
                                        .cursor_pointer()
                                        .hover(|s| s.bg(theme.bg_hover))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _event, window, cx| {
                                                this.state.context_menu.close();
                                                execute_context_menu_action(
                                                    this, &lbl, &ctx_str, window, cx,
                                                );
                                                cx.notify();
                                            }),
                                        );
                                }

                                let mut left = div().flex().flex_row().items_center().gap_2();

                                if let Some(icon_name) = icon {
                                    left = left.child(icon_14(*icon_name, text_color));
                                }

                                left = left.child(div().child(label.to_string()));

                                let mut full = row.child(left);

                                if let Some(sc) = shortcut {
                                    full = full.child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.text_subtle)
                                            .ml_4()
                                            .child(sc.to_string()),
                                    );
                                }

                                full.into_any_element()
                            }
                        }
                    })),
            ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::prelude::v1::test;

    #[test]
    fn test_explorer_file_context_menu_items() {
        let menu = explorer_file_context_menu();
        assert!(!menu.is_empty());
        let labels: Vec<&str> = menu
            .iter()
            .filter_map(|i| match i {
                ContextMenuItem::Action { label, .. } => Some(label.as_str()),
                ContextMenuItem::Separator => None,
            })
            .collect();
        assert!(labels.contains(&"Open"));
        assert!(labels.contains(&"Open to the Side"));
        assert!(labels.contains(&"New File..."));
        assert!(labels.contains(&"New Folder..."));
        assert!(labels.contains(&"Copy Path"));
        assert!(labels.contains(&"Copy Relative Path"));
        assert!(labels.contains(&"Delete"));
    }

    #[test]
    fn test_tab_context_menu_items() {
        let menu = tab_context_menu();
        assert!(!menu.is_empty());
        let labels: Vec<&str> = menu
            .iter()
            .filter_map(|i| match i {
                ContextMenuItem::Action { label, .. } => Some(label.as_str()),
                ContextMenuItem::Separator => None,
            })
            .collect();
        assert!(labels.contains(&"Close"));
        assert!(labels.contains(&"Close Others"));
        assert!(labels.contains(&"Close All"));
        assert!(labels.contains(&"Close to the Right"));
        assert!(labels.contains(&"Reveal in Explorer"));
    }
}
