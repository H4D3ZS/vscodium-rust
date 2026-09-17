use crate::app_state::HadesNativeState;
use crate::commands::{Command, CommandRegistry};
use crate::HadesAppView;
use gpui_kit::gpui::*;

#[derive(Clone, Debug)]
pub struct WorkspaceSymbolMatch {
    pub name: String,
    pub kind: crate::editor::engine::lsp::SymbolKind,
    pub file_path: String,
    pub line: usize,
    pub preview: String,
}

#[derive(Clone, Default)]
pub struct QuickOpenState {
    pub is_open: bool,
    pub query: String,
    pub selected_idx: usize,
    pub filtered_files: Vec<String>,
    pub filtered_commands: Vec<Command>,
    pub filtered_symbols: Vec<crate::editor::engine::lsp::DocumentSymbol>,
    pub filtered_workspace_symbols: Vec<WorkspaceSymbolMatch>,
}

impl QuickOpenState {
    pub fn is_command_mode(&self) -> bool {
        self.query.starts_with('>')
    }

    pub fn is_symbol_mode(&self) -> bool {
        self.query.starts_with('@')
    }

    pub fn is_workspace_symbol_mode(&self) -> bool {
        self.query.starts_with('#')
    }

    pub fn is_line_mode(&self) -> bool {
        self.query.starts_with(':')
    }

    pub fn open(&mut self, state: &HadesNativeState, commands: &CommandRegistry) {
        self.is_open = true;
        self.query.clear();
        self.selected_idx = 0;
        self.refresh_list(state, commands);
    }

    pub fn open_commands(&mut self, state: &HadesNativeState, commands: &CommandRegistry) {
        self.is_open = true;
        self.query = ">".to_string();
        self.selected_idx = 0;
        self.refresh_list(state, commands);
    }

    pub fn open_symbols(&mut self, state: &HadesNativeState, commands: &CommandRegistry) {
        self.is_open = true;
        self.query = "@".to_string();
        self.selected_idx = 0;
        self.refresh_list(state, commands);
    }

    pub fn open_workspace_symbols(&mut self, state: &HadesNativeState, commands: &CommandRegistry) {
        self.is_open = true;
        self.query = "#".to_string();
        self.selected_idx = 0;
        self.refresh_list(state, commands);
    }

    pub fn open_line(&mut self, state: &HadesNativeState, commands: &CommandRegistry) {
        self.is_open = true;
        self.query = ":".to_string();
        self.selected_idx = 0;
        self.refresh_list(state, commands);
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.query.clear();
        self.selected_idx = 0;
    }

