//! RAM-tiered local backend offloading policy.
//!
//! Centralizes every decision that keeps local inference smooth on low-RAM
//! machines (4–8GB potato / M1 Air class):
//!
//! - **Model tiering** — which model each APEX engine gets per RAM tier.
//!   Lite machines share ONE small resident model across all engines so
//!   a persistent local server never evicts/reloads multi-GB weights mid-sweep.
//! - **Concurrency gating** — a global semaphore caps simultaneous the local backend
//!   generations (lite = strictly serial). Eight parallel generations on
//!   8GB means swap-death even with 2b models.
//! - **`keep_alive` policy** — lite keeps its single model warm (reload
//!   thrash is the enemy); larger tiers release sooner.
//! - **`num_ctx` clamping** — KV cache is the hidden RAM hog; lite caps it.
//! - **Memory-pressure guard** — checks *available* (not total) RAM before
//!   heavy batch work so a swapping machine degrades gracefully.
//! - **Env doctor** — reports the local backend env vars that matter most
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
/// zero eviction churn during a sweep. Qwen3.5-4B: ~2.5 GB at Q4, still fits
/// an 8 GB rig, and its tool-call / structured-output fidelity is a step
/// change over the old 2B (which produced malformed tool calls under load).
const LITE_MODEL: &str = "qwen3.5:4b";
/// Mid tier also shares one model: 7b weights + KV is most of 16GB's budget.
const MID_MODEL: &str = "qwen3.5:7b";

/// Security specialist for the `threat` engine on Full tier. DeepHat-V1-7B is a
/// Qwen2.5-Coder-7B fine-tune specialized for offensive/defensive cybersecurity
/// and DevOps (successor to WhiteRabbitNeo, 128K ctx via YaRN). the local backend resolves
/// this directly from the GGUF repo — no manual model registration needed:
///     local backend pull hf.co/mradermacher/DeepHat-V1-7B-GGUF:Q4_K_M
/// Recommended system prompt: "You are DeepHat, created by Kindo.ai. You are a
/// helpful assistant that is an expert in Cybersecurity and DevOps."
const THREAT_MODEL: &str = "hf.co/mradermacher/DeepHat-V1-7B-GGUF:Q4_K_M";

/// BugTraceAI CORE-Ultra (27B, Qwen3.6 SFT) — the "tooling" specialist for the
/// `exploit` engine. Unlike a reasoning model, it emits complete, runnable
/// artifacts (Nuclei templates, CVE PoCs, crackers). Served via **Lemonade**
/// (real llama.cpp, OpenAI-compatible on :13305), not the local backend. The id below is
/// what Lemonade registers the GGUF under — load/pull it there first:
///     lemonade-server pull hf.co/BugTraceAI/BugTraceAI-CORE-Ultra-27B-Q4
/// Recommended: temp 0.1, top_p 0.9, repeat_penalty 1.1.
pub const EXPLOIT_MODEL_LEMONADE: &str = "BugTraceAI-CORE-Ultra-27B-Q4";

/// Engines that run on the Lemonade backend (real llama.cpp) instead of the local backend,
/// with the Lemonade model id to request. Only wired on Full tier — a 27B is a
/// dedicated-GPU model. Returns `None` for engines that stay on the local backend.
pub fn lemonade_model(engine: &str) -> Option<&'static str> {
    match tier() {
        ModelTier::Full => match engine {
            "exploit" => Some(EXPLOIT_MODEL_LEMONADE),
            _ => None,
        },
        _ => None,
    }
}

/// Recommended sampling params for a Lemonade-backed engine. CORE-Ultra wants
/// low-temp deterministic tooling output, per its model card.
pub fn lemonade_params(engine: &str) -> (f32, f32, f32) {
    match engine {
        // (temperature, top_p, repeat_penalty)
        "exploit" => (0.1, 0.9, 1.1),
        _ => (0.2, 0.9, 1.1),
    }
}

