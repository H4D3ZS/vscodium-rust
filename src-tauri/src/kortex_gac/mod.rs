//! Kortex Geometry-Aware Consolidation (GAC) inference engine.
//!
//! Applies the geometry-of-consolidation theorem (`gac/theory.py` from the
//! companion research repo) to LLM inference scheduling. The end-to-end flow:
//!
//!   1. `profiler::profile_gguf` reads a GGUF, computes (d_bar, d_eff, rho)
//!      per weight tensor, writes a `<model>.geometry.aim` JSON profile.
//!   2. `planner::plan_tiers` consumes that profile, applies the GAC routing
//!      rule against a VRAM budget, and outputs a `TierPlan` (which tensors
//!      go to GPU, which to CPU).
//!   3. `launcher::launch` spawns `llama-server` with the plan's flags.
//!
//! Why this gives an 8 GiB GPU a real shot at 35 B–70 B models:
//!
//!   * Naive `--n-gpu-layers N` paging chooses by block index, not signal.
//!     It splits a transformer in half and hopes for the best.
//!   * Geometry-aware routing knows that within the same block, attention
//!     projections are typically *spread* (every direction matters) and MLP
//!     downs are *tight* (lots of redundancy). It puts the spread tensors on
//!     GPU and ships the tight ones to CPU. Same VRAM footprint, much higher
//!     fraction of the *bandwidth-critical* compute lands on the fast path.
//!   * The bound `eps_id >= 1 - c1 * (theta'/d_bar)^d_eff` turns this from
//!     folklore into a proof: tight tensors degrade gracefully under any
//!     compression / bandwidth pressure, by Theorem §2.1 of the paper.

pub mod launcher;
pub mod planner;
pub mod profiler;
pub mod theory;
pub mod types;

use std::path::PathBuf;
#[cfg(feature = "tauri")]
use tauri::command;

pub use launcher::{
    await_healthy, build_argv, current_server_info, launch, resolve_server_binary,
    server_log_tail, stop_server, LaunchOpts, RunningInfo,
};
pub use planner::{plan_tiers, render_args};
pub use profiler::{default_profile_path, profile_gguf, read_profile, write_profile};
pub use types::{
    Geometry, GeometryProfile, GlobalStats, MoeMeta, PlanOptions, ProfilerConfig, Regime,
    RoutingCounts, TensorGeometry, TensorOverride, TierBuffer, TierPlan,
};

// ───────────────────────── Tauri command surface ─────────────────────────────
//
// All commands return `Result<T, String>` because Tauri serialises errors as
// strings into the frontend. Internally we use anyhow.

/// Profile a GGUF and write `<model>.geometry.aim` next to it. Returns the
/// path to the written profile.
#[command]
pub async fn kortex_gac_profile(
    model_path: String,
    sample_rows: Option<u32>,
    seed: Option<u64>,
) -> Result<String, String> {
    let model = PathBuf::from(&model_path);
    let cfg = ProfilerConfig {
        sample_rows: sample_rows.unwrap_or(256),
        seed: seed.unwrap_or(0xC0FFEE),
    };
    let profile = tokio::task::spawn_blocking(move || profile_gguf(&model, &cfg))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;
    let out = default_profile_path(std::path::Path::new(&model_path));
    write_profile(&profile, &out).map_err(|e| e.to_string())?;
    Ok(out.to_string_lossy().into_owned())
}

/// Read an existing profile and return it as JSON.
#[command]
pub async fn kortex_gac_load_profile(profile_path: String) -> Result<GeometryProfile, String> {
    let p = PathBuf::from(profile_path);
    tokio::task::spawn_blocking(move || read_profile(&p))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}

/// Plan a tier assignment. Caller supplies the profile path and PlanOptions.
#[command]
pub async fn kortex_gac_plan(
    profile_path: String,
    options: PlanOptions,
) -> Result<TierPlan, String> {
    let p = PathBuf::from(profile_path);
    tokio::task::spawn_blocking(move || -> anyhow::Result<TierPlan> {
        let profile = read_profile(&p)?;
        let plan = plan_tiers(&profile, &options)?;
        Ok(plan)
    })
    .await
    .map_err(|e| e.to_string())?
    .map_err(|e| e.to_string())
}

