// Pure Rust Sticky Scroll Engine for Monaco Editor.
// Tracks enclosing functions, structs, impl blocks, and classes
// as the user scrolls, pinning active scope headers at the top of the viewport.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StickyHeader {
    pub row: usize,
    pub text: String,
    pub badge: &'static str,
    pub indent_spaces: usize,
}

#[derive(Clone, Debug)]
pub struct StickyScrollModel {
    pub enabled: bool,
    pub max_headers: usize,
}

impl Default for StickyScrollModel {
    fn default() -> Self {
        Self {
            enabled: true,
            max_headers: 3,
        }
    }
}

impl StickyScrollModel {
    pub fn new() -> Self {
        Self::default()
    }

    /// Compute active sticky headers for the given scroll row
    pub fn compute_headers(
        &self,
        lines: &[String],
        folding_ranges: &[crate::editor::engine::folding::FoldingRange],
        scroll_row: usize,
    ) -> Vec<StickyHeader> {
        if !self.enabled || scroll_row == 0 || lines.is_empty() {
            return Vec::new();
        }

        let mut candidate_ranges: Vec<_> = folding_ranges
            .iter()
            .filter(|r| r.start_row < scroll_row && r.end_row >= scroll_row)
            .collect();

        // Sort by start_row ascending so outermost scope is first
        candidate_ranges.sort_by_key(|r| r.start_row);

        let mut headers = Vec::new();
        for r in candidate_ranges {
            if let Some(line) = lines.get(r.start_row) {
                if let Some(badge) = detect_scope_badge(line) {
                    let indent = line.chars().take_while(|c| c.is_whitespace()).count();
                    headers.push(StickyHeader {
                        row: r.start_row,
                        text: line.trim().to_string(),
                        badge,
                        indent_spaces: indent,
                    });
                }
            }
            if headers.len() >= self.max_headers {
                break;
            }
        }

        headers
    }
}

pub fn detect_scope_badge(line: &str) -> Option<&'static str> {
    let trimmed = line.trim();
    if trimmed.starts_with("//") || trimmed.starts_with('#') || trimmed.is_empty() {
        return None;
    }

    let words: Vec<&str> = trimmed.split_whitespace().collect();
    for (i, word) in words.iter().enumerate() {
        match *word {
            "fn" => return Some("fn"),
            "struct" => return Some("struct"),
            "enum" => return Some("enum"),
            "trait" => return Some("trait"),
            "impl" => return Some("impl"),
            "mod" => return Some("mod"),
            "class" => return Some("class"),
            "function" => return Some("function"),
            "interface" => return Some("interface"),
            "def" => return Some("def"),
            _ => {}
        }
        if i > 5 {
            break;
        }
    }

    None
}

/// Compute enclosing scopes for the active cursor row to render in Breadcrumbs
pub fn compute_breadcrumbs_trail(
    lines: &[String],
    folding_ranges: &[crate::editor::engine::folding::FoldingRange],
    cursor_row: usize,
) -> Vec<StickyHeader> {
    if lines.is_empty() {
        return Vec::new();
    }

    let mut candidate_ranges: Vec<_> = folding_ranges
        .iter()
        .filter(|r| r.start_row <= cursor_row && r.end_row >= cursor_row)
        .collect();

    candidate_ranges.sort_by_key(|r| r.start_row);

    let mut headers = Vec::new();
    for r in candidate_ranges {
        if let Some(line) = lines.get(r.start_row) {
            if let Some(badge) = detect_scope_badge(line) {
                let indent = line.chars().take_while(|c| c.is_whitespace()).count();
                let raw_clean = line.trim().trim_end_matches('{').trim();
                let without_pub = raw_clean.strip_prefix("pub ").unwrap_or(raw_clean).trim();
                let text = without_pub
                    .strip_prefix(badge)
                    .unwrap_or(without_pub)
                    .trim();
                let final_text = if text.is_empty() { without_pub } else { text }.to_string();
                headers.push(StickyHeader {
                    row: r.start_row,
                    text: final_text,
                    badge,
                    indent_spaces: indent,
                });
            }
        }
        if headers.len() >= 2 {
            break;
        }
    }

    headers
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::engine::folding::FoldingModel;
    use core::prelude::v1::test;

    #[test]
    fn test_sticky_scroll_computation() {
        let code = vec![
            "// My Module".to_string(),
            "pub mod my_mod {".to_string(),
            "    pub struct Worker {".to_string(),
            "        pub id: u64,".to_string(),
            "    }".to_string(),
            "    impl Worker {".to_string(),
            "        pub fn process(&self) {".to_string(),
            "            let x = 10;".to_string(),
            "            let y = 20;".to_string(),
            "            println!(\"{x} {y}\");".to_string(),
            "        }".to_string(),
            "    }".to_string(),
            "}".to_string(),
        ];

        let mut folding = FoldingModel::new();
        folding.compute_from_lines(&code);

        let sticky = StickyScrollModel::new();

        // At scroll_row 0 -> nothing pinned
        let h0 = sticky.compute_headers(&code, &folding.ranges, 0);
        assert!(h0.is_empty());

        // At scroll_row 8 (inside Worker::process)
        // Scopes enclosing line 8 are: my_mod (line 1), Worker impl (line 5), process fn (line 6)
        let h8 = sticky.compute_headers(&code, &folding.ranges, 8);
        assert_eq!(h8.len(), 3);
        assert_eq!(h8[0].row, 1);
        assert_eq!(h8[0].badge, "mod");
        assert_eq!(h8[1].row, 5);
        assert_eq!(h8[1].badge, "impl");
        assert_eq!(h8[2].row, 6);
        assert_eq!(h8[2].badge, "fn");
    }

    #[test]
    fn test_breadcrumbs_trail_computation() {
        let code = vec![
            "pub struct Node {".to_string(),
            "    pub val: i32,".to_string(),
            "}".to_string(),
            "impl Node {".to_string(),
            "    pub fn get_val(&self) -> i32 {".to_string(),
            "        self.val".to_string(),
            "    }".to_string(),
            "}".to_string(),
        ];

        let mut folding = FoldingModel::new();
        folding.compute_from_lines(&code);

        // At cursor line 5 (inside get_val)
        let trail = compute_breadcrumbs_trail(&code, &folding.ranges, 5);
        assert_eq!(trail.len(), 2);
        assert_eq!(trail[0].badge, "impl");
        assert_eq!(trail[1].badge, "fn");
        assert_eq!(trail[1].row, 4);
    }
}
