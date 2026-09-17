use crate::app_state::{FocusedPanel, HadesNativeState, OpenTab};
use crate::editor::engine::bracket_pairs::{self, BracketPairState, ColorizedBracket};
use crate::editor::engine::cursor::Position;
use crate::editor::engine::multi_cursor::{self, MultiCursorState, OccurrenceHighlight};
use crate::editor::engine::vsx::languages::HoverInfo;
use crate::editor::engine::{TokenType, Tokenizer};
use crate::theme::Theme;
use crate::ui::icons::{icon_12, icon_14, ui_icon, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;

pub fn render_native_editor(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    render_editor_pane(state, 0, cx)
}

pub fn render_editor_pane(
    state: &HadesNativeState,
    pane_idx: usize,
    cx: &mut Context<HadesAppView>,
) -> AnyElement {
    let theme = &state.theme;
    let is_focused = state.focused_panel == FocusedPanel::Editor && state.active_pane == pane_idx;

    let target_tab = if pane_idx == 0 {
        state.active_tab_idx.and_then(|idx| state.tabs.get(idx))
    } else {
        state
            .active_secondary_tab_idx
            .and_then(|idx| state.secondary_tabs.get(idx))
    };

    if let Some(tab) = target_tab {
        return div()
            .size_full()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, _event, _window, cx| {
                    this.state.focused_panel = FocusedPanel::Editor;
                    this.state.focus_pane(pane_idx);
                    cx.notify();
                }),
            )
            .child(render_active_editor_tab(
                tab,
                is_focused,
                &state.hover_info,
                state.hover_row,
                &state.code_actions,
                &state.rename,
                &state.references,
                &state.sticky_scroll,
                &state.minimap,
                &state.bracket_pairs,
                &state.multi_cursor,
                &state.debug,
                &state.find_query,
                &state.markdown_preview,
                &state.minimap_hover,
                theme,
                cx,
            ))
            .into_any_element();
    }

    div()
        .size_full()
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this, _event, _window, cx| {
                this.state.focused_panel = FocusedPanel::Editor;
                this.state.focus_pane(pane_idx);
                cx.notify();
            }),
        )
        .child(render_empty_editor(theme))
        .into_any_element()
}

