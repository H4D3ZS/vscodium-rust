/**
 * Keep subscription state fresh after checkout — poll + window focus sync.
 */
import { invoke } from '../tauri_bridge';

let pollTimer: ReturnType<typeof setInterval> | null = null;

export function startBillingSyncPoll(durationMs = 120_000, intervalMs = 15_000): void {
    stopBillingSyncPoll();
    const started = Date.now();
    pollTimer = setInterval(() => {
        if (Date.now() - started > durationMs) {
            stopBillingSyncPoll();
            return;
        }
        void invoke('account_sync')
            .then(() => window.dispatchEvent(new Event('account:changed')))
            .catch(() => { });
    }, intervalMs);
}

export function stopBillingSyncPoll(): void {
    if (pollTimer) {
        clearInterval(pollTimer);
        pollTimer = null;
    }
}

export function wireBillingFocusSync(): () => void {
    const onFocus = () => {
        void invoke('account_sync')
            .then(() => window.dispatchEvent(new Event('account:changed')))
            .catch(() => { });
    };
    const onVisibility = () => {
        if (document.visibilityState === 'visible') onFocus();
    };
    window.addEventListener('focus', onFocus);
    document.addEventListener('visibilitychange', onVisibility);
    return () => {
        window.removeEventListener('focus', onFocus);
        document.removeEventListener('visibilitychange', onVisibility);
    };
}
