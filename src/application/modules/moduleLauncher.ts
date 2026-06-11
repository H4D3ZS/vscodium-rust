import { invoke } from '../../tauri_bridge';
import { useStore } from '../../store';
import { openPyTorchStudio } from '../pytorch/openPyTorchStudio';
import { openSecurityReviewPanel } from '../security/runCodebaseSecurityReview';

type LaunchResult = {
    ok?: boolean;
    mode?: 'component' | 'process';
    component_id?: string;
    launch_url?: string;
    install_dir?: string;
    pid?: number;
};

/**
 * Open an installed module — in-IDE component or external process / URL.
 * Returns a human-readable status so the caller can surface feedback
 * (a silent void here is how "Open does nothing" bugs hide).
 */
export async function launchInstalledModule(moduleId: string): Promise<string> {
    const result = await invoke<LaunchResult>('modules_launch', { id: moduleId });

    if (result.mode === 'component' && result.component_id) {
        switch (result.component_id) {
            case 'pytorch-studio':
                openPyTorchStudio();
                return 'Opened PyTorch Studio';
            case 'architecture-visualizer':
                useStore.getState().setSpecsWizardOpen?.(true);
                return 'Opened the Specs wizard (architecture view lives in its review step)';
            case 'chunk-secret-scanner':
            case 'zero-day-hunter':
            case 'hawk-eye-secrets':
                openSecurityReviewPanel('chunks');
                return 'Opened Security Review (secrets)';
            case 'vega-dast':
                openSecurityReviewPanel('vega');
                return 'Opened Security Review (Vega DAST)';
            default:
                // Fail loudly: an unknown component means the catalog promises
                // a panel this build doesn't ship. Surfacing it beats a no-op.
                throw new Error(
                    `Module panel "${result.component_id}" is not available in this build`,
                );
        }
    }

    if (result.launch_url) {
        window.open(result.launch_url, '_blank', 'noopener,noreferrer');
        return `Opened ${result.launch_url}`;
    }

    if (result.mode === 'process') {
        const pid = result.pid ? ` (pid ${result.pid})` : '';
        return `Launched module process${pid} in ${result.install_dir ?? 'its install dir'}`;
    }

    throw new Error('Module launched but produced nothing to open');
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
