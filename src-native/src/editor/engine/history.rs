// Pure Rust Undo / Redo Transaction Manager for Monaco Editor Engine.
// Supports multi-operation transactions and intelligent typing coalescing.

use super::cursor::{CursorSet, Position};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EditOperation {
    pub start_pos: Position,
    pub end_pos: Position,
    pub inserted_text: String,
    pub deleted_text: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transaction {
    pub operations: Vec<EditOperation>,
    pub before_cursors: CursorSet,
    pub after_cursors: CursorSet,
    pub timestamp_ms: u64,
}

#[derive(Clone, Debug)]
pub struct UndoRedoManager {
    undo_stack: Vec<Transaction>,
    redo_stack: Vec<Transaction>,
    max_history: usize,
}

impl Default for UndoRedoManager {
    fn default() -> Self {
        Self::new(500)
    }
}

impl UndoRedoManager {
    pub fn new(max_history: usize) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_history,
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    pub fn push_transaction(&mut self, tx: Transaction) {
        self.redo_stack.clear();

        // Check if we can coalesce with previous transaction (continuous typing within 800ms)
        if let Some(last) = self.undo_stack.last_mut() {
            if last.operations.len() == 1
                && tx.operations.len() == 1
                && last.operations[0].deleted_text.is_empty()
                && tx.operations[0].deleted_text.is_empty()
                && tx.operations[0].inserted_text.chars().count() == 1
                && !tx.operations[0].inserted_text.contains('\n')
                && tx.timestamp_ms.saturating_sub(last.timestamp_ms) < 800
                && tx.operations[0].start_pos == last.after_cursors.main_cursor()
            {
                last.operations[0]
                    .inserted_text
                    .push_str(&tx.operations[0].inserted_text);
                last.after_cursors = tx.after_cursors;
                last.timestamp_ms = tx.timestamp_ms;
                return;
            }
        }

        self.undo_stack.push(tx);
        if self.undo_stack.len() > self.max_history {
            self.undo_stack.remove(0);
        }
    }

    pub fn pop_undo(&mut self) -> Option<Transaction> {
        let tx = self.undo_stack.pop()?;
        self.redo_stack.push(tx.clone());
        Some(tx)
    }

    pub fn pop_redo(&mut self) -> Option<Transaction> {
        let tx = self.redo_stack.pop()?;
        self.undo_stack.push(tx.clone());
        Some(tx)
    }

    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_undo_redo_stack() {
        let mut mgr = UndoRedoManager::default();
        assert!(!mgr.can_undo());
        assert!(!mgr.can_redo());

        let tx = Transaction {
            operations: vec![EditOperation {
                start_pos: Position::new(0, 0),
                end_pos: Position::new(0, 0),
                inserted_text: "a".into(),
                deleted_text: "".into(),
            }],
            before_cursors: CursorSet::default(),
            after_cursors: CursorSet::new(Position::new(0, 1)),
            timestamp_ms: 1000,
        };

        mgr.push_transaction(tx);
        assert!(mgr.can_undo());
        assert!(!mgr.can_redo());

        let undone = mgr.pop_undo();
        assert!(undone.is_some());
        assert!(!mgr.can_undo());
        assert!(mgr.can_redo());

        let redone = mgr.pop_redo();
        assert!(redone.is_some());
        assert!(mgr.can_undo());
        assert!(!mgr.can_redo());
    }
}
