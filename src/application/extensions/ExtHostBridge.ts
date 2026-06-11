import { invoke, listen } from '../../tauri_bridge';
import { showToast } from '../../components/ToastManager';
import { useStore } from '../../store';
import { get as getUiSetting } from '../../infrastructure/SettingsRepository';

type Resolver = (value: unknown) => void;

const pendingUiRequests = new Map<string, Resolver>();
const pendingProviderRequests = new Map<string, Resolver>();
const extensionCommands = new Map<string, string>();

let bridgeWired = false;
let providerCounter = 0;

export function getExtensionCommands(): Map<string, string> {
    return extensionCommands;
}

export async function extHostSend(payload: Record<string, unknown>): Promise<void> {
    if (!(window as any).__TAURI__) return;
    await invoke('ext_host_send', { msg: JSON.stringify(payload) });
}

export async function requestExtHostProvider(
    type: string,
    params: Record<string, unknown>,
    timeoutMs = 5000,
): Promise<unknown> {
    if (!(window as any).__TAURI__) return null;
    const reqId = `fe_${Date.now()}_${++providerCounter}`;
    return new Promise((resolve) => {
        const timer = setTimeout(() => {
            pendingProviderRequests.delete(reqId);
            resolve(null);
        }, timeoutMs);
        pendingProviderRequests.set(reqId, (result) => {
            clearTimeout(timer);
            resolve(result);
        });
        void extHostSend({ type, reqId, ...params });
    });
}

function uriToPath(uri: string): string {
    return uri.replace(/^file:\/\/\/?/, '');
}

function applyTextEdit(content: string, range: any, newText: string): string {
    const lines = content.split('\n');
    const startLine = range.start?.line ?? 0;
    const endLine = range.end?.line ?? startLine;
    const startChar = range.start?.character ?? 0;
    const endChar = range.end?.character ?? 0;
    const prefix = [
        ...lines.slice(0, startLine),
        (lines[startLine] ?? '').slice(0, startChar),
    ].join('\n');
    const suffix = [
        (lines[endLine] ?? '').slice(endChar),
        ...lines.slice(endLine + 1),
    ].join('\n');
    const joiner = startLine > 0 && startChar === 0 ? '\n' : '';
    const midJoiner = endLine > startLine ? '\n' : '';
    if (startLine === endLine) {
        const line = lines[startLine] ?? '';
        return lines.slice(0, startLine).join('\n')
            + (startLine > 0 ? '\n' : '')
            + line.slice(0, startChar) + newText + line.slice(endChar)
            + (lines.length > endLine + 1 ? '\n' : '')
            + lines.slice(endLine + 1).join('\n');
    }
    return prefix + (prefix && startChar > 0 ? '' : joiner) + newText + midJoiner + suffix;
}

function applyWorkspaceEdits(edits: any[]): void {
    const store = useStore.getState();
    for (const edit of edits) {
        const path = uriToPath(String(edit.uri ?? ''));
        const tab = store.tabs.find((t) =>
            t.path === path
            || t.path.replace(/\\/g, '/') === path.replace(/\\/g, '/'),
        );
        if (!tab) continue;
        const next = applyTextEdit(tab.content, edit.range, edit.newText ?? '');
        store.updateTabContent(tab.id, next);
        const editor = (window as any).activeEditor;
        const model = editor?.getModel?.();
        if (model && (model.uri?.fsPath === tab.path || model.uri?.path === tab.path)) {
            model.setValue(next);
        }
    }
}

function registerExtensionCommand(id: string, title?: string): void {
    extensionCommands.set(id, title ?? id);
    const registry: Array<{ id: string; label: string; run: () => void }> =
        (window as any).commandRegistry ?? [];
    if (registry.some((c) => c.id === id)) return;
    registry.push({
        id,
        label: title ?? id,
        run: () => { void executeExtensionCommand(id); },
    });
    (window as any).commandRegistry = registry;
}

export async function executeExtensionCommand(id: string, args: unknown[] = []): Promise<void> {
    await invoke('check_activation_event', { event: `onCommand:${id}` }).catch(() => {});
    await extHostSend({ type: 'executeCommand', id, args });
}

function mergeContributedCommands(contributions: any): void {
    const blocks = contributions?.commands;
    if (!Array.isArray(blocks)) return;
    for (const block of blocks) {
        const value = block?.value;
        const list = Array.isArray(value) ? value : (value?.commands ?? []);
        for (const cmd of list) {
            const id = cmd?.command ?? cmd?.id;
            if (id) registerExtensionCommand(id, cmd?.title ?? cmd?.label ?? id);
        }
    }
}

