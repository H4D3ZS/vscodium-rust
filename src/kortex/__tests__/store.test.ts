import { beforeAll, describe, it, expect, vi } from 'vitest';

class LocalStorageMock {
  private store: Record<string, string> = {};

  clear() {
    this.store = {};
  }

  getItem(key: string) {
    return this.store[key] || null;
  }

  setItem(key: string, value: string) {
    this.store[key] = String(value);
  }

  removeItem(key: string) {
    delete this.store[key];
  }
}

delete (global as any).localStorage;
delete (global as any).window;

const localStorageMock = new LocalStorageMock();
Object.defineProperty(global, 'localStorage', {
  value: localStorageMock,
  writable: true,
  configurable: true
});
Object.defineProperty(global, 'window', {
  value: {
    localStorage: localStorageMock,
    dispatchEvent: () => {},
    CustomEvent: class {},
  },
  writable: true,
  configurable: true
});
global.self = global as any;

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn().mockResolvedValue(null),
}));

vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn().mockResolvedValue(() => {}),
}));

vi.mock('../../terminal', () => ({
  terminalManager: {},
}));

let useStore: any;

beforeAll(async () => {
  const storeModule = await import('../../store');
  useStore = storeModule.useStore;
});

describe('Zustand Store Preferences & Kortex Integrations', () => {
  it('correctly initializes GAC and KV cache default values', () => {
    const state = useStore.getState();
    expect(state.kortexGacEnabled).toBe(true);
    expect(state.kortexVramTotalMb).toBe(16384);
    expect(state.kortexTheta).toBe(0.85);
    expect(state.kortexBackend).toBe('vulkan');
    expect(state.kvCacheEnabled).toBe(true);
    expect(state.kvCacheMaxBytes).toBe(16 * 1024 * 1024 * 1024);
    expect(state.ccetEnabled).toBe(true);
  });

  it('correctly persists changes in settings to localStorage', () => {
    const state = useStore.getState();
    state.setKortexGacEnabled(false);
    expect(localStorage.getItem('kortex.gacEnabled')).toBe('0');
    expect(useStore.getState().kortexGacEnabled).toBe(false);

    state.setKortexVramTotalMb(16384);
    expect(localStorage.getItem('kortex.vramTotalMb')).toBe('16384');
    expect(useStore.getState().kortexVramTotalMb).toBe(16384);
  });

  it('picking a lemonade|-tagged model does NOT yank the backend off llama-cpp (Kortex)', () => {
    const state = useStore.getState();

    // User selects the Kortex ROCmFPX backend.
    state.setInferenceBackend('llama-cpp');
    expect(useStore.getState().inferenceBackend).toBe('llama-cpp');

    // A model-list refresh re-applies the persisted tag, which carries a
    // `lemonade|` prefix because agent.ts maps Kortex's routingProvider to
    // 'lemonade'. This must not flip the backend back to 'lemonade'.
    state.setAgentModel('lemonade|Escha-W2-35B-A3B-ROCmFP2.gguf');
    expect(useStore.getState().inferenceBackend).toBe('llama-cpp');

    // But from a non-local backend, a lemonade| pick still aligns to lemonade.
    state.setInferenceBackend('openai');
    state.setAgentModel('lemonade|some-local-model');
    expect(useStore.getState().inferenceBackend).toBe('lemonade');
  });

  it('proposePendingChange supports both oldContent and originalContent formats', () => {
    const state = useStore.getState();
    
    // Clear pending changes
    useStore.setState({ pendingChanges: [] });

    state.proposePendingChange({
      path: '/mock/file.ts',
      oldContent: 'old code',
      newContent: 'new code',
      description: 'Refactor test',
    });

    const pending = useStore.getState().pendingChanges;
    expect(pending.length).toBe(1);
    expect(pending[0].originalContent).toBe('old code');
    expect(pending[0].proposedContent).toBe('new code');
    expect(pending[0].newContent).toBe('new code');
    expect(pending[0].description).toBe('Refactor test');
  });
});
