use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::editor::engine::completion::{CompletionItem, CompletionState};
use crate::editor::engine::decorations::DiagnosticMarker;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HoverInfo {
    pub title: String,
    pub signature: String,
    pub docs: String,
    pub start_col: usize,
    pub end_col: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DefinitionLocation {
    pub file_path: PathBuf,
    pub row: usize,
    pub col: usize,
    pub preview: String,
}

#[derive(Clone, Debug)]
pub struct LanguageServiceRegistry {
    custom_snippets: HashMap<String, Vec<CompletionItem>>,
    diagnostics: HashMap<PathBuf, Vec<DiagnosticMarker>>,
    pub active_lsp: Option<std::sync::Arc<crate::editor::engine::lsp::NativeLspClient>>,
    pub active_ext_host: Option<
        std::sync::Arc<std::sync::Mutex<crate::editor::engine::vsx::ext_host::ExtHostBridge>>,
    >,
}

impl Default for LanguageServiceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageServiceRegistry {
    pub fn new() -> Self {
        Self {
            custom_snippets: HashMap::new(),
            diagnostics: HashMap::new(),
            active_lsp: None,
            active_ext_host: None,
        }
    }

    /// Register snippets contributed by an installed extension
    pub fn register_snippets(&mut self, language: &str, items: Vec<CompletionItem>) {
        let entry = self
            .custom_snippets
            .entry(language.to_lowercase())
            .or_default();
        for item in items {
            if !entry.iter().any(|existing| existing.label == item.label) {
                entry.push(item);
            }
        }
    }

    /// Retrieve completions (LSP + ext-host + built-in + contributed snippets) filtered by prefix
    pub fn get_completions(
        &self,
        language: &str,
        prefix: &str,
        path: Option<&Path>,
        row: usize,
        col: usize,
    ) -> Vec<CompletionItem> {
        let mut results = Vec::new();

        if let (Some(lsp), Some(p)) = (&self.active_lsp, path) {
            results.extend(lsp.request_completions(p, row, col));
        }

        if let (Some(ext_host), Some(p)) = (&self.active_ext_host, path) {
            if let Ok(bridge) = ext_host.lock() {
                results.extend(bridge.request_completions(p, language, "", row, col));
            }
        }

        if results.is_empty() {
            results = match language.to_lowercase().as_str() {
                "rust" | "rs" => CompletionState::default_rust_completions(),
                _ => Vec::new(),
            };
        }

        if let Some(snippets) = self.custom_snippets.get(&language.to_lowercase()) {
            results.extend(snippets.clone());
        }

        if !prefix.is_empty() {
            let lower_prefix = prefix.to_lowercase();
            results.retain(|item| item.label.to_lowercase().contains(&lower_prefix));
        }

        results
    }

    /// Provide hover information for a symbol at (row, col)
    pub fn provide_hover(
        &self,
        lines: &[String],
        language: &str,
        row: usize,
        col: usize,
        path: Option<&Path>,
    ) -> Option<HoverInfo> {
        // 1. Query active LSP if connected
        if let (Some(lsp), Some(p)) = (&self.active_lsp, path) {
            if let Some(hover) = lsp.request_hover(p, row, col) {
                return Some(hover);
            }
        }

        // 1b. Query active ext-host if connected
        if let (Some(ext_host), Some(p)) = (&self.active_ext_host, path) {
            if let Ok(bridge) = ext_host.lock() {
                let full_text = lines.join("\n");
                if let Some(hover) = bridge.request_hover(p, language, &full_text, row, col) {
                    return Some(hover);
                }
            }
        }

        let line = lines.get(row)?;
        let (word, start_col, end_col) = get_word_at(line, col)?;

        // 2. Keyword hover documentation
        if let Some((sig, docs)) = get_keyword_docs(language, word) {
            return Some(HoverInfo {
                title: format!("{language} keyword: {word}"),
                signature: sig.to_string(),
                docs: docs.to_string(),
                start_col,
                end_col,
            });
        }

        // 3. Intra-file declaration lookup & doc-comments
        if let Some((def_row, def_line)) = find_symbol_declaration(lines, language, word) {
            let docs = collect_preceding_doc_comments(lines, def_row);
            let sig = def_line.trim().to_string();
            return Some(HoverInfo {
                title: format!("symbol: {word}"),
                signature: sig,
                docs,
                start_col,
                end_col,
            });
        }

        // 4. Fallback generic hover
        Some(HoverInfo {
            title: format!("{language} identifier"),
            signature: format!("{word}: <inferred>"),
            docs: format!("Identifier `{word}` in {language} source buffer."),
            start_col,
            end_col,
        })
    }

    /// Go to Definition: find declaration location of symbol at (row, col)
    pub fn find_definition(
        &self,
        lines: &[String],
        current_path: &Path,
        row: usize,
        col: usize,
    ) -> Option<DefinitionLocation> {
        // 1. Query active LSP if connected
        if let Some(lsp) = &self.active_lsp {
            if let Some(def) = lsp.request_definition(current_path, row, col) {
                return Some(def);
            }
        }

        let language = current_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("rust");

        // 1b. Query active ext-host if connected
        if let Some(ext_host) = &self.active_ext_host {
            if let Ok(bridge) = ext_host.lock() {
                let full_text = lines.join("\n");
                if let Some(def) =
                    bridge.request_definition(current_path, language, &full_text, row, col)
                {
                    return Some(def);
                }
            }
        }

        let line = lines.get(row)?;
        let (word, _, _) = get_word_at(line, col)?;

        if let Some((def_row, def_line)) = find_symbol_declaration(lines, language, word) {
            let col = def_line.find(word).unwrap_or(0);
            return Some(DefinitionLocation {
                file_path: current_path.to_path_buf(),
                row: def_row,
                col,
                preview: def_line.trim().to_string(),
            });
        }

        None
    }

    /// Set diagnostic markers for a file
    pub fn set_diagnostics(&mut self, path: PathBuf, diags: Vec<DiagnosticMarker>) {
        self.diagnostics.insert(path, diags);
    }

    /// Get diagnostic markers for a file (merges LSP, ext-host, and manual diagnostics)
    pub fn get_diagnostics(&self, path: &Path) -> Vec<DiagnosticMarker> {
        let mut diags = Vec::new();
        if let Some(lsp) = &self.active_lsp {
            diags.extend(lsp.get_diagnostics(path));
        }
        if let Some(ext_host) = &self.active_ext_host {
            if let Ok(bridge) = ext_host.lock() {
                if let Ok(map) = bridge.diagnostics.read() {
                    if let Some(ext_diags) = map.get(path) {
                        diags.extend(ext_diags.clone());
                    }
                }
            }
        }
        if let Some(custom) = self.diagnostics.get(path) {
            diags.extend(custom.clone());
        }
        diags
    }

    /// Retrieve all diagnostics across all known files from LSP, ext-host, and manual contributions
    pub fn get_all_diagnostics(&self) -> HashMap<PathBuf, Vec<DiagnosticMarker>> {
        let mut map = HashMap::new();
        if let Some(lsp) = &self.active_lsp {
            map.extend(lsp.get_all_diagnostics());
        }
        if let Some(ext_host) = &self.active_ext_host {
            if let Ok(bridge) = ext_host.lock() {
                if let Ok(guard) = bridge.diagnostics.read() {
                    for (k, v) in guard.iter() {
                        map.entry(k.clone())
                            .or_insert_with(Vec::new)
                            .extend(v.clone());
                    }
                }
            }
        }
        for (k, v) in &self.diagnostics {
            map.entry(k.clone())
                .or_insert_with(Vec::new)
                .extend(v.clone());
        }
        map
    }

    /// Format document via active LSP, falling back to CLI toolchain (e.g. rustfmt)
    pub fn format_document(
        &self,
        path: &Path,
        content: &str,
        tab_size: usize,
        insert_spaces: bool,
    ) -> Option<Vec<crate::editor::engine::lsp::LspTextEdit>> {
        // 1. Try active LSP
        if let Some(lsp) = &self.active_lsp {
            if let Ok(edits) = lsp.request_formatting(path, tab_size, insert_spaces) {
                if !edits.is_empty() {
                    return Some(edits);
                }
            }
        }

        // 2. Fallback to CLI formatters (e.g. rustfmt for Rust)
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if ext == "rs" {
            if let Ok(mut child) = std::process::Command::new("rustfmt")
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::null())
                .spawn()
            {
                if let Some(mut stdin) = child.stdin.take() {
                    let _ = std::io::Write::write_all(&mut stdin, content.as_bytes());
                }
                if let Ok(output) = child.wait_with_output() {
                    if output.status.success() {
                        let formatted = String::from_utf8_lossy(&output.stdout).to_string();
                        if formatted != content {
                            let line_count = content.lines().count();
                            let last_line_len =
                                content.lines().last().map(|l| l.len()).unwrap_or(0);
                            return Some(vec![crate::editor::engine::lsp::LspTextEdit {
                                range: crate::editor::engine::lsp::LspRange {
                                    start: crate::editor::engine::lsp::LspPosition {
                                        line: 0,
                                        character: 0,
                                    },
                                    end: crate::editor::engine::lsp::LspPosition {
                                        line: line_count,
                                        character: last_line_len,
                                    },
                                },
                                new_text: formatted,
                            }]);
                        }
                    }
                }
            }
        }

        None
    }

    /// Retrieve code actions / quick fixes for active range
    pub fn get_code_actions(
        &self,
        path: &Path,
        start_row: usize,
        start_col: usize,
        end_row: usize,
        end_col: usize,
    ) -> Vec<crate::editor::engine::lsp::LspCodeAction> {
        if let Some(lsp) = &self.active_lsp {
            lsp.request_code_actions(path, start_row, start_col, end_row, end_col)
        } else {
            Vec::new()
        }
    }

    /// Rename symbol at (row, col) across workspace
    pub fn rename_symbol(
        &self,
        path: &Path,
        row: usize,
        col: usize,
        new_name: &str,
    ) -> Option<crate::editor::engine::lsp::WorkspaceEdit> {
        if let Some(lsp) = &self.active_lsp {
            lsp.request_rename(path, row, col, new_name)
        } else {
            None
        }
    }

    /// Find all references to symbol at (row, col), falling back to intra-file text search
    pub fn find_references(
        &self,
        lines: &[String],
        path: &Path,
        row: usize,
        col: usize,
    ) -> Vec<DefinitionLocation> {
        if let Some(lsp) = &self.active_lsp {
            let refs = lsp.request_references(path, row, col, true);
            if !refs.is_empty() {
                return refs;
            }
        }

        // Fallback: intra-file symbol occurrence search
        let line = match lines.get(row) {
            Some(l) => l,
            None => return Vec::new(),
        };

        let (word, _, _) = match get_word_at(line, col) {
            Some(w) => w,
            None => return Vec::new(),
        };

        let mut results = Vec::new();
        for (idx, l) in lines.iter().enumerate() {
            let mut search_start = 0;
            while let Some(found) = l[search_start..].find(word) {
                let actual_col = search_start + found;
                let before_ok = actual_col == 0
                    || !l[..actual_col]
                        .chars()
                        .last()
                        .map(|c| c.is_alphanumeric() || c == '_')
                        .unwrap_or(false);
                let after_pos = actual_col + word.len();
                let after_ok = after_pos >= l.len()
                    || !l[after_pos..]
                        .chars()
                        .next()
                        .map(|c| c.is_alphanumeric() || c == '_')
                        .unwrap_or(false);

                if before_ok && after_ok {
                    results.push(DefinitionLocation {
                        file_path: path.to_path_buf(),
                        row: idx,
                        col: actual_col,
                        preview: l.trim().to_string(),
                    });
                }
                search_start = actual_col + word.len();
            }
        }

        results
    }

    /// Retrieve document symbols / outline, falling back to regex declaration extractor
    pub fn get_document_symbols(
        &self,
        lines: &[String],
        path: &Path,
    ) -> Vec<crate::editor::engine::lsp::DocumentSymbol> {
        if let Some(lsp) = &self.active_lsp {
            let symbols = lsp.request_document_symbols(path);
            if !symbols.is_empty() {
                return symbols;
            }
        }

        // Fallback: scan lines for declaration keywords
        let mut symbols = Vec::new();
        for (idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            let (kind, prefix) = if trimmed.starts_with("fn ")
                || trimmed.starts_with("pub fn ")
                || trimmed.starts_with("async fn ")
                || trimmed.starts_with("pub async fn ")
            {
                (crate::editor::engine::lsp::SymbolKind::Function, "fn ")
            } else if trimmed.starts_with("struct ") || trimmed.starts_with("pub struct ") {
                (crate::editor::engine::lsp::SymbolKind::Struct, "struct ")
            } else if trimmed.starts_with("enum ") || trimmed.starts_with("pub enum ") {
                (crate::editor::engine::lsp::SymbolKind::Enum, "enum ")
            } else if trimmed.starts_with("trait ") || trimmed.starts_with("pub trait ") {
                (crate::editor::engine::lsp::SymbolKind::Interface, "trait ")
            } else if trimmed.starts_with("impl ") {
                (crate::editor::engine::lsp::SymbolKind::Class, "impl ")
            } else if trimmed.starts_with("const ") || trimmed.starts_with("pub const ") {
                (crate::editor::engine::lsp::SymbolKind::Constant, "const ")
            } else if trimmed.starts_with("mod ") || trimmed.starts_with("pub mod ") {
                (crate::editor::engine::lsp::SymbolKind::Module, "mod ")
            } else if trimmed.starts_with("def ") {
                (crate::editor::engine::lsp::SymbolKind::Function, "def ")
            } else if trimmed.starts_with("class ") {
                (crate::editor::engine::lsp::SymbolKind::Class, "class ")
            } else {
                continue;
            };

            if let Some(pos) = trimmed.find(prefix) {
                let rest = &trimmed[pos + prefix.len()..];
                let name: String = rest
                    .chars()
                    .take_while(|c| c.is_alphanumeric() || *c == '_')
                    .collect();
                if !name.is_empty() {
                    let col = line.find(&name).unwrap_or(0);
                    let range = crate::editor::engine::lsp::LspRange {
                        start: crate::editor::engine::lsp::LspPosition {
                            line: idx,
                            character: col,
                        },
                        end: crate::editor::engine::lsp::LspPosition {
                            line: idx,
                            character: col + name.len(),
                        },
                    };
                    symbols.push(crate::editor::engine::lsp::DocumentSymbol {
                        name,
                        detail: Some(trimmed.to_string()),
                        kind,
                        range: range.clone(),
                        selection_range: range,
                        children: Vec::new(),
                    });
                }
            }
        }

        symbols
    }
}

