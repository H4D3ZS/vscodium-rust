// Pure Rust Monaco Editor Engine Model.
// Unites the Piece Table buffer, multi-cursor set, undo/redo transaction stack,
// syntax tokenizer, code folding, diagnostic markers, and IntelliSense completions.

use super::buffer::PieceTable;
use super::completion::CompletionState;
use super::cursor::{CursorSet, Position, Selection};
use super::decorations::DiagnosticMarker;
use super::folding::FoldingModel;
use super::ghost_text::GhostText;
use super::history::{EditOperation, Transaction, UndoRedoManager};
use super::inlay_hints::{compute_code_lenses, compute_inlay_hints, CodeLens, InlayHint};
use super::snippets::{parse_snippet, SnippetSession};
use super::tokenizer::{TokenSpan, Tokenizer};
use super::wrap::WordWrapEngine;

#[derive(Clone, Debug)]
pub struct MonacoModel {
    pub buffer: PieceTable,
    pub cursors: CursorSet,
    pub history: UndoRedoManager,
    pub language: String,
    pub folding: FoldingModel,
    pub completions: CompletionState,
    pub ghost_text: GhostText,
    pub diagnostics: Vec<DiagnosticMarker>,
    pub scroll_row: usize,
    pub tab_size: usize,
    pub snippet_session: Option<SnippetSession>,
    pub wrap: WordWrapEngine,
    pub inlay_hints: Vec<InlayHint>,
    pub code_lenses: Vec<CodeLens>,
}

impl MonacoModel {
    pub fn new(content: String, language: String) -> Self {
        let is_plain_or_huge = language == "plaintext" || content.len() > 1_000_000;
        let mut buffer = PieceTable::new(content);
        let mut folding = FoldingModel::new();
        if !is_plain_or_huge {
            folding.compute_from_lines(buffer.lines());
        }

        let mut wrap = WordWrapEngine::default();
        if !is_plain_or_huge {
            wrap.recompute(buffer.lines());
        }

        let inlay_hints = if is_plain_or_huge {
            Vec::new()
        } else {
            compute_inlay_hints(buffer.lines(), &language)
        };
        let code_lenses = if is_plain_or_huge {
            Vec::new()
        } else {
            compute_code_lenses(buffer.lines(), &language)
        };

        Self {
            buffer,
            cursors: CursorSet::default(),
            history: UndoRedoManager::default(),
            language,
            folding,
            completions: CompletionState::new(),
            ghost_text: GhostText::new(),
            diagnostics: Vec::new(),
            scroll_row: 0,
            tab_size: 4,
            snippet_session: None,
            wrap,
            inlay_hints,
            code_lenses,
        }
    }

    pub fn recompute_inlays_and_lenses(&mut self) {
        self.inlay_hints = compute_inlay_hints(self.buffer.lines(), &self.language);
        self.code_lenses = compute_code_lenses(self.buffer.lines(), &self.language);
    }

    pub fn toggle_word_wrap(&mut self) -> bool {
        let res = self.wrap.toggle();
        self.wrap.recompute(self.buffer.lines());
        res
    }

    pub fn content(&mut self) -> String {
        self.buffer.text().to_string()
    }

    pub fn line_count(&mut self) -> usize {
        self.buffer.line_count()
    }

    pub fn line(&mut self, row: usize) -> &str {
        self.buffer.line(row)
    }

    pub fn current_line(&mut self) -> &str {
        let row = self.cursors.main_cursor().row;
        self.buffer.line(row)
    }

    pub fn cursor_row(&self) -> usize {
        self.cursors.main_cursor().row
    }

    pub fn cursor_col(&self) -> usize {
        self.cursors.main_cursor().col
    }

    pub fn set_cursor(&mut self, row: usize, col: usize) {
        self.cursors.set_main_cursor(Position::new(row, col));
        self.ghost_text.clear();
        self.completions.close();
    }

    pub fn insert_char(&mut self, c: char) {
        self.insert_text(&c.to_string());
    }

