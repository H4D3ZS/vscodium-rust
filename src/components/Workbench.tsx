import React from 'react';
import ActivityBar from './ActivityBar';
import { invoke } from '../tauri_bridge';
import Sidebar from './Sidebar';
import BottomPanel from './BottomPanel';
import RightSidebar from './RightSidebar';
import Editor from './Editor';
import SettingsPage from './SettingsPage';
import { useStore } from '../store';

const Workbench: React.FC = () => {
    const isSidebarOpen = useStore(state => state.isSidebarOpen);
    const isBottomPanelOpen = useStore(state => state.isBottomPanelOpen);
    const isRightSidebarOpen = useStore(state => state.isRightSidebarOpen);
    const tabs = useStore(state => state.tabs);
    const activeTabId = useStore(state => state.activeTabId);
    const closeTab = useStore(state => state.closeTab);
    const setActiveTab = useStore(state => state.setActiveTab);

    const hasOpenFile = activeTabId !== null && tabs.length > 0;

    return (
        <div id="workbench">
            <ActivityBar />
            <Sidebar />

            {isSidebarOpen && <div className="resizer-v" id="sidebar-resizer" />}

            <div className="main-content" style={{ display: 'flex', flexDirection: 'column', flex: 1, overflow: 'hidden', minWidth: 0 }}>
                <main className="editors-layout" id="editors-layout" style={{ display: 'flex', flex: 1, overflow: 'hidden', background: 'var(--vscode-editor-background)' }}>
                    {/* Primary Editor Group */}
                    <div className="editor-group active" id="group-1" style={{ display: 'flex', flex: 1, flexDirection: 'column', overflow: 'hidden' }}>
                        <div className="editor-main">
                            {/* Tab strip */}
                            <div className="tabs-row">
                                {tabs.map(tab => (
                                    <div
                                        key={tab.id}
                                        className={`tab${tab.id === activeTabId ? ' active' : ''}`}
                                        onClick={() => setActiveTab(tab.id)}
                                        title={tab.path}
                                        style={{ display: 'flex', alignItems: 'center', cursor: 'pointer' }}
                                    >
                                        <span style={{ marginRight: '6px' }}>{tab.filename}</span>
                                        {tab.isModified && (
                                            <span style={{ width: 8, height: 8, borderRadius: '50%', background: 'var(--vscode-tab-activeForeground)', display: 'inline-block', marginRight: 4 }} />
                                        )}
                                        <i
                                            className="codicon codicon-close"
                                            style={{ marginLeft: tab.isModified ? 0 : '10px', fontSize: '14px', cursor: 'pointer', opacity: 0.7 }}
                                            onClick={(e) => { e.stopPropagation(); closeTab(tab.id); }}
                                        />
                                    </div>
                                ))}
                                {tabs.length === 0 && (
                                    <div className="tab active" style={{ display: 'flex', alignItems: 'center' }}>
                                        <i className="codicon codicon-markdown" style={{ marginRight: '6px', fontSize: '14px', color: '#1f9ceb' }} />
                                        Welcome
                                        <i className="codicon codicon-close" style={{ marginLeft: '10px', fontSize: '14px', cursor: 'pointer' }} />
                                    </div>
                                )}
                            </div>

                            {/* Breadcrumbs */}
                            <div className="breadcrumbs" id="breadcrumbs">
                                {hasOpenFile ? (
                                    <>
                                        <span style={{ padding: '0 4px' }}>{tabs.find(t => t.id === activeTabId)?.path.split('/').slice(-2, -1)[0] ?? 'VSCodium Rust'}</span>
                                        <i className="codicon codicon-chevron-right" style={{ fontSize: '12px', opacity: 0.6 }} />
                                        <span style={{ padding: '0 4px', color: 'var(--vscode-tab-activeForeground)' }}>
                                            {tabs.find(t => t.id === activeTabId)?.filename}
                                        </span>
                                    </>
                                ) : (
                                    <>
                                        <span style={{ padding: '0 4px' }}>VSCodium Rust</span>
                                        <i className="codicon codicon-chevron-right" style={{ fontSize: '12px', opacity: 0.6 }} />
                                        <span style={{ padding: '0 4px', color: 'var(--vscode-tab-activeForeground)' }}>Welcome</span>
                                    </>
                                )}
                            </div>

                            <div className="editor-wrapper" style={{ position: 'relative', width: '100%', height: '100%', flex: 1 }}>
                                {/* Welcome screen when no file is open */}
                                {!hasOpenFile && (
                                    <div id="welcome-view" className="welcome-view" style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', width: '100%', height: '100%', background: 'var(--vscode-editor-background)' }}>
                                        <div className="welcome-logo" style={{ marginBottom: '20px', opacity: 0.7 }}>
                                            <svg width="120" height="120" viewBox="0 0 100 100" fill="none" xmlns="http://www.w3.org/2000/svg">
                                                <path d="M72.4 15.2L28.8 45.3L15.6 33.7L6.4 41.5L25.3 71.3L72.4 84.8C75.8 85.8 79.1 83.3 79.1 79.7V20.3C79.1 16.7 75.8 14.2 72.4 15.2Z" fill="#0065A9" />
                                                <path d="M72.4 15.2L28.8 45.3L25.3 71.3L72.4 84.8C75.8 85.8 79.1 83.3 79.1 79.7V20.3C79.1 16.7 75.8 14.2 72.4 15.2Z" fill="#007ACC" />
                                                <path d="M28.8 45.3L15.6 33.7L25.3 54.7L28.8 45.3Z" fill="#1F9CEB" />
                                            </svg>
                                        </div>
                                        <h1 className="welcome-title" style={{ fontSize: '32px', fontWeight: 300, marginBottom: '4px', color: 'var(--vscode-sideBar-foreground)' }}>Visual Studio Code</h1>
                                        <h2 style={{ fontSize: '18px', fontWeight: 300, color: 'var(--vscode-sideBar-foreground)', opacity: 0.6, marginBottom: '30px' }}>Editing evolved</h2>
                                        <div className="welcome-actions" style={{ display: 'flex', flexDirection: 'column', gap: '8px', alignItems: 'flex-start' }}>
                                            <span style={{ fontSize: '14px', color: 'var(--vscode-sideBar-foreground)', fontWeight: 400, marginBottom: '10px' }}>Start</span>
                                            <a href="#" className="welcome-link" id="welcome-new-file" 
                                               onClick={(e) => { e.preventDefault(); (window as any).executeCommand('explorer.newFile'); }}
                                               style={{ color: 'var(--vscode-focusBorder)', textDecoration: 'none', fontSize: '13px', display: 'flex', alignItems: 'center', cursor: 'pointer', marginBottom: '4px' }}>
                                                <i className="codicon codicon-new-file" style={{ marginRight: '8px', fontSize: '16px' }} />New File...
                                            </a>
                                            <a href="#" className="welcome-link" id="welcome-open-folder" 
                                               onClick={(e) => { e.preventDefault(); invoke('open_folder'); }}
                                               style={{ color: 'var(--vscode-focusBorder)', textDecoration: 'none', fontSize: '13px', display: 'flex', alignItems: 'center', cursor: 'pointer', marginBottom: '4px' }}>
                                                <i className="codicon codicon-folder-opened" style={{ marginRight: '8px', fontSize: '16px' }} />Open Folder...
                                            </a>
                                            <a href="#" className="welcome-link" 
                                               onClick={(e) => { e.preventDefault(); (window as any).executeCommand('git.clone'); }}
                                               style={{ color: 'var(--vscode-focusBorder)', textDecoration: 'none', fontSize: '13px', display: 'flex', alignItems: 'center', cursor: 'pointer' }}>
                                                <i className="codicon codicon-repo-clone" style={{ marginRight: '8px', fontSize: '16px' }} />Clone Git Repository...
                                            </a>
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

                {isBottomPanelOpen && <div className="resizer-h" id="panel-resizer" />}
                <BottomPanel />
            </div>

            {isRightSidebarOpen && <div className="resizer-v" id="right-sidebar-resizer" />}
            <RightSidebar />
        </div>
    );
};

export default Workbench;
