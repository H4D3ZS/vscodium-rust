import { showToast } from '../../components/ToastManager';

export type AgentDoneReason = 'mission' | 'stopped' | 'error';

let lastNotifyAt = 0;

function readNotifyPref(): boolean {
    try {
        return localStorage.getItem('agent.notifyOnComplete') !== 'false';
    } catch {
        return true;
    }
}

async function tryOsNotification(title: string, body: string): Promise<void> {
    if (typeof Notification === 'undefined') return;
    try {
        if (Notification.permission === 'default') {
            await Notification.requestPermission();
        }
        if (Notification.permission !== 'granted') return;
        // Always show OS toast when permitted; especially useful if IDE is in background.
        new Notification(title, { body, tag: 'vscodium-rust-agent-done' });
    } catch {
        /* non-fatal — in-app toast still shows */
    }
}

/** Inform the user that the agent loop finished (mission, stop, or error). */
export async function notifyAgentComplete(opts: {
    reason: AgentDoneReason;
    mode?: string;
    detail?: string;
}): Promise<void> {
    if (!readNotifyPref()) return;
    const now = Date.now();
    if (opts.reason === 'mission' && now - lastNotifyAt < 2500) return;
    lastNotifyAt = now;

    const mode = opts.mode || 'Agent';
    const detail = opts.detail?.trim() || '';

    let title: string;
    let toastMsg: string;
    let toastType: 'success' | 'warning' | 'info' | 'error' = 'success';

    switch (opts.reason) {
        case 'stopped':
            title = 'Agent stopped';
            toastMsg = `${mode} run stopped.`;
            toastType = 'warning';
            break;
        case 'error':
            title = 'Agent finished with errors';
            toastMsg = detail || `${mode} run ended — check the activity feed.`;
            toastType = 'error';
            break;
        default:
            title = 'Mission complete';
            toastMsg = detail
                ? `${mode}: ${detail}`
                : `${mode} finished — check reports/ and the chat for MISSION_ACCOMPLISHED.`;
            toastType = 'success';
            break;
    }

    showToast(toastMsg, toastType, 6000);
    await tryOsNotification(title, toastMsg);
}
