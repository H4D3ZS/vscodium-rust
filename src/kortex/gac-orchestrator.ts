/**
 * Kortex GAC Orchestrator
 *
 * Front-end binding for the Rust `kortex_gac` module. Encapsulates the full flow:
 *
 *   profile  -> plan  -> launch  -> stream
 *
 * The hot promise: a single call to `startKortexInference(modelPath, opts)`
 * makes the IDE detect / generate a `<model>.geometry.aim` profile, build a
 * tier plan against the user's VRAM budget, spin up `llama-server` with the
 * geometry-aware overrides, and return a ready-to-use base URL.
 *
 * The math behind the overrides comes from the geometry-of-consolidation
 * theorem (see `geometry-of-consolidation/gac/theory.py`):
 *
 *     eps_id  >=  1 - c1 * (theta'/d_bar)^d_eff
 *
 * Tensors with small d_bar / large rho ("tight regime") tolerate any
 * compression — they're the safe ones to ship to CPU. Tensors with large
 * d_bar / small rho ("spread regime") need precision and bandwidth — they win
 * the GPU. This is the only thing `--n-gpu-layers N` doesn't know.
 */

import { invoke } from '../tauri_bridge';

export type Regime = 'tight' | 'borderline' | 'spread';
export type TierBuffer = 'Gpu' | 'Cpu';
export type Backend = 'cuda' | 'rocm' | 'vulkan' | 'metal' | 'sycl';

export interface TensorGeometry {
    name: string;
    shape: number[];
    quant_type: string;
    size_bytes: number;
    block_index: number | null;
    geometry: {
        d_bar: number;
        d_eff: number;
        rho: number;
        regime: Regime;
    };
}

export interface MoeMeta {
    expert_count: number;
    expert_used_count: number;
}

export interface GeometryProfile {
    version: number;
    model_path: string;
    model_size_bytes: number;
    block_count: number;
    tensors: TensorGeometry[];
    global: {
        d_eff_global: number;
        d_bar_mean: number;
        n_profiled: number;
    };
    profiler_config: {
        sample_rows: number;
        seed: number;
    };
    /** MoE metadata; null for dense models. */
    moe?: MoeMeta | null;
}

export interface TensorOverride {
    pattern: string;
    buffer: TierBuffer;
    bytes: number;
}

export interface RoutingCounts {
    spread_to_gpu: number;
    spread_to_cpu: number;
    borderline_to_gpu: number;
    borderline_to_cpu: number;
    tight_to_gpu: number;
    tight_to_cpu: number;
}

export interface TierPlan {
    n_gpu_layers: number;
    overrides: TensorOverride[];
    total_gpu_bytes: number;
    total_cpu_bytes: number;
    vram_budget_mb: number;
    theta: number;
    d_bar_critical: number;
    routing_counts: RoutingCounts;
    backend: string;
}

export interface PlanOptions {
    /** Total physical VRAM in MB. RX 580 8GB = 8192. */
    vram_total_mb: number;
    /** Fraction of VRAM reserved for KV cache + activations. Default 0.30. */
    kv_reserve_frac?: number;
    /** Retrieval threshold theta. Default 0.85. */
    theta?: number;
    /** Override the global d_eff. None = read from profile. */
    d_eff_global?: number;
    safe_mult?: number;
    unsafe_mult?: number;
    /** Backend determines the GPU buffer name in --override-tensor flags. */
    backend?: Backend;
}

export interface RunningInfo {
    port: number;
    host: string;
    argv: string[];
    uptime_secs: number;
    /** False if the llama-server process has exited since spawn. */
    alive?: boolean;
    /** Exit code once it has exited. */
    exit_code?: number | null;
}

