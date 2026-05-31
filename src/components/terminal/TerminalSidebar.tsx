import React, { useState } from 'react';
import { useStore } from '../../store';
import ContextMenu from './ContextMenu';

const TerminalSidebar: React.FC = () => {
    const groups = useStore(state => state.terminalGroups);
    const activeGroupId = useStore(state => state.activeTerminalGroupId);
    const setActiveGroup = useStore(state => state.setActiveTerminalGroup);
    const renameGroup = useStore(state => state.renameTerminalGroup);
    const closeGroup = useStore(state => state.closeTerminalGroup);
    const splitTerminal = useStore(state => state.splitTerminal);
    
    const [menuState, setMenuState] = useState({ visible: false, x: 0, y: 0, groupId: '' });
    const [renamingId, setRenamingId] = useState<string | null>(null);
    const [renameValue, setRenameValue] = useState('');

    const handleContextMenu = (e: React.MouseEvent, groupId: string) => {
        e.preventDefault();
        setMenuState({
            visible: true,
            x: e.clientX,
            y: e.clientY,
            groupId
        });
    };

    const startRename = (groupId: string, initialName: string) => {
        setRenamingId(groupId);
        setRenameValue(initialName);
    };

    const handleRenameSubmit = () => {
        if (renamingId && renameValue.trim()) {
            renameGroup(renamingId, renameValue.trim());
        }
        setRenamingId(null);
    };

    const contextMenuOptions = [
        { 
            label: 'Split Terminal', 
            icon: 'codicon-split-horizontal', 
            onClick: () => {
                const group = groups.find(g => g.id === menuState.groupId);
                if (group) splitTerminal(group.id, group.activeInstanceId);
            }
        },
        { 
            label: 'Rename...', 
            icon: 'codicon-edit', 
            onClick: () => {
                const group = groups.find(g => g.id === menuState.groupId);
                if (group) startRename(group.id, group.name);
            }
        },
        { 
            label: 'Close Terminal', 
            icon: 'codicon-close', 
            danger: true,
            onClick: () => closeGroup(menuState.groupId)
        }
    ];

    return (
        <div 
            className="terminal-sidebar"
            style={{ 
                width: '100%', 
                height: '100%', 
                background: 'var(--vscode-panel-background)',
                display: 'flex',
                flexDirection: 'column',
                overflow: 'hidden'
            }}
        >
            <ContextMenu 
                visible={menuState.visible}
                x={menuState.x}
                y={menuState.y}
                options={contextMenuOptions}
                onClose={() => setMenuState({ ...menuState, visible: false })}
            />

            {/* Terminal List */}
            <div className="terminal-groups-section" style={{ 
                flex: 1, 
                overflowY: 'auto',
                padding: '4px 0'
            }}>
                {groups.map((group) => (
                    <div 
                        key={group.id}
                        onClick={() => setActiveGroup(group.id)}
                        onContextMenu={(e) => handleContextMenu(e, group.id)}
                        className={`terminal-group-item ${activeGroupId === group.id ? 'active' : ''}`}
                        style={{ 
                            height: '22px',
                            padding: '0 12px',
                            fontSize: '11px',
                            display: 'flex',
                            alignItems: 'center',
                            cursor: 'pointer',
                            position: 'relative',
                            background: activeGroupId === group.id ? 'var(--vscode-list-activeSelectionBackground, #04395e)' : 'transparent',
                            color: activeGroupId === group.id ? 'var(--vscode-list-activeSelectionForeground, #ffffff)' : 'var(--vscode-foreground, #cccccc)',
                        }}
                    >
                        <div style={{ display: 'flex', alignItems: 'center', gap: '8px', flex: 1, overflow: 'hidden' }}>
                            <i className="codicon codicon-terminal" style={{ 
                                fontSize: '13px', 
                                opacity: activeGroupId === group.id ? 1 : 0.6
                            }}></i>
                            {renamingId === group.id ? (
                                <input 
                                    autoFocus
                                    value={renameValue}
                                    onChange={(e) => setRenameValue(e.target.value)}
                                    onBlur={handleRenameSubmit}
                                    onKeyDown={(e) => {
                                        if (e.key === 'Enter') handleRenameSubmit();
                                        if (e.key === 'Escape') setRenamingId(null);
                                    }}
                                    style={{
                                        background: 'var(--vscode-input-background, #3c3c3c)',
                                        color: 'var(--vscode-input-foreground, #cccccc)',
                                        border: '1px solid var(--vscode-focusBorder, #007acc)',
                                        fontSize: '11px',
                                        padding: '0 4px',
                                        width: '100%',
                                        outline: 'none',
                                        height: '18px'
                                    }}
                                />
                            ) : (
                                <span style={{ 
                                    flex: 1, 
                                    overflow: 'hidden', 
                                    textOverflow: 'ellipsis', 
                                    whiteSpace: 'nowrap'
                                }}>{group.name}</span>
                            )}
                        </div>
                        
                        <div className="tab-actions" style={{ 
                            display: 'flex', 
                            gap: '4px', 
                            opacity: activeGroupId === group.id ? 1 : 0, 
                            marginLeft: '4px'
                        }}>
                            <i className="codicon codicon-split-horizontal" style={{ fontSize: '12px', cursor: 'pointer', opacity: 0.7 }} 
                               onClick={(e) => { e.stopPropagation(); splitTerminal(group.id, group.activeInstanceId); }} 
                               title="Split Terminal"
                            />
                            <i className="codicon codicon-close" style={{ fontSize: '12px', cursor: 'pointer', opacity: 0.7 }} 
                               onClick={(e) => { e.stopPropagation(); closeGroup(group.id); }} 
                               title="Close Terminal Group"
                            />
                        </div>
                    </div>
                ))}
            </div>
            
            <style>{`
                .terminal-group-item:hover {
                    background: var(--vscode-list-hoverBackground, rgba(128, 128, 128, 0.1));
                }
                .terminal-group-item.active:hover {
                    background: var(--vscode-list-activeSelectionBackground, #04395e);
                }
                .terminal-group-item:hover .tab-actions {
                    opacity: 1 !important;
                }
                .tab-actions i:hover {
                    opacity: 1 !important;
                }
            `}</style>
        </div>
    );
};

export default TerminalSidebar;
