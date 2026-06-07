/** Pure helpers for launch.json / tasks.json — no React or Tauri. */

export function tryParseJsonc<T>(raw: string): T | null {
    try {
        const sanitised = raw
            .replace(/^\s*\/\/.*$/gm, '')
            .replace(/\/\*[\s\S]*?\*\//g, '')
            .replace(/,(\s*[\]}])/g, '$1');
        return JSON.parse(sanitised) as T;
    } catch {
        return null;
    }
}

export function substituteVars(
    s: string,
    ctx: { workspaceFolder?: string; file?: string },
): string {
    if (!s) return s;
    const base = (ctx.file || '').split(/[\\/]/).pop()?.replace(/\.[^.]+$/, '') || '';
    return s
        .replaceAll('${workspaceFolder}', ctx.workspaceFolder || '')
        .replaceAll('${cwd}', ctx.workspaceFolder || '')
        .replaceAll('${file}', ctx.file || '')
        .replaceAll('${fileBasenameNoExtension}', base);
}

export function buildTaskCommand(
    task: { command?: string; args?: string[] },
    ctx: { workspaceFolder?: string; file?: string },
): string {
    const cmd = substituteVars(task.command || '', ctx);
    const argv = (task.args || []).map((a) => substituteVars(a, ctx));
    if (!argv.length) return cmd;
    return `${cmd} ${argv.map((a) => (/\s/.test(a) ? `"${a}"` : a)).join(' ')}`.trim();
}