/// Extract word under col index in line
pub fn get_word_at(line: &str, col: usize) -> Option<(&str, usize, usize)> {
    if line.is_empty() {
        return None;
    }
    let chars: Vec<(usize, char)> = line.char_indices().collect();
    if chars.is_empty() {
        return None;
    }
    let idx = col.min(chars.len().saturating_sub(1));
    let is_ident_char = |c: char| c.is_alphanumeric() || c == '_';

    if !is_ident_char(chars[idx].1) {
        return None;
    }

    let mut start = idx;
    while start > 0 && is_ident_char(chars[start - 1].1) {
        start -= 1;
    }
    let mut end = idx;
    while end + 1 < chars.len() && is_ident_char(chars[end + 1].1) {
        end += 1;
    }

    let start_byte = chars[start].0;
    let end_byte = if end + 1 < chars.len() {
        chars[end + 1].0
    } else {
        line.len()
    };

    Some((&line[start_byte..end_byte], start, end + 1))
}

fn get_keyword_docs(language: &str, word: &str) -> Option<(&'static str, &'static str)> {
    match language.to_lowercase().as_str() {
        "rust" | "rs" => match word {
            "fn" => Some((
                "fn name(...) -> ReturnType",
                "Defines a function or method in Rust.",
            )),
            "pub" => Some((
                "pub <item>",
                "Makes an item visible outside its declaring module.",
            )),
            "struct" => Some((
                "struct Name { ... }",
                "Declares a custom nominal struct type.",
            )),
            "enum" => Some((
                "enum Name { ... }",
                "Declares an enumerated type with variants.",
            )),
            "impl" => Some((
                "impl Type { ... }",
                "Implements methods or traits for a type.",
            )),
            "trait" => Some((
                "trait Name { ... }",
                "Defines a shared interface/behavior contract.",
            )),
            "async" => Some((
                "async fn / async block",
                "Constructs a future for asynchronous evaluation.",
            )),
            "await" => Some((".await", "Suspends execution until the future is resolved.")),
            "match" => Some((
                "match value { ... }",
                "Pattern-matches a value exhaustively.",
            )),
            "mut" => Some(("mut <ident>", "Marks a binding or reference as mutable.")),
            "let" => Some((
                "let [mut] <ident> = <expr>;",
                "Binds a value to an identifier.",
            )),
            "const" => Some((
                "const NAME: Type = ...;",
                "Declares a compile-time constant.",
            )),
            "use" => Some((
                "use path::item;",
                "Binds symbols from other modules into scope.",
            )),
            "mod" => Some(("mod name;", "Declares a submodule.")),
            _ => None,
        },
        "typescript" | "javascript" | "ts" | "js" => match word {
            "function" => Some((
                "function name(...) { ... }",
                "Declares a JavaScript/TypeScript function.",
            )),
            "class" => Some(("class Name { ... }", "Declares an ES6/TypeScript class.")),
            "interface" => Some((
                "interface Name { ... }",
                "Defines a TypeScript object interface.",
            )),
            "type" => Some(("type Name = ...;", "Declares a TypeScript type alias.")),
            "const" => Some((
                "const name = ...;",
                "Declares a block-scoped, immutable variable binding.",
            )),
            "let" => Some((
                "let name = ...;",
                "Declares a block-scoped mutable variable.",
            )),
            "export" => Some((
                "export <declaration>;",
                "Exports functions, objects, or values from a module.",
            )),
            "import" => Some(("import { ... } from '...';", "Imports module bindings.")),
            "async" => Some((
                "async function",
                "Declares an asynchronous function returning a Promise.",
            )),
            "await" => Some((
                "await <promise>",
                "Pauses async function execution until Promise settles.",
            )),
            _ => None,
        },
        "python" | "py" => match word {
            "def" => Some(("def name(...):", "Defines a Python function or method.")),
            "class" => Some(("class Name(Base):", "Defines a Python class.")),
            "import" => Some(("import module", "Imports a module or package.")),
            "from" => Some((
                "from module import item",
                "Imports specific items from a module.",
            )),
            "return" => Some(("return [value]", "Exits function and returns value.")),
            "yield" => Some(("yield [value]", "Yields value from a generator function.")),
            _ => None,
        },
        _ => None,
    }
}

