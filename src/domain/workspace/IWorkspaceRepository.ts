import type { WorkspacePath } from './WorkspacePath';
// WorkspacePath is `{ value: string }` — see parseWorkspacePath()

/**
 * Port — workspace root and file tree on disk / Rust backend.
 */
export interface IWorkspaceRepository {
    pathExists(path: WorkspacePath): Promise<boolean>;
    getBackendRoot(): Promise<WorkspacePath | null>;
    setActiveRoot(path: WorkspacePath): Promise<void>;
}