    pub fn refresh_list(&mut self, state: &HadesNativeState, commands: &CommandRegistry) {
        if self.is_line_mode() {
            return;
        }

        if self.is_command_mode() {
            let cmd_query = self.query.trim_start_matches('>').trim();
            self.filtered_commands = commands.search(cmd_query);
            if self.selected_idx >= self.filtered_commands.len() {
                self.selected_idx = 0;
            }
            return;
        }

        if self.is_symbol_mode() {
            let sym_query = self.query.trim_start_matches('@').trim().to_lowercase();
            let symbols = if let Some(tab) = state.active_tab() {
                state
                    .vsx
                    .language_services
                    .get_document_symbols(&tab.lines, &std::path::PathBuf::from(&tab.path))
            } else {
                Vec::new()
            };

            self.filtered_symbols = if sym_query.is_empty() {
                symbols
            } else {
                symbols
                    .into_iter()
                    .filter(|s| s.name.to_lowercase().contains(&sym_query))
                    .collect()
            };

            if self.selected_idx >= self.filtered_symbols.len() {
                self.selected_idx = 0;
            }
            return;
        }

        if self.is_workspace_symbol_mode() {
            let sym_query = self.query.trim_start_matches('#').trim().to_lowercase();
            let mut matches = Vec::new();

            // 1. Collect from open tabs
            for tab in &state.tabs {
                let symbols = state
                    .vsx
                    .language_services
                    .get_document_symbols(&tab.lines, &std::path::PathBuf::from(&tab.path));
                for sym in symbols {
                    if sym_query.is_empty() || sym.name.to_lowercase().contains(&sym_query) {
                        matches.push(WorkspaceSymbolMatch {
                            name: sym.name,
                            kind: sym.kind,
                            file_path: tab.path.clone(),
                            line: sym.range.start.line,
                            preview: sym.detail.unwrap_or_default(),
                        });
                    }
                }
            }

            // 2. Also search top workspace code files
            if matches.len() < 30 {
                let mut code_files = Vec::new();
                fn collect_code(nodes: &[crate::app_state::FileNode], out: &mut Vec<String>) {
                    for n in nodes {
                        if n.is_dir {
                            collect_code(&n.children, out);
                        } else if n.path.ends_with(".rs")
                            || n.path.ends_with(".ts")
                            || n.path.ends_with(".js")
                            || n.path.ends_with(".py")
                        {
                            out.push(n.path.clone());
                        }
                    }
                }
                collect_code(&state.file_tree, &mut code_files);

                for f in code_files.into_iter().take(20) {
                    if !state.tabs.iter().any(|t| t.path == f) {
                        let full = state.workspace_root.join(&f);
                        if let Ok(content) = std::fs::read_to_string(&full) {
                            let lines: Vec<String> =
                                content.lines().map(|s| s.to_string()).collect();
                            let symbols = state
                                .vsx
                                .language_services
                                .get_document_symbols(&lines, &full);
                            for sym in symbols {
                                if sym_query.is_empty()
                                    || sym.name.to_lowercase().contains(&sym_query)
                                {
                                    matches.push(WorkspaceSymbolMatch {
                                        name: sym.name,
                                        kind: sym.kind,
                                        file_path: f.clone(),
                                        line: sym.range.start.line,
                                        preview: sym.detail.unwrap_or_default(),
                                    });
                                }
                            }
                        }
                    }
                }
            }

            self.filtered_workspace_symbols = matches;
            if self.selected_idx >= self.filtered_workspace_symbols.len() {
                self.selected_idx = 0;
            }
            return;
        }

        let mut all = Vec::new();
        fn collect_files(nodes: &[crate::app_state::FileNode], out: &mut Vec<String>) {
            for n in nodes {
                if n.is_dir {
                    collect_files(&n.children, out);
                } else {
                    out.push(n.path.clone());
                }
            }
        }
        collect_files(&state.file_tree, &mut all);

        let q = self.query.to_lowercase();
        if q.is_empty() {
            self.filtered_files = all.into_iter().take(30).collect();
        } else {
            self.filtered_files = all
                .into_iter()
                .filter(|p| p.to_lowercase().contains(&q))
                .take(30)
                .collect();
        }

        if self.selected_idx >= self.filtered_files.len() {
            self.selected_idx = 0;
        }
    }
}

