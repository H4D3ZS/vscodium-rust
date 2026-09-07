//! llama-server launcher — turns a TierPlan into a running process.
//!
//! Spawns `llama-server` (or the user-supplied binary) with the planner's
//! tensor-placement flags plus the standard model/port/context options.
//! Tracks one server at a time via a global Mutex so we can stop it from a
//! Tauri command later.

use anyhow::{anyhow, Context, Result};
use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use super::planner::render_args;
use super::types::TierPlan;

/// Last N log lines from the running llama-server (stdout + stderr merged).
/// Drained by reader threads so the child never blocks on a full pipe — the
/// bug that made model loading hang forever with no visible progress.
static SERVER_LOG: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());
const SERVER_LOG_CAP: usize = 500;

fn server_log_path() -> PathBuf {
    std::env::temp_dir().join("kortex-llama-server.log")
}

fn server_log_reset() {
    if let Ok(mut buf) = SERVER_LOG.lock() {
        buf.clear();
    }
    let _ = std::fs::write(server_log_path(), b"");
}

fn server_log_push(line: &str) {
    if let Ok(mut buf) = SERVER_LOG.lock() {
        if buf.len() >= SERVER_LOG_CAP {
            buf.pop_front();
        }
        buf.push_back(line.to_string());
    }
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(server_log_path())
    {
        let _ = writeln!(f, "{}", line);
    }
}

/// The last `n` log lines from the running/most-recent llama-server.
pub fn server_log_tail(n: usize) -> Vec<String> {
    SERVER_LOG
        .lock()
        .map(|b| b.iter().rev().take(n).rev().cloned().collect())
        .unwrap_or_default()
}

/// Spawn a thread that copies a child stream line-by-line into the ring buffer.
fn spawn_log_pump<R: std::io::Read + Send + 'static>(stream: R, tag: &'static str) {
    std::thread::spawn(move || {
        let reader = BufReader::new(stream);
        for line in reader.lines().map_while(|l| l.ok()) {
            server_log_push(&format!("[{tag}] {line}"));
        }
    });
}

/// Options passed to the launcher in addition to the TierPlan.
#[derive(Debug, Clone)]
pub struct LaunchOpts {
    /// Path to the llama-server (or compatible) binary.
    pub server_binary: PathBuf,
    /// Path to the GGUF model.
    pub model_path: PathBuf,
    /// Port for the HTTP server. The IDE expects 8081 by default.
    pub port: u16,
    /// Bind address. Default 127.0.0.1.
    pub host: String,
    /// Context window size (`-c`).
    pub ctx_size: u32,
    /// Number of CPU threads (`-t`).
    pub n_threads: u32,
    /// Batch size (`-b`).
    pub batch_size: u32,
    /// Whether to enable flash attention (`--flash-attn`). Vulkan/ROCm support
    /// varies; default off so RX 580 owners don't hit silent breakage.
    pub flash_attn: bool,
    /// Directory for `--slot-save-path`. When set, llama-server can persist
    /// slot KV state, which the Kortex KV cache proxy (kortex_kvcache) layers
    /// SHA-keyed prefix matching on top of.
    pub slot_save_path: Option<PathBuf>,

