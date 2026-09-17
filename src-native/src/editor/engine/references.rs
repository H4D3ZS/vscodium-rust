// References Subsystem for Monaco Engine.
// Manages the Find All References peek session, search results, and navigation.

use crate::editor::engine::vsx::languages::DefinitionLocation;

#[derive(Clone, Debug, Default)]
pub struct ReferencesState {
    pub is_open: bool,
    pub symbol_name: String,
    pub references: Vec<DefinitionLocation>,
    pub selected_idx: usize,
    pub anchor_row: usize,
    pub anchor_col: usize,
}

impl ReferencesState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open(&mut self, symbol: String, refs: Vec<DefinitionLocation>, row: usize, col: usize) {
        if refs.is_empty() {
            self.close();
            return;
        }
        self.symbol_name = symbol;
        self.references = refs;
        self.selected_idx = 0;
        self.anchor_row = row;
        self.anchor_col = col;
        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.symbol_name.clear();
        self.references.clear();
        self.selected_idx = 0;
    }

    pub fn select_next(&mut self) {
        if !self.references.is_empty() && self.selected_idx + 1 < self.references.len() {
            self.selected_idx += 1;
        }
    }

    pub fn select_prev(&mut self) {
        if self.selected_idx > 0 {
            self.selected_idx -= 1;
        }
    }

    pub fn selected_reference(&self) -> Option<&DefinitionLocation> {
        self.references.get(self.selected_idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_references_navigation() {
        let mut state = ReferencesState::new();
        let refs = vec![
            DefinitionLocation {
                file_path: PathBuf::from("src/main.rs"),
                row: 10,
                col: 4,
                preview: "let x = foo();".into(),
            },
            DefinitionLocation {
                file_path: PathBuf::from("src/lib.rs"),
                row: 25,
                col: 8,
                preview: "foo();".into(),
            },
        ];

        state.open("foo".into(), refs, 5, 2);
        assert!(state.is_open);
        assert_eq!(state.symbol_name, "foo");
        assert_eq!(state.references.len(), 2);
        assert_eq!(state.selected_idx, 0);

        state.select_next();
        assert_eq!(state.selected_idx, 1);
        assert_eq!(state.selected_reference().unwrap().row, 25);

        state.select_next(); // clamped
        assert_eq!(state.selected_idx, 1);

        state.select_prev();
        assert_eq!(state.selected_idx, 0);

        state.close();
        assert!(!state.is_open);
    }
}
