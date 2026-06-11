// Settings persistence adapter (Milestone C): ONE backing file
// (ui_settings.json in the Tauri config dir) behind two backend commands.
// The Zustand settings state is an in-memory cache hydrated at boot via
// hydrate(); components never touch localStorage for settings.
//
// API keys are NOT stored here — they stay in the backend api_keys.json
// (secrets never enter the renderer's generic settings bag).

import { invoke } from '../tauri_bridge';

export type UiSettings = Record<string, unknown>;

/** Known pre-Milestone-C localStorage keys swept into ui_settings.json once. */
const LEGACY_LOCALSTORAGE_KEYS = [
    'tab.predictionEnabled',
    'void.globalSettings',
    'reasoning.budget',
    'settings.item',
    'airi.companion',
    'airi.autonomous24x7',
] as const;

const MIGRATION_FLAG = 'ui-settings.migrated.v1';

let cache: UiSettings | null = null;

function parseMaybeJson(raw: string): unknown {
    try {
        return JSON.parse(raw);
    } catch {
        return raw; // plain string value (e.g. '1')
    }
}

/** One-time sweep of known localStorage keys into the settings file. */
async function migrateLocalStorage(current: UiSettings): Promise<UiSettings> {
    try {
        if (localStorage.getItem(MIGRATION_FLAG)) return current;
        const next = { ...current };
        for (const key of LEGACY_LOCALSTORAGE_KEYS) {
            const raw = localStorage.getItem(key);
            if (raw === null || key in next) continue;
            const value = parseMaybeJson(raw);
            next[key] = value;
            await invoke('ui_settings_set', { key, value });
        }
        localStorage.setItem(MIGRATION_FLAG, '1');
        return next;
    } catch {
        return current; // storage unavailable — nothing to migrate
    }
}

/** Load everything from disk (and run the one-time migration). Call at boot. */
export async function hydrate(): Promise<UiSettings> {
    let all: UiSettings = {};
    try {
        all = await invoke<UiSettings>('ui_settings_get_all');
    } catch {
        /* backend unavailable (tests / web preview) — empty cache */
    }
    all = await migrateLocalStorage(all);
    cache = all;
    return all;
}

export function get<T = unknown>(key: string, fallback?: T): T | undefined {
    if (cache && key in cache) return cache[key] as T;
    return fallback;
}

export async function set(key: string, value: unknown): Promise<void> {
    cache = { ...(cache ?? {}), [key]: value };
    try {
        await invoke('ui_settings_set', { key, value });
    } catch {
        /* persist best-effort; cache stays correct for this session */
    }
}

export async function remove(key: string): Promise<void> {
    if (cache) {
        const next = { ...cache };
        delete next[key];
        cache = next;
    }
    try {
        await invoke('ui_settings_set', { key, value: null });
    } catch {
        /* best-effort */
    }
}

/** Test seam. */
export function _resetCacheForTests(): void {
    cache = null;
}
