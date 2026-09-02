import type { StateCreator } from 'zustand';
import type { AppState } from './index';
import type { EditorTab } from './types';
import { invoke } from '../tauri_bridge';
import {
    type CanvasSpec,
    CANVAS_DIR,
    CANVAS_FILE_SUFFIX,
    normalizeCanvasSpec,
} from '../domain/canvas/CanvasSpec';

export interface CanvasSlice {
    canvases: CanvasSpec[];

    /** Insert or replace a canvas; optionally open it as an editor tab. */
    upsertCanvas: (spec: CanvasSpec, opts?: { open?: boolean; persist?: boolean }) => void;
    removeCanvas: (id: string) => void;
    /** Open (or focus) the editor tab rendering a canvas. */
    openCanvasTab: (id: string) => void;
    /** Load persisted canvases from the workspace .agent/canvases/ dir. */
    loadPersistedCanvases: (root: string) => Promise<void>;
}

function canvasTabPath(id: string): string {
    return `canvas://${id}`;
}

async function persistCanvas(root: string, spec: CanvasSpec): Promise<void> {
    const dir = `${root}/${CANVAS_DIR}`;
    try {
        await invoke('create_directory', { path: dir });
    } catch { /* may already exist */ }
    await invoke('write_file', {
        path: `${dir}/${spec.id}${CANVAS_FILE_SUFFIX}`,
        content: JSON.stringify(spec, null, 2),
    });
}

export const createCanvasSlice: StateCreator<AppState, [], [], CanvasSlice> = (set, get) => ({
    canvases: [],

    upsertCanvas: (spec, opts) => {
        set((s) => {
            const idx = s.canvases.findIndex((c) => c.id === spec.id);
            const canvases = idx === -1
                ? [...s.canvases, spec]
                : s.canvases.map((c, i) => (i === idx ? spec : c));
            return { canvases };
        });
        if (opts?.persist !== false) {
            const root = (get() as any).activeRoot;
            if (root) void persistCanvas(root, spec).catch(() => { /* non-fatal */ });
        }
        if (opts?.open) get().openCanvasTab(spec.id);
    },

    removeCanvas: (id) => {
        set((s) => ({
            canvases: s.canvases.filter((c) => c.id !== id),
            tabs: s.tabs.filter((t) => t.path !== canvasTabPath(id)),
        }));
        const root = (get() as any).activeRoot;
        if (root) {
            void invoke('delete_path', { path: `${root}/${CANVAS_DIR}/${id}${CANVAS_FILE_SUFFIX}` }).catch(() => {});
        }
    },

    openCanvasTab: (id) => {
        const st = get();
        const spec = st.canvases.find((c) => c.id === id);
        const path = canvasTabPath(id);
        const existing = st.tabs.find((t) => t.path === path);
        if (existing) {
            set({ activeTabId: existing.id });
            return;
        }
        const tabId = `canvas-tab-${id}`;
        const tab = {
            id: tabId,
            filename: spec?.title || 'Canvas',
            path,
            content: '',
            isModified: false,
            language: '',
            type: 'canvas',
        } as EditorTab;
        set((s) => {
            const history = s.tabHistory.slice(0, s.tabHistoryIndex + 1);
            history.push(tabId);
            return {
                tabs: [...s.tabs, tab],
                activeTabId: tabId,
                tabHistory: history,
                tabHistoryIndex: history.length - 1,
            };
        });
    },

    loadPersistedCanvases: async (root) => {
        try {
            const entries = await invoke<any[]>('list_directory', { path: `${root}/${CANVAS_DIR}` });
            const files = (entries || []).filter(
                (e) => !e.is_dir && String(e.name || '').endsWith(CANVAS_FILE_SUFFIX),
            );
            for (const f of files) {
                try {
                    const raw = await invoke<string>('read_file', {
                        path: f.path || `${root}/${CANVAS_DIR}/${f.name}`,
                    });
                    const spec = normalizeCanvasSpec(raw);
                    if (spec) get().upsertCanvas(spec, { open: false, persist: false });
                } catch { /* skip unreadable canvas */ }
            }
        } catch { /* no canvases dir yet */ }
    },
});
