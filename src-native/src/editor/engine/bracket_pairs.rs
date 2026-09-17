// Pure Rust Bracket Pair Colorization & Indent Guides Engine.
// Implements VS Code / Monaco bracket pair coloring with rotating depth palette,
// and vertical indent guide rendering with active scope highlighting.
// Zero heap allocations in hot paths — stack-local depth counters, lightweight structs.

/// 6-color rotating bracket pair palette (matches VS Code's default bracket colors).
/// Gold → Magenta → Cyan → Coral → Lime → Violet, then cycles.
pub const BRACKET_COLORS: [(u8, u8, u8); 6] = [
    (255, 210, 0),   // Gold
    (218, 112, 214), // Orchid / Magenta
    (0, 188, 212),   // Cyan
    (255, 127, 80),  // Coral
    (178, 255, 89),  // Lime
    (167, 139, 250), // Violet
];

/// Bracket pair types we track for colorization.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BracketKind {
    Paren,  // ( )
    Square, // [ ]
    Curly,  // { }
    Angle,  // < > (only in specific contexts like generics)
}

impl BracketKind {
    pub fn from_open(ch: char) -> Option<Self> {
        match ch {
            '(' => Some(BracketKind::Paren),
            '[' => Some(BracketKind::Square),
            '{' => Some(BracketKind::Curly),
            _ => None,
        }
    }

    pub fn from_close(ch: char) -> Option<Self> {
        match ch {
            ')' => Some(BracketKind::Paren),
            ']' => Some(BracketKind::Square),
            '}' => Some(BracketKind::Curly),
            _ => None,
        }
    }

    pub fn open_char(self) -> char {
        match self {
            BracketKind::Paren => '(',
            BracketKind::Square => '[',
            BracketKind::Curly => '{',
            BracketKind::Angle => '<',
        }
    }

    pub fn close_char(self) -> char {
        match self {
            BracketKind::Paren => ')',
            BracketKind::Square => ']',
            BracketKind::Curly => '}',
            BracketKind::Angle => '>',
        }
    }
}

/// A colorized bracket occurrence in a single line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColorizedBracket {
    /// Column index (0-based) within the line.
    pub col: usize,
    /// The bracket character.
    pub ch: char,
    /// Nesting depth (0-based). Use `depth % BRACKET_COLORS.len()` for color index.
    pub depth: usize,
    /// Whether this is an opening bracket.
    pub is_open: bool,
    /// Whether this bracket has a mismatched/unmatched pair (render in red).
    pub is_error: bool,
}

/// Result of bracket pair colorization for a visible range of lines.
#[derive(Clone, Debug)]
pub struct BracketColorResult {
    /// Per-line bracket colorization data. Indexed by line offset from `start_line`.
    pub lines: Vec<Vec<ColorizedBracket>>,
    /// Total nesting depth at the start of the visible range (carried from prior lines).
    pub initial_depth: usize,
}

