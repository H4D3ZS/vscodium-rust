/**
 * Public agent API — import from here in new code, not from `agent.ts`.
 *
 * WHY a barrel?
 * - Application use-cases are the stable contract.
 * - `agent.ts` remains legacy implementation detail (lazy-loaded on send).
 */
export { sendAgentTurn } from '../application/agent/sendAgentTurn';
export { stopAgent } from '../application/agent/stopAgent';
export { pauseAgent } from '../application/agent/pauseAgent';
export { resumeAgent } from '../application/agent/resumeAgent';
export { bootstrapAgentRuntime } from '../application/agent/bootstrapAgentRuntime';

/** @deprecated Use sendAgentTurn — kept for gradual migration */
export async function sendAgentMessage(
    prompt: string,
    onUpdate?: (msg: string) => void,
    context?: any[],
): Promise<void> {
    const { sendAgentTurn } = await import('../application/agent/sendAgentTurn');
    return sendAgentTurn({ prompt, onStreamChunk: onUpdate, context });
}
