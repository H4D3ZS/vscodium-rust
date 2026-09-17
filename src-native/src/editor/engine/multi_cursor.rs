// Pure Rust Multi-Cursor Selection & Word Occurrence Highlighting Engine for Monaco Editor.
// Implements VS Code-grade Ctrl+D (add next occurrence), Ctrl+Shift+L (select all),
// Ctrl+U (cursor undo), Escape to collapse, and viewport occurrence highlight markers.

use super::cursor::{Position, Selection};
use super::model::MonacoModel;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OccurrenceHighlight {
    pub row: usize,
    pub start_col: usize,
    pub end_col: usize,
    pub is_active_selection: bool,
}

#[derive(Clone, Debug)]
pub struct MultiCursorState {
    pub highlight_occurrences: bool,
}

impl Default for MultiCursorState {
    fn default() -> Self {
        Self {
            highlight_occurrences: true,
        }
    }
}

impl MultiCursorState {
    pub fn toggle_highlights(&mut self) -> bool {
        self.highlight_occurrences = !self.highlight_occurrences;
        self.highlight_occurrences
    }
}

/// Locate identifier boundaries around `col` in `line`.
/// Returns `(start_col, end_col, word_text)` if a valid word exists.
pub fn word_at_position(line: &str, col: usize) -> Option<(usize, usize, String)> {
    let chars: Vec<char> = line.chars().collect();
    if chars.is_empty() {
        return None;
    }

    let is_ident_char = |c: char| c.is_alphanumeric() || c == '_';

    let probe_idx = if col >= chars.len() {
        chars.len().saturating_sub(1)
    } else {
        col
    };

    // If probing directly on a non-ident, try one char before if col > 0
    let target_idx = if !is_ident_char(chars[probe_idx]) {
        if col > 0 && is_ident_char(chars[col - 1]) {
            col - 1
        } else {
            return None;
        }
    } else {
        probe_idx
    };

    let mut start = target_idx;
    while start > 0 && is_ident_char(chars[start - 1]) {
        start -= 1;
    }

    let mut end = target_idx;
    while end < chars.len() && is_ident_char(chars[end]) {
        end += 1;
    }

    if start == end {
        return None;
    }

    let word: String = chars[start..end].iter().collect();
    Some((start, end, word))
}

