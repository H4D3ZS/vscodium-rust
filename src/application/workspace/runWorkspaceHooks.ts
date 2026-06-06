import { workspaceDispatchHooks } from '../../infrastructure/workspace/workspaceProject';
import { useStore } from '../../store';

/** Run Kiro-style `.hooks/*.json` + legacy store hooks after a file event. */
export async function runWorkspaceHooks(event: 'on_save' | 'on_file_create', filePath: string): Promise<void> {
    const store = useStore.getState();

    try {
        const dispatched = await workspaceDispatchHooks(
            event === 'on_save' ? 'fileEdited' : 'fileCreated',
            filePath,
        );
        for (const hit of dispatched) {
            if (hit.action === 'alert' && hit.message) {
                store.addAgentMessage?.('assistant', `🔔 **Hook:** ${hit.message}`);
            }
            if (hit.action === 'askAgent' && hit.prompt) {
                const fullPrompt = `[Kiro hook: ${hit.hook_name} on ${filePath}]\n${hit.prompt}`;
                store.runBackgroundAgent?.(fullPrompt).catch(console.error);
            }
        }
    } catch (e) {
        console.warn('[hooks] workspace dispatch failed:', e);
    }

    const hooks = store.agentHooks || [];
    const matchingHooks = hooks.filter(
        (h) =>
            h.enabled
            && (h.trigger || 'on_save') === event
            && new RegExp((h.pattern || '.*').replace(/\*/g, '.*')).test(filePath),
    );
    for (const hook of matchingHooks) {
        const globalRule = store.globalSteeringRule;
        const fullPrompt = `[Triggered by ${event} on ${filePath}]\n`
            + (globalRule ? `Global Rule: ${globalRule}\n` : '')
            + `Task: ${hook.prompt}`;
        store.runBackgroundAgent?.(fullPrompt).catch(console.error);
    }
}
