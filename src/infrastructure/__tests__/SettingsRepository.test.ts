import { describe, it, expect, vi, beforeEach } from 'vitest';

// vitest runs in node env — stub localStorage for the migration path.
const store = new Map<string, string>();
(globalThis as any).localStorage = {
    getItem: (k: string) => (store.has(k) ? store.get(k)! : null),
    setItem: (k: string, v: string) => void store.set(k, String(v)),
    removeItem: (k: string) => void store.delete(k),
    clear: () => void store.clear(),
};

// Mock the tauri bridge with an in-memory backend file.
const backendFile: Record<string, unknown> = {};
vi.mock('../../tauri_bridge', () => ({
    invoke: vi.fn(async (cmd: string, args?: { key?: string; value?: unknown }) => {
        if (cmd === 'ui_settings_get_all') return { ...backendFile };
        if (cmd === 'ui_settings_set' && args?.key !== undefined) {
            if (args.value === null) delete backendFile[args.key];
            else backendFile[args.key] = args.value;
            return;
        }
        throw new Error(`unexpected cmd ${cmd}`);
    }),
}));

import { hydrate, get, set, remove, _resetCacheForTests } from '../SettingsRepository';

beforeEach(() => {
    for (const k of Object.keys(backendFile)) delete backendFile[k];
    localStorage.clear();
    _resetCacheForTests();
});

describe('SettingsRepository', () => {
    it('round-trips: set → restart (re-hydrate) → get', async () => {
        await hydrate();
        await set('reasoning.budget', 4096);
        _resetCacheForTests(); // simulate app restart
        await hydrate();
        expect(get('reasoning.budget')).toBe(4096);
    });

    it('migrates known localStorage keys once', async () => {
        localStorage.setItem('tab.predictionEnabled', 'true');
        localStorage.setItem('reasoning.budget', '2048');
        await hydrate();
        expect(get('tab.predictionEnabled')).toBe(true);
        expect(get('reasoning.budget')).toBe(2048);
        // second hydrate must not re-run migration
        localStorage.setItem('reasoning.budget', '9999');
        _resetCacheForTests();
        await hydrate();
        expect(get('reasoning.budget')).toBe(2048);
    });

    it('remove deletes from cache and backend', async () => {
        await hydrate();
        await set('chat.companion', '1');
        await remove('chat.companion');
        _resetCacheForTests();
        await hydrate();
        expect(get('chat.companion', 'absent')).toBe('absent');
    });

    it('get falls back when key missing', async () => {
        await hydrate();
        expect(get('nope', 42)).toBe(42);
    });
});
