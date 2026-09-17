// Monaco Editor Inlay Hints & CodeLens Engine.
// Supports type annotations, parameter names, chaining hints, and actionable CodeLens headers.

use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InlayHintKind {
    Type,
    Parameter,
    Chaining,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InlayHint {
    pub row: usize,
    pub col: usize,
    pub label: String,
    pub kind: InlayHintKind,
    pub tooltip: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodeLens {
    pub row: usize,
    pub label: String,
    pub command: Option<String>,
}

/// Computes inlay hints via syntax inference for the given source buffer
pub fn compute_inlay_hints(lines: &[String], language: &str) -> Vec<InlayHint> {
    let mut hints = Vec::new();
    let lang = language.to_lowercase();

    for (row, line) in lines.iter().enumerate() {
        let trimmed = line.trim_start();
        let indent = line.len() - trimmed.len();

        if lang == "rust" || lang == "rs" {
            // 1. Rust `let [mut] var = ...;` type hint (if no explicit type `: Type` provided)
            if trimmed.starts_with("let ") {
                let rest = trimmed["let ".len()..].trim_start();
                let rest = if rest.starts_with("mut ") {
                    rest["mut ".len()..].trim_start()
                } else {
                    rest
                };

                if let Some(eq_pos) = rest.find('=') {
                    let var_part = rest[..eq_pos].trim();
                    let val_part = rest[eq_pos + 1..].trim().trim_end_matches(';').trim();

                    // Only infer if no explicit type is declared
                    if !var_part.contains(':') && is_ident(var_part) {
                        if let Some(inferred_type) = infer_rust_type(val_part) {
                            let var_start = line.find(var_part).unwrap_or(indent);
                            let col = var_start + var_part.len();
                            hints.push(InlayHint {
                                row,
                                col,
                                label: format!(": {inferred_type}"),
                                kind: InlayHintKind::Type,
                                tooltip: Some(format!("Inferred type for `{var_part}`")),
                            });
                        }
                    }
                }
            }

            // 2. Chaining hints at end of builder methods (e.g. `.collect()`, `.iter()`)
            if trimmed.starts_with('.') && trimmed.contains('(') {
                if trimmed.contains(".iter()") {
                    hints.push(InlayHint {
                        row,
                        col: line.len(),
                        label: " [Iter]".to_string(),
                        kind: InlayHintKind::Chaining,
                        tooltip: Some("Iterator chain hint".to_string()),
                    });
                } else if trimmed.contains(".into_iter()") {
                    hints.push(InlayHint {
                        row,
                        col: line.len(),
                        label: " [IntoIter]".to_string(),
                        kind: InlayHintKind::Chaining,
                        tooltip: Some("IntoIterator chain hint".to_string()),
                    });
                }
            }
        }
    }

    hints
}

/// Computes CodeLens entries (e.g. reference counts, Run/Debug actions)
pub fn compute_code_lenses(lines: &[String], language: &str) -> Vec<CodeLens> {
    let mut lenses = Vec::new();
    let lang = language.to_lowercase();

    // Collect defined symbol identifiers and their declaration rows
    let mut symbol_rows: Vec<(usize, String, bool)> = Vec::new(); // (row, name, is_test_or_main)

    for (row, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if lang == "rust" || lang == "rs" {
            let is_test = row > 0 && lines[row - 1].trim().contains("#[test]");
            let is_main = trimmed.starts_with("fn main") || trimmed.starts_with("pub fn main");

            if let Some(name) = extract_declared_item(trimmed) {
                symbol_rows.push((row, name, is_test || is_main));
            }
        }
    }

    // Count intra-buffer references for each symbol
    let mut ref_counts: HashMap<String, usize> = HashMap::new();
    for line in lines {
        for (_, name, _) in &symbol_rows {
            // Count whole-word occurrences
            let count = count_word_occurrences(line, name);
            if count > 0 {
                *ref_counts.entry(name.clone()).or_default() += count;
            }
        }
    }

    for (row, name, is_actionable) in symbol_rows {
        // Subtract 1 for the declaration itself
        let total = ref_counts.get(&name).copied().unwrap_or(1);
        let refs = total.saturating_sub(1);
        let ref_label = if refs == 1 {
            "1 reference".to_string()
        } else {
            format!("{refs} references")
        };

        let full_label = if is_actionable {
            if name == "main" {
                format!("{ref_label} | ▶ Run | ⚙ Debug")
            } else {
                format!("{ref_label} | ▶ Run Test | ⚙ Debug")
            }
        } else {
            ref_label
        };

        lenses.push(CodeLens {
            row,
            label: full_label,
            command: if is_actionable {
                Some(format!("cargo test {name}"))
            } else {
                None
            },
        });
    }

    lenses
}

fn is_ident(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_alphanumeric() || c == '_')
}

fn infer_rust_type(val: &str) -> Option<&'static str> {
    if val == "true" || val == "false" {
        Some("bool")
    } else if val.starts_with('"') && val.ends_with('"') {
        Some("&str")
    } else if val.starts_with("String::new") || val.starts_with("String::from") {
        Some("String")
    } else if val.starts_with("Vec::new") || val.starts_with("vec![") {
        Some("Vec<_>")
    } else if val.starts_with("HashMap::new") {
        Some("HashMap<_, _>")
    } else if val.parse::<i64>().is_ok() {
        Some("i32")
    } else if val.parse::<f64>().is_ok() {
        Some("f64")
    } else if val.starts_with("PathBuf::from") {
        Some("PathBuf")
    } else {
        None
    }
}

