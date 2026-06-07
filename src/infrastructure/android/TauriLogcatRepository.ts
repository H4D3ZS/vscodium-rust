import { invoke } from '../../tauri_bridge';
import type { ILogcatRepository } from '../../domain/android/ILogcatRepository';

export class TauriLogcatRepository implements ILogcatRepository {
    async start(device?: string, filter?: string): Promise<void> {
        await invoke('logcat_start', { device, filter });
    }

    async stop(): Promise<void> {
        await invoke('logcat_stop');
    }

    async status(): Promise<{ running: boolean }> {
        return invoke<{ running: boolean }>('logcat_status');
    }
}

export const logcatRepository = new TauriLogcatRepository();
