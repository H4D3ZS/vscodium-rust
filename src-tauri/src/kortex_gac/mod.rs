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
    lookup_cache: Option<String>,
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
        lookup_cache: lookup_cache.filter(|s| !s.trim().is_empty()).map(PathBuf::from),
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