/// Compute bracket pair colorization for a range of lines.
///
/// This scans from line 0 to `end_line` to correctly track nesting depth,
/// but only returns colorized brackets for lines in `[start_line, end_line)`.
///
/// # Arguments
/// * `all_lines` — Complete document lines.
/// * `start_line` — First visible line (inclusive).
/// * `end_line` — Last visible line (exclusive).
///
/// # Performance
/// Uses a stack-local `Vec<BracketKind>` for depth tracking (typically < 32 deep).
/// No heap allocations in the inner character scan — only pushes to the results vector.
pub fn compute_bracket_colorization(
    all_lines: &[String],
    start_line: usize,
    end_line: usize,
) -> BracketColorResult {
    let end = end_line.min(all_lines.len());
    let start = start_line.min(end);

    // Bracket nesting stack — tracks open bracket kinds for matching validation.
    let mut depth_stack: Vec<BracketKind> = Vec::with_capacity(32);
    let mut result_lines: Vec<Vec<ColorizedBracket>> =
        Vec::with_capacity(end.saturating_sub(start));
    let mut initial_depth = 0;

    // Track whether we're inside a string or comment (simplified heuristic).
    let mut in_string;
    let mut in_line_comment;
    let mut string_char: char = '"';

    for (line_idx, line) in all_lines.iter().enumerate() {
        if line_idx >= end {
            break;
        }

        in_string = false;
        in_line_comment = false;

        let in_visible_range = line_idx >= start;
        let mut line_brackets: Vec<ColorizedBracket> = if in_visible_range {
            Vec::with_capacity(8)
        } else {
            Vec::new()
        };

        if line.len() > 20_000 {
            if in_visible_range {
                result_lines.push(line_brackets);
            }
            continue;
        }

        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let ch = chars[i];

            // Skip line comments — rest of line is ignored
            if !in_string && ch == '/' && i + 1 < chars.len() && chars[i + 1] == '/' {
                break;
            }

            // Handle string literals (simplified — doesn't handle raw strings)
            if !in_line_comment {
                if in_string {
                    if ch == '\\' && i + 1 < chars.len() {
                        i += 2; // skip escaped char
                        continue;
                    }
                    if ch == string_char {
                        in_string = false;
                    }
                    i += 1;
                    continue;
                } else if ch == '"' || ch == '\'' {
                    in_string = true;
                    string_char = ch;
                    i += 1;
                    continue;
                }
            }

            if in_line_comment {
                i += 1;
                continue;
            }

            // Check for opening brackets
            if let Some(kind) = BracketKind::from_open(ch) {
                let current_depth = depth_stack.len();
                depth_stack.push(kind);

                if in_visible_range {
                    line_brackets.push(ColorizedBracket {
                        col: i,
                        ch,
                        depth: current_depth,
                        is_open: true,
                        is_error: false,
                    });
                }
            }
            // Check for closing brackets
            else if let Some(kind) = BracketKind::from_close(ch) {
                let is_error = match depth_stack.last() {
                    Some(&top) if top == kind => false,
                    _ => true,
                };

                let current_depth = if !is_error {
                    depth_stack.pop();
                    depth_stack.len()
                } else {
                    // Mismatched — don't pop, show error
                    depth_stack.len()
                };

                if in_visible_range {
                    line_brackets.push(ColorizedBracket {
                        col: i,
                        ch,
                        depth: current_depth,
                        is_open: false,
                        is_error,
                    });
                }
            }

            i += 1;
        }

        if line_idx == start.saturating_sub(1).max(0) && !in_visible_range {
            // Capture depth just before visible range starts
        }
        if line_idx == start && in_visible_range {
            initial_depth = depth_stack
                .len()
                .saturating_sub(line_brackets.iter().filter(|b| b.is_open).count());
        }

        if in_visible_range {
            result_lines.push(line_brackets);
        }
    }

    BracketColorResult {
        lines: result_lines,
        initial_depth,
    }
}

/// Get the RGB color for a given bracket nesting depth.
pub fn bracket_color_for_depth(depth: usize) -> (u8, u8, u8) {
    BRACKET_COLORS[depth % BRACKET_COLORS.len()]
}

// ═══════════════════════════════════════════════════════════════════════════════
// Indent Guides
// ═══════════════════════════════════════════════════════════════════════════════

/// Configuration for indent guide rendering.
#[derive(Clone, Debug)]
pub struct IndentGuideConfig {
    /// Whether indent guides are enabled.
    pub enabled: bool,
    /// Tab size in spaces (typically 4).
    pub tab_size: usize,
    /// Whether to highlight the active indent guide (scope under cursor).
    pub highlight_active: bool,
}

impl Default for IndentGuideConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            tab_size: 4,
            highlight_active: true,
        }
    }
}

