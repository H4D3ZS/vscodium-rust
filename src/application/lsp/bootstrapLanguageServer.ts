import { invoke } from '../../tauri_bridge';
import { useStore } from '../../store';

export interface LspBootstrapResult {
    status: string;
    id?: string;
    command?: string;
}

/**
 * Auto-start a language server when a workspace folder opens.
 * Picks rust-analyzer / typescript-language-server / pyright / gopls from PATH.
 */
export async function bootstrapLanguageServer(root: string): Promise<LspBootstrapResult | null> {
    if (!root?.trim()) return null;
    try {
        useStore.getState().setLspStatus({ downloading: true, error: null });
        const running = await invoke<boolean>('lsp_is_running');
        if (running) {
            useStore.getState().setLspStatus({ running: true, downloading: false, serverId: useStore.getState().lspServerId ?? 'active' });
            return { status: 'already_running' };
        }
        // IDE-managed bundles — auto-download rust-analyzer/gopls; TS/Python from installer.
        await invoke('lsp_ensure_bundle', { root }).catch(() => null);
        const result = await invoke<LspBootstrapResult & { managed?: boolean }>('lsp_auto_start', { root });
        useStore.getState().setLspStatus({
            running: true,
            downloading: false,
            serverId: result.id ?? 'lsp',
            error: null,
        });
        return result;
    } catch (e: unknown) {
        const msg = e instanceof Error ? e.message : String(e);
        console.warn('[bootstrapLanguageServer]', msg);
        useStore.getState().setLspStatus({ running: false, downloading: false, serverId: null, error: msg });
        return null;
    }
}

export async function stopLanguageServer(): Promise<void> {
    try {
        await invoke('lsp_stop');
    } catch { /* already stopped */ }
    useStore.getState().setLspStatus({ running: false, serverId: null, error: null });
}
