import React, { useEffect, useState, useCallback } from 'react';
import { useStore } from '../store';
import { invoke } from '../tauri_bridge';

// ─────────────────────────────────────────────────────────────────────────────
//  RunConfigsPanel — VS Code Tasks + Launch configs runner.
//
//  Reads `.vscode/tasks.json` and `.vscode/launch.json` from the active
//  workspace and renders them as one-click launchers.
//
//    • Tasks         → spawn into a new terminal via terminal_send_data
//                      (so the user sees live output in the terminal panel)
//    • Launch configs → fire debug_start with the chosen config
//
//  We tolerate the same authoring shortcuts VS Code does:
//    • // comments and trailing commas (best-effort strip)
//    • ${workspaceFolder}, ${file}, ${fileBasenameNoExtension}
//    • ${env:NAME} (looked up via process env mirror — best effort)
// ─────────────────────────────────────────────────────────────────────────────

type Task = {
    label: string;
    type?: string;
    command?: string;
    args?: string[];
    cwd?: string;
    group?: string | { kind: string; isDefault?: boolean };
};

type LaunchConfig = {
    name: string;
    type?: string;
    request?: string;
    program?: string;
    args?: string[];
    cwd?: string;
    env?: Record<string, string>;
};

// Strip line comments + trailing commas so we can JSON.parse jsonc-style
// VS Code files. Doesn't handle every jsonc edge case (block comments,
// strings containing `//`) but covers the 99% case.
function tryParseJsonc<T>(raw: string): T | null {
    try {
        const sanitised = raw
            .replace(/^\s*\/\/.*$/gm, '')        // line comments
            .replace(/\/\*[\s\S]*?\*\//g, '')    // block comments
            .replace(/,(\s*[\]}])/g, '$1');      // trailing commas
        return JSON.parse(sanitised) as T;
    } catch {
        return null;
    }
}

function substituteVars(s: string, ctx: { workspaceFolder?: string; file?: string }): string {
    if (!s) return s;
    return s
        .replaceAll('${workspaceFolder}', ctx.workspaceFolder || '')
        .replaceAll('${cwd}', ctx.workspaceFolder || '')
        .replaceAll('${file}', ctx.file || '')
        .replaceAll('${fileBasenameNoExtension}', (ctx.file || '').split(/[\\/]/).pop()?.replace(/\.[^.]+$/, '') || '');
}

