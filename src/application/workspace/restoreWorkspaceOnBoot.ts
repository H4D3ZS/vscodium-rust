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
        if (backend) {
            callbacks.setActiveRoot(workspacePathToString(backend));
            await callbacks.refreshFileTree();
        }
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
            useStore.setState({ activeRoot: null, activeRootName: null, fileTree: [], fileTreeError: null });
            await fallback();
            return;
        }

        const pathStr = workspacePathToString(parsed);
        callbacks.setActiveRoot(pathStr);
        // setActiveRoot kicks off refresh asynchronously; await explicit reload with path sync.
        await callbacks.refreshFileTree();
        // Potato mode: skip workspace-level LSP auto-start (100–300MB per server).
        // LSP still starts on first file open via ensureLanguageServerForFile.
        void import('../../lib/systemProfile').then(async ({ isLiteMode }) => {
            if (await isLiteMode()) return;
            const m = await import('../lsp/bootstrapLanguageServer');
            void m.bootstrapLanguageServer(pathStr);
        });
        // Restore persisted agent canvases (cheap: small JSON files, no tabs opened).
        void useStore.getState().loadPersistedCanvases?.(pathStr);
    } catch (err) {
        console.warn('[restoreWorkspaceOnBoot] failed — fallback:', err);
        callbacks.clearPersistedRoot();
        useStore.setState({ activeRoot: null, activeRootName: null, fileTree: [], fileTreeError: null });
        await fallback();
    }
}
