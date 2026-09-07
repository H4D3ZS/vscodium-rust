/**
 * Shared local-LLM client for every AIRI subsystem.
 *
 * Lemonade is the only local backend (real llama.cpp) and it exposes an
 * the local backend-compatible surface at `/api/generate` and `/api/chat`, which
 * is what the `the local backend` npm client speaks. It also sends
 * `Access-Control-Allow-Origin: *`, so the webview can call it directly — the
 * old Rust CORS-bypass bridge (`native_api_get`/`_post`) is gone.
 *
 * Host + optional `Authorization: Bearer …` come from Settings → Inference
 * Backend and API Keys; `refreshLocalModelConfig` keeps them in sync.
 */
import { invoke } from '../tauri_bridge';
import { useStore } from '../store';

type Headers = Record<string, string>;

let cachedHost: string = 'http://localhost:13305';
let cachedHeaders: Headers | undefined;
let bootstrapped = false;
let bootstrapPromise: Promise<void> | null = null;

/**
 * Lemonade's the local backend-compatible `/api/tags` mimics the local backend's tag format and
 * appends `:latest` to every id. Its native `/api/v1/models` — and the
 * Anthropic endpoint the agent actually calls — use the bare id. Strip the
 * synthetic tag so one canonical name flows everywhere; a persisted
 * `…:latest` otherwise fails model validation on every request.
 */
export function canonicalLocalModelId(name: string): string {
    return (name || '').trim().replace(/:latest$/i, '');
}

function normalizeHost(raw: string): string {
    let s = (raw || '').trim();
    if (!s) return 'http://localhost:13305';
    if (!/^https?:\/\//i.test(s)) s = `http://${s}`;
    return s.replace(/\/+$/, '');
}

function readStoredHost(): string {
    try {
        if (typeof localStorage === 'undefined') return 'http://localhost:13305';
        const v = localStorage.getItem('lemonadeUrl') ?? localStorage.getItem('inferenceUrl');
        return normalizeHost(v || 'http://localhost:13305');
    } catch {
        return 'http://localhost:13305';
    }
}

/**
 * When the renderer is the Vite dev server (or any non-Tauri browser surface
 * served from the same host as the IDE bundle), we cannot send the configured
 * remote the local backend URL directly — nginx CORS will reject it. Instead we route
 * Tauri builds never take this path because they use Rust IPC.
 */
/**
 * Force a fresh host/headers snapshot. Call from the agent bridge once the
 * store + API keys are loaded, and again whenever settings change.
 */
export function refreshLocalModelConfig(host?: string, headers?: Headers | null): void {
    if (host !== undefined) cachedHost = normalizeHost(host);
    if (headers !== undefined) cachedHeaders = headers ?? undefined;
    bootstrapped = true;
}

export function getLocalModelHost(): string {
    if (!bootstrapped) cachedHost = readStoredHost();
    return cachedHost;
}

export function getLocalModelHeaders(): Headers | undefined {
    return cachedHeaders;
}

// ─── Concurrency gate + 503 retry ──────────────────────────────────────────
// AIRI fires many background generate/chat calls in parallel (consciousness,
// vision, continuous-improvement, self-learning, social, …). A single VPS
// behind nginx with `limit_conn inference_conn 20` will reject the burst with
// 503. This gate serializes traffic to a small concurrency cap and applies
// exponential backoff when nginx (or the local backend) signals pressure.

const MAX_CONCURRENT_LOCAL = 3;
let inflight = 0;
const waiters: Array<() => void> = [];

async function acquireSlot(): Promise<() => void> {
    if (inflight < MAX_CONCURRENT_LOCAL) {
        inflight++;
        let released = false;
        return () => {
            if (released) return;
            released = true;
            inflight--;
            const next = waiters.shift();
            if (next) next();
        };
    }
    await new Promise<void>((resolve) => waiters.push(resolve));
    return acquireSlot();
}

function isRetryableInferenceError(err: unknown): { retry: boolean; backoffMs: number } {
    const msg = (err instanceof Error ? err.message : String(err || '')).toLowerCase();
    if (msg.includes('503') || msg.includes('service temporarily unavailable')) {
        return { retry: true, backoffMs: 1500 };
    }
    if (msg.includes('429') || msg.includes('too many requests')) {
        return { retry: true, backoffMs: 2000 };
    }
    if (msg.includes('limit_conn') || msg.includes('limiting connections')) {
        return { retry: true, backoffMs: 1500 };
    }
    return { retry: false, backoffMs: 0 };
}

const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));

