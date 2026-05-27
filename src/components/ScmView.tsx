import React, { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useStore } from '../store';
import GitGraph from './GitGraph';

interface GitStatus {
    path: string;
    status: string; // two-column porcelain status: "M ", " M", "??", etc.
}

const DiffInline: React.FC<{ content: string; loading: boolean }> = ({ content, loading }) => {
    if (loading) return <div style={{ padding: '6px 12px', fontSize: '11px', opacity: 0.5, fontFamily: 'var(--font-mono)' }}>Loading diff…</div>;
    if (!content) return null;
    return (
        <div style={{ maxHeight: '240px', overflowY: 'auto', margin: '0 0 6px 0', borderRadius: '4px', border: '1px solid rgba(255,255,255,0.06)', fontSize: '11px', fontFamily: 'var(--font-mono)', background: 'rgba(0,0,0,0.25)' }}>
            {content.split('\n').map((line, i) => {
                let bg = 'transparent', color = 'inherit';
                if (line.startsWith('+') && !line.startsWith('+++')) { bg = 'rgba(16,185,129,0.12)'; color = '#4ade80'; }
                else if (line.startsWith('-') && !line.startsWith('---')) { bg = 'rgba(239,68,68,0.12)'; color = '#f87171'; }
                else if (line.startsWith('@@')) { color = '#60a5fa'; bg = 'rgba(96,165,250,0.06)'; }
                else if (line.startsWith('diff ') || line.startsWith('index ') || line.startsWith('---') || line.startsWith('+++')) { color = 'rgba(255,255,255,0.35)'; }
                return (
                    <div key={i} style={{ padding: '0 8px', background: bg, color, lineHeight: '18px', whiteSpace: 'pre', overflow: 'hidden' }}>
                        {line || ' '}
                    </div>
                );
            })}
        </div>
    );
};

