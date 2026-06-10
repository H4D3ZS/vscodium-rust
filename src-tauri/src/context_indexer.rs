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

/// Indexing-only ignores via `cursor_compat` (excludes `.cursorignore`).
#[derive(Clone)]
struct IgnoreSet {
    root: PathBuf,
    matcher: crate::cursor_compat::CursorIgnoreSet,
}

impl IgnoreSet {
    fn load(root: &Path) -> Self {
        Self {
            root: root.to_path_buf(),
            matcher: crate::cursor_compat::CursorIgnoreSet::load(
                root,
                crate::cursor_compat::IgnoreScope::Indexing,
            ),
        }
    }

    fn is_ignored(&self, path: &Path) -> bool {
        self.matcher.is_ignored(path)
    }
}

pub struct ContextIndexer {
    memory_store: Arc<MemoryStore>,
    root_path: PathBuf,
    hashes: Arc<RwLock<HashMap<PathBuf, String>>>,
    ignore_set: Arc<RwLock<IgnoreSet>>,
}

impl ContextIndexer {
    pub fn new(memory_store: Arc<MemoryStore>, root_path: PathBuf) -> Self {
        let ignore_set = IgnoreSet::load(&root_path);
        Self {
            memory_store,
            root_path,
            hashes: Arc::new(RwLock::new(HashMap::new())),
            ignore_set: Arc::new(RwLock::new(ignore_set)),
        }
    }

    /// Re-read `.hadesignore` / `.cursorignore` / `.cursorindexignore` / `.gitignore`. Called
    /// at the top of each index cycle so edits to the ignore files take
    /// effect on the next pass.
    fn refresh_ignores(&self) {
        let fresh = IgnoreSet::load(&self.root_path);
        if let Ok(mut w) = self.ignore_set.write() {
            *w = fresh;
        }
    }

