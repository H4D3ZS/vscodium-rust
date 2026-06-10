import React, { useCallback, useMemo, useState } from 'react';
import { invoke } from '../../tauri_bridge';

type Hit = {
    index: number;
    payload: string;
    status: number;
    length: number;
    duration_ms: number;
    grep_match: boolean;
    error?: string | null;
};

type Result = { total: number; hits: Hit[]; anomalies: number[] };

const statusColor = (code: number): string => {
    if (code === 0) return '#f87171';
    if (code >= 500) return '#ef4444';
    if (code >= 400) return '#f97316';
    if (code >= 300) return '#eab308';
    return '#22c55e';
};

const IntruderPanel: React.FC = () => {
    const [method, setMethod] = useState('GET');
    const [url, setUrl] = useState('https://target/path?id=§');
    const [headersText, setHeadersText] = useState('User-Agent: HADES-Intruder/1.0');
    const [bodyTpl, setBodyTpl] = useState('');
    const [payloadsText, setPayloadsText] = useState('1\n2\n3\n0\n-1\n9999999');
    const [grep, setGrep] = useState('');
    const [busy, setBusy] = useState(false);
    const [error, setError] = useState('');
    const [result, setResult] = useState<Result | null>(null);
    const [onlyAnomalies, setOnlyAnomalies] = useState(false);

    const run = useCallback(async () => {
        setError('');
        setBusy(true);
        setResult(null);
        try {
            const headers = headersText.split('\n').map((l) => l.trim()).filter(Boolean).map((l) => {
                const i = l.indexOf(':');
                return i < 0 ? [l, ''] as [string, string] : [l.slice(0, i).trim(), l.slice(i + 1).trim()] as [string, string];
            });
            const payloads = payloadsText.split('\n').map((p) => p.trim()).filter(Boolean);
            const r = await invoke<Result>('intruder_run', {
                request: { method, url: url.trim(), headers, body: bodyTpl, payloads, grep: grep.trim() || null, concurrency: 10 },
            });
            setResult(r);
        } catch (e) {
            setError(String(e));
        } finally {
            setBusy(false);
        }
    }, [method, url, headersText, bodyTpl, payloadsText, grep]);

    const anomalySet = useMemo(() => new Set(result?.anomalies ?? []), [result]);
    const rows = useMemo(() => {
        if (!result) return [];
        return onlyAnomalies ? result.hits.filter((h) => anomalySet.has(h.index)) : result.hits;
    }, [result, onlyAnomalies, anomalySet]);

    return (
        <div style={{ display: 'flex', flexDirection: 'column', height: '100%', minHeight: 0 }}>
            <div style={{ padding: '10px 12px', borderBottom: '1px solid var(--vscode-panel-border)', flexShrink: 0 }}>
                <div style={{ display: 'flex', gap: 6, marginBottom: 6 }}>
                    <select value={method} onChange={(e) => setMethod(e.target.value)} style={{ ...inp(80), cursor: 'pointer' }}>
                        {['GET', 'POST', 'PUT', 'PATCH', 'DELETE'].map((m) => <option key={m}>{m}</option>)}
                    </select>
                    <input value={url} onChange={(e) => setUrl(e.target.value)} placeholder="url with § marker" style={{ ...inp(0), flex: 1 }} />
                </div>
                <div style={{ fontSize: 10, opacity: 0.55, marginBottom: 6 }}>
                    Put <code>§</code> where each payload goes (url, headers, or body).
                </div>
                <div style={{ display: 'flex', gap: 6 }}>
                    <textarea value={payloadsText} onChange={(e) => setPayloadsText(e.target.value)} placeholder="payloads, one per line"
                        spellCheck={false} style={{ ...area(), height: 90, flex: 1 }} />
                    <textarea value={headersText} onChange={(e) => setHeadersText(e.target.value)} placeholder="headers"
                        spellCheck={false} style={{ ...area(), height: 90, flex: 1 }} />
                </div>
                <textarea value={bodyTpl} onChange={(e) => setBodyTpl(e.target.value)} placeholder="body template (optional, supports §)"
                    spellCheck={false} style={{ ...area(), height: 40, marginTop: 6 }} />
                <div style={{ display: 'flex', gap: 6, marginTop: 6, alignItems: 'center' }}>
                    <input value={grep} onChange={(e) => setGrep(e.target.value)} placeholder="grep match (optional)" style={{ ...inp(0), flex: 1 }} />
                    <button type="button" disabled={busy} onClick={() => void run()} style={btn('#0e639c')}>
                        {busy ? 'Running…' : 'Attack'}
                    </button>
                </div>
                {error && <div style={{ marginTop: 6, fontSize: 10, color: '#f87171' }}>{error}</div>}
            </div>

            {result && (
                <div style={{ padding: '6px 12px', borderBottom: '1px solid var(--vscode-panel-border)', flexShrink: 0, fontSize: 10, display: 'flex', gap: 10, alignItems: 'center' }}>
                    <span>{result.total} requests</span>
                    <span style={{ color: result.anomalies.length ? '#eab308' : 'inherit' }}>{result.anomalies.length} anomalies</span>
                    <label style={{ display: 'flex', gap: 4, alignItems: 'center', cursor: 'pointer', marginLeft: 'auto' }}>
                        <input type="checkbox" checked={onlyAnomalies} onChange={(e) => setOnlyAnomalies(e.target.checked)} />
                        anomalies only
                    </label>
                </div>
            )}

            <div style={{ flex: 1, overflowY: 'auto', minHeight: 0 }}>
                {!result && <div style={{ padding: 16, fontSize: 11, opacity: 0.5 }}>Define an injection point and payloads, then Attack. Outliers in status/length get flagged.</div>}
                {rows.map((h) => {
                    const anom = anomalySet.has(h.index);
                    return (
                        <div key={h.index} style={{ display: 'flex', gap: 8, alignItems: 'center', padding: '5px 12px', borderBottom: '1px solid var(--vscode-panel-border)', fontSize: 11, background: anom ? 'rgba(234,179,8,0.08)' : 'transparent' }}>
                            <span style={{ width: 28, opacity: 0.5, fontSize: 9 }}>{h.index}</span>
                            <span style={{ color: statusColor(h.status), fontWeight: 700, width: 32 }}>{h.status || 'ERR'}</span>
                            <code style={{ flex: 1, minWidth: 0, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{h.payload}</code>
                            {h.grep_match && <span style={{ color: '#22c55e', fontSize: 9, fontWeight: 700 }}>MATCH</span>}
                            <span style={{ fontSize: 9, opacity: 0.6, width: 64, textAlign: 'right' }}>{h.length}b</span>
                            <span style={{ fontSize: 9, opacity: 0.4, width: 44, textAlign: 'right' }}>{h.duration_ms}ms</span>
                        </div>
                    );
                })}
            </div>
        </div>
    );
};

const inp = (w: number): React.CSSProperties => ({
    width: w || undefined,
    padding: '5px 8px',
    fontSize: 11,
    borderRadius: 4,
    border: '1px solid var(--vscode-panel-border)',
    background: 'var(--vscode-input-background)',
    color: 'var(--vscode-input-foreground)',
});

const area = (): React.CSSProperties => ({
    boxSizing: 'border-box',
    padding: '6px 8px',
    fontSize: 11,
    fontFamily: 'monospace',
    borderRadius: 4,
    border: '1px solid var(--vscode-panel-border)',
    background: 'var(--vscode-input-background)',
    color: 'var(--vscode-input-foreground)',
    resize: 'vertical',
});

const btn = (bg: string): React.CSSProperties => ({
    padding: '5px 14px',
    border: 'none',
    borderRadius: 4,
    cursor: 'pointer',
    fontSize: 11,
    fontWeight: 600,
    background: bg,
    color: '#fff',
});

export default IntruderPanel;
