import React from 'react';
import { useStore } from '../store';
import { Globe, Shield, Lock, RefreshCw, ChevronLeft, ChevronRight, Search, Activity } from 'lucide-react';
import { invoke } from '@tauri-apps/api/core';

const BrowserSurface: React.FC = () => {
    const [url, setUrl] = React.useState('https://google.com');
    const [screenshot, setScreenshot] = React.useState<string | null>(null);
    const [isNavigating, setIsNavigating] = React.useState(false);
    const [isDesignMode, setIsDesignMode] = React.useState(false);
    const [selectedElement, setSelectedElement] = React.useState<{x: number, y: number} | null>(null);
    const isAgentThinking = useStore(state => state.isAgentThinking);
    const agentMode = useStore(state => state.agentMode);

    // Live Polling for Agent Vision
    React.useEffect(() => {
        let interval: any;
        if (isAgentThinking && agentMode === 'sentient') {
            /* 
            interval = setInterval(async () => {
                const b64 = await invoke<string>('browser_screenshot');
                if (b64) setScreenshot(`data:image/jpeg;base64,${b64}`);
            }, 2000); // Poll every 2s during autonomous work
            */
        }
        return () => clearInterval(interval);
    }, [isAgentThinking, agentMode]);

    const handleNavigate = async (newUrl: string) => {
        setIsNavigating(true);
        setUrl(newUrl);
        try {
            await invoke('browser_open'); // Ensure it's launched
            await invoke('browser_navigate', { url: newUrl });
            const b64 = await invoke<string>('browser_screenshot');
            if (b64) setScreenshot(`data:image/jpeg;base64,${b64}`);
        } catch (e) {
            console.error(e);
        } finally {
            setIsNavigating(false);
        }
    };

    return (
        <div className="browser-surface" style={{ display: 'flex', flex: 1, flexDirection: 'column', height: '100%', background: '#fff', color: '#000' }}>
            {/* Browser Navigation Bar */}
            <div className="browser-navbar" style={{
                height: '48px',
                background: 'var(--vscode-editor-background)',
                borderBottom: '1px solid var(--vscode-panel-border)',
                display: 'flex',
                alignItems: 'center',
                padding: '0 12px',
                gap: '8px',
                color: 'var(--vscode-foreground)'
            }}>
                <div style={{ display: 'flex', gap: '4px' }}>
                    <button style={navButtonStyle}><ChevronLeft size={16} /></button>
                    <button style={navButtonStyle}><ChevronRight size={16} /></button>
                    <button style={navButtonStyle} onClick={() => handleNavigate(url)} disabled={isNavigating}>
                        <RefreshCw size={16} className={isNavigating ? 'animate-spin' : ''} />
                    </button>
                </div>

                <div className="url-bar" style={{
                    flex: 1,
                    height: '28px',
                    background: 'var(--vscode-sideBar-background)',
                    borderRadius: '4px',
                    border: '1px solid var(--vscode-panel-border)',
                    display: 'flex',
                    alignItems: 'center',
                    padding: '0 10px',
                    fontSize: '12px',
                    gap: '8px'
                }}>
                    <Lock size={12} style={{ opacity: 0.5 }} />
                    <input
                        value={url}
                        onChange={(e) => setUrl(e.target.value)}
                        onKeyDown={(e) => e.key === 'Enter' && handleNavigate(url)}
                        style={{
                            background: 'transparent',
                            border: 'none',
                            color: 'inherit',
                            width: '100%',
                            outline: 'none',
                            fontSize: '11px'
                        }}
                    />
                    <Shield size={12} color="var(--termainator-success)" />
                </div>

                {isAgentThinking && (
                    <div style={{
                        padding: '4px 10px',
                        background: 'rgba(59, 130, 246, 0.1)',
                        borderRadius: '20px',
                        fontSize: '10px',
                        fontWeight: 700,
                        color: '#3b82f6',
                        display: 'flex',
                        alignItems: 'center',
                        gap: '6px'
                    }}>
                        <Activity size={10} className="animate-pulse" />
                        AGENT ACTUATING
                    </div>
                )}
                <button 
                    onClick={() => setIsDesignMode(!isDesignMode)}
                    style={{
                        ...navButtonStyle,
                        background: isDesignMode ? 'rgba(168, 85, 247, 0.2)' : 'transparent',
                        color: isDesignMode ? '#c084fc' : 'inherit',
                        border: isDesignMode ? '1px solid rgba(168, 85, 247, 0.5)' : 'none'
                    }}
                    title="Toggle Design Mode"
                >
                    <i className="codicon codicon-symbol-ruler" style={{ fontSize: '14px' }} />
                </button>
            </div>

            {/* Browser Content Area (Live Vision) */}
            <div className="browser-content" style={{
                flex: 1,
                display: 'flex',
                overflow: 'auto',
                background: '#f1f5f9',
                position: 'relative'
            }}>
                {screenshot ? (
                    <div style={{ position: 'relative', width: '100%', display: 'flex', justifyContent: 'center' }}>
                        <img
                            src={screenshot}
                            style={{
                                maxWidth: '100%',
                                height: 'auto',
                                objectFit: 'contain',
                                background: '#fff',
                                cursor: isDesignMode ? 'crosshair' : 'default'
                            }}
                            alt="Agent Vision"
                            onClick={(e) => {
                                if (isDesignMode) {
                                    const rect = e.currentTarget.getBoundingClientRect();
                                    setSelectedElement({ x: e.clientX - rect.left, y: e.clientY - rect.top });
                                }
                            }}
                        />
                        {isDesignMode && (
                            <div style={{
                                position: 'absolute',
                                top: 0, left: 0, right: 0, bottom: 0,
                                background: 'url("data:image/svg+xml,%3Csvg width=\'20\' height=\'20\' xmlns=\'http://www.w3.org/2000/svg\'%3E%3Cpath d=\'M 20 0 L 0 0 0 20\' fill=\'none\' stroke=\'rgba(168, 85, 247, 0.2)\' stroke-width=\'1\'/%3E%3C/svg%3E")',
                                pointerEvents: 'none'
                            }} />
                        )}
                        {isDesignMode && selectedElement && (
                            <div style={{
                                position: 'absolute',
                                left: selectedElement.x - 4,
                                top: selectedElement.y - 4,
                                width: '8px',
                                height: '8px',
                                background: '#c084fc',
                                borderRadius: '50%',
                                boxShadow: '0 0 0 4px rgba(168,85,247,0.3)'
                            }} />
                        )}
                        {isDesignMode && selectedElement && (
                            <div style={{
                                position: 'absolute',
                                left: Math.min(selectedElement.x + 12, 600),
                                top: selectedElement.y - 20,
                                background: 'var(--vscode-editor-background)',
                                border: '1px solid var(--vscode-widget-border)',
                                borderRadius: '6px',
                                padding: '8px 12px',
                                display: 'flex',
                                flexDirection: 'column',
                                gap: '8px',
                                boxShadow: '0 4px 12px rgba(0,0,0,0.5)',
                                zIndex: 10
                            }}>
                                <span style={{ fontSize: '11px', fontWeight: 600, color: 'var(--vscode-foreground)' }}>Element Selected</span>
                                <input 
                                    autoFocus
                                    placeholder="Rewrite, resize, or move..." 
                                    style={{
                                        background: 'rgba(0,0,0,0.2)',
                                        border: '1px solid rgba(255,255,255,0.1)',
                                        color: 'var(--vscode-editor-foreground, #fff)',
                                        padding: '4px 8px',
                                        borderRadius: '4px',
                                        fontSize: '11px',
                                        outline: 'none',
                                        width: '200px'
                                    }}
                                    onKeyDown={(e) => {
                                        if (e.key === 'Enter') {
                                            setSelectedElement(null);
                                            setIsDesignMode(false);
                                            // Real app would dispatch to agent here
                                        }
                                    }}
                                />
                                <div style={{ display: 'flex', alignItems: 'center', gap: '4px', fontSize: '9px', opacity: 0.6 }}>
                                    <i className="codicon codicon-sparkle" /> Ask Agent to Edit
                                </div>
                            </div>
                        )}
                    </div>
                ) : (
                    <div style={{ display: 'flex', flex: 1, alignItems: 'center', justifyContent: 'center' }}>
                        <div style={{ textAlign: 'center', maxWidth: '400px' }}>
                            <div style={{ fontSize: '48px', marginBottom: '16px' }}>🌐</div>
                            <h2 style={{ color: '#1e293b', marginBottom: '8px' }}>Neural Browser Bridge</h2>
                            <p style={{ color: '#64748b', fontSize: '13px', lineHeight: 1.5 }}>
                                Start an autonomous task or enter a URL to activate the Neural VFS Browser surface.
                            </p>
                        </div>
                    </div>
                )}
            </div>
        </div>
    );
};

const navButtonStyle: React.CSSProperties = {
    background: 'none',
    border: 'none',
    color: 'inherit',
    cursor: 'pointer',
    width: '28px',
    height: '28px',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    borderRadius: '4px',
    opacity: 0.7
};

export default BrowserSurface;
