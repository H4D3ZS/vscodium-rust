import type { StateCreator } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import { MAX_AGENT_MESSAGES_IN_UI, MAX_AGENT_MESSAGE_CHARS } from '../domain/agent/AgentSessionPolicy';
import type { AppState } from './index';
import type {
    AgentMessage, AgentStep, Artifact, AttachedContext, AgentTask, TaskArtifact, SemanticSlot,
} from './types';
import type { AgentToolBlock } from '../domain/agent/agentToolBlocks';
import { createToolBlock, enrichEditBlockFromResult, toolsMatchForFinish } from '../domain/agent/agentToolBlocks';

/** A user-defined agent mode (Kilo-style): name + persona prompt + optional model. */
export interface CustomMode {
    id: string;
    label: string;
    systemPrompt: string;
    model?: string;       // optional "Provider|model" override
    readOnly?: boolean;   // if true, no file writes / commands (chat-like)
}

function loadCustomModes(): CustomMode[] {
    try {
        const raw = localStorage.getItem('airi.customModes');
        const arr = raw ? JSON.parse(raw) : [];
        return Array.isArray(arr) ? arr : [];
    } catch { return []; }
}

function parseThought(thought: any): { logic: string; action: string; confidence?: number } | null {
    if (!thought) return null;
    if (typeof thought === 'object') {
        return {
            logic: thought.logic || '',
            action: thought.action || 'Reasoning',
            confidence: thought.confidence !== undefined ? thought.confidence : 0.95
        };
    }
    if (typeof thought === 'string') {
        const trimmed = thought.trim();
        if (trimmed.startsWith('{') && trimmed.endsWith('}')) {
            try {
                const parsed = JSON.parse(trimmed);
                return {
                    logic: parsed.logic || parsed.reasoning || trimmed,
                    action: parsed.action || 'Thinking',
                    confidence: parsed.confidence !== undefined ? parsed.confidence : 0.95
                };
            } catch {
                // fall through
            }
        }
        return {
            logic: trimmed,
            action: 'Reasoning',
            confidence: 0.95
        };
    }
    return null;
}

/** Normalize Tauri `get_agent_messages` / `load_chat_session` payloads. */
function normalizeBackendMessages(raw: unknown): any[] {
    if (Array.isArray(raw)) return raw;
    if (raw && typeof raw === 'object') {
        const o = raw as Record<string, unknown>;
        if (Array.isArray(o.messages)) return o.messages;
        if (Array.isArray(o.data)) return o.data;
    }
    return [];
}

/** Normalize Rust ChatMessage JSON into UI AgentMessage rows. */
export function mapBackendChatMessages(raw: unknown): AgentMessage[] {
    const extract = (c: any): string => {
        if (c == null) return '';
        if (typeof c === 'string') return c;
        if (Array.isArray(c)) return c.map((p) => extract(p?.text ?? p?.Text ?? p)).join('');
        if (typeof c === 'object') {
            if (typeof c.text === 'string') return c.text;
            if (typeof c.Text === 'string') return c.Text;
            if (typeof c.content === 'string') return c.content;
            if (Array.isArray(c.parts)) return extract(c.parts);
            // serde untagged enum sometimes round-trips as {"Text":"..."}
            const keys = Object.keys(c);
            if (keys.length === 1 && typeof (c as any)[keys[0]] === 'string') {
                return String((c as any)[keys[0]]);
            }
        }
        return String(c);
    };
    return normalizeBackendMessages(raw)
        .filter((m: any) => m && (m.role === 'user' || m.role === 'assistant'))
        .map((m: any, i: number) => ({
            id: m.id || `restored-${i}-${m.role}`,
            role: m.role as 'user' | 'assistant',
            content: extract(m.content),
            timestamp: m.timestamp
                ?? m.metadata?.timestamp
                ?? (typeof m.metadata === 'object' ? m.metadata?.['timestamp'] : undefined)
                ?? Date.now() + i,
            steps: [],
        }))
        .filter((m) => m.content.trim().length > 0);
}

export interface AgentSlice {
    // State
    agentMessages: AgentMessage[];
    isAgentThinking: boolean;
    isAgentPaused: boolean;
    isAgentBlocked: boolean;
    isYoloMode: boolean;
    isContinuousMode: boolean;
    /** Live agent browser vision (screenshot polling). Default OFF — it polls the
     *  headless browser ~1.5s and is memory/CPU-heavy on low-spec machines. */
    isAgentVisionEnabled: boolean;
    agentMode: string;
    agentModel: string;
    /** Hybrid deep-reasoning planner: strongest model PLANS, executor (agentModel) ACTS. */
    plannerModel: string;    // explicit "provider|id"; empty = auto-detect from availableModels
    plannerEnabled: boolean; // master switch for the hybrid plan→act→verify pipeline
    hybridAuto: boolean;     // true = auto-pick planner via classifyModels()
    /** Which agent core runs turns: built-in Sentient, or the external claurst process. */
    agentBackend: 'sentient' | 'claurst';
    setAgentBackend: (b: 'sentient' | 'claurst') => void;
    agentRootAccess: boolean;
    agentCurrentAction: string | null;
    agentTrajectory: {
        id: string; ts: number;
        kind: 'tool_call' | 'tool_result' | 'content' | 'phase' | 'error';
        tool?: string; title: string; detail?: string; success?: boolean; turn?: number;
    }[];
    /** Live Cursor-style tool cards for the current agent turn (terminal stream, reads, edits). */
    agentToolBlocks: AgentToolBlock[];
    isTrajectoryOpen: boolean;
    currentTurnId: number;
    backgroundAgents: { id: string; prompt: string; status: 'pending' | 'running' | 'done' | 'error'; result: string; startedAt: number; finishedAt?: number }[];
    agentHooks: { id: string; pattern: string; prompt: string; enabled: boolean; trigger?: string; name?: string }[];
    globalSteeringRule: string;
    pendingAgentEdits: { path: string; tool: string; timestamp: number; preview?: string }[];
    isMultiFileReviewOpen: boolean;
    lastAgentCheckpoint: { id: string; description: string; timestamp: number } | null;
    attachedFiles: { id: string; path: string; name: string; gist?: string; thumbnail?: string; type: 'file' | 'attachment' | 'mention' }[];
    attachedContext: AttachedContext[];
    agentTask: AgentTask | null;
    agentTasks: any[];
    agentFiles: string[];
    agentSteps: any[];
    currentPhase: 'ANALYZE' | 'PLAN' | 'EXECUTE' | 'VERIFY' | 'REPORT' | 'IDLE';
    currentPhaseStatus: string;
    currentThought: { logic: string; action: string; confidence?: number } | null;
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
    pendingToolPermission: { id: string; tool: string; args: any; level: 'caution' | 'dangerous' } | null;
    ghostRuntimeResults: any[];
    agentThreads: Record<string, {
        id: string;
        name: string;
        messages: any[];
        isThinking: boolean;
        tasks: any[];
        artifacts: any[];
        /** .aim path this tab was restored from — prevents duplicate tabs on re-click. */
        chatSessionPath?: string;
    }>;
    activeAgentThreadId: string;
    /** Bumped after history restore so UI clears live tool-call overlays. */
    chatRestoreToken: number;
    artifactReviewPolicy: 'always_proceed' | 'request_review';
    /** Antigravity-style cascade/run id for brain + trajectory persistence. */
    activeCascadeId: string | null;
    setActiveCascadeId: (id: string | null) => void;
    terminalAutoExecution: 'always_proceed' | 'request_review';
    contextSlots: SemanticSlot[];
    activeFileContext: { symbols: string[]; related_files: string[]; relevant_lessons: SemanticSlot[] } | null;
    activeProjectSpec: any | null;
    projectMemory: string;
    memoryFiles: string[];
    chatSessions: any[];
    brainTelemetry: any | null;
    kairosSuggestions: any[];
    kairosStatus: 'idle' | 'indexing' | 'dreaming';
    processStats: import('../domain/performance/ProcessMemorySnapshot').ProcessStatsDto | null;
    memorySavings: { original: number; compressed: number } | null;

