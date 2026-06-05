import type { StateCreator } from 'zustand';
import type { AppState } from './index';

export interface LspSlice {
    lspRunning: boolean;
    lspDownloading: boolean;
    lspServerId: string | null;
    lspError: string | null;
    setLspStatus: (s: { running?: boolean; downloading?: boolean; serverId?: string | null; error?: string | null }) => void;
}

export const createLspSlice: StateCreator<AppState, [], [], LspSlice> = (set) => ({
    lspRunning: false,
    lspDownloading: false,
    lspServerId: null,
    lspError: null,
    setLspStatus: (s) => set((state) => ({
        lspRunning: s.running ?? state.lspRunning,
        lspDownloading: s.downloading ?? state.lspDownloading,
        lspServerId: s.serverId !== undefined ? s.serverId : state.lspServerId,
        lspError: s.error !== undefined ? s.error : state.lspError,
    })),
});
