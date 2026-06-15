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
    if (cmd === 'get_file_tree') return Promise.resolve([] as any);
    if (cmd === 'get_settings') return Promise.resolve({ theme: 'vs-dark', font_size: 14 } as any);
    if (cmd === 'get_config_path') return Promise.resolve('/mock/config.json' as any);
    if (cmd === 'ai_chat' || cmd === 'ai_chat_fast') {
        return Promise.reject(
            new Error('Rust backend not connected — run `npm run dev:tauri` (Vite-only dev cannot call Ollama).'),
        ) as any;
    }

    return Promise.resolve(null as any);
}

/** Map an on-disk path to a WebView-safe asset URL (no base64 IPC). */
export function convertFileSrc(filePath: string): string {
    const tauri = (window as any).__TAURI__;
    if (tauri?.core?.convertFileSrc) {
        return tauri.core.convertFileSrc(filePath);
    }
    return `asset://localhost/${encodeURIComponent(filePath)}`;
}

export async function listen<T = any>(event: string, handler: (event: { payload: T; [k: string]: any }) => void): Promise<() => void> {
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
