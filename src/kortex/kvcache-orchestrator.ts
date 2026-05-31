/**
 * Kortex Disk KV Cache (KDKVC) — frontend bindings.
 *
 * Ports antirez/ds4's "KV cache is a first-class disk citizen" design onto the
 * llama.cpp / llama-server stack. The Rust side runs an axum HTTP proxy that
 * fronts llama-server and:
 *
 *   1. tokenizes the rolling chat prefix,
 *   2. computes SHA-256 over the LE-encoded u32 token stream (ds4 scheme),
 *   3. looks up the longest cached prefix in `<index_dir>/<sha>.kkv`,
 *   4. on hit  -> POSTs `/slots/0?action=restore` so llama-server skips prefill
 *               for the matching tokens,
 *   5. on miss -> forwards the request as-is and saves the slot afterwards.
 *
 * This delivers the CCET "token burning bound" in practice: tokens already in
 * a previous session never get re-prefilled, regardless of whether
 * llama-server was restarted in between.
 */

import { invoke } from '../tauri_bridge';

export interface KvCacheOptions {
    /** Directory holding our `.kkv` index sidecars. */
    index_dir: string;
    /** Directory holding llama-server's slot binaries. Must equal the value
     *  passed to `kortex_gac_launch` as `slot_save_path`. */
    slot_dir: string;
    /** Total bytes budget for the slot binaries; LRU evicts when exceeded. */
    max_bytes: number;
    /** Don't cache prefixes shorter than this. Default 512. */
    min_tokens: number;
    /** Hard ceiling on saved prefix length. Default 30000. */
    cold_max_tokens: number;
    /** Trim this many tokens off the saved prefix. Default 32. */
    boundary_trim_tokens: number;
    /** Align saved prefixes down to this multiple. Default 2048. */
    boundary_align_tokens: number;
    /** Save again when conversation grows by this many tokens. Default 10000. */
    continued_interval_tokens: number;
    /** Slot id (always 0 in single-slot servers). */
    slot_id: number;
    /** Upstream llama-server URL the proxy forwards to. */
    upstream_url: string;
    /** Bind address for the proxy. */
    proxy_host: string;
    /** Bind port for the proxy. */
    proxy_port: number;
}

export interface KvCacheStats {
    entries: number;
    total_bytes: number;
    hits: number;
    misses: number;
    saves: number;
    evictions: number;
    /** Sum of tokens we *didn't* prefill thanks to cache hits. */
    tokens_skipped: number;
}

export interface RunningCacheInfo {
    proxy_url: string;
    upstream_url: string;
    index_dir: string;
    slot_dir: string;
    max_bytes: number;
    slot_id: number;
    /** Stable identifier of the model the proxy is currently bound to (the
     *  basename of the GGUF, or `model.id` from llama-server's `/props`).
     *  Empty when the proxy couldn't reach `/props` at startup. */
    model_id?: string;
    /** Quantisation signature parsed from the GGUF filename, e.g. "Q4_K_M".
     *  Used by the cache to decide whether a hit from a different quant is
     *  acceptable under the configured ModelMatchPolicy. */
    quant_signature?: string;
    /** SHA-256 of the chat template — distinguishes "same GGUF path but
     *  tokenizer config got swapped underneath us". */
    tokenizer_hash?: string;
}

export const DEFAULT_KV_CACHE_OPTS: Omit<KvCacheOptions, 'index_dir' | 'slot_dir'> = {
    max_bytes: 16 * 1024 * 1024 * 1024,
    min_tokens: 512,
    cold_max_tokens: 30_000,
    boundary_trim_tokens: 32,
    boundary_align_tokens: 2048,
    continued_interval_tokens: 10_000,
    slot_id: 0,
    upstream_url: 'http://127.0.0.1:8081',
    proxy_host: '127.0.0.1',
    proxy_port: 8090,
};

/**
 * Construct a default-pathed options bundle. Caller can override any field.
 *
 * Convention: `<base>/index/` for `.kkv` sidecars, `<base>/slots/` for the
 * slot binaries that llama-server owns.
 */
export function makeKvCacheOptions(baseDir: string, overrides: Partial<KvCacheOptions> = {}): KvCacheOptions {
    return {
        ...DEFAULT_KV_CACHE_OPTS,
        index_dir: `${baseDir}/index`,
        slot_dir: `${baseDir}/slots`,
        ...overrides,
    };
}

/**
 * Boot the proxy. Returns the bound port (== `opts.proxy_port`). The upstream
 * llama-server must already be running and reachable at `opts.upstream_url`.
 *
 * The IDE should re-route any code that posts to `:8081` to instead post to
 * `http://${proxy_host}:${proxy_port}` after this resolves.
 */
export async function startKvCache(opts: KvCacheOptions): Promise<number> {
    return invoke<number>('kortex_kvcache_start', { opts });
}

export async function stopKvCache(): Promise<void> {
    await invoke<void>('kortex_kvcache_stop');
}

export async function getKvCacheStats(): Promise<KvCacheStats> {
    return invoke<KvCacheStats>('kortex_kvcache_stats');
}

export async function getKvCacheStatus(): Promise<RunningCacheInfo | null> {
    const info = await invoke<RunningCacheInfo | null>('kortex_kvcache_status');
    return info ?? null;
}

/** Wipe all cached prefixes. Returns the number of entries that were removed. */
export async function clearKvCache(): Promise<number> {
    return invoke<number>('kortex_kvcache_clear');
}

/**
 * Format stats as a one-liner for the status bar:
 *   "12 entries, 3.4 GB, 87% hit rate, 142k tokens skipped"
 */
export function summarizeKvCache(stats: KvCacheStats): string {
    const total = stats.hits + stats.misses;
    const hitRate = total > 0 ? Math.round((stats.hits / total) * 100) : 0;
    const gb = (stats.total_bytes / (1024 * 1024 * 1024)).toFixed(2);
    const skipped = stats.tokens_skipped >= 1000
        ? `${(stats.tokens_skipped / 1000).toFixed(1)}k`
        : `${stats.tokens_skipped}`;
    return `${stats.entries} entries, ${gb} GB, ${hitRate}% hit rate, ${skipped} tokens skipped`;
}
