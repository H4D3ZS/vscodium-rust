// Store composition root.
// Each domain slice owns its state + actions.
// AppState = union of all slices — backward compat preserved via re-export from ../store.ts.

import { create } from 'zustand';
import { listen } from '@tauri-apps/api/event';

import { createEditorSlice, type EditorSlice } from './editorSlice';
import { createLayoutSlice, type LayoutSlice } from './layoutSlice';
import { createAgentSlice, type AgentSlice } from './agentSlice';
import { createInferenceSlice, type InferenceSlice } from './inferenceSlice';
import { createSettingsSlice, type SettingsSlice } from './settingsSlice';
import { createGitSlice, type GitSlice } from './gitSlice';
import { createSpecsSlice, type SpecsSlice } from './specsSlice';
import { createExtensionSlice, type ExtensionSlice } from './extensionSlice';
import { createTerminalSlice, type TerminalSlice } from './terminalSlice';
import { createSecurityReviewSlice, type SecurityReviewSlice } from './securityReviewSlice';
import { createLspSlice, type LspSlice } from './lspSlice';
import { createDebugSlice, type DebugSlice } from './debugSlice';
import { createModuleSlice, type ModuleSlice } from './moduleSlice';
import { scheduleDeferredInit } from '../memory_budget';

// AppState is the full composed type — all slices merged.
export type AppState =
    EditorSlice &
    LayoutSlice &
    AgentSlice &
    InferenceSlice &
    SettingsSlice &
    GitSlice &
    SpecsSlice &
    ExtensionSlice &
    TerminalSlice &
    SecurityReviewSlice &
    LspSlice &
    DebugSlice &
    ModuleSlice & {
        // Fields that appear in cross-slice calls but aren't in a single slice
        activeProjectSpec: any | null;
    };

export const useStore = create<AppState>()((...a) => ({
    ...createEditorSlice(...a),
    ...createLayoutSlice(...a),
    ...createAgentSlice(...a),
    ...createInferenceSlice(...a),
    ...createSettingsSlice(...a),
    ...createGitSlice(...a),
    ...createSpecsSlice(...a),
    ...createExtensionSlice(...a),
    ...createTerminalSlice(...a),
    ...createSecurityReviewSlice(...a),
    ...createLspSlice(...a),
    ...createDebugSlice(...a),
    ...createModuleSlice(...a),
}));

// Wire global Tauri event listeners once (deferred — not needed for first paint).
function wireGlobalStoreListeners(): void {
    listen('sentient://patch_staged', (event: any) => {
        const { path, diff, originalContent } = event.payload;
        const state = useStore.getState() as any;
        const existingIndex = state.pendingChanges.findIndex((c: any) => c.path === path);
        if (existingIndex !== -1) {
            const updatedChanges = [...state.pendingChanges];
            updatedChanges[existingIndex] = { ...updatedChanges[existingIndex], proposedContent: diff, newContent: diff, originalContent };
            useStore.setState({ pendingChanges: updatedChanges });
        } else {
            state.proposePendingChange({ path, originalContent, proposedContent: diff, description: 'Sentient AI Surgical Patch' });
        }
    });

    let _kairosLastLog = 0;
    listen('kairos://suggestion', (event: any) => {
        const debugOn = (() => { try { return localStorage.getItem('kairos.debug') === '1'; } catch { return false; } })();
        const now = Date.now();
        if (debugOn && now - _kairosLastLog > 60_000) { _kairosLastLog = now; console.log('[KAIROS] Suggestion received (throttled):', event.payload); }
        useStore.getState().addKairosSuggestion(event.payload);
    });

    (window as any).useStore = useStore;
}

if (typeof window !== 'undefined') {
    scheduleDeferredInit(wireGlobalStoreListeners, 1_500);
}
