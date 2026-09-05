// Agent tools slice: tool blocks, trajectory, steps, background agents, permissions, telemetry.
// Split from agentSlice.ts; combined in the agentSlice.ts barrel.

import type { StateCreator } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import { MAX_AGENT_MESSAGES_IN_UI, MAX_AGENT_MESSAGE_CHARS } from '../domain/agent/AgentSessionPolicy';
import type { AppState } from './index';
import type {
    AgentMessage, AgentStep, Artifact, AttachedContext, AgentTask, TaskArtifact, SemanticSlot,
} from './types';
import type { AgentToolBlock } from '../domain/agent/agentToolBlocks';
import { createToolBlock, enrichCanvasBlockFromResult, enrichEditBlockFromResult, enrichExploreBlockFromResult } from '../domain/agent/agentToolBlocks';
import { toolsMatchForFinish } from '../domain/agent/toolAliases';
import { cleanAgentContent, shouldReplaceAgentContent } from '../domain/agent/cleanAgentContent';
import { onAgentModeChanged } from '../lib/agentAutonomy';
import {
    boundedPush, boundedTail,
    MAX_AGENT_STEPS_PER_MSG, MAX_TRAJECTORY_EVENTS, MAX_TOOL_BLOCKS, MAX_TOOL_OUTPUT_LINES, MAX_COMPLETED_TASKS,
} from '../domain/utils/boundedArray';
import { type CustomMode, loadCustomModes, parseThought, normalizeBackendMessages, mapBackendChatMessages } from './agentSliceShared';

export interface AgentToolsSlice {
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
    agentTask: AgentTask | null;
    agentTasks: any[];
    agentFiles: string[];
    agentSteps: any[];
    currentPhase: 'ANALYZE' | 'PLAN' | 'EXECUTE' | 'VERIFY' | 'REPORT' | 'IDLE';
    currentPhaseStatus: string;
    currentThought: { logic: string; action: string; confidence?: number } | null;
    pendingToolPermission: { id: string; tool: string; args: any; level: 'caution' | 'dangerous' } | null;
    ghostRuntimeResults: any[];
    brainTelemetry: any | null;
    kairosSuggestions: any[];
    kairosStatus: 'idle' | 'indexing' | 'dreaming';
    processStats: import('../domain/performance/ProcessMemorySnapshot').ProcessStatsDto | null;
    memorySavings: { original: number; compressed: number } | null;

    // Actions
    addAgentStep: (name: string, type?: AgentStep['type'], args?: any, callId?: string) => void;
    updateAgentStepStatus: (name: string, status: 'running' | 'success' | 'error', result?: string, summary?: string, callId?: string) => void;
    addAgentFile: (path: string) => void;
    setAgentCurrentAction: (action: string | null) => void;
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
    updateAgentTask: (task: Partial<AgentTask> & { id: string }) => void;
    setAgentTask: (task: AgentTask | null) => void;
    setAgentTasks: (tasks: any[]) => void;
    setAgentFiles: (files: string[]) => void;
    setAgentSteps: (steps: AgentStep[]) => void;
    refreshProcessStats: () => Promise<void>;
    refreshMemorySavings: () => Promise<void>;
    refreshBrainTelemetry: () => Promise<void>;
    addKairosSuggestion: (suggestion: any) => void;
    setPendingToolPermission: (req: { id: string; tool: string; args: any; level: 'caution' | 'dangerous' } | null) => void;
    respondToolPermission: (id: string, approved: boolean) => Promise<void>;
}

