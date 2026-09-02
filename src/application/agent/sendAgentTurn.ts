import type { AgentTurnRequest } from '../../domain/agent/AgentTurnRequest';
import type { IAgentRepository } from '../../domain/agent/IAgentRepository';
import { agentRepository } from '../../infrastructure/agent/LegacyAgentEngine';

/**
 * Use-case: send one user turn to the agent.
 * UI calls this — never imports agent.ts directly.
 */
export async function sendAgentTurn(
    request: AgentTurnRequest,
    repo: IAgentRepository = agentRepository,
): Promise<void> {
    await repo.sendTurn(request);
}
