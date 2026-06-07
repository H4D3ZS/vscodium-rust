/**
 * Composer 2 local parity (Cursor closed-source — Kimi K2.5/K2.6-class base + RL).
 * We ship routing presets, not Cursor weights.
 *
 * @see https://cursor.com/blog/composer-2
 */

import { invoke } from '@tauri-apps/api/core';
import { useStore } from '../store';

export const COMPOSER2_BLOG_URL = 'https://cursor.com/blog/composer-2';

/** Sharded HF GGUF cannot `ollama pull hf.co/...` — use Ollama library tags or llama-server. */
export const KIMI_K26_OLLAMA_TAGS = [
    'batiai/kimi-k2.6:iq3',
    'moonshotai/kimi-k2.5:cloud',
] as const;

/** Default GPU-server agent model for Composer 2 hybrid (3900X desk + remote Ollama). */
export const COMPOSER2_DEFAULT_GPU_MODEL = 'batiai/minimax-m2.7:iq3';

/** Models that must never run on the desk PC — remote GPU server only. */
export const COMPOSER2_GPU_SERVER_ONLY_PATTERNS = [
    /minimax-m2/i,
    /kimi-k2\.6/i,
    /kimi-k2\.5:cloud/i,
    /glm-5/i,
] as const;

/** Safe local chat tags for hybrid mode (3900X / RX 580). */
export const COMPOSER2_LOCAL_CHAT_DEFAULTS = [
    'qwen2.5-coder:7b',
    'airi-fast:latest',
    'qwen2.5-coder:14b',
] as const;

export function isGpuServerOnlyModel(modelTag: string): boolean {
    const m = modelTag.toLowerCase();
    if (COMPOSER2_GPU_SERVER_ONLY_PATTERNS.some((p) => p.test(m))) return true;
    return /(?:^|[/:\-_])(32|35|40|70|72|128|229)(?:b|-)/.test(m);
}

export function isLocalOllamaUrl(url: string): boolean {
    try {
        const host = new URL(url.replace(/\/$/, '') || 'http://127.0.0.1:11434').hostname.toLowerCase();
        return host === 'localhost' || host === '127.0.0.1' || host === '::1';
    } catch {
        return /localhost|127\.0\.0\.1/i.test(url);
    }
}

/**
 * Lighter GPU-server models that still work for Composer-style agent + planner.
 * IQ3 quants ≈ aggressive compression; MoE models activate fewer params per token.
 */
export const COMPOSER2_LIGHTER_GPU_MODELS = [
    {
        tag: 'batiai/minimax-m2.7:iq3',
        vramHint: '~24–48 GB',
        note: 'Best drop-in: MoE coding agent, less RAM than Kimi K2.6, strong tool loops.',
    },
    {
        tag: 'moonshotai/kimi-k2.5:cloud',
        vramHint: 'varies',
        note: 'Same model family Cursor Composer 2 builds on — often easier pull than community K2.6.',
    },
    {
        tag: 'qwen2.5-coder:32b',
        vramHint: '~20 GB Q4',
        note: 'Non-Kimi but excellent repo edits; good planner+executor on one GPU.',
    },
    {
        tag: 'deepseek-r1:32b',
        vramHint: '~20 GB Q4',
        note: 'Strong reasoning; slower than MoE, good for planning-only hybrid slot.',
    },
] as const;

export type Composer2OllamaMode = 'local' | 'remote' | 'hybrid';

export interface Composer2Stack {
    id: string;
    label: string;
    desc: string;
    ollamaMode: Composer2OllamaMode;
    /** Remote GPU server (planner + agent executor) */
    remoteHost?: string;
    /** Local Ollama for Composer 2 Fast chat (Ryzen 3900X box) */
    localHost?: string;
    /** Deep planner on GPU server */
    planner: string;
    /** Agent tool-loop model (remote in hybrid mode) */
    executor: string;
    /** Local fast chat model (Composer 2 Fast tier) */
    chatFast: string;
    /** Fallback agent model when remote Kimi unavailable (local RX 580 / CPU) */
    executorLocalFallback?: string;
    enableHybrid: boolean;
}

