/**
 * Public agent API — import from here in new code, not from `agent.ts`.
 *
 * WHY a barrel?
 * - Application use-cases are the stable contract.
 * - `agent.ts` remains legacy implementation detail (lazy-loaded on send).
 */
export { sendAgentTurn } from './sendAgentTurn';
export { stopAgent } from './stopAgent';
export { pauseAgent } from './pauseAgent';
export { resumeAgent } from './resumeAgent';
export { bootstrapAgentRuntime } from './bootstrapAgentRuntime';

/** @deprecated Use sendAgentTurn — kept for gradual migration */
export async function sendAgentMessage(
    prompt: string,
    onUpdate?: (msg: string) => void,
    context?: any[],
): Promise<void> {
    const { sendAgentTurn } = await import('./sendAgentTurn');
    return sendAgentTurn({ prompt, onStreamChunk: onUpdate, context });
}
