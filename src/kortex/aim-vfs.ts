/**
 * Kortex `.aim` neural VFS — frontend bindings.
 *
 * Mirrors `src-tauri/src/kortex_commands.rs::Telemetry*` types into TypeScript
 * so the renderer can read/write the durable side of the Kortex memory
 * system. Two `.aim` files live under `<userprofile>/.kortex/`:
 *
 *   - `airi_memory.aim`  — 1-token gist + relationship/personality metadata
 *                          (handled by src/airi/kortex-integration.ts).
 *   - `telemetry.aim`    — rolling inference telemetry ring buffer + lifetime
 *                          tallies + last KDKVC model identity (this file).
 *
 * The telemetry file is updated automatically by the store's
 * `recordKortexCompletion` action; the helpers below are for the panel UI
 * that wants to read snapshots or manage the file lifecycle directly.
 */

import { invoke } from '../tauri_bridge';

/** Mirrors `kortex_commands::TelemetrySample`. */
export interface AimTelemetrySample {
    wall_clock_ms: number;
    prefill_ms?: number | null;
    output_tokens: number;
    input_tokens: number;
    backend: string;
    cache_hit: boolean;
    tokens_skipped: number;
    model_id?: string | null;
    ts_unix_ms: number;
}

/** Mirrors the manual `Serialize for TelemetryState` impl on the Rust side. */
export interface AimTelemetrySnapshot {
    schema: string;
    capacity: number;
    lifetime_samples: number;
    lifetime_output_tokens: number;
    lifetime_input_tokens: number;
    lifetime_tokens_skipped: number;
    lifetime_cache_hits: number;
    last_model_id: string | null;
    last_quant_signature: string | null;
    last_tokenizer_hash: string | null;
  samples: AimTelemetrySample[];
}

export interface AimTrustManifest {
  schema: 'kortex.aim.trust/v1';
  status: 'ready' | 'degraded' | 'missing';
  path: string;
  root: string;
  valid: boolean;
  confidence: number;
  reasons: string[];
  sha256?: string;
  size_bytes?: number;
  modified_unix_ms?: number | null;
  age_ms?: number | null;
  header_end?: number | null;
  metadata?: Record<string, unknown>;
  git?: {
    head?: string | null;
    dirty_files: number;
  };
}

export interface AimSpan {
  file: string;
  absolute_path: string;
  line_start: number;
  line_end: number;
  score: number;
  hash: string;
  summary: string;
  snippet: string;
}

export interface AimSpanQueryResult {
  schema: 'kortex.aim.spans/v1';
  query: string;
  root: string;
  terms: string[];
  scanned_files: number;
  index_hits?: number;
  source?: 'aim-index+source' | 'source-scan';
  limit: number;
  spans: AimSpan[];
  reason?: string;
}

/**
 * Return the trust envelope for the active workspace AIM memory file.
 * This is the cheap first call every agent/provider should make before
 * relying on compressed context. High confidence means "use the gist";
 * degraded/missing means "fall back to exact source reads for this slice."
 */
export async function getAimTrustManifest(options: { path?: string; root?: string } = {}): Promise<AimTrustManifest> {
    return invoke<AimTrustManifest>('aim_trust_manifest', {
        path: options.path ?? null,
        root: options.root ?? null,
    });
}

/**
 * Ask Kortex for exact source spans related to a query. This is the bridge
 * from "compressed AIM map" to "verified source truth": agents should call
 * this before broad repo grep when AIM confidence is usable.
 */
export async function queryAimSpans(request: {
    query: string;
    root?: string;
    limit?: number;
    max_files?: number;
    max_bytes_per_file?: number;
}): Promise<AimSpanQueryResult> {
    return invoke<AimSpanQueryResult>('aim_query_spans', { request });
}

/**
 * Load `telemetry.aim` from disk into the Rust-side state. Idempotent. Call
 * once on IDE boot so the in-memory mirror has the latest disk contents
 * before any new completions get appended.
 *
 * Returns the snapshot that was just loaded. Missing file yields a clean
 * default state instead of an error.
 */
export async function loadAimTelemetry(path?: string): Promise<AimTelemetrySnapshot> {
    return invoke<AimTelemetrySnapshot>('aim_load_telemetry', { path: path ?? null });
}

/**
 * Read-only snapshot of the live state. Cheap (no disk IO); call from a
 * polling loop in the panel. Refresh interval of ~2-4 s is plenty since the
 * ring buffer only changes once per completion.
 */
export async function getAimTelemetrySnapshot(): Promise<AimTelemetrySnapshot> {
    return invoke<AimTelemetrySnapshot>('aim_telemetry_snapshot');
}

/**
 * Force a flush of the in-memory ring buffer to `telemetry.aim`. Called
 * automatically every 16 appends, on graceful proxy stop, and on IDE
 * shutdown — but exposing it manually lets the user "Save now" from the UI.
 */
export async function flushAimTelemetry(): Promise<void> {
    await invoke<void>('aim_flush_telemetry');
}

/**
 * Wipe the ring buffer but keep lifetime tallies. Useful when the user
 * wants to start a fresh measurement window without losing the cumulative
 * "tokens routed" totals.
 */
export async function clearAimTelemetrySamples(): Promise<void> {
    await invoke<void>('aim_clear_telemetry_samples');
}

/**
 * Format a snapshot as a single-line summary for the panel header.
 *   "1.4M lifetime tokens · 87% hit rate · bound to qwen3-coder.Q4_K_M.gguf"
 */
export function summarizeAimTelemetry(s: AimTelemetrySnapshot): string {
    const total = s.lifetime_output_tokens + s.lifetime_input_tokens;
    const tokens = total >= 1_000_000 ? `${(total / 1_000_000).toFixed(2)}M`
        : total >= 1_000 ? `${(total / 1_000).toFixed(1)}k`
            : `${total}`;
    const hitRate = s.lifetime_samples > 0
        ? Math.round((s.lifetime_cache_hits / s.lifetime_samples) * 100)
        : 0;
    const bound = s.last_model_id ? ` · bound to ${s.last_model_id}` : '';
    return `${tokens} lifetime tokens · ${hitRate}% hit rate · ${s.lifetime_samples} samples${bound}`;
}
