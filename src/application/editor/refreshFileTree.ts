import { invoke } from '../../tauri_bridge';
import { useStore } from '../../store';
import type { FileEntry } from '../../store/types';

export async function refreshFileTree(): Promise<void> {
    try {
        const tree = await invoke<FileEntry[]>('get_file_tree');
        useStore.setState({ fileTree: tree });
    } catch {
        useStore.setState({ fileTree: [] });
    }
}