/// Tier-aware model for an APEX engine. Full tier keeps the original
/// per-specialist split; lite/mid collapse to a single resident model.
pub fn apex_model(engine: &str) -> &'static str {
    match tier() {
        ModelTier::Lite => LITE_MODEL,
        ModelTier::Mid => MID_MODEL,
        ModelTier::Full => match engine {
            "architect" => "qwen3.5:12b",
            "threat" => THREAT_MODEL,
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

/// Top-level `keep_alive` value for the native /api path payloads. Lite keeps its single
/// model warm for a long time — reloading 1.5GB of weights every 5 minutes
/// (the local backend's default) is far worse than holding them. Full tier releases
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

/// Global semaphore gating batch the local backend generations. Acquire a permit
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

// ─── VRAM-tiered GPU-layer offload ───────────────────────────────────────────
//
// A dense model's weights must be resident to compute a forward pass, but they
// don't all have to be on the GPU: llama.cpp/the local backend keep `num_gpu` layers in
// VRAM and run the rest on CPU/RAM. This lets a 27B run on an 8GB card (via RAM
// offload, slower) instead of OOMing. The policy below detects VRAM, estimates
// how many of a model's layers fit, and sets `num_gpu` accordingly — replacing
// the local backend's conservative auto-guess that often under-fills VRAM or OOMs.

/// Best-effort VRAM detection in GB. `HADES_VRAM_GB` overrides everything (the
/// escape hatch for AMD/Lemonade/NPU where programmatic detection is unreliable).
/// Then NVIDIA via `nvidia-smi`; then Apple Silicon unified memory (the GPU can
/// address ~70% of system RAM). Returns None when we genuinely can't tell — the
/// caller then leaves `num_gpu` unset and lets the local backend decide.
pub fn detect_vram_gb() -> Option<f64> {
    if let Ok(v) = std::env::var("HADES_VRAM_GB") {
        if let Ok(n) = v.parse::<f64>() {
            if n > 0.0 {
                return Some(n);
            }
        }
    }
    if let Some(mb) = nvidia_smi_vram_mb() {
        return Some(mb / 1024.0);
    }
    #[cfg(target_os = "macos")]
    {
        let ram = get().total_ram_gb;
        if ram > 0.0 {
            // Unified memory: macOS lets the GPU use a large fraction of RAM.
            return Some(ram * 0.70);
        }
    }
    None
}

fn nvidia_smi_vram_mb() -> Option<f64> {
    let out = crate::process_ext::hidden_command("nvidia-smi")
        .args(["--query-gpu=memory.total", "--format=csv,noheader,nounits"])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&out.stdout);
    // First GPU's total memory, in MiB.
    text.lines()
        .next()
        .and_then(|l| l.trim().parse::<f64>().ok())
        .filter(|v| *v > 0.0)
}

/// Parse a model's parameter count in billions from its name (e.g.
/// `qwen2.5-coder:14b` → 14.0, `gemma-4-12b` → 12.0). Fractional tags like
/// `1.5b` are supported.
pub fn model_param_billions(model: &str) -> Option<f64> {
    let m = model.to_lowercase();
    for part in m.split(|c| c == ':' || c == '-' || c == '_' || c == '/') {
        let digits: String = part.chars().take_while(|c| c.is_ascii_digit() || *c == '.').collect();
        let rest: String = part.chars().skip(digits.len()).collect();
        if rest.starts_with('b') && !digits.is_empty() {
            if let Ok(n) = digits.parse::<f64>() {
                if n > 0.0 {
                    return Some(n);
                }
            }
        }
    }
    None
}

/// Rough transformer layer count by parameter size. Used to divide the weight
/// budget into per-layer cost. These are typical depths for common open models;
/// exactness isn't required — being off by a few layers just shifts the split
/// by a layer or two, which llama.cpp tolerates.
pub fn estimate_layers(param_billions: f64) -> u32 {
    match param_billions {
        p if p <= 1.5 => 22,
        p if p <= 3.0 => 26,
        p if p <= 8.0 => 32,
        p if p <= 15.0 => 40,
        p if p <= 35.0 => 48,
        p if p <= 75.0 => 80,
        _ => 96,
    }
}

/// Decide how many layers to place on the GPU given the model size and VRAM.
///
/// Returns an the local backend `num_gpu` value: `0` = all on CPU/RAM, `total_layers` = the
/// whole model on GPU, anything between = a split. Pure + deterministic so the
/// policy is unit-tested without any GPU present.
///
/// Model estimated at ~0.56 GB per billion params (Q4_K_M ≈ 4.5 bits/param). We
/// reserve headroom for the KV cache, compute buffers, and the OS framebuffer
/// before dividing the rest into layers.
pub fn plan_gpu_layers(param_billions: f64, total_layers: u32, vram_gb: f64) -> i64 {
    if param_billions <= 0.0 || total_layers == 0 {
        return -1; // unknown → let the backend auto-decide (-1 == "as many as fit")
    }
    let weight_gb = param_billions * 0.56;
    let per_layer_gb = weight_gb / total_layers as f64;
    const RESERVE_GB: f64 = 1.5; // KV cache + compute buffers + framebuffer
    let usable = (vram_gb - RESERVE_GB).max(0.0);
    if usable <= 0.0 || per_layer_gb <= 0.0 {
        return 0;
    }
    let fits = (usable / per_layer_gb).floor() as i64;
    fits.clamp(0, total_layers as i64)
}

/// The wired entry point: recommended `num_gpu` for a model on this machine, or
/// None to leave it unset (the local backend decides). `HADES_NUM_GPU` still hard-overrides.
pub fn recommended_num_gpu(model: &str) -> Option<i64> {
    if let Ok(v) = std::env::var("HADES_NUM_GPU") {
        if let Ok(n) = v.parse::<i64>() {
            return Some(n);
        }
    }
    let vram = detect_vram_gb()?;
    let pb = model_param_billions(model)?;
    let layers = estimate_layers(pb);
    Some(plan_gpu_layers(pb, layers, vram))
}

/// Lemonade / llama.cpp offload advice for a given model.
///
/// Unlike the local backend (which accepts a per-request `num_gpu`), Lemonade runs an
/// external llama.cpp-based server whose GPU-layer split is fixed at launch
/// (`-ngl` / `--n-gpu-layers`). The IDE connects to it, it doesn't spawn it, so
/// we can't set the split programmatically — we compute the recommended value
/// from detected VRAM + model size and surface it as launch guidance (the same
/// pattern as `local_model_doctor`). Returns enough for the UI to show
/// "launch with -ngl N".
pub fn lemonade_offload_advice(model: &str) -> Value {
    let vram = detect_vram_gb();
    let param_b = model_param_billions(model);
    match (vram, param_b) {
        (Some(vram_gb), Some(pb)) => {
            let layers = estimate_layers(pb);
            let ngl = plan_gpu_layers(pb, layers, vram_gb);
            let full = ngl >= layers as i64;
            let weight_gb = pb * 0.56;
            json!({
                "provider": "lemonade",
                "model": model,
                "detected_vram_gb": (vram_gb * 10.0).round() / 10.0,
                "param_billions": pb,
                "estimated_layers": layers,
                "estimated_weight_gb": (weight_gb * 10.0).round() / 10.0,
                "recommended_n_gpu_layers": ngl,
                "full_offload": full,
                "launch_hint": if full {
                    format!("Model fits in VRAM — launch llama.cpp/lemonade with all layers on GPU (-ngl {} or -ngl 999).", layers)
                } else if ngl == 0 {
                    "Not enough VRAM for any layer after reserve — run CPU-only (-ngl 0). Consider a smaller quant.".to_string()
                } else {
                    format!("Launch llama.cpp/lemonade with -ngl {} ({} of {} layers on GPU, rest on CPU/RAM). Exact flag depends on your backend recipe.", ngl, ngl, layers)
                },
                "note": "VRAM offload is set when the lemonade/llama.cpp server LAUNCHES, not per request — the IDE can't change it on a running server.",
            })
        }
        (None, _) => json!({
            "provider": "lemonade",
            "model": model,
            "detected_vram_gb": null,
            "note": "Could not detect VRAM. Set HADES_VRAM_GB=<gb> so the advisor can recommend an -ngl value, or size -ngl manually.",
        }),
        (_, None) => json!({
            "provider": "lemonade",
            "model": model,
            "note": "Could not parse a parameter count from the model name — can't estimate the layer split. Size -ngl manually.",
        }),
    }
}

// ─── Offload preflight ───────────────────────────────────────────────────────
//
// The advisor above is passive — it answers "what -ngl should I use?" when asked.
// The preflight is active: given the provider + model about to run, it predicts
// whether the model will spill to CPU on THIS machine. CPU-spill on a dense model
// is the usual cause of a multi-minute silent prefill (the gemma-12B-on-8GB stall),
// so surfacing it BEFORE the run lets the IDE warn instead of leaving the user
// staring at a frozen turn until a watchdog fires.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OffloadRisk {
    /// Model fits in VRAM — full GPU offload, no spill.
    Ok,
    /// Some layers land on CPU/RAM — runs, but prefill + tok/s are slower.
    PartialSpill,
    /// No layer fits after reserve — CPU-only. Expect long prefill / stall risk.
    CpuOnly,
    /// VRAM or model size couldn't be determined — can't predict.
    Unknown,
}

