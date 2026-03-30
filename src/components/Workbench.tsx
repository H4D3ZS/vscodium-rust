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
        <div id="workbench">
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
                    <div className="editor-group active" id="group-1" style={{ display: 'flex', flex: 1, flexDirection: 'column', overflow: 'hidden' }}>
                        <div className="editor-main">
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
                                                        onClick={(e) => { e.stopPropagation(); closeTab(tab.id); }}
                                                    />
                                                )}
                                            </div>
                                        </div>
                                    );
                                })}
                            </div>

                            {/* Breadcrumbs */}
                            <div className="breadcrumbs" id="breadcrumbs">
                                {hasOpenFile ? (
                                    <>
                                        <i className="codicon codicon-folder" style={{ fontSize: '14px', marginRight: '4px', opacity: 0.6 }} />
                                        <span className="breadcrumb-item" style={{ cursor: 'pointer' }}>{(tabs.find(t => t.id === activeTabId)?.path.split('/').slice(-2, -1)[0]) ?? (activeRootName || 'vscodium-rust')}</span>
                                        <i className="codicon codicon-chevron-right" style={{ fontSize: '12px', margin: '0 4px', opacity: 0.4 }} />
                                        <i className={`codicon codicon-${detectLanguageIcon(tabs.find(t => t.id === activeTabId)?.filename || '')}`} style={{ fontSize: '14px', marginRight: '4px', opacity: 0.6 }} />
                                        <span className="breadcrumb-item active" style={{ color: 'var(--vscode-tab-activeForeground)', fontWeight: 400 }}>
                                            {tabs.find(t => t.id === activeTabId)?.filename}
                                        </span>
                                    </>
                                ) : (
                                    <>
                                        <i className="codicon codicon-folder" style={{ fontSize: '14px', marginRight: '4px', opacity: 0.6 }} />
                                        <span className="breadcrumb-item">{activeRootName || 'vscodium-rust'}</span>
                                        <i className="codicon codicon-chevron-right" style={{ fontSize: '12px', margin: '0 4px', opacity: 0.4 }} />
                                        <span className="breadcrumb-item active" style={{ color: 'var(--vscode-tab-activeForeground)' }}>Welcome</span>
                                    </>
                                )}
                            </div>

                            <div className="editor-wrapper" style={{ position: 'relative', width: '100%', height: '100%', flex: 1 }}>
                                {/* Welcome screen when no root is open and no tabs */}
                                {!activeRoot && tabs.length === 0 && (
                                    <div id="welcome-view" className="welcome-container">
                                        <div className="premium-glow" style={{ top: '-100px', right: '-100px', background: 'radial-gradient(circle, rgba(0, 114, 255, 0.2) 0%, transparent 70%)' }}></div>
                                        <div className="premium-glow" style={{ bottom: '-100px', left: '-100px' }}></div>

                                        <div className="welcome-hero">
                                            <div className="welcome-badge">Community Edition</div>
                                            <div className="welcome-logo" style={{ marginBottom: '32px' }}>
                                                <svg width="80" height="80" viewBox="0 0 100 100" fill="none" xmlns="http://www.w3.org/2000/svg">
                                                    <path d="M72.4 15.2L28.8 45.3L15.6 33.7L6.4 41.5L25.3 71.3L72.4 84.8C75.8 85.8 79.1 83.3 79.1 79.7V20.3C79.1 16.7 75.8 14.2 72.4 15.2Z" fill="#00C6FF" />
                                                    <path d="M28.8 45.3L15.6 33.7L25.3 54.7L28.8 45.3Z" fill="#0072FF" />
                                                </svg>
                                            </div>
                                            <h1 className="welcome-title" style={{ fontSize: '48px', fontWeight: 800, marginBottom: '12px', letterSpacing: '-1.5px', color: 'var(--vscode-foreground)' }}>
                                                VSCodium <span className="premium-gradient-text">Rust</span>
                                            </h1>
                                            <h2 style={{ fontSize: '20px', fontWeight: 300, opacity: 0.6, marginBottom: '0', maxWidth: '500px', lineHeight: '1.5' }}>
                                                The high-performance, open-source IDE built for the modern developer.
                                            </h2>
                                        </div>

                                        <div className="welcome-content-wrapper">
                                            <div className="welcome-main-section">
                                                <h3 className="section-title">Get Started</h3>

                                                <div className="premium-cards-grid">
                                                    <a href="#" className="premium-card" onClick={(e) => { e.preventDefault(); (window as any).executeCommand('explorer.newFile'); }}>
                                                        <div className="premium-card-icon">
                                                            <i className="codicon codicon-new-file" />
                                                        </div>
                                                        <div className="premium-card-content">
                                                            <span className="premium-card-title">New File...</span>
                                                            <span className="premium-card-desc">Create a new workspace member</span>
                                                        </div>
                                                    </a>

                                                    <a href="#" className="premium-card" onClick={(e) => { e.preventDefault(); (window as any).executeCommand('explorer.openFolder'); }}>
                                                        <div className="premium-card-icon">
                                                            <i className="codicon codicon-folder-opened" />
                                                        </div>
                                                        <div className="premium-card-content">
                                                            <span className="premium-card-title">Open Folder...</span>
                                                            <span className="premium-card-desc">Open an existing project folder</span>
                                                        </div>
                                                    </a>

                                                    <a href="#" className="premium-card" onClick={(e) => { e.preventDefault(); (window as any).executeCommand('git.clone'); }}>
                                                        <div className="premium-card-icon">
                                                            <i className="codicon codicon-source-control" />
                                                        </div>
                                                        <div className="premium-card-content">
                                                            <span className="premium-card-title">Clone Repository...</span>
                                                            <span className="premium-card-desc">Download project from a remote host</span>
                                                        </div>
                                                    </a>
                                                </div>
                                            </div>

                                            <div className="welcome-recent-section">
                                                <h3 className="section-title">Recent</h3>
                                                <div className="recent-empty-state">No recent folders found</div>
                                            </div>
                                        </div>
                                    </div>
                                )}

                                {/* Monaco Editor or Settings Page */}
                                {hasOpenFile && (
                                    <div style={{ width: '100%', height: '100%' }}>
                                        {tabs.find(t => t.id === activeTabId)?.type === 'settings' ? (
                                            <SettingsPage />
                                        ) : (
                                            <Editor />
                                        )}
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
        </div>
    );
};

export default Workbench;
