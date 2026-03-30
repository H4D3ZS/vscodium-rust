import React from 'react';
import { useStore } from '../store';
import { invoke } from '@tauri-apps/api/core';

const StatusBar: React.FC = () => {
    const theme = useStore(state => state.theme);
    const setTheme = useStore(state => state.setTheme);
    const setActiveSidebarView = useStore(state => state.setActiveSidebarView);
    const toggleBottomPanel = useStore(state => state.toggleBottomPanel);
    const agentModel = useStore(state => state.agentModel);
    const ollamaStatus = useStore(state => state.ollamaStatus);
    const toggleRightSidebar = useStore(state => state.toggleRightSidebar);

    const toggleTheme = () => {
        const themes = ['vs', 'vs-dark', 'hc-black'];
        const next = themes[(themes.indexOf(theme) + 1) % themes.length];
        setTheme(next);
    };

    const handleOptimize = async () => {
        try {
            await invoke('optimize_memory');
            console.log('App memory optimized');
        } catch (e) {
            console.error('Failed to optimize memory:', e);
        }
    };

    React.useEffect(() => {
        // Auto-optimize memory every 5 minutes
        const timer = setInterval(handleOptimize, 5 * 60 * 1000);
        return () => clearInterval(timer);
    }, []);

    return (
        <footer className="status-bar" style={{
            backgroundColor: 'var(--vscode-statusBar-background, #007acc)',
            color: 'var(--vscode-statusBar-foreground, #ffffff)',
            height: '22px',
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center',
            fontSize: '12px',
            fontFamily: 'var(--font-ui)',
            userSelect: 'none',
            zIndex: 1000
        }}>
            <div className="status-left" style={{ display: 'flex', alignItems: 'center', height: '100%' }}>
                <div style={{
                    background: 'var(--vscode-statusBarItem-remoteBackground, #16825d)',
                    height: '100%',
                    display: 'flex',
                    alignItems: 'center',
                    padding: '0 8px',
                    marginRight: '8px'
                }}>
                    <i className="codicon codicon-remote" style={{ fontSize: '14px' }}></i>
                </div>
                <div className="status-item hoverable" style={{ cursor: 'pointer', display: 'flex', alignItems: 'center', height: '100%', padding: '0 6px' }}>
                    <i className="codicon codicon-layout-sidebar-left" style={{ fontSize: '12px', marginRight: '4px' }}></i>
                    vscodium-rust
                </div>
                <div className="status-item hoverable" onClick={() => setActiveSidebarView('scm-view')} style={{ cursor: 'pointer', display: 'flex', alignItems: 'center', height: '100%', padding: '0 6px' }}>
                    <i className="codicon codicon-source-control" style={{ fontSize: '12px', marginRight: '4px' }}></i>main*
                </div>
                <div className="status-item hoverable" style={{ cursor: 'pointer', display: 'flex', alignItems: 'center', height: '100%', padding: '0 6px', opacity: 0.8 }}>
                    <i className="codicon codicon-sync" style={{ fontSize: '12px' }}></i>
                </div>
                <div className="status-item hoverable" style={{ cursor: 'pointer', display: 'flex', alignItems: 'center', height: '100%', padding: '0 6px' }}>
                    <i className="codicon codicon-error" style={{ fontSize: '12px', marginRight: '2px' }}></i>0
                    <i className="codicon codicon-warning" style={{ fontSize: '12px', marginLeft: '6px', marginRight: '2px' }}></i>0
                </div>
                <div
                    className="status-item hoverable"
                    onClick={() => toggleRightSidebar()}
                    style={{
                        cursor: 'pointer',
                        display: 'flex',
                        alignItems: 'center',
                        height: '100%',
                        padding: '0 8px',
                        marginLeft: '4px',
                        borderLeft: '1px solid rgba(255,255,255,0.1)',
                        gap: '6px'
                    }}
                >
                    <i className={`codicon codicon-sparkle`} style={{
                        fontSize: '12px',
                        color: useStore.getState().isAgentThinking ? '#4ade80' : 'rgba(255,255,255,0.7)',
                        animation: useStore.getState().isAgentThinking ? 'spin 2s linear infinite' : 'none'
                    }}></i>
                    <span style={{ fontSize: '11px', opacity: 0.9 }}>
                        {agentModel.split('|').pop()?.split(':')[0].toUpperCase() || 'AGENT'}
                    </span>
                    {agentModel.toLowerCase().includes('ollama') && (
                        <div
                            title={ollamaStatus === 'running' ? 'Ollama: Connected' : 'Ollama: Not Connected'}
                            style={{
                                width: '6px',
                                height: '6px',
                                borderRadius: '50%',
                                background: ollamaStatus === 'running' ? '#10b981' : '#f43f5e',
                                boxShadow: ollamaStatus === 'running' ? '0 0 4px #10b981' : 'none'
                            }}
                        ></div>
                    )}
                </div>
            </div>
            <div className="status-right" style={{ display: 'flex', alignItems: 'center', height: '100%' }}>
                <div className="status-item hoverable" style={{ cursor: 'pointer', height: '100%', display: 'flex', alignItems: 'center', padding: '0 8px', opacity: 0.9 }}>
                    <i className="codicon codicon-broadcast" style={{ fontSize: '12px', marginRight: '6px' }} />Discord RPC
                </div>
                <div className="status-item hoverable" style={{ cursor: 'pointer', height: '100%', display: 'flex', alignItems: 'center', padding: '0 8px' }}>Ln 1, Col 1</div>
                <div className="status-item hoverable" style={{ cursor: 'pointer', height: '100%', display: 'flex', alignItems: 'center', padding: '0 8px' }}>Spaces: 4</div>
                <div className="status-item hoverable" style={{ cursor: 'pointer', height: '100%', display: 'flex', alignItems: 'center', padding: '0 8px' }}>UTF-8</div>
                <div className="status-item hoverable" onClick={handleOptimize} style={{ cursor: 'pointer', height: '100%', display: 'flex', alignItems: 'center', padding: '0 8px' }}>
                    <i className="codicon codicon-dashboard" style={{ fontSize: '12px', marginRight: '4px' }} />Optimize
                </div>
            </div>
        </footer>
    );
};

export default StatusBar;