export const COMPOSER2_STACKS: Composer2Stack[] = [
    {
        id: 'composer2-amd3900',
        label: 'Composer 2 — AMD 3900X + RX 580 ★',
        desc:
            'Cursor agent parity for your desk: Ryzen 3900X runs fast chat (qwen2.5-coder:7b). ' +
            'MiniMax M2.7 IQ3 planner + agent executor on the remote GPU server. ' +
            'Pull on server: batiai/minimax-m2.7:iq3',
        ollamaMode: 'hybrid',
        remoteHost: '192.168.1.50',
        localHost: 'http://127.0.0.1:11434',
        planner: `ollama|${COMPOSER2_DEFAULT_GPU_MODEL}`,
        executor: `ollama|${COMPOSER2_DEFAULT_GPU_MODEL}`,
        chatFast: 'ollama|qwen2.5-coder:7b',
        executorLocalFallback: 'ollama|qwen2.5-coder:14b',
        enableHybrid: true,
    },
    {
        id: 'composer2-amd3900-kimi',
        label: 'Composer 2 — 3900X + Kimi K2.6 (heavier GPU)',
        desc:
            'Same hybrid desk setup, but GPU server runs Kimi K2.6 IQ3 instead of MiniMax — ' +
            'more VRAM, closer to Cursor Composer 2 base model. Pull: batiai/kimi-k2.6:iq3',
        ollamaMode: 'hybrid',
        remoteHost: '192.168.1.50',
        localHost: 'http://127.0.0.1:11434',
        planner: 'ollama|batiai/kimi-k2.6:iq3',
        executor: 'ollama|batiai/kimi-k2.6:iq3',
        chatFast: 'ollama|qwen2.5-coder:7b',
        executorLocalFallback: 'ollama|qwen2.5-coder:14b',
        enableHybrid: true,
    },
    {
        id: 'composer2-amd3900-local-only',
        label: 'Composer 2 Lite — 3900X local only',
        desc: 'No GPU server: fast chat + qwen2.5-coder:14b agent on CPU/RX 580. No Kimi K2.6.',
        ollamaMode: 'local',
        localHost: 'http://127.0.0.1:11434',
        planner: '',
        executor: 'ollama|qwen2.5-coder:14b',
        chatFast: 'ollama|airi-fast:latest',
        enableHybrid: false,
    },
    {
        id: 'composer2-gpu-server',
        label: 'Composer 2 — GPU server (192GB)',
        desc: 'All inference on remote Ollama: MiniMax M2.7 IQ3 planner + agent + chat.',
        ollamaMode: 'remote',
        remoteHost: '192.168.1.50',
        planner: `ollama|${COMPOSER2_DEFAULT_GPU_MODEL}`,
        executor: `ollama|${COMPOSER2_DEFAULT_GPU_MODEL}`,
        chatFast: `ollama|${COMPOSER2_DEFAULT_GPU_MODEL}`,
        enableHybrid: true,
    },
    {
        id: 'composer2-fast',
        label: 'Composer 2 Fast (single machine)',
        desc: 'Cursor default tier: airi-fast chat + optional Kimi hybrid when installed locally.',
        ollamaMode: 'local',
        planner: 'ollama|moonshotai/kimi-k2.5:cloud',
        executor: 'ollama|airi-fast:latest',
        chatFast: 'ollama|airi-fast:latest',
        enableHybrid: true,
    },
];

const CHAT_FAST_KEY = 'composer2.chatFastModel';
const STACK_KEY = 'composer2.activeStack';
const MODE_KEY = 'composer2.mode';
const LOCAL_URL_KEY = 'composer2.localOllamaUrl';
const REMOTE_URL_KEY = 'composer2.remoteOllamaUrl';
const FALLBACK_EXEC_KEY = 'composer2.executorLocalFallback';

function normalizeRemoteOllamaUrl(hostOrUrl: string): string {
    const raw = hostOrUrl.trim();
    if (!raw) return 'http://127.0.0.1:11434';
    if (raw.startsWith('http://') || raw.startsWith('https://')) return raw.replace(/\/$/, '');
    return `http://${raw.replace(/\/$/, '')}:11434`;
}

export function getComposer2ChatFastModel(): string | null {
    try {
        return localStorage.getItem(CHAT_FAST_KEY);
    } catch {
        return null;
    }
}

export function getActiveComposer2StackId(): string | null {
    try {
        return localStorage.getItem(STACK_KEY);
    } catch {
        return null;
    }
}

export function isComposer2HybridMode(): boolean {
    try {
        return localStorage.getItem(MODE_KEY) === 'hybrid';
    } catch {
        return false;
    }
}

/** Ollama URL for Composer 2 Fast chat (local box in hybrid mode). */
export function getComposer2LocalOllamaUrl(): string | null {
    try {
        return localStorage.getItem(LOCAL_URL_KEY);
    } catch {
        return null;
    }
}

export function getComposer2FastOllamaUrl(defaultUrl: string): string {
    if (isComposer2HybridMode()) {
        return getComposer2LocalOllamaUrl() || 'http://127.0.0.1:11434';
    }
    return defaultUrl;
}

