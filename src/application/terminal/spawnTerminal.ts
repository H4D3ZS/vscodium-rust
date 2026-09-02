import { useStore } from '../../store';
import { getTerminalManager } from './getTerminalManager';
import type { TerminalSplitLayout } from '../../domain/terminal/TerminalLayout';

/**
 * Spawn a new terminal group + first instance.
 * Returns the group id for UI selection.
 */
/** Spawn an OpenCode TUI terminal group, pre-wired with IDE AI provider config. */
export async function spawnOpenCodeGroup(): Promise<string> {
    const terminalManager = await getTerminalManager();
    const groupId = `group-opencode-${Date.now()}`;
    const instanceId = `term-opencode-${Date.now()}-${Math.random().toString(36).slice(2, 6)}`;

    await terminalManager.createOpenCodeTerminal(instanceId, groupId);

    useStore.setState((s) => ({
        terminalGroups: [
            ...s.terminalGroups,
            {
                id: groupId,
                name: 'OpenCode',
                instances: [instanceId],
                activeInstanceId: instanceId,
                layout: 'single' as TerminalSplitLayout,
            },
        ],
        activeTerminalGroupId: groupId,
        activePanelTab: 'TERMINAL',
        isBottomPanelOpen: true,
    }));

    return groupId;
}

/**
 * Spawn a Claude Code terminal group wired to the local Lemonade server.
 *
 * The backend applies the measured per-model `ctx_size`/`llamacpp.args`, reloads
 * the model if they changed, maps every model alias to the local model, and runs
 * skip-permissions + airgapped by default. Omit `model` to use the IDE's
 * currently selected one.
 */
export async function spawnClaudeCodeGroup(opts: {
    model?: string;
    skipPermissions?: boolean;
    allowNet?: boolean;
} = {}): Promise<string> {
    const terminalManager = await getTerminalManager();
    const groupId = `group-claude-${Date.now()}`;
    const instanceId = `term-claude-${Date.now()}-${Math.random().toString(36).slice(2, 6)}`;

    await terminalManager.createClaudeCodeTerminal({ ...opts, explicitId: instanceId, groupId });

    useStore.setState((s) => ({
        terminalGroups: [
            ...s.terminalGroups,
            {
                id: groupId,
                name: 'Claude Code',
                instances: [instanceId],
                activeInstanceId: instanceId,
                layout: 'single' as TerminalSplitLayout,
            },
        ],
        activeTerminalGroupId: groupId,
        activePanelTab: 'TERMINAL',
        isBottomPanelOpen: true,
    }));

    return groupId;
}

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
