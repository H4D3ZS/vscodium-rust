import React, { useCallback, useEffect, useRef, useState } from 'react';
import { invoke, listen } from '../../tauri_bridge.ts';

// ── Domain types (mirror Rust CanvasSnapshot) ─────────────────────────────────

type Tier = 'critical' | 'high' | 'medium' | 'low' | 'info';

interface CanvasFinding {
    id: string;
    tier: Tier;
    category: string;
    title: string;
    description: string;
    affected_paths: string[];
    recommendation: string;
    timestamp_ns: number;
    raw?: string;
}

interface DependencyEdge {
    from: string;
    to: string;
    risk: string;
}

interface TierCounts {
    critical: number;
    high: number;
    medium: number;
    low: number;
    info: number;
}

interface CanvasSnapshot {
    run_id: string;
    generated_at_ns: number;
    risk_score: number;
    summary: string;
    findings: CanvasFinding[];
    dependency_chains: DependencyEdge[];
    tier_counts: TierCounts;
}

// ── Constants ─────────────────────────────────────────────────────────────────

const TIER_ORDER: Tier[] = ['critical', 'high', 'medium', 'low', 'info'];

const TIER_LABEL: Record<Tier, string> = {
    critical: 'CRITICAL',
    high: 'HIGH',
    medium: 'MEDIUM',
    low: 'LOW',
    info: 'INFO',
};

const TIER_COLOR: Record<Tier, string> = {
    critical: '#f85149',
    high: '#ff7b72',
    medium: '#d29922',
    low: '#79c0ff',
    info: '#8b949e',
};

const TIER_BG: Record<Tier, string> = {
    critical: 'rgba(248,81,73,0.10)',
    high: 'rgba(255,123,114,0.08)',
    medium: 'rgba(210,153,34,0.08)',
    low: 'rgba(121,192,255,0.06)',
    info: 'rgba(139,148,158,0.06)',
};

// ── Sub-components ────────────────────────────────────────────────────────────

const RiskBar: React.FC<{ score: number }> = ({ score }) => {
    const color = score >= 75 ? TIER_COLOR.critical
        : score >= 50 ? TIER_COLOR.high
        : score >= 25 ? TIER_COLOR.medium
        : TIER_COLOR.low;

    return (
        <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
            <div style={{
                width: 120,
                height: 6,
                background: 'rgba(255,255,255,0.08)',
                borderRadius: 3,
                overflow: 'hidden',
            }}>
                <div style={{
                    width: `${score}%`,
                    height: '100%',
                    background: color,
                    borderRadius: 3,
                }} />
            </div>
            <span style={{ color, fontSize: 11, fontFamily: 'monospace', fontWeight: 700 }}>
                {score}
            </span>
        </div>
    );
};

const TierBadge: React.FC<{ tier: Tier }> = ({ tier }) => (
    <span style={{
        display: 'inline-block',
        padding: '1px 6px',
        borderRadius: 3,
        fontSize: 10,
        fontFamily: 'monospace',
        fontWeight: 700,
        letterSpacing: '0.05em',
        color: TIER_COLOR[tier],
        background: TIER_BG[tier],
        border: `1px solid ${TIER_COLOR[tier]}33`,
    }}>
        {TIER_LABEL[tier]}
    </span>
);

const TierCount: React.FC<{ tier: Tier; count: number }> = ({ tier, count }) => {
    if (count === 0) return null;
    return (
        <span style={{
            display: 'inline-flex',
            alignItems: 'center',
            gap: 4,
            padding: '2px 8px',
            borderRadius: 4,
            fontSize: 11,
            fontFamily: 'monospace',
            color: TIER_COLOR[tier],
            background: TIER_BG[tier],
        }}>
            <span style={{ fontWeight: 700 }}>{count}</span>
            <span style={{ opacity: 0.7 }}>{TIER_LABEL[tier]}</span>
        </span>
    );
};

const CodeViewer: React.FC<{ raw: string }> = ({ raw }) => (
    <pre style={{
        margin: '8px 0 0',
        padding: '10px 12px',
        background: '#0d1117',
        border: '1px solid rgba(255,255,255,0.07)',
        borderRadius: 4,
        fontFamily: '"Cascadia Mono","JetBrains Mono",Consolas,monospace',
        fontSize: 11,
        lineHeight: 1.5,
        color: '#c9d1d9',
        overflowX: 'auto',
        maxHeight: 240,
        overflowY: 'auto',
        whiteSpace: 'pre-wrap',
        wordBreak: 'break-all',
    }}>
        {raw}
    </pre>
);