fn find_symbol_declaration<'a>(
    lines: &'a [String],
    language: &'a str,
    word: &str,
) -> Option<(usize, &'a String)> {
    let patterns = match language.to_lowercase().as_str() {
        "rust" | "rs" => vec![
            format!("fn {word}("),
            format!("fn {word} "),
            format!("struct {word} "),
            format!("struct {word}{{"),
            format!("struct {word}<"),
            format!("enum {word} "),
            format!("enum {word}{{"),
            format!("trait {word} "),
            format!("trait {word}<"),
            format!("type {word} "),
            format!("type {word}="),
            format!("const {word}:"),
            format!("const {word} :"),
            format!("let mut {word}"),
            format!("let {word}"),
        ],
        "typescript" | "javascript" | "ts" | "js" => vec![
            format!("function {word}("),
            format!("function {word} "),
            format!("class {word} "),
            format!("interface {word} "),
            format!("type {word} ="),
            format!("const {word} ="),
            format!("let {word} ="),
        ],
        "python" | "py" => vec![
            format!("def {word}("),
            format!("def {word} "),
            format!("class {word}:"),
            format!("class {word}("),
        ],
        _ => vec![
            format!("fn {word}"),
            format!("def {word}"),
            format!("function {word}"),
            format!("class {word}"),
        ],
    };

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        for pat in &patterns {
            if trimmed.contains(pat) {
                return Some((idx, line));
            }
        }
    }

    None
}

