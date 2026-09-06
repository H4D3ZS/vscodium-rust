import React, { useCallback, useEffect, useMemo, useState } from 'react';
import { invoke } from '../../tauri_bridge';
import { useStore } from '../../store';
import { buildVegaFindingsCanvas } from '../../application/security/buildVegaFindingsCanvas';

type VegaAlert = {
    type_key: string;
    title: string;
    severity: string;
    resource: string;
    output: string;
    detection_type?: string;
};

type ScanResult = {
    target: string;
    paths_scanned: number;
    modules_run: number;
    alerts: VegaAlert[];
    duration_ms: number;
    ai_triage?: string[];
};

const SEV_COLOR: Record<string, string> = {
    critical: '#ef4444',
    high: '#f97316',
    medium: '#eab308',
    low: '#94a3b8',
    info: '#64748b',
};

const exportBtn: React.CSSProperties = {
    fontSize: 9,
    padding: '3px 8px',
    borderRadius: 3,
    border: '1px solid var(--vscode-panel-border)',
    background: 'var(--vscode-button-secondaryBackground, #2d2d2d)',
    color: 'var(--vscode-button-secondaryForeground, #ccc)',
    cursor: 'pointer',
};

const TRIAGE_COLOR: Record<string, string> = {
    CONFIRMED: '#ef4444',
    LIKELY: '#f97316',
    FALSE_POSITIVE: '#64748b',
    UNKNOWN: '#94a3b8',
    SKIPPED: '#475569',
};