const FindingRow: React.FC<{ finding: CanvasFinding }> = ({ finding }) => {
    const [expanded, setExpanded] = useState(false);

    return (
        <div style={{
            borderBottom: '1px solid rgba(255,255,255,0.05)',
            background: expanded ? TIER_BG[finding.tier] : 'transparent',
        }}>
            {/* Summary row — clickable */}
            <div
                role="button"
                tabIndex={0}
                onClick={() => setExpanded(e => !e)}
                onKeyDown={e => e.key === 'Enter' && setExpanded(v => !v)}
                style={{
                    display: 'grid',
                    gridTemplateColumns: '80px 120px 1fr 16px',
                    alignItems: 'center',
                    gap: 8,
                    padding: '6px 12px',
                    cursor: 'pointer',
                    outline: 'none',
                }}
            >
                <TierBadge tier={finding.tier} />
                <span style={{
                    fontSize: 10,
                    color: '#8b949e',
                    fontFamily: 'monospace',
                    overflow: 'hidden',
                    textOverflow: 'ellipsis',
                    whiteSpace: 'nowrap',
                }}>
                    {finding.category}
                </span>
                <span style={{
                    fontSize: 12,
                    color: '#e6edf3',
                    overflow: 'hidden',
                    textOverflow: 'ellipsis',
                    whiteSpace: 'nowrap',
                }}>
                    {finding.title}
                </span>
                <span style={{
                    fontSize: 10,
                    color: '#8b949e',
                    userSelect: 'none',
                    justifySelf: 'end',
                }}>
                    {expanded ? '▾' : '▸'}
                </span>
            </div>

            {/* Detail panel */}
            {expanded && (
                <div style={{ padding: '0 12px 12px 12px' }}>
                    <p style={{ margin: '0 0 6px', fontSize: 12, color: '#c9d1d9', lineHeight: 1.6 }}>
                        {finding.description}
                    </p>
                    {finding.recommendation && (
                        <p style={{ margin: '0 0 6px', fontSize: 11, color: '#79c0ff', lineHeight: 1.5 }}>
                            <strong>Recommendation:</strong> {finding.recommendation}
                        </p>
                    )}
                    {finding.affected_paths.length > 0 && (
                        <div style={{ marginBottom: 6 }}>
                            <span style={{ fontSize: 10, color: '#8b949e', fontFamily: 'monospace' }}>
                                Affected:{' '}
                            </span>
                            {finding.affected_paths.map(p => (
                                <span key={p} style={{
                                    fontSize: 10,
                                    fontFamily: 'monospace',
                                    color: '#d2a8ff',
                                    background: 'rgba(210,168,255,0.08)',
                                    padding: '1px 4px',
                                    borderRadius: 3,
                                    marginRight: 4,
                                }}>
                                    {p}
                                </span>
                            ))}
                        </div>
                    )}
                    {finding.raw && <CodeViewer raw={finding.raw} />}
                </div>
            )}
        </div>
    );
};

const TierSection: React.FC<{ tier: Tier; findings: CanvasFinding[] }> = ({ tier, findings }) => {
    if (findings.length === 0) return null;
    return (
        <div style={{ marginBottom: 4 }}>
            {/* Sticky tier header */}
            <div style={{
                position: 'sticky',
                top: 0,
                display: 'flex',
                alignItems: 'center',
                gap: 8,
                padding: '4px 12px',
                background: '#161b22',
                borderBottom: `2px solid ${TIER_COLOR[tier]}44`,
                zIndex: 1,
            }}>
                <span style={{
                    fontSize: 10,
                    fontFamily: 'monospace',
                    fontWeight: 700,
                    color: TIER_COLOR[tier],
                    letterSpacing: '0.08em',
                }}>
                    {TIER_LABEL[tier]}
                </span>
                <span style={{
                    fontSize: 10,
                    color: '#8b949e',
                    fontFamily: 'monospace',
                }}>
                    {findings.length} finding{findings.length !== 1 ? 's' : ''}
                </span>
            </div>
            {findings.map(f => <FindingRow key={f.id} finding={f} />)}
        </div>
    );
};