export function getComposer2ExecutorLocalFallback(): string | null {
    try {
        return localStorage.getItem(FALLBACK_EXEC_KEY);
    } catch {
        return null;
    }
}

function parseModelSpec(spec: string): { provider: string; model: string } | null {
    const s = spec.trim();
    if (!s.includes('|')) return null;
    const [provider, ...rest] = s.split('|');
    const model = rest.join('|').trim();
    if (!provider.trim() || !model) return null;
    return { provider: provider.trim(), model };
}

async function resolveOllamaTagAtUrl(
    url: string,
    requested: string,
    restoreAfter?: string,
): Promise<string> {
    const base = url.replace(/\/$/, '');
    await invoke('set_ollama_url', { url: base }).catch(() => {});
    try {
        const { resolveOllamaModelTag } = await import('../airi/shared-ollama');
        return await resolveOllamaModelTag(requested);
    } catch {
        return requested;
    } finally {
        if (restoreAfter) {
            await invoke('set_ollama_url', { url: restoreAfter.replace(/\/$/, '') }).catch(() => {});
        }
    }
}

function getComposer2RemoteOllamaUrl(): string | null {
    try {
        return localStorage.getItem(REMOTE_URL_KEY);
    } catch {
        return null;
    }
}

/** Pick a chat model guaranteed safe for local Ollama (never MiniMax/Kimi-class). */
export async function ensureLocalChatModel(requested: string, localUrl?: string): Promise<string> {
    const url = (localUrl || getComposer2LocalOllamaUrl() || 'http://127.0.0.1:11434').replace(/\/$/, '');
    const restoreUrl = isComposer2HybridMode()
        ? (getComposer2RemoteOllamaUrl() || useStore.getState().customOllamaUrl || undefined)
        : undefined;
    if (!isGpuServerOnlyModel(requested)) {
        const hit = await resolveOllamaTagAtUrl(url, requested);
        if (!isGpuServerOnlyModel(hit)) {
            if (restoreUrl) await invoke('set_ollama_url', { url: restoreUrl.replace(/\/$/, '') }).catch(() => {});
            return hit;
        }
    }
    for (const fallback of COMPOSER2_LOCAL_CHAT_DEFAULTS) {
        const hit = await resolveOllamaTagAtUrl(url, fallback);
        if (!isGpuServerOnlyModel(hit)) {
            if (restoreUrl) await invoke('set_ollama_url', { url: restoreUrl.replace(/\/$/, '') }).catch(() => {});
            return hit;
        }
    }
    if (restoreUrl) await invoke('set_ollama_url', { url: restoreUrl.replace(/\/$/, '') }).catch(() => {});
    return COMPOSER2_LOCAL_CHAT_DEFAULTS[0];
}

async function verifyOllamaReachable(url: string): Promise<boolean> {
    try {
        await invoke('set_ollama_url', { url: url.replace(/\/$/, '') });
        const base = url.replace(/\/$/, '');
        const r = await fetch(`${base}/api/tags`, { signal: AbortSignal.timeout(4000) });
        return r.ok;
    } catch {
        return false;
    }
}