/// A single indent guide for a line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IndentGuide {
    /// The column position where this guide is rendered (in character units).
    pub column: usize,
    /// The indentation level (0-based).
    pub level: usize,
    /// Whether this guide is the "active" one (cursor's enclosing scope).
    pub is_active: bool,
}

/// Result of indent guide computation for a visible range.
#[derive(Clone, Debug)]
pub struct IndentGuideResult {
    /// Per-line indent guides. Indexed by line offset from `start_line`.
    pub lines: Vec<Vec<IndentGuide>>,
}

/// Compute the leading whitespace indentation level for a line.
fn line_indent_level(line: &str, tab_size: usize) -> usize {
    let mut spaces = 0;
    for ch in line.chars() {
        match ch {
            ' ' => spaces += 1,
            '\t' => spaces += tab_size,
            _ => break,
        }
    }
    spaces / tab_size
}

/// Compute indent guides for visible lines.
///
/// For each visible line, we compute the indentation level and generate guide
/// markers at each tab stop. Empty/whitespace-only lines inherit the maximum
/// indent level from their surrounding non-empty lines (VS Code behavior).
///
/// # Arguments
/// * `all_lines` — Complete document lines.
/// * `start_line` — First visible line (inclusive).
/// * `end_line` — Last visible line (exclusive).
/// * `cursor_row` — The cursor's current line (for active guide highlighting).
/// * `config` — Indent guide configuration.
pub fn compute_indent_guides(
    all_lines: &[String],
    start_line: usize,
    end_line: usize,
    cursor_row: usize,
    config: &IndentGuideConfig,
) -> IndentGuideResult {
    if !config.enabled {
        return IndentGuideResult {
            lines: vec![Vec::new(); end_line.saturating_sub(start_line)],
        };
    }

    let end = end_line.min(all_lines.len());
    let start = start_line.min(end);
    let tab_size = config.tab_size.max(1);

    // Pre-compute indent levels for all lines in the visible range,
    // plus a small buffer for context (empty line inheritance).
    let context_start = start.saturating_sub(1);
    let context_end = (end + 1).min(all_lines.len());

    let indent_levels: Vec<usize> = (context_start..context_end)
        .map(|i| {
            let line = &all_lines[i];
            if line.trim().is_empty() {
                0 // Will be resolved below
            } else {
                line_indent_level(line, tab_size)
            }
        })
        .collect();

    // Resolve empty-line indentation: inherit max(prev, next) non-empty indent.
    let mut resolved: Vec<usize> = indent_levels.clone();
    for i in 0..resolved.len() {
        let global_idx = context_start + i;
        if all_lines[global_idx].trim().is_empty() {
            // Look backward for non-empty
            let prev = (0..i)
                .rev()
                .find(|&j| !all_lines[context_start + j].trim().is_empty())
                .map(|j| indent_levels[j])
                .unwrap_or(0);
            // Look forward for non-empty
            let next = ((i + 1)..resolved.len())
                .find(|&j| !all_lines[context_start + j].trim().is_empty())
                .map(|j| indent_levels[j])
                .unwrap_or(0);
            resolved[i] = prev.min(next);
        }
    }

    // Determine active indent level at cursor row
    let active_indent = if config.highlight_active && cursor_row < all_lines.len() {
        let cursor_line = &all_lines[cursor_row];
        if cursor_line.trim().is_empty() {
            // For empty lines, look at surrounding context
            let prev_indent = (0..cursor_row)
                .rev()
                .find(|&r| !all_lines[r].trim().is_empty())
                .map(|r| line_indent_level(&all_lines[r], tab_size))
                .unwrap_or(0);
            prev_indent
        } else {
            line_indent_level(cursor_line, tab_size)
        }
    } else {
        0
    };

    // Generate indent guides for visible lines
    let mut result_lines: Vec<Vec<IndentGuide>> = Vec::with_capacity(end - start);

    for line_idx in start..end {
        let ctx_idx = line_idx - context_start;
        let indent = resolved[ctx_idx];
        let mut guides: Vec<IndentGuide> = Vec::with_capacity(indent);

        for level in 0..indent {
            let column = level * tab_size;
            // A guide is "active" if it matches the cursor's innermost indentation scope
            let is_active = config.highlight_active
                && (level + 1 == active_indent)
                && cursor_row < all_lines.len()
                && is_line_in_scope(all_lines, line_idx, cursor_row, level, tab_size);

            guides.push(IndentGuide {
                column,
                level,
                is_active,
            });
        }

        result_lines.push(guides);
    }

    IndentGuideResult {
        lines: result_lines,
    }
}