    pub fn insert_text(&mut self, text: &str) {
        if text.is_empty() {
            return;
        }

        let before_cursors = self.cursors.clone();
        let mut ops = Vec::new();
        let mut after_cursors = CursorSet::default();
        let mut new_cursors_list = Vec::new();

        // Process selections in reverse order to preserve earlier coordinates
        let mut sels = self.cursors.all_selections();
        sels.sort_by_key(|s| std::cmp::Reverse(s.start()));

        for sel in &sels {
            let start = sel.start();
            let end = sel.end();
            let deleted_text = if !sel.is_empty() {
                // Collect deleted text
                let start_off = self.buffer.position_to_offset(start.row, start.col);
                let end_off = self.buffer.position_to_offset(end.row, end.col);
                if end_off > start_off {
                    self.buffer.delete(start_off, end_off - start_off);
                }
                String::new()
            } else {
                String::new()
            };

            let insert_off = self.buffer.position_to_offset(start.row, start.col);
            self.buffer.insert(insert_off, text);

            let (new_row, new_col) = self.buffer.offset_to_position(insert_off + text.len());
            new_cursors_list.push(Selection::point(Position::new(new_row, new_col)));

            ops.push(EditOperation {
                start_pos: start,
                end_pos: end,
                inserted_text: text.to_string(),
                deleted_text,
            });
        }

        // Restore cursor order
        new_cursors_list.reverse();
        if let Some(first) = new_cursors_list.first() {
            after_cursors.primary = *first;
            after_cursors.secondary = new_cursors_list[1..].to_vec();
        }

        self.history.push_transaction(Transaction {
            operations: ops,
            before_cursors,
            after_cursors: after_cursors.clone(),
            timestamp_ms: chrono::Local::now().timestamp_millis() as u64,
        });

        self.cursors = after_cursors;
        self.folding.compute_from_lines(self.buffer.lines());

        // Check if we should trigger auto-completion on '.' or '::' or typing identifier
        if text == "."
            || text == "::"
            || (text.len() == 1 && text.chars().next().unwrap().is_alphanumeric())
        {
            let pos = (self.cursor_row(), self.cursor_col());
            let word = self.current_word_at_cursor();
            if word.len() >= 2 || text == "." || text == "::" {
                let candidates = match self.language.as_str() {
                    "rust" => CompletionState::default_rust_completions(),
                    _ => Vec::new(),
                };
                if !candidates.is_empty() {
                    self.completions.open(candidates, word, pos);
                }
            }
        }
    }

    pub fn insert_newline(&mut self) {
        // Compute smart indentation from current line
        let cur_line = self.current_line().to_string();
        let trimmed = cur_line.trim_start();
        let indent_len = cur_line.len() - trimmed.len();
        let indent = " ".repeat(indent_len);
        let extra_indent = if cur_line.trim_end().ends_with('{') {
            "    "
        } else {
            ""
        };
        let nl = format!("\n{}{}", indent, extra_indent);
        self.insert_text(&nl);
    }