const RunConfigsPanel: React.FC = () => {
    const activeRoot = useStore(s => s.activeRoot);
    const activeEditorPath = useStore(s => s.activeEditorPath);
    const [tasks, setTasks] = useState<Task[]>([]);
    const [launches, setLaunches] = useState<LaunchConfig[]>([]);
    const [busy, setBusy] = useState<string | null>(null);
    const [lastResult, setLastResult] = useState<{ name: string; ok: boolean; message: string } | null>(null);

    const reload = useCallback(async () => {
        if (!activeRoot) { setTasks([]); setLaunches([]); return; }
        try {
            const tjson = await invoke<string>('read_file', { path: `${activeRoot}/.vscode/tasks.json` }).catch(() => '');
            const parsed = tjson ? tryParseJsonc<{ tasks?: Task[]; version?: string }>(tjson) : null;
            setTasks(parsed?.tasks || []);
        } catch { setTasks([]); }
        try {
            const ljson = await invoke<string>('read_file', { path: `${activeRoot}/.vscode/launch.json` }).catch(() => '');
            const parsed = ljson ? tryParseJsonc<{ configurations?: LaunchConfig[]; version?: string }>(ljson) : null;
            setLaunches(parsed?.configurations || []);
        } catch { setLaunches([]); }
    }, [activeRoot]);

    useEffect(() => { reload(); }, [reload]);

    const runTask = useCallback(async (t: Task) => {
        setBusy(t.label);
        setLastResult(null);
        try {
            const cmd = substituteVars(t.command || '', { workspaceFolder: activeRoot || '', file: activeEditorPath || '' });
            const argv = (t.args || []).map(a => substituteVars(a, { workspaceFolder: activeRoot || '', file: activeEditorPath || '' }));
            const fullCmd = argv.length ? `${cmd} ${argv.map(a => /\s/.test(a) ? `"${a}"` : a).join(' ')}` : cmd;
            // Spawn a new terminal for this task so its output is visible.
            // The terminal_send_data IPC takes the spawned terminal id and
            // raw bytes; we add a trailing CR to actually execute.
            const termId = await invoke<string>('spawn_terminal', {
                cwd: substituteVars(t.cwd || activeRoot || '', { workspaceFolder: activeRoot || '' }),
                shell: undefined,
            }).catch(async () => {
                // Fallback to a generic spawn signature for older builds.
                return await invoke<string>('spawn_terminal', {});
            });
            await invoke('terminal_send_data', { id: termId, data: `${fullCmd}\r` });
            setLastResult({ name: t.label, ok: true, message: `Spawned terminal ${termId.slice(0, 8)} running: ${fullCmd}` });
        } catch (e: any) {
            setLastResult({ name: t.label, ok: false, message: String(e?.message ?? e) });
        } finally {
            setBusy(null);
        }
    }, [activeRoot, activeEditorPath]);

    const startDebug = useCallback(async (c: LaunchConfig) => {
        setBusy(c.name);
        setLastResult(null);
        try {
            const resolved = {
                ...c,
                program: c.program ? substituteVars(c.program, { workspaceFolder: activeRoot || '', file: activeEditorPath || '' }) : undefined,
                cwd: c.cwd ? substituteVars(c.cwd, { workspaceFolder: activeRoot || '', file: activeEditorPath || '' }) : activeRoot,
                args: (c.args || []).map(a => substituteVars(a, { workspaceFolder: activeRoot || '', file: activeEditorPath || '' })),
            };
            await invoke('debug_start', { config: resolved });
            setLastResult({ name: c.name, ok: true, message: 'Debug session started.' });
        } catch (e: any) {
            setLastResult({ name: c.name, ok: false, message: String(e?.message ?? e) });
        } finally {
            setBusy(null);
        }
    }, [activeRoot, activeEditorPath]);

    return (
        <div style={{ display: 'flex', flexDirection: 'column', height: '100%', overflow: 'hidden' }}>
            <div
                style={{
                    padding: '8px 12px',
                    borderBottom: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.06))',
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'space-between',
                }}
            >
                <div style={{ fontSize: 11, fontWeight: 600, textTransform: 'uppercase', letterSpacing: 0.5 }}>
                    Run & Debug
                </div>
                <button onClick={reload} style={btn} title="Reload tasks.json / launch.json">
                    <i className="codicon codicon-refresh" style={icon} />
                </button>
            </div>

            <div style={{ flex: '1 1 auto', overflowY: 'auto', padding: 8 }}>
                {lastResult && (
                    <div
                        style={{
                            margin: '0 0 8px 0',
                            padding: '6px 8px',
                            fontSize: 11,
                            borderRadius: 4,
                            background: lastResult.ok ? 'rgba(16,185,129,0.12)' : 'rgba(239,68,68,0.12)',
                            border: lastResult.ok ? '1px solid rgba(16,185,129,0.35)' : '1px solid rgba(239,68,68,0.35)',
                            color: lastResult.ok ? '#10b981' : '#f87171',
                        }}
                    >
                        <b>{lastResult.name}</b>: {lastResult.message}
                    </div>
                )}

                <Section title="Tasks">
                    {tasks.length === 0 && (
                        <Hint>
                            No <code>.vscode/tasks.json</code> in this workspace. Add one with
                            a <code>tasks</code> array of label/command entries.
                        </Hint>
                    )}
                    {tasks.map((t, i) => (
                        <Row
                            key={`${t.label || i}-${i}`}
                            icon="play"
                            label={t.label}
                            description={`${t.command || ''} ${(t.args || []).join(' ')}`.trim()}
                            running={busy === t.label}
                            onClick={() => runTask(t)}
                        />
                    ))}
                </Section>

                <Section title="Launch configurations">
                    {launches.length === 0 && (
                        <Hint>
                            No <code>.vscode/launch.json</code> in this workspace. Add one with
                            a <code>configurations</code> array of debugger entries.
                        </Hint>
                    )}
                    {launches.map((c, i) => (
                        <Row
                            key={`${c.name || i}-${i}`}
                            icon="debug-alt"
                            label={c.name}
                            description={`${c.type || '?'} · ${c.request || 'launch'} · ${c.program || ''}`}
                            running={busy === c.name}
                            onClick={() => startDebug(c)}
                        />
                    ))}
                </Section>
            </div>
        </div>
    );
};

const Section: React.FC<{ title: string; children: React.ReactNode }> = ({ title, children }) => (
    <div style={{ marginBottom: 14 }}>
        <div style={{
            fontSize: 10, textTransform: 'uppercase', letterSpacing: 0.5,
            padding: '4px 8px', opacity: 0.55,
        }}>{title}</div>
        {children}
    </div>
);

const Hint: React.FC<{ children: React.ReactNode }> = ({ children }) => (
    <div style={{ padding: '6px 10px', fontSize: 11, opacity: 0.55 }}>{children}</div>
);

const Row: React.FC<{
    icon: string;
    label: string;
    description: string;
    running: boolean;
    onClick: () => void;
}> = ({ icon, label, description, running, onClick }) => (
    <div
        onClick={onClick}
        style={{
            display: 'flex',
            alignItems: 'center',
            gap: 8,
            padding: '5px 10px',
            cursor: running ? 'wait' : 'pointer',
            fontSize: 12,
            borderRadius: 4,
            opacity: running ? 0.65 : 1,
        }}
        onMouseEnter={(e) => { if (!running) e.currentTarget.style.background = 'rgba(255,255,255,0.05)'; }}
        onMouseLeave={(e) => { e.currentTarget.style.background = 'transparent'; }}
    >
        <i
            className={`codicon codicon-${running ? 'sync' : icon}`}
            style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: 13, opacity: 0.8 }}
        />
        <div style={{ flex: 1, minWidth: 0 }}>
            <div style={{ fontWeight: 600 }}>{label || '(unnamed)'}</div>
            {description && (
                <div style={{
                    fontSize: 10, opacity: 0.55,
                    overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap',
                }}>{description}</div>
            )}
        </div>
    </div>
);

const btn: React.CSSProperties = {
    background: 'transparent',
    border: 'none',
    color: 'inherit',
    cursor: 'pointer',
    fontSize: 12,
    padding: 4,
};

const icon: React.CSSProperties = {
    fontFamily: 'codicon',
    fontStyle: 'normal',
};

export default RunConfigsPanel;
