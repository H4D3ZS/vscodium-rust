import React, { useCallback, useRef } from 'react';
import ActivityBar from './ActivityBar';
import { invoke } from '../tauri_bridge';
import Sidebar from './Sidebar';
import BottomPanel from './BottomPanel';
import RightSidebar from './RightSidebar';
import Editor from './Editor';
import SettingsPage from './SettingsPage';
import { useStore } from '../store';

function detectLanguageIcon(filename: string): string {
    const ext = filename.split('.').pop()?.toLowerCase() ?? '';
    const map: Record<string, string> = {
        rs: 'rust', ts: 'typescript', tsx: 'react', js: 'javascript',
        jsx: 'react', json: 'json', css: 'css', html: 'html',
        md: 'markdown', toml: 'settings', yaml: 'symbol-method', yml: 'symbol-method',
    };
    return map[ext] ?? 'file';
}

const Workbench: React.FC = () => {
    const isSidebarOpen = useStore(state => state.isSidebarOpen);
    const isBottomPanelOpen = useStore(state => state.isBottomPanelOpen);
    const isRightSidebarOpen = useStore(state => state.isRightSidebarOpen);
    const sidebarWidth = useStore(state => state.sidebarWidth);
    const rightSidebarWidth = useStore(state => state.rightSidebarWidth);
    const bottomPanelHeight = useStore(state => state.bottomPanelHeight);

    const setSidebarWidth = useStore(state => state.setSidebarWidth);
    const setRightSidebarWidth = useStore(state => state.setRightSidebarWidth);
    const setBottomPanelHeight = useStore(state => state.setBottomPanelHeight);

    const tabs = useStore(state => state.tabs);
    const activeTabId = useStore(state => state.activeTabId);
    const closeTab = useStore(state => state.closeTab);
    const setActiveTab = useStore(state => state.setActiveTab);

    const resizingRef = useRef<'sidebar' | 'right-sidebar' | 'panel' | null>(null);

    const startResizing = useCallback((type: 'sidebar' | 'right-sidebar' | 'panel') => {
        resizingRef.current = type;
        document.body.style.cursor = type === 'panel' ? 'row-resize' : 'col-resize';
        document.body.classList.add('resizing');

        const onMouseMove = (e: MouseEvent) => {
            if (resizingRef.current === 'sidebar') {
                const newWidth = Math.max(160, Math.min(600, e.clientX - 48)); // 48 is activity bar width
                setSidebarWidth(newWidth);
            } else if (resizingRef.current === 'right-sidebar') {
                const newWidth = Math.max(200, Math.min(800, window.innerWidth - e.clientX));
                setRightSidebarWidth(newWidth);
            } else if (resizingRef.current === 'panel') {
                const newHeight = Math.max(100, Math.min(window.innerHeight - 100, window.innerHeight - e.clientY - 22)); // 22 is status bar height
                setBottomPanelHeight(newHeight);
            }
        };

        const onMouseUp = () => {
            resizingRef.current = null;
            document.body.style.cursor = '';
            document.body.classList.remove('resizing');
            window.removeEventListener('mousemove', onMouseMove);
            window.removeEventListener('mouseup', onMouseUp);
        };

        window.addEventListener('mousemove', onMouseMove);
        window.addEventListener('mouseup', onMouseUp);
    }, [setSidebarWidth, setRightSidebarWidth, setBottomPanelHeight]);

    const hasOpenFile = activeTabId !== null && tabs.length > 0;

    const activeRoot = useStore(state => state.activeRoot);
    const activeRootName = useStore(state => state.activeRootName);

    return (
        <div id="workbench" style={{ display: 'flex', flex: 1, height: '100%', minHeight: 0, overflow: 'hidden' }}>
            <ActivityBar />
            {isSidebarOpen && <div style={{ width: sidebarWidth, flexShrink: 0, display: 'flex' }}><Sidebar /></div>}

            {isSidebarOpen && (
                <div
                    className="resizer-v"
                    id="sidebar-resizer"
                    onMouseDown={() => startResizing('sidebar')}
                />
            )}

            <div className="main-content" style={{ display: 'flex', flexDirection: 'column', flex: 1, overflow: 'hidden', minWidth: 0 }}>
                <main className="editors-layout" id="editors-layout" style={{ display: 'flex', flex: 1, overflow: 'hidden', background: 'var(--vscode-editor-background)' }}>
                    {/* Primary Editor Group */}
                    <div className="editor-group active" id="group-1" style={{ display: 'flex', flex: 1, flexDirection: 'column', overflow: 'hidden', minWidth: 0 }}>
                        <div className="editor-main" style={{ display: 'flex', flexDirection: 'column', flex: 1, overflow: 'hidden', width: '100%', height: '100%' }}>
                            {/* Tab strip */}
                            <div className="tabs-row">
                                {tabs.map(tab => {
                                    const isActive = tab.id === activeTabId;
                                    const icon = detectLanguageIcon(tab.filename);
                                    return (
                                        <div
                                            key={tab.id}
                                            className={`tab${isActive ? ' active' : ''}`}
                                            onClick={() => setActiveTab(tab.id)}
                                            title={tab.path}
                                        >
                                            <i className={`codicon codicon-${icon} tab-icon`} style={{
                                                fontFamily: 'codicon',
                                                fontStyle: 'normal',
                                                fontSize: '14px',
                                                marginRight: '6px',
                                                color: isActive ? 'inherit' : 'var(--vscode-tab-activeForeground)',
                                                opacity: isActive ? 1 : 0.6
                                            }} />
                                            <span className="tab-label">{tab.filename}</span>
                                            <div className="tab-actions">
                                                {tab.isModified ? (
                                                    <span className="dirty-indicator" />
                                                ) : (
                                                    <i
                                                        className="codicon codicon-close"
                                                        style={{ fontFamily: 'codicon', fontStyle: 'normal' }}
                                                        onClick={(e) => { e.stopPropagation(); closeTab(tab.id); }}
                                                    />
                                                )}
                                            </div>
                                        </div>
                                    );
                                })}
                            </div>

                            {/* Breadcrumbs */}
                            {hasOpenFile && (
                                <div className="breadcrumbs" id="breadcrumbs">
                                    <i className="codicon codicon-folder" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '14px', marginRight: '4px', opacity: 0.6 }} />
                                    <span className="breadcrumb-item" style={{ cursor: 'pointer' }}>{(tabs.find(t => t.id === activeTabId)?.path.split('/').slice(-2, -1)[0]) ?? (activeRootName || 'vscodium-rust')}</span>
                                    <i className="codicon codicon-chevron-right" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px', margin: '0 4px', opacity: 0.4 }} />
                                    <i className={`codicon codicon-${detectLanguageIcon(tabs.find(t => t.id === activeTabId)?.filename || '')}`} style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '14px', marginRight: '4px', opacity: 0.6 }} />
                                    <span className="breadcrumb-item active" style={{ color: 'var(--vscode-tab-activeForeground)', fontWeight: 400 }}>
                                        {tabs.find(t => t.id === activeTabId)?.filename}
                                    </span>
                                </div>
                            )}

                            <div className="editor-wrapper" style={{ position: 'relative', width: '100%', height: '100%', flex: 1, overflow: 'hidden' }}>
                                {(!activeRoot && tabs.length === 0) ? (
                                    /* Welcome screen when no root is open and no tabs */
                                    <div className="welcome-screen-container" style={{
                                        position: 'absolute',
                                        inset: 0,
                                        display: 'flex',
                                        flexDirection: 'column',
                                        alignItems: 'flex-start',
                                        justifyContent: 'flex-start',
                                        zIndex: 1,
                                        overflowY: 'auto',
                                        background: 'var(--vscode-editor-background)'
                                    }}>
                                        <div className="welcome-view-content" style={{
                                            display: 'flex',
                                            flexDirection: 'column',
                                            flex: 1,
                                            height: '100%',
                                            alignItems: 'flex-start',
                                            justifyContent: 'flex-start',
                                            padding: '10px 48px 40px',
                                            width: '100%',
                                            textAlign: 'left',
                                            marginTop: 0
                                        }}>
                                            <div className="hero-section-beside" style={{ maxWidth: '900px', marginBottom: '2vh', width: '100%', textAlign: 'left', display: 'flex', flexDirection: 'row', alignItems: 'center', gap: '24px' }}>
                                                <img src="/assets/rust-logo.png" alt="Rust Logo" style={{ width: '80px', height: '80px', opacity: 0.9, flexShrink: 0 }} />
                                                <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'flex-start' }}>
                                                    <h1 style={{ fontSize: 'min(5vw, 42px)', fontWeight: 800, marginBottom: '2px', letterSpacing: '-1.5px', color: 'var(--vscode-foreground)', lineHeight: 1, margin: 0 }}>
                                                        TERMINATOR <span style={{ fontSize: '10px', background: 'var(--terminator-accent)', color: 'white', padding: '2px 6px', borderRadius: '4px', verticalAlign: 'middle', marginLeft: '12px' }}>v0.2.0-ELITE</span>
                                                    </h1>
                                                    <p style={{ fontSize: '14px', opacity: 0.6, maxWidth: '600px', margin: '8px 0 0', lineHeight: '1.4' }}>
                                                        The ultimate high-performance, native IDE optimized for speed, autonomy, and the future of software construction.
                                                    </p>
                                                </div>
                                            </div>

                                            <div className="pros-grid" style={{
                                                display: 'grid',
                                                gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))',
                                                gap: '12px',
                                                textAlign: 'left',
                                                marginBottom: '2vh',
                                                width: '100%',
                                                maxWidth: '900px'
                                            }}>
                                                <div className="pro-item" style={{ padding: '12px 16px', borderRadius: '8px', background: 'var(--vscode-sideBar-background)', border: '1px solid var(--vscode-panel-border)', boxShadow: '0 4px 15px rgba(0,0,0,0.1)' }}>
                                                    <div style={{ color: 'var(--terminator-accent)', marginBottom: '2px', fontSize: '14px', display: 'flex', alignItems: 'center', gap: '8px' }}><i className="codicon codicon-zap" style={{ fontFamily: 'codicon', fontStyle: 'normal' }} /> <strong>Performance</strong></div>
                                                    <div style={{ fontSize: '11px', opacity: 0.7, lineHeight: 1.2 }}>Zero-cost abstractions and Rust efficiency.</div>
                                                </div>
                                                <div className="pro-item" style={{ padding: '12px 16px', borderRadius: '8px', background: 'var(--vscode-sideBar-background)', border: '1px solid var(--vscode-panel-border)', boxShadow: '0 4px 15px rgba(0,0,0,0.1)' }}>
                                                    <div style={{ color: 'var(--terminator-success)', marginBottom: '2px', fontSize: '14px', display: 'flex', alignItems: 'center', gap: '8px' }}><i className="codicon codicon-shield" style={{ fontFamily: 'codicon', fontStyle: 'normal' }} /> <strong>Privacy</strong></div>
                                                    <div style={{ fontSize: '11px', opacity: 0.7, lineHeight: 1.2 }}>Local-first processing, safe and secure.</div>
                                                </div>
                                                <div className="pro-item" style={{ padding: '12px 16px', borderRadius: '8px', background: 'var(--vscode-sideBar-background)', border: '1px solid var(--vscode-panel-border)', boxShadow: '0 4px 15px rgba(0,0,0,0.1)' }}>
                                                    <div style={{ color: 'var(--terminator-accent)', marginBottom: '2px', fontSize: '14px', display: 'flex', alignItems: 'center', gap: '8px' }}><i className="codicon codicon-bot" style={{ fontFamily: 'codicon', fontStyle: 'normal' }} /> <strong>Autonomy</strong></div>
                                                    <div style={{ fontSize: '11px', opacity: 0.7, lineHeight: 1.2 }}>Integrated TERMINATOR agent with full filesystem access.</div>
                                                </div>
                                            </div>

                                            <div className="welcome-content-wrapper" style={{ width: '100%', maxWidth: '900px', display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '40px', textAlign: 'left' }}>
                                                <div className="welcome-main-section" style={{ textAlign: 'left' }}>
                                                    <h3 className="section-title" style={{ fontSize: '10px', fontWeight: 600, textTransform: 'uppercase', letterSpacing: '1.2px', opacity: 0.5, marginBottom: '12px' }}>Get Started</h3>

                                                    <div className="premium-cards-grid" style={{ display: 'grid', gap: '10px' }}>
                                                        <a href="#" className="premium-card" style={{ display: 'flex', alignItems: 'center', padding: '12px 16px', background: 'var(--vscode-sideBar-background)', border: '1px solid var(--vscode-panel-border)', borderRadius: '10px', textDecoration: 'none', transition: 'transform 0.2s, background 0.2s' }}
                                                            onMouseEnter={(e) => { e.currentTarget.style.background = 'var(--vscode-list-hoverBackground)'; e.currentTarget.style.transform = 'translateY(-1px)'; }}
                                                            onMouseLeave={(e) => { e.currentTarget.style.background = 'var(--vscode-sideBar-background)'; e.currentTarget.style.transform = 'translateY(0)'; }}
                                                            onClick={(e) => { e.preventDefault(); (window as any).executeCommand('explorer.newFile'); }}>
                                                            <div className="premium-card-icon" style={{ width: '32px', height: '32px', borderRadius: '6px', background: 'rgba(0, 198, 255, 0.1)', color: 'var(--terminator-accent)', display: 'flex', alignItems: 'center', justifyContent: 'center', marginRight: '14px', fontSize: '16px' }}>
                                                                <i className="codicon codicon-new-file" style={{ fontFamily: 'codicon', fontStyle: 'normal' }} />
                                                            </div>
                                                            <div className="premium-card-content">
                                                                <span className="premium-card-title" style={{ display: 'block', fontSize: '14px', fontWeight: 600, color: 'var(--vscode-foreground)' }}>New File...</span>
                                                                <span className="premium-card-desc" style={{ fontSize: '12px', opacity: 0.5 }}>Create in workspace</span>
                                                            </div>
                                                        </a>

                                                        <a href="#" className="premium-card" style={{ display: 'flex', alignItems: 'center', padding: '12px 16px', background: 'var(--vscode-sideBar-background)', border: '1px solid var(--vscode-panel-border)', borderRadius: '10px', textDecoration: 'none', transition: 'transform 0.2s, background 0.2s' }}
                                                            onMouseEnter={(e) => { e.currentTarget.style.background = 'var(--vscode-list-hoverBackground)'; e.currentTarget.style.transform = 'translateY(-1px)'; }}
                                                            onMouseLeave={(e) => { e.currentTarget.style.background = 'var(--vscode-sideBar-background)'; e.currentTarget.style.transform = 'translateY(0)'; }}
                                                            onClick={(e) => { e.preventDefault(); (window as any).executeCommand('explorer.openFolder'); }}>
                                                            <div className="premium-card-icon" style={{ width: '32px', height: '32px', borderRadius: '6px', background: 'rgba(0, 198, 255, 0.1)', color: 'var(--terminator-accent)', display: 'flex', alignItems: 'center', justifyContent: 'center', marginRight: '14px', fontSize: '16px' }}>
                                                                <i className="codicon codicon-folder-opened" style={{ fontFamily: 'codicon', fontStyle: 'normal' }} />
                                                            </div>
                                                            <div className="premium-card-content">
                                                                <span className="premium-card-title" style={{ display: 'block', fontSize: '14px', fontWeight: 600, color: 'var(--vscode-foreground)' }}>Open Folder...</span>
                                                                <span className="premium-card-desc" style={{ fontSize: '12px', opacity: 0.5 }}>Open from filesystem</span>
                                                            </div>
                                                        </a>

                                                        <a href="#" className="premium-card" style={{ display: 'flex', alignItems: 'center', padding: '12px 16px', background: 'var(--vscode-sideBar-background)', border: '1px solid var(--vscode-panel-border)', borderRadius: '10px', textDecoration: 'none', transition: 'transform 0.2s, background 0.2s' }}
                                                            onMouseEnter={(e) => { e.currentTarget.style.background = 'var(--vscode-list-hoverBackground)'; e.currentTarget.style.transform = 'translateY(-1px)'; }}
                                                            onMouseLeave={(e) => { e.currentTarget.style.background = 'var(--vscode-sideBar-background)'; e.currentTarget.style.transform = 'translateY(0)'; }}
                                                            onClick={(e) => { e.preventDefault(); (window as any).executeCommand('git.clone'); }}>
                                                            <div className="premium-card-icon" style={{ width: '32px', height: '32px', borderRadius: '6px', background: 'rgba(0, 198, 255, 0.1)', color: 'var(--terminator-accent)', display: 'flex', alignItems: 'center', justifyContent: 'center', marginRight: '14px', fontSize: '16px' }}>
                                                                <i className="codicon codicon-source-control" style={{ fontFamily: 'codicon', fontStyle: 'normal' }} />
                                                            </div>
                                                            <div className="premium-card-content">
                                                                <span className="premium-card-title" style={{ display: 'block', fontSize: '14px', fontWeight: 600, color: 'var(--vscode-foreground)' }}>Clone Repository...</span>
                                                                <span className="premium-card-desc" style={{ fontSize: '12px', opacity: 0.5 }}>Sync with Git</span>
                                                            </div>
                                                        </a>
                                                    </div>
                                                </div>

                                                <div className="welcome-recent-section" style={{ textAlign: 'left' }}>
                                                    <h3 className="section-title" style={{ fontSize: '10px', fontWeight: 600, textTransform: 'uppercase', letterSpacing: '1.2px', opacity: 0.5, marginBottom: '12px' }}>Recent Workspaces</h3>
                                                    <div className="recent-empty-state" style={{ padding: '20px 16px', borderRadius: '10px', background: 'var(--vscode-editor-background)', border: '1px dashed var(--vscode-panel-border)', fontSize: '11px', opacity: 0.4, textAlign: 'center' }}>
                                                        <i className="codicon codicon-history" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '18px', display: 'block', marginBottom: '6px' }} />
                                                        No recent folders found
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                ) : hasOpenFile ? (
                                    /* Monaco Editor or Settings Page */
                                    <div style={{ width: '100%', height: '100%', display: 'flex', flexDirection: 'column' }}>
                                        {tabs.find(t => t.id === activeTabId)?.type === 'settings' ? (
                                            <SettingsPage />
                                        ) : (
                                            <Editor />
                                        )}
                                    </div>
                                ) : (
                                    /* Fallback when a folder is open but no file is selected */
                                    <div style={{ flex: 1, display: 'flex', alignItems: 'center', justifyContent: 'center', opacity: 0.2 }}>
                                        <i className="codicon codicon-symbol-method" style={{ fontFamily: 'codicon', fontSize: '64px' }} />
                                    </div>
                                )}
                            </div>
                        </div>
                    </div>
                </main>
                {isBottomPanelOpen && (
                    <div
                        className="resizer-h"
                        id="panel-resizer"
                        onMouseDown={() => startResizing('panel')}
                    />
                )}
                <div style={{ height: isBottomPanelOpen ? bottomPanelHeight : 0, overflow: 'hidden', display: 'flex', flexDirection: 'column' }}>
                    <BottomPanel />
                </div>
            </div>

            <div
                className="right-sidebar-container"
                style={{
                    display: 'flex',
                    width: isRightSidebarOpen ? `${rightSidebarWidth}px` : '0px',
                    transition: 'width 0.2s cubic-bezier(0.4, 0, 0.2, 1)',
                    position: 'relative',
                    flexShrink: 0,
                    overflow: 'visible'
                }}
            >
                {isRightSidebarOpen && (
                    <div
                        className="resizer-v"
                        id="right-sidebar-resizer"
                        onMouseDown={() => startResizing('right-sidebar')}
                        style={{ position: 'absolute', left: 0, height: '100%' }}
                    />
                )}
                <div style={{ flex: 1, minWidth: isRightSidebarOpen ? '200px' : '0', overflow: 'hidden', height: '100%' }}>
                    <RightSidebar />
                </div>
            </div>
        </div >
    );
};

export default Workbench;
