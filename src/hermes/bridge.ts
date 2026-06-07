// Native Hermes skills + IDE shell status (no subprocess agent).

import { invoke } from '@tauri-apps/api/core';

export interface HermesIntegrationStatus {
    mode: string;
    subprocess: boolean;
    skillsCount: number;
    skillRoots: { path: string; source: string; exists: boolean }[];
    license?: string;
}

export interface IdeShellStatus {
    hadesHome: string;
    portableGitDir: string;
    gitBash?: string;
    sh?: string;
    ready: boolean;
    pathExtensions: string[];
    installHint: string;
    bundledInInstaller?: boolean;
    bundledSource?: string;
}

export interface IntegratedSkill {
    id: string;
    name: string;
    description: string;
    path: string;
    category: string;
    source: string;
}

export async function hermesIntegrationStatus(): Promise<HermesIntegrationStatus> {
    return invoke<HermesIntegrationStatus>('hermes_integration_status');
}

export async function listIntegratedSkills(limit = 200): Promise<{ count: number; skills: IntegratedSkill[] }> {
    return invoke('hermes_skills_list', { limit });
}

export async function getIntegratedSkill(id: string): Promise<{ skill: IntegratedSkill; body: string }> {
    return invoke('hermes_skills_get', { id });
}

export async function searchIntegratedSkills(query: string, limit = 20): Promise<{ results: IntegratedSkill[] }> {
    return invoke('hermes_skills_search', { query, limit });
}

export async function ideShellStatus(): Promise<IdeShellStatus> {
    return invoke<IdeShellStatus>('ide_shell_status');
}

export async function ideGitBashPath(): Promise<string | null> {
    const r = await invoke<{ path?: string }>('ide_git_bash_path');
    return r.path || null;
}