export interface LaunchExtras {
    /** Absolute path to llama-server. If omitted, kortex resolves it from PATH. */
    server_binary?: string;
    /** HTTP port. Default 8081 (matches LlamaCppService default). */
    port?: number;
    /** Bind host. Default 127.0.0.1. */
    host?: string;
    /** Context window size. Default 8192. */
    ctx_size?: number;
    /** Number of CPU threads. Default 0 (let llama.cpp pick). */
    n_threads?: number;
    /** Batch size. Default 512. */
    batch_size?: number;
    /** Enable --flash-attn (off by default; Vulkan/RX 580 may not support it). */
    flash_attn?: boolean;
    /** Directory passed as `--slot-save-path` so the Kortex KV cache proxy can
     *  save / restore slot KV state across server restarts. */
    slot_save_path?: string;
    /** Verbatim extra args appended to the launch line. */
    extra_args?: string[];
    /**
     * Speculative decoding — the software path around the memory-bandwidth
     * wall. A comma-separated list from the ROCmFPX fork's `--spec-type` menu,
     * in priority order. The big model verifies every drafted token, so the
     * output is bit-identical to a plain run.
     *   - `ngram-*` (`ngram-simple`, `ngram-map-k`, `ngram-map-k4v`,
     *     `ngram-mod`, `ngram-cache`) — zero model, zero VRAM; guesses the
     *     continuation from the prompt/context. Big win on code.
     *   - `draft-mtp` — the model's own multi-token head (auto-discovered).
     *   - `draft-eagle3` / `draft-simple` / `draft-dflash` / `draft-dspark` —
     *     a separate small draft/EAGLE GGUF (needs `draft_model_path`).
     */
    spec_type?: string;
    /** Draft / EAGLE-3 GGUF path — required for the `draft-simple`/`-eagle3`/
     *  `-dflash`/`-dspark` types; optional for `draft-mtp`; unused for `ngram-*`. */
    draft_model_path?: string;
    /** Draft-model GPU layers. Default: all (it's tiny). Ignored without a draft model. */
    draft_ngl?: number;
    /** Tokens to speculate per step (fork default 16). */
    draft_max?: number;
    /** Persisted n-gram cache file for `ngram-cache` (`--lookup-cache-dynamic`). */
    lookup_cache?: string;
    /** Seconds to wait for /health before returning. Default 60. 0 = don't wait. */
    wait_healthy_secs?: number;
}

/** Valid `--spec-type` names in the ROCmFPX fork (mirror of launcher.rs SPEC_TYPES). */
export const SPEC_TYPES = [
    'ngram-simple', 'ngram-map-k', 'ngram-map-k4v', 'ngram-mod', 'ngram-cache',
    'draft-mtp', 'draft-eagle3', 'draft-simple', 'draft-dflash', 'draft-dspark',
] as const;

/** Read persisted speculative-decoding settings (off by default). The Rust
 *  launcher filters unknown names and drops `draft-*` types with no model, so
 *  this stays permissive and just forwards what the user picked. */
export function specDecodeExtras(): Pick<LaunchExtras,
    'spec_type' | 'draft_model_path' | 'draft_ngl' | 'draft_max' | 'lookup_cache'> {
    try {
        const raw = (localStorage.getItem('kortex.spec.type') || '').trim();
        if (!raw) return {};
        const type = raw
            .split(',')
            .map((s) => s.trim())
            .filter((s) => (SPEC_TYPES as readonly string[]).includes(s))
            .join(',');
        if (!type) return {};
        const dm = localStorage.getItem('kortex.spec.draftModel') || undefined;
        const ngl = Number(localStorage.getItem('kortex.spec.draftNgl') || '') || undefined;
        const dmax = Number(localStorage.getItem('kortex.spec.draftMax') || '') || undefined;
        const lc = localStorage.getItem('kortex.spec.lookupCache') || undefined;
        return {
            spec_type: type,
            draft_model_path: /(?:^|,)draft-/.test(type) ? dm : undefined,
            draft_ngl: ngl,
            draft_max: dmax,
            lookup_cache: type.split(',').includes('ngram-cache') ? lc : undefined,
        };
    } catch { return {}; }
}

const DEFAULT_PLAN_OPTS: Required<Pick<PlanOptions,
    'kv_reserve_frac' | 'theta' | 'safe_mult' | 'unsafe_mult' | 'backend'>> = {
    kv_reserve_frac: 0.30,
    theta: 0.85,
    safe_mult: 0.75,
    unsafe_mult: 1.25,
    backend: 'vulkan',
};

