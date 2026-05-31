import React from 'react';
import TerminalSidebar from './TerminalSidebar';
import TerminalGroupView from './TerminalGroupView';
import { useStore } from '../../store';

const TerminalView: React.FC = () => {
    const groups = useStore(state => state.terminalGroups);
    const activeGroupId = useStore(state => state.activeTerminalGroupId);
    const addTerminalGroup = useStore(state => state.addTerminalGroup);

    const handleAddTerminal = () => {
        addTerminalGroup();
    };

    return (
        <div
            className="terminal-view-host"
            style={{
                display: 'flex',
                flexDirection: 'row',
                width: '100%',
                height: '100%',
                background: 'var(--vscode-panel-background, #1e1e1e)',
                color: 'var(--vscode-terminal-foreground, #cccccc)'
            }}
        >
            <div
                className="terminal-body"
                style={{
                    flex: 1,
                    display: 'flex',
                    flexDirection: 'row',
                    width: '100%',
                    height: '100%',
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
                {groups.length > 1 && (
                    <div style={{ width: '160px', height: '100%', borderLeft: '1px solid var(--vscode-panel-border, rgba(128, 128, 128, 0.35))' }}>
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
