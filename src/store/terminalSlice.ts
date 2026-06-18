import type { StateCreator } from 'zustand';
import type { AppState } from './index';
import type { TerminalGroup } from './types';
import { spawnTerminalGroup, spawnOpenCodeGroup } from '../application/terminal/spawnTerminal';
import { splitTerminalInGroup } from '../application/terminal/splitTerminal';
import { closeTerminalInstance, closeTerminalGroup } from '../application/terminal/closeTerminal';
import { getTerminalManager } from '../application/terminal/getTerminalManager';

export interface TerminalSlice {
    terminalGroups: TerminalGroup[];
    activeTerminalGroupId: string | null;

    addTerminalGroup: (shell?: string) => Promise<string>;
    addOpenCodeTerminalGroup: () => Promise<string>;
    addAiriActivityTerminal: () => Promise<string>;
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

    addAiriActivityTerminal: async () => {
        const terminalManager = await getTerminalManager();
        const existing = get().terminalGroups.find((g) => g.name === 'AIRI');
        if (existing) {
            set({ activeTerminalGroupId: existing.id, activePanelTab: 'TERMINAL', isBottomPanelOpen: true });
            return existing.id;
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
            activeTerminalGroupId: groupId,
            activePanelTab: 'TERMINAL',
            isBottomPanelOpen: true,
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
