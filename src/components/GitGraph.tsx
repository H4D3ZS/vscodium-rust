import React, { useEffect, useState, useRef } from 'react';
import { invoke } from '../tauri_bridge';

interface GitCommit {
    hash: string;
    author: string;
    date: string;
    message: string;
    parents: string[];
}

const GitGraph: React.FC = () => {
    const [history, setHistory] = useState<GitCommit[]>([]);
    const [loading, setLoading] = useState(true);
    const containerRef = useRef<HTMLDivElement>(null);

    const fetchHistory = async () => {
        try {
            setLoading(true);
            const data = await invoke<GitCommit[]>('get_git_history', { path: "" });
            setHistory(data);
        } catch (e) {
            console.error("Failed to fetch git history:", e);
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        fetchHistory();
    }, []);

    if (loading) return <div style={{ padding: '20px', fontSize: '12px', opacity: 0.7 }}>Loading visual git history...</div>;

    const nodeRadius = 6;

    return (
        <div className="git-graph-view" ref={containerRef} style={{ display: 'flex', flexDirection: 'column', height: '100%', overflow: 'hidden', background: 'var(--vscode-sideBar-background)', position: 'relative' }}>
            <div className="git-graph-container" style={{ flex: 1, overflowX: 'hidden', overflowY: 'auto', position: 'relative', padding: '10px 0' }}>
                {/* Vertical background line */}
                <div style={{ 
                    position: 'absolute', 
                    top: 0, 
                    left: '20px', 
                    bottom: 0, 
                    width: '1px', 
                    background: 'var(--vscode-focusBorder)', 
                    opacity: 0.3,
                    zIndex: 1 
                }}></div>

                <div style={{ position: 'relative', zIndex: 2 }}>
                    {history.map((commit, i) => (
                        <div key={commit.hash} className="git-commit-node" style={{ 
                            position: 'relative', 
                            display: 'flex', 
                            padding: '12px 12px 12px 0',
                            cursor: 'pointer',
                            minHeight: '60px'
                        }}>
                            <div style={{ 
                                width: '40px', 
                                display: 'flex', 
                                justifyContent: 'center', 
                                alignItems: 'flex-start',
                                paddingTop: '4px',
                                flexShrink: 0,
                                position: 'relative'
                            }}>
                                <div style={{ 
                                    width: nodeRadius * 2, 
                                    height: nodeRadius * 2, 
                                    borderRadius: '50%', 
                                    background: 'var(--vscode-focusBorder)',
                                    border: '2px solid var(--vscode-sideBar-background)',
                                    zIndex: 3
                                }}></div>
                            </div>
                            
                            <div style={{ 
                                display: 'flex', 
                                flexDirection: 'column', 
                                gap: '4px',
                                overflow: 'hidden',
                                flex: 1
                            }}>
                                <div style={{ 
                                    fontSize: '12px', 
                                    fontWeight: 500, 
                                    color: 'var(--vscode-sideBar-foreground)', 
                                    whiteSpace: 'normal', 
                                    wordBreak: 'break-word',
                                    lineHeight: '1.4'
                                }}>
                                    {commit.message}
                                </div>
                                <div style={{ fontSize: '10px', opacity: 0.6, display: 'flex', flexWrap: 'wrap', gap: '8px', alignItems: 'center' }}>
                                    <span style={{ background: 'var(--vscode-badge-background)', color: 'var(--vscode-badge-foreground)', padding: '0 4px', borderRadius: '2px', fontSize: '9px', fontFamily: 'monospace' }}>{commit.hash.substring(0, 7)}</span>
                                    <span style={{ fontWeight: 600 }}>{commit.author}</span>
                                    <span style={{ opacity: 0.7 }}>{commit.date.split(' ')[0]}</span>
                                </div>
                            </div>
                        </div>
                    ))}
                </div>
            </div>
        </div>
    );
};

export default GitGraph;
