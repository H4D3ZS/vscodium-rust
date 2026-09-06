import { invoke } from '../tauri_bridge.ts';

export interface VisionSidecarResult {
    attachments: any[];
    skipped: boolean;
    vision_model?: string | null;
    analyzed_count: number;
    message?: string | null;
}

/** Mirrors `vision_sidecar::is_vision_capable_model` for fast UI checks. */
export function isVisionCapableModel(model: string): boolean {
    const m = (model || '').toLowerCase();
    if (m.includes('gemma4') || m.includes('gemma-4')) return true;
    if ((m.includes('gemma3') || m.includes('gemma-3')) && m.includes('vision')) return true;
    if (m.includes('llava') || m.includes('bakllava') || m.includes('moondream')) return true;
    if (m.includes('minicpm-v') || m.includes('mimo-vl')) return true;
    if ((m.includes('qwen') && m.includes('-vl')) || m.includes('qwen-vl') || m.includes('qwen2.5vl')) return true;
    if (m.includes('phi') && m.includes('vision')) return true;
    if (m.includes('nemotron') && m.includes('vl')) return true;
    return false;
}

function hasImageAttachment(items: any[]): boolean {
    return items.some((c) =>
        c?.data?.startsWith?.('data:image/') ||
        c?.thumbnail?.startsWith?.('data:image/'),
    );
}

/** When the agent model is text-only, summarize images via a local VL sidecar. */
export async function applyVisionSidecar(
    attachments: any[],
    agentModel: string,
    userPrompt: string,
    inferenceUrl?: string,
): Promise<VisionSidecarResult> {
    if (!attachments.length || !hasImageAttachment(attachments) || isVisionCapableModel(agentModel)) {
        return { attachments, skipped: true, analyzed_count: 0 };
    }

    try {
        return await invoke<VisionSidecarResult>('vision_sidecar_process_attachments', {
            agentModel,
            attachments,
            userPrompt,
            inferenceUrl: inferenceUrl?.trim() || null,
        });
    } catch (e) {
        console.warn('[vision-sidecar]', e);
        return {
            attachments,
            skipped: false,
            analyzed_count: 0,
            message: String(e),
        };
    }
}

/** Patch the last user turn so history uses text summaries instead of raw pixels. */
export function patchLastUserMessageContext(store: { getState: () => any; setState: (p: any) => void }, context: any[]): void {
    const state = store.getState();
    const msgs = [...(state.agentMessages || [])];
    for (let i = msgs.length - 1; i >= 0; i--) {
        if (msgs[i]?.role === 'user') {
            msgs[i] = { ...msgs[i], context };
            break;
        }
    }
    const threadId = state.activeAgentThreadId;
    const agentThreads = threadId && state.agentThreads?.[threadId]
        ? {
            ...state.agentThreads,
            [threadId]: { ...state.agentThreads[threadId], messages: msgs },
        }
        : state.agentThreads;
    store.setState({ agentMessages: msgs, agentThreads });
}
