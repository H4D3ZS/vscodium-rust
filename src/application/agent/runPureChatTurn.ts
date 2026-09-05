import { invoke } from '../../tauri_bridge';
import type { useStore } from '../../store';

type StoreState = ReturnType<typeof useStore.getState>;

const PENTEST_URL_OR_TOOL =
    /\b(https?:\/\/\S+|nmap|burp|sqlmap|metasploit|nuclei|ffuf|hydra|john|hashcat|wireshark|masscan|nikto|gobuster|dirbuster|wpscan|subfinder|amass|httpx|cve-\d{4}-\d+|xss|sqli|ssrf|lfi|rfi|rce|pentest|pen[\s-]?test|exploit|payload|recon|enumerate|fingerprint|osint)\b/i;

export function shouldForceToolLoop(prompt: string, mode: string): boolean {
    const secMode = mode === 'BugBounty' || mode === 'Bug Bounty'
        || mode === 'RedTeam' || mode === 'Red Team'
        || mode === 'BlueTeam' || mode === 'Blue Team'
        || mode === 'SecureDev' || mode === 'Secure Dev';
    if (secMode) return true;
    if (PENTEST_URL_OR_TOOL.test(prompt)) return true;
    if (/^\s*\[INTENT\s*:/i.test(prompt)) return true;
    return false;
}

/** True when we should skip the autonomous tool loop for an instant chat reply. */
export function shouldRunPureChat(prompt: string, state: StoreState): boolean {
    if (state.agentMode !== 'Chat') return false;
    if (shouldForceToolLoop(prompt, state.agentMode)) return false;
    if (state.attachedFiles?.length || state.attachedContext?.length) return false;
    return true;
}

export async function runPureChatTurn(params: {
    userPrompt: string;
    messages: { role: string; content: string; tool_calls?: unknown; metadata?: unknown }[];
    store: { getState: () => StoreState };
    onUpdate?: (text: string) => void;
    routingProvider: string;
    routingModel: string;
    routingOllamaUrl?: string;
    inferenceBackend?: string;
}): Promise<boolean> {
    const { userPrompt, messages, store, onUpdate, routingProvider, routingModel, routingOllamaUrl, inferenceBackend } = params;
    const state = store.getState();

    try {
        state.setIsAgentThinking?.(true);
        const isLocal = routingProvider === 'lemonade' || inferenceBackend === 'llama-cpp';
        const leanMessages = [
            {
                role: 'system',
                content: 'You are a helpful AI assistant inside an IDE. Answer conversationally. You are in read-only Chat mode — describe approaches but do not claim you executed tools or changed files. Suggest switching to Agent mode for actions.',
                tool_calls: null,
                metadata: null,
            },
            ...messages.slice(1).slice(-8),
        ];
        const chatCall = invoke<string>('ai_chat_fast', {
            request: {
                provider: routingProvider,
                model: routingModel,
                messages: isLocal ? leanMessages : messages,
                temperature: 0.7,
                autonomous: false,
                mode: 'Chat',
                ollama_url: routingOllamaUrl,
                tools: [],
            },
        });
        const chatTimeout = new Promise<never>((_, reject) =>
            setTimeout(() => reject(new Error(`Chat reply timed out (model=${routingModel})`)), 120_000),
        );
        const reply = await Promise.race([chatCall, chatTimeout]);
        const text = typeof reply === 'string' ? reply.trim() : '';
        const streamed = state.agentMessages.at(-1)?.content?.trim() ?? '';
        state.updateLastAgentMessage?.(text || streamed || '(no response)');
        state.setIsAgentThinking?.(false);
        try { onUpdate?.(text); } catch { /* */ }
        return true;
    } catch (e: unknown) {
        state.setIsAgentThinking?.(false);
        const msg = e instanceof Error ? e.message : String(e);
        state.updateLastAgentMessage?.(`**Chat error:** ${msg.slice(0, 300)}`);
        return true;
    }
}