export async function applyComposer2Stack(
    stack: Composer2Stack,
    remoteHostOverride?: string,
): Promise<string[]> {
    const st = useStore.getState();
    const notes: string[] = [];

    const remoteHost = remoteHostOverride?.trim() || stack.remoteHost || '';
    const localUrl = (stack.localHost || 'http://127.0.0.1:11434').replace(/\/$/, '');
    const remoteUrl = remoteHost ? normalizeRemoteOllamaUrl(remoteHost) : '';

    try {
        localStorage.setItem(STACK_KEY, stack.id);
        localStorage.setItem(MODE_KEY, stack.ollamaMode);
        if (stack.ollamaMode === 'hybrid') {
            localStorage.setItem(LOCAL_URL_KEY, localUrl);
            if (remoteUrl) localStorage.setItem(REMOTE_URL_KEY, remoteUrl);
        } else {
            localStorage.removeItem(LOCAL_URL_KEY);
            localStorage.removeItem(REMOTE_URL_KEY);
        }
        if (stack.executorLocalFallback) {
            localStorage.setItem(FALLBACK_EXEC_KEY, stack.executorLocalFallback);
        }
    } catch { /* ignore */ }

    if (stack.ollamaMode === 'remote' || stack.ollamaMode === 'hybrid') {
        if (!remoteUrl) throw new Error('Enter your GPU server IP or URL first');
        st.setCustomOllamaUrl?.(remoteUrl);
        await st.setOllamaServerMode?.('remote');
        notes.push(`Agent + planner → remote Ollama at ${remoteUrl}`);
    } else {
        await st.setOllamaServerMode?.('local');
        notes.push(`All inference → local Ollama at ${localUrl}`);
    }

    await st.syncOllamaEndpoint?.();
    st.setInferenceBackend?.('ollama');

    st.setHybridAuto?.(stack.enableHybrid);
    st.setPlannerEnabled?.(stack.enableHybrid && !!stack.planner);
    st.setPlannerModel?.(stack.planner || '');

    const exec = parseModelSpec(stack.executor);
    if (exec) {
        const execUrl = stack.ollamaMode === 'hybrid' ? remoteUrl : (stack.ollamaMode === 'remote' ? remoteUrl : localUrl);
        let tag: string;
        let usedFallback = false;
        const remoteOk = stack.ollamaMode === 'local' || !execUrl || await verifyOllamaReachable(execUrl);
        if (remoteOk) {
            tag = await resolveOllamaTagAtUrl(execUrl, exec.model);
        } else if (stack.executorLocalFallback) {
            const fb = parseModelSpec(stack.executorLocalFallback);
            const fbModel = fb?.model || COMPOSER2_LOCAL_CHAT_DEFAULTS[0];
            tag = await ensureLocalChatModel(fbModel, localUrl);
            usedFallback = true;
            notes.push(`⚠ Remote GPU unreachable — agent executor fell back to local ${tag}`);
        } else {
            tag = await ensureLocalChatModel(COMPOSER2_LOCAL_CHAT_DEFAULTS[0], localUrl);
            usedFallback = true;
            notes.push(`⚠ Remote GPU unreachable — using local ${tag}`);
        }
        st.setAgentModel?.(`Ollama|${tag}`);
        notes.push(`Agent executor → ${tag}${stack.ollamaMode === 'hybrid' && !usedFallback ? ' (remote GPU)' : ''}`);
    }

    const chat = parseModelSpec(stack.chatFast);
    if (chat) {
        const chatUrl = stack.ollamaMode === 'hybrid' ? localUrl : (stack.ollamaMode === 'remote' ? remoteUrl : localUrl);
        const tag = stack.ollamaMode === 'hybrid'
            ? await ensureLocalChatModel(chat.model, localUrl)
            : await resolveOllamaTagAtUrl(chatUrl, chat.model);
        try {
            localStorage.setItem(CHAT_FAST_KEY, tag);
        } catch { /* ignore */ }
        notes.push(
            `Composer 2 Fast chat → ${tag}${stack.ollamaMode === 'hybrid' ? ' (local desk only)' : ''}`,
        );
    }

    if (stack.ollamaMode === 'hybrid') {
        const localOk = await verifyOllamaReachable(localUrl);
        await invoke('set_ollama_url', { url: remoteUrl }).catch(() => {});
        if (!localOk) {
            notes.push('⚠ Local Ollama not reachable — run `ollama serve` on this PC and pull qwen2.5-coder:7b');
        } else {
            notes.push(`Local desk pulls only: ${COMPOSER2_LOCAL_CHAT_DEFAULTS.slice(0, 2).join(', ')}`);
            notes.push(`Do NOT pull ${COMPOSER2_DEFAULT_GPU_MODEL} on this PC — GPU server only`);
        }
        const remotePull =
            parseModelSpec(stack.executor)?.model ||
            parseModelSpec(stack.planner)?.model ||
            COMPOSER2_DEFAULT_GPU_MODEL;
        notes.push(`On GPU server (${remoteUrl}) run: ollama pull ${remotePull}`);
        if (remotePull.includes('kimi-k2.6')) {
            notes.push('Avoid: ollama pull hf.co/unsloth/Kimi-K2.6-GGUF (sharded — use merged GGUF + llama-server instead)');
        }
    }

    await st.refreshAvailableModels?.('ollama');
    return notes;
}

/** Prefer Composer 2 Fast slot when a stack is active (local-safe in hybrid mode). */
export async function pickComposer2FastChatModel(preferred: string): Promise<string | null> {
    const slot = getComposer2ChatFastModel();
    if (!slot) return null;
    if (isComposer2HybridMode()) {
        return ensureLocalChatModel(slot);
    }
    if (isGpuServerOnlyModel(slot)) return null;
    return resolveOllamaTagAtUrl(
        getComposer2LocalOllamaUrl() || 'http://127.0.0.1:11434',
        slot,
    );
}
