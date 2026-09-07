// Agent messages slice: transcript, threads, sessions, attached context, pending edits.
// Split from agentSlice.ts; combined in the agentSlice.ts barrel.

import type { StateCreator } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import { MAX_AGENT_MESSAGES_IN_UI, MAX_AGENT_MESSAGE_CHARS } from '../domain/agent/AgentSessionPolicy';
import type { AppState } from './index';
import { boundedTail, MAX_THREAD_COUNT, MAX_RAW_BUFFER, MAX_MESSAGE_CONTENT } from '../domain/utils/boundedArray';
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
    /** Antigravity-style run summary from the last completed agent run. */
    lastRunSummary: { filesChanged: number; files: string[] } | null;
    setLastRunSummary: (s: { filesChanged: number; files: string[] } | null) => void;
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
        isThinking: boolean;
        tasks: any[];
        artifacts: any[];
        chatSessionPath?: string;
        firstUserSnippet?: string;
    }>;
    threadMessages: Record<string, AgentMessage[]>;
    activeAgentThreadId: string;
    /** Bumped after history restore so UI clears live tool-call overlays. */
    chatRestoreToken: number;
    contextSlots: SemanticSlot[];
    activeFileContext: { symbols: string[]; related_files: string[]; relevant_lessons: SemanticSlot[] } | null;
    activeProjectSpec: any | null;
    projectMemory: string;
    memoryFiles: string[];
    chatSessions: any[];
    pendingThoughts: string;
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
    threadMessages: {},
    activeAgentThreadId: '',
    chatRestoreToken: 0,
    contextSlots: [],
    activeFileContext: null,
    activeProjectSpec: null,
    projectMemory: '',
    memoryFiles: [],
    chatSessions: [],
    pendingThoughts: '',
    isDemoMode: false,

    setIsAgentThinking: (isAgentThinking) => set((state) => {
        // When thinking stops, clear raw_buffer on all messages except the last
        // (the last one may still be streaming). This frees up to ~1.28MB of
        // dead weight from completed turns.
        if (!isAgentThinking && state.agentMessages.length > 1) {
            const msgs = state.agentMessages.map((m, i) =>
                i < state.agentMessages.length - 1 ? { ...m, raw_buffer: undefined } : m
            );
            return { isAgentThinking, agentMessages: msgs };
        }
        return { isAgentThinking };
    }),
    lastRunSummary: null,
    setLastRunSummary: (lastRunSummary) => set({ lastRunSummary }),
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
        // Tighter transcript on low-end (≤4GB) machines — older messages are
        // trimmed sooner so a long session stays lean (VS Code-style ring buffer).
        const lowEnd = typeof document !== 'undefined' && document.body.classList.contains('low-end');
        const cap = lowEnd ? 20 : MAX_AGENT_MESSAGES_IN_UI;
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
        let threadMessages = { ...state.threadMessages };
        if (!threadId || !agentThreads[threadId]) {
            threadId = `agent-${Date.now()}`;
            agentThreads = {
                ...agentThreads,
                [threadId]: { id: threadId, name: 'Chat', isThinking: false, tasks: [], artifacts: [] },
            };
            threadMessages[threadId] = [];
        }
        threadMessages[threadId] = newMessages;

        const firstSnippet = role === 'user'
            ? (typeof content === 'string' ? content.slice(0, 18) : '').trim()
            : (agentThreads[threadId]?.firstUserSnippet || '');
        agentThreads = {
            ...agentThreads,
            [threadId]: { ...agentThreads[threadId], firstUserSnippet: firstSnippet },
        };

        const threadKeys = Object.keys(agentThreads);
        if (threadKeys.length > MAX_THREAD_COUNT) {
            const sorted = threadKeys.sort((a, b) => {
                const aTs = parseInt(a.split('-')[1] || '0');
                const bTs = parseInt(b.split('-')[1] || '0');
                return aTs - bTs;
            });
            const toRemove = sorted.slice(0, sorted.length - MAX_THREAD_COUNT);
            for (const k of toRemove) {
                delete agentThreads[k];
                delete threadMessages[k];
            }
        }
        return { agentMessages: newMessages, activeAgentThreadId: threadId, agentThreads, threadMessages, agentToolBlocks };
    }),

    updateLastAgentMessage: (content) => set((state) => {
        const messages = [...state.agentMessages];
        const lastIndex = messages.length - 1;
        const last = messages[lastIndex];
        if (last && last.role === 'assistant') {
            const rawContent = typeof content === 'string' ? content : (content && typeof content === 'object' && (content as any).content ? (content as any).content : String(content));
            let newContent = rawContent;
            let newThoughts = last.thoughts;

            // Consume pending thoughts — thinking events that arrived before
            // the assistant message was created are now attached to it.
            if (state.pendingThoughts && !newThoughts) {
                newThoughts = state.pendingThoughts;
            }

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
        // MEMORY: skip the per-update agentThreads array duplication (synced on
        // addAgentMessage). Live rendering reads agentMessages.
        return { agentMessages: messages, pendingThoughts: '' };
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

            // Consume pendingThoughts on first content delta (thinking that arrived
            // before the assistant message was created).
            if (!newThoughts && state.pendingThoughts && currentContent === '' && delta.length > 0) {
                newThoughts = state.pendingThoughts;
                messages[lastIndex] = { ...last, thoughts: newThoughts };
            }
            // Incremental <think>-block tracking. The old code regex-scanned the
            // ENTIRE raw buffer on every streamed token (O(n) per token → O(n²)
            // per turn) which caused massive GC churn mid-generation. Instead we
            // memoize the parse phase on the message and only scan the delta
            // (plus a small carry tail for tags split across chunk boundaries).
            let phase: 'none' | 'open' | 'closed' = (last as any)._thinkPhase
                || (currentRaw.includes('<think>')
                    ? (currentRaw.includes('</think>') ? 'closed' : 'open')
                    : 'none');
            if (phase === 'none') {
                if ((currentRaw.slice(-8) + delta).includes('<think>')) {
                    // Transition — one full scan is fine here (once per message).
                    phase = 'open';
                    const openIdx = fullRaw.indexOf('<think>');
                    newContent = fullRaw.slice(0, openIdx).trim();
                    const afterOpen = fullRaw.slice(openIdx + 7);
                    const closeIdx = afterOpen.indexOf('</think>');
                    if (closeIdx >= 0) {
                        phase = 'closed';
                        const thoughtText = afterOpen.slice(0, closeIdx).trim();
                        if (thoughtText) set({ currentThought: parseThought(thoughtText) });
                        newThoughts = newThoughts ? newThoughts + '\n' + thoughtText : thoughtText;
                        newContent = (newContent + ' ' + afterOpen.slice(closeIdx + 8)).trim();
                    } else {
                        if (afterOpen.trim()) set({ currentThought: parseThought(afterOpen.slice(-600).trim()) });
                        newThoughts = newThoughts ? newThoughts + '\n' + afterOpen : afterOpen;
                    }
                } else {
                    newContent += delta;
                }
            } else if (phase === 'open') {
                const carry = (last.thoughts || '').slice(-9);
                if ((carry + delta).includes('</think>')) {
                    phase = 'closed';
                    // Transition — resolve boundaries with one full scan.
                    const openIdx = fullRaw.indexOf('<think>');
                    const closeIdx = fullRaw.indexOf('</think>', openIdx >= 0 ? openIdx : 0);
                    const thoughtText = fullRaw.slice(openIdx >= 0 ? openIdx + 7 : 0, closeIdx).trim();
                    if (thoughtText) set({ currentThought: parseThought(thoughtText) });
                    newThoughts = thoughtText;
                    newContent = ((openIdx > 0 ? fullRaw.slice(0, openIdx) : '') + ' ' + fullRaw.slice(closeIdx + 8)).trim();
                } else {
                    newThoughts = currentThoughts + delta;
                    if (delta.trim()) set({ currentThought: parseThought(newThoughts.slice(-600).trim()) });
                }
            } else {
                newContent += delta;
            }
            const boundedRaw = fullRaw.length > MAX_RAW_BUFFER ? fullRaw.slice(-MAX_RAW_BUFFER) : fullRaw;
            const boundedContent = newContent.length > MAX_MESSAGE_CONTENT ? newContent.slice(0, MAX_MESSAGE_CONTENT) : newContent;
            messages[lastIndex] = { ...last, _thinkPhase: phase, content: boundedContent, thoughts: newThoughts, raw_buffer: boundedRaw,
                thoughtStartedAt: last.thoughtStartedAt ?? (newThoughts ? Date.now() : undefined),
                thoughtDurationMs: newThoughts && newContent && last.thoughtStartedAt
                    ? Date.now() - last.thoughtStartedAt
                    : last.thoughtDurationMs,
            };
            // MEMORY: do NOT re-duplicate the whole array into agentThreads on every
            // token — that doubled heap churn during streaming. agentThreads is
            // re-synced on addAgentMessage; live rendering reads agentMessages.
            return { agentMessages: messages };
        }
        return state;
    }),

    updateLastAgentThought: (thought) => set((state) => {
        const messages = [...state.agentMessages];
        const last = messages[messages.length - 1];
        // The backend emits `ai-thinking` BOTH as per-token deltas (the local backend native
        // streaming) and, in a few places, as a full accumulated snapshot. Replacing
        // blindly showed "one word at a time". Merge intelligently: if the incoming
        // text already contains everything we have, it's a snapshot → replace; if it's
        // brand-new text → append; if it's a duplicate tail → ignore.
        const merge = (cur: string, incoming: string): string => {
            const c = cur || '';
            const inc = incoming || '';
            if (!inc) return c;
            if (!c) return inc;
            if (c.endsWith(inc)) return c;                 // duplicate delta
            if (inc.length >= c.length && inc.startsWith(c)) return inc; // accumulated snapshot
            return c + inc;                                // new delta → append
        };
        let merged = thought;

        // If the last message is an assistant message, merge into its thoughts.
        // If NOT (e.g. thinking events arrived before the assistant message was
        // created), store in a pending buffer so the thoughts aren't silently
        // discarded — they'll be attached when the assistant message appears.
        if (last?.role === 'assistant') {
            merged = merge(last.thoughts || '', thought);
            messages[messages.length - 1] = {
                ...last,
                thoughts: merged,
                thoughtStartedAt: last.thoughtStartedAt ?? Date.now(),
            };
            return { agentMessages: messages, currentThought: parseThought(merged) };
        } else {
            // No assistant message yet — buffer the thinking. It will be
            // consumed when the first content chunk creates the assistant message.
            const pending = merge(state.pendingThoughts || '', thought);
            return { pendingThoughts: pending, currentThought: parseThought(pending) };
        }
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
            const threadMessages = { ...state.threadMessages };
            if (prevId && prevId !== id) {
                threadMessages[prevId] = state.agentMessages;
            }
            const thread = state.agentThreads[id];
            if (!thread) return state;
            return {
                threadMessages,
                activeAgentThreadId: id,
                agentMessages: threadMessages[id] || [],
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
