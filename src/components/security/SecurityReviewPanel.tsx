import React, { Suspense, useEffect, useMemo, useState, useCallback } from 'react';
import { useStore } from '../../store';
import { runCodebaseSecurityReview } from '../../application/security/runCodebaseSecurityReview';
import type { SecurityPanelTab } from '../../application/security/runCodebaseSecurityReview';
import { severityColor, severityRank, type SecurityFinding, type SecuritySeverity } from '../../domain/security/SecurityFinding';
import CyberOpsOverview from './CyberOpsOverview';

const SecurityArsenalPanel = React.lazy(() => import('./SecurityArsenalPanel'));
const ChunkSecretScannerPanel = React.lazy(() => import('./ChunkSecretScannerPanel'));
const VegaScannerPanel = React.lazy(() => import('./VegaScannerPanel'));
const InterceptProxyPanel = React.lazy(() => import('./InterceptProxyPanel'));

const PanelFallback = () => (
    <div style={{ padding: 20, fontSize: 11, opacity: 0.5, textAlign: 'center' }}>Loading…</div>
);

const SEVERITIES: SecuritySeverity[] = ['CRITICAL', 'HIGH', 'MEDIUM', 'LOW', 'INFO'];

const phaseLabel: Record<string, string> = {
    secrets: 'Pass 1/3 — scanning for hardcoded secrets…',
    patterns: 'Pass 2/3 — CWE source-pattern scan…',
    dependencies: 'Pass 3/3 — dependency posture…',
    done: 'Review complete',
    error: 'Review failed',
};

