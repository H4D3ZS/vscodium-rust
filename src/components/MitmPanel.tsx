import React, { useEffect, useRef } from 'react';
import { useStore } from '../store';

const MitmPanel: React.FC = () => {
    const status = useStore(state => state.mitmStatus);
    const logs = useStore(state => state.mitmLogs);
    const startMitm = useStore(state => state.startMitm);
    const stopMitm = useStore(state => state.stopMitm);
    const logEndRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        if (logEndRef.current) {
            logEndRef.current.scrollIntoView({ behavior: 'smooth' });
        }
    }, [logs]);

    const handleToggle = () => {
        if (status === 'running') {
            stopMitm();
        } else {
            startMitm();
        }
    };

    return (
        <div className="mitm-panel" style={{ display: 'flex', flexDirection: 'column', height: '100%', padding: '16px', boxSizing: 'border-box' }}>
            <div className="mitm-header" style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: '16px' }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                    <div style={{ 
                        width: '10px', 
                        height: '100%', 
                        minHeight: '10px',
                        borderRadius: '50%', 
                        background: status === 'running' ? '#4ade80' : status === 'error' ? '#f87171' : '#94a3b8',
                        boxShadow: status === 'running' ? '0 0 8px #4ade80' : 'none'
                    }}></div>
                    <span style={{ fontSize: '14px', fontWeight: 600, color: 'var(--vscode-editor-foreground)' }}>
                        MITM Proxy Server
                    </span>
                    <span style={{ fontSize: '11px', opacity: 0.6, marginLeft: '4px' }}>
                        (Port 8080)
                    </span>
                </div>
                <button 
                    onClick={handleToggle}
                    style={{
                        background: status === 'running' ? 'var(--vscode-button-secondaryBackground, #303031)' : 'var(--vscode-button-background)',
                        color: 'var(--vscode-button-foreground)',
                        border: 'none',
                        padding: '6px 12px',
                        borderRadius: '4px',
                        cursor: 'pointer',
                        fontSize: '12px',
                        fontWeight: 500,
                        display: 'flex',
                        alignItems: 'center',
                        gap: '6px'
                    }}
                >
                    <i className={`codicon codicon-${status === 'running' ? 'stop' : 'play'}`}></i>
                    {status === 'running' ? 'Stop Proxy' : 'Start Proxy'}
                </button>
            </div>

            <div className="mitm-logs" style={{ 
                flex: 1, 
                background: 'var(--vscode-editor-background)', 
                border: '1px solid var(--vscode-panel-border)', 
                borderRadius: '6px',
                padding: '12px',
                overflowY: 'auto',
                fontFamily: 'var(--font-mono, monospace)',
                fontSize: '12px',
                lineHeight: '1.6',
                color: 'var(--vscode-editor-foreground)',
                opacity: 0.9
            }}>
                {logs.length === 0 ? (
                    <div style={{ opacity: 0.4, fontStyle: 'italic', textAlign: 'center', marginTop: '20px' }}>
                        Waiting for proxy initialization...
                    </div>
                ) : (
                    logs.map((log, i) => (
                        <div key={i} style={{ marginBottom: '4px', borderBottom: '1px solid rgba(255,255,255,0.02)', paddingBottom: '2px' }}>
                            {log}
                        </div>
                    ))
                )}
                <div ref={logEndRef} />
            </div>

            <div style={{ marginTop: '12px', fontSize: '11px', opacity: 0.5, display: 'flex', alignItems: 'center', gap: '6px' }}>
                <i className="codicon codicon-info" style={{ fontSize: '12px' }}></i>
                Configure your target device or application to use 127.0.0.1:8080 as HTTP/HTTPS proxy.
            </div>
        </div>
    );
};

export default MitmPanel;
