use crate::ai_engine::{AiRequest, ChatMessage, MessageContent, AiResponse};
use crate::ripgrep_search::{self, RipgrepQuery};
pub use crate::ripgrep_search::SearchResult;
use tauri::{State, AppHandle, Emitter};
use serde_json::{Value, json};
use std::path::PathBuf;
use std::fs;

/// Detect provider from model name string. Returns `(provider, model, local_url)`.
///
/// **Lemonade is the only local backend.** It runs real llama.cpp and serves
/// every local GGUF, so anything that isn't recognisably a cloud model name
/// routes there. `preferred_provider` still wins for local names, since Lemonade
/// serves qwen/llama/gemma GGUFs whose names look like cloud families.
fn detect_provider(model: &str, preferred_provider: Option<&str>) -> (String, String, Option<String>) {
    let ml = model.to_lowercase();

    let lemonade_url = std::env::var("LEMONADE_URL")
        .unwrap_or_else(|_| "http://localhost:13305".to_string());

    if let Some(p) = preferred_provider.map(str::trim).filter(|p| !p.is_empty()) {
        // Only honor the preference for local backends whose model names are
        // ambiguous; cloud models (claude/gemini/gpt) still name-sniff below.
        let is_cloud_name = ml.contains("claude") || ml.contains("gemini")
            || ml.contains("gpt") || ml.contains("o1") || ml.contains("o3");
        if !is_cloud_name && p.to_lowercase() == "lemonade" {
            return ("lemonade".into(), model.to_string(), Some(lemonade_url));
        }
    }

    if ml.contains("claude-opus-4-8") {
        ("highwayapi".into(), model.to_string(), None)
    } else if ml.contains("claude") {
        ("anthropic".into(), model.to_string(), None)
    } else if ml.contains("gemini") {
        ("google".into(), model.to_string(), None)
    } else if ml.contains("gpt") || ml.contains("o1") || ml.contains("o3") {
        ("openai".into(), model.to_string(), None)
    } else if ml.contains("glm") || ml.contains("huggingface") || ml.contains("hf") {
        ("huggingface".into(), model.to_string(), None)
    } else {
        // Everything else — bare GGUF family names, colon-tagged names, anything
        // unrecognised — is local, and local means Lemonade.
        ("lemonade".into(), model.to_string(), Some(lemonade_url))
    }
}

// ---------------------------------------------------------------------------
// Lemonade per-model tuning
//
// `ctx_size` and `llamacpp.args` are GLOBAL to the Lemonade server, not
// per-model, and only take effect at model load. Whichever process loaded a
// model last wins, silently: a model left running under another model's
// settings is up to 4x slower with no error (measured — the 35B does 7.8 tok/s
// under gemma's config vs 16.6 tok/s under its own). So we push the right
// values before loading, and force a reload when either one changed.
// ---------------------------------------------------------------------------

/// Small weights: room for a huge window, f16 KV cache, big batches.
const LEMONADE_ARGS_SMALL: &str = "-fa on -rea off -b 2048 -ub 2048";
/// Mid-size weights: quantized KV cache to keep the window affordable.
const LEMONADE_ARGS_MID: &str = "-fa on -ctk q8_0 -ctv q8_0 -rea off";
/// Large weights that spill past 16GB VRAM: offload MoE expert layers to CPU.
///
/// `-ncmoe 12`, not 8. Offloading MORE experts makes PREFILL FASTER, which is
/// counter-intuitive and cost real measurement to find. The bottleneck at a large
/// window is not the offload, it is the KV cache competing with weights for VRAM;
/// pushing experts to system RAM frees the room prefill needs. Measured on the
/// 35B-A3B Q4_K_M (2026-08-07), generation / prefill tok/s:
///
///   ctx 32768, -ncmoe  8 : 17.8 / 526   <- dies silently past 32k context
///   ctx 65536, -ncmoe  8 : 17.8 / 167   <- VRAM starved, prefill collapses
///   ctx 98304, -ncmoe 16 : 11.8 / 434   <- over-offloaded
///   ctx 98304, -ncmoe 12 : 13.3 / 500   <- best on both axes
///
/// Do not tune `-ncmoe` down on generation tok/s alone; re-measure prefill too.
const LEMONADE_ARGS_LARGE: &str = "-fa on -ctk q8_0 -ctv q8_0 -ncmoe 12 -rea off";

/// Weights above this spill past VRAM once KV cache and compute buffers land.
const LEMONADE_SAFE_GB: f32 = 13.0;

// ---------------------------------------------------------------------------
// Model roles — the basis of Cursor-style responsiveness on local hardware
//
// One model cannot serve both roles here. Inline completion has to answer in
// ~200-300ms to feel like Tab; the 35B generates at 13.3 tok/s and cannot do
// that at any context size. So roles are served by DIFFERENT models held
// resident at the same time.
//
// This requires Lemonade's `max_loaded_models >= 2`. Measured 2026-08-07: with
// it raised, the 35B (ctx 98304) and Qwen3-0.6B (ctx 8192) stay resident
// together as separate llama-server processes, each keeping the ctx_size and
// args in force AT ITS OWN LOAD TIME. That is the important subtlety — the
// "global config" is really load-time config, so per-role tuning survives once
// there is a slot for each. Warm completion latency measured 217-328ms.
// ---------------------------------------------------------------------------

/// What a request is for. Picks both the model and its tuning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelRole {
    /// Multi-turn tool-using work: the 35B MoE. Quality over latency.
    Agent,
    /// Inline completion / ghost text. Latency over quality — a suggestion that
    /// arrives after the keystroke it predicted is worthless.
    Completion,
    /// Single-shot editor assists: quick edit, next-edit prediction, explain,
    /// document, refactor, debug, multi-cursor, PR review.
    ///
    /// Between the other two. These need real capability — a 0.6B cannot refactor
    /// — but they are one-shot and interactive, so a mid-size model at 30 tok/s
    /// beats the 35B at 13.3. Falls back to the agent model when nothing better
    /// is resident, which keeps behaviour correct if slower.
    Edit,
}

/// Preferred model per role, best first. The first one Lemonade actually serves
/// wins, so an uninstalled preference degrades instead of failing.
pub fn lemonade_role_candidates(role: ModelRole) -> &'static [&'static str] {
    match role {
        // The 35B leads deliberately: it has carried long autonomous builds and
        // security work that single-turn benchmarks cannot measure. Do not
        // reorder this on benchmark scores alone.
        ModelRole::Agent => &[
            "Qwen3.6-35B-A3B-Abliterated-Heretic-GGUF-Q4_K_M",
            "Huihui-Qwen3.6-35B-A3B-abliterated-MTP-GGUF-Q2_K",
            "Huihui-gemma-4-12B-agentic-fable5-abliterated-i1-Q4_K_M",
        ],
        // Smallest first — this role is latency-bound, not quality-bound.
        ModelRole::Completion => &[
            "Qwen3-0.6B-GGUF-BF16",
            "Huihui-gemma-4-12B-agentic-fable5-abliterated-i1-Q4_K_M",
        ],
        // Capable-but-fast first. gemma-12B is 30.3 tok/s with 8/8 tool calls and
        // is abliterated, so it does not refuse security work. Deliberately NO
        // 0.6B here: it cannot refactor or review, and a fast wrong answer is
        // worse than a slow right one for these.
        ModelRole::Edit => &["Huihui-gemma-4-12B-agentic-fable5-abliterated-i1-Q4_K_M"],
    }
}

/// Path to the Lemonade Server executable, if installed.
///
/// `LEMONADE_SERVER_EXE` wins so a portable or non-default install works.
pub fn lemonade_server_exe() -> Option<std::path::PathBuf> {
    if let Ok(p) = std::env::var("LEMONADE_SERVER_EXE") {
        let p = std::path::PathBuf::from(p);
        if p.is_file() {
            return Some(p);
        }
    }
    #[cfg(windows)]
    {
        let mut roots: Vec<std::path::PathBuf> = Vec::new();
        if let Ok(local) = std::env::var("LOCALAPPDATA") {
            roots.push(std::path::PathBuf::from(local).join("lemonade_server").join("bin"));
        }
        roots.push(std::path::PathBuf::from(r"C:\Program Files\Lemonade Server\bin"));
        for r in roots {
            let p = r.join("LemonadeServer.exe");
            if p.is_file() {
                return Some(p);
            }
        }
    }
    None
}

/// Poll `GET /api/v1/health` until the server answers, or `timeout` elapses.
pub async fn lemonade_wait_healthy(base: &str, timeout: std::time::Duration) -> bool {
    let deadline = std::time::Instant::now() + timeout;
    let Ok(client) = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build()
    else {
        return false;
    };
    let url = format!("{}/api/v1/health", base.trim_end_matches('/'));
    loop {
        if let Ok(r) = client.get(&url).send().await {
            if r.status().is_success() {
                return true;
            }
        }
        if std::time::Instant::now() >= deadline {
            return false;
        }
        tokio::time::sleep(std::time::Duration::from_millis(750)).await;
    }
}

/// Bring Lemonade up and the agent model resident, so the FIRST prompt works.
///
/// Without this the first prompt after launching the IDE fails: Lemonade may not
/// be running yet, and the served-model guard deliberately passes when the
/// server is unreachable (it cannot tell "not served" from "not up"), so a
/// Claude Code process is spawned that cannot reach any backend. The user sees
/// an error on prompt one and success on prompt two, once the server has come up
/// on its own.
///
/// Also pre-loads the model. A cold load is ~20-30s for the 35B; paying it at
/// startup rather than inside the user's first prompt is the difference between
/// "slow app launch" and "the AI is broken".
///
/// Never fails the caller — returns a human-readable status either way, because
/// this runs at startup where nothing useful can be done with an error.
pub async fn lemonade_ensure_ready(base: &str, model: &str) -> String {
    if !lemonade_wait_healthy(base, std::time::Duration::from_secs(2)).await {
        let Some(exe) = lemonade_server_exe() else {
            return "Lemonade Server is not running and was not found on this machine.".into();
        };
        let mut cmd = std::process::Command::new(&exe);
        {
            use crate::process_ext::CommandExtHidden as _;
            // Detached: the server outlives the IDE and must not inherit our
            // console or die with us. Hidden: no stray window on launch.
            cmd.detached_sidecar();
        }
        cmd.stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null());
        if cmd.spawn().is_err() {
            return format!("Failed to start Lemonade Server at {}", exe.display());
        }
        if !lemonade_wait_healthy(base, std::time::Duration::from_secs(90)).await {
            return "Started Lemonade Server but it did not become healthy in 90s.".into();
        }
    }

    if model.trim().is_empty() {
        return "Lemonade is up. No model selected.".into();
    }
    if lemonade_loaded_models(base).await.iter().any(|m| m == model) {
        return format!("Lemonade ready; {model} already loaded.");
    }

    // Tuning must be applied BEFORE the load — ctx_size and llamacpp.args only
    // take effect at load time.
    apply_lemonade_tuning(base, model).await;
    if lemonade_loaded_models(base).await.iter().any(|m| m == model) {
        format!("Lemonade ready; {model} loaded.")
    } else {
        format!("Lemonade is up but {model} did not load — check `lemonade list`.")
    }
}

/// The context window actually in force, in tokens.
///
/// Prefers the `ctx_size` the running llama-server was loaded with (ground
/// truth), falling back to the tuned value for the selected model.
///
/// Exists because the UI used to GUESS the window from the model name — a
/// `35B` in the id mapped to 32768, so the status bar read "11/33k" while the
/// server was actually serving 98304. A wrong context number is not cosmetic on
/// this stack: over-reporting hides an impending overflow, and overflow returns
/// an empty HTTP 200 that looks like the agent hanging.
#[tauri::command]
pub async fn lemonade_context_window(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
) -> Result<u32, String> {
    let base = state.ai.engine.lemonade_base().await;
    let base = base.trim_end_matches('/');
    let model = state.ai.current_model.lock().await.clone();

    // Ground truth: what the loaded process is actually running with.
    if let Ok(client) = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build()
    {
        if let Ok(resp) = client.get(format!("{base}/api/v1/health")).send().await {
            if let Ok(body) = resp.json::<Value>().await {
                if let Some(rows) = body.get("all_models_loaded").and_then(|a| a.as_array()) {
                    if let Some(ctx) = rows
                        .iter()
                        .find(|m| {
                            m.get("model_name").and_then(|n| n.as_str()) == Some(model.as_str())
                        })
                        .and_then(|m| m.get("recipe_options"))
                        .and_then(|o| o.get("ctx_size"))
                        .and_then(|c| c.as_u64())
                    {
                        return Ok(ctx as u32);
                    }
                }
            }
        }
    }

    Ok(lemonade_tuning(&model).0)
}