const SecurityReviewPanel: React.FC = () => {
    const activeRoot = useStore(s => s.activeRoot);
    const report = useStore(s => s.securityReviewReport);
    const running = useStore(s => s.securityReviewRunning);
    const phase = useStore(s => s.securityReviewPhase);
    const error = useStore(s => s.securityReviewError);
    const depth = useStore(s => s.securityReviewDepth);
    const setDepth = useStore(s => s.setSecurityReviewDepth);
    const clearReview = useStore(s => s.clearSecurityReview);
    const openFile = useStore(s => s.openFile);

    const [filter, setFilter] = useState<SecuritySeverity | 'ALL'>('ALL');
    const [expandedId, setExpandedId] = useState<string | null>(null);
    const [tab, setTab] = useState<SecurityPanelTab>('overview');

    const openModules = useCallback(() => {
        try {
            localStorage.setItem('settings.category.agent', 'modules');
            sessionStorage.setItem('settings.initialTab', 'agent');
        } catch { /* ignore */ }
        useStore.getState().openSettings('agent');
    }, []);

    useEffect(() => {
        if (tab === 'modules') {
            openModules();
            setTab('overview');
        }
    }, [tab, openModules]);

    useEffect(() => {
        const onTab = (e: Event) => {
            const t = (e as CustomEvent<{ tab?: SecurityPanelTab }>).detail?.tab;
            if (t) setTab(t);
        };
        window.addEventListener('hades:security-tab', onTab);
        return () => window.removeEventListener('hades:security-tab', onTab);
    }, []);

    const filtered = useMemo(() => {
        if (!report) return [];
        const list = [...report.findings].sort(
            (a, b) => severityRank(a.severity) - severityRank(b.severity),
        );
        if (filter === 'ALL') return list;
        return list.filter(f => f.severity === filter);
    }, [report, filter]);

    const onRun = async () => {
        try {
            await runCodebaseSecurityReview({ depth });
        } catch { /* error stored in slice */ }
    };

    const jumpToFinding = (f: SecurityFinding) => {
        if (!f.path || !activeRoot) return;
        const full = f.path.includes(':') || f.path.startsWith('/') || /^[A-Za-z]:/.test(f.path)
            ? f.path
            : `${activeRoot.replace(/\\/g, '/')}/${f.path.replace(/\\/g, '/')}`;
        void openFile(full).then(() => {
            if (f.line > 0) {
                setTimeout(() => {
                    window.dispatchEvent(new CustomEvent('editor:jump-to-line', {
                        detail: { path: full, line: f.line, column: 1 },
                    }));
                }, 120);
            }
        });
    };

    const openReport = () => {
        if (!report?.reportPath || !activeRoot) return;
        const full = `${activeRoot.replace(/\\/g, '/')}/${report.reportPath.replace(/\\/g, '/')}`;
        void openFile(full);
    };

    if (!activeRoot) {
        return (
            <div style={{ display: 'flex', flexDirection: 'column', height: '100%' }}>
                <TabBar tab={tab} setTab={setTab} />
                {tab === 'arsenal' ? (
                    <Suspense fallback={<PanelFallback />}>
                        <SecurityArsenalPanel onOpenReview={() => setTab('review')} />
                    </Suspense>
                ) : (
                    <div style={{ padding: 20, textAlign: 'center', fontSize: 12, opacity: 0.65 }}>
                        <i className="codicon codicon-shield" style={{ fontSize: 32, display: 'block', marginBottom: 12, opacity: 0.4 }} />
                        Open a folder to run a codebase security review.
                    </div>
                )}
            </div>
        );
    }

    if (tab === 'overview') {
        return (
            <div style={{ display: 'flex', flexDirection: 'column', height: '100%' }}>
                <TabBar tab={tab} setTab={setTab} />
                <CyberOpsOverview onNavigate={setTab} onOpenModules={openModules} />
            </div>
        );
    }

    if (tab === 'arsenal') {
        return (
            <div style={{ display: 'flex', flexDirection: 'column', height: '100%' }}>
                <TabBar tab={tab} setTab={setTab} />
                <Suspense fallback={<PanelFallback />}>
                    <SecurityArsenalPanel onOpenReview={() => setTab('review')} />
                </Suspense>
            </div>
        );
    }

    if (tab === 'chunks') {
        return (
            <div style={{ display: 'flex', flexDirection: 'column', height: '100%' }}>
                <TabBar tab={tab} setTab={setTab} />
                <Suspense fallback={<PanelFallback />}>
                    <ChunkSecretScannerPanel />
                </Suspense>
            </div>
        );
    }

    if (tab === 'vega') {
        return (
            <div style={{ display: 'flex', flexDirection: 'column', height: '100%' }}>
                <TabBar tab={tab} setTab={setTab} />
                <Suspense fallback={<PanelFallback />}>
                    <VegaScannerPanel />
                </Suspense>
            </div>
        );
    }

    if (tab === 'proxy') {
        return (
            <div style={{ display: 'flex', flexDirection: 'column', height: '100%' }}>
                <TabBar tab={tab} setTab={setTab} />
                <Suspense fallback={<PanelFallback />}>
                    <InterceptProxyPanel />
                </Suspense>
            </div>
        );
    }

    if (tab === 'modules') {
        return (
            <div style={{ display: 'flex', flexDirection: 'column', height: '100%' }}>
                <TabBar tab={tab} setTab={setTab} />
                <PanelFallback />
            </div>
        );
    }

    return (
        <div style={{ display: 'flex', flexDirection: 'column', height: '100%', overflow: 'hidden' }}>
            <TabBar tab={tab} setTab={setTab} />
            {/* Action bar */}
            <div style={{ padding: '10px 12px', borderBottom: '1px solid var(--vscode-panel-border)', flexShrink: 0 }}>
                <button
                    type="button"
                    disabled={running}
                    onClick={() => void onRun()}
                    style={{
                        width: '100%',
                        padding: '8px 12px',
                        border: 'none',
                        borderRadius: 4,
                        cursor: running ? 'wait' : 'pointer',
                        background: 'var(--vscode-button-background, #0e639c)',
                        color: 'var(--vscode-button-foreground, #fff)',
                        fontSize: 12,
                        fontWeight: 600,
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'center',
                        gap: 8,
                    }}
                >
                    <i className="codicon codicon-shield" style={{ fontFamily: 'codicon', fontStyle: 'normal' }} />
                    {running ? 'Running Security Review…' : 'Run Security Review'}
                </button>

                <div style={{ display: 'flex', gap: 6, marginTop: 8, fontSize: 10 }}>
                    {(['standard', 'deep'] as const).map(d => (
                        <button
                            key={d}
                            type="button"
                            disabled={running}
                            onClick={() => setDepth(d)}
                            style={{
                                flex: 1,
                                padding: '4px 6px',
                                borderRadius: 4,
                                border: '1px solid var(--vscode-panel-border)',
                                background: depth === d ? 'rgba(59,130,246,0.15)' : 'transparent',
                                color: 'var(--vscode-foreground)',
                                cursor: 'pointer',
                                textTransform: 'capitalize',
                            }}
                        >
                            {d}
                        </button>
                    ))}
                    {report && (
                        <button
                            type="button"
                            onClick={clearReview}
                            title="Clear results"
                            style={{
                                padding: '4px 8px',
                                border: '1px solid var(--vscode-panel-border)',
                                borderRadius: 4,
                                background: 'transparent',
                                color: 'var(--vscode-foreground)',
                                cursor: 'pointer',
                            }}
                        >
                            <i className="codicon codicon-clear-all" style={{ fontFamily: 'codicon', fontStyle: 'normal' }} />
                        </button>
                    )}
                </div>

                {running && (
                    <div style={{ marginTop: 8, fontSize: 10, opacity: 0.75 }}>
                        {phaseLabel[phase] ?? 'Scanning…'}
                    </div>
                )}
                {error && (
                    <div style={{ marginTop: 8, fontSize: 10, color: '#f87171' }}>{error}</div>
                )}
            </div>

            {/* Summary */}
            {report && (
                <div style={{ padding: '8px 12px', borderBottom: '1px solid var(--vscode-panel-border)', flexShrink: 0 }}>
                    <div style={{ fontSize: 10, opacity: 0.55, marginBottom: 6 }}>
                        {report.filesScanned} files · {report.totalFindings} findings
                        {report.reportPath && (
                            <span
                                style={{ marginLeft: 8, color: 'var(--vscode-textLink-foreground)', cursor: 'pointer' }}
                                onClick={openReport}
                                title="Open Markdown report"
                            >
                                {report.reportPath}
                            </span>
                        )}
                    </div>
                    <div style={{ display: 'flex', flexWrap: 'wrap', gap: 4 }}>
                        {SEVERITIES.map(sev => {
                            const n = report.bySeverity[sev] ?? 0;
                            if (!n && sev === 'INFO') return null;
                            return (
                                <button
                                    key={sev}
                                    type="button"
                                    onClick={() => setFilter(filter === sev ? 'ALL' : sev)}
                                    style={{
                                        fontSize: 9,
                                        fontWeight: 700,
                                        padding: '2px 6px',
                                        borderRadius: 10,
                                        border: `1px solid ${severityColor(sev)}44`,
                                        background: filter === sev ? `${severityColor(sev)}22` : 'transparent',
                                        color: severityColor(sev),
                                        cursor: 'pointer',
                                    }}
                                >
                                    {sev} {n}
                                </button>
                            );
                        })}
                        <button
                            type="button"
                            onClick={() => setFilter('ALL')}
                            style={{
                                fontSize: 9,
                                padding: '2px 6px',
                                borderRadius: 10,
                                border: '1px solid var(--vscode-panel-border)',
                                background: filter === 'ALL' ? 'rgba(255,255,255,0.06)' : 'transparent',
                                color: 'var(--vscode-foreground)',
                                cursor: 'pointer',
                            }}
                        >
                            ALL
                        </button>
                    </div>
                    {report.dependencyNotes.length > 0 && (
                        <div style={{ marginTop: 8, fontSize: 10, opacity: 0.6 }}>
                            {report.dependencyNotes.map((n, i) => (
                                <div key={i}>• {n}</div>
                            ))}
                        </div>
                    )}
                </div>
            )}

            {/* Findings list */}
            <div style={{ flex: 1, overflowY: 'auto', minHeight: 0 }}>
                {!report && !running && (
                    <div style={{ padding: 16, fontSize: 11, lineHeight: 1.55, opacity: 0.6 }}>
                        <p style={{ margin: '0 0 8px' }}>
                            CodeRabbit-style <b>on-demand</b> review — not real-time monitoring.
                        </p>
                        <p style={{ margin: 0 }}>
                            Click <b>Run Security Review</b> for a full pass: hardcoded secrets, CWE-tagged
                            vulnerability patterns, and dependency posture. Report saved to <code>reports/</code>.
                        </p>
                    </div>
                )}
                {filtered.map(f => (
                    <div
                        key={f.id}
                        style={{
                            borderBottom: '1px solid var(--vscode-panel-border)',
                            padding: '8px 12px',
                            cursor: 'pointer',
                            background: expandedId === f.id ? 'var(--vscode-list-hoverBackground)' : 'transparent',
                        }}
                        onClick={() => setExpandedId(expandedId === f.id ? null : f.id)}
                    >
                        <div style={{ display: 'flex', alignItems: 'flex-start', gap: 8 }}>
                            <span style={{
                                fontSize: 8,
                                fontWeight: 800,
                                padding: '2px 5px',
                                borderRadius: 3,
                                background: `${severityColor(f.severity)}33`,
                                color: severityColor(f.severity),
                                flexShrink: 0,
                            }}>
                                {f.severity}
                            </span>
                            <div style={{ flex: 1, minWidth: 0 }}>
                                <div style={{ fontSize: 11, fontWeight: 600, marginBottom: 2 }}>{f.title}</div>
                                <div style={{ fontSize: 10, opacity: 0.5, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                                    {f.path}{f.line ? `:${f.line}` : ''} · {f.cwe}
                                </div>
                            </div>
                            <button
                                type="button"
                                title="Open in editor"
                                onClick={(e) => { e.stopPropagation(); jumpToFinding(f); }}
                                style={{
                                    border: 'none',
                                    background: 'transparent',
                                    color: 'var(--vscode-foreground)',
                                    cursor: 'pointer',
                                    opacity: 0.6,
                                    padding: 2,
                                }}
                            >
                                <i className="codicon codicon-go-to-file" style={{ fontFamily: 'codicon', fontStyle: 'normal' }} />
                            </button>
                        </div>
                        {expandedId === f.id && (
                            <div style={{ marginTop: 8, fontSize: 10, lineHeight: 1.5, opacity: 0.85 }}>
                                <div style={{
                                    fontFamily: 'var(--vscode-editor-font-family, monospace)',
                                    fontSize: 10,
                                    background: 'rgba(0,0,0,0.2)',
                                    padding: 6,
                                    borderRadius: 4,
                                    marginBottom: 6,
                                    wordBreak: 'break-all',
                                }}>
                                    {f.evidence}
                                </div>
                                <div><b>Fix:</b> {f.remediation}</div>
                            </div>
                        )}
                    </div>
                ))}
                {report && filtered.length === 0 && (
                    <div style={{ padding: 20, textAlign: 'center', fontSize: 11, opacity: 0.5 }}>
                        No findings at this severity level.
                    </div>
                )}
            </div>
        </div>
    );
};

const TabBar: React.FC<{ tab: SecurityPanelTab; setTab: (t: SecurityPanelTab) => void }> = ({ tab, setTab }) => (
    <div style={{ display: 'flex', gap: 3, padding: '8px 8px', borderBottom: '1px solid var(--vscode-panel-border)', flexShrink: 0, overflowX: 'auto' }}>
        {([
            ['overview', 'Hub'],
            ['vega', 'Vega DAST'],
            ['chunks', 'Bundles'],
            ['proxy', 'Proxy'],
            ['review', 'Audit'],
            ['arsenal', 'Arsenal'],
        ] as const).map(([t, label]) => (
            <button
                key={t}
                type="button"
                onClick={() => setTab(t)}
                style={{
                    padding: '5px 12px',
                    borderRadius: 6,
                    border: 'none',
                    cursor: 'pointer',
                    fontSize: 11,
                    fontWeight: 600,
                    background: tab === t ? 'var(--vscode-button-background, #4daafc)' : 'transparent',
                    color: tab === t ? 'var(--vscode-button-foreground, #fff)' : 'inherit',
                }}
            >
                {label}
            </button>
        ))}
    </div>
);

export default SecurityReviewPanel;
