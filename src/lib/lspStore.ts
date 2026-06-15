import { invoke } from '../tauri_bridge';

export interface UserLspRecord {
    id: string;
    name: string;
    languages: string[];
    command: string;
    args: string[];
    enabled: boolean;
    source: string;
    install_source: string;
    installed_at: string;
    install_dir?: string | null;
    file_extensions?: string[];
    preset_id?: string | null;
}

export interface LspPreset {
    id: string;
    name: string;
    languages: string[];
    install_kind: 'npm' | 'path' | string;
    npm_package?: string | null;
    path_commands: string[];
    default_args: string[];
    note: string;
}

export interface PathImportRow {
    presetId: string;
    name: string;
    command: string;
    languages: string[];
    alreadyInstalled: boolean;
}

export interface LspStoreStatus {
    storeDir: string;
    installedCount: number;
    registryPath: string;
}

export async function lspStoreStatus(): Promise<LspStoreStatus> {
    return invoke<LspStoreStatus>('lsp_store_status');
}

export async function lspStoreList(): Promise<{ servers: UserLspRecord[] }> {
    return invoke('lsp_store_list');
}

export async function lspStoreCatalog(): Promise<{ presets: LspPreset[] }> {
    return invoke('lsp_store_catalog');
}

export async function lspStoreScanPath(): Promise<{ imports: PathImportRow[] }> {
    return invoke('lsp_store_scan_path');
}

export async function lspStoreInstallPreset(presetId: string): Promise<{ ok: boolean; server: UserLspRecord }> {
    return invoke('lsp_store_install_preset', { presetId });
}

export async function lspStoreInstallPath(opts: {
    name: string;
    command: string;
    args?: string[];
    languages?: string[];
    file_extensions?: string[];
    id?: string;
}): Promise<{ ok: boolean; server: UserLspRecord }> {
    return invoke('lsp_store_install_path', opts);
}

export async function lspStoreInstallNpm(opts: {
    package: string;
    name?: string;
    languages?: string[];
    file_extensions?: string[];
    args?: string[];
    id?: string;
}): Promise<{ ok: boolean; server: UserLspRecord }> {
    return invoke('lsp_store_install_npm', opts);
}

export async function lspStoreSetEnabled(id: string, enabled: boolean): Promise<{ ok: boolean; server: UserLspRecord }> {
    return invoke('lsp_store_set_enabled', { id, enabled });
}

export async function lspStoreUninstall(id: string): Promise<{ ok: boolean; id: string }> {
    return invoke('lsp_store_uninstall', { id });
}
