import React, { useCallback, useEffect, useState } from 'react';
import { invoke } from '../../tauri_bridge';

type ManualResponse = {
    status: number;
    status_text: string;
    headers: [string, string][];
    body: string;
    body_bytes: number;
    duration_ms: number;
    truncated: boolean;
};

const statusColor = (code: number): string => {
    if (code === 0) return '#f87171';
    if (code >= 500) return '#ef4444';
    if (code >= 400) return '#f97316';
    if (code >= 300) return '#eab308';
    return '#22c55e';
};

const RepeaterPanel: React.FC = () => {
    const [method, setMethod] = useState('GET');
    const [url, setUrl] = useState('https://');
    const [headersText, setHeadersText] = useState('User-Agent: HADES-Repeater/1.0');
    const [body, setBody] = useState('');
    const [follow, setFollow] = useState(false);
    const [resp, setResp] = useState<ManualResponse | null>(null);
    const [busy, setBusy] = useState(false);
    const [error, setError] = useState('');

    // Allow other panels (e.g. the proxy flow list) to push a request here.
    useEffect(() => {
        const onSend = (e: Event) => {
            const d = (e as CustomEvent<{ method?: string; url?: string; headers?: [string, string][]; body?: string }>).detail;
            if (!d) return;
            if (d.method) setMethod(d.method);
            if (d.url) setUrl(d.url);
            if (d.headers) setHeadersText(d.headers.map(([k, v]) => `${k}: ${v}`).join('\n'));
            if (typeof d.body === 'string') setBody(d.body);
        };
        window.addEventListener('hades:repeater-load', onSend);
        return () => window.removeEventListener('hades:repeater-load', onSend);
    }, []);

    const parseHeaders = useCallback((): [string, string][] => {
        return headersText
            .split('\n')
            .map((l) => l.trim())
            .filter(Boolean)
            .map((l) => {
                const idx = l.indexOf(':');
                if (idx < 0) return [l, ''] as [string, string];
                return [l.slice(0, idx).trim(), l.slice(idx + 1).trim()] as [string, string];
            });
    }, [headersText]);

    const send = useCallback(async () => {
        setError('');
        setBusy(true);
        try {
            const r = await invoke<ManualResponse>('repeater_send', {
                request: { method, url: url.trim(), headers: parseHeaders(), body, followRedirects: follow },
            });
            setResp(r);
        } catch (e) {
            setError(String(e));
        } finally {
            setBusy(false);
        }
    }, [method, url, body, follow, parseHeaders]);

    return (
        <div style={{ display: 'flex', flexDirection: 'column', height: '100%', minHeight: 0 }}>
            <div style={{ padding: '10px 12px', borderBottom: '1px solid var(--vscode-panel-border)', flexShrink: 0 }}>
                <div style={{ display: 'flex', gap: 6, marginBottom: 6 }}>
                    <select value={method} onChange={(e) => setMethod(e.target.value)} style={{ ...inp(80), cursor: 'pointer' }}>
                        {['GET', 'POST', 'PUT', 'PATCH', 'DELETE', 'HEAD', 'OPTIONS'].map((m) => <option key={m}>{m}</option>)}
                    </select>
                    <input value={url} onChange={(e) => setUrl(e.target.value)} placeholder="https://target/path" style={{ ...inp(0), flex: 1 }} />
                    <button type="button" disabled={busy || url.trim().length < 8} onClick={() => void send()} style={btn('#0e639c')}>
                        {busy ? '…' : 'Send'}
                    </button>
                </div>
                <textarea value={headersText} onChange={(e) => setHeadersText(e.target.value)} placeholder="Header: value (one per line)"
                    spellCheck={false} style={{ ...area(), height: 64 }} />
                <textarea value={body} onChange={(e) => setBody(e.target.value)} placeholder="Request body"
                    spellCheck={false} style={{ ...area(), height: 48, marginTop: 6 }} />
                <label style={{ display: 'flex', gap: 6, alignItems: 'center', fontSize: 10, marginTop: 6, cursor: 'pointer' }}>
                    <input type="checkbox" checked={follow} onChange={(e) => setFollow(e.target.checked)} />
                    Follow redirects
                </label>
                {error && <div style={{ marginTop: 6, fontSize: 10, color: '#f87171' }}>{error}</div>}
            </div>

            <div style={{ flex: 1, overflowY: 'auto', minHeight: 0 }}>
                {!resp && <div style={{ padding: 16, fontSize: 11, opacity: 0.5 }}>Craft a request and Send. Response renders here.</div>}
                {resp && (
                    <div style={{ padding: '8px 12px' }}>
                        <div style={{ fontSize: 11, marginBottom: 8 }}>
                            <span style={{ color: statusColor(resp.status), fontWeight: 800 }}>
                                {resp.status || 'ERR'} {resp.status_text}
                            </span>
                            <span style={{ opacity: 0.6, marginLeft: 10 }}>{resp.body_bytes} bytes · {resp.duration_ms}ms</span>
                            {resp.truncated && <span style={{ color: '#eab308', marginLeft: 8 }}>truncated</span>}
                        </div>
                        <div style={{ fontSize: 10, fontFamily: 'monospace', opacity: 0.7, marginBottom: 8, whiteSpace: 'pre-wrap', wordBreak: 'break-all' }}>
                            {resp.headers.map(([k, v]) => `${k}: ${v}`).join('\n')}
                        </div>
                        <pre style={{ fontSize: 10, background: 'rgba(0,0,0,0.2)', padding: 8, borderRadius: 4, whiteSpace: 'pre-wrap', wordBreak: 'break-all' }}>
                            {resp.body}
                        </pre>
                    </div>
                )}
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
    width: '100%',
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

export default RepeaterPanel;
