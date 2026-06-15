import type { StateCreator } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import type { AppState } from './index';
import { normalizeOllamaUrl } from './utils';
import { applyLocalOllamaAgentDefaults } from '../lib/localOllamaAgentDefaults';

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
    inferenceBackend: 'ollama' | 'llama-cpp' | 'openai';
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

    setOllamaUrl: (url: string) => void;
    setOllamaConnectionMode: (mode: 'proxy' | 'direct') => void;
    setOllamaServerMode: (mode: 'local' | 'cloud' | 'remote') => void;
    setCustomOllamaUrl: (url: string) => void;
    syncOllamaEndpoint: () => Promise<void>;
    checkOllamaStatus: () => Promise<void>;
    pullOllamaModel: (name: string) => Promise<void>;
    setInferenceBackend: (backend: 'ollama' | 'llama-cpp' | 'openai') => void;
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
const LOCAL_OLLAMA_URL = 'http://127.0.0.1:11434';

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
    inferenceBackend: (localStorage.getItem('inferenceBackend') as 'ollama' | 'llama-cpp' | 'openai') || 'ollama',
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

    setAiStatus: (aiStatus) => set({ aiStatus }),
    setTokenUsage: (tokenUsage) => set({ tokenUsage }),

    setOllamaUrl: (url) => {
        const normalized = normalizeOllamaUrl(url);
        set({ ollamaUrl: normalized });
        try { localStorage.setItem('ollamaUrl', normalized); } catch { }
        invoke('set_ollama_url', { url: normalized }).catch(console.error);
    },
    setOllamaConnectionMode: (mode) => {
        const url = mode === 'proxy' ? 'http://127.0.0.1:1536' : 'http://127.0.0.1:11434';
        set({ ollamaConnectionMode: mode, ollamaUrl: url });
        (async () => {
            try { await invoke('set_ollama_url', { url }); } catch { }
            try { await get().refreshAvailableModels?.('ollama'); } catch { }
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
        try { await invoke('set_ollama_url', { url }); } catch (e) { console.warn('[Ollama] set_ollama_url failed:', e); }
        try { await get().refreshAvailableModels?.('ollama'); } catch { /* ignore */ }
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
        try { const isRunning = await invoke<boolean>('check_ollama_status'); set({ ollamaStatus: isRunning ? 'running' : 'error' }); }
        catch { set({ ollamaStatus: 'error' }); }
    },
    pullOllamaModel: async (name) => {
        set({ isPullingModel: true, pullProgress: 0 });
        try { await invoke('pull_ollama_model', { name }); get().refreshAvailableModels('ollama'); }
        catch (e) { console.error('Failed to pull model:', e); }
        finally { set({ isPullingModel: false }); }
    },
    setInferenceBackend: (backend) => {
        localStorage.setItem('inferenceBackend', backend);
        set({ inferenceBackend: backend });
        const st = get();
        if (backend === 'ollama' && st.ollamaServerMode === 'local') {
            applyLocalOllamaAgentDefaults(st as Parameters<typeof applyLocalOllamaAgentDefaults>[0]);
        }
        if (backend === 'llama-cpp') {
            import('../tauri_bridge').then(({ invoke }) => invoke('set_ollama_url', { url: st.llamaCppUrl }).catch(() => { }));
        } else if (backend === 'ollama') {
            import('../tauri_bridge').then(({ invoke }) => invoke('set_ollama_url', { url: st.ollamaUrl }).catch(() => { }));
        }
    },
    setLlamaCppUrl: (url) => {
        localStorage.setItem('llamaCppUrl', url);
        set({ llamaCppUrl: url });
        if (get().inferenceBackend === 'llama-cpp') {
            import('../tauri_bridge').then(({ invoke }) => invoke('set_ollama_url', { url }).catch(() => { }));
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
        const { ollamaUrl } = get();
        try {
            const keys: any = await invoke('get_api_keys').catch(() => ({}));
            const providers: string[] = ['Ollama'];
            if (keys.google) providers.push('Google');
            if (keys.anthropic) providers.push('Anthropic');
            if (keys.openai) providers.push('OpenAI');
            if (keys.openrouter) providers.push('Openrouter');
            if (keys.mistral) providers.push('Mistral');
            if ((keys as any).deepseek) providers.push('Deepseek');
            if ((keys as any).mimo) providers.push('Mimo');
            // Interface AI / highwayapi.ai — Claude Opus 4.8 (BYO key).
            if ((keys as any).highwayapi || (keys as any).highwayapi_base_url) providers.push('Highwayapi');
            if (typeof navigator !== 'undefined' && /Mac/i.test(navigator.platform || navigator.userAgent || '')) providers.push('Deepseek-ANE');
            if (keys.groq) providers.push('Groq');
            if (keys.xai) providers.push('xAI');
            if (keys.cerebras) providers.push('Cerebras');
            if (keys.alibaba) providers.push('Alibaba');
            if ((keys as any).nvidia) providers.push('Nvidia');
            // WebUI / personal-subscription models DISABLED — they scrape a browser session
            // and don't work reliably. Focus is API-key (BYOK) + Cyber-Ifrit Cloud.
            // Flip WEBUI_MODELS_ENABLED to true to re-enable.
            const WEBUI_MODELS_ENABLED = false;
            if (WEBUI_MODELS_ENABLED) {
                providers.push('OpenWebUI', 'Claude (WebUI)', 'Gemini (WebUI)', 'OpenAI (WebUI)', 'DeepSeek (WebUI)', 'Qwen (WebUI)');
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
                    if (p.toLowerCase() === 'ollama') {
                        const raw = get().ollamaUrl || 'http://localhost:11434';
                        const ollamaToUse = normalizeOllamaUrl(raw);
                        await invoke('set_ollama_url', { url: ollamaToUse });
                        set({ ollamaUrl: ollamaToUse });
                        try { localStorage.setItem('ollamaUrl', ollamaToUse); } catch { }
                        const isLocal = /localhost|127\.|0\.0\.0\.0/.test(ollamaToUse);
                        set({ ollamaConnectionMode: 'direct', ollamaMode: isLocal ? 'local' : 'cloud' });
                    }
                    let models: string[] = [];
                    if (p.includes('WebUI') && p !== 'OpenWebUI') {
                        const baseProvider = p.split(' ')[0].toLowerCase();
                        allModels.push({ id: `WebUI Session (${baseProvider})`, provider: p.toLowerCase() });
                        continue;
                    } else {
                        models = await invoke<string[]>('list_provider_models', { provider: p });
                        // Show only ACTUALLY-INSTALLED Ollama models in the picker —
                        // do not flood it with registry pull-hints the user hasn't
                        // downloaded (that belongs in the pull/install wizard). Keeps
                        // the picker Cursor-clean: exactly `ollama list`.
                        allModels = [...allModels, ...models.map(m => ({ id: m, provider: p.toLowerCase() }))];
                    }
                    if (p.toLowerCase() === 'ollama' && models.length > 0) set({ ollamaStatus: 'running' });
                } catch (e: any) {
                    const msg = typeof e === 'string' ? e : String(e ?? '');
                    const quiet =
                        msg.includes('API key not found') ||
                        msg.includes('Connection refused') ||
                        msg.includes('not reachable') ||
                        msg.includes('error trying to connect');
                    if (!quiet) console.error(`Failed to fetch models for ${p}:`, e);
                    if (p.toLowerCase() === 'ollama') set({ ollamaStatus: 'error' });
                }
            }
            // Guarantee Opus 4.8 appears when the Interface AI key is set + enabled,
            // even if the provider's /models listing is unavailable.
            if (((keys as any).highwayapi) && isEnabled('highwayapi') && !allModels.some(m => m.provider === 'highwayapi')) {
                allModels.push({ id: 'claude-opus-4-8', provider: 'highwayapi' });
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
    addMitmLog: (log) => set((s) => ({ mitmLogs: [...s.mitmLogs, `[${new Date().toLocaleTimeString()}] ${log}`].slice(-100) })),
    addMcpServer: async (name, config) => {
        await invoke('add_mcp_server', { name, config });
        await get().listMcpServers();
    },
    removeMcpServer: async (name) => { try { await invoke('remove_mcp_server', { name }); await get().listMcpServers(); } catch { } },
    listMcpServers: async () => { try { const servers = await invoke<any[]>('list_mcp_servers'); set({ mcpServers: servers }); } catch { } },
    setMcpServerEnabled: async (name, enabled) => { try { await invoke('set_mcp_server_enabled', { name, enabled }); await get().listMcpServers(); } catch { } },

    setVllmUrl: (url) => { try { localStorage.setItem('provider.vllm.url', url); } catch { } set({ vllmUrl: url }); },
    setLmStudioUrl: (url) => { try { localStorage.setItem('provider.lmstudio.url', url); } catch { } set({ lmStudioUrl: url }); },
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