    // Actions
    addAgentMessage: (role: 'user' | 'assistant', content: string, context?: AttachedContext[] | boolean) => void;
    updateLastAgentMessage: (content: string) => void;
    appendLastAgentMessage: (delta: string) => void;
    updateLastAgentThought: (thought: string) => void;
    addAgentStep: (name: string, type?: AgentStep['type'], args?: any, callId?: string) => void;
    updateAgentStepStatus: (name: string, status: 'running' | 'success' | 'error', result?: string, summary?: string, callId?: string) => void;
    addAgentFile: (path: string) => void;
    addAgentArtifact: (artifact: Omit<Artifact, 'id' | 'timestamp'>) => void;
    setIsAgentThinking: (v: boolean) => void;
    setIsAgentPaused: (v: boolean) => void;
    setAgentBlocked: (v: boolean) => void;
    setYoloMode: (v: boolean) => void;
    setContinuousMode: (v: boolean) => void;
    setAgentVisionEnabled: (v: boolean) => void;
    setAgentMode: (mode: string) => void;
    setAgentModel: (model: string) => void;
    setPlannerModel: (model: string) => void;
    setPlannerEnabled: (v: boolean) => void;
    setHybridAuto: (v: boolean) => void;
    setAgentRootAccess: (v: boolean) => void;
    setAgentCurrentAction: (action: string | null) => void;
    setAgentMessages: (messages: any[]) => void;
    clearAgentMessages: () => void;
    resetThread: () => void;
    truncateAgentMessages: (index: number) => void;
    pushTrajectoryEvent: (evt: { kind: 'tool_call' | 'tool_result' | 'content' | 'phase' | 'error'; tool?: string; title: string; detail?: string; success?: boolean }) => void;
    clearTrajectory: () => void;
    clearAgentToolBlocks: () => void;
    registerAgentToolCall: (tool: string, args: unknown, callId?: string) => void;
    appendAgentToolOutput: (streamId: string, line: string, stream?: string) => void;
    bindAgentToolStream: (streamId: string, command: string) => void;
    finishAgentToolCall: (tool: string, success: boolean, result?: string, streamId?: string, callId?: string) => void;
    finalizeAgentToolBlocks: () => void;
    openTrajectory: () => void;
    closeTrajectory: () => void;
    beginNewTurn: () => void;
    runBackgroundAgent: (prompt: string) => Promise<string>;
    removeBackgroundAgent: (id: string) => void;
    clearBackgroundAgents: () => void;
    setAgentHooks: (hooks: { id: string; pattern: string; prompt: string; enabled: boolean; trigger?: string; name?: string }[]) => void;
    /** Fire all enabled hooks whose trigger matches the given event (e.g. 'on_commit'). */
    fireHooks: (trigger: string, context?: string) => void;
    setGlobalSteeringRule: (rule: string) => void;
    addPendingAgentEdit: (edit: { path: string; tool: string; preview?: string }) => void;
    clearPendingAgentEdits: () => void;
    openMultiFileReview: () => void;
    closeMultiFileReview: () => void;
    setLastAgentCheckpoint: (cp: { id: string; description: string; timestamp: number } | null) => void;
    rollbackLastAgentCheckpoint: () => Promise<{ ok: boolean; message: string }>;
    setLastUserMessageCheckpoint: (checkpointId: string, description?: string) => void;
    restoreToMessageCheckpoint: (timestamp: number) => Promise<{ ok: boolean; message: string }>;
    attachFile: (file: any | any[]) => void;
    removeFile: (path: string) => void;
    clearAttachedFiles: () => void;
    addAttachedContext: (item: AttachedContext) => void;
    removeAttachedContext: (index: number) => void;
    clearAttachedContext: () => void;
    updateAgentTask: (task: Partial<AgentTask> & { id: string }) => void;
    setAgentTask: (task: AgentTask | null) => void;
    setAgentTasks: (tasks: any[]) => void;
    setAgentFiles: (files: string[]) => void;
    setAgentSteps: (steps: AgentStep[]) => void;
    setPhase: (phase: 'ANALYZE' | 'PLAN' | 'EXECUTE' | 'VERIFY' | 'REPORT' | 'IDLE', status: string) => void;
    setAiriVisionEnabled: (v: boolean) => void;
    setAiriVisionModel: (model: string) => void;
    setAiriConsciousnessEnabled: (v: boolean) => void;
    setAiriConsciousnessModel: (model: string) => void;
    setAgentUiMode: (mode: 'chat' | 'airi') => void;
    setAgentCleanUi: (v: boolean) => void;
    setTtsStrategy: (strategy: 'elevenlabs' | 'openai' | 'browser' | 'qwen' | 'qwen-native') => void;
    setArtifactReviewPolicy: (policy: 'always_proceed' | 'request_review') => void;
    setTerminalAutoExecution: (policy: 'always_proceed' | 'request_review') => void;
    createAgentThread: (name: string) => string;
    closeAgentThread: (id: string) => void;
    setActiveAgentThread: (id: string) => void;
    approveArtifact: (threadId: string, artifactId: string) => void;
    rejectArtifact: (threadId: string, artifactId: string) => void;
    submitArtifactFeedback: (threadId: string, artifactId: string, feedback: string) => void;
    setActiveAgentThreadId: (id: string) => void;
    refreshProcessStats: () => Promise<void>;
    compressSessionData: (key: string, data: string) => Promise<void>;
    refreshMemorySavings: () => Promise<void>;
    fetchWorkspaceMemory: (category: string) => Promise<void>;
    fetchFileContext: (path: string) => Promise<void>;
    fetchActiveProjectSpec: () => Promise<void>;
    refreshChatSessions: () => Promise<void>;
    loadChatSession: (path: string) => Promise<void>;
    archiveCurrentSession: () => Promise<void>;
    createNewSession: () => Promise<void>;
    refreshBrainTelemetry: () => Promise<void>;
    addKairosSuggestion: (suggestion: any) => void;
    setProjectMemory: (content: string, files?: string[]) => void;
    backendPing: () => Promise<string>;
    isDemoMode: boolean;
    startDemoMode: () => void;
    endDemoMode: () => void;
    togglePlanMode: () => void;
    toggleCascadeWriteMode: () => void;
    setPendingToolPermission: (req: { id: string; tool: string; args: any; level: 'caution' | 'dangerous' } | null) => void;
    respondToolPermission: (id: string, approved: boolean) => Promise<void>;
}

