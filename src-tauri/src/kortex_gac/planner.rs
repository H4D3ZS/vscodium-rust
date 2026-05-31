//! Tier planner — turns a GeometryProfile into a llama.cpp launch plan.
//!
//! The plan consists of:
//!   * `n_gpu_layers` — passed to `llama-server -ngl <N>`. We always set this
//!     to "all blocks" (== block_count); the per-tensor overrides below do the
//!     real work. This means the GPU is the default destination and overrides
//!     surgically move things to CPU.
//!   * `overrides`   — a list of `--override-tensor PATTERN=BUFFER` rules.
//!     Each rule covers one *tensor kind* (e.g. all `\.ffn_down\.weight$`).
//!     llama.cpp matches against full tensor names, so a kind regex catches
//!     every block at once.
//!
//! Algorithm (greedy, geometry-aware):
//!   1. Group tensors by kind (`attn_q`, `ffn_down`, ...).
//!   2. Score each kind: priority := mean over its tensors of
//!         (d_bar / d_bar_critical)   -- higher = more spread = more GPU-worthy.
//!   3. Sort kinds descending by priority (most spread first).
//!   4. Greedy pack into the GPU budget. Anything that doesn't fit is shipped
//!      to CPU via an override.
//!
//! This rule has a clean interpretation:
//!   * spread tensors win the GPU (they lose the most under bandwidth pressure).
//!   * tight tensors lose gracefully to CPU — by Theorem §2 their identity
//!     error stays bounded even under aggressive consolidation.

use anyhow::Result;
use std::collections::BTreeMap;

use super::theory::d_bar_critical;
use super::types::{
    GeometryProfile, PlanOptions, Regime, RoutingCounts, TensorOverride, TierBuffer, TierPlan,
};

/// Plan a tier assignment for a profile under a given budget.
pub fn plan_tiers(profile: &GeometryProfile, opts: &PlanOptions) -> Result<TierPlan> {
    let d_eff_global = opts.d_eff_global.unwrap_or(profile.global.d_eff_global);
    let critical = d_bar_critical(opts.theta, d_eff_global);
    let safe_t = opts.safe_mult * critical;
    let unsafe_t = opts.unsafe_mult * critical;

    // Reserve some VRAM for KV cache + activations + scratch.
    let usable_mb = (opts.vram_total_mb as f32 * (1.0 - opts.kv_reserve_frac)) as u64;
    let usable_bytes = usable_mb * 1024 * 1024;

    // Group tensors by kind.
    let mut groups: BTreeMap<String, KindGroup> = BTreeMap::new();
    for t in &profile.tensors {
        let kind = tensor_kind(&t.name);
        let regime = classify(t.geometry.d_bar, t.geometry.rho, safe_t, unsafe_t);
        let g = groups.entry(kind.clone()).or_insert_with(|| KindGroup {
            kind,
            total_bytes: 0,
            d_bar_sum: 0.0,
            count: 0,
            regime_counts: [0; 3],
            score: 0.0,
        });
        g.total_bytes += t.size_bytes;
        g.d_bar_sum += t.geometry.d_bar as f64;
        g.count += 1;
        match regime {
            Regime::Tight => g.regime_counts[0] += 1,
            Regime::Borderline => g.regime_counts[1] += 1,
            Regime::Spread => g.regime_counts[2] += 1,
        }
    }

    // Score each kind. Higher score = more spread = more deserving of GPU.
    //
    // MoE models get the ds4-style sparsity discount on routed-expert kinds:
    // since only top-k of N experts fire per token, their effective bandwidth
    // requirement is smaller than dense weights of the same byte count. We
    // multiply their score by `top_k / N` so dense weights win the GPU first.
    //
    // The router itself (`ffn_gate_inp`, `ffn_norm`, `attn_norm`) gets pinned
    // GPU regardless of geometry — it's tiny and fires every token.
    let moe_factor = profile.moe.as_ref().map(|m| m.sparsity_factor()).unwrap_or(1.0);
    let mut kinds: Vec<KindGroup> = groups.into_values().collect();
    for k in kinds.iter_mut() {
        let mean_d_bar = (k.d_bar_sum / k.count.max(1) as f64) as f32;
        let base = mean_d_bar / critical.max(1e-6);
        if is_pinned_gpu_kind(&k.kind) {
            // Score above any spread tensor so it always lands on GPU first.
            k.score = f32::INFINITY;
        } else if is_moe_expert_kind(&k.kind) {
            k.score = base * moe_factor;
        } else {
            k.score = base;
        }
    }
    kinds.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

    // Greedy: pack highest-priority kinds onto GPU until usable_bytes is exhausted.
    // Anything that doesn't fit gets a CPU override.
    let mut gpu_total: u64 = 0;
    let mut cpu_total: u64 = 0;
    let mut overrides: Vec<TensorOverride> = Vec::new();
    let mut counts = RoutingCounts::default();

    for k in kinds {
        let pattern = kind_regex(&k.kind);
        let majority_regime = majority(&k.regime_counts);
        let fits = gpu_total + k.total_bytes <= usable_bytes;

        if fits {
            // Stays on GPU implicitly via -ngl 999. We *don't* emit an override
            // for the GPU side — that would be a no-op. Just record the bytes.
            gpu_total += k.total_bytes;
            match majority_regime {
                Regime::Spread => counts.spread_to_gpu += k.count as u32,
                Regime::Borderline => counts.borderline_to_gpu += k.count as u32,
                Regime::Tight => counts.tight_to_gpu += k.count as u32,
            }
        } else {
            // Push to CPU.
            overrides.push(TensorOverride {
                pattern,
                buffer: TierBuffer::Cpu,
                bytes: k.total_bytes,
            });
            cpu_total += k.total_bytes;
            match majority_regime {
                Regime::Spread => counts.spread_to_cpu += k.count as u32,
                Regime::Borderline => counts.borderline_to_cpu += k.count as u32,
                Regime::Tight => counts.tight_to_cpu += k.count as u32,
            }
        }
    }

    // n_gpu_layers: always set to (block_count + 1) so non-block tensors that
    // didn't get overridden also land on GPU. Saturate at 999.
    let n_gpu_layers = ((profile.block_count + 1) as u32).min(999);

    Ok(TierPlan {
        n_gpu_layers,
        overrides,
        total_gpu_bytes: gpu_total,
        total_cpu_bytes: cpu_total,
        vram_budget_mb: usable_mb as u32,
        theta: opts.theta,
        d_bar_critical: critical,
        routing_counts: counts,
        backend: opts.backend.clone(),
    })
}

