//! Language-Agnostic Structural Blueprints for Token Budget Optimization.
//!
//! Strips raw function/method/class bodies from source files, leaving only
//! type signatures, trait boundaries, interface contracts, and doc comments.
//! Produces an ultra-compact topological map of the codebase for the AI model's
//! context window — forcing crisp architectural awareness without raw code bloat.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tree_sitter::{Parser, Query, QueryCursor};
use streaming_iterator::StreamingIterator;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueprintEntry {
    pub file_path: String,
    pub language: String,
    pub signatures: Vec<SignatureBlock>,
    pub line_count: usize,
    pub byte_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureBlock {
    pub kind: String,
    pub name: String,
    pub signature: String,
    pub doc_comment: Option<String>,
    pub line_start: usize,
    pub line_end: usize,
}

pub struct StructuralBlueprints {
    root_path: PathBuf,
}

impl StructuralBlueprints {
    pub fn new(root_path: PathBuf) -> Self {
        Self { root_path }
    }

    /// Generate a structural blueprint for a single source file.
    /// Returns the compact signature-only representation.
    pub fn blueprint_file(&self, file_path: &Path, content: &str) -> Option<BlueprintEntry> {
        let ext = file_path
            .extension()
            .and_then(|e| e.to_str())?
            .to_lowercase();

        let language = match ext.as_str() {
            "rs" => "rust",
            "ts" | "tsx" | "mts" | "cts" => "typescript",
            "js" | "jsx" | "mjs" | "cjs" => "javascript",
            "py" | "pyi" => "python",
            _ => return None,
        };

        let signatures = match ext.as_str() {
            "rs" => extract_rust_signatures(content),
            "ts" | "tsx" | "mts" | "cts" => extract_typescript_signatures(content),
            "js" | "jsx" | "mjs" | "cjs" => extract_javascript_signatures(content),
            "py" | "pyi" => extract_python_signatures(content),
            _ => Vec::new(),
        };

        Some(BlueprintEntry {
            file_path: file_path
                .strip_prefix(&self.root_path)
                .unwrap_or(file_path)
                .to_string_lossy()
                .to_string(),
            language: language.to_string(),
            signatures,
            line_count: content.lines().count(),
            byte_size: content.len(),
        })
    }

    /// Generate blueprints for all indexable source files under a directory.
    pub fn blueprint_project(&self) -> Vec<BlueprintEntry> {
        let mut entries = Vec::new();
        let walker = walkdir::WalkDir::new(&self.root_path)
            .into_iter()
            .filter_entry(|e| {
                !e.file_type().is_dir()
                    || (!e.file_name().to_string_lossy().starts_with('.')
                        && e.file_name().to_string_lossy() != "node_modules"
                        && e.file_name().to_string_lossy() != "target"
                        && e.file_name().to_string_lossy() != "dist")
            })
            .filter_map(|e| e.ok());

        for entry in walker {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let ext = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();
            if !matches!(ext.as_str(), "rs" | "ts" | "tsx" | "js" | "jsx" | "py") {
                continue;
            }
            if let Ok(content) = std::fs::read_to_string(path) {
                if let Some(blueprint) = self.blueprint_file(path, &content) {
                    entries.push(blueprint);
                }
            }
        }
        entries
    }

    /// Serialize blueprints to a compact string for .aim context packing.
    /// Format: file_path:\n  kind name(signature)\n
    pub fn serialize_blueprints(entries: &[BlueprintEntry]) -> String {
        let mut output = String::with_capacity(entries.len() * 200);
        for entry in entries {
            output.push_str(&format!("// {} ({} lines, {} bytes)\n", entry.file_path, entry.line_count, entry.byte_size));
            for sig in &entry.signatures {
                if let Some(doc) = &sig.doc_comment {
                    let doc_trimmed: String = doc.lines().take(3).collect::<Vec<_>>().join(" ");
                    output.push_str(&format!("  /// {}\n", doc_trimmed));
                }
                output.push_str(&format!("  {} {}:{}\n", sig.kind, sig.name, sig.signature));
            }
            output.push('\n');
        }
        output
    }

    /// Compute total token budget saved vs. raw source code.
    pub fn budget_savings(entries: &[BlueprintEntry], raw_sizes: &HashMap<String, usize>) -> (usize, usize, f64) {
        let mut raw_total = 0usize;
        let mut blueprint_total = 0usize;
        for entry in entries {
            raw_total += raw_sizes.get(&entry.file_path).copied().unwrap_or(entry.byte_size);
            blueprint_total += Self::estimate_blueprint_size(entry);
        }
        let saved = raw_total.saturating_sub(blueprint_total);
        let ratio = if raw_total > 0 {
            saved as f64 / raw_total as f64
        } else {
            0.0
        };
        (raw_total, saved, ratio)
    }

    fn estimate_blueprint_size(entry: &BlueprintEntry) -> usize {
        entry.signatures.iter().map(|s| s.signature.len() + s.name.len() + 10).sum()
    }
}

// ── Rust signature extraction ────────────────────────────────────────────

fn extract_rust_signatures(content: &str) -> Vec<SignatureBlock> {
    let mut parser = Parser::new();
    if parser.set_language(&tree_sitter_rust::LANGUAGE.into()).is_err() {
        return Vec::new();
    }
    let tree = match parser.parse(content, None) {
        Some(t) => t,
        None => return Vec::new(),
    };

    let query_str = "\
        (function_item name: (identifier) @name parameters: (parameters) @params return_type: (type_identifier)? @ret) @kind_func
        (struct_item name: (type_identifier) @name) @kind_struct
        (enum_item name: (type_identifier) @name) @kind_enum
        (trait_item name: (type_identifier) @name) @kind_trait
        (impl_item type: (type_identifier) @name) @kind_impl
        (type_alias name: (type_identifier) @name) @kind_type
        (use_declaration) @kind_use";

    let query = match Query::new(&tree_sitter_rust::LANGUAGE.into(), query_str) {
        Ok(q) => q,
        Err(_) => return Vec::new(),
    };

    extract_signatures_from_tree(content, &tree, &query)
}

// ── TypeScript signature extraction ──────────────────────────────────────

fn extract_typescript_signatures(content: &str) -> Vec<SignatureBlock> {
    let mut parser = Parser::new();
    if parser.set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()).is_err() {
        return Vec::new();
    }
    let tree = match parser.parse(content, None) {
        Some(t) => t,
        None => return Vec::new(),
    };

    let query_str = "\
        (function_declaration name: (identifier) @name parameters: (formal_parameters) @params return_type: (type_annotation)? @ret) @kind_func
        (class_declaration name: (type_identifier) @name) @kind_class
        (interface_declaration name: (type_identifier) @name) @kind_interface
        (type_alias_declaration name: (type_identifier) @name) @kind_type
        (export_statement declaration: (function_declaration)) @kind_export";

    let query = match Query::new(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(), query_str) {
        Ok(q) => q,
        Err(_) => return Vec::new(),
    };

    extract_signatures_from_tree(content, &tree, &query)
}

