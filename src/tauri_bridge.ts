/**
 * Centralized Tauri IPC Bridge
 * Provides safe access to Tauri's 'invoke' command with fallback for browser environments.
 */

const warnedCommands = new Set<string>();
const warnedEvents = new Set<string>();
const mockLogEnabled = Boolean((import.meta as any).env?.DEV || (window as any).__TAURI_MOCK_LOGS__);

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

    if (mockLogEnabled && !warnedCommands.has(cmd)) {
        console.warn(`[Tauri Bridge] MOCK INVOKE: ${cmd}`, args);
        warnedCommands.add(cmd);
    }

    // Provide mock responses for browser demo mode to avoid hard failures.
    if (cmd === 'open_folder') return Promise.resolve(null as any);
    if (cmd === 'set_ollama_url') return Promise.resolve(null as any);
    if (cmd === 'get_available_shells') return Promise.resolve(['bash', 'sh'] as any);
    if (cmd === 'get_api_keys') return Promise.resolve({} as any);
    if (cmd === 'save_api_key' || cmd === 'save_api_keys') return Promise.resolve({} as any);
    if (cmd === 'get_active_root') return Promise.resolve(null as any);
    if (cmd === 'list_directory') return Promise.resolve([] as any);
    if (cmd === 'get_file_tree') return Promise.resolve([] as any);
    if (cmd === 'read_file') return Promise.resolve('' as any);
    if (cmd === 'write_file' || cmd === 'create_dir') return Promise.resolve(null as any);
    if (cmd === 'list_provider_models') return Promise.resolve([] as any);
    if (cmd === 'list_mcp_servers') return Promise.resolve([] as any);
    if (cmd === 'get_process_stats') return Promise.resolve({ memory_mb: 0, cpu_usage: 0, total_ram_gb: 0, available_ram_gb: 0 } as any);
    if (cmd === 'get_memory_savings') return Promise.resolve([0, 0] as any);
    if (cmd === 'ext_host_init') return Promise.resolve(null as any);
    if (cmd === 'get_icon_theme_mapping') return Promise.resolve({ iconDefinitions: {} } as any);
    if (cmd === 'get_extension_contributions') return Promise.resolve({ viewsContainers: { activitybar: [] }, views: {} } as any);
    if (cmd === 'get_running_extensions') return Promise.resolve([] as any);
    if (cmd === 'get_popular_extensions') return Promise.resolve([] as any);
    if (cmd === 'search_extensions') return Promise.resolve([] as any);
    if (cmd === 'list_available_avds') return Promise.resolve([] as any);
    if (cmd === 'list_running_emulators') return Promise.resolve([] as any);
    if (cmd === 'get_android_config') return Promise.resolve({} as any);
    if (cmd === 'adb_list_devices') return Promise.resolve([] as any);
    if (cmd === 'adb_list_emulators') return Promise.resolve([] as any);
    if (cmd === 'spawn_terminal' || cmd === 'resize_terminal') return Promise.resolve(null as any);
    if (cmd === 'load_kortex_memory' || cmd === 'load_kortex_metadata') return Promise.resolve({} as any);
    if (cmd === 'airi_event') return Promise.resolve(null as any);
    if (cmd === 'airi_vision_capture_screen') return Promise.resolve(new Uint8Array() as any);
    if (cmd === 'check_ollama_status') return Promise.resolve(false as any);
    if (cmd === 'get_settings') return Promise.resolve({ theme: 'vs-dark', font_size: 14 } as any);
    if (cmd === 'get_config_path') return Promise.resolve('/mock/config.json' as any);
    if (cmd === 'ai_chat') {
        const userMessage = typeof args?.message === 'string' ? args.message : '';
        const response = userMessage
            ? `Web demo response: ${userMessage}\n\nDesktop-only tools are unavailable on GitHub Pages.`
            : 'Web demo response: AI sidebar is running in limited browser mode.';
        return Promise.resolve(response as any);
    }

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

    if (mockLogEnabled && !warnedEvents.has(event)) {
        console.warn(`[Tauri Bridge] MOCK LISTEN: ${event}`);
        warnedEvents.add(event);
    }
    return () => { };
}
