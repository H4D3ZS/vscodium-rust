import type { AttachedContext } from '../../store/types';

/**
 * Value object: one user turn sent to the agent engine.
 * WHY a typed request? Presentation (RightSidebar) maps UI state → this shape;
 * application layer validates; infrastructure sends to Rust/the local backend.
 */
export interface AgentTurnRequest {
    prompt: string;
    context?: AttachedContext[];
    /** Optional streaming callback — legacy engine only. */
    onStreamChunk?: (chunk: string) => void;
}