fn render_active_editor_tab(
    tab: &OpenTab,
    is_focused: bool,
    hover_info: &Option<HoverInfo>,
    hover_row: usize,
    code_actions: &crate::editor::engine::code_actions::CodeActionState,
    rename: &crate::editor::engine::rename::RenameState,
    references: &crate::editor::engine::references::ReferencesState,
    sticky_scroll: &crate::editor::engine::sticky_scroll::StickyScrollModel,
    minimap: &crate::editor::engine::minimap::MinimapModel,
    bracket_pair_state: &BracketPairState,
    multi_cursor_state: &MultiCursorState,
    debug: &crate::panels::DebugState,
    find_query: &str,
    markdown_preview: &crate::panels::MarkdownPreviewState,
    minimap_hover: &Option<(usize, f32)>,
    theme: &Theme,
    cx: &mut Context<HadesAppView>,
) -> AnyElement {
    let cursor_row = tab.cursor_row;
    let cursor_col = tab.cursor_col;
    let cursors_list = tab.model.cursors.all_cursors();

    div()
        .relative()
        .flex()
        .flex_col()
        .size_full()
        .bg(theme.bg_editor)
        .cursor_text()
        .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
            this.state.focused_panel = FocusedPanel::Editor;
            cx.notify();
        }))
        // Breadcrumbs Bar
        .child({
            let norm_path = tab.path.replace('\\', "/");
            let mut segments: Vec<String> = norm_path.split('/').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
            if let Some(first) = segments.first() {
                if first.ends_with(':') {
                    segments.remove(0);
                }
            }
            let total_segs = segments.len();
            let display_folders: Vec<String> = if total_segs > 1 {
                let start = total_segs.saturating_sub(4);
                segments[start..total_segs - 1].to_vec()
            } else {
                Vec::new()
            };

            let crumb_symbols = crate::editor::engine::sticky_scroll::compute_breadcrumbs_trail(
                &tab.lines,
                &tab.model.folding.ranges,
                tab.cursor_row,
            );

            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .h(px(26.0))
                .w_full()
                .bg(theme.bg_editor)
                .border_b_1()
                .border_color(theme.border_subtle)
                .px_3()
                .text_xs()
                .on_mouse_move(cx.listener(|this, _event: &MouseMoveEvent, _window, cx| {
                    if this.state.minimap_hover.is_some() {
                        this.state.minimap_hover = None;
                        cx.notify();
                    }
                }))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1()
                        .overflow_x_hidden()
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1()
                                .px_1()
                                .py_0p5()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .text_color(theme.text_subtle)
                                .child(ui_icon(IconName::Folder, 11.0, theme.text_muted))
                                .child("vscodium-rust")
                        )
                        .children(display_folders.into_iter().map(|folder| {
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1()
                                .child(div().text_color(theme.text_subtle).child("›"))
                                .child(
                                    div()
                                        .px_1()
                                        .py_0p5()
                                        .rounded(px(3.0))
                                        .cursor_pointer()
                                        .hover(|s| s.bg(theme.bg_hover).text_color(theme.text_primary))
                                        .text_color(theme.text_subtle)
                                        .child(folder)
                                )
                        }))
                        .child(div().text_color(theme.text_subtle).child("›"))
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1p5()
                                .px_1()
                                .py_0p5()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .child(ui_icon(
                                    if tab.path.ends_with(".rs") {
                                        IconName::FileCode
                                    } else if tab.path.ends_with(".md") {
                                        IconName::FileText
                                    } else {
                                        IconName::File
                                    },
                                    12.0,
                                    theme.accent,
                                ))
                                .child(
                                    div()
                                        .text_color(theme.text_primary)
                                        .font_weight(FontWeight::MEDIUM)
                                        .child(tab.title.clone())
                                )
                                .children(tab.dirty.then(|| {
                                    div()
                                        .w(px(6.0))
                                        .h(px(6.0))
                                        .rounded_full()
                                        .bg(theme.accent)
                                }))
                        )
                        .children(crumb_symbols.into_iter().map(|sym| {
                            let badge_color = match sym.badge {
                                "fn" | "function" | "def" => theme.accent,
                                "struct" | "class" | "interface" => theme.status_green,
                                "impl" => rgb(0xa855f7),
                                "enum" => theme.status_yellow,
                                "trait" => rgb(0xec4899),
                                _ => theme.text_muted,
                            };
                            let target_row = sym.row;
                            let text_label = sym.text.clone();

                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1()
                                .child(div().text_color(theme.text_subtle).child("›"))
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_1()
                                        .px_1p5()
                                        .py_0p5()
                                        .rounded(px(3.0))
                                        .cursor_pointer()
                                        .hover(|s| s.bg(theme.bg_hover))
                                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _e, _w, cx| {
                                            if let Some(tab) = this.state.active_tab_mut() {
                                                tab.cursor_row = target_row;
                                                tab.cursor_col = 0;
                                                tab.model.set_cursor(target_row, 0);
                                                tab.scroll_row = target_row.saturating_sub(10);
                                                this.state.status_message = format!("Navigated to {}", text_label);
                                                cx.notify();
                                            }
                                        }))
                                        .child(
                                            div()
                                                .px_1()
                                                .py_0p5()
                                                .rounded(px(2.0))
                                                .bg(badge_color.opacity(0.18))
                                                .border_1()
                                                .border_color(badge_color.opacity(0.4))
                                                .text_xs()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(badge_color)
                                                .child(sym.badge)
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.text_primary)
                                                .font_weight(FontWeight::NORMAL)
                                                .child(sym.text)
                                        )
                                )
                        }))
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
                                .rounded_sm()
                                .bg(theme.bg_hover)
                                .text_xs()
                                .text_color(theme.accent)
                                .child(tab.model.language.to_uppercase())
                        )
                        .children(tab.model.wrap.enabled.then(|| {
                            div()
                                .px_1p5()
                                .py_0p5()
                                .rounded_sm()
                                .bg(theme.accent)
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.text_primary)
                                .cursor_pointer()
                                .child("Wrap: On")
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _e, _w, cx| {
                                    if let Some(tab) = this.state.active_tab_mut() {
                                        tab.model.toggle_word_wrap();
                                        cx.notify();
                                    }
                                }))
                        }))
                        .child(
                            div()
                                .px_1p5()
                                .py_0p5()
                                .rounded_sm()
                                .bg(if sticky_scroll.enabled { theme.bg_hover } else { theme.bg_editor })
                                .text_xs()
                                .text_color(if sticky_scroll.enabled { theme.accent } else { theme.text_subtle })
                                .cursor_pointer()
                                .child("Sticky")
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _e, _w, cx| {
                                    this.state.toggle_sticky_scroll();
                                    cx.notify();
                                }))
                        )
                        .child(
                            div()
                                .px_1p5()
                                .py_0p5()
                                .rounded_sm()
                                .bg(if minimap.enabled { theme.bg_hover } else { theme.bg_editor })
                                .text_xs()
                                .text_color(if minimap.enabled { theme.accent } else { theme.text_subtle })
                                .cursor_pointer()
                                .child("Map")
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _e, _w, cx| {
                                    this.state.toggle_minimap();
                                    cx.notify();
                                }))
                        )
                        .child(
                            div()
                                .px_1p5()
                                .py_0p5()
                                .rounded_sm()
                                .bg(if bracket_pair_state.colorization_enabled { theme.bg_hover } else { theme.bg_editor })
                                .text_xs()
                                .text_color(if bracket_pair_state.colorization_enabled { theme.accent } else { theme.text_subtle })
                                .cursor_pointer()
                                .child("⟨⟩")
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _e, _w, cx| {
                                    this.state.bracket_pairs.toggle_colorization();
                                    cx.notify();
                                }))
                        )
                        .child(
                            div()
                                .px_1p5()
                                .py_0p5()
                                .rounded_sm()
                                .bg(if bracket_pair_state.indent_guides.enabled { theme.bg_hover } else { theme.bg_editor })
                                .text_xs()
                                .text_color(if bracket_pair_state.indent_guides.enabled { theme.accent } else { theme.text_subtle })
                                .cursor_pointer()
                                .child("⫼")
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _e, _w, cx| {
                                    this.state.bracket_pairs.toggle_indent_guides();
                                    cx.notify();
                                }))
                        )
                        .child(
                            div()
                                .px_1p5()
                                .py_0p5()
                                .rounded_sm()
                                .bg(if multi_cursor_state.highlight_occurrences { theme.bg_hover } else { theme.bg_editor })
                                .text_xs()
                                .text_color(if multi_cursor_state.highlight_occurrences { theme.accent } else { theme.text_subtle })
                                .cursor_pointer()
                                .child("HL")
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _e, _w, cx| {
                                    this.state.toggle_word_highlight();
                                    cx.notify();
                                }))
                        )
                        .children((tab.path.ends_with(".md") || tab.path.ends_with(".markdown")).then(|| {
                            let is_active = markdown_preview.is_open;
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1()
                                .px_1p5()
                                .py_0p5()
                                .rounded_sm()
                                .bg(if is_active { theme.accent.opacity(0.2) } else { theme.bg_hover })
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(if is_active { theme.accent } else { theme.text_primary })
                                .cursor_pointer()
                                .child(ui_icon(IconName::FileText, 11.0, if is_active { theme.accent } else { theme.text_muted }))
                                .child("Preview")
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _e, _w, cx| {
                                    this.state.toggle_markdown_preview();
                                    cx.notify();
                                }))
                        }))
                        .children((!tab.model.cursors.secondary.is_empty()).then(|| {
                            let count = tab.model.cursors.all_selections().len();
                            div()
                                .px_1p5()
                                .py_0p5()
                                .rounded_sm()
                                .bg(theme.accent.opacity(0.18))
                                .border_1()
                                .border_color(theme.accent.opacity(0.4))
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.accent)
                                .cursor_pointer()
                                .child(format!("{} Carets (Esc)", count))
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _e, _w, cx| {
                                    if let Some(tab) = this.state.active_tab_mut() {
                                        tab.clear_secondary_cursors();
                                    }
                                    cx.notify();
                                }))
                        }))
                )
        })
        // Editor Surface (Gutter + Code Lines + Scrollbar + Minimap)
        .child({
            let total_lines = tab.lines.len().max(1);
            let visible_count = 50;
            let scroll_row = tab.scroll_row.min(total_lines.saturating_sub(1));
            let end_row = (scroll_row + visible_count).min(total_lines);
            let sticky_headers = sticky_scroll.compute_headers(&tab.lines, &tab.model.folding.ranges, scroll_row);

            // Compute bracket pair colorization for visible range
            let bracket_result = if bracket_pair_state.colorization_enabled {
                Some(bracket_pairs::compute_bracket_colorization(&tab.lines, scroll_row, end_row))
            } else {
                None
            };

            // Compute indent guides for visible range
            let indent_guide_result = bracket_pairs::compute_indent_guides(
                &tab.lines,
                scroll_row,
                end_row,
                cursor_row,
                &bracket_pair_state.indent_guides,
            );

            // Compute word occurrence highlights for visible range
            let occurrence_highlights = if multi_cursor_state.highlight_occurrences {
                multi_cursor::compute_occurrence_highlights(&tab.lines, &tab.model, scroll_row, end_row)
            } else {
                Vec::new()
            };

            let scroll_pct = if total_lines > visible_count {
                scroll_row as f32 / (total_lines - visible_count).max(1) as f32
            } else {
                0.0
            };
            let thumb_h_px = ((visible_count as f32 / total_lines as f32) * 400.0).clamp(24.0, 380.0);
            let thumb_top_px = scroll_pct * (400.0 - thumb_h_px);

            div()
                .relative()
                .flex()
                .flex_row()
                .flex_1()
                .w_full()
                .overflow_hidden()
                .on_scroll_wheel(cx.listener(|this, event: &ScrollWheelEvent, _window, cx| {
                    if this.state.minimap_hover.is_some() {
                        this.state.minimap_hover = None;
                    }
                    if let Some(tab) = this.state.active_tab_mut() {
                        let dy = match event.delta {
                            ScrollDelta::Lines(d) => -d.y * 3.0,
                            ScrollDelta::Pixels(d) => {
                                let p: f32 = d.y.into();
                                -p / 12.0
                            }
                        };
                        let step = if dy.abs() < 1.0 { if dy > 0.0 { 1 } else { -1 } } else { dy as i32 };
                        tab.scroll_by(step);
                        cx.notify();
                    }
                }))
                .on_mouse_exit(cx.listener(|this, _event, _window, cx| {
                    if this.state.minimap_hover.is_some() {
                        this.state.minimap_hover = None;
                        cx.notify();
                    }
                }))
                // Code Viewport (Gutter + Code Lines)
                .child(
                    div()
                        .id("native_editor_viewport_scroll")
                        .flex()
                        .flex_row()
                        .flex_1()
                        .h_full()
                        .py_2()
                        .cursor_text()
                        .on_mouse_move(cx.listener(|this, _event: &MouseMoveEvent, _window, cx| {
                            if this.state.minimap_hover.is_some() {
                                this.state.minimap_hover = None;
                                cx.notify();
                            }
                        }))
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                            this.state.minimap_hover = None;
                            this.state.focused_panel = FocusedPanel::Editor;
                            cx.notify();
                        }))
                        // Line Numbers Gutter with Folding toggles
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .w(px(68.0))
                                .pr_2()
                                .items_end()
                                .text_sm()
                                .font_family("Cascadia Code, Consolas, monospace")
                                .text_color(theme.text_subtle)
                                .children((scroll_row..end_row).map(|line_idx| {
                                    let num = line_idx + 1;
                                    let is_cursor_line = line_idx == cursor_row;
                                    let fold_range = tab.model.folding.range_at(line_idx);
                                    let has_bp = debug.has_breakpoint_at(&tab.path, num);
                                    let tab_path_clone = tab.path.clone();
                                    let code_lens = tab.model.code_lenses.iter().find(|l| l.row == line_idx);

                                    div()
                                        .flex()
                                        .flex_col()
                                        .w_full()
                                        .children(code_lens.map(|_| {
                                            div().h(px(16.0))
                                        }))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .justify_between()
                                                .w_full()
                                                .h(px(22.0))
                                                .cursor_pointer()
                                                .text_color(if is_cursor_line { theme.accent } else { theme.text_subtle })
                                                .font_weight(if is_cursor_line { FontWeight::BOLD } else { FontWeight::NORMAL })
                                                // Breakpoint Dot Target (Click to toggle)
                                                .child(
                                                    div()
                                                        .w(px(12.0))
                                                        .h(px(22.0))
                                                        .flex()
                                                        .items_center()
                                                        .justify_center()
                                                        .cursor_pointer()
                                                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _e, _w, cx| {
                                                            let added = this.state.debug.toggle_breakpoint(&tab_path_clone, num);
                                                            if added {
                                                                this.state.toast_manager.push_success(&format!("Breakpoint added at line {}", num));
                                                            } else {
                                                                this.state.toast_manager.push_info(&format!("Breakpoint removed at line {}", num));
                                                            }
                                                            cx.notify();
                                                        }))
                                                        .child(if has_bp {
                                                            div()
                                                                .w(px(8.0))
                                                                .h(px(8.0))
                                                                .rounded_full()
                                                                .bg(theme.status_red)
                                                                .into_any_element()
                                                        } else {
                                                            div()
                                                                .w(px(8.0))
                                                                .h(px(8.0))
                                                                .rounded_full()
                                                                .hover(|s| s.bg(theme.status_red.opacity(0.35)))
                                                                .into_any_element()
                                                        })
                                                )
                                                // Folding toggle chevron
                                                .child(
                                                    div()
                                                        .w(px(14.0))
                                                        .children(fold_range.map(|fr| {
                                                            let is_collapsed = fr.is_collapsed;
                                                            div()
                                                                .cursor_pointer()
                                                                .hover(|s| s.text_color(theme.text_primary))
                                                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _e, _w, cx| {
                                                                    if let Some(tab) = this.state.active_tab_mut() {
                                                                        tab.model.folding.toggle_fold(line_idx);
                                                                        cx.notify();
                                                                    }
                                                                }))
                                                                .child(if is_collapsed {
                                                                    icon_12(IconName::ChevronRight, theme.text_muted).into_any_element()
                                                                } else {
                                                                    icon_12(IconName::ChevronDown, theme.text_subtle).into_any_element()
                                                                })
                                                        }))
                                                )
                                                // Line number & Diagnostic indicator
                                                .child({
                                                    let diag = tab.model.diagnostics.iter().find(|d| d.row == line_idx);
                                                    div()
                                                        .flex()
                                                        .flex_row()
                                                        .items_center()
                                                        .gap_1()
                                                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, event: &MouseDownEvent, _window, cx| {
                                                            if let Some(tab) = this.state.active_tab_mut() {
                                                                if event.modifiers.alt {
                                                                    tab.model.cursors.add_cursor(Position::new(line_idx, 0));
                                                                    let count = tab.model.cursors.all_selections().len();
                                                                    this.state.status_message = format!("Multi-Cursor: {} selections", count);
                                                                } else {
                                                                    tab.cursor_row = line_idx;
                                                                    tab.cursor_col = tab.current_line().chars().count().min(tab.cursor_col);
                                                                    tab.model.set_cursor(tab.cursor_row, tab.cursor_col);
                                                                }
                                                            }
                                                            this.state.focused_panel = FocusedPanel::Editor;
                                                            cx.notify();
                                                        }))
                                                        .children(diag.map(|d| {
                                                            let dot_color = match d.severity {
                                                                crate::editor::engine::decorations::DiagnosticSeverity::Error => theme.status_red,
                                                                crate::editor::engine::decorations::DiagnosticSeverity::Warning => theme.status_yellow,
                                                                _ => theme.status_blue,
                                                            };
                                                            div()
                                                                .w(px(4.0))
                                                                .h(px(4.0))
                                                                .rounded_full()
                                                                .bg(dot_color)
                                                        }))
                                                        .child(format!("{num}"))
                                                })
                                        )
                                }))
                        )
                        // Code Content Lines with syntax token highlights and live multi-carets
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .flex_1()
                                .pl_2()
                                .text_sm()
                                .font_family("Cascadia Code, Consolas, monospace")
                                .children((scroll_row..end_row).map(|line_idx| {
                                    if tab.model.folding.is_line_hidden(line_idx) {
                                        return div().into_any_element();
                                    }

                                    let line = tab.lines.get(line_idx).map(|s| s.as_str()).unwrap_or("");
                                    let is_cursor_line = line_idx == cursor_row;

                                    let cursors_here: Vec<usize> = cursors_list
                                        .iter()
                                        .filter(|pos| pos.row == line_idx)
                                        .map(|pos| pos.col)
                                        .collect();

                                    let ghost = if tab.model.ghost_text.is_active && tab.model.ghost_text.row == line_idx {
                                        Some((tab.model.ghost_text.col, tab.model.ghost_text.text.as_str()))
                                    } else {
                                        None
                                    };

                                    let cached_tokens = tab.line_tokens.get(line_idx).map(|t| t.as_slice());

                                    let inlays_here: Vec<crate::editor::engine::inlay_hints::InlayHint> = tab
                                        .model
                                        .inlay_hints
                                        .iter()
                                        .filter(|h| h.row == line_idx)
                                        .cloned()
                                        .collect();

                                    // Get bracket colorization data for this line
                                    let brackets_here: Vec<ColorizedBracket> = bracket_result
                                        .as_ref()
                                        .and_then(|r| r.lines.get(line_idx.saturating_sub(scroll_row)))
                                        .cloned()
                                        .unwrap_or_default();

                                    // Get indent guides for this line
                                    let guides_here = indent_guide_result
                                        .lines
                                        .get(line_idx.saturating_sub(scroll_row))
                                        .cloned()
                                        .unwrap_or_default();

                                    let line_highlights: Vec<OccurrenceHighlight> = occurrence_highlights
                                        .iter()
                                        .filter(|h| h.row == line_idx)
                                        .cloned()
                                        .collect();

                                    let line_selections: Vec<(usize, usize)> = tab.model.cursors
                                        .all_selections()
                                        .iter()
                                        .filter_map(|s| {
                                            if s.is_empty() {
                                                return None;
                                            }
                                            let st = s.start();
                                            let en = s.end();
                                            if line_idx >= st.row && line_idx <= en.row {
                                                let start_c = if line_idx == st.row { st.col } else { 0 };
                                                let end_c = if line_idx == en.row { en.col } else { line.chars().count() };
                                                if start_c < end_c {
                                                    Some((start_c, end_c))
                                                } else {
                                                    None
                                                }
                                            } else {
                                                None
                                            }
                                        })
                                        .collect();

                                    let code_lens = tab.model.code_lenses.iter().find(|l| l.row == line_idx);

                                    div()
                                        .flex()
                                        .flex_col()
                                        .w_full()
                                        // Interactive CodeLens Header
                                        .children(code_lens.map(|lens| {
                                            let label = lens.label.clone();
                                            div()
                                                .flex()
                                                .items_center()
                                                .h(px(16.0))
                                                .text_xs()
                                                .text_color(theme.text_subtle)
                                                .opacity(0.65)
                                                .cursor_pointer()
                                                .hover(|s| s.opacity(1.0).text_color(theme.accent))
                                                .child(label)
                                        }))
                                        .child(
                                            div()
                                                .relative()
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .h(px(22.0))
                                                .w_full()
                                                .cursor_text()
                                                .bg(if is_cursor_line { theme.active_line_bg } else { theme.bg_editor })
                                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, event: &MouseDownEvent, _window, cx| {
                                                    if let Some(tab) = this.state.active_tab_mut() {
                                                        let max_col = tab.current_line().chars().count();
                                                        let target_col = tab.cursor_col.min(max_col);
                                                        if event.modifiers.alt {
                                                            tab.model.cursors.add_cursor(Position::new(line_idx, target_col));
                                                            let count = tab.model.cursors.all_selections().len();
                                                            this.state.status_message = format!("Multi-Cursor: {} selections", count);
                                                        } else {
                                                            tab.cursor_row = line_idx;
                                                            tab.cursor_col = target_col;
                                                            tab.model.set_cursor(tab.cursor_row, tab.cursor_col);
                                                        }
                                                    }
                                                    this.state.focused_panel = FocusedPanel::Editor;
                                                    cx.notify();
                                                }))
                                                // Indent Guide Vertical Lines (Subtle Monochrome VS Code Parity)
                                                .children(guides_here.iter().map(|guide| {
                                                    let guide_left = guide.column as f32 * 8.4;
                                                    let guide_color = if guide.is_active {
                                                        rgba(0xffffff26)
                                                    } else {
                                                        rgba(0xffffff0e)
                                                    };
                                                    div()
                                                        .absolute()
                                                        .left(px(guide_left))
                                                        .top(px(0.0))
                                                        .bottom(px(0.0))
                                                        .w(px(1.0))
                                                        .bg(guide_color)
                                                }))
                                                // Active Selection Backgrounds
                                                .children(line_selections.iter().map(|&(s_col, e_col)| {
                                                    let sel_left = s_col as f32 * 8.4;
                                                    let sel_width = (e_col - s_col) as f32 * 8.4;
                                                    div()
                                                        .absolute()
                                                        .left(px(sel_left))
                                                        .top(px(1.0))
                                                        .bottom(px(1.0))
                                                        .w(px(sel_width.max(3.0)))
                                                        .bg(rgba(0x3b82f644))
                                                        .rounded(px(2.0))
                                                }))
                                                // Passive Word Occurrence Highlights
                                                .children(line_highlights.iter().filter(|h| !h.is_active_selection).map(|h| {
                                                    let occ_left = h.start_col as f32 * 8.4;
                                                    let occ_width = (h.end_col - h.start_col) as f32 * 8.4;
                                                    div()
                                                        .absolute()
                                                        .left(px(occ_left))
                                                        .top(px(1.0))
                                                        .bottom(px(1.0))
                                                        .w(px(occ_width.max(3.0)))
                                                        .bg(rgba(0xffffff0e))
                                                        .border_1()
                                                        .border_color(rgba(0xffffff28))
                                                        .rounded(px(2.0))
                                                }))
                                                // Multi-Caret Indicators positioned at exact column coordinates
                                                .children(cursors_here.iter().map(|&col| {
                                                    if is_focused {
                                                        let caret_left = col as f32 * 8.4;
                                                        div()
                                                            .absolute()
                                                            .left(px(caret_left))
                                                            .top(px(2.5))
                                                            .h(px(17.0))
                                                            .w(px(2.0))
                                                            .bg(theme.accent)
                                                            .into_any_element()
                                                    } else {
                                                        div().into_any_element()
                                                    }
                                                }))
                                                .child(render_monaco_line(
                                                    line,
                                                    &tab.model.language,
                                                    cached_tokens,
                                                    &cursors_here,
                                                    ghost,
                                                    &inlays_here,
                                                    &brackets_here,
                                                    is_focused,
                                                    theme,
                                                ))
                                        )
                                        .into_any_element()
                                }))
                        )
                )
                // Floating IntelliSense Completion Popup
                .children(tab.model.completions.is_open.then(|| {
                    render_completion_popup(&tab.model.completions, cursor_row, cursor_col, scroll_row, theme, cx)
                }))
                // Floating Quick Fix / Code Action Menu (Ctrl+.)
                .children(code_actions.is_open.then(|| {
                    render_code_action_menu(code_actions, cursor_row, cursor_col, scroll_row, theme, cx)
                }))
                // Inline Symbol Rename Dialog (F2)
                .children(rename.is_open.then(|| {
                    render_rename_dialog(rename, cursor_row, cursor_col, scroll_row, theme, cx)
                }))
                // References Peek Overlay (Shift+F12)
                .children(references.is_open.then(|| {
                    render_references_peek(references, cursor_row, cursor_col, scroll_row, theme, cx)
                }))
                // Floating Hover Tooltip
                .children(hover_info.as_ref().map(|info| {
                    render_hover_tooltip(info, hover_row, scroll_row, theme)
                }))
                // Interactive Scrollbar Gutter (VS Code Parity)
                .child(
                    div()
                        .relative()
                        .w(px(14.0))
                        .h_full()
                        .bg(theme.bg_editor)
                        .cursor_pointer()
                        .hover(|s| s.bg(theme.bg_hover))
                        .on_mouse_move(cx.listener(|this, _event: &MouseMoveEvent, _window, cx| {
                            if this.state.minimap_hover.is_some() {
                                this.state.minimap_hover = None;
                                cx.notify();
                            }
                        }))
                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, event: &MouseDownEvent, _window, cx| {
                            let y: f32 = event.position.y.into();
                            let pct = ((y - 80.0) / 400.0).clamp(0.0, 1.0);
                            if let Some(tab) = this.state.active_tab_mut() {
                                let max_s = tab.lines.len().saturating_sub(visible_count);
                                tab.scroll_to(((max_s as f32) * pct) as usize);
                                cx.notify();
                            }
                        }))
                        // Draggable/Visible Scrollbar Thumb
                        .child(
                            div()
                                .absolute()
                                .top(px(thumb_top_px))
                                .h(px(thumb_h_px))
                                .w(px(8.0))
                                .left(px(3.0))
                                .rounded(px(4.0))
                                .bg(theme.text_subtle)
                                .opacity(0.35)
                                .hover(|s| s.opacity(0.8).bg(theme.accent))
                        )
                )
                // Interactive Monaco Minimap Column
                .children((minimap.enabled).then(|| {
                    let (error_lines, warning_lines): (Vec<usize>, Vec<usize>) = {
                        let mut errs = Vec::new();
                        let mut warns = Vec::new();
                        for d in &tab.model.diagnostics {
                            match d.severity {
                                crate::editor::engine::DiagnosticSeverity::Error => errs.push(d.row),
                                crate::editor::engine::DiagnosticSeverity::Warning => warns.push(d.row),
                                _ => {}
                            }
                        }
                        (errs, warns)
                    };

                    let search_match_lines: Vec<usize> = if !find_query.is_empty() {
                        let q = find_query.to_lowercase();
                        tab.lines
                            .iter()
                            .enumerate()
                            .filter_map(|(idx, line)| {
                                if line.to_lowercase().contains(&q) {
                                    Some(idx)
                                } else {
                                    None
                                }
                            })
                            .collect()
                    } else {
                        Vec::new()
                    };

                    let track_lines = minimap.compute_lines(
                        &tab.lines,
                        cursor_row,
                        &error_lines,
                        &warning_lines,
                        &search_match_lines,
                        80,
                    );

                    let (lens_top, lens_h) = crate::editor::engine::minimap::MinimapModel::compute_viewport_lens(
                        total_lines,
                        scroll_row,
                        visible_count,
                        400.0,
                    );

                    div()
                        .relative()
                        .flex()
                        .flex_col()
                        .w(px(minimap.width_px))
                        .h_full()
                        .bg(theme.bg_editor)
                        .border_l_1()
                        .border_color(theme.border_subtle)
                        .p_1()
                        .overflow_hidden()
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, event: &MouseDownEvent, _window, cx| {
                            let pos_y: f32 = event.position.y.into();
                            let track_y = (pos_y - 94.0).max(0.0);
                            if let Some(tab) = this.state.active_tab_mut() {
                                let target_line = crate::editor::engine::minimap::MinimapModel::line_from_y(
                                    track_y,
                                    400.0,
                                    tab.lines.len(),
                                );
                                tab.cursor_row = target_line;
                                tab.cursor_col = tab.current_line().chars().count().min(tab.cursor_col);
                                tab.scroll_to(target_line.saturating_sub(15));
                                this.state.focused_panel = FocusedPanel::Editor;
                                cx.notify();
                            }
                        }))
                        .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _window, cx| {
                            let pos_y: f32 = event.position.y.into();
                            let track_y = pos_y - 94.0;
                            if track_y < 0.0 || track_y > 400.0 {
                                if this.state.minimap_hover.is_some() {
                                    this.state.minimap_hover = None;
                                    cx.notify();
                                }
                            } else if let Some(tab) = this.state.active_tab() {
                                let target_line = crate::editor::engine::minimap::MinimapModel::line_from_y(
                                    track_y,
                                    400.0,
                                    tab.lines.len(),
                                );
                                this.state.minimap_hover = Some((target_line, pos_y));
                                cx.notify();
                            }
                        }))
                        .on_mouse_exit(cx.listener(|this, _event, _window, cx| {
                            this.state.minimap_hover = None;
                            cx.notify();
                        }))
                        // Viewport Lens Box (VS Code subtle translucent slider)
                        .child(
                            div()
                                .absolute()
                                .left(px(1.0))
                                .right(px(1.0))
                                .top(px(lens_top))
                                .h(px(lens_h))
                                .rounded(px(2.0))
                                .bg(rgba(0xffffff14))
                                .border_1()
                                .border_color(rgba(0xffffff28))
                                .hover(|s| s.bg(rgba(0xffffff22)).border_color(rgba(0xffffff40)))
                        )
                        // Miniature Code Lines Track (Legible Micro-Code Syntax Spans)
                        .children(track_lines.into_iter().map(|line| {
                            let line_idx = line.line_idx;
                            let is_curr = line.is_cursor;
                            let marker = line.marker;
                            let line_text = tab.lines.get(line_idx).map(|s| s.as_str()).unwrap_or("");
                            let tokens = tab.line_tokens.get(line_idx);

                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .h(px(4.0))
                                .w_full()
                                .overflow_hidden()
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                    if let Some(tab) = this.state.active_tab_mut() {
                                        tab.cursor_row = line_idx;
                                        tab.cursor_col = tab.current_line().chars().count().min(tab.cursor_col);
                                        tab.scroll_to(line_idx.saturating_sub(25));
                                    }
                                    this.state.focused_panel = FocusedPanel::Editor;
                                    cx.notify();
                                }))
                                .child(
                                    if line_text.trim().is_empty() {
                                        div().h(px(4.0)).w_full().into_any_element()
                                    } else if let Some(toks) = tokens.filter(|t| !t.is_empty()) {
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .overflow_hidden()
                                            .text_size(px(3.5))
                                            .line_height(px(4.0))
                                            .font_family("Cascadia Code, Consolas, monospace")
                                            .children(toks.iter().map(|tok| {
                                                let col = match tok.token_type {
                                                    crate::editor::engine::tokenizer::TokenType::Keyword => theme.syn_keyword,
                                                    crate::editor::engine::tokenizer::TokenType::Function => theme.syn_function,
                                                    crate::editor::engine::tokenizer::TokenType::Type => theme.syn_type,
                                                    crate::editor::engine::tokenizer::TokenType::String => theme.syn_string,
                                                    crate::editor::engine::tokenizer::TokenType::Comment => theme.syn_comment,
                                                    crate::editor::engine::tokenizer::TokenType::Number => theme.syn_number,
                                                    crate::editor::engine::tokenizer::TokenType::Plain => theme.text_primary,
                                                    _ => theme.text_muted,
                                                };
                                                div()
                                                    .text_color(if is_curr { theme.accent } else { col.opacity(0.8) })
                                                    .child(tok.text.clone())
                                            }))
                                            .into_any_element()
                                    } else {
                                        div()
                                            .overflow_hidden()
                                            .text_size(px(3.5))
                                            .line_height(px(4.0))
                                            .font_family("Cascadia Code, Consolas, monospace")
                                            .text_color(if is_curr { theme.accent } else { theme.text_muted.opacity(0.7) })
                                            .child(line_text.to_string())
                                            .into_any_element()
                                    }
                                )
                                // Right edge diagnostic indicator dot
                                .children((marker == crate::editor::engine::minimap::MinimapMarkerKind::Error).then(|| {
                                    div()
                                        .w(px(3.0))
                                        .h(px(3.0))
                                        .rounded_full()
                                        .bg(theme.status_red)
                                }))
                        }))
                }))
                // Sticky Scroll Headers Overlay (Rendered above viewport, VS Code Parity)
                .children((!sticky_headers.is_empty()).then(|| {
                    render_sticky_scroll_headers(&sticky_headers, theme, cx)
                }))
                // Floating Code Magnifying Glass Preview (Monaco Scroller Parity)
                .children(minimap_hover.as_ref().map(|(hover_line, hover_y)| {
                    render_minimap_magnifier(tab, *hover_line, *hover_y, theme, cx)
                }))
        })
        .into_any_element()
}

