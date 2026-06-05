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

    const hooks = useStore.getState().agentHooks;
    const matchingHooks = hooks.filter(
        (h) =>
            h.enabled
            && (h.trigger || 'on_save') === 'on_save'
            && new RegExp(h.pattern.replace(/\*/g, '.*')).test(tab.path),
    );
    for (const hook of matchingHooks) {
        const globalRule = useStore.getState().globalSteeringRule;
        const fullPrompt = `[Triggered by save on ${tab.path}]\n`
            + (globalRule ? `Global Rule: ${globalRule}\n` : '')
            + `Task: ${hook.prompt}`;
        useStore.getState().runBackgroundAgent(fullPrompt).catch(console.error);
    }
}
