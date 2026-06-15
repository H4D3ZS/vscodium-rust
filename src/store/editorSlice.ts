import type { StateCreator } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import { computeDiffBlocks, patchContentSelective } from '../domain/editor/DiffService';
import type { AppState } from './index';
import type { EditorTab, FileEntry, PendingChange, WorkspaceFolder } from './types';

export interface EditorSlice {
    // State
    activeTabId: string | null;
    tabs: EditorTab[];
    fileTree: FileEntry[];
    fileTreeLoading: boolean;
    fileTreeError: string | null;
    activeEditorPath: string;
    activeRoot: string | null;
    activeRootName: string | null;
    workspaceFolders: WorkspaceFolder[];
    recentWorkspaces: { path: string; name: string; openedAt: number }[];
    pendingChanges: PendingChange[];
    autoAcceptChanges: boolean;
    checkpoint: Record<string, string> | null;
    tabHistory: string[];
    tabHistoryIndex: number;
    splitEditorTabId: string | null;
    isSplitEditorOpen: boolean;
    focusedHunkId: string | null;
    isMarkdownPreviewOpen: boolean;
    markdownPreviewWidthPct: number;
    isVisualLabOpen: boolean;
    isVisualLabFullScreen: boolean;
    isVisualLabSplitView: boolean;
    visualLabMode: 'none' | 'json' | 'flow' | 'erd' | 'summary';
    visualLabData: any;
    openWorkflowFiles: string[];
    openRuleFiles: string[];
    diagnosticsMap: Record<string, { severity: number; message: string; startLine: number; startCol: number; endLine: number; endCol: number; source: string; code: string }[]>;

    // Actions
    openFile: (path: string) => Promise<void>;
    closeTab: (id: string) => void;
    setActiveTab: (id: string) => void;
    updateTabContent: (id: string, content: string) => void;
    saveActiveFile: () => Promise<void>;
    setActiveRoot: (path: string | null) => void;
    setActiveEditorPath: (path: string) => void;
    refreshFileTree: () => Promise<void>;
    toggleDirectory: (path: string) => Promise<void>;
    closeFolder: () => void;
    addWorkspaceFolder: (path: string) => Promise<void>;
    removeWorkspaceFolder: (path: string) => Promise<void>;
    removeRecentWorkspace: (path: string) => void;
    getFlattenedFiles: () => FileEntry[];
    openSettings: (tab?: 'user' | 'workspace' | 'agent') => void;
    openMcpStore: (view?: 'store' | 'manage') => void;
    welcomeForceVisible: boolean;
    showWelcomeTab: () => void;
    setWelcomeForceVisible: (visible: boolean) => void;
    navigateBack: () => void;
    navigateForward: () => void;
    setSplitEditorTab: (tabId: string | null) => void;
    toggleSplitEditor: () => void;
    proposePendingChange: (change: Omit<PendingChange, 'id'>) => void;
    acceptPendingChange: (id: string) => Promise<void>;
    rejectPendingChange: (id: string) => void;
    acceptAllPendingChanges: () => Promise<void>;
    rejectAllPendingChanges: () => void;
    setAutoAcceptChanges: (v: boolean) => void;
    snapshotCheckpoint: (paths: string[]) => Promise<void>;
    revertToCheckpoint: () => Promise<void>;
    acceptHunk: (changeId: string, hunkId: string) => Promise<void>;
    rejectHunk: (changeId: string, hunkId: string) => void;
    setFocusedHunk: (id: string | null) => void;
    acceptFocusedHunk: () => void;
    rejectFocusedHunk: () => void;
    openMarkdownPreview: () => void;
    closeMarkdownPreview: () => void;
    toggleMarkdownPreview: () => void;
    setMarkdownPreviewWidthPct: (pct: number) => void;
    setVisualLabMode: (mode: 'none' | 'json' | 'flow' | 'erd' | 'summary') => void;
    setVisualLabData: (data: any) => void;
    setIsVisualLabFullScreen: (v: boolean) => void;
    setIsVisualLabSplitView: (v: boolean) => void;
    toggleVisualLab: (open?: boolean) => void;
    openWorkflowFile: (path: string) => void;
    closeWorkflowFile: (path: string) => void;
    openRuleFile: (path: string) => void;
    closeRuleFile: (path: string) => void;
    setFileTree: (tree: FileEntry[]) => void;
    setDiagnosticsForUri: (uri: string, diags: any[]) => void;
}