fn collect_preceding_doc_comments(lines: &[String], def_row: usize) -> String {
    let mut docs = Vec::new();
    let mut cur = def_row;

    while cur > 0 {
        cur -= 1;
        let line = lines[cur].trim();
        if line.starts_with("///") || line.starts_with("//!") {
            docs.push(
                line.trim_start_matches("///")
                    .trim_start_matches("//!")
                    .trim()
                    .to_string(),
            );
        } else if line.starts_with("//") {
            docs.push(line.trim_start_matches("//").trim().to_string());
        } else if line.is_empty() {
            continue;
        } else {
            break;
        }
    }

    docs.reverse();
    if docs.is_empty() {
        "No documentation comment provided.".to_string()
    } else {
        docs.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_word_at_basic() {
        let line = "fn hello_world(x: u32) {";
        let (word, start, end) = get_word_at(line, 3).unwrap();
        assert_eq!(word, "hello_world");
        assert_eq!(start, 3);
        assert_eq!(end, 14);
    }

    #[test]
    fn test_get_word_at_beginning() {
        let line = "fn main() {";
        let (word, _, _) = get_word_at(line, 0).unwrap();
        assert_eq!(word, "fn");
    }

    #[test]
    fn test_get_word_at_non_ident() {
        let line = "  (  )  ";
        assert!(get_word_at(line, 2).is_none());
    }

    #[test]
    fn test_keyword_hover_rust() {
        let registry = LanguageServiceRegistry::new();
        let lines = vec![
            "fn main() {".to_string(),
            "    let x = 42;".to_string(),
            "}".to_string(),
        ];
        let hover = registry.provide_hover(&lines, "rust", 0, 0, None).unwrap();
        assert!(hover.title.contains("keyword"));
        assert!(hover.signature.contains("fn"));
    }

    #[test]
    fn test_symbol_definition_lookup() {
        let registry = LanguageServiceRegistry::new();
        let lines = vec![
            "/// Adds two numbers".to_string(),
            "fn add(a: i32, b: i32) -> i32 {".to_string(),
            "    a + b".to_string(),
            "}".to_string(),
            "".to_string(),
            "fn main() {".to_string(),
            "    let result = add(1, 2);".to_string(),
            "}".to_string(),
        ];
        let path = PathBuf::from("test.rs");
        let def = registry.find_definition(&lines, &path, 6, 17).unwrap();
        assert_eq!(def.row, 1);
        assert!(def.preview.contains("fn add"));
    }

    #[test]
    fn test_hover_with_doc_comment() {
        let registry = LanguageServiceRegistry::new();
        let lines = vec![
            "/// Creates a new widget".to_string(),
            "fn create_widget() -> Widget {".to_string(),
            "    Widget::new()".to_string(),
            "}".to_string(),
            "".to_string(),
            "let w = create_widget();".to_string(),
        ];
        let hover = registry.provide_hover(&lines, "rust", 5, 10, None).unwrap();
        assert!(hover.docs.contains("Creates a new widget"));
    }

    #[test]
    fn test_register_and_get_completions() {
        let mut registry = LanguageServiceRegistry::new();
        let items = vec![CompletionItem {
            label: "mySnippet".to_string(),
            kind: crate::editor::engine::completion::CompletionKind::Snippet,
            detail: "Custom snippet".to_string(),
            insert_text: "mySnippet()".to_string(),
        }];
        registry.register_snippets("rust", items);

        let results = registry.get_completions("rust", "myS", None, 0, 0);
        assert!(results.iter().any(|c| c.label == "mySnippet"));
    }

    #[test]
    fn test_completions_empty_prefix() {
        let registry = LanguageServiceRegistry::new();
        let results = registry.get_completions("rust", "", None, 0, 0);
        // Should return all built-in Rust completions
        assert!(!results.is_empty());
    }

    #[test]
    fn test_registry_diagnostics_merge() {
        let mut registry = LanguageServiceRegistry::new();
        let path = PathBuf::from("main.rs");
        let diags = vec![DiagnosticMarker {
            row: 10,
            start_col: 0,
            end_col: 5,
            message: "Unused variable".to_string(),
            severity: crate::editor::engine::decorations::DiagnosticSeverity::Warning,
        }];
        registry.set_diagnostics(path.clone(), diags);
        let retrieved = registry.get_diagnostics(&path);
        assert_eq!(retrieved.len(), 1);
        assert_eq!(retrieved[0].message, "Unused variable");
    }

    #[test]
    fn test_find_references() {
        let registry = LanguageServiceRegistry::new();
        let lines = vec![
            "fn calculate_total() -> u64 {".to_string(),
            "    42".to_string(),
            "}".to_string(),
            "".to_string(),
            "let a = calculate_total();".to_string(),
            "let b = calculate_total() + 10;".to_string(),
        ];
        let path = PathBuf::from("src/calc.rs");
        let refs = registry.find_references(&lines, &path, 0, 8);
        assert_eq!(refs.len(), 3);
        assert_eq!(refs[0].row, 0);
        assert_eq!(refs[1].row, 4);
        assert_eq!(refs[2].row, 5);
        assert!(refs[1].preview.contains("calculate_total()"));
    }

    #[test]
    fn test_document_symbols() {
        let registry = LanguageServiceRegistry::new();
        let lines = vec![
            "pub struct AppConfig {".to_string(),
            "    pub port: u16,".to_string(),
            "}".to_string(),
            "".to_string(),
            "pub fn start_server() {".to_string(),
            "}".to_string(),
            "".to_string(),
            "pub enum Status {".to_string(),
            "    Active,".to_string(),
            "}".to_string(),
        ];
        let path = PathBuf::from("src/server.rs");
        let symbols = registry.get_document_symbols(&lines, &path);
        assert_eq!(symbols.len(), 3);
        assert_eq!(symbols[0].name, "AppConfig");
        assert_eq!(
            symbols[0].kind,
            crate::editor::engine::lsp::SymbolKind::Struct
        );
        assert_eq!(symbols[1].name, "start_server");
        assert_eq!(
            symbols[1].kind,
            crate::editor::engine::lsp::SymbolKind::Function
        );
        assert_eq!(symbols[2].name, "Status");
        assert_eq!(
            symbols[2].kind,
            crate::editor::engine::lsp::SymbolKind::Enum
        );
    }
}
