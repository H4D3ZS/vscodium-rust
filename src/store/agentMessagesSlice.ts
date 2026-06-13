// Agent messages slice: transcript, threads, sessions, attached context, pending edits.
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

export interface AgentMessagesSlice {
    agentMessages: AgentMessage[];
    isAgentThinking: boolean;
    isAgentPaused: boolean;
    isAgentBlocked: boolean;
    pendingAgentEdits: { path: string; tool: string; timestamp: number; preview?: string }[];
    isMultiFileReviewOpen: boolean;
    lastAgentCheckpoint: { id: string; description: string; timestamp: number } | null;
    attachedFiles: { id: string; path: string; name: string; gist?: string; thumbnail?: string; type: 'file' | 'attachment' | 'mention' }[];
    attachedContext: AttachedContext[];
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
    contextSlots: SemanticSlot[];
    activeFileContext: { symbols: string[]; related_files: string[]; relevant_lessons: SemanticSlot[] } | null;
    activeProjectSpec: any | null;
    projectMemory: string;
    memoryFiles: string[];
    chatSessions: any[];
    addAgentMessage: (role: 'user' | 'assistant', content: string, context?: AttachedContext[] | boolean) => void;
    updateLastAgentMessage: (content: string) => void;
    appendLastAgentMessage: (delta: string) => void;
    updateLastAgentThought: (thought: string) => void;
    addAgentArtifact: (artifact: Omit<Artifact, 'id' | 'timestamp'>) => void;
    setIsAgentThinking: (v: boolean) => void;
    setIsAgentPaused: (v: boolean) => void;
    setAgentBlocked: (v: boolean) => void;
    setAgentMessages: (messages: any[]) => void;
    clearAgentMessages: () => void;
    resetThread: () => void;
    truncateAgentMessages: (index: number) => void;
    addPendingAgentEdit: (edit: { path: string; tool: string; preview?: string }) => void;
    removePendingAgentEdit: (path: string) => void;
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
    setPhase: (phase: 'ANALYZE' | 'PLAN' | 'EXECUTE' | 'VERIFY' | 'REPORT' | 'IDLE', status: string) => void;
    createAgentThread: (name: string) => string;
    closeAgentThread: (id: string) => void;
    setActiveAgentThread: (id: string) => void;
    approveArtifact: (threadId: string, artifactId: string) => void;
    rejectArtifact: (threadId: string, artifactId: string) => void;
    submitArtifactFeedback: (threadId: string, artifactId: string, feedback: string) => void;
    setActiveAgentThreadId: (id: string) => void;
    compressSessionData: (key: string, data: string) => Promise<void>;
    fetchWorkspaceMemory: (category: string) => Promise<void>;
    fetchFileContext: (path: string) => Promise<void>;
    fetchActiveProjectSpec: () => Promise<void>;
    refreshChatSessions: () => Promise<void>;
    loadChatSession: (path: string) => Promise<void>;
    archiveCurrentSession: () => Promise<void>;
    createNewSession: () => Promise<void>;
    setProjectMemory: (content: string, files?: string[]) => void;
    isDemoMode: boolean;
    startDemoMode: () => void;
    endDemoMode: () => void;
}

export const createAgentMessagesSlice: StateCreator<AppState, [], [], AgentMessagesSlice> = (set, get) => ({
    agentMessages: [],
    isAgentThinking: false,
    isAgentPaused: false,
    isAgentBlocked: false,
    pendingAgentEdits: [],
    isMultiFileReviewOpen: false,
    lastAgentCheckpoint: null,
    attachedFiles: [],
    attachedContext: [],
    agentThreads: {},
    activeAgentThreadId: '',
    chatRestoreToken: 0,
    contextSlots: [],
    activeFileContext: null,
    activeProjectSpec: null,
    projectMemory: '',
    memoryFiles: [],
    chatSessions: [],
    isDemoMode: false,

    setIsAgentThinking: (isAgentThinking) => set({ isAgentThinking }),
    setIsAgentPaused: (isAgentPaused) => set({ isAgentPaused }),
    setAgentBlocked: (isAgentBlocked) => set({ isAgentBlocked }),
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
            const existingClean = cleanAgentContent(last.content || '');
            const incomingClean = cleanAgentContent(newContent);
            if (!shouldReplaceAgentContent(existingClean, incomingClean)) {
                newContent = last.content || '';
            } else {
                newContent = incomingClean;
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

    addPendingAgentEdit: (edit) => set((s) => {
        const existing = s.pendingAgentEdits.find(e => e.path === edit.path);
        if (existing) {
            return { pendingAgentEdits: s.pendingAgentEdits.map(e => e.path === edit.path ? { ...e, tool: edit.tool, timestamp: Date.now(), preview: edit.preview ?? e.preview } : e) };
        }
        return { pendingAgentEdits: [...s.pendingAgentEdits, { ...edit, timestamp: Date.now() }] };
    }),
    removePendingAgentEdit: (path) => set((s) => ({ pendingAgentEdits: s.pendingAgentEdits.filter(e => e.path !== path) })),
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

    setPhase: (phase, status) => set({ currentPhase: phase, currentPhaseStatus: status }),

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

    compressSessionData: async (key, data) => {
        try { await invoke('compress_session_data', { key, data }); get().refreshMemorySavings(); } catch { }
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
    startDemoMode: () => set({ isDemoMode: true }),
    endDemoMode: () => set({ isDemoMode: false }),

});
