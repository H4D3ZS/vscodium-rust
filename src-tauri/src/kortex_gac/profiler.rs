//! GGUF geometry profiler.
//!
//! Reads a GGUF model, walks every transformer-block weight tensor, dequantises
//! a small row sample (default 256 rows × N features), and records the GAC
//! geometry triple (d_bar, d_eff, rho) per tensor. The output is a JSON profile
//! the planner consumes to decide GPU/CPU placement.
//!
//! Memory: peak usage is the size of the largest dequantised tensor in fp32.
//! For 70B Q4_K_M that is ~940 MB (ffn_down). The embedding and output tensors
//! are skipped via the `max_tensor_bytes` knob (default 2 GiB).

use anyhow::{anyhow, Result};
use candle_core::quantized::gguf_file::{Content, Value};
use candle_core::Device;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand::rngs::SmallRng;
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

use super::theory::{
    cluster_spread, d_bar_critical, d_eff, l2_normalize_rows, rho_cluster,
};
use super::types::{
    Geometry, GeometryProfile, GlobalStats, MoeMeta, ProfilerConfig, Regime, TensorGeometry,
};

const MAX_TENSOR_BYTES_DEFAULT: u64 = 2 * 1024 * 1024 * 1024;

/// Profile a GGUF file. Default config samples 256 rows per weight tensor.
pub fn profile_gguf(path: &Path, cfg: &ProfilerConfig) -> Result<GeometryProfile> {
    profile_gguf_with_limits(path, cfg, MAX_TENSOR_BYTES_DEFAULT)
}

