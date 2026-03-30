import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useStore } from '../store';
import GitGraph from './GitGraph';

interface GitStatus {
    path: string;
    status: string; // M, A, D, ??
}

const ScmView: React.FC = () => {
    const [statuses, setStatuses] = useState<GitStatus[]>([]);
    const [commitMessage, setCommitMessage] = useState('');
    const [isRefreshing, setIsRefreshing] = useState(false);
    const [activeTab, setActiveTab] = useState<'changes' | 'graph'>('changes');
    const activeRoot = useStore(state => state.activeRoot);

    useEffect(() => {
        refreshStatus();
    }, [activeRoot]);

    const refreshStatus = async () => {
        if (!activeRoot) return;
        setIsRefreshing(true);
        try {
            const result = await invoke<GitStatus[]>('git_status', { path: activeRoot });
            setStatuses(result);
        } catch (e) {
            console.error('Git status failed', e);
        } finally {
            setIsRefreshing(false);
        }
    };

    const handleStage = async (path: string) => {
        try {
            await invoke('git_stage', { path: activeRoot, filePath: path });
            refreshStatus();
        } catch (e) {
            console.error('Stage failed', e);
        }
    };

    const handleUnstage = async (path: string) => {
        try {
            await invoke('git_unstage', { path: activeRoot, filePath: path });
            refreshStatus();
        } catch (e) {
            console.error('Unstage failed', e);
        }
    };

    const handleCommit = async () => {
        if (!commitMessage) return;
        try {
            await invoke('git_commit', { path: activeRoot, message: commitMessage });
            setCommitMessage('');
            refreshStatus();
        } catch (e) {
            alert(`Commit failed: ${e}`);
        }
    };

    const staged = statuses.filter(s => ['M', 'A', 'D'].includes(s.status) && !s.status.includes(' '));
    const unstaged = statuses.filter(s => s.status === '??' || s.status.includes(' '));

    return (
        <div className="scm-view" style={{ display: 'flex', flexDirection: 'column', height: '100%' }}>
            {/* Commit input (always visible) */}
            <div style={{ padding: '10px 10px 0 10px', flexShrink: 0 }}>
                <textarea
                    value={commitMessage}
                    onChange={(e) => setCommitMessage(e.target.value)}
                    placeholder="Commit message (Ctrl+Enter to commit)"
                    style={{
                        width: '100%', height: '50px',
                        background: 'var(--vscode-input-background)',
                        color: 'var(--vscode-input-foreground)',
                        border: '1px solid var(--vscode-input-border, var(--vscode-panel-border))',
                        padding: '6px', fontSize: '12px', outline: 'none',
                        resize: 'none', borderRadius: '2px'
                    }}
                />
                <button
                    onClick={handleCommit}
                    disabled={!commitMessage}
                    style={{
                        width: '100%', marginTop: '6px',
                        background: 'var(--vscode-button-background)',
                        color: 'white', border: 'none', padding: '5px',
                        cursor: 'pointer', borderRadius: '2px',
                        fontSize: '12px', opacity: commitMessage ? 1 : 0.6
                    }}
                >
                    Commit
                </button>
            </div>

            {/* Tab bar */}
            <div style={{
                display: 'flex', margin: '10px 10px 0 10px',
                borderBottom: '1px solid var(--vscode-sideBar-border, var(--vscode-panel-border))',
                flexShrink: 0
            }}>
                <button
                    onClick={() => setActiveTab('changes')}
                    style={{
                        flex: 1, padding: '6px 0', fontSize: '11px', fontWeight: 600,
                        background: 'transparent', border: 'none', cursor: 'pointer',
                        color: activeTab === 'changes' ? 'var(--vscode-foreground)' : 'var(--vscode-foreground)',
                        opacity: activeTab === 'changes' ? 1 : 0.5,
                        borderBottom: activeTab === 'changes' ? '2px solid var(--vscode-focusBorder)' : '2px solid transparent',
                        textTransform: 'uppercase', letterSpacing: '0.5px',
                        transition: 'opacity 0.15s'
                    }}
                >
                    <i className="codicon codicon-file" style={{ marginRight: 4, fontSize: 12 }} />
                    Changes {(staged.length + unstaged.length) > 0 ? `(${staged.length + unstaged.length})` : ''}
                </button>
                <button
                    onClick={() => setActiveTab('graph')}
                    style={{
                        flex: 1, padding: '6px 0', fontSize: '11px', fontWeight: 600,
                        background: 'transparent', border: 'none', cursor: 'pointer',
                        color: activeTab === 'graph' ? 'var(--vscode-foreground)' : 'var(--vscode-foreground)',
                        opacity: activeTab === 'graph' ? 1 : 0.5,
                        borderBottom: activeTab === 'graph' ? '2px solid var(--vscode-focusBorder)' : '2px solid transparent',
                        textTransform: 'uppercase', letterSpacing: '0.5px',
                        transition: 'opacity 0.15s'
                    }}
                >
                    <i className="codicon codicon-git-merge" style={{ marginRight: 4, fontSize: 12 }} />
                    Visual Graph
                </button>
            </div>

            {/* Tab content */}
            <div style={{ flex: 1, overflow: 'hidden', display: 'flex', flexDirection: 'column' }}>
                {activeTab === 'changes' ? (
                    <div style={{ flex: 1, overflowY: 'auto', padding: '10px' }}>
                        {staged.length > 0 && (
                            <div className="scm-section">
                                <div style={{ fontSize: '11px', fontWeight: 'bold', marginBottom: '8px', opacity: 0.8 }}>STAGED CHANGES</div>
                                {staged.map((s, i) => (
                                    <div key={i} style={{ display: 'flex', alignItems: 'center', padding: '4px 0', fontSize: '12px' }}>
                                        <span style={{ flex: 1, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{s.path}</span>
                                        <span style={{ color: '#4ec9b0', width: '15px', textAlign: 'center' }}>{s.status}</span>
                                        <i className="codicon codicon-remove" onClick={() => handleUnstage(s.path)} style={{ marginLeft: '8px', cursor: 'pointer', opacity: 0.6 }} />
                                    </div>
                                ))}
                            </div>
                        )}
                        <div className="scm-section" style={{ marginTop: '15px' }}>
                            <div style={{ fontSize: '11px', fontWeight: 'bold', marginBottom: '8px', opacity: 0.8 }}>CHANGES</div>
                            {unstaged.length > 0 ? (
                                unstaged.map((s, i) => (
                                    <div key={i} style={{ display: 'flex', alignItems: 'center', padding: '4px 0', fontSize: '12px' }}>
                                        <span style={{ flex: 1, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{s.path}</span>
                                        <span style={{ color: '#d16d9e', width: '15px', textAlign: 'center' }}>{s.status}</span>
                                        <i className="codicon codicon-add" onClick={() => handleStage(s.path)} style={{ marginLeft: '8px', cursor: 'pointer', opacity: 0.6 }} />
                                    </div>
                                ))
                            ) : (
                                <div style={{ opacity: 0.5, fontSize: '11px' }}>No changes detected.</div>
                            )}
                        </div>
                    </div>
                ) : (
                    /* ── Visual Graph Tab ── */
                    <GitGraph />
                )}
            </div>
        </div>
    );
};

export default ScmView;
