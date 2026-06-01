// ════════════════════════════════════════════════════════════════════════════
// Terminal workflows — saved commands (cmder "tasks" / Warp "workflows").
//
// Pure terminal feature: a persisted, searchable list of command snippets the
// user can recall and run/insert. No AI involved. Stored in localStorage so it
// survives restarts and stays local (data sovereignty).
// ════════════════════════════════════════════════════════════════════════════

export interface TerminalWorkflow {
  id: string;
  name: string;
  command: string;
  tags?: string[];
}

const STORAGE_KEY = 'vscr.terminal.workflows';

const DEFAULT_WORKFLOWS: TerminalWorkflow[] = [
  { id: 'wf-git-status', name: 'Git status', command: 'git status', tags: ['git'] },
  { id: 'wf-git-log', name: 'Git log (graph)', command: 'git log --oneline --graph --decorate -20', tags: ['git'] },
  { id: 'wf-cargo-check', name: 'Cargo check', command: 'cargo check', tags: ['rust', 'build'] },
  { id: 'wf-typecheck', name: 'TypeScript typecheck', command: 'npm run typecheck', tags: ['ts', 'build'] },
  { id: 'wf-tauri-dev', name: 'Run Tauri dev', command: 'npx tauri dev', tags: ['run'] },
];

function read(): TerminalWorkflow[] {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return [];
    const parsed = JSON.parse(raw);
    return Array.isArray(parsed) ? (parsed as TerminalWorkflow[]) : [];
  } catch {
    return [];
  }
}

function write(list: TerminalWorkflow[]): void {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(list));
  } catch {
    /* storage full / unavailable — non-fatal */
  }
}

/** All workflows. Seeds a few useful defaults the first time. */
export function getWorkflows(): TerminalWorkflow[] {
  const stored = read();
  if (stored.length === 0) {
    write(DEFAULT_WORKFLOWS);
    return [...DEFAULT_WORKFLOWS];
  }
  return stored;
}

export function addWorkflow(w: TerminalWorkflow): void {
  const list = getWorkflows();
  // De-dupe by command so the Save button can't pile up duplicates.
  if (list.some((x) => x.command === w.command)) return;
  list.unshift(w);
  write(list.slice(0, 200));
}

export function deleteWorkflow(id: string): void {
  write(getWorkflows().filter((w) => w.id !== id));
}

/** Save a raw command line as a workflow (used by the block "Save" action). */
export function saveWorkflowFromCommand(command: string): void {
  const cmd = (command || '').trim();
  if (!cmd) return;
  const name = cmd.length > 48 ? cmd.slice(0, 45) + '…' : cmd;
  addWorkflow({
    id: `wf-${Date.now().toString(36)}`,
    name,
    command: cmd,
    tags: ['saved'],
  });
}
