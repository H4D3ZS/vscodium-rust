// VS Code Snippet Parser & Interactive Tab Stop Session Engine.
// Supports $1, $2, ${1:default}, ${2|opt1,opt2|}, and $0 final stop.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TabStopRange {
    pub start_offset: usize,
    pub end_offset: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TabStop {
    pub index: usize,
    pub default_value: String,
    pub ranges: Vec<TabStopRange>,
}

#[derive(Clone, Debug)]
pub struct ParsedSnippet {
    pub plain_text: String,
    pub tab_stops: Vec<TabStop>,
}

/// Parse a VS Code snippet template into plain text and relative tab stop ranges.
pub fn parse_snippet(template: &str) -> ParsedSnippet {
    let mut plain_text = String::with_capacity(template.len());
    let mut tab_stops: Vec<TabStop> = Vec::new();

    let chars: Vec<char> = template.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        if chars[i] == '\\' && i + 1 < len {
            // Escaped character: \$, \\, \}, etc.
            plain_text.push(chars[i + 1]);
            i += 2;
            continue;
        }

        if chars[i] == '$' && i + 1 < len {
            if chars[i + 1] == '{' {
                // Bracketed tabstop: ${1:placeholder} or ${1|choice1,choice2|} or ${1}
                i += 2; // skip ${
                let mut num_str = String::new();
                while i < len && chars[i].is_ascii_digit() {
                    num_str.push(chars[i]);
                    i += 1;
                }

                let index: usize = num_str.parse().unwrap_or(0);
                let mut default_val = String::new();

                if i < len && chars[i] == ':' {
                    i += 1; // skip ':'
                    let mut brace_depth = 1;
                    while i < len {
                        if chars[i] == '\\' && i + 1 < len {
                            default_val.push(chars[i + 1]);
                            i += 2;
                            continue;
                        }
                        if chars[i] == '{' {
                            brace_depth += 1;
                        } else if chars[i] == '}' {
                            brace_depth -= 1;
                            if brace_depth == 0 {
                                i += 1; // consume closing '}'
                                break;
                            }
                        }
                        default_val.push(chars[i]);
                        i += 1;
                    }
                } else if i < len && chars[i] == '|' {
                    // Choices: ${1|one,two,three|}
                    i += 1;
                    while i < len && chars[i] != '|' {
                        default_val.push(chars[i]);
                        i += 1;
                    }
                    if i < len && chars[i] == '|' {
                        i += 1;
                    }
                    if i < len && chars[i] == '}' {
                        i += 1;
                    }
                    // Default value is the first choice
                    if let Some(first) = default_val.split(',').next() {
                        default_val = first.to_string();
                    }
                } else if i < len && chars[i] == '}' {
                    i += 1; // consume '}'
                }

                let start = plain_text.len();
                plain_text.push_str(&default_val);
                let end = plain_text.len();

                record_tab_stop(&mut tab_stops, index, default_val, start, end);
                continue;
            } else if chars[i + 1].is_ascii_digit() {
                // Simple numeric tabstop: $1, $2, $0
                i += 1; // skip $
                let mut num_str = String::new();
                while i < len && chars[i].is_ascii_digit() {
                    num_str.push(chars[i]);
                    i += 1;
                }
                let index: usize = num_str.parse().unwrap_or(0);
                let start = plain_text.len();
                let end = start;

                record_tab_stop(&mut tab_stops, index, String::new(), start, end);
                continue;
            }
        }

        plain_text.push(chars[i]);
        i += 1;
    }

    // Sort tab stops: 1, 2, 3, ... followed by 0 (final cursor stop)
    tab_stops.sort_by(|a, b| {
        if a.index == 0 {
            std::cmp::Ordering::Greater
        } else if b.index == 0 {
            std::cmp::Ordering::Less
        } else {
            a.index.cmp(&b.index)
        }
    });

    // If no $0 was specified, add a virtual $0 at the end of the text
    if !tab_stops.iter().any(|s| s.index == 0) {
        let end = plain_text.len();
        tab_stops.push(TabStop {
            index: 0,
            default_value: String::new(),
            ranges: vec![TabStopRange {
                start_offset: end,
                end_offset: end,
            }],
        });
    }

    ParsedSnippet {
        plain_text,
        tab_stops,
    }
}