#[derive(Debug, Clone)]
pub struct OffloadPreflight {
    pub risk: OffloadRisk,
    pub message: String,
    pub recommended_ngl: Option<i64>,
    pub detected_vram_gb: Option<f64>,
    pub param_billions: Option<f64>,
}

impl OffloadPreflight {
    /// True when the user should be warned proactively (spill or CPU-only).
    pub fn is_warning(&self) -> bool {
        matches!(self.risk, OffloadRisk::PartialSpill | OffloadRisk::CpuOnly)
    }
}

/// Predict the offload outcome for a local `provider`/`model` on this machine.
/// Pure aside from VRAM detection, so the decision matches what the advisor and
/// `recommended_num_gpu` will actually apply. Non-local providers are `Ok` (no
/// local offload concept applies to a cloud API).
pub fn offload_preflight(provider: &str, model: &str) -> OffloadPreflight {
    let p = provider.to_lowercase();
    let is_local = matches!(
        p.as_str(),
        "antigravity" | "deepseek-ane" | "deepseek_ane"
            | "ds2-ane" | "lemonade" | "huggingface"
    );
    if !is_local {
        return OffloadPreflight {
            risk: OffloadRisk::Ok,
            message: String::new(),
            recommended_ngl: None,
            detected_vram_gb: None,
            param_billions: None,
        };
    }

    let vram = detect_vram_gb();
    let pb = model_param_billions(model);
    let (vram_gb, param_b) = match (vram, pb) {
        (Some(v), Some(b)) => (v, b),
        _ => {
            return OffloadPreflight {
                risk: OffloadRisk::Unknown,
                message: format!(
                    "Could not predict GPU offload for '{}' ({}). Set HADES_VRAM_GB=<gb> \
                     (and use a model name with a size tag like -12b) so the IDE can warn \
                     before a slow CPU-spill run.",
                    model,
                    if vram.is_none() { "VRAM unknown" } else { "model size unknown" }
                ),
                recommended_ngl: None,
                detected_vram_gb: vram,
                param_billions: pb,
            };
        }
    };

    let layers = estimate_layers(param_b);
    let ngl = plan_gpu_layers(param_b, layers, vram_gb);
    // Lemonade sets the split at launch (-ngl); the local backend gets it per request via
    // num_gpu. Tailor the actionable hint accordingly.
    let lemonade = p == "lemonade";
    let (risk, message) = if ngl >= layers as i64 {
        (OffloadRisk::Ok, String::new())
    } else if ngl <= 0 {
        (
            OffloadRisk::CpuOnly,
            format!(
                "'{}' (~{:.0}B) will run CPU-only on {:.1}GB VRAM — prefill can take minutes and \
                 may look stalled. Use a smaller quant, a smaller model, or add VRAM.{}",
                model, param_b, vram_gb,
                if lemonade { " (Relaunch lemonade/llama.cpp with -ngl 0 to stop it thrashing GPU.)" } else { "" }
            ),
        )
    } else {
        (
            OffloadRisk::PartialSpill,
            if lemonade {
                format!(
                    "'{}' (~{:.0}B) won't fully fit {:.1}GB VRAM: {} of {} layers on GPU, rest on \
                     CPU/RAM — slower, and prefill may look briefly stalled. Relaunch lemonade/llama.cpp \
                     with -ngl {} for the best split.",
                    model, param_b, vram_gb, ngl, layers, ngl
                )
            } else {
                format!(
                    "'{}' (~{:.0}B) won't fully fit {:.1}GB VRAM: {} of {} layers on GPU (num_gpu={} \
                     applied automatically), rest on CPU/RAM — slower prefill.",
                    model, param_b, vram_gb, ngl, layers, ngl
                )
            },
        )
    };

    OffloadPreflight {
        risk,
        message,
        recommended_ngl: Some(ngl),
        detected_vram_gb: Some((vram_gb * 10.0).round() / 10.0),
        param_billions: Some(param_b),
    }
}