fn render_monaco_line(
    line: &str,
    language: &str,
    cached_tokens: Option<&[crate::editor::engine::tokenizer::TokenSpan]>,
    _cursors: &[usize],
    ghost: Option<(usize, &str)>,
    inlays: &[crate::editor::engine::inlay_hints::InlayHint],
    brackets: &[ColorizedBracket],
    _is_focused: bool,
    theme: &Theme,
) -> impl IntoElement {
    let tokens_cow: std::borrow::Cow<[crate::editor::engine::tokenizer::TokenSpan]> =
        match cached_tokens {
            Some(toks) => std::borrow::Cow::Borrowed(toks),
            None => std::borrow::Cow::Owned(Tokenizer::tokenize_line(line, language)),
        };
    let tokens = tokens_cow.as_ref();

    let bracket_col_map: std::collections::HashMap<usize, (u8, u8, u8, bool)> = brackets
        .iter()
        .map(|b| {
            let color = if b.is_error {
                (239, 68, 68)
            } else {
                bracket_pairs::bracket_color_for_depth(b.depth)
            };
            (b.col, (color.0, color.1, color.2, b.is_error))
        })
        .collect();

    let inlays_by_col: std::collections::HashMap<
        usize,
        Vec<&crate::editor::engine::inlay_hints::InlayHint>,
    > = {
        let mut map = std::collections::HashMap::new();
        for hint in inlays {
            map.entry(hint.col).or_insert_with(Vec::new).push(hint);
        }
        map
    };

    let mut line_row = div().flex().flex_row().items_center();
    let mut col_offset = 0usize;

    for span in tokens {
        if col_offset > 5_000 {
            break;
        }

        let span_start = col_offset;
        let span_len = span.text.chars().count();
        let span_end = span_start + span_len;
        col_offset = span_end;

        let has_bracket = (span_start..span_end).any(|c| bracket_col_map.contains_key(&c));
        let has_inlay = (span_start..span_end).any(|c| inlays_by_col.contains_key(&c));
        let has_ghost = ghost.map_or(false, |(g_col, _)| span_start <= g_col && g_col < span_end);

        let default_color = match span.token_type {
            TokenType::Keyword => theme.syn_keyword,
            TokenType::Function => theme.syn_function,
            TokenType::Type => theme.syn_type,
            TokenType::String => theme.syn_string,
            TokenType::Number => theme.syn_number,
            TokenType::Comment => theme.syn_comment,
            TokenType::Macro => theme.accent,
            TokenType::Operator | TokenType::Punctuation => theme.text_muted,
            TokenType::Plain => theme.text_primary,
        };

        if !has_bracket && !has_inlay && !has_ghost {
            line_row = line_row.child(div().text_color(default_color).child(span.text.clone()));
            continue;
        }

        let mut cur_chunk = String::new();

        for (char_idx, ch) in span.text.chars().enumerate() {
            let cur_col = span_start + char_idx;

            if let Some(hints_at_col) = inlays_by_col.get(&cur_col) {
                if !cur_chunk.is_empty() {
                    line_row = line_row.child(
                        div()
                            .text_color(default_color)
                            .child(std::mem::take(&mut cur_chunk)),
                    );
                }
                for hint in hints_at_col {
                    line_row = line_row.child(
                        div()
                            .mx_0p5()
                            .px_1()
                            .py_0p5()
                            .rounded(px(2.0))
                            .bg(theme.bg_hover)
                            .text_xs()
                            .text_color(theme.text_subtle)
                            .opacity(0.85)
                            .child(hint.label.clone()),
                    );
                }
            }

            if let Some((g_col, g_text)) = ghost {
                if g_col == cur_col {
                    if !cur_chunk.is_empty() {
                        line_row = line_row.child(
                            div()
                                .text_color(default_color)
                                .child(std::mem::take(&mut cur_chunk)),
                        );
                    }
                    line_row = line_row.child(
                        div()
                            .text_color(theme.text_subtle)
                            .opacity(0.45)
                            .child(g_text.to_string()),
                    );
                }
            }

            if let Some(&(r, g, b, _is_err)) = bracket_col_map.get(&cur_col) {
                if !cur_chunk.is_empty() {
                    line_row = line_row.child(
                        div()
                            .text_color(default_color)
                            .child(std::mem::take(&mut cur_chunk)),
                    );
                }
                line_row = line_row.child(
                    div()
                        .text_color(rgb(((r as u32) << 16) | ((g as u32) << 8) | (b as u32)))
                        .font_weight(FontWeight::BOLD)
                        .child(ch.to_string()),
                );
            } else {
                cur_chunk.push(ch);
            }
        }

        if !cur_chunk.is_empty() {
            line_row = line_row.child(div().text_color(default_color).child(cur_chunk));
        }
    }

    if let Some(hints_at_end) = inlays_by_col.get(&col_offset) {
        for hint in hints_at_end {
            line_row = line_row.child(
                div()
                    .mx_0p5()
                    .px_1()
                    .py_0p5()
                    .rounded(px(2.0))
                    .bg(theme.bg_hover)
                    .text_xs()
                    .text_color(theme.text_subtle)
                    .opacity(0.85)
                    .child(hint.label.clone()),
            );
        }
    }

    if let Some((g_col, g_text)) = ghost {
        if g_col >= col_offset {
            line_row = line_row.child(
                div()
                    .text_color(theme.text_subtle)
                    .opacity(0.45)
                    .child(g_text.to_string()),
            );
        }
    }

    line_row
}

