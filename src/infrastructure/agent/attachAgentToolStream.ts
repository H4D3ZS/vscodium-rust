/**
 * Polls the backend activity buffer and drives Cursor-style tool cards in chat.
 * Idle when the agent is not running — no 100ms polling in the background.
 */
import { invoke } from '../../tauri_bridge';
import { useStore } from '../../store';
import { canonicalToolName } from '../../domain/agent/toolAliases';
import { parseToolArgs } from '../../domain/agent/agentToolBlocks';

let attached = false;
let timer: ReturnType<typeof setInterval> | null = null;
let unsubscribe: (() => void) | null = null;

function toolResultFailed(raw: unknown): boolean {
    const rs = typeof raw === 'string' ? raw : JSON.stringify(raw ?? '');
    if (
        rs.startsWith('Error:')
        || rs.startsWith('Tool execution error:')
        || rs.startsWith('Tool not found:')
        || rs.startsWith('Unknown tool:')
        || rs.includes('"Tool not found:')
        || rs.includes('"Unknown tool:')
        || rs.includes('"status":"error"')
        || rs.includes('"status":"blocked"')
    ) {
        return true;
    }
    try {
        const j = typeof raw === 'string' ? JSON.parse(raw) : raw;
        if (j?.status === 'error' || j?.status === 'blocked' || j?.success === false) return true;
    } catch { /* */ }
    return false;
}

function handleActivity(kind: string, payload: unknown) {
    const p = (payload && typeof payload === 'object') ? payload as Record<string, unknown> : {};
    const st = useStore.getState();

    switch (kind) {
        case 'ai-tool-call': {
            const name = String(p.name || 'tool');
            const args = p.args;
            st.registerAgentToolCall(name, args, p.call_id as string | undefined);
            break;
        }
        case 'ai-tool-stdout-start': {
            const streamId = String(p.stream_id || '');
            const command = String(p.command || '');
            if (streamId) st.bindAgentToolStream(streamId, command);
            break;
        }
        case 'ai-tool-stdout': {
            const streamId = String(p.stream_id || '');
            const line = String(p.line || '');
            if (streamId && line) st.appendAgentToolOutput(streamId, line, String(p.stream || 'stdout'));
            break;
        }
        case 'ai-tool-stdout-end': {
            const streamId = String(p.stream_id || '');
            const success = p.success !== false;
            st.finishAgentToolCall('run_command', success, undefined, streamId || undefined);
            break;
        }
        case 'ai-tool-result': {
            const name = String(p.name || '');
            const raw = p.result ?? '';
            const failed = toolResultFailed(raw);
            const callId = p.call_id as string | undefined;
            if (name || callId) {
                st.finishAgentToolCall(
                    name || canonicalToolName(String(p.name || '')),
                    !failed,
                    typeof raw === 'string' ? raw.slice(0, 500) : JSON.stringify(raw).slice(0, 500),
                    undefined,
                    callId,
                );
            }
            break;
        }
        default:
            break;
    }
}

function shouldPoll(): boolean {
    const s = useStore.getState();
    return s.isAgentThinking || s.agentToolBlocks.some((b) => b.status === 'running');
}

async function drain() {
    if (!shouldPoll()) return;
    try {
        const lines = await invoke<string[]>('agent_activity_drain');
        for (const raw of lines) {
            try {
                const evt = JSON.parse(raw);
                if (evt?.kind) handleActivity(evt.kind, evt.payload);
            } catch { /* skip */ }
        }
    } catch { /* backend not ready */ }
}

function setPolling(active: boolean) {
    if (active) {
        if (!timer) {
            timer = setInterval(() => { void drain(); }, 150);
            void drain();
        }
    } else if (timer) {
        clearInterval(timer);
        timer = null;
    }
}

export function attachAgentToolStream(): void {
    if (attached) return;
    attached = true;

    unsubscribe = useStore.subscribe((state, prev) => {
        const now = state.isAgentThinking || state.agentToolBlocks.some((b) => b.status === 'running');
        const was = prev.isAgentThinking || prev.agentToolBlocks.some((b) => b.status === 'running');
        if (now !== was) setPolling(now);
    });

    setPolling(shouldPoll());
}

export function detachAgentToolStream(): void {
    unsubscribe?.();
    unsubscribe = null;
    setPolling(false);
    attached = false;
}

/** Parse args for display when listening via Tauri events (fallback). */
export function toolCallArgsPreview(args: unknown): string {
    const parsed = parseToolArgs(args);
    return JSON.stringify(parsed).slice(0, 120);
}
