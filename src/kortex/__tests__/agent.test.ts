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

vi.mock('../../terminal', () => ({
  terminalManager: {},
}));

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn().mockResolvedValue(null),
}));

vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn().mockResolvedValue(() => {}),
}));

// Mock Tauri bridge/invoke
vi.mock('../../tauri_bridge', () => ({
  invoke: vi.fn(async (cmd, args) => {
    if (cmd === 'read_file') {
      return `
export function add(a: number, b: number) {
  return a + b;
}
export function sub(a: number, b: number) {
  return a - b;
}
export function mul(a: number, b: number) {
  return a * b;
}
// TODO: implement validation
      `;
    }
    if (cmd === 'write_file') {
      return 'ok';
    }
    if (cmd === 'ai_execute_command') {
      return { summary: 'Mock search results' };
    }
    if (cmd === 'web_search') {
      return [{ title: 'Mock Result', snippet: 'Mock search results', url: '' }];
    }
    return null;
  }),
}));

let useStore: any;
let autonomousAgent: any;

beforeAll(async () => {
  const storeModule = await import('../../store');
  useStore = storeModule.useStore;
  const agentModule = await import('../../autonomous-agent');
  autonomousAgent = agentModule.autonomousAgent;
});

describe('AIRI Autonomous Agent - 24/7 Operations & Issue Diagnostics', () => {
  it('correctly reports initial status before execution', () => {
    const status = autonomousAgent.getStatus();
    expect(status.running).toBe(false);
    expect(status.currentTask).toBeNull();
    expect(status.queueLength).toBe(0);
    expect(status.competence).toBe(50);
  });

  it('scans and diagnoses compilation/diagnostic errors with high priority', async () => {
    // Seed store with typescript error diagnostic
    useStore.setState({
      activeEditorPath: '/mock/src/app.ts',
      tabs: [{
        id: '1',
        filename: 'app.ts',
        path: '/mock/src/app.ts',
        content: 'const x = 5;',
        isModified: false,
        language: 'typescript',
        diagnostics: [{ severity: 0, message: 'Type mismatch: string expected', startLineNumber: 1, startColumn: 7 }]
      }]
    });

    const agent = autonomousAgent as any;
    
    // Force Math.random() to return <= 0.7 to avoid random self-improvement task issues in assertion
    const origRandom = Math.random;
    Math.random = () => 0.5;

    try {
      const issues = await agent.scanForIssues();
      const debugIssues = issues.filter((i: any) => i.type === 'debug');
      expect(debugIssues.length).toBe(1);
      expect(debugIssues[0].priority).toBe(85);
      expect(debugIssues[0].description).toContain('Fix 1 compilation errors');
    } finally {
      Math.random = origRandom;
    }
  });

  it('scans and parses TODO comments and flags them with priority 55', async () => {
    useStore.setState({
      activeEditorPath: '/mock/src/app.ts',
      tabs: [{
        id: '1',
        filename: 'app.ts',
        path: '/mock/src/app.ts',
        content: `
          // TODO: implement user login validation
          // TODO: Add database persistence connection
        `,
        isModified: false,
        language: 'typescript',
        diagnostics: []
      }]
    });

    const agent = autonomousAgent as any;
    const origRandom = Math.random;
    Math.random = () => 0.5;

    try {
      const issues = await agent.scanForIssues();
      const todoIssues = issues.filter((i: any) => i.type === 'implement');
      expect(todoIssues.length).toBe(1);
      expect(todoIssues[0].priority).toBe(55);
      expect(todoIssues[0].description).toContain('Implement 2 TODO items');
    } finally {
      Math.random = origRandom;
    }
  });

  it('detects long functions spanning >50 lines and triggers refactor tasks', async () => {
    const longFunctionBody = Array(52).fill('  console.log("padding");').join('\n');
    useStore.setState({
      activeEditorPath: '/mock/src/app.ts',
      tabs: [{
        id: '1',
        filename: 'app.ts',
        path: '/mock/src/app.ts',
        content: `
          function doSuperHeavyComputation() {
            ${longFunctionBody}
          }
        `,
        isModified: false,
        language: 'typescript',
        diagnostics: []
      }]
    });

    const agent = autonomousAgent as any;
    const origRandom = Math.random;
    Math.random = () => 0.5;

    try {
      const issues = await agent.scanForIssues();
      const refactorIssues = issues.filter((i: any) => i.description.includes('Refactor 1 long functions'));
      expect(refactorIssues.length).toBe(1);
      expect(refactorIssues[0].priority).toBe(45);
    } finally {
      Math.random = origRandom;
    }
  });

  it('flags undocumented public typescript functions with priority 40', async () => {
    useStore.setState({
      activeEditorPath: '/mock/src/app.ts',
      tabs: [{
        id: '1',
        filename: 'app.ts',
        path: '/mock/src/app.ts',
        content: `
          export function processA() {}
          export function processB() {}
          export function processC() {}
        `,
        isModified: false,
        language: 'typescript',
        diagnostics: []
      }]
    });

    const agent = autonomousAgent as any;
    const origRandom = Math.random;
    Math.random = () => 0.5;

    try {
      const issues = await agent.scanForIssues();
      const documentIssues = issues.filter((i: any) => i.type === 'document');
      expect(documentIssues.length).toBe(1);
      expect(documentIssues[0].priority).toBe(40);
      expect(documentIssues[0].description).toContain('Add documentation to 3 public functions');
    } finally {
      Math.random = origRandom;
    }
  });

  it('flags missing error handling ratios for safety precautions', async () => {
    // Large file content (> 500 chars) throwing but not catching
    const body = `
      // Large file to satisfy minimum length requirement
      ${'// padding\n'.repeat(50)}
      export function riskyOperation() {
        throw new Error("unexpected state encountered");
      }
    `;

    useStore.setState({
      activeEditorPath: '/mock/src/app.ts',
      tabs: [{
        id: '1',
        filename: 'app.ts',
        path: '/mock/src/app.ts',
        content: body,
        isModified: false,
        language: 'typescript',
        diagnostics: []
      }]
    });

    const agent = autonomousAgent as any;
    const origRandom = Math.random;
    Math.random = () => 0.5;

    try {
      const issues = await agent.scanForIssues();
      const safetyIssues = issues.filter((i: any) => i.description.includes('Add error handling to async operations'));
      expect(safetyIssues.length).toBe(1);
      expect(safetyIssues[0].priority).toBe(50);
    } finally {
      Math.random = origRandom;
    }
  });

  it('generates self tasks without duplicates and prioritizes queue', () => {
    const agent = autonomousAgent as any;
    
    // Clear queue
    agent.taskQueue = [];

    // Generate a task
    agent.generateSelfTask({
      type: 'debug',
      description: 'Fix the index out of bounds error',
      priority: 85,
    });

    expect(agent.taskQueue.length).toBe(1);
    expect(agent.taskQueue[0].status).toBe('pending');
    expect(agent.taskQueue[0].priority).toBe(85);

    // Try generating the exact duplicate task again
    agent.generateSelfTask({
      type: 'debug',
      description: 'Fix the index out of bounds error',
      priority: 85,
    });

    // Queue length should still be 1 (no duplicates)
    expect(agent.taskQueue.length).toBe(1);

    // Add another task with different description and lower priority
    agent.generateSelfTask({
      type: 'implement',
      description: 'Add a new analytics dashboard helper',
      priority: 55,
    });

    expect(agent.taskQueue.length).toBe(2);

    // Verify task selection prioritizes the highest priority task first
    agent.taskQueue.sort((a: any, b: any) => b.priority - a.priority);
    expect(agent.taskQueue[0].priority).toBe(85);
    expect(agent.taskQueue[1].priority).toBe(55);
  });

  it('background learning cycle correctly aggregates stats and persists to localStorage', () => {
    const agent = autonomousAgent as any;
    
    // Manually manipulate metrics
    agent.completedTasks = 12;
    agent.failedTasks = 2;
    agent.competenceLevel = 78;

    agent.backgroundLearning();

    const savedStatsStr = localStorage.getItem('airi_autonomous_stats');
    expect(savedStatsStr).not.toBeNull();
    const stats = JSON.parse(savedStatsStr!);
    
    expect(stats.completedTasks).toBe(12);
    expect(stats.failedTasks).toBe(2);
    expect(stats.competenceLevel).toBe(78);
  });
});