/// Start Lemonade and preload the selected model. Safe to call repeatedly.
#[tauri::command]
pub async fn lemonade_autostart(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
) -> Result<String, String> {
    let base = state.ai.engine.lemonade_base().await;
    let model = state.ai.current_model.lock().await.clone();
    Ok(lemonade_ensure_ready(base.trim_end_matches('/'), &model).await)
}

/// Models that are **loaded right now**, from `GET /api/v1/health`.
///
/// Distinct from [`lemonade_served_models`], which lists what is *downloaded*.
/// Every caller here cares about residency, because loading a model costs the
/// agent model VRAM and therefore speed.
pub async fn lemonade_loaded_models(base: &str) -> Vec<String> {
    let Ok(client) = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build()
    else {
        return Vec::new();
    };
    let Ok(resp) = client
        .get(format!("{}/api/v1/health", base.trim_end_matches('/')))
        .send()
        .await
    else {
        return Vec::new();
    };
    let Ok(body) = resp.json::<Value>().await else {
        return Vec::new();
    };
    body.get("all_models_loaded")
        .and_then(|a| a.as_array())
        .map(|rows| {
            rows.iter()
                .filter_map(|m| m.get("model_name").and_then(|n| n.as_str()))
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

/// Pick the model to serve `role` with, falling back to `fallback` (normally the
/// user's chat model).
///
/// **Only returns a role model that is already resident.** It will never cause a
/// load. Co-residency is not free — measured on this box, holding a completion
/// and an embedding model alongside the 35B cost the agent ~30% on both
/// generation (12.4 -> 8.7 tok/s) and prefill (460 -> 320 tok/s), and raising
/// `-ncmoe` recovered almost none of it. The agent model is the one the user
/// works in, so a latency optimisation for a side feature must not slow it down.
///
/// Net effect: load a small model deliberately and these features get faster;
/// do nothing and they behave exactly as before.
pub async fn model_for_role_or(role: ModelRole, fallback: &str) -> String {
    let base = std::env::var("LEMONADE_URL")
        .unwrap_or_else(|_| "http://localhost:13305".to_string());
    model_for_role_or_at(role, fallback, &base).await
}

/// [`model_for_role_or`] against an explicit base URL.
///
/// Split out so tests can point at a dead port without mutating the process
/// environment — `LEMONADE_URL` is global, and setting it from one test leaks
/// into every other test running in parallel.
pub async fn model_for_role_or_at(role: ModelRole, fallback: &str, base: &str) -> String {
    let loaded = lemonade_loaded_models(base).await;
    lemonade_role_candidates(role)
        .iter()
        .find(|want| loaded.iter().any(|l| l == *want))
        .map(|m| m.to_string())
        .unwrap_or_else(|| fallback.to_string())
}

/// How long a client must wait for a full-length answer, in milliseconds.
///
/// Claude Code's default request timeout is far shorter than a local model needs.
/// Observed on the CLI: "Waiting for API response · will retry in 3m 40s" while
/// llama-server was healthily generating — prompt processed, 138 of 8192 output
/// tokens decoded, `has_next_token: true`. Nothing was wrong; the model is just
/// slower than the timeout. The retry then queues behind the still-running
/// request and makes the next attempt slower again.
///
/// Budget = output tokens / measured tok/s, plus a cold-prefill allowance, times
/// a margin, floored at 30 minutes.
///
/// The floor is set by COMPACTION, not by an ordinary turn, and that is why it is
/// so much larger than a single answer needs. Compaction re-prefills the entire
/// conversation with a cold KV cache and then generates a summary on top. Measured
/// on the 35B at ~68k of context: **over 16 minutes**. The previous 15-minute floor
/// expired mid-compaction, and the retry re-ran the whole thing from the start —
/// which presented as "Compacting conversation… (16m 4s)" and never converged.
pub fn claude_api_timeout_ms(expected_tok_s: f32, max_output_tokens: u32) -> u64 {
    let tps = if expected_tok_s > 0.0 { expected_tok_s } else { 12.0 };
    let generation_s = max_output_tokens as f32 / tps;
    const COLD_PREFILL_ALLOWANCE_S: f32 = 180.0;
    /// Compaction at ~68k measured at 16m+; 30 minutes covers it with margin.
    const FLOOR_S: f32 = 1800.0;
    let total_s = (generation_s + COLD_PREFILL_ALLOWANCE_S) * 1.5;
    (total_s.max(FLOOR_S) * 1000.0) as u64
}

/// The context window to declare to Claude Code, given the served `ctx_size`.
///
/// **Claude Code auto-compacts at ~80% of `CLAUDE_CODE_MAX_CONTEXT_TOKENS`**, and
/// that is the only lever that controls it — `CLAUDE_CODE_AUTO_COMPACT_WINDOW` was
/// measured to have no effect at all. So declaring the true window is a trap:
/// at ctx_size 98,304 compaction would not start until ~78,600, and Claude Code's
/// own system prompt and tool schemas are **36,700 tokens** on top of the
/// conversation (measured, not estimated). The compaction request would exceed the
/// window and fail with "summarization produced empty response" — a session that
/// cannot save itself.
///
/// 70% puts the trigger near 55k, where compaction is both possible and cheap:
/// its cost scales with conversation size, and at ~68k it took 16+ minutes.
pub fn claude_max_context_tokens(ctx_size: u32) -> u32 {
    (ctx_size as f32 * 0.70) as u32
}

/// The output-token budget to give Claude Code.
///
/// llama.cpp enforces `prompt + max_tokens <= n_ctx` on **every** request, so every
/// output token is headroom taken away from the prompt. Compaction is the request
/// where the prompt is largest, which makes a generous budget actively harmful:
/// measured, a session at ~85k asking for 8,192 needed ~93k of a 98,304 window and
/// died. 4,096 is still far more than a summary or a tool call ever uses.
pub fn claude_max_output_tokens() -> u32 {
    4_096
}

/// What Claude Code's own system prompt and tool schemas cost on EVERY request,
/// before a single line of conversation.
///
/// Measured, not estimated: a session that sent 106,119 tokens contained 69,413
/// tokens of actual history, so the fixed overhead is 36,706.
///
/// This number is why a local model is squeezed so much harder than a cloud one.
/// Against a 200k window it is 18%; against 98,304 it is **37% gone before you
/// type anything**.
pub const CLAUDE_CODE_OVERHEAD_TOKENS: u32 = 36_700;

/// Can this context window host a Claude Code session at all?
///
/// Found by a test, and it is not a rounding concern: at `ctx_size` 32,768 the
/// 36,700-token overhead **exceeds the entire window**, so such a model cannot
/// serve a single request no matter how good it is. Several entries in the tuning
/// table are configured at 32k.
///
/// The bar is the overhead plus enough room to be worth starting: with less than
/// ~8k of conversation the agent compacts almost immediately and makes no progress.
pub fn claude_code_viable_ctx(ctx_size: u32) -> bool {
    const MIN_USEFUL_CONVERSATION: u32 = 8_000;
    claude_max_context_tokens(ctx_size) >= CLAUDE_CODE_OVERHEAD_TOKENS + MIN_USEFUL_CONVERSATION
}

/// Tuning for the completion role: a small window and small batches, because
/// only the latency of the first token matters.
///
/// `-rea off` is not optional. Qwen3-0.6B is a reasoning model: measured without
/// it, every completion came back EMPTY — the whole 48-token budget was spent
/// inside `<think>`, which Lemonade's Anthropic adapter drops. With it, the same
/// prompts return in 217-328ms. This is the same failure as on the 35B, and it
/// applies to every Qwen3 model regardless of size.
const LEMONADE_ARGS_COMPLETION: &str = "-fa on -b 1024 -ub 1024 -rea off";

/// `(ctx_size, llamacpp_args, expected_tok_s)` for a role-served model.
///
/// The completion role gets a deliberately small window: 8192 is far more than
/// a cursor-local prompt needs, and a small KV cache leaves VRAM for the agent
/// model sharing the card.
pub fn lemonade_role_tuning(role: ModelRole, model: &str) -> (u32, &'static str, f32) {
    match role {
        // Edit-role models are ordinary models doing one-shot work — their own
        // measured tuning applies unchanged.
        ModelRole::Agent | ModelRole::Edit => lemonade_tuning(model),
        ModelRole::Completion => (8192, LEMONADE_ARGS_COMPLETION, 0.0),
    }
}

/// ctx_size and llamacpp.args measured on RX 9060 XT 16GB (ROCm, ~218 GB/s).
/// Applied before load. Returns `(ctx_size, llamacpp_args, expected_tok_s)`;
/// an `expected_tok_s` of 0.0 means "unmeasured" and disables the warm-up check.
///
/// `-rea off` is REQUIRED on Qwen3.6: it is a reasoning model, and left on it
/// spends the whole token budget inside `<think>`, which Lemonade's Anthropic
/// adapter drops — the caller sees an empty response, not an error.
pub fn lemonade_tuning(model: &str) -> (u32, &'static str, f32) {
    let m = model.to_lowercase();

    if m.contains("gemma-4-12b") && m.contains("i1-q4_k_m") {
        return (131072, LEMONADE_ARGS_SMALL, 30.3);
    }
    if m.contains("gemma-4-12b") && m.contains("q8_0") {
        return (65536, LEMONADE_ARGS_MID, 18.8);
    }
    if m.contains("qwen3.6-35b-a3b") && m.contains("q4_k_m") {
        // 98304, NOT 32768. At 32768 any request over ~32k tokens gets an empty
        // HTTP 200 forever: llama.cpp returns 400 for a prompt past n_ctx, and
        // Lemonade's Anthropic adapter converts that into a 200 with empty
        // content. Nothing is logged on either side, so the caller just waits.
        // An agent session crosses 32k within a few turns, so this reads as
        // "the model stopped responding" — it is the single worst failure here.
        return (98304, LEMONADE_ARGS_LARGE, 13.3);
    }
    // Same family at 2-bit. Fits VRAM whole (12.3GB, no offload) and still scores
    // 8/8 on tool calls — the "2-bit breaks tool calling" rule does NOT hold for
    // this model, though it does for gemma-4-26B-A4B. But Q2_K is compute-bound on
    // this ROCm build, so it generates SLOWER than the 4-bit despite being 40%
    // smaller (15.8 vs 17.8 tok/s). Fallback only: it costs quality for no speed.
    if m.contains("qwen3.6-35b-a3b") && m.contains("q2_k") {
        return (65536, LEMONADE_ARGS_MID, 15.8);
    }

    // Unknown model and no size to go on — assume mid-size, which is the only
    // guess that is merely suboptimal rather than broken in either direction.
    lemonade_tuning_by_size(LEMONADE_SAFE_GB)
}

/// Fallback tuning for a model that isn't in the measured table, keyed on the
/// GGUF weight size in GB reported by `/api/v1/models`.
fn lemonade_tuning_by_size(size_gb: f32) -> (u32, &'static str, f32) {
    if size_gb <= 8.0 {
        (131072, LEMONADE_ARGS_SMALL, 0.0)
    } else if size_gb <= LEMONADE_SAFE_GB {
        (65536, LEMONADE_ARGS_MID, 0.0)
    } else {
        // 65536, not 32768. An unknown large model is exactly the case where we
        // cannot afford the silent empty-200 failure, and 32k is below the ~21k
        // agent overhead plus any real conversation. 64k is the largest window
        // that reliably loads for a spilling model without measurement.
        (65536, LEMONADE_ARGS_LARGE, 0.0)
    }
}

/// Weight size in GB for `model` as reported by Lemonade's model catalog.
/// `None` when the server is unreachable or doesn't list the model.
async fn lemonade_model_size_gb(client: &reqwest::Client, base: &str, model: &str) -> Option<f32> {
    let resp = client
        .get(format!("{}/api/v1/models", base.trim_end_matches('/')))
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await
        .ok()?;
    if !resp.status().is_success() {
        return None;
    }
    let body: Value = resp.json().await.ok()?;
    let rows = body.get("data").and_then(|d| d.as_array()).or_else(|| body.as_array())?;
    rows.iter()
        .find(|m| m.get("id").and_then(|i| i.as_str()) == Some(model))
        .and_then(|m| m.get("size"))
        .and_then(|s| s.as_f64().or_else(|| s.as_str().and_then(|t| t.parse::<f64>().ok())))
        .map(|g| g as f32)
}

/// Canonical Lemonade model id.
///
/// Lemonade's Ollama-compatible `/api/tags` appends `:latest` to every name,
/// while its native `/api/v1/models` and the Anthropic endpoint use the bare id.
/// A `…:latest` value persisted from the tags list would otherwise fail every
/// request, so normalize before comparing or dispatching.
pub fn canonical_model_id(model: &str) -> &str {
    model.strip_suffix(":latest").unwrap_or(model)
}

/// Model IDs Lemonade currently serves as text-generation models (downloaded,
/// `llamacpp` recipe). Empty when the server is unreachable — callers must treat
/// that as "unknown", not as "nothing is served".
pub async fn lemonade_served_models(base: &str) -> Vec<String> {
    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
    {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    let Ok(resp) = client
        .get(format!("{}/api/v1/models", base.trim_end_matches('/')))
        .send()
        .await
    else {
        return Vec::new();
    };
    if !resp.status().is_success() {
        return Vec::new();
    }
    let Ok(body): Result<Value, _> = resp.json().await else {
        return Vec::new();
    };
    let Some(rows) = body.get("data").and_then(|d| d.as_array()).or_else(|| body.as_array()) else {
        return Vec::new();
    };
    // Non-LLM recipes (sd-cpp, whispercpp, kokoro) also appear in this list and
    // would be nonsense to hand to Claude Code.
    rows.iter()
        .filter(|m| m.get("downloaded").and_then(|d| d.as_bool()).unwrap_or(false))
        .filter(|m| m.get("recipe").and_then(|r| r.as_str()) == Some("llamacpp"))
        .filter_map(|m| m.get("id").and_then(|i| i.as_str()).map(str::to_string))
        .collect()
}

/// Tuning for `model`, preferring the measured table and falling back to the
/// weight size from the server's catalog when the model isn't in it.
async fn lemonade_resolved_tuning(
    client: &reqwest::Client,
    base: &str,
    model: &str,
) -> (u32, &'static str, f32) {
    let measured = lemonade_tuning(model);
    // Only the fallback arm returns 0.0 expected throughput; a measured hit
    // wins over anything the catalog says.
    if measured.2 > 0.0 {
        return measured;
    }
    match lemonade_model_size_gb(client, base, model).await {
        Some(gb) => lemonade_tuning_by_size(gb),
        None => measured,
    }
}

/// `(ctx_size, llamacpp_args)` the named model is currently loaded under, from
/// `GET /api/v1/health` → `all_models_loaded[].recipe_options`. `None` when the
/// model isn't resident (so it needs loading regardless).
async fn lemonade_loaded_settings(
    client: &reqwest::Client,
    base: &str,
    model: &str,
) -> Option<(u32, String)> {
    let resp = client
        .get(format!("{}/api/v1/health", base.trim_end_matches('/')))
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await
        .ok()?;
    let body: Value = resp.json().await.ok()?;
    let entry = body
        .get("all_models_loaded")?
        .as_array()?
        .iter()
        .find(|e| {
            e.get("model_name").and_then(|n| n.as_str()) == Some(model)
                && e.get("loaded").and_then(|l| l.as_bool()).unwrap_or(false)
        })?;
    let opts = entry.get("recipe_options")?;
    let ctx = opts.get("ctx_size")?.as_u64()? as u32;
    let args = opts
        .get("llamacpp_args")
        .and_then(|a| a.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
    Some((ctx, args))
}

/// `POST /api/v1/load`. Loads can take minutes — the caller's client timeout covers it.
async fn lemonade_load(client: &reqwest::Client, base: &str, model: &str) -> Result<(), String> {
    match client
        .post(format!("{}/api/v1/load", base))
        .json(&json!({ "model_name": model }))
        .send()
        .await
    {
        Ok(r) if r.status().is_success() => Ok(()),
        Ok(r) => Err(format!("load {} failed: HTTP {}", model, r.status())),
        Err(e) => Err(format!("load {} failed: {}", model, e)),
    }
}

/// One short generation, timed. Returns tok/s, or `None` if the probe itself
/// failed (unreachable server, malformed reply) — which is not evidence of a
/// bad load and must not trigger a reload.
async fn lemonade_probe_tok_s(client: &reqwest::Client, base: &str, model: &str) -> Option<f32> {
    let payload = json!({
        "model": model,
        "max_tokens": 80,
        "messages": [{ "role": "user", "content": "Count from 1 to 40, numbers only." }],
    });

    let started = std::time::Instant::now();
    let resp = client
        .post(format!("{}/v1/messages", base))
        .timeout(std::time::Duration::from_secs(120))
        .json(&payload)
        .send()
        .await
        .ok()?;
    if !resp.status().is_success() {
        return None;
    }
    let body: Value = resp.json().await.ok()?;
    let elapsed = started.elapsed().as_secs_f32();

    let out_tokens = body.get("usage")?.get("output_tokens")?.as_f64()? as f32;
    if out_tokens <= 0.0 || elapsed <= 0.0 {
        return None;
    }
    Some(out_tokens / elapsed)
}

/// Catch a bad load before a long session is wasted on it.
///
/// A model can load into a degraded state that sits ~25% below normal and
/// persists until reloaded — no config change fixes it. Within a single load,
/// run-to-run variance is only 8%, so a reading well under baseline is signal,
/// not noise. Measured repeatedly on this hardware.
///
/// Skipped when `expected_tps` is 0.0 (an unmeasured model has no baseline to
/// compare against, and crying wolf is worse than staying quiet). Reloads at
/// most once, then reports whatever the second reading says.
pub async fn lemonade_warm_check(
    client: &reqwest::Client,
    base: &str,
    model: &str,
    expected_tps: f32,
) -> Option<f32> {
    if expected_tps <= 0.0 {
        return None;
    }
    // 25% below expected. Variance within one load is only 8%, so this
    // threshold sits comfortably outside the noise band.
    let floor = expected_tps * 0.75;

    let first = lemonade_probe_tok_s(client, base, model).await?;
    if first >= floor {
        eprintln!(
            "[lemonade] warm-up: {:.1} tok/s (expected ~{:.1})",
            first, expected_tps
        );
        return Some(first);
    }

    eprintln!(
        "[lemonade] warm-up: {:.1} tok/s, expected ~{:.1} — BAD LOAD. Reloading {}...",
        first, expected_tps, model
    );
    if let Err(e) = lemonade_load(client, base, model).await {
        eprintln!("[lemonade] reload after bad load failed: {}", e);
        return Some(first);
    }

    // Re-measure once. Whatever this says, we stop here — a second reload has
    // never been observed to help, and looping would stall the session.
    match lemonade_probe_tok_s(client, base, model).await {
        Some(second) if second >= floor => {
            eprintln!(
                "[lemonade] warm-up after reload: {:.1} tok/s (expected ~{:.1}) — recovered",
                second, expected_tps
            );
            Some(second)
        }
        Some(second) => {
            eprintln!(
                "[lemonade] warm-up after reload: {:.1} tok/s, still under ~{:.1}. \
                 Continuing anyway — check for another process holding the GPU.",
                second, expected_tps
            );
            Some(second)
        }
        None => Some(first),
    }
}

/// Push the tuned `ctx_size` + `llamacpp.args` into Lemonade's global config and
/// reload `model` if either differs from what it is currently loaded under.
/// Config only takes effect at load time, so a changed value forces a reload
/// even when the model is already resident.
///
/// Returns the expected tok/s for the applied tuning (0.0 = unmeasured).
/// Best-effort: a server that doesn't expose these endpoints is not an error —
/// the request still goes through, just untuned.
pub async fn apply_lemonade_tuning(base_url: &str, model: &str) -> f32 {
    let base = base_url.trim().trim_end_matches('/');
    if base.is_empty() || model.trim().is_empty() {
        return 0.0;
    }
    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(300))
        .build()
    {
        Ok(c) => c,
        Err(_) => return 0.0,
    };

    let (want_ctx, want_args, expected_tps) = lemonade_resolved_tuning(&client, base, model).await;

    // Already loaded under exactly these settings — nothing to do.
    let current = lemonade_loaded_settings(&client, base, model).await;
    if let Some((cur_ctx, ref cur_args)) = current {
        if cur_ctx == want_ctx && cur_args == want_args {
            return expected_tps;
        }
    }

    let payload = json!({ "ctx_size": want_ctx, "llamacpp": { "args": want_args } });
    match client
        .post(format!("{}/api/v1/params", base))
        .timeout(std::time::Duration::from_secs(10))
        .json(&payload)
        .send()
        .await
    {
        Ok(r) if r.status().is_success() => {}
        Ok(r) => {
            eprintln!("[lemonade] config set failed: HTTP {}", r.status());
            return 0.0;
        }
        Err(e) => {
            eprintln!("[lemonade] config set failed: {}", e);
            return 0.0;
        }
    }

    eprintln!(
        "[lemonade] {} -> ctx_size={} args=\"{}\" (was {}), reloading",
        model,
        want_ctx,
        want_args,
        current
            .map(|(c, a)| format!("ctx_size={} args=\"{}\"", c, a))
            .unwrap_or_else(|| "not loaded".to_string())
    );

    // Model loads can take minutes; the client timeout above covers it.
    match lemonade_load(&client, base, model).await {
        Ok(()) => {
            lemonade_warm_check(&client, base, model, expected_tps).await;
            expected_tps
        }
        Err(e) => {
            eprintln!("[lemonade] {}", e);
            0.0
        }
    }
}

/// Whether `model` can reliably emit well-formed tool calls.
///
/// **2-bit tool calling is per-model, not a blanket rule** (corrected 2026-08-07).
/// The old blanket refusal came from two models and did not generalise:
///
///   gemma-4-26B-A4B      Q2_K    0/6  — emits `tool_use` with params missing
///   Qwen3.6-35B-A3B      Q2_K    8/8  — measured good, 214 tok/s prefill
///
/// So 2-bit is refused by default but a model measured good is allowed through.
/// Add to the allow-list only with a real `bench-model.js <id> 8` result — never
/// on the assumption that a bigger sibling's score carries over.
///
/// Note this buys capability, not speed: 2-bit is compute-bound here, so the
/// smaller file generates no faster (15.8 vs 17.8 tok/s on the same 35B).
/// Two specific models are also denied by name: both score 0/6 despite being
/// fast — the Q2_K gemma emits `tool_use` with required params missing, and the
/// merged GPT-OSS answers in prose and never calls a tool at all.
///
/// Routing an agentic request to a model where this is false makes
/// `tool_invoker.rs` / `streaming_tool_executor.rs` loop on malformed calls
/// forever, so callers should refuse with a clear error naming the model.
pub fn supports_tool_calling(model: &str) -> bool {
    let m = model.to_lowercase();

    // Measured good at 2-bit despite the general rule: 8/8 tool calls.
    if m.contains("qwen3.6-35b-a3b") && m.contains("q2_k") {
        return true;
    }

    // 2-bit quants, in any spelling.
    if m.contains("q2_k") || m.contains("iq2") || m.contains("iq1") {
        return false;
    }

    // Measured 0/6 despite not being 2-bit.
    if m.contains("gemma-4-26b-a4b-it-abliterated-gguf-q2_k") {
        return false;
    }
    if m.contains("gpt-oss-cybersecurity-20b-merged-heretic") {
        return false;
    }

    true
}

/// Build a simple AI request with system + user messages.
fn simple_ai_request(
    provider: &str,
    model: &str,
    system: &str,
    user: &str,
    temperature: f32,
    mode: &str,
) -> AiRequest {
    AiRequest {
        provider: provider.to_string(),
        model: model.to_string(),
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: Some(MessageContent::Text(system.to_string())),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
            ChatMessage {
                role: "user".to_string(),
                content: Some(MessageContent::Text(user.to_string())),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
        ],
        temperature: Some(temperature),
        autonomous: false,
        cyber_mode: None,
        root_access: Some(false),
        mode: Some(mode.to_string()),
        ollama_url: None,
        tools: None,
        reasoning_budget: None,
        reasoning_effort: None,
        reasoning_enabled: None,
        feature: None,
    }
}

#[tauri::command]
pub async fn grep_files(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    pattern: String,
    path: Option<String>,
    include: Option<String>,
) -> Result<Vec<SearchResult>, String> {
    let root = if let Some(p) = path {
        PathBuf::from(p)
    } else {
        state
            .editor.active_root
            .lock()
            .await
            .clone()
            .unwrap_or_else(|| PathBuf::from("."))
    };

    ripgrep_search::ripgrep_search(RipgrepQuery {
        pattern: &pattern,
        root: &root,
        include: include.as_deref(),
        max_results: 100,
        case_insensitive: true,
        fixed_string: false,
        file: None,
    })
}

#[tauri::command]
pub async fn set_ai_status(
    _state: State<'_, std::sync::Arc<crate::EditorState>>,
    status: String,
) -> Result<(), String> {
    println!("[AI Status] Updated to: {}", status);
    Ok(())
}

#[tauri::command]
pub async fn ai_tool_result(
    _state: State<'_, std::sync::Arc<crate::EditorState>>,
    call_id: String,
    result: String,
) -> Result<(), String> {
    eprintln!(
        "[Tool Result] call_id={}, result_len={}",
        call_id,
        result.len()
    );
    Ok(())
}

#[tauri::command]
pub async fn ai_chat(
    app: AppHandle,
    state: State<'_, std::sync::Arc<crate::EditorState>>, 
    mut request: AiRequest
) -> Result<String, String> {
    let log_entry = format!("[ai_chat] REQUEST: {:?}\n", request);
    let _ = std::fs::OpenOptions::new()
        .create(true).append(true)
        .open("ai_chat.log")
        .and_then(|mut f| { use std::io::Write; f.write_all(log_entry.as_bytes()) });

    // Signal Kairos: user is actively using AI â€” reset idle timer
    state.services.kairos.report_activity().await;

    // Ensure the local backend URL is on the request (agent loop bearer auth uses it).
    if request.ollama_url.as_ref().map(|u| u.trim().is_empty()).unwrap_or(true) {
        let url = state.ai.engine.lemonade_base().await;
        if !url.trim().is_empty() {
            request.ollama_url = Some(url);
        }
    }

    // Lemonade's ctx_size/llamacpp.args are global and load-time only, so make
    // sure the resident model is loaded under ITS tuning and not whatever the
    // last process to touch the server left behind.
    if request.provider.eq_ignore_ascii_case("lemonade") {
        let base = state.ai.engine.lemonade_base().await;
        apply_lemonade_tuning(&base, &request.model).await;
    }

    // Update MemoryLayer state â€” agent is now active
    let _ = state.memory.layer.update_state("Active", &format!("Processing: {}",
        request.messages.last()
            .and_then(|m| m.content.as_ref().map(|c| c.to_text()))
            .unwrap_or_default()
            .chars().take(80).collect::<String>()
    ));

    // Inject the Hades persistent memory context as a system message.
    // Cap memory for small local models to prevent context overflow.
    let is_small = matches!(request.provider.to_lowercase().as_str(), "lemonade" | "huggingface")
        && crate::ai_engine::Sentient::is_small_model_name(&request.model);
    let mem_cap = if is_small { 2000 } else { 0 };
    if let Ok(hades_ctx) = state.memory.layer.get_aggregate_context_sized(mem_cap) {
        if !hades_ctx.trim().is_empty() {
            request.messages.insert(0, ChatMessage {
                role: "system".to_string(),
                content: Some(MessageContent::Text(hades_ctx)),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            });
        }
    }

    // Set up chunk callback for real-time UI streaming
    // Clone app BEFORE moving it into the Arc so we can emit the final ai-content event
    // directly after the loop completes, bypassing the silent_emits suppression used
    // by background AIRI autonomous tasks.
    let app_for_final = app.clone();
    let app_handle = std::sync::Arc::new(app);
    let accumulated = std::sync::Arc::new(std::sync::Mutex::new(String::new()));
    let accumulated_clone = accumulated.clone();

    let on_chunk = Some(std::sync::Arc::new(move |chunk: &str| {
        if !chunk.is_empty() {
            let _ = accumulated_clone.lock().map(|mut acc| acc.push_str(chunk));
        }
        let _ = app_handle.emit("ai-content-delta", serde_json::json!({ "delta": chunk }));
    }) as std::sync::Arc<dyn Fn(&str) + Send + Sync>);

    // Clear any stale streamed tokens from a prior turn before this one starts.
    if let Ok(mut b) = state.ai.engine.chat_stream_buf.lock() { b.clear(); }

    let result = state
        .ai.engine
        .clone()
        .autonomous_loop(request, on_chunk)
        .await
        .map_err(|e| {
            let err_log = format!("[ai_chat] ERROR: {}\n", e);
            eprintln!("{}", err_log.trim());
            let _ = std::fs::OpenOptions::new()
                .create(true).append(true)
                .open("ai_chat.log")
                .and_then(|mut f| { use std::io::Write; f.write_all(err_log.as_bytes()) });
            e.to_string()
        })?;

    // If result is empty but we streamed content, use accumulated chunks
    let final_response = if result.trim().is_empty() {
        accumulated
            .lock()
            .ok()
            .map(|acc| acc.trim().to_string())
            .unwrap_or_default()
    } else {
        result.clone()
    };

    // ALWAYS emit response to frontend (CRITICAL: use accumulated if result empty)
    let trimmed = final_response.trim();
    let emit_result = app_for_final.emit("ai-content", serde_json::json!({ "content": trimmed }));

    if let Err(e) = &emit_result {
        eprintln!("[ai_chat] EMIT FAILED: {}", e);
    } else {
        eprintln!("[ai_chat] Response emitted to frontend");
    }

    let done_log = format!(
        "[ai_chat] DONE: result_len={}, final_len={}, emit_ok={}\n",
        result.len(),
        trimmed.len(),
        emit_result.is_ok()
    );
    eprintln!("{}", done_log.trim());
    let _ = std::fs::OpenOptions::new()
        .create(true).append(true)
        .open("ai_chat.log")
        .and_then(|mut f| { use std::io::Write; f.write_all(done_log.as_bytes()) });
        
    // Update MemoryLayer: agent completed the task
    let _ = state.memory.layer.update_state("Idle", "Task completed");

    // Trim conversation state after each agent turn to keep RSS bounded.
    let engine_clone = state.ai.engine.clone();
    tauri::async_runtime::spawn(async move {
        let _ = engine_clone.optimize_memory().await;
    });

    // Satisfy AiResponse usage warning
    let _response = AiResponse { content: final_response.clone() };

    Ok(final_response)
}

/// Trivial-chat fast path. Skips the autonomous loop, the phase
/// machinery, tool catalog construction, system-prompt assembly, and
/// every retry. One HTTP round-trip to the provider and we're done.
/// Used by the frontend for short, action-less prompts like "hello"
/// where running the full agent loop is gross overkill (and was making
/// "hi" take 5+ seconds while the model dutifully ran git_status and
/// grep). Cursor's Agent mode behaves the same way â€” it doesn't open
/// the codebase index for a greeting.
#[tauri::command]
pub async fn ai_chat_fast(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    request: AiRequest,
) -> Result<String, String> {
    state.services.kairos.report_activity().await;

    let engine = state.ai.engine.clone();
    if let Ok(mut b) = engine.chat_stream_buf.lock() {
        b.clear();
    }
    // We deliberately do NOT inject the heavy Hades context here; the
    // whole point is sub-second latency. The conversation history the
    // frontend already passed is enough for trivial chat.
    let result = engine
        .single_shot_completion(request)
        .await
        .map_err(|e| e.to_string())?;

    // Frontend polls `chat_stream_drain` during fast chat — merge any tail
    // still in the buffer with the HTTP return value.
    let drained = {
        let mut b = engine
            .chat_stream_buf
            .lock()
            .map_err(|e| format!("chat_stream_buf lock poisoned: {e}"))?;
        std::mem::take(&mut *b)
    };
    let final_response = if result.trim().is_empty() {
        drained.trim().to_string()
    } else {
        result
    };

    // Keep the existing UI plumbing happy: every other code path
    // delivers the response via the `ai-content` event, so emit it
    // here too. The frontend `ai-content` listener calls
    // `updateLastAgentMessage` and flips `isAgentThinking` off.
    engine.emit_event(
        "ai-content",
        serde_json::json!({ "content": final_response.clone() }),
    );

    Ok(final_response)
}

/// Background-agent entry point. Same engine, same tool surface as
/// `ai_chat`, but every `emit_event` is suppressed for the duration of
/// the run so the foreground chat UI keeps streaming whatever it was
/// already showing. The frontend `runBackgroundAgent` slice calls this
/// to dispatch parallel work the user kicked off via `/bg <prompt>` or
/// the Background Agents tray.
#[tauri::command]
pub async fn ai_chat_oneshot(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    mut request: AiRequest,
) -> Result<String, String> {
    state.services.kairos.report_activity().await;

    // Same context injection as `ai_chat` â€” background work still needs
    // the Hades memory header so the model has consistent grounding.
    let is_small_bg = matches!(request.provider.to_lowercase().as_str(), "lemonade" | "huggingface")
        && crate::ai_engine::Sentient::is_small_model_name(&request.model);
    let mem_cap_bg = if is_small_bg { 2000 } else { 0 };
    if let Ok(hades_ctx) = state.memory.layer.get_aggregate_context_sized(mem_cap_bg) {
        if !hades_ctx.trim().is_empty() {
            request.messages.insert(0, ChatMessage {
                role: "system".to_string(),
                content: Some(MessageContent::Text(hades_ctx)),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            });
        }
    }

    let engine = state.ai.engine.clone();
    let _silent = engine.enter_silent();
    if request.ollama_url.as_ref().map(|u| u.trim().is_empty()).unwrap_or(true) {
        let url = state.ai.engine.lemonade_base().await;
        if !url.trim().is_empty() {
            request.ollama_url = Some(url);
        }
    }
    let result = engine
        .autonomous_loop(request, None)
        .await
        .map_err(|e| e.to_string())?;

    if let Some(root) = state.editor.active_root.lock().await.as_ref() {
        let root_str = root.to_string_lossy().to_string();
        let _hooks = crate::stop_hooks::run_stop_hooks(&root_str, &result);
    }

    Ok(result)
}

#[tauri::command]
pub async fn ai_inline_complete(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    prefix: String,
    suffix: String,
    language: String,
    file_path: String,
    model: Option<String>,
    provider: Option<String>,
) -> Result<String, String> {
    // Fast path: a dedicated completion-role model, served raw.
    //
    // Without this, completion falls through to `state.ai.current_model` — the
    // CHAT model. On this hardware that is the 35B MoE at 13.3 tok/s, which
    // cannot produce ghost text before the keystroke it predicted is gone. It
    // also goes through the chat endpoint, and a chat-shaped prompt makes a
    // small model converse about the code instead of continuing it (measured:
    // it echoed the current line back).
    //
    // `super::inline_completion` uses `/v1/completions` on a small model held
    // resident alongside the agent model. Measured 217-328ms warm. Only taken
    // when the caller did NOT pin a specific model — an explicit per-feature
    // choice in Settings still wins.
    if model.as_ref().map_or(true, |m| m.is_empty()) {
        let fast = super::inline_completion::ai_inline_completion(
            super::inline_completion::InlineCompletionArgs {
                prefix: prefix.clone(),
                suffix: suffix.clone(),
                language: language.clone(),
                single_line: false,
            },
        )
        .await;
        if let Ok(c) = fast {
            if !c.text.trim().is_empty() {
                return Ok(c.text);
            }
        }
        // Empty means the completion-role model is not loaded (or had nothing to
        // say). Fall through to the configured model rather than dropping Tab.
    }

    // Use active provider/model from state for completions
    let current_model = state.ai.current_model.lock().await.clone();

    // Honor an explicit Autocomplete-feature model when the frontend supplies one
    // (Settings → per-feature model selection). This is usually a small fast coder
    // model — previously these args were dropped and the heavy chat model was used.
    let (comp_provider, comp_model, comp_local_url) = if let (Some(p), Some(m)) =
        (provider.as_ref().filter(|s| !s.is_empty()), model.as_ref().filter(|s| !s.is_empty()))
    {
        (p.to_lowercase(), m.to_string(), None)
    } else {
        detect_provider(&current_model, provider.as_deref())
    };

    // Coder models trained with fill-in-the-middle do better on the raw FIM
    // sentinels than on a prose instruction. Lemonade serves these through the
    // chat endpoint, so this is prompt shape only — no separate native call.
    let uses_fim_tokens = comp_provider == "lemonade" && {
        let m = comp_model.to_lowercase();
        m.contains("coder") || m.contains("codellama") || m.contains("deepseek")
    };

    let fim_prompt = if uses_fim_tokens {
        format!("<fim_prefix>{}<fim_suffix>{}<fim_middle>", prefix, suffix)
    } else {
        format!(
            "Complete the following {} code. Return ONLY the completion text, no explanation, no markdown fencing, no extra whitespace.\n\n<prefix>\n{}\n</prefix>\n<suffix>\n{}\n</suffix>",
            language, prefix, suffix
        )
    };

    let messages = vec![
        ChatMessage {
            role: "system".to_string(),
            content: Some(MessageContent::Text(
                format!("You are an inline code completion engine for file '{}' (language: {}). Return ONLY the exact code that should be inserted at the cursor position. No explanation, no markdown, no comments. Just the raw code completion.", file_path, language)
            )),
            tool_calls: None,
            tool_call_id: None,
            metadata: None,
        },
        ChatMessage {
            role: "user".to_string(),
            content: Some(MessageContent::Text(fim_prompt)),
            tool_calls: None,
            tool_call_id: None,
            metadata: None,
        },
    ];

    let request = AiRequest {
        provider: comp_provider,
        model: comp_model,
        messages,
        temperature: Some(0.1),
        autonomous: false,
        cyber_mode: None,
        root_access: Some(false),
        mode: Some("Completion".to_string()),
        ollama_url: comp_local_url,
        tools: None,
        reasoning_budget: None,
        reasoning_effort: None,
        reasoning_enabled: None,
        feature: None,
    };

    // Single-shot, no agentic loop: inline completion must be FAST. The old path
    // ran the full autonomous_loop (tool parsing, memory, brain injection) for a
    // one-line FIM call — huge latency. single_shot_completion hits the provider
    // directly with stream:false.
    let result = state.ai.engine
        .single_shot_completion(request)
        .await
        .map_err(|e| e.to_string())?;

    // Strip any markdown fences
    let cleaned = result
        .trim()
        .trim_start_matches("```")
        .trim_start_matches(&language)
        .trim_start_matches('\n')
        .trim_end_matches("```")
        .trim()
        .to_string();

    Ok(cleaned)
}

/// Cursor-style "next edit" prediction. Given the current buffer + cursor line and the
/// span the user just changed, predict the SINGLE most likely next edit elsewhere in the
/// file (line range + replacement). Returns null-ish (empty `new_text`) when nothing useful.
#[tauri::command]
pub async fn predict_next_edit(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    content: String,
    cursor_line: usize,
    language: String,
    file_path: String,
    recent_change: Option<String>,
    model_override: Option<String>,
    provider: Option<String>,
) -> Result<serde_json::Value, String> {
    use serde_json::json;

    // Don't bother on tiny / huge buffers — keeps latency sane and signal high.
    let total_lines = content.lines().count();
    if total_lines < 4 || content.len() > 60_000 {
        return Ok(json!({ "has_edit": false }));
    }

    let current_model = {
        // Next-edit prediction is interactive and one-shot — the Edit role suits
        // it. An explicit `model_override` from Settings still wins.
        let chat = state.ai.current_model.lock().await.clone();
        model_for_role_or(ModelRole::Edit, &chat).await
    };
    let model_name = model_override
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(current_model);
    let (provider, model, ollama_url) = detect_provider(&model_name, provider.as_deref());

    // Number the lines so the model can reference exact line numbers (1-based).
    let numbered: String = content
        .lines()
        .enumerate()
        .map(|(i, l)| format!("{:>4}| {}", i + 1, l))
        .collect::<Vec<_>>()
        .join("\n");

    let change_note = recent_change
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .map(|s| format!("\nThe user JUST changed this near line {}:\n{}\n", cursor_line, s))
        .unwrap_or_default();

    let sys = format!(
        "You are a next-edit prediction engine for file '{}' ({}). The user is editing. \
Based on the change they just made, predict the SINGLE most likely NEXT edit they need elsewhere \
in the file (a propagation: update a matching signature, a stale reference, a paired return, an \
import, a sibling case, etc.). Respond with STRICT JSON only, no prose, no markdown:\n\
{{\"has_edit\":bool,\"start_line\":int,\"end_line\":int,\"new_text\":\"...\",\"reason\":\"<=8 words\"}}\n\
Rules: start_line/end_line are 1-based inclusive and reference the NUMBERED source. new_text fully \
replaces those lines (no line-number prefixes, preserve indentation). If no confident propagation \
exists, return {{\"has_edit\":false}}. Never invent edits at the cursor itself.",
        file_path, language
    );

    let messages = vec![
        ChatMessage {
            role: "system".to_string(),
            content: Some(MessageContent::Text(sys)),
            tool_calls: None, tool_call_id: None, metadata: None,
        },
        ChatMessage {
            role: "user".to_string(),
            content: Some(MessageContent::Text(format!(
                "Cursor at line {}.{}\nSOURCE:\n{}",
                cursor_line, change_note, numbered
            ))),
            tool_calls: None, tool_call_id: None, metadata: None,
        },
    ];

    let request = AiRequest {
        provider, model, messages,
        temperature: Some(0.1),
        autonomous: false,
        cyber_mode: None,
        root_access: Some(false),
        mode: Some("Completion".to_string()),
        ollama_url,
        tools: None,
        reasoning_budget: None,
        reasoning_effort: None,
        reasoning_enabled: None,
        feature: Some("Autocomplete".to_string()),
    };

    let raw = state.ai.engine.clone()
        .autonomous_loop(request, None)
        .await
        .map_err(|e| e.to_string())?;

    // Extract the JSON object even if the model wrapped it in fences/prose.
    let body = {
        let t = raw.trim().trim_start_matches("```json").trim_start_matches("```").trim_end_matches("```").trim();
        match (t.find('{'), t.rfind('}')) {
            (Some(a), Some(b)) if b > a => t[a..=b].to_string(),
            _ => return Ok(json!({ "has_edit": false })),
        }
    };

    let parsed: serde_json::Value = match serde_json::from_str(&body) {
        Ok(v) => v,
        Err(_) => return Ok(json!({ "has_edit": false })),
    };

    let has = parsed.get("has_edit").and_then(|v| v.as_bool()).unwrap_or(false);
    let start = parsed.get("start_line").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
    let end = parsed.get("end_line").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
    let new_text = parsed.get("new_text").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let reason = parsed.get("reason").and_then(|v| v.as_str()).unwrap_or("").to_string();

    // Validate range, reject no-ops and edits that land on the cursor line.
    if !has || start == 0 || end < start || end > total_lines || new_text.is_empty() {
        return Ok(json!({ "has_edit": false }));
    }
    let original: String = content.lines().skip(start - 1).take(end - start + 1).collect::<Vec<_>>().join("\n");
    if original.trim() == new_text.trim() {
        return Ok(json!({ "has_edit": false }));
    }

    Ok(json!({
        "has_edit": true,
        "start_line": start,
        "end_line": end,
        "new_text": new_text,
        "reason": reason,
        "old_text": original
    }))
}

#[tauri::command]
pub async fn ai_explain_code(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    code: String,
    file_path: String,
    detail_level: String,
) -> Result<String, String> {
    let model = {
        let chat = state.ai.current_model.lock().await.clone();
        model_for_role_or(ModelRole::Edit, &chat).await
    };
    let prompt = format!(
        "Explain what this {} code does in {} detail level:\n\n```\n{}\n```\n\nProvide a clear explanation covering:\n1. What the code does (plain English)\n2. Key logic flow\n3. Any important patterns or concepts used",
        file_path.split('.').last().unwrap_or("code"),
        detail_level,
        code
    );
    let request = simple_ai_request(
        "google", &model,
        "You are a code explanation assistant. Explain code clearly in plain English.",
        &prompt, 0.3, "Explain",
    );
    state.ai.engine.clone().autonomous_loop(request, None).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn ai_document_code(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    code: String,
    _file_path: String,
    format: String,
    language: String,
) -> Result<String, String> {
    let model = {
        let chat = state.ai.current_model.lock().await.clone();
        model_for_role_or(ModelRole::Edit, &chat).await
    };
    let prompt = format!(
        "Generate {} documentation for this {} code:\n\n```{}\n```\n\nInclude:\n- Function/class descriptions\n- Parameter explanations\n- Return value descriptions\n- Usage examples if helpful",
        format, language, code
    );
    let request = simple_ai_request(
        "google", &model,
        "You are a documentation generator. Generate clean, professional code documentation.",
        &prompt, 0.3, "Document",
    );
    state.ai.engine.clone().autonomous_loop(request, None).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn ai_generate_code(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    prompt: String,
    language: String,
    framework: Option<String>,
    _file_path: Option<String>,
) -> Result<String, String> {
    let full_prompt = if let Some(fw) = framework {
        format!("Generate {} code using {} framework for: {}\n\nInclude proper imports, error handling, and best practices.", language, fw, prompt)
    } else {
        format!("Generate {} code for: {}\n\nInclude proper imports, error handling, and best practices.", language, prompt)
    };
    
    let request = AiRequest {
        provider: "google".to_string(),
        model: {
            // Editor assists prefer a resident Edit-role model (gemma-12B at
            // 30.3 tok/s) over the chat model (35B at 13.3). Falls back to the
            // chat model when nothing better is loaded, and never triggers a
            // load — see `model_for_role_or`.
            let chat = state.ai.current_model.lock().await.clone();
            model_for_role_or(ModelRole::Edit, &chat).await
        },
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: Some(MessageContent::Text(
                    "You are a code generation assistant. Generate clean, functional, production-ready code.".to_string()
                )),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
            ChatMessage {
                role: "user".to_string(),
                content: Some(MessageContent::Text(full_prompt)),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
        ],
        temperature: Some(0.4),
        autonomous: false,
        cyber_mode: None,
        root_access: Some(false),
        mode: Some("Generate".to_string()),
        ollama_url: None,
        tools: None,
        reasoning_budget: None,
        reasoning_effort: None,
        reasoning_enabled: None,
        feature: None,
    };

    state.ai.engine.clone().autonomous_loop(request, None).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn ai_refactor_code(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    _code: String,
    file_path: String,
    start_line: Option<usize>,
    end_line: Option<usize>,
    refactor_type: String,
    target_name: Option<String>,
) -> Result<String, String> {
    let range = match (start_line, end_line) {
        (Some(s), Some(e)) => format!(" (lines {} to {})", s, e),
        (Some(s), None) => format!(" (starting from line {})", s),
        _ => String::new(),
    };
    
    let prompt = format!(
        "Refactor this {} code{} using {} refactoring approach.{}\n\nProvide improved code with better readability, performance, and best practices.",
        file_path.split('.').last().unwrap_or("code"),
        range,
        refactor_type,
        target_name.map(|n| format!(" Target name: {}", n)).unwrap_or_default()
    );
    
    let request = AiRequest {
        provider: "google".to_string(),
        model: {
            // Editor assists prefer a resident Edit-role model (gemma-12B at
            // 30.3 tok/s) over the chat model (35B at 13.3). Falls back to the
            // chat model when nothing better is loaded, and never triggers a
            // load — see `model_for_role_or`.
            let chat = state.ai.current_model.lock().await.clone();
            model_for_role_or(ModelRole::Edit, &chat).await
        },
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: Some(MessageContent::Text(
                    "You are a code refactoring assistant. Improve code quality while preserving functionality.".to_string()
                )),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
            ChatMessage {
                role: "user".to_string(),
                content: Some(MessageContent::Text(prompt)),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
        ],
        temperature: Some(0.3),
        autonomous: false,
        cyber_mode: None,
        root_access: Some(false),
        mode: Some("Refactor".to_string()),
        ollama_url: None,
        tools: None,
        reasoning_budget: None,
        reasoning_effort: None,
        reasoning_enabled: None,
        feature: None,
    };

    state.ai.engine.clone().autonomous_loop(request, None).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn ai_debug_code(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    code: String,
    file_path: String,
    error_message: Option<String>,
    start_line: Option<usize>,
    end_line: Option<usize>,
) -> Result<Value, String> {
    let code_section = match (start_line, end_line) {
        (Some(s), Some(e)) => format!(" (lines {} to {})", s, e),
        _ => String::new(),
    };
    
    let prompt = if let Some(err) = error_message {
        format!(
            "Debug this {} code{} that has error: {}\n\nError: {}\n\nProvide:\n1. Diagnosis of the problem\n2. List of specific issues found\n3. Fixed code\n4. Suggestions for prevention",
            file_path.split('.').last().unwrap_or("code"),
            code_section,
            err,
            code
        )
    } else {
        format!(
            "Debug this {} code{} for bugs, errors, and issues.\n\nProvide:\n1. Diagnosis of problems found\n2. List of specific issues (logic errors, race conditions, security issues, etc.)\n3. Fixed code\n4. Suggestions for improvement\n\nCode:\n```\n{}\n```",
            file_path.split('.').last().unwrap_or("code"),
            code_section,
            code
        )
    };
    
    let request = AiRequest {
        provider: "google".to_string(),
        model: {
            // Editor assists prefer a resident Edit-role model (gemma-12B at
            // 30.3 tok/s) over the chat model (35B at 13.3). Falls back to the
            // chat model when nothing better is loaded, and never triggers a
            // load — see `model_for_role_or`.
            let chat = state.ai.current_model.lock().await.clone();
            model_for_role_or(ModelRole::Edit, &chat).await
        },
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: Some(MessageContent::Text(
                    "You are a code debugging assistant. Find and fix bugs, errors, and issues in code.".to_string()
                )),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
            ChatMessage {
                role: "user".to_string(),
                content: Some(MessageContent::Text(prompt)),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
        ],
        temperature: Some(0.2),
        autonomous: false,
        cyber_mode: None,
        root_access: Some(false),
        mode: Some("Debug".to_string()),
        ollama_url: None,
        tools: None,
        reasoning_budget: None,
        reasoning_effort: None,
        reasoning_enabled: None,
        feature: None,
    };

    let response = state.ai.engine.clone().autonomous_loop(request, None).await.map_err(|e| e.to_string())?;
    
    Ok(json!({
        "diagnosis": "Analysis complete",
        "issues": ["See fixed code below"],
        "fixed_code": response,
        "suggestions": ["Review the fixed code and apply any needed adjustments"]
    }))
}

#[tauri::command]
pub async fn ai_multi_cursor_edit(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    _code: String,
    file_path: String,
    pattern: String,
    replacement: String,
    match_scope: String,
    apply: bool,
) -> Result<Value, String> {
    let prompt = format!(
        "Find all occurrences of '{}' in this {} code and {} them.\n\nPattern: {}\nReplacement: {}\nMatch scope: {}\n\nReturn the modified code with all changes applied. If apply=false, show preview of changes.",
        pattern,
        file_path.split('.').last().unwrap_or("code"),
        if apply { "replace" } else { "preview replacement for" },
        pattern,
        replacement,
        match_scope
    );
    
    let request = AiRequest {
        provider: "google".to_string(),
        model: {
            // Editor assists prefer a resident Edit-role model (gemma-12B at
            // 30.3 tok/s) over the chat model (35B at 13.3). Falls back to the
            // chat model when nothing better is loaded, and never triggers a
            // load — see `model_for_role_or`.
            let chat = state.ai.current_model.lock().await.clone();
            model_for_role_or(ModelRole::Edit, &chat).await
        },
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: Some(MessageContent::Text(
                    "You are a multi-cursor editing assistant. Find patterns and edit them consistently across code.".to_string()
                )),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
            ChatMessage {
                role: "user".to_string(),
                content: Some(MessageContent::Text(prompt)),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
        ],
        temperature: Some(0.3),
        autonomous: false,
        cyber_mode: None,
        root_access: Some(false),
        mode: Some("MultiEdit".to_string()),
        ollama_url: None,
        tools: None,
        reasoning_budget: None,
        reasoning_effort: None,
        reasoning_enabled: None,
        feature: None,
    };

    let modified = state.ai.engine.clone().autonomous_loop(request, None).await.map_err(|e| e.to_string())?;
    
    Ok(json!({
        "matches": [format!("Found occurrences of: {}", pattern)],
        "modified_code": modified,
        "preview_only": !apply
    }))
}

#[tauri::command]
pub async fn ai_pr_review(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    _pr_url: Option<String>,
    diff_content: Option<String>,
    focus_areas: Option<Vec<String>>,
) -> Result<Value, String> {
    let diff = diff_content.ok_or("diff_content required for PR review")?;
    let focus = focus_areas.unwrap_or_else(|| {
        vec!["security".to_string(), "performance".to_string(), "style".to_string()]
    });

    // Cap diff size so 2b–4b local models stay coherent; report the truncation.
    const MAX_DIFF_CHARS: usize = 24_000;
    let truncated = diff.chars().count() > MAX_DIFF_CHARS;
    let diff_slice: String = diff.chars().take(MAX_DIFF_CHARS).collect();

    let prompt = format!(
        "Review the following code diff. Focus areas: {}.\n\
         For each issue report: file, severity (critical/major/minor), what is wrong, and a concrete fix.\n\
         Be specific — quote the offending lines. If the diff is clean, say so.\n\
         End with exactly one verdict line: VERDICT: APPROVE or VERDICT: REQUEST_CHANGES.\n\n\
         ```diff\n{}\n```{}",
        focus.join(", "),
        diff_slice,
        if truncated { "\n\n(NOTE: diff truncated for review)" } else { "" }
    );

    let request = AiRequest {
        provider: "google".to_string(),
        model: {
            // Editor assists prefer a resident Edit-role model (gemma-12B at
            // 30.3 tok/s) over the chat model (35B at 13.3). Falls back to the
            // chat model when nothing better is loaded, and never triggers a
            // load — see `model_for_role_or`.
            let chat = state.ai.current_model.lock().await.clone();
            model_for_role_or(ModelRole::Edit, &chat).await
        },
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: Some(MessageContent::Text(
                    "You are a rigorous senior code reviewer. Review diffs for correctness, security, performance, and style. Never invent issues; only report what the diff shows.".to_string()
                )),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
            ChatMessage {
                role: "user".to_string(),
                content: Some(MessageContent::Text(prompt)),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
        ],
        temperature: Some(0.2),
        autonomous: false,
        cyber_mode: None,
        root_access: Some(false),
        mode: Some("Review".to_string()),
        ollama_url: None,
        tools: None,
        reasoning_budget: None,
        reasoning_effort: None,
        reasoning_enabled: None,
        feature: None,
    };

    let review = state
        .ai.engine
        .clone()
        .autonomous_loop(request, None)
        .await
        .map_err(|e| e.to_string())?;

    let verdict = if review.to_uppercase().contains("VERDICT: APPROVE") {
        "approve"
    } else if review.to_uppercase().contains("REQUEST_CHANGES") {
        "request_changes"
    } else {
        "unspecified"
    };

    Ok(json!({
        "review": review,
        "verdict": verdict,
        "focus": focus,
        "diff_chars": diff.chars().count(),
        "truncated": truncated,
    }))
}

#[tauri::command]
pub async fn ai_get_context(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    query: String,
    max_files: Option<usize>,
    _include_types: Option<Vec<String>>,
) -> Result<Value, String> {
    let max = max_files.unwrap_or(5);

    // Semantic-first: use the vector index when embeddings are available
    // (requires Ollama + an indexed workspace), then fall back to grep so the
    // tool always returns something — and reports which method it used.
    if let Ok(hits) = state.memory.vector_indexer.search_codebase(&query, max).await {
        if !hits.is_empty() {
            let files: Vec<Value> = hits
                .into_iter()
                .map(|h| {
                    json!({
                        "path": h.file_path,
                        "line": h.start_line,
                        "snippet": h.content.chars().take(200).collect::<String>(),
                        "score": h.relevance_score,
                    })
                })
                .collect();
            return Ok(json!({
                "query": query,
                "files": files,
                "count": files.len(),
                "method": "semantic"
            }));
        }
    }

    let results = grep_files(state.clone(), query.clone(), None, None).await.unwrap_or_default();

    let mut unique_files: Vec<Value> = Vec::new();
    let mut seen_paths: std::collections::HashSet<String> = std::collections::HashSet::new();

    for r in results.into_iter().take(max * 3) {
        let path = r.path.clone();
        if !seen_paths.contains(&path) && unique_files.len() < max {
            seen_paths.insert(path.clone());
            unique_files.push(json!({
                "path": path,
                "line": r.line,
                "snippet": r.content.chars().take(100).collect::<String>(),
            }));
        }
    }

    Ok(json!({
        "query": query,
        "files": unique_files,
        "count": unique_files.len(),
        "method": "grep_fallback"
    }))
}

#[tauri::command]
pub async fn airi_broadcast(
    app: AppHandle,
    event: String,
    payload: Option<Value>,
) -> Result<(), String> {
    let payload = payload.unwrap_or(json!({}));
    app.emit(&event, &payload)
        .map_err(|e| format!("Failed to broadcast: {}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn call_tool(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    name: String,
    arguments: Value,
) -> Result<Value, String> {
    state.ai.tools
        .call_tool(&name, arguments.clone())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn ai_execute_command(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    command: String,
    cwd: Option<String>,
    timeout: Option<u64>,
) -> Result<String, String> {
    println!("[DEBUG] ai_execute_command: {}", command);

    // Route through run_command so shell grep/rg is intercepted → bundled ripgrep.
    let mut args = json!({
        "command": command,
        "shell_hint": "bash",
    });
    if timeout.is_some() {
        args["timeout_ms"] = json!(timeout.unwrap_or(120_000));
    }
    if cwd.is_some() {
        args["cwd"] = json!(cwd);
    }

    let result = state
        .ai.tools
        .call_tool("run_command", args)
        .await
        .map_err(|e| e.to_string())?;

    if result.get("status").and_then(|v| v.as_str()) == Some("blocked") {
        return Err(
            result
                .get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("Command blocked")
                .to_string(),
        );
    }

    if let Some(results) = result.get("results").and_then(|v| v.as_str()) {
        return Ok(if results.is_empty() {
            "No matches".to_string()
        } else {
            results.to_string()
        });
    }

    let stdout = result.get("stdout").and_then(|v| v.as_str()).unwrap_or("");
    let stderr = result.get("stderr").and_then(|v| v.as_str()).unwrap_or("");
    let success = result.get("success").and_then(|v| v.as_bool()).unwrap_or(false);

    if success {
        if stdout.is_empty() && !stderr.is_empty() {
            Ok(format!("Command succeeded (stderr only):\n{}", stderr))
        } else if stdout.is_empty() {
            Ok("Command succeeded (no output)".to_string())
        } else {
            Ok(stdout.to_string())
        }
    } else {
        Err(format!(
            "Command failed:\nSTDOUT: {}\nSTDERR: {}",
            stdout, stderr
        ))
    }
}

#[tauri::command]
pub fn ai_modify_file(
    _state: State<'_, std::sync::Arc<crate::EditorState>>,
    path: String,
    instruction: String,
) -> Result<(), String> {
    println!(
        "AI requested modification for path: {}, instruction: {}",
        path, instruction
    );
    Ok(())
}

#[tauri::command]
pub async fn propose_file_change(
    app: AppHandle,
    _state: State<'_, std::sync::Arc<crate::EditorState>>,
    path: String,
    content: String,
    description: String,
) -> Result<Value, String> {
    let path_buf = PathBuf::from(&path);

    let old_content = if path_buf.exists() {
        fs::read_to_string(&path_buf).unwrap_or_default()
    } else {
        String::new()
    };

    let payload = json!({
        "path": path,
        "old_content": old_content,
        "new_content": content,
        "description": description
    });
    let _ = app.emit("propose-edit", &payload);

    Ok(json!({
        "path": path,
        "oldContent": old_content,
        "newContent": content,
        "description": description
    }))
}

/// Composer-style fast apply: merge a SEARCH/REPLACE block and open the diff review panel.
#[tauri::command]
pub async fn preview_search_replace(
    app: AppHandle,
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    path: String,
    search_text: String,
    replace_text: String,
    description: Option<String>,
) -> Result<(), String> {
    use crate::patch_engine::PatchBlock;

    let path_buf = PathBuf::from(&path);
    if !path_buf.exists() {
        return Err(format!("File not found: {path}"));
    }
    let old_content = fs::read_to_string(&path_buf).map_err(|e| e.to_string())?;
    let patch = PatchBlock {
        search: search_text,
        replace: replace_text,
    };
    let mut pe = state.services.patch_engine.lock().await;
    let new_content = pe
        .apply_patches(&path_buf, &old_content, &[patch])
        .await
        .map_err(|e| e.to_string())?;

    let desc = description.unwrap_or_else(|| "Search/replace preview".to_string());
    app.emit(
        "propose-edit",
        json!({
            "path": path,
            "old_content": old_content,
            "new_content": new_content,
            "description": desc,
        }),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
#[tauri::command]
pub async fn compress_session_data(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    key: String,
    data: String,
) -> Result<(), String> {
    state
        .memory.optimizer
        .compress_and_store(&key, &data)

        .await
        .map_err(|e| e.to_string())
}

/// Health probe for the Lemonade backend. Invoked by agentResilience.ts and
/// inferenceSlice.checkLemonadeStatus. Returns Ok(false) when unreachable.
#[tauri::command]
pub async fn check_lemonade_status(state: State<'_, std::sync::Arc<crate::EditorState>>) -> Result<bool, String> {
    state
        .ai.engine
        .check_lemonade_status()
        .await
        .map_err(|e| e.to_string())
}

/// Pull/download a model on the Lemonade server (`POST /api/v1/pull`).
/// Routed through Rust so cloud Lemonade gets the JWT bearer and local
/// avoids webview CORS — the settings panel must not fetch() this directly.
#[tauri::command]
pub async fn pull_lemonade_model(state: State<'_, std::sync::Arc<crate::EditorState>>, name: String) -> Result<(), String> {
    state
        .ai.engine
        .pull_lemonade_model(&name)
        .await
        .map_err(|e| e.to_string())
}

/// Propagate the configured Lemonade server URL (from Settings → Inference
/// Backend) into the Rust process so `get_endpoint("lemonade")` and
/// `list_models("lemonade")` resolve against the user's actual port instead
/// of the hardcoded `http://localhost:13305` default.
#[tauri::command]
pub async fn set_lemonade_url(state: State<'_, std::sync::Arc<crate::EditorState>>, url: String) -> Result<(), String> {
    state.ai.engine.set_lemonade_url(url.clone()).await;

    // Propagate to the APEX orchestrator so Lemonade-backed engines (BugTrace
    // CORE-Ultra tooling) hit the user's configured llama.cpp server.
    state.ai.apex.set_lemonade_url(&url).await;

    // NOTE: embeddings are deliberately NOT re-pointed here. `url` is the *chat*
    // backend — on the Kortex ROCmFPX path that's a proxy fronting a chat-only
    // model which 404s /v1/embeddings, and that stalled the vector index at
    // ~108 chunks. Embeddings stay on `default_embed_base_url()` (Lemonade
    // :13305 / the LEMONADE_URL env). Use `set_embed_url` for a custom embed host.

    // Vision/attachment inference still follows the configured backend.
    state.memory.attachments.set_inference_url(url).await;

    Ok(())
}

/// Force the context indexer to rescan the active workspace. Powers the
/// "Re-index" button under Settings â†’ Indexing & Docs and the `/reindex`
/// slash command. We call `reindex_if_needed` (not a hard rebuild) so
/// large repos don't get flattened by an accidental click; the indexer
/// itself decides if anything has changed on disk.
#[tauri::command]
pub async fn reindex_workspace(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
) -> Result<Value, String> {
    let root = {
        let guard = state.editor.active_root.lock().await;
        guard.clone().unwrap_or_else(|| std::path::PathBuf::from("."))
    };
    state
        .memory.context_indexer
        .reindex_if_needed(&root)
        .map_err(|e| e.to_string())?;
    Ok(serde_json::json!({
        "status": "ok",
        "root": root.to_string_lossy(),
    }))
}

/// List every workspace rule the rules engine currently sees. Used by the
/// Cursor-style "Rules, Skills, Subagents" settings panel so the user can
/// inspect which files are being injected into the system prompt without
/// having to grep the workspace. Returns each rule's name, full text, and
/// source path so the UI can deep-link to the file.
#[tauri::command]
pub async fn list_workspace_rules(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
) -> Result<Value, String> {
    let rules = state.ai.engine.rules_engine.get_workspace_rules();
    let items: Vec<Value> = rules
        .into_iter()
        .map(|r| serde_json::json!({
            "name": r.name,
            "content": r.content,
            "file_path": r.file_path.to_string_lossy(),
        }))
        .collect();
    Ok(serde_json::json!({
        "count": items.len(),
        "rules": items,
    }))
}

#[tauri::command]
pub async fn set_ai_model(state: State<'_, std::sync::Arc<crate::EditorState>>, model: String) -> Result<(), String> {
    let mut current = state.ai.current_model.lock().await;
    *current = model.clone();
    state.ai.engine.set_advisor_model(Some(model)).await;

    Ok(())
}

#[tauri::command]
pub async fn set_advisor_model(state: State<'_, std::sync::Arc<crate::EditorState>>, model: Option<String>) -> Result<(), String> {
    let mut advisor = state.ai.advisor_model.lock().await;
    *advisor = model;
    Ok(())
}

#[tauri::command]
pub async fn list_provider_models(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    provider: String,
) -> Result<Vec<String>, String> {
    state
        .ai.engine
        .list_models(&provider)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_agent_messages(state: State<'_, std::sync::Arc<crate::EditorState>>) -> Result<Value, String> {
    Ok(json!(state.ai.engine.memory_store.get_messages().await))
}

#[tauri::command]
pub async fn get_brain_telemetry(state: State<'_, std::sync::Arc<crate::EditorState>>) -> Result<Value, String> {
    Ok(state.ai.engine.memory_store.get_brain_telemetry().await)
}

#[tauri::command]
pub async fn store_message(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    role: String,
    content: String,
    timestamp: i64,
) -> Result<(), String> {
    state.ai.engine.memory_store.store_message_params(role, content, timestamp).await;
    Ok(())
}

#[tauri::command]
pub async fn sync_agent_messages(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    messages: Vec<ChatMessage>,
) -> Result<(), String> {
    state.ai.engine.memory_store.store_conversation(&messages).await;
    state.ai.engine.memory_store.flush_to_disk().await;
    Ok(())
}
#[tauri::command]
pub async fn list_chat_sessions(state: State<'_, std::sync::Arc<crate::EditorState>>) -> Result<Value, String> {
    Ok(json!(state.ai.engine.memory_store.list_sessions().await))
}

#[tauri::command]
pub async fn load_chat_session(state: State<'_, std::sync::Arc<crate::EditorState>>, path: String) -> Result<Value, String> {
    let messages = state
        .ai.engine
        .memory_store
        .restore_session_from_path(PathBuf::from(path))
        .await;
    Ok(json!(messages))
}

#[tauri::command]
pub async fn archive_chat_session(state: State<'_, std::sync::Arc<crate::EditorState>>) -> Result<(), String> {
    state.ai.engine.memory_store.archive_current_session().await;
    Ok(())
}

#[tauri::command]
pub async fn create_new_session(state: State<'_, std::sync::Arc<crate::EditorState>>) -> Result<(), String> {
    state.ai.engine.memory_store.create_new_session().await;
    Ok(())
}

/// Drain the live agent-activity buffer. The webview can't receive the Tauri
/// event stream, so the activity terminal polls this to mirror what the agent
/// is doing in real time. Each entry is a JSON line: `{"kind","payload"}`.
#[tauri::command]
pub async fn agent_activity_drain(state: State<'_, std::sync::Arc<crate::EditorState>>) -> Result<Vec<String>, String> {
    let mut log = state
        .ai.engine
        .activity_log
        .lock()
        .map_err(|e| format!("activity_log lock poisoned: {e}"))?;
    Ok(std::mem::take(&mut *log))
}

/// Drain buffered chat tokens (live model output). The `ai-content-delta` event
/// is dead in the webview, so the chat panel polls this to render streamed text.
#[tauri::command]
pub async fn chat_stream_drain(state: State<'_, std::sync::Arc<crate::EditorState>>) -> Result<String, String> {
    let mut b = state
        .ai.engine
        .chat_stream_buf
        .lock()
        .map_err(|e| format!("chat_stream_buf lock poisoned: {e}"))?;
    Ok(std::mem::take(&mut *b))
}

/// Drain queued edit proposals from the autonomous loop. Each is
/// `{path, oldContent, newContent, description, additions, deletions}`. The diff-
/// review panel polls this (the event stream is dead in the webview) and turns
/// each into a reviewable pending change.
#[tauri::command]
pub async fn agent_proposals_drain(state: State<'_, std::sync::Arc<crate::EditorState>>) -> Result<Vec<Value>, String> {
    let mut q = state
        .ai.engine
        .pending_proposals
        .lock()
        .map_err(|e| format!("pending_proposals lock poisoned: {e}"))?;
    Ok(std::mem::take(&mut *q))
}

/// Restore a file to the given content. Used by the diff-review panel to REJECT
/// an already-applied agent edit (revert to the pre-edit snapshot). Path may be
/// absolute or project-relative.
#[tauri::command]
pub async fn revert_file_content(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    path: String,
    content: String,
) -> Result<(), String> {
    let root = state.ai.engine.ai_tools.get_root_path();
    let full = if std::path::Path::new(&path).is_absolute() {
        PathBuf::from(&path)
    } else {
        root.join(&path)
    };
    fs::write(&full, content).map_err(|e| format!("revert failed for {path}: {e}"))?;
    Ok(())
}

/// Inspect a `.aim` Neural Weight-Map binary for the in-IDE viewer. Returns the
/// header (magic/version/written_at/entry_count/size) plus a capped list of
/// entries (key, weight, mtime, gist preview). Path may be absolute or
/// workspace-relative.
#[tauri::command]
pub async fn aim_inspect(state: State<'_, std::sync::Arc<crate::EditorState>>, path: String) -> Result<serde_json::Value, String> {
    let root = state.ai.engine.ai_tools.get_root_path();
    let full = if std::path::Path::new(&path).is_absolute() {
        PathBuf::from(&path)
    } else {
        root.join(&path)
    };
    if !full.exists() {
        return Err(format!("AIM file not found: {}", full.display()));
    }
    let bytes = std::fs::read(&full).map_err(|e| e.to_string())?;
    let size = bytes.len() as u64;

    // `.aim` has two on-disk flavors in this project:
    //   • JSON   — the Kortex brain/memory (`{"kortex":{...}}`)
    //   • binary — the aim_store Neural Weight-Map (magic `AIM\x01`)
    // Detect by the first non-whitespace byte and handle both.
    let first = bytes.iter().copied().find(|b| !b.is_ascii_whitespace()).unwrap_or(0);
    if first == b'{' || first == b'[' {
        let v: serde_json::Value = serde_json::from_slice(&bytes)
            .map_err(|e| format!("AIM file looks like JSON but failed to parse: {e}"))?;
        let mut pretty = serde_json::to_string_pretty(&v).unwrap_or_default();
        let truncated = pretty.len() > 300_000;
        if truncated {
            pretty.truncate(300_000);
            pretty.push_str("\n… [truncated for display]");
        }
        let entities = v.pointer("/kortex/entities").and_then(|e| e.as_object()).map(|o| o.len());
        let tree_count = v.pointer("/kortex/project_tree").and_then(|t| t.as_array()).map(|a| a.len());
        return Ok(serde_json::json!({
            "format": "json",
            "size_bytes": size,
            "pretty": pretty,
            "truncated": truncated,
            "entities": entities,
            "tree_count": tree_count,
        }));
    }

    // Binary AIM\x01 (aim_store).
    let insp = crate::aim_store::AimStore::inspect(&full, 2000).map_err(|e| e.to_string())?;
    let mut val = serde_json::to_value(insp).map_err(|e| e.to_string())?;
    if let Some(obj) = val.as_object_mut() {
        obj.insert("format".to_string(), serde_json::json!("aim-binary"));
    }
    Ok(val)
}

#[cfg(test)]
mod lemonade_tuning_tests {
    use super::*;

    /// Exact model IDs as served by `GET /api/v1/models` on this machine. The
    /// matchers are substring-based, so pin them against real IDs — a rename
    /// upstream silently degrades to the size fallback otherwise.
    const GEMMA_12B_Q4: &str = "Huihui-gemma-4-12B-agentic-fable5-abliterated-i1-Q4_K_M";
    const QWEN_35B: &str = "Qwen3.6-35B-A3B-Abliterated-Heretic-GGUF-Q4_K_M";
    const GEMMA_26B_Q2: &str = "gemma-4-26B-A4B-it-abliterated-GGUF-Q2_K";
    const GPT_OSS: &str = "GPT-OSS-Cybersecurity-20B-Merged-heretic-i1-GGUF-Q4_K_M";

    /// Lemonade's Ollama-compat `/api/tags` appends `:latest`; its native
    /// `/api/v1/models` and the Anthropic endpoint do not. A persisted tagged id
    /// must still resolve, or every request fails model validation.
    /// Lemonade labels EVERY downloaded llamacpp model `tool-calling`, including
    /// both models measured at 0/6 here. `lemonade-claude.sh` says as much
    /// ("the label is metadata, sometimes wrong"), so the measured denylist stays
    /// authoritative — the label may add restrictions, never lift them.
    #[test]
    fn measured_denylist_overrides_the_servers_tool_calling_label() {
        // Both are labelled tool-calling by Lemonade, and both score 0/6.
        assert!(!supports_tool_calling(GPT_OSS));
        assert!(!supports_tool_calling(GEMMA_26B_Q2));
    }

    #[test]
    fn latest_tag_is_stripped_to_the_canonical_id() {
        assert_eq!(
            canonical_model_id("Qwen3.6-35B-A3B-Abliterated-Heretic-GGUF-Q4_K_M:latest"),
            "Qwen3.6-35B-A3B-Abliterated-Heretic-GGUF-Q4_K_M"
        );
        // Already canonical, and a real quant tag, both pass through untouched.
        assert_eq!(canonical_model_id(QWEN_35B), QWEN_35B);
        assert_eq!(canonical_model_id("some-model:Q4_K_M"), "some-model:Q4_K_M");
    }

    #[test]
    fn measured_models_get_their_measured_tuning() {
        assert_eq!(lemonade_tuning(GEMMA_12B_Q4), (131072, LEMONADE_ARGS_SMALL, 30.3));
        assert_eq!(lemonade_tuning(QWEN_35B), (98304, LEMONADE_ARGS_LARGE, 13.3));
        assert_eq!(
            lemonade_tuning("Huihui-gemma-4-12B-agentic-fable5-abliterated-GGUF-Q8_0"),
            (65536, LEMONADE_ARGS_MID, 18.8)
        );
    }

    /// `-rea off` is required on Qwen3.6: without it the model spends the whole
    /// token budget inside `<think>`, which Lemonade's Anthropic adapter drops,
    /// and the caller sees an empty response rather than an error.
    #[test]
    fn qwen_tuning_disables_reasoning_and_offloads_experts() {
        let (_, args, _) = lemonade_tuning(QWEN_35B);
        assert!(args.contains("-rea off"), "Qwen3.6 must have -rea off, got: {args}");
        assert!(args.contains("-ncmoe 12"), "20.6GB spills without -ncmoe, got: {args}");
    }

    /// The 35B must never be tuned back below the agent overhead. Claude Code sends
    /// ~21k tokens of system prompt and tool definitions before reading a single
    /// file, and a request past `n_ctx` comes back as an empty HTTP 200 rather than
    /// an error — so a too-small window presents as the model hanging, not failing.
    #[test]
    fn large_model_windows_clear_the_agent_overhead() {
        const AGENT_OVERHEAD: u32 = 21_000;
        for model in [QWEN_35B, "Qwen3.6-35B-A3B-abliterated-MTP-GGUF-Q2_K"] {
            let (ctx, _, _) = lemonade_tuning(model);
            assert!(
                ctx >= AGENT_OVERHEAD * 3,
                "{model}: ctx {ctx} leaves too little room above the ~21k agent \
                 overhead; overflow returns an empty 200 and reads as a hang"
            );
        }
        // The unmeasured-large fallback must clear it too.
        let (ctx, _, _) = lemonade_tuning_by_size(20.0);
        assert!(ctx >= AGENT_OVERHEAD * 2, "large fallback ctx {ctx} is too small");
    }

    /// Every Qwen3 model needs `-rea off`, not just the big one. Measured on
    /// Qwen3-0.6B: without it EVERY completion returned empty, because the whole
    /// token budget went inside `<think>` and Lemonade's Anthropic adapter drops
    /// thinking content. It presents as "completion silently does nothing".
    #[test]
    fn every_role_disables_reasoning() {
        for role in [ModelRole::Agent, ModelRole::Completion] {
            for model in lemonade_role_candidates(role) {
                let (_, args, _) = lemonade_role_tuning(role, model);
                assert!(
                    args.contains("-rea off"),
                    "{role:?}/{model}: missing -rea off; reasoning models return empty without it"
                );
            }
        }
    }

    /// Completion is latency-bound. A big window costs KV that the agent model
    /// sharing the card needs, and buys nothing — the prompt is cursor-local.
    #[test]
    fn completion_role_keeps_a_small_window() {
        let (ctx, args, _) = lemonade_role_tuning(ModelRole::Completion, "Qwen3-0.6B-GGUF-BF16");
        assert!(ctx <= 16_384, "completion ctx {ctx} wastes VRAM the agent model needs");
        assert!(!args.contains("-ncmoe"), "expert offload is meaningless for a dense small model");
    }

    /// The agent role must never silently become the completion model. The 35B
    /// is the user's choice for security and long agentic work; a reorder here
    /// would change which model does that job without anyone noticing.
    #[test]
    fn agent_role_leads_with_the_35b_moe() {
        assert_eq!(lemonade_role_candidates(ModelRole::Agent)[0], QWEN_35B);
        // And the agent role keeps the measured large-model tuning.
        let (ctx, args, _) = lemonade_role_tuning(ModelRole::Agent, QWEN_35B);
        assert_eq!(ctx, 98304);
        assert!(args.contains("-ncmoe 12"));
    }

    /// A local model must never be cut off mid-answer by the client. The failure
    /// is silent and self-worsening: the client retries, and the retry queues
    /// behind the request that is still running.
    #[test]
    fn api_timeout_covers_a_full_length_answer() {
        // 35B at its measured rate, 16k output budget.
        let ms = claude_api_timeout_ms(13.3, 16_384);
        let generation_s = 16_384.0 / 13.3;
        assert!(
            ms as f32 / 1000.0 > generation_s,
            "timeout {ms}ms is shorter than the {generation_s:.0}s the model needs to generate"
        );
        // Never below 30 minutes, even for a fast model with a small budget.
        assert!(claude_api_timeout_ms(30.3, 2_048) >= 1_800_000);
        // A missing/zero measurement must not produce a zero timeout.
        assert!(claude_api_timeout_ms(0.0, 8_192) >= 1_800_000);
    }

    /// Compaction, not a normal turn, sets the floor. It re-prefills the whole
    /// conversation cold and then summarises it: measured at over 16 minutes on
    /// the 35B at ~68k of context. A 15-minute timeout expired mid-compaction and
    /// the retry restarted it, so the session could never finish compacting.
    #[test]
    fn api_timeout_outlasts_a_real_compaction() {
        const OBSERVED_COMPACTION_S: u64 = 16 * 60 + 4;
        for (tps, out) in [(13.3, 4_096u32), (16.6, 4_096), (30.3, 4_096)] {
            let ms = claude_api_timeout_ms(tps, out);
            assert!(
                ms / 1000 > OBSERVED_COMPACTION_S,
                "timeout {}s would expire during a {OBSERVED_COMPACTION_S}s compaction",
                ms / 1000
            );
        }
    }

    /// Declaring the true window means compaction starts too late to fit. Claude
    /// Code compacts at ~80% of what it is told, and its own prompt and tools add
    /// ~36,700 tokens on top of the conversation.
    #[test]
    fn declared_context_leaves_room_to_compact() {
        for ctx in [65_536u32, 98_304] {
            let declared = claude_max_context_tokens(ctx);
            assert!(declared < ctx, "declared {declared} must be under the served {ctx}");
            // Where auto-compact actually fires, plus the summary it must generate.
            let compaction_request = (declared as f32 * 0.80) as u32 + claude_max_output_tokens();
            assert!(
                compaction_request < ctx,
                "compaction at ctx {ctx} would need {compaction_request} tokens and overflow"
            );
            // And the conversation alone must still leave room for the fixed overhead.
            assert!(
                declared > CLAUDE_CODE_OVERHEAD_TOKENS,
                "ctx {ctx} leaves no room for the {CLAUDE_CODE_OVERHEAD_TOKENS} overhead"
            );
        }
    }

    /// A 32k window CANNOT host Claude Code. Its system prompt and tool schemas
    /// alone are 36,700 tokens — more than the whole window — so the session fails
    /// before any conversation exists. Several models in the tuning table are
    /// configured at 32,768, and offering them for an agent session is a trap:
    /// the failure looks like the model being broken, not the window being small.
    #[test]
    fn tiny_windows_cannot_host_claude_code() {
        assert!(!claude_code_viable_ctx(32_768), "32k cannot fit the fixed overhead");
        assert!(!claude_code_viable_ctx(16_384));
        assert!(claude_code_viable_ctx(65_536), "64k is tight but usable");
        assert!(claude_code_viable_ctx(98_304));
    }

    /// The Edit role must never be served by the tiny completion model. A 0.6B
    /// can continue a line but cannot refactor, debug or review a PR, and a fast
    /// wrong answer is worse than a slow right one for those.
    #[test]
    fn edit_role_excludes_the_tiny_completion_model() {
        let edit = lemonade_role_candidates(ModelRole::Edit);
        assert!(!edit.is_empty());
        for m in edit {
            assert!(
                !m.contains("0.6B"),
                "{m} is too small to refactor or review; Edit needs real capability"
            );
        }
    }

    /// Edit-role models must be abliterated — these commands are used on security
    /// work, and a refusing model breaks the feature in the user's main use case.
    #[test]
    fn edit_role_models_are_uncensored() {
        for m in lemonade_role_candidates(ModelRole::Edit) {
            let ml = m.to_lowercase();
            assert!(
                ml.contains("abliterated") || ml.contains("heretic") || ml.contains("uncensored"),
                "{m} is not abliterated and will refuse security work"
            );
        }
    }

    /// With nothing resident, role resolution must return the caller's model
    /// unchanged rather than inventing one — the whole point is that it never
    /// causes a load. Proven by pointing at a dead port.
    #[tokio::test]
    async fn role_resolution_falls_back_when_nothing_is_resident() {
        // Explicit base, not an env var: `LEMONADE_URL` is process-global and
        // setting it here leaked into a parallel test in another module.
        let out = model_for_role_or_at(ModelRole::Edit, QWEN_35B, "http://127.0.0.1:1").await;
        assert_eq!(out, QWEN_35B, "must fall back to the caller's model, not load one");
    }

    /// Roles must resolve to DIFFERENT models. If they collapse to one, the
    /// completion path inherits the 35B's 13.3 tok/s and Tab becomes unusable
    /// while looking correctly wired.
    #[test]
    fn roles_do_not_collapse_onto_one_model() {
        assert_ne!(
            lemonade_role_candidates(ModelRole::Agent)[0],
            lemonade_role_candidates(ModelRole::Completion)[0],
            "completion must not be served by the agent model - 13.3 tok/s cannot serve Tab"
        );
    }

    #[test]
    fn unknown_model_falls_back_by_size() {
        assert_eq!(lemonade_tuning_by_size(6.87), (131072, LEMONADE_ARGS_SMALL, 0.0));
        assert_eq!(lemonade_tuning_by_size(12.2), (65536, LEMONADE_ARGS_MID, 0.0));
        assert_eq!(lemonade_tuning_by_size(20.6), (65536, LEMONADE_ARGS_LARGE, 0.0));
        // Unmeasured models report 0.0 so the warm-up check has no baseline to
        // compare against and stays quiet rather than crying wolf.
        assert_eq!(lemonade_tuning_by_size(20.6).2, 0.0);
    }

    /// An unmeasured model has no baseline, so the probe must bail out before
    /// issuing any request — proven here by pointing at a dead port and still
    /// returning immediately.
    #[tokio::test]
    async fn warm_check_skips_unmeasured_models() {
        let client = reqwest::Client::new();
        let out = lemonade_warm_check(&client, "http://127.0.0.1:1", "whatever", 0.0).await;
        assert_eq!(out, None);
    }

    /// The 25% floor has to sit outside the 8% run-to-run variance band, or the
    /// probe reloads a perfectly good model. Checked against readings actually
    /// taken on this hardware: the 35B measured 16.2 and 17.7 tok/s against an
    /// expected 15.9.
    #[test]
    fn bad_load_floor_clears_normal_variance() {
        let expected = 15.9_f32;
        let floor = expected * 0.75;
        for measured in [16.2_f32, 17.7, 15.9, 14.6] {
            assert!(measured >= floor, "{measured} tok/s would falsely trip the bad-load reload");
        }
        // A genuine bad load sits ~25% low and must trip it.
        assert!(expected * 0.74 < floor);
    }

    #[test]
    fn two_bit_quants_are_refused_for_tool_calling() {
        assert!(!supports_tool_calling(GEMMA_26B_Q2));
        assert!(!supports_tool_calling("some-model-IQ2_XXS"));
        assert!(!supports_tool_calling("some-model-IQ1_S"));
    }

    #[test]
    fn named_broken_models_are_refused() {
        assert!(!supports_tool_calling(GPT_OSS));
    }

    #[test]
    fn working_models_are_allowed() {
        assert!(supports_tool_calling(GEMMA_12B_Q4));
        assert!(supports_tool_calling(QWEN_35B));
        // Slow (2-5 tok/s hybrid-SSM) is not the same as broken — these can
        // still call tools, so the capability gate must not reject them.
        assert!(supports_tool_calling("Qwen3.6-27B-Abliterated-Heretic-Uncensored-GGUF-Q3_K_M"));
        assert!(supports_tool_calling("Qwythos-27B-v1-GGUF-Q4_K_M"));
    }
}