    // ── Speculative decoding — the decode-throughput lever ──
    // The big model verifies every drafted token, so output is bit-identical to
    // a plain run; the speculator just proposes several tokens per forward pass
    // of the big weights. This is the software path around the memory-bandwidth
    // wall: N accepted tokens per weight read instead of one.
    //
    /// `--spec-type`: a comma-separated list from the ROCmFPX fork's menu, in
    /// priority order. Known names (`SPEC_TYPES`):
    ///   * `ngram-simple` / `ngram-map-k` / `ngram-map-k4v` / `ngram-mod` /
    ///     `ngram-cache` — zero model, zero VRAM. Guess the continuation by
    ///     matching recent tokens against the prompt / context. Big win on
    ///     code (imports, repeated identifiers, boilerplate).
    ///   * `draft-mtp` — the model's own multi-token-prediction head,
    ///     auto-discovered next to the `-hf` model or passed explicitly.
    ///   * `draft-eagle3` — a trained lightweight EAGLE-3 head (needs a model).
    ///   * `draft-simple` / `draft-dflash` / `draft-dspark` — a separate small
    ///     draft GGUF (needs a model).
    /// Unknown names are dropped; `draft-*` names that need a model are dropped
    /// when none is supplied. `None`/empty = no speculation. Stacking e.g.
    /// `"ngram-simple,draft-mtp"` lets the cheap guesser run first.
    pub spec_type: Option<String>,
    /// `--spec-draft-model` / `-md`: the small draft / EAGLE-3 GGUF. Required
    /// for `draft-simple` / `draft-eagle3` / `draft-dflash` / `draft-dspark`;
    /// optional for `draft-mtp` (auto-discovered); ignored for `ngram-*`.
    pub draft_model_path: Option<PathBuf>,
    /// `--spec-draft-ngl` / `-ngld`: draft-model GPU layers (put it fully on
    /// the GPU — it's tiny). Default: all. Ignored without a draft model.
    pub draft_ngl: Option<u32>,
    /// `--draft-max` / `--spec-draft-n-max`: tokens to speculate per step. The
    /// community sweeps for Qwen3.8-27B MTP put the mixed-workload optimum at
    /// **2** (3 for code-heavy); deeper loses on prose as acceptance decays.
    pub draft_max: Option<u32>,
    /// `--spec-draft-p-min`: confidence gate — the draft head stops once its
    /// own probability drops below this, so a deeper `draft_max` costs nothing
    /// on rounds it would have been rejected anyway. On a packed 16 GB card
    /// the sweeps land on ~0.75 (n-max 3); looser (0.60) buys noise.
    pub draft_p_min: Option<f32>,
    /// `--lookup-cache-dynamic` / `-lcd`: file the `ngram-cache` speculator
    /// reads at start and writes back on exit, so learned n-grams persist
    /// across server restarts. Only meaningful with `ngram-cache` in the list.
    pub lookup_cache: Option<PathBuf>,
    /// `--n-cpu-moe` / `-ncmoe`: keep the first N MoE layers' experts in system
    /// RAM instead of VRAM. The knob that lets a Q4 35B-A3B fit 16 GB — the
    /// dense attention/norm weights stay on the GPU, the fat expert FFNs spill
    /// to RAM. Only meaningful for a MoE model; `0`/`None` = everything on GPU.
    pub n_cpu_moe: Option<u32>,

    /// Extra free-form args appended verbatim. Useful for `--mlock`, `--no-mmap`, etc.
    pub extra_args: Vec<String>,
}

/// Speculative-decoding type names accepted by the ROCmFPX llama.cpp fork's
/// `--spec-type` (see `common/speculative.cpp::common_speculative_type_from_name_map`).
/// Anything not in this set makes the server abort at parse time, so the
/// launcher filters against it.
pub const SPEC_TYPES: &[&str] = &[
    "ngram-simple",
    "ngram-map-k",
    "ngram-map-k4v",
    "ngram-mod",
    "ngram-cache",
    "draft-mtp",
    "draft-eagle3",
    "draft-simple",
    "draft-dflash",
    "draft-dspark",
];

/// Does this spec type load a *separate* model that the launcher must be able
/// to point at? `draft-mtp` doesn't (the fork auto-discovers the head next to
/// the `-hf` model); the `ngram-*` guessers don't touch a model at all.
fn spec_type_needs_draft_model(t: &str) -> bool {
    matches!(t, "draft-simple" | "draft-eagle3" | "draft-dflash" | "draft-dspark")
}

impl Default for LaunchOpts {
    fn default() -> Self {
        Self {
            server_binary: PathBuf::from("llama-server"),
            model_path: PathBuf::new(),
            port: 8081,
            host: "127.0.0.1".to_string(),
            ctx_size: 8192,
            n_threads: 0, // 0 = let llama.cpp pick.
            batch_size: 512,
            flash_attn: false,
            slot_save_path: None,
            spec_type: None,
            draft_model_path: None,
            draft_ngl: None,
            draft_max: None,
            draft_p_min: None,
            lookup_cache: None,
            n_cpu_moe: None,
            extra_args: Vec::new(),
        }
    }
}

