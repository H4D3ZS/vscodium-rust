import type { StateCreator } from 'zustand';
import { initTheme } from '../theme_engine';
import type { AppState } from './index';

export interface LayoutSlice {
    // State
    isSidebarOpen: boolean;
    activeSidebarView: string;
    isBottomPanelOpen: boolean;
    activePanelTab: string;
    isRightSidebarOpen: boolean;
    theme: string;
    sidebarWidth: number;
    rightSidebarWidth: number;
    bottomPanelHeight: number;
    isCommandPaletteOpen: boolean;
    isContextMenuOpen: boolean;
    isDebugToolbarOpen: boolean;
    contextMenuPosition: { x: number; y: number };
    commandPaletteQuery: string;
    layoutMode: 'editor' | 'ml-studio' | 'browser-preview';
    /** True while the external stealth-Firefox OS window is (expected to be) running. */
    externalBrowserActive: boolean;
    isAiriOpen: boolean;
    isComposerOpen: boolean;
    isAiriPanelOpen: boolean;
    isEmulatorPanelOpen: boolean;
    emulatorPanelPosition: 'android' | 'iphone' | 'toolchain' | 'gradle';
    emulatorLayout: 'left' | 'right' | 'hidden';

    // Actions
    toggleSidebar: () => void;
    setActiveSidebarView: (view: string) => void;
    toggleBottomPanel: () => void;
    setActivePanelTab: (tab: string) => void;
    toggleRightSidebar: () => void;
    setTheme: (theme: string) => void;
    setSidebarWidth: (width: number) => void;
    setRightSidebarWidth: (width: number) => void;
    setBottomPanelHeight: (height: number) => void;
    setCommandPaletteOpen: (open: boolean) => void;
    setContextMenuOpen: (open: boolean, x?: number, y?: number) => void;
    setDebugToolbarOpen: (open: boolean) => void;
    setCommandPaletteQuery: (query: string) => void;
    setLayoutMode: (mode: 'editor' | 'ml-studio' | 'browser-preview' | 'browser' | 'browser-ml') => void;
    setExternalBrowserActive: (active: boolean) => void;
    toggleAiri: (open?: boolean) => void;
    toggleComposer: (open?: boolean) => void;
    toggleAiriPanel: () => void;
    toggleEmulatorPanel: () => void;
    openAiriPanel: () => void;
    closeAiriPanel: () => void;
    openEmulatorPanel: () => void;
    closeEmulatorPanel: () => void;
    setEmulatorPanelPosition: (pos: 'android' | 'iphone' | 'toolchain' | 'gradle') => void;
    setEmulatorLayout: (layout: 'left' | 'right' | 'hidden') => void;
    // Zen mode
    isZenMode: boolean;
    toggleZenMode: () => void;
    // Git blame gutter
    isGitBlameVisible: boolean;
    toggleGitBlame: () => void;
    // Document outline
    isOutlinePanelOpen: boolean;
    toggleOutlinePanel: () => void;
    // Editor view
    editorWordWrap: boolean;
    toggleEditorWordWrap: () => void;
}

