import type { IProcessMemoryRepository } from '../../domain/performance/IProcessMemoryRepository';
import type { ProcessStatsDto } from '../../domain/performance/ProcessMemorySnapshot';
import { processMemoryRepository } from '../../infrastructure/tauri/TauriProcessMemoryRepository';

/**
 * Use-case: refresh IDE memory stats for the status bar.
 * Presentation layer calls this — not invoke() directly.
 */
export async function refreshProcessMemory(
    repo: IProcessMemoryRepository = processMemoryRepository,
): Promise<ProcessStatsDto | null> {
    try {
        return await repo.fetchProcessStats();
    } catch {
        return null;
    }
}

export async function optimizeProcessMemory(
    repo: IProcessMemoryRepository = processMemoryRepository,
): Promise<string> {
    return repo.optimizeMemory();
}