fn render_completion_popup(
    completions: &crate::editor::engine::CompletionState,
    cursor_row: usize,
    cursor_col: usize,
    scroll_row: usize,
    theme: &Theme,
    cx: &mut Context<HadesAppView>,
) -> AnyElement {
    let top_px = (cursor_row.saturating_sub(scroll_row) as f32 + 1.0) * 22.0 + 8.0;
    let left_px = (cursor_col as f32 * 8.4).min(500.0) + 72.0;

    div()
        .absolute()
        .top(px(top_px))
        .left(px(left_px))
        .w(px(280.0))
        .max_h(px(220.0))
        .bg(theme.bg_glass_card)
        .border_1()
        .border_color(theme.border_glass)
        .rounded(px(8.0))
        .shadow_lg()
        .overflow_hidden()
        .flex()
        .flex_col()
        // Header
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .px_2()
                .py_1()
                .bg(theme.bg_titlebar)
                .border_b_1()
                .border_color(theme.border_subtle)
                .text_xs()
                .child(div().text_color(theme.text_subtle).child("IntelliSense"))
                .child(div().text_color(theme.accent).child(format!(
                    "{}/{}",
                    completions.selected_index + 1,
                    completions.items.len()
                ))),
        )
        // Items list
        .child(
            div().flex().flex_col().p_1().gap_0p5().children(
                completions
                    .items
                    .iter()
                    .enumerate()
                    .take(8)
                    .map(|(idx, item)| {
                        let is_sel = idx == completions.selected_index;
                        let insert_text = item.insert_text.clone();
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .px_2()
                            .py_1()
                            .rounded_sm()
                            .cursor_pointer()
                            .bg(if is_sel {
                                theme.accent
                            } else {
                                theme.bg_raised
                            })
                            .text_color(if is_sel {
                                theme.text_on_accent
                            } else {
                                theme.text_primary
                            })
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _e, _w, cx| {
                                    if let Some(tab) = this.state.active_tab_mut() {
                                        tab.insert_char(' ');
                                        tab.model.insert_text(&insert_text);
                                        tab.model.completions.close();
                                    }
                                    cx.notify();
                                }),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_1p5()
                                    .text_xs()
                                    .font_weight(if is_sel {
                                        FontWeight::BOLD
                                    } else {
                                        FontWeight::NORMAL
                                    })
                                    .child(item.label.clone()),
                            )
                            .child(div().text_xs().opacity(0.8).child(item.detail.clone()))
                    }),
            ),
        )
        // Footer hint
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .px_2()
                .py_0p5()
                .bg(theme.bg_editor)
                .border_t_1()
                .border_color(theme.border_subtle)
                .text_xs()
                .text_color(theme.text_subtle)
                .child("Tab to insert · Esc to close"),
        )
        .into_any_element()
}

