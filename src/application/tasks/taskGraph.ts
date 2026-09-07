/** Pure task-graph helpers — no store, no Tauri, so it's cheap to unit-test. */

export interface VscTask {
    label: string;
    type?: string;
    command?: string;
    args?: string[];
    cwd?: string;
    group?: string | { kind?: string; isDefault?: boolean };
    dependsOn?: string | string[];
    dependsOrder?: 'sequence' | 'parallel';
    options?: { cwd?: string; env?: Record<string, string> };
}

export function isBuild(t: VscTask): boolean {
    const g = t.group;
    return g === 'build' || (typeof g === 'object' && g?.kind === 'build');
}

function isDefaultBuild(t: VscTask): boolean {
    return typeof t.group === 'object' && t.group?.kind === 'build' && t.group?.isDefault === true;
}

/** Pick the task Ctrl+Shift+B should run. */
export function resolveBuildTask(tasks: VscTask[]): VscTask | null {
    return (
        tasks.find(isDefaultBuild) ||
        (tasks.filter(isBuild).length === 1 ? tasks.find(isBuild) ?? null : null) ||
        tasks.filter(isBuild)[0] ||
        null
    );
}

/**
 * Depth-first `dependsOn` resolution → the labels to run, in order, each once.
 * Cycle-safe.
 */
export function taskRunOrder(t: VscTask, all: VscTask[]): string[] {
    const out: string[] = [];
    const seen = new Set<string>();
    const visit = (task: VscTask) => {
        if (seen.has(task.label)) return;
        seen.add(task.label);
        const deps = task.dependsOn
            ? Array.isArray(task.dependsOn)
                ? task.dependsOn
                : [task.dependsOn]
            : [];
        for (const d of deps) {
            const dt = all.find((x) => x.label === d);
            if (dt) visit(dt);
        }
        out.push(task.label);
    };
    visit(t);
    return out;
}