/// Render a plan to llama-server argv (without launching).
#[command]
pub async fn kortex_gac_render_args(plan: TierPlan) -> Result<Vec<String>, String> {
    Ok(render_args(&plan))
}

/// Convenience: profile (or read cached profile) + plan + render. One call from the UI.
#[command]
pub async fn kortex_gac_quickplan(
    model_path: String,
    options: PlanOptions,
    refresh_profile: Option<bool>,
) -> Result<TierPlan, String> {
    let model = PathBuf::from(&model_path);
    let profile_path = default_profile_path(&model);
    let refresh = refresh_profile.unwrap_or(false);

    let profile = tokio::task::spawn_blocking(move || -> anyhow::Result<GeometryProfile> {
        if profile_path.is_file() && !refresh {
            return read_profile(&profile_path);
        }
        let cfg = ProfilerConfig::default();
        let profile = profile_gguf(&model, &cfg)?;
        write_profile(&profile, &profile_path)?;
        Ok(profile)
    })
    .await
    .map_err(|e| e.to_string())?
    .map_err(|e| e.to_string())?;

    let plan = plan_tiers(&profile, &options).map_err(|e| e.to_string())?;
    Ok(plan)
}

/// Launch llama-server with a plan + extra opts. Returns the bound port.
#[command]
pub async fn kortex_gac_launch(
    plan: TierPlan,
    server_binary: Option<String>,
    model_path: String,
    port: Option<u16>,
    host: Option<String>,
    ctx_size: Option<u32>,
    n_threads: Option<u32>,
    batch_size: Option<u32>,
    flash_attn: Option<bool>,
    slot_save_path: Option<String>,
    // Speculative decoding. Comma-separated list from the ROCmFPX fork's menu
    // (`ngram-simple`, `ngram-map-k`, `ngram-map-k4v`, `ngram-mod`,
    // `ngram-cache`, `draft-mtp`, `draft-eagle3`, `draft-simple`, ...). The
    // big model verifies every drafted token, so output is bit-identical;
    // the ngram-* guessers need no model and no extra VRAM.
    spec_type: Option<String>,
    draft_model_path: Option<String>,
    draft_ngl: Option<u32>,
    draft_max: Option<u32>,
    // `--spec-draft-p-min`: MTP draft confidence gate (~0.75 on a packed 16 GB card).
    draft_p_min: Option<f32>,
    lookup_cache: Option<String>,
    // `--n-cpu-moe`: MoE experts to keep in RAM (fits a Q4 35B-A3B on 16 GB).
    n_cpu_moe: Option<u32>,
    extra_args: Option<Vec<String>>,
    wait_healthy_secs: Option<u64>,
) -> Result<RunningInfo, String> {
    let server = resolve_server_binary(server_binary.as_deref().map(std::path::Path::new))
        .map_err(|e| e.to_string())?;
    let opts = LaunchOpts {
        server_binary: server,
        model_path: PathBuf::from(model_path),
        port: port.unwrap_or(8081),
        host: host.unwrap_or_else(|| "127.0.0.1".to_string()),
        ctx_size: ctx_size.unwrap_or(8192),
        n_threads: n_threads.unwrap_or(0),
        batch_size: batch_size.unwrap_or(512),
        flash_attn: flash_attn.unwrap_or(false),
        slot_save_path: slot_save_path.map(PathBuf::from),
        spec_type: spec_type.filter(|s| !s.trim().is_empty()),
        draft_model_path: draft_model_path.filter(|s| !s.trim().is_empty()).map(PathBuf::from),
        draft_ngl,
        draft_max,
        draft_p_min: draft_p_min.filter(|p| (0.0..=1.0).contains(p)),
        lookup_cache: lookup_cache.filter(|s| !s.trim().is_empty()).map(PathBuf::from),
        n_cpu_moe: n_cpu_moe.filter(|n| *n > 0),
        extra_args: extra_args.unwrap_or_default(),
    };
    let host_clone = opts.host.clone();
    let port = launch(&plan, &opts).map_err(|e| e.to_string())?;
    let wait = wait_healthy_secs.unwrap_or(60);
    if wait > 0 {
        if let Err(e) = await_healthy(&host_clone, port, wait).await {
            // Health didn't come up: leave the process running so the user
            // can inspect logs, but report the failure.
            return Err(format!("server spawned but unhealthy: {}", e));
        }
    }
    current_server_info().ok_or_else(|| "server launched but registry empty".to_string())
}

