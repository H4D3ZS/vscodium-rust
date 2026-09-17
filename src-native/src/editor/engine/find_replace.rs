// Pure Rust Regex Find & Replace Engine.
// Full feature parity with Monaco / VS Code find widget:
// Supports regular expressions, capture groups ($1, $2), case-sensitivity, whole-word matching,
// match highlighting, find next/prev wrap-around, and atomic replace-all.

use regex::{Regex, RegexBuilder};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FindMatch {
    pub row: usize,
    pub start_col: usize,
    pub end_col: usize,
    pub text: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FindOptions {
    pub case_sensitive: bool,
    pub whole_word: bool,
    pub is_regex: bool,
}

pub struct FindReplaceEngine;

impl FindReplaceEngine {
    /// Compiles a Regex from query and options, or constructs a literal/word pattern
    pub fn compile_pattern(query: &str, options: &FindOptions) -> Option<Regex> {
        if query.is_empty() {
            return None;
        }

        let pattern = if options.is_regex {
            if options.whole_word {
                format!(r"\b(?:{query})\b")
            } else {
                query.to_string()
            }
        } else {
            let escaped = regex::escape(query);
            if options.whole_word {
                format!(r"\b{escaped}\b")
            } else {
                escaped
            }
        };

        RegexBuilder::new(&pattern)
            .case_insensitive(!options.case_sensitive)
            .build()
            .ok()
    }

    /// Finds all matches in the buffer
    pub fn find_all(lines: &[String], query: &str, options: &FindOptions) -> Vec<FindMatch> {
        let regex = match Self::compile_pattern(query, options) {
            Some(r) => r,
            None => return Vec::new(),
        };

        let mut matches = Vec::new();
        for (row, line) in lines.iter().enumerate() {
            for m in regex.find_iter(line) {
                matches.push(FindMatch {
                    row,
                    start_col: m.start(),
                    end_col: m.end(),
                    text: m.as_str().to_string(),
                });
            }
        }
        matches
    }

    /// Finds the next match relative to current cursor position, wrapping around if needed
    pub fn find_next(
        lines: &[String],
        query: &str,
        options: &FindOptions,
        current_row: usize,
        current_col: usize,
    ) -> Option<FindMatch> {
        let all = Self::find_all(lines, query, options);
        if all.is_empty() {
            return None;
        }

        // Look for first match after current position
        for m in &all {
            if m.row > current_row || (m.row == current_row && m.start_col > current_col) {
                return Some(m.clone());
            }
        }

        // Wrap around to the very first match
        all.first().cloned()
    }

    /// Finds the previous match relative to current cursor position, wrapping around if needed
    pub fn find_prev(
        lines: &[String],
        query: &str,
        options: &FindOptions,
        current_row: usize,
        current_col: usize,
    ) -> Option<FindMatch> {
        let all = Self::find_all(lines, query, options);
        if all.is_empty() {
            return None;
        }

        // Look for last match strictly before current position
        for m in all.iter().rev() {
            if m.row < current_row || (m.row == current_row && m.start_col < current_col) {
                return Some(m.clone());
            }
        }

        // Wrap around to the last match
        all.last().cloned()
    }

    /// Replace one match with the replacement string (supporting capture groups $1, $2, etc.)
    pub fn replace_one(
        lines: &mut [String],
        query: &str,
        replacement: &str,
        options: &FindOptions,
        m: &FindMatch,
    ) -> bool {
        let regex = match Self::compile_pattern(query, options) {
            Some(r) => r,
            None => return false,
        };

        let norm_replacement = Self::normalize_replacement(replacement);

        if let Some(line) = lines.get_mut(m.row) {
            if m.start_col <= line.len() && m.end_col <= line.len() {
                let target_slice = &line[m.start_col..m.end_col];
                let replaced = regex
                    .replace(target_slice, norm_replacement.as_str())
                    .into_owned();

                let mut new_line =
                    String::with_capacity(line.len() - (m.end_col - m.start_col) + replaced.len());
                new_line.push_str(&line[..m.start_col]);
                new_line.push_str(&replaced);
                new_line.push_str(&line[m.end_col..]);

                *line = new_line;
                return true;
            }
        }
        false
    }

    /// Replace all occurrences across all lines, returning total replacements made
    pub fn replace_all(
        lines: &mut [String],
        query: &str,
        replacement: &str,
        options: &FindOptions,
    ) -> usize {
        let regex = match Self::compile_pattern(query, options) {
            Some(r) => r,
            None => return 0,
        };

        let norm_replacement = Self::normalize_replacement(replacement);

        let mut total = 0;
        for line in lines.iter_mut() {
            if regex.is_match(line) {
                let matches_in_line = regex.find_iter(line).count();
                total += matches_in_line;
                *line = regex
                    .replace_all(line, norm_replacement.as_str())
                    .into_owned();
            }
        }
        total
    }

    /// Normalizes VS Code / Monaco style $1 capture groups to Rust regex ${1} syntax
    pub fn normalize_replacement(replacement: &str) -> String {
        let mut out = String::with_capacity(replacement.len() + 8);
        let chars: Vec<char> = replacement.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            if chars[i] == '$' && i + 1 < chars.len() && chars[i + 1].is_ascii_digit() {
                out.push_str("${");
                i += 1;
                while i < chars.len() && chars[i].is_ascii_digit() {
                    out.push(chars[i]);
                    i += 1;
                }
                out.push('}');
                continue;
            }
            out.push(chars[i]);
            i += 1;
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_all_basic_and_case_sensitivity() {
        let lines = vec![
            "Hello world".to_string(),
            "hello rust".to_string(),
            "HELLO GPUI".to_string(),
        ];

        // Case insensitive
        let opt_i = FindOptions {
            case_sensitive: false,
            whole_word: false,
            is_regex: false,
        };
        let m1 = FindReplaceEngine::find_all(&lines, "hello", &opt_i);
        assert_eq!(m1.len(), 3);

        // Case sensitive
        let opt_s = FindOptions {
            case_sensitive: true,
            whole_word: false,
            is_regex: false,
        };
        let m2 = FindReplaceEngine::find_all(&lines, "hello", &opt_s);
        assert_eq!(m2.len(), 1);
        assert_eq!(m2[0].row, 1);
    }

    #[test]
    fn test_find_whole_word() {
        let lines = vec!["cat concatenate cat".to_string()];
        let opt = FindOptions {
            case_sensitive: false,
            whole_word: true,
            is_regex: false,
        };
        let matches = FindReplaceEngine::find_all(&lines, "cat", &opt);
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].start_col, 0);
        assert_eq!(matches[1].start_col, 16);
    }

    #[test]
    fn test_regex_with_capture_groups() {
        let mut lines = vec![
            "fn compute_a() {}".to_string(),
            "fn compute_b() {}".to_string(),
        ];
        let opt = FindOptions {
            case_sensitive: true,
            whole_word: false,
            is_regex: true,
        };

        let count = FindReplaceEngine::replace_all(&mut lines, r"fn (\w+)", "pub fn $1_v2", &opt);
        assert_eq!(count, 2);
        assert_eq!(lines[0], "pub fn compute_a_v2() {}");
        assert_eq!(lines[1], "pub fn compute_b_v2() {}");
    }

    #[test]
    fn test_find_next_and_prev_wrapping() {
        let lines = vec!["alpha".to_string(), "beta".to_string(), "alpha".to_string()];
        let opt = FindOptions::default();

        // Next from line 0 -> line 2
        let n1 = FindReplaceEngine::find_next(&lines, "alpha", &opt, 0, 1).unwrap();
        assert_eq!(n1.row, 2);

        // Next from line 2 -> wrap to line 0
        let n2 = FindReplaceEngine::find_next(&lines, "alpha", &opt, 2, 1).unwrap();
        assert_eq!(n2.row, 0);

        // Prev from line 0 -> wrap to line 2
        let p1 = FindReplaceEngine::find_prev(&lines, "alpha", &opt, 0, 0).unwrap();
        assert_eq!(p1.row, 2);
    }
}