    pub fn backspace(&mut self) {
        let before_cursors = self.cursors.clone();
        let mut ops = Vec::new();
        let mut new_cursors_list = Vec::new();

        let mut sels = self.cursors.all_selections();
        sels.sort_by_key(|s| std::cmp::Reverse(s.start()));

        for sel in &sels {
            if !sel.is_empty() {
                let start = sel.start();
                let end = sel.end();
                let start_off = self.buffer.position_to_offset(start.row, start.col);
                let end_off = self.buffer.position_to_offset(end.row, end.col);
                self.buffer.delete(start_off, end_off - start_off);
                new_cursors_list.push(Selection::point(start));
                ops.push(EditOperation {
                    start_pos: start,
                    end_pos: end,
                    inserted_text: String::new(),
                    deleted_text: String::new(),
                });
            } else {
                let pos = sel.active;
                if pos.col > 0 {
                    let target_pos = Position::new(pos.row, pos.col - 1);
                    let off = self
                        .buffer
                        .position_to_offset(target_pos.row, target_pos.col);
                    self.buffer.delete(off, 1);
                    new_cursors_list.push(Selection::point(target_pos));
                    ops.push(EditOperation {
                        start_pos: target_pos,
                        end_pos: pos,
                        inserted_text: String::new(),
                        deleted_text: String::new(),
                    });
                } else if pos.row > 0 {
                    let prev_row = pos.row - 1;
                    let prev_len = self.buffer.line_len(prev_row);
                    let target_pos = Position::new(prev_row, prev_len);
                    let off = self
                        .buffer
                        .position_to_offset(target_pos.row, target_pos.col);
                    self.buffer.delete(off, 1); // delete '\n'
                    new_cursors_list.push(Selection::point(target_pos));
                    ops.push(EditOperation {
                        start_pos: target_pos,
                        end_pos: pos,
                        inserted_text: String::new(),
                        deleted_text: "\n".into(),
                    });
                } else {
                    new_cursors_list.push(*sel);
                }
            }
        }

        new_cursors_list.reverse();
        let mut after_cursors = CursorSet::default();
        if let Some(first) = new_cursors_list.first() {
            after_cursors.primary = *first;
            after_cursors.secondary = new_cursors_list[1..].to_vec();
        }

        self.history.push_transaction(Transaction {
            operations: ops,
            before_cursors,
            after_cursors: after_cursors.clone(),
            timestamp_ms: chrono::Local::now().timestamp_millis() as u64,
        });

        self.cursors = after_cursors;
        self.folding.compute_from_lines(self.buffer.lines());
    }

    pub fn delete(&mut self) {
        let cur = self.cursors.main_cursor();
        let line_len = self.buffer.line_len(cur.row);
        let max_lines = self.buffer.line_count();

        if cur.col < line_len {
            let off = self.buffer.position_to_offset(cur.row, cur.col);
            self.buffer.delete(off, 1);
        } else if cur.row + 1 < max_lines {
            let off = self.buffer.position_to_offset(cur.row, cur.col);
            self.buffer.delete(off, 1);
        }
        self.folding.compute_from_lines(self.buffer.lines());
    }

    pub fn move_left(&mut self, select: bool) {
        let cur = self.cursors.main_cursor();
        let new_pos = if cur.col > 0 {
            Position::new(cur.row, cur.col - 1)
        } else if cur.row > 0 {
            let prev_row = cur.row - 1;
            Position::new(prev_row, self.buffer.line_len(prev_row))
        } else {
            cur
        };

        if select {
            self.cursors.primary.active = new_pos;
        } else {
            self.cursors.set_main_cursor(new_pos);
        }
    }

    pub fn move_right(&mut self, select: bool) {
        let cur = self.cursors.main_cursor();
        let cur_len = self.buffer.line_len(cur.row);
        let max_lines = self.buffer.line_count();

        let new_pos = if cur.col < cur_len {
            Position::new(cur.row, cur.col + 1)
        } else if cur.row + 1 < max_lines {
            Position::new(cur.row + 1, 0)
        } else {
            cur
        };

        if select {
            self.cursors.primary.active = new_pos;
        } else {
            self.cursors.set_main_cursor(new_pos);
        }
    }

    pub fn move_up(&mut self, select: bool) {
        let cur = self.cursors.main_cursor();
        if cur.row > 0 {
            let new_row = cur.row - 1;
            let target_col = cur.col.min(self.buffer.line_len(new_row));
            let new_pos = Position::new(new_row, target_col);
            if select {
                self.cursors.primary.active = new_pos;
            } else {
                self.cursors.set_main_cursor(new_pos);
            }
        }
    }

    pub fn move_down(&mut self, select: bool) {
        let cur = self.cursors.main_cursor();
        if cur.row + 1 < self.buffer.line_count() {
            let new_row = cur.row + 1;
            let target_col = cur.col.min(self.buffer.line_len(new_row));
            let new_pos = Position::new(new_row, target_col);
            if select {
                self.cursors.primary.active = new_pos;
            } else {
                self.cursors.set_main_cursor(new_pos);
            }
        }
    }

    pub fn add_cursor_above(&mut self) {
        let cur = self.cursors.main_cursor();
        if cur.row > 0 {
            let new_row = cur.row - 1;
            let target_col = cur.col.min(self.buffer.line_len(new_row));
            self.cursors.add_cursor(Position::new(new_row, target_col));
        }
    }

