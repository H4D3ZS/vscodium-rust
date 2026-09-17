use crate::app_state::{ExplorerCreateState, FileNode, FocusedPanel, HadesNativeState};
use crate::ui::icons::{icon_12, icon_14, ui_icon, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;

pub fn render_explorer_panel(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;

    div()
        .flex()
        .flex_col()
        .size_full()
        .bg(theme.bg_sidebar)
        // Explorer Header Bar
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .h(px(35.0))
                .px_3()
                .border_b_1()
                .border_color(theme.border_subtle)
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.text_muted)
                        .child("EXPLORER"),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1p5()
                        // New File action
                        .child(
                            div()
                                .p_1()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, window, cx| {
                                        this.state.explorer_create = Some(ExplorerCreateState {
                                            is_dir: false,
                                            name: String::new(),
                                            parent_dir: None,
                                        });
                                        this.state.focused_panel = FocusedPanel::Explorer;
                                        window.focus(&this.focus_handle, cx);
                                        cx.notify();
                                    }),
                                )
                                .child(icon_14(IconName::Plus, theme.text_muted)),
                        )
                        // New Folder action
                        .child(
                            div()
                                .p_1()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, window, cx| {
                                        this.state.explorer_create = Some(ExplorerCreateState {
                                            is_dir: true,
                                            name: String::new(),
                                            parent_dir: None,
                                        });
                                        this.state.focused_panel = FocusedPanel::Explorer;
                                        window.focus(&this.focus_handle, cx);
                                        cx.notify();
                                    }),
                                )
                                .child(icon_14(IconName::Folder, theme.text_muted)),
                        )
                        // Refresh tree action
                        .child(
                            div()
                                .p_1()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.state.refresh_file_tree();
                                        cx.notify();
                                    }),
                                )
                                .child(icon_14(IconName::RefreshCw, theme.text_muted)),
                        ),
                ),
        )
        // Main Explorer Sections Container
        .child(
            div()
                .flex()
                .flex_col()
                .flex_1()
                .overflow_hidden()
                // Section 1: Root Workspace Section Header
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1p5()
                        .h(px(24.0))
                        .px_2()
                        .bg(theme.bg_sidebar)
                        .child(icon_12(IconName::ChevronDown, theme.text_muted))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child(state.workspace_display_name().to_uppercase()),
                        ),
                )
                // File Tree List or No Folder Opened
                .child(if !state.has_workspace || state.file_tree.is_empty() {
                    div()
                        .flex()
                        .flex_col()
                        .p_4()
                        .gap_2p5()
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child("You have not yet opened a folder."),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .justify_center()
                                .gap_2()
                                .px_3()
                                .py_1p5()
                                .rounded(px(5.0))
                                .bg(theme.accent)
                                .text_color(theme.text_on_accent)
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .cursor_pointer()
                                .hover(|s| s.opacity(0.9))
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _e, window, cx| {
                                    this.prompt_open_folder(window, cx);
                                }))
                                .child(icon_14(IconName::Folder, theme.text_on_accent))
                                .child("Open Folder"),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .justify_center()
                                .gap_2()
                                .px_3()
                                .py_1p5()
                                .rounded(px(5.0))
                                .bg(theme.bg_raised)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .text_color(theme.text_primary)
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _e, window, cx| {
                                    this.prompt_open_file(window, cx);
                                }))
                                .child(icon_14(IconName::File, theme.text_muted))
                                .child("Open File"),
                        )
                        .into_any_element()
                } else {
                    div()
                        .id("explorer_file_tree_scroll")
                        .flex()
                        .flex_col()
                        .flex_1()
                        .overflow_y_scroll()
                        .py_0p5()
                        .children(
                            state.explorer_create.as_ref().filter(|c| c.parent_dir.is_none()).map(|c| {
                                render_inline_create_node(c, theme, 0, cx)
                            })
                        )
                        .children(
                            state
                                .file_tree
                                .iter()
                                .map(|node| render_file_node(node, state, 0, cx)),
                        )
                        .into_any_element()
                })
                // Section 2: AI PROJECT SPECS
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1p5()
                        .h(px(24.0))
                        .px_2()
                        .border_t_1()
                        .border_color(theme.border_subtle)
                        .cursor_pointer()
                        .hover(|s| s.bg(theme.bg_hover))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                this.state.active_right_tab =
                                    crate::app_state::RightSidebarTab::Specs;
                                this.state.right_sidebar_open = true;
                                cx.notify();
                            }),
                        )
                        .child(icon_12(IconName::ChevronRight, theme.text_subtle))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_muted)
                                .child("AI PROJECT SPECS"),
                        ),
                )
                // Section 3: OUTLINE
                .child(render_outline_section(state, cx))
                // Section 4: TIMELINE
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1p5()
                        .h(px(24.0))
                        .px_2()
                        .border_t_1()
                        .border_color(theme.border_subtle)
                        .child(icon_12(IconName::ChevronRight, theme.text_subtle))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_muted)
                                .child("TIMELINE"),
                        ),
                ),
        )
}

