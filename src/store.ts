// Re-export shim — maintains full backward compatibility with all existing imports.
// The implementation now lives in src/store/ slices.
// Consumers importing from '../store' or './store' continue to work unchanged.

export { useStore } from './store/index';
export type { AppState } from './store/index';
export { normalizeInferenceUrl } from './store/utils';

export type {
    EditorTab,
    AgentStep,
    PendingChange,
    Artifact,
    AgentMessage,
    DevWorkflowProject,
    AttachedContext,
    SemanticSlot,
    FileEntry,
    McpServer,
    TerminalInstance,
    TerminalGroup,
    AgentTask,
    TaskArtifact,
} from './store/types';
