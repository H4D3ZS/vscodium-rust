// Pure Rust Multi-Cursor & Selection Model for Monaco Editor Engine.
// Supports primary and secondary cursors, range selections, and multi-caret navigation.

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Selection {
    pub anchor: Position,
    pub active: Position,
}

impl Selection {
    pub fn new(anchor: Position, active: Position) -> Self {
        Self { anchor, active }
    }

    pub fn point(pos: Position) -> Self {
        Self {
            anchor: pos,
            active: pos,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.anchor == self.active
    }

    pub fn start(&self) -> Position {
        self.anchor.min(self.active)
    }

    pub fn end(&self) -> Position {
        self.anchor.max(self.active)
    }

    pub fn is_reversed(&self) -> bool {
        self.active < self.anchor
    }

    pub fn contains(&self, pos: Position) -> bool {
        pos >= self.start() && pos <= self.end()
    }

    pub fn collapse_to_active(&mut self) {
        self.anchor = self.active;
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CursorSet {
    pub primary: Selection,
    pub secondary: Vec<Selection>,
}

impl Default for CursorSet {
    fn default() -> Self {
        Self {
            primary: Selection::point(Position::new(0, 0)),
            secondary: Vec::new(),
        }
    }
}

impl CursorSet {
    pub fn new(primary_pos: Position) -> Self {
        Self {
            primary: Selection::point(primary_pos),
            secondary: Vec::new(),
        }
    }

    pub fn main_cursor(&self) -> Position {
        self.primary.active
    }

    pub fn set_main_cursor(&mut self, pos: Position) {
        self.primary = Selection::point(pos);
        self.secondary.clear();
    }

    pub fn add_cursor(&mut self, pos: Position) {
        let sel = Selection::point(pos);
        if self.primary != sel && !self.secondary.contains(&sel) {
            self.secondary.push(sel);
        }
    }

    pub fn clear_secondary(&mut self) {
        self.secondary.clear();
    }

    pub fn all_selections(&self) -> Vec<Selection> {
        let mut all = vec![self.primary];
        all.extend_from_slice(&self.secondary);
        all
    }

    pub fn all_cursors(&self) -> Vec<Position> {
        let mut cur = vec![self.primary.active];
        for s in &self.secondary {
            cur.push(s.active);
        }
        cur
    }

    pub fn collapse_all(&mut self) {
        self.primary.collapse_to_active();
        for s in &mut self.secondary {
            s.collapse_to_active();
        }
    }

    pub fn deduplicate(&mut self) {
        let mut unique: Vec<Selection> = Vec::new();
        for s in self.all_selections() {
            if !unique.iter().any(|u| u.active == s.active) {
                unique.push(s);
            }
        }
        if let Some(first) = unique.first() {
            self.primary = *first;
            self.secondary = unique[1..].to_vec();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_bounds() {
        let sel = Selection::new(Position::new(5, 10), Position::new(2, 4));
        assert!(!sel.is_empty());
        assert!(sel.is_reversed());
        assert_eq!(sel.start(), Position::new(2, 4));
        assert_eq!(sel.end(), Position::new(5, 10));
        assert!(sel.contains(Position::new(3, 0)));
        assert!(!sel.contains(Position::new(1, 0)));
    }

    #[test]
    fn test_multicursor_set() {
        let mut cs = CursorSet::new(Position::new(0, 0));
        cs.add_cursor(Position::new(1, 0));
        cs.add_cursor(Position::new(2, 0));
        assert_eq!(cs.all_cursors().len(), 3);

        cs.clear_secondary();
        assert_eq!(cs.all_cursors().len(), 1);
    }
}
