// Agent modes slice: mode/model/backend toggles, planner, AIRI, custom modes, hooks, steering.
// Split from agentSlice.ts; combined in the agentSlice.ts barrel.

import type { StateCreator } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import { MAX_AGENT_MESSAGES_IN_UI, MAX_AGENT_MESSAGE_CHARS } from '../domain/agent/AgentSessionPolicy';
import type { AppState } from './index';
import type {
    AgentMessage, AgentStep, Artifact, AttachedContext, AgentTask, TaskArtifact, SemanticSlot,
} from './types';
import type { AgentToolBlock } from '../domain/agent/agentToolBlocks';
import { createToolBlock, enrichCanvasBlockFromResult, enrichEditBlockFromResult } from '../domain/agent/agentToolBlocks';
import { toolsMatchForFinish } from '../domain/agent/toolAliases';
import { cleanAgentContent, shouldReplaceAgentContent } from '../domain/agent/cleanAgentContent';
import { onAgentModeChanged } from '../lib/agentAutonomy';
import { type CustomMode, loadCustomModes, parseThought, normalizeBackendMessages, mapBackendChatMessages } from './agentSliceShared';

export interface AgentModesSlice {
    isYoloMode: boolean;
    isContinuousMode: boolean;
    /** Live agent browser vision (screenshot polling). Default OFF — it polls the
     *  stealth browser ~1.5s and is memory/CPU-heavy on low-spec machines. */
    isAgentVisionEnabled: boolean;
    /** Stealth Firefox: true = invisible desktop (off-screen), false = visible OS window. */
    browserStealthHidden: boolean;
    setBrowserStealthHidden: (v: boolean) => void;
    agentMode: string;
    agentModel: string;
    /** Hybrid deep-reasoning planner: strongest model PLANS, executor (agentModel) ACTS. */
    plannerModel: string;    // explicit "provider|id"; empty = auto-detect from availableModels
    plannerEnabled: boolean; // master switch for the hybrid plan→act→verify pipeline
    hybridAuto: boolean;     // true = auto-pick planner via classifyModels()
    /** Which agent core runs turns: built-in Sentient (Hermes skills native) or external claurst. */
    agentBackend: 'sentient' | 'claurst';
    setAgentBackend: (b: 'sentient' | 'claurst') => void;
    agentRootAccess: boolean;
    agentHooks: { id: string; pattern: string; prompt: string; enabled: boolean; trigger?: string; name?: string }[];
    globalSteeringRule: string;
    agentUiMode: 'chat' | 'airi';
    /** Cursor-style chat: hide raw tool names/JSON in panel; tooling lives in terminal. */
    agentCleanUi: boolean;
    ttsStrategy: 'elevenlabs' | 'openai' | 'browser' | 'qwen' | 'qwen-native';
    airiVisionEnabled: boolean;
    airiVisionModel: string;
    customModes: CustomMode[];
    addCustomMode: (m: CustomMode) => void;
    removeCustomMode: (id: string) => void;
    airiConsciousnessEnabled: boolean;
    airiConsciousnessModel: string;
    taskPlannerState: any | null;
    isPlanMode: boolean;
    isCascadeWriteMode: boolean;
    artifactReviewPolicy: 'always_proceed' | 'request_review';
    /** Antigravity-style cascade/run id for brain + trajectory persistence. */
    activeCascadeId: string | null;
    setActiveCascadeId: (id: string | null) => void;
    terminalAutoExecution: 'always_proceed' | 'request_review';
    setYoloMode: (v: boolean) => void;
    setContinuousMode: (v: boolean) => void;
    setAgentVisionEnabled: (v: boolean) => void;
    setAgentMode: (mode: string) => void;
    setAgentModel: (model: string) => void;
    setPlannerModel: (model: string) => void;
    setPlannerEnabled: (v: boolean) => void;
    setHybridAuto: (v: boolean) => void;
    setAgentRootAccess: (v: boolean) => void;
    setAgentHooks: (hooks: { id: string; pattern: string; prompt: string; enabled: boolean; trigger?: string; name?: string }[]) => void;
    /** Fire all enabled hooks whose trigger matches the given event (e.g. 'on_commit'). */
    fireHooks: (trigger: string, context?: string) => void;
    setGlobalSteeringRule: (rule: string) => void;
    setAiriVisionEnabled: (v: boolean) => void;
    setAiriVisionModel: (model: string) => void;
    setAiriConsciousnessEnabled: (v: boolean) => void;
    setAiriConsciousnessModel: (model: string) => void;
    setAgentUiMode: (mode: 'chat' | 'airi') => void;
    setAgentCleanUi: (v: boolean) => void;
    setTtsStrategy: (strategy: 'elevenlabs' | 'openai' | 'browser' | 'qwen' | 'qwen-native') => void;
    setArtifactReviewPolicy: (policy: 'always_proceed' | 'request_review') => void;
    setTerminalAutoExecution: (policy: 'always_proceed' | 'request_review') => void;
    backendPing: () => Promise<string>;
    togglePlanMode: () => void;
    toggleCascadeWriteMode: () => void;
}

