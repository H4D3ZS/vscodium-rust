import React from 'react';
import { useStore } from '../store';
import { FileDiff } from './DiffViewer';

const Composer: React.FC = () => {
    const pendingChanges = useStore(state => state.pendingChanges);
    const acceptChange = useStore(state => state.acceptPendingChange);
    const rejectChange = useStore(state => state.rejectPendingChange);
    const isBottomPanelOpen = useStore(state => state.isBottomPanelOpen);
    const toggleBottomPanel = useStore(state => state.toggleBottomPanel);

    if (pendingChanges.length === 0) {
        return (
            <div style={{
                display: 'flex',
                flexDirection: 'column',
                alignItems: 'center',
                justifyContent: 'center',
                height: '100%',
                opacity: 0.5,
                color: 'var(--vscode-foreground)'
            }}>
                <i className="codicon codicon-combine" style={{ fontSize: '48px', marginBottom: '16px', opacity: 0.2 }}></i>
                <div>No pending changes to review.</div>
                <div style={{ fontSize: '11px', marginTop: '8px' }}>Ask AIRI to make some edits!</div>
            </div>
        );
    }

    return (
        <div className="composer-container" style={{
            display: 'flex',
            flexDirection: 'column',
            height: '100%',
            background: 'var(--vscode-editor-background)',
            color: 'var(--vscode-foreground)'
        }}>
            <div className="composer-header" style={{
                height: '35px',
                display: 'flex',
                alignItems: 'center',
                padding: '0 12px',
                borderBottom: '1px solid var(--vscode-panel-border)',
                background: 'var(--vscode-panel-background)',
                justifyContent: 'space-between'
            }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                    <span style={{ fontSize: '11px', fontWeight: 600, opacity: 0.8 }}>COMPOSER</span>
                    <span style={{
                        background: 'var(--vscode-badge-background)',
                        color: 'var(--vscode-badge-foreground)',
                        padding: '1px 6px',
                        borderRadius: '10px',
                        fontSize: '10px'
                    }}>{pendingChanges.length}</span>
                </div>
                <div className="composer-actions" style={{ display: 'flex', gap: '8px' }}>
                    <button
                        onClick={() => { }} // TODO: Accept All
                        style={{
                            background: 'var(--vscode-button-background)',
                            color: 'var(--vscode-button-foreground)',
                            border: 'none',
                            padding: '2px 8px',
                            fontSize: '11px',
                            borderRadius: '2px',
                            cursor: 'pointer'
                        }}
                    >
                        Accept All
                    </button>
                </div>
            </div>

            <div className="composer-body" style={{ flex: 1, overflowY: 'auto' }}>
                {pendingChanges.map(change => (
                    <div key={change.id} style={{
                        borderBottom: '1px solid var(--vscode-panel-border)',
                        padding: '12px'
                    }}>
                        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '8px' }}>
                            <div style={{ display: 'flex', alignItems: 'center', gap: '6px' }}>
                                <i className="codicon codicon-file"></i>
                                <span style={{ fontSize: '12px', fontWeight: 600 }}>{change.path}</span>
                            </div>
                            <div style={{ display: 'flex', gap: '4px' }}>
                                <button
                                    onClick={() => acceptChange(change.id)}
                                    title="Accept"
                                    style={{
                                        background: 'transparent',
                                        border: '1px solid var(--vscode-testing-iconPassed)',
                                        color: 'var(--vscode-testing-iconPassed)',
                                        borderRadius: '2px',
                                        padding: '2px 6px',
                                        cursor: 'pointer',
                                        fontSize: '10px'
                                    }}
                                >
                                    Approve
                                </button>
                                <button
                                    onClick={() => rejectChange(change.id)}
                                    title="Reject"
                                    style={{
                                        background: 'transparent',
                                        border: '1px solid var(--vscode-testing-iconFailed)',
                                        color: 'var(--vscode-testing-iconFailed)',
                                        borderRadius: '2px',
                                        padding: '2px 6px',
                                        cursor: 'pointer',
                                        fontSize: '10px'
                                    }}
                                >
                                    Reject
                                </button>
                            </div>
                        </div>
                        {change.description && (
                            <div style={{ fontSize: '11px', opacity: 0.7, marginBottom: '8px', fontStyle: 'italic' }}>
                                {change.description}
                            </div>
                        )}
                        <div style={{ height: '300px', border: '1px solid var(--vscode-panel-border)', borderRadius: '4px', overflow: 'hidden' }}>
                            <FileDiff change={change} />
                        </div>
                    </div>
                ))}
            </div>
        </div>
    );
};

export default Composer;
