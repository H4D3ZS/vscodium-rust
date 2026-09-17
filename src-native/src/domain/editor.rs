use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct OpenTab {
    pub path: String,
    pub title: String,
    pub dirty: bool,
    pub cursor_row: usize,
    pub cursor_col: usize,
    pub scroll_row: usize,
    pub lines: Vec<String>,
}

impl OpenTab {
    pub fn new(path: String, title: String, content: String) -> Self {
        let lines: Vec<String> = if content.is_empty() {
            vec![String::new()]
        } else {
            content.lines().map(|s| s.to_string()).collect()
        };

        Self {
            path,
            title,
            dirty: false,
            cursor_row: 0,
            cursor_col: 0,
            scroll_row: 0,
            lines: if lines.is_empty() {
                vec![String::new()]
            } else {
                lines
            },
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
        if self.cursor_row >= self.lines.len() {
            self.lines.resize(self.cursor_row + 1, String::new());
        }

        let line = &mut self.lines[self.cursor_row];
        let mut char_indices: Vec<usize> = line.char_indices().map(|(i, _)| i).collect();
        char_indices.push(line.len());

        let byte_pos = if self.cursor_col < char_indices.len() {
            char_indices[self.cursor_col]
        } else {
            line.len()
        };

        line.insert(byte_pos, c);
        self.cursor_col += 1;
        self.dirty = true;
    }

    pub fn insert_newline(&mut self) {
        if self.cursor_row >= self.lines.len() {
            self.lines.resize(self.cursor_row + 1, String::new());
        }

        let current = &mut self.lines[self.cursor_row];
        let char_indices: Vec<usize> = current.char_indices().map(|(i, _)| i).collect();
        let split_pos = if self.cursor_col < char_indices.len() {
            char_indices[self.cursor_col]
        } else {
            current.len()
        };

        let remainder = current.split_off(split_pos);
        self.cursor_row += 1;
        self.cursor_col = 0;
        self.lines.insert(self.cursor_row, remainder);
        self.dirty = true;
    }

    pub fn backspace(&mut self) {
        if self.cursor_col > 0 {
            let line = &mut self.lines[self.cursor_row];
            let char_indices: Vec<usize> = line.char_indices().map(|(i, _)| i).collect();
            if self.cursor_col <= char_indices.len() {
                let remove_idx = char_indices[self.cursor_col - 1];
                line.remove(remove_idx);
                self.cursor_col -= 1;
                self.dirty = true;
            }
        } else if self.cursor_row > 0 {
            let current = self.lines.remove(self.cursor_row);
            self.cursor_row -= 1;
            let prev = &mut self.lines[self.cursor_row];
            self.cursor_col = prev.chars().count();
            prev.push_str(&current);
            self.dirty = true;
        }
    }

    pub fn delete(&mut self) {
        if self.cursor_row < self.lines.len() {
            let line = &mut self.lines[self.cursor_row];
            let char_indices: Vec<usize> = line.char_indices().map(|(i, _)| i).collect();
            if self.cursor_col < char_indices.len() {
                let remove_idx = char_indices[self.cursor_col];
                line.remove(remove_idx);
                self.dirty = true;
            } else if self.cursor_row + 1 < self.lines.len() {
                let next_line = self.lines.remove(self.cursor_row + 1);
                self.lines[self.cursor_row].push_str(&next_line);
                self.dirty = true;
            }
        }
    }

    pub fn save_to_disk(&mut self) -> std::io::Result<()> {
        let content = self.content();
        std::fs::write(&self.path, content)?;
        self.dirty = false;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct FileNode {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub is_expanded: bool,
    pub children: Vec<FileNode>,
}
