import type { AgentTurnRequest } from './AgentTurnRequest';

/**
 * Port — agent execution and control.
 * Infrastructure: LegacyAgentEngine (wraps agent.ts) + TauriAgentControlRepository.
 */
export interface IAgentRepository {
    /** Run one agent turn (may stream via Tauri events). */
    sendTurn(request: AgentTurnRequest): Promise<void>;
    stop(): Promise<void>;
    pause(): Promise<void>;
    resume(): Promise<void>;
    setYoloMode(enabled: boolean): Promise<string>;
    getYoloMode(): Promise<boolean>;
}
