import { invoke } from '../../tauri_bridge';
import type { IProcessMemoryRepository } from '../../domain/performance/IProcessMemoryRepository';
import type { ProcessStatsDto } from '../../domain/performance/ProcessMemorySnapshot';

/**
 * Adapter: Tauri IPC → domain DTO.
 * WHY isolated? So StatusBar/store never call invoke() directly — swap to mock in tests.
 */
export class TauriProcessMemoryRepository implements IProcessMemoryRepository {
    async fetchProcessStats(): Promise<ProcessStatsDto> {
        return invoke<ProcessStatsDto>('get_process_stats');
    }

    async optimizeMemory(): Promise<string> {
        return invoke<string>('optimize_memory');
    }
}

/** Singleton — one adapter per renderer process. */
export const processMemoryRepository = new TauriProcessMemoryRepository();