/// Build the full argv for `llama-server` given a plan + options.
pub fn build_argv(plan: &TierPlan, opts: &LaunchOpts) -> Vec<String> {
    let mut argv = Vec::new();
    argv.push("-m".into());
    argv.push(opts.model_path.to_string_lossy().into_owned());
    argv.push("--host".into());
    argv.push(opts.host.clone());
    argv.push("--port".into());
    argv.push(opts.port.to_string());
    argv.push("-c".into());
    argv.push(opts.ctx_size.to_string());
    if opts.n_threads > 0 {
        argv.push("-t".into());
        argv.push(opts.n_threads.to_string());
    }
    argv.push("-b".into());
    argv.push(opts.batch_size.to_string());
    if opts.flash_attn {
        argv.push("--flash-attn".into());
    }
    if let Some(p) = opts.slot_save_path.as_ref() {
        argv.push("--slot-save-path".into());
        argv.push(p.to_string_lossy().into_owned());
    }
    // MoE expert offload — spill the first N layers' experts to RAM so a Q4
    // 35B-A3B fits 16 GB VRAM. No-op on a dense model.
    if let Some(n) = opts.n_cpu_moe.filter(|n| *n > 0) {
        argv.push("--n-cpu-moe".into());
        argv.push(n.to_string());
    }

    // Speculative decoding (see LaunchOpts). The fork's `--spec-type` takes a
    // comma-separated list from SPEC_TYPES; unknown names abort the server, and
    // `draft-*` names that load a separate model are useless without one, so we
    // filter to the types that will actually run.
    let has_draft_model = opts.draft_model_path.is_some();
    let spec_list: Vec<&str> = opts
        .spec_type
        .as_deref()
        .unwrap_or("")
        .split(',')
        .map(str::trim)
        .filter(|t| !t.is_empty())
        .filter(|t| SPEC_TYPES.contains(t))
        .filter(|t| !spec_type_needs_draft_model(t) || has_draft_model)
        .collect();
    if !spec_list.is_empty() {
        let wants_ngram_cache = spec_list.contains(&"ngram-cache");
        let wants_mtp = spec_list.contains(&"draft-mtp");
        argv.push("--spec-type".into());
        argv.push(spec_list.join(","));
        if let Some(dm) = opts.draft_model_path.as_ref() {
            argv.push("--spec-draft-model".into());
            argv.push(dm.to_string_lossy().into_owned());
            argv.push("--spec-draft-ngl".into());
            argv.push(opts.draft_ngl.unwrap_or(999).to_string());
        }
        if let Some(n) = opts.draft_max {
            argv.push("--draft-max".into());
            argv.push(n.to_string());
        }
        if let Some(p) = opts.draft_p_min.filter(|p| (0.0..=1.0).contains(p)) {
            argv.push("--spec-draft-p-min".into());
            argv.push(format!("{p}"));
        }
        // MTP needs a single generation slot — with parallel slots the head is
        // disabled / degraded (sudoingX/qwen38-mtp). Force it unless the caller
        // already set --parallel / -np in extra_args.
        if wants_mtp
            && !opts
                .extra_args
                .iter()
                .any(|a| a == "--parallel" || a == "-np")
        {
            argv.push("--parallel".into());
            argv.push("1".into());
        }
        if wants_ngram_cache {
            if let Some(lc) = opts.lookup_cache.as_ref() {
                argv.push("--lookup-cache-dynamic".into());
                argv.push(lc.to_string_lossy().into_owned());
            }
        }
    }

    // GAC tier-placement flags.
    argv.extend(render_args(plan));
    // User-supplied extras last so they win over our defaults.
    argv.extend(opts.extra_args.clone());
    argv
}

/// One running `llama-server` process. Kept alive in `SERVER` until stopped.
pub struct ServerHandle {
    pub child: Child,
    pub port: u16,
    pub host: String,
    pub argv: Vec<String>,
    pub started_at: Instant,
}

/// Single-slot global server registry. We don't run two llama-servers in parallel
/// — the user's GPU couldn't host them anyway.
pub static SERVER: Mutex<Option<ServerHandle>> = Mutex::new(None);

