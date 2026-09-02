import { invoke } from '../../tauri_bridge';
import { useStore } from '../../store';

/**
 * Launch the external stealth-Firefox sidecar — a REAL OS window the user watches
 * in real time while the agent drives it. NOT the in-IDE BrowserSurface iframe.
 */
export async function launchExternalBrowser(): Promise<void> {
    const hidden = useStore.getState().browserStealthHidden;
    useStore.getState().setExternalBrowserActive(true);
        try {
        const msg = await invoke<string>('browser_open', { headless: hidden });
        console.log('[browser]', msg);
    } catch (e: unknown) {
        useStore.getState().setExternalBrowserActive(false);
        const err = e instanceof Error ? e.message : String(e);
        console.error('[browser] launch failed:', err);
        alert(
            'Could not start the external browser sidecar.\n\n' +
            'Release: browser-agent.exe should ship in binaries/\n' +
            'Dev: pip install playwright invisible_playwright\n\n' +
            err,
        );
    }
}

export async function closeExternalBrowser(): Promise<void> {
    try {
        await invoke<string>('browser_close');
    } catch { /* already closed */ }
    useStore.getState().setExternalBrowserActive(false);
}

/** Globe / activity bar / Ctrl+Shift+U — spawn or close the external window. */
export function toggleExternalBrowser(): void {
    const s = useStore.getState();
    if (s.externalBrowserActive) {
        void closeExternalBrowser();
    } else {
        void launchExternalBrowser();
    }
}

/** @deprecated Use launchExternalBrowser — kept for command registry imports. */
export const openBrowserPanel = launchExternalBrowser;
export const closeBrowserPanel = closeExternalBrowser;
export const toggleBrowserPanel = toggleExternalBrowser;
