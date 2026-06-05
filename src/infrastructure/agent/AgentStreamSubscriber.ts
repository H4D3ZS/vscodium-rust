/**
 * Infrastructure: Tauri event → Zustand store bridge for agent streaming.
 *
 * WHY extracted from agent.ts?
 * - agent.ts was 4k lines; listeners ran at import time risk.
 * - React owns the message list — we removed legacy DOM writes (saves RAM + duplicate nodes).
 * - Single place to audit which events mutate store state.
 */
import { listen } from '../../tauri_bridge';
import { invoke } from '../../tauri_bridge';
import { useStore } from '../../store';
import { extractSearchReplaceBlocks } from '../../model_capabilities';
import { MAX_WEBUI_RESPONSE_CACHE_ENTRIES } from '../../domain/agent/AgentSessionPolicy';
import { SubAgentManager } from '../../task_manager';
import {
    navigatePendingChange,
    acceptFocusedPendingChange,
    rejectFocusedPendingChange,
} from '../../application/editor/navigatePendingChange';
import { notifyAgentComplete } from '../../application/agent/notifyAgentComplete';

let attached = false;

function boundedWebUiCache(): Record<string, string> {
    const w = window as any;
    if (!w.__hadesWebUiResponseCache) w.__hadesWebUiResponseCache = {};
    const cache = w.__hadesWebUiResponseCache as Record<string, string>;
    const keys = Object.keys(cache);
    while (keys.length > MAX_WEBUI_RESPONSE_CACHE_ENTRIES) {
        delete cache[keys[0]];
        keys.shift();
    }
    return cache;
}

/** Composer diff review shortcuts (Alt+J/K/Enter/Shift+Backspace). */
export function registerAgentKeyboardShortcuts(): void {
    window.addEventListener('keydown', (e: KeyboardEvent) => {
        if (!e.altKey) return;
        if (e.key === 'j' || e.key === 'J') {
            e.preventDefault();
            navigatePendingChange('next');
        } else if (e.key === 'k' || e.key === 'K') {
            e.preventDefault();
            navigatePendingChange('prev');
        } else if (e.key === 'Enter') {
            e.preventDefault();
            acceptFocusedPendingChange();
        } else if (e.key === 'Backspace' && e.shiftKey) {
            e.preventDefault();
            rejectFocusedPendingChange();
        }
    });
}