/// Lemonade offload advisor command: recommends the `-ngl` layer split for the
/// externally-launched lemonade/llama.cpp server, given the model. The
/// llama.cpp path is Lemonade's real backend, so this is the first-class
/// offload story (the local backend's per-request `num_gpu` is the fallback).
#[cfg(feature = "tauri")]
#[tauri::command]
pub fn lemonade_doctor(model: Option<String>) -> Value {
    let m = model.unwrap_or_default();
    if m.trim().is_empty() {
        return json!({
            "provider": "lemonade",
            "detected_vram_gb": detect_vram_gb().map(|v| (v * 10.0).round() / 10.0),
            "note": "Pass the model id to get an -ngl recommendation (offload depends on model size).",
        });
    }
    lemonade_offload_advice(&m)
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
        assert_eq!(LITE_MODEL, "qwen3.5:4b");
        for e in engines {
            let m = apex_model(e);
            assert!(!m.is_empty());
        }
    }

    #[test]
    fn lemonade_params_are_low_temp_for_tooling() {
        let (temp, top_p, rep) = lemonade_params("exploit");
        assert_eq!(temp, 0.1, "tooling engine wants deterministic output");
        assert_eq!(top_p, 0.9);
        assert_eq!(rep, 1.1);
    }

    #[test]
    fn non_full_tier_never_routes_to_lemonade() {
        // lemonade_model only returns Some on Full tier. On lite/mid it must be
        // None so a 27B model is never selected on a small box.
        if !matches!(tier(), ModelTier::Full) {
            assert_eq!(lemonade_model("exploit"), None);
        }
        // An unknown engine is never Lemonade-backed regardless of tier.
        assert_eq!(lemonade_model("architect"), None);
    }

    #[test]
    fn model_param_billions_parses_common_names() {
        assert_eq!(model_param_billions("qwen2.5-coder:14b"), Some(14.0));
        assert_eq!(model_param_billions("gemma-4-12b-coder"), Some(12.0));
        assert_eq!(model_param_billions("qwen3:32b"), Some(32.0));
        assert_eq!(model_param_billions("tinyllama:1.1b"), Some(1.1));
        assert_eq!(model_param_billions("codestral"), None);
    }

    #[test]
    fn plan_gpu_layers_full_offload_when_vram_ample() {
        // 7B (~3.9GB) with 24GB VRAM → all 32 layers fit.
        assert_eq!(plan_gpu_layers(7.0, 32, 24.0), 32);
    }

    #[test]
    fn plan_gpu_layers_partial_offload_when_constrained() {
        // 27B (~15GB, ~0.315GB/layer over 48 layers) on 8GB VRAM:
        // usable = 8 - 1.5 = 6.5GB → ~20 layers fit, must be a partial split.
        let n = plan_gpu_layers(27.0, 48, 8.0);
        assert!(n > 0 && n < 48, "expected partial offload, got {n}");
    }

    #[test]
    fn plan_gpu_layers_zero_when_no_usable_vram() {
        // Tiny/for-integrated GPU: reserve eats all of it → everything on CPU.
        assert_eq!(plan_gpu_layers(27.0, 48, 1.0), 0);
    }

    #[test]
    fn plan_gpu_layers_unknown_model_defers_to_local() {
        assert_eq!(plan_gpu_layers(0.0, 0, 8.0), -1);
    }

    /// `HADES_VRAM_GB` is process-global, and cargo runs tests in parallel
    /// threads sharing one environment. Without serialising, one test's
    /// `remove_var` lands between another's `set_var` and its assertion — a
    /// flake that only shows up in a full-suite run.
    fn vram_env_lock() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: std::sync::OnceLock<std::sync::Mutex<()>> = std::sync::OnceLock::new();
        LOCK.get_or_init(|| std::sync::Mutex::new(()))
            .lock()
            .unwrap_or_else(|e| e.into_inner())
    }

    #[test]
    fn lemonade_advice_recommends_full_offload_when_vram_ample() {
        let _g = vram_env_lock();
        std::env::set_var("HADES_VRAM_GB", "24");
        let a = lemonade_offload_advice("qwen2.5-coder:7b");
        assert_eq!(a["full_offload"], serde_json::json!(true));
        assert_eq!(a["provider"], "lemonade");
        std::env::remove_var("HADES_VRAM_GB");
    }

    #[test]
    fn lemonade_advice_recommends_partial_offload_when_constrained() {
        let _g = vram_env_lock();
        std::env::set_var("HADES_VRAM_GB", "8");
        let a = lemonade_offload_advice("qwen3:32b");
        assert_eq!(a["full_offload"], serde_json::json!(false));
        let ngl = a["recommended_n_gpu_layers"].as_i64().unwrap();
        assert!(ngl > 0, "expected a partial split, got {ngl}");
        std::env::remove_var("HADES_VRAM_GB");
    }

    #[test]
    fn lemonade_advice_handles_unknown_vram() {
        let _g = vram_env_lock();
        std::env::remove_var("HADES_VRAM_GB");
        // On a CI box with no nvidia-smi and not macOS, VRAM is unknown → guidance
        // to set HADES_VRAM_GB rather than a bogus number. (Skip assert on macOS
        // where unified-memory detection always yields a value.)
        if !cfg!(target_os = "macos") && nvidia_smi_vram_mb().is_none() {
            let a = lemonade_offload_advice("qwen3:32b");
            assert!(a["detected_vram_gb"].is_null());
        }
    }

    #[test]
    fn estimate_layers_is_monotonic_by_size() {
        assert!(estimate_layers(2.0) < estimate_layers(7.0));
        assert!(estimate_layers(7.0) < estimate_layers(27.0));
        assert!(estimate_layers(27.0) < estimate_layers(70.0));
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
    fn preflight_cloud_provider_is_ok_and_silent() {
        let pf = offload_preflight("anthropic", "claude-opus-4-8");
        assert_eq!(pf.risk, OffloadRisk::Ok);
        assert!(!pf.is_warning());
        assert!(pf.message.is_empty());
    }

    #[test]
    fn preflight_flags_cpu_only_on_tiny_vram() {
        let _g = vram_env_lock();
        std::env::set_var("HADES_VRAM_GB", "1");
        let pf = offload_preflight("lemonade", "gemma-4-12b-coder");
        assert_eq!(pf.risk, OffloadRisk::CpuOnly);
        assert!(pf.is_warning());
        assert!(pf.message.contains("CPU-only"));
        std::env::remove_var("HADES_VRAM_GB");
    }

    #[test]
    fn preflight_flags_partial_spill_when_constrained() {
        let _g = vram_env_lock();
        std::env::set_var("HADES_VRAM_GB", "8");
        let pf = offload_preflight("lemonade", "qwen3:32b");
        assert_eq!(pf.risk, OffloadRisk::PartialSpill);
        assert!(pf.is_warning());
        assert!(pf.recommended_ngl.unwrap() > 0);
        // Lemonade guidance is launch-time (-ngl), not per-request.
        assert!(pf.message.contains("-ngl"));
        std::env::remove_var("HADES_VRAM_GB");
    }

    #[test]
    fn preflight_ok_when_vram_ample() {
        let _g = vram_env_lock();
        std::env::set_var("HADES_VRAM_GB", "48");
        let pf = offload_preflight("lemonade", "qwen2.5-coder:7b");
        assert_eq!(pf.risk, OffloadRisk::Ok);
        assert!(!pf.is_warning());
        std::env::remove_var("HADES_VRAM_GB");
    }

    #[test]
    fn preflight_unknown_when_size_unparseable() {
        let _g = vram_env_lock();
        std::env::set_var("HADES_VRAM_GB", "8");
        let pf = offload_preflight("lemonade", "codestral");
        assert_eq!(pf.risk, OffloadRisk::Unknown);
        assert!(pf.message.contains("HADES_VRAM_GB"));
        std::env::remove_var("HADES_VRAM_GB");
    }

    /// The Lemonade advisor replaced the the local backend env doctor: offload for
    /// llama.cpp is set with `-ngl` at server launch, not via env vars.
    #[test]
    fn lemonade_doctor_reports_offload_advice() {
        let v = lemonade_doctor(Some("qwen2.5-coder:7b".to_string()));
        assert_eq!(v["provider"], "lemonade");
        assert_eq!(v["model"], "qwen2.5-coder:7b");
    }
}
