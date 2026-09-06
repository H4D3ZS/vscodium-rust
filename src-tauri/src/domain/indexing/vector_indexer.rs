// =============================================================================
// Vector Indexer - Codebase Semantic Search & Context Injection
// Similar to Cursor's codebase indexing with vector embeddings
// =============================================================================

use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use tokio::sync::Mutex;
use walkdir::WalkDir;
use sha2::{Sha256, Digest};
use rayon::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeChunk {
    pub id: String,
    pub file_path: String,
    pub content: String,
    pub start_line: usize,
    pub end_line: usize,
    pub language: String,
    pub symbols: Vec<String>,
    pub embedding: Option<Vec<f32>>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub file_path: String,
    pub content: String,
    pub start_line: usize,
    pub end_line: usize,
    pub relevance_score: f32,
    pub context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexStats {
    pub total_files: usize,
    pub total_chunks: usize,
    pub total_symbols: usize,
    pub languages: HashMap<String, usize>,
    pub last_indexed: Option<u64>,
    pub index_size_bytes: usize,
}

pub struct VectorIndexer {
    db_path: PathBuf,
    root_path: Arc<RwLock<PathBuf>>,
    fallback_dir: PathBuf,
    conn: Arc<Mutex<Connection>>,
    file_hashes: Arc<RwLock<HashMap<PathBuf, String>>>,
    is_indexing: Arc<RwLock<bool>>,
    indexing_progress: Arc<RwLock<IndexingProgress>>,
    ann: Arc<std::sync::Mutex<crate::ann_index::AnnIndex>>,
    /// Base URL for the embedding endpoint. Defaults to **Lemonade**
    /// (`http://localhost:13305`) — this machine has no raw :11434 server, and
    /// pointing here at a dead host silently disabled `@codebase`,
    /// `semantic_search` and `search_codebase`, because a failed embed degrades
    /// to an empty result instead of an error anyone sees.
    /// Updated via `set_embed_url` when the user switches inference backends.
    embed_url: Arc<RwLock<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexingProgress {
    pub is_indexing: bool,
    pub current_file: String,
    pub files_processed: usize,
    pub total_files: usize,
    pub chunks_created: usize,
    pub progress_percent: f32,
}

impl VectorIndexer {
    /// `fallback_dir` is used when `root_path` is read-only (e.g. `Program Files`
    /// when the app is launched from its install folder without a project open).
    pub fn new(root_path: PathBuf, fallback_dir: PathBuf) -> anyhow::Result<Self> {
        let db_path = Self::resolve_db_path(&root_path, &fallback_dir)?;
        let conn = Connection::open(&db_path)?;
        
        // Enable WAL mode for better concurrent performance
        conn.execute_batch("
            PRAGMA journal_mode=WAL;
            PRAGMA synchronous=NORMAL;
            PRAGMA cache_size=10000;
            PRAGMA temp_store=MEMORY;
        ")?;

        // Create tables
        Self::create_tables(&conn)?;

        let kortex_dir = db_path.parent().unwrap_or(&db_path).to_path_buf();
        let ann = Arc::new(std::sync::Mutex::new(crate::ann_index::AnnIndex::new(kortex_dir)));
        if let Ok(a) = ann.lock() {
            let _ = a.load();
        }

        Ok(Self {
            db_path: db_path.clone(),
            root_path: Arc::new(RwLock::new(root_path)),
            fallback_dir,
            conn: Arc::new(Mutex::new(conn)),
            file_hashes: Arc::new(RwLock::new(HashMap::new())),
            is_indexing: Arc::new(RwLock::new(false)),
            indexing_progress: Arc::new(RwLock::new(IndexingProgress {
                is_indexing: false,
                current_file: String::new(),
                files_processed: 0,
                total_files: 0,
                chunks_created: 0,
                progress_percent: 0.0,
            })),
            ann,
            embed_url: Arc::new(RwLock::new(
                crate::embeddings::default_embed_base_url(),
            )),
        })
    }

    fn resolve_db_path(root: &Path, fallback_dir: &Path) -> anyhow::Result<PathBuf> {
        let preferred = root.join(".kortex").join("vector_index.db");
        if Self::ensure_db_parent(&preferred).is_ok() {
            return Ok(preferred);
        }
        let fb = fallback_dir.join(".kortex").join("vector_index.db");
        Self::ensure_db_parent(&fb)?;
        Ok(fb)
    }

    /// Update the embedding endpoint URL (e.g. when switching to Lemonade).
    pub fn set_embed_url(&self, url: String) {
        if let Ok(mut u) = self.embed_url.write() {
            *u = url;
        }
    }

    fn current_embed_url(&self) -> String {
        self.embed_url
            .read()
            .map(|u| u.clone())
            .unwrap_or_else(|_| crate::embeddings::default_embed_base_url())
    }

    fn ensure_db_parent(db_path: &Path) -> anyhow::Result<()> {
        let parent = db_path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("invalid vector index path"))?;
        std::fs::create_dir_all(parent).map_err(|e| anyhow::anyhow!("{e}"))
    }

    fn create_tables(conn: &Connection) -> anyhow::Result<()> {
        conn.execute_batch("
            CREATE TABLE IF NOT EXISTS code_chunks (
                id TEXT PRIMARY KEY,
                file_path TEXT NOT NULL,
                content TEXT NOT NULL,
                start_line INTEGER NOT NULL,
                end_line INTEGER NOT NULL,
                language TEXT NOT NULL,
                symbols TEXT,
                embedding BLOB,
                timestamp INTEGER NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_file_path ON code_chunks(file_path);
            CREATE INDEX IF NOT EXISTS idx_language ON code_chunks(language);

            CREATE TABLE IF NOT EXISTS symbols (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                file_path TEXT NOT NULL,
                kind TEXT NOT NULL,
                line_start INTEGER,
                line_end INTEGER,
                UNIQUE(name, file_path, kind)
            );

            CREATE INDEX IF NOT EXISTS idx_symbol_name ON symbols(name);
            CREATE INDEX IF NOT EXISTS idx_symbol_file ON symbols(file_path);

            CREATE TABLE IF NOT EXISTS file_metadata (
                file_path TEXT PRIMARY KEY,
                hash TEXT NOT NULL,
                last_modified INTEGER NOT NULL,
                language TEXT NOT NULL,
                size_bytes INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS index_metadata (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
        ")?;

        Ok(())
    }

    /// Switch vector DB to `{workspace}/.kortex/vector_index.db` when the user opens a folder.
    pub async fn set_workspace(&self, root: PathBuf) -> anyhow::Result<()> {
        {
            let current = self.root_path.read().map_err(|_| anyhow::anyhow!("Lock poisoned"))?;
            if *current == root {
                return Ok(());
            }
        }
        let db_path = Self::resolve_db_path(&root, &self.fallback_dir)?;
        let new_conn = Connection::open(&db_path)?;
        Self::create_tables(&new_conn)?;
        {
            let mut rp = self.root_path.write().map_err(|_| anyhow::anyhow!("Lock poisoned"))?;
            *rp = root;
        }
        *self.conn.lock().await = new_conn;
        if let Ok(mut hashes) = self.file_hashes.write() {
            hashes.clear();
        }
        let kortex_dir = db_path.parent().unwrap_or(&db_path).to_path_buf();
        if let Ok(mut a) = self.ann.lock() {
            *a = crate::ann_index::AnnIndex::new(kortex_dir);
            let _ = a.load();
        }
        Ok(())
    }

    async fn rebuild_ann_from_db(&self) -> anyhow::Result<()> {
        if let Ok(a) = self.ann.lock() {
            let _ = a.clear();
        }
        let rows: Vec<(String, Vec<u8>)> = {
            let conn = self.conn.lock().await;
            let mut stmt = conn.prepare(
                "SELECT id, embedding FROM code_chunks WHERE embedding IS NOT NULL",
            )?;
            let mapped = stmt.query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, Option<Vec<u8>>>(1)?.unwrap_or_default(),
                ))
            })?;
            mapped
                .filter_map(|r| r.ok())
                .filter(|(_, b)| !b.is_empty())
                .collect()
        };
        if let Ok(a) = self.ann.lock() {
            for (id, blob) in rows {
                if let Some(emb) = Self::parse_embedding_blob(&blob) {
                    let _ = a.upsert(&id, &emb);
                }
            }
        }
        Ok(())
    }

    // ── Main Indexing Entry Point ─────────────────────────────────────────

    pub async fn index_codebase(&self) -> anyhow::Result<IndexingProgress> {
        // Idempotent: duplicate starts (StrictMode double-mount, folder + status bar)
        // should poll progress instead of failing.
        {
            let mut is_indexing = self.is_indexing.write().map_err(|_| anyhow::anyhow!("Lock poisoned"))?;
            if *is_indexing {
                let progress = self.indexing_progress.read().map_err(|_| anyhow::anyhow!("Lock poisoned"))?;
                return Ok(progress.clone());
            }
            *is_indexing = true;
        }

        let progress = IndexingProgress {
            is_indexing: true,
            current_file: String::new(),
            files_processed: 0,
            total_files: 0,
            chunks_created: 0,
            progress_percent: 0.0,
        };
        *self.indexing_progress.write().map_err(|_| anyhow::anyhow!("Lock poisoned"))? = progress.clone();

        let root = self
            .root_path
            .read()
            .map_err(|_| anyhow::anyhow!("Lock poisoned"))?
            .clone();
        let conn = self.conn.clone();
        let file_hashes = self.file_hashes.clone();
        let indexing_progress = self.indexing_progress.clone();
        let is_indexing = self.is_indexing.clone();
        let ann = self.ann.clone();
        let embed_url = self.current_embed_url();

        tokio::spawn(async move {
            let result = Self::run_indexing_internal(&root, &conn, &file_hashes, &indexing_progress, &ann, &embed_url).await;

            // Mark indexing as complete
            if let Ok(mut indexing) = is_indexing.write() {
                *indexing = false;
            }

            if let Err(e) = result {
                eprintln!("[VECTOR_INDEX] Error: {:?}", e);
            }
        });

        Ok(progress)
    }

    async fn run_indexing_internal(
        root: &PathBuf,
        conn: &Arc<Mutex<Connection>>,
        file_hashes: &Arc<RwLock<HashMap<PathBuf, String>>>,
        indexing_progress: &Arc<RwLock<IndexingProgress>>,
        ann: &Arc<std::sync::Mutex<crate::ann_index::AnnIndex>>,
        embed_url: &str,
    ) -> anyhow::Result<()> {
        println!("[VECTOR_INDEX] Starting codebase indexing for: {:?}", root);

        // Collect all indexable files.
        // Markdown/text are included so the semantic (embedding) path can retrieve
        // the knowledge corpus — docs, knowledge briefs, and `.agent/` skills — not
        // just code. This is what lets a small local model punch above its weight on
        // this project's domain: retrieval over meaning, not just keyword gists.
        let extensions = &["rs", "ts", "tsx", "js", "jsx", "py", "go", "c", "cpp", "h", "hpp", "java", "cs", "rb", "md", "mdx", "markdown", "txt"];
        let ignore_set = crate::cursor_compat::CursorIgnoreSet::load(
            root,
            crate::cursor_compat::IgnoreScope::Indexing,
        );

        let files: Vec<PathBuf> = WalkDir::new(root.as_path())
            .into_iter()
            .filter_entry(|e| {
                if e.file_type().is_dir() {
                    let name = e.file_name().to_string_lossy();
                    if name == "node_modules" || name == ".git" || name == ".aim" || name == ".kortex" || name == "target" || name == "dist" || name == ".cache" || name == "build" {
                        return false;
                    }
                    // Vendored/generated bulk. Previously masked by the code-only
                    // extension filter; now that markdown is indexed we must prune
                    // these explicitly or we'd embed thousands of vendored docs.
                    // NOTE: `.agent/` is intentionally NOT pruned here — its skills
                    // are exactly the domain knowledge we want semantically retrievable.
                    if matches!(name.as_ref(),
                        "airi" | "llama.cpp" | "vendor" | ".venv" | "venv"
                        | "__pycache__" | ".next" | "out" | ".turbo" | ".cache") {
                        return false;
                    }
                    if ignore_set.is_ignored(e.path()) {
                        return false;
                    }
                }
                true
            })
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.file_type().is_file() &&
                !ignore_set.is_ignored(e.path()) &&
                e.path().extension().map_or(false, |ext| {
                    extensions.contains(&ext.to_str().unwrap_or(""))
                })
            })
            .map(|e| e.path().to_path_buf())
            .collect();

        let total_files = files.len();
        println!("[VECTOR_INDEX] Found {} files to index", total_files);

        {
            let mut progress = indexing_progress.write().map_err(|_| anyhow::anyhow!("Lock poisoned"))?;
            progress.total_files = total_files;
        }

        let mut chunks_created = 0;
        let mut symbols_extracted = 0;

        for (idx, file_path) in files.iter().enumerate() {
            let relative_path = file_path.strip_prefix(root)
                .unwrap_or(file_path)
                .to_string_lossy()
                .to_string();

            {
                let mut progress = indexing_progress.write().map_err(|_| anyhow::anyhow!("Lock poisoned"))?;
                progress.current_file = relative_path.clone();
                progress.files_processed = idx + 1;
                progress.progress_percent = ((idx + 1) as f32 / total_files as f32) * 100.0;
            }

            // Read file content
            let content = match std::fs::read_to_string(file_path) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("[VECTOR_INDEX] Error reading {:?}: {:?}", file_path, e);
                    continue;
                }
            };

            // Check if file changed
            let current_hash = Self::compute_hash(&content);
            {
                let hashes = file_hashes.read().map_err(|_| anyhow::anyhow!("Lock poisoned"))?;
                if let Some(existing_hash) = hashes.get(file_path) {
                    if *existing_hash == current_hash {
                        continue; // File unchanged
                    }
                }
            }

            // Update hash
            {
                let mut hashes = file_hashes.write().map_err(|_| anyhow::anyhow!("Lock poisoned"))?;
                hashes.insert(file_path.clone(), current_hash.clone());
            }

            // Chunk the file
            let extension = file_path.extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");

            let mut chunks = Self::chunk_code(&content, extension, &relative_path);

            // Compute embeddings OUTSIDE the SQLite connection lock
            for chunk in &mut chunks {
                let embed_text: String = chunk.content.chars().take(1500).collect();
                match crate::embeddings::embed_text_at(&embed_text, None, embed_url).await {
                    Ok(emb) => chunk.embedding = Some(emb),
                    Err(e) => {
                        // Never swallow this again. Discarding the error here is
                        // how the index ended up with 0 of 33,205 chunks
                        // embedded: the backend was unreachable, every call
                        // failed, NULL was stored, and `@codebase` quietly
                        // degraded to text matching for months with no symptom.
                        //
                        // Warned once per indexing run — a per-chunk log would
                        // print tens of thousands of identical lines.
                        static WARNED: std::sync::atomic::AtomicBool =
                            std::sync::atomic::AtomicBool::new(false);
                        if !WARNED.swap(true, std::sync::atomic::Ordering::Relaxed) {
                            eprintln!(
                                "[index] EMBEDDINGS UNAVAILABLE at {embed_url}: {e}\n\
                                 [index] Indexing will continue but semantic search WILL NOT WORK \
                                 (chunks are stored without vectors).\n\
                                 [index] Fix: start Lemonade and run `lemonade pull {}`.",
                                crate::embeddings::default_embed_model()
                            );
                        }
                    }
                }
            }

            // Extract symbols OUTSIDE the SQLite lock
            let symbols = Self::extract_symbols(&content, extension, &relative_path);

            // Store chunks and symbols (Brief Lock Scope)
            {
                let conn_lock = conn.lock().await;
                for chunk in &chunks {
                    Self::store_chunk(&conn_lock, chunk)?;
                    chunks_created += 1;
                }

                // Store symbols
                for symbol in &symbols {
                    Self::store_symbol(&conn_lock, symbol)?;
                    symbols_extracted += 1;
                }

                // Store file metadata
                let file_size = file_path.metadata().map(|m| m.len()).unwrap_or(0);
                let file_modified = file_path.metadata()
                    .and_then(|m| m.modified())
                    .map(|t| t.duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0))
                    .unwrap_or(0);

                Self::store_file_metadata(&conn_lock, &relative_path, &current_hash, file_modified, extension, file_size)?;
            }

            // Update progress with chunks
            {
                let mut progress = indexing_progress.write().map_err(|_| anyhow::anyhow!("Lock poisoned"))?;
                progress.chunks_created = chunks_created;
            }
        }

        // Update index metadata
        {
            let conn_lock = conn.lock().await;
            Self::update_index_metadata(&conn_lock, "last_indexed", &chrono::Utc::now().timestamp().to_string())?;
            // Record which model produced these vectors. Embedding dimensions
            // differ between models (nomic-embed-text is 768, Qwen3-Embedding is
            // 1024) and `cosine_similarity` returns 0.0 on a length mismatch —
            // so an index built by a different model does not error, it just
            // ranks everything as unrelated. Storing this lets a reader detect
            // the mismatch instead of concluding the codebase has no matches.
            Self::update_index_metadata(&conn_lock, "embed_model", crate::embeddings::default_embed_model())?;
            Self::update_index_metadata(
                &conn_lock,
                "embed_dims",
                &crate::embeddings::DEFAULT_EMBED_DIMS.to_string(),
            )?;
        }

        println!("[VECTOR_INDEX] Indexing complete: {} files, {} chunks, {} symbols", 
                 total_files, chunks_created, symbols_extracted);

        // Rebuild ANN flat index from stored embeddings
        if let Ok(a) = ann.lock() {
            let _ = a.clear();
        }
        let rows: Vec<(String, Vec<u8>)> = {
            let conn_lock = conn.lock().await;
            let mut stmt = conn_lock.prepare(
                "SELECT id, embedding FROM code_chunks WHERE embedding IS NOT NULL",
            )?;
            let mapped = stmt.query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, Option<Vec<u8>>>(1)?.unwrap_or_default(),
                ))
            })?;
            mapped
                .filter_map(|r| r.ok())
                .filter(|(_, b)| !b.is_empty())
                .collect()
        };
        if let Ok(a) = ann.lock() {
            for (id, blob) in rows {
                if let Some(emb) = Self::parse_embedding_blob(&blob) {
                    let _ = a.upsert(&id, &emb);
                }
            }
            println!("[VECTOR_INDEX] ANN flat index: {} vectors", a.len());
        }

        {
            let mut progress = indexing_progress.write().map_err(|_| anyhow::anyhow!("Lock poisoned"))?;
            progress.is_indexing = false;
            progress.progress_percent = 100.0;
        }

        Ok(())
    }

    // ── Code Chunking ─────────────────────────────────────────────────────

    fn chunk_code(content: &str, extension: &str, file_path: &str) -> Vec<CodeChunk> {
        let lines: Vec<&str> = content.lines().collect();
        let mut chunks = Vec::new();
        const CHUNK_SIZE: usize = 50; // lines per chunk
        const OVERLAP: usize = 10;    // overlap between chunks

        let language = Self::extension_to_language(extension);

        // Split into chunks with overlap
        let mut start_line = 0;
        let mut chunk_index = 0;

        while start_line < lines.len() {
            let end_line = (start_line + CHUNK_SIZE).min(lines.len());
            let chunk_content = lines[start_line..end_line].join("\n");
            
            // Skip empty chunks
            if chunk_content.trim().is_empty() {
                start_line = if end_line == lines.len() { 
                    lines.len() 
                } else { 
                    end_line 
                };
                continue;
            }

            let chunk_id = format!("{}:{}:{}", file_path, chunk_index, start_line);

            // Extract symbols from this chunk
            let symbols: Vec<String> = Self::extract_symbols_from_chunk(&chunk_content, extension);

            chunks.push(CodeChunk {
                id: chunk_id,
                file_path: file_path.to_string(),
                content: chunk_content,
                start_line: start_line + 1,
                end_line,
                language: language.to_string(),
                symbols,
                embedding: None, // Embedding will be computed later
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0),
            });

            chunk_index += 1;
            start_line = if end_line == lines.len() {
                lines.len()
            } else {
                end_line.saturating_sub(OVERLAP)
            };
        }

        chunks
    }

    fn extension_to_language(ext: &str) -> &'static str {
        match ext {
            "rs" => "Rust",
            "ts" | "tsx" => "TypeScript",
            "js" | "jsx" => "JavaScript",
            "py" => "Python",
            "go" => "Go",
            "c" => "C",
            "cpp" | "cxx" | "cc" => "C++",
            "h" | "hpp" => "C/C++ Header",
            "java" => "Java",
            "cs" => "C#",
            "rb" => "Ruby",
            _ => "Unknown",
        }
    }

    // ── Symbol Extraction ─────────────────────────────────────────────────

    fn extract_symbols(content: &str, extension: &str, file_path: &str) -> Vec<SymbolInfo> {
        use tree_sitter::{Parser, Query, QueryCursor};
        use streaming_iterator::StreamingIterator;

        let mut parser = Parser::new();
        
        let language = match extension {
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

        let query_str = match extension {
            "rs" => "(function_item name: (identifier) @name) @kind
                     (struct_item name: (type_identifier) @name) @kind
                     (enum_item name: (type_identifier) @name) @kind
                     (trait_item name: (type_identifier) @name) @kind
                     (impl_item type: (type_identifier) @name) @kind
                     (const_item name: (identifier) @name) @kind
                     (static_item name: (identifier) @name) @kind",
            "ts" | "tsx" | "js" | "jsx" => "(function_declaration name: (identifier) @name) @kind
                                             (class_declaration name: (type_identifier) @name) @kind
                                             (interface_declaration name: (type_identifier) @name) @kind
                                             (type_alias_declaration name: (type_identifier) @name) @kind
                                             (method_definition name: (property_identifier) @name) @kind
                                             (lexical_declaration (variable_declarator name: (identifier) @name)) @kind
                                             (variable_declaration (variable_declarator name: (identifier) @name)) @kind",
            "py" => "(function_definition name: (identifier) @name) @kind
                     (class_definition name: (identifier) @name) @kind",
            _ => return Vec::new(),
        };

        let query = match Query::new(&language.into(), query_str) {
            Ok(q) => q,
            Err(_) => return Vec::new(),
        };

        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&query, tree.root_node(), content.as_bytes());

        let mut symbols = Vec::new();

        while let Some(m) = StreamingIterator::next(&mut matches) {
            let mut name = String::new();
            let mut kind = "unknown".to_string();
            let mut start_line = 0;
            let mut end_line = 0;

            for capture in m.captures {
                let capture_name = query.capture_names()[capture.index as usize];
                match capture_name {
                    "name" => {
                        if let Ok(n) = capture.node.utf8_text(content.as_bytes()) {
                            name = n.to_string();
                        }
                    }
                    "kind" | "kind_func" | "kind_class" | "kind_struct" | "kind_enum" | 
                    "kind_interface" | "kind_type" | "kind_trait" | "kind_impl" => {
                        kind = capture_name.trim_start_matches("kind_").to_string();
                        start_line = capture.node.start_position().row + 1;
                        end_line = capture.node.end_position().row + 1;
                    }
                    _ => {}
                }
            }

            if !name.is_empty() {
                symbols.push(SymbolInfo {
                    name,
                    file_path: file_path.to_string(),
                    kind,
                    start_line,
                    end_line,
                });
            }
        }

        symbols
    }

    fn extract_symbols_from_chunk(content: &str, extension: &str) -> Vec<String> {
        // Simplified symbol extraction for chunks
        let mut symbols = Vec::new();

        let lines: Vec<&str> = content.lines().collect();
        for line in lines {
            let trimmed = line.trim();
            
            // Look for function/class declarations
            let patterns = match extension {
                "rs" => vec!["fn ", "struct ", "enum ", "trait ", "impl "],
                "ts" | "tsx" | "js" | "jsx" => vec!["function ", "class ", "interface ", "type ", "const ", "let "],
                "py" => vec!["def ", "class "],
                _ => vec![],
            };

            for pattern in patterns {
                if trimmed.starts_with(pattern) {
                    if let Some(name) = Self::extract_name_from_line(trimmed, pattern) {
                        symbols.push(name);
                    }
                }
            }
        }

        symbols
    }

    fn extract_name_from_line(line: &str, pattern: &str) -> Option<String> {
        let rest = line.strip_prefix(pattern)?;
        let name = rest.split_whitespace().next()?;
        // Remove parentheses for functions
        Some(name.trim_end_matches('(').to_string())
    }

    // ── Database Operations ───────────────────────────────────────────────

    fn store_chunk(conn: &Connection, chunk: &CodeChunk) -> anyhow::Result<()> {
        let symbols_json = serde_json::to_string(&chunk.symbols).unwrap_or_default();
        let embedding_bytes = chunk.embedding.as_ref().map(|e| {
            let bytes: Vec<u8> = e.iter()
                .flat_map(|f| f.to_le_bytes())
                .collect();
            bytes
        });

        conn.execute(
            "INSERT OR REPLACE INTO code_chunks
             (id, file_path, content, start_line, end_line, language, symbols, embedding, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                chunk.id,
                chunk.file_path,
                chunk.content,
                chunk.start_line as i64,
                chunk.end_line as i64,
                chunk.language,
                symbols_json,
                embedding_bytes,
                chunk.timestamp as i64
            ],
        )?;

        Ok(())
    }

    fn store_symbol(conn: &Connection, symbol: &SymbolInfo) -> anyhow::Result<()> {
        conn.execute(
            "INSERT OR IGNORE INTO symbols (name, file_path, kind, line_start, line_end)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                symbol.name,
                symbol.file_path,
                symbol.kind,
                symbol.start_line as i64,
                symbol.end_line as i64
            ],
        )?;

        Ok(())
    }

    fn store_file_metadata(
        conn: &Connection,
        file_path: &str,
        hash: &str,
        last_modified: u64,
        language: &str,
        size_bytes: u64,
    ) -> anyhow::Result<()> {
        conn.execute(
            "INSERT OR REPLACE INTO file_metadata (file_path, hash, last_modified, language, size_bytes)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                file_path,
                hash,
                last_modified as i64,
                language,
                size_bytes as i64
            ],
        )?;

        Ok(())
    }

    fn update_index_metadata(conn: &Connection, key: &str, value: &str) -> anyhow::Result<()> {
        conn.execute(
            "INSERT OR REPLACE INTO index_metadata (key, value) VALUES (?1, ?2)",
            params![key, value],
        )?;

        Ok(())
    }

    // ── Search Operations ─────────────────────────────────────────────────

    pub async fn search_codebase(&self, query: &str, limit: usize) -> anyhow::Result<Vec<SearchResult>> {
        let query_embedding = crate::embeddings::embed_text_at(query, None, &self.current_embed_url()).await.ok();

        // ANN fast path — hydrate top-k from SQLite by chunk id
        if let Some(ref qe) = query_embedding {
            let hits = self
                .ann
                .lock()
                .ok()
                .and_then(|ann| ann.search(qe, limit.saturating_mul(3).max(limit)).ok())
                .unwrap_or_default();
            if !hits.is_empty() {
                let conn = self.conn.lock().await;
                let mut ann_results = Vec::new();
                for (chunk_id, ann_score) in hits {
                    let row = conn.query_row(
                        "SELECT file_path, content, start_line, end_line FROM code_chunks WHERE id = ?1",
                        params![chunk_id],
                        |row| {
                            Ok(SearchResult {
                                file_path: row.get(0)?,
                                content: row.get(1)?,
                                start_line: row.get::<_, i64>(2)? as usize,
                                end_line: row.get::<_, i64>(3)? as usize,
                                relevance_score: ann_score * 10.0,
                                context: Self::extract_context(
                                    &row.get::<_, String>(1)?,
                                    &query.to_lowercase(),
                                ),
                            })
                        },
                    );
                    if let Ok(r) = row {
                        ann_results.push(r);
                    }
                }
                if ann_results.len() >= limit {
                    ann_results.truncate(limit);
                    return Ok(ann_results);
                }
            }
        }

        let conn = self.conn.lock().await;

        let query_lower = query.to_lowercase();
        let keywords: Vec<&str> = query_lower.split_whitespace().collect();

        let mut stmt = conn.prepare(
            "SELECT file_path, content, start_line, end_line, language, symbols, embedding
             FROM code_chunks
             ORDER BY timestamp DESC
             LIMIT ?1"
        )?;

        let scan_limit = if query_embedding.is_some() { (limit * 20) as i64 } else { (limit * 5) as i64 };
        let rows: Vec<_> = stmt.query_map(params![scan_limit], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, i64>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, Option<Vec<u8>>>(6)?,
            ))
        })?.collect::<Result<Vec<_>, _>>()?;

        let q_emb = query_embedding.clone();
        let keywords_owned: Vec<String> = keywords.iter().map(|s| s.to_string()).collect();
        let mut results: Vec<SearchResult> = rows
            .par_iter()
            .filter_map(|(file_path, content, start_line, end_line, _language, symbols_json, embedding_blob)| {
                let mut score = 0.0f32;
                if let (Some(ref qe), Some(ref blob)) = (&q_emb, embedding_blob) {
                    if let Some(chunk_emb) = Self::parse_embedding_blob(blob) {
                        let sim = crate::embeddings::cosine_similarity(qe, &chunk_emb);
                        if sim > 0.25 {
                            score += sim * 10.0;
                        }
                    }
                }
                let content_lower = content.to_lowercase();
                for keyword in &keywords_owned {
                    let matches = content_lower.matches(keyword.as_str()).count();
                    score += matches as f32;
                    if let Ok(symbols) = serde_json::from_str::<Vec<String>>(symbols_json) {
                        for symbol in &symbols {
                            if symbol.to_lowercase().contains(keyword.as_str()) {
                                score += 2.0;
                            }
                        }
                    }
                }
                if score <= 0.0 {
                    return None;
                }
                Some(SearchResult {
                    file_path: file_path.clone(),
                    content: content.clone(),
                    start_line: *start_line as usize,
                    end_line: *end_line as usize,
                    relevance_score: score,
                    context: Self::extract_context(content, &query_lower),
                })
            })
            .collect();

        results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(limit);

        Ok(results)
    }

    fn parse_embedding_blob(blob: &[u8]) -> Option<Vec<f32>> {
        if blob.len() < 4 || blob.len() % 4 != 0 {
            return None;
        }
        Some(
            blob.chunks_exact(4)
                .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
                .collect(),
        )
    }

    pub async fn find_symbol(&self, symbol_name: &str) -> anyhow::Result<Vec<SearchResult>> {
        let conn = self.conn.lock().await;

        let mut stmt = conn.prepare(
            "SELECT s.name, s.file_path, s.kind, s.line_start, s.line_end, c.content
             FROM symbols s
             LEFT JOIN code_chunks c ON s.file_path = c.file_path
                 AND s.line_start BETWEEN c.start_line AND c.end_line
             WHERE s.name LIKE ?1
             LIMIT 20"
        )?;

        let rows = stmt.query_map(params![format!("%{}%", symbol_name)], |row| {
            Ok(SearchResult {
                file_path: row.get::<_, String>(1)?,
                content: row.get::<_, String>(5).unwrap_or_default(),
                start_line: row.get::<_, i64>(3)? as usize,
                end_line: row.get::<_, i64>(4)? as usize,
                relevance_score: 10.0,
                context: format!("{}: {}", row.get::<_, String>(2)?, row.get::<_, String>(0)?),
            })
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }

        Ok(results)
    }

    pub async fn get_index_stats(&self) -> anyhow::Result<IndexStats> {
        let conn = self.conn.lock().await;

        let total_files: i64 = conn.query_row("SELECT COUNT(*) FROM file_metadata", [], |row| row.get(0))?;
        let total_chunks: i64 = conn.query_row("SELECT COUNT(*) FROM code_chunks", [], |row| row.get(0))?;
        let total_symbols: i64 = conn.query_row("SELECT COUNT(*) FROM symbols", [], |row| row.get(0))?;

        let mut stmt = conn.prepare("SELECT language, COUNT(*) FROM code_chunks GROUP BY language")?;
        let languages: HashMap<String, usize> = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)? as usize))
        })?.filter_map(|r| r.ok()).collect();

        let last_indexed: Option<String> = conn.query_row(
            "SELECT value FROM index_metadata WHERE key = 'last_indexed'",
            [],
            |row| row.get(0)
        ).optional()?;

        let index_size: i64 = conn.query_row(
            "SELECT SUM(length(content)) FROM code_chunks",
            [],
            |row| row.get(0)
        ).optional()?.unwrap_or(0);

        Ok(IndexStats {
            total_files: total_files as usize,
            total_chunks: total_chunks as usize,
            total_symbols: total_symbols as usize,
            languages,
            last_indexed: last_indexed.and_then(|t| t.parse().ok()),
            index_size_bytes: index_size as usize,
        })
    }

    pub async fn get_file_chunks(&self, file_path: &str) -> anyhow::Result<Vec<CodeChunk>> {
        let conn = self.conn.lock().await;

        let mut stmt = conn.prepare(
            "SELECT id, file_path, content, start_line, end_line, language, symbols, timestamp
             FROM code_chunks
             WHERE file_path = ?1
             ORDER BY start_line"
        )?;

        let rows = stmt.query_map(params![file_path], |row| {
            let symbols_json: String = row.get(6)?;
            let symbols: Vec<String> = serde_json::from_str(&symbols_json).unwrap_or_default();

            Ok(CodeChunk {
                id: row.get(0)?,
                file_path: row.get(1)?,
                content: row.get(2)?,
                start_line: row.get::<_, i64>(3)? as usize,
                end_line: row.get::<_, i64>(4)? as usize,
                language: row.get(5)?,
                symbols,
                embedding: None,
                timestamp: row.get::<_, i64>(7)? as u64,
            })
        })?;

        let mut chunks = Vec::new();
        for row in rows {
            chunks.push(row?);
        }

        Ok(chunks)
    }

    // ── Utility Functions ─────────────────────────────────────────────────

    fn compute_hash(content: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    fn extract_context(content: &str, query: &str) -> String {
        let lines: Vec<&str> = content.lines().collect();
        
        // Find the line with the best match
        let mut best_line = 0;
        let mut best_score = 0;
        
        for (i, line) in lines.iter().enumerate() {
            let line_lower = line.to_lowercase();
            let score = line_lower.matches(query).count();
            if score > best_score {
                best_score = score;
                best_line = i;
            }
        }

        // Get context around the best line (5 lines before and after)
        let start = best_line.saturating_sub(5);
        let end = (best_line + 6).min(lines.len());
        
        lines[start..end].join("\n")
    }

    pub fn get_db_path(&self) -> &Path {
        &self.db_path
    }

    pub fn get_indexing_progress(&self) -> IndexingProgress {
        self.indexing_progress.read().unwrap().clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolInfo {
    pub name: String,
    pub file_path: String,
    pub kind: String,
    pub start_line: usize,
    pub end_line: usize,
}
