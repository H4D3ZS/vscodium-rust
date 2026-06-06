/**
 * Antigravity IDE compatibility — thin Tauri adapter for brain artifacts,
 * trajectories, lifecycle hooks, and autonomy policies.
 */
import { invoke } from '../../tauri_bridge';

export interface SubagentState {
    id: string;
    name: string;
    role?: string;
    status: string;
    parent_id?: string;
    started_at: number;
    summary?: string;
    progress?: number;
}

export interface TrajectoryStep {
    id: string;
    kind: string;
    title: string;
    detail?: string;
    tool?: string;
    timestamp: number;
    success?: boolean;
    subagent_id?: string;
    media_path?: string;
}

export interface TrajectoryRecord {
    id: string;
    objective: string;
    status: string;
    started_at: number;
    finished_at?: number;
    steps: TrajectoryStep[];
    subagents: SubagentState[];
    artifact_paths: string[];
    summary?: string;
}

export interface BrainArtifactInfo {
    name: string;
    path: string;
    artifact_type: string;
    summary?: string;
    updated_at: string;
    is_media: boolean;
}

export interface LifecycleHookResult {
    hook_name: string;
    event: string;
    matcher: string;
    command: string;
    exit_code: number;
    stdout: string;
    stderr: string;
}

export interface AutonomyPolicies {
    secure_mode?: boolean;
    artifact_review?: string;
    terminal_auto?: string;
    browser_js?: string;
    file_access?: string;
}

export interface TrajectoryEventInput {
    kind: string;
    title: string;
    detail?: string;
    tool?: string;
    success?: boolean;
    subagentId?: string;
    mediaPath?: string;
}

export function newCascadeId(): string {
    return `cascade-${Date.now()}`;
}

export async function agInitLayout(root: string): Promise<void> {
    await invoke('ag_init_layout', { root });
}

export async function agBrainList(root: string, cascadeId: string): Promise<BrainArtifactInfo[]> {
    return invoke<BrainArtifactInfo[]>('ag_brain_list', { root, cascadeId });
}

export async function agBrainSaveMedia(
    root: string,
    cascadeId: string,
    pngBase64: string,
    summary?: string,
): Promise<string> {
    return invoke<string>('ag_brain_save_media', {
        root,
        cascadeId,
        pngBase64,
        summary: summary ?? null,
    });
}

export async function agListTrajectories(root: string): Promise<TrajectoryRecord[]> {
    return invoke<TrajectoryRecord[]>('ag_list_trajectories', { root });
}

export async function agGetTrajectory(root: string, cascadeId: string): Promise<TrajectoryRecord | null> {
    return invoke<TrajectoryRecord | null>('ag_get_trajectory', { root, cascadeId });
}

export async function agSaveTrajectory(root: string, record: TrajectoryRecord): Promise<string> {
    return invoke<string>('ag_save_trajectory', { root, record });
}

export async function agUpsertSubagent(
    root: string,
    cascadeId: string,
    subagent: SubagentState,
): Promise<void> {
    await invoke('ag_upsert_subagent', { root, cascadeId, subagent });
}

export async function persistAgentTrajectoryEvent(
    root: string,
    cascadeId: string,
    evt: TrajectoryEventInput,
): Promise<void> {
    const step: TrajectoryStep = {
        id: `step-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`,
        kind: evt.kind,
        title: evt.title,
        detail: evt.detail,
        tool: evt.tool,
        timestamp: Date.now(),
        success: evt.success,
        subagent_id: evt.subagentId,
        media_path: evt.mediaPath,
    };
    await invoke('ag_append_trajectory_step', { root, cascadeId, step });
}

export async function agLoadLifecycleHooks(root: string): Promise<Record<string, unknown>> {
    return invoke<Record<string, unknown>>('ag_load_lifecycle_hooks', { root });
}

export async function agSaveLifecycleHooks(root: string, hooks: unknown): Promise<string> {
    return invoke<string>('ag_save_lifecycle_hooks', { root, hooks });
}

export async function agDispatchLifecycleHooks(
    root: string,
    event: string,
    context: string,
): Promise<LifecycleHookResult[]> {
    return invoke<LifecycleHookResult[]>('ag_dispatch_lifecycle_hooks', { root, event, context });
}

export async function agApplyAutonomyPreset(
    root: string,
    preset: string,
    secureMode: boolean,
): Promise<AutonomyPolicies> {
    return invoke<AutonomyPolicies>('ag_apply_autonomy_preset', {
        root,
        preset,
        secureMode,
    });
}