fn render_hover_tooltip(
    info: &HoverInfo,
    hover_row: usize,
    scroll_row: usize,
    theme: &Theme,
) -> AnyElement {
    let top_px = (hover_row.saturating_sub(scroll_row) as f32) * 22.0 + 32.0;
    let left_px = (info.start_col as f32 * 8.4).max(72.0);

    div()
        .absolute()
        .top(px(top_px))
        .left(px(left_px))
        .w(px(380.0))
        .max_h(px(200.0))
        .bg(theme.bg_glass_card)
        .border_1()
        .border_color(theme.border_glass)
        .rounded(px(8.0))
        .shadow_lg()
        .overflow_hidden()
        .flex()
        .flex_col()
        // Title bar
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .px_3()
                .py_1()
                .bg(theme.bg_titlebar)
                .border_b_1()
                .border_color(theme.border_subtle)
                .text_xs()
                .text_color(theme.text_subtle)
                .child(info.title.clone()),
        )
        // Signature
        .child(
            div()
                .px_3()
                .py_1()
                .text_xs()
                .font_family("Cascadia Code, Consolas, monospace")
                .text_color(theme.syn_function)
                .bg(theme.bg_card)
                .border_b_1()
                .border_color(theme.border_subtle)
                .child(info.signature.clone()),
        )
        // Documentation
        .child(
            div()
                .px_3()
                .py_1p5()
                .text_xs()
                .text_color(theme.text_muted)
                .child(info.docs.clone()),
        )
        // Footer hint
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .px_3()
                .py_0p5()
                .bg(theme.bg_editor)
                .border_t_1()
                .border_color(theme.border_subtle)
                .text_xs()
                .text_color(theme.text_subtle)
                .child("F12 Go to Definition · Esc close"),
        )
        .into_any_element()
}

