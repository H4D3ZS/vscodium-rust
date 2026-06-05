import type { IAgentRepository } from '../../domain/agent/IAgentRepository';
import { agentRepository } from '../../infrastructure/agent/LegacyAgentEngine';
import { useStore } from '../../store';

export async function pauseAgent(repo: IAgentRepository = agentRepository): Promise<void> {
    try {
        await repo.pause();
        useStore.getState().setIsAgentPaused(true);
    } catch (error) {
        console.error('[pauseAgent] failed:', error);
    }
}