fn extract_declared_item(line: &str) -> Option<String> {
    let prefixes = [
        "pub fn ",
        "fn ",
        "pub struct ",
        "struct ",
        "pub enum ",
        "enum ",
        "pub trait ",
        "trait ",
    ];
    for p in prefixes {
        if let Some(idx) = line.find(p) {
            let after = &line[idx + p.len()..];
            let name: String = after
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect();
            if !name.is_empty() {
                return Some(name);
            }
        }
    }
    None
}

fn count_word_occurrences(line: &str, word: &str) -> usize {
    if line.is_empty() || word.is_empty() {
        return 0;
    }
    let mut count = 0;
    let mut start = 0;
    while let Some(pos) = line[start..].find(word) {
        let actual_pos = start + pos;
        let before_ok =
            actual_pos == 0 || !line[..actual_pos].chars().last().unwrap().is_alphanumeric();
        let end_pos = actual_pos + word.len();
        let after_ok =
            end_pos >= line.len() || !line[end_pos..].chars().next().unwrap().is_alphanumeric();

        if before_ok && after_ok {
            count += 1;
        }
        start = actual_pos + word.len();
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inlay_hints_inference() {
        let lines = vec![
            "let x = 42;".to_string(),
            "let s = \"hello\";".to_string(),
            "let b = true;".to_string(),
            "let mut v = Vec::new();".to_string(),
            "let explicit: i32 = 10;".to_string(),
        ];

        let hints = compute_inlay_hints(&lines, "rust");
        assert_eq!(hints.len(), 4);
        assert_eq!(hints[0].label, ": i32");
        assert_eq!(hints[0].row, 0);
        assert_eq!(hints[1].label, ": &str");
        assert_eq!(hints[1].row, 1);
        assert_eq!(hints[2].label, ": bool");
        assert_eq!(hints[2].row, 2);
        assert_eq!(hints[3].label, ": Vec<_>");
        assert_eq!(hints[3].row, 3);
    }

    #[test]
    fn test_codelens_references_and_actions() {
        let lines = vec![
            "fn compute_sum(a: i32, b: i32) -> i32 {".to_string(),
            "    a + b".to_string(),
            "}".to_string(),
            "".to_string(),
            "fn main() {".to_string(),
            "    let res = compute_sum(1, 2);".to_string(),
            "    let res2 = compute_sum(3, 4);".to_string(),
            "}".to_string(),
        ];

        let lenses = compute_code_lenses(&lines, "rust");
        assert_eq!(lenses.len(), 2);
        // compute_sum has 2 references in main
        assert_eq!(lenses[0].row, 0);
        assert!(lenses[0].label.contains("2 references"));

        // main has Run action
        assert_eq!(lenses[1].row, 4);
        assert!(lenses[1].label.contains("Run"));
        assert!(lenses[1].label.contains("Debug"));
    }
}
