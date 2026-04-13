use crate::memory_store::{MemoryStore, SemanticSlot};
use serde_json::json;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use tokio::time::{sleep, Duration};
use walkdir::WalkDir;
use tree_sitter::{Parser, Query, QueryCursor};
use streaming_iterator::StreamingIterator;
use notify::{Watcher, RecursiveMode, RecommendedWatcher, Event, EventKind};
use tokio::sync::mpsc;
use rayon::prelude::*;
use sha2::{Sha256, Digest};

pub struct ContextIndexer {
    memory_store: Arc<MemoryStore>,
    root_path: PathBuf,
    hashes: Arc<RwLock<HashMap<PathBuf, String>>>,
}

impl ContextIndexer {
    pub fn new(memory_store: Arc<MemoryStore>, root_path: PathBuf) -> Self {
        Self {
            memory_store,
            root_path,
            hashes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn start_background_indexing(&self) {
        let ms = self.memory_store.clone();
        let root = self.root_path.clone();

        // Start real-time incremental indexing
        let (tx, mut rx) = mpsc::channel(100);
        let mut watcher = RecommendedWatcher::new(move |res: notify::Result<Event>| {
            if let Ok(event) = res {
                let _ = tx.blocking_send(event);
            }
        }, notify::Config::default()).expect("Failed to create watcher");

        watcher.watch(&root, RecursiveMode::Recursive).expect("Failed to start watching");

        tauri::async_runtime::spawn(async move {
            println!("[CONTEXT] Starting incremental indexing loop for: {:?}", root);
            // Keep watcher alive in this thread
            let _watcher = watcher;
            
            while let Some(event) = rx.recv().await {
                match event.kind {
                    EventKind::Modify(_) | EventKind::Create(_) => {
                        for path in event.paths {
                            if Self::is_indexable(&path) {
                                if let Err(e) = Self::index_single_file(&ms, &root, &path).await {
                                    eprintln!("[CONTEXT] Error indexing file {:?}: {:?}", path, e);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        });

        // Also run a full cycle periodically to ensure consistency
        let ms_full = self.memory_store.clone();
        let root_full = self.root_path.clone();
        let hashes_full = self.hashes.clone();
        tauri::async_runtime::spawn(async move {
            loop {
                if let Err(e) = Self::run_index_cycle(&ms_full, &root_full, hashes_full.clone()).await {
                    eprintln!("[CONTEXT] Periodic full indexing error: {:?}", e);
                }
                sleep(Duration::from_secs(3600)).await; // Full sync every hour
            }
        });
    }

    pub async fn trigger_index_cycle(&self) -> anyhow::Result<()> {
        Self::run_index_cycle(&self.memory_store, &self.root_path, self.hashes.clone()).await
    }

    async fn run_index_cycle(ms: &MemoryStore, root: &Path, hashes: Arc<RwLock<HashMap<PathBuf, String>>>) -> anyhow::Result<()> {
        if !root.join(".aim").exists() {
            println!("[CONTEXT] Project is dormant (No .aim detected). Skipping index cycle.");
            return Ok(());
        }

        println!("[CONTEXT] Starting parallel index cycle for: {:?}", root);

        // 1. Collect all indexable paths first
        let paths: Vec<PathBuf> = WalkDir::new(root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| Self::is_indexable(e.path()))
            .map(|e| e.path().to_path_buf())
            .collect();

        println!("[CONTEXT] Found {} indexable files. Dispatching to Rayon threadpool.", paths.len());

        // 2. Process in parallel using Rayon
        paths.par_iter().for_each(|path: &PathBuf| {
            let relative_path = path.strip_prefix(root).unwrap_or(path).to_string_lossy().to_string();
            let content_res = std::fs::read_to_string(path);
            
            if let Ok(content) = content_res {
                // Hashing Check
                let current_hash = Self::compute_hash(&content);
                {
                    let h_lock = hashes.read().unwrap();
                    if h_lock.get(path) == Some(&current_hash) {
                        return; // Skip unchanged file
                    }
                }
                
                // Update Hash
                {
                    let mut h_lock = hashes.write().unwrap();
                    h_lock.insert(path.clone(), current_hash);
                }

                let extension = path.extension().and_then(|e: &std::ffi::OsStr| e.to_str()).unwrap_or("");
                let category = if extension == "md" { "fix_lessons" } else { "file_map" };
                let mut tags = Vec::new();

                // 2.1 Calculate Pythagorean Embedding (Geometric Logic)
                let embedder = hades_harness::PythagoreanEmbedder::new(1536);
                let geometry = embedder.embed_path(Path::new(&relative_path));

                if extension == "rs" || extension == "ts" || extension == "tsx" {
                    let symbols = Self::extract_symbols_sync(&content, extension, &relative_path);
                    for sym in symbols {
                        tags.push(format!("symbol:{}", sym.name));
                        futures::executor::block_on(ms.store_symbol(sym));
                    }
                }

                // 2.2 Security Analysis (Distillation)
                let security_meta = crate::security_distiller::SecurityDistiller::get_security_metadata(&content);

                futures::executor::block_on(ms.store_slot(crate::memory_store::SemanticSlot {
                    id: format!("{}:{}", category, relative_path),
                    category: category.to_string(),
                    content: relative_path,
                    tags,
                    metadata: Some(serde_json::json!({
                        "extension": extension,
                        "size": content.len(),
                        "geometry_map": geometry.get(0..10).map(|s| s.to_vec()),
                        "security": security_meta
                    })),
                    timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                }));
            }
        });

        println!("[CONTEXT] Parallel index cycle complete.");
        Ok(())
    }

    fn compute_hash(content: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    fn extract_symbols_sync(content: &str, ext: &str, path: &str) -> Vec<crate::memory_store::SymbolDefinition> {
        // Reuse the existing extraction logic but ensuring it's available synchronously
        Self::extract_symbols_detailed(content, ext, path)
    }

    fn is_indexable(p: &Path) -> bool {
        p.is_file()
            && (p.extension().map_or(false, |ext| {
                ext == "rs" || ext == "ts" || ext == "tsx" || ext == "json" || ext == "md"
            }))
            && !p.to_string_lossy().contains("node_modules")
            && !p.to_string_lossy().contains("target")
            && !p.to_string_lossy().contains(".git")
    }

    async fn index_single_file(ms: &MemoryStore, root: &Path, path: &Path) -> anyhow::Result<()> {
        let relative_path = path.strip_prefix(root)?.to_string_lossy().to_string();
        let extension = path.extension().and_then(|e: &std::ffi::OsStr| e.to_str()).unwrap_or("");

        let mut category = if extension == "md" {
            "fix_lessons"
        } else {
            "file_map"
        };

        let content = std::fs::read_to_string(path).unwrap_or_default();
        let mut tags = Vec::new();

        if extension == "rs" || extension == "ts" || extension == "tsx" {
            let symbols = Self::extract_symbols_detailed(&content, extension, &relative_path);
            for sym in symbols {
                tags.push(format!("symbol:{}", sym.name));
                ms.store_symbol(sym).await;
            }
        }

        if extension == "md" {
            if content.contains("# Learning") || content.contains("# Fix") {
                category = "fix_lessons";
                tags.push("discovery:lesson".to_string());
            }
        }

        ms.store_slot(SemanticSlot {
            id: format!("{}:{}", category, relative_path),
            category: category.to_string(),
            content: relative_path.clone(),
            tags,
            metadata: Some(json!({
                "extension": extension,
                "size": std::fs::metadata(path)?.len()
            })),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
        })
        .await;

        Ok(())
    }

    fn extract_symbols_detailed(content: &str, ext: &str, path: &str) -> Vec<crate::memory_store::SymbolDefinition> {
        let mut symbols = Vec::new();
        let mut parser = Parser::new();

        let language = match ext {
            "rs" => tree_sitter_rust::LANGUAGE,
            "ts" | "tsx" => tree_sitter_typescript::LANGUAGE_TYPESCRIPT,
            "js" | "jsx" => tree_sitter_typescript::LANGUAGE_TSX,
            "py" => tree_sitter_python::LANGUAGE,
            _ => return Vec::new(),
        };

        parser.set_language(&language.into()).expect("Error loading language");
        let tree = parser.parse(content, None).expect("Error parsing code");

        let query_str = match ext {
            "rs" => "(function_item name: (identifier) @name) @kind_func
                     (struct_item name: (type_identifier) @name) @kind_struct
                     (enum_item name: (type_identifier) @name) @kind_enum
                     (trait_item name: (type_identifier) @name) @kind_trait
                     (impl_item type: (type_identifier) @name) @kind_impl",
            "ts" | "tsx" | "js" | "jsx" => "(function_declaration name: (identifier) @name) @kind_func
                                             (variable_declarator name: (identifier) @name value: (arrow_function)) @kind_func
                                             (method_definition name: (property_identifier) @name) @kind_func
                                             (class_declaration name: (type_identifier) @name) @kind_class
                                             (interface_declaration name: (type_identifier) @name) @kind_interface
                                             (type_alias_declaration name: (type_identifier) @name) @kind_type",
            "py" => "(function_definition name: (identifier) @name) @kind_func
                     (class_definition name: (identifier) @name) @kind_class",
            _ => "",
        };

        if query_str.is_empty() {
            return Vec::new();
        }

        let query = Query::new(&language.into(), query_str).expect("Error creating query");
        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&query, tree.root_node(), content.as_bytes());
        while let Some(m) = StreamingIterator::next(&mut matches) {
            let mut name = String::new();
            let mut kind = "unknown".to_string();
            let mut node_range = (0, 0);

            for capture in m.captures {
                let capture_name = query.capture_names()[capture.index as usize];
                if capture_name == "name" {
                    if let Ok(n) = capture.node.utf8_text(content.as_bytes()) {
                        let n_str: &str = n;
                        name = n_str.to_string();
                    }
                } else if capture_name.starts_with("kind_") {
                    kind = capture_name[5..].to_string();
                    node_range = (capture.node.start_position().row + 1, capture.node.end_position().row + 1);
                }
            }

            if !name.is_empty() {
                symbols.push(crate::memory_store::SymbolDefinition {
                    name,
                    path: path.to_string(),
                    kind,
                    line_range: node_range,
                });
            }
        }
        symbols
    }

    fn extract_symbols(content: &str, ext: &str) -> Vec<String> {
        Self::extract_symbols_detailed(content, ext, "").into_iter().map(|s| s.name).collect()
    }
 
    pub fn reindex_if_needed(&self, root: &Path) -> Result<(), anyhow::Error> {
        let _meta = std::fs::metadata(root)?;
        
        // Use a simple mtime check to see if we should trigger a full scan
        // In a real Antigravity implementation, this would be more granular.
        println!("[CONTEXT] Proactive re-index check triggered for: {:?}", root);
        let ms = self.memory_store.clone();
        let rt = tokio::runtime::Handle::current();
        let hashes = self.hashes.clone();
        let root_buf = root.to_path_buf();
        
        rt.spawn(async move {
            let _ = Self::run_index_cycle(&ms, &root_buf, hashes).await;
        });

        Ok(())
    }
}
