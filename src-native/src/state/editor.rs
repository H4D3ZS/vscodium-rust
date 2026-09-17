// Editor Tab and Buffer State for Native GPUI powered by Monaco Engine.

use crate::editor::engine::MonacoModel;

#[derive(Clone, Debug)]
pub struct OpenTab {
    pub path: String,
    pub title: String,
    pub dirty: bool,
    pub is_pinned: bool,
    pub cursor_row: usize, // 0-indexed line
    pub cursor_col: usize, // 0-indexed char column
    pub scroll_row: usize,
    pub lines: Vec<String>,
    pub line_tokens: Vec<Vec<crate::editor::engine::tokenizer::TokenSpan>>,
    pub model: MonacoModel,
    pub markdown_reader_mode: bool,
    pub is_settings: bool,
}

impl OpenTab {
    pub fn new_settings() -> Self {
        let mut tab = Self::new(
            "hades://settings".to_string(),
            "Settings".to_string(),
            String::new(),
        );
        tab.is_settings = true;
        tab
    }

    pub fn new(path: String, title: String, content: String) -> Self {
        let lower = path.to_lowercase();
        let is_large_file = content.len() > 1_000_000;
        let is_markdown = lower.ends_with(".md") || lower.ends_with(".markdown");
        let lang = if is_large_file {
            "plaintext"
        } else if lower.ends_with(".rs") {
            "rust"
        } else if lower.ends_with(".ts") || lower.ends_with(".tsx") {
            "typescript"
        } else if lower.ends_with(".js") || lower.ends_with(".jsx") {
            "javascript"
        } else if lower.ends_with(".py") {
            "python"
        } else if lower.ends_with(".json") {
            "json"
        } else if lower.ends_with(".toml") {
            "toml"
        } else if is_markdown {
            "markdown"
        } else if lower.ends_with(".html") || lower.ends_with(".htm") {
            "html"
        } else if lower.ends_with(".css") || lower.ends_with(".scss") {
            "css"
        } else if lower.ends_with(".sh") || lower.ends_with(".bash") {
            "bash"
        } else if lower.ends_with(".yaml") || lower.ends_with(".yml") {
            "yaml"
        } else if lower.ends_with(".sql") {
            "sql"
        } else {
            "plaintext"
        };

        let mut model = MonacoModel::new(content.clone(), lang.to_string());
        let lines = model.buffer.lines().to_vec();
        let line_tokens = if is_large_file {
            Vec::new()
        } else {
            lines
                .iter()
                .map(|l| crate::editor::engine::tokenizer::Tokenizer::tokenize_line(l, lang))
                .collect()
        };

        Self {
            path,
            title,
            dirty: false,
            is_pinned: false,
            cursor_row: 0,
            cursor_col: 0,
            scroll_row: 0,
            lines,
            line_tokens,
            model,
            markdown_reader_mode: is_markdown,
            is_settings: false,
        }
    }

    pub fn refresh_line_tokens(&mut self) {
        self.line_tokens = self
            .lines
            .iter()
            .map(|l| {
                crate::editor::engine::tokenizer::Tokenizer::tokenize_line(l, &self.model.language)
            })
            .collect();
    }

    fn sync_from_model(&mut self) {
        self.cursor_row = self.model.cursor_row();
        self.cursor_col = self.model.cursor_col();
        self.scroll_row = self.model.scroll_row;
        let new_lines = self.model.buffer.lines().to_vec();
        if new_lines.len() != self.lines.len() {
            self.lines = new_lines;
            self.refresh_line_tokens();
        } else {
            self.lines = new_lines;
            if self.line_tokens.len() != self.lines.len() {
                self.refresh_line_tokens();
            } else if let Some(line) = self.lines.get(self.cursor_row) {
                self.line_tokens[self.cursor_row] =
                    crate::editor::engine::tokenizer::Tokenizer::tokenize_line(
                        line,
                        &self.model.language,
                    );
            }
        }
    }

