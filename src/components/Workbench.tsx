import React, { useCallback, useRef, useEffect, lazy, Suspense } from 'react';
import ActivityBar from './ActivityBar';
import Sidebar from './Sidebar';
import BottomPanel from './BottomPanel';
import EmptyEditorWelcome from './EmptyEditorWelcome';
import OllamaProgressBar from './OllamaProgressBar';
import { useStore } from '../store';
import TabStrip from './workbench/TabStrip';
import ToastManager from './ToastManager';

const RightSidebar = lazy(() => import('./RightSidebar'));
const Editor = lazy(() => import('./Editor'));
const SettingsPage = lazy(() => import('./SettingsPage'));
const AimViewer = lazy(() => import('./AimViewer'));
const VisualLab = lazy(() => import('./visual/VisualLab'));
const SpecsToCodeWizard = lazy(() => import('./SpecsToCodeWizard'));
const BrowserSurface = lazy(() => import('./BrowserSurface'));
const DiffViewer = lazy(() => import('./DiffViewer'));
const PlanningPanel = lazy(() => import('./PlanningPanel').then(m => ({ default: m.PlanningPanel })));
const GhostRuntimePanel = lazy(() => import('./GhostRuntimePanel').then(m => ({ default: m.GhostRuntimePanel })));
const ThoughtProcess = lazy(() => import('./ThoughtProcess').then(m => ({ default: m.ThoughtProcess })));
const AiriOverlay = lazy(() => import('./AiriOverlay').then(m => ({ default: m.AiriOverlay })));
const UnifiedEmulatorPanel = lazy(() => import('./UnifiedEmulatorPanel'));
const EmulatorPreview = lazy(() => import('./EmulatorPreview').then(m => ({ default: m.EmulatorPreview })));
const ComposerOverlay = lazy(() => import('./ComposerOverlay'));
const DocumentOutline = lazy(() => import('./DocumentOutline'));

const PanelFallback = () => null;

