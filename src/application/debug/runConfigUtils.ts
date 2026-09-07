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

export interface VarContext {
    workspaceFolder?: string;
    file?: string;
    /** 1-based cursor line, for ${lineNumber}. */
    lineNumber?: number;
    /** current selection text, for ${selectedText}. */
    selectedText?: string;
    /** path to the running IDE binary, for ${execPath}. */
    execPath?: string;
}

/** VS Code's full launch/tasks variable set. Unknown `${...}` tokens are left
 *  as-is (VS Code does the same). Supports ${env:NAME} and ${NAME:-default}. */
export function substituteVars(s: string, ctx: VarContext): string {
    if (!s) return s;
    const ws = ctx.workspaceFolder || '';
    const file = ctx.file || '';
    const sep = file.includes('\\') || ws.includes('\\') ? '\\' : '/';
    const parts = file.split(/[\\/]/);
    const basename = parts[parts.length - 1] || '';
    const dot = basename.lastIndexOf('.');
    const dirname = parts.slice(0, -1).join(sep);
    const rel = file && ws && file.startsWith(ws) ? file.slice(ws.length).replace(/^[\\/]/, '') : file;
    const relDir = rel.split(/[\\/]/).slice(0, -1).join(sep);

    const table: Record<string, string> = {
        workspaceFolder: ws,
        workspaceFolderBasename: ws.split(/[\\/]/).pop() || '',
        cwd: ws,
        file,
        fileWorkspaceFolder: ws,
        fileBasename: basename,
        fileBasenameNoExtension: dot > 0 ? basename.slice(0, dot) : basename,
        fileExtname: dot > 0 ? basename.slice(dot) : '',
        fileDirname: dirname,
        fileDirnameBasename: dirname.split(/[\\/]/).pop() || '',
        relativeFile: rel,
        relativeFileDirname: relDir,
        lineNumber: ctx.lineNumber != null ? String(ctx.lineNumber) : '',
        selectedText: ctx.selectedText || '',
        execPath: ctx.execPath || '',
        pathSeparator: sep,
        '/': sep,
    };

    return s.replace(/\$\{([^}]+)\}/g, (whole, expr: string) => {
        // ${env:NAME}
        const env = /^env:(.+)$/.exec(expr);
        if (env) {
            try { return (globalThis as any).process?.env?.[env[1]] ?? ''; } catch { return ''; }
        }
        // ${config:section} / ${command:id} / ${input:id} — not resolvable here.
        if (/^(config|command|input):/.test(expr)) return whole;
        return expr in table ? table[expr] : whole;
    });
}

export function buildTaskCommand(
    task: { command?: string; args?: string[] },
    ctx: VarContext,
): string {
    const cmd = substituteVars(task.command || '', ctx);
    const argv = (task.args || []).map((a) => substituteVars(a, ctx));
    if (!argv.length) return cmd;
    return `${cmd} ${argv.map((a) => (/\s/.test(a) ? `"${a}"` : a)).join(' ')}`.trim();
}