pub fn render_quick_open_overlay(
    state: &HadesNativeState,
    qo: &QuickOpenState,
    cx: &mut Context<HadesAppView>,
) -> Option<AnyElement> {
    if !qo.is_open {
        return None;
    }

    let theme = &state.theme;
    let is_cmd = qo.is_command_mode();
    let is_sym = qo.is_symbol_mode();
    let is_ws_sym = qo.is_workspace_symbol_mode();
    let is_line = qo.is_line_mode();
    let selected_idx = qo.selected_idx;

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
            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                this.quick_open.close();
                cx.notify();
            }))
            .child(
                div()
                    .w(px(560.0))
                    .max_h(px(400.0))
                    .bg(theme.bg_raised)
                    .border_1()
                    .border_color(theme.border_focus)
                    .rounded(px(8.0))
                    .shadow_lg()
                    .flex()
                    .flex_col()
                    .overflow_hidden()
                    .on_mouse_down(MouseButton::Left, cx.listener(|_this, _event, _window, _cx| {
                        // Prevent click propagation to backdrop
                    }))
                    // Search input header
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2()
                            .h(px(40.0))
                            .px_3()
                            .border_b_1()
                            .border_color(theme.border_subtle)
                            .child(
                                if is_cmd {
                                    crate::ui::icons::ui_icon(crate::ui::icons::IconName::Terminal, 14.0, theme.accent).into_any_element()
                                } else if is_sym {
                                    crate::ui::icons::ui_icon(crate::ui::icons::IconName::FileCode, 14.0, theme.accent).into_any_element()
                                } else if is_ws_sym {
                                    crate::ui::icons::ui_icon(crate::ui::icons::IconName::Sparkles, 14.0, theme.accent).into_any_element()
                                } else if is_line {
                                    crate::ui::icons::ui_icon(crate::ui::icons::IconName::Code, 14.0, theme.accent).into_any_element()
                                } else {
                                    crate::ui::icons::ui_icon(crate::ui::icons::IconName::Search, 14.0, theme.text_subtle).into_any_element()
                                }
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(if qo.query.is_empty() { theme.text_subtle } else { theme.text_primary })
                                    .child(
                                        if qo.query.is_empty() {
                                            "Search files by name (type > for commands, @ for symbols, # for workspace, : for line)...".to_string()
                                        } else {
                                            qo.query.clone()
                                        }
                                    )
                            )
                    )
                    // Results list: Commands vs Symbols vs Files
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .overflow_hidden()
                            .py_1()
                            .children(if is_cmd {
                                qo.filtered_commands.iter().enumerate().map(|(idx, cmd)| {
                                    let is_sel = idx == selected_idx;
                                    let cmd_id = cmd.id;

                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .justify_between()
                                        .h(px(30.0))
                                        .px_3()
                                        .cursor_pointer()
                                        .bg(if is_sel { theme.bg_active } else { theme.bg_raised })
                                        .hover(|s| s.bg(theme.bg_hover))
                                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, window, cx| {
                                            execute_command(this, cmd_id, window, cx);
                                            this.quick_open.close();
                                            cx.notify();
                                        }))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .gap_2()
                                                .text_xs()
                                                .child(crate::ui::icons::ui_icon(crate::ui::icons::IconName::Terminal, 12.0, theme.accent))
                                                .child(
                                                    div()
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .text_color(if is_sel { theme.text_primary } else { theme.text_muted })
                                                        .child(cmd.title)
                                                )
                                        )
                                        .child(
                                            div()
                                                .px_1p5()
                                                .py_0p5()
                                                .rounded(px(3.0))
                                                .bg(theme.bg_card)
                                                .text_xs()
                                                .text_color(theme.text_subtle)
                                                .child(cmd.shortcut.unwrap_or(""))
                                        )
                                        .into_any_element()
                                }).collect::<Vec<_>>()
                            } else if is_sym {
                                qo.filtered_symbols.iter().enumerate().map(|(idx, sym)| {
                                    let is_sel = idx == selected_idx;
                                    let line_num = sym.range.start.line;
                                    let kind_badge = sym.kind.badge();
                                    let sym_name = sym.name.clone();
                                    let sym_detail = sym.detail.clone().unwrap_or_default();

                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .justify_between()
                                        .h(px(28.0))
                                        .px_3()
                                        .cursor_pointer()
                                        .bg(if is_sel { theme.bg_active } else { theme.bg_raised })
                                        .hover(|s| s.bg(theme.bg_hover))
                                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                            if let Some(tab) = this.state.active_tab_mut() {
                                                tab.cursor_row = line_num;
                                                tab.cursor_col = 0;
                                                tab.scroll_row = line_num.saturating_sub(5);
                                            }
                                            this.quick_open.close();
                                            cx.notify();
                                        }))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .gap_2()
                                                .text_xs()
                                                .child(
                                                    div()
                                                        .px_1p5()
                                                        .py_0p5()
                                                        .rounded(px(3.0))
                                                        .bg(theme.accent.opacity(0.15))
                                                        .text_color(theme.accent)
                                                        .font_weight(FontWeight::BOLD)
                                                        .child(kind_badge)
                                                )
                                                .child(
                                                    div()
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .text_color(if is_sel { theme.text_primary } else { theme.text_muted })
                                                        .child(sym_name)
                                                )
                                                .child(
                                                    div()
                                                        .text_color(theme.text_subtle)
                                                        .child(sym_detail)
                                                )
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.text_subtle)
                                                .child(format!(":{}", line_num + 1))
                                        )
                                        .into_any_element()
                                }).collect::<Vec<_>>()
                            } else if is_ws_sym {
                                qo.filtered_workspace_symbols.iter().enumerate().map(|(idx, sym)| {
                                    let is_sel = idx == selected_idx;
                                    let line_num = sym.line;
                                    let kind_badge = sym.kind.badge();
                                    let sym_name = sym.name.clone();
                                    let sym_file = sym.file_path.clone();

                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .justify_between()
                                        .h(px(28.0))
                                        .px_3()
                                        .cursor_pointer()
                                        .bg(if is_sel { theme.bg_active } else { theme.bg_raised })
                                        .hover(|s| s.bg(theme.bg_hover))
                                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, window, cx| {
                                            this.open_file(&sym_file, window, cx);
                                            if let Some(tab) = this.state.active_tab_mut() {
                                                tab.cursor_row = line_num;
                                                tab.cursor_col = 0;
                                                tab.scroll_row = line_num.saturating_sub(5);
                                            }
                                            this.quick_open.close();
                                            cx.notify();
                                        }))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .gap_2()
                                                .text_xs()
                                                .child(
                                                    div()
                                                        .px_1p5()
                                                        .py_0p5()
                                                        .rounded(px(3.0))
                                                        .bg(theme.accent.opacity(0.15))
                                                        .text_color(theme.accent)
                                                        .font_weight(FontWeight::BOLD)
                                                        .child(kind_badge)
                                                )
                                                .child(
                                                    div()
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .text_color(if is_sel { theme.text_primary } else { theme.text_muted })
                                                        .child(sym_name)
                                                )
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.text_subtle)
                                                .child(format!("{}:{}", sym.file_path, line_num + 1))
                                        )
                                        .into_any_element()
                                }).collect::<Vec<_>>()
                            } else if is_line {
                                let line_query = qo.query.trim_start_matches(':').trim();
                                let parts: Vec<&str> = line_query.split(':').collect();
                                let target_line = parts.first().and_then(|s| s.parse::<usize>().ok());
                                let target_col = parts.get(1).and_then(|s| s.parse::<usize>().ok()).unwrap_or(1);

                                let (cur_line, max_line) = if let Some(tab) = state.active_tab() {
                                    (tab.cursor_row + 1, tab.lines.len().max(1))
                                } else {
                                    (1, 1)
                                };

                                let (display_text, detail_text, is_valid) = match target_line {
                                    Some(l) if l >= 1 && l <= max_line => (
                                        format!("Go to line {}, column {}", l, target_col),
                                        format!("Current: line {} • File range: 1-{}", cur_line, max_line),
                                        true,
                                    ),
                                    Some(l) => (
                                        format!("Line {} is out of range (1-{})", l, max_line),
                                        format!("Current: line {}", cur_line),
                                        false,
                                    ),
                                    None => (
                                        format!("Type a line number between 1 and {}...", max_line),
                                        format!("Current: line {}", cur_line),
                                        false,
                                    ),
                                };

                                vec![
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .justify_between()
                                        .h(px(32.0))
                                        .px_3()
                                        .cursor_pointer()
                                        .bg(if is_valid { theme.bg_active } else { theme.bg_raised })
                                        .hover(|s| s.bg(theme.bg_hover))
                                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                            if is_valid {
                                                if let Some(tab) = this.state.active_tab_mut() {
                                                    let row = target_line.unwrap_or(1).saturating_sub(1);
                                                    let col = target_col.saturating_sub(1);
                                                    tab.cursor_row = row;
                                                    tab.cursor_col = col;
                                                    tab.model.set_cursor(row, col);
                                                    tab.scroll_row = row.saturating_sub(10);
                                                    this.state.status_message = format!("Jumped to line {}, column {}", row + 1, col + 1);
                                                }
                                                this.quick_open.close();
                                                cx.notify();
                                            }
                                        }))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .gap_2()
                                                .text_xs()
                                                .child(
                                                    div()
                                                        .px_1p5()
                                                        .py_0p5()
                                                        .rounded(px(3.0))
                                                        .bg(if is_valid { theme.accent.opacity(0.18) } else { theme.bg_card })
                                                        .text_color(if is_valid { theme.accent } else { theme.text_subtle })
                                                        .font_weight(FontWeight::BOLD)
                                                        .child(if is_valid { "LINE" } else { "INFO" })
                                                )
                                                .child(
                                                    div()
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .text_color(if is_valid { theme.text_primary } else { theme.text_muted })
                                                        .child(display_text)
                                                )
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.text_subtle)
                                                .child(detail_text)
                                        )
                                        .into_any_element()
                                ]
                            } else {
                                qo.filtered_files.iter().enumerate().map(|(idx, path)| {
                                    let is_sel = idx == selected_idx;
                                    let p_str = path.clone();
                                    let file_name = std::path::Path::new(path)
                                        .file_name()
                                        .unwrap_or_default()
                                        .to_string_lossy()
                                        .to_string();

                                    let (icon_name, icon_color) = if file_name.ends_with(".rs") {
                                        (crate::ui::icons::IconName::FileCode, rgb(0xe57373))
                                    } else if file_name.ends_with(".ts") || file_name.ends_with(".tsx") || file_name.ends_with(".js") {
                                        (crate::ui::icons::IconName::FileCode, rgb(0x64b5f6))
                                    } else {
                                        (crate::ui::icons::IconName::File, theme.text_subtle)
                                    };

                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .justify_between()
                                        .h(px(28.0))
                                        .px_3()
                                        .cursor_pointer()
                                        .bg(if is_sel { theme.bg_active } else { theme.bg_raised })
                                        .hover(|s| s.bg(theme.bg_hover))
                                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, window, cx| {
                                            this.open_file(&p_str, window, cx);
                                            this.quick_open.close();
                                            cx.notify();
                                        }))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .gap_2()
                                                .text_xs()
                                                .child(crate::ui::icons::ui_icon(icon_name, 13.0, icon_color))
                                                .child(
                                                    div()
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .text_color(if is_sel { theme.text_primary } else { theme.text_muted })
                                                        .child(file_name)
                                                )
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.text_subtle)
                                                .child(path.clone())
                                        )
                                        .into_any_element()
                                }).collect::<Vec<_>>()
                            })
                    )
            )
            .into_any_element()
    )
}

