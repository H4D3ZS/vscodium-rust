import React, { useCallback, useEffect, useState } from 'react';
import { invoke } from '../../tauri_bridge';

interface PortInfo {
    listening: number[];
    forwarded: { local_port: number; label: string; protocol: string }[];
}

const PortsPanel: React.FC = () => {
    const [data, setData] = useState<PortInfo>({ listening: [], forwarded: [] });
    const [busy, setBusy] = useState(false);
    const [customPort, setCustomPort] = useState('');

    const refresh = useCallback(async () => {
        setBusy(true);
        try {
            const res = await invoke<PortInfo>('list_listening_ports');
            setData(res);
        } catch {
            setData({ listening: [], forwarded: [] });
        } finally {
            setBusy(false);
        }
    }, []);

    useEffect(() => { void refresh(); }, [refresh]);

    const pinPort = async (port: number) => {
        await invoke('port_forward_add', { localPort: port, label: `localhost:${port}` });
        await refresh();
    };

    const unpin = async (port: number) => {
        await invoke('port_forward_remove', { localPort: port });
        await refresh();
    };

    const addCustom = async () => {
        const p = parseInt(customPort, 10);
        if (!p || p < 1 || p > 65535) return;
        await pinPort(p);
        setCustomPort('');
    };

    const openInBrowser = (port: number) => {
        window.open(`http://localhost:${port}`, '_blank', 'noopener');
    };

    return (
        <div style={{ padding: 8, fontSize: 12, height: '100%', overflow: 'auto' }}>
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 8 }}>
                <span style={{ fontWeight: 600, fontSize: 11, textTransform: 'uppercase', opacity: 0.7 }}>Ports</span>
                <button type="button" onClick={() => void refresh()} disabled={busy} style={btnStyle} title="Refresh">
                    <i className="codicon codicon-refresh" style={iconStyle} />
                </button>
            </div>

            <div style={{ display: 'flex', gap: 6, marginBottom: 12 }}>
                <input
                    value={customPort}
                    onChange={e => setCustomPort(e.target.value)}
                    placeholder="Port number"
                    style={inputStyle}
                    onKeyDown={e => e.key === 'Enter' && void addCustom()}
                />
                <button type="button" onClick={() => void addCustom()} style={btnStyle}>Forward</button>
            </div>

            {data.forwarded.length > 0 && (
                <>
                    <div style={sectionTitle}>Forwarded</div>
                    {data.forwarded.map(p => (
                        <div key={`f-${p.local_port}`} style={rowStyle}>
                            <span>{p.label}</span>
                            <span style={{ display: 'flex', gap: 6 }}>
                                <button type="button" onClick={() => openInBrowser(p.local_port)} style={linkBtn}>Open</button>
                                <button type="button" onClick={() => void unpin(p.local_port)} style={linkBtn}>Remove</button>
                            </span>
                        </div>
                    ))}
                </>
            )}

            <div style={sectionTitle}>Listening (local)</div>
            {data.listening.length === 0 ? (
                <div style={{ opacity: 0.5, padding: '8px 0' }}>No listening ports detected.</div>
            ) : (
                data.listening.map(port => (
                    <div key={port} style={rowStyle}>
                        <span>localhost:{port}</span>
                        <span style={{ display: 'flex', gap: 6 }}>
                            <button type="button" onClick={() => openInBrowser(port)} style={linkBtn}>Open</button>
                            <button type="button" onClick={() => void pinPort(port)} style={linkBtn}>Forward</button>
                        </span>
                    </div>
                ))
            )}
        </div>
    );
};

const sectionTitle: React.CSSProperties = {
    fontSize: 10,
    fontWeight: 700,
    textTransform: 'uppercase',
    opacity: 0.5,
    margin: '8px 0 4px',
};

const rowStyle: React.CSSProperties = {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    padding: '4px 0',
    borderBottom: '1px solid var(--vscode-panel-border)',
};

const btnStyle: React.CSSProperties = {
    background: 'var(--vscode-button-secondaryBackground, transparent)',
    border: '1px solid var(--vscode-panel-border)',
    color: 'var(--vscode-foreground)',
    borderRadius: 3,
    padding: '2px 8px',
    cursor: 'pointer',
    fontSize: 11,
};

const linkBtn: React.CSSProperties = { ...btnStyle, border: 'none', padding: '2px 4px', opacity: 0.8 };

const inputStyle: React.CSSProperties = {
    flex: 1,
    background: 'var(--vscode-input-background)',
    border: '1px solid var(--vscode-input-border)',
    color: 'var(--vscode-input-foreground)',
    padding: '4px 8px',
    fontSize: 11,
    borderRadius: 3,
};

const iconStyle: React.CSSProperties = { fontFamily: 'codicon', fontStyle: 'normal', fontSize: 12 };

export default PortsPanel;
