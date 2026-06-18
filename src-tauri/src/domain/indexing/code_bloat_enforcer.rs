//! Tree-Sitter Code Bloat & Polymorphism Enforcer
//!
//! Analyzes proposed code changes via AST to detect redundant imperative patterns,
//! duplicate data shapes, and boilerplate that could be replaced with polymorphic
//! abstractions, generics, or native interface extensions.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tree_sitter::{Parser, Query, QueryCursor};
use streaming_iterator::StreamingIterator;

/// Maximum lines before structural analysis is triggered.
const BLOAT_THRESHOLD_LINES: usize = 100;

/// Maximum allowed lines for an accepted change.
const MAX_ACCEPTED_LINES: usize = 300;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BloatAnalysis {
    pub node_count: usize,
    pub line_count: usize,
    pub redundancy_score: f64,
    pub detected_patterns: Vec<String>,
    pub existing_symbols: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BloatVerdict {
    Clean,
    BloatDetected,
    NoLanguageSupport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BloatResult {
    pub verdict: BloatVerdict,
    pub analysis: Option<BloatAnalysis>,
    pub message: String,
    pub target_budget: usize,
}

pub struct CodeBloatEnforcer {
    /// Global symbol map: symbol_name -> (file_path, kind, line_range)
    symbol_index: HashMap<String, Vec<IndexedSymbol>>,
}

#[derive(Debug, Clone)]
pub struct IndexedSymbol {
    pub name: String,
    pub path: String,
    pub kind: String,
    pub line_range: (usize, usize),
}

impl CodeBloatEnforcer {
    pub fn new() -> Self {
        Self {
            symbol_index: HashMap::new(),
        }
    }

    /// Index a symbol from the context indexer's output.
    pub fn index_symbol(&mut self, name: String, path: String, kind: String, line_range: (usize, usize)) {
        self.symbol_index
            .entry(name.clone())
            .or_default()
            .push(IndexedSymbol {
                name,
                path,
                kind,
                line_range,
            });
    }

    /// Bulk-load symbols from the existing context indexer's symbol definitions.
    pub fn load_symbols(&mut self, symbols: &[crate::memory_store::SymbolDefinition]) {
        for sym in symbols {
            self.index_symbol(
                sym.name.clone(),
                sym.path.clone(),
                sym.kind.clone(),
                sym.line_range,
            );
        }
    }

    /// Analyze a proposed code delta (new code added by the agent) against the
    /// existing project symbol map. Returns a BloatResult.
    pub fn analyze_proposal(
        &self,
        proposed_content: &str,
        original_content: &str,
        file_ext: &str,
        file_path: &str,
    ) -> BloatResult {
        let line_count = proposed_content.lines().count();
        let original_line_count = original_content.lines().count();

        if line_count <= original_line_count + BLOAT_THRESHOLD_LINES {
            return BloatResult {
                verdict: BloatVerdict::Clean,
                analysis: None,
                message: "Change is within acceptable size threshold.".to_string(),
                target_budget: MAX_ACCEPTED_LINES,
            };
        }

        let delta_lines = line_count.saturating_sub(original_line_count);

        // Parse the proposed content with tree-sitter
        let analysis = match self.analyze_ast(proposed_content, file_ext, file_path, delta_lines) {
            Some(a) => a,
            None => {
                return BloatResult {
                    verdict: BloatVerdict::NoLanguageSupport,
                    analysis: None,
                    message: format!(
                        "No tree-sitter grammar available for .{} files. \
                         Manual review required for code bloat.",
                        file_ext
                    ),
                    target_budget: MAX_ACCEPTED_LINES,
                };
            }
        };

        if analysis.redundancy_score > 0.5 || delta_lines > MAX_ACCEPTED_LINES {
            let mut msg = format!(
                "CRITICAL FAILURE: Code bloat detected. Your solution uses {} \
                 lines (delta: {} lines, {} AST nodes) with a redundancy score of {:.1}. ",
                line_count,
                delta_lines,
                analysis.node_count,
                analysis.redundancy_score,
            );
            if !analysis.detected_patterns.is_empty() {
                msg.push_str("Detected patterns: ");
                for p in &analysis.detected_patterns {
                    msg.push_str(&format!("{}; ", p));
                }
                msg.push(' ');
            }
            if !analysis.existing_symbols.is_empty() {
                msg.push_str(
                    "Existing project symbols that could be reused: ",
                );
                for s in analysis.existing_symbols.iter().take(5) {
                    msg.push_str(&format!("{}; ", s));
                }
                msg.push(' ');
            }
            msg.push_str(
                "Re-engineer this logic into an elegant polymorphic abstraction, \
                 native interface extension, or generic utility function. \
                 Target budget constraint: Under 30 lines.",
            );
            BloatResult {
                verdict: BloatVerdict::BloatDetected,
                analysis: Some(analysis),
                message: msg,
                target_budget: 30,
            }
        } else {
            BloatResult {
                verdict: BloatVerdict::Clean,
                analysis: Some(analysis),
                message: "Change passes structural complexity analysis.".to_string(),
                target_budget: MAX_ACCEPTED_LINES,
            }
        }
    }

    fn analyze_ast(
        &self,
        content: &str,
        ext: &str,
        _file_path: &str,
        delta_lines: usize,
    ) -> Option<BloatAnalysis> {
        let mut parser = Parser::new();
        let language = match ext {
            "rs" => tree_sitter_rust::LANGUAGE,
            "ts" | "tsx" => tree_sitter_typescript::LANGUAGE_TYPESCRIPT,
            "js" | "jsx" => tree_sitter_typescript::LANGUAGE_TSX,
            "py" => tree_sitter_python::LANGUAGE,
            _ => return None,
        };

        parser.set_language(&language.into()).ok()?;
        let tree = parser.parse(content, None)?;

        let root = tree.root_node();
        let node_count = count_nodes(root);

        // Extract structural symbols from the proposed code
        let proposed_symbols = extract_structural_symbols(content, ext);
        let existing_symbols = self.find_redundancies(&proposed_symbols);

        // Compute redundancy score: how many proposed symbols already exist
        let redundancy_score = if proposed_symbols.is_empty() {
            0.0
        } else {
            existing_symbols.len() as f64 / proposed_symbols.len() as f64
        };

        // Detect specific bloat patterns
        let mut patterns = Vec::new();
        detect_loop_patterns(content, ext, &mut patterns);
        detect_duplicate_shapes(content, ext, &mut patterns);
        detect_boilerplate_wrappers(content, ext, &mut patterns);

        Some(BloatAnalysis {
            node_count,
            line_count: delta_lines,
            redundancy_score,
            detected_patterns: patterns,
            existing_symbols: existing_symbols.into_iter().take(10).collect(),
        })
    }

    fn find_redundancies(&self, proposed: &[ProposedSymbol]) -> Vec<String> {
        let mut redundancies = Vec::new();
        for prop in proposed {
            if let Some(existing) = self.symbol_index.get(&prop.name) {
                for sym in existing {
                    if sym.kind == prop.kind {
                        redundancies.push(format!(
                            "{} ({}, line {})",
                            prop.name, sym.path, sym.line_range.0
                        ));
                    }
                }
            }
        }
        redundancies
    }
}

struct ProposedSymbol {
    name: String,
    kind: String,
}

fn extract_structural_symbols(content: &str, ext: &str) -> Vec<ProposedSymbol> {
    let mut parser = Parser::new();
    let language = match ext {
        "rs" => tree_sitter_rust::LANGUAGE,
        "ts" | "tsx" => tree_sitter_typescript::LANGUAGE_TYPESCRIPT,
        "js" | "jsx" => tree_sitter_typescript::LANGUAGE_TSX,
        "py" => tree_sitter_python::LANGUAGE,
        _ => return Vec::new(),
    };

    if parser.set_language(&language.into()).is_err() {
        return Vec::new();
    }
        let tree = match parser.parse(content, None) {
            Some(t) => t,
            None => return Vec::new(),
        };

    let query_str = match ext {
        "rs" => "(function_item name: (identifier) @name) @kind_func
                 (struct_item name: (type_identifier) @name) @kind_struct
                 (enum_item name: (type_identifier) @name) @kind_enum
                 (trait_item name: (type_identifier) @name) @kind_trait
                 (impl_item type: (type_identifier) @name) @kind_impl",
        "ts" | "tsx" | "js" | "jsx" => "(function_declaration name: (identifier) @name) @kind_func
                                         (class_declaration name: (type_identifier) @name) @kind_class
                                         (interface_declaration name: (type_identifier) @name) @kind_interface",
        "py" => "(function_definition name: (identifier) @name) @kind_func
                 (class_definition name: (identifier) @name) @kind_class",
        _ => return Vec::new(),
    };

    if query_str.is_empty() {
        return Vec::new();
    }

    let query = match Query::new(&language.into(), query_str) {
        Ok(q) => q,
        Err(_) => return Vec::new(),
    };
    let mut cursor = QueryCursor::new();
    let mut symbols = Vec::new();
    let mut matches = cursor.matches(&query, tree.root_node(), content.as_bytes());
    while let Some(m) = StreamingIterator::next(&mut matches) {
        let mut name = String::new();
        let mut kind = "unknown".to_string();
        for cap in m.captures {
            let cap_name = query.capture_names()[cap.index as usize];
            if cap_name == "name" {
                if let Ok(n) = cap.node.utf8_text(content.as_bytes()) {
                    name = n.to_string();
                }
            } else if cap_name.starts_with("kind_") {
                kind = cap_name[5..].to_string();
            }
        }
        if !name.is_empty() {
            symbols.push(ProposedSymbol { name, kind });
        }
    }
    symbols
}

fn count_nodes(node: tree_sitter::Node) -> usize {
    let mut count = 1;
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        count += count_nodes(child);
    }
    count
}

/// Detect repetitive explicit matching loops (if/else chains that could be match or map).
fn detect_loop_patterns(content: &str, ext: &str, patterns: &mut Vec<String>) {
    match ext {
        "rs" => {
            // Detect chains of `if x == "..." { ... } else if` that could be `match`
            let if_count = content.matches("else if").count()
                + content.matches("} else if").count();
            if if_count >= 3 {
                patterns.push(format!(
                    "if/else-if chain with {} branches — consider using `match` or a lookup table",
                    if_count + 1
                ));
            }
        }
        "ts" | "tsx" | "js" | "jsx" => {
            let switch_count = content.matches("case ").count();
            let if_chain = content.matches("else if").count();
            if if_chain >= 3 {
                patterns.push(format!(
                    "if/else-if chain with {} branches — consider a `switch`, `Map`, or lookup object",
                    if_chain + 1
                ));
            }
            if switch_count >= 4 {
                patterns.push(format!(
                    "switch with {} cases — consider a Map/dictionary lookup pattern",
                    switch_count
                ));
            }
        }
        "py" => {
            let elif_count = content.matches("elif ").count();
            if elif_count >= 3 {
                patterns.push(format!(
                    "if/elif chain with {} branches — consider a dict dispatch or pattern matching",
                    elif_count + 1
                ));
            }
        }
        _ => {}
    }
}

/// Detect duplicate data shapes (structs/classes with same field names).
fn detect_duplicate_shapes(content: &str, _ext: &str, patterns: &mut Vec<String>) {
    // Simple heuristic: count struct definitions with more than 3 fields
    let struct_count = content.matches("pub struct ").count()
        + content.matches("struct ").count();
    if struct_count >= 3 {
        patterns.push(format!(
            "{} struct definitions — check for duplicate data shapes that could be unified",
            struct_count
        ));
    }
}

/// Detect boilerplate wrapper patterns (repetitive impl blocks or methods).
fn detect_boilerplate_wrappers(content: &str, ext: &str, patterns: &mut Vec<String>) {
    match ext {
        "rs" => {
            let impl_count = content.matches("impl ").count();
            if impl_count >= 5 {
                patterns.push(format!(
                    "{} impl blocks — consider consolidating with trait implementations or macros",
                    impl_count
                ));
            }
            let unwrap_count = content.matches(".unwrap()").count();
            if unwrap_count >= 5 {
                patterns.push(format!(
                    "{} `.unwrap()` calls — consider using `?` operator or `map_err`",
                    unwrap_count
                ));
            }
        }
        "ts" | "tsx" | "js" | "jsx" => {
            let try_count = content.matches("try {").count();
            let catch_count = content.matches("catch (").count();
            if try_count >= 3 || catch_count >= 3 {
                patterns.push(format!(
                    "{} try/catch blocks — consider a centralized error handler or Result type",
                    std::cmp::max(try_count, catch_count)
                ));
            }
        }
        _ => {}
    }
}