fn render_empty_editor(theme: &Theme) -> AnyElement {
    div()
        .flex()
        .flex_col()
        .size_full()
        .items_center()
        .justify_center()
        .bg(theme.bg_editor)
        .child(
            div()
                .flex()
                .flex_col()
                .items_center()
                .gap_4()
                .max_w(px(480.0))
                // Logo + Title block
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .w(px(52.0))
                                .h(px(52.0))
                                .rounded_lg()
                                .bg(rgba(0x2d374822))
                                .border_1()
                                .border_color(theme.border_subtle)
                                .child(ui_icon(IconName::Sparkles, 28.0, theme.accent)),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_2()
                                .child(
                                    div()
                                        .text_lg()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_primary)
                                        .child("VSCODIUM RUST"),
                                )
                                .child(
                                    div()
                                        .px_1p5()
                                        .py_0p5()
                                        .rounded_sm()
                                        .bg(theme.bg_hover)
                                        .border_1()
                                        .border_color(theme.border_subtle)
                                        .text_xs()
                                        .text_color(theme.accent)
                                        .child("v0.1.0"),
                                ),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_subtle)
                                .child("High-Performance Native IDE · Direct3D 12 · Monaco Engine"),
                        ),
                )
                // Divider
                .child(div().w(px(280.0)).h(px(1.0)).bg(theme.border_subtle).my_1())
                // Quick actions / Shortcuts list
                .child(
                    div().flex().flex_col().w_full().gap_2().children(
                        [
                            (IconName::Sparkles, "Code with Agent", "Ctrl L"),
                            (IconName::Search, "Go to File", "Ctrl P"),
                            (IconName::Terminal, "Command Palette", "Ctrl Shift P"),
                            (IconName::SquareTerminal, "Toggle Terminal", "Ctrl `"),
                        ]
                        .into_iter()
                        .map(|(icon, label, shortcut)| {
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .justify_between()
                                .px_3()
                                .py_1p5()
                                .rounded_md()
                                .bg(theme.bg_panel)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_2()
                                        .child(icon_14(icon, theme.text_muted))
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.text_primary)
                                                .child(label),
                                        ),
                                )
                                .child(
                                    div()
                                        .px_2()
                                        .py_0p5()
                                        .rounded_sm()
                                        .bg(theme.bg_editor)
                                        .border_1()
                                        .border_color(theme.border_subtle)
                                        .text_xs()
                                        .font_family("Cascadia Code, Consolas, monospace")
                                        .text_color(theme.text_subtle)
                                        .child(shortcut),
                                )
                        }),
                    ),
                ),
        )
        .into_any_element()
}

