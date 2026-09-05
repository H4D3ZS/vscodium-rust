import type { StateCreator } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import type { AppState } from './index';
import { normalizeOllamaUrl } from './utils';
import { applyLocalOllamaAgentDefaults } from '../lib/localOllamaAgentDefaults';
import { boundedPush, MAX_MITM_LOGS } from '../domain/utils/boundedArray';

// Throttle guard for checkLemonadeStatus — several components fire it on mount.
let lastLemonadeCheckAt = 0;

export interface InferenceSlice {
    ollamaUrl: string;
    ollamaStatus: 'idle' | 'checking' | 'running' | 'error';
    ollamaConnectionMode: 'proxy' | 'direct';
    ollamaMode: 'local' | 'cloud' | 'auto';
    ollamaServerMode: 'local' | 'cloud' | 'remote';
    customOllamaUrl: string;
    llamaCppUrl: string;
    llamaCppStatus: 'idle' | 'checking' | 'running' | 'error';
    llamaCppModelPath: string;
    llamaCppNgl: number;
    llamaCppHadesEnabled: boolean;
    inferenceBackend: 'llama-cpp' | 'openai' | 'lemonade' | 'huggingface' | 'fcc';
    availableModels: any[];
    isPullingModel: boolean;
    pullProgress: number;
    webuiSessions: { session_id: string; provider: string; display_name: string; has_token: boolean; is_active: boolean }[];
    activeWebuiSessionId: string | null;
    mitmStatus: 'idle' | 'running' | 'error';
    mitmLogs: string[];
    mcpServers: any[];
    aiStatus: 'alive' | 'dead';
    tokenUsage: number;
    vllmUrl: string;
    lmStudioUrl: string;
    lemonadeUrl: string;
    /**
     * Route chat-panel prompts through the Claude Code CLI (running against
     * local Lemonade) instead of the IDE's own `autonomous_loop`. Same model,
     * different harness — Claude Code brings its own tools, hooks and skills.
     */
    useClaudeCodeAgent: boolean;
    /** Claude Code session id, so follow-up turns continue the same thread. */
    claudeCodeSessionId: string | null;
    lemonadeStatus: 'idle' | 'checking' | 'running' | 'error';
    lemonadeLatencyMs: number | null;
    liteLLMUrl: string;
    liteLLMApiKey: string;
    googleVertexProject: string;
    googleVertexRegion: string;
    azureProject: string;
    azureApiKey: string;
    azureApiVersion: string;
    awsBedrockApiKey: string;
    awsBedrockRegion: string;
    awsBedrockEndpoint: string;
    fccUrl: string;
    fccStatus: 'idle' | 'checking' | 'running' | 'error';
    fccEnabled: boolean;