pub fn profile_gguf_with_limits(
    path: &Path,
    cfg: &ProfilerConfig,
    max_tensor_bytes: u64,
) -> Result<GeometryProfile> {
    let started = Instant::now();
    let model_size_bytes = std::fs::metadata(path)?.len();
    let mut file = File::open(path)?;
    let content = Content::read(&mut file)?;

    let arch = lookup_str(&content.metadata, "general.architecture")
        .unwrap_or_else(|| "unknown".to_string());
    let block_count = lookup_block_count(&content.metadata, &arch);
    let moe = detect_moe_meta(&content.metadata, &arch, &content.tensor_infos);

    tracing::info!(
        "[kortex-gac] Profiling {} ({} arch, {} blocks, {:.2} GiB){}",
        path.display(),
        arch,
        block_count,
        model_size_bytes as f64 / (1024.0 * 1024.0 * 1024.0),
        moe.as_ref().map(|m| format!(" [MoE: {}/{} experts]", m.expert_used_count, m.expert_count))
            .unwrap_or_default()
    );

    // Iterate tensor entries in deterministic order (sorted by name).
    let mut tensor_names: Vec<&String> = content.tensor_infos.keys().collect();
    tensor_names.sort();

    let mut tensors_out: Vec<TensorGeometry> = Vec::new();
    let mut d_eff_sum = 0.0f64;
    let mut d_bar_sum = 0.0f64;
    let mut profiled_count: u32 = 0;

    for name in tensor_names {
        let info = &content.tensor_infos[name];
        let shape: Vec<usize> = info.shape.dims().to_vec();
        let qtype_str = format!("{:?}", info.ggml_dtype);

        // Only profile 2D weight tensors. 1D bias tensors and norms have
        // trivial geometry; we let them inherit their block's plan.
        if !is_profilable_weight(name, &shape) {
            continue;
        }

        // Estimate raw GGUF bytes for this tensor (approximation; true sizes
        // depend on quant block layout but the per-element rate is fixed).
        let est_bytes = estimate_tensor_bytes(&shape, &qtype_str);
        if est_bytes > max_tensor_bytes {
            tracing::debug!(
                "[kortex-gac] skipping {} ({} bytes > limit {})",
                name,
                est_bytes,
                max_tensor_bytes
            );
            continue;
        }

        // Dequantise to fp32 and sample rows.
        let qt = match content.tensor(&mut file, name, &Device::Cpu) {
            Ok(t) => t,
            Err(e) => {
                tracing::warn!("[kortex-gac] dequant failed for {}: {}", name, e);
                continue;
            }
        };
        let dequant = match qt.dequantize(&Device::Cpu) {
            Ok(t) => t,
            Err(e) => {
                tracing::warn!("[kortex-gac] dequantize failed for {}: {}", name, e);
                continue;
            }
        };

        let n_rows = shape[0];
        let n_sample = std::cmp::min(cfg.sample_rows as usize, n_rows.max(1));
        let chosen_indices = sample_indices(n_rows, n_sample, cfg.seed, name);

        let mut rows: Vec<Vec<f32>> = Vec::with_capacity(n_sample);
        let mut sample_ok = true;
        for &idx in &chosen_indices {
            let row = match dequant.get(idx) {
                Ok(t) => match t.to_vec1::<f32>() {
                    Ok(v) => v,
                    Err(e) => {
                        tracing::warn!("[kortex-gac] row to_vec1 failed: {}: {}", name, e);
                        sample_ok = false;
                        break;
                    }
                },
                Err(e) => {
                    tracing::warn!("[kortex-gac] row get failed: {}: {}", name, e);
                    sample_ok = false;
                    break;
                }
            };
            rows.push(row);
        }
        // Free the dequantised tensor as soon as we have the sample.
        drop(dequant);

        if !sample_ok || rows.len() < 2 {
            continue;
        }
        l2_normalize_rows(&mut rows);

        let d_bar = cluster_spread(&rows);
        let de = d_eff(&rows);
        let rho = rho_cluster(&rows);

        d_bar_sum += d_bar as f64;
        d_eff_sum += de as f64;
        profiled_count += 1;

        let block_index = parse_block_index(name);

        tensors_out.push(TensorGeometry {
            name: name.clone(),
            shape,
            quant_type: qtype_str,
            size_bytes: est_bytes,
            block_index,
            // Regime is filled in below once we know d_eff_global.
            geometry: Geometry {
                d_bar,
                d_eff: de,
                rho,
                regime: Regime::Borderline,
            },
        });
    }

    if profiled_count == 0 {
        return Err(anyhow!("no profilable weight tensors found in {}", path.display()));
    }

    let d_eff_global = (d_eff_sum / profiled_count as f64) as f32;
    let d_bar_mean = (d_bar_sum / profiled_count as f64) as f32;

    // Classify each tensor against the global thresholds at theta = 0.85.
    // Planner can re-classify at a different theta later — this is a default tag.
    let theta_default = 0.85f32;
    let critical = d_bar_critical(theta_default, d_eff_global);
    let safe_t = 0.75 * critical;
    let unsafe_t = 1.25 * critical;

    for t in tensors_out.iter_mut() {
        t.geometry.regime = classify_regime(t.geometry.d_bar, t.geometry.rho, safe_t, unsafe_t);
    }

    let elapsed = started.elapsed().as_secs_f32();
    tracing::info!(
        "[kortex-gac] Profile complete: {} tensors, d_eff_global={:.2}, d_bar_mean={:.3}, took {:.1}s",
        profiled_count,
        d_eff_global,
        d_bar_mean,
        elapsed
    );

    Ok(GeometryProfile {
        version: 1,
        model_path: path.to_string_lossy().into_owned(),
        model_size_bytes,
        block_count,
        tensors: tensors_out,
        global: GlobalStats {
            d_eff_global,
            d_bar_mean,
            n_profiled: profiled_count,
        },
        profiler_config: cfg.clone(),
        moe,
    })
}

/// Write a profile to disk as pretty JSON. Conventional path: alongside the GGUF
/// with extension `.geometry.aim`.
pub fn write_profile(profile: &GeometryProfile, out_path: &Path) -> Result<()> {
    let json = serde_json::to_string_pretty(profile)?;
    let mut f = File::create(out_path)?;
    f.write_all(json.as_bytes())?;
    Ok(())
}

/// Read a previously-written profile. Returns Err if the file does not exist.
pub fn read_profile(path: &Path) -> Result<GeometryProfile> {
    let json = std::fs::read_to_string(path)?;
    let profile: GeometryProfile = serde_json::from_str(&json)?;
    Ok(profile)
}

