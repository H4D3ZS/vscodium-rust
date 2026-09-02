import { fileRepository } from '../../infrastructure/editor/TauriFileRepository';
import { useStore } from '../../store';

/** Save the active editor tab to disk. */
export async function saveActiveFile(): Promise<void> {
    const { tabs, activeTabId } = useStore.getState();
    const tab = tabs.find((t) => t.id === activeTabId);
    if (!tab || tab.type === 'settings') return;

    await fileRepository.write(tab.path, tab.content);
    useStore.setState((state) => ({
        tabs: state.tabs.map((t) => (t.id === activeTabId ? { ...t, isModified: false } : t)),
    }));

    const { syncDocumentSaved } = await import('../extensions/extHostDocumentSync');
    await syncDocumentSaved(tab.path).catch(() => {});

    const { runWorkspaceHooks } = await import('../workspace/runWorkspaceHooks');
    await runWorkspaceHooks('on_save', tab.path);
}
