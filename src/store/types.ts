// Shared types extracted from the monolithic store.ts.
// All slices import from here; components import from '../store'.

export interface EditorTab {
    id: string;
    filename: string;
    path: string;
    content: string;
    isModified: boolean;
    language: string;
    type?: 'file' | 'settings' | 'mcp-store' | 'aim' | 'canvas';
    diagnostics?: any[];
}

export interface AgentStep {
    name: string;
    status: 'running' | 'success' | 'error';
    args?: any;
    result?: string;
    summary?: string;
    type?: 'filesystem' | 'git' | 'terminal' | 'browser' | 'system' | 'other';
    callId?: string;
}

export interface PendingChange {
    id: string;
    path: string;
    originalContent?: string;
    proposedContent?: string;
    newContent: string;
    description?: string;
    additions?: number;
    deletions?: number;
    acceptedHunkIds?: string[];
    rejectedHunkIds?: string[];
    oldContent?: string;
    /** True when the edit is already on disk (agent applied it). Accept = keep,
     *  reject = revert to oldContent via `revert_file_content`. */
    applied?: boolean;
}

export interface Artifact {
    id: string;
    type: 'screenshot' | 'terminal_log' | 'diff' | 'task_plan' | 'walkthrough' | 'record';
    path: string;
    timestamp: number;
    title?: string;
    description?: string;
    metadata?: Record<string, any>;
    feedback?: string;
}

export interface AgentMessage {
    role: 'user' | 'assistant' | 'system' | 'tool';
    content: string;
    thoughts?: string;
    thoughtStartedAt?: number;
    thoughtDurationMs?: number;
    steps?: AgentStep[];
    toolBlocks?: import('../domain/agent/agentToolBlocks').AgentToolBlock[];
    files?: string[];
    artifacts?: Artifact[];
    context?: AttachedContext[];
    timestamp?: number;
    isSubAgentResponse?: boolean;
    checkpointId?: string;
    checkpointDescription?: string;
}

export interface DevWorkflowProject {
    id: string;
    name: string;
    currentPhase: string;
    progress: number;
    emulatorPreview: boolean;
    platform: 'ios' | 'android' | 'cross-platform';
}

export interface AttachedContext {
    type: 'attachment' | 'mention' | 'workflow' | 'file';
    id: string;
    name: string;
    data?: string;
    path?: string;
    thumbnail?: string;
}

export interface SemanticSlot {
    id: string;
    category: string;
    content: string;
    metadata?: any;
    timestamp: number;
}

export interface FileEntry {
    name: string;
    path: string;
    is_dir: boolean;
    is_expanded?: boolean;
    children?: any[];
}

export interface McpServer {
    name: string;
    config: any;
}

export interface TerminalInstance {
    id: string;
    shell: string;
    parentGroupId: string;
}

export interface WorkspaceFolder {
    path: string;
    name: string;
}

export interface TerminalGroup {
    id: string;
    name: string;
    instances: string[];
    activeInstanceId: string;
    splitWeights?: number[];
    /** Horizontal (side-by-side) or vertical (stacked) when split. */
    layout?: 'single' | 'split-horizontal' | 'split-vertical';
}

export interface AgentTask {
    id: string;
    title: string;
    summary: string;
    message?: string;
    status: 'running' | 'completed' | 'error' | 'pending' | 'failed' | 'blocked';
    progress: number;
    createdAt: number;
    updatedAt: number;
    artifacts: TaskArtifact[];
    mode?: string;
    task_status?: string;
}

export interface TaskArtifact {
    id: string;
    title: string;
    path: string;
    content: string;
    timestamp: number;
    metadata?: { reviewed?: boolean; status?: 'approved' | 'rejected' };
    feedback?: string;
    type?: string;
}
