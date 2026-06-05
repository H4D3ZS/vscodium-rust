import { workflowRepository } from '../../infrastructure/terminal/LocalStorageWorkflowRepository';
import { getTerminalManager } from './getTerminalManager';
import type { TerminalWorkflow } from '../../domain/terminal/TerminalWorkflow';

export function listTerminalWorkflows(): TerminalWorkflow[] {
    return workflowRepository.list();
}

export function deleteTerminalWorkflow(id: string): void {
    workflowRepository.delete(id);
}

export function saveWorkflowFromCommand(command: string): void {
    workflowRepository.saveFromCommand(command);
}

/** Insert command at prompt (user reviews before Enter). */
export async function insertWorkflowCommand(command: string): Promise<void> {
    const mgr = await getTerminalManager();
    mgr.insertInActive(command);
}

/** Insert and execute immediately (palette Ctrl+Enter). */
export async function runWorkflowCommand(command: string): Promise<void> {
    const mgr = await getTerminalManager();
    mgr.runInActive(command);
}
