import type { TerminalWorkflow } from '../../domain/terminal/TerminalWorkflow';
import type { IWorkflowRepository } from '../../domain/terminal/IWorkflowRepository';

const STORAGE_KEY = 'vscr.terminal.workflows';

const DEFAULT_WORKFLOWS: TerminalWorkflow[] = [
    { id: 'wf-git-status', name: 'Git status', command: 'git status', tags: ['git'], icon: 'source-control' },
    { id: 'wf-git-log', name: 'Git log (graph)', command: 'git log --oneline --graph --decorate -20', tags: ['git'], icon: 'git-commit' },
    { id: 'wf-cargo-check', name: 'Cargo check', command: 'cargo check', tags: ['rust', 'build'], icon: 'tools' },
    { id: 'wf-typecheck', name: 'TypeScript typecheck', command: 'npm run typecheck', tags: ['ts', 'build'], icon: 'typescript' },
    { id: 'wf-tauri-dev', name: 'Run Tauri dev', command: 'npx tauri dev', tags: ['run'], icon: 'debug-start' },
    { id: 'wf-cargo-test', name: 'Cargo test', command: 'cargo test', tags: ['rust', 'test'], icon: 'beaker' },
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
    } catch { /* */ }
}

export class LocalStorageWorkflowRepository implements IWorkflowRepository {
    list(): TerminalWorkflow[] {
        const stored = read();
        if (stored.length === 0) {
            write(DEFAULT_WORKFLOWS);
            return [...DEFAULT_WORKFLOWS];
        }
        return stored;
    }

    save(workflow: TerminalWorkflow): void {
        const list = this.list();
        if (list.some((x) => x.command === workflow.command)) return;
        list.unshift(workflow);
        write(list.slice(0, 200));
    }

    delete(id: string): void {
        write(this.list().filter((w) => w.id !== id));
    }

    saveFromCommand(command: string): void {
        const cmd = (command || '').trim();
        if (!cmd) return;
        const name = cmd.length > 48 ? cmd.slice(0, 45) + '…' : cmd;
        this.save({
            id: `wf-${Date.now().toString(36)}`,
            name,
            command: cmd,
            tags: ['saved'],
            icon: 'bookmark',
        });
    }
}

export const workflowRepository = new LocalStorageWorkflowRepository();
