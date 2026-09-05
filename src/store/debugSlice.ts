import type { StateCreator } from 'zustand';
import type { AppState } from './index';
import { boundedPush, MAX_DEBUG_OUTPUT } from '../domain/utils/boundedArray';

export interface DebugBreakpoint {
    id: string;
    path: string;
    line: number;
    enabled: boolean;
    condition?: string;
}

export interface DebugStackFrame {
    id: number;
    name: string;
    line: number;
    column: number;
    source?: { path?: string; name?: string };
}

export interface DebugVariable {
    name: string;
    value: string;
    type?: string;
    variablesReference?: number;
}

export interface DebugWatchEntry {
    id: string;
    expression: string;
    value: string;
}

export interface DebugSlice {
    isDebugging: boolean;
    debugSessionName: string | null;
    debugThreads: { id: number; name: string }[];
    debugStackFrames: DebugStackFrame[];
    debugVariables: DebugVariable[];
    debugBreakpoints: DebugBreakpoint[];
    debugOutput: string[];
    debugWatch: DebugWatchEntry[];
    lastEvaluateResult: string | null;

    setDebugging: (active: boolean, name?: string | null) => void;
    addDebugOutput: (line: string) => void;
    clearDebugOutput: () => void;
    toggleBreakpoint: (path: string, line: number) => void;
    setDebugThreads: (threads: { id: number; name: string }[]) => void;
    setDebugStackFrames: (frames: DebugStackFrame[]) => void;
    setDebugVariables: (vars: DebugVariable[]) => void;
    addDebugWatch: (expression: string) => void;
    removeDebugWatch: (id: string) => void;
    updateDebugWatchValue: (id: string, value: string) => void;
    setLastEvaluateResult: (result: string | null) => void;
}

export const createDebugSlice: StateCreator<AppState, [], [], DebugSlice> = (set, get) => ({
    isDebugging: false,
    debugSessionName: null,
    debugThreads: [],
    debugStackFrames: [],
    debugVariables: [],
    debugBreakpoints: (() => {
        try {
            return JSON.parse(localStorage.getItem('vscr.breakpoints') || '[]');
        } catch {
            return [];
        }
    })(),
    debugOutput: [],
    debugWatch: [],
    lastEvaluateResult: null,

    setDebugging: (active, name = null) =>
        set({ isDebugging: active, debugSessionName: name, ...(active ? {} : { debugThreads: [], debugStackFrames: [], debugVariables: [] }) }),

    addDebugOutput: (line) =>
        set((s) => ({ debugOutput: boundedPush(s.debugOutput, line, MAX_DEBUG_OUTPUT) })),

    clearDebugOutput: () => set({ debugOutput: [] }),

    toggleBreakpoint: (path, line) => {
        const bps = get().debugBreakpoints;
        const existing = bps.find((b) => b.path === path && b.line === line);
        let next: DebugBreakpoint[];
        if (existing) {
            next = bps.filter((b) => b.id !== existing.id);
        } else {
            next = [
                ...bps,
                { id: `bp-${Date.now()}`, path, line, enabled: true },
            ];
        }
        try {
            localStorage.setItem('vscr.breakpoints', JSON.stringify(next));
        } catch { /* */ }
        set({ debugBreakpoints: next });
    },

    setDebugThreads: (threads) => set({ debugThreads: threads }),
    setDebugStackFrames: (frames) => set({ debugStackFrames: frames }),
    setDebugVariables: (vars) => set({ debugVariables: vars }),

    addDebugWatch: (expression) =>
        set((s) => ({
            debugWatch: [
                ...s.debugWatch,
                { id: `watch-${Date.now()}`, expression, value: '…' },
            ],
        })),

    removeDebugWatch: (id) =>
        set((s) => ({ debugWatch: s.debugWatch.filter((w) => w.id !== id) })),

    updateDebugWatchValue: (id, value) =>
        set((s) => ({
            debugWatch: s.debugWatch.map((w) => (w.id === id ? { ...w, value } : w)),
        })),

    setLastEvaluateResult: (lastEvaluateResult) => set({ lastEvaluateResult }),
});