fn render_code_action_menu(
    code_actions: &crate::editor::engine::code_actions::CodeActionState,
    cursor_row: usize,
    cursor_col: usize,
    scroll_row: usize,
    theme: &Theme,
    cx: &mut Context<HadesAppView>,
) -> AnyElement {
    let top_px = (cursor_row.saturating_sub(scroll_row) as f32 + 1.0) * 22.0 + 8.0;
    let left_px = (cursor_col as f32 * 8.4).min(500.0) + 72.0;

    div()
        .absolute()
        .top(px(top_px))
        .left(px(left_px))
        .w(px(320.0))
        .max_h(px(240.0))
        .bg(theme.bg_glass_card)
        .border_1()
        .border_color(theme.border_glass)
        .rounded(px(8.0))
        .shadow_lg()
        .overflow_hidden()
        .flex()
        .flex_col()
        // Header
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .px_2()
                .py_1()
                .bg(theme.bg_editor)
                .border_b_1()
                .border_color(theme.border_subtle)
                .text_xs()
                .font_weight(FontWeight::BOLD)
                .text_color(theme.accent)
                .child("Quick Fix (Ctrl+.)")
                .child(
                    div()
                        .text_color(theme.text_subtle)
                        .child(format!("{} available", code_actions.actions.len())),
                ),
        )
        // Actions list
        .child(
            div().flex().flex_col().p_1().gap_0p5().children(
                code_actions
                    .actions
                    .iter()
                    .enumerate()
                    .map(|(idx, action)| {
                        let is_sel = idx == code_actions.selected_idx;
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .px_2()
                            .py_1()
                            .rounded_sm()
                            .cursor_pointer()
                            .bg(if is_sel {
                                theme.accent
                            } else {
                                theme.bg_raised
                            })
                            .text_color(if is_sel {
                                theme.text_on_accent
                            } else {
                                theme.text_primary
                            })
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _e, _w, cx| {
                                    this.state.code_actions.selected_idx = idx;
                                    this.state.apply_selected_code_action();
                                    cx.notify();
                                }),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_1p5()
                                    .text_xs()
                                    .font_weight(if is_sel {
                                        FontWeight::BOLD
                                    } else {
                                        FontWeight::NORMAL
                                    })
                                    .child("⚡")
                                    .child(action.title.clone()),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .opacity(0.8)
                                    .child(action.kind.clone().unwrap_or_default()),
                            )
                    }),
            ),
        )
        // Footer
        .child(
            div()
                .px_2()
                .py_0p5()
                .bg(theme.bg_editor)
                .border_t_1()
                .border_color(theme.border_subtle)
                .text_xs()
                .text_color(theme.text_subtle)
                .child("Enter to apply · Esc to dismiss"),
        )
        .into_any_element()
}

fn render_rename_dialog(
    rename: &crate::editor::engine::rename::RenameState,
    cursor_row: usize,
    cursor_col: usize,
    scroll_row: usize,
    theme: &Theme,
    cx: &mut Context<HadesAppView>,
) -> AnyElement {
    let top_px = (cursor_row.saturating_sub(scroll_row) as f32) * 22.0 - 4.0;
    let left_px = (cursor_col as f32 * 8.4).min(500.0) + 72.0;

    div()
        .absolute()
        .top(px(top_px))
        .left(px(left_px))
        .w(px(260.0))
        .bg(theme.bg_glass_card)
        .border_1()
        .border_color(theme.border_glass)
        .rounded(px(8.0))
        .shadow_lg()
        .p_2()
        .flex()
        .flex_col()
        .gap_1p5()
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .text_xs()
                .font_weight(FontWeight::BOLD)
                .text_color(theme.accent)
                .child("Rename Symbol (F2)")
                .child(
                    div()
                        .cursor_pointer()
                        .text_color(theme.text_subtle)
                        .child("✕")
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _e, _w, cx| {
                                this.state.rename.close();
                                cx.notify();
                            }),
                        ),
                ),
        )
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .px_2()
                .py_1()
                .bg(theme.bg_editor)
                .border_1()
                .border_color(theme.border_subtle)
                .rounded_sm()
                .text_xs()
                .text_color(theme.text_primary)
                .child(format!("{}▏", rename.new_name)),
        )
        .child(
            div()
                .text_xs()
                .text_color(theme.text_subtle)
                .child("Enter to commit · Esc to cancel"),
        )
        .into_any_element()
}