export const createLayoutSlice: StateCreator<AppState, [], [], LayoutSlice> = (set) => ({
    isSidebarOpen: true,
    activeSidebarView: 'explorer-view',
    isBottomPanelOpen: false,
    activePanelTab: 'TERMINAL',
    isRightSidebarOpen: true,
    theme: localStorage.getItem('active-monaco-theme') || 'vs-dark',
    sidebarWidth: parseInt(localStorage.getItem('sidebarWidth') || '260'),
    rightSidebarWidth: parseInt(localStorage.getItem('rightSidebarWidth') || '300'),
    bottomPanelHeight: parseInt(localStorage.getItem('bottomPanelHeight') || '240'),
    isCommandPaletteOpen: false,
    isContextMenuOpen: false,
    isDebugToolbarOpen: false,
    contextMenuPosition: { x: 0, y: 0 },
    commandPaletteQuery: '',
    layoutMode: 'editor',
    externalBrowserActive: false,
    isAiriOpen: true,
    isComposerOpen: false,
    isAiriPanelOpen: true,
    isEmulatorPanelOpen: false,
    emulatorPanelPosition: 'android',
    emulatorLayout: (localStorage.getItem('emulatorLayout') as 'left' | 'right' | 'hidden') || 'hidden',
    isZenMode: false,
    isGitBlameVisible: false,
    isOutlinePanelOpen: false,
    editorWordWrap: (() => { try { return localStorage.getItem('editor.wordWrap') === '1'; } catch { return false; } })(),

    toggleSidebar: () => set((s) => ({ isSidebarOpen: !s.isSidebarOpen })),
    // VS Code parity: click active view again → hide sidebar; click it while hidden → show.
    setActiveSidebarView: (view) => set((s) => {
        if (s.activeSidebarView === view) {
            return { isSidebarOpen: !s.isSidebarOpen };
        }
        return { activeSidebarView: view, isSidebarOpen: true };
    }),
    toggleBottomPanel: () => set((s) => ({ isBottomPanelOpen: !s.isBottomPanelOpen })),
    setActivePanelTab: (tab) => set({ activePanelTab: tab, isBottomPanelOpen: true }),
    toggleRightSidebar: () => set((s) => {
        const open = !s.isRightSidebarOpen;
        return open
            ? { isRightSidebarOpen: true, isAiriPanelOpen: true, isEmulatorPanelOpen: false }
            : { isRightSidebarOpen: false, isAiriPanelOpen: false, isEmulatorPanelOpen: false };
    }),
    setTheme: (theme) => {
        set({ theme });
        localStorage.setItem('active-monaco-theme', theme);
        if (['vs', 'vs-dark', 'hc-black'].includes(theme)) localStorage.removeItem('active-theme-path');
    },
    setSidebarWidth: (sidebarWidth) => {
        localStorage.setItem('sidebarWidth', sidebarWidth.toString());
        set({ sidebarWidth });
    },
    setRightSidebarWidth: (rightSidebarWidth) => {
        localStorage.setItem('rightSidebarWidth', rightSidebarWidth.toString());
        set({ rightSidebarWidth });
    },
    setBottomPanelHeight: (bottomPanelHeight) => {
        localStorage.setItem('bottomPanelHeight', bottomPanelHeight.toString());
        set({ bottomPanelHeight });
    },
    setCommandPaletteOpen: (isCommandPaletteOpen) => set({ isCommandPaletteOpen }),
    setContextMenuOpen: (isContextMenuOpen, x = 0, y = 0) => set({ isContextMenuOpen, contextMenuPosition: { x, y } }),
    setDebugToolbarOpen: (isDebugToolbarOpen) => set({ isDebugToolbarOpen }),
    setCommandPaletteQuery: (commandPaletteQuery) => set({ commandPaletteQuery }),
    setLayoutMode: (mode) => set({
        layoutMode: mode === 'browser' || mode === 'browser-ml' ? 'browser-preview' : mode,
    }),
    setExternalBrowserActive: (externalBrowserActive) => set({ externalBrowserActive }),
    toggleAiri: (open?) => set((s) => ({ isAiriOpen: open !== undefined ? open : !s.isAiriOpen })),
    toggleComposer: (open?) => set((s) => ({ isComposerOpen: open !== undefined ? open : !s.isComposerOpen })),
    toggleAiriPanel: () => set((s) => ({ isRightSidebarOpen: true, isAiriPanelOpen: !s.isAiriPanelOpen, isEmulatorPanelOpen: false })),
    toggleEmulatorPanel: () => set((s) => ({ isRightSidebarOpen: true, isEmulatorPanelOpen: !s.isEmulatorPanelOpen, isAiriPanelOpen: false })),
    openAiriPanel: () => set({ isRightSidebarOpen: true, isAiriPanelOpen: true, isEmulatorPanelOpen: false }),
    closeAiriPanel: () => set({ isAiriPanelOpen: false, isRightSidebarOpen: false }),
    openEmulatorPanel: () => set({ isRightSidebarOpen: true, isEmulatorPanelOpen: true, isAiriPanelOpen: false }),
    closeEmulatorPanel: () => set({ isEmulatorPanelOpen: false }),
    setEmulatorPanelPosition: (pos) => set({ emulatorPanelPosition: pos }),
    setEmulatorLayout: (layout) => {
        try { localStorage.setItem('emulatorLayout', layout); } catch { }
        set({ emulatorLayout: layout });
    },
    toggleZenMode: () => set((s) => ({ isZenMode: !s.isZenMode })),
    toggleGitBlame: () => set((s) => ({ isGitBlameVisible: !s.isGitBlameVisible })),
    toggleOutlinePanel: () => set((s) => ({ isOutlinePanelOpen: !s.isOutlinePanelOpen })),
    toggleEditorWordWrap: () => set((s) => {
        const next = !s.editorWordWrap;
        try { localStorage.setItem('editor.wordWrap', next ? '1' : '0'); } catch { /* */ }
        const ed = (window as any).activeEditor;
        ed?.updateOptions?.({ wordWrap: next ? 'on' : 'off' });
        return { editorWordWrap: next };
    }),
});

