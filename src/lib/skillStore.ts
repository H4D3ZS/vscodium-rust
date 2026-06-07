import { invoke } from '@tauri-apps/api/core';

export const AGENTSKILLS_SPEC = 'https://agentskills.io/';
export const SKILLS_SH = 'https://www.skills.sh/';

export interface SkillInstallRecord {
    id: string;
    name: string;
    description: string;
    source: string;
    install_source: string;
    installed_at: string;
    path: string;
    red_team_skill: boolean;
    safe_to_use: boolean;
    blocked: boolean;
    audit_summary: string;
    warning_count: number;
    critical_count: number;
}

export interface AuditFinding {
    severity: 'info' | 'warning' | 'critical';
    category: string;
    file: string;
    line: number;
    message: string;
    snippet: string;
}

export interface SkillAuditReport {
    skill_id: string;
    skill_path: string;
    red_team_skill: boolean;
    safe_to_use: boolean;
    blocked: boolean;
    summary: string;
    findings: AuditFinding[];
    info_count: number;
    warning_count: number;
    critical_count: number;
}

export interface SkillStoreStatus {
    storeDir: string;
    installedDir: string;
    installedCount: number;
    registryPath: string;
    agentskillsSpec: string;
    skillsSh: string;
}

export async function skillStoreStatus(): Promise<SkillStoreStatus> {
    return invoke<SkillStoreStatus>('skill_store_status');
}

export async function skillStoreList(): Promise<{ skills: SkillInstallRecord[] }> {
    return invoke('skill_store_list');
}

export async function skillStoreInstall(
    source: string,
    opts?: { id?: string; force?: boolean },
): Promise<{ ok: boolean; skill: SkillInstallRecord; audit: SkillAuditReport }> {
    return invoke('skill_store_install', {
        source,
        id: opts?.id,
        force: opts?.force ?? false,
    });
}

export async function skillStoreUninstall(id: string): Promise<{ ok: boolean; id: string }> {
    return invoke('skill_store_uninstall', { id });
}

export async function skillStoreAudit(id: string): Promise<SkillAuditReport> {
    return invoke<SkillAuditReport>('skill_store_audit', { id });
}

export async function skillStoreRefresh(): Promise<{ ok: boolean; count: number }> {
    return invoke('skill_store_refresh');
}

export function auditBadge(skill: SkillInstallRecord): { label: string; color: string } {
    if (skill.blocked || skill.critical_count > 0) {
        return { label: 'Blocked', color: '#f85149' };
    }
    if (skill.red_team_skill) {
        return { label: 'Red team', color: '#d29922' };
    }
    if (skill.warning_count > 0) {
        return { label: 'Review', color: '#d29922' };
    }
    if (skill.safe_to_use) {
        return { label: 'Clean', color: '#3fb950' };
    }
    return { label: 'Unknown', color: '#8b949e' };
}