pub fn execute_command(
    view: &mut HadesAppView,
    cmd_id: &str,
    window: &mut Window,
    cx: &mut Context<HadesAppView>,
) {
    if let Some(cmd) = view.commands.find(cmd_id).cloned() {
        let handler = cmd.handler.clone();
        handler(view, window, cx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::prelude::v1::test;

    #[test]
    fn test_quick_open_modes_and_queries() {
        let mut qo = QuickOpenState::default();
        assert!(!qo.is_open);

        qo.query = ">Preferences: Open Settings".to_string();
        assert!(qo.is_command_mode());
        assert!(!qo.is_symbol_mode());
        assert!(!qo.is_workspace_symbol_mode());
        assert!(!qo.is_line_mode());

        qo.query = "@handle_click".to_string();
        assert!(!qo.is_command_mode());
        assert!(qo.is_symbol_mode());
        assert!(!qo.is_workspace_symbol_mode());

        qo.query = "#HadesAppView".to_string();
        assert!(!qo.is_command_mode());
        assert!(!qo.is_symbol_mode());
        assert!(qo.is_workspace_symbol_mode());

        qo.query = ":42:10".to_string();
        assert!(qo.is_line_mode());

        qo.close();
        assert!(!qo.is_open);
        assert!(qo.query.is_empty());
    }
}