async function withInferenceConcurrency<T>(label: string, fn: () => Promise<T>): Promise<T> {
    const MAX_ATTEMPTS = 4;
    let attempt = 0;
    let release = await acquireSlot();
    try {
        while (true) {
            attempt++;
            try {
                return await fn();
            } catch (err) {
                const { retry, backoffMs } = isRetryableInferenceError(err);
                if (!retry || attempt >= MAX_ATTEMPTS) throw err;
                const jitter = Math.floor(Math.random() * 250);
                const wait = backoffMs * Math.pow(2, attempt - 1) + jitter;
                console.warn(
                    `[local-inference] ${label} hit upstream throttle (attempt ${attempt}/${MAX_ATTEMPTS}); retrying in ${wait}ms.`,
                );
                await sleep(wait);
            }
        }
    } finally {
        release();
    }
}

// ─── Installed-model cache + auto-fallback ──────────────────────────────────
// AIRI subsystems still ship hardcoded model tags (`gemma3:12b`,
// `qwen3.6:32b-q4_K_M`, …) that almost certainly aren't on a paying
// customer's remote VPS. The browser-side the local backend-guard handles this by
// rewriting the `fetch` body, but that interceptor never sees the Tauri IPC
// path. We replicate the same substitution here so AIRI uses whatever the
// user actually has installed (preferring their selected agent model).

let installedModels: Set<string> | null = null;
let refreshingInstalled: Promise<void> | null = null;
let lastInstalledRefresh = 0;
const INSTALLED_TTL_MS = 60_000;
const FALLBACK_STORAGE_KEYS = [
    'agentModel', // persisted by store.setAgentModel — "lemonade|namespace/tag"
    'airi.consciousness.model',
    'airi.vision.model',
];

let lastSubstWarnKey = '';
let lastSubstWarnAt = 0;

function maybeWarnSubstitution(requested: string, fallback: string): void {
    const key = `${requested}→${fallback}`;
    const now = Date.now();
    if (key === lastSubstWarnKey && now - lastSubstWarnAt < 15_000) return;
    lastSubstWarnKey = key;
    lastSubstWarnAt = now;
    console.warn(
        `[local-inference] Model "${requested}" not installed on this server — substituting "${fallback}".`,
    );
}

function normalizeModelTag(s: string): string {
    return s.replace(/:latest$/i, '').trim().toLowerCase();
}

/** Map a requested tag (often a default like \`qwen3:35b\`) to an installed name (e.g. \`balia/qwen3.6-35b\`). */
function fuzzyMatchInstalled(requested: string, installed: Set<string>): string | null {
    const r = normalizeModelTag(requested);
    if (!r) return null;
    for (const m of installed) {
        if (normalizeModelTag(m) === r) return m;
    }
    const afterColon = r.includes(':') ? r.split(':').slice(1).join(':') : '';
    const compact = afterColon.replace(/[^a-z0-9.]/gi, '');
    if (compact.length >= 3) {
        for (const m of installed) {
            const ml = m.toLowerCase();
            if (ml.includes(compact)) return m;
        }
    }
    const stem = r.split(/[:/]/).filter((p) => p.length >= 3)[0];
    if (stem) {
        for (const m of installed) {
            if (m.toLowerCase().includes(stem)) return m;
        }
    }
    return null;
}

function resolveInstalledOrNull(requested: string): string | null {
    if (!installedModels?.size) return null;
    if (installedModels.has(requested)) return requested;
    return fuzzyMatchInstalled(requested, installedModels);
}

function preferredFallbackTags(): string[] {
    const out: string[] = [];
    try {
        const am = useStore.getState().agentModel;
        if (am?.includes('|')) {
            const id = am.split('|').slice(1).join('|').trim();
            if (id) out.push(id);
        }
    } catch {
        /* store not ready */
    }
    if (typeof localStorage === 'undefined') return out;
    for (const key of FALLBACK_STORAGE_KEYS) {
        try {
            let v = localStorage.getItem(key);
            if (!v) continue;
            v = v.includes('|') ? v.split('|').slice(1).join('|') : v;
            v = v.trim();
            if (v && !out.includes(v)) out.push(v);
        } catch {
            /* tracking prevention */
        }
    }
    return out;
}

