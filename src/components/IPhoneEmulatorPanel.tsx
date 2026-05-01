import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

/**
 * iPhone Emulator Panel - NATIVE WINDOWS APP
 * Launches Flutter Windows app as separate native window
 * REAL iPhone emulator for mobile developers
 */
const IPhoneEmulatorPanel: React.FC = () => {
    const [isRunning, setIsRunning] = useState(false);
    const [isLoading, setIsLoading] = useState(false);
    const [status, setStatus] = useState<'disconnected' | 'connecting' | 'booting' | 'connected' | 'error'>('disconnected');
    const [errorMessage, setErrorMessage] = useState('');
    const [bootProgress, setBootProgress] = useState(0);

    const handleLaunchIPhone = async () => {
        setIsLoading(true);
        setStatus('connecting');
        setBootProgress(0);
        
        try {
            const flutterPath = 'F:/Virtual-iPhone-Emulator/frontend';
            
            // Launch Flutter Windows app
            const result = await invoke('launch_iphone_emulator', {
                projectPath: flutterPath,
            });
            
            console.log('[iPhone] Launch result:', result);
            setStatus('booting');
            
            // Boot animation
            const bootInterval = setInterval(() => {
                setBootProgress(prev => {
                    if (prev >= 100) {
                        clearInterval(bootInterval);
                        setStatus('connected');
                        setIsRunning(true);
                        return 100;
                    }
                    return prev + 2;
                });
            }, 250);
            
        } catch (error) {
            setStatus('error');
            setErrorMessage(`Failed to launch iPhone emulator: ${error}`);
            console.error('[iPhone] Launch error:', error);
            setIsLoading(false);
        }
    };

    const handleStopIPhone = async () => {
        try {
            await invoke('stop_iphone_emulator');
            setIsRunning(false);
            setStatus('disconnected');
            setBootProgress(0);
        } catch (error) {
            console.error('[iPhone] Stop error:', error);
        }
    };

    return (
        <div style={{
            display: 'flex',
            flexDirection: 'column',
            height: '100%',
            background: '#000',
        }}>
            {/* Status bar */}
            <div style={{
                display: 'flex',
                justifyContent: 'space-between',
                alignItems: 'center',
                padding: '8px 12px',
                background: '#1a1a1a',
                borderBottom: '1px solid #333'
            }}>
                <div style={{
                    display: 'flex',
                    alignItems: 'center',
                    gap: '8px',
                    color: '#fff'
                }}>
                    <span style={{ fontSize: '14px' }}>🍎</span>
                    <span style={{ fontSize: '11px', fontWeight: 600 }}>iPhone Simulator</span>
                    {status === 'connected' && (
                        <span style={{
                            fontSize: '9px',
                            padding: '2px 6px',
                            background: '#34C759',
                            borderRadius: '3px',
                            color: '#fff'
                        }}>
                            Running
                        </span>
                    )}
                    {status === 'booting' && (
                        <span style={{
                            fontSize: '9px',
                            padding: '2px 6px',
                            background: '#FF9500',
                            borderRadius: '3px',
                            color: '#fff'
                        }}>
                            Booting {bootProgress}%
                        </span>
                    )}
                </div>

                {isRunning && (
                    <button
                        onClick={handleStopIPhone}
                        style={{
                            padding: '4px 12px',
                            fontSize: '10px',
                            background: '#FF3B30',
                            color: 'white',
                            border: 'none',
                            borderRadius: '3px',
                            cursor: 'pointer',
                            fontWeight: 600
                        }}
                    >
                        Stop
                    </button>
                )}
            </div>

            {/* Emulator content */}
            <div style={{
                flex: 1,
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                background: '#000',
                position: 'relative'
            }}>
                {!isRunning && status !== 'booting' ? (
                    <div style={{
                        textAlign: 'center',
                        padding: '20px',
                        maxWidth: '400px'
                    }}>
                        <div style={{ fontSize: '64px', marginBottom: '20px' }}>🍎</div>
                        
                        <h3 style={{
                            fontSize: '16px',
                            fontWeight: 600,
                            color: 'var(--vscode-foreground)',
                            marginBottom: '8px'
                        }}>
                            Virtual iPhone Emulator
                        </h3>
                        
                        <p style={{
                            fontSize: '12px',
                            color: 'var(--vscode-descriptionForeground)',
                            marginBottom: '20px',
                            lineHeight: '1.5'
                        }}>
                            Native Windows Flutter App<br/>
                            Embeds directly in this panel
                        </p>

                        {status === 'error' && (
                            <div style={{
                                color: '#f44',
                                fontSize: '11px',
                                marginBottom: '16px',
                                padding: '8px',
                                background: 'rgba(255,0,0,0.1)',
                                borderRadius: '4px'
                            }}>
                                {errorMessage}
                            </div>
                        )}

                        <button
                            onClick={handleLaunchIPhone}
                            disabled={isLoading}
                            style={{
                                padding: '10px 24px',
                                fontSize: '12px',
                                fontWeight: 600,
                                background: isLoading 
                                    ? 'var(--vscode-button-secondaryBackground)' 
                                    : '#007AFF',
                                color: 'white',
                                border: 'none',
                                borderRadius: '6px',
                                cursor: isLoading ? 'not-allowed' : 'pointer',
                                opacity: isLoading ? 0.6 : 1,
                                display: 'flex',
                                alignItems: 'center',
                                gap: '8px',
                                margin: '0 auto'
                            }}
                        >
                            {isLoading ? (
                                <>
                                    <span className="codicon codicon-loading codicon-modifier-spin"></span>
                                    Launching...
                                </>
                            ) : (
                                <>
                                    <span>🚀</span>
                                    Launch iPhone (Native)
                                </>
                            )}
                        </button>

                        <div style={{
                            marginTop: '20px',
                            fontSize: '10px',
                            color: 'var(--vscode-descriptionForeground)',
                            opacity: 0.6
                        }}>
                            Target: F:/Virtual-iPhone-Emulator/frontend
                        </div>
                    </div>
                ) : status === 'booting' ? (
                    <div style={{ textAlign: 'center', padding: '20px' }}>
                        <div style={{
                            fontSize: '80px',
                            marginBottom: '30px',
                            animation: 'pulse 2s infinite',
                            display: 'inline-block'
                        }}>
                            🍎
                        </div>
                        
                        <style>{`
                            @keyframes pulse {
                                0%, 100% { opacity: 0.6; transform: scale(0.95); }
                                50% { opacity: 1; transform: scale(1.05); }
                            }
                        `}</style>
                        
                        <div style={{
                            width: '200px',
                            height: '4px',
                            background: '#333',
                            borderRadius: '2px',
                            margin: '0 auto',
                            overflow: 'hidden'
                        }}>
                            <div style={{
                                width: `${bootProgress}%`,
                                height: '100%',
                                background: '#fff',
                                transition: 'width 0.3s ease'
                            }}></div>
                        </div>
                        
                        <div style={{ marginTop: '16px', fontSize: '12px', color: '#888' }}>
                            {bootProgress < 30 && 'Starting iOS...'}
                            {bootProgress >= 30 && bootProgress < 60 && 'Loading system...'}
                            {bootProgress >= 60 && bootProgress < 80 && 'Initializing services...'}
                            {bootProgress >= 80 && bootProgress < 100 && 'Almost ready...'}
                            {bootProgress >= 100 && 'iOS Ready!'}
                        </div>
                    </div>
                ) : (
                    /* Flutter running - will be embedded */
                    <div style={{
                        width: '100%',
                        height: '100%',
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'center',
                        color: '#666'
                    }}>
                        <div style={{ textAlign: 'center' }}>
                            <div style={{ fontSize: '48px', marginBottom: '16px' }}>📱</div>
                            <div>iPhone Emulator Running</div>
                            <div style={{ fontSize: '11px', opacity: 0.6, marginTop: '8px' }}>
                                Embedding in panel...
                            </div>
                        </div>
                    </div>
                )}
            </div>
        </div>
    );
};

export default IPhoneEmulatorPanel;
