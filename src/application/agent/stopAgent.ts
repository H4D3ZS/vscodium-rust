import type { IAgentRepository } from '../../domain/agent/IAgentRepository';
import { agentRepository } from '../../infrastructure/agent/LegacyAgentEngine';
import { useStore } from '../../store';

/**
 * Use-case: hard-stop the running agent loop.
 */
export async function stopAgent(repo: IAgentRepository = agentRepository): Promise<void> {
    try {
        await repo.stop();
        const st = useStore.getState();
        st.setIsAgentPaused(false);
        st.setIsAgentThinking(false);
        st.setAgentCurrentAction(null);
    } catch (error) {
        console.error('[stopAgent] failed:', error);
    }
}
