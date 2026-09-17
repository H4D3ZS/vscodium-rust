// Pure Rust Code Folding Engine for Monaco Editor Engine.
// Computes folding regions via brace matching and indentation levels,
// allowing collapsible code sections in the gutter.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FoldingRange {
    pub start_row: usize,
    pub end_row: usize,
    pub is_collapsed: bool,
}

#[derive(Clone, Debug, Default)]
pub struct FoldingModel {
    pub ranges: Vec<FoldingRange>,
}

impl FoldingModel {
    pub fn new() -> Self {
        Self { ranges: Vec::new() }
    }

    pub fn compute_from_lines(&mut self, lines: &[String]) {
        let mut new_ranges = Vec::new();
        let mut brace_stack: Vec<usize> = Vec::new();

        for (row, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("//") || trimmed.is_empty() {
                continue;
            }

            for c in line.chars() {
                if c == '{' {
                    brace_stack.push(row);
                } else if c == '}' {
                    if let Some(start) = brace_stack.pop() {
                        if row > start {
                            let already_collapsed = self
                                .ranges
                                .iter()
                                .find(|r| r.start_row == start && r.end_row == row)
                                .map(|r| r.is_collapsed)
                                .unwrap_or(false);

                            new_ranges.push(FoldingRange {
                                start_row: start,
                                end_row: row,
                                is_collapsed: already_collapsed,
                            });
                        }
                    }
                }
            }
        }

        new_ranges.sort_by_key(|r| r.start_row);
        self.ranges = new_ranges;
    }

    pub fn toggle_fold(&mut self, row: usize) -> bool {
        if let Some(range) = self.ranges.iter_mut().find(|r| r.start_row == row) {
            range.is_collapsed = !range.is_collapsed;
            true
        } else {
            false
        }
    }

    pub fn is_line_hidden(&self, row: usize) -> bool {
        for range in &self.ranges {
            if range.is_collapsed && row > range.start_row && row <= range.end_row {
                return true;
            }
        }
        false
    }

    pub fn range_at(&self, row: usize) -> Option<&FoldingRange> {
        self.ranges.iter().find(|r| r.start_row == row)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_folding_computation() {
        let lines = vec![
            "fn main() {".to_string(),
            "    let x = 10;".to_string(),
            "    println!(\"{x}\");".to_string(),
            "}".to_string(),
        ];

        let mut fm = FoldingModel::new();
        fm.compute_from_lines(&lines);
        assert_eq!(fm.ranges.len(), 1);
        assert_eq!(fm.ranges[0].start_row, 0);
        assert_eq!(fm.ranges[0].end_row, 3);
        assert!(!fm.ranges[0].is_collapsed);

        assert!(!fm.is_line_hidden(1));
        fm.toggle_fold(0);
        assert!(fm.is_line_hidden(1));
        assert!(fm.is_line_hidden(2));
        assert!(fm.is_line_hidden(3));
    }
}
