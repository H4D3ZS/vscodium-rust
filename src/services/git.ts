// Git service — typed wrappers around Tauri git commands.

import { invoke } from '@tauri-apps/api/core';

export interface GitStatus {
    path: string;
    status: string;
    staged: boolean;
}

export interface GitBlameEntry {
    line: number;
    commit: string;
    author: string;
    date: string;
    message: string;
}

export async function getStatus(root?: string): Promise<GitStatus[]> {
    return invoke<GitStatus[]>('get_git_status', root ? { root } : {});
}

export async function getBranch(): Promise<string> {
    return invoke<string>('get_git_branch').then(b => b?.trim() ?? '');
}

export async function stage(root: string, file: string): Promise<void> {
    return invoke('git_stage_file', { root, file });
}

export async function unstage(root: string, file: string): Promise<void> {
    return invoke('git_unstage_file', { root, file });
}

export async function commit(root: string, message: string): Promise<void> {
    return invoke('git_commit', { root, message });
}

export async function diffFile(root: string, file: string): Promise<string> {
    return invoke<string>('get_git_diff_file', { root, file });
}

export async function getDiff(staged: boolean): Promise<string> {
    return invoke<string>('get_git_diff', { staged }).catch(() => '');
}

export async function stash(root: string): Promise<void> {
    return invoke('git_stash', { root });
}

export async function stashPop(root: string): Promise<void> {
    return invoke('git_stash_pop', { root });
}

export async function getUnmerged(root: string): Promise<string[]> {
    return invoke<string[]>('git_get_unmerged', { root });
}

export async function createCheckpoint(description?: string): Promise<string> {
    return invoke<string>('git_create_checkpoint', { description: description ?? '' });
}

export async function rollbackCheckpoint(checkpointId: string): Promise<string> {
    return invoke<string>('git_rollback_checkpoint', { checkpointId });
}
