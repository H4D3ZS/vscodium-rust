import React, { useCallback, useRef, useEffect, useState } from 'react';
import ActivityBar from './ActivityBar';
import { invoke } from '../tauri_bridge';
import Sidebar from './Sidebar';
import BottomPanel from './BottomPanel';
import RightSidebar from './RightSidebar';
import Editor from './Editor';
import SettingsPage from './SettingsPage';
import VisualLab from './visual/VisualLab';
import SpecsToCodeWizard from './SpecsToCodeWizard';
import { useStore } from '../store';
import { Sparkles, Zap, Bot, Globe, Layout as LayoutIcon } from 'lucide-react';

import BrowserSurface from './BrowserSurface';
import DiffViewer from './DiffViewer';
import { PlanningPanel } from './PlanningPanel';
import { GhostRuntimePanel } from './GhostRuntimePanel';
import { ThoughtProcess } from './ThoughtProcess';
import AgentDiffView from './agent/AgentDiffView';
import { AiriPanel } from './AiriPanel';
import { AiriOverlay } from './AiriOverlay';
import OllamaProgressBar from './OllamaProgressBar';
import { EmulatorPreview } from './EmulatorPreview';

function detectLanguageIcon(filename: string): { type: 'icon' | 'img'; value: string } {
    const ext = filename.split('.').pop()?.toLowerCase() ?? '';

    // Core Unbreakable SVGs
    const fileSvg = `data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9IiNhZGRmZmYiIHN0cm9rZS13aWR0aD0iMiIgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIiBzdHJva2UtbGluZWpvaW49InJvdW5kIj48cGF0aCBkPSJNMTMgM0g2YTIgMiAwIDAgMC0yIDJ2MTRhMiAyIDAgMCAwIDIgMmgxMmEyIDIgMCAwIDAgMi0yVjlsLTYtNnoiPjwvcGF0aD48cG9seWxpbmUgcG9pbnRzPSIxMyAzIDEzIDkgMTkgOSI+PC9wb2x5bGluZT48L3N2Zz4=`;
    const codeSvg = `data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9IiM3OWI4ZmYiIHN0cm9rZS13aWR0aD0iMiIgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIiBzdHJva2UtbGluZWpvaW49InJvdW5kIj48cG9seWxpbmUgcG9pbnRzPSIxNiAxOCAyMiAxMiAxNiA2Ii8+PHBvbHlsaW5lIHBvaW50cz0iOCA2IDIgMTIgOCAxOCIvPjwvc3ZnPg==`;

    const codeExts = ['rs', 'ts', 'tsx', 'js', 'jsx', 'c', 'cpp', 'py', 'go', 'java'];

    if (codeExts.includes(ext)) {
        return { type: 'img', value: codeSvg };
    }

    return { type: 'img', value: fileSvg };
}



