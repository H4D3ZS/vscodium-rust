/**
 * Composer 2 local parity (Cursor closed-source — Kimi K2.5/K2.6-class base + RL).
 * We ship routing presets, not Cursor weights.
 *
 * @see https://cursor.com/blog/composer-2
 */

import { invoke } from '@tauri-apps/api/core';
import { useStore } from '../store';

export const COMPOSER2_BLOG_URL = 'https://cursor.com/blog/composer-2';

/**Sharded HF GGUF cannot `the local backend pull hf.co/...` — use the local backend library tags or llama-server. */
export const KIMI_K26_LOCAL_TAGS = [
    'batiai/kimi-k2.6:iq3',
    'moonshotai/kimi-k2.5:cloud',
] as const;

/**Default GPU-server agent model for Composer 2 hybrid (3900X desk + remote the local backend). */
export const COMPOSER2_DEFAULT_GPU_MODEL = 'batiai/minimax-m2.7:iq3';

/**Models that must never run on the desk PC — remote GPU server only. */
export const COMPOSER2_GPU_SERVER_ONLY_PATTERNS = [
    /minimax-m2/i,
    /kimi-k2\.6/i,
    /kimi-k2\.5:cloud/i,
    /glm-5/i,
] as const;

/**Safe local chat tags for hybrid mode (3900X / RX 580). */
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

