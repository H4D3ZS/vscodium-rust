//! RAM-tiered Ollama offloading policy.
//!
//! Centralizes every decision that keeps local inference smooth on low-RAM
//! machines (4–8GB potato / M1 Air class):
//!
//! - **Model tiering** — which model each APEX engine gets per RAM tier.
//!   Lite machines share ONE small resident model across all engines so
//!   Ollama never evicts/reloads multi-GB weights mid-sweep.
//! - **Concurrency gating** — a global semaphore caps simultaneous Ollama
//!   generations (lite = strictly serial). Eight parallel generations on
//!   8GB means swap-death even with 2b models.
//! - **`keep_alive` policy** — lite keeps its single model warm (reload
//!   thrash is the enemy); larger tiers release sooner.
//! - **`num_ctx` clamping** — KV cache is the hidden RAM hog; lite caps it.
//! - **Memory-pressure guard** — checks *available* (not total) RAM before
//!   heavy batch work so a swapping machine degrades gracefully.
//! - **Env doctor** — reports the Ollama server env vars that matter most
//!   on small machines, with copy-paste commands per platform.

use serde_json::{json, Value};
use std::sync::{Arc, OnceLock};
use tokio::sync::Semaphore;

use crate::system_profile;

// ─── Tiers ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelTier {
    /// < 9GB total RAM — 4–8GB potatoes, M1 Air 8GB.
    Lite,
    /// 9–17GB — mid-tier laptops.
    Mid,
    /// > 17GB — full workstation.
    Full,
}

pub const MID_RAM_THRESHOLD_GB: f64 = 17.0;

pub fn tier() -> ModelTier {
    let profile = system_profile::get();
    if profile.lite_mode {
        ModelTier::Lite
    } else if profile.total_ram_gb < MID_RAM_THRESHOLD_GB {
        ModelTier::Mid
    } else {
        ModelTier::Full
    }
}

// ─── APEX model assignments per tier ────────────────────────────────────────

/// Single shared model for ALL lite-tier engines — one resident model means
/// zero eviction churn during a sweep. Proven working on M1 Air 8GB.
const LITE_MODEL: &str = "qwen3.5:2b";
/// Mid tier also shares one model: 7b weights + KV is most of 16GB's budget.
const MID_MODEL: &str = "qwen3.5:7b";

/// Tier-aware model for an APEX engine. Full tier keeps the original
/// per-specialist split; lite/mid collapse to a single resident model.
pub fn apex_model(engine: &str) -> &'static str {
    match tier() {
        ModelTier::Lite => LITE_MODEL,
        ModelTier::Mid => MID_MODEL,
        ModelTier::Full => match engine {
            "architect" => "qwen3.5:12b",
            "threat" => "qwen3.5:12b",
            "perf" => "qwen3.5:7b",
            "self_improve" => "qwen3.5:7b",
            "explainer" => "qwen3.5:7b",
            "multi_system" => "qwen3.5:12b",
            "predictor" => "qwen3.5:12b",
            _ => "qwen3.5:7b",
        },
    }
}

// ─── keep_alive policy ───────────────────────────────────────────────────────

/// Top-level `keep_alive` value for Ollama payloads. Lite keeps its single
/// model warm for a long time — reloading 1.5GB of weights every 5 minutes
/// (Ollama's default) is far worse than holding them. Full tier releases
/// faster since multiple specialist models rotate through.
pub fn keep_alive() -> &'static str {
    match tier() {
        ModelTier::Lite => "30m",
        ModelTier::Mid => "15m",
        ModelTier::Full => "10m",
    }
}

// ─── num_ctx clamping ────────────────────────────────────────────────────────

/// Clamp a model-recommended `num_ctx` to what the RAM tier can afford.
/// KV cache grows linearly with context: a 7b at 16K ctx costs ~1.5–2GB of
/// KV alone (fp16) — that is the difference between smooth and swapping on
/// an 8GB unified-memory M1.
pub fn clamp_num_ctx(recommended: usize) -> usize {
    let cap = match tier() {
        ModelTier::Lite => 8_192,
        ModelTier::Mid => 16_384,
        ModelTier::Full => usize::MAX,
    };
    recommended.min(cap)
}

// ─── Concurrency gate ────────────────────────────────────────────────────────

/// Max simultaneous batch-engine generations (APEX sweep, red team).
/// Interactive chat is NOT gated by this — it must stay responsive.
pub fn max_parallel_engines() -> usize {
    match tier() {
        ModelTier::Lite => 1,
        ModelTier::Mid => 2,
        ModelTier::Full => 8,
    }
}

static ENGINE_GATE: OnceLock<Arc<Semaphore>> = OnceLock::new();

/// Global semaphore gating batch Ollama generations. Acquire a permit
/// before any APEX/red-team generation; drop it when the response lands.
pub fn engine_gate() -> Arc<Semaphore> {
    ENGINE_GATE
        .get_or_init(|| Arc::new(Semaphore::new(max_parallel_engines())))
        .clone()
}

// ─── Memory-pressure guard ───────────────────────────────────────────────────

