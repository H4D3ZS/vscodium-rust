import { invoke } from '../../tauri_bridge';
import { useStore } from '../../store';

let persistTimer: ReturnType<typeof setTimeout> | null = null;
let autoSaveInterval: ReturnType<typeof setInterval> | null = null;
const AUTO_SAVE_INTERVAL_MS = 30_000; // 30 seconds
const LAST_ACTIVE_KEY = 'agent.lastActiveTimestamp';
const CRASH_THRESHOLD_MS = 5 * 60 * 1000; // 5 minutes — if last active was >5min ago, likely a crash

/** Push the live chat panel messages into the Rust memory store before archive/restore. */
export async function syncAgentMessagesToBackend(): Promise<void> {
    const { agentMessages, activeAgentThreadId, threadMessages } = useStore.getState() as any;
    let messages = agentMessages;
    if (activeAgentThreadId && threadMessages?.[activeAgentThreadId]?.length) {
        messages = threadMessages[activeAgentThreadId];
    }
    const payload = messages
        .filter(m => m.role === 'user' || m.role === 'assistant')
        .map(m => ({
            role: m.role,
            content: typeof m.content === 'string' ? m.content : JSON.stringify(m.content),
            tool_calls: null,
            tool_call_id: null,
            metadata: { timestamp: m.timestamp ?? Date.now() },
        }));
    if (payload.length === 0) return;
    await invoke('sync_agent_messages', { messages: payload });

    // Update last-active timestamp so crash detection works
    try { localStorage.setItem(LAST_ACTIVE_KEY, String(Date.now())); } catch {}
}

/** Debounced sync so History always lists the live conversation. */
export function scheduleChatHistorySync(): void {
    if (persistTimer) clearTimeout(persistTimer);
    persistTimer = setTimeout(() => {
        void syncAgentMessagesToBackend().catch(() => {});
    }, 600);
}

// ── Periodic auto-save ───────────────────────────────────────────────────────

/** Start the periodic auto-save timer. Call once on app boot. */
export function startAutoSave(): void {
    if (autoSaveInterval) return;
    autoSaveInterval = setInterval(() => {
        const { agentMessages } = useStore.getState();
        if (agentMessages.length > 0) {
            void syncAgentMessagesToBackend().catch(() => {});
        }
    }, AUTO_SAVE_INTERVAL_MS);
}

/** Stop the periodic auto-save timer. */
export function stopAutoSave(): void {
    if (autoSaveInterval) {
        clearInterval(autoSaveInterval);
        autoSaveInterval = null;
    }
}

// ── Crash recovery detection ─────────────────────────────────────────────────

export interface CrashRecoveryInfo {
    /** Whether a crash was detected (conversation was active when app closed). */
    crashed: boolean;
    /** How many messages were in the conversation at crash time. */
    messageCount: number;
    /** Timestamp of last activity before crash. */
    lastActiveAt: number;
    /** Milliseconds since the crash. */
    elapsedMs: number;
}

/**
 * Check if the previous session crashed mid-conversation.
 * Returns recovery info if a crash is detected, null otherwise.
 */
export function detectCrash(): CrashRecoveryInfo | null {
    try {
        const lastActive = localStorage.getItem(LAST_ACTIVE_KEY);
        if (!lastActive) return null;

        const lastActiveTs = parseInt(lastActive, 10);
        if (isNaN(lastActiveTs)) return null;

        const now = Date.now();
        const elapsed = now - lastActiveTs;

        // Only flag as crash if last active was within the last 24 hours
        // (not ancient history) but more than the threshold ago
        if (elapsed > CRASH_THRESHOLD_MS && elapsed < 24 * 60 * 60 * 1000) {
            const { agentMessages } = useStore.getState();
            return {
                crashed: true,
                messageCount: agentMessages.length,
                lastActiveAt: lastActiveTs,
                elapsedMs: elapsed,
            };
        }

        return null;
    } catch {
        return null;
    }
}

/** Clear the crash marker after recovery. */
export function clearCrashMarker(): void {
    try { localStorage.removeItem(LAST_ACTIVE_KEY); } catch {}
}

/** Format elapsed time for display. */
export function formatElapsed(ms: number): string {
    const minutes = Math.floor(ms / 60_000);
    if (minutes < 1) return 'just now';
    if (minutes < 60) return `${minutes}m ago`;
    const hours = Math.floor(minutes / 60);
    return `${hours}h ${minutes % 60}m ago`;
}
