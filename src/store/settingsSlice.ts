import type { StateCreator } from 'zustand';
import type { AppState } from './index';
import {
    type ProviderName, type FeatureName, type ModelSelection, type ModelSelectionOptions,
    type ModelSelectionOfFeature, type GlobalSettings,
    defaultGlobalSettings, defaultModelSelectionOfFeature,
} from '../model_capabilities';

export interface SettingsSlice {
    tabPredictionEnabled: boolean;
    tabMultilineSuggestions: boolean;
    tabAcceptKey: 'Tab' | 'Enter';
    betaFastApply: boolean;
    betaSemanticSearch: boolean;
    betaShadowWorkspace: boolean;
    aimVfsEnabled: boolean;
    thermalGovernorEnabled: boolean;
    jitDecompressionEnabled: boolean;
    jitThreshold: number;
    networkProxyUrl: string;
    networkAllowInsecureTls: boolean;
    indexingEnabled: boolean;
    indexingDocsUrls: string[];
    kortexGacEnabled: boolean;
    kortexVramTotalMb: number;
    kortexTheta: number;
    kortexBackend: string;
    kortexServerBinary: string;
    kvCacheEnabled: boolean;
    kvCacheBaseDir: string;
    kvCacheMaxBytes: number;
    kvCacheProxyPort: number;
    kvCacheStats: any;
    ccetEnabled: boolean;
    ccetTauSkip: number;
    ccetTauCompress: number;
    ccetMaxSkipFraction: number;
    ccetEfficiency: any;
    kortexTelemetry: any;
    avatarCharacter: string;
    avatarCustomConfig?: { stickerUrl?: string; wallpaperUrl?: string; enabled?: boolean };
    avatar3dConfig?: { modelUrl?: string; modelId?: string; customModels?: Array<{ id: string; name: string; url: string }> };
    showVrmAvatar: boolean;
    iconThemeMapping: any;
    extensionContributions: any;
    activeDevice: string | null;
    emulators: string[];
    emulatorPlatform: 'ios' | 'android';
    isDevWorkflowActive: boolean;
    currentDevProject: any | null;
    modelSelectionOfFeature: ModelSelectionOfFeature;
    modelSelectionOptions: Partial<Record<FeatureName, Partial<Record<ProviderName, Record<string, ModelSelectionOptions>>>>>;
    voidGlobalSettings: GlobalSettings;
    currentReasoningBudget: number;
    currentReasoningEffort: string;
    isReasoningEnabled: boolean;

    setTabPredictionEnabled: (v: boolean) => void;
    setTabMultilineSuggestions: (v: boolean) => void;
    setTabAcceptKey: (k: 'Tab' | 'Enter') => void;
    setBetaFastApply: (v: boolean) => void;
    setBetaSemanticSearch: (v: boolean) => void;
    setBetaShadowWorkspace: (v: boolean) => void;
    setAimVfsEnabled: (v: boolean) => void;
    setThermalGovernorEnabled: (v: boolean) => void;
    setJitDecompressionEnabled: (v: boolean) => void;
    setJitThreshold: (v: number) => void;
    setNetworkProxyUrl: (url: string) => void;
    setNetworkAllowInsecureTls: (v: boolean) => void;
    setIndexingEnabled: (v: boolean) => void;
    setIndexingDocsUrls: (urls: string[]) => void;
    setKortexGacEnabled: (v: boolean) => void;
    setKortexVramTotalMb: (v: number) => void;
    setKortexTheta: (v: number) => void;
    setKortexBackend: (v: string) => void;
    setKortexServerBinary: (v: string) => void;
    setKvCacheEnabled: (v: boolean) => void;
    setKvCacheBaseDir: (v: string) => void;
    setKvCacheMaxBytes: (v: number) => void;
    setKvCacheProxyPort: (v: number) => void;
    refreshKvCacheStats: () => Promise<void>;
    setCcetEnabled: (v: boolean) => void;
    setCcetTauSkip: (v: number) => void;
    setCcetTauCompress: (v: number) => void;
    setCcetMaxSkipFraction: (v: number) => void;
    refreshCcetEfficiency: () => void;
    setAvatarCharacter: (v: string) => void;
    setAvatarCustomConfig: (config: { stickerUrl?: string; wallpaperUrl?: string; enabled?: boolean }) => void;
    setAvatar3dConfig: (config: { modelUrl?: string; modelId?: string; customModels?: Array<{ id: string; name: string; url: string }> }) => void;
    setShowVrmAvatar: (show: boolean) => void;
    setIconThemeMapping: (mapping: any) => void;
    setExtensionContributions: (contributions: any) => void;
    setActiveDevice: (id: string | null) => void;
    setEmulators: (ems: string[]) => void;
    setEmulatorPlatform: (platform: 'ios' | 'android') => void;
    setDevWorkflowActive: (active: boolean) => void;
    updateDevProject: (project: Partial<any>) => void;
    setModelSelectionForFeature: (feature: FeatureName, selection: ModelSelection | null) => void;
    setModelSelectionOptions: (feature: FeatureName, providerName: ProviderName, modelName: string, opts: ModelSelectionOptions) => void;
    setVoidGlobalSetting: <K extends keyof GlobalSettings>(key: K, value: GlobalSettings[K]) => void;
    setReasoningBudget: (v: number) => void;
    setReasoningEffort: (v: string) => void;
    setReasoningEnabled: (v: boolean) => void;
    importFromEditor: (source: 'vscode' | 'cursor' | 'windsurf' | 'cider') => Promise<void>;
}