/// Render the plan to a llama-server argument list. The caller still chooses
/// the binary path, model path, port, and context size.
pub fn render_args(plan: &TierPlan) -> Vec<String> {
    let buf_name = backend_buffer_name(&plan.backend);
    let mut args = Vec::new();
    args.push("--n-gpu-layers".to_string());
    args.push(plan.n_gpu_layers.to_string());
    for ov in &plan.overrides {
        args.push("--override-tensor".to_string());
        let target = match ov.buffer {
            TierBuffer::Cpu => "CPU".to_string(),
            TierBuffer::Gpu => buf_name.clone(),
        };
        args.push(format!("{}={}", ov.pattern, target));
    }
    args
}

// ────────────────────────────────────────────────────────────────────────────
// internals
// ────────────────────────────────────────────────────────────────────────────

#[derive(Debug)]
struct KindGroup {
    kind: String,
    total_bytes: u64,
    d_bar_sum: f64,
    count: u64,
    /// Counts of [tight, borderline, spread] tensors in this kind.
    regime_counts: [u32; 3],
    /// Patched after the aggregation loop; depends on `critical`.
    score: f32,
}

/// Extract the "kind" of a tensor name. The kind is the per-block role:
/// `blk.0.attn_q.weight` -> `attn_q`. Non-block tensors keep their full name.
fn tensor_kind(name: &str) -> String {
    if let Some(rest) = name.strip_prefix("blk.") {
        if let Some(dot) = rest.find('.') {
            let after_block = &rest[dot + 1..];
            // Drop the trailing ".weight" / ".bias" / ".g".
            let kind = after_block
                .strip_suffix(".weight")
                .or_else(|| after_block.strip_suffix(".bias"))
                .unwrap_or(after_block);
            return kind.to_string();
        }
    }
    name.to_string()
}

/// Regex string suitable for `--override-tensor` to catch every block of a kind.
fn kind_regex(kind: &str) -> String {
    let escaped = regex_escape(kind);
    // Match `blk.<digits>.<kind>.weight` (and .bias, just in case).
    format!(r"blk\.\d+\.{}\.(weight|bias)", escaped)
}

