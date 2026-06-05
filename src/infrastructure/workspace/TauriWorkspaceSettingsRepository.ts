import { invoke } from '../../tauri_bridge';
import type { IWorkspaceSettingsRepository } from '../../domain/workspace/IWorkspaceSettingsRepository';

export class TauriWorkspaceSettingsRepository implements IWorkspaceSettingsRepository {
    async load(root?: string): Promise<Record<string, unknown>> {
        const raw = await invoke<Record<string, unknown>>('get_workspace_settings', { root: root ?? null });
        return raw && typeof raw === 'object' ? raw : {};
    }

    async save(settings: Record<string, unknown>, root?: string): Promise<void> {
        await invoke('update_workspace_settings', { settings, root: root ?? null });
    }
}

export const workspaceSettingsRepository = new TauriWorkspaceSettingsRepository();