fn render_file_node(
    node: &FileNode,
    state: &HadesNativeState,
    depth: usize,
    cx: &mut Context<HadesAppView>,
) -> AnyElement {
    let theme = &state.theme;
    let indent = px((depth * 14 + 8) as f32);
    let path = node.path.clone();
    let is_dir = node.is_dir;

    let is_active_file = !is_dir
        && state
            .active_tab()
            .map(|t| t.path.as_str() == path.as_str())
            .unwrap_or(false);

    let click_path = path.clone();
    let right_click_path = path.clone();

    let row = div()
        .flex()
        .flex_row()
        .items_center()
        .h(px(22.0))
        .w_full()
        .pl(indent)
        .pr_2()
        .cursor_pointer()
        .bg(if is_active_file {
            theme.bg_active
        } else {
            theme.bg_sidebar
        })
        .hover(|s| s.bg(theme.bg_hover))
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this, _event, window, cx| {
                if is_dir {
                    toggle_expand(&mut this.state.file_tree, &click_path);
                } else {
                    this.open_file(&click_path, window, cx);
                    this.state.focused_panel = FocusedPanel::Editor;
                }
                cx.notify();
            }),
        )
        .on_mouse_down(
            MouseButton::Right,
            cx.listener(move |this, event: &MouseDownEvent, _window, cx| {
                let x: f32 = event.position.x.into();
                let y: f32 = event.position.y.into();
                this.state.context_menu.open(
                    x,
                    y,
                    crate::ui::context_menu::explorer_file_context_menu(),
                    right_click_path.clone(),
                );
                cx.notify();
            }),
        );

    // Git Status decoration lookup
    let git_status_change = state.scm.changes.iter().find(|c| {
        c.path == node.path || node.path.ends_with(&c.path) || c.path.ends_with(&node.name)
    });
    let git_status_badge = git_status_change.map(|c| {
        let (status_text, status_color) = match c.status.as_str() {
            "M" => ("M", theme.status_yellow),
            "A" => ("A", theme.status_green),
            "D" => ("D", theme.status_red),
            _ => ("U", rgb(0x60a5fa)),
        };
        div()
            .text_xs()
            .font_weight(FontWeight::BOLD)
            .text_color(status_color)
            .child(status_text)
    });

    if is_dir {
        // Folder row with chevron + folder icon
        let chevron = if node.is_expanded {
            icon_12(IconName::ChevronDown, theme.text_subtle)
        } else {
            icon_12(IconName::ChevronRight, theme.text_subtle)
        };

        let folder_icon = if node.is_expanded {
            ui_icon(IconName::FolderOpen, 14.0, rgb(0xe5c07b))
        } else {
            ui_icon(IconName::Folder, 14.0, rgb(0xe5c07b))
        };

        let element = div()
            .flex()
            .flex_col()
            .w_full()
            .child(
                row.justify_between()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_1p5()
                            .child(chevron)
                            .child(folder_icon)
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_primary)
                                    .child(node.name.clone()),
                            ),
                    )
                    .children(git_status_badge),
            )
            .children((node.is_expanded).then(|| {
                let node_path = std::path::PathBuf::from(&node.path);
                div().flex().flex_col().w_full()
                    .children(state.explorer_create.as_ref().filter(|c| c.parent_dir.as_ref() == Some(&node_path)).map(|c| {
                        render_inline_create_node(c, theme, depth + 1, cx)
                    }))
                    .children(
                        node.children
                            .iter()
                            .map(|child| render_file_node(child, state, depth + 1, cx)),
                    )
            }));

        element.into_any_element()
    } else {
        // File row with type-based vector icon matching TypeScript Material Theme
        let (file_icon_name, file_icon_color) = if node.name.ends_with(".rs") {
            (IconName::FileCode, rgb(0xe57373))
        } else if node.name.ends_with(".ts") || node.name.ends_with(".tsx") {
            (IconName::FileCode, rgb(0x3178c6))
        } else if node.name.ends_with(".js")
            || node.name.ends_with(".cjs")
            || node.name.ends_with(".mjs")
        {
            (IconName::FileCode, rgb(0xf7df1e))
        } else if node.name.ends_with(".json")
            || node.name.ends_with(".toml")
            || node.name.ends_with(".yaml")
            || node.name.ends_with(".yml")
        {
            (IconName::FileCode, rgb(0xffb74d))
        } else if node.name.ends_with(".md") {
            (IconName::FileText, rgb(0x42a5f5))
        } else if node.name.ends_with(".py") {
            (IconName::FileCode, rgb(0x38bdf8))
        } else if node.name.ends_with(".sh")
            || node.name.ends_with(".ps1")
            || node.name.ends_with(".bat")
            || node.name.ends_with(".cmd")
        {
            (IconName::Terminal, rgb(0x4ade80))
        } else if node.name.ends_with(".lock") {
            (IconName::Lock, rgb(0xf59e0b))
        } else if node.name.starts_with(".git") {
            (IconName::GitBranch, rgb(0xf97316))
        } else if node.name.contains("LICENSE") {
            (IconName::Key, rgb(0xfbbf24))
        } else if node.name.ends_with(".css") || node.name.ends_with(".scss") {
            (IconName::FileCode, rgb(0xba68c8))
        } else {
            (IconName::File, theme.text_subtle)
        };

        let file_icon = ui_icon(file_icon_name, 14.0, file_icon_color);

        let element = row
            .justify_between()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_1p5()
                    // Extra indent padding where chevron would be
                    .pl(px(14.0))
                    .child(file_icon)
                    .child(
                        div()
                            .text_xs()
                            .text_color(if is_active_file {
                                theme.text_primary
                            } else {
                                theme.text_muted
                            })
                            .font_weight(if is_active_file {
                                FontWeight::MEDIUM
                            } else {
                                FontWeight::NORMAL
                            })
                            .child(node.name.clone()),
                    ),
            )
            .children(git_status_badge);

        element.into_any_element()
    }
}

