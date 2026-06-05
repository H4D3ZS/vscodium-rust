import { invoke } from '../../tauri_bridge';
import { useStore } from '../../store';

/**
 * Launch the external stealth-Firefox sidecar — a REAL OS window the user watches
 * in real time while the agent drives it. NOT the in-IDE BrowserSurface iframe.
 */
export async function launchExternalBrowser(): Promise<void> {
    useStore.getState().setExternalBrowserActive(true);
    try {
        const msg = await invoke<string>('browser_open');
        console.log('[browser]', msg);
    } catch (e: unknown) {
        useStore.getState().setExternalBrowserActive(false);
        const err = e instanceof Error ? e.message : String(e);
        alert(
            'Could not launch the external browser.\n\n' +
            'Install stealth Firefox support:\n' +
            '  pip install playwright invisible_playwright\n\n' +
            'Then click the globe again (first run downloads Firefox).\n\n' +
            'Error: ' + err,
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
