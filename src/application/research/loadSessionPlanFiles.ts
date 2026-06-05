import type { SessionPlanFile, SessionPlanFileId } from '../../domain/research/SessionPlanFile';
import { SESSION_PLAN_FILENAMES } from '../../domain/research/SessionPlanFile';
import { fileRepository } from '../../infrastructure/editor/TauriFileRepository';
import { invoke } from '../../tauri_bridge';

const TEMPLATES: Record<SessionPlanFileId, string> = {
    task_plan: `# Task Plan\n\n## Goal\n[Describe the end state]\n\n## Current Phase\nPhase 1\n\n## Phases\n### Phase 1: Discovery\n- [ ] Understand requirements\n- **Status:** in_progress\n`,
    findings: `# Findings\n\n## Summary\n[Research notes and discoveries]\n`,
    progress: `# Progress Log\n\n## Session\n- Started session notes\n`,
};

async function pathExists(path: string): Promise<boolean> {
    try {
        return await invoke<boolean>('path_exists', { path });
    } catch {
        return false;
    }
}

export async function loadSessionPlanFiles(workspaceRoot: string): Promise<SessionPlanFile[]> {
    const root = workspaceRoot.replace(/\//g, '\\');
    const ids = Object.keys(SESSION_PLAN_FILENAMES) as SessionPlanFileId[];

    return Promise.all(ids.map(async (id) => {
        const filename = SESSION_PLAN_FILENAMES[id];
        const fullPath = `${root}\\${filename}`;
        const exists = await pathExists(fullPath);
        let content = TEMPLATES[id];
        if (exists) {
            try {
                content = await fileRepository.read(fullPath);
            } catch {
                content = TEMPLATES[id];
            }
        }
        return { id, filename, content, exists };
    }));
}

export async function ensureSessionPlanFiles(workspaceRoot: string): Promise<SessionPlanFile[]> {
    const root = workspaceRoot.replace(/\//g, '\\');
    const ids = Object.keys(SESSION_PLAN_FILENAMES) as SessionPlanFileId[];

    for (const id of ids) {
        const filename = SESSION_PLAN_FILENAMES[id];
        const fullPath = `${root}\\${filename}`;
        if (!(await pathExists(fullPath))) {
            await fileRepository.write(fullPath, TEMPLATES[id]);
        }
    }

    return loadSessionPlanFiles(workspaceRoot);
}
