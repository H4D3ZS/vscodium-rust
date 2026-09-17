use crate::app_state::{FocusedPanel, HadesNativeState};
use crate::editor::engine::lsp::{DocumentSymbol, SymbolKind};
use crate::panels::traits::{AuxiliaryTab, WorkbenchPanel};
use crate::theme::Theme;
use crate::ui::icons::{icon_12, ui_icon, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;
use std::collections::HashSet;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct OutlineState {
    pub filter_query: String,
    pub follow_cursor: bool,
    pub expanded_symbols: HashSet<String>,
}

impl Default for OutlineState {
    fn default() -> Self {
        Self {
            filter_query: String::new(),
            follow_cursor: true,
            expanded_symbols: HashSet::new(),
        }
    }
}

impl OutlineState {
    pub fn toggle_expanded(&mut self, id: &str) {
        if self.expanded_symbols.contains(id) {
            self.expanded_symbols.remove(id);
        } else {
            self.expanded_symbols.insert(id.to_string());
        }
    }

    pub fn is_expanded(&self, id: &str) -> bool {
        self.expanded_symbols.contains(id)
    }

    pub fn toggle_follow_cursor(&mut self) {
        self.follow_cursor = !self.follow_cursor;
    }
}

pub struct OutlinePanel;

impl WorkbenchPanel for OutlinePanel {
    fn id(&self) -> &'static str {
        "outline"
    }

    fn title(&self) -> &'static str {
        "Outline"
    }

    fn icon(&self) -> IconName {
        IconName::ListTree
    }

    fn render(&self, state: &HadesNativeState, cx: &mut Context<HadesAppView>) -> AnyElement {
        render_outline_panel(state, cx).into_any_element()
    }
}

impl AuxiliaryTab for OutlinePanel {
    fn id(&self) -> &'static str {
        "outline"
    }

    fn title(&self) -> &'static str {
        "Outline"
    }

    fn icon(&self) -> IconName {
        IconName::ListTree
    }

    fn render(&self, state: &HadesNativeState, cx: &mut Context<HadesAppView>) -> AnyElement {
        render_outline_panel(state, cx).into_any_element()
    }
}

pub fn render_outline_panel(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;

    let (active_path, cursor_row, symbols) = if let Some(tab) = state.active_tab() {
        let syms = state
            .vsx
            .language_services
            .get_document_symbols(&tab.lines, Path::new(&tab.path));
        (tab.path.clone(), tab.cursor_row, syms)
    } else {
        (String::new(), 0, Vec::new())
    };

    let filter = state.outline.filter_query.to_lowercase();
    let filtered_symbols: Vec<&DocumentSymbol> = symbols
        .iter()
        .filter(|s| {
            filter.is_empty()
                || s.name.to_lowercase().contains(&filter)
                || s.kind.badge().contains(&filter)
        })
        .collect();

    let total_count = symbols.len();

    div()
        .flex()
        .flex_col()
        .size_full()
        .bg(theme.bg_sidebar)
        // Outline Header
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
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_muted)
                                .child("OUTLINE"),
                        )
                        .child(
                            div()
                                .px_1p5()
                                .py_0p5()
                                .rounded(px(3.0))
                                .bg(theme.bg_raised)
                                .text_xs()
                                .text_color(theme.accent)
                                .font_weight(FontWeight::BOLD)
                                .child(format!("{total_count}")),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1p5()
                        // Follow Cursor Toggle
                        .child(
                            div()
                                .p_1()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .bg(if state.outline.follow_cursor {
                                    rgba(0x38bdf820)
                                } else {
                                    rgba(0x00000000)
                                })
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _e, _w, cx| {
                                        this.state.outline.toggle_follow_cursor();
                                        cx.notify();
                                    }),
                                )
                                .child(ui_icon(
                                    IconName::Radio,
                                    12.0,
                                    if state.outline.follow_cursor {
                                        theme.accent
                                    } else {
                                        theme.text_subtle
                                    },
                                )),
                        ),
                ),
        )
        // Search / Filter input
        .child(
            div()
                .px_2()
                .py_1p5()
                .border_b_1()
                .border_color(theme.border_subtle)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .h(px(24.0))
                        .px_2()
                        .bg(theme.bg_input)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .rounded(px(4.0))
                        .child(ui_icon(IconName::Search, 11.0, theme.text_subtle))
                        .child(
                            div()
                                .text_xs()
                                .text_color(if state.outline.filter_query.is_empty() {
                                    theme.text_subtle
                                } else {
                                    theme.text_primary
                                })
                                .child(if state.outline.filter_query.is_empty() {
                                    "Filter symbols (e.g. fn, struct)...".to_string()
                                } else {
                                    state.outline.filter_query.clone()
                                }),
                        ),
                ),
        )
        // Symbol tree content
        .child(
            div()
                .id("outline_symbol_tree_scroll")
                .flex()
                .flex_col()
                .flex_1()
                .overflow_y_scroll()
                .py_1()
                .children(if active_path.is_empty() {
                    vec![div()
                        .p_4()
                        .text_xs()
                        .text_color(theme.text_subtle)
                        .child("No active editor open")
                        .into_any_element()]
                } else if filtered_symbols.is_empty() {
                    vec![div()
                        .p_4()
                        .text_xs()
                        .text_color(theme.text_subtle)
                        .child(if total_count == 0 {
                            "No symbols detected in this document"
                        } else {
                            "No symbols match filter query"
                        })
                        .into_any_element()]
                } else {
                    filtered_symbols
                        .into_iter()
                        .map(|sym| render_symbol_tree_node(sym, 0, cursor_row, theme, cx))
                        .collect()
                }),
        )
}