fn record_tab_stop(
    stops: &mut Vec<TabStop>,
    index: usize,
    default_value: String,
    start: usize,
    end: usize,
) {
    if let Some(existing) = stops.iter_mut().find(|s| s.index == index) {
        existing.ranges.push(TabStopRange {
            start_offset: start,
            end_offset: end,
        });
        if existing.default_value.is_empty() && !default_value.is_empty() {
            existing.default_value = default_value;
        }
    } else {
        stops.push(TabStop {
            index,
            default_value,
            ranges: vec![TabStopRange {
                start_offset: start,
                end_offset: end,
            }],
        });
    }
}

/// Active interactive snippet session in the editor.
#[derive(Clone, Debug)]
pub struct SnippetSession {
    pub tab_stops: Vec<TabStop>,
    pub current_stop_idx: usize,
    pub insertion_base_offset: usize,
}

impl SnippetSession {
    pub fn new(parsed: ParsedSnippet, base_offset: usize) -> Option<Self> {
        if parsed.tab_stops.is_empty() {
            return None;
        }

        Some(Self {
            tab_stops: parsed.tab_stops,
            current_stop_idx: 0,
            insertion_base_offset: base_offset,
        })
    }

    pub fn current_stop(&self) -> Option<&TabStop> {
        self.tab_stops.get(self.current_stop_idx)
    }

    /// Advance to the next tab stop. Returns None if session completes (reached $0 or beyond).
    pub fn next(&mut self) -> Option<&TabStop> {
        if self.current_stop_idx + 1 < self.tab_stops.len() {
            self.current_stop_idx += 1;
            let stop = &self.tab_stops[self.current_stop_idx];
            if stop.index == 0 {
                // Reached final stop
                Some(stop)
            } else {
                Some(stop)
            }
        } else {
            None
        }
    }

    /// Step backward to previous tab stop.
    pub fn prev(&mut self) -> Option<&TabStop> {
        if self.current_stop_idx > 0 {
            self.current_stop_idx -= 1;
            Some(&self.tab_stops[self.current_stop_idx])
        } else {
            None
        }
    }

    pub fn is_at_final_stop(&self) -> bool {
        self.tab_stops
            .get(self.current_stop_idx)
            .map(|s| s.index == 0)
            .unwrap_or(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_snippet() {
        let parsed = parse_snippet("fn $1($2) -> $3 {\n    $0\n}");
        assert_eq!(parsed.plain_text, "fn () ->  {\n    \n}");
        assert_eq!(parsed.tab_stops.len(), 4);
        assert_eq!(parsed.tab_stops[0].index, 1);
        assert_eq!(parsed.tab_stops[1].index, 2);
        assert_eq!(parsed.tab_stops[2].index, 3);
        assert_eq!(parsed.tab_stops[3].index, 0);
    }

    #[test]
    fn test_parse_placeholders_with_defaults() {
        let parsed = parse_snippet("let ${1:x}: ${2:i32} = ${3:42};$0");
        assert_eq!(parsed.plain_text, "let x: i32 = 42;");
        assert_eq!(parsed.tab_stops[0].default_value, "x");
        assert_eq!(parsed.tab_stops[1].default_value, "i32");
        assert_eq!(parsed.tab_stops[2].default_value, "42");
    }

    #[test]
    fn test_parse_mirrored_tabstops() {
        let parsed = parse_snippet("for ${1:item} in ${2:items} {\n    println!(\"{}\", $1);\n}");
        assert_eq!(parsed.tab_stops[0].ranges.len(), 2);
    }

    #[test]
    fn test_choices_parsing() {
        let parsed = parse_snippet("${1|public,private,protected|} class Foo {}");
        assert_eq!(parsed.plain_text, "public class Foo {}");
        assert_eq!(parsed.tab_stops[0].default_value, "public");
    }

    #[test]
    fn test_session_navigation() {
        let parsed = parse_snippet("test ${1:first} ${2:second} $0");
        let mut session = SnippetSession::new(parsed, 10).unwrap();

        assert_eq!(session.current_stop().unwrap().index, 1);
        assert_eq!(session.next().unwrap().index, 2);
        assert_eq!(session.next().unwrap().index, 0);
        assert!(session.is_at_final_stop());

        assert_eq!(session.prev().unwrap().index, 2);
        assert_eq!(session.prev().unwrap().index, 1);
    }
}
