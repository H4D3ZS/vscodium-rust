import { ensureSessionPlanFiles } from '../research/loadSessionPlanFiles';
import { fileRepository } from '../../infrastructure/editor/TauriFileRepository';

export async function persistManusReport(
    workspaceRoot: string,
    query: string,
    report: string,
): Promise<void> {
    const root = workspaceRoot.replace(/\//g, '\\');
    await ensureSessionPlanFiles(workspaceRoot);

    const stamp = new Date().toISOString();
    const findingsPath = `${root}\\findings.md`;
    const progressPath = `${root}\\progress.md`;
    const planPath = `${root}\\task_plan.md`;

    try {
        const findings = await fileRepository.read(findingsPath).catch(() => '# Findings\n\n');
        await fileRepository.write(
            findingsPath,
            `${findings}\n\n---\n## Web mission · ${stamp}\n**Query:** ${query}\n\n${report.slice(0, 24000)}\n`,
        );
    } catch { /* */ }

    try {
        const progress = await fileRepository.read(progressPath).catch(() => '# Progress Log\n\n');
        await fileRepository.write(
            progressPath,
            `${progress}\n- [${stamp}] Web mission completed: ${query.slice(0, 120)}\n`,
        );
    } catch { /* */ }

    try {
        const plan = await fileRepository.read(planPath).catch(() => '');
        if (!plan.includes(query.slice(0, 40))) {
            await fileRepository.write(
                planPath,
                `${plan}\n\n## Active goal\n${query}\n\n## Status\nWeb mission pipeline finished ${stamp}\n`,
            );
        }
    } catch { /* */ }
}
