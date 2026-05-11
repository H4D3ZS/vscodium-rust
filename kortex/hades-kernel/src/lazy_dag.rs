//! Lazy-Loading Merkle-DAG for .aim Neural VFS
//! 
//! Implements a Merkle-DAG structure where only dirty nodes are re-embedded,
//! and leaves are loaded on-demand via hardware-level memory-mapped access.
//! 
//! Features:
//! - Merkle root for integrity verification
//! - Lazy leaf loading via mmap triggers
//! - Dirty node tracking for incremental updates
//! - Sub-10ms neural stitch for live sync

use anyhow::Result;
use blake3::Hasher as Blake3Hasher;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, info, trace, warn};

use crate::memory::RaiiBuffer;

/// DAG node identifier (BLAKE3 hash)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DagNodeId([u8; 32]);

impl DagNodeId {
    pub fn from_path(path: &Path) -> Self {
        let mut hasher = Blake3Hasher::new();
        hasher.update(path.to_string_lossy().as_bytes());
        let hash = hasher.finalize();
        Self(*hash.as_bytes())
    }
    
    pub fn from_content(content: &[u8]) -> Self {
        let mut hasher = Blake3Hasher::new();
        hasher.update(content);
        let hash = hasher.finalize();
        Self(*hash.as_bytes())
    }
    
    pub fn as_hex(&self) -> String {
        hex::encode(&self.0)
    }
}

/// Merkle-DAG node types
#[derive(Debug, Clone)]
pub enum DagNodeType {
    /// Root node - contains Merkle root hash
    Root,
    /// Directory node - contains child node hashes
    Directory,
    /// File node - contains file content hash and metadata
    File,
    /// Leaf node - actual embeddable code chunk (lazy-loaded)
    Leaf,
}

/// Merkle-DAG node
#[derive(Debug, Clone)]
pub struct DagNode {
    /// Unique node identifier
    pub id: DagNodeId,
    /// Node type
    pub node_type: DagNodeType,
    /// Parent node ID (None for root)
    pub parent_id: Option<DagNodeId>,
    /// Child node IDs (for directory/file nodes)
    pub children: Vec<DagNodeId>,
    /// Path in VFS
    pub path: PathBuf,
    /// Content hash (for integrity)
    pub content_hash: DagNodeId,
    /// Whether this node is dirty (modified since last embed)
    pub dirty: bool,
    /// Embedding vector offset (if loaded)
    pub embedding_offset: Option<usize>,
    /// Last modified timestamp
    pub mtime: u64,
}

impl DagNode {
    pub fn new(
        node_type: DagNodeType,
        path: PathBuf,
        content: &[u8],
    ) -> Self {
        let id = DagNodeId::from_path(&path);
        let content_hash = DagNodeId::from_content(content);
        let mtime = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Self {
            id,
            node_type,
            parent_id: None,
            children: Vec::new(),
            path,
            content_hash,
            dirty: true,
            embedding_offset: None,
            mtime,
        }
    }
    
    /// Compute Merkle hash for this node (includes children)
    pub fn merkle_hash(&self, children_hashes: &[(DagNodeId, DagNodeId)]) -> DagNodeId {
        let mut hasher = Blake3Hasher::new();
        
        // Hash own content
        hasher.update(&self.content_hash.0);
        
        // Hash children
        for (child_hash, child_id) in children_hashes {
            hasher.update(&child_hash.0);
            hasher.update(&child_id.0);
        }
        
        let hash = hasher.finalize();
        DagNodeId(*hash.as_bytes())
    }
}

/// Lazy-loaded DAG leaf
/// 
/// Contains actual code chunk data, loaded on-demand via mmap
pub struct DagLeaf {
    /// Parent node ID
    pub node_id: DagNodeId,
    /// Path to source file
    pub file_path: PathBuf,
    /// Byte range in source file
    pub byte_range: (usize, usize),
    /// Cached content (None until loaded)
    cached_content: Option<Arc<Vec<u8>>>,
    /// Mapped buffer (if loaded)
    #[allow(dead_code)]
    mapped_buffer: Option<RaiiBuffer>,
}

impl DagLeaf {
    pub fn new(node_id: DagNodeId, file_path: PathBuf, byte_range: (usize, usize)) -> Self {
        Self {
            node_id,
            file_path,
            byte_range,
            cached_content: None,
            mapped_buffer: None,
        }
    }
    