fn toggle_expand(nodes: &mut [FileNode], target_path: &str) -> bool {
    for node in nodes {
        if node.path == target_path {
            node.is_expanded = !node.is_expanded;
            if node.is_expanded && node.children.is_empty() {
                let p = std::path::Path::new(&node.path);
                if let Ok(entries) = std::fs::read_dir(p) {
                    let mut items = Vec::new();
                    for entry in entries.flatten() {
                        let child_p = entry.path();
                        let name = child_p
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string();
                        if name.starts_with('.') || name == "target" || name == "node_modules" {
                            continue;
                        }
                        let is_dir = child_p.is_dir();
                        items.push(FileNode {
                            name,
                            path: child_p.to_string_lossy().to_string(),
                            is_dir,
                            is_expanded: false,
                            children: Vec::new(),
                        });
                    }
                    items.sort_by(|a, b| {
                        if a.is_dir == b.is_dir {
                            a.name.cmp(&b.name)
                        } else if a.is_dir {
                            std::cmp::Ordering::Less
                        } else {
                            std::cmp::Ordering::Greater
                        }
                    });
                    node.children = items;
                }
            }
            return true;
        }
        if toggle_expand(&mut node.children, target_path) {
            return true;
        }
    }
    false
}

fn render_outline_section(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;
    let maybe_tab = state.active_tab();

    let symbols = if let Some(tab) = maybe_tab {
        state
            .vsx
            .language_services
            .get_document_symbols(&tab.lines, &std::path::PathBuf::from(&tab.path))
    } else {
        Vec::new()
    };

    let cursor_line = maybe_tab.map(|t| t.cursor_row).unwrap_or(0);
    let active_path = maybe_tab.map(|t| t.path.clone()).unwrap_or_default();

    div()
        .flex()
        .flex_col()
        .border_t_1()
        .border_color(theme.border_subtle)
        // Outline Header
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .h(px(24.0))
                .px_2()
                .bg(theme.bg_sidebar)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1p5()
                        .child(icon_12(IconName::ChevronDown, theme.text_subtle))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_muted)
                                .child(format!("OUTLINE ({})", symbols.len())),
                        ),
                ),
        )
        // Symbols List
        .child(
            div()
                .id("outline_scroll")
                .flex()
                .flex_col()
                .max_h(px(200.0))
                .overflow_y_scroll()
                .p_1()
                .gap_0p5()
                .children(if maybe_tab.is_none() {
                    vec![div()
                        .px_3()
                        .py_1p5()
                        .text_xs()
                        .text_color(theme.text_subtle)
                        .child("Open a file to see its outline.")
                        .into_any_element()]
                } else if symbols.is_empty() {
                    vec![div()
                        .px_3()
                        .py_1p5()
                        .text_xs()
                        .text_color(theme.text_subtle)
                        .child("No declarations found in current file.")
                        .into_any_element()]
                } else {
                    symbols
                        .into_iter()
                        .map(|sym| {
                            let sym_name = sym.name;
                            let sym_row = sym.range.start.line;
                            let sym_line = sym_row + 1;
                            let path_clone = active_path.clone();

                            let is_current = cursor_line >= sym.range.start.line
                                && cursor_line <= sym.range.end.line;

                            let (badge, badge_color) = match sym.kind {
                                crate::editor::engine::lsp::SymbolKind::Function
                                | crate::editor::engine::lsp::SymbolKind::Method => {
                                    ("fn", theme.accent)
                                }
                                crate::editor::engine::lsp::SymbolKind::Struct
                                | crate::editor::engine::lsp::SymbolKind::Class => {
                                    ("struct", rgb(0xffb74d))
                                }
                                crate::editor::engine::lsp::SymbolKind::Enum => {
                                    ("enum", rgb(0x81c784))
                                }
                                crate::editor::engine::lsp::SymbolKind::Interface => {
                                    ("trait", rgb(0x64b5f6))
                                }
                                crate::editor::engine::lsp::SymbolKind::Module => {
                                    ("mod", rgb(0xa855f7))
                                }
                                crate::editor::engine::lsp::SymbolKind::Constant => {
                                    ("const", rgb(0xf59e0b))
                                }
                                crate::editor::engine::lsp::SymbolKind::Variable
                                | crate::editor::engine::lsp::SymbolKind::Field => {
                                    ("var", rgb(0x38bdf8))
                                }
                                _ => ("sym", theme.text_muted),
                            };

                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .justify_between()
                                .px_2()
                                .py_0p5()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .bg(if is_current {
                                    theme.accent.opacity(0.12)
                                } else {
                                    theme.bg_sidebar
                                })
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _e, window, cx| {
                                        this.goto_file_location(
                                            &path_clone,
                                            sym_row,
                                            0,
                                            window,
                                            cx,
                                        );
                                    }),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_1p5()
                                        .child(
                                            div()
                                                .px_1()
                                                .py_0p5()
                                                .rounded(px(2.0))
                                                .bg(badge_color.opacity(0.15))
                                                .text_xs()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(badge_color)
                                                .child(badge),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(if is_current {
                                                    theme.accent
                                                } else {
                                                    theme.text_primary
                                                })
                                                .child(sym_name),
                                        ),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_subtle)
                                        .child(sym_line.to_string()),
                                )
                                .into_any_element()
                        })
                        .collect()
                }),
        )
}

