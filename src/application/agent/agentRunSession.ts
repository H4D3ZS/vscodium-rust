/**
 * Coordinates hard-stop across the frontend agent turn:
 * - clears the stream poll timer immediately on Stop
 * - ignores late `chat_stream_drain` chunks after abort
 * - activity-aware watchdog (idle timeout, not fixed wall clock)
 */
let aborted = false;
let streamTimer: ReturnType<typeof setInterval> | null = null;

/** No tool/stream activity for this long → treat as stuck. */
const IDLE_TIMEOUT_MS = 20 * 60 * 1000;
/** Hard ceiling so a run cannot hang forever. */
const MAX_RUN_MS = 3 * 60 * 60 * 1000;

let idleTimer: ReturnType<typeof setTimeout> | null = null;
let watchdogReject: ((err: Error) => void) | null = null;
let runStartedAt = 0;

export function beginAgentRun(): void {
    aborted = false;
}

export function abortAgentRun(): void {
    aborted = true;
    stopAgentRunWatchdog();
    if (streamTimer) {
        clearInterval(streamTimer);
        streamTimer = null;
    }
}

export function isAgentRunAborted(): boolean {
    return aborted;
}

export function registerStreamPollTimer(timer: ReturnType<typeof setInterval>): void {
    if (streamTimer) clearInterval(streamTimer);
    streamTimer = timer;
}

export function clearStreamPollTimer(): void {
    if (streamTimer) {
        clearInterval(streamTimer);
        streamTimer = null;
    }
}

function fireWatchdog(reason: 'idle' | 'max'): void {
    if (aborted || !watchdogReject) return;
    const elapsedMin = Math.round((Date.now() - runStartedAt) / 60_000);
    const msg =
        reason === 'idle'
            ? `Agent idle for ${IDLE_TIMEOUT_MS / 60_000} minutes (no tools or tokens). Stopped after ${elapsedMin}m total.`
            : `Agent reached ${MAX_RUN_MS / 3_600_000}h maximum run time.`;
    watchdogReject(
        new Error(
            `${msg}\n\nThe Rust backend may still be running — use Stop or restart if needed.`,
        ),
    );
}

function armIdleTimer(): void {
    if (idleTimer) clearTimeout(idleTimer);
    if (aborted || !watchdogReject) return;

    if (Date.now() - runStartedAt >= MAX_RUN_MS) {
        fireWatchdog('max');
        return;
    }

    const remainingMax = MAX_RUN_MS - (Date.now() - runStartedAt);
    idleTimer = setTimeout(
        () => fireWatchdog('idle'),
        Math.min(IDLE_TIMEOUT_MS, remainingMax),
    );
}

/** Reset the idle clock when tools stream, tokens arrive, etc. */
export function bumpAgentRunActivity(): void {
    if (aborted) return;
    armIdleTimer();
}

/** Start watchdog; returns cleanup. Rejects the returned promise on idle/max. */
export function startAgentRunWatchdog(reject: (err: Error) => void): () => void {
    stopAgentRunWatchdog();
    runStartedAt = Date.now();
    watchdogReject = reject;
    armIdleTimer();
    return stopAgentRunWatchdog;
}

export function stopAgentRunWatchdog(): void {
    if (idleTimer) {
        clearTimeout(idleTimer);
        idleTimer = null;
    }
    watchdogReject = null;
}
