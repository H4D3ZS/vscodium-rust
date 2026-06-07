//! In-memory flat ANN — top-k cosine search over code chunk embeddings.
//! (turbovec submodule available for future upgrade; flat index ships today.)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct AnnSnapshot {
    entries: HashMap<String, Vec<f32>>,
}

pub struct AnnIndex {
    base_dir: PathBuf,
    entries: std::sync::Mutex<HashMap<String, Vec<f32>>>,
}

impl AnnIndex {
    pub fn new(base_dir: PathBuf) -> Self {
        Self {
            base_dir,
            entries: std::sync::Mutex::new(HashMap::new()),
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
        Ok(())
    }

    pub fn clear(&self) -> Result<(), String> {
        self.entries.lock().map_err(|e| e.to_string())?.clear();
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
        self.persist()
    }

    pub fn search(&self, query: &[f32], k: usize) -> Result<Vec<(String, f32)>, String> {
        if query.is_empty() {
            return Ok(Vec::new());
        }
        let map = self.entries.lock().map_err(|e| e.to_string())?;
        let mut scored: Vec<(String, f32)> = map
            .iter()
            .filter_map(|(id, emb)| {
                if emb.len() != query.len() {
                    return None;
                }
                Some((id.clone(), crate::embeddings::cosine_similarity(query, emb)))
            })
            .filter(|(_, s)| *s > 0.2)
            .collect();
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
