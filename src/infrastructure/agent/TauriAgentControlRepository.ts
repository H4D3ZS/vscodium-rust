import { invoke } from '../../tauri_bridge';
import type { IAgentRepository } from '../../domain/agent/IAgentRepository';
import type { AgentTurnRequest } from '../../domain/agent/AgentTurnRequest';

/**
 * Tauri IPC for agent transport control (stop/pause/resume/yolo).
 * WHY split from LegacyAgentEngine? Control commands are cheap invoke() calls;
 * sendTurn loads the 4k-line legacy engine only when the user actually chats.
 */
export class TauriAgentControlRepository implements Pick<IAgentRepository, 'stop' | 'pause' | 'resume' | 'setYoloMode' | 'getYoloMode'> {
    async stop(): Promise<void> {
        await invoke('stop_ai_agent');
    }

    async pause(): Promise<void> {
        await invoke('pause_ai_agent');
    }

    async resume(): Promise<void> {
        await invoke('resume_ai_agent');
    }

    async setYoloMode(enabled: boolean): Promise<string> {
        return invoke<string>('set_yolo_mode', { enabled });
    }

    async getYoloMode(): Promise<boolean> {
        return invoke<boolean>('get_yolo_mode');
    }
}

export const agentControlRepository = new TauriAgentControlRepository();

/** Lazy-load legacy sendTurn implementation (~large bundle). */
export async function sendTurnViaLegacyEngine(request: AgentTurnRequest): Promise<void> {
    const { sendAgentMessage } = await import('../../agent');
    await sendAgentMessage(request.prompt, request.onStreamChunk, request.context);
}