fn render_references_peek(
    references: &crate::editor::engine::references::ReferencesState,
    cursor_row: usize,
    cursor_col: usize,
    scroll_row: usize,
    theme: &Theme,
    cx: &mut Context<HadesAppView>,
) -> AnyElement {
    let top_px = ((cursor_row.saturating_sub(scroll_row) as f32) * 22.0 + 24.0).min(400.0);
    let left_px = (cursor_col as f32 * 8.4).min(200.0) + 72.0;

    div()
        .absolute()
        .top(px(top_px))
        .left(px(left_px))
        .w(px(520.0))
        .max_h(px(280.0))
        .bg(theme.bg_raised)
        .border_1()
        .border_color(theme.accent)
        .rounded_md()
        .shadow_lg()
        .flex()
        .flex_col()
        .overflow_hidden()
        // Header
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .px_3()
                .py_1p5()
                .bg(theme.bg_card)
                .border_b_1()
                .border_color(theme.border_subtle)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.accent)
                        .child(format!("References: \"{}\"", references.symbol_name))
                        .child(
                            div()
                                .text_color(theme.text_subtle)
                                .child(format!("({} found)", references.references.len())),
                        ),
                )
                .child(
                    div()
                        .cursor_pointer()
                        .text_color(theme.text_subtle)
                        .hover(|s| s.text_color(theme.text_primary))
                        .child("✕")
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _e, _w, cx| {
                                this.state.references.close();
                                cx.notify();
                            }),
                        ),
                ),
        )
        // References list
        .child(
            div()
                .id("references_peek_list_scroll")
                .flex()
                .flex_col()
                .flex_1()
                .overflow_y_scroll()
                .py_1()
                .children(
                    references
                        .references
                        .iter()
                        .enumerate()
                        .map(|(idx, loc)| {
                            let is_sel = idx == references.selected_idx;
                            let file_path = loc.file_path.to_string_lossy().to_string();
                            let file_name = std::path::Path::new(&file_path)
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string();
                            let row = loc.row;
                            let col = loc.col;
                            let preview = loc.preview.trim().to_string();

                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .justify_between()
                                .h(px(28.0))
                                .px_3()
                                .cursor_pointer()
                                .bg(if is_sel {
                                    theme.bg_active
                                } else {
                                    theme.bg_raised
                                })
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _e, window, cx| {
                                        this.open_file(&file_path, window, cx);
                                        if let Some(tab) = this.state.active_tab_mut() {
                                            tab.cursor_row = row;
                                            tab.cursor_col = col;
                                            tab.scroll_row = row.saturating_sub(5);
                                        }
                                        this.state.references.close();
                                        cx.notify();
                                    }),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_2()
                                        .text_xs()
                                        .child(
                                            div()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(if is_sel {
                                                    theme.accent
                                                } else {
                                                    theme.text_primary
                                                })
                                                .child(file_name),
                                        )
                                        .child(div().text_color(theme.text_subtle).child(format!(
                                            ":{}:{}",
                                            row + 1,
                                            col + 1
                                        )))
                                        .child(
                                            div()
                                                .text_color(if is_sel {
                                                    theme.text_primary
                                                } else {
                                                    theme.text_muted
                                                })
                                                .child(preview),
                                        ),
                                )
                                .into_any_element()
                        })
                        .collect::<Vec<_>>(),
                ),
        )
        // Footer
        .child(
            div()
                .px_3()
                .py_1()
                .bg(theme.bg_editor)
                .border_t_1()
                .border_color(theme.border_subtle)
                .text_xs()
                .text_color(theme.text_subtle)
                .child("Enter or Click to jump · ↑/↓ to navigate · Esc to dismiss"),
        )
        .into_any_element()
}

fn render_minimap_magnifier(
    tab: &OpenTab,
    hover_line: usize,
    hover_y: f32,
    theme: &Theme,
    cx: &mut Context<HadesAppView>,
) -> AnyElement {
    let total = tab.lines.len();
    let start_row = hover_line.saturating_sub(3);
    let end_row = (hover_line + 4).min(total);

    div()
        .absolute()
        .right(px(112.0))
        .top(px((hover_y - 94.0 - 65.0).clamp(8.0, 520.0)))
        .w(px(660.0))
        .rounded(px(6.0))
        .bg(theme.bg_sidebar)
        .border_1()
        .border_color(theme.border_focus)
        .shadow_lg()
        .overflow_hidden()
        .flex()
        .flex_col()
        // Monaco parity: hovering out of minimap or moving into/across the preview lens dismisses it
        .on_mouse_move(cx.listener(|this, _event: &MouseMoveEvent, _window, cx| {
            this.state.minimap_hover = None;
            cx.notify();
        }))
        .on_mouse_exit(cx.listener(|this, _event, _window, cx| {
            this.state.minimap_hover = None;
            cx.notify();
        }))
        // Clicking directly on the magnifier jumps to that line and exits
        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event: &MouseDownEvent, _window, cx| {
            if let Some(tab) = this.state.active_tab_mut() {
                tab.cursor_row = hover_line;
                tab.scroll_to(hover_line.saturating_sub(15));
                this.state.focused_panel = FocusedPanel::Editor;
            }
            this.state.minimap_hover = None;
            cx.notify();
        }))
        // Header
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .h(px(26.0))
                .px_2p5()
                .bg(theme.bg_raised)
                .border_b_1()
                .border_color(theme.border_subtle)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1p5()
                        .child(crate::ui::icons::ui_icon(
                            IconName::Search,
                            12.0,
                            theme.accent,
                        ))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child(format!("Line {} of {}", hover_line + 1, total)),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_subtle)
                                .child("· Click to jump · Esc or hover away to dismiss"),
                        ),
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
                                .bg(theme.accent.opacity(0.18))
                                .text_xs()
                                .text_color(theme.accent)
                                .child(tab.model.language.to_uppercase()),
                        )
                        .child(
                            div()
                                .px_1p5()
                                .py_0p5()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .text_xs()
                                .text_color(theme.text_subtle)
                                .child("✕")
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _e, _w, cx| {
                                    this.state.minimap_hover = None;
                                    cx.notify();
                                }))
                        ),
                ),
        )
        // Code lines preview
        .child(
            div()
                .flex()
                .flex_col()
                .p_2()
                .bg(theme.bg_editor)
                .font_family("Cascadia Code, Consolas, monospace")
                .text_sm()
                .children((start_row..end_row).map(|r| {
                    let is_target = r == hover_line;
                    let line_content = tab.lines.get(r).map(|s| s.as_str()).unwrap_or("");
                    let tokens = tab.line_tokens.get(r).map(|t| t.as_slice());

                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .h(px(22.0))
                        .w_full()
                        .px_1()
                        .bg(if is_target {
                            theme.active_line_bg
                        } else {
                            rgba(0x00000000)
                        })
                        .border_l_2()
                        .border_color(if is_target {
                            theme.accent
                        } else {
                            rgba(0x00000000)
                        })
                        // Line number
                        .child(
                            div()
                                .w(px(44.0))
                                .pr_2()
                                .text_sm()
                                .text_color(if is_target {
                                    theme.accent
                                } else {
                                    theme.text_subtle
                                })
                                .font_weight(if is_target {
                                    FontWeight::BOLD
                                } else {
                                    FontWeight::NORMAL
                                })
                                .child(format!("{}", r + 1)),
                        )
                        // Code line
                        .child(render_monaco_line(
                            line_content,
                            &tab.model.language,
                            tokens,
                            &[],
                            None,
                            &[],
                            &[],
                            false,
                            theme,
                        ))
                        .into_any_element()
                })),
        )
        .into_any_element()
}

fn render_sticky_scroll_headers(
    headers: &[crate::editor::engine::sticky_scroll::StickyHeader],
    theme: &Theme,
    cx: &mut Context<HadesAppView>,
) -> AnyElement {
    div()
        .absolute()
        .top_0()
        .left_0()
        .right(px(104.0))
        .bg(theme.bg_editor)
        .border_b_1()
        .border_color(theme.border_subtle)
        .shadow_md()
        .flex()
        .flex_col()
        .children(headers.iter().map(|h| {
            let row = h.row;
            let text = h.text.clone();
            let badge = h.badge;

            div()
                .flex()
                .flex_row()
                .items_center()
                .h(px(22.0))
                .w_full()
                .bg(theme.bg_editor)
                .cursor_pointer()
                .hover(|s| s.bg(theme.bg_hover))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _e, _w, cx| {
                        if let Some(tab) = this.state.active_tab_mut() {
                            tab.cursor_row = row;
                            tab.cursor_col = 0;
                            tab.scroll_row = row.saturating_sub(2);
                        }
                        cx.notify();
                    }),
                )
                .child(
                    div()
                        .w(px(68.0))
                        .pr_2()
                        .flex()
                        .justify_end()
                        .bg(theme.bg_editor)
                        .text_sm()
                        .font_family("Cascadia Code, Consolas, monospace")
                        .text_color(theme.text_subtle)
                        .child(format!("{}", row + 1)),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .px_2()
                        .child(
                            div()
                                .px_1()
                                .py_0p5()
                                .rounded(px(2.0))
                                .bg(theme.accent.opacity(0.18))
                                .text_color(theme.accent)
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .child(badge),
                        )
                        .child(
                            div()
                                .text_sm()
                                .font_family("Cascadia Code, Consolas, monospace")
                                .text_color(theme.text_primary)
                                .child(text),
                        ),
                )
                .into_any_element()
        }))
        .into_any_element()
}