function chooseFallback(): string | null {
    const prefs = preferredFallbackTags();
    if (!installedModels || installedModels.size === 0) {
        const firstPref = prefs[0];
        if (firstPref && !firstPref.toLowerCase().includes('hades') && !firstPref.toLowerCase().includes('qwen3.6')) {
            return firstPref;
        }
        return 'soft-eng-qwen:latest';
    }
    for (const p of prefs) {
        if (installedModels.has(p)) return p;
        const fuzzy = fuzzyMatchInstalled(p, installedModels);
        if (fuzzy) return fuzzy;
    }
    const cheap = ['airi-fast:latest', 'soft-eng-qwen:latest', 'llama3.2:3b', 'llama3.2:1b', 'gemma2:2b', 'qwen2.5:3b'];
    for (const c of cheap) if (installedModels.has(c)) return c;
    return installedModels.values().next().value ?? null;
}

async function refreshInstalled(force = false): Promise<void> {
    if (refreshingInstalled) return refreshingInstalled;
    const fresh = Date.now() - lastInstalledRefresh < INSTALLED_TTL_MS;
    if (installedModels && fresh && !force) return;
    refreshingInstalled = (async () => {
        try {
            const data = (await tauriListRaw()) as {
                models?: Array<{ name?: string; model?: string }>;
            };
            const names = (data?.models || [])
                .map((m) => canonicalLocalModelId(String(m?.name || m?.model || '')))
                .filter(Boolean);
            installedModels = new Set(names);
            lastInstalledRefresh = Date.now();
        } catch {
            /* leave previous cache intact */
        } finally {
            refreshingInstalled = null;
        }
    })();
    return refreshingInstalled;
}

/**
 * Public helper: pick a tag that exists on the configured server (exact,
 * fuzzy match on defaults, then user preference order).
 */
export async function resolveLocalModelTag(requested: string): Promise<string> {
    await refreshInstalled();
    const r = (requested || '').trim();
    if (!r) return chooseFallback() || 'airi-fast:latest';
    const hit = resolveInstalledOrNull(r);
    if (hit) return hit;
    
    const fallback = chooseFallback();
    if (fallback && installedModels && installedModels.has(fallback)) {
        return fallback;
    }
    if (installedModels && installedModels.size > 0) {
        const cheap = ['airi-fast:latest', 'soft-eng-qwen:latest', 'llama3.2:3b', 'llama3.2:1b', 'gemma2:2b', 'qwen2.5:3b'];
        for (const c of cheap) {
            if (installedModels.has(c)) return c;
        }
        return installedModels.values().next().value ?? 'airi-fast:latest';
    }
    return fallback || r;
}

/**
 * List local models from Lemonade's **native** catalog.
 *
 * Deliberately NOT `/api/tags`: that is Lemonade's the local backend-compatibility shim,
 * and it appends a synthetic `:latest` to every id, which then fails validation
 * against the native id everywhere else. `lemonade-claude.sh` reads
 * `/api/v1/models` and keys on `m.id` for exactly this reason — mirror it.
 *
 * Also filters the way the launcher does: downloaded, `llamacpp` recipe only,
 * so image/speech recipes (sd-cpp, whispercpp, kokoro) never reach a chat picker.
 */
async function tauriListRaw(): Promise<{ models: Array<{ name: string;[k: string]: unknown }> }> {
    return withInferenceConcurrency('list', async () => {
        const res = await fetch(`${getLocalModelHost()}/api/v1/models`, { headers: getLocalModelHeaders() });
        const data = (await res.json()) as Record<string, unknown>;
        const rows: any[] = Array.isArray((data as any)?.data)
            ? (data as any).data
            : Array.isArray(data) ? (data as any) : [];
        const models = rows
            .filter((m) => m?.downloaded && m?.recipe === 'llamacpp')
            .map((m) => ({ ...m, name: String(m?.id ?? '') }))
            .filter((m) => m.name);
        return { models } as { models: Array<{ name: string;[k: string]: unknown }> };
    });
}

async function tauriList(): Promise<{ models: Array<{ name: string;[k: string]: unknown }> }> {
    const data = await tauriListRaw();
    installedModels = new Set(
        (data?.models || [])
            .map((m) => canonicalLocalModelId(String(m?.name || (m as any)?.model || '')))
            .filter(Boolean),
    );
    lastInstalledRefresh = Date.now();
    return data;
}

/** Drop the cached model list; next request will refresh from `/api/v1/models`. */
