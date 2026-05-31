import React, { useEffect, useState } from 'react';
import { listen } from '@tauri-apps/api/event';
import { useStore } from '../store';

interface OllamaProgress {
    progress: number;
    tokens_per_sec: number;
    elapsed_secs: number;
    remaining_secs: number;
    status: 'loading' | 'generating' | 'complete';
    total_tokens?: number;
}

const OllamaProgressBar: React.FC = () => {
    const [progress, setProgress] = useState<OllamaProgress | null>(null);
    const [visible, setVisible] = useState(false);
    const [showDetails, setShowDetails] = useState(false);
    const [loadingStartTime, setLoadingStartTime] = useState<number | null>(null);

    useEffect(() => {
        let unlisten: (() => void) | undefined;

        listen<OllamaProgress>('ollama-progress', (event) => {
            const payload = event.payload;
            
            if (payload.status === 'generating' && loadingStartTime) {
                const loadTime = (Date.now() - loadingStartTime) / 1000;
                console.log('[Ollama] Model loaded in', loadTime.toFixed(1), 'seconds');
                setLoadingStartTime(null);
            }
            
            setProgress(payload);
            setVisible(true);
            
            if (payload.status === 'complete') {
                setTimeout(() => {
                    setVisible(false);
                    setShowDetails(false);
                }, 2000);
            }
        }).then(unsub => {
            unlisten = unsub;
        });

        return () => {
            if (unlisten) unlisten();
        };
    }, []);

    if (!visible || !progress) return null;

    const formatTime = (secs: number) => {
        if (secs < 0) return '...';
        if (secs < 60) return `${Math.round(secs)}s`;
        const mins = Math.floor(secs / 60);
        const remaining = Math.round(secs % 60);
        return `${mins}m${remaining}s`;
    };

    const getStatusColor = () => {
        if (progress.status === 'complete') return '#10b981';
        if (progress.status === 'loading') return '#f59e0b';
        return '#8b5cf6';
    };

    return (
        <div style={{
            position: 'fixed',
            bottom: '10px',
            right: '10px',
            minWidth: '200px',
            background: 'rgba(15, 23, 42, 0.9)',
            borderRadius: '8px',
            padding: showDetails ? '12px' : '8px 12px',
            boxShadow: '0 4px 16px rgba(0,0,0,0.3)',
            border: `1px solid ${getStatusColor()}`,
            zIndex: 10000,
            transition: 'all 0.3s ease',
            cursor: 'pointer',
        }}
        onClick={() => setShowDetails(!showDetails)}
        title="Click to show/hide details">
            {/* Minimal Bar */}
            <div style={{
                display: 'flex',
                alignItems: 'center',
                gap: '8px',
                marginBottom: showDetails ? '8px' : '0',
            }}>
                <span style={{
                    width: '6px',
                    height: '6px',
                    borderRadius: '50%',
                    background: getStatusColor(),
                    animation: progress.status !== 'complete' ? 'pulse 1.5s ease-in-out infinite' : 'none',
                }}></span>
                <div style={{
                    flex: 1,
                    height: '4px',
                    background: 'rgba(255,255,255,0.1)',
                    borderRadius: '2px',
                    overflow: 'hidden',
                }}>
                    <div style={{
                        width: progress.status === 'loading' ? '30%' : `${progress.progress}%`,
                        height: '100%',
                        background: getStatusColor(),
                        transition: 'width 0.3s ease',
                    }}></div>
                </div>
                <span style={{
                    fontSize: '10px',
                    color: '#94a3b8',
                    minWidth: '35px',
                }}>
                    {progress.status === 'loading' ? '...' : `${progress.progress}%`}
                </span>
            </div>

            {/* Expanded Details */}
            {showDetails && (
                <div style={{
                    display: 'grid',
                    gridTemplateColumns: 'repeat(3, 1fr)',
                    gap: '8px',
                    paddingTop: '8px',
                    borderTop: '1px solid rgba(255,255,255,0.1)',
                }}>
                    <div>
                        <div style={{ fontSize: '8px', color: '#64748b', textTransform: 'uppercase' }}>Speed</div>
                        <div style={{ fontSize: '11px', color: 'var(--vscode-editor-foreground, #fff)' }}>
                            {progress.status === 'loading' ? '...' : `${progress.tokens_per_sec} t/s`}
                        </div>
                    </div>
                    <div>
                        <div style={{ fontSize: '8px', color: '#64748b', textTransform: 'uppercase' }}>Elapsed</div>
                        <div style={{ fontSize: '11px', color: 'var(--vscode-editor-foreground, #fff)' }}>{formatTime(progress.elapsed_secs)}</div>
                    </div>
                    <div>
                        <div style={{ fontSize: '8px', color: '#64748b', textTransform: 'uppercase' }}>
                            {progress.status === 'loading' ? 'Loading' : progress.status === 'complete' ? 'Total' : 'Remaining'}
                        </div>
                        <div style={{ fontSize: '11px', color: progress.status === 'loading' ? '#f59e0b' : '#fff' }}>
                            {progress.status === 'loading' ? '...' : formatTime(progress.remaining_secs)}
                        </div>
                    </div>
                </div>
            )}

            <style>{`
                @keyframes pulse {
                    0%, 100% { opacity: 1; transform: scale(1); }
                    50% { opacity: 0.5; transform: scale(1.2); }
                }
            `}</style>
        </div>
    );
};

export default OllamaProgressBar;
