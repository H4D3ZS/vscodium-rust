// AI service — wraps raw invoke calls behind a typed API.
// Components import from here, not raw invoke().

import { invoke } from '@tauri-apps/api/core';

export interface ChatMessage {
    role: 'system' | 'user' | 'assistant';
    content: string;
}

export interface ChatCompletionOptions {
    temperature?: number;
    max_tokens?: number;
    stream?: boolean;
    ollama_url?: string;
    provider?: string;
    autonomous?: boolean;
    root_access?: boolean;
    mode?: string;
}

export interface ChatCompletionResult {
    content: string;
    thoughts?: string;
}

export async function chatCompletion(
    messages: ChatMessage[],
    model: string,
    opts: ChatCompletionOptions = {},
): Promise<ChatCompletionResult> {
    const provider = model.includes('|') ? model.split('|')[0].toLowerCase() : (opts.provider ?? 'ollama');
    const modelId = model.includes('|') ? model.split('|').slice(1).join('|') : model;
    return invoke<ChatCompletionResult>('ai_chat_fast', {
        request: {
            messages,
            model: modelId,
            provider,
            temperature: opts.temperature ?? 0.7,
            ollama_url: opts.ollama_url,
        },
    });
}

export async function inlineComplete(
    prefix: string,
    suffix: string,
    lang: string,
    path: string,
): Promise<string> {
    return invoke<string>('ai_inline_complete', { prefix, suffix, language: lang, filePath: path });
}

export async function generateCommitMessage(diff: string, model: string, ollamaUrl: string): Promise<string> {
    const provider = model.includes('|') ? model.split('|')[0].toLowerCase() : 'ollama';
    const modelId = model.includes('|') ? model.split('|').slice(1).join('|') : model;
    const result = await invoke<{ content: string }>('ai_chat_fast', {
        request: {
            messages: [{
                role: 'user',
                content: `Write a concise git commit message for this diff.\nFormat: <type>(<scope>): <description>\n\nTypes: feat|fix|refactor|docs|test|chore\n\nDiff:\n${diff.slice(0, 4000)}`,
            }],
            model: modelId,
            provider,
            temperature: 0.3,
            ollama_url: ollamaUrl,
        },
    }).catch(() => null);
    return result?.content?.trim() ?? '';
}
