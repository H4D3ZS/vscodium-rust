import type { IAgentRepository } from '../../domain/agent/IAgentRepository';
import { agentRepository } from '../../infrastructure/agent/LegacyAgentEngine';
import { useStore } from '../../store';
import { abortAgentRun } from './agentRunSession';

/**
 * Use-case: hard-stop the running agent loop.
 */
export async function stopAgent(repo: IAgentRepository = agentRepository): Promise<void> {
    abortAgentRun();
    const st = useStore.getState();
    st.setContinuousMode?.(false);
    try {
        await repo.stop();
        st.setIsAgentPaused(false);
        st.setIsAgentThinking(false);
        st.setAgentCurrentAction(null);
        try {
            st.appendLastAgentMessage?.('\n\n⏹ **Stopped by user.**');
        } catch { /* no partial message yet */ }
    } catch (error) {
        console.error('[stopAgent] failed:', error);
        st.setIsAgentThinking(false);
        st.setAgentCurrentAction(null);
    }
}
