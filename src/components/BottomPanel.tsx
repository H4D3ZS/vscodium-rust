import React, { useEffect } from 'react';
import { useStore } from '../store';
import TerminalView from './terminal/TerminalView';
import Composer from './Composer';
import { listen } from '@tauri-apps/api/event';

const BottomPanel: React.FC = () => {
    const isOpen = useStore(state => state.isBottomPanelOpen);
    const activeTab = useStore(state => state.activePanelTab);
    const setActiveTab = useStore(state => state.setActivePanelTab);
    const toggleBottomPanel = useStore(state => state.toggleBottomPanel);

    const terminalGroups = useStore(state => state.terminalGroups);
    const activeTerminalGroupId = useStore(state => state.activeTerminalGroupId);
    const addTerminalGroup = useStore(state => state.addTerminalGroup);
    const splitTerminal = useStore(state => state.splitTerminal);

    useEffect(() => {
        const unlisten = listen<boolean>('toggle-terminal', (event) => {
            if (event.payload) {
                setActiveTab('TERMINAL');
            } else if (isOpen) {
                toggleBottomPanel();
            }
        });
        return () => { unlisten.then(f => f()); };
    }, [isOpen, setActiveTab, toggleBottomPanel]);

    const activeGroup = terminalGroups.find(g => g.id === activeTerminalGroupId);
    const agentTask = useStore(state => state.agentTask);

    if (!isOpen) return null;

    return (
        <div
            className="bottom-panel"
            style={{
                height: '100%',
                width: '100%',
                background: 'var(--vscode-panel-background)',
                borderTop: '1px solid var(--vscode-panel-border)',
                display: 'flex',
                flexDirection: 'column',
                zIndex: 10,
                position: 'relative',
                overflow: 'hidden'
            }}
        >
            {/* Header / Tabs */}
            <div className="panel-header" style={{
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'space-between',
                height: '35px',
                padding: '0 8px',
                borderBottom: '1px solid var(--vscode-panel-border)',
                background: 'var(--vscode-panel-background)',
                fontSize: '11px',
                textTransform: 'uppercase',
                letterSpacing: '0.05em'
            }}>
                <div className="panel-tabs" style={{ display: 'flex', gap: '2px', height: '100%', alignItems: 'center' }}>
                    {['Problems', 'Output', 'Debug Console', 'Terminal', 'Composer', 'Ports'].map(tab => (
                        <div
                            key={tab}
                            className={`panel-tab ${activeTab === tab.toUpperCase() ? 'active' : ''}`}
                            onClick={() => setActiveTab(tab.toUpperCase() as any)}
                        >
                            {tab}
                            {tab === 'Composer' && (
                                <span style={{
                                    background: 'var(--vscode-badge-background)',
                                    color: 'var(--vscode-badge-foreground)',
                                    padding: '0px 6px',
                                    borderRadius: '10px',
                                    fontSize: '9px',
                                    fontWeight: 700,
                                    height: '14px',
                                    display: 'flex',
                                    alignItems: 'center',
                                    justifyContent: 'center',
                                    marginLeft: '6px'
                                }}>0</span>
                            )}
                            {tab === 'Problems' && (
                                <span style={{
                                    background: 'var(--terminator-accent)',
                                    color: '#ffffff',
                                    padding: '0px 6px',
                                    borderRadius: '10px',
                                    fontSize: '9px',
                                    fontWeight: 700,
                                    height: '14px',
                                    display: 'flex',
                                    alignItems: 'center',
                                    justifyContent: 'center',
                                    marginLeft: '6px'
                                }}>0</span>
                            )}
                        </div>
                    ))}
                </div>

                <div className="panel-toolbar" style={{ display: 'flex', alignItems: 'center', gap: '8px', paddingRight: '12px' }}>
                    {activeTab === 'TERMINAL' && (
                        <div style={{ display: 'flex', alignItems: 'center', gap: '4px' }}>
                            <div className="toolbar-item" style={{
                                display: 'flex',
                                alignItems: 'center',
                                gap: '6px',
                                padding: '2px 8px',
                                borderRadius: '4px',
                                background: 'var(--vscode-badge-background, rgba(255,255,255,0.05))',
                                fontSize: '11px',
                                cursor: 'pointer',
                                color: 'var(--vscode-badge-foreground, #ccc)'
                            }}>
                                <i className="codicon codicon-terminal" style={{ fontSize: '13px', opacity: 0.7 }}></i>
                                <span>zsh</span>
                                <i className="codicon codicon-chevron-down" style={{ fontSize: '10px', opacity: 0.5 }}></i>
                            </div>
                            <div className="toolbar-icon" onClick={() => addTerminalGroup()} title="New Terminal"><i className="codicon codicon-add"></i></div>
                            <div className="toolbar-icon" onClick={() => activeGroup && splitTerminal(activeGroup.id, activeGroup.activeInstanceId)} title="Split Terminal"><i className="codicon codicon-split-horizontal"></i></div>
                            <div className="toolbar-icon" title="Kill Terminal"><i className="codicon codicon-trash"></i></div>
                        </div>
                    )}
                    <span style={{ height: '14px', width: '1px', background: 'var(--vscode-panel-border, rgba(255,255,255,0.1))', margin: '0 4px' }}></span>
                    <div className="toolbar-icon" title="Maximize Panel Size"><i className="codicon codicon-chevron-up"></i></div>
                    <div className="toolbar-icon" title="Close Panel" onClick={toggleBottomPanel}><i className="codicon codicon-close"></i></div>
                </div>
            </div>

            {/* Content Area */}
            <div className="panel-content" style={{ flex: 1, overflow: 'hidden', background: 'var(--vscode-terminal-background, var(--vscode-panel-background))' }}>
                {activeTab === 'TERMINAL' && <TerminalView />}
                {activeTab === 'COMPOSER' && <Composer />}
                {activeTab === 'OUTPUT' && (
                    <div style={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
                        <div style={{ padding: '8px 16px', borderBottom: '1px solid #333', display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                            <select style={{ background: 'transparent', color: '#ccc', border: 'none', fontSize: '11px', cursor: 'pointer', outline: 'none' }}>
                                <option>Main</option>
                                <option>Extension Host</option>
                                <option>Tasks</option>
                            </select>
                            <div style={{ display: 'flex', gap: '8px', color: '#666' }}>
                                <i className="codicon codicon-clear-all" style={{ fontSize: '12px', cursor: 'pointer' }}></i>
                                <i className="codicon codicon-lock" style={{ fontSize: '12px', cursor: 'pointer' }}></i>
                            </div>
                        </div>
                        <div style={{ padding: '12px 16px', color: '#888', fontFamily: 'var(--font-mono)', fontSize: '12px' }}>
                            [info] Initializing vscodium-rust...
                            <br />
                            [info] Backend connected via Tauri
                        </div>
                    </div>
                )}
                {activeTab === 'DEBUG CONSOLE' && (
                    <div style={{ height: '100%', display: 'flex', flexDirection: 'column', padding: '12px 16px' }}>
                        <div style={{ color: '#666', fontSize: '12px', fontStyle: 'italic', marginBottom: '8px' }}>
                            Debug Console is ready. No active debug session.
                        </div>
                        <div style={{ display: 'flex', alignItems: 'center', color: '#ccc', fontSize: '12px', marginTop: 'auto' }}>
                            <i className="codicon codicon-chevron-right" style={{ fontSize: '12px', marginRight: '8px', color: '#3794ef' }}></i>
                            <input
                                type="text"
                                placeholder="Filter or evaluate expression"
                                style={{ background: 'transparent', border: 'none', color: '#fff', fontSize: '12px', width: '100%', outline: 'none' }}
                            />
                        </div>
                    </div>
                )}
                {['PROBLEMS', 'PORTS'].includes(activeTab) && (
                    <div style={{ padding: '32px', color: '#666', fontSize: '12px', textAlign: 'center' }}>
                        {activeTab} view is currently empty.
                    </div>
                )}
            </div>
        </div>
    );
};

export default BottomPanel;
