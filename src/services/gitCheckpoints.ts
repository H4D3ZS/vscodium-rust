import { invoke } from '../tauri_bridge';

export interface Checkpoint {
    id: string;
    name: string;
    description: string;
    timestamp: number;
    datetime: string;
    commit_hash: string;
    parent_hash: string | null;
    files_changed: number;
    is_ai_generated: boolean;
    can_rollback: boolean;
}

export interface CheckpointDiff {
    files: FileDiff[];
    total_additions: number;
    total_deletions: number;
}

export interface FileDiff {
    path: string;
    status: string;
    additions: number;
    deletions: number;
    patch: string | null;
}

export async function createCheckpoint(
    description: string,
    isAi: boolean = false
): Promise<Checkpoint> {
    return invoke<any>('git_create_checkpoint', {
        description,
        is_ai: isAi,
    });
}

export async function listCheckpoints(limit?: number): Promise<Checkpoint[]> {
    return invoke<any>('git_list_checkpoints', { limit: limit || 50 });
}

export async function rollbackToCheckpoint(checkpointId: string): Promise<string> {
    return invoke<string>('git_rollback_checkpoint', {
        checkpoint_id: checkpointId,
    });
}

export async function getCheckpointDiff(checkpointId: string): Promise<CheckpointDiff> {
    return invoke<any>('git_get_checkpoint_diff', {
        checkpoint_id: checkpointId,
    });
}

export async function deleteCheckpoint(checkpointId: string): Promise<void> {
    return invoke<void>('git_delete_checkpoint', {
        checkpoint_id: checkpointId,
    });
}

export async function autoCheckpoint(description: string): Promise<{
    checkpoint: Checkpoint | null;
    created: boolean;
}> {
    return invoke<any>('git_auto_checkpoint', { description });
}

export async function ensureCheckpointBeforeAiEdit(action: string): Promise<Checkpoint | null> {
    const result = await autoCheckpoint(`Before: ${action}`);
    return result.checkpoint;
}
