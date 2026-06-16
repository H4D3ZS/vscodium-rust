/**
 * HistoryPanel — Chat history viewer with session restore.
 * Extracted from RightSidebar.tsx (A2 decomposition).
 */
import React, { useEffect } from 'react';
import { useStore } from '../../store';

const HistoryPanel: React.FC = () => {
    const chatSessions = useStore(s => s.chatSessions);
    const refreshChatSessions = useStore(s => s.refreshChatSessions);
    const loadChatSession = useStore(s => s.loadChatSession);
    const archiveCurrentSession = useStore(s => s.archiveCurrentSession);

    useEffect(() => { refreshChatSessions(); }, [refreshChatSessions]);

    const restore = (path: string) => {
        void loadChatSession(path);
    };

    return (
        <div className="right-sidebar-scroll" style={{ padding: '8px 16px 16px', gap: '12px', justifyContent: 'flex-start', alignItems: 'stretch' }}>
            {/* Header */}
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '4px', gap: 8 }}>
                <div>
                    <div style={{ fontSize: '11px', fontWeight: 600, textTransform: 'uppercase', opacity: 0.5 }}>Conversation history</div>
                    <div style={{ fontSize: 10, opacity: 0.4, marginTop: 2 }}>Click any session to restore the full chat.</div>
                </div>
                <button
                    type="button"
                    onClick={() => { void archiveCurrentSession?.(); refreshChatSessions(); }}
                    title="Save current chat as a named archive"
                    style={{
                        padding: '4px 8px', fontSize: 10, borderRadius: 4, cursor: 'pointer',
                        border: '1px solid rgba(255,255,255,0.12)', background: 'rgba(255,255,255,0.04)',
                        color: 'inherit',
                    }}
                >
                    Archive current
                </button>
            </div>

            {/* Session list */}
            {chatSessions.length === 0 ? (
                <div style={{ padding: '20px', textAlign: 'center', opacity: 0.5, fontSize: '12px', lineHeight: 1.5 }}>
                    No conversations yet.<br />
                    Send a message in Chat — it appears here automatically.
                </div>
            ) : (
                <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                    {chatSessions.map((session: any) => {
                        const title = session.title || String(session.name || '').replace('session_', 'Chat ') || 'Conversation';
                        const preview = session.preview || '';
                        const ts = session.updated_at ? new Date(session.updated_at * 1000).toLocaleString() : '';
                        const isCurrent = !!session.is_current;

                        return (
                            <div
                                key={session.path}
                                role="button"
                                tabIndex={0}
                                onClick={() => restore(session.path)}
                                onKeyDown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); restore(session.path); } }}
                                style={{
                                    padding: '10px 12px', borderRadius: '8px', cursor: 'pointer',
                                    background: isCurrent ? 'rgba(0,122,204,0.12)' : 'rgba(255,255,255,0.03)',
                                    border: isCurrent ? '1px solid rgba(0,122,204,0.35)' : '1px solid rgba(255,255,255,0.05)',
                                    transition: 'background 0.2s',
                                }}
                                onMouseEnter={(e) => e.currentTarget.style.background = isCurrent ? 'rgba(0,122,204,0.18)' : 'rgba(255,255,255,0.06)'}
                                onMouseLeave={(e) => e.currentTarget.style.background = isCurrent ? 'rgba(0,122,204,0.12)' : 'rgba(255,255,255,0.03)'}
                            >
                                <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '4px', gap: 8, alignItems: 'center' }}>
                                    <span style={{ fontSize: '12px', fontWeight: 600, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{title}</span>
                                    {isCurrent && (
                                        <span style={{ fontSize: 9, padding: '1px 6px', borderRadius: 4, background: 'rgba(0,122,204,0.25)', color: '#7ec8ff', flexShrink: 0 }}>Live</span>
                                    )}
                                    <span style={{ fontSize: '10px', opacity: 0.4, flexShrink: 0 }}>{session.messages} msgs</span>
                                </div>
                                {preview && (
                                    <div style={{ fontSize: 11, opacity: 0.55, marginBottom: 6, lineHeight: 1.4, overflow: 'hidden', display: '-webkit-box', WebkitLineClamp: 2, WebkitBoxOrient: 'vertical' as any }}>
                                        {preview}
                                    </div>
                                )}
                                <div style={{ fontSize: '10px', opacity: 0.45, display: 'flex', alignItems: 'center', gap: 4 }}>
                                    <i className="codicon codicon-comment-discussion" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: 10 }} />{ts}
                                </div>
                            </div>
                        );
                    })}
                </div>
            )}

            <div style={{ marginTop: 16, paddingTop: 12, borderTop: '1px solid rgba(255,255,255,0.06)', fontSize: 10, opacity: 0.4, lineHeight: 1.5 }}>
                Code restore points (git checkpoints) live in Source Control — not here.
            </div>
        </div>
    );
};

export default HistoryPanel;
