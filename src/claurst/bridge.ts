// claurst/bridge.ts — frontend client for the external claurst agent backend.
//
// claurst is an optional, user-selected agent backend that runs as a SEPARATE
// process (GPL-3.0, kept at arm's length from the proprietary IDE — see
// src-tauri/src/claurst_bridge.rs). This module spawns a headless claurst run
// and streams its NDJSON output into the normal agent chat panel so it looks
// and feels like any other turn.

import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export interface ClaurstStatus {
    available: boolean;
    path?: string;
    version?: string;
    reason?: string;
}

let cachedStatus: ClaurstStatus | null = null;

export async function claurstStatus(force = false): Promise<ClaurstStatus> {
    if (cachedStatus && !force) return cachedStatus;
    try {
        cachedStatus = await invoke<ClaurstStatus>('claurst_status');
    } catch (e) {
        cachedStatus = { available: false, reason: String(e) };
    }
    return cachedStatus;
}

/**
 * Run one claurst turn and stream it into the agent chat. Adds the user message
 * and an assistant placeholder, then fills the placeholder from `claurst-stream`
 * events until the process exits.
 */
export async function runClaurstTurn(userPrompt: string): Promise<void> {
    const store = (window as any).useStore;
    const state = store.getState();

    state.addAgentMessage?.('user', userPrompt);
    state.addAgentMessage?.('assistant', '');
    state.setIsAgentThinking?.(true);

    const status = await claurstStatus();
    if (!status.available) {
        store.getState().updateLastAgentMessage?.(
            ` **Claurst backend unavailable.**\n\n${status.reason ?? 'Binary not found.'}\n\n` +
            'Build it once with:\n```powershell\ncd claurst/src-rust\ncargo build --release --bin claurst\n```\n' +
            'or set the `CLAURST_BIN` environment variable, then reselect the Claurst backend.'
        );
        store.getState().setIsAgentThinking?.(false);
        return;
    }

    let acc = '';
    let toolNote = '';
    const session = `ide-${Date.now()}`;

    const unlisten = await listen('claurst-stream', (event: any) => {
        const p = event?.payload || {};
        switch (p.type) {
            case 'text_delta':
                acc += p.text ?? '';
                store.getState().updateLastAgentMessage?.(acc + toolNote);
                break;
            case 'tool_start':
                toolNote = `\n\n_ running tool: ${p.tool}…_`;
                store.getState().updateLastAgentMessage?.(acc + toolNote);
                break;
            case 'error':
                store.getState().updateLastAgentMessage?.(`${acc}\n\n ${p.error ?? 'claurst error'}`);
                break;
            case 'done':
            default:
                break;
        }
    });

    try {
        const finalText = await invoke<string>('claurst_run', { prompt: userPrompt, sessionId: session });
        // Prefer the streamed accumulation; fall back to the returned text.
        store.getState().updateLastAgentMessage?.((acc.trim() || finalText || '').trim() || '_(no output)_');
    } catch (e: any) {
        store.getState().updateLastAgentMessage?.(`${acc}\n\n **Claurst failed:**${e?.message ?? e}`);
    } finally {
        unlisten();
        store.getState().setIsAgentThinking?.(false);
    }
}
