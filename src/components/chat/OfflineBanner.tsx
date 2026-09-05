import React, { useState, useEffect } from 'react';
import { agentResilience, type BackendStatus } from '../../lib/agentResilience';

/**
 * OfflineBanner — persistent status bar shown at the top of the chat panel
 * when the inference backend is unreachable. Auto-dismisses when connection
 * is restored.
 */
const OfflineBanner: React.FC = () => {
    const [status, setStatus] = useState<BackendStatus>(agentResilience.getStatus());
    const [queuedCount, setQueuedCount] = useState(agentResilience.getQueuedCount());
    const [dismissed, setDismissed] = useState(false);

    useEffect(() => {
        const unsub = agentResilience.onStatusChange((s, q) => {
            setStatus(s);
            setQueuedCount(q);
            if (s === 'connected') setDismissed(false);
        });
        return unsub;
    }, []);

    // Auto-dismiss is OFF — user must explicitly dismiss or it stays visible
    // while the backend is down. Re-appears when status changes.

    if (status === 'connected' || dismissed) return null;

    const backend = localStorage.getItem('inferenceBackend') || 'lemonade';
    const backendLabel = backend === 'lemonade' ? 'Lemonade'
        : backend === 'llama-cpp' ? 'llama.cpp'
        : backend;
    const url = backend === 'lemonade'
        ? (localStorage.getItem('provider.lemonade.url') || 'http://127.0.0.1:13305')
        : '';

    return (
        <div style={{
            display: 'flex',
            alignItems: 'center',
            gap: '8px',
            padding: '8px 12px',
            background: status === 'reconnecting'
                ? 'rgba(251, 191, 36, 0.12)'
                : 'rgba(239, 68, 68, 0.12)',
            borderBottom: '1px solid',
            borderColor: status === 'reconnecting'
                ? 'rgba(251, 191, 36, 0.3)'
                : 'rgba(239, 68, 68, 0.3)',
            fontSize: '12px',
            color: 'var(--vscode-foreground, #e2e8f0)',
            flexShrink: 0,
        }}>
            {/* Status dot */}
            <div style={{
                width: '8px',
                height: '8px',
                borderRadius: '50%',
                background: status === 'reconnecting' ? '#f59e0b' : '#ef4444',
                animation: status === 'reconnecting' ? 'pulse 2s ease-in-out infinite' : 'none',
                flexShrink: 0,
            }} />

            {/* Message */}
            <div style={{ flex: 1, minWidth: 0 }}>
                {status === 'reconnecting' ? (
                    <span>
                        <strong>{backendLabel}</strong> unreachable — reconnecting…
                        {url && <span style={{ opacity: 0.6 }}> ({url})</span>}
                    </span>
                ) : (
                    <span>
                        <strong>{backendLabel}</strong> offline — start the server to use AI features
                        {url && <span style={{ opacity: 0.6 }}> ({url})</span>}
                    </span>
                )}
                {queuedCount > 0 && (
                    <span style={{ marginLeft: '8px', opacity: 0.7 }}>
                        {queuedCount} message{queuedCount !== 1 ? 's' : ''} queued
                    </span>
                )}
            </div>

            {/* Retry button */}
            <button
                onClick={() => agentResilience.forceReconnect()}
                style={{
                    padding: '4px 10px',
                    background: 'rgba(255,255,255,0.1)',
                    border: '1px solid rgba(255,255,255,0.15)',
                    borderRadius: '4px',
                    color: 'inherit',
                    fontSize: '11px',
                    cursor: 'pointer',
                    whiteSpace: 'nowrap',
                }}
            >
                Retry
            </button>

            {/* Dismiss */}
            <button
                onClick={() => setDismissed(true)}
                style={{
                    padding: '4px',
                    background: 'none',
                    border: 'none',
                    color: 'inherit',
                    opacity: 0.5,
                    cursor: 'pointer',
                    fontSize: '14px',
                    lineHeight: 1,
                }}
                title="Dismiss"
            >
                ×
            </button>

            <style>{`
                @keyframes pulse {
                    0%, 100% { opacity: 1; }
                    50% { opacity: 0.4; }
                }
            `}</style>
        </div>
    );
};

export default OfflineBanner;
