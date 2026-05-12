import React, { useCallback, useEffect, useMemo, useState } from 'react';
import { useStore } from '../store';
import { invoke } from '../tauri_bridge';

// ─────────────────────────────────────────────────────────────────────────────
//  TestExplorer — discovers tests in the active workspace and lets the
//  user run them with one click.
//
//  Discovery is a fast filename scan rather than a full LSP/test-server
//  integration. We support the common conventions:
//
//    • TS/JS  → *.test.ts, *.test.tsx, *.spec.ts, *.spec.tsx, *.test.js
//    • Python → test_*.py, *_test.py
//    • Rust   → tests/*.rs, src/**/tests.rs, #[test] grep
//    • Go     → *_test.go
//
//  Runners are picked off package manifests (package.json scripts,
//  Cargo.toml, pyproject.toml, go.mod). Falls back to a generic prompt.
// ─────────────────────────────────────────────────────────────────────────────

type TestFile = {
    path: string;
    framework: 'jest' | 'vitest' | 'bun' | 'cargo' | 'pytest' | 'go' | 'unknown';
};

type RunStatus = 'idle' | 'running' | 'pass' | 'fail';

const TestExplorer: React.FC = () => {
    const activeRoot = useStore(s => s.activeRoot);
    const [files, setFiles] = useState<TestFile[]>([]);
    const [loading, setLoading] = useState(false);
    const [status, setStatus] = useState<Record<string, RunStatus>>({});
    const [framework, setFramework] = useState<TestFile['framework']>('unknown');
    const [lastOutput, setLastOutput] = useState<string>('');

    const sniffFramework = useCallback(async (): Promise<TestFile['framework']> => {
        if (!activeRoot) return 'unknown';
        try {
            const pkg = await invoke<string>('read_file', { path: `${activeRoot}/package.json` }).catch(() => '');
            if (pkg) {
                const json = JSON.parse(pkg);
                const deps = { ...(json.dependencies || {}), ...(json.devDependencies || {}) };
                if ('vitest' in deps) return 'vitest';
                if ('jest' in deps) return 'jest';
                if ('@types/bun' in deps || (json.scripts?.test || '').includes('bun')) return 'bun';
            }
        } catch { /* ignore */ }
        try {
            const cargo = await invoke<string>('read_file', { path: `${activeRoot}/Cargo.toml` }).catch(() => '');
            if (cargo) return 'cargo';
        } catch { /* ignore */ }
        try {
            const pyproject = await invoke<string>('read_file', { path: `${activeRoot}/pyproject.toml` }).catch(() => '');
            if (pyproject) return 'pytest';
        } catch { /* ignore */ }
        try {
            const gomod = await invoke<string>('read_file', { path: `${activeRoot}/go.mod` }).catch(() => '');
            if (gomod) return 'go';
        } catch { /* ignore */ }
        return 'unknown';
    }, [activeRoot]);

    const scan = useCallback(async () => {
        if (!activeRoot) return;
        setLoading(true);
        try {
            const fw = await sniffFramework();
            setFramework(fw);

            // Build a regex of filename patterns based on framework.
            const patterns: string[] = [];
            if (fw === 'jest' || fw === 'vitest' || fw === 'bun' || fw === 'unknown') {
                patterns.push('\\.(test|spec)\\.(t|j)sx?$');
            }
            if (fw === 'pytest' || fw === 'unknown') {
                patterns.push('(^|/)test_[^/]+\\.py$', '[^/]+_test\\.py$');
            }
            if (fw === 'go' || fw === 'unknown') {
                patterns.push('[^/]+_test\\.go$');
            }
            if (fw === 'cargo' || fw === 'unknown') {
                patterns.push('(^|/)tests/[^/]+\\.rs$');
            }
            const combined = patterns.join('|');
            const res = await invoke<any>('search_codebase_files', { query: combined, root: activeRoot }).catch(() => null);
            const list = Array.isArray(res?.files) ? res.files : [];
            const mapped: TestFile[] = list.map((p: string) => ({ path: p, framework: fw }));
            setFiles(mapped);
        } catch (e) {
            console.error('[TestExplorer] scan failed', e);
        } finally {
            setLoading(false);
        }
    }, [activeRoot, sniffFramework]);

    useEffect(() => { scan(); }, [scan]);

    const cmdFor = useCallback((f: TestFile): string => {
        const rel = activeRoot && f.path.startsWith(activeRoot) ? f.path.slice(activeRoot.length).replace(/^[\\/]+/, '') : f.path;
        switch (f.framework) {
            case 'jest':   return `npx jest "${rel}"`;
            case 'vitest': return `npx vitest run "${rel}"`;
            case 'bun':    return `bun test "${rel}"`;
            case 'pytest': return `pytest "${rel}"`;
            case 'go':     return `go test "./${rel.replace(/\.go$/, '')}"`;
            case 'cargo':  return `cargo test --test "${rel.replace(/^tests\//, '').replace(/\.rs$/, '')}"`;
            default:       return `# unknown test framework — please run manually: ${rel}`;
        }
    }, [activeRoot]);

    const runFile = useCallback(async (f: TestFile) => {
        const cmd = cmdFor(f);
        setStatus(s => ({ ...s, [f.path]: 'running' }));
        try {
            const out = await invoke<any>('call_tool', { name: 'run_command', arguments: { command: cmd, cwd: activeRoot || '' } });
            const text = typeof out === 'string' ? out : JSON.stringify(out, null, 2);
            const passed = !/fail|error|✗|✖|panic/i.test(text);
            setStatus(s => ({ ...s, [f.path]: passed ? 'pass' : 'fail' }));
            setLastOutput(text);
        } catch (e: any) {
            setStatus(s => ({ ...s, [f.path]: 'fail' }));
            setLastOutput(String(e?.message || e));
        }
    }, [activeRoot, cmdFor]);

    const runAll = useCallback(async () => {
        let cmd: string;
        switch (framework) {
            case 'jest':   cmd = 'npx jest'; break;
            case 'vitest': cmd = 'npx vitest run'; break;
            case 'bun':    cmd = 'bun test'; break;
            case 'pytest': cmd = 'pytest'; break;
            case 'go':     cmd = 'go test ./...'; break;
            case 'cargo':  cmd = 'cargo test'; break;
            default:       cmd = 'npm test';
        }
        const placeholder: Record<string, RunStatus> = {};
        files.forEach(f => placeholder[f.path] = 'running');
        setStatus(placeholder);
        try {
            const out = await invoke<any>('call_tool', { name: 'run_command', arguments: { command: cmd, cwd: activeRoot || '' } });
            const text = typeof out === 'string' ? out : JSON.stringify(out, null, 2);
            const passed = !/fail|error|✗|✖|panic/i.test(text);
            const final: Record<string, RunStatus> = {};
            files.forEach(f => final[f.path] = passed ? 'pass' : 'fail');
            setStatus(final);
            setLastOutput(text);
        } catch (e: any) {
            const final: Record<string, RunStatus> = {};
            files.forEach(f => final[f.path] = 'fail');
            setStatus(final);
            setLastOutput(String(e?.message || e));
        }
    }, [activeRoot, files, framework]);

    const counts = useMemo(() => {
        const c = { pass: 0, fail: 0, running: 0, idle: 0 };
        files.forEach(f => { c[status[f.path] || 'idle']++; });
        return c;
    }, [files, status]);

    return (
        <div style={{ display: 'flex', flexDirection: 'column', height: '100%', overflow: 'hidden' }}>
            <div style={{ padding: '8px 12px', borderBottom: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.06))', display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                <div style={{ fontSize: 11, fontWeight: 600, textTransform: 'uppercase', letterSpacing: 0.5 }}>
                    Tests · {framework}
                </div>
                <div style={{ display: 'flex', gap: 4 }}>
                    <button onClick={scan} style={btn} title="Re-scan workspace">
                        <i className="codicon codicon-refresh" style={iconStyle} />
                    </button>
                    <button onClick={runAll} disabled={files.length === 0} style={btn} title="Run all tests">
                        <i className="codicon codicon-run-all" style={iconStyle} />
                    </button>
                </div>
            </div>

            <div style={{ padding: '4px 12px', fontSize: 11, opacity: 0.55, display: 'flex', gap: 12, borderBottom: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.04))' }}>
                <span>{files.length} files</span>
                {counts.pass > 0 && <span style={{ color: '#22c55e' }}>✓ {counts.pass}</span>}
                {counts.fail > 0 && <span style={{ color: '#f87171' }}>✗ {counts.fail}</span>}
                {counts.running > 0 && <span style={{ color: '#3b82f6' }}>● {counts.running}</span>}
            </div>

            <div style={{ flex: 1, overflowY: 'auto' }}>
                {loading && <div style={{ padding: 12, fontSize: 12, opacity: 0.55 }}>Scanning…</div>}
                {!loading && files.length === 0 && (
                    <div style={{ padding: 12, fontSize: 12, opacity: 0.55 }}>
                        No test files found.
                    </div>
                )}
                {files.map(f => {
                    const st = status[f.path] || 'idle';
                    const colour =
                        st === 'pass' ? '#22c55e' :
                        st === 'fail' ? '#f87171' :
                        st === 'running' ? '#3b82f6' :
                        'rgba(255,255,255,0.35)';
                    return (
                        <div
                            key={f.path}
                            onClick={() => runFile(f)}
                            style={{
                                display: 'flex', alignItems: 'center', gap: 6,
                                padding: '4px 12px', fontSize: 12, cursor: 'pointer',
                            }}
                            onMouseEnter={(e) => { e.currentTarget.style.background = 'rgba(255,255,255,0.04)'; }}
                            onMouseLeave={(e) => { e.currentTarget.style.background = 'transparent'; }}
                        >
                            <i
                                className={`codicon codicon-${
                                    st === 'running' ? 'sync' :
                                    st === 'pass' ? 'pass-filled' :
                                    st === 'fail' ? 'error' :
                                    'beaker'
                                }`}
                                style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: 12, color: colour }}
                            />
                            <span style={{ flex: 1, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                                {f.path.replace(activeRoot || '', '').replace(/^[\\/]+/, '')}
                            </span>
                        </div>
                    );
                })}
            </div>

            {lastOutput && (
                <details style={{ borderTop: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.06))', background: 'rgba(0,0,0,0.15)' }}>
                    <summary style={{ padding: '4px 12px', fontSize: 11, cursor: 'pointer', opacity: 0.7 }}>
                        Last run output
                    </summary>
                    <pre style={{
                        margin: 0,
                        padding: 10,
                        maxHeight: 200,
                        overflow: 'auto',
                        fontSize: 11,
                        fontFamily: 'var(--font-mono, monospace)',
                        whiteSpace: 'pre-wrap',
                    }}>{lastOutput.slice(0, 8000)}</pre>
                </details>
            )}
        </div>
    );
};

const btn: React.CSSProperties = {
    background: 'transparent',
    border: 'none',
    color: 'inherit',
    cursor: 'pointer',
    padding: 2,
    fontSize: 12,
};

const iconStyle: React.CSSProperties = {
    fontFamily: 'codicon',
    fontStyle: 'normal',
    fontSize: 12,
};

export default TestExplorer;