fn render_symbol_tree_node(
    sym: &DocumentSymbol,
    depth: usize,
    cursor_row: usize,
    theme: &Theme,
    cx: &mut Context<HadesAppView>,
) -> AnyElement {
    let start_line = sym.range.start.line;
    let end_line = sym.range.end.line;
    let is_active = cursor_row >= start_line && cursor_row <= end_line;
    let has_children = !sym.children.is_empty();

    let (badge_bg, badge_fg) = match sym.kind {
        SymbolKind::Function | SymbolKind::Method | SymbolKind::Constructor => {
            (rgba(0x38bdf820), rgb(0x38bdf8))
        }
        SymbolKind::Struct | SymbolKind::Class => (rgba(0xa78bfa20), rgb(0xa78bfa)),
        SymbolKind::Enum | SymbolKind::EnumMember => (rgba(0xfbbf2420), rgb(0xfbbf24)),
        SymbolKind::Interface => (rgba(0xf43f5e20), rgb(0xf43f5e)),
        SymbolKind::Constant => (rgba(0x34d39920), rgb(0x34d399)),
        SymbolKind::Module | SymbolKind::Namespace | SymbolKind::Package => {
            (rgba(0x60a5fa20), rgb(0x60a5fa))
        }
        _ => (rgba(0x94a3b820), rgb(0x94a3b8)),
    };

    let target_row = start_line;
    let target_col = sym.range.start.character;
    let sym_name_clone = sym.name.clone();

    div()
        .flex()
        .flex_col()
        .w_full()
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .h(px(22.0))
                .px_2()
                .pl(px(8.0 + (depth as f32 * 14.0)))
                .cursor_pointer()
                .bg(if is_active {
                    theme.active_line_bg
                } else {
                    rgba(0x00000000)
                })
                .border_l_2()
                .border_color(if is_active {
                    theme.accent
                } else {
                    rgba(0x00000000)
                })
                .hover(|s| s.bg(theme.bg_hover))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _e, _w, cx| {
                        if let Some(tab) = this.state.active_tab_mut() {
                            tab.cursor_row = target_row;
                            tab.cursor_col = target_col;
                            tab.scroll_to(target_row.saturating_sub(10));
                            tab.model.set_cursor(target_row, target_col);
                        }
                        this.state.focused_panel = FocusedPanel::Editor;
                        this.state.status_message = format!(
                            "Outline: jump to {} (line {})",
                            sym_name_clone,
                            target_row + 1
                        );
                        cx.notify();
                    }),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1p5()
                        .overflow_hidden()
                        // Chevron or spacer
                        .child(if has_children {
                            icon_12(IconName::ChevronDown, theme.text_subtle).into_any_element()
                        } else {
                            div().w(px(12.0)).into_any_element()
                        })
                        // Kind Badge
                        .child(
                            div()
                                .px_1()
                                .py_0p5()
                                .rounded(px(2.0))
                                .bg(badge_bg)
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(badge_fg)
                                .child(sym.kind.badge()),
                        )
                        // Symbol Name
                        .child(
                            div()
                                .text_xs()
                                .text_color(if is_active {
                                    theme.accent
                                } else {
                                    theme.text_primary
                                })
                                .font_weight(if is_active {
                                    FontWeight::BOLD
                                } else {
                                    FontWeight::NORMAL
                                })
                                .child(sym.name.clone()),
                        ),
                )
                // Line Number Tag
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_subtle)
                        .child(format!(":{}", start_line + 1)),
                ),
        )
        // Nested children
        .children(
            sym.children
                .iter()
                .map(|child| render_symbol_tree_node(child, depth + 1, cursor_row, theme, cx)),
        )
        .into_any_element()
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::prelude::v1::test;

    #[test]
    fn test_outline_state_defaults_and_toggle() {
        let mut state = OutlineState::default();
        assert!(state.follow_cursor);
        assert!(state.filter_query.is_empty());
        assert!(!state.is_expanded("foo"));

        state.toggle_expanded("foo");
        assert!(state.is_expanded("foo"));

        state.toggle_expanded("foo");
        assert!(!state.is_expanded("foo"));

        state.toggle_follow_cursor();
        assert!(!state.follow_cursor);
    }
}
