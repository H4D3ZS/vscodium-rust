/**
 * Renderer scheduling + legacy caps.
 * Agent message limits live in domain/agent/AgentSessionPolicy.ts (single source of truth).
 */
export const MAX_AGENT_MESSAGES = 40;
export const MAX_MESSAGE_CONTENT_CHARS = 12_000;
/** Active editing — status bar RAM sample */
export const STATS_POLL_MS = 45_000;
/** No input for 90s — slow polls */
export const IDLE_STATS_POLL_MS = 120_000;
/** Tab hidden — skip polls entirely */
export const HIDDEN_PAUSE_POLL_MS = 0;
export const ACCOUNT_POLL_MS = 90_000;
/** Secondary subsystems (search, extensions, cyber stack) */
export const DEFERRED_INIT_MS = 5_000;
/** Agent spine idle fallback */
export const AGENT_BOOT_DEFER_MS = 14_000;
/** Heavy offline/enterprise/ANE — after first paint */
export const HEAVY_STACK_DEFER_MS = 8_000;
/** Target idle working-set band (MB) — watchdog + status bar hint */
export const LEAN_IDLE_TARGET_MB = 54;
export const SOFT_TRIM_THRESHOLD_MB = 90;

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