#[command]
pub async fn kortex_gac_stop() -> Result<(), String> {
    stop_server().map_err(|e| e.to_string())
}

#[command]
pub async fn kortex_gac_status() -> Result<Option<RunningInfo>, String> {
    Ok(current_server_info())
}

/// Tail of the running llama-server's stdout/stderr, so the UI can show load
/// progress ("loaded 340/900 tensors…") instead of an opaque spinner.
#[command]
pub async fn kortex_gac_log(lines: Option<usize>) -> Result<Vec<String>, String> {
    Ok(server_log_tail(lines.unwrap_or(80)))
}

/// Default profile path for a given model (helper for the UI to display).
#[command]
pub async fn kortex_gac_default_profile_path(model_path: String) -> Result<String, String> {
    let p = default_profile_path(std::path::Path::new(&model_path));
    Ok(p.to_string_lossy().into_owned())
}

/// One GGUF found on disk, for the panel's model picker.
#[derive(Debug, Clone, serde::Serialize)]
pub struct LocalGguf {
    /// `Org/Repo` when the file lives under a HF `models--Org--Repo` tree,
    /// else the containing directory name.
    pub repo: String,
    /// File name, e.g. `Qwen3.8-27B-AD-IQ3_XXS.gguf`.
    pub file: String,
    /// Quant tag parsed from the file name (`IQ3_XXS`, `Q4_K_M`, …) or "".
    pub quant: String,
    /// Absolute path — this is what goes in the launcher's `-m`.
    pub path: String,
    pub size_mb: u64,
    /// `mmproj` / vision projector / embedding sidecar — not a chat model.
    pub aux: bool,
}

fn parse_quant(name: &str) -> String {
    let up = name.to_uppercase();
    // Longest / most-specific tags first so `Q4_K_M` wins over `Q4_0`, and the
    // Escha/ROCm custom types are caught before the plain ones.
    for tag in [
        "Q2_0_ROCMFPX", "ROCMFPX", "ROCMFP2", "ROCMI4",
        "IQ1_M", "IQ1_S", "IQ2_XXS", "IQ2_XS", "IQ2_S", "IQ2_M", "IQ3_XXS", "IQ3_XS", "IQ3_S",
        "IQ3_M", "IQ4_XS", "IQ4_NL", "Q2_K_XL", "Q2_K", "Q3_K_XL", "Q3_K_M", "Q3_K_S", "Q4_K_XL",
        "Q4_K_M", "Q4_K_S", "Q5_K_M", "Q5_K_S", "Q6_K", "Q8_0",
        "Q2_0", "Q3_0", "Q4_0", "Q4_1", "Q5_0", "Q5_1",
        "BF16", "F16", "F32",
    ] {
        if up.contains(tag) {
            return tag.to_string();
        }
    }
    String::new()
}

