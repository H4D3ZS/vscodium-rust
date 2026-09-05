import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface RetrievalStatus {
    running: boolean;
    port: number | null;
}

interface VfsStatus {
    running: boolean;
    port: number | null;
}

export function KortexPanel() {
    const [retrieval, setRetrieval] = useState<RetrievalStatus | null>(null);
    const [vfs, setVfs] = useState<VfsStatus | null>(null);
    const [busy, setBusy] = useState(false);
    const [error, setError] = useState('');

    const refreshStatus = useCallback(async () => {
        try {
            const [r, v] = await Promise.all([
                invoke<RetrievalStatus>('kortex_retrieval_status'),
                invoke<VfsStatus>('kortex_vfs_status'),
            ]);
            setRetrieval(r);
            setVfs(v);
        } catch { /* ignore */ }
    }, []);

    useEffect(() => {
        refreshStatus();
        const t = setInterval(refreshStatus, 5000);
        return () => clearInterval(t);
    }, [refreshStatus]);

    const stopRetrieval = useCallback(async () => {
        setBusy(true); setError('');
        try {
            await invoke('kortex_retrieval_stop');
            await refreshStatus();
        } catch (e) { setError(String(e)); }
        finally { setBusy(false); }
    }, [refreshStatus]);

    const stopVfs = useCallback(async () => {
        setBusy(true); setError('');
        try {
            await invoke('kortex_vfs_stop');
            await refreshStatus();
        } catch (e) { setError(String(e)); }
        finally { setBusy(false); }
    }, [refreshStatus]);

    const ghost: React.CSSProperties = {
        fontSize: 11, padding: '4px 10px', borderRadius: 3,
        background: 'transparent', color: 'var(--vscode-descriptionForeground)',
        border: '1px solid var(--vscode-panel-border)',
        cursor: busy ? 'wait' : 'pointer',
    };
    const badge = (on: boolean) => ({
        display: 'inline-block', width: 8, height: 8, borderRadius: '50%',
        background: on ? '#4ec9b0' : '#808080', marginRight: 6,
    });

    return (
        <div style={{ display: 'flex', flexDirection: 'column', padding: 8, gap: 8, color: 'var(--vscode-foreground)' }}>
            <div style={{ fontSize: 12, fontWeight: 600 }}>Kortex Services</div>

            {error && <div style={{ fontSize: 11, color: 'var(--vscode-errorForeground)' }}>{error}</div>}

            {/* Retrieval Proxy — auto-starts on boot */}
            <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                <span style={badge(!!retrieval?.running)} />
                <span style={{ fontSize: 11, flex: 1 }}>
                    Retrieval {retrieval?.running
                        ? `(:${retrieval.port})`
                        : '(starting\u2026)'}
                </span>
                {retrieval?.running && (
                    <button style={ghost} onClick={stopRetrieval} disabled={busy}>Stop</button>
                )}
            </div>
            <div style={{ fontSize: 10, color: 'var(--vscode-descriptionForeground)', marginTop: -4 }}>
                Auto-starts on boot. Augments AI prompts with .aim context.
            </div>

            {/* VFS Daemon — auto-starts on boot */}
            <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                <span style={badge(!!vfs?.running)} />
                <span style={{ fontSize: 11, flex: 1 }}>
                    VFS Daemon {vfs?.running
                        ? `(:${vfs.port})`
                        : '(starting\u2026)'}
                </span>
                {vfs?.running && (
                    <button style={ghost} onClick={stopVfs} disabled={busy}>Stop</button>
                )}
            </div>
            <div style={{ fontSize: 10, color: 'var(--vscode-descriptionForeground)', marginTop: -4 }}>
                Auto-starts on boot. Manages .aim memory and file watching.
            </div>
        </div>
    );
}
