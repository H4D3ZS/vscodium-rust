//! Indexing domain: semantic/vector indexing, codebase context indexing,
//! embeddings, ANN search, ripgrep integration, and symbol extraction.

pub mod ann_index;
pub mod code_bloat_enforcer;
pub mod codebase_map;
pub mod context_indexer;
pub mod embeddings;
pub mod knowledge_distiller;
pub mod ripgrep_search;
pub mod structural_blueprints;
pub mod symbols;
pub mod vector_indexer;