/// Spawn `llama-server` with the given plan + opts and store the handle.
/// If a server is already running, returns Err — caller should `stop_server` first.
pub fn launch(plan: &TierPlan, opts: &LaunchOpts) -> Result<u16> {
    if !opts.model_path.is_file() {
        return Err(anyhow!("model not found at {}", opts.model_path.display()));
    }
    {
        let guard = SERVER.lock().unwrap();
        if guard.is_some() {
            return Err(anyhow!("a kortex llama-server is already running; stop it first"));
        }
    }

    // llama-server requires `--slot-save-path` to be an existing directory and
    // rejects a relative one oddly on Windows — create it and pass it absolute.
    let opts = if let Some(dir) = opts.slot_save_path.as_ref() {
        std::fs::create_dir_all(dir)
            .with_context(|| format!("could not create slot dir {}", dir.display()))?;
        let abs = std::fs::canonicalize(dir).unwrap_or_else(|_| dir.clone());
        // strip Windows \\?\ verbatim prefix which some tools choke on
        let abs = PathBuf::from(abs.to_string_lossy().trim_start_matches(r"\\?\"));
        let mut o = opts.clone();
        o.slot_save_path = Some(abs);
        o
    } else {
        opts.clone()
    };
    let opts = &opts;

    let argv = build_argv(plan, opts);
    tracing::info!(
        "[kortex-gac] launching {}: {}",
        opts.server_binary.display(),
        argv.join(" ")
    );

    let mut cmd = Command::new(&opts.server_binary);
    cmd.args(&argv);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    // On Windows, hide the spawned process from the user's taskbar.
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    server_log_reset();
    server_log_push(&format!("[gac] launching: {}", argv.join(" ")));

    let mut child = cmd
        .spawn()
        .with_context(|| format!("failed to spawn {}", opts.server_binary.display()))?;

    // Drain both pipes on their own threads. Without this the child blocks on a
    // full stdout/stderr pipe partway through loading the model and never
    // becomes healthy.
    if let Some(out) = child.stdout.take() {
        spawn_log_pump(out, "out");
    }
    if let Some(err) = child.stderr.take() {
        spawn_log_pump(err, "err");
    }

    let handle = ServerHandle {
        child,
        port: opts.port,
        host: opts.host.clone(),
        argv,
        started_at: Instant::now(),
    };
    *SERVER.lock().unwrap() = Some(handle);
    Ok(opts.port)
}

/// Wait until `GET /health` returns 200, or `timeout_secs` elapses.
/// Uses blocking reqwest to avoid pulling tokio into the launcher core.
pub async fn await_healthy(host: &str, port: u16, timeout_secs: u64) -> Result<()> {
    let url = format!("http://{}:{}/health", host, port);
    let deadline = Instant::now() + Duration::from_secs(timeout_secs);
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(2))
        .build()?;
    while Instant::now() < deadline {
        match client.get(&url).send().await {
            Ok(r) if r.status().is_success() => return Ok(()),
            _ => tokio::time::sleep(Duration::from_millis(500)).await,
        }
    }
    Err(anyhow!("llama-server did not become healthy within {}s", timeout_secs))
}

/// Kill the running server (if any). Returns Ok(()) even when no server is registered.
pub fn stop_server() -> Result<()> {
    let mut guard = SERVER.lock().unwrap();
    if let Some(mut handle) = guard.take() {
        let _ = handle.child.kill();
        let _ = handle.child.wait();
        tracing::info!("[kortex-gac] server stopped (port {})", handle.port);
    }
    Ok(())
}

/// Return basic info about the running server, or None. Also reports whether
/// the child is still alive (it may have crashed during model load).
pub fn current_server_info() -> Option<RunningInfo> {
    let mut guard = SERVER.lock().unwrap();
    let h = guard.as_mut()?;
    let exit_code = match h.child.try_wait() {
        Ok(Some(status)) => Some(status.code().unwrap_or(-1)),
        _ => None,
    };
    Some(RunningInfo {
        port: h.port,
        host: h.host.clone(),
        argv: h.argv.clone(),
        uptime_secs: h.started_at.elapsed().as_secs(),
        alive: exit_code.is_none(),
        exit_code,
    })
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct RunningInfo {
    pub port: u16,
    pub host: String,
    pub argv: Vec<String>,
    pub uptime_secs: u64,
    /// False if the process has exited since it was spawned.
    pub alive: bool,
    /// Exit code when it has exited, else None.
    pub exit_code: Option<i32>,
}

/// Try to find `llama-server` on PATH, honoring an optional explicit override.
pub fn resolve_server_binary(explicit: Option<&Path>) -> Result<PathBuf> {
    if let Some(p) = explicit {
        if p.is_file() {
            return Ok(p.to_path_buf());
        }
        return Err(anyhow!("server binary not found at {}", p.display()));
    }
    if let Ok(p) = which::which("llama-server") {
        return Ok(p);
    }
    if let Ok(p) = which::which("llama-server.exe") {
        return Ok(p);
    }
    Err(anyhow!(
        "could not find llama-server on PATH; pass an explicit path"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kortex_gac::types::{RoutingCounts, TensorOverride, TierBuffer, TierPlan};

    #[test]
    fn build_argv_includes_model_port_overrides() {
        let plan = TierPlan {
            n_gpu_layers: 33,
            overrides: vec![TensorOverride {
                pattern: r"blk\.\d+\.ffn_down\.(weight|bias)".into(),
                buffer: TierBuffer::Cpu,
                bytes: 0,
            }],
            total_gpu_bytes: 0,
            total_cpu_bytes: 0,
            vram_budget_mb: 0,
            theta: 0.85,
            d_bar_critical: 0.0,
            routing_counts: RoutingCounts::default(),
            backend: "vulkan".into(),
        };
        let opts = LaunchOpts {
            server_binary: PathBuf::from("llama-server"),
            model_path: PathBuf::from("model.gguf"),
            port: 8081,
            host: "127.0.0.1".into(),
            ctx_size: 4096,
            n_threads: 8,
            batch_size: 512,
            flash_attn: false,
            slot_save_path: None,
            extra_args: vec![],
            ..Default::default()
        };
        let argv = build_argv(&plan, &opts);
        assert!(argv.contains(&"-m".to_string()));
        assert!(argv.contains(&"model.gguf".to_string()));
        assert!(argv.contains(&"--port".to_string()));
        assert!(argv.contains(&"8081".to_string()));
        assert!(argv.contains(&"--n-gpu-layers".to_string()));
        assert!(argv.contains(&"--override-tensor".to_string()));
        // no speculation flags unless asked
        assert!(!argv.contains(&"--spec-type".to_string()));
    }

    fn min_plan() -> TierPlan {
        TierPlan {
            n_gpu_layers: 99,
            overrides: vec![],
            total_gpu_bytes: 0,
            total_cpu_bytes: 0,
            vram_budget_mb: 0,
            theta: 0.85,
            d_bar_critical: 0.0,
            routing_counts: RoutingCounts::default(),
            backend: "vulkan".into(),
        }
    }

    #[test]
    fn spec_type_mtp_needs_no_draft_model() {
        let opts = LaunchOpts {
            model_path: PathBuf::from("escha.gguf"),
            spec_type: Some("draft-mtp".into()),
            ..Default::default()
        };
        let argv = build_argv(&min_plan(), &opts);
        assert!(argv.windows(2).any(|w| w == ["--spec-type", "draft-mtp"]));
        // no separate model -> no --spec-draft-model / --spec-draft-ngl
        assert!(!argv.contains(&"--spec-draft-model".to_string()));
        assert!(!argv.contains(&"--spec-draft-ngl".to_string()));
    }

    #[test]
    fn ngram_types_run_with_no_model_and_no_vram() {
        let opts = LaunchOpts {
            model_path: PathBuf::from("escha.gguf"),
            spec_type: Some("ngram-simple".into()),
            ..Default::default()
        };
        let argv = build_argv(&min_plan(), &opts);
        assert!(argv.windows(2).any(|w| w == ["--spec-type", "ngram-simple"]));
        assert!(!argv.contains(&"--spec-draft-model".to_string()));
    }

    #[test]
    fn unknown_spec_names_are_filtered_out() {
        let opts = LaunchOpts {
            model_path: PathBuf::from("m.gguf"),
            // "draft" / "mtp" are NOT valid fork names; only ngram-mod survives.
            spec_type: Some("draft, mtp, bogus, ngram-mod".into()),
            ..Default::default()
        };
        let argv = build_argv(&min_plan(), &opts);
        assert!(argv.windows(2).any(|w| w == ["--spec-type", "ngram-mod"]));
    }

    #[test]
    fn draft_only_list_without_a_model_emits_nothing() {
        let opts = LaunchOpts {
            model_path: PathBuf::from("m.gguf"),
            spec_type: Some("draft-simple".into()),
            draft_model_path: None,
            ..Default::default()
        };
        assert!(!build_argv(&min_plan(), &opts).contains(&"--spec-type".to_string()));
    }

    #[test]
    fn spec_type_draft_emits_model_ngl_and_max() {
        let opts = LaunchOpts {
            model_path: PathBuf::from("m.gguf"),
            spec_type: Some("draft-simple".into()),
            draft_model_path: Some(PathBuf::from("draft-0.5b.gguf")),
            draft_ngl: Some(99),
            draft_max: Some(8),
            ..Default::default()
        };
        let argv = build_argv(&min_plan(), &opts);
        assert!(argv.windows(2).any(|w| w == ["--spec-type", "draft-simple"]));
        assert!(argv.windows(2).any(|w| w == ["--spec-draft-model", "draft-0.5b.gguf"]));
        assert!(argv.windows(2).any(|w| w == ["--spec-draft-ngl", "99"]));
        assert!(argv.windows(2).any(|w| w == ["--draft-max", "8"]));
    }

    #[test]
    fn stacked_ngram_plus_mtp_keeps_order_and_joins() {
        let opts = LaunchOpts {
            model_path: PathBuf::from("m.gguf"),
            spec_type: Some("ngram-simple,draft-mtp".into()),
            ..Default::default()
        };
        let argv = build_argv(&min_plan(), &opts);
        assert!(argv.windows(2).any(|w| w == ["--spec-type", "ngram-simple,draft-mtp"]));
    }

    #[test]
    fn ngram_cache_takes_a_dynamic_lookup_file() {
        let opts = LaunchOpts {
            model_path: PathBuf::from("m.gguf"),
            spec_type: Some("ngram-cache".into()),
            lookup_cache: Some(PathBuf::from("/w/.aim/ngram.bin")),
            ..Default::default()
        };
        let argv = build_argv(&min_plan(), &opts);
        assert!(argv.windows(2).any(|w| w == ["--lookup-cache-dynamic", "/w/.aim/ngram.bin"]));
    }

    #[test]
    fn lookup_file_is_ignored_without_ngram_cache() {
        let opts = LaunchOpts {
            model_path: PathBuf::from("m.gguf"),
            spec_type: Some("ngram-simple".into()),
            lookup_cache: Some(PathBuf::from("/w/.aim/ngram.bin")),
            ..Default::default()
        };
        assert!(!build_argv(&min_plan(), &opts).contains(&"--lookup-cache-dynamic".to_string()));
    }

    #[test]
    fn n_cpu_moe_emits_the_offload_flag_when_positive() {
        let opts = LaunchOpts {
            model_path: PathBuf::from("m.gguf"),
            n_cpu_moe: Some(22),
            ..Default::default()
        };
        let argv = build_argv(&min_plan(), &opts);
        assert!(argv.windows(2).any(|w| w == ["--n-cpu-moe", "22"]));
    }

    #[test]
    fn n_cpu_moe_zero_or_none_emits_nothing() {
        for n in [None, Some(0)] {
            let opts = LaunchOpts {
                model_path: PathBuf::from("m.gguf"),
                n_cpu_moe: n,
                ..Default::default()
            };
            assert!(!build_argv(&min_plan(), &opts).contains(&"--n-cpu-moe".to_string()));
        }
    }

    #[test]
    fn draft_mtp_forces_a_single_slot() {
        let opts = LaunchOpts {
            model_path: PathBuf::from("m.gguf"),
            spec_type: Some("draft-mtp".into()),
            ..Default::default()
        };
        let argv = build_argv(&min_plan(), &opts);
        assert!(argv.windows(2).any(|w| w == ["--parallel", "1"]));
    }

    #[test]
    fn ngram_only_does_not_force_a_single_slot() {
        let opts = LaunchOpts {
            model_path: PathBuf::from("m.gguf"),
            spec_type: Some("ngram-simple".into()),
            ..Default::default()
        };
        assert!(!build_argv(&min_plan(), &opts).contains(&"--parallel".to_string()));
    }

    #[test]
    fn caller_parallel_override_is_respected() {
        let opts = LaunchOpts {
            model_path: PathBuf::from("m.gguf"),
            spec_type: Some("draft-mtp".into()),
            extra_args: vec!["--parallel".into(), "4".into()],
            ..Default::default()
        };
        let argv = build_argv(&min_plan(), &opts);
        // we don't inject our own "1"; the caller's "4" rides in via extra_args
        assert!(argv.windows(2).any(|w| w == ["--parallel", "4"]));
        assert!(!argv.windows(2).any(|w| w == ["--parallel", "1"]));
    }

    #[test]
    fn draft_p_min_emits_the_gate_when_in_range() {
        let opts = LaunchOpts {
            model_path: PathBuf::from("m.gguf"),
            spec_type: Some("draft-mtp".into()),
            draft_p_min: Some(0.75),
            ..Default::default()
        };
        let argv = build_argv(&min_plan(), &opts);
        assert!(argv.windows(2).any(|w| w == ["--spec-draft-p-min", "0.75"]));

        // out-of-range is dropped
        let bad = LaunchOpts { draft_p_min: Some(1.5), ..opts.clone() };
        assert!(!build_argv(&min_plan(), &bad).contains(&"--spec-draft-p-min".to_string()));
    }
}
