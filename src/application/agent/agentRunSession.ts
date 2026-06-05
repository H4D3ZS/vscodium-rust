/**
 * Coordinates hard-stop across the frontend agent turn:
 * - clears the stream poll timer immediately on Stop
 * - ignores late `chat_stream_drain` chunks after abort
 */
let aborted = false;
let streamTimer: ReturnType<typeof setInterval> | null = null;

export function beginAgentRun(): void {
    aborted = false;
}

export function abortAgentRun(): void {
    aborted = true;
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
