import { invoke } from '../../tauri_bridge';
import { useStore } from '../../store';
import { bootstrapLanguageServer } from '../lsp/bootstrapLanguageServer';

export interface WorkspaceFolder {
    path: string;
    name: string;
}

const STORAGE_KEY = 'vscr.workspaceFolders';

function loadFolders(): WorkspaceFolder[] {
    try {
        const raw = localStorage.getItem(STORAGE_KEY);
        return raw ? (JSON.parse(raw) as WorkspaceFolder[]) : [];
    } catch {
        return [];
    }
}

function saveFolders(folders: WorkspaceFolder[]): void {
    try {
        localStorage.setItem(STORAGE_KEY, JSON.stringify(folders));
    } catch { /* */ }
}

function toFileUri(path: string): string {
    const normalized = path.replace(/\\/g, '/');
    return normalized.startsWith('/') ? `file://${normalized}` : `file:///${normalized}`;
}

/** Sync LSP workspace folders after add/remove. */
async function notifyLspFolders(folders: WorkspaceFolder[]): Promise<void> {
    try {
        await invoke('lsp_change_workspace_folders', {
            folders: folders.map((f) => ({ uri: toFileUri(f.path), name: f.name })),
        });
    } catch { /* LSP may not be running */ }
}

export function getWorkspaceFolders(): WorkspaceFolder[] {
    const st = useStore.getState();
    const stored = loadFolders();
    if (stored.length > 0) return stored;
    if (st.activeRoot) {
        return [{ path: st.activeRoot, name: st.activeRootName || 'workspace' }];
    }
    return [];
}

export async function addWorkspaceFolder(path: string): Promise<void> {
    const cleaned = path.trim();
    if (!cleaned) return;
    const name = cleaned.replace(/\\/g, '/').split('/').pop() || cleaned;
    const folders = getWorkspaceFolders();
    if (folders.some((f) => f.path === cleaned)) return;

    const next = [...folders, { path: cleaned, name }];
    saveFolders(next);
    useStore.setState({ workspaceFolders: next });
    await notifyLspFolders(next);

    if (!useStore.getState().activeRoot) {
        useStore.getState().setActiveRoot(cleaned);
    }
}

export async function removeWorkspaceFolder(path: string): Promise<void> {
    const next = getWorkspaceFolders().filter((f) => f.path !== path);
    saveFolders(next);
    useStore.setState({ workspaceFolders: next });
    await notifyLspFolders(next);

    if (useStore.getState().activeRoot === path) {
        const fallback = next[0]?.path ?? null;
        useStore.getState().setActiveRoot(fallback);
    }
}

export function initWorkspaceFoldersFromStorage(): void {
    const folders = loadFolders();
    if (folders.length > 0) {
        useStore.setState({ workspaceFolders: folders });
    }
}

export async function bootstrapAllWorkspaceLsp(): Promise<void> {
    const primary = useStore.getState().activeRoot;
    if (primary) await bootstrapLanguageServer(primary);
}
