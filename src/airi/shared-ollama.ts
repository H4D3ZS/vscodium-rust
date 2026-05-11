/**
 * Shared Ollama client for every AIRI subsystem.
 *
 * When the UI runs from `http://localhost:5173` (Vite) or `tauri://` / `https://`
 * webview origins, direct `fetch()` to a remote Ollama proxy is often blocked by
 * nginx CORS (`Access-Control-Allow-Origin` allow-list). Rust has no CORS layer, so
 * under Tauri we route `/api/tags`, `/api/generate`, and `/api/chat` through
 * `ollama_native_get` / `ollama_native_post` instead of the browser `Ollama` client.
 *
 * Host + optional `Authorization: Bearer …` still come from Settings → Ollama URL
 * and API Keys; `refreshOllamaConfig` keeps them in sync with the bridge.
 */
import type { Ollama } from 'ollama';
import { Ollama as OllamaClient } from 'ollama';
import { invoke } from '../tauri_bridge';

type Headers = Record<string, string>;

let cachedHost: string = 'http://localhost:11434';
let cachedHeaders: Headers | undefined;
let bootstrapped = false;
let bootstrapPromise: Promise<void> | null = null;

function normalizeHost(raw: string): string {
    let s = (raw || '').trim();
    if (!s) return 'http://localhost:11434';
    if (!/^https?:\/\//i.test(s)) s = `http://${s}`;
    return s.replace(/\/+$/, '');
}

function readStoredHost(): string {
    try {
        if (typeof localStorage === 'undefined') return 'http://localhost:11434';
        const v = localStorage.getItem('ollamaUrl');
        return normalizeHost(v || 'http://localhost:11434');
    } catch {
        return 'http://localhost:11434';
    }
}

function isTauri(): boolean {
    return typeof window !== 'undefined' && !!(window as any).__TAURI__;
}

/**
 * When the renderer is the Vite dev server (or any non-Tauri browser surface
 * served from the same host as the IDE bundle), we cannot send the configured
 * remote Ollama URL directly — nginx CORS will reject it. Instead we route
 * through the same-origin dev proxy at `/__ollama` declared in `vite.config.ts`.
 * Tauri builds never take this path because they use Rust IPC.
 */
function browserOllamaBase(): string {
    if (typeof window === 'undefined') return getOllamaHost();
    const host = getOllamaHost();
    try {
        const u = new URL(host);
        const pageOrigin = window.location.origin;
        if (u.origin === pageOrigin) return host; // same-origin local Ollama; no proxy
    } catch {
        /* malformed host → fall through to proxy */
    }
    return `${window.location.origin}/__ollama`;
}

async function bootstrap(): Promise<void> {
    if (bootstrapped) return;
    if (bootstrapPromise) return bootstrapPromise;
    bootstrapPromise = (async () => {
        try {
            cachedHost = readStoredHost();
            if (isTauri()) {
                try {
                    const keys = await invoke<Record<string, string>>('get_api_keys');
                    const tok = keys?.ollama?.trim();
                    if (tok) cachedHeaders = { Authorization: `Bearer ${tok}` };
                } catch {
                    /* no keys yet */
                }
            }
        } finally {
            bootstrapped = true;
        }
    })();
    return bootstrapPromise;
}

/**
 * Force a fresh host/headers snapshot. Call from the agent bridge once the
 * store + API keys are loaded, and again whenever settings change.
 */
export function refreshOllamaConfig(host?: string, headers?: Headers | null): void {
    if (host !== undefined) cachedHost = normalizeHost(host);
    if (headers !== undefined) cachedHeaders = headers ?? undefined;
    bootstrapped = true;
}

export function getOllamaHost(): string {
    if (!bootstrapped) cachedHost = readStoredHost();
    return cachedHost;
}

export function getOllamaHeaders(): Headers | undefined {
    return cachedHeaders;
}

async function syncRustOllamaUrl(): Promise<void> {
    if (!isTauri()) return;
    try {
        await invoke('set_ollama_url', { url: getOllamaHost() });
    } catch {
        /* best-effort */
    }
}

// ─── Installed-model cache + auto-fallback ──────────────────────────────────
// AIRI subsystems still ship hardcoded model tags (`gemma3:12b`,
// `qwen3.6:32b-q4_K_M`, …) that almost certainly aren't on a paying
// customer's remote VPS. The browser-side ollama-guard handles this by
// rewriting the `fetch` body, but that interceptor never sees the Tauri IPC
// path. We replicate the same substitution here so AIRI uses whatever the
// user actually has installed (preferring their selected agent model).

let installedModels: Set<string> | null = null;
let refreshingInstalled: Promise<void> | null = null;
let lastInstalledRefresh = 0;
const INSTALLED_TTL_MS = 60_000;
const FALLBACK_STORAGE_KEYS = [
    'agentModel', // "ollama|tag"
    'airi.consciousness.model',
    'airi.vision.model',
];

function preferredFallbackTags(): string[] {
    const out: string[] = [];
    if (typeof localStorage === 'undefined') return out;
    for (const key of FALLBACK_STORAGE_KEYS) {
        try {
            let v = localStorage.getItem(key);
            if (!v) continue;
            v = v.includes('|') ? v.split('|')[1] : v;
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
    if (!installedModels || installedModels.size === 0) return prefs[0] ?? null;
    for (const p of prefs) if (installedModels.has(p)) return p;
    const cheap = ['llama3.2:3b', 'llama3.2:1b', 'gemma2:2b', 'qwen2.5:3b'];
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
                .map((m) => String(m?.name || m?.model || '').trim())
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

async function substituteUnknownModel(req: Record<string, unknown>): Promise<Record<string, unknown>> {
    const requested = typeof req.model === 'string' ? req.model.trim() : '';
    if (!requested) return req;
    await refreshInstalled();
    if (!installedModels || installedModels.size === 0) return req;
    if (installedModels.has(requested)) return req;
    const fallback = chooseFallback();
    if (!fallback || fallback === requested) return req;
    console.warn(
        `[AIRI Ollama] Model "${requested}" not installed on this server — substituting "${fallback}".`,
    );
    return { ...req, model: fallback };
}

async function tauriListRaw(): Promise<{ models: Array<{ name: string; [k: string]: unknown }> }> {
    await syncRustOllamaUrl();
    const data = await invoke<Record<string, unknown>>('ollama_native_get', { path: '/api/tags' });
    return data as { models: Array<{ name: string; [k: string]: unknown }> };
}

async function tauriList(): Promise<{ models: Array<{ name: string; [k: string]: unknown }> }> {
    const data = await tauriListRaw();
    installedModels = new Set(
        (data?.models || []).map((m) => String(m?.name || (m as any)?.model || '').trim()).filter(Boolean),
    );
    lastInstalledRefresh = Date.now();
    return data;
}

async function tauriGenerate(request: Record<string, unknown>): Promise<Record<string, unknown>> {
    if (request.stream === true) {
        throw new Error(
            'AIRI: streaming Ollama generate is not supported through the Tauri bridge; use stream: false.',
        );
    }
    const finalReq = await substituteUnknownModel(request);
    await syncRustOllamaUrl();
    return invoke<Record<string, unknown>>('ollama_native_post', {
        path: '/api/generate',
        body: finalReq,
    });
}

async function tauriChat(request: Record<string, unknown>): Promise<Record<string, unknown>> {
    if (request.stream === true) {
        throw new Error(
            'AIRI: streaming Ollama chat is not supported through the Tauri bridge; use stream: false.',
        );
    }
    const finalReq = await substituteUnknownModel(request);
    await syncRustOllamaUrl();
    return invoke<Record<string, unknown>>('ollama_native_post', {
        path: '/api/chat',
        body: finalReq,
    });
}

/** Drop the cached model list; next request will refresh from `/api/tags`. */
export function invalidateInstalledModelCache(): void {
    installedModels = null;
    lastInstalledRefresh = 0;
}

function buildClient(): Ollama {
    const host = isTauri() ? getOllamaHost() : browserOllamaBase();
    const headers = getOllamaHeaders();
    return new OllamaClient({ host, ...(headers ? { headers } : {}) } as any);
}

/**
 * Proxy-wrapped Ollama client. Under Tauri, `list` / `generate` / `chat` use IPC
 * (no CORS). Other methods fall back to the browser client (may still hit CORS
 * on exotic remote-only setups).
 */
export function createSharedOllama(): Ollama {
    void bootstrap();
    if (isTauri()) {
        return new Proxy({} as Record<string, unknown>, {
            get(_target, prop) {
                if (prop === 'list') return tauriList.bind(null);
                if (prop === 'generate') return (req: Record<string, unknown>) => tauriGenerate(req);
                if (prop === 'chat') return (req: Record<string, unknown>) => tauriChat(req);
                const client = buildClient();
                const value = (client as any)[prop];
                return typeof value === 'function' ? (value as Function).bind(client) : value;
            },
        }) as unknown as Ollama;
    }
    const handler: ProxyHandler<Record<string, unknown>> = {
        get(_target, prop) {
            const client = buildClient();
            const value = (client as any)[prop];
            return typeof value === 'function' ? (value as Function).bind(client) : value;
        },
    };
    return new Proxy({} as any, handler) as Ollama;
}

export async function fetchOllama(path: string, init?: RequestInit): Promise<Response> {
    void bootstrap();
    const method = (init?.method || 'GET').toUpperCase();
    if (isTauri()) {
        await syncRustOllamaUrl();
        try {
            if (method === 'GET') {
                const data = await invoke<Record<string, unknown>>('ollama_native_get', { path });
                return new Response(JSON.stringify(data), {
                    status: 200,
                    headers: { 'Content-Type': 'application/json' },
                });
            }
            if (method === 'POST' && init?.body) {
                const raw = typeof init.body === 'string' ? init.body : JSON.stringify(init.body);
                const body = JSON.parse(raw) as Record<string, unknown>;
                const data = await invoke<Record<string, unknown>>('ollama_native_post', {
                    path,
                    body,
                });
                return new Response(JSON.stringify(data), {
                    status: 200,
                    headers: { 'Content-Type': 'application/json' },
                });
            }
        } catch (e) {
            const msg = e instanceof Error ? e.message : String(e);
            return new Response(JSON.stringify({ error: msg }), {
                status: 502,
                headers: { 'Content-Type': 'application/json' },
            });
        }
    }
    const host = browserOllamaBase();
    const headers: Record<string, string> = {
        ...(init?.headers as Record<string, string> | undefined),
        ...(cachedHeaders ?? {}),
    };
    const url = `${host}${path.startsWith('/') ? path : `/${path}`}`;
    return fetch(url, { ...init, headers });
}