export async function attachAgentStreamSubscriber(): Promise<void> {
    if (attached) return;
    attached = true;

    listen('ai-content', (event: any) => {
        const { updateLastAgentMessage, setIsAgentThinking } = useStore.getState();
        setIsAgentThinking(false);
        const content = typeof event.payload === 'object' && event.payload.content
            ? event.payload.content
            : (typeof event.payload === 'string' ? event.payload : '');
        updateLastAgentMessage(content);
        if (/MISSION_ACCOMPLISHED|TASK_COMPLETE/i.test(content)) {
            const mode = useStore.getState().agentMode || 'Agent';
            void notifyAgentComplete({
                reason: 'mission',
                mode,
                detail: 'Deliverables ready — review reports/ exploits/ recon/',
            });
        }
        import('../../application/agent/syncAgentMessages').then(m => m.scheduleChatHistorySync()).catch(() => {});

        const stFa = useStore.getState() as any;
        if (stFa.betaFastApply !== false && content.includes('<<<<<<< ORIGINAL')) {
            const blocks = extractSearchReplaceBlocks(content);
            const activeFile = stFa.activeEditorPath || stFa.tabs?.find((t: any) => t.id === stFa.activeTabId)?.path;
            if (blocks.length > 0 && activeFile) {
                blocks.forEach((blk: any) => {
                    invoke('propose_file_change', {
                        path: activeFile,
                        searchText: blk.original,
                        replaceText: blk.updated,
                    }).catch(() => { /* non-fatal */ });
                });
            }
        }
    });

    listen('ai-content-delta', (event: any) => {
        const { appendLastAgentMessage } = useStore.getState();
        const delta = typeof event.payload === 'object' && event.payload.delta
            ? event.payload.delta
            : (typeof event.payload === 'string' ? event.payload : '');
        if (delta) appendLastAgentMessage(delta);
    });

    // React chat panel owns rendering — no DOM injection for aim-active.

    listen('session-captured', (event: any) => {
        const { setSession, setAiStatus, refreshAvailableModels, addAgentMessage } = useStore.getState() as any;
        setSession?.(event.payload);
        const { provider, cookies, userAgent } = event.payload;
        invoke('save_ai_session', {
            session: { provider, cookies, user_agent: userAgent },
        }).then(() => {
            setAiStatus?.('alive');
            refreshAvailableModels?.(provider);
            addAgentMessage?.('assistant', `✅ Session for **${provider}** synced successfully.`);
        }).catch(err => console.error('save_ai_session failed:', err));
    });

    window.addEventListener('airi:ai-content-delta' as any, (event: CustomEvent) => {
        const { appendLastAgentMessage } = useStore.getState();
        if (event.detail?.delta) appendLastAgentMessage(event.detail.delta);
    });

    listen<any>('ai-tool-call', (event) => {
        const { addAgentStep } = useStore.getState();
        const toolName = event.payload.name || 'tool_call';
        let type: any = 'other';
        if (toolName.startsWith('git_')) type = 'git';
        else if (toolName.startsWith('terminal_')) type = 'terminal';
        else if (toolName.includes('file') || toolName.includes('glob')) type = 'filesystem';
        else if (toolName.startsWith('browser_')) type = 'browser';
        else if (toolName.includes('health') || toolName.includes('system')) type = 'system';
        addAgentStep(toolName, type);
    });

    listen<any>('update-agent-task', (event) => {
        useStore.getState().updateAgentTask({ ...event.payload, updatedAt: Date.now() });
    });

    listen<any>('add-agent-step', (event) => {
        useStore.getState().addAgentStep(event.payload.name, event.payload.type || 'other', {});
    });

    listen<any>('notify-user', (event) => {
        const { setAgentBlocked, addAgentMessage } = useStore.getState();
        const { message, blocked } = event.payload;
        setAgentBlocked(blocked);
        addAgentMessage('assistant', blocked ? `⚠️ **Action Required**: ${message}` : `ℹ️ ${message}`);
    });

    listen<any>('ai-artifact', (event) => {
        useStore.getState().addAgentArtifact(event.payload);
    });

    listen<any>('webui-response', (event) => {
        const payload = event.payload || {};
        const text = String(payload.text || '').trim();
        if (!text) return;
        const key = `${payload.provider || 'webui'}:${payload.window || ''}`;
        const cache = boundedWebUiCache();
        if (cache[key] === text) return;
        cache[key] = text;
        useStore.getState().addAgentMessage('assistant', `### ${payload.provider || 'WebUI'} response\n\n${text}`);
    });

    listen<any>('propose-edit', (event) => {
        const { proposePendingChange } = useStore.getState();
        const { path, old_content, new_content, description } = event.payload;
        proposePendingChange({
            path,
            oldContent: old_content,
            newContent: new_content,
            description: description || 'AI suggested modification',
        });
    });

    listen<any>('subagent-progress', (event) => {
        SubAgentManager.handleProgress(event.payload);
    });

    listen<string>('ai-action', (event: any) => {
        useStore.getState().setAgentCurrentAction(event.payload);
    });

    listen<string>('ai-stopped', () => {
        const state = useStore.getState();
        state.setIsAgentPaused(false);
        state.setAgentCurrentAction(null);
        state.setIsAgentThinking(false);
        void notifyAgentComplete({
            reason: 'stopped',
            mode: state.agentMode || 'Agent',
        });
    });

    listen<any>('ai-mission-complete', (event) => {
        const mode = event.payload?.mode || useStore.getState().agentMode || 'Agent';
        useStore.getState().pushTrajectoryEvent?.({
            kind: 'tool_result',
            tool: 'mission',
            title: '✓ Mission complete',
            detail: `${mode} finished — review reports/ exploits/ recon/ for deliverables`,
            success: true,
        });
        void notifyAgentComplete({
            reason: 'mission',
            mode,
            detail: 'Mission accomplished — review your artifacts.',
        });
    });
}
