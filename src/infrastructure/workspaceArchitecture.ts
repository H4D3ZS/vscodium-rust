// Adapter for the workspace architecture layout (Architecture Visualizer module).

import { invoke } from '../tauri_bridge';

export interface ArchitectureFileNode {
    id: number;
    path: string;
    functions: Array<{ name: string; line: number; type: string }>;
}

export async function fetchWorkspaceArchitecture(root?: string): Promise<ArchitectureFileNode[]> {
    return invoke<ArchitectureFileNode[]>('workspace_architecture_layout', { root: root ?? null });
}
