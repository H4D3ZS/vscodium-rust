// Pure Rust IntelliSense Auto-Completion Engine for Monaco Editor Engine.
// Provides fuzzy-ranked suggestions with icons, details, and snippet insertion.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompletionKind {
    Keyword,
    Function,
    Variable,
    Struct,
    Trait,
    Module,
    Snippet,
    Method,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionKind,
    pub detail: String,
    pub insert_text: String,
}

#[derive(Clone, Debug, Default)]
pub struct CompletionState {
    pub is_open: bool,
    pub selected_index: usize,
    pub items: Vec<CompletionItem>,
    pub prefix: String,
    pub trigger_pos: (usize, usize),
}

impl CompletionState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open(&mut self, items: Vec<CompletionItem>, prefix: String, pos: (usize, usize)) {
        self.items = items;
        self.prefix = prefix;
        self.trigger_pos = pos;
        self.selected_index = 0;
        self.is_open = !self.items.is_empty();
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.items.clear();
        self.prefix.clear();
        self.selected_index = 0;
    }

    pub fn select_next(&mut self) {
        if !self.items.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.items.len();
        }
    }

    pub fn select_prev(&mut self) {
        if !self.items.is_empty() {
            self.selected_index = if self.selected_index == 0 {
                self.items.len().saturating_sub(1)
            } else {
                self.selected_index - 1
            };
        }
    }

    pub fn current_item(&self) -> Option<&CompletionItem> {
        self.items.get(self.selected_index)
    }

    pub fn default_rust_completions() -> Vec<CompletionItem> {
        vec![
            CompletionItem {
                label: "fn".into(),
                kind: CompletionKind::Snippet,
                detail: "fn name() -> Type { ... }".into(),
                insert_text: "fn ".into(),
            },
            CompletionItem {
                label: "pub fn".into(),
                kind: CompletionKind::Snippet,
                detail: "pub fn name() -> Type { ... }".into(),
                insert_text: "pub fn ".into(),
            },
            CompletionItem {
                label: "println!".into(),
                kind: CompletionKind::Function,
                detail: "macro println!(...)".into(),
                insert_text: "println!(\"{}\");".into(),
            },
            CompletionItem {
                label: "struct".into(),
                kind: CompletionKind::Keyword,
                detail: "struct Name { ... }".into(),
                insert_text: "struct ".into(),
            },
            CompletionItem {
                label: "impl".into(),
                kind: CompletionKind::Keyword,
                detail: "impl Type { ... }".into(),
                insert_text: "impl ".into(),
            },
            CompletionItem {
                label: "match".into(),
                kind: CompletionKind::Keyword,
                detail: "match expr { ... }".into(),
                insert_text: "match ".into(),
            },
            CompletionItem {
                label: "String::new()".into(),
                kind: CompletionKind::Function,
                detail: "Creates a new empty String".into(),
                insert_text: "String::new()".into(),
            },
            CompletionItem {
                label: "Vec::new()".into(),
                kind: CompletionKind::Function,
                detail: "Creates a new empty Vec".into(),
                insert_text: "Vec::new()".into(),
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completion_navigation() {
        let mut cs = CompletionState::new();
        cs.open(
            CompletionState::default_rust_completions(),
            "f".into(),
            (0, 1),
        );
        assert!(cs.is_open);
        assert_eq!(cs.selected_index, 0);

        cs.select_next();
        assert_eq!(cs.selected_index, 1);

        cs.select_prev();
        assert_eq!(cs.selected_index, 0);

        cs.close();
        assert!(!cs.is_open);
    }
}