export function isLocalInferenceUrl(url: string): boolean {
    try {
        const host = new URL(url.replace(/\/$/, '') || 'http://127.0.0.1:13305').hostname.toLowerCase();
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

export type Composer2InferenceMode = 'local' | 'remote' | 'hybrid';

export interface Composer2Stack {
    id: string;
    label: string;
    desc: string;
    inferenceMode: Composer2InferenceMode;
    /**Remote GPU server (planner + agent executor) */
    remoteHost?: string;
    /**Local the local backend for Composer 2 Fast chat (Ryzen 3900X box) */
    localHost?: string;
    /**Deep planner on GPU server */
    planner: string;
    /**Agent tool-loop model (remote in hybrid mode) */
    executor: string;
    /**Local fast chat model (Composer 2 Fast tier) */
    chatFast: string;
    /**Fallback agent model when remote Kimi unavailable (local RX 580 / CPU) */
    executorLocalFallback?: string;
    enableHybrid: boolean;
}

export const COMPOSER2_STACKS: Composer2Stack[] = [
    {
        id: 'composer2-amd3900',
        label: 'Composer 2 — AMD 3900X + RX 580 ',
        desc:
            'Cursor agent parity for your desk: Ryzen 3900X runs fast chat (qwen2.5-coder:7b). ' +
            'MiniMax M2.7 IQ3 planner + agent executor on the remote GPU server. ' +
            'Pull on server: batiai/minimax-m2.7:iq3',
        inferenceMode: 'hybrid',
        remoteHost: '192.168.1.50',
        localHost: 'http://127.0.0.1:13305',
        planner: `lemonade|${COMPOSER2_DEFAULT_GPU_MODEL}`,
        executor: `lemonade|${COMPOSER2_DEFAULT_GPU_MODEL}`,
        chatFast: 'lemonade|qwen2.5-coder:7b',
        executorLocalFallback: 'lemonade|qwen2.5-coder:14b',
        enableHybrid: true,
    },
    {
        id: 'composer2-amd3900-kimi',
        label: 'Composer 2 — 3900X + Kimi K2.6 (heavier GPU)',
        desc:
            'Same hybrid desk setup, but GPU server runs Kimi K2.6 IQ3 instead of MiniMax — ' +
            'more VRAM, closer to Cursor Composer 2 base model. Pull: batiai/kimi-k2.6:iq3',
        inferenceMode: 'hybrid',
        remoteHost: '192.168.1.50',
        localHost: 'http://127.0.0.1:13305',
        planner: 'lemonade|batiai/kimi-k2.6:iq3',
        executor: 'lemonade|batiai/kimi-k2.6:iq3',
        chatFast: 'lemonade|qwen2.5-coder:7b',
        executorLocalFallback: 'lemonade|qwen2.5-coder:14b',
        enableHybrid: true,
    },
    {
        id: 'composer2-amd3900-local-only',
        label: 'Composer 2 Lite — 3900X local only',
        desc: 'No GPU server: fast chat + qwen2.5-coder:14b agent on CPU/RX 580. No Kimi K2.6.',
        inferenceMode: 'local',
        localHost: 'http://127.0.0.1:13305',
        planner: '',
        executor: 'lemonade|qwen2.5-coder:14b',
        chatFast: 'lemonade|airi-fast:latest',
        enableHybrid: false,
    },
    {
        id: 'composer2-gpu-server',
        label: 'Composer 2 — GPU server (192GB)',
        desc: 'All inference on a remote local server: MiniMax M2.7 IQ3 planner + agent + chat.',
        inferenceMode: 'remote',
        remoteHost: '192.168.1.50',
        planner: `lemonade|${COMPOSER2_DEFAULT_GPU_MODEL}`,
        executor: `lemonade|${COMPOSER2_DEFAULT_GPU_MODEL}`,
        chatFast: `lemonade|${COMPOSER2_DEFAULT_GPU_MODEL}`,
        enableHybrid: true,
    },
    {
        id: 'composer2-fast',
        label: 'Composer 2 Fast (single machine)',
        desc: 'Cursor default tier: airi-fast chat + optional Kimi hybrid when installed locally.',
        inferenceMode: 'local',
        planner: 'lemonade|moonshotai/kimi-k2.5:cloud',
        executor: 'lemonade|airi-fast:latest',
        chatFast: 'lemonade|airi-fast:latest',
        enableHybrid: true,
    },
];

const CHAT_FAST_KEY = 'composer2.chatFastModel';
const STACK_KEY = 'composer2.activeStack';
const MODE_KEY = 'composer2.mode';
const LOCAL_URL_KEY = 'composer2.localInferenceUrl';
const REMOTE_URL_KEY = 'composer2.remoteInferenceUrl';
const FALLBACK_EXEC_KEY = 'composer2.executorLocalFallback';

function normalizeRemoteInferenceUrl(hostOrUrl: string): string {
    const raw = hostOrUrl.trim();
    if (!raw) return 'http://127.0.0.1:13305';
    if (raw.startsWith('http://') || raw.startsWith('https://')) return raw.replace(/\/$/, '');
    return `http://${raw.replace(/\/$/, '')}:13305`;
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

/**the local backend URL for Composer 2 Fast chat (local box in hybrid mode). */
export function getComposer2LocalInferenceUrl(): string | null {
    try {
        return localStorage.getItem(LOCAL_URL_KEY);
    } catch {
        return null;
    }
}

export function getComposer2FastInferenceUrl(defaultUrl: string): string {
    if (isComposer2HybridMode()) {
        return getComposer2LocalInferenceUrl() || 'http://127.0.0.1:13305';
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

async function resolveModelTagAtUrl(
    url: string,
    requested: string,
    restoreAfter?: string,
): Promise<string> {
    const base = url.replace(/\/$/, '');
    await invoke('set_lemonade_url', { url: base }).catch(() => {});
    try {
        const { resolveLocalModelTag } = await import('./localModelClient');
        return await resolveLocalModelTag(requested);
    } catch {
        return requested;
    } finally {
        if (restoreAfter) {
            await invoke('set_lemonade_url', { url: restoreAfter.replace(/\/$/, '') }).catch(() => {});
        }
    }
}

function getComposer2RemoteInferenceUrl(): string | null {
    try {
        return localStorage.getItem(REMOTE_URL_KEY);
    } catch {
        return null;
    }
}

/**Pick a chat model guaranteed safe for local the local backend (never MiniMax/Kimi-class). */
export async function ensureLocalChatModel(requested: string, localUrl?: string): Promise<string> {
    const url = (localUrl || getComposer2LocalInferenceUrl() || 'http://127.0.0.1:13305').replace(/\/$/, '');
    const restoreUrl = isComposer2HybridMode()
? (getComposer2RemoteInferenceUrl() || useStore.getState().customInferenceUrl || undefined)
: undefined;
    if (!isGpuServerOnlyModel(requested)) {
        const hit = await resolveModelTagAtUrl(url, requested);
        if (!isGpuServerOnlyModel(hit)) {
            if (restoreUrl) await invoke('set_lemonade_url', { url: restoreUrl.replace(/\/$/, '') }).catch(() => {});
            return hit;
        }
    }
    for (const fallback of COMPOSER2_LOCAL_CHAT_DEFAULTS) {
        const hit = await resolveModelTagAtUrl(url, fallback);
        if (!isGpuServerOnlyModel(hit)) {
            if (restoreUrl) await invoke('set_lemonade_url', { url: restoreUrl.replace(/\/$/, '') }).catch(() => {});
            return hit;
        }
    }
    if (restoreUrl) await invoke('set_lemonade_url', { url: restoreUrl.replace(/\/$/, '') }).catch(() => {});
    return COMPOSER2_LOCAL_CHAT_DEFAULTS[0];
}

async function verifyInferenceReachable(url: string): Promise<boolean> {
    try {
        await invoke('set_lemonade_url', { url: url.replace(/\/$/, '') });
        const base = url.replace(/\/$/, '');
        const r = await fetch(`${base}/api/v1/models`, { signal: AbortSignal.timeout(4000) });
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
    const localUrl = (stack.localHost || 'http://127.0.0.1:13305').replace(/\/$/, '');
    const remoteUrl = remoteHost? normalizeRemoteInferenceUrl(remoteHost): '';

    try {
        localStorage.setItem(STACK_KEY, stack.id);
        localStorage.setItem(MODE_KEY, stack.inferenceMode);
        if (stack.inferenceMode === 'hybrid') {
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

    if (stack.inferenceMode === 'remote' || stack.inferenceMode === 'hybrid') {
        if (!remoteUrl) throw new Error('Enter your GPU server IP or URL first');
        st.setCustomInferenceUrl?.(remoteUrl);
        await st.setInferenceServerMode?.('remote');
        notes.push(`Agent + planner → remote local backend at ${remoteUrl}`);
    } else {
        await st.setInferenceServerMode?.('local');
        notes.push(`All inference → local Lemonade at ${localUrl}`);
    }

    await st.syncInferenceEndpoint?.();
    st.setInferenceBackend?.('lemonade');

    st.setHybridAuto?.(stack.enableHybrid);
    st.setPlannerEnabled?.(stack.enableHybrid && !!stack.planner);
    st.setPlannerModel?.(stack.planner || '');

    const exec = parseModelSpec(stack.executor);
    if (exec) {
        const execUrl = stack.inferenceMode === 'hybrid'? remoteUrl: (stack.inferenceMode === 'remote'? remoteUrl: localUrl);
        let tag: string;
        let usedFallback = false;
        const remoteOk = stack.inferenceMode === 'local' || !execUrl || await verifyInferenceReachable(execUrl);
        if (remoteOk) {
            tag = await resolveModelTagAtUrl(execUrl, exec.model);
        } else if (stack.executorLocalFallback) {
            const fb = parseModelSpec(stack.executorLocalFallback);
            const fbModel = fb?.model || COMPOSER2_LOCAL_CHAT_DEFAULTS[0];
            tag = await ensureLocalChatModel(fbModel, localUrl);
            usedFallback = true;
            notes.push(` Remote GPU unreachable — agent executor fell back to local ${tag}`);
        } else {
            tag = await ensureLocalChatModel(COMPOSER2_LOCAL_CHAT_DEFAULTS[0], localUrl);
            usedFallback = true;
            notes.push(` Remote GPU unreachable — using local ${tag}`);
        }
        st.setAgentModel?.(`lemonade|${tag}`);
        notes.push(`Agent executor → ${tag}${stack.inferenceMode === 'hybrid' && !usedFallback? ' (remote GPU)': ''}`);
    }

    const chat = parseModelSpec(stack.chatFast);
    if (chat) {
        const chatUrl = stack.inferenceMode === 'hybrid'? localUrl: (stack.inferenceMode === 'remote'? remoteUrl: localUrl);
        const tag = stack.inferenceMode === 'hybrid'
? await ensureLocalChatModel(chat.model, localUrl)
: await resolveModelTagAtUrl(chatUrl, chat.model);
        try {
            localStorage.setItem(CHAT_FAST_KEY, tag);
        } catch { /* ignore */ }
        notes.push(
            `Composer 2 Fast chat → ${tag}${stack.inferenceMode === 'hybrid'? ' (local desk only)': ''}`,
        );
    }

    if (stack.inferenceMode === 'hybrid') {
        const localOk = await verifyInferenceReachable(localUrl);
        await invoke('set_lemonade_url', { url: remoteUrl }).catch(() => {});
        if (!localOk) {
            notes.push(' Local backend not reachable — start it on this PC and load qwen2.5-coder:7b');
        } else {
            notes.push(`Local desk pulls only: ${COMPOSER2_LOCAL_CHAT_DEFAULTS.slice(0, 2).join(', ')}`);
            notes.push(`Do NOT pull ${COMPOSER2_DEFAULT_GPU_MODEL} on this PC — GPU server only`);
        }
        const remotePull =
            parseModelSpec(stack.executor)?.model ||
            parseModelSpec(stack.planner)?.model ||
            COMPOSER2_DEFAULT_GPU_MODEL;
        notes.push(`On the GPU server (${remoteUrl}) load: ${remotePull}`);
        if (remotePull.includes('kimi-k2.6')) {
            notes.push('Avoid the sharded hf.co/unsloth/Kimi-K2.6-GGUF — use a merged GGUF + llama-server instead');
        }
    }

    await st.refreshAvailableModels?.('lemonade');
    return notes;
}

/**Prefer Composer 2 Fast slot when a stack is active (local-safe in hybrid mode). */
export async function pickComposer2FastChatModel(preferred: string): Promise<string | null> {
    const slot = getComposer2ChatFastModel();
    if (!slot) return null;
    if (isComposer2HybridMode()) {
        return ensureLocalChatModel(slot);
    }
    if (isGpuServerOnlyModel(slot)) return null;
    return resolveModelTagAtUrl(
        getComposer2LocalInferenceUrl() || 'http://127.0.0.1:13305',
        slot,
    );
}
