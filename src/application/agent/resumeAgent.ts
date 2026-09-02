import type { IAgentRepository } from '../../domain/agent/IAgentRepository';
import { agentRepository } from '../../infrastructure/agent/LegacyAgentEngine';
import { useStore } from '../../store';

export async function resumeAgent(repo: IAgentRepository = agentRepository): Promise<void> {
    try {
        await repo.resume();
        useStore.getState().setIsAgentPaused(false);
    } catch (error) {
        console.error('[resumeAgent] failed:', error);
    }
}