// ── JavaScript signature extraction ──────────────────────────────────────

fn extract_javascript_signatures(content: &str) -> Vec<SignatureBlock> {
    let mut parser = Parser::new();
    if parser.set_language(&tree_sitter_typescript::LANGUAGE_TSX.into()).is_err() {
        return Vec::new();
    }
    let tree = match parser.parse(content, None) {
        Some(t) => t,
        None => return Vec::new(),
    };

    let query_str = "\
        (function_declaration name: (identifier) @name parameters: (formal_parameters) @params) @kind_func
        (class_declaration name: (identifier) @name) @kind_class
        (export_statement declaration: (function_declaration)) @kind_export";

    let query = match Query::new(&tree_sitter_typescript::LANGUAGE_TSX.into(), query_str) {
        Ok(q) => q,
        Err(_) => return Vec::new(),
    };

    extract_signatures_from_tree(content, &tree, &query)
}

// ── Python signature extraction ──────────────────────────────────────────

fn extract_python_signatures(content: &str) -> Vec<SignatureBlock> {
    let mut parser = Parser::new();
    if parser.set_language(&tree_sitter_python::LANGUAGE.into()).is_err() {
        return Vec::new();
    }
    let tree = match parser.parse(content, None) {
        Some(t) => t,
        None => return Vec::new(),
    };

    let query_str = "\
        (function_definition name: (identifier) @name parameters: (parameters) @params return_type: (type)? @ret) @kind_func
        (class_definition name: (identifier) @name) @kind_class";

    let query = match Query::new(&tree_sitter_python::LANGUAGE.into(), query_str) {
        Ok(q) => q,
        Err(_) => return Vec::new(),
    };

    extract_signatures_from_tree(content, &tree, &query)
}