pub struct ExplorerPanel;

impl crate::panels::traits::WorkbenchPanel for ExplorerPanel {
    fn id(&self) -> &'static str {
        "explorer"
    }

    fn title(&self) -> &'static str {
        "EXPLORER"
    }

    fn icon(&self) -> IconName {
        IconName::Files
    }

    fn render(&self, state: &HadesNativeState, cx: &mut Context<HadesAppView>) -> AnyElement {
        render_explorer_panel(state, cx).into_any_element()
    }
}

fn render_inline_create_node(
    create: &ExplorerCreateState,
    theme: &crate::theme::Theme,
    depth: usize,
    cx: &mut Context<HadesAppView>,
) -> AnyElement {
    let indent = px((depth * 14 + 8) as f32);
    let icon = if create.is_dir {
        ui_icon(IconName::Folder, 14.0, rgb(0xe5c07b))
    } else {
        ui_icon(IconName::FileCode, 14.0, theme.accent)
    };
    let placeholder = if create.is_dir { "Folder name" } else { "File name" };

    div()
        .flex()
        .flex_row()
        .items_center()
        .h(px(24.0))
        .w_full()
        .pl(indent)
        .pr_2()
        .gap_1p5()
        .bg(theme.bg_editor)
        .border_1()
        .border_color(theme.accent)
        .rounded(px(2.0))
        .cursor_text()
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(|this, _event, window, cx| {
                window.focus(&this.focus_handle, cx);
                cx.notify();
            }),
        )
        .child(icon)
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .flex_1()
                .child(if create.name.is_empty() {
                    div()
                        .text_xs()
                        .text_color(theme.text_subtle)
                        .child(placeholder)
                } else {
                    div()
                        .text_xs()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(theme.text_primary)
                        .child(create.name.clone())
                })
                .child(
                    div()
                        .w(px(1.5))
                        .h(px(13.0))
                        .bg(theme.accent)
                        .ml_0p5(),
                ),
        )
        .into_any_element()
}