fn regex_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    for c in s.chars() {
        match c {
            '.' | '+' | '*' | '?' | '(' | ')' | '|' | '[' | ']' | '{' | '}' | '^' | '$' | '\\' => {
                out.push('\\');
                out.push(c);
            }
            _ => out.push(c),
        }
    }
    out
}

fn classify(d_bar: f32, rho: f32, safe_t: f32, unsafe_t: f32) -> Regime {
    if d_bar < safe_t && rho > 0.55 {
        Regime::Tight
    } else if d_bar > unsafe_t {
        Regime::Spread
    } else {
        Regime::Borderline
    }
}

/// Tensors that should always live on GPU regardless of geometry.
/// Routers and norms fire on every token and are tiny — pin them.
fn is_pinned_gpu_kind(kind: &str) -> bool {
    matches!(
        kind,
        "ffn_gate_inp"  // MoE router
            | "attn_norm"
            | "ffn_norm"
            | "attn_q_norm"
            | "attn_k_norm"
            | "post_attention_norm"
    )
}

/// Detects routed-expert kinds in MoE models.
/// In GGUF these are `ffn_{gate,down,up}_exps`.
fn is_moe_expert_kind(kind: &str) -> bool {
    kind.ends_with("_exps")
}

fn majority(regime_counts: &[u32; 3]) -> Regime {
    let (mut best_idx, mut best_val) = (0usize, regime_counts[0]);
    for (i, &v) in regime_counts.iter().enumerate().skip(1) {
        if v > best_val {
            best_val = v;
            best_idx = i;
        }
    }
    match best_idx {
        0 => Regime::Tight,
        2 => Regime::Spread,
        _ => Regime::Borderline,
    }
}

