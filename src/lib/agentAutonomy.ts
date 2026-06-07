/**
 * Agent autonomy helpers — persistent action modes (Bug Bounty, Harness, etc.)
 * opt into full auto-accept + YOLO so the user is not clicking Apply on every tool.
 */
import { invoke } from '@tauri-apps/api/core';
import { useStore } from '../store';

const PERSISTENT_MODES = new Set([
    'agent', 'harness', 'execution', 'fast', 'sentient', 'autonomous', 'yolo',
    'bugbounty', 'bug bounty',
    'redteam', 'red team',
]);

const READ_ONLY_MODES = new Set([
    'chat', 'planning', 'planning (source control)',
]);

export function normalizeAgentMode(mode: string | undefined | null): string {
    return (mode || '').trim().toLowerCase();
}

/** Modes that run the autonomous tool loop until done (Cursor-style). */
export function isPersistentAgentMode(mode: string | undefined | null): boolean {
    const m = normalizeAgentMode(mode);
    if (!m || READ_ONLY_MODES.has(m)) return false;
    if (PERSISTENT_MODES.has(m)) return true;
    if (m.startsWith('custom:')) return false;
    return false;
}

/** Offensive-security modes — always full autonomy. */
export function isOffensiveAgentMode(mode: string | undefined | null): boolean {
    const m = normalizeAgentMode(mode);
    return m === 'bugbounty' || m === 'bug bounty' || m === 'redteam' || m === 'red team' || m === 'yolo';
}

export function shouldAutoAcceptEverything(mode: string | undefined | null): boolean {
    const st = useStore.getState() as any;
    return !!st.isYoloMode || isPersistentAgentMode(mode);
}

/**
 * Sync frontend + backend autonomy flags for agentic modes.
 * Called when switching mode and at the start of each agent turn.
 */
export async function ensureAgenticAutonomy(mode?: string | undefined | null): Promise<void> {
    const st = useStore.getState() as any;
    const activeMode = mode ?? st.agentMode;
    if (!isPersistentAgentMode(activeMode)) return;

    const offensive = isOffensiveAgentMode(activeMode);

    if (!st.autoAcceptChanges) {
        st.setAutoAcceptChanges?.(true);
    }

    if (!st.isYoloMode || offensive) {
        try {
            await invoke<string>('set_yolo_mode', { enabled: true });
        } catch { /* backend optional in web-only dev */ }
        st.setYoloMode?.(true);
    }

    if (offensive) {
        st.setArtifactReviewPolicy?.('always_proceed');
        st.setTerminalAutoExecution?.('always_proceed');
        if (!st.agentCleanUi) st.setAgentCleanUi?.(true);
    }
}

export function onAgentModeChanged(mode: string): void {
    if (isPersistentAgentMode(mode)) {
        void ensureAgenticAutonomy(mode);
    }
}
