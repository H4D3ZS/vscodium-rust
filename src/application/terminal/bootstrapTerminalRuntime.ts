import { registerTerminalKeybindings } from './registerTerminalKeybindings';

let bootstrapped = false;

/** Register global terminal keybindings once (block nav, palette, splits). */
export async function bootstrapTerminalRuntime(): Promise<void> {
    if (bootstrapped) return;
    bootstrapped = true;
    const { terminalManager } = await import('../../terminal');
    registerTerminalKeybindings(terminalManager);
}

let initialSpawned = false;

/** Create exactly one terminal on first IDE boot (StrictMode-safe). */
export async function initDefaultTerminal(): Promise<void> {
    if (initialSpawned) return;
    initialSpawned = true;
    const { spawnTerminalGroup } = await import('./spawnTerminal');
    try {
        await spawnTerminalGroup();
    } catch (e) {
        console.warn('[terminal] Failed to create initial terminal:', e);
    }
}
