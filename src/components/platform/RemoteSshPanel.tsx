import React, { useCallback, useEffect, useState } from 'react';
import { invoke } from '../../tauri_bridge';
import { useStore } from '../../store';

const RemoteSshPanel: React.FC = () => {
    const setActiveRoot = useStore((s) => s.setActiveRoot);
    const [host, setHost] = useState('');
    const [user, setUser] = useState('root');
    const [port, setPort] = useState('22');
    const [remotePath, setRemotePath] = useState('~');
    const [log, setLog] = useState('');
    const [busy, setBusy] = useState(false);
    const [status, setStatus] = useState<Record<string, unknown> | null>(null);

    const refreshStatus = useCallback(async () => {
        try {
            setStatus(await invoke<Record<string, unknown>>('remote_ssh_status'));
        } catch {
            setStatus(null);
        }
    }, []);

    useEffect(() => { void refreshStatus(); }, [refreshStatus]);

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

    const mount = async () => {
        setBusy(true);
        setLog('');
        try {
            const r = await invoke<{ local_path?: string; message?: string }>('remote_ssh_mount', {
                host, user, remotePath, port: parseInt(port, 10) || 22,
            });
            if (r.local_path) {
                await setActiveRoot(r.local_path);
                setLog(`Mounted → ${r.local_path}\n${r.message ?? ''}`);
            }
            await refreshStatus();
        } catch (e) {
            setLog(String(e));
        } finally {
            setBusy(false);
        }
    };

    const syncPull = async () => {
        setBusy(true);
        try {
            const r = await invoke<Record<string, unknown>>('remote_ssh_sync_pull');
            setLog(`Pulled remote changes → ${String(r.local_path ?? '')}`);
        } catch (e) {
            setLog(String(e));
        } finally {
            setBusy(false);
        }
    };

    const syncPush = async () => {
        setBusy(true);
        try {
            const r = await invoke<Record<string, unknown>>('remote_ssh_sync_push');
            setLog(`Pushed local changes → ${String(r.remote_path ?? '')}`);
        } catch (e) {
            setLog(String(e));
        } finally {
            setBusy(false);
        }
    };

    const disconnect = async () => {
        await invoke('remote_ssh_disconnect');
        await refreshStatus();
        setLog('Disconnected remote session.');
    };

    return (
        <div className="settings-card" style={{ maxWidth: 520 }}>
            <div style={{ fontWeight: 700, marginBottom: 8 }}>Remote SSH workspace</div>
            <p className="afi-desc">
                Mount a remote folder as a local mirror (rsync/scp), then open it as the workspace root.
                Sync pull/push keeps the mirror in step with the server.
            </p>
            {status?.mounted && (
                <p className="afi-subtle" style={{ fontSize: 11, marginBottom: 8 }}>
                    Active: {String(status.user)}@{String(status.host)}:{String(status.remote_path)}
                </p>
            )}
            <div style={{ display: 'grid', gap: 8, marginBottom: 10 }}>
                <input className="settings-input" placeholder="host" value={host} onChange={(e) => setHost(e.target.value)} />
                <input className="settings-input" placeholder="user" value={user} onChange={(e) => setUser(e.target.value)} />
                <input className="settings-input" placeholder="port" value={port} onChange={(e) => setPort(e.target.value)} />
                <input className="settings-input" placeholder="remote path" value={remotePath} onChange={(e) => setRemotePath(e.target.value)} />
            </div>
            <div style={{ display: 'flex', gap: 8, marginBottom: 10, flexWrap: 'wrap' }}>
                <button type="button" className="settings-button" disabled={busy || !host} onClick={() => void probe()}>Test</button>
                <button type="button" className="settings-button" disabled={busy || !host} onClick={() => void listDir()}>List</button>
                <button type="button" className="settings-button success" disabled={busy || !host} onClick={() => void mount()}>Mount & open</button>
                <button type="button" className="settings-button" disabled={busy || !status?.mounted} onClick={() => void syncPull()}>Pull</button>
                <button type="button" className="settings-button" disabled={busy || !status?.mounted} onClick={() => void syncPush()}>Push</button>
                <button type="button" className="settings-button" disabled={busy || !status?.mounted} onClick={() => void disconnect()}>Disconnect</button>
            </div>
            {log && <pre style={{ fontSize: 10, padding: 8, background: 'rgba(0,0,0,0.2)', borderRadius: 6, maxHeight: 200, overflow: 'auto' }}>{log}</pre>}
        </div>
    );
};

export default RemoteSshPanel;