// ── Main workbench ────────────────────────────────────────────────────────────

const TriageWorkbench: React.FC = () => {
    const [snapshot, setSnapshot] = useState<CanvasSnapshot | null>(null);
    const [running, setRunning] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [filter, setFilter] = useState<Tier | 'all'>('all');
    const filterRef = useRef(filter);
    filterRef.current = filter;

    // Load last snapshot on mount and subscribe to live updates.
    useEffect(() => {
        invoke<CanvasSnapshot | null>('triage_last_snapshot')
            .then(s => { if (s) setSnapshot(s); })
            .catch(() => {});

        const unlisten = listen<CanvasSnapshot>('triage:snapshot', ev => {
            setSnapshot(ev.payload);
            setRunning(false);
        });

        return () => { unlisten.then(fn => fn()); };
    }, []);

    const handleRun = useCallback(async () => {
        setRunning(true);
        setError(null);
        try {
            const s = await invoke<CanvasSnapshot>('triage_run');
            setSnapshot(s);
        } catch (e: unknown) {
            setError(String(e));
        } finally {
            setRunning(false);
        }
    }, []);

    // Group findings by tier, sorted priority-first.
    const grouped: Record<Tier, CanvasFinding[]> = {
        critical: [], high: [], medium: [], low: [], info: [],
    };
    if (snapshot) {
        for (const f of snapshot.findings) {
            if (filter === 'all' || f.tier === filter) {
                grouped[f.tier].push(f);
            }
        }
    }

    const totalVisible = TIER_ORDER.reduce((n, t) => n + grouped[t].length, 0);

    return (
        <div style={{
            display: 'flex',
            flexDirection: 'column',
            height: '100%',
            background: '#0d1117',
            color: '#e6edf3',
            fontFamily: '-apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif',
            overflow: 'hidden',
        }}>
            {/* ── Toolbar ── */}
            <div style={{
                display: 'grid',
                gridTemplateColumns: '1fr auto auto',
                alignItems: 'center',
                gap: 8,
                padding: '8px 12px',
                background: '#161b22',
                borderBottom: '1px solid rgba(255,255,255,0.08)',
                flexShrink: 0,
            }}>
                {/* Summary / tier counts */}
                <div style={{ display: 'flex', alignItems: 'center', gap: 6, flexWrap: 'wrap', minWidth: 0 }}>
                    <span style={{ fontSize: 11, fontWeight: 600, color: '#8b949e', marginRight: 4 }}>
                        TRIAGE
                    </span>
                    {snapshot && (
                        <>
                            <RiskBar score={snapshot.risk_score} />
                            {TIER_ORDER.map(t => (
                                <TierCount
                                    key={t}
                                    tier={t}
                                    count={snapshot.tier_counts[t]}
                                />
                            ))}
                        </>
                    )}
                </div>

                {/* Filter selector */}
                <select
                    value={filter}
                    onChange={e => setFilter(e.target.value as Tier | 'all')}
                    style={{
                        background: '#21262d',
                        color: '#c9d1d9',
                        border: '1px solid rgba(255,255,255,0.12)',
                        borderRadius: 4,
                        padding: '3px 6px',
                        fontSize: 11,
                        fontFamily: 'monospace',
                        cursor: 'pointer',
                        outline: 'none',
                    }}
                >
                    <option value="all">All tiers</option>
                    {TIER_ORDER.map(t => (
                        <option key={t} value={t}>{TIER_LABEL[t]}</option>
                    ))}
                </select>

                {/* Run button */}
                <button
                    onClick={handleRun}
                    disabled={running}
                    style={{
                        display: 'flex',
                        alignItems: 'center',
                        gap: 5,
                        padding: '4px 12px',
                        background: running ? 'rgba(255,255,255,0.04)' : '#238636',
                        color: running ? '#8b949e' : '#fff',
                        border: 'none',
                        borderRadius: 4,
                        fontSize: 11,
                        fontWeight: 600,
                        cursor: running ? 'not-allowed' : 'pointer',
                        letterSpacing: '0.03em',
                        flexShrink: 0,
                    }}
                >
                    {running
                        ? <><SpinnerIcon /> Analysing…</>
                        : <>▶ Run Analysis</>}
                </button>
            </div>

            {/* ── Summary bar ── */}
            {snapshot?.summary && (
                <div style={{
                    padding: '6px 12px',
                    fontSize: 11,
                    color: '#8b949e',
                    background: '#0d1117',
                    borderBottom: '1px solid rgba(255,255,255,0.04)',
                    flexShrink: 0,
                }}>
                    {snapshot.summary}
                    <span style={{ marginLeft: 8, opacity: 0.5, fontFamily: 'monospace' }}>
                        {snapshot.run_id}
                    </span>
                </div>
            )}

            {/* ── Error banner ── */}
            {error && (
                <div style={{
                    padding: '6px 12px',
                    fontSize: 11,
                    color: TIER_COLOR.critical,
                    background: TIER_BG.critical,
                    borderBottom: `1px solid ${TIER_COLOR.critical}33`,
                    flexShrink: 0,
                    fontFamily: 'monospace',
                }}>
                    {error}
                </div>
            )}

            {/* ── Findings canvas ── */}
            <div style={{ flex: 1, overflowY: 'auto', overflowX: 'hidden' }}>
                {!snapshot && !running && (
                    <EmptyState onRun={handleRun} />
                )}

                {snapshot && totalVisible === 0 && (
                    <div style={{
                        padding: '40px 0',
                        textAlign: 'center',
                        fontSize: 12,
                        color: '#8b949e',
                    }}>
                        No findings match the selected filter.
                    </div>
                )}

                {TIER_ORDER.map(tier => (
                    <TierSection key={tier} tier={tier} findings={grouped[tier]} />
                ))}

                {/* Dependency chain table */}
                {snapshot && snapshot.dependency_chains.length > 0 && (filter === 'all') && (
                    <ChainTable edges={snapshot.dependency_chains} />
                )}
            </div>
        </div>
    );
};