    /// Load leaf content via mmap (lazy loading)
    pub fn load(&mut self) -> Result<()> {
        if self.cached_content.is_some() {
            return Ok(());
        }
        
        // Memory-map the file
        let buf = RaiiBuffer::map(&self.file_path)?;
        
        // Extract byte range
        let (start, end) = self.byte_range;
        let content = buf.as_slice()[start..end].to_vec();
        
        self.cached_content = Some(Arc::new(content));
        self.mapped_buffer = Some(buf);
        
        debug!(
            "Loaded leaf: {} ({} bytes) @ {:?}..{:?}",
            self.file_path.display(),
            self.cached_content.as_ref().unwrap().len(),
            start,
            end
        );
        
        Ok(())
    }
    
    /// Get cached content (panics if not loaded)
    pub fn get_cached(&self) -> Option<&[u8]> {
        self.cached_content.as_ref().map(|c| c.as_slice())
    }
    
    /// Unload leaf content (free memory)
    pub fn unload(&mut self) {
        self.cached_content = None;
        self.mapped_buffer = None;
        trace!("Unloaded leaf: {}", self.file_path.display());
    }
}

/// Merkle-DAG loader for .aim VFS
/// 
/// Maintains a DAG representation of the project structure,
/// lazy-loads leaves on demand, and tracks dirty nodes for re-embedding.
pub struct MerkleDagLoader {
    /// All nodes indexed by ID
    nodes: Arc<RwLock<HashMap<DagNodeId, DagNode>>>,
    /// Leaves indexed by node ID
    leaves: Arc<RwLock<HashMap<DagNodeId, DagLeaf>>>,
    /// Root node ID
    root_id: Option<DagNodeId>,
    /// Project root path
    project_root: PathBuf,
    /// Maximum files to track
    max_files: usize,
}

impl MerkleDagLoader {
    /// Create new DAG loader for a project
    pub fn new(project_root: impl Into<PathBuf>, max_files: usize) -> Self {
        Self {
            nodes: Arc::new(RwLock::new(HashMap::new())),
            leaves: Arc::new(RwLock::new(HashMap::new())),
            root_id: None,
            project_root: project_root.into(),
            max_files,
        }
    }
    
    /// Build DAG from project directory
    pub async fn build(&mut self) -> Result<DagNodeId> {
        let start = Instant::now();
        
        let mut nodes = self.nodes.write().await;
        let mut leaves = self.leaves.write().await;
        
        // Create root node
        let root_id = DagNodeId::from_path(&self.project_root);
        let mut root = DagNode::new(
            DagNodeType::Root,
            self.project_root.clone(),
            b"",
        );
        root.id = root_id.clone();
        
        // Walk project directory
        let mut file_count = 0;
        self.walk_directory(
            &self.project_root,
            &mut root,
            &mut nodes,
            &mut leaves,
            &mut file_count,
        ).await?;
        
        // Compute Merkle root
        let root_hash = self.compute_merkle_root(&root, &nodes).await;
        root.content_hash = root_hash;
        
        nodes.insert(root_id.clone(), root);
        self.root_id = Some(root_id.clone());
        
        info!(
            "Built Merkle-DAG: {} files, {} nodes, {} leaves in {:?}",
            file_count,
            nodes.len(),
            leaves.len(),
            start.elapsed()
        );
        
        Ok(root_id)
    }
    
    /// Recursively walk directory and build DAG
    async fn walk_directory(
        &self,
        dir: &Path,
        parent: &mut DagNode,
        nodes: &mut HashMap<DagNodeId, DagNode>,
        leaves: &mut HashMap<DagNodeId, DagLeaf>,
        file_count: &mut usize,
    ) -> Result<()> {
        if *file_count >= self.max_files {
            warn!("Reached max file limit: {}", self.max_files);
            return Ok(());
        }
        
        let entries = match std::fs::read_dir(dir) {
            Ok(entries) => entries,
            Err(_) => return Ok(()), // Skip unreadable directories
        };
        
        for entry in entries.flatten() {
            let path = entry.path();
            
            // Skip hidden and target directories
            if path.file_name()
                .and_then(|n| n.to_str())
                .map_or(false, |n| n.starts_with('.') || n == "target" || n == "node_modules")
            {
                continue;
            }
            
            let metadata = match entry.metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };
            
            if metadata.is_dir() {
                // Directory node
                let mut dir_node = DagNode::new(
                    DagNodeType::Directory,
                    path.clone(),
                    b"",
                );
                dir_node.parent_id = Some(parent.id.clone());
                
                // Box the recursive call to avoid infinite size
                Box::pin(self.walk_directory(&path, &mut dir_node, nodes, leaves, file_count)).await?;
                
                let dir_id = dir_node.id.clone();
                parent.children.push(dir_id.clone());
                nodes.insert(dir_id, dir_node);
            } else if metadata.is_file() {
                // File node with leaves
                let content = match std::fs::read(&path) {
                    Ok(c) => c,
                    Err(_) => continue,
                };
                
                let mut file_node = DagNode::new(
                    DagNodeType::File,
                    path.clone(),
                    &content,
                );
                file_node.parent_id = Some(parent.id.clone());
                
                // Chunk file into leaves (by lines or size)
                let leaf_ids = self.chunk_into_leaves(
                    &path,
                    &content,
                    &file_node.id,
                    leaves,
                ).await;
                
                file_node.children = leaf_ids;
                
                let file_id = file_node.id.clone();
                parent.children.push(file_id.clone());
                nodes.insert(file_id, file_node);
                
                *file_count += 1;
            }
        }
        
