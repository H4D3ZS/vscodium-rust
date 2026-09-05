import type { StateCreator } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import type { AppState } from './index';

export interface GitSlice {
    gitBranch: string;
    isGeneratingCommitMessage: boolean;
    lastGeneratedCommitMessage: string;

    setGitBranch: (branch: string) => void;
    generateAiCommitMessage: () => Promise<string>;
}

export const createGitSlice: StateCreator<AppState, [], [], GitSlice> = (set, get) => ({
    gitBranch: '',
    isGeneratingCommitMessage: false,
    lastGeneratedCommitMessage: '',

    setGitBranch: (gitBranch) => set({ gitBranch }),

    generateAiCommitMessage: async () => {
        set({ isGeneratingCommitMessage: true });
        try {
            const state = get();
            const root = state.activeRoot || '.';
            const diff = await invoke<string>('get_git_diff', { path: root, staged: true }).catch(() => '');
            if (!diff.trim()) { set({ isGeneratingCommitMessage: false }); return ''; }
            const scmSel = state.modelSelectionOfFeature?.['SCM'];
            const rawModel = scmSel?.modelName || state.agentModel || '';
            const provider = scmSel?.providerName
                || (rawModel.includes('|') ? rawModel.split('|')[0].toLowerCase() : (state.inferenceBackend === 'lemonade' ? 'lemonade' : 'openai'));
            const model = scmSel?.modelName
                || (rawModel.includes('|') ? rawModel.split('|').slice(1).join('|') : rawModel);
            const result = await invoke<{ content: string }>('ai_chat_fast', {
                request: {
                    messages: [{ role: 'user', content: `Write a concise git commit message for this diff. Format: <type>(<scope>): <description>\\n\\n<body if needed>\\n\\nTypes: feat|fix|refactor|docs|test|chore\\n\\nDiff:\\n${diff.slice(0, 4000)}` }],
                    model,
                    provider,
                    temperature: 0.3,
                    ollama_url: state.ollamaUrl,
                },
            }).catch(() => null);
            const msg = result?.content?.trim() || '';
            set({ lastGeneratedCommitMessage: msg, isGeneratingCommitMessage: false });
            return msg;
        } catch {
            set({ isGeneratingCommitMessage: false });
            return '';
        }
    },
});

