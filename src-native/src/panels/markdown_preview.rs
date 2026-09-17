use crate::app_state::HadesNativeState;
use crate::ui::icons::{icon_12, icon_14, ui_icon, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MarkdownBlock {
    Heading {
        level: usize,
        text: String,
    },
    Paragraph(String),
    CodeBlock {
        lang: String,
        code: String,
    },
    Blockquote(String),
    ListItem {
        ordered: bool,
        number: usize,
        text: String,
        is_task: bool,
        checked: bool,
    },
    HorizontalRule,
    Table {
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
    },
}

#[derive(Clone, Debug)]
pub struct TocItem {
    pub level: usize,
    pub text: String,
    #[allow(dead_code)]
    pub block_idx: usize,
}

#[derive(Clone, Debug)]
pub struct MarkdownPreviewState {
    pub is_open: bool,
    pub show_toc: bool,
    pub zoom_level: f32,
    pub width_pct: f32,
    pub is_resizing: bool,
}

impl Default for MarkdownPreviewState {
    fn default() -> Self {
        Self {
            is_open: false,
            show_toc: false,
            zoom_level: 1.0,
            width_pct: 45.0,
            is_resizing: false,
        }
    }
}

/// Parse raw markdown text into structured blocks
pub fn parse_markdown(text: &str) -> (Vec<MarkdownBlock>, Vec<TocItem>) {
    let mut blocks = Vec::new();
    let mut toc = Vec::new();
    let lines: Vec<&str> = text.lines().collect();
    let mut idx = 0;

    while idx < lines.len() {
        let line = lines[idx];
        let trimmed = line.trim();

        if trimmed.is_empty() {
            idx += 1;
            continue;
        }

        // Fenced Code Block: ```lang
        if trimmed.starts_with("```") {
            let lang = trimmed.trim_start_matches("```").trim().to_string();
            let mut code_lines = Vec::new();
            idx += 1;
            while idx < lines.len() && !lines[idx].trim().starts_with("```") {
                code_lines.push(lines[idx]);
                idx += 1;
            }
            if idx < lines.len() {
                idx += 1; // skip closing ```
            }
            blocks.push(MarkdownBlock::CodeBlock {
                lang: if lang.is_empty() {
                    "text".to_string()
                } else {
                    lang
                },
                code: code_lines.join("\n"),
            });
            continue;
        }

        // Headers: # H1, ## H2, ### H3, #### H4
        if trimmed.starts_with('#') {
            let hash_count = trimmed.chars().take_while(|c| *c == '#').count();
            if hash_count <= 6 {
                let heading_text = trimmed[hash_count..].trim().to_string();
                let block_idx = blocks.len();
                toc.push(TocItem {
                    level: hash_count,
                    text: heading_text.clone(),
                    block_idx,
                });
                blocks.push(MarkdownBlock::Heading {
                    level: hash_count,
                    text: heading_text,
                });
                idx += 1;
                continue;
            }
        }

        // Blockquotes: > quote
        if trimmed.starts_with('>') {
            let quote_text = trimmed.trim_start_matches('>').trim().to_string();
            blocks.push(MarkdownBlock::Blockquote(quote_text));
            idx += 1;
            continue;
        }

        // Horizontal Rule: --- or ***
        if trimmed == "---" || trimmed == "***" || trimmed == "___" {
            blocks.push(MarkdownBlock::HorizontalRule);
            idx += 1;
            continue;
        }

        // Markdown Tables: | col 1 | col 2 |
        if trimmed.starts_with('|') && trimmed.ends_with('|') && idx + 1 < lines.len() {
            let next_trim = lines[idx + 1].trim();
            if next_trim.starts_with('|') && next_trim.contains("---") {
                let headers: Vec<String> = trimmed
                    .trim_matches('|')
                    .split('|')
                    .map(|c| c.trim().to_string())
                    .collect();
                idx += 2; // skip header and separator
                let mut rows = Vec::new();
                while idx < lines.len() {
                    let r_trim = lines[idx].trim();
                    if !r_trim.starts_with('|') || !r_trim.ends_with('|') {
                        break;
                    }
                    let row: Vec<String> = r_trim
                        .trim_matches('|')
                        .split('|')
                        .map(|c| c.trim().to_string())
                        .collect();
                    rows.push(row);
                    idx += 1;
                }
                blocks.push(MarkdownBlock::Table { headers, rows });
                continue;
            }
        }

        // Lists: - item, * item, - [x] task, 1. item
        if trimmed.starts_with("- ") || trimmed.starts_with("* ") || trimmed.starts_with("+ ") {
            let item_content = trimmed[2..].trim();
            if item_content.starts_with("[ ] ") {
                blocks.push(MarkdownBlock::ListItem {
                    ordered: false,
                    number: 0,
                    text: item_content[4..].to_string(),
                    is_task: true,
                    checked: false,
                });
            } else if item_content.starts_with("[x] ") || item_content.starts_with("[X] ") {
                blocks.push(MarkdownBlock::ListItem {
                    ordered: false,
                    number: 0,
                    text: item_content[4..].to_string(),
                    is_task: true,
                    checked: true,
                });
            } else {
                blocks.push(MarkdownBlock::ListItem {
                    ordered: false,
                    number: 0,
                    text: item_content.to_string(),
                    is_task: false,
                    checked: false,
                });
            }
            idx += 1;
            continue;
        }

        // Ordered lists: 1. item
        if let Some(dot_pos) = trimmed.find(". ") {
            let prefix = &trimmed[..dot_pos];
            if let Ok(num) = prefix.parse::<usize>() {
                blocks.push(MarkdownBlock::ListItem {
                    ordered: true,
                    number: num,
                    text: trimmed[dot_pos + 2..].to_string(),
                    is_task: false,
                    checked: false,
                });
                idx += 1;
                continue;
            }
        }

        // Standard Paragraph (collect consecutive lines)
        let mut para_lines = vec![line];
        idx += 1;
        while idx < lines.len() {
            let next = lines[idx].trim();
            if next.is_empty()
                || next.starts_with('#')
                || next.starts_with("```")
                || next.starts_with('>')
                || next.starts_with("- ")
                || next.starts_with("* ")
                || next == "---"
            {
                break;
            }
            para_lines.push(lines[idx]);
            idx += 1;
        }
        blocks.push(MarkdownBlock::Paragraph(para_lines.join(" ")));
    }

    (blocks, toc)
}

pub fn render_markdown_preview(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;
    let maybe_tab = state.active_tab();

    let content = maybe_tab.map(|t| t.content()).unwrap_or_default();
    let file_name = maybe_tab
        .map(|t| t.title.clone())
        .unwrap_or_else(|| "Untitled.md".to_string());

    let (blocks, toc) = parse_markdown(&content);
    let show_toc = state.markdown_preview.show_toc;

    div()
        .flex()
        .flex_col()
        .size_full()
        .bg(theme.bg_editor)
        // Preview Header Bar
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .h(px(32.0))
                .px_3()
                .bg(theme.bg_card)
                .border_b_1()
                .border_color(theme.border_subtle)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .child(ui_icon(IconName::FileText, 14.0, theme.accent))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child(format!("Preview {}", file_name)),
                        )
                        .child(
                            div()
                                .px_1p5()
                                .py_0p5()
                                .rounded(px(3.0))
                                .bg(theme.accent.opacity(0.18))
                                .text_xs()
                                .text_color(theme.accent)
                                .child("Live"),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        // TOC Toggle
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1()
                                .px_2()
                                .py_0p5()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .bg(if show_toc {
                                    theme.bg_raised
                                } else {
                                    theme.bg_card
                                })
                                .border_1()
                                .border_color(if show_toc {
                                    theme.border_focus
                                } else {
                                    theme.border_subtle
                                })
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _e, _w, cx| {
                                        this.state.markdown_preview.show_toc =
                                            !this.state.markdown_preview.show_toc;
                                        cx.notify();
                                    }),
                                )
                                .child(icon_12(
                                    IconName::ListOrdered,
                                    if show_toc {
                                        theme.accent
                                    } else {
                                        theme.text_subtle
                                    },
                                ))
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(if show_toc {
                                            theme.accent
                                        } else {
                                            theme.text_subtle
                                        })
                                        .child("Outline"),
                                ),
                        )
                        // Copy Markdown Button
                        .child({
                            let content_clone = content.clone();
                            div()
                                .p_1()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _e, _w, cx| {
                                        cx.write_to_clipboard(ClipboardItem::new_string(
                                            content_clone.clone(),
                                        ));
                                        this.state
                                            .toast_manager
                                            .push_info("Markdown copied to clipboard");
                                        cx.notify();
                                    }),
                                )
                                .child(icon_12(IconName::Copy, theme.text_subtle))
                        })
                        // Close Preview Button
                        .child(
                            div()
                                .p_1()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _e, _w, cx| {
                                        this.state.markdown_preview.is_open = false;
                                        if let Some(tab) = this.state.active_tab_mut() {
                                            tab.markdown_reader_mode = false;
                                        }
                                        cx.notify();
                                    }),
                                )
                                .child(icon_12(IconName::X, theme.text_subtle)),
                        ),
                ),
        )
        // Body: Optional TOC sidebar + Markdown rendered document
        .child(
            div()
                .flex()
                .flex_row()
                .flex_1()
                .size_full()
                .overflow_hidden()
                // TOC Sidebar
                .children(show_toc.then(|| {
                    div()
                        .id("markdown_toc_scroll")
                        .w(px(200.0))
                        .h_full()
                        .bg(theme.bg_sidebar)
                        .border_r_1()
                        .border_color(theme.border_subtle)
                        .p_2()
                        .overflow_y_scroll()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_muted)
                                .pb_1()
                                .child("TABLE OF CONTENTS"),
                        )
                        .children(toc.iter().map(|item| {
                            let indent = match item.level {
                                1 => px(4.0),
                                2 => px(12.0),
                                3 => px(20.0),
                                _ => px(28.0),
                            };
                            let heading_text = item.text.clone();
                            div()
                                .pl(indent)
                                .py_0p5()
                                .px_1p5()
                                .rounded(px(3.0))
                                .text_xs()
                                .text_color(theme.text_primary)
                                .hover(|s| s.text_color(theme.accent).bg(theme.bg_hover))
                                .cursor_pointer()
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _e, _w, cx| {
                                        if let Some(tab) = this.state.active_tab_mut() {
                                            let target = heading_text.trim();
                                            if let Some(pos) = tab.lines.iter().position(|l| {
                                                let tr = l.trim();
                                                tr.starts_with('#')
                                                    && tr.trim_start_matches('#').trim() == target
                                            }) {
                                                tab.cursor_row = pos;
                                                tab.cursor_col = 0;
                                                tab.scroll_row = pos.saturating_sub(2);
                                                tab.model
                                                    .set_cursor(tab.cursor_row, tab.cursor_col);
                                                this.state
                                                    .toast_manager
                                                    .push_info(&format!("Jumped to: {}", target));
                                            }
                                        }
                                        cx.notify();
                                    }),
                                )
                                .child(item.text.clone())
                        }))
                }))
                // Markdown Document Flow
                .child(
                    div()
                        .id("markdown_preview_scroll")
                        .flex()
                        .flex_col()
                        .flex_1()
                        .h_full()
                        .overflow_y_scroll()
                        .px_8()
                        .py_6()
                        .gap_3()
                        .max_w(px(850.0))
                        .children(blocks.into_iter().map(|b| render_markdown_block(&b, theme))),
                ),
        )
}

