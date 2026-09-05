import React from 'react';
import { FileJson, Database } from 'lucide-react';
import { useStore } from '../../store';
import type { EditorTab, PendingChange } from '../../store';

interface EditorToolbarProps {
    activeTab: EditorTab;
    activeFilePendingChange: PendingChange | undefined;
}

const EditorToolbar: React.FC<EditorToolbarProps> = ({ activeTab, activeFilePendingChange }) => {
    const setVisualLabData = useStore(state => state.setVisualLabData);
    const toggleVisualLab = useStore(state => state.toggleVisualLab);
    const setVisualLabMode = useStore(state => state.setVisualLabMode);
    const acceptPendingChange = useStore(state => state.acceptPendingChange);
    const rejectPendingChange = useStore(state => state.rejectPendingChange);

    const isSql = activeTab.path.endsWith('.sql');
    const isMongo = activeTab.path.endsWith('.mongodb');
    const showVisualLab = activeTab.language === 'json' || activeTab.path.endsWith('.json') || isSql || isMongo;

    const openVisualizer = () => {
        setVisualLabData(activeTab.content);
        setVisualLabMode(isSql ? 'erd' : 'json');
        toggleVisualLab(true);
    };

    return (
        <>
            {/* Visual Lab Quick Action */}
            {showVisualLab && (
                <button
                    onClick={openVisualizer}
                    style={{
                        position: 'absolute', top: '10px', right: '40px', zIndex: 100,
                        background: isSql ? 'rgba(16,185,129,0.15)' : 'rgba(168,85,247,0.15)',
                        border: `1px solid ${isSql ? 'rgba(16,185,129,0.3)' : 'rgba(168,85,247,0.3)'}`,
                        borderRadius: '6px',
                        color: isSql ? '#10b981' : '#c084fc',
                        padding: '4px 10px', fontSize: '11px', fontWeight: 600,
                        display: 'flex', alignItems: 'center', gap: '6px',
                        cursor: 'pointer', backdropFilter: 'blur(4px)', transition: 'all 0.2s',
                    }}
                >
                    {isSql ? <Database size={12} /> : <FileJson size={12} />}
                    {isSql ? 'Visualize Schema' : 'Visualize Content'}
                </button>
            )}

            {/* Diff / pending-change accept-reject banner */}
            {activeFilePendingChange && (
                <div style={{
                    position: 'absolute', top: '40px', left: '50%', transform: 'translateX(-50%)',
                    zIndex: 2000, display: 'flex', alignItems: 'center', gap: '16px',
                    padding: '10px 20px', borderRadius: '12px',
                    background: 'rgba(18,18,29,0.82)', backdropFilter: 'blur(16px)',
                    border: '1px solid rgba(0,198,255,0.35)',
                    boxShadow: '0 8px 32px rgba(0,0,0,0.5), 0 0 20px rgba(0,198,255,0.2)',
                    color: 'var(--vscode-editor-foreground, #ffffff)', fontSize: '12px', fontFamily: 'var(--font-ui, sans-serif)',
                    animation: 'bannerSlideIn 0.35s cubic-bezier(0.16,1,0.3,1)',
                }}>
                    <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
                        <span style={{
                            width: '24px', height: '24px', borderRadius: '50%',
                            background: 'rgba(0,198,255,0.18)', color: '#00c6ff',
                            fontSize: '13px', display: 'flex', alignItems: 'center', justifyContent: 'center',
                        }}></span>
                        <span style={{ fontWeight: 500 }}>
                            AI edits for <span style={{ color: '#00c6ff', fontWeight: 600 }}>{activeTab.filename}</span> are pending review
                        </span>
                    </div>
                    <div style={{ display: 'flex', gap: '8px', alignItems: 'center' }}>
                        <button
                            onClick={() => acceptPendingChange(activeFilePendingChange.id)}
                            style={{
                                padding: '6px 14px', borderRadius: '6px', border: 'none',
                                background: 'linear-gradient(135deg,#00c6ff 0%,#0072ff 100%)',
                                color: 'var(--vscode-editor-foreground, #fff)', fontSize: '11px', fontWeight: 700, cursor: 'pointer',
                            }}
                        >
                            Accept
                        </button>
                        <button
                            onClick={() => rejectPendingChange(activeFilePendingChange.id)}
                            style={{
                                padding: '6px 14px', borderRadius: '6px',
                                border: '1px solid rgba(255,255,255,0.15)',
                                background: 'rgba(255,255,255,0.06)',
                                color: 'rgba(255,255,255,0.7)', fontSize: '11px', fontWeight: 600, cursor: 'pointer',
                            }}
                        >
                            Reject
                        </button>
                    </div>
                </div>
            )}
        </>
    );
};

export default EditorToolbar;