function fillPlanOpts(opts: PlanOptions) {
    return {
        vram_total_mb: opts.vram_total_mb,
        kv_reserve_frac: opts.kv_reserve_frac ?? DEFAULT_PLAN_OPTS.kv_reserve_frac,
        theta: opts.theta ?? DEFAULT_PLAN_OPTS.theta,
        d_eff_global: opts.d_eff_global ?? null,
        safe_mult: opts.safe_mult ?? DEFAULT_PLAN_OPTS.safe_mult,
        unsafe_mult: opts.unsafe_mult ?? DEFAULT_PLAN_OPTS.unsafe_mult,
        backend: opts.backend ?? DEFAULT_PLAN_OPTS.backend,
    };
}

/**
 * Walk a GGUF and write a `<model>.geometry.aim` profile next to it.
 * Returns the absolute path to the written profile.
 *
 * Cost: profiling a 35B Q4_K_M takes ~30s on a desktop CPU (one dequant per
 * block, 256 rows sampled, Jacobi eigendecomposition on each 256×256 Gram).
 * The profile is reusable across runs — generate once per model.
 */
export async function profileModel(modelPath: string, sampleRows = 256, seed = 0xC0FFEE): Promise<string> {
    return invoke<string>('kortex_gac_profile', {
        modelPath,
        sampleRows,
        seed,
    });
}

/** Read an existing profile from disk. */
export async function loadProfile(profilePath: string): Promise<GeometryProfile> {
    return invoke<GeometryProfile>('kortex_gac_load_profile', { profilePath });
}

/** Build a tier plan from a profile + budget. */
export async function planTiers(profilePath: string, options: PlanOptions): Promise<TierPlan> {
    return invoke<TierPlan>('kortex_gac_plan', {
        profilePath,
        options: fillPlanOpts(options),
    });
}

/**
 * One-shot: profile-or-cache + plan. Skips re-profiling if the profile exists
 * and `refreshProfile` is false.
 */
export async function quickPlan(
    modelPath: string,
    options: PlanOptions,
    refreshProfile = false,
): Promise<TierPlan> {
    return invoke<TierPlan>('kortex_gac_quickplan', {
        modelPath,
        options: fillPlanOpts(options),
        refreshProfile,
    });
}

/** Render a plan as a llama-server argv list (without launching). Useful for debugging. */
export async function renderArgs(plan: TierPlan): Promise<string[]> {
    return invoke<string[]>('kortex_gac_render_args', { plan });
}

/** Launch llama-server with the given plan. Returns server info once /health is up. */
export async function launchServer(
    plan: TierPlan,
    modelPath: string,
    extras: LaunchExtras = {},
): Promise<RunningInfo> {
    return invoke<RunningInfo>('kortex_gac_launch', {
        plan,
        serverBinary: extras.server_binary ?? null,
        modelPath,
        port: extras.port ?? null,
        host: extras.host ?? null,
        ctxSize: extras.ctx_size ?? null,
        nThreads: extras.n_threads ?? null,
        batchSize: extras.batch_size ?? null,
        flashAttn: extras.flash_attn ?? null,
        slotSavePath: extras.slot_save_path ?? null,
        specType: extras.spec_type ?? null,
        draftModelPath: extras.draft_model_path ?? null,
        draftNgl: extras.draft_ngl ?? null,
        draftMax: extras.draft_max ?? null,
        lookupCache: extras.lookup_cache ?? null,
        extraArgs: extras.extra_args ?? null,
        waitHealthySecs: extras.wait_healthy_secs ?? null,
    });
}

export async function stopServer(): Promise<void> {
    await invoke<void>('kortex_gac_stop');
}

export async function getRunningServer(): Promise<RunningInfo | null> {
    const info = await invoke<RunningInfo | null>('kortex_gac_status');
    return info ?? null;
}

/** Tail of the llama-server log (stdout+stderr), for showing load progress. */
export async function getServerLog(lines = 80): Promise<string[]> {
    try { return await invoke<string[]>('kortex_gac_log', { lines }); } catch { return []; }
}

