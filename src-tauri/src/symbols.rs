use regex::Regex;
use serde_json::{json, Value};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct FileSymbolEntry {
    pub name: String,
    pub symbol_type: String,
    pub line: u32,
}

pub fn analyze_path(path: &Path) -> Result<Vec<FileSymbolEntry>, String> {
    let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let extension = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();
    Ok(parse_symbols(&content, &extension))
}

pub fn analyze_to_json(path: &Path) -> Result<Value, String> {
    let symbols = analyze_path(path)?;
    let path_str = path.to_string_lossy().to_string();
    let extension = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string();
    Ok(json!({
        "path": path_str,
        "extension": extension,
        "symbols_count": symbols.len(),
        "symbols": symbols.iter().map(|s| json!({
            "type": s.symbol_type,
            "name": s.name,
            "line": s.line,
        })).collect::<Vec<_>>()
    }))
}

fn line_number(content: &str, byte_offset: usize) -> u32 {
    content[..byte_offset.min(content.len())]
        .bytes()
        .filter(|b| *b == b'\n')
        .count() as u32
        + 1
}

fn push_match(symbols: &mut Vec<FileSymbolEntry>, content: &str, cap: &regex::Captures<'_>, kind: &str) {
    if let Some(m) = cap.get(1) {
        symbols.push(FileSymbolEntry {
            name: m.as_str().to_string(),
            symbol_type: kind.to_string(),
            line: line_number(content, m.start()),
        });
    }
}

pub fn parse_symbols(content: &str, extension: &str) -> Vec<FileSymbolEntry> {
    let mut symbols = Vec::new();

    match extension {
        "rs" => {
            let patterns: &[(&str, &str)] = &[
                (r"(?m)^\s*(?:pub\s+)?(?:async\s+)?fn\s+([a-zA-Z_][a-zA-Z0-9_]*)", "function"),
                (r"(?m)^\s*(?:pub\s+)?struct\s+([a-zA-Z_][a-zA-Z0-9_]*)", "struct"),
                (r"(?m)^\s*(?:pub\s+)?enum\s+([a-zA-Z_][a-zA-Z0-9_]*)", "enum"),
                (r"(?m)^\s*(?:pub\s+)?trait\s+([a-zA-Z_][a-zA-Z0-9_]*)", "trait"),
                (r"(?m)^\s*impl(?:\s+<[^>]+>)?\s+([a-zA-Z_][a-zA-Z0-9_]*)", "impl"),
            ];
            for (pat, kind) in patterns {
                if let Ok(re) = Regex::new(pat) {
                    for cap in re.captures_iter(content) {
                        push_match(&mut symbols, content, &cap, kind);
                    }
                }
            }
        }
        "ts" | "tsx" | "js" | "jsx" | "mjs" => {
            let patterns: &[(&str, &str)] = &[
                (r"(?m)^\s*(?:export\s+)?(?:async\s+)?function\s+([a-zA-Z_][a-zA-Z0-9_]*)", "function"),
                (r"(?m)^\s*(?:export\s+)?class\s+([a-zA-Z_][a-zA-Z0-9_]*)", "class"),
                (r"(?m)^\s*(?:export\s+)?interface\s+([a-zA-Z_][a-zA-Z0-9_]*)", "interface"),
                (r"(?m)^\s*(?:export\s+)?type\s+([a-zA-Z_][a-zA-Z0-9_]*)", "type"),
                (r"(?m)^\s*(?:export\s+)?const\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*=\s*(?:\(|async)", "function"),
            ];
            for (pat, kind) in patterns {
                if let Ok(re) = Regex::new(pat) {
                    for cap in re.captures_iter(content) {
                        push_match(&mut symbols, content, &cap, kind);
                    }
                }
            }
        }
        "py" => {
            if let Ok(def_re) = Regex::new(r"(?m)^\s*def\s+([a-zA-Z_][a-zA-Z0-9_]*)") {
                for cap in def_re.captures_iter(content) {
                    push_match(&mut symbols, content, &cap, "function");
                }
            }
            if let Ok(class_re) = Regex::new(r"(?m)^\s*class\s+([a-zA-Z_][a-zA-Z0-9_]*)") {
                for cap in class_re.captures_iter(content) {
                    push_match(&mut symbols, content, &cap, "class");
                }
            }
        }
        "go" => {
            if let Ok(fn_re) = Regex::new(r"(?m)^func\s+(?:\([^)]+\)\s+)?([a-zA-Z_][a-zA-Z0-9_]*)") {
                for cap in fn_re.captures_iter(content) {
                    push_match(&mut symbols, content, &cap, "function");
                }
            }
        }
        _ => {}
    }

    symbols.sort_by_key(|s| s.line);
    symbols
}
