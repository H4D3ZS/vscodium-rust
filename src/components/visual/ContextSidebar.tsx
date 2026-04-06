import React, { useEffect } from 'react';
import { useStore } from '../../store';
import { marked } from 'marked';
import NeuralSummaryView from './NeuralSummaryView';

const ContextSidebar: React.FC = () => {
    const activeFileContext = useStore(state => state.activeFileContext);
    const fetchFileContext = useStore(state => state.fetchFileContext);
    const activeTabId = useStore(state => state.activeTabId);
    const tabs = useStore(state => state.tabs);
    const activeRoot = useStore(state => state.activeRoot);
    const activeRootName = useStore(state => state.activeRootName);

    const activeTab = tabs.find(t => t.id === activeTabId);

    useEffect(() => {
        if (activeTab && activeTab.path) {
            fetchFileContext(activeTab.path);
        }
    }, [activeTab, fetchFileContext]);

    const displayName = activeTab ? activeTab.filename : 'No active file';

    return (
        <div className="context-sidebar-view" style={{
            display: 'flex',
            flexDirection: 'column',
            height: '100%',
            padding: '16px',
            background: 'var(--vscode-sideBar-background)',
            color: 'var(--vscode-sideBar-foreground)',
            overflowY: 'auto'
        }}>
            <div style={{ marginBottom: '24px' }}>
                <h3 style={{ fontSize: '11px', textTransform: 'uppercase', opacity: 0.5, marginBottom: '12px', letterSpacing: '0.08em', fontWeight: 700 }}>
                    Active Insight
                </h3>
                {activeTab ? (
                    <div style={{ padding: '12px', background: 'rgba(59, 130, 246, 0.05)', borderRadius: '8px', border: '1px solid rgba(59, 130, 246, 0.2)' }}>
                        <div style={{ fontSize: '13px', fontWeight: 600, display: 'flex', alignItems: 'center', gap: '8px' }}>
                            <i className="codicon codicon-record-keys" style={{ color: '#3b82f6' }}></i>
                            {displayName}
                        </div>
                        <div style={{ fontSize: '10px', opacity: 0.4, marginTop: '4px', wordBreak: 'break-all', fontFamily: 'var(--font-mono)' }}>
                            {activeTab.path}
                        </div>
                    </div>
                ) : activeRoot ? (
                    <div style={{ padding: '12px', background: 'rgba(168, 85, 247, 0.05)', borderRadius: '8px', border: '1px solid rgba(168, 85, 247, 0.2)' }}>
                        <div style={{ fontSize: '13px', fontWeight: 600, display: 'flex', alignItems: 'center', gap: '8px' }}>
                            <i className="codicon codicon-briefcase" style={{ color: '#a855f7' }}></i>
                            {activeRootName || 'Active Workspace'}
                        </div>
                        <div style={{ fontSize: '10px', opacity: 0.4, marginTop: '4px', wordBreak: 'break-all', fontFamily: 'var(--font-mono)' }}>
                            {activeRoot}
                        </div>
                    </div>
                ) : (
                    <div style={{ fontSize: '12px', opacity: 0.3, fontStyle: 'italic' }}>Select a file or open a folder to see context</div>
                )}
            </div>

            {/* If we have a project loaded, always show the graph section, even if specific file context is missing */}
            {(activeFileContext || activeRoot) && (
                <>
                    {/* SYMBOLS SECTION - Only if file is active */}
                    {activeFileContext && activeFileContext.symbols.length > 0 && (
                        <div style={{ marginBottom: '24px' }}>
                            <h3 style={{ fontSize: '10px', textTransform: 'uppercase', opacity: 0.5, marginBottom: '8px', letterSpacing: '0.05em' }}>
                                Detected Symbols ({activeFileContext.symbols.length})
                            </h3>
                            <div style={{ display: 'flex', flexWrap: 'wrap', gap: '6px' }}>
                                {activeFileContext.symbols.map(sym => (
                                    <div key={sym} style={{
                                        padding: '4px 8px',
                                        background: 'rgba(255,255,255,0.03)',
                                        borderRadius: '4px',
                                        border: '1px solid rgba(255,255,255,0.08)',
                                        fontSize: '11px',
                                        fontFamily: 'var(--font-mono)',
                                        color: '#61afef'
                                    }}>
                                        {sym}
                                    </div>
                                ))}
                            </div>
                        </div>
                    )}

                    <div style={{ marginBottom: '24px' }}>
                        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '12px' }}>
                            <h3 style={{ fontSize: '10px', textTransform: 'uppercase', opacity: 0.5, margin: 0, letterSpacing: '0.05em' }}>
                                Neural Context
                            </h3>
                        </div>
                        <div style={{ minHeight: '200px', maxHeight: '400px', overflow: 'hidden', position: 'relative', borderRadius: '8px', border: '1px solid rgba(255,255,255,0.05)', background: 'rgba(0,0,0,0.2)' }}>
                            <NeuralSummaryView />
                        </div>
                    </div>

                    {/* LESSONS SECTION - Only if we have them */}
                    {activeFileContext && activeFileContext.relevant_lessons.length > 0 && (
                        <div style={{ marginBottom: '16px' }}>
                            <h3 style={{ fontSize: '10px', textTransform: 'uppercase', opacity: 0.5, marginBottom: '12px', letterSpacing: '0.05em' }}>
                                Historical Insights
                            </h3>
                            <div style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
                                {activeFileContext.relevant_lessons.map(lesson => (
                                    <div key={lesson.id} style={{
                                        padding: '12px',
                                        background: 'rgba(245, 158, 11, 0.05)',
                                        borderRadius: '8px',
                                        border: '1px solid rgba(245, 158, 11, 0.2)',
                                        fontSize: '12px'
                                    }}>
                                        <div style={{ fontWeight: 700, color: '#f59e0b', marginBottom: '6px', fontSize: '10px', textTransform: 'uppercase' }}>
                                            Memory Locked
                                        </div>
                                        <div className="lesson-content" style={{ opacity: 0.9, lineHeight: '1.5' }}
                                            dangerouslySetInnerHTML={{ __html: marked.parse(lesson.content || "") as string }} />
                                    </div>
                                ))}
                            </div>
                        </div>
                    )}
                </>
            )}

            {!activeFileContext && !activeRoot && (
                <div style={{ flex: 1, display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', opacity: 0.2 }}>
                    <i className="codicon codicon-loading codicon-modifier-spin" style={{ fontSize: '32px', marginBottom: '16px' }}></i>
                    <div style={{ fontSize: '12px' }}>Connecting to Omni-Graph...</div>
                </div>
            )}
        </div>
    );
};

export default ContextSidebar;

