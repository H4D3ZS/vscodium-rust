// Markdown-to-GPUI Renderer for AI Chat Messages.
//
// Converts a subset of markdown text into GPUI element trees for rendering
// in the Agent Chat panel. Supports:
// - **bold** and *italic*
// - `inline code` and ```fenced code blocks```
// - # Headings (H1-H3)
// - - Bullet lists
// - 1. Numbered lists
// - > Blockquotes
// - Horizontal rules (---)
// - [links](url) (rendered as colored text)
//
// This is intentionally NOT a full markdown parser — it covers the output
// patterns of LLM chat responses. For full markdown, a crate like pulldown-cmark
// would be used, but we avoid the dependency to keep the binary lean.

use crate::theme::Theme;
use gpui_kit::gpui::*;

/// Render a markdown string into a GPUI element tree.
pub fn render_markdown(text: &str, theme: &Theme) -> impl IntoElement {
    let mut elements: Vec<AnyElement> = Vec::new();
    let mut in_code_block = false;
    let mut code_block_lang = String::new();
    let mut code_lines: Vec<String> = Vec::new();

    for line in text.lines() {
        let trimmed = line.trim();

        // Code block fence toggle
        if trimmed.starts_with("```") {
            if in_code_block {
                // End code block — flush accumulated code
                elements.push(
                    render_code_block(&code_lines, &code_block_lang, theme).into_any_element(),
                );
                code_lines.clear();
                code_block_lang.clear();
                in_code_block = false;
            } else {
                // Start code block
                code_block_lang = trimmed.trim_start_matches('`').to_string();
                in_code_block = true;
            }
            continue;
        }

        if in_code_block {
            code_lines.push(line.to_string());
            continue;
        }

        // Horizontal rule
        if trimmed == "---" || trimmed == "***" || trimmed == "___" {
            elements.push(
                div()
                    .w_full()
                    .h(px(1.0))
                    .bg(theme.border_subtle)
                    .my_1()
                    .into_any_element(),
            );
            continue;
        }

        // Heading
        if let Some(heading) = parse_heading(trimmed) {
            elements.push(render_heading(heading.0, heading.1, theme).into_any_element());
            continue;
        }

        // Blockquote
        if let Some(quote_text) = trimmed.strip_prefix("> ") {
            elements.push(render_blockquote(quote_text, theme).into_any_element());
            continue;
        }

        // Bullet list
        if trimmed.starts_with("- ") || trimmed.starts_with("* ") || trimmed.starts_with("• ") {
            let item_text = &trimmed[2..];
            elements.push(render_list_item(item_text, false, 0, theme).into_any_element());
            continue;
        }

        // Numbered list
        if let Some(rest) = parse_numbered_list(trimmed) {
            elements.push(render_list_item(rest.1, true, rest.0, theme).into_any_element());
            continue;
        }

        // Empty line → spacing
        if trimmed.is_empty() {
            elements.push(div().h(px(4.0)).into_any_element());
            continue;
        }

        // Regular paragraph with inline formatting
        elements.push(render_inline_text(line, theme).into_any_element());
    }

    // Flush any unclosed code block
    if in_code_block && !code_lines.is_empty() {
        elements.push(render_code_block(&code_lines, &code_block_lang, theme).into_any_element());
    }

    div().flex().flex_col().gap_0p5().children(elements)
}

/// Parse heading level and text: "## Foo" → Some((2, "Foo"))
fn parse_heading(line: &str) -> Option<(usize, &str)> {
    let level = line.chars().take_while(|c| *c == '#').count();
    if level >= 1 && level <= 6 {
        let text = line[level..].trim();
        if !text.is_empty() {
            return Some((level, text));
        }
    }
    None
}

/// Parse numbered list: "1. Foo" → Some((1, "Foo"))
fn parse_numbered_list(line: &str) -> Option<(usize, &str)> {
    let dot_pos = line.find(". ")?;
    let num_str = &line[..dot_pos];
    let num: usize = num_str.parse().ok()?;
    Some((num, &line[dot_pos + 2..]))
}