export async function defaultProfilePath(modelPath: string): Promise<string> {
    return invoke<string>('kortex_gac_default_profile_path', { modelPath });
}

// ─────────────────────────────────────────────────────────────────────────────
// High-level convenience: one call from the IDE settings to spin up inference.
// ─────────────────────────────────────────────────────────────────────────────

export interface KortexBootOptions extends PlanOptions {
    /** GGUF model path. */
    model_path: string;
    /** Force re-profile even if a `.geometry.aim` already exists next to the model. */
    refresh_profile?: boolean;
    /** llama-server launch options. */
    launch?: LaunchExtras;
}

export interface KortexBootResult {
    plan: TierPlan;
    server: RunningInfo;
    /** HTTP base URL the IDE should point its inference client at. */
    base_url: string;
}

/**
 * The "one button" path. Profile (or read) -> plan -> launch -> wait healthy.
 * Returns the running server info plus the convenient base_url.
 */
export async function startKortexInference(opts: KortexBootOptions): Promise<KortexBootResult> {
    const plan = await quickPlan(opts.model_path, opts, opts.refresh_profile);
    // Persisted spec-decode toggles apply unless the caller overrode them explicitly.
    const launch: LaunchExtras = { ...specDecodeExtras(), ...(opts.launch ?? {}) };
    const server = await launchServer(plan, opts.model_path, launch);
    const base_url = `http://${server.host}:${server.port}`;
    return { plan, server, base_url };
}

/**
 * A trivial "all layers on GPU" plan. Skips the GAC geometry profiler, which
 * reads the GGUF with candle and chokes on custom quant types (ROCmFP2/FP4 →
 * "unknown dtype for tensor N"). Models that fit VRAM don't need tensor
 * tiering anyway — just `-ngl 999`.
 */
export function fullGpuPlan(backend: Backend = 'vulkan', vramMb = 16384): TierPlan {
    return {
        n_gpu_layers: 999,
        overrides: [],
        total_gpu_bytes: 0,
        total_cpu_bytes: 0,
        vram_budget_mb: vramMb,
        theta: 0,
        d_bar_critical: 0,
        routing_counts: {
            spread_to_gpu: 0, spread_to_cpu: 0,
            borderline_to_gpu: 0, borderline_to_cpu: 0,
            tight_to_gpu: 0, tight_to_cpu: 0,
        },
        backend,
    };
}

/**
 * Launch llama-server for a model that fits VRAM, with no profiling step.
 * Use this for ROCmFPX / custom-quant GGUFs. Returns the same shape as
 * `startKortexInference`.
 */
export async function startLocalInference(opts: {
    model_path: string;
    backend?: Backend;
    vram_total_mb?: number;
    launch?: LaunchExtras;
}): Promise<KortexBootResult> {
    const plan = fullGpuPlan(opts.backend ?? 'vulkan', opts.vram_total_mb ?? 16384);
    const launch: LaunchExtras = { ...specDecodeExtras(), ...(opts.launch ?? {}) };
    const server = await launchServer(plan, opts.model_path, launch);
    const base_url = `http://${server.host}:${server.port}`;
    return { plan, server, base_url };
}

/**
 * Format a routing summary in one line. Useful for the status bar / settings UI.
 *   "spread→GPU 24, tight→CPU 28, theta=0.85, d̄_crit=0.151, GPU 5.4G / CPU 14.3G"
 */
export function summarizePlan(plan: TierPlan): string {
    const c = plan.routing_counts;
    const gpu_g = (plan.total_gpu_bytes / (1024 * 1024 * 1024)).toFixed(2);
    const cpu_g = (plan.total_cpu_bytes / (1024 * 1024 * 1024)).toFixed(2);
    return [
        `spread→GPU ${c.spread_to_gpu}`,
        `borderline→GPU ${c.borderline_to_gpu}`,
        `tight→CPU ${c.tight_to_cpu}`,
        `θ=${plan.theta.toFixed(2)}`,
        `d̄_crit=${plan.d_bar_critical.toFixed(3)}`,
        `GPU ${gpu_g}G / CPU ${cpu_g}G`,
    ].join(', ');
}
