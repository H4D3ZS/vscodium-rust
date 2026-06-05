import { useStore } from '../../store';
import { scheduleDeferredInit } from '../../memory_budget';

let ready = false;
let bootPromise: Promise<void> | null = null;

/** Boot agent event spine once — on chat open or after idle fallback. */
export function ensureAgentRuntime(): Promise<void> {
    if (ready) return Promise.resolve();
    if (bootPromise) return bootPromise;
    bootPromise = import('../agent/bootstrapAgentRuntime')
        .then(m => m.bootstrapAgentRuntime())
        .then(() => { ready = true; })
        .catch(err => {
            bootPromise = null;
            console.error('[ensureAgentRuntime] failed:', err);
        });
    return bootPromise;
}

/** Wire chat-open + idle fallback so we skip agent spine until needed. */
export function scheduleAgentRuntimeBootstrap(): () => void {
    const unsub = useStore.subscribe((state, prev) => {
        if (state.isRightSidebarOpen && !prev.isRightSidebarOpen) {
            void ensureAgentRuntime();
        }
    });
    scheduleDeferredInit(() => { void ensureAgentRuntime(); }, 12_000);
    return unsub;
}