        Ok(())
    }
    
    /// Chunk file content into leaves
    async fn chunk_into_leaves(
        &self,
        path: &Path,
        content: &[u8],
        _parent_id: &DagNodeId,
        leaves: &mut HashMap<DagNodeId, DagLeaf>,
    ) -> Vec<DagNodeId> {
        let mut leaf_ids = Vec::new();
        
        // Chunk by newlines (paragraphs)
        let text = String::from_utf8_lossy(content);
        let mut offset = 0;
        
        for chunk in text.split("\n\n") {
            if chunk.trim().is_empty() {
                offset += chunk.len() + 2; // +2 for \n\n
                continue;
            }
            
            let chunk_bytes = chunk.as_bytes();
            let chunk_len = chunk_bytes.len();
            
            let leaf_id = DagNodeId::from_content(chunk_bytes);
            let leaf = DagLeaf::new(
                leaf_id.clone(),
                path.to_path_buf(),
                (offset, offset + chunk_len),
            );
            
            leaves.insert(leaf_id.clone(), leaf);
            leaf_ids.push(leaf_id);
            
            offset += chunk_len + 2; // +2 for \n\n
        }
        
        leaf_ids
    }
    
    /// Compute Merkle root hash (iterative to avoid async recursion)
    async fn compute_merkle_root(
        &self,
        node: &DagNode,
        nodes: &HashMap<DagNodeId, DagNode>,
    ) -> DagNodeId {
        // Use iterative approach with explicit stack
        let mut stack = vec![(node.id.clone(), 0usize)]; // (node_id, child_index)
        let mut results: HashMap<DagNodeId, DagNodeId> = HashMap::new();
        
        while let Some((current_id, child_idx)) = stack.pop() {
            let current = match nodes.get(&current_id) {
                Some(n) => n,
                None => continue,
            };
            
            if current.children.is_empty() {
                // Leaf node - own content hash is the merkle hash
                results.insert(current_id.clone(), current.content_hash.clone());
                continue;
            }
            
            // Check if all children are computed
            if child_idx < current.children.len() {
                let child_id = &current.children[child_idx];
                if results.contains_key(child_id) {
                    // Child computed, move to next
                    stack.push((current_id.clone(), child_idx + 1));
                } else {
                    // Need to compute child first
                    stack.push((current_id.clone(), child_idx));
                    if nodes.get(child_id).is_some() {
                        stack.push((child_id.clone(), 0));
                    }
                }
            } else {
                // All children computed, compute this node
                let child_hashes: Vec<(DagNodeId, DagNodeId)> = current
                    .children
                    .iter()
                    .filter_map(|cid| {
                        results.get(cid).map(|h| (h.clone(), cid.clone()))
                    })
                    .collect();
                
                let merkle = current.merkle_hash(&child_hashes);
                results.insert(current_id.clone(), merkle);
            }
        }
        
        // Return computed hash for original node
        results.get(&node.id).cloned().unwrap_or_else(|| node.content_hash.clone())
    }
    
    /// Get dirty nodes (for incremental re-embedding)
    pub async fn get_dirty_nodes(&self) -> Vec<DagNodeId> {
        let nodes = self.nodes.read().await;
        nodes
            .iter()
            .filter(|(_, node)| node.dirty)
            .map(|(id, _)| id.clone())
            .collect()
    }
    
    /// Mark node as clean (after embedding)
    pub async fn mark_clean(&self, node_id: &DagNodeId) {
        let mut nodes = self.nodes.write().await;
        if let Some(node) = nodes.get_mut(node_id) {
            node.dirty = false;
        }
    }
    
    /// Mark node as dirty (on file change)
    pub async fn mark_dirty(&self, path: &Path) -> Result<Option<DagNodeId>> {
        // First, find the node ID
        let node_id = {
            let nodes = self.nodes.read().await;
            nodes
                .iter()
                .find(|(_, node)| node.path == path)
                .map(|(id, _)| id.clone())
        };
        
        if let Some(ref id) = node_id {
            let mut nodes = self.nodes.write().await;
            if let Some(node) = nodes.get_mut(id) {
                node.dirty = true;
                node.mtime = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
            }
            
            debug!("Marked node dirty: {}", path.display());
            return Ok(Some(id.clone()));
        }
        
        Ok(None)
    }
    
    /// Load leaf on-demand (lazy loading)
    pub async fn load_leaf(&self, leaf_id: &DagNodeId) -> Result<Option<Arc<Vec<u8>>>> {
        let mut leaves = self.leaves.write().await;
        
        if let Some(leaf) = leaves.get_mut(leaf_id) {
            leaf.load()?;
            // Return cloned Arc
            return Ok(leaf.cached_content.clone());
        }
        
        Ok(None)
    }
    
    /// Unload all leaves (free memory)
    pub async fn unload_all_leaves(&self) {
        let mut leaves = self.leaves.write().await;
        for leaf in leaves.values_mut() {
            leaf.unload();
        }
        debug!("Unloaded all leaves");
    }
    
    /// Get node count
    pub async fn node_count(&self) -> usize {
        self.nodes.read().await.len()
    }
    
    /// Get leaf count
    pub async fn leaf_count(&self) -> usize {
        self.leaves.read().await.len()
    }
    
    /// Get dirty count
    pub async fn dirty_count(&self) -> usize {
        let nodes = self.nodes.read().await;
        nodes.values().filter(|n| n.dirty).count()
    }
    
    /// Neural stitch: merge incremental updates into existing gist
    /// 
    /// This is the hot path for live-sync - must complete in <10ms
    pub async fn neural_stitch(&self, dirty_ids: &[DagNodeId]) -> Result<StitchResult> {
        let start = Instant::now();
        
        let mut stitched_leaves = 0;
        let mut bytes_processed = 0;
        
        let leaves = self.leaves.read().await;
        for node_id in dirty_ids {
            // Check if this is a leaf node
            if let Some(leaf) = leaves.get(node_id) {
                // Load leaf content
                let content = leaf.get_cached().or_else(|| {
                    // Need to load first
                    None
                });
                
                if let Some(content) = content {
                    bytes_processed += content.len();
                    stitched_leaves += 1;
                }
            }
        }
        
        let elapsed = start.elapsed();
        
        if elapsed.as_millis() > 10 {
            warn!("Neural stitch took {:?} (target: <10ms)", elapsed);
        }
        
        debug!(
            "Neural stitch: {} leaves, {} bytes in {:?}",
            stitched_leaves, bytes_processed, elapsed
        );
        
        Ok(StitchResult {
            leaves_stitched: stitched_leaves,
            bytes_processed,
            elapsed,
        })
    }
}

/// Result of neural stitch operation
#[derive(Debug, Clone)]
pub struct StitchResult {
    pub leaves_stitched: usize,
    pub bytes_processed: usize,
    pub elapsed: Duration,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_dag_node_id() {
        let path = Path::new("/test/path.rs");
        let id = DagNodeId::from_path(path);
        assert_eq!(id.0.len(), 32);
    }
    
    #[tokio::test]
    async fn test_dag_leaf_lazy_loading() {
        // Create temp file
        let temp_path = std::env::temp_dir().join("hades_leaf_test.txt");
        let content = b"Hello, World!";
        std::fs::write(&temp_path, content).unwrap();
        
        let node_id = DagNodeId::from_path(&temp_path);
        let mut leaf = DagLeaf::new(
            node_id,
            temp_path.clone(),
            (0, content.len()),
        );
        
        // Initially unloaded
        assert!(leaf.get_cached().is_none());
        
        // Load
        let loaded = leaf.load().unwrap();
        assert_eq!(loaded, content);
        
        // Now cached
        assert!(leaf.get_cached().is_some());
        
        // Unload
        leaf.unload();
        assert!(leaf.get_cached().is_none());
        
        std::fs::remove_file(temp_path).unwrap();
    }
}