export const createAgentSlice: StateCreator<AppState, [], [], AgentSlice> = (set, get) => ({
    agentMessages: [],
    isAgentThinking: false,
    isAgentPaused: false,
    isAgentBlocked: false,
    isYoloMode: false,
    isContinuousMode: false,
    isAgentVisionEnabled: (typeof localStorage !== 'undefined' && localStorage.getItem('agent.liveVision') === '1') || false,
    agentMode: (typeof localStorage !== 'undefined' && localStorage.getItem('agent.mode')) || 'Harness',
    agentModel: (() => {
        if (typeof localStorage === 'undefined') return '';
        const saved = localStorage.getItem('agentModel') || '';
        const oldDefaults = new Set(['Ollama|airi-fast:latest', 'Ollama|qwen3:35b', 'qwen3:35b', 'cyberifrit|qwen3:35b', 'cyberifrit|cyberifrit/qwen3:35b', 'huihui_ai/qwen2.5-coder-abliterate:7b', 'Ollama|huihui_ai/qwen2.5-coder-abliterate:7b']);
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
    agentCurrentAction: null,
    agentTrajectory: [],
    agentToolBlocks: [],
    isTrajectoryOpen: false,
    currentTurnId: 0,
    backgroundAgents: [],
    agentHooks: [
        { id: '1', pattern: '*.tsx', prompt: 'Check for React performance anti-patterns.', enabled: false },
        { id: '2', pattern: 'src/backend/*.rs', prompt: 'Ensure all public functions have rustdoc comments.', enabled: false },
    ],
    globalSteeringRule: '',
    pendingAgentEdits: [],
    isMultiFileReviewOpen: false,
    lastAgentCheckpoint: null,
    attachedFiles: [],
    attachedContext: [],
    agentTask: null,
    agentTasks: [],
    agentFiles: [],
    agentSteps: [],
    currentPhase: 'IDLE',
    currentPhaseStatus: 'Waiting for task...',
    currentThought: null,
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
    pendingToolPermission: null,
    ghostRuntimeResults: [],
    agentThreads: {},
    activeAgentThreadId: '',
    chatRestoreToken: 0,
    artifactReviewPolicy: 'request_review',
    activeCascadeId: null,
    terminalAutoExecution: 'request_review',
    contextSlots: [],
    activeFileContext: null,
    activeProjectSpec: null,
    projectMemory: '',
    memoryFiles: [],
    chatSessions: [],
    brainTelemetry: null,
    kairosSuggestions: [],
    kairosStatus: 'idle',
    processStats: null,
    memorySavings: null,
    isDemoMode: false,

    agentBackend: ((typeof localStorage !== 'undefined' && localStorage.getItem('agent.backend')) as 'sentient' | 'claurst') || 'sentient',
    setAgentBackend: (agentBackend) => { try { localStorage.setItem('agent.backend', agentBackend); } catch { } set({ agentBackend }); },
    setAgentMode: (agentMode) => { try { localStorage.setItem('agent.mode', agentMode); } catch { } set({ agentMode }); },
    setAgentModel: (agentModel) => { try { localStorage.setItem('agentModel', agentModel); } catch { } set({ agentModel }); },
    setPlannerModel: (plannerModel) => { try { localStorage.setItem('agent.plannerModel', plannerModel); } catch { } set({ plannerModel }); },
    setPlannerEnabled: (plannerEnabled) => { try { localStorage.setItem('agent.plannerEnabled', plannerEnabled ? '1' : '0'); } catch { } set({ plannerEnabled }); },
    setHybridAuto: (hybridAuto) => { try { localStorage.setItem('agent.hybridAuto', hybridAuto ? '1' : '0'); } catch { } set({ hybridAuto }); },
    setAgentRootAccess: (_) => set({ agentRootAccess: true }),
    setAgentCurrentAction: (agentCurrentAction) => set({ agentCurrentAction }),
    setIsAgentThinking: (isAgentThinking) => set({ isAgentThinking }),
    setIsAgentPaused: (isAgentPaused) => set({ isAgentPaused }),
    setAgentBlocked: (isAgentBlocked) => set({ isAgentBlocked }),
    setYoloMode: (isYoloMode) => set({ isYoloMode }),
    setContinuousMode: (isContinuousMode) => set({ isContinuousMode }),
    setAgentVisionEnabled: (v) => {
        try { localStorage.setItem('agent.liveVision', v ? '1' : '0'); } catch { /* */ }
        set({ isAgentVisionEnabled: v });
    },

    setAgentMessages: (agentMessages) => set({ agentMessages }),
    clearAgentMessages: () => set({ agentMessages: [] }),
    resetThread: () => {
        set({ agentMessages: [], pendingChanges: [], attachedContext: [] });
        invoke('set_ai_status', { status: 'alive' }).catch(console.error);
    },
    truncateAgentMessages: (index) => set((s) => ({ agentMessages: s.agentMessages.slice(0, index) })),

    addAgentMessage: (role, content, contextOrSubAgent) => set((state) => {
        const isSubAgent = typeof contextOrSubAgent === 'boolean' ? contextOrSubAgent : false;
        const context = Array.isArray(contextOrSubAgent) ? contextOrSubAgent : [];
        const timestamp = Date.now();

        let agentMessages = state.agentMessages;
        let agentToolBlocks = state.agentToolBlocks;
        if (role === 'user' && state.agentToolBlocks.length > 0) {
            const msgs = [...state.agentMessages];
            const last = msgs[msgs.length - 1];
            if (last?.role === 'assistant') {
                msgs[msgs.length - 1] = {
                    ...last,
                    toolBlocks: [...(last.toolBlocks || []), ...state.agentToolBlocks],
                };
            }
            agentMessages = msgs;
            agentToolBlocks = [];
        }

        const newMessage: any = { role, content, context, timestamp, isSubAgentResponse: isSubAgent, steps: role === 'assistant' ? [] : undefined };
        invoke('store_message', { role, content: typeof content === 'string' ? content : JSON.stringify(content), timestamp }).catch(console.error);
        let newMessages = [...agentMessages, newMessage];
        const cap = MAX_AGENT_MESSAGES_IN_UI;
        if (newMessages.length > cap) {
            newMessages = newMessages.slice(newMessages.length - cap).map((m, i, arr) => {
                if (i < arr.length - 6 && m.content && m.content.length > MAX_AGENT_MESSAGE_CHARS) {
                    return { ...m, content: m.content.slice(0, MAX_AGENT_MESSAGE_CHARS) + '\n\n…[trimmed]' };
                }
                return m;
            });
        }
        let threadId = state.activeAgentThreadId;
        let agentThreads = state.agentThreads;
        if (!threadId || !agentThreads[threadId]) {
            threadId = `agent-${Date.now()}`;
            agentThreads = {
                ...agentThreads,
                [threadId]: { id: threadId, name: 'Chat', messages: [], isThinking: false, tasks: [], artifacts: [] },
            };
        }
        agentThreads = {
            ...agentThreads,
            [threadId]: { ...agentThreads[threadId], messages: newMessages },
        };
        return { agentMessages: newMessages, activeAgentThreadId: threadId, agentThreads, agentToolBlocks };
    }),

    updateLastAgentMessage: (content) => set((state) => {
        const messages = [...state.agentMessages];
        const lastIndex = messages.length - 1;
        const last = messages[lastIndex];
        if (last && last.role === 'assistant') {
            const rawContent = typeof content === 'string' ? content : (content && typeof content === 'object' && (content as any).content ? (content as any).content : String(content));
            let newContent = rawContent;
            let newThoughts = last.thoughts;
            const thinkMatch = rawContent.match(/<think>([\s\S]*?)<\/think>/);
            if (thinkMatch) {
                newThoughts = thinkMatch[1].trim();
                newContent = rawContent.replace(/<think>[\s\S]*?<\/think>/, '').trim();
            } else if (rawContent.startsWith('<think>') && !rawContent.includes('</think>')) {
                newThoughts = rawContent.replace('<think>', '').trim();
                newContent = '';
            }
            const thoughtStartedAt = last.thoughtStartedAt ?? (newThoughts ? Date.now() : undefined);
            const thoughtDurationMs = newThoughts && newContent && thoughtStartedAt
                ? Date.now() - thoughtStartedAt
                : last.thoughtDurationMs;
            messages[lastIndex] = {
                ...last,
                content: newContent,
                thoughts: newThoughts || last.thoughts,
                thoughtStartedAt,
                thoughtDurationMs,
            };
        }
        const threadId = state.activeAgentThreadId;
        const agentThreads = threadId && state.agentThreads[threadId]
            ? { ...state.agentThreads, [threadId]: { ...state.agentThreads[threadId], messages } }
            : state.agentThreads;
        return { agentMessages: messages, agentThreads };
    }),

    appendLastAgentMessage: (delta) => set((state: any) => {
        const messages = [...state.agentMessages];
        const lastIndex = messages.length - 1;
        const last = messages[lastIndex];
        if (last && last.role === 'assistant') {
            const currentContent = last.content || '';
            const currentThoughts = last.thoughts || '';
            const currentRaw = last.raw_buffer || (currentContent + currentThoughts);
            if (delta.length > 0 && currentRaw.endsWith(delta)) return state;
            const fullRaw = currentRaw + delta;
            let newContent = currentContent;
            let newThoughts = currentThoughts;
            if (fullRaw.includes('<think>')) {
                const thinkMatch = fullRaw.match(/<think>([\s\S]*?)<\/think>/);
                if (thinkMatch) {
                    const thoughtText = thinkMatch[1].trim();
                    if (thoughtText) set({ currentThought: parseThought(thoughtText) });
                    newContent = fullRaw.replace(/<think>[\s\S]*?<\/think>/g, '').trim();
                } else {
                    const parts = fullRaw.split('<think>');
                    newContent = parts[0] || '';
                    const partialThought = parts[1] || '';
                    if (partialThought) set({ currentThought: parseThought(partialThought.trim()) });
                }
            } else {
                if (fullRaw.includes('</think>')) {
                    newContent = fullRaw.split('</think>').pop()?.trim() || '';
                } else {
                    newContent += delta;
                }
            }
            messages[lastIndex] = { ...last, content: newContent, thoughts: newThoughts, raw_buffer: fullRaw,
                thoughtStartedAt: last.thoughtStartedAt ?? (newThoughts ? Date.now() : undefined),
                thoughtDurationMs: newThoughts && newContent && last.thoughtStartedAt
                    ? Date.now() - last.thoughtStartedAt
                    : last.thoughtDurationMs,
            };
            const threadId = state.activeAgentThreadId;
            const agentThreads = threadId && state.agentThreads[threadId]
                ? { ...state.agentThreads, [threadId]: { ...state.agentThreads[threadId], messages } }
                : state.agentThreads;
            return { agentMessages: messages, agentThreads };
        }
        return state;
    }),

    updateLastAgentThought: (thought) => set((state) => {
        const messages = [...state.agentMessages];
        const last = messages[messages.length - 1];
        if (last?.role === 'assistant') {
            messages[messages.length - 1] = {
                ...last,
                thoughts: thought,
                thoughtStartedAt: last.thoughtStartedAt ?? Date.now(),
            };
        }
        return { agentMessages: messages, currentThought: parseThought(thought) };
    }),

    addAgentStep: (name, type, args, callId) => set((state) => {
        const messages = [...state.agentMessages];
        if (messages.length === 0) return state;
        const last = messages[messages.length - 1];
        if (last && last.role === 'assistant') {
            const steps = last.steps || [];
            if (!steps.find((s: any) => (callId && s.callId === callId) || (!callId && s.name === name && s.status === 'running'))) {
                last.steps = [...steps, { name, status: 'running', type, args, callId }];
            }
        }
        return { agentMessages: messages };
    }),

    updateAgentStepStatus: (name, status, result, summary, callId) => set((state) => {
        const messages = [...state.agentMessages];
        if (messages.length === 0) return state;
        const last = messages[messages.length - 1];
        if (last && last.role === 'assistant' && last.steps) {
            const step = last.steps.find((s: any) => (callId && s.callId === callId) || (!callId && s.name === name && s.status === 'running'));
            if (step) {
                step.status = status;
                if (result !== undefined) step.result = result;
                if (summary !== undefined) step.summary = summary;
            }
        }
        return { agentMessages: messages };
    }),

    addAgentFile: (path) => {
        set((state) => {
            const last = state.agentMessages[state.agentMessages.length - 1];
            if (last && last.role === 'assistant') {
                const files = last.files || [];
                if (!files.includes(path)) {
                    const msgs = [...state.agentMessages];
                    msgs[msgs.length - 1] = { ...last, files: [...files, path] };
                    return { agentMessages: msgs };
                }
            }
            return state;
        });
    },

    addAgentArtifact: (art) => {
        set((state) => {
            const artifact: Artifact = { ...art, id: Math.random().toString(36).substring(7), timestamp: Date.now() };
            const last = state.agentMessages[state.agentMessages.length - 1];
            if (last && last.role === 'assistant') {
                const artifacts = last.artifacts || [];
                if (!artifacts.find((a: any) => a.path === artifact.path)) {
                    const msgs = [...state.agentMessages];
                    msgs[msgs.length - 1] = { ...last, artifacts: [...artifacts, artifact] };
                    const taskArtifact: TaskArtifact = {
                        id: artifact.id,
                        title: artifact.title || 'Artifact',
                        path: artifact.path,
                        content: (art as any).content || artifact.description || '',
                        timestamp: artifact.timestamp,
                        feedback: artifact.feedback,
                        type: artifact.type,
                        metadata: artifact.metadata as any
                    };
                    const currentTask = state.agentTask ? { ...state.agentTask, artifacts: [...state.agentTask.artifacts, taskArtifact], updatedAt: Date.now() } : null;
                    return { agentMessages: msgs, agentTask: currentTask };
                }
            }
            return state;
        });
    },

    pushTrajectoryEvent: (evt) => {
        set((s) => ({
            agentTrajectory: [...s.agentTrajectory.slice(-199), { id: `tr-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`, ts: Date.now(), turn: s.currentTurnId, ...evt }],
        }));
        const st = get();
        if (st.activeRoot && st.activeCascadeId) {
            import('../infrastructure/antigravity/antigravityClient').then(m =>
                m.persistAgentTrajectoryEvent(st.activeRoot!, st.activeCascadeId!, {
                    kind: evt.kind,
                    title: evt.title,
                    detail: evt.detail,
                    tool: evt.tool,
                    success: evt.success,
                }),
            ).catch(() => {});
        }
    },
    clearTrajectory: () => set({ agentTrajectory: [] }),
    clearAgentToolBlocks: () => set({ agentToolBlocks: [] }),

    registerAgentToolCall: (tool, args, callId) => set((state) => {
        if (callId && state.agentToolBlocks.some((b) => b.id === callId)) return state;
        const block = createToolBlock(tool, args, callId);
        // Skip duplicate recon cards within 1.5s (glob/list spam)
        if (block.kind === 'search' || block.kind === 'read') {
            const dup = [...state.agentToolBlocks].reverse().find((b) =>
                b.title === block.title && b.tool === block.tool && Date.now() - b.ts < 1500,
            );
            if (dup) return state;
        }
        return { agentToolBlocks: [...state.agentToolBlocks.slice(-24), block] };
    }),

    appendAgentToolOutput: (streamId, line, stream) => set((state) => ({
        agentToolBlocks: state.agentToolBlocks.map((b) => {
            if (b.streamId !== streamId) return b;
            const prefix = stream === 'stderr' ? '[stderr] ' : '';
            return { ...b, outputLines: [...b.outputLines, prefix + line].slice(-40) };
        }),
    })),

    bindAgentToolStream: (streamId, command) => set((state) => {
        let matched = false;
        const blocks = state.agentToolBlocks.map((b) => {
            if (matched || b.kind !== 'terminal' || b.status !== 'running') return b;
            if (b.streamId && b.streamId !== streamId) return b;
            matched = true;
            return {
                ...b,
                streamId,
                command: command || b.command,
                title: command ? (command.length > 72 ? command.slice(0, 72) + '…' : command) : b.title,
            };
        });
        if (!matched) {
            blocks.push({
                id: `tb-stream-${streamId}`,
                kind: 'terminal',
                tool: 'run_command',
                title: command.length > 72 ? command.slice(0, 72) + '…' : command,
                status: 'running',
                ts: Date.now(),
                streamId,
                command,
                outputLines: [],
            });
        }
        return { agentToolBlocks: blocks.slice(-24) };
    }),

    finishAgentToolCall: (tool, success, result, streamId, callId) => set((state) => {
        const blocks = state.agentToolBlocks.map((b) => {
            const match = streamId
                ? b.streamId === streamId
                : callId
                    ? b.id === callId
                    : toolsMatchForFinish(b.tool, tool) && b.status === 'running';
            if (!match) return b;
            let next = { ...b, status: success ? 'done' as const : 'error' as const };
            if (result && b.kind === 'edit') {
                const enriched = enrichEditBlockFromResult(next, result);
                next = {
                    ...enriched,
                    status: success ? 'done' as const : 'error' as const,
                    preview: enriched.preview || (result.length < 500 ? result.slice(0, 400) : enriched.preview),
                };
            }
            return next;
        });
        return { agentToolBlocks: blocks };
    }),

    finalizeAgentToolBlocks: () => set((state) => {
        if (!state.agentToolBlocks.length) return state;
        const msgs = [...state.agentMessages];
        const lastIdx = msgs.length - 1;
        const last = msgs[lastIdx];
        if (last?.role === 'assistant') {
            msgs[lastIdx] = {
                ...last,
                toolBlocks: [...(last.toolBlocks || []), ...state.agentToolBlocks],
            };
            const threadId = state.activeAgentThreadId;
            const agentThreads = threadId && state.agentThreads[threadId]
                ? {
                    ...state.agentThreads,
                    [threadId]: { ...state.agentThreads[threadId], messages: msgs },
                }
                : state.agentThreads;
            return { agentMessages: msgs, agentThreads, agentToolBlocks: [] };
        }
        return { agentToolBlocks: [] };
    }),

    openTrajectory: () => set({ isTrajectoryOpen: true }),
    closeTrajectory: () => set({ isTrajectoryOpen: false }),
    beginNewTurn: () => set((s) => ({ currentTurnId: s.currentTurnId + 1 })),

    addPendingAgentEdit: (edit) => set((s) => {
        const existing = s.pendingAgentEdits.find(e => e.path === edit.path);
        if (existing) {
            return { pendingAgentEdits: s.pendingAgentEdits.map(e => e.path === edit.path ? { ...e, tool: edit.tool, timestamp: Date.now(), preview: edit.preview ?? e.preview } : e) };
        }
        return { pendingAgentEdits: [...s.pendingAgentEdits, { ...edit, timestamp: Date.now() }] };
    }),
    clearPendingAgentEdits: () => set({ pendingAgentEdits: [] }),
    openMultiFileReview: () => set({ isMultiFileReviewOpen: true }),
    closeMultiFileReview: () => set({ isMultiFileReviewOpen: false, pendingAgentEdits: [] }),
    setLastAgentCheckpoint: (cp) => set({ lastAgentCheckpoint: cp }),

    rollbackLastAgentCheckpoint: async () => {
        const { lastAgentCheckpoint } = get();
        if (!lastAgentCheckpoint) return { ok: false, message: 'No checkpoint' };
        try {
            await invoke('git_rollback_checkpoint', { checkpointId: lastAgentCheckpoint.id });
            set({ lastAgentCheckpoint: null });
            get().refreshFileTree();
            return { ok: true, message: 'Restored' };
        } catch (e) {
            return { ok: false, message: String(e) };
        }
    },

    setLastUserMessageCheckpoint: (checkpointId, description?) => set((state) => {
        const messages = [...state.agentMessages];
        for (let i = messages.length - 1; i >= 0; i--) {
            if (messages[i].role === 'user') {
                if (messages[i].checkpointId === checkpointId) break;
                messages[i] = { ...messages[i], checkpointId, checkpointDescription: description };
                break;
            }
        }
        return { agentMessages: messages };
    }),

    restoreToMessageCheckpoint: async (timestamp) => {
        const state = get();
        const idx = state.agentMessages.findIndex((m: any) => m.timestamp === timestamp);
        if (idx < 0) return { ok: false, message: 'Message not found in chat history.' };
        const target: any = state.agentMessages[idx];
        if (!target?.checkpointId) return { ok: false, message: 'This message has no checkpoint attached.' };
        try {
            const message = await invoke<string>('git_rollback_checkpoint', { checkpointId: target.checkpointId });
            set({ agentMessages: state.agentMessages.slice(0, idx), lastAgentCheckpoint: null });
            return { ok: true, message };
        } catch (e: any) {
            return { ok: false, message: String(e?.message ?? e) };
        }
    },

    runBackgroundAgent: async (prompt) => {
        const id = `bg-${Date.now()}-${Math.floor(Math.random() * 1000)}`;
        set((s) => ({ backgroundAgents: [...s.backgroundAgents, { id, prompt, status: 'running', result: '', startedAt: Date.now() }] }));
        try {
            const { ensureAgentRuntime } = await import('../application/performance/ensureAgentRuntime');
            await ensureAgentRuntime();
            const state = get();
            const provider = state.agentModel.includes('|') ? state.agentModel.split('|')[0] : 'ollama';
            const model = state.agentModel.includes('|') ? state.agentModel.split('|')[1] : state.agentModel;
            const resultText = await invoke<string>('ai_chat_oneshot', {
                request: {
                    provider, model,
                    messages: [
                        { role: 'system', content: 'You are a persistent autonomous coding agent inside VSCodium-Rust. Execute tools, inspect the workspace, write fixes, run verification, and continue until the requested goal is actually finished.' },
                        { role: 'user', content: prompt },
                    ],
                    autonomous: true, root_access: true,
                    mode: state.agentMode === 'Chat' ? 'Agent' : state.agentMode,
                    ollama_url: state.ollamaUrl,
                },
            });
            set((s) => ({ backgroundAgents: s.backgroundAgents.map(b => b.id === id ? { ...b, status: 'done', result: resultText, finishedAt: Date.now() } : b) }));
        } catch (e: any) {
            set((s) => ({ backgroundAgents: s.backgroundAgents.map(b => b.id === id ? { ...b, status: 'error', result: String(e?.message ?? e), finishedAt: Date.now() } : b) }));
        }
        return id;
    },
    removeBackgroundAgent: (id) => set((s) => ({ backgroundAgents: s.backgroundAgents.filter(b => b.id !== id) })),
    clearBackgroundAgents: () => set({ backgroundAgents: [] }),
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

    attachFile: (file) => set((state: any) => {
        const files = Array.isArray(file) ? file : [file];
        const newAttached = [...state.attachedFiles];
        for (const f of files) {
            if (!newAttached.find((e: any) => e.path === f.path)) newAttached.push({ ...f, type: f.type || 'file' });
        }
        return { attachedFiles: newAttached };
    }),
    removeFile: (path) => set((state: any) => ({ attachedFiles: state.attachedFiles.filter((f: any) => f.path !== path) })),
    clearAttachedFiles: () => set({ attachedFiles: [] }),
    addAttachedContext: (item) => set((state: any) => ({ attachedContext: [...state.attachedContext, item] })),
    removeAttachedContext: (index) => set((state: any) => ({ attachedContext: state.attachedContext.filter((_: any, i: number) => i !== index) })),
    clearAttachedContext: () => set({ attachedContext: [] }),

    updateAgentTask: (taskUpdate) => set((state) => {
        const existingTasks = [...state.agentTasks];
        const index = existingTasks.findIndex(t => t.id === taskUpdate.id);
        let updatedTask: AgentTask;
        if (index > -1) {
            existingTasks[index] = { ...existingTasks[index], ...taskUpdate, updatedAt: Date.now() } as AgentTask;
            updatedTask = existingTasks[index];
        } else {
            updatedTask = { id: taskUpdate.id, title: taskUpdate.title || 'Agent Task', summary: taskUpdate.summary || '', status: (taskUpdate.status as any) || 'running', progress: taskUpdate.progress || 0, createdAt: Date.now(), updatedAt: Date.now(), artifacts: [] };
            existingTasks.push(updatedTask);
        }
        return { agentTasks: existingTasks, agentTask: updatedTask };
    }),
    setAgentTask: (agentTask) => set({ agentTask }),
    setAgentTasks: (agentTasks) => set({ agentTasks }),
    setAgentFiles: (agentFiles) => set({ agentFiles }),
    setAgentSteps: (agentSteps) => set({ agentSteps }),
    setPhase: (phase, status) => set({ currentPhase: phase, currentPhaseStatus: status }),

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
                const mod = await import('../airi/vision-system');
                if (enabled) await mod.airiVision.start();
                else if (typeof (mod.airiVision as any).stop === 'function') (mod.airiVision as any).stop();
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
                    const { visionAnalyzer } = await import('../airi/vision-analysis');
                    if (typeof (visionAnalyzer as any).reconfigure === 'function') (visionAnalyzer as any).reconfigure({ model });
                } catch { }
            })();
        }
    },
    setAiriConsciousnessEnabled: (enabled) => {
        try { localStorage.setItem('airi.consciousness.enabled', enabled ? '1' : '0'); } catch { }
        set({ airiConsciousnessEnabled: enabled });
        (async () => {
            try {
                const { airiConsciousness } = await import('../airi/consciousness');
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
                const { airiConsciousness } = await import('../airi/consciousness');
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

    createAgentThread: (name) => {
        const id = `agent-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
        set((state: any) => ({
            agentThreads: { ...state.agentThreads, [id]: { id, name, messages: [], isThinking: false, tasks: [], artifacts: [] } },
            activeAgentThreadId: id,
            agentMessages: [],
        }));
        return id;
    },
    closeAgentThread: (id) => set((state: any) => {
        const keys = Object.keys(state.agentThreads || {});
        if (keys.length === 0) return state;
        if (keys.length === 1) {
            const only = keys[0];
            return {
                agentThreads: {
                    ...state.agentThreads,
                    [only]: { ...state.agentThreads[only], messages: [], isThinking: false, tasks: [], artifacts: [] },
                },
                agentMessages: [],
                agentTasks: [],
            };
        }
        const { [id]: _removed, ...rest } = state.agentThreads;
        const remaining = Object.keys(rest);
        if (remaining.length === 0) {
            return { agentThreads: {}, activeAgentThreadId: '', agentMessages: [], agentTasks: [] };
        }
        const nextId = state.activeAgentThreadId === id
            ? remaining[remaining.length - 1]
            : state.activeAgentThreadId;
        const next = rest[nextId];
        return {
            agentThreads: rest,
            activeAgentThreadId: nextId,
            agentMessages: next?.messages || [],
            agentTasks: next?.tasks || [],
        };
    }),
    setActiveAgentThread: (id) => {
        set((state: any) => {
            const prevId = state.activeAgentThreadId;
            let threads = state.agentThreads;
            if (prevId && threads[prevId] && prevId !== id) {
                threads = {
                    ...threads,
                    [prevId]: { ...threads[prevId], messages: state.agentMessages },
                };
            }
            const thread = threads[id];
            if (!thread) return state;
            return {
                agentThreads: threads,
                activeAgentThreadId: id,
                agentMessages: thread.messages,
                agentTasks: thread.tasks,
            };
        });
    },
    setActiveAgentThreadId: (activeAgentThreadId) => set({ activeAgentThreadId }),
    approveArtifact: (threadId, artifactId) => set((state: any) => {
        const thread = state.agentThreads[threadId];
        if (!thread) return state;
        return { agentThreads: { ...state.agentThreads, [threadId]: { ...thread, artifacts: thread.artifacts.map((a: any) => a.id === artifactId ? { ...a, metadata: { ...a.metadata, reviewed: true, status: 'approved' } } : a) } } };
    }),
    rejectArtifact: (threadId, artifactId) => set((state: any) => {
        const thread = state.agentThreads[threadId];
        if (!thread) return state;
        return { agentThreads: { ...state.agentThreads, [threadId]: { ...thread, artifacts: thread.artifacts.map((a: any) => a.id === artifactId ? { ...a, metadata: { ...a.metadata, reviewed: true, status: 'rejected' } } : a) } } };
    }),
    submitArtifactFeedback: (threadId, artifactId, feedback) => set((state: any) => {
        const thread = state.agentThreads[threadId];
        if (!thread) return state;
        return { agentThreads: { ...state.agentThreads, [threadId]: { ...thread, artifacts: thread.artifacts.map((a: any) => a.id === artifactId ? { ...a, feedback } : a) } } };
    }),

    refreshProcessStats: async () => {
        const { refreshProcessMemory } = await import('../application/performance/refreshProcessMemory');
        const stats = await refreshProcessMemory();
        if (stats) set({ processStats: stats });
    },
    compressSessionData: async (key, data) => {
        try { await invoke('compress_session_data', { key, data }); get().refreshMemorySavings(); } catch { }
    },
    refreshMemorySavings: async () => {
        try { const [original, compressed] = await invoke<[number, number]>('get_memory_savings'); set({ memorySavings: { original, compressed } }); } catch { }
    },

    fetchWorkspaceMemory: async (category) => {
        try { const slots = await invoke<SemanticSlot[]>('query_workspace_memory', { category }); set({ contextSlots: slots }); } catch { }
    },
    fetchFileContext: async (path) => {
        try { const context = await invoke<any>('get_file_context', { filePath: path }); set({ activeFileContext: context }); } catch { }
    },
    fetchActiveProjectSpec: async () => {
        try {
            const projects: any = await invoke('cmd_specs_get_projects');
            const activeRoot = get().activeRoot;
            if (activeRoot && projects) {
                const activeSpec = projects.find((p: any) => p.root_path === activeRoot || p.path === activeRoot);
                set({ activeProjectSpec: activeSpec || null });
            }
        } catch { }
    },

    setProjectMemory: (content, files = []) => set({ projectMemory: content, memoryFiles: files }),

    refreshChatSessions: async () => {
        try {
            const { syncAgentMessagesToBackend } = await import('../application/agent/syncAgentMessages');
            await syncAgentMessagesToBackend().catch(() => {});
            const sessions = await invoke<any[]>('list_chat_sessions');
            set({ chatSessions: sessions });
        } catch { }
    },
    loadChatSession: async (path) => {
        try {
            // Stop any in-flight agent turn — restore is read-only; YOLO must not keep executing.
            try {
                const { stopAgent } = await import('../application/agent/stopAgent');
                await stopAgent();
            } catch { /* ignore */ }

            const state = get();
            const existingId = Object.values(state.agentThreads || {}).find(
                (t: any) => t.chatSessionPath === path,
            )?.id;

            // Always reload from disk → live memory.aim (fixes empty UI + missing AI context).
            const raw = await invoke<unknown>('load_chat_session', { path });
            let messages = mapBackendChatMessages(raw);
            if (messages.length === 0) {
                const fallback = await invoke<unknown>('get_agent_messages');
                messages = mapBackendChatMessages(fallback);
            }

            if (messages.length === 0) {
                console.warn('[loadChatSession] no messages for', path);
                set({
                    isAgentThinking: false,
                    isAgentPaused: false,
                    agentCurrentAction: null,
                    agentTrajectory: [],
                    agentSteps: [],
                    agentToolBlocks: [],
                    chatRestoreToken: Date.now(),
                });
                return;
            }

            const firstUser = messages.find((m) => m.role === 'user');
            const title = firstUser?.content?.slice(0, 48)?.trim() || 'Restored chat';

            const restoredState = {
                isAgentThinking: false,
                isAgentPaused: false,
                agentCurrentAction: null,
                agentTrajectory: [],
                agentSteps: [],
                agentToolBlocks: [],
                chatRestoreToken: Date.now(),
                agentMessages: messages,
            };

            if (existingId) {
                set((s: any) => ({
                    ...restoredState,
                    activeAgentThreadId: existingId,
                    agentThreads: {
                        ...s.agentThreads,
                        [existingId]: {
                            ...s.agentThreads[existingId],
                            name: title,
                            messages,
                            isThinking: false,
                            chatSessionPath: path,
                        },
                    },
                }));
            } else {
                const threadId = `restored-${Date.now()}`;
                set((s: any) => ({
                    ...restoredState,
                    activeAgentThreadId: threadId,
                    agentThreads: {
                        ...s.agentThreads,
                        [threadId]: {
                            id: threadId,
                            name: title,
                            messages,
                            isThinking: false,
                            tasks: [],
                            artifacts: [],
                            chatSessionPath: path,
                        },
                    },
                }));
            }

            const { syncAgentMessagesToBackend } = await import('../application/agent/syncAgentMessages');
            await syncAgentMessagesToBackend().catch(() => {});
            get().refreshChatSessions();
        } catch (e) { console.error('[loadChatSession] failed:', e); }
    },
    archiveCurrentSession: async () => {
        try {
            const { syncAgentMessagesToBackend } = await import('../application/agent/syncAgentMessages');
            await syncAgentMessagesToBackend();
            await invoke('archive_chat_session');
            get().refreshChatSessions();
        } catch (e) { console.error('[archiveCurrentSession]', e); }
    },
    createNewSession: async () => {
        try {
            const { syncAgentMessagesToBackend } = await import('../application/agent/syncAgentMessages');
            await syncAgentMessagesToBackend();
            await invoke('archive_chat_session').catch(() => {});
        } catch { /* */ }
        const count = Object.keys(get().agentThreads || {}).length;
        get().createAgentThread(`Chat ${count + 1}`);
        try {
            await invoke('create_new_session');
            await invoke('clear_ai_memory').catch(() => {});
            get().refreshChatSessions();
        } catch (e) { console.error('[createNewSession] failed:', e); }
    },
    refreshBrainTelemetry: async () => {
        try { const telemetry = await invoke<any>('get_brain_telemetry'); set({ brainTelemetry: telemetry }); } catch { }
    },
    addKairosSuggestion: (suggestion) => set((state: any) => ({ kairosSuggestions: [suggestion, ...state.kairosSuggestions].slice(0, 50) })),

    backendPing: async () => {
        try { return await invoke<string>('backend_ping'); } catch (error) { return `Error: ${error}`; }
    },

    startDemoMode: () => set({ isDemoMode: true }),
    endDemoMode: () => set({ isDemoMode: false }),

    togglePlanMode: () => set((s) => ({ isPlanMode: !s.isPlanMode })),
    toggleCascadeWriteMode: () => set((s) => {
        const next = !s.isCascadeWriteMode;
        try { localStorage.setItem('agent.cascadeWrite', next ? '1' : '0'); } catch { /* */ }
        return { isCascadeWriteMode: next };
    }),

    setPendingToolPermission: (req) => set({ pendingToolPermission: req }),

    respondToolPermission: async (id, approved) => {
        set({ pendingToolPermission: null });
        try { await invoke('respond_tool_permission', { toolId: id, approved }); } catch { /* ignore */ }
    },
});

