/**
 * ApiProbe — multi-key API probe workbench (brutecat methodology).
 *
 * Implements the full workflow:
 *   1. Create / load a test session (target URL + endpoint groups)
 *   2. Add API keys with restriction types
 *   3. Browse endpoint groups; fire probes across all keys
 *   4. See responses grouped by hash (identical responses collapse)
 *   5. Report findings with provenance op_ids
 *   6. confirm_testing_complete() to ensure no endpoint skipped
 *   7. Context carry-forward — view prior findings before each group
 *
 * Zero heavy animations. CSS Grid / Flexbox only.
 */

import React, { useCallback, useEffect, useRef, useState } from 'react';
import { invoke } from '../../tauri_bridge.ts';

// ── Domain types (mirror Rust) ────────────────────────────────────────────────

type KeyRestriction =
    | { type: 'none' }
    | { type: 'browser'; origin: string }
    | { type: 'android'; package: string; certificate_hash: string }
    | { type: 'ios'; bundle_id: string }
    | { type: 'server' };

interface ApiKey {
    value: string;
    label: string;
    restriction: KeyRestriction;
}

interface EndpointSpec {
    method: string;
    path: string;
    schema?: unknown;
}

interface EndpointGroup {
    name: string;
    description: string;
    attack_vectors: string[];
    endpoints: EndpointSpec[];
    out_of_scope: string[];
}

interface ProbeResult {
    op_id: string;
    key_label: string;
    status: number;
    body: string;
    body_hash: string;
    duration_ms: number;
    error?: { type: string; detail?: string; label?: string; code?: number };
}

interface GroupEntry {
    sample: ProbeResult;
    key_labels: string[];
    count: number;
}

interface ProbeGroup {
    by_hash: Record<string, GroupEntry>;
    fired: number;
    errors: string[];
}

interface ProbeApiResult {
    group: ProbeGroup;
    op_ids: string[];
}

type Severity = 'critical' | 'high' | 'medium' | 'low' | 'info';

interface SecurityFinding {
    find_id: string;
    title: string;
    severity: Severity;
    description: string;
    recommendation: string;
    op_ids: string[];
    verified: boolean;
    mitre_id?: string;
    timestamp_ns: number;
}

interface CompletionReport {
    complete: boolean;
    untested: [string, string, string][];
    total_endpoints: number;
    tested_count: number;
    finding_count: number;
}

interface LedgerStats {
    op_count: number;
    find_count: number;
    verified_count: number;
}

// ── Colours ───────────────────────────────────────────────────────────────────

const SEV_COLOR: Record<Severity, string> = {
    critical: '#f85149',
    high: '#ff7b72',
    medium: '#d29922',
    low: '#79c0ff',
    info: '#8b949e',
};

const STATUS_COLOR = (s: number) =>
    s >= 500 ? '#f85149' : s >= 400 ? '#d29922' : s >= 300 ? '#79c0ff' : '#3fb950';

const METHOD_COLOR: Record<string, string> = {
    GET: '#3fb950',
    POST: '#d2a8ff',
    PUT: '#d29922',
    DELETE: '#f85149',
    PATCH: '#79c0ff',
    OPTIONS: '#8b949e',
};

// ── Small components ──────────────────────────────────────────────────────────

const MethodBadge: React.FC<{ method: string }> = ({ method }) => (
    <span style={{
        display: 'inline-block',
        minWidth: 52,
        textAlign: 'center',
        padding: '1px 6px',
        borderRadius: 3,
        fontSize: 10,
        fontFamily: 'monospace',
        fontWeight: 700,
        color: METHOD_COLOR[method] ?? '#8b949e',
        background: `${METHOD_COLOR[method] ?? '#8b949e'}18`,
    }}>
        {method}
    </span>
);

const SevBadge: React.FC<{ sev: Severity }> = ({ sev }) => (
    <span style={{
        padding: '1px 6px',
        borderRadius: 3,
        fontSize: 10,
        fontFamily: 'monospace',
        fontWeight: 700,
        color: SEV_COLOR[sev],
        background: `${SEV_COLOR[sev]}18`,
        border: `1px solid ${SEV_COLOR[sev]}33`,
    }}>
        {sev.toUpperCase()}
    </span>
);

const StatusDot: React.FC<{ status: number }> = ({ status }) => (
    <span style={{
        fontFamily: 'monospace',
        fontSize: 11,
        fontWeight: 700,
        color: STATUS_COLOR(status),
    }}>
        {status}
    </span>
);

const Hash: React.FC<{ h: string }> = ({ h }) => (
    <span style={{ fontFamily: 'monospace', fontSize: 10, color: '#8b949e' }} title={h}>
        {h.slice(0, 8)}
    </span>
);

// ── Key manager panel ─────────────────────────────────────────────────────────