fn render_heading(level: usize, text: &str, theme: &Theme) -> impl IntoElement {
    let size_class = match level {
        1 => FontWeight::EXTRA_BOLD,
        2 => FontWeight::BOLD,
        _ => FontWeight::SEMIBOLD,
    };

    div()
        .text_color(theme.text_primary)
        .font_weight(size_class)
        .mt_1()
        .mb_0p5()
        .child(text.to_string())
}

fn render_blockquote(text: &str, theme: &Theme) -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .child(
            div()
                .w(px(3.0))
                .h_full()
                .bg(theme.accent)
                .rounded_full()
                .mr_2(),
        )
        .child(
            div()
                .text_xs()
                .text_color(theme.text_muted)
                .italic()
                .child(text.to_string()),
        )
}

fn render_list_item(text: &str, numbered: bool, num: usize, theme: &Theme) -> impl IntoElement {
    let bullet = if numbered {
        format!("{}.", num)
    } else {
        "•".to_string()
    };

    div()
        .flex()
        .flex_row()
        .items_start()
        .pl_2()
        .child(
            div()
                .w(px(16.0))
                .text_xs()
                .text_color(theme.text_muted)
                .child(bullet),
        )
        .child(render_inline_text(text, theme))
}

fn render_code_block(lines: &[String], lang: &str, theme: &Theme) -> impl IntoElement {
    let normalized_lang = match lang.to_lowercase().as_str() {
        "rs" | "rust" => "rust",
        "ts" | "typescript" => "typescript",
        "js" | "javascript" => "javascript",
        "py" | "python" => "python",
        "json" => "json",
        "toml" => "toml",
        "sh" | "bash" | "powershell" | "pwsh" => "bash",
        _ => "plaintext",
    };

    div()
        .flex()
        .flex_col()
        .w_full()
        .rounded(px(6.0))
        .bg(theme.bg_app)
        .border_1()
        .border_color(theme.border_subtle)
        .my_1()
        // Language label & Action Buttons header
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .h(px(26.0))
                .px_3()
                .border_b_1()
                .border_color(theme.border_subtle)
                .bg(theme.bg_card)
                .child(
                    div().flex().items_center().gap_1p5().child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.accent)
                            .child(if lang.is_empty() {
                                "code".to_string()
                            } else {
                                lang.to_string()
                            }),
                    ),
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
                                .items_center()
                                .gap_1()
                                .px_1p5()
                                .py_0p5()
                                .rounded(px(3.0))
                                .bg(theme.bg_raised)
                                .text_xs()
                                .text_color(theme.text_subtle)
                                .cursor_pointer()
                                .hover(|s| s.text_color(theme.text_primary).bg(theme.bg_hover))
                                .child("📋 Copy"),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .px_1p5()
                                .py_0p5()
                                .rounded(px(3.0))
                                .bg(theme.bg_raised)
                                .text_xs()
                                .text_color(theme.accent)
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .child("⚡ Apply"),
                        ),
                ),
        )
        // Code content with syntax highlighted spans
        .child(
            div()
                .flex()
                .flex_col()
                .p_3()
                .text_xs()
                .font_family("Cascadia Code, Consolas, monospace")
                .text_color(theme.text_primary)
                .children(lines.iter().map(|line| {
                    let spans = crate::editor::engine::tokenizer::Tokenizer::tokenize_line(
                        line,
                        normalized_lang,
                    );
                    if spans.is_empty() {
                        div().h(px(14.0)).into_any_element()
                    } else {
                        div()
                            .flex()
                            .flex_row()
                            .flex_wrap()
                            .children(spans.into_iter().map(|span| {
                                let color = match span.token_type {
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
                                    crate::editor::engine::tokenizer::TokenType::Operator => {
                                        theme.accent
                                    }
                                    crate::editor::engine::tokenizer::TokenType::Macro => {
                                        theme.syn_function
                                    }
                                    crate::editor::engine::tokenizer::TokenType::Punctuation => {
                                        theme.text_muted
                                    }
                                    crate::editor::engine::tokenizer::TokenType::Plain => {
                                        theme.text_primary
                                    }
                                };
                                div().text_color(color).child(span.text)
                            }))
                            .into_any_element()
                    }
                })),
        )
}

