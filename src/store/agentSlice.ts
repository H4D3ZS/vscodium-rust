import type { StateCreator } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import type { AppState } from './index';
import type {
    AgentMessage, AgentStep, Artifact, AttachedContext, AgentTask, TaskArtifact, SemanticSlot,
} from './types';

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

export interface AgentSlice {
    // State
    agentMessages: AgentMessage[];
    isAgentThinking: boolean;
    isAgentPaused: boolean;
    isAgentBlocked: boolean;
    isYoloMode: boolean;
    isContinuousMode: boolean;
    agentMode: string;
    agentModel: string;
    agentRootAccess: boolean;
    agentCurrentAction: string | null;
    agentTrajectory: {
        id: string; ts: number;
        kind: 'tool_call' | 'tool_result' | 'content' | 'phase' | 'error';
        tool?: string; title: string; detail?: string; success?: boolean; turn?: number;
    }[];
    isTrajectoryOpen: boolean;
    currentTurnId: number;
    backgroundAgents: { id: string; prompt: string; status: 'pending' | 'running' | 'done' | 'error'; result: string; startedAt: number; finishedAt?: number }[];
    agentHooks: { id: string; pattern: string; prompt: string; enabled: boolean }[];
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
    ttsStrategy: 'elevenlabs' | 'openai' | 'browser' | 'qwen' | 'qwen-native';
    airiVisionEnabled: boolean;
    airiVisionModel: string;
    airiConsciousnessEnabled: boolean;
    airiConsciousnessModel: string;
    taskPlannerState: any | null;
    isPlanMode: boolean;
    isCascadeWriteMode: boolean;
    pendingToolPermission: { id: string; tool: string; args: any; level: 'caution' | 'dangerous' } | null;
    ghostRuntimeResults: any[];
    agentThreads: Record<string, { id: string; name: string; messages: any[]; isThinking: boolean; tasks: any[]; artifacts: any[] }>;
    activeAgentThreadId: string;
    artifactReviewPolicy: 'always_proceed' | 'request_review';
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
    processStats: { memory_mb: number; cpu_usage: number; total_ram_gb: number; available_ram_gb: number } | null;
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
    setAgentMode: (mode: string) => void;
    setAgentModel: (model: string) => void;
    setAgentRootAccess: (v: boolean) => void;
    setAgentCurrentAction: (action: string | null) => void;
    setAgentMessages: (messages: any[]) => void;
    clearAgentMessages: () => void;
    resetThread: () => void;
    truncateAgentMessages: (index: number) => void;
    pushTrajectoryEvent: (evt: { kind: 'tool_call' | 'tool_result' | 'content' | 'phase' | 'error'; tool?: string; title: string; detail?: string; success?: boolean }) => void;
    clearTrajectory: () => void;
    openTrajectory: () => void;
    closeTrajectory: () => void;
    beginNewTurn: () => void;
    runBackgroundAgent: (prompt: string) => Promise<string>;
    removeBackgroundAgent: (id: string) => void;
    clearBackgroundAgents: () => void;
    setAgentHooks: (hooks: { id: string; pattern: string; prompt: string; enabled: boolean }[]) => void;
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
    setTtsStrategy: (strategy: 'elevenlabs' | 'openai' | 'browser' | 'qwen' | 'qwen-native') => void;
    setArtifactReviewPolicy: (policy: 'always_proceed' | 'request_review') => void;
    setTerminalAutoExecution: (policy: 'always_proceed' | 'request_review') => void;
    createAgentThread: (name: string) => string;
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
    agentMode: (typeof localStorage !== 'undefined' && localStorage.getItem('agent.mode')) || 'Harness',
    agentModel: (() => {
        if (typeof localStorage === 'undefined') return '';
        const saved = localStorage.getItem('agentModel') || '';
        const oldDefaults = new Set(['Ollama|airi-fast:latest', 'Ollama|qwen3:35b', 'qwen3:35b', 'huihui_ai/qwen2.5-coder-abliterate:7b', 'Ollama|huihui_ai/qwen2.5-coder-abliterate:7b']);
        if (oldDefaults.has(saved)) { localStorage.removeItem('agentModel'); return ''; }
        return saved;
    })(),
    agentRootAccess: true,
    agentCurrentAction: null,
    agentTrajectory: [],
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
    ttsStrategy: (localStorage.getItem('ttsStrategy') as any) || 'elevenlabs',
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
    isCascadeWriteMode: true, // ON by default — agent writes stream directly to editor
    pendingToolPermission: null,
    ghostRuntimeResults: [],
    agentThreads: {},
    activeAgentThreadId: '',
    artifactReviewPolicy: 'request_review',
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