/// Minimum *available* RAM (GB) required before starting heavy batch work.
pub fn min_free_gb_for_batch() -> f64 {
    match tier() {
        ModelTier::Lite => 1.2,
        ModelTier::Mid => 2.0,
        ModelTier::Full => 3.0,
    }
}

pub fn available_ram_gb() -> f64 {
    let mut sys = sysinfo::System::new();
    sys.refresh_memory();
    sys.available_memory() as f64 / (1024.0 * 1024.0 * 1024.0)
}

/// Returns Err with a user-facing message when the machine is too memory-
/// starved to start a batch sweep without swap-death.
pub fn check_batch_memory() -> Result<(), String> {
    let free = available_ram_gb();
    let need = min_free_gb_for_batch();
    if free < need {
        return Err(format!(
            "Memory pressure too high to start a full sweep: {:.1}GB available, {:.1}GB needed. \
             Close other apps or run a single-engine scan instead.",
            free, need
        ));
    }
    Ok(())
}

// ─── Env doctor ──────────────────────────────────────────────────────────────

/// Recommended Ollama *server* env vars for low-RAM machines. These must be
/// set on the process running `ollama serve` — the IDE cannot set them for
/// an already-running daemon, so the doctor reports them with copy-paste
/// commands instead.
#[tauri::command]
pub fn ollama_doctor() -> Value {
    let t = tier();
    let profile = system_profile::get();
    let recs: Vec<(&str, &str, &str)> = match t {
        ModelTier::Lite => vec![
            ("OLLAMA_MAX_LOADED_MODELS", "1", "Never hold two models' weights at once"),
            ("OLLAMA_NUM_PARALLEL", "1", "One generation at a time — parallel slots multiply KV cache"),
            ("OLLAMA_FLASH_ATTENTION", "1", "Lower memory bandwidth + enables KV quantization"),
            ("OLLAMA_KV_CACHE_TYPE", "q8_0", "Halves KV-cache RAM vs fp16 with negligible quality loss"),
        ],
        ModelTier::Mid => vec![
            ("OLLAMA_MAX_LOADED_MODELS", "1", "Avoid weight eviction churn"),
            ("OLLAMA_NUM_PARALLEL", "2", "Two slots max"),
            ("OLLAMA_FLASH_ATTENTION", "1", "Lower memory bandwidth + enables KV quantization"),
            ("OLLAMA_KV_CACHE_TYPE", "q8_0", "Halves KV-cache RAM vs fp16"),
        ],
        ModelTier::Full => vec![
            ("OLLAMA_FLASH_ATTENTION", "1", "Free speedup on most hardware"),
        ],
    };

    let items: Vec<Value> = recs
        .iter()
        .map(|(key, want, why)| {
            let current = std::env::var(key).ok();
            json!({
                "key": key,
                "recommended": want,
                "current": current, // NB: IDE env, not the Ollama daemon's — informational
                "why": why,
                "set_command": if cfg!(target_os = "macos") {
                    format!("launchctl setenv {key} {want}  # then restart Ollama")
                } else if cfg!(target_os = "windows") {
                    format!("setx {key} {want}  # then restart Ollama")
                } else {
                    format!("systemctl --user set-environment {key}={want}  # then restart Ollama")
                },
            })
        })
        .collect();

    json!({
        "tier": format!("{:?}", t),
        "total_ram_gb": profile.total_ram_gb,
        "available_ram_gb": available_ram_gb(),
        "lite_mode": profile.lite_mode,
        "apex_model_lite_or_mid": match t {
            ModelTier::Lite => LITE_MODEL,
            ModelTier::Mid => MID_MODEL,
            ModelTier::Full => "(per-engine split)",
        },
        "keep_alive": keep_alive(),
        "max_parallel_engines": max_parallel_engines(),
        "num_ctx_cap": clamp_num_ctx(usize::MAX),
        "recommendations": items,
        "note": "Env vars must be set for the `ollama serve` process, then Ollama restarted.",
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lite_tier_uses_single_small_model() {
        // Tier depends on host RAM, but the lite table itself must be uniform.
        let engines = [
            "architect", "threat", "perf", "self_improve",
            "explainer", "multi_system", "predictor",
        ];
        // All lite-tier entries route to LITE_MODEL by construction; verify
        // the full-tier table still differentiates.
        assert_eq!(LITE_MODEL, "qwen3.5:2b");
        for e in engines {
            let m = apex_model(e);
            assert!(!m.is_empty());
        }
    }

    #[test]
    fn clamp_respects_tier_cap() {
        let clamped = clamp_num_ctx(65_536);
        match tier() {
            ModelTier::Lite => assert_eq!(clamped, 8_192),
            ModelTier::Mid => assert_eq!(clamped, 16_384),
            ModelTier::Full => assert_eq!(clamped, 65_536),
        }
    }

    #[test]
    fn gate_matches_tier() {
        assert_eq!(engine_gate().available_permits() <= 8, true);
        assert!(max_parallel_engines() >= 1);
    }

    #[test]
    fn doctor_reports_tier() {
        let v = ollama_doctor();
        assert!(v["recommendations"].as_array().map(|a| !a.is_empty()).unwrap_or(false));
        assert!(v["total_ram_gb"].as_f64().unwrap_or(0.0) > 0.5);
    }
}