    setOllamaUrl: (url: string) => void;
    setOllamaConnectionMode: (mode: 'proxy' | 'direct') => void;
    setOllamaServerMode: (mode: 'local' | 'cloud' | 'remote') => void;
    setCustomOllamaUrl: (url: string) => void;
    syncOllamaEndpoint: () => Promise<void>;
    checkOllamaStatus: () => Promise<void>;
    pullOllamaModel: (name: string) => Promise<void>;
    setInferenceBackend: (backend: 'llama-cpp' | 'openai' | 'lemonade' | 'huggingface' | 'fcc') => void;
    setLlamaCppUrl: (url: string) => void;
    setLlamaCppModelPath: (path: string) => void;
    setLlamaCppNgl: (ngl: number) => void;
    setLlamaCppHadesEnabled: (v: boolean) => void;
    checkLlamaCppStatus: () => Promise<void>;
    refreshAvailableModels: (provider?: string) => Promise<void>;
    refreshWebuiSessions: (provider?: string) => Promise<void>;
    switchWebuiSession: (sessionId: string) => Promise<void>;
    deleteWebuiSession: (sessionId: string) => Promise<void>;
    startMitm: () => Promise<void>;
    stopMitm: () => Promise<void>;
    addMitmLog: (log: string) => void;
    addMcpServer: (name: string, config: any) => Promise<void>;
    removeMcpServer: (name: string) => Promise<void>;
    listMcpServers: () => Promise<void>;
    setMcpServerEnabled: (name: string, enabled: boolean) => Promise<void>;
    setAiStatus: (status: 'alive' | 'dead') => void;
    setTokenUsage: (usage: number) => void;
    setVllmUrl: (url: string) => void;
    setLmStudioUrl: (url: string) => void;
    setLemonadeUrl: (url: string) => void;
    setUseClaudeCodeAgent: (on: boolean) => void;
    setClaudeCodeSessionId: (id: string | null) => void;
    checkLemonadeStatus: () => Promise<void>;
    setFccUrl: (url: string) => void;
    checkFccStatus: () => Promise<void>;
    setFccEnabled: (v: boolean) => void;
    setLiteLLMUrl: (url: string) => void;
    setLiteLLMApiKey: (k: string) => void;
    setGoogleVertexProject: (v: string) => void;
    setGoogleVertexRegion: (v: string) => void;
    setAzureProject: (v: string) => void;
    setAzureApiKey: (k: string) => void;
    setAzureApiVersion: (v: string) => void;
    setAwsBedrockApiKey: (k: string) => void;
    setAwsBedrockRegion: (v: string) => void;
    setAwsBedrockEndpoint: (v: string) => void;
}

/** Managed Cyber-Ifrit cloud Ollama (AMD MI300X gateway). */
export const CYBERIFRIT_CLOUD_OLLAMA_URL = 'https://ai.cyberifrit.xyz';
const LOCAL_OLLAMA_URL = 'http://127.0.0.1:13305';

function readStoredOllamaUrl(): string {
    try {
        if (typeof localStorage === 'undefined') return LOCAL_OLLAMA_URL;
        const raw = (localStorage.getItem('ollamaUrl') || '').trim();
        if (!raw) return LOCAL_OLLAMA_URL;
        return normalizeOllamaUrl(raw);
    } catch { return LOCAL_OLLAMA_URL; }
}

function readStoredOllamaServerMode(): 'local' | 'cloud' | 'remote' {
    try {
        const raw = localStorage.getItem('ollamaServerMode');
        if (raw === 'auto') return 'cloud';
        if (raw === 'local' || raw === 'cloud' || raw === 'remote') return raw;
    } catch { /* ignore */ }
    return 'local';
}

function resolveOllamaUrlForMode(mode: 'local' | 'cloud' | 'remote', customUrl: string): string {
    if (mode === 'local') return LOCAL_OLLAMA_URL;
    if (mode === 'cloud') return CYBERIFRIT_CLOUD_OLLAMA_URL;
    const trimmed = customUrl.trim();
    return trimmed ? normalizeOllamaUrl(trimmed) : LOCAL_OLLAMA_URL;
}

function readInitialCustomOllamaUrl(): string {
    try { return localStorage.getItem('customOllamaUrl') || ''; } catch { return ''; }
}

const _initialOllamaMode = readStoredOllamaServerMode();
const _initialCustomOllama = readInitialCustomOllamaUrl();

