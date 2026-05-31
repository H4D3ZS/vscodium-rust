//! Shared types for the Kortex Geometry-Aware Consolidation (GAC) inference engine.
//!
//! The vocabulary here mirrors `gac/theory.py` from the geometry-of-consolidation
//! repo. We use the same names (d_bar, d_eff, rho, theta) so the math stays
//! readable side-by-side with the paper.

use serde::{Deserialize, Serialize};

/// Routing regime decided per-tensor by the GAC inequality.
///
/// Recall the law: identity error is bounded by
///   eps_id >= 1 - c1 * (theta'/d_bar)^(d_eff/2)
/// with theta' = 1 - theta.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Regime {
    /// d_bar < safe_mult * d_bar_critical AND rho high.
    /// Centroid-style consolidation is safe; tensor has lots of redundancy.
    /// In inference: cheap to ship to CPU because it loses little under reduced bandwidth.
    Tight,
    /// Between safe and unsafe thresholds. Mixed routing; rank-r residuals appropriate.
    Borderline,
    /// d_bar > unsafe_mult * d_bar_critical OR rho low.
    /// Every direction in the cluster matters; identity collapses fast under compression.
    /// In inference: precision-critical, prefer GPU residency.
    Spread,
}

/// The three geometric quantities the GAC paper defines, computed per-tensor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Geometry {
    /// Mean within-cluster cosine distance over the tensor's row set.
    pub d_bar: f32,
    /// Effective dimensionality (participation-ratio estimator).
    pub d_eff: f32,
    /// Spectral concentration: top eigenvalue / sum of eigenvalues.
    /// rho -> 1: rank-1 cluster (collapse-safe). rho -> 1/d_eff: isotropic.
    pub rho: f32,
    /// Routing decision; computed from d_bar, d_eff, rho given (theta, d_eff_global).
    pub regime: Regime,
}

/// One entry per tensor in the GGUF.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TensorGeometry {
    /// Full tensor name as stored in the GGUF, e.g. "blk.0.attn_q.weight".
    pub name: String,
    /// Tensor shape in row-major (matches GGUF metadata).
    pub shape: Vec<usize>,
    /// String name of the quant type (e.g. "Q4_K", "Q5_K", "F16").
    pub quant_type: String,
    /// Size in bytes of the tensor as it sits in the GGUF file.
    pub size_bytes: u64,
    /// The transformer block index this tensor belongs to, if any.
    /// Tensors named "blk.<n>.*" carry n; embeddings/output carry None.
    pub block_index: Option<u32>,
    /// Computed geometry. Populated by the profiler.
    pub geometry: Geometry,
}

/// Top-level profile written to `<model>.geometry.aim`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeometryProfile {
    pub version: u32,
    /// Absolute path to the GGUF that produced this profile.
    pub model_path: String,
    /// Total file size of the GGUF.
    pub model_size_bytes: u64,
    /// Number of transformer blocks (read from `<arch>.block_count` GGUF metadata).
    pub block_count: u32,
    /// One entry per profiled weight tensor.
    pub tensors: Vec<TensorGeometry>,
    /// Aggregate stats across all tensors.
    pub global: GlobalStats,
    /// Profiler config used to generate this profile.
    pub profiler_config: ProfilerConfig,
    /// MoE metadata (None = dense model). Populated at profile time.
    #[serde(default)]
    pub moe: Option<MoeMeta>,
}

/// Mixture-of-Experts metadata read from GGUF and used by the planner to apply
/// the ds4-style "experts get aggressive offload because they're sparse" policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoeMeta {
    /// Total number of experts per layer (e.g. 256 for DeepSeek V4 Flash).
    pub expert_count: u32,
    /// Number of experts activated per token (top-k routing). Typical: 2..8.
    pub expert_used_count: u32,
}

