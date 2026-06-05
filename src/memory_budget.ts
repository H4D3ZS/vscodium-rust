/**
 * Renderer scheduling + legacy caps.
 * Agent message limits live in domain/agent/AgentSessionPolicy.ts (single source of truth).
 */
export const MAX_AGENT_MESSAGES = 40;
export const MAX_MESSAGE_CONTENT_CHARS = 12_000;
export const STATS_POLL_MS = 30_000;
export const ACCOUNT_POLL_MS = 60_000;
export const DEFERRED_INIT_MS = 4_000;

export function scheduleDeferredInit(fn: () => void, timeoutMs = DEFERRED_INIT_MS): void {
    if (typeof requestIdleCallback !== 'undefined') {
        requestIdleCallback(() => fn(), { timeout: timeoutMs });
    } else {
        setTimeout(fn, Math.min(timeoutMs, 2000));
    }
}

export function trimAgentMessageContent(content: string): string {
    if (!content || content.length <= MAX_MESSAGE_CONTENT_CHARS) return content;
    return content.slice(0, MAX_MESSAGE_CONTENT_CHARS) + '\n\n…[trimmed for memory]';
}
