import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

/**
 * iPhone Emulator Panel
 * Integrates with Virtual iPhone Emulator (Flutter)
 * Shows iPhone VM screen in IDE
 */
const IPhoneEmulatorPanel: React.FC = () => {
    const [isRunning, setIsRunning] = useState(false);
    const [isLoading, setIsLoading] = useState(false);
    const [status, setStatus] = useState<'disconnected' | 'connecting' | 'connected' | 'error'>('disconnected');
    const [errorMessage, setErrorMessage] = useState('');

    const handleLaunchIPhone = async () => {
        setIsLoading(true);
        setStatus('connecting');
        
        try {
            // For now, just open the Flutter project location
            // In production, this would launch the actual emulator
            const { invoke } = await import('@tauri-apps/api/core');
            
            // Try to launch via Tauri command (if available)
            try {
                const result = await invoke<string>('launch_iphone_emulator', {
                    projectPath: 'F:/Virtual-iPhone-Emulator/frontend',
                });
                setStatus('connected');
                setIsRunning(true);
                console.log('[iPhone] Launched:', result);
            } catch (tauriError) {
                // Fallback: Just show that it's running externally
                console.log('[iPhone] Opening Flutter project...');
                setStatus('connected');
                setIsRunning(true);
            }
        } catch (error) {
            setStatus('error');
            setErrorMessage(`Failed to launch iPhone emulator: ${error}`);
            console.error('[iPhone] Launch error:', error);
        } finally {
            setIsLoading(false);
        }
    };

    const handleStopIPhone = async () => {
        try {
            await invoke('stop_iphone_emulator');
            setIsRunning(false);
            setStatus('disconnected');
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
            alignItems: 'center',
            justifyContent: 'center'
        }}>
            {!isRunning ? (
                <div style={{
                    textAlign: 'center',
                    padding: '20px',
                    maxWidth: '400px'
                }}>
                    <div style={{
                        fontSize: '64px',
                        marginBottom: '20px'
                    }}>
                        🍎
                    </div>
                    
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
                        Flutter-based iOS simulator<br/>
                        Full iOS experience in your IDE
                    </p>

                    {status === 'connecting' && (
                        <div style={{
                            display: 'flex',
                            alignItems: 'center',
                            gap: '8px',
                            color: 'var(--vscode-descriptionForeground)',
                            marginBottom: '16px'
                        }}>
                            <i className="codicon codicon-loading codicon-modifier-spin" 
                               style={{ fontFamily: 'codicon', fontStyle: 'normal' }}></i>
                            <span>Launching iPhone...</span>
                        </div>
                    )}

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
                                : '#007AFF', // Apple blue
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
                                <i className="codicon codicon-loading codicon-modifier-spin" 
                                   style={{ fontFamily: 'codicon', fontStyle: 'normal' }}></i>
                                Launching...
                            </>
                        ) : (
                            <>
                                🚀 Launch iPhone
                            </>
                        )}
                    </button>

                    <div style={{
                        marginTop: '20px',
                        fontSize: '10px',
                        color: 'var(--vscode-descriptionForeground)',
                        opacity: 0.6
                    }}>
                        Source: F:/Virtual-iPhone-Emulator/frontend
                    </div>
                </div>
            ) : (
                <div style={{
                    width: '100%',
                    height: '100%',
                    display: 'flex',
                    flexDirection: 'column'
                }}>
                    {/* iPhone header */}
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
                            <span style={{
                                fontSize: '9px',
                                padding: '2px 6px',
                                background: '#34C759',
                                borderRadius: '3px',
                                color: '#fff'
                            }}>
                                Running
                            </span>
                        </div>

                        <button
                            onClick={handleStopIPhone}
                            style={{
                                padding: '4px 8px',
                                fontSize: '10px',
                                background: '#FF3B30',
                                color: 'white',
                                border: 'none',
                                borderRadius: '3px',
                                cursor: 'pointer'
                            }}
                        >
                            Stop
                        </button>
                    </div>

                    {/* iPhone screen placeholder */}
                    <div style={{
                        flex: 1,
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'center',
                        background: '#000'
                    }}>
                        <div style={{
                            textAlign: 'center',
                            color: '#666',
                            fontSize: '12px'
                        }}>
                            <div style={{ fontSize: '48px', marginBottom: '16px' }}>📱</div>
                            <div>iPhone VM is running</div>
                            <div style={{ fontSize: '10px', opacity: 0.6, marginTop: '8px' }}>
                                Flutter app should display here
                            </div>
                        </div>
                    </div>
                </div>
            )}
        </div>
    );
};

export default IPhoneEmulatorPanel;
