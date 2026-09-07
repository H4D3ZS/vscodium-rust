/**
 * VS Code "run build task" (Ctrl+Shift+B) + `dependsOn` chains.
 *
 * Self-contained: reads `.vscode/tasks.json`, resolves the default build task
 * (or the sole build-group task), runs its `dependsOn` chain, then the task
 * itself — each in the integrated terminal, same as RunConfigsPanel.
 */
import { invoke } from '../../tauri_bridge';
import { useStore } from '../../store';
import { tryParseJsonc, substituteVars, buildTaskCommand } from '../debug/runConfigUtils';
import { resolveBuildTask, taskRunOrder, type VscTask } from './taskGraph';

export { resolveBuildTask, taskRunOrder };
export type { VscTask };

export async function loadWorkspaceTasks(): Promise<VscTask[]> {
    const root = useStore.getState().activeRoot;
    if (!root) return [];
    const raw = await invoke<string>('read_file', { path: `${root}/.vscode/tasks.json` }).catch(() => '');
    if (!raw) return [];
    const parsed = tryParseJsonc<{ tasks?: VscTask[] }>(raw);
    return parsed?.tasks || [];
}

async function runOneTaskInTerminal(t: VscTask): Promise<void> {
    const st = useStore.getState();
    const ws = st.activeRoot || '';
    const ctx = { workspaceFolder: ws, file: st.activeEditorPath || '' };
    const cwd = substituteVars(t.options?.cwd || t.cwd || ws, ctx);
    const fullCmd = buildTaskCommand(t, ctx);
    if (!fullCmd.trim()) return;

    const groupId = await st.addTerminalGroup();
    const group = useStore.getState().terminalGroups.find((g) => g.id === groupId);
    const termId = group?.activeInstanceId;
    if (!termId) throw new Error('Failed to open the integrated terminal');
    if (cwd) {
        const cd = cwd.includes(' ') ? `cd "${cwd}"` : `cd ${cwd}`;
        await invoke('terminal_send_data', { id: termId, data: `${cd}\r` });
    }
    for (const [k, v] of Object.entries(t.options?.env || {})) {
        await invoke('terminal_send_data', { id: termId, data: `set ${k}=${substituteVars(v, ctx)}\r` }).catch(() => {});
    }
    await invoke('terminal_send_data', { id: termId, data: `${fullCmd}\r` });
}

/** Run `dependsOn` (sequence or parallel) then the task itself. */
export async function runTaskWithDeps(t: VscTask, all: VscTask[], seen = new Set<string>()): Promise<void> {
    if (seen.has(t.label)) return; // cycle guard
    seen.add(t.label);
    const deps = t.dependsOn ? (Array.isArray(t.dependsOn) ? t.dependsOn : [t.dependsOn]) : [];
    const depTasks = deps.map((d) => all.find((x) => x.label === d)).filter(Boolean) as VscTask[];
    if (depTasks.length) {
        if (t.dependsOrder === 'parallel') {
            await Promise.all(depTasks.map((d) => runTaskWithDeps(d, all, seen)));
        } else {
            for (const d of depTasks) await runTaskWithDeps(d, all, seen);
        }
    }
    await runOneTaskInTerminal(t);
}

/** Ctrl+Shift+B entry point. Returns the label run, or null if there's nothing. */
export async function runBuildTask(): Promise<string | null> {
    const tasks = await loadWorkspaceTasks();
    const build = resolveBuildTask(tasks);
    if (!build) return null;
    await runTaskWithDeps(build, tasks);
    return build.label;
}
