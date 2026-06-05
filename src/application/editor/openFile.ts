import { fileRepository } from '../../infrastructure/editor/TauriFileRepository';
import { useStore } from '../../store';
import type { EditorTab } from '../../store/types';

function detectLanguage(filename: string): string {
    const ext = filename.split('.').pop()?.toLowerCase() ?? '';
    const map: Record<string, string> = {
        rs: 'rust', ts: 'typescript', tsx: 'typescript', js: 'javascript',
        jsx: 'javascript', json: 'json', css: 'css', html: 'html',
        md: 'markdown', toml: 'toml', yaml: 'yaml', yml: 'yaml',
        sh: 'shell', py: 'python', go: 'go', c: 'c', cpp: 'cpp',
        h: 'c', hpp: 'cpp', txt: 'plaintext',
    };
    return map[ext] ?? 'plaintext';
}

/** Open a file in the editor (or focus existing tab). */
export async function openFile(path: string): Promise<void> {
    const st = useStore.getState();
    const existing = st.tabs.find((t) => t.path === path);
    if (existing) {
        st.setActiveTab(existing.id);
        return;
    }

    if (path.toLowerCase().endsWith('.aim')) {
        const filename = path.replace(/\\/g, '/').split('/').pop() ?? path;
        const id = `tab-${Date.now()}-${Math.random()}`;
        const tab = { id, filename, path, content: '', isModified: false, language: '', type: 'aim' } as unknown as EditorTab;
        useStore.setState((state) => {
            const history = state.tabHistory.slice(0, state.tabHistoryIndex + 1);
            history.push(id);
            return { tabs: [...state.tabs, tab], activeTabId: id, tabHistory: history, tabHistoryIndex: history.length - 1 };
        });
        return;
    }

    const content = await fileRepository.read(path);
    const filename = path.replace(/\\/g, '/').split('/').pop() ?? path;
    const id = `tab-${Date.now()}-${Math.random()}`;
    const tab: EditorTab = { id, filename, path, content, isModified: false, language: detectLanguage(filename) };
    useStore.setState((state) => {
        const history = state.tabHistory.slice(0, state.tabHistoryIndex + 1);
        history.push(id);
        return { tabs: [...state.tabs, tab], activeTabId: id, tabHistory: history, tabHistoryIndex: history.length - 1 };
    });
}