const ScmView: React.FC = () => {
    const [statuses, setStatuses] = useState<GitStatus[]>([]);
    const [commitMessage, setCommitMessage] = useState('');
    const [isRefreshing, setIsRefreshing] = useState(false);
    const [isGeneratingMsg, setIsGeneratingMsg] = useState(false);
    const [activeTab, setActiveTab] = useState<'changes' | 'graph'>('changes');
    const [conflicts, setConflicts] = useState<string[]>([]);
    const [diffFile, setDiffFile] = useState<string | null>(null);
    const [diffContent, setDiffContent] = useState<string>('');
    const [isDiffLoading, setIsDiffLoading] = useState(false);
    const activeRoot = useStore(state => state.activeRoot);

    const showDiff = useCallback(async (filePath: string) => {
        if (diffFile === filePath) { setDiffFile(null); setDiffContent(''); return; }
        setDiffFile(filePath);
        setIsDiffLoading(true);
        try {
            // Use git diff for the specific file (unstaged changes)
            const diff = await invoke<string>('git_diff_file', { path: activeRoot, filePath });
            setDiffContent(diff);
        } catch {
            setDiffContent('(diff unavailable)');
        } finally {
            setIsDiffLoading(false);
        }
    }, [activeRoot, diffFile]);

    useEffect(() => {
        refreshStatus();
        checkConflicts();
    }, [activeRoot]);

    const refreshStatus = async () => {
        if (!activeRoot) return;
        setIsRefreshing(true);
        try {
            const result = await invoke<GitStatus[]>('git_status', { path: activeRoot });
            setStatuses(result);
            await checkConflicts();
        } catch (e) {
            console.error('Git status failed', e);
        } finally {
            setIsRefreshing(false);
        }
    };

    const checkConflicts = async () => {
        if (!activeRoot) return;
        try {
            const result = await invoke<string[]>('git_get_unmerged', { path: activeRoot });
            setConflicts(result);
        } catch (e) {
            console.error('Conflict check failed', e);
        }
    };

    const handleGitAction = async (action: string) => {
        if (!activeRoot) return;
        try {
            setIsRefreshing(true);
            if (action === 'stash') {
                await invoke('git_stash', { path: activeRoot });
            } else if (action === 'stash_pop') {
                await invoke('git_stash_pop', { path: activeRoot });
            } else {
                // For Push/Pull/Fetch, we currently use terminal stubs or direct shell calls
                // In a full implementation, these would be dedicated Tauri commands
                console.log(`Executing global action: ${action}`);
            }
            refreshStatus();
        } catch (e) {
            alert(`${action} failed: ${e}`);
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

    const visibleStatus = (status: string) => status.trim() || status;
    const staged = statuses.filter(s => s.status !== '??' && s.status[0] !== ' ' && s.status[0] !== undefined);
    const unstaged = statuses.filter(s => s.status === '??' || (s.status[1] !== ' ' && s.status[1] !== undefined));

    return (
        <div className="scm-view" style={{ display: 'flex', flexDirection: 'column', height: '100%' }}>
            {/* Global Actions Header */}
            <div style={{
                display: 'flex', gap: 4, padding: '8px 10px',
                background: 'var(--vscode-sideBar-background)',
                borderBottom: '1px solid var(--vscode-sideBar-border)',
                flexWrap: 'wrap'
            }}>
                {[
                    { id: 'pull', icon: 'cloud-download', label: 'Pull' },
                    { id: 'push', icon: 'cloud-upload', label: 'Push' },
                    { id: 'fetch', icon: 'sync', label: 'Fetch' },
                    { id: 'stash', icon: 'archive', label: 'Stash' },
                    { id: 'stash_pop', icon: 'unarchive', label: 'Pop' }
                ].map(btn => (
                    <button key={btn.id}
                        onClick={() => handleGitAction(btn.id)}
                        disabled={isRefreshing}
                        title={btn.label}
                        style={{
                            background: 'rgba(255,255,255,0.05)', border: 'none', color: 'rgba(255,255,255,0.7)',
                            padding: '4px 8px', borderRadius: 4, cursor: 'pointer', fontSize: 10,
                            display: 'flex', alignItems: 'center', gap: 4, transition: 'all 0.15s'
                        }}>
                        <i className={`codicon codicon-${btn.icon}`} style={{ fontSize: 12 }}></i>
                        {btn.label}
                    </button>
                ))}
            </div>

            {/* Conflict Alert (Agentic Bridge) */}
            {conflicts.length > 0 && (
                <div style={{
                    margin: '10px', padding: '10px', borderRadius: 6,
                    background: 'rgba(245, 158, 11, 0.15)', border: '1px solid rgba(245, 158, 11, 0.3)',
                    display: 'flex', flexDirection: 'column', gap: 8
                }}>
                    <div style={{ fontSize: 11, fontWeight: 700, color: '#f59e0b', display: 'flex', alignItems: 'center', gap: 6 }}>
                        <i className="codicon codicon-warning"></i> {conflicts.length} MERGE CONFLICTS
                    </div>
                    <button
                        onClick={async () => {
                            const agentModel = useStore.getState().agentModel;
                            const agentMode = useStore.getState().agentMode;
                            const addAgentMessage = useStore.getState().addAgentMessage;

                            const provider = agentModel.includes(':') ? agentModel.split(':')[0] : 'anthropic';
                            const model = agentModel.includes(':') ? agentModel.split(':')[1] : agentModel;

                            addAgentMessage('user', `Help me resolve merge conflicts in: ${conflicts.join(', ')}`);

                            await invoke('ai_chat', {
                                request: {
                                    provider,
                                    model,
                                    messages: [
                                        {
                                            role: 'user',
                                            content: `I have merge conflicts in the following files: ${conflicts.join(', ')}. Please analyze them and suggest or apply a resolution strategy.`
                                        }
                                    ],
                                    autonomous: true,
                                    mode: agentMode
                                }
                            });
                        }}
                        style={{
                            background: '#f59e0b', color: '#000', border: 'none',
                            padding: '6px', borderRadius: 4, cursor: 'pointer',
                            fontSize: 11, fontWeight: 700, display: 'flex', alignItems: 'center', justifyContent: 'center', gap: 6
                        }}>
                        <i className="codicon codicon-sparkle"></i> RESOLVE VIA AI
                    </button>
                </div>
            )}

            {/* Commit input (always visible) */}
            <div style={{ padding: '10px 10px 0 10px', flexShrink: 0 }}>
                <div style={{ position: 'relative' }}>
                    <textarea
                        value={commitMessage}
                        onChange={(e) => setCommitMessage(e.target.value)}
                        placeholder="Commit message (Ctrl+Enter to commit)"
                        style={{
                            width: '100%', height: '50px',
                            background: 'var(--vscode-input-background)',
                            color: 'var(--vscode-input-foreground)',
                            border: '1px solid var(--vscode-input-border, var(--vscode-panel-border))',
                            padding: '6px 32px 6px 6px', fontSize: '11px', outline: 'none',
                            resize: 'none', borderRadius: '4px', boxSizing: 'border-box'
                        }}
                    />
                    {/* Sparkle button: AI commit message */}
                    <button
                        title="Generate AI commit message from staged diff"
                        disabled={isGeneratingMsg}
                        onClick={async () => {
                            setIsGeneratingMsg(true);
                            try {
                                const msg = await useStore.getState().generateAiCommitMessage();
                                if (msg) setCommitMessage(msg);
                            } finally {
                                setIsGeneratingMsg(false);
                            }
                        }}
                        style={{
                            position: 'absolute', top: '6px', right: '6px',
                            background: 'none', border: 'none', cursor: isGeneratingMsg ? 'wait' : 'pointer',
                            color: '#a78bfa', fontSize: '14px', padding: '0', opacity: isGeneratingMsg ? 0.5 : 1,
                            transition: 'opacity 0.2s',
                        }}
                    >
                        {isGeneratingMsg
                            ? <i className="codicon codicon-sync" style={{ fontFamily: 'codicon', fontStyle: 'normal', animation: 'spin 1s linear infinite', display: 'inline-block' }} />
                            : <i className="codicon codicon-sparkle" style={{ fontFamily: 'codicon', fontStyle: 'normal' }} />
                        }
                    </button>
                </div>
                <button
                    onClick={handleCommit}
                    disabled={!commitMessage}
                    style={{
                        width: '100%', marginTop: '6px',
                        background: 'var(--vscode-button-background)',
                        color: 'white', border: 'none', padding: '6px',
                        cursor: 'pointer', borderRadius: '4px',
                        fontSize: '11px', fontWeight: 600, opacity: commitMessage ? 1 : 0.6
                    }}
                >
                    Commit to Main
                </button>
            </div>


            {/* Tab bar (Changes / Visual Graph) */}
            <div style={{
                display: 'flex', margin: '10px 10px 0 10px',
                borderBottom: '1px solid var(--vscode-sideBar-border, var(--vscode-panel-border))',
                flexShrink: 0
            }}>
                <button
                    onClick={() => setActiveTab('changes')}
                    style={{
                        flex: 1, padding: '8px 0', fontSize: '11px', fontWeight: 600,
                        background: 'transparent', border: 'none', cursor: 'pointer',
                        color: activeTab === 'changes' ? 'var(--vscode-foreground)' : 'var(--vscode-foreground)',
                        opacity: activeTab === 'changes' ? 1 : 0.5,
                        borderBottom: activeTab === 'changes' ? '2px solid var(--vscode-focusBorder)' : '2px solid transparent',
                        transition: 'all 0.2s'
                    }}
                >
                    CHANGES {(staged.length + unstaged.length) > 0 ? `(${staged.length + unstaged.length})` : ''}
                </button>
                <button
                    onClick={() => setActiveTab('graph')}
                    style={{
                        flex: 1, padding: '8px 0', fontSize: '11px', fontWeight: 600,
                        background: 'transparent', border: 'none', cursor: 'pointer',
                        color: activeTab === 'graph' ? 'var(--vscode-foreground)' : 'var(--vscode-foreground)',
                        opacity: activeTab === 'graph' ? 1 : 0.5,
                        borderBottom: activeTab === 'graph' ? '2px solid var(--vscode-focusBorder)' : '2px solid transparent',
                        transition: 'all 0.2s'
                    }}
                >
                    VISUAL GRAPH
                </button>
            </div>

            {/* Tab content */}
            <div style={{ flex: 1, overflow: 'hidden', display: 'flex', flexDirection: 'column' }}>
                {activeTab === 'changes' ? (
                    <div style={{ flex: 1, overflowY: 'auto', padding: '10px' }}>
                        {staged.length > 0 && (
                            <div className="scm-section">
                                <div style={{ fontSize: '10px', fontWeight: 'bold', marginBottom: '8px', opacity: 0.6, letterSpacing: '0.5px' }}>STAGED</div>
                                {staged.map((s, i) => (
                                    <React.Fragment key={i}>
                                        <div
                                            className="scm-file-item"
                                            style={{ display: 'flex', alignItems: 'center', padding: '4px 6px', fontSize: '12px', borderRadius: 4, margin: '2px 0', cursor: 'pointer', background: diffFile === s.path ? 'var(--vscode-list-activeSelectionBackground, rgba(0,122,204,0.2))' : 'transparent' }}
                                            onClick={() => showDiff(s.path)}
                                        >
                                            <i className="codicon codicon-file" style={{ fontSize: 13, marginRight: 8, opacity: 0.4 }}></i>
                                            <span style={{ flex: 1, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{s.path}</span>
                                            <span style={{ color: '#4ec9b0', width: '15px', textAlign: 'center', fontSize: 10, fontWeight: 700 }}>{visibleStatus(s.status)}</span>
                                            <i className="codicon codicon-remove" onClick={e => { e.stopPropagation(); handleUnstage(s.path); }} style={{ marginLeft: '8px', cursor: 'pointer', opacity: 0.4 }} />
                                        </div>
                                        {diffFile === s.path && (
                                            <DiffInline content={diffContent} loading={isDiffLoading} />
                                        )}
                                    </React.Fragment>
                                ))}
                            </div>
                        )}
                        <div className="scm-section" style={{ marginTop: '15px' }}>
                            <div style={{ fontSize: '10px', fontWeight: 'bold', marginBottom: '8px', opacity: 0.6, letterSpacing: '0.5px' }}>UNSTAGED</div>
                            {unstaged.length > 0 ? (
                                unstaged.map((s, i) => (
                                    <React.Fragment key={i}>
                                        <div
                                            className="scm-file-item"
                                            style={{ display: 'flex', alignItems: 'center', padding: '4px 6px', fontSize: '12px', borderRadius: 4, margin: '2px 0', cursor: 'pointer', background: diffFile === s.path ? 'var(--vscode-list-activeSelectionBackground, rgba(0,122,204,0.2))' : 'transparent' }}
                                            onClick={() => showDiff(s.path)}
                                        >
                                            <i className="codicon codicon-file" style={{ fontSize: 13, marginRight: 8, opacity: 0.4 }}></i>
                                            <span style={{ flex: 1, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{s.path}</span>
                                            <span style={{ color: '#d16d9e', width: '15px', textAlign: 'center', fontSize: 10, fontWeight: 700 }}>{visibleStatus(s.status)}</span>
                                            <i className="codicon codicon-add" onClick={e => { e.stopPropagation(); handleStage(s.path); }} style={{ marginLeft: '8px', cursor: 'pointer', opacity: 0.4 }} />
                                        </div>
                                        {diffFile === s.path && (
                                            <DiffInline content={diffContent} loading={isDiffLoading} />
                                        )}
                                    </React.Fragment>
                                ))
                            ) : (
                                <div style={{ opacity: 0.5, fontSize: '11px', textAlign: 'center', padding: '20px 0' }}>No local changes.</div>
                            )}
                        </div>
                    </div>
                ) : (
                    <GitGraph />
                )}
            </div>
        </div>
    );
};

export default ScmView;