    pub fn content(&self) -> String {
        self.lines.join("\n")
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn current_line(&self) -> &str {
        self.lines
            .get(self.cursor_row)
            .map(|s| s.as_str())
            .unwrap_or("")
    }

    pub fn insert_char(&mut self, c: char) {
        self.model.insert_char(c);
        self.dirty = true;
        self.sync_from_model();
    }

    pub fn insert_newline(&mut self) {
        self.model.insert_newline();
        self.dirty = true;
        self.sync_from_model();
    }

    pub fn backspace(&mut self) {
        self.model.backspace();
        self.dirty = true;
        self.sync_from_model();
    }

    pub fn delete(&mut self) {
        self.model.delete();
        self.dirty = true;
        self.sync_from_model();
    }

    pub fn move_left(&mut self) {
        self.model.move_left(false);
        self.sync_from_model();
    }

    pub fn move_right(&mut self) {
        self.model.move_right(false);
        self.sync_from_model();
    }

    pub fn move_up(&mut self) {
        self.model.move_up(false);
        self.sync_from_model();
    }

    pub fn move_down(&mut self) {
        self.model.move_down(false);
        self.sync_from_model();
    }

    pub fn undo(&mut self) -> bool {
        let res = self.model.undo();
        if res {
            self.sync_from_model();
        }
        res
    }

    pub fn redo(&mut self) -> bool {
        let res = self.model.redo();
        if res {
            self.sync_from_model();
        }
        res
    }

    pub fn toggle_comment(&mut self) {
        self.model.toggle_comment();
        self.dirty = true;
        self.sync_from_model();
    }

    pub fn add_cursor_above(&mut self) {
        self.model.add_cursor_above();
        self.sync_from_model();
    }

    pub fn add_cursor_below(&mut self) {
        self.model.add_cursor_below();
        self.sync_from_model();
    }

    pub fn add_next_occurrence(&mut self) -> bool {
        let res = crate::editor::engine::multi_cursor::add_next_occurrence(&mut self.model);
        self.sync_from_model();
        res
    }

    pub fn select_all_occurrences(&mut self) -> usize {
        let count = crate::editor::engine::multi_cursor::select_all_occurrences(&mut self.model);
        self.sync_from_model();
        count
    }

    pub fn cursor_undo(&mut self) -> bool {
        let res = crate::editor::engine::multi_cursor::cursor_undo(&mut self.model);
        self.sync_from_model();
        res
    }

    pub fn clear_secondary_cursors(&mut self) {
        self.model.cursors.clear_secondary();
        self.sync_from_model();
    }

    pub fn select_all(&mut self) {
        self.model.select_all();
        self.sync_from_model();
    }

    pub fn scroll_by(&mut self, delta: i32) {
        self.model.scroll_by(delta);
        self.scroll_row = self.model.scroll_row;
    }

    pub fn scroll_to(&mut self, row: usize) {
        self.model.scroll_to(row);
        self.scroll_row = self.model.scroll_row;
    }

    pub fn ensure_cursor_visible(&mut self, visible_lines: usize) {
        self.model.ensure_cursor_visible(visible_lines);
        self.scroll_row = self.model.scroll_row;
    }

    pub fn save_to_disk(&mut self) -> std::io::Result<()> {
        let content = self.model.content();
        std::fs::write(&self.path, &content)?;
        self.dirty = false;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opentab_insert_char_and_backspace() {
        let mut tab = OpenTab::new("test.rs".into(), "test.rs".into(), "".into());
        assert_eq!(tab.content(), "");
        tab.insert_char('f');
        tab.insert_char('n');
        tab.insert_char(' ');
        assert_eq!(tab.content(), "fn ");
        assert_eq!(tab.cursor_col, 3);
        assert!(tab.dirty);

        tab.backspace();
        assert_eq!(tab.content(), "fn");
        assert_eq!(tab.cursor_col, 2);
    }

    #[test]
    fn test_opentab_newline_and_multiline_navigation() {
        let mut tab = OpenTab::new("test.rs".into(), "test.rs".into(), "line1\nline2".into());
        assert_eq!(tab.lines.len(), 2);
        assert_eq!(tab.cursor_row, 0);

        tab.move_down();
        assert_eq!(tab.cursor_row, 1);

        tab.insert_newline();
        assert_eq!(tab.lines.len(), 3);
        assert_eq!(tab.cursor_row, 2);
        assert_eq!(tab.cursor_col, 0);

        tab.move_up();
        assert_eq!(tab.cursor_row, 1);
    }

    #[test]
    fn test_opentab_delete() {
        let mut tab = OpenTab::new("test.rs".into(), "test.rs".into(), "ab".into());
        tab.cursor_col = 0;
        tab.model.set_cursor(0, 0);
        tab.delete();
        assert_eq!(tab.content(), "b");
    }

    #[test]
    fn test_opentab_scrolling() {
        let text = (0..100)
            .map(|i| format!("line {i}"))
            .collect::<Vec<_>>()
            .join("\n");
        let mut tab = OpenTab::new("scroll.rs".into(), "scroll.rs".into(), text);
        assert_eq!(tab.scroll_row, 0);

        tab.scroll_by(25);
        assert_eq!(tab.scroll_row, 25);

        tab.scroll_by(-10);
        assert_eq!(tab.scroll_row, 15);

        tab.scroll_by(-50);
        assert_eq!(tab.scroll_row, 0);

        tab.scroll_to(80);
        assert_eq!(tab.scroll_row, 80);

        tab.cursor_row = 10;
        tab.model.set_cursor(10, 0);
        tab.ensure_cursor_visible(30);
        assert_eq!(tab.scroll_row, 10);
    }

    #[test]
    fn test_opentab_undo_redo() {
        let mut tab = OpenTab::new("test.rs".into(), "test.rs".into(), "".into());
        tab.insert_char('x');
        assert_eq!(tab.content(), "x");
        assert!(tab.undo());
        assert_eq!(tab.content(), "");
        assert!(tab.redo());
        assert_eq!(tab.content(), "x");
    }
}
