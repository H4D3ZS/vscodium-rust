import { invoke } from '../../tauri_bridge';
import type { ITerminalRepository } from '../../domain/terminal/ITerminalRepository';

/** Tauri IPC adapter for PTY commands in `terminal_commands.rs`. */
export class TauriTerminalRepository implements ITerminalRepository {
    async spawn(id: string, shell?: string): Promise<void> {
        await invoke('spawn_terminal', { id, shell: shell ?? null });
    }

    async close(id: string): Promise<void> {
        await invoke('close_terminal', { id });
    }

    async send(id: string, data: string): Promise<void> {
        await invoke('terminal_send_data', { id, data });
    }

    async resize(id: string, rows: number, cols: number): Promise<void> {
        await invoke('resize_terminal', { id, rows, cols });
    }

    async takePending(id: string): Promise<string> {
        return invoke<string>('terminal_take_pending', { id });
    }

    async readOutput(id: string): Promise<string> {
        return invoke<string>('terminal_read_output', { id });
    }

    async getStatus(id: string): Promise<{ active: boolean; success?: boolean }> {
        const raw = await invoke<Record<string, unknown>>('terminal_get_status', { id });
        return {
            active: Boolean(raw.active),
            success: typeof raw.success === 'boolean' ? raw.success : undefined,
        };
    }

    async listAvailableShells(): Promise<string[]> {
        try {
            const shells = await invoke<string[]>('get_available_shells');
            return Array.isArray(shells) ? shells : [];
        } catch {
            return [];
        }
    }
}

export const terminalRepository = new TauriTerminalRepository();
