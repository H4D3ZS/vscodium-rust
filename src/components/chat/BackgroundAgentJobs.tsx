/**
 * Cursor-style Background Agent Jobs panel — shows running/completed
 * background agents with progress, results, and management controls.
 */
import React, { useState } from 'react';
import { useStore } from '../../store';

const BackgroundAgentJobs: React.FC = () => {
    const backgroundAgents = useStore(s => s.backgroundAgents);
    const removeBackgroundAgent = useStore(s => s.removeBackgroundAgent);
    const clearBackgroundAgents = useStore(s => s.clearBackgroundAgents);
    const [expandedId, setExpandedId] = useState<string | null>(null);

    if (backgroundAgents.length === 0) return null;

    const running = backgroundAgents.filter(a => a.status === 'running' || a.status === 'pending');
    const completed = backgroundAgents.filter(a => a.status === 'done' || a.status === 'error');

    return (
        <div style={{
            margin: '0 10px 8px', borderRadius: '8px',
            border: '1px solid rgba(255,255,255,0.06)',
            background: 'rgba(255,255,255,0.02)',
            overflow: 'hidden',
        }}>
            {/* Header */}
            <div style={{
                display: 'flex', alignItems: 'center', gap: '6px',
                padding: '6px 10px',
                borderBottom: '1px solid rgba(255,255,255,0.04)',
                fontSize: '10px', fontWeight: 600, color: 'rgba(255,255,255,0.6)',
            }}>
                <i className="codicon codicon-workspace-trusted" style={{
                    fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px',
                    color: running.length > 0 ? '#3b82f6' : 'rgba(255,255,255,0.3)',
                }} />
                <span>Background Agents</span>
                <span style={{
                    fontSize: '9px', padding: '1px 5px', borderRadius: '8px',
                    background: running.length > 0 ? 'rgba(59,130,246,0.15)' : 'rgba(255,255,255,0.06)',
                    color: running.length > 0 ? '#3b82f6' : 'rgba(255,255,255,0.4)',
                }}>
                    {running.length} active
                </span>
                <div style={{ flex: 1 }} />
                {backgroundAgents.length > 1 && (
                    <button
                        onClick={clearBackgroundAgents}
                        style={{
                            background: 'transparent', border: 'none', color: 'rgba(255,255,255,0.3)',
                            fontSize: '9px', cursor: 'pointer', padding: '2px 4px',
                        }}
                        title="Clear all completed"
                    >
                        Clear all
                    </button>
                )}
            </div>

            {/* Running agents */}
            {running.map(agent => (
                <div key={agent.id} style={{
                    padding: '6px 10px',
                    borderBottom: '1px solid rgba(255,255,255,0.03)',
                }}>
                    <div style={{ display: 'flex', alignItems: 'center', gap: '6px' }}>
                        <span style={{
                            width: 6, height: 6, borderRadius: '50%', flexShrink: 0,
                            background: '#3b82f6',
                            animation: 'hubPulse 1.5s infinite',
                        }} />
                        <span style={{
                            fontSize: '11px', color: 'rgba(255,255,255,0.8)',
                            flex: 1, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap',
                        }}>
                            {agent.prompt.slice(0, 60)}{agent.prompt.length > 60 ? '...' : ''}
                        </span>
                        <span style={{ fontSize: '9px', color: 'rgba(255,255,255,0.3)' }}>
                            {formatDuration(agent.startedAt)}
                        </span>
                    </div>
                    {/* Progress bar */}
                    <div style={{
                        height: '2px', marginTop: '4px', borderRadius: '1px',
                        background: 'rgba(255,255,255,0.06)',
                        overflow: 'hidden',
                    }}>
                        <div style={{
                            height: '100%', borderRadius: '1px',
                            background: 'linear-gradient(90deg, #3b82f6, #60a5fa)',
                            width: '60%',
                            animation: 'progressPulse 2s ease-in-out infinite',
                        }} />
                    </div>
                </div>
            ))}

            {/* Completed agents (collapsed) */}
            {completed.length > 0 && (
                <div style={{ padding: '4px 10px' }}>
                    {completed.map(agent => {
                        const isExpanded = expandedId === agent.id;
                        const isError = agent.status === 'error';
                        return (
                            <div key={agent.id}>
                                <div
                                    onClick={() => setExpandedId(isExpanded ? null : agent.id)}
                                    style={{
                                        display: 'flex', alignItems: 'center', gap: '6px',
                                        padding: '4px 0', cursor: 'pointer',
                                        fontSize: '11px', color: 'rgba(255,255,255,0.5)',
                                    }}
                                >
                                    <i className={`codicon codicon-${isError ? 'error' : 'check'}`} style={{
                                        fontFamily: 'codicon', fontStyle: 'normal', fontSize: '11px',
                                        color: isError ? '#f85149' : '#34d399',
                                    }} />
                                    <span style={{
                                        flex: 1, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap',
                                    }}>
                                        {agent.prompt.slice(0, 50)}...
                                    </span>
                                    {agent.finishedAt && (
                                        <span style={{ fontSize: '9px', opacity: 0.3 }}>
                                            {formatDuration(agent.finishedAt - agent.startedAt)}
                                        </span>
                                    )}
                                    <i className={`codicon codicon-chevron-${isExpanded ? 'up' : 'down'}`} style={{
                                        fontFamily: 'codicon', fontStyle: 'normal', fontSize: '10px', opacity: 0.3,
                                    }} />
                                </div>
                                {isExpanded && (
                                    <div style={{
                                        padding: '4px 0 6px 18px', fontSize: '10px',
                                        color: isError ? '#f85149' : 'rgba(255,255,255,0.6)',
                                        lineHeight: 1.5, whiteSpace: 'pre-wrap',
                                        maxHeight: '120px', overflow: 'auto',
                                    }}>
                                        {agent.result || '(no output)'}
                                    </div>
                                )}
                            </div>
                        );
                    })}
                </div>
            )}
        </div>
    );
};

function formatDuration(ms: number): string {
    const secs = Math.floor(ms / 1000);
    if (secs < 60) return `${secs}s`;
    const mins = Math.floor(secs / 60);
    return `${mins}m ${secs % 60}s`;
}

export default BackgroundAgentJobs;
