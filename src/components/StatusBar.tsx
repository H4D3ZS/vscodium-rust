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

    const processStats = useStore(state => state.processStats);
    const memorySavings = useStore(state => state.memorySavings);
    const refreshProcessStats = useStore(state => state.refreshProcessStats);
    const refreshMemorySavings = useStore(state => state.refreshMemorySavings);

    const handleOptimize = async () => {
        try {
            await invoke('optimize_memory');
            console.log('App memory optimized');
            refreshMemorySavings();
        } catch (e) {
            console.error('Failed to optimize memory:', e);
        }
    };

    React.useEffect(() => {
        // Initial fetch
        refreshProcessStats();
        refreshMemorySavings();

        // Polling loop for Memory Guard and Stats
        const statsTimer = setInterval(() => {
            refreshProcessStats();

            // Memory Guard: If available RAM is less than 1GB, trigger optimization
            if (processStats && processStats.available_ram_gb > 0 && processStats.available_ram_gb < 1) {
                console.warn('Memory Guard: Low memory detected, triggering optimization.');
                handleOptimize();
            }
        }, 5000);

        return () => clearInterval(statsTimer);
    }, [processStats?.available_ram_gb]);

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
                    <i className="codicon codicon-remote" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '14px' }}></i>
                </div>
                <div className="status-item hoverable" style={{ cursor: 'pointer', display: 'flex', alignItems: 'center', height: '100%', padding: '0 6px' }}>
                    <i className="codicon codicon-layout-sidebar-left" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px', marginRight: '4px' }}></i>
                    vscodium-rust
                </div>
                <div className="status-item hoverable" onClick={() => setActiveSidebarView('scm-view')} style={{ cursor: 'pointer', display: 'flex', alignItems: 'center', height: '100%', padding: '0 6px' }}>
                    <i className="codicon codicon-source-control" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px', marginRight: '4px' }}></i>main*
                </div>
                <div className="status-item hoverable" style={{ cursor: 'pointer', display: 'flex', alignItems: 'center', height: '100%', padding: '0 6px', opacity: 0.8 }}>
                    <i className="codicon codicon-sync" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px' }}></i>
                </div>
                <div className="status-item hoverable" style={{ cursor: 'pointer', display: 'flex', alignItems: 'center', height: '100%', padding: '0 6px' }}>
                    <i className="codicon codicon-error" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px', marginRight: '2px' }}></i>0
                    <i className="codicon codicon-warning" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px', marginLeft: '6px', marginRight: '2px' }}></i>0
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
                        fontFamily: 'codicon',
                        fontStyle: 'normal',
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
                    <i className="codicon codicon-broadcast" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px', marginRight: '6px' }} />Discord RPC
                </div>
                <div className="status-item hoverable" style={{ cursor: 'pointer', height: '100%', display: 'flex', alignItems: 'center', padding: '0 8px' }}>Ln 1, Col 1</div>
                <div className="status-item hoverable" style={{ cursor: 'pointer', height: '100%', display: 'flex', alignItems: 'center', padding: '0 8px' }}>Spaces: 4</div>
                <div className="status-item hoverable" style={{ cursor: 'pointer', height: '100%', display: 'flex', alignItems: 'center', padding: '0 8px' }}>UTF-8</div>

                {processStats && (
                    <div
                        className="status-item hoverable"
                        title={`Total: ${processStats.total_ram_gb}GB | Available: ${processStats.available_ram_gb}GB`}
                        style={{
                            cursor: 'help',
                            height: '100%',
                            display: 'flex',
                            alignItems: 'center',
                            padding: '0 8px',
                            color: processStats.available_ram_gb < 1 ? '#f87171' : 'inherit'
                        }}
                    >
                        <i className="codicon codicon-pulse" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px', marginRight: '4px' }} />
                        {processStats.memory_mb.toFixed(0)}MB ({processStats.cpu_usage.toFixed(1)}%)
                    </div>
                )}

                {memorySavings && memorySavings.original > 0 && (
                    <div
                        className="status-item hoverable"
                        title={`LZ4 Compression: ${((1 - memorySavings.compressed / memorySavings.original) * 100).toFixed(1)}% savings`}
                        style={{
                            cursor: 'help',
                            height: '100%',
                            display: 'flex',
                            alignItems: 'center',
                            padding: '0 8px',
                            background: 'rgba(74, 222, 128, 0.1)',
                            color: '#4ade80'
                        }}
                    >
                        <i className="codicon codicon-file-zip" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px', marginRight: '4px' }} />
                        {((memorySavings.original - memorySavings.compressed) / 1024).toFixed(1)}KB Saved
                    </div>
                )}

                <div className="status-item hoverable" onClick={handleOptimize} style={{ cursor: 'pointer', height: '100%', display: 'flex', alignItems: 'center', padding: '0 8px' }}>
                    <i className="codicon codicon-dashboard" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px', marginRight: '4px' }} />Optimize
                </div>
            </div>
        </footer>
    );
};

export default StatusBar;
