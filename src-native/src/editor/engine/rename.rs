// Rename Symbol Subsystem for Monaco Engine.
// Manages the inline rename session, target symbol coordinates, and new name input.

use std::path::PathBuf;

#[derive(Clone, Debug, Default)]
pub struct RenameState {
    pub is_open: bool,
    pub original_name: String,
    pub new_name: String,
    pub row: usize,
    pub col: usize,
    pub file_path: PathBuf,
}

impl RenameState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open(&mut self, symbol: String, row: usize, col: usize, file_path: PathBuf) {
        self.original_name = symbol.clone();
        self.new_name = symbol;
        self.row = row;
        self.col = col;
        self.file_path = file_path;
        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.original_name.clear();
        self.new_name.clear();
    }

    pub fn insert_char(&mut self, c: char) {
        self.new_name.push(c);
    }

    pub fn backspace(&mut self) {
        self.new_name.pop();
    }

    pub fn has_changed(&self) -> bool {
        !self.new_name.is_empty() && self.new_name != self.original_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rename_state_input() {
        let mut state = RenameState::new();
        state.open("old_fn".into(), 10, 4, PathBuf::from("main.rs"));
        assert!(state.is_open);
        assert_eq!(state.original_name, "old_fn");
        assert_eq!(state.new_name, "old_fn");
        assert!(!state.has_changed());

        state.new_name.clear();
        state.insert_char('n');
        state.insert_char('e');
        state.insert_char('w');
        assert_eq!(state.new_name, "new");
        assert!(state.has_changed());

        state.backspace();
        assert_eq!(state.new_name, "ne");

        state.close();
        assert!(!state.is_open);
    }
}
