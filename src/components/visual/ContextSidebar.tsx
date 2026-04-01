import React, { useEffect } from 'react';
import { useStore } from '../../store';

const ContextSidebar: React.FC = () => {
    const contextSlots = useStore(state => state.contextSlots);
    const fetchMemory = useStore(state => state.fetchWorkspaceMemory);
    const activeTabId = useStore(state => state.activeTabId);
    const tabs = useStore(state => state.tabs);

    const activeTab = tabs.find(t => t.id === activeTabId);

    useEffect(() => {
        // Fetch all categories for now to populate the sidebar
        fetchMemory('file_map');

        const interval = setInterval(() => {
            fetchMemory('file_map');
        }, 10000); // Sync every 10s

        return () => clearInterval(interval);
    }, [fetchMemory]);

    return (
        <div className="context-sidebar-view" style={{ display: 'flex', flexDirection: 'column', height: '100%', padding: '12px' }}>
            <div style={{ marginBottom: '16px' }}>
                <h3 style={{ fontSize: '11px', textTransform: 'uppercase', opacity: 0.5, marginBottom: '8px', letterSpacing: '0.05em' }}>Current Focus</h3>
                {activeTab ? (
                    <div style={{ padding: '8px', background: 'rgba(255,255,255,0.03)', borderRadius: '6px', border: '1px solid rgba(255,255,255,0.1)' }}>
                        <div style={{ fontSize: '12px', fontWeight: 600 }}>{activeTab.filename}</div>
                        <div style={{ fontSize: '10px', opacity: 0.5, marginTop: '2px', wordBreak: 'break-all' }}>{activeTab.path}</div>
                    </div>
                ) : (
                    <div style={{ fontSize: '11px', opacity: 0.3 }}>No active file</div>
                )}
            </div>

            <div style={{ flex: 1, overflowY: 'auto' }}>
                <h3 style={{ fontSize: '11px', textTransform: 'uppercase', opacity: 0.5, marginBottom: '8px', letterSpacing: '0.05em' }}>Omni-Context Map</h3>
                <div style={{ display: 'flex', flexDirection: 'column', gap: '4px' }}>
                    {contextSlots.length === 0 ? (
                        <div style={{ fontSize: '11px', opacity: 0.3, textAlign: 'center', padding: '20px' }}>
                            <i className="codicon codicon-sync codicon-modifier-spin" style={{ fontFamily: 'codicon', display: 'block', marginBottom: '8px', fontSize: '16px' }}></i>
                            Indexing workspace...
                        </div>
                    ) : (
                        contextSlots.map(slot => (
                            <div key={slot.id} style={{
                                padding: '6px 8px',
                                background: 'rgba(255,255,255,0.01)',
                                borderRadius: '4px',
                                border: '1px solid rgba(255,255,255,0.05)',
                                fontSize: '11px',
                                display: 'flex',
                                alignItems: 'center',
                                gap: '8px'
                            }}>
                                <i className="codicon codicon-file-code" style={{ fontFamily: 'codicon', opacity: 0.4 }}></i>
                                <span style={{ overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{slot.content}</span>
                            </div>
                        ))
                    )}
                </div>
            </div>

            <div style={{ marginTop: 'auto', paddingTop: '16px' }}>
                <div style={{ fontSize: '10px', opacity: 0.4, fontStyle: 'italic' }}>
                    Workspace Memory: {contextSlots.length} slots indexed
                </div>
            </div>
        </div>
    );
};

export default ContextSidebar;