/// Extract text enclosed by a `Selection`.
pub fn get_selection_text(lines: &[String], sel: &Selection) -> Option<String> {
    if sel.is_empty() {
        return None;
    }

    let start = sel.start();
    let end = sel.end();

    if start.row == end.row {
        let line = lines.get(start.row)?;
        let chars: Vec<char> = line.chars().collect();
        let s_col = start.col.min(chars.len());
        let e_col = end.col.min(chars.len());
        if s_col < e_col {
            return Some(chars[s_col..e_col].iter().collect());
        }
        return None;
    }

    // Multi-line selection
    let mut result = String::new();
    for r in start.row..=end.row.min(lines.len().saturating_sub(1)) {
        let line = &lines[r];
        let chars: Vec<char> = line.chars().collect();
        if r == start.row {
            let s = start.col.min(chars.len());
            let part: String = chars[s..].iter().collect();
            result.push_str(&part);
            result.push('\n');
        } else if r == end.row {
            let e = end.col.min(chars.len());
            let part: String = chars[..e].iter().collect();
            result.push_str(&part);
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }

    Some(result)
}

/// Add next occurrence of the selected text (or word at cursor) to the cursor set (Ctrl+D).
/// If primary selection is empty: expands selection to current word.
/// If primary selection is active: finds the next non-selected match and adds it.
pub fn add_next_occurrence(model: &mut MonacoModel) -> bool {
    let lines = model.buffer.lines().to_vec();

    if model.cursors.primary.is_empty() {
        let cur = model.cursors.main_cursor();
        let line = lines.get(cur.row).map(|s| s.as_str()).unwrap_or("");
        if let Some((start_col, end_col, _)) = word_at_position(line, cur.col) {
            model.cursors.primary = Selection::new(
                Position::new(cur.row, start_col),
                Position::new(cur.row, end_col),
            );
            return true;
        }
        return false;
    }

    let target = match get_selection_text(&lines, &model.cursors.primary) {
        Some(t) if !t.is_empty() => t,
        _ => return false,
    };

    let all_sels = model.cursors.all_selections();

    // Find the latest selection in reading order to search after it
    let latest_pos = all_sels
        .iter()
        .map(|s| s.end())
        .max()
        .unwrap_or(model.cursors.primary.end());

    let total_lines = lines.len();
    if total_lines == 0 {
        return false;
    }

    // Pass 1: search from latest_pos to end of file
    for r in latest_pos.row..total_lines {
        let line = &lines[r];
        let min_col = if r == latest_pos.row {
            latest_pos.col
        } else {
            0
        };

        if let Some(col) = find_occurrence_in_line(line, &target, min_col) {
            let end_col = col + target.chars().count();
            let new_sel = Selection::new(Position::new(r, col), Position::new(r, end_col));
            if !all_sels
                .iter()
                .any(|s| s.start() == new_sel.start() && s.end() == new_sel.end())
            {
                model.cursors.secondary.push(new_sel);
                model.ensure_cursor_visible(45);
                return true;
            }
        }
    }

    // Pass 2: wrap around from top of file to latest_pos
    for r in 0..=latest_pos.row {
        let line = &lines[r];
        let max_col = if r == latest_pos.row {
            latest_pos.col
        } else {
            line.len()
        };

        let mut search_col = 0;
        while let Some(col) = find_occurrence_in_line(line, &target, search_col) {
            if r == latest_pos.row && col >= max_col {
                break;
            }
            let end_col = col + target.chars().count();
            let new_sel = Selection::new(Position::new(r, col), Position::new(r, end_col));
            if !all_sels
                .iter()
                .any(|s| s.start() == new_sel.start() && s.end() == new_sel.end())
            {
                model.cursors.secondary.push(new_sel);
                model.ensure_cursor_visible(45);
                return true;
            }
            search_col = col + 1;
        }
    }

    false
}

/// Select all occurrences of the selected text (or word under cursor) in the entire file (Ctrl+Shift+L).
pub fn select_all_occurrences(model: &mut MonacoModel) -> usize {
    let lines = model.buffer.lines().to_vec();

    if model.cursors.primary.is_empty() {
        let cur = model.cursors.main_cursor();
        let line = lines.get(cur.row).map(|s| s.as_str()).unwrap_or("");
        if let Some((start_col, end_col, _)) = word_at_position(line, cur.col) {
            model.cursors.primary = Selection::new(
                Position::new(cur.row, start_col),
                Position::new(cur.row, end_col),
            );
        } else {
            return 0;
        }
    }

    let target = match get_selection_text(&lines, &model.cursors.primary) {
        Some(t) if !t.is_empty() => t,
        _ => return 0,
    };

    let target_char_len = target.chars().count();
    let mut matches = Vec::new();

    for (r, line) in lines.iter().enumerate() {
        let mut col_off = 0;
        while let Some(col) = find_occurrence_in_line(line, &target, col_off) {
            let end_col = col + target_char_len;
            matches.push(Selection::new(
                Position::new(r, col),
                Position::new(r, end_col),
            ));
            col_off = end_col;
        }
    }

    if matches.is_empty() {
        return 0;
    }

    let count = matches.len();
    model.cursors.primary = matches[0];
    model.cursors.secondary = matches[1..].to_vec();
    count
}

/// Undo the last added cursor from multi-cursor selection (Ctrl+U).
pub fn cursor_undo(model: &mut MonacoModel) -> bool {
    if !model.cursors.secondary.is_empty() {
        model.cursors.secondary.pop();
        true
    } else {
        false
    }
}

/// Helper to find character offset of substring within a line starting at `min_char_col`.
fn find_occurrence_in_line(line: &str, target: &str, min_char_col: usize) -> Option<usize> {
    let line_chars: Vec<char> = line.chars().collect();
    let target_chars: Vec<char> = target.chars().collect();

    if target_chars.is_empty() || target_chars.len() > line_chars.len() {
        return None;
    }

    let max_start = line_chars.len().saturating_sub(target_chars.len());
    for i in min_char_col..=max_start {
        if line_chars[i..i + target_chars.len()] == target_chars[..] {
            return Some(i);
        }
    }
    None
}

/// Compute visible occurrence highlights for the Monaco viewport.
/// Returns bounding columns for all matches within rows `visible_start..visible_end`.
pub fn compute_occurrence_highlights(
    lines: &[String],
    model: &MonacoModel,
    visible_start: usize,
    visible_end: usize,
) -> Vec<OccurrenceHighlight> {
    let (target, is_word_token) = if !model.cursors.primary.is_empty() {
        let text = get_selection_text(lines, &model.cursors.primary);
        match text {
            Some(t) if !t.contains('\n') && t.chars().count() <= 120 => (Some(t), false),
            _ => (None, false),
        }
    } else {
        let cur = model.cursors.main_cursor();
        let line = lines.get(cur.row).map(|s| s.as_str()).unwrap_or("");
        if let Some((_, _, word)) = word_at_position(line, cur.col) {
            if word.chars().count() >= 2 {
                (Some(word), true)
            } else {
                (None, false)
            }
        } else {
            (None, false)
        }
    };

    let target = match target {
        Some(t) if !t.is_empty() => t,
        _ => return Vec::new(),
    };

    let all_sels = model.cursors.all_selections();
    let target_len = target.chars().count();
    let mut highlights = Vec::new();

    let end_row = visible_end.min(lines.len());
    for r in visible_start..end_row {
        let line = &lines[r];
        let mut col_off = 0;

        while let Some(col) = find_occurrence_in_line(line, &target, col_off) {
            let end_col = col + target_len;

            // If it was auto-detected from word token at point, enforce whole-word matching
            if is_word_token {
                let chars: Vec<char> = line.chars().collect();
                let prev_is_ident =
                    col > 0 && (chars[col - 1].is_alphanumeric() || chars[col - 1] == '_');
                let next_is_ident = end_col < chars.len()
                    && (chars[end_col].is_alphanumeric() || chars[end_col] == '_');
                if prev_is_ident || next_is_ident {
                    col_off = col + 1;
                    continue;
                }
            }

            let is_active = all_sels.iter().any(|s| {
                let st = s.start();
                let en = s.end();
                st.row == r && st.col == col && en.row == r && en.col == end_col
            });

            highlights.push(OccurrenceHighlight {
                row: r,
                start_col: col,
                end_col,
                is_active_selection: is_active,
            });

            col_off = end_col.max(col + 1);
        }
    }

    highlights
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_at_position() {
        let line = "let mut count = count + 1;";
        // Probing 'count' at index 8
        let res = word_at_position(line, 8).unwrap();
        assert_eq!(res.0, 8);
        assert_eq!(res.1, 13);
        assert_eq!(res.2, "count");

        // Probing non-ident
        assert!(word_at_position(line, 14).is_none());
    }

    #[test]
    fn test_add_next_occurrence_from_point() {
        let content = "foo bar foo baz foo\n".to_string();
        let mut model = MonacoModel::new(content, "rust".to_string());
        model.set_cursor(0, 1); // inside first 'foo'

        // First Ctrl+D: selects 'foo'
        assert!(add_next_occurrence(&mut model));
        assert_eq!(model.cursors.primary.start(), Position::new(0, 0));
        assert_eq!(model.cursors.primary.end(), Position::new(0, 3));
        assert_eq!(model.cursors.secondary.len(), 0);

        // Second Ctrl+D: adds second 'foo'
        assert!(add_next_occurrence(&mut model));
        assert_eq!(model.cursors.secondary.len(), 1);
        assert_eq!(model.cursors.secondary[0].start(), Position::new(0, 8));
        assert_eq!(model.cursors.secondary[0].end(), Position::new(0, 11));

        // Third Ctrl+D: adds third 'foo'
        assert!(add_next_occurrence(&mut model));
        assert_eq!(model.cursors.secondary.len(), 2);
        assert_eq!(model.cursors.secondary[1].start(), Position::new(0, 16));
        assert_eq!(model.cursors.secondary[1].end(), Position::new(0, 19));

        // Fourth Ctrl+D: all matches selected, returns false
        assert!(!add_next_occurrence(&mut model));
    }

    #[test]
    fn test_select_all_occurrences() {
        let content = "abc 123 abc 456 abc\nabc 789".to_string();
        let mut model = MonacoModel::new(content, "rust".to_string());
        model.set_cursor(0, 0);

        let count = select_all_occurrences(&mut model);
        assert_eq!(count, 4);
        assert_eq!(model.cursors.secondary.len(), 3);
        assert_eq!(model.cursors.all_selections().len(), 4);
    }

    #[test]
    fn test_cursor_undo() {
        let content = "item item item".to_string();
        let mut model = MonacoModel::new(content, "rust".to_string());
        model.set_cursor(0, 0);

        add_next_occurrence(&mut model); // select item 1
        add_next_occurrence(&mut model); // add item 2
        add_next_occurrence(&mut model); // add item 3
        assert_eq!(model.cursors.secondary.len(), 2);

        assert!(cursor_undo(&mut model));
        assert_eq!(model.cursors.secondary.len(), 1);

        assert!(cursor_undo(&mut model));
        assert_eq!(model.cursors.secondary.len(), 0);

        assert!(!cursor_undo(&mut model));
    }

    #[test]
    fn test_occurrence_highlights_computation() {
        let lines = vec![
            "fn compute() {".to_string(),
            "    let value = 10;".to_string(),
            "    let next = value + 5;".to_string(),
            "    println!(\"{}\", value);".to_string(),
            "}".to_string(),
        ];
        let mut model = MonacoModel::new(lines.join("\n"), "rust".to_string());
        model.set_cursor(1, 10); // on 'value'

        let highlights = compute_occurrence_highlights(&lines, &model, 0, 5);
        assert_eq!(highlights.len(), 3);
        assert_eq!(highlights[0].row, 1);
        assert_eq!(highlights[1].row, 2);
        assert_eq!(highlights[2].row, 3);
    }
}