const Workbench: React.FC = () => {
    const isSidebarOpen = useStore(state => state.isSidebarOpen);
    const isBottomPanelOpen = useStore(state => state.isBottomPanelOpen);
    const isRightSidebarOpen = useStore(state => state.isRightSidebarOpen);
    const sidebarWidth = useStore(state => state.sidebarWidth);
    const rightSidebarWidth = useStore(state => state.rightSidebarWidth);
    const bottomPanelHeight = useStore(state => state.bottomPanelHeight);

    // Panel state (from store)
    const isAiriPanelOpen = useStore(state => state.isAiriPanelOpen);
    const isEmulatorPanelOpen = useStore(state => state.isEmulatorPanelOpen);
    const emulatorLayout = useStore(state => state.emulatorLayout);

    const setSidebarWidth = useStore(state => state.setSidebarWidth);
    const setRightSidebarWidth = useStore(state => state.setRightSidebarWidth);
    const setBottomPanelHeight = useStore(state => state.setBottomPanelHeight);

    const tabs = useStore(state => state.tabs);
    const activeTabId = useStore(state => state.activeTabId);
    const layoutMode = useStore(state => state.layoutMode);
    const closeTab = useStore(state => state.closeTab);
    const setActiveTab = useStore(state => state.setActiveTab);
    const isVisualLabSplitView = useStore(state => state.isVisualLabSplitView);
    const isVisualLabOpen = useStore(state => state.isVisualLabOpen);
    const isSplitEditorOpen = useStore(state => state.isSplitEditorOpen);
    const splitEditorTabId = useStore(state => state.splitEditorTabId);
    const setSplitEditorTab = useStore(state => state.setSplitEditorTab);
    const toggleSplitEditor = useStore(state => state.toggleSplitEditor);
    const recentWorkspaces = useStore(state => state.recentWorkspaces);
    const removeRecentWorkspace = useStore(state => state.removeRecentWorkspace);

    // Dev Workflow State
    const isDevWorkflowActive = useStore(state => state.isDevWorkflowActive);
    const showLeftEmulatorDock = emulatorLayout === 'left' && isEmulatorPanelOpen && !isDevWorkflowActive;
    const isZenMode = useStore(state => (state as any).isZenMode ?? false);
    const toggleZenMode = useStore(state => (state as any).toggleZenMode);

    // Ctrl+\ = toggle split editor (global listener, works regardless of Monaco focus)
    // Ctrl+Shift+V = toggle the markdown side-by-side preview (VS Code parity)
    // Ctrl+K Z = toggle zen mode; Escape exits zen mode
    useEffect(() => {
        const handler = (e: KeyboardEvent) => {
            if (e.key === 'Escape' && isZenMode) {
                e.preventDefault();
                toggleZenMode();
                return;
            }
            if ((e.ctrlKey || e.metaKey) && e.key === '\\') {
                e.preventDefault();
                toggleSplitEditor();
            }
            if ((e.ctrlKey || e.metaKey) && e.shiftKey && (e.key === 'V' || e.key === 'v')) {
                e.preventDefault();
                const s = useStore.getState() as any;
                s.toggleMarkdownPreview?.();
            }
        };
        window.addEventListener('keydown', handler);
        return () => window.removeEventListener('keydown', handler);
    }, [toggleSplitEditor, isZenMode, toggleZenMode]);

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

    return (
        <div id="workbench" style={{ display: 'flex', flex: 1, height: '100%', minHeight: 0, overflow: 'hidden', position: 'relative' }}>
            {!isZenMode && <ActivityBar />}
            {!isZenMode && isSidebarOpen && <div style={{ width: sidebarWidth, flexShrink: 0, display: 'flex' }}><Sidebar /></div>}

            {!isZenMode && isSidebarOpen && (
                <div
                    className="resizer-v"
                    id="sidebar-resizer"
                    onMouseDown={() => startResizing('sidebar')}
                />
            )}

            <div className="main-content" style={{ display: 'flex', flexDirection: 'column', flex: 1, overflow: 'hidden', minWidth: 0 }}>
                {layoutMode === 'editor' ? (
                    <main className="editors-layout" id="editors-layout" style={{ display: 'flex', flex: 1, overflow: 'hidden', background: 'var(--vscode-editor-background)' }}>
                        {/* Primary Editor Group */}
                        <div className="editor-group active" id="group-1" style={{ display: 'flex', flex: 1, flexDirection: 'column', overflow: 'hidden', minWidth: 0 }}>
                            <div className="editor-main" style={{ display: 'flex', flexDirection: 'column', flex: 1, overflow: 'hidden', width: '100%', height: '100%' }}>
                                {/* Tab strip */}
                                <TabStrip />

                                <div className="editor-wrapper" style={{ position: 'relative', width: '100%', height: '100%', flex: 1, overflow: 'hidden' }}>
                                    {(!activeRoot && tabs.length === 0) ? (
                                        <div className="welcome-screen-container" style={{
                                            position: 'absolute', inset: 0, zIndex: 1,
                                            overflowY: 'auto', background: 'var(--vscode-editor-background)',
                                            display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center',
                                            padding: '40px 48px',
                                        }}>
                                            <div style={{ width: '100%', maxWidth: '720px' }}>
                                                <h1 style={{ fontSize: '26px', fontWeight: 600, margin: '0 0 8px', color: 'var(--vscode-foreground)' }}>
                                                    VSCodium-Rust
                                                </h1>
                                                <p style={{ fontSize: '13px', opacity: 0.55, margin: '0 0 28px', lineHeight: 1.5 }}>
                                                    Open a folder to start. Chat, Composer, and mobile emulators work like VS Code + Cursor.
                                                </p>
                                                <div style={{ display: 'grid', gap: '4px', marginBottom: '32px' }}>
                                                    {[
                                                        { label: 'Open Folder...', icon: 'codicon-folder-opened', cmd: 'explorer.openFolder' },
                                                        { label: 'Clone Repository...', icon: 'codicon-source-control', cmd: 'git.clone' },
                                                        { label: 'New File...', icon: 'codicon-new-file', cmd: 'explorer.newFile' },
                                                        { label: 'Open Chat...', icon: 'codicon-comment-discussion', cmd: 'workbench.action.openChat' },
                                                        { label: 'Mobile Emulators...', icon: 'codicon-device-mobile', cmd: 'workbench.action.openEmulators' },
                                                    ].map(item => (
                                                        <button key={item.cmd} type="button"
                                                            onClick={() => (window as any).executeCommand?.(item.cmd)}
                                                            style={{
                                                                display: 'flex', alignItems: 'center', gap: '10px',
                                                                padding: '8px 12px', border: 'none', borderRadius: '2px',
                                                                background: 'transparent', color: 'var(--vscode-textLink-foreground, #3794ff)',
                                                                fontSize: '13px', cursor: 'pointer', textAlign: 'left', width: '100%',
                                                            }}
                                                            onMouseEnter={e => { e.currentTarget.style.background = 'var(--vscode-list-hoverBackground)'; }}
                                                            onMouseLeave={e => { e.currentTarget.style.background = 'transparent'; }}
                                                        >
                                                            <i className={`codicon ${item.icon}`} style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '16px' }} />
                                                            {item.label}
                                                        </button>
                                                    ))}
                                                </div>
                                                {recentWorkspaces.length > 0 && (
                                                    <>
                                                        <h3 style={{ fontSize: '11px', fontWeight: 600, textTransform: 'uppercase', letterSpacing: '0.08em', opacity: 0.45, marginBottom: '8px' }}>Recent</h3>
                                                        <div style={{ display: 'grid', gap: '2px' }}>
                                                            {recentWorkspaces.map(ws => (
                                                                <div key={ws.path}
                                                                    style={{ display: 'flex', alignItems: 'center', padding: '6px 10px', borderRadius: '2px', gap: '8px', cursor: 'pointer' }}
                                                                    onMouseEnter={e => e.currentTarget.style.background = 'var(--vscode-list-hoverBackground)'}
                                                                    onMouseLeave={e => e.currentTarget.style.background = 'transparent'}
                                                                    onClick={() => useStore.getState().setActiveRoot(ws.path)}
                                                                >
                                                                    <i className="codicon codicon-folder" style={{ fontFamily: 'codicon', fontStyle: 'normal', opacity: 0.7 }} />
                                                                    <div style={{ flex: 1, minWidth: 0 }}>
                                                                        <div style={{ fontSize: '13px', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{ws.name}</div>
                                                                        <div style={{ fontSize: '11px', opacity: 0.4, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{ws.path}</div>
                                                                    </div>
                                                                </div>
                                                            ))}
                                                        </div>
                                                    </>
                                                )}
                                            </div>
                                        </div>
                                    ) : hasOpenFile ? (
                                        /* Monaco Editor or Settings Page */
                                        <div style={{ width: '100%', height: '100%', display: 'flex', flexDirection: (isVisualLabSplitView && isVisualLabOpen) ? 'row' : 'column' }}>
                                            {tabs.find(t => t.id === activeTabId)?.type === 'settings' ? (
                                                <Suspense fallback={<PanelFallback />}><SettingsPage /></Suspense>
                                            ) : (tabs.find(t => t.id === activeTabId) as any)?.type === 'aim' ? (
                                                <Suspense fallback={<PanelFallback />}><AimViewer path={(tabs.find(t => t.id === activeTabId) as any)?.path} /></Suspense>
                                            ) : (
                                                <div style={{ display: 'flex', flex: 1, width: '100%', height: '100%', minWidth: 0 }}>
                                                    {showLeftEmulatorDock && (
                                                        <div style={{
                                                            flex: '0 0 340px',
                                                            minWidth: 280,
                                                            maxWidth: 420,
                                                            height: '100%',
                                                            borderRight: '1px solid var(--vscode-panel-border)',
                                                            display: 'flex',
                                                            flexDirection: 'column',
                                                            background: 'var(--vscode-sideBar-background)',
                                                        }}>
                                                            <div style={{
                                                                padding: '6px 10px',
                                                                fontSize: '11px',
                                                                fontWeight: 600,
                                                                textTransform: 'uppercase',
                                                                letterSpacing: '0.05em',
                                                                borderBottom: '1px solid var(--vscode-panel-border)',
                                                                color: 'var(--vscode-sideBarTitle-foreground, var(--vscode-foreground))',
                                                                opacity: 0.85,
                                                            }}>
                                                                Mobile Emulators
                                                            </div>
                                                            <div style={{ flex: 1, minHeight: 0, overflow: 'hidden' }}>
                                                                <Suspense fallback={<PanelFallback />}><UnifiedEmulatorPanel /></Suspense>
                                                            </div>
                                                        </div>
                                                    )}
                                                    {/* Primary editor */}
                                                    <div style={{
                                                        flex: (isVisualLabSplitView && isVisualLabOpen) ? '0 0 50%' : (isSplitEditorOpen ? '0 0 50%' : 1),
                                                        borderRight: (isVisualLabSplitView && isVisualLabOpen) || isSplitEditorOpen ? '1px solid var(--vscode-panel-border)' : 'none',
                                                        height: '100%',
                                                        minWidth: 0,
                                                        display: 'flex',
                                                        flexDirection: 'column'
                                                    }}>
                                                        <Suspense fallback={<PanelFallback />}><Editor /></Suspense>
                                                    </div>
                                                    {/* Visual Lab split */}
                                                    {(isVisualLabSplitView && isVisualLabOpen) && (
                                                        <div style={{ flex: '0 0 50%', height: '100%', minWidth: 0, background: '#090909' }}>
                                                            <Suspense fallback={<PanelFallback />}><VisualLab isInline={true} /></Suspense>
                                                        </div>
                                                    )}
                                                    {/* Emulator Preview (Dev Workflow) */}
                                                    {isDevWorkflowActive && (
                                                        <div style={{ flex: '0 0 50%', height: '100%', minWidth: 0, background: 'var(--vscode-editor-background)' }}>
                                                            <Suspense fallback={<PanelFallback />}><EmulatorPreview /></Suspense>
                                                        </div>
                                                    )}
                                                    {/* Split editor pane (Ctrl+\) */}
                                                    {isSplitEditorOpen && !(isVisualLabSplitView && isVisualLabOpen) && (
                                                        <div style={{ flex: '0 0 50%', height: '100%', minWidth: 0, display: 'flex', flexDirection: 'column' }}>
                                                            {/* Split pane tab strip */}
                                                            <div className="tabs-row" style={{ flexShrink: 0 }}>
                                                                {tabs.map(tab => (
                                                                    <div
                                                                        key={tab.id}
                                                                        className={`tab${splitEditorTabId === tab.id ? ' active' : ''}`}
                                                                        onClick={() => setSplitEditorTab(tab.id)}
                                                                        title={tab.path}
                                                                        style={{ maxWidth: '150px' }}
                                                                    >
                                                                        <span className="tab-label">{tab.filename}</span>
                                                                    </div>
                                                                ))}
                                                                <div
                                                                    title="Close split"
                                                                    onClick={() => setSplitEditorTab(null)}
                                                                    style={{ marginLeft: 'auto', padding: '0 8px', cursor: 'pointer', display: 'flex', alignItems: 'center', opacity: 0.5, fontSize: '12px', flexShrink: 0 }}
                                                                >
                                                                    <i className="codicon codicon-close" style={{ fontFamily: 'codicon', fontStyle: 'normal' }} />
                                                                </div>
                                                            </div>
                                                            <div style={{ flex: 1, overflow: 'hidden' }}>
                                                                {splitEditorTabId && <Suspense fallback={<PanelFallback />}><Editor tabId={splitEditorTabId} /></Suspense>}
                                                            </div>
                                                        </div>
                                                    )}
                                                </div>
                                            )}
                                        </div>
                                    ) : (
                                        /* Native empty state — folder open, no file selected */
                                        <EmptyEditorWelcome />
                                    )}
                                </div>
                            </div>
                        </div>
                    </main>
                ) : (
                    <Suspense fallback={<PanelFallback />}><BrowserSurface /></Suspense>
                )}
                {!isZenMode && isBottomPanelOpen && (
                    <div
                        className="resizer-h"
                        id="panel-resizer"
                        onMouseDown={() => startResizing('panel')}
                    />
                )}
                {!isZenMode && (
                    <div style={{ height: isBottomPanelOpen ? bottomPanelHeight : 0, overflow: 'hidden', display: 'flex', flexDirection: 'column' }}>
                        <BottomPanel />
                    </div>
                )}
            </div>

            {/* Right Sidebar - Contains SEPARATE AIRI and Emulator panels */}
            <div
                className="right-sidebar-container"
                style={{
                    display: 'flex',
                    width: !isZenMode && isRightSidebarOpen ? `${rightSidebarWidth}px` : '0px',
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
                <div style={{
                    flex: 1,
                    minWidth: isRightSidebarOpen ? '350px' : '0',
                    overflow: 'hidden',
                    height: '100%',
                    display: 'flex',
                    flexDirection: 'column'
                }}>
                    {/* Panel content */}
                    <div style={{
                        flex: 1,
                        overflow: 'hidden',
                        display: 'flex',
                        flexDirection: 'column'
                    }}>
                        <Suspense fallback={<PanelFallback />}><RightSidebar /></Suspense>
                    </div>
                </div>
            </div>

            {!isVisualLabSplitView && (
                <Suspense fallback={<PanelFallback />}><VisualLab /></Suspense>
            )}
            <Suspense fallback={<PanelFallback />}><DocumentOutline /></Suspense>
            {localStorage.getItem('airi.companion') === '1' && (
                <Suspense fallback={<PanelFallback />}><AiriOverlay /></Suspense>
            )}
            <Suspense fallback={<PanelFallback />}><SpecsToCodeWizard /></Suspense>
            {useStore(state => state.pendingChanges).length > 0 && (
                <Suspense fallback={<PanelFallback />}><DiffViewer /></Suspense>
            )}
            <Suspense fallback={<PanelFallback />}><ThoughtProcess /></Suspense>
            {useStore(state => {
                const s = state.taskPlannerState?.state;
                return s === 'Planning' || s === 'Running' || s === 'Reviewing';
            }) && (
                    <div style={{
                        position: 'absolute',
                        top: '48px',
                        right: '16px',
                        bottom: '96px',
                        width: '320px',
                        display: 'flex',
                        flexDirection: 'column',
                        gap: '16px',
                        pointerEvents: 'none',
                        zIndex: 50,
                        overflow: 'hidden',
                    }}>
                        <div style={{ flex: 1, pointerEvents: 'auto', boxShadow: '0 25px 50px rgba(0,0,0,0.5)', borderRadius: '16px', overflow: 'hidden' }}>
                            <Suspense fallback={<PanelFallback />}><PlanningPanel /></Suspense>
                        </div>
                        <div style={{ height: '256px', pointerEvents: 'auto', boxShadow: '0 25px 50px rgba(0,0,0,0.5)', borderRadius: '16px', overflow: 'hidden' }}>
                            <Suspense fallback={<PanelFallback />}><GhostRuntimePanel /></Suspense>
                        </div>
                    </div>
                )}

            {/* Ollama Progress Bar */}
            <OllamaProgressBar />
            <Suspense fallback={<PanelFallback />}><ComposerOverlay /></Suspense>
            <ToastManager />
        </div >
    );
};

export default Workbench;
