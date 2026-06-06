import { invoke } from '../../tauri_bridge';

export interface WorkspaceScan {
    cursor: Record<string, unknown>;
    steering_count: number;
    steering: SteeringDoc[];
    kiro_hooks_count: number;
    kiro_mcp_count: number;
    antigravity: {
        workflows_count: number;
        specs_root_count: number;
        kiro_specs_count: number;
        rules_count: number;
        has_state: boolean;
    };
}

export interface SteeringDoc {
    name: string;
    content: string;
    path: string;
    source: string;
}

export interface HookDispatchResult {
    hook_id: string;
    hook_name: string;
    action: string;
    prompt?: string;
    message?: string;
}

export interface KiroHookFile {
    id?: string;
    name?: string;
    comment?: string;
    when: { type: string; pattern?: string };
    then: { type: string; prompt?: string; message?: string };
    enabled?: boolean;
    file_path?: string;
}

export interface AgentRunRecord {
    id: string;
    objective: string;
    status: string;
    started_at: number;
    finished_at?: number;
    tool_count?: number;
    summary?: string;
}

export async function workspaceScan(root?: string): Promise<WorkspaceScan> {
    return invoke('workspace_scan', { root: root ?? null });
}

export async function workspaceInit(root?: string): Promise<unknown> {
    return invoke('workspace_init', { root: root ?? null });
}

export async function workspaceReload(root?: string): Promise<unknown> {
    return invoke('workspace_reload', { root: root ?? null });
}

export async function workspaceDispatchHooks(
    event: string,
    filePath: string,
    root?: string,
): Promise<HookDispatchResult[]> {
    return invoke('workspace_dispatch_hooks', { event, file_path: filePath, root: root ?? null });
}

export async function workspaceListHooks(root?: string): Promise<KiroHookFile[]> {
    return invoke('workspace_list_hooks', { root: root ?? null });
}

export async function workspaceSaveHook(
    filename: string,
    hook: KiroHookFile,
    root?: string,
): Promise<string> {
    return invoke('workspace_save_hook', { filename, hook, root: root ?? null });
}

export async function workspaceDeleteHook(filePath: string, root?: string): Promise<void> {
    return invoke('workspace_delete_hook', { file_path: filePath, root: root ?? null });
}

export async function workspaceListAgentRuns(root?: string): Promise<AgentRunRecord[]> {
    return invoke('workspace_list_agent_runs', { root: root ?? null });
}

/** Reload Cursor + Kiro + Antigravity project layout. */
export async function syncWorkspaceCompat(root?: string): Promise<void> {
    try {
        await workspaceReload(root);
        if (root) {
            const { agInitLayout } = await import('../antigravity/antigravityClient');
            await agInitLayout(root).catch(() => {});
        }
    } catch (e) {
        console.warn('[workspace] sync failed:', e);
    }
}