    pub fn add_cursor_below(&mut self) {
        let cur = self.cursors.main_cursor();
        if cur.row + 1 < self.buffer.line_count() {
            let new_row = cur.row + 1;
            let target_col = cur.col.min(self.buffer.line_len(new_row));
            self.cursors.add_cursor(Position::new(new_row, target_col));
        }
    }

    pub fn undo(&mut self) -> bool {
        if let Some(tx) = self.history.pop_undo() {
            for op in tx.operations.iter().rev() {
                let off = self
                    .buffer
                    .position_to_offset(op.start_pos.row, op.start_pos.col);
                if !op.inserted_text.is_empty() {
                    self.buffer.delete(off, op.inserted_text.len());
                }
                if !op.deleted_text.is_empty() {
                    self.buffer.insert(off, &op.deleted_text);
                }
            }
            self.cursors = tx.before_cursors;
            self.folding.compute_from_lines(self.buffer.lines());
            true
        } else {
            false
        }
    }

    pub fn redo(&mut self) -> bool {
        if let Some(tx) = self.history.pop_redo() {
            for op in &tx.operations {
                let off = self
                    .buffer
                    .position_to_offset(op.start_pos.row, op.start_pos.col);
                if !op.deleted_text.is_empty() {
                    self.buffer.delete(off, op.deleted_text.len());
                }
                if !op.inserted_text.is_empty() {
                    self.buffer.insert(off, &op.inserted_text);
                }
            }
            self.cursors = tx.after_cursors;
            self.folding.compute_from_lines(self.buffer.lines());
            true
        } else {
            false
        }
    }

    pub fn select_all(&mut self) {
        let last_row = self.buffer.line_count().saturating_sub(1);
        let last_col = self.buffer.line_len(last_row);
        self.cursors.primary =
            Selection::new(Position::new(0, 0), Position::new(last_row, last_col));
        self.cursors.clear_secondary();
    }

    /// Apply a sequence of LSP TextEdits atomically to the piece table buffer,
    /// recording the change as an undo/redo transaction and updating the folding model.
    pub fn apply_text_edits(&mut self, edits: &[crate::editor::engine::lsp::LspTextEdit]) {
        if edits.is_empty() {
            return;
        }

        let before_cursors = self.cursors.clone();
        let mut sorted_edits = edits.to_vec();
        // Sort in reverse order so earlier edits do not invalidate later position offsets
        sorted_edits.sort_by(|a, b| {
            b.range
                .start
                .line
                .cmp(&a.range.start.line)
                .then_with(|| b.range.start.character.cmp(&a.range.start.character))
        });

        let mut ops = Vec::new();

        for edit in sorted_edits {
            let start_row = edit
                .range
                .start
                .line
                .min(self.buffer.line_count().saturating_sub(1));
            let start_col = edit
                .range
                .start
                .character
                .min(self.buffer.line_len(start_row));
            let end_row = edit
                .range
                .end
                .line
                .min(self.buffer.line_count().saturating_sub(1));
            let end_col = edit.range.end.character.min(self.buffer.line_len(end_row));

            let start_off = self.buffer.position_to_offset(start_row, start_col);
            let end_off = self.buffer.position_to_offset(end_row, end_col);

            let deleted_text = if end_off > start_off {
                let full = self.buffer.text();
                full.chars()
                    .skip(start_off)
                    .take(end_off - start_off)
                    .collect::<String>()
            } else {
                String::new()
            };

            if end_off > start_off {
                self.buffer.delete(start_off, end_off - start_off);
            }

            if !edit.new_text.is_empty() {
                self.buffer.insert(start_off, &edit.new_text);
            }

            ops.push(EditOperation {
                start_pos: Position::new(start_row, start_col),
                end_pos: Position::new(end_row, end_col),
                inserted_text: edit.new_text,
                deleted_text,
            });
        }

        // Clamp cursor to valid range
        let max_r = self.buffer.line_count().saturating_sub(1);
        let cur_r = self.cursor_row().min(max_r);
        let cur_c = self.cursor_col().min(self.buffer.line_len(cur_r));
        self.cursors.set_main_cursor(Position::new(cur_r, cur_c));

        let after_cursors = self.cursors.clone();
        self.history.push_transaction(Transaction {
            operations: ops,
            before_cursors,
            after_cursors,
            timestamp_ms: chrono::Local::now().timestamp_millis() as u64,
        });

        self.folding.compute_from_lines(self.buffer.lines());
    }