// ── Generic extraction helper ────────────────────────────────────────────

fn extract_signatures_from_tree(
    content: &str,
    tree: &tree_sitter::Tree,
    query: &tree_sitter::Query,
) -> Vec<SignatureBlock> {
    let mut cursor = QueryCursor::new();
    let mut results = Vec::new();
    let mut matches = cursor.matches(query, tree.root_node(), content.as_bytes());
    let lines: Vec<&str> = content.lines().collect();

    while let Some(m) = StreamingIterator::next(&mut matches) {
        let mut name = String::new();
        let mut kind = "unknown".to_string();
        let mut node_start = 0;
        let mut node_end = 0;

        for cap in m.captures {
            let cap_name = query.capture_names()[cap.index as usize];
            if cap_name == "name" {
                if let Ok(n) = cap.node.utf8_text(content.as_bytes()) {
                    name = n.to_string();
                }
            } else if cap_name.starts_with("kind_") {
                kind = cap_name[5..].to_string();
                node_start = cap.node.start_position().row;
                node_end = cap.node.end_position().row;
            }
        }

        if name.is_empty() {
            continue;
        }

        // Extract just the first line of the node (the signature line)
        let signature_line = if node_start < lines.len() {
            lines[node_start].trim().to_string()
        } else {
            format!("{} {}", kind, name)
        };

        // Look for doc comments above the node
        let doc_comment = find_doc_comment(&lines, node_start);

        results.push(SignatureBlock {
            kind,
            name,
            signature: signature_line,
            doc_comment,
            line_start: node_start + 1,
            line_end: node_end + 1,
        });
    }

    results
}

fn find_doc_comment(lines: &[&str], target_line: usize) -> Option<String> {
    if target_line == 0 {
        return None;
    }

    let mut doc_lines = Vec::new();
    let mut i = target_line.saturating_sub(1);

    // Walk backwards looking for doc comments (/// or // or # or /** */)
    while i > 0 {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("///") || trimmed.starts_with("//!") {
            doc_lines.insert(0, trimmed.trim_start_matches("///").trim_start_matches("//!").trim().to_string());
            i -= 1;
        } else if trimmed.starts_with("//") {
            doc_lines.insert(0, trimmed.trim_start_matches("//").trim().to_string());
            i -= 1;
        } else if trimmed.starts_with("# ") {
            doc_lines.insert(0, trimmed.trim_start_matches("# ").trim().to_string());
            i -= 1;
        } else if trimmed.starts_with("/**") || trimmed.starts_with("/**") {
            doc_lines.insert(0, trimmed.trim_start_matches("/**").trim().to_string());
            i -= 1;
        } else if trimmed.starts_with("*") {
            doc_lines.insert(0, trimmed.trim_start_matches("*").trim().to_string());
            i -= 1;
        } else {
            break;
        }
    }

    if doc_lines.is_empty() {
        None
    } else {
        Some(doc_lines.join(" "))
    }
}