const KeyManager: React.FC<{
    sessionId: string;
    onAdded: () => void;
}> = ({ sessionId, onAdded }) => {
    const [raw, setRaw] = useState('');
    const [label, setLabel] = useState('');
    const [restriction, setRestriction] = useState<'none' | 'browser' | 'android' | 'ios' | 'server'>('none');
    const [origin, setOrigin] = useState('');
    const [adding, setAdding] = useState(false);

    const handleAdd = async () => {
        const keys: ApiKey[] = raw
            .split('\n')
            .map(l => l.trim())
            .filter(Boolean)
            .map((v, i): ApiKey => ({
                value: v,
                label: label || `key-${i + 1}`,
                restriction: restriction === 'browser'
                    ? { type: 'browser' as const, origin }
                    : restriction === 'android'
                    ? { type: 'android' as const, package: '', certificate_hash: '' }
                    : restriction === 'ios'
                    ? { type: 'ios' as const, bundle_id: '' }
                    : restriction === 'server'
                    ? { type: 'server' as const }
                    : { type: 'none' as const },
            }));
        if (!keys.length) return;
        setAdding(true);
        try {
            await invoke('probe_add_keys', { args: { session_id: sessionId, keys } });
            setRaw('');
            setLabel('');
            onAdded();
        } finally {
            setAdding(false);
        }
    };

    return (
        <div style={{ display: 'flex', flexDirection: 'column', gap: 6 }}>
            <div style={{ fontSize: 10, color: '#8b949e', fontFamily: 'monospace', fontWeight: 700, letterSpacing: '0.06em' }}>
                ADD API KEYS
            </div>
            <textarea
                placeholder={'Paste keys, one per line\nAIza...\nAIzb...\n'}
                value={raw}
                onChange={e => setRaw(e.target.value)}
                style={{
                    background: '#0d1117', color: '#c9d1d9', border: '1px solid rgba(255,255,255,0.1)',
                    borderRadius: 4, padding: '6px 8px', fontSize: 11, fontFamily: 'monospace',
                    resize: 'vertical', minHeight: 80, outline: 'none',
                }}
            />
            <div style={{ display: 'flex', gap: 6 }}>
                <input
                    placeholder="Label prefix (optional)"
                    value={label}
                    onChange={e => setLabel(e.target.value)}
                    style={inputStyle}
                />
                <select
                    value={restriction}
                    onChange={e => setRestriction(e.target.value as typeof restriction)}
                    style={{ ...inputStyle, flexShrink: 0, width: 110 }}
                >
                    <option value="none">None</option>
                    <option value="browser">Browser</option>
                    <option value="android">Android</option>
                    <option value="ios">iOS</option>
                    <option value="server">Server</option>
                </select>
            </div>
            {restriction === 'browser' && (
                <input
                    placeholder="Referer origin (https://…)"
                    value={origin}
                    onChange={e => setOrigin(e.target.value)}
                    style={inputStyle}
                />
            )}
            <button
                onClick={handleAdd}
                disabled={adding || !raw.trim()}
                style={btnStyle(adding || !raw.trim())}
            >
                {adding ? 'Adding…' : '+ Add Keys'}
            </button>
        </div>
    );
};

// ── Probe panel ───────────────────────────────────────────────────────────────

const ProbePanel: React.FC<{
    sessionId: string;
    baseUrl: string;
    onProbed: (result: ProbeApiResult, method: string, url: string, groupName?: string, path?: string) => void;
    prefillMethod?: string;
    prefillPath?: string;
    prefillGroupName?: string;
}> = ({ sessionId, baseUrl, onProbed, prefillMethod, prefillPath, prefillGroupName }) => {
    const [method, setMethod] = useState(prefillMethod ?? 'GET');
    const [path, setPath] = useState(prefillPath ?? '');
    const [body, setBody] = useState('');
    const [visLabel, setVisLabel] = useState('');
    const [firing, setFiring] = useState(false);
    const [error, setError] = useState('');

    // Sync prefill when parent changes selection.
    useEffect(() => {
        if (prefillMethod) setMethod(prefillMethod);
        if (prefillPath !== undefined) setPath(prefillPath);
    }, [prefillMethod, prefillPath]);

    const fire = async () => {
        const url = path.startsWith('http') ? path : `${baseUrl.replace(/\/$/, '')}${path}`;
        setFiring(true);
        setError('');
        try {
            const result = await invoke<ProbeApiResult>('probe_api', {
                args: {
                    session_id: sessionId,
                    method,
                    url,
                    headers: [],
                    body,
                    key_labels: [],
                    visibility_label: visLabel || null,
                    follow_redirects: false,
                    group_name: prefillGroupName ?? null,
                    endpoint_path: prefillPath ?? null,
                },
            });
            onProbed(result, method, url, prefillGroupName, prefillPath);
        } catch (e) {
            setError(String(e));
        } finally {
            setFiring(false);
        }
    };

    return (
        <div style={{ display: 'flex', flexDirection: 'column', gap: 6, padding: '8px 0' }}>
            <div style={{ display: 'flex', gap: 6, alignItems: 'stretch' }}>
                <select
                    value={method}
                    onChange={e => setMethod(e.target.value)}
                    style={{ ...inputStyle, width: 80, flexShrink: 0 }}
                >
                    {['GET', 'POST', 'PUT', 'PATCH', 'DELETE', 'OPTIONS'].map(m => (
                        <option key={m} value={m}>{m}</option>
                    ))}
                </select>
                <input
                    value={path}
                    onChange={e => setPath(e.target.value)}
                    placeholder="/v1/endpoint  or  https://full-url"
                    style={{ ...inputStyle, flex: 1 }}
                />
                <button onClick={fire} disabled={firing || !path.trim()} style={btnStyle(firing || !path.trim())}>
                    {firing ? '…' : '▶ Fire All Keys'}
                </button>
            </div>
            <div style={{ display: 'flex', gap: 6 }}>
                <input
                    value={visLabel}
                    onChange={e => setVisLabel(e.target.value)}
                    placeholder="Visibility label (optional: GOOGLE_INTERNAL, TRUSTED_TESTER…)"
                    style={{ ...inputStyle, flex: 1 }}
                />
            </div>
            <textarea
                value={body}
                onChange={e => setBody(e.target.value)}
                placeholder='{"key": "value"} — optional request body'
                style={{
                    background: '#0d1117', color: '#c9d1d9',
                    border: '1px solid rgba(255,255,255,0.1)', borderRadius: 4,
                    padding: '6px 8px', fontSize: 11, fontFamily: 'monospace',
                    resize: 'vertical', minHeight: 56, outline: 'none',
                }}
            />
            {error && <div style={{ fontSize: 11, color: SEV_COLOR.critical, fontFamily: 'monospace' }}>{error}</div>}
        </div>
    );
};

