import type { IAgentRepository } from '../../domain/agent/IAgentRepository';
import type { AgentTurnRequest } from '../../domain/agent/AgentTurnRequest';
import { agentControlRepository, sendTurnViaLegacyEngine } from './TauriAgentControlRepository';

/**
 * Adapter: implements IAgentRepository by composing Tauri control + lazy legacy send.
 *
 * WHY not rewrite sendAgentMessage yet?
 * It encodes 2+ years of provider routing, tool loops, and Cyber-Ifrit gating.
 * This adapter lets new code depend on the port while we peel layers off incrementally.
 */
export class LegacyAgentEngine implements IAgentRepository {
    async sendTurn(request: AgentTurnRequest): Promise<void> {
        await sendTurnViaLegacyEngine(request);
    }

    stop(): Promise<void> {
        return agentControlRepository.stop();
    }

    pause(): Promise<void> {
        return agentControlRepository.pause();
    }

    resume(): Promise<void> {
        return agentControlRepository.resume();
    }

    setYoloMode(enabled: boolean): Promise<string> {
        return agentControlRepository.setYoloMode(enabled);
    }

    getYoloMode(): Promise<boolean> {
        return agentControlRepository.getYoloMode();
    }
}

export const agentRepository: IAgentRepository = new LegacyAgentEngine();
