//! In-memory flat ANN — top-k cosine search over code chunk embeddings.
//! Scoring runs on the Apple Neural Engine when available (batched dot products
//! via ane_inference), with a CPU fallback. Embeddings are pre-normalized into a
//! cache so both paths reduce to plain dot products.
//! (turbovec submodule available for future upgrade; flat index ships today.)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Minimum entries before ANE dispatch is worth the packing overhead.
const ANE_MIN_BATCH: usize = 32;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct AnnSnapshot {
    entries: HashMap<String, Vec<f32>>,
}

/// Query-independent search cache, rebuilt lazily after upsert/load/clear.
struct SearchCache {
    ids: Vec<String>,
    /// Unit-normalized embeddings (dot == cosine).
    normalized: Vec<Vec<f32>>,
    /// Pre-packed ANE input buffers (Some only when the index is uniform-dim,
    /// large enough, and the ANE kernel compiled). Packing once here moves the
    /// f32→f16 conversion of the whole index out of the per-query hot path.
    ane_batches: Option<Vec<crate::ane_inference::PreparedSimBatch>>,
}

pub struct AnnIndex {
    base_dir: PathBuf,
    entries: std::sync::Mutex<HashMap<String, Vec<f32>>>,
    /// None = dirty, rebuilt on next search.
    norm_cache: std::sync::Mutex<Option<SearchCache>>,
}

impl AnnIndex {
    pub fn new(base_dir: PathBuf) -> Self {
        Self {
            base_dir,
            entries: std::sync::Mutex::new(HashMap::new()),
            norm_cache: std::sync::Mutex::new(None),
        }
    }

    fn snapshot_path(&self) -> PathBuf {
        self.base_dir.join("ann_flat.json")
    }

    pub fn load(&self) -> Result<(), String> {
        if !self.snapshot_path().exists() {
            return Ok(());
        }
        let raw = std::fs::read_to_string(self.snapshot_path()).map_err(|e| e.to_string())?;
        let snap: AnnSnapshot = serde_json::from_str(&raw).unwrap_or_default();
        *self.entries.lock().map_err(|e| e.to_string())? = snap.entries;
        self.invalidate_cache();
        Ok(())
    }

    pub fn clear(&self) -> Result<(), String> {
        self.entries.lock().map_err(|e| e.to_string())?.clear();
        self.invalidate_cache();
        let _ = std::fs::remove_file(self.snapshot_path());
        Ok(())
    }

    pub fn upsert(&self, chunk_id: &str, embedding: &[f32]) -> Result<(), String> {
        if embedding.is_empty() {
            return Ok(());
        }
        self.entries
            .lock()
            .map_err(|e| e.to_string())?
            .insert(chunk_id.to_string(), embedding.to_vec());
        self.invalidate_cache();
        self.persist()
    }

    fn invalidate_cache(&self) {
        if let Ok(mut cache) = self.norm_cache.lock() {
            *cache = None;
        }
    }

    fn normalize(v: &[f32]) -> Option<Vec<f32>> {
        let norm = v.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm <= f32::EPSILON {
            return None;
        }
        Some(v.iter().map(|x| x / norm).collect())
    }

    pub fn search(&self, query: &[f32], k: usize) -> Result<Vec<(String, f32)>, String> {
        if query.is_empty() {
            return Ok(Vec::new());
        }
        let query = match Self::normalize(query) {
            Some(q) => q,
            None => return Ok(Vec::new()),
        };

        // Rebuild the query-independent cache if dirty.
        let mut cache_guard = self.norm_cache.lock().map_err(|e| e.to_string())?;
        if cache_guard.is_none() {
            let map = self.entries.lock().map_err(|e| e.to_string())?;
            let mut ids = Vec::with_capacity(map.len());
            let mut normalized = Vec::with_capacity(map.len());
            for (id, emb) in map.iter() {
                if let Some(n) = Self::normalize(emb) {
                    ids.push(id.clone());
                    normalized.push(n);
                }
            }
            // Pre-pack ANE buffers when the index is uniform-dim and large
            // enough for the dispatch to beat CPU.
            let uniform = normalized
                .first()
                .map(|f| normalized.iter().all(|v| v.len() == f.len()))
                .unwrap_or(false);
            let ane_batches = if uniform && normalized.len() >= ANE_MIN_BATCH {
                crate::ane_inference::global().and_then(|opt| {
                    let refs: Vec<&[f32]> = normalized.iter().map(|v| v.as_slice()).collect();
                    opt.prepare_sim_batches(&refs)
                })
            } else {
                None
            };
            *cache_guard = Some(SearchCache {
                ids,
                normalized,
                ane_batches,
            });
        }
        let cache = cache_guard.as_mut().expect("cache rebuilt above");

        // ANE path: batched dot products on the Neural Engine, freeing CPU/GPU
        // for Ollama. Falls back to CPU on any failure or dim mismatch.
        let ane_scores: Option<Vec<f32>> = cache.ane_batches.as_mut().and_then(|batches| {
            let opt = crate::ane_inference::global()?;
            opt.similarity_prepared(&query, batches)
        });

        let mut scored: Vec<(String, f32)> = match ane_scores {
            Some(scores) => cache
                .ids
                .iter()
                .zip(scores)
                .filter(|(_, s)| *s > 0.2)
                .map(|(id, s)| (id.clone(), s))
                .collect(),
            None => cache
                .ids
                .iter()
                .zip(cache.normalized.iter())
                .filter(|(_, emb)| emb.len() == query.len())
                .map(|(id, emb)| {
                    let s: f32 = query.iter().zip(emb.iter()).map(|(a, b)| a * b).sum();
                    (id.clone(), s)
                })
                .filter(|(_, s)| *s > 0.2)
                .collect(),
        };
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(k);
        Ok(scored)
    }

    fn persist(&self) -> Result<(), String> {
        let map = self.entries.lock().map_err(|e| e.to_string())?;
        if let Some(parent) = self.snapshot_path().parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let snap = AnnSnapshot {
            entries: map.clone(),
        };
        std::fs::write(
            &self.snapshot_path(),
            serde_json::to_string(&snap).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())
    }

    pub fn len(&self) -> usize {
        self.entries.lock().ok().map(|m| m.len()).unwrap_or(0)
    }
}

pub fn stable_id(chunk_id: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    chunk_id.hash(&mut h);
    h.finish()
}