async function handleExtHostMessage(raw: string): Promise<void> {
    let msg: any;
    try {
        msg = JSON.parse(raw);
    } catch {
        return;
    }

    if (msg._reqId && pendingUiRequests.has(msg._reqId)) {
        pendingUiRequests.get(msg._reqId)?.(msg.result ?? null);
        pendingUiRequests.delete(msg._reqId);
        return;
    }

    if (msg.type === 'providerResult' && msg.reqId && pendingProviderRequests.has(msg.reqId)) {
        const payload = msg.kind === 'completion'
            ? { items: msg.items ?? [] }
            : (msg.result ?? msg.items ?? null);
        pendingProviderRequests.get(msg.reqId)?.(payload);
        pendingProviderRequests.delete(msg.reqId);
        return;
    }

    switch (msg.type) {
        case 'notification': {
            const level = msg.level === 'error' ? 'error'
                : msg.level === 'warning' ? 'warning'
                    : msg.level === 'info' ? 'info' : 'info';
            showToast(String(msg.message ?? ''), level);
            break;
        }
        case 'openFile': {
            const p = String(msg.path ?? '');
            if (p) void useStore.getState().openFile(p);
            break;
        }
        case 'applyEdit':
            applyWorkspaceEdits(msg.edit ?? []);
            break;
        case 'commandRegistered':
            registerExtensionCommand(String(msg.id ?? ''), String(msg.title ?? msg.id ?? ''));
            break;
        case 'createWebviewPanel':
            showToast(`Extension webview: ${msg.title ?? msg.viewType ?? 'panel'}`, 'info');
            break;
        case 'outputChannel':
            window.dispatchEvent(new CustomEvent('ext-host:output', {
                detail: { channel: msg.channel, text: msg.text },
            }));
            break;
        case 'extensionActivated':
            console.log('[ext-host] activated', msg.id);
            break;
        case 'extensionActivationFailed':
            showToast(`Extension failed: ${msg.id} — ${msg.error}`, 'error', 8000);
            break;
        case 'ready':
            console.log(`[ext-host] ready (${msg.count ?? 0} extensions scanned)`);
            break;
        case 'showInputBox': {
            const reqId = msg._reqId;
            const prompt = msg.options?.prompt ?? 'Input';
            const value = window.prompt(prompt, msg.options?.value ?? '');
            if (reqId) await extHostSend({ _reqId: reqId, result: value ?? undefined });
            break;
        }
        case 'showQuickPick': {
            const reqId = msg._reqId;
            const items: string[] = (msg.items ?? []).map((i: any) =>
                typeof i === 'string' ? i : (i.label ?? i.description ?? String(i)),
            );
            const choice = items.length === 1
                ? items[0]
                : window.prompt(`Choose:\n${items.map((x, i) => `${i + 1}. ${x}`).join('\n')}`, items[0]);
            if (reqId) await extHostSend({ _reqId: reqId, result: choice ?? undefined });
            break;
        }
        // ── HADES API (Milestone E) ──────────────────────────────────────
        case 'permissionDenied':
            showToast(
                `Extension "${msg.extension}" tried capability "${msg.capability}" without declaring it — denied`,
                'warning',
                8000,
            );
            break;
        case 'settingsGet': {
            // hades.settings.get round-trip: answer from the settings registry cache.
            if (msg._reqId) {
                await extHostSend({ _reqId: msg._reqId, result: getUiSetting(String(msg.key)) ?? null });
            }
            break;
        }
        case 'executeUiCommand': {
            // hades.commands.execute for a command the sidecar doesn't own.
            const registry: Array<{ id: string; run: () => unknown }> =
                (window as any).commandRegistry ?? [];
            const cmd = registry.find((c) => c.id === msg.id);
            let result: unknown = null;
            try { result = cmd ? await cmd.run() : null; } catch { result = null; }
            if (msg._reqId) await extHostSend({ _reqId: msg._reqId, result });
            break;
        }
        case 'error':
            console.error('[ext-host]', msg.message);
            break;
        default:
            break;
    }
}

export async function refreshExtensionCommandRegistry(): Promise<void> {
    if (!(window as any).__TAURI__) return;
    try {
        const contributions = await invoke<any>('get_extension_contributions');
        mergeContributedCommands(contributions);
    } catch { /* ignore */ }
}

export async function wireExtHostBridge(): Promise<() => void> {
    if (bridgeWired || !(window as any).__TAURI__) return () => {};
    bridgeWired = true;

    (window as any).__extensionCommands = extensionCommands;
    (window as any).executeExtensionCommand = executeExtensionCommand;

    const unlistenMsg = await listen<string>('ext-host-message', (event) => {
        void handleExtHostMessage(String(event.payload ?? ''));
    });
    const unlistenLog = await listen<string>('ext-host-log', (event) => {
        console.warn('[ext-host:stderr]', event.payload);
    });

    await refreshExtensionCommandRegistry();
    await invoke('check_activation_event', { event: 'onStartupFinished' }).catch(() => {});

    // Forward settings changes to extensions (hades.settings.onChange).
    window.addEventListener('hades:settings-changed', (e: Event) => {
        const detail = (e as CustomEvent).detail as { key: string; value: unknown };
        void extHostSend({ type: 'settingsChanged', key: detail.key, value: detail.value });
    });

    return () => {
        bridgeWired = false;
        unlistenMsg();
        unlistenLog();
    };
}
