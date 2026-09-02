import type { ProcessStatsDto } from './ProcessMemorySnapshot';

/**
 * Port — domain defines WHAT it needs; infrastructure defines HOW (Tauri IPC).
 */
export interface IProcessMemoryRepository {
    fetchProcessStats(): Promise<ProcessStatsDto>;
    optimizeMemory(): Promise<string>;
}
