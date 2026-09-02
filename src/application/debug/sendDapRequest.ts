import { invoke } from '../../tauri_bridge';

let seq = 1;

/** Send a JSON DAP request to the active debug adapter session. */
export async function sendDapRequest(
    command: string,
    arguments_?: Record<string, unknown>,
): Promise<void> {
    const body = JSON.stringify({
        type: 'request',
        command,
        arguments: arguments_ ?? {},
        seq: seq++,
    });
    await invoke('debug_send', { msg: body });
}

export async function startDebugSession(config: Record<string, unknown>): Promise<void> {
    await invoke('debug_start', { config });
}

export async function stopDebugSession(): Promise<void> {
    await invoke('debug_stop');
}
