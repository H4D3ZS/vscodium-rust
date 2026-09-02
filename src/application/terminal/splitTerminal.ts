import { useStore } from '../../store';
import { getTerminalManager } from './getTerminalManager';
import type { TerminalSplitLayout } from '../../domain/terminal/TerminalLayout';

/**
 * Split the active instance in a group (horizontal or vertical).
 * Updates store layout so `TerminalGroupView` renders the correct flex direction.
 */
export async function splitTerminalInGroup(
    groupId: string,
    instanceId: string,
    direction: 'horizontal' | 'vertical' = 'horizontal',
): Promise<string> {
    const terminalManager = await getTerminalManager();
    const currentInstance = terminalManager.getTerminal(instanceId);
    const newInstanceId = `term-${Date.now()}-${Math.random().toString(36).slice(2, 6)}`;
    const layout: TerminalSplitLayout =
        direction === 'vertical' ? 'split-vertical' : 'split-horizontal';

    await terminalManager.createTerminal(
        currentInstance?.shell,
        groupId,
        undefined,
        newInstanceId,
    );

    const mgrGroup = terminalManager.getGroup(groupId);
    if (mgrGroup) mgrGroup.layout = layout;

    useStore.setState((s) => ({
        terminalGroups: s.terminalGroups.map((g) =>
            g.id === groupId
                ? {
                      ...g,
                      instances: [...g.instances, newInstanceId],
                      activeInstanceId: newInstanceId,
                      layout,
                  }
                : g,
        ),
    }));

    return newInstanceId;
}
