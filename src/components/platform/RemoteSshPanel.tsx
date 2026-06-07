import React, { useState } from 'react';
import { invoke } from '../../tauri_bridge';

const RemoteSshPanel: React.FC = () => {
    const [host, setHost] = useState('');
    const [user, setUser] = useState('root');
    const [port, setPort] = useState('22');
    const [remotePath, setRemotePath] = useState('~');
    const [log, setLog] = useState('');
    const [busy, setBusy] = useState(false);

    const probe = async () => {
        setBusy(true);
        try {
            const r = await invoke<Record<string, unknown>>('remote_ssh_probe', {
                host, user, port: parseInt(port, 10) || 22,
            });
            setLog(JSON.stringify(r, null, 2));
        } catch (e) {
            setLog(String(e));
        } finally {
            setBusy(false);
        }
    };

    const listDir = async () => {
        setBusy(true);
        try {
            const r = await invoke<Record<string, unknown>>('remote_ssh_list_dir', {
                host, user, remotePath, port: parseInt(port, 10) || 22,
            });
            setLog(String(r.listing ?? JSON.stringify(r)));
        } catch (e) {
            setLog(String(e));
        } finally {
            setBusy(false);
        }
    };

    return (
        <div className="settings-card" style={{ maxWidth: 520 }}>
            <div style={{ fontWeight: 700, marginBottom: 8 }}>Remote SSH</div>
            <p className="afi-desc">Probe SSH connectivity and list a remote directory. Full Remote-SSH workspace mounting is planned.</p>
            <div style={{ display: 'grid', gap: 8, marginBottom: 10 }}>
                <input className="settings-input" placeholder="host" value={host} onChange={(e) => setHost(e.target.value)} />
                <input className="settings-input" placeholder="user" value={user} onChange={(e) => setUser(e.target.value)} />
                <input className="settings-input" placeholder="port" value={port} onChange={(e) => setPort(e.target.value)} />
                <input className="settings-input" placeholder="remote path" value={remotePath} onChange={(e) => setRemotePath(e.target.value)} />
            </div>
            <div style={{ display: 'flex', gap: 8, marginBottom: 10 }}>
                <button type="button" className="settings-button" disabled={busy || !host} onClick={() => void probe()}>Test connection</button>
                <button type="button" className="settings-button" disabled={busy || !host} onClick={() => void listDir()}>List directory</button>
            </div>
            {log && <pre style={{ fontSize: 10, padding: 8, background: 'rgba(0,0,0,0.2)', borderRadius: 6, maxHeight: 200, overflow: 'auto' }}>{log}</pre>}
        </div>
    );
};

export default RemoteSshPanel;
