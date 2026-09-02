import React, { useCallback, useEffect, useRef, useState } from 'react';
import { invoke } from '../../tauri_bridge';

type Interaction = {
    id: number;
    token: string;
    protocol: string;
    timestamp_ms: number;
    remote_addr: string;
    method: string;
    path: string;
    host_header: string;
    user_agent: string;
    raw_head: string;
};

type Payload = { token: string; http_url: string; authority: string };
type Status = { running: boolean; port: number; public_host: string; interaction_count: number };

const OastPanel: React.FC = () => {
    const [status, setStatus] = useState<Status>({ running: false, port: 8889, public_host: '127.0.0.1', interaction_count: 0 });
    const [port, setPort] = useState('8889');
    const [publicHost, setPublicHost] = useState('');
    const [payloads, setPayloads] = useState<Payload[]>([]);
    const [interactions, setInteractions] = useState<Interaction[]>([]);
    const [expanded, setExpanded] = useState<number | null>(null);
    const [error, setError] = useState('');
    const [copied, setCopied] = useState('');
    const poll = useRef<number | null>(null);

    const refresh = useCallback(async () => {
        try {
            const [s, i] = await Promise.all([
                invoke<Status>('oast_status'),
                invoke<Interaction[]>('oast_poll', { token: null }),
            ]);
            setStatus(s);
            setInteractions(i);
            if (s.public_host && !publicHost) setPublicHost(s.public_host);
        } catch (e) {
            setError(String(e));
        }
    }, [publicHost]);

    useEffect(() => {
        void refresh();
        poll.current = window.setInterval(() => void refresh(), 2000);
        return () => {
            if (poll.current) window.clearInterval(poll.current);
        };
    }, [refresh]);

    const start = useCallback(async () => {
        setError('');
        try {
            await invoke<number>('oast_start', { port: Number(port) || 8889, publicHost: publicHost.trim() || null });
            await refresh();
        } catch (e) {
            setError(String(e));
        }
    }, [port, publicHost, refresh]);

    const stop = useCallback(async () => {
        await invoke('oast_stop');
        await refresh();
    }, [refresh]);

    const mint = useCallback(async () => {
        try {
            const p = await invoke<Payload>('oast_register');
            setPayloads((prev) => [p, ...prev].slice(0, 20));
        } catch (e) {
            setError(String(e));
        }
    }, []);

    const copy = useCallback(async (text: string) => {
        await navigator.clipboard.writeText(text);
        setCopied(text);
        window.setTimeout(() => setCopied(''), 1500);
    }, []);

    const clear = useCallback(async () => {
        await invoke('oast_clear');
        await refresh();
    }, [refresh]);

    return (
        <div style={{ display: 'flex', flexDirection: 'column', height: '100%', minHeight: 0 }}>
            <div style={{ padding: '10px 12px', borderBottom: '1px solid var(--vscode-panel-border)', flexShrink: 0 }}>
                <div style={{ display: 'flex', gap: 6, alignItems: 'center', marginBottom: 8, flexWrap: 'wrap' }}>
                    <input value={port} onChange={(e) => setPort(e.target.value)} disabled={status.running}
                        placeholder="port" style={inp(64)} />
                    <input value={publicHost} onChange={(e) => setPublicHost(e.target.value)}
                        placeholder="public host / LAN IP / collab domain" style={inp(220)} />
                    {status.running ? (
                        <button type="button" onClick={() => void stop()} style={btn('#b91c1c')}>Stop</button>
                    ) : (
                        <button type="button" onClick={() => void start()} style={btn('#0e639c')}>Start OAST</button>
                    )}
                    <span style={{ marginLeft: 'auto', fontSize: 10, opacity: 0.7 }}>
                        {status.running ? <span style={{ color: '#22c55e' }}>● live :{status.port}</span> : <span style={{ opacity: 0.5 }}>○ stopped</span>}
                        {' '}· {status.interaction_count} hits
                    </span>
                </div>
                <div style={{ display: 'flex', gap: 6 }}>
                    <button type="button" onClick={() => void mint()} disabled={!status.running} style={btn('#0e639c')}>
                        New Payload
                    </button>
                    <button type="button" onClick={() => void clear()} style={btn('transparent', true)}>Clear hits</button>
                </div>
                <div style={{ fontSize: 10, opacity: 0.55, lineHeight: 1.45, marginTop: 8 }}>
                    Mint a payload, inject the URL into the target (SSRF, RCE, XXE, blind XSS). A callback here
                    <b> confirms a blind vuln</b>. Set a reachable public host/LAN IP if the target isn&apos;t local.
                </div>
                {error && <div style={{ marginTop: 6, fontSize: 10, color: '#f87171' }}>{error}</div>}
            </div>

            {payloads.length > 0 && (
                <div style={{ padding: '6px 12px', borderBottom: '1px solid var(--vscode-panel-border)', flexShrink: 0, maxHeight: 120, overflowY: 'auto' }}>
                    {payloads.map((p) => {
                        const hits = interactions.filter((i) => i.token === p.token).length;
                        return (
                            <div key={p.token} style={{ display: 'flex', gap: 8, alignItems: 'center', fontSize: 10, padding: '3px 0' }}>
                                <span style={{ color: hits > 0 ? '#22c55e' : 'inherit', fontWeight: 700, width: 40 }}>
                                    {hits > 0 ? `✓ ${hits}` : '· 0'}
                                </span>
                                <code style={{ flex: 1, minWidth: 0, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{p.http_url}</code>
                                <button type="button" onClick={() => void copy(p.http_url)} style={btn('transparent', true)}>
                                    {copied === p.http_url ? 'Copied' : 'Copy'}
                                </button>
                            </div>
                        );
                    })}
                </div>
            )}

            <div style={{ flex: 1, overflowY: 'auto', minHeight: 0 }}>
                {interactions.length === 0 && (
                    <div style={{ padding: 16, fontSize: 11, opacity: 0.5 }}>
                        No callbacks yet. Interactions appear here the moment a target reaches your payload.
                    </div>
                )}
                {interactions.map((i) => (
                    <div key={i.id} onClick={() => setExpanded(expanded === i.id ? null : i.id)}
                        style={{ padding: '6px 12px', borderBottom: '1px solid var(--vscode-panel-border)', cursor: 'pointer', fontSize: 11, background: expanded === i.id ? 'var(--vscode-list-hoverBackground)' : 'transparent' }}>
                        <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
                            <span style={{ fontWeight: 700, color: '#22c55e', textTransform: 'uppercase', width: 36 }}>{i.protocol}</span>
                            <span style={{ fontWeight: 700, width: 44 }}>{i.method}</span>
                            <span style={{ flex: 1, minWidth: 0, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{i.path}</span>
                            <span style={{ fontSize: 9, opacity: 0.6 }}>{i.remote_addr}</span>
                        </div>
                        {expanded === i.id && (
                            <pre style={{ marginTop: 6, fontSize: 10, background: 'rgba(0,0,0,0.2)', padding: 6, borderRadius: 4, whiteSpace: 'pre-wrap', wordBreak: 'break-all', maxHeight: 200, overflow: 'auto' }}>
                                {i.raw_head}
                            </pre>
                        )}
                    </div>
                ))}
            </div>
        </div>
    );
};

const inp = (w: number): React.CSSProperties => ({
    width: w,
    padding: '5px 8px',
    fontSize: 11,
    borderRadius: 4,
    border: '1px solid var(--vscode-panel-border)',
    background: 'var(--vscode-input-background)',
    color: 'var(--vscode-input-foreground)',
});

const btn = (bg: string, ghost = false): React.CSSProperties => ({
    padding: '5px 12px',
    border: ghost ? '1px solid var(--vscode-panel-border)' : 'none',
    borderRadius: 4,
    cursor: 'pointer',
    fontSize: 11,
    fontWeight: 600,
    background: ghost ? 'transparent' : bg,
    color: ghost ? 'inherit' : '#fff',
});

export default OastPanel;