    pub fn toggle_comment(&mut self) {
        let cur_row = self.cursor_row();
        let line = self.buffer.line(cur_row).to_string();
        let comment_prefix = match self.language.as_str() {
            "python" | "toml" | "bash" => "# ",
            _ => "// ",
        };

        if line.trim_start().starts_with(comment_prefix.trim()) {
            // Uncomment
            if let Some(pos) = line.find(comment_prefix) {
                let off = self.buffer.position_to_offset(cur_row, pos);
                self.buffer.delete(off, comment_prefix.len());
            }
        } else {
            // Comment
            let off = self.buffer.position_to_offset(cur_row, 0);
            self.buffer.insert(off, comment_prefix);
        }
    }

    pub fn current_word_at_cursor(&mut self) -> String {
        let cur = self.cursors.main_cursor();
        let line = self.buffer.line(cur.row);
        let chars: Vec<char> = line.chars().collect();
        let mut start = cur.col;
        while start > 0
            && chars
                .get(start - 1)
                .map(|c| c.is_alphanumeric() || *c == '_')
                .unwrap_or(false)
        {
            start -= 1;
        }
        chars[start..cur.col.min(chars.len())].iter().collect()
    }

    pub fn tokenized_lines(
        &mut self,
        start_row: usize,
        end_row: usize,
    ) -> Vec<(usize, Vec<TokenSpan>)> {
        let lang = self.language.clone();
        let lines = self.buffer.lines();
        let mut result = Vec::new();

        for r in start_row..end_row.min(lines.len()) {
            let l = &lines[r];
            let tokens = Tokenizer::tokenize_line(l, &lang);
            result.push((r, tokens));
        }

        result
    }

    pub fn scroll_by(&mut self, delta: i32) {
        let max_scroll = self.buffer.line_count().saturating_sub(1);
        let new_scroll = (self.scroll_row as i32 + delta).max(0) as usize;
        self.scroll_row = new_scroll.min(max_scroll);
    }

    pub fn scroll_to(&mut self, row: usize) {
        let max_scroll = self.buffer.line_count().saturating_sub(1);
        self.scroll_row = row.min(max_scroll);
    }

    pub fn ensure_cursor_visible(&mut self, visible_lines: usize) {
        let cur_row = self.cursor_row();
        let visible = visible_lines.max(5);
        if cur_row < self.scroll_row {
            self.scroll_row = cur_row;
        } else if cur_row >= self.scroll_row + visible {
            self.scroll_row = cur_row.saturating_sub(visible - 1);
        }
    }

    /// Insert a snippet template, expand placeholders, and initiate interactive tab stop stepping.
    pub fn insert_snippet(&mut self, template: &str) {
        let parsed = parse_snippet(template);
        let cur_row = self.cursor_row();
        let cur_col = self.cursor_col();
        let start_offset = self.buffer.position_to_offset(cur_row, cur_col);

        // Insert plain text at cursor
        self.insert_text(&parsed.plain_text);

        // If snippet has non-trivial tab stops, initialize interactive session
        if parsed.tab_stops.len() > 1
            || (parsed.tab_stops.len() == 1 && parsed.tab_stops[0].index != 0)
        {
            let session = SnippetSession::new(parsed, start_offset);
            self.snippet_session = session;
            self.select_current_tab_stop();
        }
    }