export const createSettingsSlice: StateCreator<AppState, [], [], SettingsSlice> = (set, get) => ({
    tabPredictionEnabled: (() => { try { return localStorage.getItem('tab.predictionEnabled') !== '0'; } catch { return true; } })(),
    tabMultilineSuggestions: (() => { try { return localStorage.getItem('tab.multilineSuggestions') !== '0'; } catch { return true; } })(),
    tabAcceptKey: (() => { try { return (localStorage.getItem('tab.acceptKey') as 'Tab' | 'Enter') || 'Tab'; } catch { return 'Tab' as const; } })(),
    betaFastApply: (() => { try { return localStorage.getItem('beta.fastApply') !== '0'; } catch { return true; } })(),
    betaSemanticSearch: (() => { try { return localStorage.getItem('beta.semanticSearch') !== '0'; } catch { return true; } })(),
    betaShadowWorkspace: (() => { try { return localStorage.getItem('beta.shadowWorkspace') === '1'; } catch { return false; } })(),
    aimVfsEnabled: localStorage.getItem('aimVfsEnabled') !== 'false',
    thermalGovernorEnabled: localStorage.getItem('thermalGovernorEnabled') !== 'false',
    jitDecompressionEnabled: localStorage.getItem('jitDecompressionEnabled') === 'true',
    jitThreshold: parseFloat(localStorage.getItem('jitThreshold') || '0.85'),
    networkProxyUrl: (() => { try { return localStorage.getItem('network.proxyUrl') || ''; } catch { return ''; } })(),
    networkAllowInsecureTls: (() => { try { return localStorage.getItem('network.allowInsecureTls') === '1'; } catch { return false; } })(),
    indexingEnabled: (() => { try { return localStorage.getItem('indexing.enabled') !== '0'; } catch { return true; } })(),
    indexingDocsUrls: (() => { try { const raw = localStorage.getItem('indexing.docsUrls'); if (!raw) return []; const arr = JSON.parse(raw); return Array.isArray(arr) ? arr.filter((s: any) => typeof s === 'string') : []; } catch { return []; } })(),
    kortexGacEnabled: (() => { try { return localStorage.getItem('kortex.gacEnabled') !== '0'; } catch { return true; } })(),
    kortexVramTotalMb: (() => { try { return parseInt(localStorage.getItem('kortex.vramTotalMb') || '16384'); } catch { return 16384; } })(),
    kortexTheta: (() => { try { return parseFloat(localStorage.getItem('kortex.theta') || '0.85'); } catch { return 0.85; } })(),
    kortexBackend: (() => {
        try {
            const saved = localStorage.getItem('kortex.backend');
            if (saved) return saved;
            const isMac = typeof navigator !== 'undefined' && /Mac|iPhone|iPad/i.test(navigator.userAgent);
            return isMac ? 'metal' : 'vulkan';
        } catch { return 'vulkan'; }
    })(),
    kortexServerBinary: (() => { try { return localStorage.getItem('kortex.serverBinary') || ''; } catch { return ''; } })(),
    kvCacheEnabled: (() => { try { return localStorage.getItem('kvcache.enabled') !== '0'; } catch { return true; } })(),
    kvCacheBaseDir: (() => { try { return localStorage.getItem('kvcache.baseDir') || ''; } catch { return ''; } })(),
    kvCacheMaxBytes: (() => { try { return parseInt(localStorage.getItem('kvcache.maxBytes') || '17179869184'); } catch { return 17179869184; } })(),
    kvCacheProxyPort: (() => { try { return parseInt(localStorage.getItem('kvcache.proxyPort') || '1537'); } catch { return 1537; } })(),
    kvCacheStats: null,
    ccetEnabled: (() => { try { return localStorage.getItem('ccet.enabled') !== '0'; } catch { return true; } })(),
    ccetTauSkip: (() => { try { return parseFloat(localStorage.getItem('ccet.tauSkip') || '0.05'); } catch { return 0.05; } })(),
    ccetTauCompress: (() => { try { return parseFloat(localStorage.getItem('ccet.tauCompress') || '0.30'); } catch { return 0.30; } })(),
    ccetMaxSkipFraction: (() => { try { return parseFloat(localStorage.getItem('ccet.maxSkipFraction') || '0.40'); } catch { return 0.40; } })(),
    ccetEfficiency: null,
    kortexTelemetry: null,
    avatarCharacter: localStorage.getItem('avatarCharacter') || 'airi',
    avatarCustomConfig: JSON.parse(localStorage.getItem('avatarCustomConfig') || '{}'),
    avatar3dConfig: JSON.parse(localStorage.getItem('avatar3dConfig') || '{}'),
    showVrmAvatar: (() => { try { return localStorage.getItem('showVrmAvatar') === 'true'; } catch { return false; } })(),
    iconThemeMapping: null,
    extensionContributions: { viewsContainers: { activitybar: [] }, views: {} },
    activeDevice: null,
    emulators: [],
    emulatorPlatform: 'ios',
    isDevWorkflowActive: false,
    currentDevProject: null,
    modelSelectionOfFeature: (() => { try { const saved = localStorage.getItem('void.modelSelectionOfFeature'); return saved ? JSON.parse(saved) : { ...defaultModelSelectionOfFeature }; } catch { return { ...defaultModelSelectionOfFeature }; } })(),
    modelSelectionOptions: {},
    voidGlobalSettings: (() => { try { const saved = localStorage.getItem('void.globalSettings'); return saved ? { ...defaultGlobalSettings, ...JSON.parse(saved) } : { ...defaultGlobalSettings }; } catch { return { ...defaultGlobalSettings }; } })(),
    currentReasoningBudget: parseInt(localStorage.getItem('reasoning.budget') || '1024'),
    currentReasoningEffort: localStorage.getItem('reasoning.effort') || 'low',
    isReasoningEnabled: localStorage.getItem('reasoning.enabled') === '1',

    setTabPredictionEnabled: (v) => { try { localStorage.setItem('tab.predictionEnabled', v ? '1' : '0'); } catch { } set({ tabPredictionEnabled: v }); },
    setTabMultilineSuggestions: (v) => { try { localStorage.setItem('tab.multilineSuggestions', v ? '1' : '0'); } catch { } set({ tabMultilineSuggestions: v }); },
    setTabAcceptKey: (k) => { try { localStorage.setItem('tab.acceptKey', k); } catch { } set({ tabAcceptKey: k }); },
    setBetaFastApply: (v) => { try { localStorage.setItem('beta.fastApply', v ? '1' : '0'); } catch { } set({ betaFastApply: v }); },
    setBetaSemanticSearch: (v) => { try { localStorage.setItem('beta.semanticSearch', v ? '1' : '0'); } catch { } set({ betaSemanticSearch: v }); },
    setBetaShadowWorkspace: (v) => { try { localStorage.setItem('beta.shadowWorkspace', v ? '1' : '0'); } catch { } set({ betaShadowWorkspace: v }); },
    setAimVfsEnabled: (v) => { try { localStorage.setItem('aimVfsEnabled', v ? 'true' : 'false'); } catch { } set({ aimVfsEnabled: v }); },
    setThermalGovernorEnabled: (v) => { try { localStorage.setItem('thermalGovernorEnabled', v ? 'true' : 'false'); } catch { } set({ thermalGovernorEnabled: v }); },
    setJitDecompressionEnabled: (v) => { try { localStorage.setItem('jitDecompressionEnabled', v ? 'true' : 'false'); } catch { } set({ jitDecompressionEnabled: v }); },
    setJitThreshold: (v) => { try { localStorage.setItem('jitThreshold', v.toString()); } catch { } set({ jitThreshold: v }); },
    setNetworkProxyUrl: (url) => { try { localStorage.setItem('network.proxyUrl', url); } catch { } set({ networkProxyUrl: url }); },
    setNetworkAllowInsecureTls: (v) => { try { localStorage.setItem('network.allowInsecureTls', v ? '1' : '0'); } catch { } set({ networkAllowInsecureTls: v }); },
    setIndexingEnabled: (v) => { try { localStorage.setItem('indexing.enabled', v ? '1' : '0'); } catch { } set({ indexingEnabled: v }); },
    setIndexingDocsUrls: (urls) => { try { localStorage.setItem('indexing.docsUrls', JSON.stringify(urls)); } catch { } set({ indexingDocsUrls: urls }); },
    setKortexGacEnabled: (v) => { try { localStorage.setItem('kortex.gacEnabled', v ? '1' : '0'); } catch { } set({ kortexGacEnabled: v }); },
    setKortexVramTotalMb: (v) => { try { localStorage.setItem('kortex.vramTotalMb', String(v)); } catch { } set({ kortexVramTotalMb: v }); },
    setKortexTheta: (v) => { try { localStorage.setItem('kortex.theta', String(v)); } catch { } set({ kortexTheta: v }); },
    setKortexBackend: (v) => { try { localStorage.setItem('kortex.backend', v); } catch { } set({ kortexBackend: v }); },
    setKortexServerBinary: (v) => { try { localStorage.setItem('kortex.serverBinary', v); } catch { } set({ kortexServerBinary: v }); },
    setKvCacheEnabled: (v) => { try { localStorage.setItem('kvcache.enabled', v ? '1' : '0'); } catch { } set({ kvCacheEnabled: v }); },
    setKvCacheBaseDir: (v) => { try { localStorage.setItem('kvcache.baseDir', v); } catch { } set({ kvCacheBaseDir: v }); },
    setKvCacheMaxBytes: (v) => { try { localStorage.setItem('kvcache.maxBytes', String(v)); } catch { } set({ kvCacheMaxBytes: v }); },
    setKvCacheProxyPort: (v) => { try { localStorage.setItem('kvcache.proxyPort', String(v)); } catch { } set({ kvCacheProxyPort: v }); },
    refreshKvCacheStats: async () => {
        try { const { getKvCacheStats } = await import('../kortex/kvcache-orchestrator'); const stats = await getKvCacheStats(); set({ kvCacheStats: stats }); } catch { }
    },
    setCcetEnabled: (v) => { try { localStorage.setItem('ccet.enabled', v ? '1' : '0'); } catch { } set({ ccetEnabled: v }); },
    setCcetTauSkip: (v) => { try { localStorage.setItem('ccet.tauSkip', String(v)); } catch { } set({ ccetTauSkip: v }); },
    setCcetTauCompress: (v) => { try { localStorage.setItem('ccet.tauCompress', String(v)); } catch { } set({ ccetTauCompress: v }); },
    setCcetMaxSkipFraction: (v) => { try { localStorage.setItem('ccet.maxSkipFraction', String(v)); } catch { } set({ ccetMaxSkipFraction: v }); },
    refreshCcetEfficiency: async () => {
        try { const { summarizeEfficiency } = await import('../kortex/ccet'); const eff = summarizeEfficiency(); set({ ccetEfficiency: eff || { sample_size: 0, avg_eta: 0, avg_saved_fraction: 0, total_skipped_segments: 0 } }); } catch { }
    },
    setAvatarCharacter: (avatarCharacter) => { localStorage.setItem('avatarCharacter', avatarCharacter); set({ avatarCharacter }); },
    setAvatarCustomConfig: (config) => {
        const existing = JSON.parse(localStorage.getItem('avatarCustomConfig') || '{}');
        const updated = { ...existing, ...config };
        localStorage.setItem('avatarCustomConfig', JSON.stringify(updated));
        set({ avatarCustomConfig: updated });
    },
    setAvatar3dConfig: (config) => {
        const existing = JSON.parse(localStorage.getItem('avatar3dConfig') || '{}');
        const updated = { ...existing, ...config };
        localStorage.setItem('avatar3dConfig', JSON.stringify(updated));
        set({ avatar3dConfig: updated });
    },
    setShowVrmAvatar: (show) => { try { localStorage.setItem('showVrmAvatar', show ? 'true' : 'false'); } catch { } set({ showVrmAvatar: show }); },
    setIconThemeMapping: (iconThemeMapping) => set({ iconThemeMapping }),
    setExtensionContributions: (extensionContributions) => set({ extensionContributions }),
    setActiveDevice: (activeDevice) => set({ activeDevice }),
    setEmulators: (emulators) => set({ emulators }),
    setEmulatorPlatform: (platform) => set({ emulatorPlatform: platform }),
    setDevWorkflowActive: (active) => set({ isDevWorkflowActive: active }),
    updateDevProject: (project) => set((state) => ({
        currentDevProject: state.currentDevProject
            ? { ...state.currentDevProject, ...project }
            : { id: `dev_${Date.now()}`, name: 'New App', currentPhase: 'requirements', progress: 0, emulatorPreview: true, platform: 'cross-platform', ...project },
    })),
    setModelSelectionForFeature: (feature, selection) => set((state: any) => {
        const next = { ...state.modelSelectionOfFeature, [feature]: selection };
        try { localStorage.setItem('void.modelSelectionOfFeature', JSON.stringify(next)); } catch { }
        return { modelSelectionOfFeature: next };
    }),
    setModelSelectionOptions: (feature, providerName, modelName, opts) => set((state: any) => ({
        modelSelectionOptions: { ...state.modelSelectionOptions, [feature]: { ...(state.modelSelectionOptions[feature] || {}), [providerName]: { ...((state.modelSelectionOptions[feature] || {})[providerName] || {}), [modelName]: opts } } },
    })),
    setVoidGlobalSetting: (key, value) => set((state: any) => {
        const next = { ...state.voidGlobalSettings, [key]: value };
        try { localStorage.setItem('void.globalSettings', JSON.stringify(next)); } catch { }
        return { voidGlobalSettings: next };
    }),
    setReasoningBudget: (v) => { try { localStorage.setItem('reasoning.budget', String(v)); } catch { } set({ currentReasoningBudget: v }); },
    setReasoningEffort: (v) => { try { localStorage.setItem('reasoning.effort', v); } catch { } set({ currentReasoningEffort: v }); },
    setReasoningEnabled: (v) => { try { localStorage.setItem('reasoning.enabled', v ? '1' : '0'); } catch { } set({ isReasoningEnabled: v }); },
    importFromEditor: async (source) => { try { const { invoke } = await import('@tauri-apps/api/core'); await invoke('import_editor_settings', { source }); } catch { } },
});