export const createAgentModesSlice: StateCreator<AppState, [], [], AgentModesSlice> = (set, get) => ({
    isYoloMode: false,
    isContinuousMode: false,
    isAgentVisionEnabled: (typeof localStorage !== 'undefined' && localStorage.getItem('agent.liveVision') === '1') || false,
    browserStealthHidden: (typeof localStorage !== 'undefined' && localStorage.getItem('agent.browserHidden') === '1') || false,
    agentMode: (typeof localStorage !== 'undefined' && localStorage.getItem('agent.mode')) || 'Harness',
    agentModel: (() => {
        if (typeof localStorage === 'undefined') return '';
        const saved = localStorage.getItem('agentModel') || '';
        const oldDefaults = new Set([
            'Ollama|airi-fast:latest', 'Ollama|qwen3:35b', 'qwen3:35b',
            'cyberifrit|qwen3:35b', 'cyberifrit|cyberifrit/qwen3:35b',
            'huihui_ai/qwen2.5-coder-abliterate:7b', 'Ollama|huihui_ai/qwen2.5-coder-abliterate:7b',
            // Legacy fake model — Antigravity is a workflow layer, not an LLM id.
            'Antigravity|antigravity-sentient', 'antigravity|antigravity-sentient',
        ]);
        if (oldDefaults.has(saved)) { localStorage.removeItem('agentModel'); return ''; }
        return saved;
    })(),
    plannerModel: (typeof localStorage !== 'undefined' && localStorage.getItem('agent.plannerModel')) || '',
    // OFF by default: the hybrid planner delegates iteration-0 to a stronger model,
    // but auto-detect can pick a cloud planner that isn't reachable (no key) and the
    // run then hangs at iter-0. Opt in explicitly after a working cloud key is set.
    plannerEnabled: (typeof localStorage !== 'undefined') && localStorage.getItem('agent.plannerEnabled') === '1',
    hybridAuto: (typeof localStorage === 'undefined') ? true : localStorage.getItem('agent.hybridAuto') !== '0',
    agentRootAccess: true,
    agentHooks: [
        { id: '1', pattern: '*.tsx', prompt: 'Check for React performance anti-patterns.', enabled: false },
        { id: '2', pattern: 'src/backend/*.rs', prompt: 'Ensure all public functions have rustdoc comments.', enabled: false },
    ],
    globalSteeringRule: '',
    agentUiMode: (localStorage.getItem('agentUiMode') as 'chat' | 'airi') || 'chat',
    agentCleanUi: (typeof localStorage !== 'undefined') ? localStorage.getItem('agent.cleanUi') !== '0' : true,
    ttsStrategy: (localStorage.getItem('ttsStrategy') as any) || 'elevenlabs',
    customModes: loadCustomModes(),
    addCustomMode: (m: CustomMode) => set((s: any) => {
        const next = [...(s.customModes || []).filter((x: CustomMode) => x.id !== m.id), m];
        try { localStorage.setItem('airi.customModes', JSON.stringify(next)); } catch { }
        return { customModes: next };
    }),
    removeCustomMode: (id: string) => set((s: any) => {
        const next = (s.customModes || []).filter((x: CustomMode) => x.id !== id);
        try { localStorage.setItem('airi.customModes', JSON.stringify(next)); } catch { }
        return { customModes: next };
    }),
    airiVisionEnabled: (typeof localStorage !== 'undefined' && localStorage.getItem('airi.vision.enabled') === '1' && !!localStorage.getItem('airi.vision.model')?.trim()) || false,
    airiVisionModel: (() => {
        if (typeof localStorage === 'undefined') return '';
        const saved = localStorage.getItem('airi.vision.model') || '';
        if (saved === 'qwen2.5vl:72b') { localStorage.removeItem('airi.vision.model'); return ''; }
        return saved;
    })(),
    airiConsciousnessEnabled: (typeof localStorage !== 'undefined' && localStorage.getItem('airi.consciousness.enabled') === '1') || false,
    airiConsciousnessModel: (() => {
        if (typeof localStorage === 'undefined') return '';
        const saved = localStorage.getItem('airi.consciousness.model') || '';
        if (saved === 'airi-fast:latest') { localStorage.removeItem('airi.consciousness.model'); return ''; }
        return saved;
    })(),
    taskPlannerState: null,
    isPlanMode: false,
    isCascadeWriteMode: (typeof localStorage !== 'undefined') && localStorage.getItem('agent.cascadeWrite') === '1',
    artifactReviewPolicy: 'request_review',
    activeCascadeId: null,
    terminalAutoExecution: 'request_review',
    agentBackend: ((typeof localStorage !== 'undefined' && localStorage.getItem('agent.backend')) as 'sentient' | 'claurst') || 'sentient',
    setAgentBackend: (agentBackend) => { try { localStorage.setItem('agent.backend', agentBackend); } catch { } set({ agentBackend }); },
    setAgentMode: (agentMode) => {
        try { localStorage.setItem('agent.mode', agentMode); } catch { }
        set({ agentMode });
        onAgentModeChanged(agentMode);
        // Bug-bounty / offensive mode auto-drives the dedicated local Ollama model
        // `claude-bug-bounty` when it's installed — zero manual switching.
        const offensive = agentMode === 'BugBounty' || agentMode === 'Bug Bounty'
            || agentMode === 'RedTeam' || agentMode === 'Red Team';
        if (offensive) {
            const models = get().availableModels || [];
            const match = models.find((m: any) => {
                const id = (typeof m === 'string' ? m : m.id || m.name || '').toLowerCase();
                return id.includes('claude-bug-bounty');
            });
            if (match) {
                const id = typeof match === 'string' ? match : (match.id || match.name);
                if (id && get().agentModel !== id) get().setAgentModel?.(id);
            }
        }
    },
    setAgentModel: (agentModel) => {
        const stale = new Set(['antigravity|antigravity-sentient', 'Antigravity|antigravity-sentient']);
        if (stale.has(agentModel)) {
            console.warn('[agent] antigravity-sentient is not a real model — pick Ollama/Cyber-Ifrit from the toolbar.');
            agentModel = '';
        }
        try { localStorage.setItem('agentModel', agentModel); } catch { }
        set({ agentModel });
        // Align the global inference backend with the picked model's provider so the
        // pre-flight probe, status pill, and endpoint URL all target the right server.
        // Only flip for local OpenAI-compat backends — cloud models route by provider.
        try {
            const provider = (agentModel.includes('|') ? agentModel.split('|')[0] : '').toLowerCase();
            const g: any = get();
            const setBackend = g.setInferenceBackend;
            if (typeof setBackend === 'function') {
                if (provider === 'lemonade' && g.inferenceBackend !== 'lemonade') setBackend('lemonade');
                else if (provider === 'huggingface' && g.inferenceBackend !== 'huggingface') setBackend('huggingface');
                else if (provider === 'openmodel' && g.inferenceBackend !== 'openmodel') setBackend('openmodel');
                else if ((provider === 'ollama' || provider === 'antigravity') && g.inferenceBackend !== 'ollama') setBackend('ollama');
            }
        } catch { /* non-fatal */ }
    },
    setPlannerModel: (plannerModel) => { try { localStorage.setItem('agent.plannerModel', plannerModel); } catch { } set({ plannerModel }); },
    setPlannerEnabled: (plannerEnabled) => {
        if (get().ollamaServerMode === 'local') plannerEnabled = false;
        try { localStorage.setItem('agent.plannerEnabled', plannerEnabled ? '1' : '0'); } catch { }
        set({ plannerEnabled });
    },
    setHybridAuto: (hybridAuto) => { try { localStorage.setItem('agent.hybridAuto', hybridAuto ? '1' : '0'); } catch { } set({ hybridAuto }); },
    setAgentRootAccess: (_) => set({ agentRootAccess: true }),
    setYoloMode: (isYoloMode) => set({ isYoloMode }),
    setContinuousMode: (isContinuousMode) => set({ isContinuousMode }),
    setAgentVisionEnabled: (v) => {
        try { localStorage.setItem('agent.liveVision', v ? '1' : '0'); } catch { /* */ }
        set({ isAgentVisionEnabled: v });
    },
    setBrowserStealthHidden: (v) => {
        try { localStorage.setItem('agent.browserHidden', v ? '1' : '0'); } catch { /* */ }
        set({ browserStealthHidden: v });
        import('../tauri_bridge').then(({ invoke }) => {
            invoke('browser_set_headless', { headless: v }).catch(() => { /* sidecar optional */ });
        });
    },

    setAgentHooks: (hooks) => set({ agentHooks: hooks }),
    fireHooks: (trigger, context) => {
        const s: any = get();
        const matching = (s.agentHooks || []).filter((h: any) => h.enabled && (h.trigger || 'on_save') === trigger);
        for (const hook of matching) {
            const globalRule = s.globalSteeringRule;
            const fullPrompt = `[Triggered by ${trigger}${context ? ` · ${context}` : ''}]\n`
                + (globalRule ? `Global Rule: ${globalRule}\n` : '')
                + `Task: ${hook.prompt}`;
            s.runBackgroundAgent?.(fullPrompt).catch?.(console.error);
        }
    },
    setGlobalSteeringRule: (rule) => set({ globalSteeringRule: rule }),

    setAiriVisionEnabled: (enabled) => {
        if (enabled) {
            const currentModel = get().airiVisionModel;
            if (!currentModel || currentModel.trim() === '') {
                alert('Please specify an AI vision model tag in the settings/AIRI panel first.');
                set({ airiVisionEnabled: false });
                try { localStorage.setItem('airi.vision.enabled', '0'); } catch { }
                return;
            }
        }
        try { localStorage.setItem('airi.vision.enabled', enabled ? '1' : '0'); } catch { }
        set({ airiVisionEnabled: enabled });
        (async () => {
            try {
                /* AIRI vision removed */
            } catch (err) { console.warn('[store] toggling AIRI vision failed:', err); }
        })();
    },
    setAiriVisionModel: (model) => {
        try { localStorage.setItem('airi.vision.model', model); } catch { }
        set({ airiVisionModel: model });
        if (!model || model.trim() === '') {
            set({ airiVisionEnabled: false });
            try { localStorage.setItem('airi.vision.enabled', '0'); } catch { }
        } else {
            (async () => {
                try {
                    /* AIRI vision-analysis removed */
                } catch { }
            })();
        }
    },
    setAiriConsciousnessEnabled: (enabled) => {
        try { localStorage.setItem('airi.consciousness.enabled', enabled ? '1' : '0'); } catch { }
        set({ airiConsciousnessEnabled: enabled });
        (async () => {
            try {
                const { airiConsciousness } = Promise.reject(new Error('AIRI removed')) as any;
                if (enabled) {
                    if (typeof (airiConsciousness as any).resumeThoughts === 'function') (airiConsciousness as any).resumeThoughts();
                    else if (typeof (airiConsciousness as any).wakeUp === 'function') (airiConsciousness as any).wakeUp();
                    else if (typeof (airiConsciousness as any).start === 'function') (airiConsciousness as any).start();
                } else if (typeof (airiConsciousness as any).pauseThoughts === 'function') {
                    (airiConsciousness as any).pauseThoughts();
                }
            } catch (err) { console.warn('[store] toggling consciousness failed:', err); }
        })();
    },
    setAiriConsciousnessModel: (model) => {
        try { localStorage.setItem('airi.consciousness.model', model); } catch { }
        set({ airiConsciousnessModel: model });
        (async () => {
            try {
                const { airiConsciousness } = Promise.reject(new Error('AIRI removed')) as any;
                if (typeof (airiConsciousness as any).reconfigure === 'function') (airiConsciousness as any).reconfigure({ model });
            } catch { }
        })();
    },
    setAgentUiMode: (agentUiMode) => { localStorage.setItem('agentUiMode', agentUiMode); set({ agentUiMode }); },
    setAgentCleanUi: (agentCleanUi) => {
        localStorage.setItem('agent.cleanUi', agentCleanUi ? '1' : '0');
        set({ agentCleanUi });
    },
    setTtsStrategy: (strategy) => {
        localStorage.setItem('ttsStrategy', strategy);
        set({ ttsStrategy: strategy });
        import('../voice').then(({ setProvider }) => setProvider(strategy)).catch(() => { });
    },
    setArtifactReviewPolicy: (policy) => set({ artifactReviewPolicy: policy }),
    setActiveCascadeId: (id) => set({ activeCascadeId: id }),
    setTerminalAutoExecution: (policy) => set({ terminalAutoExecution: policy }),

    backendPing: async () => {
        try { return await invoke<string>('backend_ping'); } catch (error) { return `Error: ${error}`; }
    },

    togglePlanMode: () => set((s) => ({ isPlanMode: !s.isPlanMode })),
    toggleCascadeWriteMode: () => set((s) => {
        const next = !s.isCascadeWriteMode;
        try { localStorage.setItem('agent.cascadeWrite', next ? '1' : '0'); } catch { /* */ }
        return { isCascadeWriteMode: next };
    }),

});
