/**
 * Shared Ollama client for every AIRI subsystem.
 *
 * Each subsystem used to call `new Ollama({ host: 'http://localhost:11434' })`
 * directly, which hard-coded the host and skipped the Bearer token configured
 * for the user's remote proxy (e.g. `https://ai.cyberifrit.xyz`). This module
 * exposes a single proxy-backed client whose `host`/`headers` are recomputed
 * for every method call (`.list`, `.chat`, `.generate`, …), so updating the
 * Ollama URL in Settings or the Ollama API key in API Keys takes effect
 * immediately without restarting AIRI.
 */
import { Ollama } from 'ollama';
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
        // Tracking Prevention etc.
        return 'http://localhost:11434';
    }
}

async function bootstrap(): Promise<void> {
    if (bootstrapped) return;
    if (bootstrapPromise) return bootstrapPromise;
    bootstrapPromise = (async () => {
        try {
            cachedHost = readStoredHost();
            const tauri = typeof window !== 'undefined' && (window as any).__TAURI__;
            if (tauri) {
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

function buildClient(): Ollama {
    const host = getOllamaHost();
    const headers = getOllamaHeaders();
    return new Ollama({ host, ...(headers ? { headers } : {}) } as any);
}

/**
 * Proxy-wrapped Ollama client. Each property access yields a fresh `Ollama`
 * instance built from the latest host/headers, then forwards the call. Cheap
 * enough for chat/generate (which are network-bound anyway) and prevents the
 * stale-host problem when subsystems capture the client at construction time.
 */
export function createSharedOllama(): Ollama {
    void bootstrap();
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
    const host = getOllamaHost();
    const headers: Record<string, string> = {
        ...(init?.headers as Record<string, string> | undefined),
        ...(cachedHeaders ?? {}),
    };
    const url = `${host}${path.startsWith('/') ? path : `/${path}`}`;
    return fetch(url, { ...init, headers });
}
