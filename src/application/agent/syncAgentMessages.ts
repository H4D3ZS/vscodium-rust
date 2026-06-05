import { invoke } from '../../tauri_bridge';
import { useStore } from '../../store';

let persistTimer: ReturnType<typeof setTimeout> | null = null;

/** Push the live chat panel messages into the Rust memory store before archive/restore. */
export async function syncAgentMessagesToBackend(): Promise<void> {
    const { agentMessages, activeAgentThreadId, agentThreads } = useStore.getState();
    let messages = agentMessages;
    if (activeAgentThreadId && agentThreads[activeAgentThreadId]?.messages?.length) {
        messages = agentThreads[activeAgentThreadId].messages;
    }
    const payload = messages
        .filter(m => m.role === 'user' || m.role === 'assistant')
        .map(m => ({
            role: m.role,
            content: typeof m.content === 'string' ? m.content : JSON.stringify(m.content),
            tool_calls: null,
            tool_call_id: null,
            metadata: { timestamp: m.timestamp ?? Date.now() },
        }));
    if (payload.length === 0) return;
    await invoke('sync_agent_messages', { messages: payload });
}

/** Debounced sync so History always lists the live conversation. */
export function scheduleChatHistorySync(): void {
    if (persistTimer) clearTimeout(persistTimer);
    persistTimer = setTimeout(() => {
        void syncAgentMessagesToBackend().catch(() => {});
    }, 600);
}
