import type { StateCreator } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import { initTheme } from '../theme_engine';
import type { AppState } from './index';

export interface ExtensionSlice {
    installedExtensions: any[];
    marketExtensions: any[];
    popularExtensions: any[];
    isSearchingExtensions: boolean;
    extensionTrustRequest: { publisher: string; name: string; version: string; onResolve: (trusted: boolean) => void } | null;
    trustedPublishers: string[];
    selectedExtensionId: string | null;
    extensionDetails: Record<string, any>;

    setInstalledExtensions: (exts: any[]) => void;
    setMarketExtensions: (exts: any[]) => void;
    setSearchingExtensions: (v: boolean) => void;
    addInstalledExtension: (ext: any) => void;
    refreshInstalledExtensions: () => Promise<void>;
    refreshPopularExtensions: () => Promise<void>;
    searchExtensions: (query: string) => Promise<void>;
    requestExtensionTrust: (publisher: string, name: string, version: string) => Promise<boolean>;
    resolveExtensionTrust: (trusted: boolean, always?: boolean) => void;
    addTrustedPublisher: (publisher: string) => void;
    removeTrustedPublisher: (publisher: string) => void;
    setSelectedExtensionId: (id: string | null) => void;
    fetchExtensionDetails: (id: string) => Promise<void>;
    installExtension: (publisher: string, name: string, version: string) => Promise<boolean>;
    uninstallExtension: (publisher: string, name: string, version?: string) => Promise<boolean>;
}

export const createExtensionSlice: StateCreator<AppState, [], [], ExtensionSlice> = (set, get) => ({
    installedExtensions: [],
    marketExtensions: [],
    popularExtensions: [],
    isSearchingExtensions: false,
    extensionTrustRequest: null,
    trustedPublishers: JSON.parse(localStorage.getItem('trustedPublishers') || '[]'),
    selectedExtensionId: null,
    extensionDetails: {},

    setInstalledExtensions: (installedExtensions) => set({ installedExtensions }),
    setMarketExtensions: (marketExtensions) => set({ marketExtensions }),
    setSearchingExtensions: (isSearchingExtensions) => set({ isSearchingExtensions }),
    addInstalledExtension: (extension) => set((s) => ({
        installedExtensions: [...s.installedExtensions.filter((e: any) => e.id !== extension.id), extension],
    })),

    refreshInstalledExtensions: async () => {
        try {
            const extensions = await invoke<any[]>('get_running_extensions');
            set({ installedExtensions: extensions });
            const iconThemeMapping = await invoke<any>('get_icon_theme_mapping');
            if (iconThemeMapping?.iconDefinitions) set({ iconThemeMapping });
            const contributions = await invoke<any>('get_extension_contributions');
            if (contributions) set({ extensionContributions: contributions });
            const { refreshExtensionCommandRegistry } = await import('../application/extensions/ExtHostBridge');
            await refreshExtensionCommandRegistry();
        } catch (err) { console.error('Failed to refresh installed extensions:', err); }
    },
    refreshPopularExtensions: async () => {
        try { const extensions = await invoke<any[]>('get_popular_extensions'); set({ popularExtensions: extensions }); } catch { }
    },
    searchExtensions: async (query) => {
        if (!query) { set({ marketExtensions: [], isSearchingExtensions: false }); return; }
        set({ isSearchingExtensions: true });
        try { const results = await invoke<any[]>('search_extensions', { query }); set({ marketExtensions: results, isSearchingExtensions: false }); }
        catch { set({ isSearchingExtensions: false }); }
    },
    requestExtensionTrust: (publisher, name, version) => {
        if (get().trustedPublishers.includes(publisher)) return Promise.resolve(true);
        return new Promise((resolve) => set({ extensionTrustRequest: { publisher, name, version, onResolve: resolve } }));
    },
    resolveExtensionTrust: (trusted, always) => {
        const { extensionTrustRequest } = get();
        if (extensionTrustRequest) {
            if (trusted && always) get().addTrustedPublisher(extensionTrustRequest.publisher);
            extensionTrustRequest.onResolve(trusted);
            set({ extensionTrustRequest: null });
        }
    },
    addTrustedPublisher: (publisher) => set((s) => {
        const trustedPublishers = [...new Set([...s.trustedPublishers, publisher])];
        localStorage.setItem('trustedPublishers', JSON.stringify(trustedPublishers));
        return { trustedPublishers };
    }),
    removeTrustedPublisher: (publisher) => set((s) => {
        const trustedPublishers = s.trustedPublishers.filter(p => p !== publisher);
        localStorage.setItem('trustedPublishers', JSON.stringify(trustedPublishers));
        return { trustedPublishers };
    }),
    setSelectedExtensionId: (id) => set({ selectedExtensionId: id }),
    fetchExtensionDetails: async (id) => {
        try { const details = await invoke<any>('get_extension_details', { id }); set((s) => ({ extensionDetails: { ...s.extensionDetails, [id]: details } })); } catch { }
    },
    installExtension: async (publisher, name, version) => {
        try {
            await invoke('install_extension', { publisher, name, version });
            await get().refreshInstalledExtensions();
            if (name.toLowerCase().includes('doki')) setTimeout(() => initTheme(), 500);
            return true;
        } catch { return false; }
    },
    uninstallExtension: async (publisher, name, version) => {
        try { await invoke('uninstall_extension', { publisher, name, version }); await get().refreshInstalledExtensions(); return true; }
        catch { return false; }
    },
});

