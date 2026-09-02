import type { StateCreator } from 'zustand';

export type ModuleCategory = {
    id: string;
    label: string;
    icon?: string;
};

export type CatalogModuleRow = {
    id: string;
    name: string;
    description: string;
    category: string;
    kind: string;
    component_id?: string;
    repo?: string;
    size_mb?: number;
    tags?: string[];
    native_rust?: boolean;
    requires?: string[];
    installed: boolean;
    enabled: boolean;
    installed_at?: string;
    launch_url?: string;
};

export type ModuleCatalogView = {
    version: number;
    updated?: string;
    publisher?: string;
    catalog_url?: string;
    categories: ModuleCategory[];
    modules: CatalogModuleRow[];
};

export interface ModuleSlice {
    moduleCatalog: ModuleCatalogView | null;
    modulesLoading: boolean;
    modulesError: string | null;
    /** Which installed module's standalone panel is open (null = none). */
    activeModulePanel: string | null;
    setActiveModulePanel: (id: string | null) => void;
    refreshModuleCatalog: (catalogUrl?: string) => Promise<void>;
    isModuleEnabled: (id: string) => boolean;
}

export const createModuleSlice: StateCreator<ModuleSlice> = (set, get) => ({
    activeModulePanel: null,
    setActiveModulePanel: (id) => set({ activeModulePanel: id }),
    moduleCatalog: null,
    modulesLoading: false,
    modulesError: null,

    refreshModuleCatalog: async (catalogUrl?: string) => {
        set({ modulesLoading: true, modulesError: null });
        try {
            const { invoke } = await import('../tauri_bridge');
            const catalog = await invoke<ModuleCatalogView>('modules_fetch_catalog', {
                url: catalogUrl ?? null,
            });
            set({ moduleCatalog: catalog, modulesLoading: false });
        } catch (e) {
            set({ modulesError: String(e), modulesLoading: false });
        }
    },

    isModuleEnabled: (id: string) => {
        const row = get().moduleCatalog?.modules.find((m) => m.id === id);
        return row?.installed === true && row?.enabled === true;
    },
});