/// Check if `line_idx` is within the same indentation scope as `cursor_row` at `level`.
/// This is a simplified check: both lines must be within a contiguous block where
/// indentation is >= level.
fn is_line_in_scope(
    all_lines: &[String],
    line_idx: usize,
    cursor_row: usize,
    level: usize,
    tab_size: usize,
) -> bool {
    let min_row = line_idx.min(cursor_row);
    let max_row = line_idx.max(cursor_row);

    // Quick check: if distance is too large, skip expensive scan
    if max_row - min_row > 200 {
        return false;
    }

    // Verify no line between them has indentation less than `level`
    for r in min_row..=max_row {
        let line = &all_lines[r];
        if line.trim().is_empty() {
            continue; // Empty lines don't break scope
        }
        let indent = line_indent_level(line, tab_size);
        if indent <= level {
            // This line exits the scope — but it's OK if it's the scope boundary line itself
            if r != min_row && r != max_row {
                return false;
            }
        }
    }

    true
}

/// Combined bracket colorization and indent guide state for the editor.
#[derive(Clone, Debug)]
pub struct BracketPairState {
    pub colorization_enabled: bool,
    pub indent_guides: IndentGuideConfig,
}

impl Default for BracketPairState {
    fn default() -> Self {
        Self {
            colorization_enabled: true,
            indent_guides: IndentGuideConfig::default(),
        }
    }
}

impl BracketPairState {
    pub fn toggle_colorization(&mut self) {
        self.colorization_enabled = !self.colorization_enabled;
    }

