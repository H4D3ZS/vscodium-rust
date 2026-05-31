/**
 * Global Ollama request guard.
 *
 * Many AIRI subsystems hardcode model tags (`qwen3.6:14b-q4_K_M`,
 * `huihui_ai/qwen3.5-abliterated:35b`, etc.) that almost certainly aren't on
 * the user's local Ollama. Each one would otherwise loop forever, spamming
 * 404s at `:11434/api/generate`.
 *
 * This module installs a one-time `fetch` interceptor that:
 *  - Caches the list of locally-available models (`/api/tags`).
 *  - Rewrites the `model` field of `POST /api/generate` and `/api/chat`
 *    payloads when the requested tag isn't installed, swapping it for the
 *    user's preferred fallback (defaults to `airi.consciousness.model` from
 *    localStorage, then the smallest available model).
 *  - After too many consecutive failures across the whole subsystem, marks
 *    the guard as disabled so requests short-circuit instead of hammering
 *    the server.
 *
 * Importing this module for its side-effects is enough; `installOllamaGuard`
 * is idempotent. We call it from `main.tsx` via `kokoro-worker-wrapper`.
 */

import { invoke } from '../tauri_bridge';

const OLLAMA_PATHS = ['/api/generate', '/api/chat'];
const TAGS_PATH = '/api/tags';
const STORAGE_FALLBACK_KEYS = [
  'airi.consciousness.model',
  'airi.vision.model',
];

const guard = {
  installed: false,
  available: null as Set<string> | null,
  refreshing: null as Promise<void> | null,
  consecutiveFailures: 0,
  disabledUntil: 0,
  lastWarn: 0,
};

function readFallbackPreferences(): string[] {
  const out: string[] = [];
  if (typeof localStorage === 'undefined') return out;
  for (const key of STORAGE_FALLBACK_KEYS) {
    try {
      const v = localStorage.getItem(key);
      if (v && !out.includes(v)) out.push(v);
    } catch {
      // localStorage blocked (tracking prevention).
    }
  }
  return out;
}

function chooseFallback(): string | null {
  const prefs = readFallbackPreferences();
  if (!guard.available) {
    return prefs[0] ?? null;
  }
  for (const p of prefs) {
    if (guard.available.has(p)) return p;
  }
  // Pick a lightweight default if it's installed.
  const cheap = ['llama3.2:3b', 'llama3.2:1b', 'gemma2:2b', 'qwen2.5:3b'];
  for (const c of cheap) {
    if (guard.available.has(c)) return c;
  }
  // Otherwise: just take the first alphabetically — better than 404.
  const first = guard.available.values().next();
  return first.done ? null : first.value;
}

function isOllamaTarget(url: string): { host: string; path: string } | null {
  try {
    const u = new URL(url, typeof location !== 'undefined' ? location.href : 'http://localhost');
    const path = u.pathname;
    if (!OLLAMA_PATHS.includes(path) && path !== TAGS_PATH) return null;
    return { host: `${u.protocol}//${u.host}`, path };
  } catch {
    return null;
  }
}

async function refreshAvailable(host: string): Promise<void> {
  if (guard.refreshing) return guard.refreshing;
  guard.refreshing = (async () => {
    try {
      let data: { models?: Array<{ name?: string; model?: string }> };
      const tauri = typeof window !== 'undefined' && (window as any).__TAURI__;
      if (tauri) {
        await invoke('set_ollama_url', { url: host }).catch(() => {});
        data = (await invoke<typeof data>('ollama_native_get', { path: '/api/tags' })) as typeof data;
      } else {
        const res = await fetch(`${host}${TAGS_PATH}`);
        if (!res.ok) return;
        data = await res.json();
      }
      const names: string[] = Array.isArray(data?.models)
        ? data.models!.map((m) => String(m?.name || m?.model || '')).filter(Boolean)
        : [];
      guard.available = new Set(names);
    } catch {
      // Leave guard.available untouched — we'll retry next call.
    } finally {
      guard.refreshing = null;
    }
  })();
  return guard.refreshing;
}

function maybeWarn(msg: string): void {
  const now = Date.now();
  if (now - guard.lastWarn < 5000) return;
  guard.lastWarn = now;
  console.warn('[OllamaGuard]', msg);
}

async function rewriteBody(
  init: RequestInit | undefined,
  host: string,
): Promise<RequestInit | undefined> {
  if (!init || !init.body || typeof init.body !== 'string') return init;
  let payload: any;
  try {
    payload = JSON.parse(init.body);
  } catch {
    return init;
  }
  const requested = String(payload?.model || '').trim();
  if (!requested) return init;
  if (!guard.available) await refreshAvailable(host);
  if (!guard.available || guard.available.size === 0) return init;
  if (guard.available.has(requested)) return init;
  const fallback = chooseFallback();
  if (!fallback || fallback === requested) {
    maybeWarn(`Model "${requested}" not installed and no fallback available. Request will 404.`);
    return init;
  }
  maybeWarn(`Model "${requested}" not installed — substituting "${fallback}".`);
  payload.model = fallback;
  return { ...init, body: JSON.stringify(payload) };
}

export function installOllamaGuard(): void {
  if (guard.installed) return;
  if (typeof globalThis.fetch !== 'function') return;
  guard.installed = true;

  const original = globalThis.fetch.bind(globalThis);

  globalThis.fetch = (async (input: RequestInfo | URL, init?: RequestInit) => {
    const url = typeof input === 'string'
      ? input
      : input instanceof URL
        ? input.href
        : (input as Request).url;
    const target = isOllamaTarget(url);
    if (!target) return original(input as any, init);

    // Short-circuit when we've decided Ollama isn't reachable / useful right now.
    if (guard.disabledUntil > Date.now() && target.path !== TAGS_PATH) {
      return new Response(
        JSON.stringify({ error: 'AIRI Ollama integration paused (too many failures).' }),
        { status: 503, headers: { 'content-type': 'application/json' } },
      );
    }

    let effectiveInit = init;
    if (target.path !== TAGS_PATH) {
      try {
        effectiveInit = await rewriteBody(init, target.host);
      } catch {
        // ignore – fall through with original init
      }
    }

    let res: Response;
    try {
      res = await original(input as any, effectiveInit);
    } catch (err) {
      guard.consecutiveFailures += 1;
      if (guard.consecutiveFailures >= 10) {
        guard.disabledUntil = Date.now() + 60_000;
        maybeWarn('Pausing AIRI Ollama traffic for 60s after repeated network errors.');
      }
      throw err;
    }

    if (target.path === TAGS_PATH && res.ok) {
      try {
        const cloned = res.clone();
        const data: any = await cloned.json();
        const names: string[] = Array.isArray(data?.models)
          ? data.models.map((m: any) => String(m?.name || m?.model || '')).filter(Boolean)
          : [];
        guard.available = new Set(names);
      } catch {
        // ignore
      }
    }

    if (res.status === 404 && target.path !== TAGS_PATH) {
      guard.consecutiveFailures += 1;
      if (guard.consecutiveFailures >= 20) {
        guard.disabledUntil = Date.now() + 120_000;
        maybeWarn('Pausing AIRI Ollama traffic for 2min after repeated 404s — check installed models.');
      }
    } else if (res.ok) {
      guard.consecutiveFailures = 0;
    }

    return res;
  }) as typeof fetch;
}

export function ollamaGuardState() {
  return {
    installed: guard.installed,
    availableModels: guard.available ? Array.from(guard.available) : [],
    consecutiveFailures: guard.consecutiveFailures,
    disabledUntil: guard.disabledUntil,
  };
}
