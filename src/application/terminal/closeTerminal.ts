import { useStore } from '../../store';
import { getTerminalManager } from './getTerminalManager';

export async function closeTerminalInstance(groupId: string, instanceId: string): Promise<void> {
    const terminalManager = await getTerminalManager();
    await terminalManager.closeTerminal(instanceId);

    useStore.setState((s) => {
        const groups = s.terminalGroups
            .map((g) => {
                if (g.id !== groupId) return g;
                const newInstances = g.instances.filter((id) => id !== instanceId);
                return {
                    ...g,
                    instances: newInstances,
                    activeInstanceId:
                        g.activeInstanceId === instanceId
                            ? newInstances[newInstances.length - 1] || ''
                            : g.activeInstanceId,
                    layout: newInstances.length <= 1 ? ('single' as const) : g.layout,
                };
            })
            .filter((g) => g.instances.length > 0);

        let activeId = s.activeTerminalGroupId;
        if (activeId === groupId && !groups.find((g) => g.id === groupId)) {
            activeId = groups.length > 0 ? groups[groups.length - 1].id : null;
        }

        return { terminalGroups: groups, activeTerminalGroupId: activeId };
    });
}

export async function closeTerminalGroup(groupId: string): Promise<void> {
    const terminalManager = await getTerminalManager();
    const group = useStore.getState().terminalGroups.find((g) => g.id === groupId);
    if (group) {
        for (const instanceId of group.instances) {
            await terminalManager.closeTerminal(instanceId);
        }
    }

    useStore.setState((s) => {
        const nextGroups = s.terminalGroups.filter((g) => g.id !== groupId);
        return {
            terminalGroups: nextGroups,
            activeTerminalGroupId:
                s.activeTerminalGroupId === groupId
                    ? nextGroups[0]?.id || null
                    : s.activeTerminalGroupId,
        };
    });
}
