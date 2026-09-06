/// ANE aux-offload — honest Apple Neural Engine integration.
///
/// Token generation is done by the local backend on the Metal GPU and is memory-bandwidth
/// bound (~45 tok/s ceiling for a 2b Q4 model on M1's ~68GB/s). The ANE cannot
/// reach into the local backend's process, so it is NOT used for token generation.
/// Instead it owns auxiliary workloads — batched cosine similarity for the
/// vector index — freeing CPU/GPU cycles for the native /api path while a stream is active.

use crate::ane::{AneEngine, f32_to_f16, f16_to_f32};
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};

/// Embeddings scored per ANE dispatch (kernel is compiled once for this width).
const SIM_BATCH: usize = 256;
/// Sequence width of the kernel. The ANE rejects matmuls with seq < 32
/// (tile granularity — verified by probe: seq=1..16 fail at eval, 32+ work).
/// We use column 0 for the query; columns 1..31 are padding.
const SIM_SEQ: usize = 32;

/// Process-wide handle so sync callers (ann_index) can reach the optimizer
/// without plumbing Tauri state through the indexing layer.
static GLOBAL: std::sync::OnceLock<Arc<AneInferenceOptimizer>> = std::sync::OnceLock::new();

pub fn set_global(optimizer: Arc<AneInferenceOptimizer>) {
    let _ = GLOBAL.set(optimizer);
}

pub fn global() -> Option<&'static Arc<AneInferenceOptimizer>> {
    GLOBAL.get()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AneStatus {
    pub available: bool,
    pub m1_or_newer: bool,
    pub model: String, // M1, M2, M3, M4, etc.
    /// "ane_aux_offload" (similarity kernel live), "metal_decode" (generation
    /// path, ANE idle), or "unavailable" (not Apple Silicon).
    pub inference_mode: String,
    /// Token-generation speedup vs the local backend alone. Always 1.0 — the ANE does not
    /// (and cannot) accelerate the local backend's decode loop; decode is bandwidth-bound.
    pub estimated_speedup: f32,
    /// Measured tok/s from real streams via update_status(). 0.0 = no sample yet.
    pub tokens_per_sec_estimate: f32,
}

/// Compiled ANE matmul kernel for batched dot products: [1, dim] @ [dim, SIM_BATCH].
struct SimilarityKernel {
    engine: AneEngine,
    dim: usize,
}

/// A pre-packed ANE input buffer: weight columns hold fp16 embeddings, query
/// columns are rewritten per search. Cache these (ann_index does) so the
/// f32→f16 conversion of the whole index happens once, not per query.
pub struct PreparedSimBatch {
    packed: Vec<u8>,
    count: usize,
    dim: usize,
}

pub struct AneInferenceOptimizer {
    available: Arc<Mutex<bool>>,
    status: Arc<Mutex<AneStatus>>,
    // std Mutex (not tokio): similarity scoring is called from sync contexts (ann_index).
    sim_kernel: std::sync::Mutex<Option<SimilarityKernel>>,
}

impl AneInferenceOptimizer {
    pub fn new() -> Self {
        let available = Self::check_ane_availability();
        let model = Self::detect_apple_silicon_model();

        let status = AneStatus {
            available,
            m1_or_newer: available,
            model: model.clone(),
            inference_mode: if available {
                "metal_decode".to_string() // generation runs on Metal; ANE idle until aux kernel init
            } else {
                "unavailable".to_string()
            },
            estimated_speedup: 1.0,        // honest: ANE never speeds up local decode
            tokens_per_sec_estimate: 0.0,  // populated from measured streams only
        };

        Self {
            available: Arc::new(Mutex::new(available)),
            status: Arc::new(Mutex::new(status)),
            sim_kernel: std::sync::Mutex::new(None),
        }
    }

    /// Detect if ANE is available (macOS + Apple Silicon)
    #[cfg(target_os = "macos")]
    fn check_ane_availability() -> bool {
        // Check if we're on Apple Silicon via sysctl
        use std::process::Command;

        if let Ok(output) = Command::new("sysctl")
            .arg("-n")
            .arg("machdep.cpu.brand_string")
            .output()
        {
            let brand = String::from_utf8_lossy(&output.stdout);
            brand.contains("Apple") || brand.contains("M1") || brand.contains("M2")
                || brand.contains("M3") || brand.contains("M4")
        } else {
            false
        }
    }

    #[cfg(not(target_os = "macos"))]
    fn check_ane_availability() -> bool {
        false
    }

    /// Detect specific Apple Silicon model
    #[cfg(target_os = "macos")]
    fn detect_apple_silicon_model() -> String {
        use std::process::Command;

        if let Ok(output) = Command::new("sysctl")
            .arg("-n")
            .arg("machdep.cpu.brand_string")
            .output()
        {
            let brand = String::from_utf8_lossy(&output.stdout);
            let brand_str = brand.trim();

            if brand_str.contains("M4") { "M4".to_string() }
            else if brand_str.contains("M3") { "M3".to_string() }
            else if brand_str.contains("M2") { "M2".to_string() }
            else if brand_str.contains("M1") { "M1".to_string() }
            else { "Unknown Apple Silicon".to_string() }
        } else {
            "Unknown".to_string()
        }
    }