/// Scan the usual local caches for `.gguf` files so the panel can offer a
/// picker instead of a paste-the-path field. Best-effort: unreadable roots are
/// skipped, never errors on a missing cache.
#[command]
pub async fn kortex_gac_list_local_ggufs(extra_dir: Option<String>) -> Result<Vec<LocalGguf>, String> {
    tokio::task::spawn_blocking(move || {
        let mut roots: Vec<PathBuf> = Vec::new();
        if let Some(h) = dirs::home_dir() {
            roots.push(h.join(".cache/huggingface/hub"));
            roots.push(h.join(".lmstudio/models"));
        }
        if let Some(c) = dirs::cache_dir() {
            roots.push(c.join("lemonade"));
            roots.push(c.join("lemonade-server"));
            roots.push(c.join("huggingface/hub"));
        }
        for (var, sub) in [
            ("HF_HOME", "hub"),
            ("HF_HUB_CACHE", ""),
            ("HUGGINGFACE_HUB_CACHE", ""),
            ("LEMONADE_MODELS", ""),
            ("LEMONADE_CACHE", ""),
        ] {
            if let Ok(v) = std::env::var(var) {
                if !v.trim().is_empty() {
                    let p = PathBuf::from(v.trim());
                    roots.push(if sub.is_empty() { p } else { p.join(sub) });
                }
            }
        }
        if let Some(d) = extra_dir.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()) {
            roots.push(PathBuf::from(d));
        }

        let mut out: Vec<LocalGguf> = Vec::new();
        let mut seen = std::collections::HashSet::new();
        for root in roots {
            if !root.is_dir() {
                continue;
            }
            for entry in walkdir::WalkDir::new(&root)
                .max_depth(7)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let p = entry.path();
                if p.extension().and_then(|e| e.to_str()) != Some("gguf") {
                    continue;
                }
                let canon = std::fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf());
                if !seen.insert(canon.clone()) {
                    continue;
                }
                let file = p.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
                let flc = file.to_lowercase();
                let aux = flc.starts_with("mmproj")
                    || flc.contains("mmproj")
                    || flc.contains("embedding")
                    || flc.contains("-proj-");
                // repo = "Org/Repo" from a `models--Org--Repo` ancestor.
                let repo = p
                    .ancestors()
                    .find_map(|a| {
                        a.file_name()
                            .and_then(|n| n.to_str())
                            .filter(|n| n.starts_with("models--"))
                            .map(|n| n.trim_start_matches("models--").replace("--", "/"))
                    })
                    .unwrap_or_else(|| {
                        p.parent()
                            .and_then(|d| d.file_name())
                            .and_then(|n| n.to_str())
                            .unwrap_or("(local)")
                            .to_string()
                    });
                let size_mb = std::fs::metadata(p).map(|m| m.len() / (1024 * 1024)).unwrap_or(0);
                out.push(LocalGguf {
                    repo,
                    quant: parse_quant(&file),
                    file,
                    path: canon.to_string_lossy().into_owned(),
                    size_mb,
                    aux,
                });
            }
        }
        // chat models first, then repo, then largest (usually the least-quantised)
        out.sort_by(|a, b| {
            a.aux
                .cmp(&b.aux)
                .then_with(|| a.repo.to_lowercase().cmp(&b.repo.to_lowercase()))
                .then_with(|| b.size_mb.cmp(&a.size_mb))
        });
        Ok(out)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[cfg(test)]
mod local_gguf_tests {
    use super::parse_quant;

    #[test]
    fn parses_common_quant_tags() {
        assert_eq!(parse_quant("Qwen3.8-27B-AD-IQ3_XXS.gguf"), "IQ3_XXS");
        assert_eq!(parse_quant("model.Q4_K_M.gguf"), "Q4_K_M");
        assert_eq!(parse_quant("Escha-W2-35B-A3B-Q2_0_ROCMFPX.gguf"), "Q2_0_ROCMFPX");
    }

    #[test]
    fn q4_k_m_not_shadowed_by_q4_0() {
        // "Q4_K_M" must win over the shorter "Q4_0" substring check order.
        assert_eq!(parse_quant("foo-Q4_K_M-bar.gguf"), "Q4_K_M");
        assert_eq!(parse_quant("foo-Q4_0-bar.gguf"), "Q4_0");
    }

    #[test]
    fn unknown_returns_empty() {
        assert_eq!(parse_quant("mmproj-Qwen3.8-27B-BF16.gguf"), "BF16");
        assert_eq!(parse_quant("some-random-model.gguf"), "");
    }
}