impl MoeMeta {
    /// Sparsity factor in (0, 1]. The planner multiplies expert kinds' GPU
    /// priority score by this so dense weights win the GPU budget when both
    /// kinds are spread.
    pub fn sparsity_factor(&self) -> f32 {
        let k = self.expert_used_count.max(1) as f32;
        let n = self.expert_count.max(self.expert_used_count).max(1) as f32;
        (k / n).max(1e-3)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalStats {
    /// Mean d_eff over all profiled tensors -- the d_eff_global the planner uses.
    pub d_eff_global: f32,
    /// Mean d_bar over all profiled tensors.
    pub d_bar_mean: f32,
    /// Number of tensors profiled.
    pub n_profiled: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilerConfig {
    /// How many rows of each tensor we sampled to compute geometry.
    pub sample_rows: u32,
    /// PRNG seed used for row sampling (so profiles are reproducible).
    pub seed: u64,
}

impl Default for ProfilerConfig {
    fn default() -> Self {
        Self { sample_rows: 256, seed: 0xC0FFEE }
    }
}

/// Compute buffer for tier placement. Maps to llama.cpp's `--override-tensor` buffer names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TierBuffer {
    /// Primary GPU buffer. Backend-specific name is filled in at launch time
    /// (e.g. "CUDA0", "Vulkan0", "ROCm0").
    Gpu,
    /// CPU buffer.
    Cpu,
}

/// One tensor (or group of tensors via regex) with an explicit placement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TensorOverride {
    /// Regex matched against tensor names by llama.cpp's --override-tensor flag.
    pub pattern: String,
    pub buffer: TierBuffer,
    /// Total bytes this override covers (for budget bookkeeping).
    pub bytes: u64,
}

/// Output of the planner. Caller turns this into argv for `llama-server`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierPlan {
    /// Number of full transformer blocks to send via the standard --n-gpu-layers path.
    /// We still emit per-tensor overrides on top of this for fine control.
    pub n_gpu_layers: u32,
    /// Tensor-level overrides ordered with most-specific first (llama.cpp matches in order).
    pub overrides: Vec<TensorOverride>,
    /// Total bytes the plan parks on the GPU.
    pub total_gpu_bytes: u64,
    /// Total bytes the plan parks on the CPU.
    pub total_cpu_bytes: u64,
    /// VRAM budget the planner aimed for (MB, after subtracting KV-cache reserve).
    pub vram_budget_mb: u32,
    /// θ used (retrieval threshold from the GAC paper).
    pub theta: f32,
    /// d_bar_critical = (1 - θ) * 2^(1/d_eff_global).
    pub d_bar_critical: f32,
    /// Counts of regime placements: how many tensors landed in each tier.
    pub routing_counts: RoutingCounts,
    /// Backend label this plan was built for: "cuda", "vulkan", "rocm", "metal".
    /// The launcher uses this to expand TierBuffer::Gpu to the concrete buffer name.
    pub backend: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoutingCounts {
    pub spread_to_gpu: u32,
    pub spread_to_cpu: u32,
    pub borderline_to_gpu: u32,
    pub borderline_to_cpu: u32,
    pub tight_to_gpu: u32,
    pub tight_to_cpu: u32,
}

/// Options the planner accepts. Sensible defaults track the paper's recommendations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanOptions {
    /// Available VRAM in MB. The planner reserves ~25% for KV cache and activations
    /// before allocating tensor weights.
    pub vram_total_mb: u32,
    /// Fraction of VRAM reserved for KV cache + activations + scratch buffers.
    /// Default 0.30 (matches llama.cpp's typical headroom for 4k context).
    pub kv_reserve_frac: f32,
    /// Retrieval threshold θ (paper notation). Higher θ = stricter, smaller cap = more spread.
    pub theta: f32,
    /// Override d_eff_global. If None, taken from GeometryProfile.global.d_eff_global.
    pub d_eff_global: Option<f32>,
    /// Multipliers on d_bar_critical that bracket the borderline regime.
    /// Defaults from gac/strategies.py: (0.75, 1.25).
    pub safe_mult: f32,
    pub unsafe_mult: f32,
    /// Backend the launcher will use. Determines buffer names at launch time.
    pub backend: String,
}

impl Default for PlanOptions {
    fn default() -> Self {
        Self {
            vram_total_mb: 8192,
            kv_reserve_frac: 0.30,
            theta: 0.85,
            d_eff_global: None,
            safe_mult: 0.75,
            unsafe_mult: 1.25,
            backend: "vulkan".into(),
        }
    }
}