/// Default profile path: `<gguf_path>.geometry.aim`.
pub fn default_profile_path(model_path: &Path) -> std::path::PathBuf {
    let mut p = model_path.to_path_buf();
    let new_name = format!(
        "{}.geometry.aim",
        p.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default()
    );
    p.set_file_name(new_name);
    p
}

// ────────────────────────────────────────────────────────────────────────────
// helpers
// ────────────────────────────────────────────────────────────────────────────

fn is_profilable_weight(name: &str, shape: &[usize]) -> bool {
    if shape.len() != 2 {
        return false;
    }
    if !name.ends_with(".weight") {
        return false;
    }
    // Skip 1D-equivalent weights (norm scales) that happened to be stored 2D.
    if shape[0] < 16 || shape[1] < 16 {
        return false;
    }
    // Skip the giant embedding / output projections — they OOM the dequant
    // path and their geometry is not useful for layer placement anyway.
    let lower = name.to_ascii_lowercase();
    if lower.contains("token_embd") || lower.starts_with("output") || lower.contains("rope_freqs") {
        return false;
    }
    true
}

fn estimate_tensor_bytes(shape: &[usize], qtype: &str) -> u64 {
    let n_elems: u64 = shape.iter().map(|&d| d as u64).product();
    // Per-element rate in bytes. Approximations for common k-quants from
    // ggml-quants.h. Good enough for budgeting; planner uses these for VRAM packing.
    //
    // Note: candle's GgmlDType Debug spells k-quants without underscores
    // ("Q4K" not "Q4_K"). We accept both spellings for forward-compat with
    // newer candle releases.
    let bpe: f64 = match qtype {
        "F32" => 4.0,
        "F16" | "BF16" => 2.0,
        "Q8_0" | "Q8_1" => 1.0625,
        "Q6K" | "Q6_K" => 0.8125,
        "Q5K" | "Q5_K" | "Q5_0" | "Q5_1" => 0.6875,
        "Q4K" | "Q4_K" | "Q4_0" | "Q4_1" => 0.5625,
        "Q3K" | "Q3_K" => 0.4375,
        "Q2K" | "Q2_K" => 0.3125,
        "Q8K" | "Q8_K" => 1.0625,
        "IQ4_XS" => 0.5,
        "IQ3_XXS" => 0.40625,
        "IQ2_XXS" | "IQ2_XS" => 0.3,
        _ => 0.5625,
    };
    ((n_elems as f64) * bpe).ceil() as u64
}

fn parse_block_index(name: &str) -> Option<u32> {
    if !name.starts_with("blk.") {
        return None;
    }
    let rest = &name[4..];
    let dot = rest.find('.')?;
    rest[..dot].parse::<u32>().ok()
}

fn classify_regime(d_bar: f32, rho: f32, safe_t: f32, unsafe_t: f32) -> Regime {
    if d_bar < safe_t && rho > 0.55 {
        Regime::Tight
    } else if d_bar > unsafe_t {
        Regime::Spread
    } else {
        Regime::Borderline
    }
}

fn sample_indices(n: usize, k: usize, seed: u64, salt: &str) -> Vec<usize> {
    if k >= n {
        return (0..n).collect();
    }
    // Mix the tensor name into the seed so two same-shape tensors don't share rows.
    let mut s: u64 = seed;
    for b in salt.bytes() {
        s = s.wrapping_mul(6364136223846793005).wrapping_add(b as u64);
    }
    let mut rng = SmallRng::seed_from_u64(s);
    let mut all: Vec<usize> = (0..n).collect();
    all.shuffle(&mut rng);
    all.truncate(k);
    all.sort();
    all
}

fn lookup_str(meta: &HashMap<String, Value>, key: &str) -> Option<String> {
    meta.get(key).and_then(|v| v.to_string().ok().map(String::from))
}

fn lookup_u32(meta: &HashMap<String, Value>, key: &str) -> Option<u32> {
    let v = meta.get(key)?;
    if let Ok(x) = v.to_u32() {
        return Some(x);
    }
    if let Ok(x) = v.to_u64() {
        return Some(x as u32);
    }
    if let Ok(x) = v.to_i32() {
        return Some(x as u32);
    }
    None
}