/// Backend-specific GPU buffer name for `--override-tensor`.
/// llama.cpp accepts: CUDA0, CUDA1, ..., Vulkan0, ..., ROCm0, ..., Metal.
pub fn backend_buffer_name(backend: &str) -> String {
    match backend.to_ascii_lowercase().as_str() {
        "cuda" | "nvidia" => "CUDA0".to_string(),
        "rocm" | "hip" | "amd-rocm" => "ROCm0".to_string(),
        "vulkan" | "amd" | "amd-vulkan" => "Vulkan0".to_string(),
        "metal" | "apple" => "Metal".to_string(),
        "sycl" | "intel" => "SYCL0".to_string(),
        other => other.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kortex_gac::types::{Geometry, GlobalStats, ProfilerConfig, TensorGeometry};

    fn make_profile(tensors: Vec<TensorGeometry>) -> GeometryProfile {
        let n = tensors.len() as u32;
        let mean_de = tensors.iter().map(|t| t.geometry.d_eff).sum::<f32>() / n.max(1) as f32;
        let mean_db = tensors.iter().map(|t| t.geometry.d_bar).sum::<f32>() / n.max(1) as f32;
        GeometryProfile {
            version: 1,
            model_path: "test.gguf".into(),
            model_size_bytes: 0,
            block_count: 32,
            tensors,
            global: GlobalStats {
                d_eff_global: mean_de,
                d_bar_mean: mean_db,
                n_profiled: n,
            },
            profiler_config: ProfilerConfig::default(),
            moe: None,
        }
    }

    fn t(name: &str, bytes: u64, d_bar: f32, rho: f32, d_eff_v: f32) -> TensorGeometry {
        TensorGeometry {
            name: name.into(),
            shape: vec![1024, 1024],
            quant_type: "Q4_K".into(),
            size_bytes: bytes,
            block_index: Some(0),
            geometry: Geometry { d_bar, d_eff: d_eff_v, rho, regime: Regime::Borderline },
        }
    }

    #[test]
    fn tensor_kind_extracts_role() {
        assert_eq!(tensor_kind("blk.0.attn_q.weight"), "attn_q");
        assert_eq!(tensor_kind("blk.31.ffn_down.weight"), "ffn_down");
        assert_eq!(tensor_kind("output.weight"), "output.weight");
    }

    #[test]
    fn kind_regex_catches_every_block() {
        let re = kind_regex("ffn_down");
        let r = regex::Regex::new(&re).unwrap();
        assert!(r.is_match("blk.0.ffn_down.weight"));
        assert!(r.is_match("blk.79.ffn_down.weight"));
        assert!(!r.is_match("blk.0.ffn_up.weight"));
        assert!(!r.is_match("token_embd.weight"));
    }

    #[test]
    fn spread_tensors_keep_gpu_tight_to_cpu_when_full() {
        // 100 MB GPU budget after KV reserve.
        let opts = PlanOptions {
            vram_total_mb: 200, // 100 MB usable after default 30% reserve actually = 140 MB. close enough.
            kv_reserve_frac: 0.0,
            theta: 0.85,
            d_eff_global: Some(8.0),
            safe_mult: 0.75,
            unsafe_mult: 1.25,
            backend: "cuda".into(),
        };
        let mb: u64 = 1024 * 1024;
        let tensors = vec![
            // Spread: high d_bar, low rho.
            t("blk.0.attn_q.weight", 80 * mb, 0.6, 0.10, 8.0),
            t("blk.1.attn_q.weight", 80 * mb, 0.6, 0.10, 8.0),
            // Tight: low d_bar, high rho.
            t("blk.0.ffn_down.weight", 60 * mb, 0.05, 0.95, 4.0),
            t("blk.1.ffn_down.weight", 60 * mb, 0.05, 0.95, 4.0),
        ];
        let p = make_profile(tensors);
        let plan = plan_tiers(&p, &opts).unwrap();
        // Total spread = 160 MB. Total tight = 120 MB. Budget = 200 MB.
        // Spread comes first (higher score), takes 160 MB. Tight (120 MB) doesn't fit -> CPU.
        assert_eq!(plan.overrides.len(), 1);
        assert!(plan.overrides[0].pattern.contains("ffn_down"));
        assert!(matches!(plan.overrides[0].buffer, TierBuffer::Cpu));
        assert_eq!(plan.routing_counts.spread_to_gpu, 2);
        assert_eq!(plan.routing_counts.tight_to_cpu, 2);
    }

    #[test]
    fn moe_experts_lose_priority_to_dense_when_budget_tight() {
        // Same geometry on both kinds — only the MoE sparsity factor differentiates
        // them. Budget is set so exactly one of the two 80 MB kinds can fit.
        let opts = PlanOptions {
            vram_total_mb: 100,
            kv_reserve_frac: 0.0,
            theta: 0.85,
            d_eff_global: Some(8.0),
            safe_mult: 0.75,
            unsafe_mult: 1.25,
            backend: "cuda".into(),
        };
        let mb: u64 = 1024 * 1024;
        let tensors = vec![
            t("blk.0.ffn_down.weight", 80 * mb, 0.6, 0.10, 8.0),
            t("blk.0.ffn_down_exps.weight", 80 * mb, 0.6, 0.10, 8.0),
        ];
        let mut profile = make_profile(tensors);
        profile.moe = Some(crate::kortex_gac::types::MoeMeta {
            expert_count: 64,
            expert_used_count: 2,
        });
        let plan = plan_tiers(&profile, &opts).unwrap();
        let exp_override = plan.overrides.iter().find(|o| o.pattern.contains("ffn_down_exps"));
        assert!(
            exp_override.is_some(),
            "MoE experts should be CPU-overridden first when budget is tight"
        );
        assert!(matches!(exp_override.unwrap().buffer, TierBuffer::Cpu));
        // Dense ffn_down should NOT have a CPU override.
        let dense_override = plan
            .overrides
            .iter()
            .find(|o| o.pattern.contains("ffn_down") && !o.pattern.contains("ffn_down_exps"));
        assert!(dense_override.is_none(), "dense ffn_down should keep GPU residency");
    }

    #[test]
    fn render_args_emits_override_tensor_flags() {
        let plan = TierPlan {
            n_gpu_layers: 33,
            overrides: vec![TensorOverride {
                pattern: r"blk\.\d+\.ffn_down\.(weight|bias)".to_string(),
                buffer: TierBuffer::Cpu,
                bytes: 0,
            }],
            total_gpu_bytes: 0,
            total_cpu_bytes: 0,
            vram_budget_mb: 0,
            theta: 0.85,
            d_bar_critical: 0.0,
            routing_counts: RoutingCounts::default(),
            backend: "cuda".into(),
        };
        let args = render_args(&plan);
        assert_eq!(args[0], "--n-gpu-layers");
        assert_eq!(args[1], "33");
        assert_eq!(args[2], "--override-tensor");
        assert!(args[3].ends_with("=CPU"));
    }
}