const VegaScannerPanel: React.FC = () => {
    const [target, setTarget] = useState('https://');
    const [authorized, setAuthorized] = useState(false);
    const [aiTriage, setAiTriage] = useState(false);
    const [sessionCookie, setSessionCookie] = useState('');
    const [busy, setBusy] = useState(false);
    const [error, setError] = useState('');
    const [result, setResult] = useState<ScanResult | null>(null);
    const [expanded, setExpanded] = useState<string | null>(null);
    const [moduleCount, setModuleCount] = useState(0);

    useEffect(() => {
        invoke<{ id: string; kind: string }[]>('vega_list_modules')
            .then((m) => setModuleCount(m.filter((x) => x.kind === 'injection').length))
            .catch(() => {});
    }, []);

    const runScan = useCallback(async () => {
        setError('');
        setBusy(true);
        setResult(null);
        try {
            const data = await invoke<ScanResult>('vega_scan', {
                options: {
                    targetUrl: target.trim(),
                    authorized,
                    maxPages: 24,
                    maxDepth: 2,
                    runPassive: true,
                    aiTriage,
                    sessionCookie: sessionCookie.trim() || undefined,
                },
            });
            setResult(data);
            // Auto-render the findings dashboard as a canvas tab.
            try {
                useStore.getState().upsertCanvas(buildVegaFindingsCanvas(data), { open: true });
            } catch { /* canvas rendering is best-effort */ }
        } catch (e) {
            setError(String(e));
        } finally {
            setBusy(false);
        }
    }, [authorized, target, aiTriage, sessionCookie]);

    const [exportMsg, setExportMsg] = useState('');
    const exportReport = useCallback(
        async (format: 'sarif' | 'markdown') => {
            if (!result) return;
            try {
                const content = await invoke<string>('vega_export_report', {
                    target: result.target,
                    alerts: result.alerts,
                    triage: result.ai_triage ?? [],
                    format,
                });
                await navigator.clipboard.writeText(content);
                setExportMsg(`${format === 'sarif' ? 'SARIF' : 'Markdown'} report copied to clipboard`);
                window.setTimeout(() => setExportMsg(''), 2500);
            } catch (e) {
                setExportMsg(`Export failed: ${String(e)}`);
            }
        },
        [result],
    );

    const criticalCount = useMemo(
        () => result?.alerts.filter((a) => a.severity === 'critical').length ?? 0,
        [result],
    );

    return (
        <div style={{ display: 'flex', flexDirection: 'column', height: '100%', overflow: 'hidden' }}>
            <div style={{ padding: '10px 12px', borderBottom: '1px solid var(--vscode-panel-border)', flexShrink: 0 }}>
                <div style={{ fontSize: 11, lineHeight: 1.5, opacity: 0.75, marginBottom: 10 }}>
                    Rust-native Vega DAST — crawls in-scope paths, runs SQLi/XSS/CMDi/SSRF modules in parallel.
                    {moduleCount > 0 && ` ${moduleCount} injection modules loaded.`}
                </div>

                <input
                    value={target}
                    onChange={(e) => setTarget(e.target.value)}
                    placeholder="https://target.app/"
                    style={{
                        width: '100%',
                        marginBottom: 8,
                        padding: '6px 8px',
                        fontSize: 11,
                        borderRadius: 4,
                        border: '1px solid var(--vscode-panel-border)',
                        background: 'var(--vscode-input-background)',
                        color: 'var(--vscode-input-foreground)',
                    }}
                />

                <input
                    value={sessionCookie}
                    onChange={(e) => setSessionCookie(e.target.value)}
                    placeholder="Session cookie (optional) — e.g. PHPSESSID=…; security=low"
                    spellCheck={false}
                    style={{
                        width: '100%',
                        marginBottom: 8,
                        padding: '6px 8px',
                        fontSize: 11,
                        borderRadius: 4,
                        border: '1px solid var(--vscode-panel-border)',
                        background: 'var(--vscode-input-background)',
                        color: 'var(--vscode-input-foreground)',
                    }}
                />

                <label style={{ display: 'flex', alignItems: 'flex-start', gap: 8, fontSize: 10, marginBottom: 8, cursor: 'pointer' }}>
                    <input
                        type="checkbox"
                        checked={authorized}
                        onChange={(e) => setAuthorized(e.target.checked)}
                        style={{ marginTop: 2 }}
                    />
                    <span>
                        I have <b>written authorization</b> to test this target (bug bounty, pentest engagement, or owned system).
                    </span>
                </label>

                <label style={{ display: 'flex', alignItems: 'center', gap: 8, fontSize: 10, marginBottom: 10, cursor: 'pointer', opacity: 0.85 }}>
                    <input
                        type="checkbox"
                        checked={aiTriage}
                        onChange={(e) => setAiTriage(e.target.checked)}
                    />
                    <span>
                        Local-LLM triage — flag likely false positives (needs a local model)
                    </span>
                </label>

                <button
                    type="button"
                    disabled={busy || !authorized || target.trim().length < 10}
                    onClick={() => void runScan()}
                    style={{
                        width: '100%',
                        padding: '8px 12px',
                        border: 'none',
                        borderRadius: 4,
                        cursor: busy ? 'wait' : 'pointer',
                        background: 'var(--vscode-button-background, #0e639c)',
                        color: 'var(--vscode-button-foreground, #fff)',
                        fontSize: 12,
                        fontWeight: 600,
                    }}
                >
                    {busy ? 'Vega scan running…' : 'Run Vega DAST Scan'}
                </button>
                {error && <div style={{ marginTop: 8, fontSize: 10, color: '#f87171' }}>{error}</div>}
            </div>

            {result && (
                <div style={{ padding: '8px 12px', borderBottom: '1px solid var(--vscode-panel-border)', fontSize: 10, flexShrink: 0 }}>
                    <div>
                        {result.paths_scanned} paths · {result.modules_run} modules · {result.alerts.length} alerts
                        · {(result.duration_ms / 1000).toFixed(1)}s
                        {criticalCount > 0 && (
                            <span style={{ color: SEV_COLOR.critical, marginLeft: 8, fontWeight: 700 }}>
                                {criticalCount} critical
                            </span>
                        )}
                    </div>
                    {result.alerts.length > 0 && (
                        <div style={{ display: 'flex', gap: 6, marginTop: 6, alignItems: 'center' }}>
                            <button type="button" onClick={() => void exportReport('markdown')} style={exportBtn}>
                                Copy Markdown
                            </button>
                            <button type="button" onClick={() => void exportReport('sarif')} style={exportBtn}>
                                Copy SARIF
                            </button>
                            {exportMsg && <span style={{ opacity: 0.7 }}>{exportMsg}</span>}
                        </div>
                    )}
                </div>
            )}

            <div style={{ flex: 1, overflowY: 'auto', minHeight: 0 }}>
                {!result && !busy && (
                    <div style={{ padding: 16, fontSize: 11, lineHeight: 1.55, opacity: 0.6 }}>
                        Authorized web pentest only. Vega uses differential detection — ideal for bounty triage before Moxy/Hetty deep dives.
                    </div>
                )}
                {result?.alerts.map((a, i) => {
                    const id = `${a.type_key}:${i}`;
                    const sev = a.severity?.toLowerCase() ?? 'info';
                    const triageVerdict = result.ai_triage?.[i];
                    return (
                        <div
                            key={id}
                            style={{
                                borderBottom: '1px solid var(--vscode-panel-border)',
                                padding: '8px 12px',
                                cursor: 'pointer',
                                background: expanded === id ? 'var(--vscode-list-hoverBackground)' : 'transparent',
                            }}
                            onClick={() => setExpanded(expanded === id ? null : id)}
                        >
                            <div style={{ display: 'flex', gap: 8 }}>
                                <span style={{
                                    fontSize: 8,
                                    fontWeight: 800,
                                    padding: '2px 5px',
                                    borderRadius: 3,
                                    background: `${SEV_COLOR[sev] ?? '#888'}33`,
                                    color: SEV_COLOR[sev] ?? '#888',
                                }}>
                                    {sev.toUpperCase()}
                                </span>
                                <div style={{ flex: 1, minWidth: 0 }}>
                                    <div style={{ fontSize: 11, fontWeight: 600 }}>{a.title || a.type_key}</div>
                                    <div style={{ fontSize: 10, opacity: 0.5, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                                        {a.resource}
                                    </div>
                                </div>
                                {triageVerdict && (
                                    <span style={{
                                        fontSize: 8,
                                        fontWeight: 700,
                                        padding: '2px 5px',
                                        borderRadius: 3,
                                        alignSelf: 'flex-start',
                                        whiteSpace: 'nowrap',
                                        background: `${TRIAGE_COLOR[triageVerdict] ?? '#888'}22`,
                                        color: TRIAGE_COLOR[triageVerdict] ?? '#888',
                                    }}>
                                        {triageVerdict.replace('_', ' ')}
                                    </span>
                                )}
                            </div>
                            {expanded === id && (
                                <div style={{ marginTop: 8, fontSize: 10, fontFamily: 'monospace', background: 'rgba(0,0,0,0.2)', padding: 6, borderRadius: 4, wordBreak: 'break-all' }}>
                                    {a.output || a.detection_type || '—'}
                                </div>
                            )}
                        </div>
                    );
                })}
                {result && result.alerts.length === 0 && (
                    <div style={{ padding: 20, textAlign: 'center', fontSize: 11, opacity: 0.5 }}>
                        No alerts on crawled parametric paths.
                    </div>
                )}
            </div>
        </div>
    );
};

export default VegaScannerPanel;
