// Pure Rust Predictive AI Ghost Text for Monaco Editor Engine.
// Displays inline faded autocomplete predictions like Cursor IDE and GitHub Copilot.

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GhostText {
    pub text: String,
    pub row: usize,
    pub col: usize,
    pub is_active: bool,
}

impl GhostText {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, text: String, row: usize, col: usize) {
        self.text = text;
        self.row = row;
        self.col = col;
        self.is_active = !self.text.is_empty();
    }

    pub fn clear(&mut self) {
        self.text.clear();
        self.is_active = false;
    }

    pub fn take(&mut self) -> Option<String> {
        if self.is_active && !self.text.is_empty() {
            let res = self.text.clone();
            self.clear();
            Some(res)
        } else {
            None
        }
    }
}
