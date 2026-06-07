import React, { useCallback, useEffect, useState } from 'react';
import { invoke } from '../../tauri_bridge';

const HermesGatewayPanel: React.FC = () => {
    const [status, setStatus] = useState<Record<string, unknown> | null>(null);
    const [busy, setBusy] = useState(false);
    const [msg, setMsg] = useState('');

    const refresh = useCallback(async () => {
        try {
            setStatus(await invoke<Record<string, unknown>>('hermes_gateway_status'));
        } catch {
            setStatus(null);
        }
    }, []);

    useEffect(() => { void refresh(); }, [refresh]);

    const start = async () => {
        setBusy(true);
        setMsg('');
        try {
            const port = await invoke<number>('hermes_gateway_start', { port: 8642 });
            setMsg(`Gateway listening on http://127.0.0.1:${port}/v1/chat/completions`);
            await refresh();
        } catch (e) {
            setMsg(String(e));
        } finally {
            setBusy(false);
        }
    };

    const stop = async () => {
        setBusy(true);
        try {
            await invoke('hermes_gateway_stop');
            setMsg('Gateway stopped.');
            await refresh();
        } catch (e) {
            setMsg(String(e));
        } finally {
            setBusy(false);
        }
    };

    return (
        <div className="settings-card" style={{ maxWidth: 520 }}>
            <div style={{ fontWeight: 700, marginBottom: 8 }}>Hermes gateway (:8642)</div>
            <p className="afi-desc">
                OpenAI-compatible local API for external tools, cron hooks, and Telegram/Discord bridges.
                Routes chat to the in-process Sentient agent.
            </p>
            <div style={{ display: 'flex', gap: 8, marginBottom: 10 }}>
                <button type="button" className="settings-button success" disabled={busy || !!status?.running} onClick={() => void start()}>
                    Start gateway
                </button>
                <button type="button" className="settings-button" disabled={busy || !status?.running} onClick={() => void stop()}>
                    Stop
                </button>
            </div>
            {status && (
                <p className="afi-subtle" style={{ fontSize: 11 }}>
                    Status: {status.running ? 'running' : 'stopped'} · port {String(status.port ?? 8642)}
                </p>
            )}
            {msg && <p className="afi-subtle" style={{ fontSize: 11, marginTop: 8 }}>{msg}</p>}
        </div>
    );
};

export default HermesGatewayPanel;
