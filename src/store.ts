import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

interface EditorTab {
    id: string;
    filename: string;
    path: string;
    content: string;
    isModified: boolean;
    language: string;
    type?: 'file' | 'settings';
}

export interface FileEntry {
    name: string;
    path: string;
    is_dir: boolean;
    children?: FileEntry[];
}

interface AppState {
    // Layout State
    isSidebarOpen: boolean;
    activeSidebarView: string;
    isBottomPanelOpen: boolean;
    activePanelTab: string;
    isRightSidebarOpen: boolean;
    theme: string;

    // Editor State
    activeTabId: string | null;
    tabs: EditorTab[];
    fileTree: FileEntry[];
    aiStatus: 'alive' | 'dead';
    tokenUsage: number; // 0 to 100
    iconThemeMapping: any;
    agentMode: string;
    agentModel: string;
    activeRoot: string | null;
    activeRootName: string | null;
    activeDevice: string | null;
    emulators: string[];

    // Actions
    toggleSidebar: () => void;
    setActiveSidebarView: (view: string) => void;
    toggleBottomPanel: () => void;
    setActivePanelTab: (tab: string) => void;
    toggleRightSidebar: () => void;
    setTheme: (theme: string) => void;
    setFileTree: (tree: FileEntry[]) => void;
    setAiStatus: (status: 'alive' | 'dead') => void;
    setTokenUsage: (usage: number) => void;
    setIconThemeMapping: (mapping: any) => void;
    setAgentMode: (mode: string) => void;
    setAgentModel: (model: string) => void;
    setActiveRoot: (path: string | null) => void;
    setActiveDevice: (id: string | null) => void;
    setEmulators: (ems: string[]) => void;
    refreshFileTree: () => Promise<void>;
    openFile: (path: string) => Promise<void>;
    closeTab: (id: string) => void;
    setActiveTab: (id: string) => void;
    updateTabContent: (id: string, content: string) => void;
    saveActiveFile: () => Promise<void>;
    openSettings: () => void;

    // Backend Actions
    backendPing: () => Promise<string>;
}

function detectLanguage(filename: string): string {
    const ext = filename.split('.').pop()?.toLowerCase() ?? '';
    const map: Record<string, string> = {
        rs: 'rust', ts: 'typescript', tsx: 'typescript', js: 'javascript',
        jsx: 'javascript', json: 'json', css: 'css', html: 'html',
        md: 'markdown', toml: 'toml', yaml: 'yaml', yml: 'yaml',
        sh: 'shell', py: 'python', go: 'go', c: 'c', cpp: 'cpp',
        h: 'c', hpp: 'cpp', txt: 'plaintext',
    };
    return map[ext] ?? 'plaintext';
}

