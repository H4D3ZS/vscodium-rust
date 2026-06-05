import { useStore } from '../../store';
import { getTerminalManager } from './getTerminalManager';
import type { TerminalSplitLayout } from '../../domain/terminal/TerminalLayout';

/**
 * Spawn a new terminal group + first instance.
 * Returns the group id for UI selection.
 */
export async function spawnTerminalGroup(shell?: string): Promise<string> {
    const terminalManager = await getTerminalManager();
    const id = `group-${Date.now()}`;
    const instanceId = `term-${Date.now()}-${Math.random().toString(36).slice(2, 6)}`;
    const name = shell ? shell.split(/[\\/]/).pop() || 'shell' : 'terminal';

    await terminalManager.createTerminal(shell || undefined, id, undefined, instanceId);

    useStore.setState((s) => ({
        terminalGroups: [
            ...s.terminalGroups,
            {
                id,
                name,
                instances: [instanceId],
                activeInstanceId: instanceId,
                layout: 'single' as TerminalSplitLayout,
            },
        ],
        activeTerminalGroupId: id,
        activePanelTab: 'TERMINAL',
        isBottomPanelOpen: true,
    }));

    return id;
}
