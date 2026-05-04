import { invoke } from '@tauri-apps/api/core';
import { useStore } from './store';

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
            // Ensure .agent/sessions directory exists
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
 * SubAgentManager handles the orchestration of specialized agents for specific tasks.
 * In this architecture, it primarily works by tagging messages and tasks
 * so the backend can switch context if needed.
 */
export class SubAgentManager {
    static async delegateTask(taskId: string, objective: string): Promise<void> {
        const state = useStore.getState();

        // This is now called via the backend 'spawn_subagent' tool
        // which returns a task_id that the frontend then listens for.

        state.updateAgentTask({
            id: taskId,
            title: `Sub-task: ${objective.slice(0, 30)}...`,
            status: 'running',
            progress: 10
        });

        console.log(`[SubAgentManager] Task ${taskId} delegated: ${objective}`);
    }

    static handleProgress(payload: { task_id: string, status: string, progress: number, message?: string, result?: string, error?: string }) {
        const state = useStore.getState();
        const { task_id, status, progress, message, result, error } = payload;

        state.updateAgentTask({
            id: task_id,
            status: status as any,
            progress: progress,
            message: message || (status === 'completed' ? 'Task finished.' : undefined)
        });

        if (status === 'completed' && result) {
            state.addAgentMessage('assistant', `[SUB-AGENT ${task_id.slice(0, 4)} COMPLETED]\n\n${result}`, true);
        } else if (status === 'failed' && error) {
            state.addAgentMessage('assistant', `[SUB-AGENT ${task_id.slice(0, 4)} FAILED]\n\nError: ${error}`, true);
        }
    }
}