export const useStore = create<AppState>((set, get) => ({
    // Initial Layout State
    isSidebarOpen: true,
    activeSidebarView: 'explorer-view',
    // Start with a clean workspace: panel closed by default
    isBottomPanelOpen: false,
    activePanelTab: 'TERMINAL',
    isRightSidebarOpen: false,
    theme: 'vs-dark',

    // Initial Editor State
    activeTabId: null,
    tabs: [],
    fileTree: [],
    aiStatus: 'alive',
    tokenUsage: 0,
    iconThemeMapping: null,
    agentMode: 'Planning',
    agentModel: 'Google|gemini-1.5-pro', // Match internal value format
    activeRoot: localStorage.getItem('activeRoot'),
    activeRootName: localStorage.getItem('activeRootName'),
    activeDevice: null,
    emulators: [],

    // Actions
    toggleSidebar: () => set((state) => ({ isSidebarOpen: !state.isSidebarOpen })),
    setActiveSidebarView: (view) => set(() => ({ activeSidebarView: view, isSidebarOpen: true })),
    toggleBottomPanel: () => set((state) => ({ isBottomPanelOpen: !state.isBottomPanelOpen })),
    setActivePanelTab: (tab) => set(() => ({ activePanelTab: tab, isBottomPanelOpen: true })),
    toggleRightSidebar: () => set((state) => ({ isRightSidebarOpen: !state.isRightSidebarOpen })),
    setTheme: (theme) => set({ theme }),
    setFileTree: (tree) => set({ fileTree: tree }),
    setAiStatus: (aiStatus) => set({ aiStatus }),
    setTokenUsage: (tokenUsage) => set({ tokenUsage }),
    setIconThemeMapping: (iconThemeMapping) => set({ iconThemeMapping }),
    setAgentMode: (agentMode) => set({ agentMode }),
    setAgentModel: (agentModel) => set({ agentModel }),
    setActiveRoot: (path) => {
        if (path) {
            const name = path.split('/').pop() || path;
            localStorage.setItem('activeRoot', path);
            localStorage.setItem('activeRootName', name);
            set({ activeRoot: path, activeRootName: name });
        } else {
            localStorage.removeItem('activeRoot');
            localStorage.removeItem('activeRootName');
            set({ activeRoot: null, activeRootName: null });
        }
    },
    setActiveDevice: (activeDevice) => set({ activeDevice }),
    setEmulators: (emulators) => set({ emulators }),

    refreshFileTree: async () => {
        try {
            const tree = await invoke<FileEntry[]>('get_file_tree');
            set({ fileTree: tree });
        } catch (error) {
            console.error('Refresh File Tree Error:', error);
        }
    },

    openFile: async (path: string) => {
        const existingTab = get().tabs.find(t => t.path === path);
        if (existingTab) {
            set({ activeTabId: existingTab.id });
            return;
        }
        try {
            const content = await invoke<string>('read_file', { path });
            const filename = path.split('/').pop() ?? path;
            const id = `tab-${Date.now()}-${Math.random()}`;
            const tab: EditorTab = { id, filename, path, content, isModified: false, language: detectLanguage(filename) };
            set((state) => ({ tabs: [...state.tabs, tab], activeTabId: id }));
        } catch (error) {
            console.error('Open File Error:', error);
        }
    },

    closeTab: (id: string) => {
        set((state) => {
            const tabs = state.tabs.filter(t => t.id !== id);
            let activeTabId = state.activeTabId;
            if (activeTabId === id) {
                activeTabId = tabs.length > 0 ? tabs[tabs.length - 1].id : null;
            }
            return { tabs, activeTabId };
        });
    },

    setActiveTab: (id: string) => set({ activeTabId: id }),

    updateTabContent: (id: string, content: string) => {
        set((state) => ({
            tabs: state.tabs.map(t => t.id === id ? { ...t, content, isModified: true } : t),
        }));
    },

    saveActiveFile: async () => {
        const { tabs, activeTabId } = get();
        const tab = tabs.find(t => t.id === activeTabId);
        if (!tab || tab.type === 'settings') return;
        try {
            await invoke('write_file', { path: tab.path, content: tab.content });
            set((state) => ({
                tabs: state.tabs.map(t => t.id === activeTabId ? { ...t, isModified: false } : t),
            }));
        } catch (error) {
            console.error('Save File Error:', error);
        }
    },

    openSettings: () => {
        const settingsTab = get().tabs.find(t => t.type === 'settings');
        if (settingsTab) {
            set({ activeTabId: settingsTab.id });
            return;
        }
        const id = 'settings-tab';
        const tab: EditorTab = {
            id,
            filename: 'Settings',
            path: 'vscode://settings',
            content: '',
            isModified: false,
            language: '',
            type: 'settings'
        };
        set((state) => ({ tabs: [...state.tabs, tab], activeTabId: id }));
    },

    // Backend Actions
    backendPing: async () => {
        try {
            return await invoke<string>('backend_ping');
        } catch (error) {
            console.error('Backend Ping Error:', error);
            return `Error: ${error}`;
        }
    },
}));

if (typeof window !== 'undefined') {
    (window as any).useStore = useStore;
}
