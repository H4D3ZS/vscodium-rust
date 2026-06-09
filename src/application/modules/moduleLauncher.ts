import { invoke } from '../../tauri_bridge';
import { openPyTorchStudio } from '../pytorch/openPyTorchStudio';

type LaunchResult = {
    ok?: boolean;
    mode?: 'component' | 'process';
    component_id?: string;
    launch_url?: string;
    install_dir?: string;
};

/** Open an installed module — in-IDE component or external process / URL. */
export async function launchInstalledModule(moduleId: string): Promise<void> {
    const result = await invoke<LaunchResult>('modules_launch', { id: moduleId });

    if (result.mode === 'component' && result.component_id) {
        switch (result.component_id) {
            case 'pytorch-studio':
                openPyTorchStudio();
                return;
            case 'architecture-visualizer':
                window.dispatchEvent(new CustomEvent('hades:open-panel', { detail: { panel: 'architecture-visualizer' } }));
                return;
            case 'code-coverage':
                window.dispatchEvent(new CustomEvent('hades:open-panel', { detail: { panel: 'code-coverage' } }));
                return;
            case 'adr-manager':
                window.dispatchEvent(new CustomEvent('hades:open-panel', { detail: { panel: 'adr-manager' } }));
                return;
            default:
                throw new Error(`Unknown component: ${result.component_id}`);
        }
    }

    if (result.launch_url) {
        window.open(result.launch_url, '_blank', 'noopener,noreferrer');
    }
}

export async function installModule(moduleId: string, catalogUrl?: string): Promise<void> {
    await invoke('modules_install', { id: moduleId, catalogUrl: catalogUrl ?? null });
}

export async function uninstallModule(moduleId: string): Promise<void> {
    await invoke('modules_uninstall', { id: moduleId });
}

export async function setModuleEnabled(moduleId: string, enabled: boolean): Promise<void> {
    await invoke('modules_set_enabled', { id: moduleId, enabled });
}
