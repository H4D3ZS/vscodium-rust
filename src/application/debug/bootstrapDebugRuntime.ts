import { listen } from '../../tauri_bridge';
import { useStore } from '../../store';
import { sendDapRequest } from './sendDapRequest';
import { parseDapPayload } from './dapPayload';
import type { DebugStackFrame, DebugVariable } from '../../store/debugSlice';

let attached = false;

async function refreshStackAndScopes(threadId?: number): Promise<void> {
    const threads = useStore.getState().debugThreads;
    const tid = threadId ?? threads[0]?.id;
    if (tid == null) return;

    try {
        await sendDapRequest('stackTrace', { threadId: tid });
        // Response arrives async via dap-message event — handled below.
    } catch { /* */ }
}

/** Wire DAP event stream → debug slice. Call once at app boot. */
export async function bootstrapDebugRuntime(): Promise<void> {
    if (attached) return;
    attached = true;

    await listen<string>('dap-message', (event) => {
        const msg = parseDapPayload(event.payload);
        if (!msg) return;

        const type = String(msg.type ?? '');
        const eventName = String(msg.event ?? '');
        const body = (msg.body ?? {}) as Record<string, unknown>;

        if (type === 'event') {
            useStore.getState().addDebugOutput(`[DAP] ${eventName}`);

            if (eventName === 'stopped') {
                void sendDapRequest('threads', {}).catch(() => {});
                if (typeof body.threadId === 'number') {
                    void refreshStackAndScopes(body.threadId as number);
                }
            }

            if (eventName === 'terminated' || eventName === 'exited') {
                useStore.getState().setDebugging(false);
            }

            if (eventName === 'output' && body.output) {
                useStore.getState().addDebugOutput(String(body.output));
            }
        }

        if (type === 'response' && msg.success !== false) {
            const command = String(msg.command ?? '');
            const responseBody = (msg.body ?? {}) as Record<string, unknown>;

            if (command === 'stackTrace' && Array.isArray(responseBody.stackFrames)) {
                const frames = (responseBody.stackFrames as DebugStackFrame[]).map((f) => ({
                    id: f.id,
                    name: f.name,
                    line: f.line ?? 0,
                    column: f.column ?? 0,
                    source: f.source,
                }));
                useStore.getState().setDebugStackFrames(frames);
                const frameId = frames[0]?.id;
                if (frameId != null) {
                    void sendDapRequest('scopes', { frameId });
                }
            }

            if (command === 'scopes' && Array.isArray(responseBody.scopes)) {
                const scopes = responseBody.scopes as { variablesReference?: number }[];
                const ref = scopes[0]?.variablesReference;
                if (ref) void sendDapRequest('variables', { variablesReference: ref });
            }

            if (command === 'evaluate' && responseBody.result != null) {
                useStore.getState().setLastEvaluateResult(String(responseBody.result));
                useStore.getState().addDebugOutput(`> ${String(responseBody.result)}`);
            }

            if (command === 'variables' && Array.isArray(responseBody.variables)) {
                const vars = (responseBody.variables as DebugVariable[]).map((v) => ({
                    name: v.name,
                    value: String(v.value ?? ''),
                    type: v.type,
                    variablesReference: v.variablesReference,
                }));
                useStore.getState().setDebugVariables(vars);
            }

            if (command === 'threads' && Array.isArray(responseBody.threads)) {
                useStore.getState().setDebugThreads(
                    responseBody.threads as { id: number; name: string }[],
                );
            }
        }
    });

    await listen<string>('debug-log', (event) => {
        if (event.payload) useStore.getState().addDebugOutput(String(event.payload));
    });
}

export async function pushBreakpointsToAdapter(): Promise<void> {
    const bps = useStore.getState().debugBreakpoints.filter((b) => b.enabled);
    if (bps.length === 0) return;

    const breakpoints = bps.map((b) => ({
        line: b.line,
        condition: b.condition,
    }));

    const path = bps[0].path;
    const uri = path.replace(/\\/g, '/').startsWith('/')
        ? `file://${path.replace(/\\/g, '/')}`
        : `file:///${path.replace(/\\/g, '/')}`;

    await sendDapRequest('setBreakpoints', {
        source: { path, name: path.split(/[\\/]/).pop() },
        breakpoints,
        sourceModified: false,
    });
    // Also send with source reference for adapters that need uri
    await sendDapRequest('setBreakpoints', {
        source: { path: uri },
        breakpoints,
    }).catch(() => {});
}

export async function debugContinue(): Promise<void> {
    await sendDapRequest('continue', { threadId: useStore.getState().debugThreads[0]?.id ?? 1 });
}

export async function debugStepOver(): Promise<void> {
    await sendDapRequest('next', { threadId: useStore.getState().debugThreads[0]?.id ?? 1 });
}

export async function debugStepInto(): Promise<void> {
    await sendDapRequest('stepIn', { threadId: useStore.getState().debugThreads[0]?.id ?? 1 });
}

export async function debugStepOut(): Promise<void> {
    await sendDapRequest('stepOut', { threadId: useStore.getState().debugThreads[0]?.id ?? 1 });
}