export const createEditorSlice: StateCreator<AppState, [], [], EditorSlice> = (set, get) => ({
    activeTabId: null,
    tabs: [],
    fileTree: [],
    fileTreeLoading: false,
    fileTreeError: null,
    activeEditorPath: '',
    activeRoot: localStorage.getItem('activeRoot') || null,
    activeRootName: localStorage.getItem('activeRootName') || null,
    workspaceFolders: [],
    recentWorkspaces: (() => {
        try { return JSON.parse(localStorage.getItem('recentWorkspaces') || '[]'); } catch { return []; }
    })(),
    pendingChanges: [],
    // Default ON: agent edits apply automatically (a git checkpoint is taken before
    // every agent turn, so changes are recoverable). Persisted; flip the toolbar
    // toggle to 'review each diff' if you want manual accept/reject.
    // Cursor-style: review diffs before applying agent edits (opt-in via toolbar AUTO toggle).
    autoAcceptChanges: (typeof localStorage !== 'undefined') && localStorage.getItem('editor.autoAcceptChanges') === '1',
    checkpoint: null,
    tabHistory: [],
    tabHistoryIndex: -1,
    splitEditorTabId: null,
    isSplitEditorOpen: false,
    focusedHunkId: null,
    isMarkdownPreviewOpen: false,
    markdownPreviewWidthPct: (() => {
        try {
            const v = parseFloat(localStorage.getItem('editor.markdownPreviewWidthPct') || '42');
            return Number.isFinite(v) ? Math.min(70, Math.max(22, v)) : 42;
        } catch { return 42; }
    })(),
    isVisualLabOpen: false,
    isVisualLabFullScreen: false,
    isVisualLabSplitView: false,
    visualLabMode: 'none',
    visualLabData: null,
    openWorkflowFiles: [],
    openRuleFiles: [],
    diagnosticsMap: {},
    welcomeForceVisible: false,

    setFileTree: (tree) => set({ fileTree: tree }),

    setDiagnosticsForUri: (uri: string, diags: any[]) => set(state => {
        const severityMap: Record<number, number> = { 1: 8, 2: 4, 3: 2, 4: 1 };
        const mapped = diags.map((d: any) => ({
            severity: severityMap[d.severity ?? 1] ?? 8,
            message: d.message ?? '',
            startLine: (d.range?.start?.line ?? 0) + 1,
            startCol: (d.range?.start?.character ?? 0) + 1,
            endLine: (d.range?.end?.line ?? 0) + 1,
            endCol: (d.range?.end?.character ?? 0) + 1,
            source: d.source ?? 'lsp',
            code: d.code?.toString() ?? '',
        }));
        const key = uri.replace(/^file:\/\/\//, '').replace(/^file:\/\//, '').replace(/\//g, '\\');
        return { diagnosticsMap: { ...state.diagnosticsMap, [key]: mapped } };
    }),

    setActiveRoot: (path) => {
        const cleaned = (path ?? '').split('\0')[0].trim();
        if (cleaned) {
            const name = cleaned.replace(/\\/g, '/').split('/').pop() || cleaned;
            localStorage.setItem('activeRoot', cleaned);
            localStorage.setItem('activeRootName', name);
            const existing: { path: string; name: string; openedAt: number }[] = (() => {
                try { return JSON.parse(localStorage.getItem('recentWorkspaces') || '[]'); } catch { return []; }
            })();
            const updated = [{ path: cleaned, name, openedAt: Date.now() }, ...existing.filter(r => r.path !== cleaned)].slice(0, 10);
            try { localStorage.setItem('recentWorkspaces', JSON.stringify(updated)); } catch { }
            set({ activeRoot: cleaned, activeRootName: name, recentWorkspaces: updated, fileTreeError: null });
            invoke('set_active_root', { path: cleaned })
                .then(() => {
                    get().refreshFileTree();
                    get().fetchActiveProjectSpec();
                    get().ensureIndexingCodebase();
                    get().refreshChatSessions?.();
                    import('../application/lsp/bootstrapLanguageServer').then(m =>
                        m.bootstrapLanguageServer(cleaned),
                    );
                    import('../application/gradle/bootstrapGradleProject').then(m =>
                        m.bootstrapGradleProject(cleaned),
                    );
                    import('../infrastructure/workspace/workspaceProject').then(m =>
                        m.syncWorkspaceCompat(cleaned),
                    );
                })
                .catch((err) => {
                    console.warn('[store] set_active_root rejected:', err);
                    localStorage.removeItem('activeRoot');
                    localStorage.removeItem('activeRootName');
                    set({ activeRoot: null, activeRootName: null, fileTree: [], fileTreeError: null, activeProjectSpec: null });
                });
        } else {
            localStorage.removeItem('activeRoot');
            localStorage.removeItem('activeRootName');
            invoke('set_active_root', { path: null }).catch(console.error);
            set({ activeRoot: null, activeRootName: null, fileTree: [], activeProjectSpec: null });
        }
    },

    setActiveEditorPath: (activeEditorPath) => set({ activeEditorPath }),

    removeRecentWorkspace: (path) => {
        const updated = get().recentWorkspaces.filter(r => r.path !== path);
        try { localStorage.setItem('recentWorkspaces', JSON.stringify(updated)); } catch { }
        set({ recentWorkspaces: updated });
    },

    refreshFileTree: async () => {
        const { refreshFileTree: refresh } = await import('../application/editor/refreshFileTree');
        await refresh();
    },

    addWorkspaceFolder: async (path) => {
        const { addWorkspaceFolder: add } = await import('../application/workspace/multiRootWorkspace');
        await add(path);
        await get().refreshFileTree();
    },

    removeWorkspaceFolder: async (path) => {
        const { removeWorkspaceFolder: remove } = await import('../application/workspace/multiRootWorkspace');
        await remove(path);
        await get().refreshFileTree();
    },

    closeFolder: () => {
        invoke('set_active_root', { path: null });
        set({ activeRoot: null, activeRootName: null, fileTree: [] });
    },

    openFile: async (path: string) => {
        try {
            const { openFile: open } = await import('../application/editor/openFile');
            await open(path);
        } catch (error) {
            console.error('Open File Error:', error);
        }
    },

    closeTab: (id: string) => {
        set((state) => {
            const tabs = state.tabs.filter((t: any) => t.id !== id);
            let activeTabId = state.activeTabId;
            if (activeTabId === id) activeTabId = tabs.length > 0 ? tabs[tabs.length - 1].id : null;
            return { tabs, activeTabId };
        });
    },

    setActiveTab: (id: string) => set((state) => {
        if (state.activeTabId === id) return {} as any;
        const history = state.tabHistory.slice(0, state.tabHistoryIndex + 1);
        history.push(id);
        return { activeTabId: id, tabHistory: history, tabHistoryIndex: history.length - 1 };
    }),

    navigateBack: () => set((state) => {
        if (state.tabHistoryIndex <= 0) return {} as any;
        const newIndex = state.tabHistoryIndex - 1;
        const tabId = state.tabHistory[newIndex];
        if (!state.tabs.find((t: any) => t.id === tabId)) return {} as any;
        return { activeTabId: tabId, tabHistoryIndex: newIndex };
    }),

    navigateForward: () => set((state) => {
        if (state.tabHistoryIndex >= state.tabHistory.length - 1) return {} as any;
        const newIndex = state.tabHistoryIndex + 1;
        const tabId = state.tabHistory[newIndex];
        if (!state.tabs.find((t: any) => t.id === tabId)) return {} as any;
        return { activeTabId: tabId, tabHistoryIndex: newIndex };
    }),

    setSplitEditorTab: (tabId) => set({ splitEditorTabId: tabId, isSplitEditorOpen: tabId !== null }),
    toggleSplitEditor: () => set((state) => {
        if (state.isSplitEditorOpen) return { isSplitEditorOpen: false, splitEditorTabId: null };
        return { isSplitEditorOpen: true, splitEditorTabId: state.activeTabId };
    }),

    updateTabContent: (id: string, content: string) => {
        set((state: any) => {
            const isVisualizing = state.isVisualLabOpen && state.activeTabId === id;
            return {
                tabs: state.tabs.map((t: any) => t.id === id ? { ...t, content, isModified: true } : t),
                visualLabData: isVisualizing ? content : state.visualLabData,
            };
        });
    },

    saveActiveFile: async () => {
        try {
            const { saveActiveFile: save } = await import('../application/editor/saveFile');
            await save();
        } catch (error) {
            console.error('Save File Error:', error);
        }
    },

    openSettings: (tab?) => {
        try { if (tab) sessionStorage.setItem('settings.initialTab', tab); } catch { }
        try { window.dispatchEvent(new CustomEvent('settings:focus-tab', { detail: { tab } })); } catch { }
        const settingsTab = get().tabs.find((t: any) => t.type === 'settings');
        if (settingsTab) { set({ activeTabId: settingsTab.id }); return; }
        const id = 'settings-tab';
        const newTab: EditorTab = { id, filename: 'Settings', path: 'vscode://settings', content: '', isModified: false, language: '', type: 'settings' };
        set((state) => ({ tabs: [...state.tabs, newTab], activeTabId: id }));
    },

    openMcpStore: (view = 'store') => {
        try { sessionStorage.setItem('mcpStore.view', view); } catch { }
        const existing = get().tabs.find((t: any) => t.type === 'mcp-store');
        if (existing) { set({ activeTabId: existing.id }); return; }
        const id = 'mcp-store-tab';
        const newTab: EditorTab = { id, filename: 'MCP Store', path: 'vscode://mcp-store', content: '', isModified: false, language: '', type: 'mcp-store' };
        set((state) => ({ tabs: [...state.tabs, newTab], activeTabId: id }));
    },

    showWelcomeTab: () => {
        try { localStorage.removeItem('welcome.dismissed'); } catch { /* */ }
        set({
            welcomeForceVisible: true,
            tabs: [],
            activeTabId: null,
            activeRoot: null,
            activeRootName: null,
        });
        try {
            localStorage.removeItem('activeRoot');
            localStorage.removeItem('activeRootName');
        } catch { /* */ }
        window.dispatchEvent(new CustomEvent('welcome:show'));
    },

    setWelcomeForceVisible: (visible) => set({ welcomeForceVisible: visible }),

    toggleDirectory: async (path: string) => {
        const { toggleDirectory: toggle } = await import('../application/editor/toggleDirectory');
        await toggle(path);
    },

    getFlattenedFiles: () => {
        const flatten = (entries: FileEntry[]): FileEntry[] => {
            let res: FileEntry[] = [];
            for (const e of entries) {
                if (!e.is_dir) res.push(e);
                if (e.children) res.push(...flatten(e.children));
            }
            return res;
        };
        return flatten(get().fileTree);
    },

    proposePendingChange: (change) => {
        // Merge by path: an agent run can edit one file repeatedly, and proposals
        // arrive across separate poll-drains. Collapse into a single review entry,
        // preserving the EARLIEST oldContent so the diff spans the whole run and
        // reject reverts fully.
        const path = change.path;
        const existing = get().pendingChanges.find(c => c.path === path);
        const oldContent = existing?.oldContent || (change as any).oldContent || '';
        const newContent = (change as any).newContent || '';
        const merged: PendingChange = {
            id: existing?.id || Math.random().toString(36).substring(7),
            path,
            originalContent: oldContent,
            proposedContent: newContent,
            newContent,
            description: change.description,
            additions: change.additions,
            deletions: change.deletions,
            acceptedHunkIds: [],
            rejectedHunkIds: [],
            oldContent,
            applied: (change as any).applied === true || existing?.applied === true,
        };
        const replace = (state: any) => ({
            pendingChanges: [...state.pendingChanges.filter((c: PendingChange) => c.path !== path), merged],
        });
        // Auto-apply when enabled OR in YOLO mode (YOLO = full autonomy, no prompts).
        if (get().autoAcceptChanges || (get() as any).isYoloMode) {
            set(replace);
            get().acceptPendingChange(merged.id).catch(console.error);
        } else {
            set(replace);
        }
    },

    acceptPendingChange: async (id) => {
        const change = get().pendingChanges.find(c => c.id === id);
        if (!change) return;
        try {
            // `applied` proposals are already on disk (agent wrote them). Accept =
            // keep: no backend patch to commit, just sync the open tab + drop.
            if (!change.applied) {
                await invoke('accept_sentient_patch', { path: change.path });
            }
            const tab = get().tabs.find((t: any) => t.path === change.path);
            if (tab) {
                const updatedContent = await invoke<string>('read_file', { path: change.path });
                get().updateTabContent(tab.id, updatedContent);
            }
            set((state) => ({ pendingChanges: state.pendingChanges.filter(c => c.id !== id) }));
            await get().refreshFileTree();
        } catch (error) {
            console.error('Failed to accept sentient patch:', error);
        }
    },

    rejectPendingChange: async (id) => {
        const change = get().pendingChanges.find(c => c.id === id);
        if (!change) return;
        try {
            if (change.applied) {
                // Revert the already-applied edit back to the pre-edit snapshot.
                await invoke('revert_file_content', { path: change.path, content: change.oldContent || change.originalContent || '' });
                const tab = get().tabs.find((t: any) => t.path === change.path);
                if (tab) get().updateTabContent(tab.id, change.oldContent || change.originalContent || '');
                await get().refreshFileTree();
            } else {
                await invoke('reject_sentient_patch', { path: change.path });
            }
            set((state) => ({ pendingChanges: state.pendingChanges.filter(c => c.id !== id) }));
        } catch (error) {
            console.error('Failed to reject sentient patch:', error);
        }
    },

    acceptAllPendingChanges: async () => {
        for (const change of get().pendingChanges) await get().acceptPendingChange(change.id);
    },

    rejectAllPendingChanges: () => set({ pendingChanges: [] }),

    setAutoAcceptChanges: (v) => {
        try { localStorage.setItem('editor.autoAcceptChanges', v ? '1' : '0'); } catch { /* */ }
        set({ autoAcceptChanges: v });
        // Keep Chat settings toggle in sync (single source of truth for users).
        const setGlobal = (get() as any).setVoidGlobalSetting;
        if (typeof setGlobal === 'function') setGlobal('autoAcceptLLMChanges', v);
    },

    snapshotCheckpoint: async (paths) => {
        const snap: Record<string, string> = {};
        for (const p of paths) {
            try { snap[p] = await invoke<string>('read_file', { path: p }); } catch { }
        }
        set({ checkpoint: Object.keys(snap).length > 0 ? snap : null });
    },

    revertToCheckpoint: async () => {
        const { checkpoint } = get();
        if (!checkpoint) return;
        for (const [path, content] of Object.entries(checkpoint)) {
            try {
                await invoke('write_file', { path, content });
                const tab = get().tabs.find((t: any) => t.path === path);
                if (tab) get().updateTabContent(tab.id, content);
            } catch (e) { console.error('Checkpoint revert failed for', path, e); }
        }
        set({ checkpoint: null, pendingChanges: [] });
        await get().refreshFileTree();
    },

    acceptHunk: async (changeId, hunkId) => {
        const { acceptHunk: acceptHunkUseCase } = await import('../application/editor/acceptHunk');
        await acceptHunkUseCase(changeId, hunkId);
    },

    rejectHunk: (changeId, hunkId) => {
        set((state) => {
            const change = state.pendingChanges.find(c => c.id === changeId);
            if (!change) return state;
            const rejected = change.rejectedHunkIds || [];
            if (rejected.includes(hunkId)) return state;
            const newRejected = [...rejected, hunkId];
            const newContent = patchContentSelective(change.originalContent, change.proposedContent, newRejected);
            return { pendingChanges: state.pendingChanges.map(c => c.id === changeId ? { ...c, rejectedHunkIds: newRejected, newContent } : c) };
        });
    },

    setFocusedHunk: (id) => set({ focusedHunkId: id }),

    acceptFocusedHunk: () => {
        const { focusedHunkId, pendingChanges } = get();
        if (!focusedHunkId) return;
        for (const change of pendingChanges) {
            if (change.id === focusedHunkId) { get().acceptPendingChange(change.id); break; }
        }
    },

    rejectFocusedHunk: () => {
        const { focusedHunkId, pendingChanges } = get();
        if (!focusedHunkId) return;
        for (const change of pendingChanges) {
            if (change.id === focusedHunkId) { get().rejectPendingChange(change.id); break; }
        }
        set({ focusedHunkId: null });
    },

    openMarkdownPreview: () => set({ isMarkdownPreviewOpen: true }),
    closeMarkdownPreview: () => set({ isMarkdownPreviewOpen: false }),
    toggleMarkdownPreview: () => set(s => ({ isMarkdownPreviewOpen: !s.isMarkdownPreviewOpen })),
    setMarkdownPreviewWidthPct: (pct) => {
        const clamped = Math.min(70, Math.max(22, pct));
        try { localStorage.setItem('editor.markdownPreviewWidthPct', String(clamped)); } catch { /* */ }
        set({ markdownPreviewWidthPct: clamped });
    },

    setVisualLabMode: (mode) => set({ visualLabMode: mode }),
    setVisualLabData: (data) => set({ visualLabData: data }),
    setIsVisualLabFullScreen: (v) => set({ isVisualLabFullScreen: v }),
    setIsVisualLabSplitView: (v) => set({ isVisualLabSplitView: v }),
    toggleVisualLab: (open?) => set((state) => ({ isVisualLabOpen: open !== undefined ? open : !state.isVisualLabOpen, isVisualLabSplitView: false })),

    openWorkflowFile: (path) => set((state: any) => ({
        openWorkflowFiles: state.openWorkflowFiles.includes(path) ? state.openWorkflowFiles : [...state.openWorkflowFiles, path],
    })),
    closeWorkflowFile: (path) => set((state: any) => ({ openWorkflowFiles: state.openWorkflowFiles.filter((p: string) => p !== path) })),
    openRuleFile: (path) => set((state: any) => ({
        openRuleFiles: state.openRuleFiles.includes(path) ? state.openRuleFiles : [...state.openRuleFiles, path],
    })),
    closeRuleFile: (path) => set((state: any) => ({ openRuleFiles: state.openRuleFiles.filter((p: string) => p !== path) })),
});