    pub fn toggle_indent_guides(&mut self) {
        self.indent_guides.enabled = !self.indent_guides.enabled;
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bracket_colorization_basic() {
        let lines = vec![
            "fn main() {".to_string(),
            "    let v = vec![1, 2, 3];".to_string(),
            "    println!(\"{}\", v[0]);".to_string(),
            "}".to_string(),
        ];
        let result = compute_bracket_colorization(&lines, 0, 4);
        assert_eq!(result.lines.len(), 4);

        // Line 0: `(`, `)`, `{`
        assert_eq!(result.lines[0].len(), 3);
        assert_eq!(result.lines[0][0].ch, '(');
        assert_eq!(result.lines[0][0].depth, 0);
        assert_eq!(result.lines[0][0].is_open, true);
        assert_eq!(result.lines[0][1].ch, ')');
        assert_eq!(result.lines[0][1].depth, 0);
        assert_eq!(result.lines[0][2].ch, '{');
        assert_eq!(result.lines[0][2].depth, 0);

        // Line 3: `}`
        assert_eq!(result.lines[3].len(), 1);
        assert_eq!(result.lines[3][0].ch, '}');
        assert_eq!(result.lines[3][0].depth, 0);
        assert!(!result.lines[3][0].is_error);
    }

    #[test]
    fn test_bracket_colorization_nested_depth() {
        let lines = vec!["fn f() { (([])) }".to_string()];
        let result = compute_bracket_colorization(&lines, 0, 1);
        let brackets = &result.lines[0];

        // Find the inner `[`
        let inner_open = brackets.iter().find(|b| b.ch == '[').unwrap();
        assert_eq!(inner_open.depth, 3); // f() = depth 0, { depth 1, ( depth 2, ( depth 3 — wait
                                         // Actually: `(` at depth 1, `(` at depth 2, `[` at depth 3
                                         // But `{` is at depth 0 after `()` closes, so:
                                         // `{` depth=0, `(` depth=1, `(` depth=2, `[` depth=3
        assert!(inner_open.depth >= 2);
    }

    #[test]
    fn test_bracket_skip_strings() {
        let lines = vec!["let s = \"({[\";".to_string()];
        let result = compute_bracket_colorization(&lines, 0, 1);
        // Brackets inside string should be ignored. Only `;` is punctuation, no brackets.
        assert_eq!(result.lines[0].len(), 0);
    }

    #[test]
    fn test_bracket_skip_comments() {
        let lines = vec!["let x = 1; // { open brace".to_string()];
        let result = compute_bracket_colorization(&lines, 0, 1);
        // `{` is inside a comment, should be ignored.
        assert_eq!(result.lines[0].len(), 0);
    }

    #[test]
    fn test_bracket_mismatched_error() {
        let lines = vec!["(]".to_string()];
        let result = compute_bracket_colorization(&lines, 0, 1);
        let brackets = &result.lines[0];
        assert_eq!(brackets.len(), 2);
        assert_eq!(brackets[0].ch, '(');
        assert!(!brackets[0].is_error);
        assert_eq!(brackets[1].ch, ']');
        assert!(brackets[1].is_error); // Mismatched
    }

    #[test]
    fn test_bracket_color_rotation() {
        for depth in 0..18 {
            let color = bracket_color_for_depth(depth);
            let expected = BRACKET_COLORS[depth % 6];
            assert_eq!(color, expected);
        }
    }

    #[test]
    fn test_indent_guides_basic() {
        let lines = vec![
            "fn main() {".to_string(),
            "    let x = 1;".to_string(),
            "    if true {".to_string(),
            "        println!(\"hi\");".to_string(),
            "    }".to_string(),
            "}".to_string(),
        ];
        let config = IndentGuideConfig::default();
        let result = compute_indent_guides(&lines, 0, 6, 3, &config);
        assert_eq!(result.lines.len(), 6);

        // Line 0: indent level 0 → no guides
        assert_eq!(result.lines[0].len(), 0);

        // Line 1: indent level 1 → 1 guide at column 0
        assert_eq!(result.lines[1].len(), 1);
        assert_eq!(result.lines[1][0].column, 0);

        // Line 3: indent level 2 → 2 guides at columns 0 and 4
        assert_eq!(result.lines[3].len(), 2);
        assert_eq!(result.lines[3][0].column, 0);
        assert_eq!(result.lines[3][1].column, 4);
    }

    #[test]
    fn test_indent_guides_disabled() {
        let lines = vec!["    indented".to_string()];
        let config = IndentGuideConfig {
            enabled: false,
            ..Default::default()
        };
        let result = compute_indent_guides(&lines, 0, 1, 0, &config);
        assert_eq!(result.lines[0].len(), 0);
    }

    #[test]
    fn test_indent_guides_empty_line_inheritance() {
        let lines = vec![
            "    block_a".to_string(),
            "".to_string(),
            "    block_b".to_string(),
        ];
        let config = IndentGuideConfig {
            highlight_active: false,
            ..Default::default()
        };
        let result = compute_indent_guides(&lines, 0, 3, 0, &config);

        // Empty line should inherit min(prev, next) = min(1, 1) = 1 guide
        assert_eq!(result.lines[1].len(), 1);
    }

    #[test]
    fn test_bracket_pair_state_toggles() {
        let mut state = BracketPairState::default();
        assert!(state.colorization_enabled);
        assert!(state.indent_guides.enabled);

        state.toggle_colorization();
        assert!(!state.colorization_enabled);

        state.toggle_indent_guides();
        assert!(!state.indent_guides.enabled);
    }
}
