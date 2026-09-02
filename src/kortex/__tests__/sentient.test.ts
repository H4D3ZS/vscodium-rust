import { beforeAll, describe, it, expect, vi } from 'vitest';

class LocalStorageMock {
  private store: Record<string, string> = {};
  clear() { this.store = {}; }
  getItem(key: string) { return this.store[key] || null; }
  setItem(key: string, value: string) { this.store[key] = String(value); }
  removeItem(key: string) { delete this.store[key]; }
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
let sentientCore: any;

beforeAll(async () => {
  const storeModule = await import('../../store');
  useStore = storeModule.useStore;
  const sentientModule = await import('../../sentient-core');
  sentientCore = sentientModule.sentientCore;
});

describe('AIRI Sentient Core - Proactive Autonomy & Initiative Cycles', () => {
  it('correctly initializes sentient config and default emotional drives', () => {
    const drives = sentientCore.getEmotionalState();
    expect(drives.curiosity).toBe(50);
    expect(drives.focus).toBe(50);
    expect(drives.satisfaction).toBe(50);
    expect(drives.urgency).toBe(0);
  });

  it('correctly assesses proactive initiative score based on store state', async () => {
    // 1. Initial State (Clean workspace)
    useStore.setState({
      tabs: [],
      agentMessages: [],
    });
    
    const core = sentientCore as any;
    let result = await core.assessInitiative();
    expect(result.score).toBe(50); // Default base score

    // 2. Add Code Diagnostics Errors (+20)
    useStore.setState({
      tabs: [{
        id: '1',
        filename: 'main.rs',
        path: '/mock/main.rs',
        content: 'fn main() {}',
        isModified: false,
        language: 'rust',
        diagnostics: [{ severity: 0, message: 'Type error' }]
      }]
    });

    result = await core.assessInitiative();
    expect(result.score).toBe(70); // 50 + 20

    // 3. Add Unsaved Changes (+10)
    useStore.setState({
      tabs: [{
        id: '1',
        filename: 'main.rs',
        path: '/mock/main.rs',
        content: 'fn main() {}',
        isModified: true,
        language: 'rust',
        diagnostics: [{ severity: 0, message: 'Type error' }]
      }]
    });

    result = await core.assessInitiative();
    expect(result.score).toBe(80); // 50 + 20 + 10

    // 4. Add Test Failures in Chat Messages (+25)
    useStore.setState({
      tabs: [{
        id: '1',
        filename: 'main.rs',
        path: '/mock/main.rs',
        content: 'fn main() {}',
        isModified: true,
        language: 'rust',
        diagnostics: [{ severity: 0, message: 'Type error' }]
      }],
      agentMessages: [
        { role: 'user', content: 'test failed on main' }
      ]
    });

    result = await core.assessInitiative();
    expect(result.score).toBe(105); // 50 + 20 + 10 + 25
  });

  it('proactively fluctuates emotional drives on user messages', () => {
    const core = sentientCore as any;
    
    // Test positive sentiment words
    core.updateEmotionFromMessage('Awesome work! Thanks, this is perfect!');
    expect(sentientCore.getEmotionalState().satisfaction).toBe(65); // 50 + 15

    // Test negative sentiment words
    core.updateEmotionFromMessage('This is wrong and stupid, complete error.');
    expect(sentientCore.getEmotionalState().satisfaction).toBe(50); // 65 - 15
  });
});
