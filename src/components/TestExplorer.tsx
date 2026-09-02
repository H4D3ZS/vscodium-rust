import React, { useCallback, useEffect, useMemo, useState } from 'react';
import { useStore } from '../store';
import { discoverTests, runTestFile, runAllTests, sniffTestFramework } from '../application/test/discoverTests';
import { formatTestOutput } from '../application/test/testRunCommands';
import type { TestCase, TestFramework } from '../domain/test/ITestRepository';

type RunStatus = 'idle' | 'running' | 'pass' | 'fail';

const TestExplorer: React.FC = () => {
    const activeRoot = useStore(s => s.activeRoot);
    const [files, setFiles] = useState<TestCase[]>([]);
    const [loading, setLoading] = useState(false);
    const [status, setStatus] = useState<Record<string, RunStatus>>({});
    const [framework, setFramework] = useState<TestFramework>('unknown');
    const [lastOutput, setLastOutput] = useState<string>('');

    const scan = useCallback(async () => {
        if (!activeRoot) return;
        setLoading(true);
        try {
            const fw = await sniffTestFramework(activeRoot);
            setFramework(fw);
            const tests = await discoverTests(activeRoot);
            setFiles(tests);
        } catch (e) {
            console.error('[TestExplorer] scan failed', e);
        } finally {
            setLoading(false);
        }
    }, [activeRoot]);

    useEffect(() => { scan(); }, [scan]);

    const runFile = useCallback(async (f: TestCase) => {
        if (!activeRoot) return;
        setStatus(s => ({ ...s, [f.path]: 'running' }));
        try {
            const result = await runTestFile(activeRoot, f.path);
            const text = formatTestOutput(result);
            setStatus(s => ({ ...s, [f.path]: result.ok ? 'pass' : 'fail' }));
            setLastOutput(text);
        } catch (e: unknown) {
            setStatus(s => ({ ...s, [f.path]: 'fail' }));
            setLastOutput(e instanceof Error ? e.message : String(e));
        }
    }, [activeRoot]);

    const runAll = useCallback(async () => {
        if (!activeRoot) return;
        const placeholder: Record<string, RunStatus> = {};
        files.forEach(f => { placeholder[f.path] = 'running'; });
        setStatus(placeholder);
        try {
            const result = await runAllTests(activeRoot);
            const text = formatTestOutput(result);
            const final: Record<string, RunStatus> = {};
            files.forEach(f => { final[f.path] = result.ok ? 'pass' : 'fail'; });
            setStatus(final);
            setLastOutput(text);
        } catch (e: unknown) {
            const final: Record<string, RunStatus> = {};
            files.forEach(f => { final[f.path] = 'fail'; });
            setStatus(final);
            setLastOutput(e instanceof Error ? e.message : String(e));
        }
    }, [activeRoot, files]);

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
                    const label = f.name || f.path.replace(activeRoot || '', '').replace(/^[\\/]+/, '');
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
                                {label}
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