fn render_markdown_block(block: &MarkdownBlock, theme: &crate::theme::Theme) -> AnyElement {
    match block {
        MarkdownBlock::Heading { level, text } => {
            let heading_el = match level {
                1 => div()
                    .text_xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(theme.text_primary),
                2 => div()
                    .text_lg()
                    .font_weight(FontWeight::BOLD)
                    .text_color(theme.text_primary),
                3 => div()
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(theme.accent),
                _ => div()
                    .text_xs()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(theme.text_muted),
            };

            let pb = match level {
                1 => px(6.0),
                2 => px(4.0),
                3 => px(2.0),
                _ => px(1.0),
            };

            div()
                .flex()
                .flex_col()
                .pt_2()
                .pb(pb)
                .border_b_1()
                .border_color(if *level <= 2 {
                    theme.border_subtle
                } else {
                    rgba(0x00000000)
                })
                .child(heading_el.child(text.clone()))
                .into_any_element()
        }
        MarkdownBlock::Paragraph(text) => div()
            .text_sm()
            .text_color(theme.text_primary)
            .line_height(px(22.0))
            .child(text.clone())
            .into_any_element(),
        MarkdownBlock::CodeBlock { lang, code } => {
            div()
                .flex()
                .flex_col()
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_subtle)
                .overflow_hidden()
                // Code block header
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .h(px(26.0))
                        .px_3()
                        .bg(theme.bg_raised)
                        .border_b_1()
                        .border_color(theme.border_subtle)
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1p5()
                                .child(ui_icon(IconName::Code, 12.0, theme.accent))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.accent)
                                        .child(lang.to_uppercase()),
                                ),
                        ),
                )
                // Code lines with Monaco syntax coloring
                .child(
                    div()
                        .p_3()
                        .font_family("Cascadia Code, Consolas, monospace")
                        .text_xs()
                        .line_height(px(20.0))
                        .bg(theme.bg_editor)
                        .flex()
                        .flex_col()
                        .children(code.lines().map(|line| {
                            let tokens = crate::editor::engine::tokenizer::Tokenizer::tokenize_line(
                                line, lang,
                            );
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .children(tokens.into_iter().map(|tok| {
                                    let col = match tok.token_type {
                                        crate::editor::engine::tokenizer::TokenType::Keyword => {
                                            theme.syn_keyword
                                        }
                                        crate::editor::engine::tokenizer::TokenType::Function => {
                                            theme.syn_function
                                        }
                                        crate::editor::engine::tokenizer::TokenType::Type => {
                                            theme.syn_type
                                        }
                                        crate::editor::engine::tokenizer::TokenType::String => {
                                            theme.syn_string
                                        }
                                        crate::editor::engine::tokenizer::TokenType::Comment => {
                                            theme.syn_comment
                                        }
                                        crate::editor::engine::tokenizer::TokenType::Number => {
                                            theme.syn_number
                                        }
                                        _ => theme.syn_variable,
                                    };
                                    div().text_color(col).child(tok.text)
                                }))
                        })),
                )
                .into_any_element()
        }
        MarkdownBlock::Blockquote(raw_text) => {
            let trimmed = raw_text.trim();
            if let Some(rest) = trimmed.strip_prefix("[!NOTE]") {
                let body = rest.trim();
                div()
                    .flex()
                    .flex_col()
                    .pl_3()
                    .pr_3()
                    .py_2()
                    .border_l_4()
                    .border_color(theme.status_blue)
                    .bg(theme.status_blue.opacity(0.12))
                    .rounded_r(px(4.0))
                    .gap_1()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_1p5()
                            .child(ui_icon(IconName::Info, 13.0, theme.status_blue))
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.status_blue)
                                    .child("Note"),
                            ),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.text_primary)
                            .child(body.to_string()),
                    )
                    .into_any_element()
            } else if let Some(rest) = trimmed.strip_prefix("[!TIP]") {
                let body = rest.trim();
                div()
                    .flex()
                    .flex_col()
                    .pl_3()
                    .pr_3()
                    .py_2()
                    .border_l_4()
                    .border_color(theme.status_green)
                    .bg(theme.status_green.opacity(0.12))
                    .rounded_r(px(4.0))
                    .gap_1()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_1p5()
                            .child(ui_icon(IconName::Sparkles, 13.0, theme.status_green))
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.status_green)
                                    .child("Tip"),
                            ),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.text_primary)
                            .child(body.to_string()),
                    )
                    .into_any_element()
            } else if let Some(rest) = trimmed.strip_prefix("[!WARNING]") {
                let body = rest.trim();
                div()
                    .flex()
                    .flex_col()
                    .pl_3()
                    .pr_3()
                    .py_2()
                    .border_l_4()
                    .border_color(theme.status_yellow)
                    .bg(theme.status_yellow.opacity(0.12))
                    .rounded_r(px(4.0))
                    .gap_1()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_1p5()
                            .child(ui_icon(IconName::TriangleAlert, 13.0, theme.status_yellow))
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.status_yellow)
                                    .child("Warning"),
                            ),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.text_primary)
                            .child(body.to_string()),
                    )
                    .into_any_element()
            } else if let Some(rest) = trimmed.strip_prefix("[!IMPORTANT]") {
                let body = rest.trim();
                div()
                    .flex()
                    .flex_col()
                    .pl_3()
                    .pr_3()
                    .py_2()
                    .border_l_4()
                    .border_color(rgb(0xa855f7))
                    .bg(rgba(0xa855f718))
                    .rounded_r(px(4.0))
                    .gap_1()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_1p5()
                            .child(ui_icon(IconName::Info, 13.0, rgb(0xa855f7)))
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xa855f7))
                                    .child("Important"),
                            ),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.text_primary)
                            .child(body.to_string()),
                    )
                    .into_any_element()
            } else if let Some(rest) = trimmed.strip_prefix("[!CAUTION]") {
                let body = rest.trim();
                div()
                    .flex()
                    .flex_col()
                    .pl_3()
                    .pr_3()
                    .py_2()
                    .border_l_4()
                    .border_color(theme.status_red)
                    .bg(theme.status_red.opacity(0.12))
                    .rounded_r(px(4.0))
                    .gap_1()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_1p5()
                            .child(ui_icon(IconName::CircleX, 13.0, theme.status_red))
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.status_red)
                                    .child("Caution"),
                            ),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.text_primary)
                            .child(body.to_string()),
                    )
                    .into_any_element()
            } else {
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .pl_3()
                    .py_1p5()
                    .border_l_4()
                    .border_color(theme.accent)
                    .bg(theme.accent.opacity(0.08))
                    .rounded_r(px(4.0))
                    .text_sm()
                    .text_color(theme.text_muted)
                    .child(raw_text.clone())
                    .into_any_element()
            }
        }
        MarkdownBlock::ListItem {
            text,
            is_task,
            checked,
            ordered,
            number,
        } => div()
            .flex()
            .flex_row()
            .items_center()
            .gap_2()
            .py_0p5()
            .child(if *is_task {
                div()
                    .w(px(14.0))
                    .h(px(14.0))
                    .rounded(px(3.0))
                    .border_1()
                    .border_color(if *checked {
                        theme.status_green
                    } else {
                        theme.border_subtle
                    })
                    .bg(if *checked {
                        theme.status_green.opacity(0.2)
                    } else {
                        theme.bg_card
                    })
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(if *checked {
                        icon_12(IconName::Check, theme.status_green).into_any_element()
                    } else {
                        div().into_any_element()
                    })
                    .into_any_element()
            } else if *ordered {
                div()
                    .text_xs()
                    .font_weight(FontWeight::BOLD)
                    .text_color(theme.accent)
                    .child(format!("{}.", number))
                    .into_any_element()
            } else {
                div()
                    .w(px(5.0))
                    .h(px(5.0))
                    .rounded_full()
                    .bg(theme.accent)
                    .into_any_element()
            })
            .child(
                div()
                    .text_sm()
                    .text_color(if *is_task && *checked {
                        theme.text_subtle
                    } else {
                        theme.text_primary
                    })
                    .child(text.clone()),
            )
            .into_any_element(),
        MarkdownBlock::HorizontalRule => div()
            .w_full()
            .h(px(1.0))
            .my_2()
            .bg(theme.border_subtle)
            .into_any_element(),
        MarkdownBlock::Table { headers, rows } => div()
            .flex()
            .flex_col()
            .w_full()
            .rounded(px(4.0))
            .border_1()
            .border_color(theme.border_subtle)
            .overflow_hidden()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .bg(theme.bg_raised)
                    .border_b_1()
                    .border_color(theme.border_subtle)
                    .children(headers.iter().map(|h| {
                        div()
                            .flex_1()
                            .p_2()
                            .text_xs()
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.text_primary)
                            .child(h.clone())
                    })),
            )
            .children(rows.iter().map(|row| {
                div()
                    .flex()
                    .flex_row()
                    .border_b_1()
                    .border_color(theme.border_subtle)
                    .children(row.iter().map(|cell| {
                        div()
                            .flex_1()
                            .p_2()
                            .text_xs()
                            .text_color(theme.text_primary)
                            .child(cell.clone())
                    }))
            }))
            .into_any_element(),
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_markdown, MarkdownBlock, MarkdownPreviewState};
    use core::prelude::v1::test;

    #[test]
    fn test_parse_markdown_headings_and_toc() {
        let md = "# Title\n\n## Subtitle\n\n### Section 1";
        let (blocks, toc) = parse_markdown(md);

        assert_eq!(blocks.len(), 3);
        assert_eq!(
            blocks[0],
            MarkdownBlock::Heading {
                level: 1,
                text: "Title".to_string()
            }
        );
        assert_eq!(
            blocks[1],
            MarkdownBlock::Heading {
                level: 2,
                text: "Subtitle".to_string()
            }
        );
        assert_eq!(
            blocks[2],
            MarkdownBlock::Heading {
                level: 3,
                text: "Section 1".to_string()
            }
        );

        assert_eq!(toc.len(), 3);
        assert_eq!(toc[0].level, 1);
        assert_eq!(toc[0].text, "Title");
        assert_eq!(toc[1].level, 2);
        assert_eq!(toc[1].text, "Subtitle");
    }

    #[test]
    fn test_parse_markdown_code_block() {
        let md = "```rust\nfn main() {\n    println!(\"Hello\");\n}\n```";
        let (blocks, _) = parse_markdown(md);

        assert_eq!(blocks.len(), 1);
        match &blocks[0] {
            MarkdownBlock::CodeBlock { lang, code } => {
                assert_eq!(lang, "rust");
                assert!(code.contains("println!"));
            }
            _ => panic!("Expected CodeBlock"),
        }
    }

    #[test]
    fn test_parse_markdown_lists_and_tasks() {
        let md = "- [ ] Unfinished task\n- [x] Completed task\n* Normal bullet\n1. First step\n2. Second step";
        let (blocks, _) = parse_markdown(md);

        assert_eq!(blocks.len(), 5);
        assert_eq!(
            blocks[0],
            MarkdownBlock::ListItem {
                ordered: false,
                number: 0,
                text: "Unfinished task".to_string(),
                is_task: true,
                checked: false,
            }
        );
        assert_eq!(
            blocks[1],
            MarkdownBlock::ListItem {
                ordered: false,
                number: 0,
                text: "Completed task".to_string(),
                is_task: true,
                checked: true,
            }
        );
        assert_eq!(
            blocks[2],
            MarkdownBlock::ListItem {
                ordered: false,
                number: 0,
                text: "Normal bullet".to_string(),
                is_task: false,
                checked: false,
            }
        );
        assert_eq!(
            blocks[3],
            MarkdownBlock::ListItem {
                ordered: true,
                number: 1,
                text: "First step".to_string(),
                is_task: false,
                checked: false,
            }
        );
    }

    #[test]
    fn test_parse_markdown_table_and_quote() {
        let md = "> Important note\n\n---\n\n| Feature | Status |\n| --- | --- |\n| Rust | Stable |\n| GPUI | Active |";
        let (blocks, _) = parse_markdown(md);

        assert_eq!(blocks.len(), 3);
        assert_eq!(
            blocks[0],
            MarkdownBlock::Blockquote("Important note".to_string())
        );
        assert_eq!(blocks[1], MarkdownBlock::HorizontalRule);
        match &blocks[2] {
            MarkdownBlock::Table { headers, rows } => {
                assert_eq!(headers, &["Feature", "Status"]);
                assert_eq!(rows.len(), 2);
                assert_eq!(rows[0], &["Rust", "Stable"]);
                assert_eq!(rows[1], &["GPUI", "Active"]);
            }
            _ => panic!("Expected Table"),
        }
    }

    #[test]
    fn test_markdown_preview_state_defaults() {
        let state = MarkdownPreviewState::default();
        assert!(!state.is_open);
        assert!(!state.show_toc);
        assert_eq!(state.width_pct, 45.0);
        assert!(!state.is_resizing);
    }
}