fn lookup_block_count(meta: &HashMap<String, Value>, arch: &str) -> u32 {
    let key = format!("{}.block_count", arch);
    lookup_u32(meta, &key)
        .or_else(|| lookup_u32(meta, "block_count"))
        .or_else(|| lookup_u32(meta, "llama.block_count"))
        .unwrap_or(0)
}

/// Detect whether the GGUF describes a Mixture-of-Experts model.
///
/// We try the two routes both used in the wild:
///   1. Explicit GGUF metadata: `<arch>.expert_count` and `<arch>.expert_used_count`.
///   2. Tensor-name heuristic: any tensor whose name ends in `_exps.weight`
///      (e.g. `blk.0.ffn_down_exps.weight`) → assume MoE; recover expert_count
///      from the leading dim of the tensor shape.
fn detect_moe_meta(
    meta: &HashMap<String, Value>,
    arch: &str,
    tensor_infos: &HashMap<String, candle_core::quantized::gguf_file::TensorInfo>,
) -> Option<MoeMeta> {
    let from_meta_n = lookup_u32(meta, &format!("{}.expert_count", arch))
        .or_else(|| lookup_u32(meta, "expert_count"));
    let from_meta_k = lookup_u32(meta, &format!("{}.expert_used_count", arch))
        .or_else(|| lookup_u32(meta, "expert_used_count"));

    if let (Some(n), Some(k)) = (from_meta_n, from_meta_k) {
        if n > 1 {
            return Some(MoeMeta {
                expert_count: n,
                expert_used_count: k.max(1),
            });
        }
    }

    // Heuristic fallback: scan tensor names for the `_exps` suffix.
    let mut inferred_n: Option<u32> = None;
    for (name, info) in tensor_infos.iter() {
        if name.contains("_exps.") || name.ends_with("_exps") {
            // `_exps` tensors carry expert_count as the first dim.
            if let Some(dim0) = info.shape.dims().first().copied() {
                inferred_n = Some(dim0 as u32);
                break;
            }
        }
    }
    if let Some(n) = inferred_n {
        if n > 1 {
            return Some(MoeMeta {
                expert_count: n,
                expert_used_count: from_meta_k.unwrap_or(2),
            });
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_block_index_works() {
        assert_eq!(parse_block_index("blk.0.attn_q.weight"), Some(0));
        assert_eq!(parse_block_index("blk.31.ffn_down.weight"), Some(31));
        assert_eq!(parse_block_index("token_embd.weight"), None);
        assert_eq!(parse_block_index("output_norm.weight"), None);
    }

    #[test]
    fn estimate_tensor_bytes_q4k_8x8() {
        // 8 * 8 = 64 elems @ 0.5625 bpe -> 36 bytes.
        // Both spellings should resolve.
        assert_eq!(estimate_tensor_bytes(&[8, 8], "Q4_K"), 36);
        assert_eq!(estimate_tensor_bytes(&[8, 8], "Q4K"), 36);
    }

    #[test]
    fn classify_regime_thresholds() {
        // safe_t = 0.1, unsafe_t = 0.3.
        assert!(matches!(classify_regime(0.05, 0.7, 0.1, 0.3), Regime::Tight));
        assert!(matches!(classify_regime(0.4, 0.1, 0.1, 0.3), Regime::Spread));
        assert!(matches!(classify_regime(0.2, 0.4, 0.1, 0.3), Regime::Borderline));
        // Low rho disqualifies tight even with low d_bar.
        assert!(matches!(classify_regime(0.05, 0.4, 0.1, 0.3), Regime::Borderline));
    }

    #[test]
    fn sample_indices_returns_k_or_n() {
        assert_eq!(sample_indices(10, 5, 42, "x").len(), 5);
        assert_eq!(sample_indices(3, 5, 42, "x").len(), 3);
    }

    #[test]
    fn is_profilable_filters_correctly() {
        assert!(is_profilable_weight("blk.0.attn_q.weight", &[4096, 4096]));
        assert!(!is_profilable_weight("blk.0.attn_q.bias", &[4096]));
        assert!(!is_profilable_weight("token_embd.weight", &[32000, 4096]));
        assert!(!is_profilable_weight("output.weight", &[32000, 4096]));
    }
}
