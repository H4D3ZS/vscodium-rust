import { invoke } from '../../tauri_bridge';
import { useStore } from '../../store';
import type { FileEntry } from '../../store/types';

function findNodeRecursive(nodes: FileEntry[], path: string): FileEntry | undefined {
    for (const node of nodes) {
        if (node.path === path) return node;
        if (node.children) {
            const found = findNodeRecursive(node.children, path);
            if (found) return found;
        }
    }
    return undefined;
}

function injectChildrenRecursive(nodes: FileEntry[], path: string, children: FileEntry[]): FileEntry[] {
    return nodes.map((node) => {
        if (node.path === path) return { ...node, children };
        if (node.children) return { ...node, children: injectChildrenRecursive(node.children, path, children) };
        return node;
    });
}

function updateExpansion(nodes: FileEntry[], path: string, isExpanded: boolean): FileEntry[] {
    return nodes.map((n) => {
        if (n.path === path) return { ...n, is_expanded: isExpanded };
        if (n.children) return { ...n, children: updateExpansion(n.children, path, isExpanded) };
        return n;
    });
}

/** Expand/collapse a directory node in the explorer tree (lazy-load children on first expand). */
export async function toggleDirectory(path: string): Promise<void> {
    const state = useStore.getState();
    const node = findNodeRecursive(state.fileTree, path);
    if (!node) return;

    const isNowExpanded = !node.is_expanded;

    if (isNowExpanded && (!node.children || node.children.length === 0)) {
        try {
            const children = await invoke<FileEntry[]>('list_dir_flat', { path });
            useStore.setState({
                fileTree: updateExpansion(
                    injectChildrenRecursive(state.fileTree, path, children),
                    path,
                    true,
                ),
            });
        } catch (e) {
            console.error('Lazy load directory failed:', e);
            useStore.setState({ fileTree: updateExpansion(state.fileTree, path, true) });
        }
    } else {
        useStore.setState({ fileTree: updateExpansion(state.fileTree, path, isNowExpanded) });
    }
}
