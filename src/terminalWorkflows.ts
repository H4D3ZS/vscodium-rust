/**
 * @deprecated Import from `application/terminal` or `infrastructure/terminal` instead.
 * Kept for backward compatibility with terminalBlocks dynamic import.
 */
export type { TerminalWorkflow } from './domain/terminal/TerminalWorkflow';
export {
    workflowRepository,
    LocalStorageWorkflowRepository,
} from './infrastructure/terminal/LocalStorageWorkflowRepository';

import { workflowRepository } from './infrastructure/terminal/LocalStorageWorkflowRepository';

export const getWorkflows = () => workflowRepository.list();
export const addWorkflow = (w: Parameters<typeof workflowRepository.save>[0]) => workflowRepository.save(w);
export const deleteWorkflow = (id: string) => workflowRepository.delete(id);
export const saveWorkflowFromCommand = (command: string) => workflowRepository.saveFromCommand(command);
