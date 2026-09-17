// Code Action & Quick Fix Subsystem for Monaco Engine.
// Manages available LSP CodeActions, selection index, and anchor position.

use crate::editor::engine::lsp::LspCodeAction;

#[derive(Clone, Debug, Default)]
pub struct CodeActionState {
    pub is_open: bool,
    pub actions: Vec<LspCodeAction>,
    pub selected_idx: usize,
    pub anchor_row: usize,
    pub anchor_col: usize,
}

impl CodeActionState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open(&mut self, actions: Vec<LspCodeAction>, row: usize, col: usize) {
        if actions.is_empty() {
            self.close();
            return;
        }
        self.actions = actions;
        self.selected_idx = 0;
        self.anchor_row = row;
        self.anchor_col = col;
        self.is_open = true;
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.actions.clear();
        self.selected_idx = 0;
    }

    pub fn select_next(&mut self) {
        if !self.actions.is_empty() && self.selected_idx + 1 < self.actions.len() {
            self.selected_idx += 1;
        }
    }

    pub fn select_prev(&mut self) {
        if self.selected_idx > 0 {
            self.selected_idx -= 1;
        }
    }

    pub fn selected_action(&self) -> Option<&LspCodeAction> {
        self.actions.get(self.selected_idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_action_state_navigation() {
        let mut state = CodeActionState::new();
        let actions = vec![
            LspCodeAction {
                title: "Fix 1".into(),
                kind: Some("quickfix".into()),
                is_preferred: true,
                edit: None,
            },
            LspCodeAction {
                title: "Fix 2".into(),
                kind: Some("refactor".into()),
                is_preferred: false,
                edit: None,
            },
        ];

        state.open(actions, 2, 5);
        assert!(state.is_open);
        assert_eq!(state.selected_idx, 0);
        assert_eq!(state.selected_action().unwrap().title, "Fix 1");

        state.select_next();
        assert_eq!(state.selected_idx, 1);
        assert_eq!(state.selected_action().unwrap().title, "Fix 2");

        state.select_next(); // clamped at end
        assert_eq!(state.selected_idx, 1);

        state.select_prev();
        assert_eq!(state.selected_idx, 0);

        state.close();
        assert!(!state.is_open);
        assert!(state.selected_action().is_none());
    }
}
