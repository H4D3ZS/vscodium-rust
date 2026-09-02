import { workspaceSettingsRepository } from '../../infrastructure/workspace/TauriWorkspaceSettingsRepository';
import { useStore } from '../../store';

export async function loadWorkspaceSettings(): Promise<Record<string, unknown>> {
    const root = useStore.getState().activeRoot;
    if (!root) return {};
    return workspaceSettingsRepository.load(root);
}

export async function saveWorkspaceSettings(
    settings: Record<string, unknown>,
): Promise<void> {
    const root = useStore.getState().activeRoot;
    if (!root) throw new Error('Open a folder first.');
    await workspaceSettingsRepository.save(settings, root);
}
