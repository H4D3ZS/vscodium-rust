import type { TerminalWorkflow } from './TerminalWorkflow';

/** Port: persisted command workflows (localStorage). */
export interface IWorkflowRepository {
    list(): TerminalWorkflow[];
    save(workflow: TerminalWorkflow): void;
    delete(id: string): void;
    saveFromCommand(command: string): void;
}
