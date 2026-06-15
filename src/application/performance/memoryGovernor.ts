/**
 * Adaptive polling + idle trim — keeps the IDE lean when the tab is hidden
 * or the user is only editing (no agent activity).
 */
import { invoke } from '../../tauri_bridge';
import { STATS_POLL_MS, IDLE_STATS_POLL_MS, HIDDEN_PAUSE_POLL_MS } from '../../memory_budget';

let statsIntervalMs = STATS_POLL_MS;
let hidden = typeof document !== 'undefined' && document.hidden;
let lastTrimAt = 0;

export function currentStatsPollMs(): number {
    if (hidden) return HIDDEN_PAUSE_POLL_MS;
    return statsIntervalMs;
}

export function scheduleMemoryGovernor(): () => void {
    if (typeof document === 'undefined') return () => {};

    const onVisibility = () => {
        hidden = document.hidden;
        if (hidden) {
            statsIntervalMs = HIDDEN_PAUSE_POLL_MS;
            void trimIfStale(true);
        } else {
            statsIntervalMs = STATS_POLL_MS;
        }
    };

    const onIdle = () => {
        statsIntervalMs = IDLE_STATS_POLL_MS;
        void trimIfStale(false);
    };

    document.addEventListener('visibilitychange', onVisibility);

    let idleTimer: ReturnType<typeof setTimeout> | undefined;
    const resetIdle = () => {
        statsIntervalMs = STATS_POLL_MS;
        if (idleTimer) clearTimeout(idleTimer);
        idleTimer = setTimeout(onIdle, 90_000);
    };
    ['keydown', 'mousedown', 'touchstart'].forEach((ev) => {
        document.addEventListener(ev, resetIdle, { passive: true });
    });
    resetIdle();

    return () => {
        document.removeEventListener('visibilitychange', onVisibility);
        if (idleTimer) clearTimeout(idleTimer);
    };
}

async function trimIfStale(force: boolean): Promise<void> {
    const now = Date.now();
    if (!force && now - lastTrimAt < 120_000) return;
    lastTrimAt = now;
    try {
        await invoke<string>('optimize_memory');
    } catch { /* offline */ }
}
