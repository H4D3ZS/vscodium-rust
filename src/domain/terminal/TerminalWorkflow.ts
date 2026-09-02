/**
 * Saved command snippet (cmder "task" / Warp "workflow").
 * Pure terminal feature — no AI involvement.
 */
export interface TerminalWorkflow {
    id: string;
    name: string;
    command: string;
    tags?: string[];
    /** Optional cwd override (cmder task `-new_console:d:<path>`). */
    cwd?: string;
    /** Optional shell profile id; defaults to active terminal shell. */
    shell?: string;
    /** Codicon name for task launcher UI. */
    icon?: string;
}