export const createAgentToolsSlice: StateCreator<AppState, [], [], AgentToolsSlice> = (set, get) => ({
    agentCurrentAction: null,
    agentTrajectory: [],
    agentToolBlocks: [],
    isTrajectoryOpen: false,
    currentTurnId: 0,
    backgroundAgents: [],
    agentTask: null,
    agentTasks: [],
    agentFiles: [],
    agentSteps: [],
    currentPhase: 'IDLE',
    currentPhaseStatus: 'Waiting for task...',
    currentThought: null,
    pendingToolPermission: null,
    ghostRuntimeResults: [],
    brainTelemetry: null,
    kairosSuggestions: [],
    kairosStatus: 'idle',
    processStats: null,
    memorySavings: null,
    setAgentCurrentAction: (agentCurrentAction) => set({ agentCurrentAction }),
    addAgentStep: (name, type, args, callId) => set((state) => {
        const messages = [...state.agentMessages];
        if (messages.length === 0) return state;
        const last = messages[messages.length - 1];
        if (last && last.role === 'assistant') {
            const steps = last.steps || [];
            if (!steps.find((s: any) => (callId && s.callId === callId) || (!callId && s.name === name && s.status === 'running'))) {
                last.steps = boundedPush(steps, { name, status: 'running' as const, type, args, callId }, MAX_AGENT_STEPS_PER_MSG);
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

    pushTrajectoryEvent: (evt) => {
        set((s) => ({
            agentTrajectory: boundedPush(s.agentTrajectory, { id: `tr-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`, ts: Date.now(), turn: s.currentTurnId, ...evt }, MAX_TRAJECTORY_EVENTS),
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
        return { agentToolBlocks: boundedPush(state.agentToolBlocks, block, MAX_TOOL_BLOCKS) };
    }),

    appendAgentToolOutput: (streamId, line, stream) => set((state) => ({
        agentToolBlocks: state.agentToolBlocks.map((b) => {
            if (b.id !== streamId) return b;
            const prefix = stream === 'stderr' ? '\x1b[31m' : '';
            return { ...b, outputLines: boundedPush(b.outputLines, prefix + line, MAX_TOOL_OUTPUT_LINES) };
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
        return { agentToolBlocks: boundedTail(blocks, MAX_TOOL_BLOCKS) };
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
            if (result && b.kind === 'canvas') {
                next = {
                    ...enrichCanvasBlockFromResult(next, result),
                    status: success ? 'done' as const : 'error' as const,
                };
            }
            if (result && b.kind === 'edit') {
                const enriched = enrichEditBlockFromResult(next, result);
                next = {
                    ...enriched,
                    status: success ? 'done' as const : 'error' as const,
                    preview: enriched.preview || (result.length < 500 ? result.slice(0, 400) : enriched.preview),
                };
            }
            if (result && b.kind === 'explore') {
                next = {
                    ...enrichExploreBlockFromResult(next, result),
                    status: success ? 'done' as const : 'error' as const,
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

    runBackgroundAgent: async (prompt) => {
        const id = `bg-${Date.now()}-${Math.floor(Math.random() * 1000)}`;
        const jobName = prompt.slice(0, 72).replace(/\s+/g, ' ').trim() || 'Background agent';
        set((s) => ({ backgroundAgents: [...s.backgroundAgents, { id, prompt, status: 'running', result: '', startedAt: Date.now() }] }));
        try {
            const { invoke } = await import('../tauri_bridge');
            await invoke('register_background_job', { id, name: jobName });
            await invoke('update_background_job', { id, progress: 5, status: 'starting' });
            const { ensureAgentRuntime } = await import('../application/performance/ensureAgentRuntime');
            await ensureAgentRuntime();
            const state = get();
            const provider = state.agentModel.includes('|') ? state.agentModel.split('|')[0] : 'ollama';
            const model = state.agentModel.includes('|') ? state.agentModel.split('|')[1] : state.agentModel;
            await invoke('update_background_job', { id, progress: 15, status: 'running' });
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
            await invoke('update_background_job', { id, progress: 100, status: 'done' });
            set((s) => ({ backgroundAgents: s.backgroundAgents.map(b => b.id === id ? { ...b, status: 'done', result: resultText, finishedAt: Date.now() } : b) }));
        } catch (e: any) {
            const { invoke } = await import('../tauri_bridge');
            await invoke('update_background_job', { id, progress: 0, status: 'error' }).catch(() => {});
            set((s) => ({ backgroundAgents: s.backgroundAgents.map(b => b.id === id ? { ...b, status: 'error', result: String(e?.message ?? e), finishedAt: Date.now() } : b) }));
        }
        return id;
    },
    removeBackgroundAgent: (id) => set((s) => ({ backgroundAgents: s.backgroundAgents.filter(b => b.id !== id) })),
    clearBackgroundAgents: () => set({ backgroundAgents: [] }),
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
        const active = existingTasks.filter((t: any) => t.status === 'running' || t.status === 'pending');
        const completed = boundedTail(existingTasks.filter((t: any) => t.status !== 'running' && t.status !== 'pending'), MAX_COMPLETED_TASKS);
        return { agentTasks: [...active, ...completed], agentTask: updatedTask };
    }),
    setAgentTask: (agentTask) => set({ agentTask }),
    setAgentTasks: (agentTasks) => set({ agentTasks }),
    setAgentFiles: (agentFiles) => set({ agentFiles }),
    setAgentSteps: (agentSteps) => set({ agentSteps }),
    refreshProcessStats: async () => {
        const { refreshProcessMemory } = await import('../application/performance/refreshProcessMemory');
        const stats = await refreshProcessMemory();
        if (stats) set({ processStats: stats });
    },
    refreshMemorySavings: async () => {
        try { const [original, compressed] = await invoke<[number, number]>('get_memory_savings'); set({ memorySavings: { original, compressed } }); } catch { }
    },

    refreshBrainTelemetry: async () => {
        try { const telemetry = await invoke<any>('get_brain_telemetry'); set({ brainTelemetry: telemetry }); } catch { }
    },
    addKairosSuggestion: (suggestion) => set((state: any) => ({ kairosSuggestions: [suggestion, ...state.kairosSuggestions].slice(0, 50) })),

    setPendingToolPermission: (req) => set({ pendingToolPermission: req }),

    respondToolPermission: async (id, approved) => {
        set({ pendingToolPermission: null });
        try { await invoke('respond_tool_permission', { toolId: id, approved }); } catch { /* ignore */ }
    },
});