// ── Dependency chain table ────────────────────────────────────────────────────

const ChainTable: React.FC<{ edges: DependencyEdge[] }> = ({ edges }) => (
    <div style={{ padding: '8px 12px 16px' }}>
        <div style={{
            fontSize: 10,
            fontFamily: 'monospace',
            fontWeight: 700,
            color: '#8b949e',
            letterSpacing: '0.08em',
            marginBottom: 6,
        }}>
            DEPENDENCY CHAINS
        </div>
        <div style={{
            display: 'grid',
            gridTemplateColumns: '1fr 16px 1fr 1fr',
            gap: '2px 8px',
            fontSize: 11,
            fontFamily: 'monospace',
        }}>
            {/* Header */}
            {['FROM', '', 'TO', 'RISK'].map((h, i) => (
                <span key={i} style={{ color: '#8b949e', fontSize: 10, padding: '2px 0' }}>{h}</span>
            ))}
            {/* Rows */}
            {edges.map((e, i) => (
                <React.Fragment key={i}>
                    <span style={{ color: '#79c0ff', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{e.from}</span>
                    <span style={{ color: '#8b949e', textAlign: 'center' }}>→</span>
                    <span style={{ color: '#79c0ff', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{e.to}</span>
                    <span style={{ color: '#d29922', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{e.risk}</span>
                </React.Fragment>
            ))}
        </div>
    </div>
);

// ── Helpers ───────────────────────────────────────────────────────────────────

const EmptyState: React.FC<{ onRun: () => void }> = ({ onRun }) => (
    <div style={{
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        height: '100%',
        gap: 12,
        color: '#8b949e',
    }}>
        <span style={{ fontSize: 32, opacity: 0.2 }}>◈</span>
        <span style={{ fontSize: 12 }}>No analysis run yet.</span>
        <button
            onClick={onRun}
            style={{
                padding: '5px 14px',
                background: '#238636',
                color: '#fff',
                border: 'none',
                borderRadius: 4,
                fontSize: 11,
                fontWeight: 600,
                cursor: 'pointer',
            }}
        >
            ▶ Run Analysis
        </button>
    </div>
);

/** Pure CSS spinner — zero GPU cost, uses CSS `animation` on a single div. */
const SpinnerIcon: React.FC = () => (
    <span style={{
        display: 'inline-block',
        width: 9,
        height: 9,
        border: '1.5px solid rgba(255,255,255,0.2)',
        borderTopColor: '#fff',
        borderRadius: '50%',
        animation: 'triage-spin 0.7s linear infinite',
    }}>
        <style>{`@keyframes triage-spin { to { transform: rotate(360deg); } }`}</style>
    </span>
);

export default TriageWorkbench;
