// Pure Rust Decorations & Diagnostic Markers for Monaco Editor Engine.
// Supports squiggly underlines, Git gutter diff bars, bracket matching, and inlay hints.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Information,
    Hint,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiagnosticMarker {
    pub row: usize,
    pub start_col: usize,
    pub end_col: usize,
    pub severity: DiagnosticSeverity,
    pub message: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GitGutterStatus {
    Added,
    Modified,
    Deleted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BracketMatch {
    pub open_pos: (usize, usize),
    pub close_pos: (usize, usize),
}

pub struct DecorationEngine;

impl DecorationEngine {
    pub fn find_matching_bracket(lines: &[String], row: usize, col: usize) -> Option<BracketMatch> {
        let line = lines.get(row)?;
        let chars: Vec<char> = line.chars().collect();
        let target_char = *chars.get(col)?;

        let (open_char, close_char, is_open) = match target_char {
            '{' => ('{', '}', true),
            '}' => ('{', '}', false),
            '(' => ('(', ')', true),
            ')' => ('(', ')', false),
            '[' => ('[', ']', true),
            ']' => ('[', ']', false),
            _ => return None,
        };

        if is_open {
            let mut depth = 1;
            for r in row..lines.len() {
                let l = &lines[r];
                let start_c = if r == row { col + 1 } else { 0 };
                for (c, ch) in l.chars().enumerate().skip(start_c) {
                    if ch == open_char {
                        depth += 1;
                    } else if ch == close_char {
                        depth -= 1;
                        if depth == 0 {
                            return Some(BracketMatch {
                                open_pos: (row, col),
                                close_pos: (r, c),
                            });
                        }
                    }
                }
            }
        } else {
            let mut depth = 1;
            for r in (0..=row).rev() {
                let l = &lines[r];
                let chars_rev: Vec<(usize, char)> = l.chars().enumerate().collect();
                let end_c = if r == row { col } else { chars_rev.len() };
                for &(c, ch) in chars_rev[..end_c].iter().rev() {
                    if ch == close_char {
                        depth += 1;
                    } else if ch == open_char {
                        depth -= 1;
                        if depth == 0 {
                            return Some(BracketMatch {
                                open_pos: (r, c),
                                close_pos: (row, col),
                            });
                        }
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matching_bracket_forward() {
        let lines = vec![
            "fn main() {".to_string(),
            "    let x = 10;".to_string(),
            "}".to_string(),
        ];
        let m = DecorationEngine::find_matching_bracket(&lines, 0, 10);
        assert!(m.is_some());
        let m = m.unwrap();
        assert_eq!(m.open_pos, (0, 10));
        assert_eq!(m.close_pos, (2, 0));
    }

    #[test]
    fn test_matching_bracket_backward() {
        let lines = vec![
            "fn main() {".to_string(),
            "    let x = 10;".to_string(),
            "}".to_string(),
        ];
        let m = DecorationEngine::find_matching_bracket(&lines, 2, 0);
        assert!(m.is_some());
        let m = m.unwrap();
        assert_eq!(m.open_pos, (0, 10));
        assert_eq!(m.close_pos, (2, 0));
    }
}
