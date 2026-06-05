export type SessionPlanFileId = 'task_plan' | 'findings' | 'progress';

export interface SessionPlanFile {
    id: SessionPlanFileId;
    filename: string;
    content: string;
    exists: boolean;
}

export const SESSION_PLAN_FILENAMES: Record<SessionPlanFileId, string> = {
    task_plan: 'task_plan.md',
    findings: 'findings.md',
    progress: 'progress.md',
};