/// Render inline text with **bold**, *italic*, `code`, and [link](url) formatting.
fn render_inline_text(text: &str, theme: &Theme) -> AnyElement {
    let mut spans: Vec<AnyElement> = Vec::new();
    let mut chars = text.chars().peekable();
    let mut buf = String::new();

    while let Some(ch) = chars.next() {
        match ch {
            '`' => {
                // Inline code
                flush_text(&mut buf, theme, &mut spans);
                let mut code = String::new();
                while let Some(&next) = chars.peek() {
                    if next == '`' {
                        chars.next();
                        break;
                    }
                    code.push(chars.next().unwrap());
                }
                spans.push(
                    div()
                        .px_1()
                        .py_0p5()
                        .rounded(px(3.0))
                        .bg(theme.bg_app)
                        .text_xs()
                        .font_family("Cascadia Code, Consolas, monospace")
                        .text_color(theme.accent)
                        .child(code)
                        .into_any_element(),
                );
            }
            '*' if chars.peek() == Some(&'*') => {
                // Bold
                chars.next(); // consume second *
                flush_text(&mut buf, theme, &mut spans);
                let mut bold_text = String::new();
                while let Some(next) = chars.next() {
                    if next == '*' && chars.peek() == Some(&'*') {
                        chars.next();
                        break;
                    }
                    bold_text.push(next);
                }
                spans.push(
                    div()
                        .text_color(theme.text_primary)
                        .font_weight(FontWeight::BOLD)
                        .text_xs()
                        .child(bold_text)
                        .into_any_element(),
                );
            }
            '*' => {
                // Italic
                flush_text(&mut buf, theme, &mut spans);
                let mut italic_text = String::new();
                while let Some(next) = chars.next() {
                    if next == '*' {
                        break;
                    }
                    italic_text.push(next);
                }
                spans.push(
                    div()
                        .text_color(theme.text_muted)
                        .italic()
                        .text_xs()
                        .child(italic_text)
                        .into_any_element(),
                );
            }
            '[' => {
                // Link: [text](url)
                flush_text(&mut buf, theme, &mut spans);
                let mut link_text = String::new();
                let mut found_close = false;
                while let Some(next) = chars.next() {
                    if next == ']' {
                        found_close = true;
                        break;
                    }
                    link_text.push(next);
                }
                if found_close && chars.peek() == Some(&'(') {
                    chars.next(); // consume (
                    let mut _url = String::new();
                    while let Some(next) = chars.next() {
                        if next == ')' {
                            break;
                        }
                        _url.push(next);
                    }
                    spans.push(
                        div()
                            .text_color(theme.accent)
                            .text_xs()
                            .cursor_pointer()
                            .hover(|s| s.underline())
                            .child(link_text)
                            .into_any_element(),
                    );
                } else {
                    // Not a link, push as-is
                    buf.push('[');
                    buf.push_str(&link_text);
                    if found_close {
                        buf.push(']');
                    }
                }
            }
            _ => {
                buf.push(ch);
            }
        }
    }

    flush_text(&mut buf, theme, &mut spans);

    if spans.len() == 1 {
        spans.pop().unwrap()
    } else {
        div()
            .flex()
            .flex_row()
            .flex_wrap()
            .text_xs()
            .children(spans)
            .into_any_element()
    }
}

fn flush_text(buf: &mut String, theme: &Theme, spans: &mut Vec<AnyElement>) {
    if !buf.is_empty() {
        let text = std::mem::take(buf);
        spans.push(
            div()
                .text_xs()
                .text_color(theme.text_primary)
                .child(text)
                .into_any_element(),
        );
    }
}
