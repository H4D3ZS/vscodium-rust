import { invoke } from '../../tauri_bridge';
import { fileRepository } from '../../infrastructure/editor/TauriFileRepository';
import { useStore } from '../../store';
import { markFileOpened } from '../performance/memoryGovernor';
import type { EditorTab } from '../../store/types';

const LARGE_FILE_BYTES = 2_000_000;
// Each open tab keeps a full Monaco model (text + undo + tokens) resident.
// 15 is the Cursor-like sweet spot; dirty tabs are never evicted.
const MAX_OPEN_EDITOR_TABS = 15;

/** Dispose the Monaco model backing `path` (no-op if none exists). */
export function disposeMonacoModelForPath(path: string): void {
    try {
        const monaco = (window as any).monaco;
        if (!monaco?.editor) return;
        const cleanPath = (p: string) => p.replace(/\\/g, '/').replace(/^\//, '').toLowerCase();
        const targetPath = cleanPath(path);
        const model = monaco.editor.getModels().find((m: any) => {
            const modelPath = cleanPath(m.uri.path);
            return modelPath === targetPath || decodeURIComponent(m.uri.toString()).toLowerCase().includes(targetPath);
        });
        if (model) model.dispose();
    } catch { /* best effort */ }
}

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
    await openFileInternal(path, false);
}

/** Open markdown and show VS Code–style side-by-side preview. */
export async function openFileWithMarkdownPreview(path: string): Promise<void> {
    await openFileInternal(path, true);
}

async function openFileInternal(path: string, withPreview: boolean): Promise<void> {
    markFileOpened(); // Prevent memory governor from disposing the model right after open
    const st = useStore.getState();
    const existing = st.tabs.find((t) => t.path === path);
    if (existing) {
        st.setActiveTab(existing.id);
        if (withPreview && detectLanguage(path) === 'markdown') {
            useStore.getState().openMarkdownPreview?.();
        }
        return;
    }

    // Agent canvas artifacts render as interactive dashboards, not raw JSON.
    if (path.toLowerCase().endsWith('.canvas.json')) {
        try {
            const { normalizeCanvasSpec } = await import('../../domain/canvas/CanvasSpec');
            const raw = await fileRepository.read(path);
            const spec = normalizeCanvasSpec(raw);
            if (spec) {
                st.upsertCanvas(spec, { open: true, persist: false });
                return;
            }
        } catch { /* fall through to raw JSON */ }
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

    const result = await invoke<{ large?: boolean; size?: number; lines?: number; content?: string; preview?: string }>('open_file', { path });
    const filename = path.replace(/\\/g, '/').split('/').pop() ?? path;
    const id = `tab-${Date.now()}-${Math.random()}`;
    const isLarge = result.large === true;
    const content = isLarge ? (result.preview || '') : (result.content || '');
    const tab: EditorTab = {
        id, filename, path, content, isModified: false,
        language: detectLanguage(filename),
        ...(isLarge ? { isLargePaged: true, fileSize: result.size, totalLines: result.lines } : {}),
    } as EditorTab;
    useStore.setState((state) => {
        let tabs = [...state.tabs, tab];
        // Evict oldest non-active, non-dirty tabs beyond the cap. Dirty tabs
        // (unsaved edits) are never auto-evicted.
        while (tabs.length > MAX_OPEN_EDITOR_TABS) {
            const activeId = state.activeTabId;
            const evictable = tabs.filter((t: any) => t.id !== activeId && t.id !== id && !t.isModified);
            if (evictable.length === 0) break;
            const oldest = evictable[0];
            tabs = tabs.filter((t: any) => t.id !== oldest.id);
            disposeMonacoModelForPath(oldest.path);
        }
        const history = state.tabHistory.slice(0, state.tabHistoryIndex + 1);
        history.push(id);
        return { tabs, activeTabId: id, tabHistory: history, tabHistoryIndex: history.length - 1 };
    });
    if (withPreview && tab.language === 'markdown') {
        useStore.getState().openMarkdownPreview?.();
    }
}