    /// Update selection to highlight the active tab stop's placeholder
    pub fn select_current_tab_stop(&mut self) {
        if let Some(session) = &self.snippet_session {
            if let Some(stop) = session.current_stop() {
                if let Some(first_range) = stop.ranges.first() {
                    let start_abs = session.insertion_base_offset + first_range.start_offset;
                    let end_abs = session.insertion_base_offset + first_range.end_offset;

                    let (start_r, start_c) = self.buffer.offset_to_position(start_abs);
                    let (end_r, end_c) = self.buffer.offset_to_position(end_abs);

                    if start_abs == end_abs {
                        self.set_cursor(start_r, start_c);
                    } else {
                        self.cursors.primary.anchor = Position::new(start_r, start_c);
                        self.cursors.primary.active = Position::new(end_r, end_c);
                    }
                }
            }
        }
    }

    /// Step to the next (or previous) tab stop. Returns true if still inside a session.
    pub fn step_snippet_tab_stop(&mut self, backward: bool) -> bool {
        if let Some(ref mut session) = self.snippet_session {
            let res = if backward {
                session.prev()
            } else {
                session.next()
            };
            if res.is_some() {
                let is_final = session.is_at_final_stop();
                self.select_current_tab_stop();
                if is_final && !backward {
                    self.snippet_session = None; // Exited snippet mode at $0
                }
                true
            } else {
                self.snippet_session = None;
                false
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monaco_model_full_lifecycle() {
        let mut model = MonacoModel::new("fn main() {\n\n}\n".into(), "rust".into());
        assert_eq!(model.line_count(), 4);
        assert_eq!(model.cursor_row(), 0);

        // Move into function body
        model.set_cursor(1, 0);
        model.insert_text("    let x = 42;");
        assert_eq!(model.line(1), "    let x = 42;");

        // Undo
        assert!(model.undo());
        assert_eq!(model.line(1), "");

        // Redo
        assert!(model.redo());
        assert_eq!(model.line(1), "    let x = 42;");
    }

    #[test]
    fn test_monaco_model_multicursor() {
        let mut model = MonacoModel::new("a\nb\nc\n".into(), "rust".into());
        model.set_cursor(0, 1);
        model.add_cursor_below(); // adds cursor at row 1, col 1
        assert_eq!(model.cursors.all_cursors().len(), 2);

        model.insert_text("1");
        assert_eq!(model.line(0), "a1");
        assert_eq!(model.line(1), "b1");
    }

    #[test]
    fn test_monaco_model_snippet_expansion() {
        let mut model = MonacoModel::new("".into(), "rust".into());
        model.insert_snippet("fn ${1:foo}() -> ${2:bool} { $0 }");

        assert_eq!(model.content(), "fn foo() -> bool {  }");
        assert!(model.snippet_session.is_some());

        // Step to $2
        assert!(model.step_snippet_tab_stop(false));
        // Step to $0 (exits session)
        assert!(model.step_snippet_tab_stop(false));
        assert!(model.snippet_session.is_none());
    }

    #[test]
    fn test_monaco_model_apply_text_edits() {
        use crate::editor::engine::lsp::{LspPosition, LspRange, LspTextEdit};

        let mut model = MonacoModel::new("let mut a = 1;\nlet mut b = 2;\n".into(), "rust".into());
        let edits = vec![
            LspTextEdit {
                range: LspRange {
                    start: LspPosition {
                        line: 0,
                        character: 8,
                    },
                    end: LspPosition {
                        line: 0,
                        character: 9,
                    },
                },
                new_text: "foo".to_string(),
            },
            LspTextEdit {
                range: LspRange {
                    start: LspPosition {
                        line: 1,
                        character: 8,
                    },
                    end: LspPosition {
                        line: 1,
                        character: 9,
                    },
                },
                new_text: "bar".to_string(),
            },
        ];

        model.apply_text_edits(&edits);
        assert_eq!(model.line(0), "let mut foo = 1;");
        assert_eq!(model.line(1), "let mut bar = 2;");

        // Undo edits
        assert!(model.undo());
        assert_eq!(model.line(0), "let mut a = 1;");
        assert_eq!(model.line(1), "let mut b = 2;");

        // Redo edits
        assert!(model.redo());
        assert_eq!(model.line(0), "let mut foo = 1;");
        assert_eq!(model.line(1), "let mut bar = 2;");
    }
}
