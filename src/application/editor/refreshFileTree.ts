import { invoke } from '../../tauri_bridge';
import { useStore } from '../../store';
import type { FileEntry } from '../../store/types';

export async function refreshFileTree(): Promise<void> {
    const activeRoot = useStore.getState().activeRoot;
    useStore.setState({ fileTreeLoading: true, fileTreeError: null });

    if (!activeRoot) {
        useStore.setState({ fileTree: [], fileTreeLoading: false, fileTreeError: null });
        return;
    }

    try {
        const tree = await invoke<FileEntry[]>('get_file_tree', { path: activeRoot });
        if (!Array.isArray(tree)) {
            throw new Error('Invalid file tree response from backend');
        }
        useStore.setState({ fileTree: tree, fileTreeLoading: false, fileTreeError: null });
    } catch (err) {
        const msg = err instanceof Error ? err.message : String(err);
        console.error('[refreshFileTree] failed:', msg);
        useStore.setState({ fileTreeLoading: false, fileTreeError: msg });
    }
}