export const createInferenceSlice: StateCreator<AppState, [], [], InferenceSlice> = (set, get) => ({
    ollamaUrl: resolveOllamaUrlForMode(_initialOllamaMode, _initialCustomOllama),
    ollamaStatus: 'idle',
    ollamaConnectionMode: (localStorage.getItem('ollamaConnectionMode') as 'proxy' | 'direct') || 'proxy',
    ollamaMode: _initialOllamaMode === 'local' ? 'local' : 'cloud',
    ollamaServerMode: _initialOllamaMode,
    customOllamaUrl: _initialCustomOllama,
    llamaCppUrl: localStorage.getItem('llamaCppUrl') || 'http://localhost:8081',
    llamaCppStatus: 'idle',
    llamaCppModelPath: localStorage.getItem('llamaCppModelPath') || '',
    llamaCppNgl: parseInt(localStorage.getItem('llamaCppNgl') || '99'),
    llamaCppHadesEnabled: localStorage.getItem('llamaCppHadesEnabled') !== 'false',
    // Lemonade is the priority local backend (faster than Ollama on most setups).
    // Only applied when the user hasn't already chosen one.
    inferenceBackend: (localStorage.getItem('inferenceBackend') as 'llama-cpp' | 'openai' | 'lemonade' | 'huggingface' | 'fcc') || 'lemonade',
    availableModels: [],
    isPullingModel: false,
    pullProgress: 0,
    webuiSessions: [],
    activeWebuiSessionId: null,
    mitmStatus: 'idle',
    mitmLogs: [],
    mcpServers: [],
    aiStatus: 'alive',
    tokenUsage: 0,
    vllmUrl: localStorage.getItem('provider.vllm.url') || 'http://localhost:8000',
    lmStudioUrl: localStorage.getItem('provider.lmstudio.url') || 'http://localhost:1234',
    lemonadeUrl: localStorage.getItem('provider.lemonade.url') || 'http://localhost:13305',
    // Default ON: a chat prompt harnesses Claude Code's agent loop against the
    // local model. Opt out explicitly to fall back to the built-in loop.
    useClaudeCodeAgent: localStorage.getItem('useClaudeCodeAgent') !== '0',
    claudeCodeSessionId: null,
    lemonadeStatus: 'idle',
    lemonadeLatencyMs: null,
    liteLLMUrl: localStorage.getItem('provider.litellm.url') || '',
    liteLLMApiKey: localStorage.getItem('provider.litellm.apikey') || '',
    googleVertexProject: localStorage.getItem('provider.vertex.project') || '',
    googleVertexRegion: localStorage.getItem('provider.vertex.region') || 'us-west2',
    azureProject: localStorage.getItem('provider.azure.project') || '',
    azureApiKey: localStorage.getItem('provider.azure.apikey') || '',
    azureApiVersion: localStorage.getItem('provider.azure.apiversion') || '2024-05-01-preview',
    awsBedrockApiKey: localStorage.getItem('provider.bedrock.apikey') || '',
    awsBedrockRegion: localStorage.getItem('provider.bedrock.region') || 'us-east-1',
    awsBedrockEndpoint: localStorage.getItem('provider.bedrock.endpoint') || '',
    fccUrl: localStorage.getItem('provider.fcc.url') || 'http://127.0.0.1:8082',
    fccStatus: 'idle',
    fccEnabled: localStorage.getItem('fcc.enabled') === 'true',

    setAiStatus: (aiStatus) => set({ aiStatus }),
    setTokenUsage: (tokenUsage) => set({ tokenUsage }),

    setOllamaUrl: (url) => {
        const normalized = normalizeOllamaUrl(url);
        set({ ollamaUrl: normalized });
        try { localStorage.setItem('ollamaUrl', normalized); } catch { }
        invoke('set_lemonade_url', { url: normalized }).catch(console.error);
    },
    setOllamaConnectionMode: (mode) => {
        const url = mode === 'proxy' ? 'http://127.0.0.1:1536' : 'http://127.0.0.1:13305';
        set({ ollamaConnectionMode: mode, ollamaUrl: url });
        (async () => {
            try { await invoke('set_lemonade_url', { url }); } catch { }
            try { await get().refreshAvailableModels?.('lemonade'); } catch { }
            try { await get().checkOllamaStatus?.(); } catch { }
        })();
        try { localStorage.setItem('ollamaConnectionMode', mode); localStorage.setItem('ollamaUrl', url); } catch { }
    },
    setOllamaServerMode: (mode) => {
        const custom = get().customOllamaUrl || '';
        const url = resolveOllamaUrlForMode(mode, custom);
        if (get().ollamaServerMode === mode && get().ollamaUrl === url) return;
        set({ ollamaServerMode: mode, ollamaUrl: url, ollamaMode: mode === 'local' ? 'local' : 'cloud' });
        try {
            localStorage.setItem('ollamaServerMode', mode);
            localStorage.setItem('ollamaUrl', url);
            if (mode === 'cloud') localStorage.setItem('customOllamaUrl', CYBERIFRIT_CLOUD_OLLAMA_URL);
        } catch { /* ignore */ }
        if (mode === 'local') {
            applyLocalOllamaAgentDefaults(get() as Parameters<typeof applyLocalOllamaAgentDefaults>[0]);
        }
        void get().syncOllamaEndpoint?.();
    },
    syncOllamaEndpoint: async () => {
        const mode = get().ollamaServerMode;
        const url = resolveOllamaUrlForMode(mode, get().customOllamaUrl || '');
        set({ ollamaUrl: url, ollamaMode: mode === 'local' ? 'local' : 'cloud' });
        try {
            localStorage.setItem('ollamaUrl', url);
            localStorage.setItem('ollamaServerMode', mode);
        } catch { /* ignore */ }
        try { await invoke('set_lemonade_url', { url }); } catch (e) { console.warn('[Ollama] set_lemonade_url failed:', e); }
        try { await get().refreshAvailableModels?.('lemonade'); } catch { /* ignore */ }
        try { await get().checkOllamaStatus?.(); } catch { /* ignore */ }
    },
    setCustomOllamaUrl: (url) => {
        const trimmed = url.trim();
        set({ customOllamaUrl: trimmed });
        try { localStorage.setItem('customOllamaUrl', trimmed); } catch { }
        if (get().ollamaServerMode === 'remote') get().setOllamaServerMode?.('remote');
    },
    checkOllamaStatus: async () => {
        set({ ollamaStatus: 'checking' });
        try { const isRunning = await invoke<boolean>('check_lemonade_status'); set({ ollamaStatus: isRunning ? 'running' : 'error' }); }
        catch { set({ ollamaStatus: 'error' }); }
    },
    pullOllamaModel: async (name) => {
        set({ isPullingModel: true, pullProgress: 0 });
        try { await invoke('pull_lemonade_model', { name }); get().refreshAvailableModels('lemonade'); }
        catch (e) { console.error('Failed to pull model:', e); }
        finally { set({ isPullingModel: false }); }
    },
    setInferenceBackend: (backend) => {
        localStorage.setItem('inferenceBackend', backend);
        set({ inferenceBackend: backend });
        const st = get();
        if (backend === 'lemonade') {
            applyLocalOllamaAgentDefaults(st as Parameters<typeof applyLocalOllamaAgentDefaults>[0]);
        }
        if (backend === 'llama-cpp') {
            import('../tauri_bridge').then(({ invoke }) => invoke('set_lemonade_url', { url: st.llamaCppUrl }).catch(() => { }));
        } else if (backend === 'lemonade') {
            // Propagate the configured Lemonade server URL into the Rust process
            // so get_endpoint("lemonade") / list_models("lemonade") hit the user's
            // actual port instead of the hardcoded default (:13305).
            import('../tauri_bridge').then(({ invoke }) => invoke('set_lemonade_url', { url: st.lemonadeUrl }).catch(() => { }));
        } else if (backend === 'fcc') {
            // FCC backend — no URL propagation needed, FCC handles provider routing
            import('../tauri_bridge').then(({ invoke }) => invoke('check_fcc_status').catch(() => { }));
        }
    },
    setLlamaCppUrl: (url) => {
        localStorage.setItem('llamaCppUrl', url);
        set({ llamaCppUrl: url });
        if (get().inferenceBackend === 'llama-cpp') {
            import('../tauri_bridge').then(({ invoke }) => invoke('set_lemonade_url', { url }).catch(() => { }));
        }
    },
    setLlamaCppModelPath: (path) => { localStorage.setItem('llamaCppModelPath', path); set({ llamaCppModelPath: path }); },
    setLlamaCppNgl: (ngl) => { localStorage.setItem('llamaCppNgl', ngl.toString()); set({ llamaCppNgl: ngl }); },
    setLlamaCppHadesEnabled: (enabled) => { localStorage.setItem('llamaCppHadesEnabled', enabled.toString()); set({ llamaCppHadesEnabled: enabled }); },
    checkLlamaCppStatus: async () => {
        set({ llamaCppStatus: 'checking' });
        try {
            const response = await fetch(`${get().llamaCppUrl}/health`, { method: 'GET', signal: AbortSignal.timeout(3000) });
            set({ llamaCppStatus: response.ok ? 'running' : 'error' });
        } catch { set({ llamaCppStatus: 'error' }); }
    },

    refreshAvailableModels: async (targetProvider?) => {
        const { lemonadeUrl } = get();
        try {
            const keys: any = await invoke('get_api_keys').catch(() => ({}));
            // Lemonade is the only local backend. 'Ollama' used to sit here and
            // cost a failed round-trip on every refresh.
            const providers: string[] = ['Lemonade'];
            // Cloud subscription models only listed when signed in (local stays free).
            const cloudUnlocked = !!(get() as any).isCloudUnlocked?.();
            if (cloudUnlocked) providers.push('Cyberifrit');
            if (keys.google) providers.push('Google');
            if (keys.anthropic) providers.push('Anthropic');
            if (keys.openai) providers.push('OpenAI');
            if (keys.openrouter) providers.push('Openrouter');
            if (keys.mistral) providers.push('Mistral');
            if ((keys as any).deepseek) providers.push('Deepseek');
            if ((keys as any).mimo) providers.push('Mimo');
            if ((keys as any).highwayapi || (keys as any).highwayapi_base_url) providers.push('Highwayapi');
            if (typeof navigator !== 'undefined' && /Mac/i.test(navigator.platform || navigator.userAgent || '')) providers.push('Deepseek-ANE');
            if (keys.groq) providers.push('Groq');
            if (keys.xai) providers.push('xAI');
            if (keys.cerebras) providers.push('Cerebras');
            if (keys.alibaba) providers.push('Alibaba');
            if ((keys as any).nvidia) providers.push('Nvidia');
            if ((keys as any).modelscope) providers.push('Modelscope');
            if ((keys as any).openmodel) providers.push('Openmodel');
            // Always list OpenModel if it has an API key OR if it's enabled in settings
            if (!providers.includes('Openmodel') && (keys as any).openmodel) {
                providers.push('Openmodel');
            }
            // Always list Highwayapi if enabled (Jiekou Claude Opus 4.8)
            if (!providers.includes('Highwayapi') && ((keys as any).highwayapi || (keys as any).highwayapi_base_url)) {
                providers.push('Highwayapi');
            }
            // Per-provider enable toggle (Settings → Providers & Keys): a provider
            // explicitly disabled is hidden from the picker, so your own models
            // aren't buried among BYOB providers. Unset = enabled (default).
            const isEnabled = (p: string) => {
                try { return localStorage.getItem('provider.enabled.' + p.toLowerCase()) !== 'false'; } catch { return true; }
            };
            let allModels: { id: string; provider: string }[] = [];
            const activeProviders = (targetProvider
                ? [targetProvider.charAt(0).toUpperCase() + targetProvider.slice(1).toLowerCase()]
                : providers
            ).filter(isEnabled);

            for (const p of activeProviders) {
                try {
                    // Skip Ollama if it's not running (avoids repeated failed requests)
                    if (p.toLowerCase() === 'ollama') {
                        const ollamaStatus = get().ollamaStatus;
                        if (ollamaStatus === 'error' || ollamaStatus === 'idle') {
                            // Check if Ollama is actually reachable first
                            try {
                                const isRunning = await invoke<boolean>('check_lemonade_status');
                                if (!isRunning) {
                                    set({ ollamaStatus: 'error' });
                                    continue;
                                }
                                set({ ollamaStatus: 'running' });
                            } catch {
                                set({ ollamaStatus: 'error' });
                                continue;
                            }
                        }
                    }
                    if (p.toLowerCase() === 'ollama') {
                        const raw = get().ollamaUrl || 'http://localhost:13305';
                        const ollamaToUse = normalizeOllamaUrl(raw);
                        await invoke('set_lemonade_url', { url: ollamaToUse });
                        set({ ollamaUrl: ollamaToUse });
                        try { localStorage.setItem('ollamaUrl', ollamaToUse); } catch { }
                        const isLocal = /localhost|127\.|0\.0\.0\.0/.test(ollamaToUse);
                        set({ ollamaConnectionMode: 'direct', ollamaMode: isLocal ? 'local' : 'cloud' });
                    } else if (p.toLowerCase() === 'lemonade') {
                        // Lemonade uses its own URL — push it into the backend BEFORE
                        // list_provider_models runs, otherwise the engine resolves
                        // list_models("lemonade") against the stored/default base
                        // (http://localhost:13305) instead of the user's actual server,
                        // and the model list silently comes back empty / errors.
                        const lemonadeUrlToUse = get().lemonadeUrl || 'http://localhost:13305';
                        try { localStorage.setItem('provider.lemonade.url', lemonadeUrlToUse); } catch { }
                        await invoke('set_lemonade_url', { url: lemonadeUrlToUse }).catch(() => { });
                    }
                    // Show only ACTUALLY-INSTALLED Ollama models in the picker —
                    // do not flood it with registry pull-hints the user hasn't
                    // downloaded (that belongs in the pull/install wizard).
                    let models: string[] = await invoke<string[]>('list_provider_models', { provider: p });
                    allModels = [...allModels, ...models.map(m => ({ id: m, provider: p.toLowerCase() }))];
                    // Hardcoded fallback: if API returned empty, add known models
                    if (models.length === 0) {
                        const fallbackModels: Record<string, string[]> = {
                            'openmodel': ['deepseek-v4-flash', 'deepseek-chat', 'deepseek-v4', 'gpt-4o', 'claude-sonnet-4-20250514', 'qwen3-max', 'mimo-v2.5-pro'],
                            'highwayapi': ['claude-opus-4-20250514', 'claude-sonnet-4-20250514'],
                            'modelscope': ['Qwen-Ambassador/Qwen3.8-Max'],
                        };
                        const fallback = fallbackModels[p.toLowerCase()];
                        if (fallback) {
                            allModels = [...allModels, ...fallback.map(m => ({ id: m, provider: p.toLowerCase() }))];
                        }
                    }
                    if (p.toLowerCase() === 'ollama' && models.length > 0) set({ ollamaStatus: 'running' });
                    if (p.toLowerCase() === 'lemonade' && models.length > 0) set({ lemonadeStatus: 'running' });
                } catch (e: any) {
                    const msg = typeof e === 'string' ? e : String(e ?? '');
                    const quiet =
                        msg.includes('API key not found') ||
                        msg.includes('Connection refused') ||
                        msg.includes('not reachable') ||
                        msg.includes('error trying to connect');
                    if (!quiet) console.error(`Failed to fetch models for ${p}:`, e);
                    if (p.toLowerCase() === 'ollama') set({ ollamaStatus: 'error' });
                    if (p.toLowerCase() === 'lemonade') set({ lemonadeStatus: 'error' });
                }
            }
            // Guarantee Opus 4.8 appears when the Interface AI key is set + enabled,
            // even if the provider's /models listing is unavailable.
            if (((keys as any).highwayapi) && isEnabled('highwayapi') && !allModels.some(m => m.provider === 'highwayapi')) {
                allModels.push({ id: 'claude-opus-4-8', provider: 'highwayapi' });
            }
            // Guarantee the curated cloud models appear once signed in, even if the
            // cyberifrit gateway's /models listing is empty/unavailable.
            if (cloudUnlocked && !allModels.some(m => m.provider === 'cyberifrit')) {
                allModels.push({ id: 'glm-5.2', provider: 'cyberifrit' });
                allModels.push({ id: 'qwen3.6-35b-moe', provider: 'cyberifrit' });
            }
            set((state) => {
                let currentModels = targetProvider ? state.availableModels.filter((m: any) => m.provider !== targetProvider.toLowerCase()) : [];
                const newModels = allModels.filter(nm => !currentModels.some((cm: any) => cm.id === nm.id && cm.provider === nm.provider));
                return { availableModels: [...currentModels, ...newModels] };
            });
        } catch (e) { console.error('Refresh Available Models Error:', e); }
    },

    refreshWebuiSessions: async (provider?) => {
        try {
            const sessions: any = await invoke('list_webui_sessions', { provider: provider || null });
            set({ webuiSessions: sessions || [] });
            const active = sessions?.find((s: any) => s.is_active);
            if (active) { set({ activeWebuiSessionId: active.session_id }); try { localStorage.setItem(`hades.webui.account.${active.provider}`, active.display_name || 'default'); } catch { } }
            else if (sessions?.length > 0) { set({ activeWebuiSessionId: sessions[0].session_id }); }
            else set({ activeWebuiSessionId: null });
        } catch { }
    },
    switchWebuiSession: async (sessionId) => {
        try {
            await invoke('switch_webui_session', { sessionId });
            set({ activeWebuiSessionId: sessionId });
            await get().refreshAvailableModels();
        } catch { }
    },
    deleteWebuiSession: async (sessionId) => {
        try { await invoke('delete_webui_session', { sessionId }); await get().refreshWebuiSessions(); } catch { }
    },

    startMitm: async () => {
        try { set({ mitmStatus: 'running' }); await invoke('start_mitm_server'); get().addMitmLog('Proxy server started on port 8080'); }
        catch (e: any) { set({ mitmStatus: 'error' }); get().addMitmLog(`Error: ${e}`); }
    },
    stopMitm: async () => {
        try { await invoke('stop_mitm_server'); set({ mitmStatus: 'idle' }); get().addMitmLog('Proxy server stopped'); }
        catch (e: any) { get().addMitmLog(`Error stopping server: ${e}`); }
    },
    addMitmLog: (log) => set((s) => ({ mitmLogs: boundedPush(s.mitmLogs, `[${new Date().toLocaleTimeString()}] ${log}`, MAX_MITM_LOGS) })),
    addMcpServer: async (name, config) => {
        await invoke('add_mcp_server', { name, config });
        await get().listMcpServers();
    },
    removeMcpServer: async (name) => { try { await invoke('remove_mcp_server', { name }); await get().listMcpServers(); } catch { } },
    listMcpServers: async () => { try { const servers = await invoke<any[]>('list_mcp_servers'); set({ mcpServers: servers }); } catch { } },
    setMcpServerEnabled: async (name, enabled) => { try { await invoke('set_mcp_server_enabled', { name, enabled }); await get().listMcpServers(); } catch { } },

    setVllmUrl: (url) => { try { localStorage.setItem('provider.vllm.url', url); } catch { } set({ vllmUrl: url }); },
    setLmStudioUrl: (url) => { try { localStorage.setItem('provider.lmstudio.url', url); } catch { } set({ lmStudioUrl: url }); },
    setUseClaudeCodeAgent: (on) => {
        localStorage.setItem('useClaudeCodeAgent', on ? '1' : '0');
        // Switching harness starts a new thread — the old session id belongs to
        // whichever agent produced it and cannot be resumed by the other.
        set({ useClaudeCodeAgent: on, claudeCodeSessionId: null });
    },

    setClaudeCodeSessionId: (id) => set({ claudeCodeSessionId: id }),

    setLemonadeUrl: (url) => {
        try { localStorage.setItem('provider.lemonade.url', url); } catch { }
        set({ lemonadeUrl: url });
        if (get().inferenceBackend === 'lemonade') {
            import('../tauri_bridge').then(({ invoke }) => invoke('set_lemonade_url', { url }).catch(() => { }));
        }
    },
    checkLemonadeStatus: async () => {
        // Single source of truth for Lemonade health. Mount-effect storms
        // (settings panels, dashboard, status bar) are deduped by a 5s guard.
        const now = Date.now();
        if (now - lastLemonadeCheckAt < 5_000 && get().lemonadeStatus !== 'idle') return;
        lastLemonadeCheckAt = now;
        const url = get().lemonadeUrl || 'http://localhost:13305';
        try {
            set({ lemonadeStatus: 'checking' });
            // Go through Rust: attaches the signed-in cloud JWT, normalizes the
            // base (avoids /v1/v1), and bypasses browser CORS for gated cloud
            // Lemonade. A raw fetch here 401s on the JWT-gated proxy.
            const { invoke } = await import('../tauri_bridge');
            await invoke('set_lemonade_url', { url }).catch(() => { });
            const t0 = performance.now();
            const up = await invoke<boolean>('check_lemonade_status');
            set({
                lemonadeStatus: up ? 'running' : 'error',
                lemonadeLatencyMs: up ? Math.round(performance.now() - t0) : null,
            });
        } catch {
            set({ lemonadeStatus: 'error', lemonadeLatencyMs: null });
        }
    },
    setFccUrl: (url) => { try { localStorage.setItem('provider.fcc.url', url); } catch { } set({ fccUrl: url }); },
    setFccEnabled: (v) => { try { localStorage.setItem('fcc.enabled', String(v)); } catch { } set({ fccEnabled: v }); },
    checkFccStatus: async () => {
        const url = get().fccUrl || 'http://127.0.0.1:8082';
        try {
            set({ fccStatus: 'checking' });
            const { invoke } = await import('../tauri_bridge');
            const healthy = await invoke<boolean>('fcc_health');
            set({ fccStatus: healthy ? 'running' : 'error' });
        } catch {
            set({ fccStatus: 'error' });
        }
    },
    setLiteLLMUrl: (url) => { try { localStorage.setItem('provider.litellm.url', url); } catch { } set({ liteLLMUrl: url }); },
    setLiteLLMApiKey: (k) => { try { localStorage.setItem('provider.litellm.apikey', k); } catch { } set({ liteLLMApiKey: k }); },
    setGoogleVertexProject: (v) => { try { localStorage.setItem('provider.vertex.project', v); } catch { } set({ googleVertexProject: v }); },
    setGoogleVertexRegion: (v) => { try { localStorage.setItem('provider.vertex.region', v); } catch { } set({ googleVertexRegion: v }); },
    setAzureProject: (v) => { try { localStorage.setItem('provider.azure.project', v); } catch { } set({ azureProject: v }); },
    setAzureApiKey: (k) => { try { localStorage.setItem('provider.azure.apikey', k); } catch { } set({ azureApiKey: k }); },
    setAzureApiVersion: (v) => { try { localStorage.setItem('provider.azure.apiversion', v); } catch { } set({ azureApiVersion: v }); },
    setAwsBedrockApiKey: (k) => { try { localStorage.setItem('provider.bedrock.apikey', k); } catch { } set({ awsBedrockApiKey: k }); },
    setAwsBedrockRegion: (v) => { try { localStorage.setItem('provider.bedrock.region', v); } catch { } set({ awsBedrockRegion: v }); },
    setAwsBedrockEndpoint: (v) => { try { localStorage.setItem('provider.bedrock.endpoint', v); } catch { } set({ awsBedrockEndpoint: v }); },
});

