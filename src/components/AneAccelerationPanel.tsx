import React, { useState, useEffect } from 'react';
import { invoke } from '../tauri_bridge';

interface AneStatus {
    available: boolean;
    m1_or_newer: boolean;
    model: string;
    inference_mode: string;
    estimated_speedup: number;
    tokens_per_sec_estimate: number;
}

const AneAccelerationPanel: React.FC = () => {
    const [status, setStatus] = useState<AneStatus | null>(null);
    const [loading, setLoading] = useState(true);
    const [initializing, setInitializing] = useState(false);
    const [diagnostics, setDiagnostics] = useState<any>(null);

    useEffect(() => {
        refreshStatus();
        const interval = setInterval(refreshStatus, 5000); // Poll every 5s
        return () => clearInterval(interval);
    }, []);

    const refreshStatus = async () => {
        try {
            const statusResult: AneStatus = await invoke('ane_get_status', {});
            setStatus(statusResult);

            const diagResult = await invoke('ane_diagnostics', {});
            setDiagnostics(diagResult);
        } catch (err) {
            console.error('Failed to fetch ANE status:', err);
        } finally {
            setLoading(false);
        }
    };

    const handleInitialize = async () => {
        setInitializing(true);
        try {
            await invoke('ane_init_inference', {});
            await refreshStatus();
        } catch (err) {
            console.error('Failed to initialize ANE:', err);
            alert(`ANE initialization failed: ${err}`);
        } finally {
            setInitializing(false);
        }
    };

    if (loading) {
        return <div style={{ padding: 16, opacity: 0.6 }}>Loading ANE status...</div>;
    }

    if (!status) {
        return <div style={{ padding: 16, opacity: 0.6 }}>Unable to detect ANE status</div>;
    }

    const statusColor = status.available ? '#9ece6a' : '#888888';
    const modeColor = status.inference_mode === 'ane_accelerated' ? '#9ece6a' : '#f7768e';

    return (
        <div style={{ maxWidth: 680 }}>
            <div style={{ fontSize: 14, fontWeight: 600, marginBottom: 12 }}>
                Apple Neural Engine Acceleration
            </div>

            <div className="settings-card">
                <div className="settings-card-title">Hardware Detection</div>

                <div style={{ display: 'flex', gap: 20, marginBottom: 20 }}>
                    <div>
                        <div style={{ fontSize: 11, opacity: 0.6, marginBottom: 4 }}>Chip Model</div>
                        <div style={{ fontSize: 13, fontWeight: 500, color: statusColor }}>
                            {status.model || 'Detecting...'}
                        </div>
                    </div>
                    <div>
                        <div style={{ fontSize: 11, opacity: 0.6, marginBottom: 4 }}>ANE Available</div>
                        <div style={{ fontSize: 13, fontWeight: 500, color: statusColor }}>
                            {status.available ? '✓ Yes' : '✗ No'}
                        </div>
                    </div>
                    <div>
                        <div style={{ fontSize: 11, opacity: 0.6, marginBottom: 4 }}>M1 or Newer</div>
                        <div style={{ fontSize: 13, fontWeight: 500, color: statusColor }}>
                            {status.m1_or_newer ? '✓ Yes' : '✗ No'}
                        </div>
                    </div>
                </div>
            </div>

            <div className="settings-card">
                <div className="settings-card-title">Inference Configuration</div>

                <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 20, marginBottom: 20 }}>
                    <div>
                        <div style={{ fontSize: 11, opacity: 0.6, marginBottom: 4 }}>Mode</div>
                        <div style={{ fontSize: 13, fontWeight: 500, color: modeColor }}>
                            {status.inference_mode === 'ane_accelerated' ? 'ANE Accelerated' : 'Ollama Fallback'}
                        </div>
                    </div>
                    <div>
                        <div style={{ fontSize: 11, opacity: 0.6, marginBottom: 4 }}>Est. Speedup</div>
                        <div style={{ fontSize: 13, fontWeight: 500 }}>
                            {status.estimated_speedup.toFixed(1)}x
                        </div>
                    </div>
                    <div>
                        <div style={{ fontSize: 11, opacity: 0.6, marginBottom: 4 }}>Tokens/sec (qwen3.5:2b)</div>
                        <div style={{ fontSize: 13, fontWeight: 500 }}>
                            ~{status.tokens_per_sec_estimate.toFixed(0)} tok/s
                        </div>
                    </div>
                </div>

                {diagnostics && (
                    <div style={{ fontSize: 10, opacity: 0.5, padding: 8, background: 'rgba(0,0,0,0.2)', borderRadius: 4, marginBottom: 12 }}>
                        <pre style={{ margin: 0, whiteSpace: 'pre-wrap', wordBreak: 'break-word' }}>
                            {JSON.stringify(diagnostics, null, 2)}
                        </pre>
                    </div>
                )}

                {status.available && status.inference_mode !== 'ane_accelerated' && (
                    <button
                        type="button"
                        className="settings-button success"
                        disabled={initializing}
                        onClick={handleInitialize}
                        style={{ width: '100%' }}
                    >
                        {initializing ? 'Initializing ANE...' : 'Enable ANE Acceleration'}
                    </button>
                )}

                {status.available && status.inference_mode === 'ane_accelerated' && (
                    <div style={{ fontSize: 11, color: '#9ece6a', padding: 8, background: 'rgba(158, 206, 106, 0.1)', borderRadius: 4 }}>
                        ✓ ANE is active and accelerating inference on qwen3.5:2b
                    </div>
                )}

                {!status.available && (
                    <div style={{ fontSize: 11, color: '#f7768e', padding: 8, background: 'rgba(247, 118, 142, 0.1)', borderRadius: 4 }}>
                        ✗ ANE is not available on this system. Using Ollama CPU inference.
                    </div>
                )}
            </div>

            <div className="settings-card">
                <div className="settings-card-title">Performance Notes</div>
                <p style={{ fontSize: 11, opacity: 0.7, margin: '0 0 8px 0', lineHeight: 1.5 }}>
                    • ANE acceleration is transparent — inference automatically uses ANE for matrix multiplications when available
                </p>
                <p style={{ fontSize: 11, opacity: 0.7, margin: '0 0 8px 0', lineHeight: 1.5 }}>
                    • For M1 Mac with 8GB RAM: ANE handles token generation at ~2-3x faster than CPU-only Ollama
                </p>
                <p style={{ fontSize: 11, opacity: 0.7, margin: 0, lineHeight: 1.5 }}>
                    • Power efficiency: ANE uses significantly less power than CPU or GPU for inference
                </p>
            </div>
        </div>
    );
};

export default AneAccelerationPanel;
