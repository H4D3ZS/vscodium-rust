import type { StateCreator } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import type { AppState } from './index';

export interface SpecsSlice {
    isSpecModeActive: boolean;
    specsPrompt: string;
    isSpecsWizardOpen: boolean;
    specsWizardStep: 'generator' | 'status' | 'project';
    currentSpecProjectId: number | null;
    isIndexingCodebase: boolean;
    indexingProgress: { files_processed: number; total_files: number; progress_percent: number; current_file: string } | null;

    setSpecModeActive: (active: boolean) => void;
    setSpecsPrompt: (prompt: string) => void;
    setSpecsWizardOpen: (open: boolean) => void;
    setSpecsWizardStep: (step: 'generator' | 'status' | 'project') => void;
    setCurrentSpecProjectId: (id: number | null) => void;
    startIndexingCodebase: () => Promise<void>;
    pollIndexingProgress: () => Promise<void>;
}

export const createSpecsSlice: StateCreator<AppState, [], [], SpecsSlice> = (set, get) => ({
    isSpecModeActive: false,
    specsPrompt: '',
    isSpecsWizardOpen: false,
    specsWizardStep: 'generator',
    currentSpecProjectId: null,
    isIndexingCodebase: false,
    indexingProgress: null,

    setSpecModeActive: (active) => set({ isSpecModeActive: active }),
    setSpecsPrompt: (prompt) => set({ specsPrompt: prompt }),
    setSpecsWizardOpen: (open) => set({ isSpecsWizardOpen: open }),
    setSpecsWizardStep: (step) => set({ specsWizardStep: step }),
    setCurrentSpecProjectId: (id) => set({ currentSpecProjectId: id }),

    startIndexingCodebase: async () => {
        const { activeRoot, indexingEnabled, isIndexingCodebase } = get();
        if (!activeRoot || !indexingEnabled) return;
        if (isIndexingCodebase) {
            get().pollIndexingProgress();
            return;
        }
        set({ isIndexingCodebase: true, indexingProgress: null });
        try {
            await invoke('vector_index_codebase');
            get().pollIndexingProgress();
        } catch (e) {
            const msg = String(e ?? '');
            if (msg.includes('already in progress')) {
                get().pollIndexingProgress();
                return;
            }
            console.warn('[Indexing] vector_index_codebase failed:', e);
            set({ isIndexingCodebase: false });
        }
    },

    pollIndexingProgress: async () => {
        const poll = async () => {
            try {
                const progress: any = await invoke('vector_get_indexing_progress');
                if (progress) {
                    set({
                        isIndexingCodebase: progress.is_indexing ?? false,
                        indexingProgress: {
                            files_processed: progress.files_processed ?? 0,
                            total_files: progress.total_files ?? 0,
                            progress_percent: progress.progress_percent ?? 0,
                            current_file: progress.current_file ?? '',
                        },
                    });
                    if (progress.is_indexing) setTimeout(poll, 500);
                } else {
                    set({ isIndexingCodebase: false });
                }
            } catch {
                set({ isIndexingCodebase: false });
            }
        };
        await poll();
    },
});