    #[cfg(not(target_os = "macos"))]
    fn detect_apple_silicon_model() -> String {
        "N/A (not macOS)".to_string()
    }

    /// Get current ANE status
    pub async fn get_status(&self) -> AneStatus {
        self.status.lock().await.clone()
    }

    /// Initialize the ANE aux-offload kernel (batched similarity for the vector
    /// index). `dim` is the embedding dimension (768 for nomic-embed-text).
    pub async fn init_aux_offload(&self, dim: usize) -> Result<(), String> {
        if !*self.available.lock().await {
            return Err("ANE not available on this hardware".to_string());
        }

        if !self.ensure_kernel(dim) {
            return Err("ANE similarity kernel compile failed".to_string());
        }

        let mut status = self.status.lock().await;
        status.inference_mode = "ane_aux_offload".to_string();
        Ok(())
    }

    /// Compile (or reuse) the similarity kernel for `dim`-wide embeddings.
    /// Sync so ann_index can lazy-init on first search. Returns false on any failure.
    fn ensure_kernel(&self, dim: usize) -> bool {
        if dim == 0 {
            return false;
        }
        let mut guard = match self.sim_kernel.lock() {
            Ok(g) => g,
            Err(_) => return false,
        };
        if let Some(k) = guard.as_ref() {
            if k.dim == dim {
                return true;
            }
        }

        // y = x @ W: input packs [SIM_SEQ query columns | SIM_BATCH embedding
        // columns], output is [SIM_BATCH, SIM_SEQ] dot products (we use column 0).
        // I/O is fp16 (2 bytes/elem) — the ANE compiler rejects fp32 I/O.
        let mil = AneEngine::gen_dyn_matmul_mil(dim, SIM_BATCH, SIM_SEQ);
        let in_bytes = dim * (SIM_SEQ + SIM_BATCH) * 2;
        let out_bytes = SIM_BATCH * SIM_SEQ * 2;

        // Empty weights: the dyn kernel takes weights as input, not as a blob.
        match AneEngine::new(&mil, &[], &[in_bytes], &[out_bytes]) {
            Ok(engine) => {
                *guard = Some(SimilarityKernel { engine, dim });
                true
            }
            Err(e) => {
                eprintln!("[ANE] similarity kernel compile failed: {}", e);
                false
            }
        }
    }

    /// Pre-pack embeddings into ANE input buffers (weight columns only; query
    /// columns are written per search by similarity_prepared). Pre-packing once
    /// moves the f32→f16 conversion cost out of the search hot path.
    pub fn prepare_sim_batches(&self, embeddings: &[&[f32]]) -> Option<Vec<PreparedSimBatch>> {
        let dim = embeddings.first()?.len();
        if dim == 0 || !self.ensure_kernel(dim) {
            return None;
        }
        let span = SIM_SEQ + SIM_BATCH;
        let mut batches = Vec::with_capacity(embeddings.len().div_ceil(SIM_BATCH));
        for chunk in embeddings.chunks(SIM_BATCH) {
            let mut packed = vec![0u8; dim * span * 2];
            for c in 0..dim {
                let row = c * span * 2;
                for (o, emb) in chunk.iter().enumerate() {
                    if emb.len() != dim {
                        return None;
                    }
                    let at = row + (SIM_SEQ + o) * 2;
                    packed[at..at + 2].copy_from_slice(&f32_to_f16(emb[c]).to_le_bytes());
                }
            }
            batches.push(PreparedSimBatch {
                packed,
                count: chunk.len(),
                dim,
            });
        }
        Some(batches)
    }

    /// Score a query against pre-packed batches: only the query column is
    /// rewritten (dim fp16 writes per batch), then the ANE does the matmul.
    pub fn similarity_prepared(
        &self,
        query: &[f32],
        batches: &mut [PreparedSimBatch],
    ) -> Option<Vec<f32>> {
        let dim = query.len();
        if dim == 0 || batches.is_empty() || !self.ensure_kernel(dim) {
            return None;
        }
        let guard = self.sim_kernel.lock().ok()?;
        let kernel = guard.as_ref()?;
        if kernel.dim != dim {
            return None;
        }

        let span = SIM_SEQ + SIM_BATCH;
        let query_f16: Vec<u16> = query.iter().map(|&v| f32_to_f16(v)).collect();
        let mut scores = Vec::new();

        for batch in batches.iter_mut() {
            if batch.dim != dim {
                return None;
            }
            for c in 0..dim {
                let row = c * span * 2;
                batch.packed[row..row + 2].copy_from_slice(&query_f16[c].to_le_bytes());
            }
            let mut outputs = kernel
                .engine
                .execute(&[batch.packed.as_slice()], &[SIM_BATCH * SIM_SEQ * 2])
                .ok()?;
            let bytes = outputs.pop()?;
            for o in 0..batch.count {
                let at = o * SIM_SEQ * 2;
                let h = u16::from_le_bytes([bytes[at], bytes[at + 1]]);
                scores.push(f16_to_f32(h));
            }
        }

        Some(scores)
    }

