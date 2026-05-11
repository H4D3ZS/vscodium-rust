/**
 * Centralized Tauri IPC Bridge
 * Provides safe access to Tauri's 'invoke' command with fallback for browser environments.
 */

export function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
    const tauri = (window as any).__TAURI__;

    if (tauri) {
        if (tauri.core && typeof tauri.core.invoke === 'function') {
            return tauri.core.invoke(cmd, args);
        }
        if (typeof tauri.invoke === 'function') {
            return tauri.invoke(cmd, args);
        }
    }

    console.warn(`[Tauri Bridge] MOCK INVOKE: ${cmd}`, args);

    // Provide some basic mock responses for common commands to help UI testing
    if (cmd === 'open_folder') return Promise.resolve(null as any);
    if (cmd === 'list_directory') return Promise.resolve([] as any);
    if (cmd === 'get_settings') return Promise.resolve({ theme: 'vs-dark', font_size: 14 } as any);
    if (cmd === 'get_config_path') return Promise.resolve('/mock/config.json' as any);
    if (cmd === 'ai_chat') return Promise.resolve("Hello! I am your VSCODE AI assistant. How can I help you today?") as any;

    return Promise.resolve(null as any);
}

export async function listen(event: string, handler: (event: any) => void): Promise<() => void> {
    const tauri = (window as any).__TAURI__;

    if (tauri) {
        if (tauri.event && typeof tauri.event.listen === 'function') {
            return await tauri.event.listen(event, handler);
        }
        // In some Tauri configurations, event might be top-level or on core
        if (typeof tauri.listen === 'function') {
            return await tauri.listen(event, handler);
        }
    }

    console.warn(`[Tauri Bridge] MOCK LISTEN: ${event}`);
    return () => { };
}
