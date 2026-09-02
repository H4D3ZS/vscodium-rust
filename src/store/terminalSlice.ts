import type { StateCreator } from 'zustand';
import type { AppState } from './index';
import type { TerminalGroup } from './types';
import { spawnTerminalGroup, spawnOpenCodeGroup, spawnClaudeCodeGroup } from '../application/terminal/spawnTerminal';
import { splitTerminalInGroup } from '../application/terminal/splitTerminal';
import { closeTerminalInstance, closeTerminalGroup } from '../application/terminal/closeTerminal';
import { getTerminalManager } from '../application/terminal/getTerminalManager';

export interface TerminalSlice {
    terminalGroups: TerminalGroup[];
    activeTerminalGroupId: string | null;

    addTerminalGroup: (shell?: string) => Promise<string>;
    addOpenCodeTerminalGroup: () => Promise<string>;
    /** Claude Code wired to the local Lemonade server. See `spawnClaudeCodeGroup`. */
    addClaudeCodeTerminalGroup: (opts?: { model?: string; skipPermissions?: boolean; allowNet?: boolean }) => Promise<string>;
    addAiriActivityTerminal: (opts?: { focus?: boolean }) => Promise<string>;
    splitTerminal: (groupId: string, instanceId: string, direction?: 'horizontal' | 'vertical') => Promise<string>;
    closeTerminalInstance: (groupId: string, instanceId: string) => Promise<void>;
    setActiveTerminalGroup: (id: string) => void;
    setActiveTerminalInstance: (groupId: string, instanceId: string) => void;
    renameTerminalGroup: (groupId: string, name: string) => void;
    closeTerminalGroup: (groupId: string) => Promise<void>;
    updateTerminalSplitWeights: (groupId: string, weights: number[]) => void;
}

export const createTerminalSlice: StateCreator<AppState, [], [], TerminalSlice> = (set, get) => ({
    terminalGroups: [],
    activeTerminalGroupId: null,

    addTerminalGroup: async (shell) => spawnTerminalGroup(shell),

    addOpenCodeTerminalGroup: async () => spawnOpenCodeGroup(),

    addClaudeCodeTerminalGroup: async (opts) => spawnClaudeCodeGroup(opts),

    addAiriActivityTerminal: async (opts) => {
        // focus defaults to true (user clicked the AIRI button). The agent passes
        // { focus: false } so an auto-created activity feed never hijacks the user's
        // real shell / forces the panel open — that was the "out of place" terminal.
        const focus = opts?.focus !== false;
        const terminalManager = await getTerminalManager();
        const existing = get().terminalGroups.find((g) => g.name === 'AIRI');
        if (existing) {
            if (focus) set({ activeTerminalGroupId: existing.id, activePanelTab: 'TERMINAL', isBottomPanelOpen: true });
            return existing.id;
        }
        // Ensure a real shell exists so AIRI is never the ONLY terminal group.
        const hasRealShell = get().terminalGroups.some((g) => g.name !== 'AIRI');
        if (!hasRealShell) {
            try { await spawnTerminalGroup(); } catch { /* best-effort */ }
        }
        const groupId = `group-airi-${Date.now()}`;
        const instanceId = await terminalManager.createAiriActivityTerminal(`airi-activity-${Date.now()}`);
        const newGroup: TerminalGroup = {
            id: groupId,
            name: 'AIRI',
            instances: [instanceId],
            activeInstanceId: instanceId,
            layout: 'single',
        };
        set((s) => ({
            terminalGroups: [...s.terminalGroups, newGroup],
            // Only take over the active group / panel when explicitly focused.
            ...(focus ? { activeTerminalGroupId: groupId, activePanelTab: 'TERMINAL', isBottomPanelOpen: true } : {}),
        }));
        return groupId;
    },

    splitTerminal: async (groupId, instanceId, direction = 'horizontal') =>
        splitTerminalInGroup(groupId, instanceId, direction),

    closeTerminalInstance: async (groupId, instanceId) => closeTerminalInstance(groupId, instanceId),

    setActiveTerminalGroup: (id) => set({ activeTerminalGroupId: id }),

    setActiveTerminalInstance: (groupId, instanceId) =>
        set((s) => ({
            terminalGroups: s.terminalGroups.map((g) =>
                g.id === groupId ? { ...g, activeInstanceId: instanceId } : g,
            ),
        })),

    renameTerminalGroup: (groupId, name) =>
        set((s) => ({
            terminalGroups: s.terminalGroups.map((g) => (g.id === groupId ? { ...g, name } : g)),
        })),

    closeTerminalGroup: async (groupId) => closeTerminalGroup(groupId),

    updateTerminalSplitWeights: (groupId, weights) =>
        set((s) => ({
            terminalGroups: s.terminalGroups.map((g) =>
                g.id === groupId ? { ...g, splitWeights: weights } : g,
            ),
        })),
});