    setAgentMode: (agentMode) => { try { localStorage.setItem('agent.mode', agentMode); } catch { } set({ agentMode }); },
    setAgentModel: (agentModel) => { try { localStorage.setItem('agentModel', agentModel); } catch { } set({ agentModel }); },
    setAgentRootAccess: (_) => set({ agentRootAccess: true }),
    setAgentCurrentAction: (agentCurrentAction) => set({ agentCurrentAction }),
    setIsAgentThinking: (isAgentThinking) => set({ isAgentThinking }),
    setIsAgentPaused: (isAgentPaused) => set({ isAgentPaused }),
    setAgentBlocked: (isAgentBlocked) => set({ isAgentBlocked }),
    setYoloMode: (isYoloMode) => set({ isYoloMode }),
    setContinuousMode: (isContinuousMode) => set({ isContinuousMode }),

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
        const newMessage: any = { role, content, context, timestamp, isSubAgentResponse: isSubAgent, steps: role === 'assistant' ? [] : undefined };
        invoke('store_message', { role, content: typeof content === 'string' ? content : JSON.stringify(content), timestamp }).catch(console.error);
        let newMessages = [...state.agentMessages, newMessage];
        if (newMessages.length > 100) newMessages = newMessages.slice(newMessages.length - 100);
        return { agentMessages: newMessages };
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
            messages[lastIndex] = { ...last, content: newContent, thoughts: newThoughts };
        }
        return { agentMessages: messages };
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
            messages[lastIndex] = { ...last, content: newContent, thoughts: newThoughts, raw_buffer: fullRaw };
            return { agentMessages: messages };
        }
        return state;
    }),

    updateLastAgentThought: (thought) => { set({ currentThought: parseThought(thought) }); },

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

    pushTrajectoryEvent: (evt) => set((s) => ({
        agentTrajectory: [...s.agentTrajectory.slice(-199), { id: `tr-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`, ts: Date.now(), turn: s.currentTurnId, ...evt }],
    })),
    clearTrajectory: () => set({ agentTrajectory: [] }),
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
    setTtsStrategy: (strategy) => {
        localStorage.setItem('ttsStrategy', strategy);
        set({ ttsStrategy: strategy });
        import('../voice').then(({ setProvider }) => setProvider(strategy)).catch(() => { });
    },
    setArtifactReviewPolicy: (policy) => set({ artifactReviewPolicy: policy }),
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
    setActiveAgentThread: (id) => {
        set((state: any) => {
            const thread = state.agentThreads[id];
            if (!thread) return state;
            return { activeAgentThreadId: id, agentMessages: thread.messages, agentTasks: thread.tasks };
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
        try { const stats = await invoke<any>('get_process_stats'); set({ processStats: stats }); } catch { }
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
        try { const sessions = await invoke<any[]>('list_chat_sessions'); set({ chatSessions: sessions }); } catch { }
    },
    loadChatSession: async (path) => {
        try {
            await invoke('load_chat_session', { path });
            const messages = await invoke<any[]>('get_agent_messages');
            set({ agentMessages: messages });
            get().refreshChatSessions();
        } catch { }
    },
    archiveCurrentSession: async () => {
        try { await invoke('archive_chat_session'); get().refreshChatSessions(); } catch { }
    },
    createNewSession: async () => {
        try { await invoke('create_new_session'); get().clearAgentMessages(); get().refreshChatSessions(); } catch { }
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
    toggleCascadeWriteMode: () => set((s) => ({ isCascadeWriteMode: !s.isCascadeWriteMode })),

    setPendingToolPermission: (req) => set({ pendingToolPermission: req }),

    respondToolPermission: async (id, approved) => {
        set({ pendingToolPermission: null });
        try { await invoke('respond_tool_permission', { toolId: id, approved }); } catch { /* ignore */ }
    },
});