    /// Batched dot products on the ANE. Query and embeddings must be
    /// pre-normalized (so dot == cosine). Returns None on any failure —
    /// callers fall back to the CPU path.
    pub fn similarity_batch(&self, query: &[f32], embeddings: &[&[f32]]) -> Option<Vec<f32>> {
        let dim = query.len();
        if dim == 0 || embeddings.is_empty() {
            return None;
        }
        if !self.ensure_kernel(dim) {
            return None;
        }
        let guard = self.sim_kernel.lock().ok()?;
        let kernel = guard.as_ref()?;

        let span = SIM_SEQ + SIM_BATCH; // last-dim width: query columns + weight columns
        let mut scores = Vec::with_capacity(embeddings.len());

        let query_f16: Vec<u16> = query.iter().map(|&v| f32_to_f16(v)).collect();
        for chunk in embeddings.chunks(SIM_BATCH) {
            // Channel-major fp16 packing to match the MIL tensor [1, dim, 1, span]:
            // element offset = c * span + p; p=0 → query (p=1..SIM_SEQ zero padding),
            // p=SIM_SEQ+o → embedding o. Tail of a short final chunk stays zero.
            let mut packed = vec![0u8; dim * span * 2];
            for c in 0..dim {
                let row = c * span * 2;
                packed[row..row + 2].copy_from_slice(&query_f16[c].to_le_bytes());
                for (o, emb) in chunk.iter().enumerate() {
                    if emb.len() != dim {
                        return None;
                    }
                    let at = row + (SIM_SEQ + o) * 2;
                    packed[at..at + 2].copy_from_slice(&f32_to_f16(emb[c]).to_le_bytes());
                }
            }

            // Output [1, SIM_BATCH, 1, SIM_SEQ]: embedding o's score for query
            // column 0 sits at element o * SIM_SEQ.
            let mut outputs = kernel
                .engine
                .execute(&[packed.as_slice()], &[SIM_BATCH * SIM_SEQ * 2])
                .ok()?;
            let bytes = outputs.pop()?;
            for o in 0..chunk.len() {
                let at = o * SIM_SEQ * 2;
                let h = u16::from_le_bytes([bytes[at], bytes[at + 1]]);
                scores.push(f16_to_f32(h));
            }
        }

        Some(scores)
    }

    /// True when the aux-offload kernel is compiled and ready.
    pub async fn can_accelerate(&self) -> bool {
        *self.available.lock().await
            && self.sim_kernel.lock().map(|g| g.is_some()).unwrap_or(false)
    }

    /// Update status with real-time metrics
    pub async fn update_status(&self, tokens_processed: u32, elapsed_secs: f32) {
        let mut status = self.status.lock().await;
        if elapsed_secs > 0.0 {
            status.tokens_per_sec_estimate = tokens_processed as f32 / elapsed_secs;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ane_status_detection() {
        let optimizer = AneInferenceOptimizer::new();
        let status = optimizer.get_status().await;

        // Honest defaults: no fake speedup, no fake tok/s.
        assert_eq!(status.estimated_speedup, 1.0);
        assert_eq!(status.tokens_per_sec_estimate, 0.0);

        #[cfg(not(target_os = "macos"))]
        assert!(!status.available);
    }

    #[tokio::test]
    async fn test_similarity_batch_rejects_empty() {
        let optimizer = AneInferenceOptimizer::new();
        assert!(optimizer.similarity_batch(&[], &[]).is_none());
    }

    /// Real hardware check: ANE dot products must match CPU within fp16 tolerance.
    /// Skips silently when kernel compile fails (e.g. dylib missing in CI).
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    #[tokio::test]
    async fn test_similarity_batch_matches_cpu_on_ane() {
        let optimizer = AneInferenceOptimizer::new();
        let dim = 768usize;

        let make_unit = |seed: usize| -> Vec<f32> {
            let mut v: Vec<f32> = (0..dim)
                .map(|i| (((seed * 31 + i * 17) % 1000) as f32 / 500.0) - 1.0)
                .collect();
            let n = v.iter().map(|x| x * x).sum::<f32>().sqrt();
            v.iter_mut().for_each(|x| *x /= n);
            v
        };

        let query = make_unit(7);
        // Cross a batch boundary: SIM_BATCH + 3 embeddings.
        let embs: Vec<Vec<f32>> = (0..SIM_BATCH + 3).map(make_unit).collect();
        let refs: Vec<&[f32]> = embs.iter().map(|e| e.as_slice()).collect();

        let Some(ane_scores) = optimizer.similarity_batch(&query, &refs) else {
            eprintln!("[ANE test] kernel unavailable — skipping correctness check");
            return;
        };
        assert_eq!(ane_scores.len(), refs.len());

        for (i, emb) in embs.iter().enumerate() {
            let cpu: f32 = query.iter().zip(emb.iter()).map(|(a, b)| a * b).sum();
            assert!(
                (ane_scores[i] - cpu).abs() < 0.02,
                "score {i}: ane={} cpu={}",
                ane_scores[i],
                cpu
            );
        }
    }
}
