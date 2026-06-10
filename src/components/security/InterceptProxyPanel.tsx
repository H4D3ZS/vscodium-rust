import React, { useCallback, useEffect, useRef, useState } from 'react';
import { invoke } from '../../tauri_bridge';

type Flow = {
    id: number;
    timestamp_ms: number;
    scheme: string;
    method: string;
    host: string;
    url: string;
    intercepted: boolean;
    req_headers: [string, string][];
    req_body_preview: string;
    status: number;
    resp_headers: [string, string][];
    resp_body_preview: string;
    bytes_up: number;
    bytes_down: number;
    duration_ms: number;
};

type Status = { running: boolean; port: number; flow_count: number };

const statusColor = (code: number): string => {
    if (code >= 500) return '#ef4444';
    if (code >= 400) return '#f97316';
    if (code >= 300) return '#eab308';
    if (code >= 200) return '#22c55e';
    return '#94a3b8';
};

const InterceptProxyPanel: React.FC = () => {
    const [status, setStatus] = useState<Status>({ running: false, port: 8888, flow_count: 0 });
    const [port, setPort] = useState('8888');
    const [flows, setFlows] = useState<Flow[]>([]);
    const [selected, setSelected] = useState<Flow | null>(null);
    const [error, setError] = useState('');
    const [replay, setReplay] = useState<string>('');
    const pollRef = useRef<number | null>(null);

    const refresh = useCallback(async () => {
        try {
            const [s, f] = await Promise.all([
                invoke<Status>('proxy_status'),
                invoke<Flow[]>('proxy_flows', { limit: 200 }),
            ]);
            setStatus(s);
            setFlows(f);
        } catch (e) {
            setError(String(e));
        }
    }, []);

    useEffect(() => {
        void refresh();
        pollRef.current = window.setInterval(() => void refresh(), 1500);
        return () => {
            if (pollRef.current) window.clearInterval(pollRef.current);
        };
    }, [refresh]);

    const start = useCallback(async () => {
        setError('');
        try {
            const bound = await invoke<number>('proxy_start', { port: Number(port) || 8888 });
            setPort(String(bound));
            await refresh();
        } catch (e) {
            setError(String(e));
        }
    }, [port, refresh]);

    const stop = useCallback(async () => {
        try {
            await invoke<Status>('proxy_stop');
            await refresh();
        } catch (e) {
            setError(String(e));
        }
    }, [refresh]);

    const clear = useCallback(async () => {
        await invoke('proxy_clear');
        setSelected(null);
        await refresh();
    }, [refresh]);

    const doReplay = useCallback(async (id: number) => {
        setReplay('Replaying…');
        try {
            const r = await invoke<{ status: number; duration_ms: number }>('proxy_replay', { id });
            setReplay(`Replay → ${r.status} in ${r.duration_ms}ms`);
        } catch (e) {
            setReplay(`Replay failed: ${String(e)}`);
        }
    }, []);

    return (
        <div style={{ display: 'flex', flexDirection: 'column', height: '100%', minHeight: 0 }}>
            <div style={{ padding: '10px 12px', borderBottom: '1px solid var(--vscode-panel-border)', flexShrink: 0 }}>
                <div style={{ display: 'flex', gap: 6, alignItems: 'center', marginBottom: 8 }}>
                    <input
                        value={port}
                        onChange={(e) => setPort(e.target.value)}
                        disabled={status.running}
                        style={{
                            width: 70,
                            padding: '5px 8px',
                            fontSize: 11,
                            borderRadius: 4,
                            border: '1px solid var(--vscode-panel-border)',
                            background: 'var(--vscode-input-background)',
                            color: 'var(--vscode-input-foreground)',
                        }}
                    />
                    {status.running ? (
                        <button type="button" onClick={() => void stop()} style={btn('#b91c1c')}>
                            Stop
                        </button>
                    ) : (
                        <button type="button" onClick={() => void start()} style={btn('#0e639c')}>
                            Start Proxy
                        </button>
                    )}
                    <button type="button" onClick={() => void clear()} style={btn('transparent', true)}>
                        Clear
                    </button>
                    <span style={{ marginLeft: 'auto', fontSize: 10, opacity: 0.7 }}>
                        {status.running ? (
                            <span style={{ color: '#22c55e' }}>● live :{status.port}</span>
                        ) : (
                            <span style={{ opacity: 0.5 }}>○ stopped</span>
                        )}{' '}
                        · {status.flow_count} flows
                    </span>
                </div>
                <div style={{ fontSize: 10, opacity: 0.55, lineHeight: 1.45 }}>
                    Point your browser/app proxy at <code>127.0.0.1:{status.port || port}</code>. HTTP is fully captured;
                    HTTPS is tunneled (metadata only). Authorized testing only.
                </div>
                {error && <div style={{ marginTop: 6, fontSize: 10, color: '#f87171' }}>{error}</div>}
            </div>

            <div style={{ flex: 1, overflowY: 'auto', minHeight: 0 }}>
                {flows.length === 0 && (
                    <div style={{ padding: 16, fontSize: 11, opacity: 0.5 }}>
                        No flows captured yet. Start the proxy and route traffic through it.
                    </div>
                )}
                {flows.map((f) => (
                    <div
                        key={f.id}
                        onClick={() => setSelected(selected?.id === f.id ? null : f)}
                        style={{
                            padding: '6px 12px',
                            borderBottom: '1px solid var(--vscode-panel-border)',
                            cursor: 'pointer',
                            background: selected?.id === f.id ? 'var(--vscode-list-hoverBackground)' : 'transparent',
                            fontSize: 11,
                        }}
                    >
                        <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
                            <span style={{ fontWeight: 700, width: 50 }}>{f.method}</span>
                            <span style={{ color: statusColor(f.status), fontWeight: 700, width: 28 }}>
                                {f.intercepted ? f.status : '—'}
                            </span>
                            <span style={{ flex: 1, minWidth: 0, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                                {f.url}
                            </span>
                            <span style={{ fontSize: 9, opacity: 0.5 }}>{f.duration_ms}ms</span>
                        </div>
                        {selected?.id === f.id && (
                            <div style={{ marginTop: 6, fontSize: 10, fontFamily: 'monospace' }}>
                                <div style={{ opacity: 0.6, marginBottom: 4 }}>
                                    {f.scheme.toUpperCase()} · ↑{f.bytes_up}b ↓{f.bytes_down}b
                                </div>
                                {f.intercepted ? (
                                    <>
                                        <div style={{ background: 'rgba(0,0,0,0.2)', padding: 6, borderRadius: 4, marginBottom: 4, maxHeight: 120, overflow: 'auto', whiteSpace: 'pre-wrap', wordBreak: 'break-all' }}>
                                            {f.req_body_preview || '(no request body)'}
                                        </div>
                                        <div style={{ background: 'rgba(0,0,0,0.2)', padding: 6, borderRadius: 4, maxHeight: 160, overflow: 'auto', whiteSpace: 'pre-wrap', wordBreak: 'break-all' }}>
                                            {f.resp_body_preview || '(no response body)'}
                                        </div>
                                        <button
                                            type="button"
                                            onClick={(e) => { e.stopPropagation(); void doReplay(f.id); }}
                                            style={{ ...btn('#0e639c'), marginTop: 6, fontSize: 9 }}
                                        >
                                            Replay
                                        </button>
                                        {replay && <span style={{ marginLeft: 8, opacity: 0.7 }}>{replay}</span>}
                                    </>
                                ) : (
                                    <div style={{ opacity: 0.5 }}>HTTPS tunnel — payload encrypted, not replayable.</div>
                                )}
                            </div>
                        )}
                    </div>
                ))}
            </div>
        </div>
    );
};

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

export default InterceptProxyPanel;
