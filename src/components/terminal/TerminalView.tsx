import React from 'react';
import TerminalSidebar from './TerminalSidebar';
import TerminalGroupView from './TerminalGroupView';
import { useStore } from '../../store';
import { terminalManager } from '../../terminal';

const TerminalView: React.FC = () => {
    const groups = useStore(state => state.terminalGroups);
    const activeGroupId = useStore(state => state.activeTerminalGroupId);
    const addTerminalGroup = useStore(state => state.addTerminalGroup);
    const addAiriActivityTerminal = useStore(state => state.addAiriActivityTerminal);
    const closeTerminalGroup = useStore(state => state.closeTerminalGroup);
    const toggleBottomPanel = useStore(state => state.toggleBottomPanel);

    const [availableShells, setAvailableShells] = React.useState<string[]>([]);
    const [selectedShell, setSelectedShell] = React.useState<string | undefined>(undefined);

    React.useEffect(() => {
        terminalManager.getAvailableShells().then(setAvailableShells);
    }, []);

    const handleAddTerminal = () => {
        addTerminalGroup(selectedShell);
    };

    const handleAddAiri = () => {
        addAiriActivityTerminal();
    };

    const handleKillTerminal = () => {
        if (activeGroupId) {
            closeTerminalGroup(activeGroupId);
        }
    };

    const activeGroup = groups.find(g => g.id === activeGroupId);

    return (
        <div
            className="terminal-view-host"
            style={{
                display: 'flex',
                flexDirection: 'column',
                width: '100%',
                height: '100%',
                background: 'var(--vscode-panel-background, #1e1e1e)',
                color: 'var(--vscode-terminal-foreground, #cccccc)'
            }}
        >
            {/* Terminal Panel Header */}
            <div className="terminal-header" style={{
                height: '35px',
                display: 'flex',
                alignItems: 'center',
                padding: '0 8px',
                borderBottom: '1px solid var(--vscode-panel-border, rgba(128, 128, 128, 0.35))',
                background: 'var(--vscode-panel-background)',
                gap: '8px'
            }}>
                <div style={{ flex: 1, display: 'flex', alignItems: 'center', gap: '8px' }}>
                    <span style={{ fontSize: '11px', fontWeight: 600, color: 'var(--vscode-foreground)', opacity: 0.8 }}>TERMINAL</span>
                </div>

                <div className="terminal-actions" style={{ display: 'flex', alignItems: 'center', gap: '4px' }}>
                    {availableShells.length > 0 && (
                        <select
                            value={selectedShell || ''}
                            onChange={(e) => setSelectedShell(e.target.value || undefined)}
                            style={{
                                background: 'transparent',
                                border: '1px solid var(--vscode-panel-border)',
                                color: 'var(--vscode-foreground)',
                                fontSize: '11px',
                                outline: 'none',
                                padding: '2px 4px',
                                borderRadius: '2px',
                                marginRight: '4px',
                                opacity: 0.8
                            }}
                        >
                            <option value="">Default Shell</option>
                            {availableShells.map(s => (
                                <option key={s} value={s}>{s.split(/[\\/]/).pop()}</option>
                            ))}
                        </select>
                    )}
                    <div
                        className="terminal-action-item"
                        onClick={handleAddAiri}
                        title="Open AIRI live activity feed"
                        style={{
                            display: 'flex',
                            alignItems: 'center',
                            gap: 4,
                            padding: '0 6px',
                            fontSize: 11,
                            color: 'var(--vscode-terminalCommandDecoration-successBackground, #1a85ff)'
                        }}
                    >
                        <i className="codicon codicon-radio-tower"></i>
                        <span>AIRI</span>
                    </div>
                    <div className="terminal-action-item" onClick={handleAddTerminal} title="New Terminal (Ctrl+Shift+`)">
                        <i className="codicon codicon-add"></i>
                    </div>
                    <div className="terminal-action-item" onClick={handleKillTerminal} title="Kill Terminal">
                        <i className="codicon codicon-trash"></i>
                    </div>
                    <div className="terminal-action-separator" style={{ width: '1px', height: '16px', background: 'var(--vscode-panel-border)', margin: '0 4px' }}></div>
                    <div className="terminal-action-item" onClick={toggleBottomPanel} title="Close Panel">
                        <i className="codicon codicon-close"></i>
                    </div>
                </div>
            </div>

            <div
                className="terminal-body"
                style={{
                    flex: 1,
                    display: 'flex',
                    flexDirection: 'row',
                    width: '100%',
                    height: 'calc(100% - 35px)',
                    overflow: 'hidden'
                }}
            >
                <div
                    className="terminal-groups-host"
                    style={{
                        flex: 1,
                        position: 'relative',
                        height: '100%',
                        overflow: 'hidden'
                    }}
                >
                    {groups.map((group) => (
                        <TerminalGroupView
                            key={group.id}
                            groupId={group.id}
                            active={activeGroupId === group.id}
                        />
                    ))}

                    {groups.length === 0 && (
                        <div style={{
                            display: 'flex',
                            flexDirection: 'column',
                            alignItems: 'center',
                            justifyContent: 'center',
                            height: '100%',
                            opacity: 0.5,
                            fontSize: '12px',
                            color: 'var(--vscode-foreground)'
                        }}>
                            <i className="codicon codicon-terminal" style={{ fontSize: '48px', marginBottom: '16px', opacity: 0.2 }}></i>
                            <div>No active terminals.</div>
                            <button
                                onClick={handleAddTerminal}
                                style={{
                                    marginTop: '12px',
                                    background: 'var(--vscode-button-background)',
                                    color: 'var(--vscode-button-foreground)',
                                    border: 'none',
                                    padding: '4px 12px',
                                    borderRadius: '2px',
                                    cursor: 'pointer',
                                    fontSize: '11px'
                                }}
                            >
                                Create Terminal
                            </button>
                        </div>
                    )}
                </div>

                {/* Vertical Tabs Section (Right Side) */}
                {groups.length > 0 && (
                    <div style={{ width: '180px', height: '100%', borderLeft: '1px solid var(--vscode-panel-border, rgba(128, 128, 128, 0.35))' }}>
                        <TerminalSidebar />
                    </div>
                )}
            </div>

            <style>{`
                .terminal-action-item {
                    width: 24px;
                    height: 24px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    cursor: pointer;
                    border-radius: 4px;
                    color: var(--vscode-foreground);
                    opacity: 0.7;
                    transition: all 0.1s;
                }
                .terminal-action-item:hover {
                    opacity: 1;
                    background: var(--vscode-toolbar-hoverBackground, rgba(128, 128, 128, 0.15));
                }
                .terminal-action-item i {
                    font-size: 14px;
                }
                .terminal-view-host ::-webkit-scrollbar {
                    width: 10px;
                    height: 10px;
                }
                .terminal-view-host ::-webkit-scrollbar-thumb {
                    background: var(--vscode-scrollbarSlider-background, rgba(121, 121, 121, 0.4));
                }
                .terminal-view-host ::-webkit-scrollbar-thumb:hover {
                    background: var(--vscode-scrollbarSlider-hoverBackground, rgba(100, 100, 100, 0.7));
                }
            `}</style>
        </div>
    );
};

export default TerminalView;
