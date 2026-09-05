import React, { useState, useEffect } from 'react';
import { useStore } from '../store';

/**
 * LemonadeHealthDashboard — compact health panel shown as a popup from the
 * StatusBar AI indicator. Shows backend status, loaded models, latency,
 * throughput, and provides quick actions.
 */
const LemonadeHealthDashboard: React.FC<{ onClose: () => void }> = ({ onClose }) => {
    const inferenceBackend = useStore(s => s.inferenceBackend);
    const ollamaStatus = useStore(s => s.ollamaStatus);
    const lemonadeStatus = useStore(s => s.lemonadeStatus);
    const llamaCppStatus = useStore(s => s.llamaCppStatus);
    const availableModels = useStore(s => s.availableModels);
    const refreshAvailableModels = useStore(s => s.refreshAvailableModels);
    const checkLemonadeStatus = useStore(s => s.checkLemonadeStatus);
    // Latency measured by the store's single health probe — no private poller.
    const latency = useStore(s => s.lemonadeLatencyMs);

    const [probing, setProbing] = useState(false);

    const backend = inferenceBackend || 'lemonade';
    const status = backend === 'lemonade' ? lemonadeStatus
        : backend === 'llama-cpp' ? llamaCppStatus
        : 'idle';
    const isConnected = status === 'running';
    const label = backend === 'lemonade' ? 'Lemonade'
        : backend === 'llama-cpp' ? 'llama.cpp'
        : backend;
    const url = backend === 'lemonade'
        ? (localStorage.getItem('provider.lemonade.url') || 'http://127.0.0.1:13305')
        : 'unknown';

    // One store-driven check when the popup opens (throttled inside the store).
    useEffect(() => {
        if (backend === 'lemonade') void checkLemonadeStatus();
    }, [backend, checkLemonadeStatus]);

    const handleRefresh = async () => {
        setProbing(true);
        try {
            await Promise.allSettled([
                refreshAvailableModels(),
                backend === 'lemonade' ? checkLemonadeStatus() : Promise.resolve(),
            ]);
        } finally {
            setProbing(false);
        }
    };

    return (
        <div style={{
            position: 'fixed',
            bottom: '30px',
            left: '50%',
            transform: 'translateX(-50%)',
            width: '380px',
            maxHeight: '450px',
            overflowY: 'auto',
            background: 'var(--vscode-menu-background, #1e1e1e)',
            border: '1px solid var(--vscode-menu-border, rgba(255,255,255,0.15))',
            borderRadius: '8px',
            boxShadow: '0 12px 40px rgba(0,0,0,0.6)',
            zIndex: 99999,
            padding: '16px',
            fontSize: '12px',
            color: 'var(--vscode-foreground, #e2e8f0)',
        }}>
            {/* Header */}
            <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: '12px' }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                    <div style={{
                        width: '10px', height: '10px', borderRadius: '50%',
                        background: isConnected ? '#10b981' : '#f43f5e',
                        boxShadow: isConnected ? '0 0 6px #10b981' : 'none',
                    }} />
                    <span style={{ fontWeight: 600, fontSize: '13px' }}>{label} Health</span>
                </div>
                <button onClick={onClose} style={{
                    background: 'none', border: 'none', color: 'inherit',
                    opacity: 0.5, cursor: 'pointer', fontSize: '16px', padding: '2px 6px',
                }}>×</button>
            </div>

            {/* Status */}
            <div style={{ marginBottom: '12px' }}>
                <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '4px' }}>
                    <span style={{ opacity: 0.6 }}>Status</span>
                    <span style={{ color: isConnected ? '#10b981' : '#f43f5e', fontWeight: 500 }}>
                        {isConnected ? 'Connected' : status === 'checking' ? 'Checking…' : 'Disconnected'}
                    </span>
                </div>
                <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '4px' }}>
                    <span style={{ opacity: 0.6 }}>Endpoint</span>
                    <span style={{ fontFamily: 'monospace', fontSize: '11px', opacity: 0.8 }}>{url}</span>
                </div>
                {latency !== null && (
                    <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '4px' }}>
                        <span style={{ opacity: 0.6 }}>Latency</span>
                        <span style={{ color: latency < 200 ? '#10b981' : latency < 500 ? '#f59e0b' : '#f43f5e' }}>
                            {latency}ms
                        </span>
                    </div>
                )}
            </div>

            {/* Models */}
            <div style={{ marginBottom: '12px' }}>
                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '6px' }}>
                    <span style={{ opacity: 0.6, fontSize: '11px', textTransform: 'uppercase', fontWeight: 600, letterSpacing: '0.05em' }}>
                        Available Models ({availableModels.length})
                    </span>
                    <button onClick={handleRefresh} disabled={probing} style={{
                        padding: '2px 8px', background: 'rgba(255,255,255,0.06)',
                        border: '1px solid rgba(255,255,255,0.1)', borderRadius: '4px',
                        color: 'inherit', fontSize: '10px', cursor: 'pointer', opacity: probing ? 0.5 : 1,
                    }}>
                        {probing ? 'Probing…' : 'Refresh'}
                    </button>
                </div>
                <div style={{ maxHeight: '150px', overflowY: 'auto' }}>
                    {availableModels.length === 0 ? (
                        <div style={{ opacity: 0.4, padding: '8px 0', textAlign: 'center' }}>
                            {isConnected ? 'No models loaded' : 'Backend not connected'}
                        </div>
                    ) : (
                        availableModels.map((m: any, i: number) => (
                            <div key={i} style={{
                                padding: '4px 8px', borderRadius: '4px',
                                background: i % 2 === 0 ? 'rgba(255,255,255,0.03)' : 'transparent',
                                display: 'flex', justifyContent: 'space-between', alignItems: 'center',
                            }}>
                                <span style={{ fontFamily: 'monospace', fontSize: '11px' }}>{m.id || m.name}</span>
                                <span style={{ opacity: 0.4, fontSize: '10px' }}>{m.provider || backend}</span>
                            </div>
                        ))
                    )}
                </div>
            </div>

            {/* Quick actions */}
            <div style={{ display: 'flex', gap: '6px' }}>
                <button onClick={() => window.dispatchEvent(new CustomEvent('open-settings', { detail: { section: 'inference-backend' } }))} style={{
                    flex: 1, padding: '6px', background: 'rgba(255,255,255,0.06)',
                    border: '1px solid rgba(255,255,255,0.1)', borderRadius: '4px',
                    color: 'inherit', fontSize: '11px', cursor: 'pointer',
                }}>
                    Settings
                </button>
                <button onClick={() => window.dispatchEvent(new CustomEvent('open-settings', { detail: { section: 'models' } }))} style={{
                    flex: 1, padding: '6px', background: 'rgba(255,255,255,0.06)',
                    border: '1px solid rgba(255,255,255,0.1)', borderRadius: '4px',
                    color: 'inherit', fontSize: '11px', cursor: 'pointer',
                }}>
                    Models
                </button>
            </div>
        </div>
    );
};

export default LemonadeHealthDashboard;
