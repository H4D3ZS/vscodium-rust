import type { StateCreator } from 'zustand';
import { terminalManager } from '../terminal';
import type { AppState } from './index';
import type { TerminalGroup } from './types';

export interface TerminalSlice {
    terminalGroups: TerminalGroup[];
    activeTerminalGroupId: string | null;

    addTerminalGroup: (shell?: string) => Promise<string>;
    addAiriActivityTerminal: () => Promise<string>;
    splitTerminal: (groupId: string, instanceId: string) => Promise<string>;
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

    addTerminalGroup: async (shell) => {
        const id = `group-${Date.now()}`;
        const instanceId = `term-${Date.now()}-${Math.random().toString(36).substr(2, 4)}`;
        const name = shell ? shell.split(/[\\/]/).pop() || 'shell' : 'terminal';
        await terminalManager.createTerminal(shell || undefined, undefined, undefined, instanceId);
        const newGroup: TerminalGroup = { id, name, instances: [instanceId], activeInstanceId: instanceId };
        set((s) => ({ terminalGroups: [...s.terminalGroups, newGroup], activeTerminalGroupId: id, activePanelTab: 'TERMINAL', isBottomPanelOpen: true }));
        return id;
    },

    addAiriActivityTerminal: async () => {
        const existing = get().terminalGroups.find((g: any) => g.name === 'AIRI');
        if (existing) { set({ activeTerminalGroupId: existing.id, activePanelTab: 'TERMINAL', isBottomPanelOpen: true }); return existing.id; }
        const groupId = `group-airi-${Date.now()}`;
        const instanceId = await terminalManager.createAiriActivityTerminal(`airi-activity-${Date.now()}`);
        const newGroup: TerminalGroup = { id: groupId, name: 'AIRI', instances: [instanceId], activeInstanceId: instanceId };
        set((s) => ({ terminalGroups: [...s.terminalGroups, newGroup], activeTerminalGroupId: groupId, activePanelTab: 'TERMINAL', isBottomPanelOpen: true }));
        return groupId;
    },

    splitTerminal: async (groupId, instanceId) => {
        const newInstanceId = `term-${Date.now()}-${Math.random().toString(36).substr(2, 4)}`;
        const currentInstance = terminalManager.getTerminal(instanceId);
        await terminalManager.createTerminal(currentInstance?.shell, undefined, undefined, newInstanceId);
        set((s) => ({
            terminalGroups: s.terminalGroups.map(g =>
                g.id === groupId ? { ...g, instances: [...g.instances, newInstanceId], activeInstanceId: newInstanceId } : g
            ),
        }));
        return newInstanceId;
    },

    closeTerminalInstance: async (groupId, instanceId) => {
        await terminalManager.closeTerminal(instanceId);
        set((s) => {
            const groups = s.terminalGroups.map(g => {
                if (g.id !== groupId) return g;
                const newInstances = g.instances.filter(id => id !== instanceId);
                return { ...g, instances: newInstances, activeInstanceId: g.activeInstanceId === instanceId ? (newInstances[newInstances.length - 1] || '') : g.activeInstanceId };
            }).filter(g => g.instances.length > 0);
            let activeId = s.activeTerminalGroupId;
            if (activeId === groupId && !groups.find(g => g.id === groupId)) activeId = groups.length > 0 ? groups[groups.length - 1].id : null;
            return { terminalGroups: groups, activeTerminalGroupId: activeId };
        });
    },

    setActiveTerminalGroup: (id) => set({ activeTerminalGroupId: id }),
    setActiveTerminalInstance: (groupId, instanceId) => set((s) => ({
        terminalGroups: s.terminalGroups.map(g => g.id === groupId ? { ...g, activeInstanceId: instanceId } : g),
    })),
    renameTerminalGroup: (groupId, name) => set((s) => ({
        terminalGroups: s.terminalGroups.map(g => g.id === groupId ? { ...g, name } : g),
    })),
    closeTerminalGroup: async (groupId) => {
        const group = get().terminalGroups.find(g => g.id === groupId);
        if (group) for (const instanceId of group.instances) await terminalManager.closeTerminal(instanceId);
        set((s) => {
            const nextGroups = s.terminalGroups.filter(g => g.id !== groupId);
            return { terminalGroups: nextGroups, activeTerminalGroupId: s.activeTerminalGroupId === groupId ? (nextGroups[0]?.id || null) : s.activeTerminalGroupId };
        });
    },
    updateTerminalSplitWeights: (groupId, weights) => set((s) => ({
        terminalGroups: s.terminalGroups.map(g => g.id === groupId ? { ...g, splitWeights: weights } : g),
    })),
});

