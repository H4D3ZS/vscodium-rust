import { invoke } from '@tauri-apps/api/core';
import { useStore } from './store';
import {
    agUpsertSubagent,
    persistAgentTrajectoryEvent,
} from './infrastructure/antigravity/antigravityClient';

export interface AgentSession {
    id: string;
    root: string;
    messages: any[];
    tasks: any[];
    timestamp: number;
}

export class TaskManager {
    static async saveSession(): Promise<void> {
        const state = useStore.getState();
        const root = state.activeRoot;
        if (!root) return;

        const session: AgentSession = {
            id: 'current',
            root: root,
            messages: state.agentMessages,
            tasks: state.agentTasks,
            timestamp: Date.now()
        };

        try {
            const sessionDir = `${root}/.agent/sessions`;
            await invoke('create_directory', { path: sessionDir });

            const sessionPath = `${sessionDir}/session_current.json`;
            await invoke('write_file', {
                path: sessionPath,
                content: JSON.stringify(session, null, 2)
            });
            console.log('Session saved to', sessionPath);
        } catch (error) {
            console.error('Failed to save session:', error);
        }
    }

    static async loadSession(): Promise<boolean> {
        const state = useStore.getState();
        const root = state.activeRoot;
        if (!root) return false;

        const sessionPath = `${root}/.agent/sessions/session_current.json`;
        try {
            const content = await invoke<string>('read_file', { path: sessionPath });
            const session: AgentSession = JSON.parse(content);

            state.setAgentMessages(session.messages || []);
            state.setAgentTasks(session.tasks || []);

            return true;
        } catch (error) {
            console.warn('No existing session found or failed to load:', error);
            return false;
        }
    }

    static async clearSession(): Promise<void> {
        const state = useStore.getState();
        const root = state.activeRoot;
        if (!root) return;

        const sessionPath = `${root}/.agent/sessions/session_current.json`;
        try {
            await invoke('remove_file', { path: sessionPath });
            state.clearAgentMessages();
        } catch (error) {
            console.error('Failed to clear session file:', error);
        }
    }
}

/**
 * SubAgentManager — Antigravity-style subagent orchestration + trajectory persistence.
 */
export class SubAgentManager {
    static async delegateTask(taskId: string, objective: string): Promise<void> {
        const state = useStore.getState();

        state.updateAgentTask({
            id: taskId,
            title: `Sub-task: ${objective.slice(0, 30)}...`,
            status: 'running',
            progress: 10
        });

        const root = state.activeRoot;
        const cascadeId = state.activeCascadeId;
        if (root && cascadeId) {
            void agUpsertSubagent(root, cascadeId, {
                id: taskId,
                name: objective.slice(0, 40),
                status: 'running',
                parent_id: cascadeId,
                started_at: Date.now(),
                progress: 10,
            });
        }

        console.log(`[SubAgentManager] Task ${taskId} delegated: ${objective}`);
    }

    static handleProgress(payload: {
        task_id?: string;
        id?: string;
        status: string;
        progress: number;
        message?: string;
        title?: string;
        result?: string;
        error?: string;
    }) {
        const state = useStore.getState();
        const task_id = payload.task_id || payload.id || 'subagent';
        const { status, progress, message, result, error } = payload;

        state.updateAgentTask({
            id: task_id,
            status: status as any,
            progress: progress,
            message: message || (status === 'completed' ? 'Task finished.' : undefined)
        });

        const root = state.activeRoot;
        const cascadeId = state.activeCascadeId;
        if (root && cascadeId) {
            void agUpsertSubagent(root, cascadeId, {
                id: task_id,
                name: payload.title || `Subagent ${String(task_id).slice(0, 6)}`,
                status,
                parent_id: cascadeId,
                started_at: Date.now(),
                summary: message || result || error,
                progress,
            });
            void persistAgentTrajectoryEvent(root, cascadeId, {
                kind: 'subagent',
                title: message || payload.title || 'Subagent',
                detail: result || error,
                subagentId: task_id,
                success: status === 'completed',
            });
        }

        if (status === 'completed' && result) {
            state.addAgentMessage('assistant', `[SUB-AGENT ${task_id.slice(0, 4)} COMPLETED]\n\n${result}`, true);
        } else if (status === 'failed' && error) {
            state.addAgentMessage('assistant', `[SUB-AGENT ${task_id.slice(0, 4)} FAILED]\n\nError: ${error}`, true);
        }
    }
}
