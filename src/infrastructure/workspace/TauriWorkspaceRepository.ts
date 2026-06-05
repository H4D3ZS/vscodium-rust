import { invoke } from '../../tauri_bridge';
import type { IWorkspaceRepository } from '../../domain/workspace/IWorkspaceRepository';
import type { WorkspacePath } from '../../domain/workspace/WorkspacePath';
import { parseWorkspacePath, workspacePathToString } from '../../domain/workspace/WorkspacePath';

/**
 * Adapter: workspace paths via Tauri file_commands.
 */
export class TauriWorkspaceRepository implements IWorkspaceRepository {
    async pathExists(path: WorkspacePath): Promise<boolean> {
        return invoke<boolean>('path_exists', { path: workspacePathToString(path) });
    }

    async getBackendRoot(): Promise<WorkspacePath | null> {
        const raw = await invoke<string | null>('get_active_root');
        return parseWorkspacePath(raw);
    }

    async setActiveRoot(path: WorkspacePath): Promise<void> {
        await invoke('set_active_root', { path: workspacePathToString(path) });
    }
}

export const workspaceRepository = new TauriWorkspaceRepository();