import ComposerOverlay from './ComposerOverlay';
import TabStrip from './workbench/TabStrip';
import ToastManager from './ToastManager';
import DocumentOutline from './DocumentOutline';

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
    const [cursorSymbol, setCursorSymbol] = useState<string>('');
    const recentWorkspaces = useStore(state => state.recentWorkspaces);
    const removeRecentWorkspace = useStore(state => state.removeRecentWorkspace);

    // Dev Workflow State
    const isDevWorkflowActive = useStore(state => state.isDevWorkflowActive);
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
    }, [toggleSplitEditor]);

    // Breadcrumb: track cursor symbol via LSP document symbols
    const symbolCacheRef = useRef<{ path: string; symbols: any[] }>({ path: '', symbols: [] });
    useEffect(() => {
        const handler = async (e: Event) => {
            const { line } = (e as CustomEvent).detail;
            const store = useStore.getState();
            const path = store.activeEditorPath;
            if (!path) return;
            // Refresh symbol cache when file changes
            if (symbolCacheRef.current.path !== path) {
                symbolCacheRef.current = { path, symbols: [] };
                try {
                    const normalized = path.replace(/\\/g, '/');
                    const uri = normalized.startsWith('/') ? `file://${normalized}` : `file:///${normalized}`;
                    const res = await invoke<any[]>('lsp_document_symbols', { uri });
                    symbolCacheRef.current = { path, symbols: Array.isArray(res) ? res : [] };
                } catch { /* LSP may not be running */ }
            }
            // Find innermost symbol containing cursor line
            const findSymbol = (syms: any[], ln: number): string => {
                for (const s of syms) {
                    const start = (s.range?.start?.line ?? s.location?.range?.start?.line ?? 0) + 1;
                    const end = (s.range?.end?.line ?? s.location?.range?.end?.line ?? 0) + 1;
                    if (ln >= start && ln <= end) {
                        const child = s.children ? findSymbol(s.children, ln) : '';
                        return child || s.name;
                    }
                }
                return '';
            };
            setCursorSymbol(findSymbol(symbolCacheRef.current.symbols, line));
        };
        window.addEventListener('editor:cursor-position', handler);
        return () => window.removeEventListener('editor:cursor-position', handler as any);
    }, []);

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

                                {/* Breadcrumbs */}
                                {hasOpenFile && (
                                    <div className="breadcrumbs" id="breadcrumbs">
                                        <img src={`data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9IiM3OWI4ZmYiIHN0cm9rZS13aWR0aD0iMiIgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIiBzdHJva2UtbGluZWpvaW49InJvdW5kIj48cGF0aCBkPSJNMjIgMTlhMiAyIDAgMCAxLTIgMkg0YTIgMiAwIDAgMS0yLTJWN2EyIDIgMCAwIDEgMi0yaDVsMiAyaDlhMiAyIDAgMCAxIDIgMnYxMHoiPjwvcGF0aD48L3N2Zz4=`} style={{ width: '14px', height: '14px', marginRight: '4px', opacity: 0.7 }} />
                                        <span className="breadcrumb-item" style={{ cursor: 'pointer' }}>{(tabs.find(t => t.id === activeTabId)?.path.split('/').slice(-2, -1)[0]) ?? (activeRootName || 'vscodium-rust')}</span>
                                        <i className="codicon codicon-chevron-right" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px', margin: '0 4px', opacity: 0.4 }} />
                                        {detectLanguageIcon(tabs.find(t => t.id === activeTabId)?.filename || '').type === 'img' ? (
                                            <img src={detectLanguageIcon(tabs.find(t => t.id === activeTabId)?.filename || '').value} style={{ width: '14px', height: '14px', marginRight: '4px', opacity: 0.6 }} />
                                        ) : (
                                            <i className={`${detectLanguageIcon(tabs.find(t => t.id === activeTabId)?.filename || '').value}`} style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '14px', marginRight: '4px', opacity: 0.6 }} />
                                        )}

                                        <span className="breadcrumb-item active" style={{ color: 'var(--vscode-tab-activeForeground)', fontWeight: 400 }}>
                                            {tabs.find(t => t.id === activeTabId)?.filename}
                                        </span>
                                        {cursorSymbol && (
                                            <>
                                                <i className="codicon codicon-chevron-right" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px', margin: '0 4px', opacity: 0.4 }} />
                                                <i className="codicon codicon-symbol-method" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px', marginRight: '4px', opacity: 0.6 }} />
                                                <span className="breadcrumb-item" style={{ color: 'var(--vscode-tab-activeForeground)', opacity: 0.75, fontStyle: 'italic' }}>
                                                    {cursorSymbol}
                                                </span>
                                            </>
                                        )}
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
                                                            VSCODIUM-RUST IDE <span style={{ fontSize: '10px', background: 'var(--terminator-accent)', color: 'white', padding: '2px 6px', borderRadius: '4px', verticalAlign: 'middle', marginLeft: '12px' }}>AIRI-CORE v0.2.0</span>
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
                                                        <div style={{ fontSize: '11px', opacity: 0.7, lineHeight: 1.2 }}>Integrated AIRI sentient core with full filesystem access.</div>
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

                                                            <a href="#" className="premium-card" style={{ display: 'flex', alignItems: 'center', padding: '12px 16px', background: 'rgba(0, 198, 255, 0.05)', border: '1px solid var(--terminator-accent)', borderRadius: '10px', textDecoration: 'none', transition: 'transform 0.2s, background 0.2s' }}
                                                                onMouseEnter={(e) => { e.currentTarget.style.background = 'rgba(0, 198, 255, 0.1)'; e.currentTarget.style.transform = 'translateY(-1px)'; }}
                                                                onMouseLeave={(e) => { e.currentTarget.style.background = 'rgba(0, 198, 255, 0.05)'; e.currentTarget.style.transform = 'translateY(0)'; }}
                                                                onClick={(e) => { e.preventDefault(); useStore.getState().setSpecsWizardOpen(true); }}>
                                                                <div className="premium-card-icon" style={{ width: '32px', height: '32px', borderRadius: '6px', background: 'var(--terminator-accent)', color: 'white', display: 'flex', alignItems: 'center', justifyContent: 'center', marginRight: '14px', fontSize: '16px' }}>
                                                                    <Sparkles size={16} />
                                                                </div>
                                                                <div className="premium-card-content">
                                                                    <span className="premium-card-title" style={{ display: 'block', fontSize: '14px', fontWeight: 600, color: 'var(--vscode-foreground)' }}>New AI Project...</span>
                                                                    <span className="premium-card-desc" style={{ fontSize: '12px', opacity: 0.5 }}>Specs-to-Code Pipeline</span>
                                                                </div>
                                                            </a>
                                                        </div>
                                                    </div>

                                                    <div className="welcome-recent-section" style={{ textAlign: 'left' }}>
                                                        <h3 className="section-title" style={{ fontSize: '10px', fontWeight: 600, textTransform: 'uppercase', letterSpacing: '1.2px', opacity: 0.5, marginBottom: '12px' }}>Recent Workspaces</h3>
                                                        {recentWorkspaces.length === 0 ? (
                                                            <div className="recent-empty-state" style={{ padding: '20px 16px', borderRadius: '10px', background: 'var(--vscode-editor-background)', border: '1px dashed var(--vscode-panel-border)', fontSize: '11px', opacity: 0.4, textAlign: 'center' }}>
                                                                <i className="codicon codicon-history" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '18px', display: 'block', marginBottom: '6px' }} />
                                                                No recent folders found
                                                            </div>
                                                        ) : (
                                                            <div style={{ display: 'grid', gap: '6px' }}>
                                                                {recentWorkspaces.map(ws => (
                                                                    <div key={ws.path}
                                                                        style={{ display: 'flex', alignItems: 'center', padding: '8px 12px', borderRadius: '8px', background: 'var(--vscode-sideBar-background)', border: '1px solid var(--vscode-panel-border)', gap: '10px', cursor: 'pointer', transition: 'background 0.15s' }}
                                                                        onMouseEnter={e => e.currentTarget.style.background = 'var(--vscode-list-hoverBackground)'}
                                                                        onMouseLeave={e => e.currentTarget.style.background = 'var(--vscode-sideBar-background)'}
                                                                        onClick={() => useStore.getState().setActiveRoot(ws.path)}
                                                                    >
                                                                        <i className="codicon codicon-folder" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '16px', color: 'var(--terminator-accent)', flexShrink: 0 }} />
                                                                        <div style={{ flex: 1, minWidth: 0 }}>
                                                                            <div style={{ fontWeight: 600, fontSize: '13px', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{ws.name}</div>
                                                                            <div style={{ fontSize: '10px', opacity: 0.4, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap', fontFamily: 'monospace' }}>{ws.path.replace(/\\/g, '/')}</div>
                                                                        </div>
                                                                        <i
                                                                            className="codicon codicon-close"
                                                                            style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px', opacity: 0.4, flexShrink: 0 }}
                                                                            title="Remove from recents"
                                                                            onClick={e => { e.stopPropagation(); removeRecentWorkspace(ws.path); }}
                                                                        />
                                                                    </div>
                                                                ))}
                                                            </div>
                                                        )}
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    ) : hasOpenFile ? (
                                        /* Monaco Editor or Settings Page */
                                        <div style={{ width: '100%', height: '100%', display: 'flex', flexDirection: (isVisualLabSplitView && isVisualLabOpen) ? 'row' : 'column' }}>
                                            {tabs.find(t => t.id === activeTabId)?.type === 'settings' ? (
                                                <SettingsPage />
                                            ) : (
                                                <div style={{ display: 'flex', flex: 1, width: '100%', height: '100%', minWidth: 0 }}>
                                                    {/* Primary editor */}
                                                    <div style={{
                                                        flex: (isVisualLabSplitView && isVisualLabOpen) ? '0 0 50%' : (isSplitEditorOpen ? '0 0 50%' : 1),
                                                        borderRight: (isVisualLabSplitView && isVisualLabOpen) || isSplitEditorOpen ? '1px solid var(--vscode-panel-border)' : 'none',
                                                        height: '100%',
                                                        minWidth: 0,
                                                        display: 'flex',
                                                        flexDirection: 'column'
                                                    }}>
                                                        <Editor />
                                                    </div>
                                                    {/* Visual Lab split */}
                                                    {(isVisualLabSplitView && isVisualLabOpen) && (
                                                        <div style={{ flex: '0 0 50%', height: '100%', minWidth: 0, background: '#090909' }}>
                                                            <VisualLab isInline={true} />
                                                        </div>
                                                    )}
                                                    {/* Emulator Preview (Dev Workflow) */}
                                                    {isDevWorkflowActive && (
                                                        <div style={{ flex: '0 0 50%', height: '100%', minWidth: 0, background: 'var(--vscode-editor-background)' }}>
                                                            <EmulatorPreview />
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
                                                                {splitEditorTabId && <Editor tabId={splitEditorTabId} />}
                                                            </div>
                                                        </div>
                                                    )}
                                                </div>
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
                ) : (
                    <BrowserSurface />
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
                        <RightSidebar />
                    </div>
                </div>
            </div>

            {!isVisualLabSplitView && <VisualLab />}
            <DocumentOutline />
            <AiriOverlay />
            <SpecsToCodeWizard />
            {useStore(state => state.pendingChanges).length > 0 && <DiffViewer />}
            <AgentDiffView />
            <ThoughtProcess />
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
                            <PlanningPanel />
                        </div>
                        <div style={{ height: '256px', pointerEvents: 'auto', boxShadow: '0 25px 50px rgba(0,0,0,0.5)', borderRadius: '16px', overflow: 'hidden' }}>
                            <GhostRuntimePanel />
                        </div>
                    </div>
                )}

            {/* Ollama Progress Bar */}
            <OllamaProgressBar />
            <ComposerOverlay />
            <ToastManager />
        </div >
    );
};

export default Workbench;
