import type { StateCreator } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import type { AppState } from './index';
import {
    clearWorkspaceIndexed,
    isWorkspaceMarkedIndexed,
    markWorkspaceIndexed,
} from '../lib/indexingState';

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
    /** Start vector indexing. Pass `{ force: true }` to re-index when idle (never restarts a run in progress). */
    startIndexingCodebase: (options?: { force?: boolean }) => Promise<void>;
    /** Refresh progress from backend without starting a new index. */
    refreshIndexingProgress: () => Promise<void>;
    /** Index once per workspace; skip if already indexed until user forces re-index. */
    ensureIndexingCodebase: () => Promise<void>;
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

    refreshIndexingProgress: async () => {
        await get().pollIndexingProgress();
    },

    startIndexingCodebase: async (options?: { force?: boolean }) => {
        const { activeRoot, indexingEnabled, isIndexingCodebase } = get();
        if (!activeRoot || !indexingEnabled) return;
        if (isIndexingCodebase) {
            await get().pollIndexingProgress();
            return;
        }
        if (options?.force) {
            clearWorkspaceIndexed(activeRoot);
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

    ensureIndexingCodebase: async () => {
        const { activeRoot, indexingEnabled } = get();
        if (!activeRoot || !indexingEnabled) return;

        if (isWorkspaceMarkedIndexed(activeRoot)) {
            set({ isIndexingCodebase: false });
            return;
        }

        try {
            const stats: any = await invoke('vector_get_index_stats');
            const files = stats?.total_files ?? 0;
            const chunks = stats?.total_chunks ?? 0;
            if (files > 0 && chunks > 0) {
                markWorkspaceIndexed(activeRoot, files, chunks);
                set({ isIndexingCodebase: false });
                return;
            }
        } catch { /* first index */ }

        await get().startIndexingCodebase();
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
                    const root = get().activeRoot;
                    if (
                        root
                        && !progress.is_indexing
                        && (progress.files_processed ?? 0) > 0
                    ) {
                        markWorkspaceIndexed(
                            root,
                            progress.files_processed ?? progress.total_files ?? 0,
                            progress.chunks_created,
                        );
                    }
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

