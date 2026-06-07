/** Per-workspace vector index readiness (survives IDE restarts). */

const STORAGE_KEY = 'vectorIndex.ready.v1';

type IndexRecord = { at: number; files: number; chunks?: number };

function readMap(): Record<string, IndexRecord> {
    try {
        return JSON.parse(localStorage.getItem(STORAGE_KEY) || '{}') as Record<string, IndexRecord>;
    } catch {
        return {};
    }
}

function normRoot(root: string): string {
    return root.trim().replace(/\\/g, '/').toLowerCase();
}

export function markWorkspaceIndexed(root: string, files: number, chunks?: number): void {
    if (!root) return;
    try {
        const map = readMap();
        map[normRoot(root)] = { at: Date.now(), files, chunks };
        localStorage.setItem(STORAGE_KEY, JSON.stringify(map));
    } catch { /* ignore */ }
}

export function isWorkspaceMarkedIndexed(root: string): boolean {
    if (!root) return false;
    return !!readMap()[normRoot(root)];
}

export function clearWorkspaceIndexed(root: string): void {
    if (!root) return;
    try {
        const map = readMap();
        delete map[normRoot(root)];
        localStorage.setItem(STORAGE_KEY, JSON.stringify(map));
    } catch { /* ignore */ }
}