// ── Response hash group view ──────────────────────────────────────────────────

const ResponseGroups: React.FC<{
    result: ProbeApiResult;
    onReportFinding: (opId: string, body: string) => void;
}> = ({ result, onReportFinding }) => {
    const [expanded, setExpanded] = useState<string | null>(null);
    const entries = Object.entries(result.group.by_hash);

    return (
        <div style={{ marginTop: 8 }}>
            <div style={{ fontSize: 10, color: '#8b949e', fontFamily: 'monospace', marginBottom: 4 }}>
                {result.group.fired} requests fired · {entries.length} distinct response{entries.length !== 1 ? 's' : ''}
                {result.group.errors.length > 0 && (
                    <span style={{ color: SEV_COLOR.high, marginLeft: 8 }}>
                        {result.group.errors.length} network error(s)
                    </span>
                )}
            </div>
            {entries.map(([hash, entry]) => (
                <div key={hash} style={{
                    border: '1px solid rgba(255,255,255,0.08)',
                    borderRadius: 4,
                    marginBottom: 4,
                    overflow: 'hidden',
                }}>
                    {/* Collapsed summary */}
                    <div
                        role="button"
                        tabIndex={0}
                        onClick={() => setExpanded(exp => exp === hash ? null : hash)}
                        onKeyDown={e => e.key === 'Enter' && setExpanded(exp => exp === hash ? null : hash)}
                        style={{
                            display: 'grid',
                            gridTemplateColumns: '44px 1fr 80px 60px 16px',
                            alignItems: 'center',
                            gap: 8,
                            padding: '5px 10px',
                            background: '#161b22',
                            cursor: 'pointer',
                            outline: 'none',
                        }}
                    >
                        <StatusDot status={entry.sample.status} />
                        <span style={{ fontSize: 11, color: '#8b949e', fontFamily: 'monospace', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                            {entry.key_labels.slice(0, 3).join(', ')}{entry.count > 3 ? ` +${entry.count - 3}` : ''}
                        </span>
                        <Hash h={hash} />
                        <span style={{ fontSize: 10, color: '#8b949e', fontFamily: 'monospace' }}>
                            {entry.sample.duration_ms}ms
                        </span>
                        <span style={{ color: '#8b949e', fontSize: 10 }}>
                            {expanded === hash ? '▾' : '▸'}
                        </span>
                    </div>

                    {/* Expanded body + action */}
                    {expanded === hash && (
                        <div style={{ padding: '8px 10px', background: '#0d1117' }}>
                            {entry.sample.error && (
                                <div style={{
                                    marginBottom: 6,
                                    padding: '4px 8px',
                                    background: `${SEV_COLOR.high}18`,
                                    borderRadius: 3,
                                    fontSize: 11,
                                    fontFamily: 'monospace',
                                    color: SEV_COLOR.high,
                                }}>
                                    {entry.sample.error.type}
                                    {entry.sample.error.detail && `: ${entry.sample.error.detail}`}
                                    {entry.sample.error.label && ` (label: ${entry.sample.error.label})`}
                                </div>
                            )}
                            <pre style={{
                                margin: 0,
                                padding: '8px',
                                background: '#010409',
                                borderRadius: 3,
                                fontFamily: '"Cascadia Mono","JetBrains Mono",Consolas,monospace',
                                fontSize: 11,
                                color: '#c9d1d9',
                                overflowX: 'auto',
                                maxHeight: 280,
                                overflowY: 'auto',
                                whiteSpace: 'pre-wrap',
                                wordBreak: 'break-all',
                            }}>
                                {(() => {
                                    try {
                                        return JSON.stringify(JSON.parse(entry.sample.body), null, 2);
                                    } catch {
                                        return entry.sample.body;
                                    }
                                })()}
                            </pre>
                            <div style={{ marginTop: 6, display: 'flex', gap: 6, justifyContent: 'flex-end' }}>
                                <button
                                    onClick={() => onReportFinding(entry.sample.op_id, entry.sample.body)}
                                    style={{ ...btnStyle(false), background: '#7d2d2d', fontSize: 10, padding: '3px 10px' }}
                                >
                                    ⚑ Report Finding
                                </button>
                            </div>
                        </div>
                    )}
                </div>
            ))}
        </div>
    );
};

// ── Report finding modal ──────────────────────────────────────────────────────

const ReportModal: React.FC<{
    sessionId: string;
    opId: string;
    prefillBody: string;
    onClose: () => void;
    onReported: (findId: string) => void;
}> = ({ sessionId, opId, prefillBody, onClose, onReported }) => {
    const [title, setTitle] = useState('');
    const [severity, setSeverity] = useState<Severity>('high');
    const [description, setDescription] = useState('');
    const [recommendation, setRecommendation] = useState('');
    const [mitreId, setMitreId] = useState('');
    const [submitting, setSubmitting] = useState(false);
    const [error, setError] = useState('');

    const submit = async () => {
        if (!title.trim() || !description.trim()) {
            setError('Title and description required.');
            return;
        }
        setSubmitting(true);
        try {
            const findId = await invoke<string>('probe_report_vulnerability', {
                args: {
                    session_id: sessionId,
                    title,
                    severity,
                    description,
                    recommendation,
                    op_ids: [opId],
                    mitre_id: mitreId || null,
                },
            });
            onReported(findId);
            onClose();
        } catch (e) {
            setError(String(e));
        } finally {
            setSubmitting(false);
        }
    };

    return (
        <div style={{
            position: 'fixed', inset: 0, background: 'rgba(0,0,0,0.7)',
            display: 'flex', alignItems: 'center', justifyContent: 'center', zIndex: 9999,
        }}
            onClick={e => { if (e.target === e.currentTarget) onClose(); }}
        >
            <div style={{
                width: 520, background: '#161b22', border: '1px solid rgba(255,255,255,0.12)',
                borderRadius: 8, padding: 20, display: 'flex', flexDirection: 'column', gap: 10,
            }}>
                <div style={{ fontSize: 12, fontWeight: 700, color: '#e6edf3' }}>Report Finding</div>
                <div style={{ fontSize: 10, color: '#8b949e', fontFamily: 'monospace' }}>
                    Linked to <span style={{ color: '#79c0ff' }}>{opId}</span>
                </div>
                <input value={title} onChange={e => setTitle(e.target.value)} placeholder="Title" style={inputStyle} />
                <div style={{ display: 'flex', gap: 6 }}>
                    <select value={severity} onChange={e => setSeverity(e.target.value as Severity)} style={{ ...inputStyle, width: 120 }}>
                        {(['critical', 'high', 'medium', 'low', 'info'] as Severity[]).map(s => (
                            <option key={s} value={s}>{s.toUpperCase()}</option>
                        ))}
                    </select>
                    <input value={mitreId} onChange={e => setMitreId(e.target.value)} placeholder="MITRE ID (optional, e.g. T1078)" style={{ ...inputStyle, flex: 1 }} />
                </div>
                <textarea
                    value={description}
                    onChange={e => setDescription(e.target.value)}
                    placeholder="Describe the vulnerability…"
                    style={{ ...textareaStyle, minHeight: 80 }}
                />
                <textarea
                    value={recommendation}
                    onChange={e => setRecommendation(e.target.value)}
                    placeholder="Recommendation / fix…"
                    style={{ ...textareaStyle, minHeight: 56 }}
                />
                {error && <div style={{ fontSize: 11, color: SEV_COLOR.critical }}>{error}</div>}
                <div style={{ display: 'flex', gap: 8, justifyContent: 'flex-end' }}>
                    <button onClick={onClose} style={{ ...btnStyle(false), background: 'transparent', border: '1px solid rgba(255,255,255,0.15)', color: '#8b949e' }}>
                        Cancel
                    </button>
                    <button onClick={submit} disabled={submitting} style={btnStyle(submitting)}>
                        {submitting ? 'Reporting…' : 'Submit Finding'}
                    </button>
                </div>
            </div>
        </div>
    );
};

// ── Findings list ─────────────────────────────────────────────────────────────

const FindingsList: React.FC<{
    sessionId: string;
    reload: number;
    onVerify: (findId: string) => void;
}> = ({ sessionId, reload, onVerify }) => {
    const [findings, setFindings] = useState<SecurityFinding[]>([]);

    useEffect(() => {
        invoke<SecurityFinding[]>('probe_session_findings', { session_id: sessionId })
            .then(setFindings)
            .catch(() => {});
    }, [sessionId, reload]);

    if (!findings.length) {
        return (
            <div style={{ padding: '20px 0', textAlign: 'center', fontSize: 11, color: '#8b949e' }}>
                No findings yet.
            </div>
        );
    }

    return (
        <div style={{ display: 'flex', flexDirection: 'column', gap: 4 }}>
            {findings.map(f => (
                <div key={f.find_id} style={{
                    padding: '8px 10px',
                    background: '#161b22',
                    border: `1px solid ${SEV_COLOR[f.severity]}33`,
                    borderRadius: 4,
                }}>
                    <div style={{ display: 'flex', alignItems: 'center', gap: 8, marginBottom: 4 }}>
                        <SevBadge sev={f.severity} />
                        <span style={{ fontSize: 12, color: '#e6edf3', flex: 1 }}>{f.title}</span>
                        {f.verified && (
                            <span style={{ fontSize: 10, color: '#3fb950', fontFamily: 'monospace' }}>✓ verified</span>
                        )}
                        {!f.verified && (
                            <button
                                onClick={() => onVerify(f.find_id)}
                                style={{ fontSize: 10, padding: '2px 8px', background: '#238636', color: '#fff', border: 'none', borderRadius: 3, cursor: 'pointer' }}
                            >
                                Verify
                            </button>
                        )}
                    </div>
                    <p style={{ margin: '0 0 4px', fontSize: 11, color: '#c9d1d9', lineHeight: 1.5 }}>
                        {f.description}
                    </p>
                    {f.recommendation && (
                        <p style={{ margin: '0 0 4px', fontSize: 10, color: '#79c0ff' }}>
                            ↳ {f.recommendation}
                        </p>
                    )}
                    <div style={{ fontSize: 10, fontFamily: 'monospace', color: '#8b949e' }}>
                        {f.op_ids.map(id => (
                            <span key={id} style={{ marginRight: 6, background: 'rgba(255,255,255,0.06)', padding: '1px 4px', borderRadius: 3 }}>{id}</span>
                        ))}
                        {f.mitre_id && <span style={{ color: '#d2a8ff', marginLeft: 4 }}>{f.mitre_id}</span>}
                    </div>
                </div>
            ))}
        </div>
    );
};

// ── Completion checker ────────────────────────────────────────────────────────

const CompletionChecker: React.FC<{ sessionId: string }> = ({ sessionId }) => {
    const [report, setReport] = useState<CompletionReport | null>(null);
    const [checking, setChecking] = useState(false);

    const check = async () => {
        setChecking(true);
        try {
            // probe_confirm_complete returns Ok if complete, Err (JSON string) if not.
            const r = await invoke<CompletionReport>('probe_confirm_complete', { session_id: sessionId });
            setReport(r);
        } catch (e: unknown) {
            try {
                setReport(JSON.parse(String(e)));
            } catch {
                setReport(null);
            }
        } finally {
            setChecking(false);
        }
    };

    return (
        <div style={{ display: 'flex', flexDirection: 'column', gap: 6 }}>
            <button onClick={check} disabled={checking} style={btnStyle(checking)}>
                {checking ? 'Checking…' : '✓ Confirm Testing Complete'}
            </button>
            {report && (
                <div style={{
                    padding: '8px 10px',
                    background: report.complete ? 'rgba(63,185,80,0.08)' : 'rgba(210,153,34,0.08)',
                    border: `1px solid ${report.complete ? '#3fb95044' : '#d2992244'}`,
                    borderRadius: 4,
                    fontSize: 11,
                    fontFamily: 'monospace',
                }}>
                    {report.complete ? (
                        <span style={{ color: '#3fb950' }}>
                            ✓ All {report.total_endpoints} in-scope endpoints tested · {report.finding_count} finding(s)
                        </span>
                    ) : (
                        <>
                            <div style={{ color: '#d29922', marginBottom: 4 }}>
                                ⚠ {report.tested_count}/{report.total_endpoints} tested — {report.untested.length} remaining:
                            </div>
                            <ul style={{ margin: 0, paddingLeft: 16 }}>
                                {report.untested.slice(0, 10).map(([g, m, p], i) => (
                                    <li key={i} style={{ color: '#c9d1d9' }}>
                                        [{g}] {m} {p}
                                    </li>
                                ))}
                                {report.untested.length > 10 && (
                                    <li style={{ color: '#8b949e' }}>+{report.untested.length - 10} more</li>
                                )}
                            </ul>
                        </>
                    )}
                </div>
            )}
        </div>
    );
};

// ── Main workbench ────────────────────────────────────────────────────────────

type TabId = 'probe' | 'groups' | 'findings' | 'keys';

const ApiProbe: React.FC = () => {
    const [sessions, setSessions] = useState<string[]>([]);
    const [sessionId, setSessionId] = useState('');
    const [baseUrl, setBaseUrl] = useState('');
    const [newTarget, setNewTarget] = useState('');
    const [creating, setCreating] = useState(false);
    const [tab, setTab] = useState<TabId>('probe');

    // Current probe state.
    const [lastResult, setLastResult] = useState<ProbeApiResult | null>(null);
    const [selectedGroup, setSelectedGroup] = useState<EndpointGroup | null>(null);
    const [selectedEndpoint, setSelectedEndpoint] = useState<EndpointSpec | null>(null);
    const [groups, setGroups] = useState<EndpointGroup[]>([]);

    // Report modal.
    const [reportOp, setReportOp] = useState<{ opId: string; body: string } | null>(null);
    const [findingsReload, setFindingsReload] = useState(0);

    // Stats.
    const [stats, setStats] = useState<LedgerStats | null>(null);

    const reloadStats = useCallback((sid: string) => {
        invoke<LedgerStats>('probe_session_stats', { session_id: sid })
            .then(setStats)
            .catch(() => {});
    }, []);

    useEffect(() => {
        invoke<string[]>('probe_list_sessions').then(setSessions).catch(() => {});
    }, []);

    useEffect(() => {
        if (sessionId) reloadStats(sessionId);
    }, [sessionId, findingsReload, reloadStats]);

    const createSession = async () => {
        if (!newTarget.trim()) return;
        setCreating(true);
        try {
            const sid = await invoke<string>('probe_create_session', {
                args: { target_base_url: newTarget.trim(), groups: [] },
            });
            setSessionId(sid);
            setBaseUrl(newTarget.trim());
            setSessions(s => [...s, sid]);
            setNewTarget('');
        } finally {
            setCreating(false);
        }
    };

    const handleProbed = (result: ProbeApiResult, _method: string, _url: string) => {
        setLastResult(result);
        reloadStats(sessionId);
    };

    const handleVerify = async (findId: string) => {
        await invoke('probe_verify_finding', { find_id: findId });
        setFindingsReload(n => n + 1);
    };

    // ── No session yet ──
    if (!sessionId) {
        return (
            <div style={containerStyle}>
                <div style={{ padding: '40px 24px', maxWidth: 480, margin: '0 auto', display: 'flex', flexDirection: 'column', gap: 12 }}>
                    <div style={{ fontSize: 13, fontWeight: 700, color: '#e6edf3' }}>API Probe — Multi-Key Engine</div>
                    <div style={{ fontSize: 11, color: '#8b949e', lineHeight: 1.6 }}>
                        Fires every request across all collected API keys simultaneously.
                        Identical responses collapse by hash — divergent auth-scoped data stands out immediately.
                    </div>
                    <div style={{ display: 'flex', gap: 6 }}>
                        <input
                            value={newTarget}
                            onChange={e => setNewTarget(e.target.value)}
                            onKeyDown={e => e.key === 'Enter' && createSession()}
                            placeholder="https://target.example.com"
                            style={{ ...inputStyle, flex: 1 }}
                        />
                        <button onClick={createSession} disabled={creating || !newTarget.trim()} style={btnStyle(creating || !newTarget.trim())}>
                            {creating ? '…' : 'Create Session'}
                        </button>
                    </div>
                    {sessions.length > 0 && (
                        <>
                            <div style={{ fontSize: 10, color: '#8b949e', marginTop: 8 }}>Resume session:</div>
                            {sessions.map(sid => (
                                <button key={sid} onClick={() => setSessionId(sid)} style={{ ...btnStyle(false), background: '#21262d', color: '#79c0ff', textAlign: 'left' }}>
                                    {sid}
                                </button>
                            ))}
                        </>
                    )}
                </div>
            </div>
        );
    }

    // ── Active session ──
    return (
        <div style={containerStyle}>
            {/* Header */}
            <div style={{
                display: 'grid',
                gridTemplateColumns: '1fr auto auto auto auto',
                alignItems: 'center',
                gap: 8,
                padding: '6px 12px',
                background: '#161b22',
                borderBottom: '1px solid rgba(255,255,255,0.08)',
                flexShrink: 0,
            }}>
                <span style={{ fontSize: 11, fontFamily: 'monospace', color: '#8b949e' }}>
                    <span style={{ color: '#79c0ff' }}>{sessionId}</span>
                    {' · '}{baseUrl}
                </span>
                {stats && (
                    <>
                        <Chip label={`${stats.op_count} ops`} />
                        <Chip label={`${stats.find_count} findings`} color={stats.find_count > 0 ? '#f85149' : undefined} />
                        <Chip label={`${stats.verified_count} verified`} color={stats.verified_count > 0 ? '#3fb950' : undefined} />
                    </>
                )}
                <button onClick={() => setSessionId('')} style={{ fontSize: 10, background: 'transparent', color: '#8b949e', border: '1px solid rgba(255,255,255,0.1)', borderRadius: 3, padding: '2px 6px', cursor: 'pointer' }}>
                    ← Sessions
                </button>
            </div>

            {/* Tabs */}
            <div style={{ display: 'flex', borderBottom: '1px solid rgba(255,255,255,0.08)', background: '#161b22', flexShrink: 0 }}>
                {(['probe', 'groups', 'findings', 'keys'] as TabId[]).map(t => (
                    <button
                        key={t}
                        onClick={() => setTab(t)}
                        style={{
                            padding: '6px 14px',
                            fontSize: 11,
                            background: 'transparent',
                            color: tab === t ? '#e6edf3' : '#8b949e',
                            border: 'none',
                            borderBottom: tab === t ? '2px solid #79c0ff' : '2px solid transparent',
                            cursor: 'pointer',
                            fontFamily: '-apple-system, sans-serif',
                            textTransform: 'capitalize',
                        }}
                    >
                        {t}
                    </button>
                ))}
            </div>

            {/* Tab content */}
            <div style={{ flex: 1, overflowY: 'auto', padding: '8px 12px' }}>
                {tab === 'probe' && (
                    <>
                        <ProbePanel
                            sessionId={sessionId}
                            baseUrl={baseUrl}
                            onProbed={handleProbed}
                            prefillMethod={selectedEndpoint?.method}
                            prefillPath={selectedEndpoint?.path}
                            prefillGroupName={selectedGroup?.name}
                        />
                        {lastResult && (
                            <ResponseGroups
                                result={lastResult}
                                onReportFinding={(opId, body) => setReportOp({ opId, body })}
                            />
                        )}
                        <div style={{ marginTop: 12 }}>
                            <CompletionChecker sessionId={sessionId} />
                        </div>
                    </>
                )}

                {tab === 'groups' && (
                    <GroupBrowser
                        groups={groups}
                        onAddGroup={g => setGroups(gs => [...gs, g])}
                        onSelectEndpoint={(group, ep) => {
                            setSelectedGroup(group);
                            setSelectedEndpoint(ep);
                            setTab('probe');
                        }}
                    />
                )}

                {tab === 'findings' && (
                    <FindingsList
                        sessionId={sessionId}
                        reload={findingsReload}
                        onVerify={handleVerify}
                    />
                )}

                {tab === 'keys' && (
                    <KeyManager sessionId={sessionId} onAdded={() => reloadStats(sessionId)} />
                )}
            </div>

            {/* Report modal */}
            {reportOp && (
                <ReportModal
                    sessionId={sessionId}
                    opId={reportOp.opId}
                    prefillBody={reportOp.body}
                    onClose={() => setReportOp(null)}
                    onReported={() => setFindingsReload(n => n + 1)}
                />
            )}
        </div>
    );
};

// ── Group browser ─────────────────────────────────────────────────────────────

const GroupBrowser: React.FC<{
    groups: EndpointGroup[];
    onAddGroup: (g: EndpointGroup) => void;
    onSelectEndpoint: (group: EndpointGroup, ep: EndpointSpec) => void;
}> = ({ groups, onAddGroup, onSelectEndpoint }) => {
    const [expanded, setExpanded] = useState<string | null>(null);
    const [newName, setNewName] = useState('');
    const [newDesc, setNewDesc] = useState('');
    const [endpointsRaw, setEndpointsRaw] = useState('');
    const [adding, setAdding] = useState(false);

    const addGroup = () => {
        const endpoints: EndpointSpec[] = endpointsRaw
            .split('\n')
            .map(l => l.trim())
            .filter(Boolean)
            .map(l => {
                const [m, ...rest] = l.split(' ');
                return { method: m.toUpperCase(), path: rest.join(' ') || '/' };
            });
        if (!newName.trim() || !endpoints.length) return;
        onAddGroup({
            name: newName.trim(),
            description: newDesc,
            attack_vectors: [],
            endpoints,
            out_of_scope: [],
        });
        setNewName('');
        setNewDesc('');
        setEndpointsRaw('');
        setAdding(false);
    };

    return (
        <div style={{ display: 'flex', flexDirection: 'column', gap: 6 }}>
            {groups.length === 0 && !adding && (
                <div style={{ fontSize: 11, color: '#8b949e', padding: '16px 0' }}>
                    No endpoint groups. Add groups to track coverage.
                </div>
            )}
            {groups.map(g => (
                <div key={g.name} style={{ border: '1px solid rgba(255,255,255,0.08)', borderRadius: 4 }}>
                    <div
                        role="button"
                        tabIndex={0}
                        onClick={() => setExpanded(e => e === g.name ? null : g.name)}
                        onKeyDown={e => e.key === 'Enter' && setExpanded(ex => ex === g.name ? null : g.name)}
                        style={{ display: 'flex', alignItems: 'center', gap: 8, padding: '6px 10px', cursor: 'pointer', outline: 'none', background: '#161b22' }}
                    >
                        <span style={{ fontSize: 12, color: '#e6edf3', flex: 1 }}>{g.name}</span>
                        <span style={{ fontSize: 10, color: '#8b949e' }}>{g.endpoints.length} endpoints</span>
                        <span style={{ fontSize: 10, color: '#8b949e' }}>{expanded === g.name ? '▾' : '▸'}</span>
                    </div>
                    {expanded === g.name && (
                        <div style={{ padding: '4px 10px 8px' }}>
                            {g.description && <div style={{ fontSize: 11, color: '#8b949e', marginBottom: 6 }}>{g.description}</div>}
                            {g.endpoints.map(ep => (
                                <div
                                    key={`${ep.method}:${ep.path}`}
                                    role="button"
                                    tabIndex={0}
                                    onClick={() => onSelectEndpoint(g, ep)}
                                    onKeyDown={e => e.key === 'Enter' && onSelectEndpoint(g, ep)}
                                    style={{
                                        display: 'flex', alignItems: 'center', gap: 8, padding: '4px 6px',
                                        cursor: 'pointer', borderRadius: 3, outline: 'none',
                                        background: g.out_of_scope.includes(ep.path) ? 'rgba(255,255,255,0.02)' : 'transparent',
                                    }}
                                >
                                    <MethodBadge method={ep.method} />
                                    <span style={{
                                        fontSize: 11, fontFamily: 'monospace',
                                        color: g.out_of_scope.includes(ep.path) ? '#8b949e' : '#e6edf3',
                                        textDecoration: g.out_of_scope.includes(ep.path) ? 'line-through' : 'none',
                                    }}>
                                        {ep.path}
                                    </span>
                                    {g.out_of_scope.includes(ep.path) && (
                                        <span style={{ fontSize: 9, color: '#8b949e', fontFamily: 'monospace' }}>out-of-scope</span>
                                    )}
                                </div>
                            ))}
                        </div>
                    )}
                </div>
            ))}
            {adding ? (
                <div style={{ padding: '8px', border: '1px dashed rgba(255,255,255,0.12)', borderRadius: 4, display: 'flex', flexDirection: 'column', gap: 6 }}>
                    <input value={newName} onChange={e => setNewName(e.target.value)} placeholder="Group name (e.g. auth)" style={inputStyle} />
                    <input value={newDesc} onChange={e => setNewDesc(e.target.value)} placeholder="Description (optional)" style={inputStyle} />
                    <textarea
                        value={endpointsRaw}
                        onChange={e => setEndpointsRaw(e.target.value)}
                        placeholder={'One endpoint per line:\nGET /v1/users\nPOST /v1/token\nDELETE /v1/sessions/{id}'}
                        style={{ ...textareaStyle, minHeight: 80 }}
                    />
                    <div style={{ display: 'flex', gap: 6 }}>
                        <button onClick={addGroup} style={btnStyle(!newName.trim() || !endpointsRaw.trim())}>Add Group</button>
                        <button onClick={() => setAdding(false)} style={{ ...btnStyle(false), background: 'transparent', border: '1px solid rgba(255,255,255,0.1)', color: '#8b949e' }}>Cancel</button>
                    </div>
                </div>
            ) : (
                <button onClick={() => setAdding(true)} style={{ ...btnStyle(false), background: 'transparent', border: '1px dashed rgba(255,255,255,0.2)', color: '#8b949e' }}>
                    + Add Endpoint Group
                </button>
            )}
        </div>
    );
};

// ── Style helpers ─────────────────────────────────────────────────────────────

const containerStyle: React.CSSProperties = {
    display: 'flex',
    flexDirection: 'column',
    height: '100%',
    background: '#0d1117',
    color: '#e6edf3',
    fontFamily: '-apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif',
    overflow: 'hidden',
};

const inputStyle: React.CSSProperties = {
    background: '#21262d',
    color: '#c9d1d9',
    border: '1px solid rgba(255,255,255,0.1)',
    borderRadius: 4,
    padding: '5px 8px',
    fontSize: 11,
    outline: 'none',
    flex: 1,
};

const textareaStyle: React.CSSProperties = {
    background: '#0d1117',
    color: '#c9d1d9',
    border: '1px solid rgba(255,255,255,0.1)',
    borderRadius: 4,
    padding: '6px 8px',
    fontSize: 11,
    fontFamily: 'monospace',
    resize: 'vertical',
    outline: 'none',
};

const btnStyle = (disabled: boolean): React.CSSProperties => ({
    padding: '5px 12px',
    background: disabled ? 'rgba(255,255,255,0.04)' : '#238636',
    color: disabled ? '#8b949e' : '#fff',
    border: 'none',
    borderRadius: 4,
    fontSize: 11,
    fontWeight: 600,
    cursor: disabled ? 'not-allowed' : 'pointer',
    flexShrink: 0,
    whiteSpace: 'nowrap',
});

const Chip: React.FC<{ label: string; color?: string }> = ({ label, color }) => (
    <span style={{
        fontSize: 10,
        fontFamily: 'monospace',
        color: color ?? '#8b949e',
        background: color ? `${color}18` : 'rgba(255,255,255,0.06)',
        padding: '2px 6px',
        borderRadius: 3,
    }}>
        {label}
    </span>
);

export default ApiProbe;
