import type { IWorkspaceRepository } from '../../domain/workspace/IWorkspaceRepository';
import { parseWorkspacePath, workspacePathToString } from '../../domain/workspace/WorkspacePath';
import { workspaceRepository } from '../../infrastructure/workspace/TauriWorkspaceRepository';
import { useStore } from '../../store';

export interface RestoreWorkspaceCallbacks {
    setActiveRoot: (path: string | null) => void;
    refreshFileTree: () => Promise<void>;
    clearPersistedRoot: () => void;
}

/**
 * Use-case: restore last workspace without leaking deleted paths into PTY cwd.
 */
export async function restoreWorkspaceOnBoot(
    persistedRoot: string | null,
    callbacks: RestoreWorkspaceCallbacks,
    repo: IWorkspaceRepository = workspaceRepository,
): Promise<void> {
    const fallback = async () => {
        const backend = await repo.getBackendRoot();
        if (backend) callbacks.setActiveRoot(workspacePathToString(backend));
    };

    const parsed = parseWorkspacePath(persistedRoot);
    if (!parsed) {
        await fallback();
        return;
    }

    try {
        const exists = await repo.pathExists(parsed);
        if (!exists) {
            console.warn('[restoreWorkspaceOnBoot] path gone:', workspacePathToString(parsed));
            callbacks.clearPersistedRoot();
            useStore.setState({ activeRoot: null, activeRootName: null, fileTree: [] });
            await fallback();
            return;
        }
        await repo.setActiveRoot(parsed);
        await callbacks.refreshFileTree();
    } catch (err) {
        console.warn('[restoreWorkspaceOnBoot] failed — fallback:', err);
        callbacks.clearPersistedRoot();
        useStore.setState({ activeRoot: null, activeRootName: null, fileTree: [] });
        await fallback();
    }
}