    pub async fn start_background_indexing(&self) {
        let ms = self.memory_store.clone();
        let root = self.root_path.clone();
        let ignore_set = self.ignore_set.clone();

        // Start real-time incremental indexing
        let (tx, mut rx) = mpsc::channel(100);
        let watcher_res = RecommendedWatcher::new(move |res: notify::Result<Event>| {
            if let Ok(event) = res {
                let _ = tx.blocking_send(event);
            }
        }, notify::Config::default());

        if let Ok(mut watcher) = watcher_res {
            if let Err(e) = watcher.watch(&root, RecursiveMode::Recursive) {
                eprintln!("[CONTEXT] ❌ Failed to start watcher: {:?}", e);
            } else {
                tauri::async_runtime::spawn(async move {
                    println!("[CONTEXT] Starting incremental indexing loop for: {:?}", root);
                    // Keep watcher alive in this thread
                    let _watcher = watcher;
                    
                    while let Some(event) = rx.recv().await {
                        match event.kind {
                            EventKind::Modify(_) | EventKind::Create(_) => {
                                // If the user edited `.cursorignore`, refresh
                                // the cached matcher so subsequent events
                                // honor the new rules immediately.
                                if event.paths.iter().any(|p| p.file_name().map(|n| n == ".hadesignore" || n == ".cursorignore" || n == ".cursorindexignore" || n == ".gitignore").unwrap_or(false)) {
                                    let fresh = IgnoreSet::load(&root);
                                    if let Ok(mut w) = ignore_set.write() {
                                        *w = fresh;
                                    }
                                }
                                let snapshot = ignore_set.read().ok().map(|g| g.clone());
                                for path in event.paths {
                                    if Self::is_indexable_with(&path, snapshot.as_ref()) {
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
            }
        } else {
            eprintln!("[CONTEXT] ❌ Failed to create file watcher.");
        }

        // Also run a full cycle periodically to ensure consistency
        let ms_full = self.memory_store.clone();
        let root_full = self.root_path.clone();
        let hashes_full = self.hashes.clone();
        let ig_full = self.ignore_set.clone();
        tauri::async_runtime::spawn(async move {
            // Never run the full repo walk on the boot path. The incremental
            // watcher above covers edits immediately; the full cycle is only a
            // consistency sweep. Lite (potato) machines wait 5 minutes, others
            // 90s. `trigger_index_cycle` remains available for on-demand runs.
            let first_delay = if crate::system_profile::is_lite() { 300 } else { 90 };
            println!("[CONTEXT] First full index cycle deferred {first_delay}s (lite={})", crate::system_profile::is_lite());
            sleep(Duration::from_secs(first_delay)).await;
            loop {
                // Re-read the ignore files at the start of each cycle so the
                // hourly resync picks up edits without an IDE restart.
                let fresh = IgnoreSet::load(&root_full);
                if let Ok(mut w) = ig_full.write() {
                    *w = fresh;
                }
                let snapshot = ig_full.read().ok().map(|g| g.clone());
                if let Err(e) = Self::run_index_cycle(&ms_full, &root_full, hashes_full.clone(), snapshot).await {
                    eprintln!("[CONTEXT] Periodic full indexing error: {:?}", e);
                }
                sleep(Duration::from_secs(3600)).await; // Full sync every hour
            }
        });
    }

    pub async fn trigger_index_cycle(&self) -> anyhow::Result<()> {
        self.refresh_ignores();
        let snapshot = self.ignore_set.read().ok().map(|g| g.clone());
        Self::run_index_cycle(&self.memory_store, &self.root_path, self.hashes.clone(), snapshot).await
    }

    async fn run_index_cycle(ms: &MemoryStore, root: &Path, hashes: Arc<RwLock<HashMap<PathBuf, String>>>, ignore_set: Option<IgnoreSet>) -> anyhow::Result<()> {
        // Auto-create .aim directory so every workspace gets indexed on first open.
        let aim_dir = root.join(".aim");
        if !aim_dir.exists() {
            if let Err(e) = std::fs::create_dir_all(&aim_dir) {
                eprintln!("[CONTEXT] Could not create .aim dir: {}", e);
            } else {
                println!("[CONTEXT] Created .aim directory at {:?}", aim_dir);
            }
        }

        println!("[CONTEXT] Starting parallel index cycle for: {:?}", root);

        // 1. Collect all indexable paths first
        let ig_ref = ignore_set.as_ref();
        let paths: Vec<PathBuf> = WalkDir::new(root)
            .into_iter()
            .filter_entry(|e| {
                // Prune ignored directories so we don't even walk into them.
                if e.file_type().is_dir() {
                    if let Some(g) = ig_ref { if g.is_ignored(e.path()) { return false; } }
                }
                true
            })
            .filter_map(|e| e.ok())
            .filter(|e| Self::is_indexable_with(e.path(), ig_ref))
            .map(|e| e.path().to_path_buf())
            .collect();

        println!("[CONTEXT] Found {} indexable files. Dispatching to Rayon threadpool.", paths.len());

        // Hard cap: skip indexing if we already have enough slots.
        // This prevents a 10k-file project from filling 800 slots with stale "code" entries.
        // The incremental watcher handles new files as they change.
        let paths = if paths.len() > 2000 {
            println!("[CONTEXT] Capping index cycle to first 2000 files for memory safety.");
            paths[..2000].to_vec()
        } else {
            paths
        };

        // Use a bounded Rayon thread count to avoid simultaneous file reads filling RAM.
        // Default Rayon pool = num_cpus. On 16-core machines, 16 files × 1MB = 16MB peak.
        // That's acceptable; the real cap above (2000 files) is the main safeguard.
        let embedder = hades_harness::PythagoreanEmbedder::new(1536);

        // 2. Process in parallel using Rayon
        paths.par_iter().for_each(|path: &PathBuf| {
            let relative_path = path.strip_prefix(root).unwrap_or(path).to_string_lossy().to_string();
            let content_res = std::fs::read_to_string(path);
            
            if let Ok(content) = content_res {
                // Hashing Check
                let current_hash = Self::compute_hash(&content);
                {
                    if let Ok(h_lock) = hashes.read() {
                        if h_lock.get(path) == Some(&current_hash) {
                            return; // Skip unchanged file
                        }
                    }
                }
                
                // Update Hash
                {
                    if let Ok(mut h_lock) = hashes.write() {
                        h_lock.insert(path.clone(), current_hash);
                    }
                }

                let extension = path.extension().and_then(|e: &std::ffi::OsStr| e.to_str()).unwrap_or("");
                let category = if extension == "md" { "fix_lessons" } else { "file_map" };
                let mut tags = Vec::new();

                // 2.1 Calculate Pythagorean Embedding (Geometric Logic)
                let geometry = embedder.embed_path(Path::new(&relative_path));

                if extension == "rs" || extension == "ts" || extension == "tsx" {
                    let symbols = Self::extract_symbols_sync(&content, extension, &relative_path);
                    for sym in symbols {
                        tags.push(format!("symbol:{}", sym.name));
                        ms.store_symbol_sync(sym);
                    }
                }

                // 2.2 Security Analysis (Distillation)
                let security_meta = crate::security_distiller::SecurityDistiller::get_security_metadata(&content);

                // Build a meaningful content gist: file path + first 400 chars of content
                // This makes keyword search in retrieve_context() actually useful.
                let content_gist = if content.len() > 400 {
                    let truncated: String = content.chars().take(400).collect();
                    format!("{}: {}", relative_path, truncated.replace('\n', " "))
                } else {
                    format!("{}: {}", relative_path, content.replace('\n', " "))
                };

                ms.store_slot_sync(crate::memory_store::SemanticSlot {
                    id: format!("{}:{}", category, relative_path),
                    category: category.to_string(),
                    content: content_gist,
                    tags,
                    metadata: Some(serde_json::json!({
                        "path": path.to_string_lossy(),
                        "extension": extension,
                        "size": content.len(),
                        "geometry_map": geometry.get(0..10).map(|s| s.to_vec()),
                        "security": security_meta
                    })),
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .map(|d| d.as_secs())
                        .unwrap_or(0),
                });
            }
        });

        // 3. Update the persistent Project Tree in MemoryStore
        let tree_paths: Vec<String> = paths.iter()
            .map(|p| p.strip_prefix(root).unwrap_or(p).to_string_lossy().to_string())
            .collect();
        ms.set_project_tree(tree_paths).await;

        println!("[CONTEXT] Parallel index cycle complete. Indexed {} files.", paths.len());
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

    #[allow(dead_code)]
    fn is_indexable(p: &Path) -> bool {
        Self::is_indexable_with(p, None)
    }

    /// Indexability gate for a universal IDE.
    /// Indexes ANY text file regardless of extension, like VSCode and Cursor.
    /// Binary files are detected by content sniffing (null bytes) and skipped.
    /// Explicitly skips known garbage/generated directories and binary extensions.
    fn is_indexable_with(p: &Path, ignore_set: Option<&IgnoreSet>) -> bool {
        if !p.is_file() {
            return false;
        }

        // 1. Hard prune directories that universally contain generated/binary artifacts
        let s = p.to_string_lossy();
        let pruned_segments = [
            "node_modules", ".git", "/target/", "\\target\\",
            "/dist/", "\\dist\\", "/.cache/", "\\.cache\\",
            "/build/", "\\build\\", "/__pycache__/", "\\__pycache__\\",
            "/.venv/", "\\.venv\\", "/venv/", "\\venv\\",
            "/vendor/", "\\vendor\\", "/.next/", "\\.next\\",
            "/out/", "\\out\\", "/.turbo/", "\\.turbo\\",
            // Monorepo bulk — skip indexing multi-GB vendored trees by default.
            "/kortex/", "\\kortex\\", "/airi/", "\\airi\\",
            "/llama.cpp/", "\\llama.cpp\\", "/.agent/", "\\.agent\\",
            "/Cyber-Ifrit-Portfolio/", "\\Cyber-Ifrit-Portfolio\\",
        ];
        for seg in &pruned_segments {
            if s.contains(seg) { return false; }
        }

        // 2. Skip universally binary extensions — no content inspection needed
        let binary_exts = [
            // Images
            "png", "jpg", "jpeg", "gif", "bmp", "ico", "webp", "svg", "tiff", "avif",
            // Video/Audio
            "mp4", "mp3", "webm", "mov", "avi", "mkv", "wav", "ogg", "flac",
            // Archives
            "zip", "tar", "gz", "bz2", "xz", "rar", "7z",
            // Compiled/Binary
            "exe", "dll", "so", "dylib", "a", "lib", "wasm",
            "class", "pyc", "pyo", "o", "obj",
            // Fonts
            "ttf", "otf", "woff", "woff2", "eot",
            // Documents/DB
            "pdf", "docx", "xlsx", "pptx", "sqlite", "db",
            // Lock files with massive content useless to AI
            "lock",
        ];
        if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
            if binary_exts.contains(&ext.to_lowercase().as_str()) {
                return false;
            }
        }

        // 3. Skip files over 2MB — too large to be useful as a context gist
        if let Ok(meta) = p.metadata() {
            if meta.len() > 2 * 1024 * 1024 {
                return false;
            }
        }

        // 4. Respect user ignore files (.gitignore, .cursorignore, etc.)
        if let Some(ig) = ignore_set {
            if ig.is_ignored(p) { return false; }
        }

        // 5. Text detection: read first 512 bytes and check for null bytes.
        //    If null bytes are present, it's binary. This is the same heuristic
        //    used by git, ripgrep, and VSCode.
        if let Ok(mut f) = std::fs::File::open(p) {
            use std::io::Read;
            let mut probe = [0u8; 512];
            let n = f.read(&mut probe).unwrap_or(0);
            if probe[..n].contains(&0u8) {
                return false; // Binary file — skip
            }
        } else {
            return false; // Can't open — skip
        }

        true
    }


    async fn index_single_file(ms: &MemoryStore, root: &Path, path: &Path) -> anyhow::Result<()> {
        let relative_path = path.strip_prefix(root)?.to_string_lossy().to_string();
        let extension = path.extension().and_then(|e: &std::ffi::OsStr| e.to_str()).unwrap_or("");

        let mut category = if extension == "md" {
            "fix_lessons"
        } else {
            "file_map"
        };

        let content = std::fs::read_to_string(path).unwrap_or_else(|_| String::new());
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
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
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

    /// Returns just the symbol names for a file — useful for quick context summaries.
    pub fn extract_symbols(content: &str, ext: &str) -> Vec<String> {
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
        let snapshot = self.ignore_set.read().ok().map(|g| g.clone());

        rt.spawn(async move {
            let _ = Self::run_index_cycle(&ms, &root_buf, hashes, snapshot).await;
        });

        Ok(())
    }
}
