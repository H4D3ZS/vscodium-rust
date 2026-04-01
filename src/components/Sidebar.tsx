import React, { useState, useEffect, useMemo, useCallback } from 'react';
import { useStore, type FileEntry } from '../store';
import { invoke } from '@tauri-apps/api/core';
import { List } from 'react-window';
import ErrorBoundary from './ErrorBoundary';
import SearchView from './SearchView';
import ExtensionsView from './ExtensionsView';
import ScmView from './ScmView';
import DebugView from './DebugView';
import EmulatorPanel from './EmulatorPanel';
import AgentSettingsView from './AgentSettingsView';

interface FlattenedNode {
    entry: FileEntry;
    depth: number;
}

const flattenTree = (entries: FileEntry[], depth = 0, visited = new Set<string>()): FlattenedNode[] => {
    let result: FlattenedNode[] = [];
    for (const entry of entries) {
        if (visited.has(entry.path)) continue;
        visited.add(entry.path);

        result.push({ entry, depth });
        if (entry.is_expanded && entry.children) {
            result = [...result, ...flattenTree(entry.children, depth + 1, visited)];
        }
    }
    return result;
};

const TreeItemIcon: React.FC<{ icon: { type: 'img' | 'icon'; value: string } }> = ({ icon }) => {
    if (icon.type === 'img') {
        return <img src={icon.value} style={{ marginRight: '6px', width: '16px', height: '16px', opacity: 0.9, objectFit: 'contain' }} />;
    }
    // Force the codicon font via explicit style to avoid overrides in some environments
    return <i className={icon.value} style={{
        fontFamily: 'codicon',
        marginRight: '6px',
        fontSize: '16px',
        width: '16px',
        height: '16px',
        display: 'inline-flex',
        alignItems: 'center',
        justifyContent: 'center',
        textAlign: 'center',
        opacity: 0.85,
        fontStyle: 'normal',
        fontWeight: 'normal',
        lineHeight: 1,
        WebkitFontSmoothing: 'antialiased'
    }}></i>;
};

const FileTreeItem: React.FC<{ entry: FileEntry; depth: number; iconThemeMapping: any; style: React.CSSProperties }> = ({ entry, depth, iconThemeMapping, style }) => {
    const openFile = useStore(state => state.openFile);
    const toggleDirectory = useStore(state => state.toggleDirectory);
    const activeTabId = useStore(state => state.activeTabId);
    const tabs = useStore(state => state.tabs);
    const setContextMenuOpen = useStore(state => state.setContextMenuOpen);

    const isExpanded = entry.is_expanded ?? false;
    const isActive = tabs.find(t => t.id === activeTabId)?.path === entry.path;

    const getIcon = (): { type: 'img' | 'icon'; value: string } => {
        // FORCE SVG for Folders and Files to bypass ALL font issues
        const folderSvg = `data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9IiM3OWI4ZmYiIHN0cm9rZS13aWR0aD0iMiIgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIiBzdHJva2UtbGluZWpvaW49InJvdW5kIj48cGF0aCBkPSJNMjIgMTlhMiAyIDAgMCAxLTIgMkg0YTIgMiAwIDAgMS0yLTJWN2EyIDIgMCAwIDEgMi0yaDVsMiAyaDlhMiAyIDAgMCAxIDIgMnYxMHoiPjwvcGF0aD48L3N2Zz4=`;
        const fileSvg = `data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9IiNhZGRmZmYiIHN0cm9rZS13aWR0aD0iMiIgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIiBzdHJva2UtbGluZWpvaW49InJvdW5kIj48cGF0aCBkPSJNMTMgM0g2YTIgMiAwIDAgMC0yIDJ2MTRhMiAyIDAgMCAwIDIgMmgxMmEyIDIgMCAwIDAgMi0yVjlsLTYtNnoiPjwvcGF0aD48cG9seWxpbmUgcG9pbnRzPSIxMyAzIDEzIDkgMTkgOSI+PC9wb2x5bGluZT48L3N2Zz4=`;

        if (entry.is_dir) {
            return { type: 'img', value: folderSvg };
        }

        // Only use mapping for specific language icons, otherwise use our unbreakable file SVG
        if (iconThemeMapping) {
            const fileName = entry.name.toLowerCase();
            const ext = entry.name.split('.').pop()?.toLowerCase();
            let iconId = null;

            if (iconThemeMapping.fileNames && iconThemeMapping.fileNames[fileName]) {
                iconId = iconThemeMapping.fileNames[fileName];
            } else if (ext && iconThemeMapping.fileExtensions && iconThemeMapping.fileExtensions[ext]) {
                iconId = iconThemeMapping.fileExtensions[ext];
            }

            if (iconId && iconThemeMapping.iconDefinitions && iconThemeMapping.iconDefinitions[iconId]) {
                const def = iconThemeMapping.iconDefinitions[iconId];
                if (def.iconPath) {
                    const src = (window as any).__TAURI__?.core?.convertFileSrc
                        ? (window as any).__TAURI__.core.convertFileSrc(def.iconPath)
                        : `asset://localhost/${encodeURIComponent(def.iconPath)}`;
                    return { type: 'img', value: src };
                }
            }
        }

        return { type: 'img', value: fileSvg };
    };

    const icon = getIcon();

    const handleToggle = (e: React.MouseEvent) => {
        e.stopPropagation();
        if (entry.is_dir) {
            toggleDirectory(entry.path);
        } else {
            openFile(entry.path).catch(err => console.error(err));
        }
    };

    const handleContextMenu = (e: React.MouseEvent) => {
        e.preventDefault();
        e.stopPropagation();

        (window as any).__explorerContext = {
            path: entry.path,
            name: entry.name,
            isDir: entry.is_dir,
        };

        setContextMenuOpen(true, e.clientX, e.clientY);
    };

    return (
        <div
            className={`tree-row${isActive ? ' active' : ''}`}
            onClick={handleToggle}
            onContextMenu={handleContextMenu}
            draggable={!entry.is_dir}
            onDragStart={(e) => {
                if (!entry.is_dir) {
                    e.dataTransfer.setData('application/vscode-file', JSON.stringify({
                        path: entry.path,
                        name: entry.name,
                        type: 'file'
                    }));
                }
            }}
            style={{
                ...style,
                display: 'flex',
                alignItems: 'center',
                height: '22px',
                paddingLeft: `${depth * 8 + 8}px`,
                cursor: 'pointer',
                fontSize: '13px',
                color: 'var(--vscode-sideBar-foreground)',
                whiteSpace: 'nowrap',
                userSelect: 'none'
            }}
        >
            {entry.is_dir && (
                <i
                    className={`codicon codicon-${isExpanded ? 'chevron-down' : 'chevron-right'}`}
                    style={{
                        fontFamily: 'codicon',
                        fontStyle: 'normal',
                        fontWeight: 'normal',
                        marginRight: '4px',
                        fontSize: '12px',
                        width: '16px',
                        textAlign: 'center',
                        opacity: 0.6,
                        cursor: 'pointer',
                        lineHeight: 1,
                        WebkitFontSmoothing: 'antialiased'
                    }}
                    onClick={handleToggle}
                ></i>
            )}
            {!entry.is_dir && (
                <div style={{ width: '20px' }}></div>
            )}
            <TreeItemIcon icon={icon} />
            <span style={{
                flex: 1,
                overflow: 'hidden',
                textOverflow: 'ellipsis',
                color: isActive ? 'var(--vscode-list-activeSelectionForeground)' : 'inherit'
            }}>{entry.name}</span>
        </div>
    );
};

const VirtualizedFileTree: React.FC<{ entries: FileEntry[]; iconThemeMapping: any }> = ({ entries, iconThemeMapping }) => {
    const flattenedNodes = useMemo(() => flattenTree(entries), [entries]);
    const [containerHeight, setContainerHeight] = useState(600);
    const containerRef = React.useRef<HTMLDivElement>(null);

    useEffect(() => {
        if (!containerRef.current) return;
        const observer = new ResizeObserver((entries) => {
            for (let entry of entries) {
                const newHeight = Math.floor(entry.contentRect.height);
                setContainerHeight(prev => Math.abs(prev - newHeight) > 2 ? newHeight : prev);
            }
        });
        observer.observe(containerRef.current);
        return () => observer.disconnect();
    }, []);

    const Row = ({ index, style }: { index: number; style: React.CSSProperties }) => {
        const node = flattenedNodes[index];
        if (!node) return null;
        return <FileTreeItem entry={node.entry} depth={node.depth} iconThemeMapping={iconThemeMapping} style={style} />;
    };

    if (flattenedNodes.length === 0) {
        return <div style={{ padding: '10px 20px', fontSize: '12px', opacity: 0.5 }}>Empty Directory</div>;
    }

    return (
        <ErrorBoundary>
            <div ref={containerRef} style={{ height: '100%', width: '100%', minHeight: '200px', flex: 1, overflow: 'hidden' }}>
                <List
                    className="file-explorer-list"
                    rowCount={flattenedNodes.length}
                    rowHeight={22}
                    rowComponent={Row as any}
                    rowProps={{}}
                    overscanCount={5}
                    style={{ height: containerHeight || 600, width: '100%' }}
                />
            </div>
        </ErrorBoundary>
    );
};

const OpenEditorsItem: React.FC<{ tab: any; active: boolean; onClick: () => void; onClose: () => void }> = ({ tab, active, onClick, onClose }) => {
    const icon = detectLanguageIcon(tab.filename);
    return (
        <div className={`pane-item${active ? ' active' : ''}`} onClick={onClick}>
            {icon.type === 'img' ? (
                <img src={icon.value} style={{ width: '16px', height: '16px', marginRight: '6px', opacity: 0.8 }} />
            ) : (
                <i className={icon.value} style={{ fontFamily: 'codicon', fontStyle: 'normal', marginRight: '6px' }}></i>
            )}
            <span style={{ flex: 1, overflow: 'hidden', textOverflow: 'ellipsis' }}>{tab.filename}</span>
            {tab.isModified && <div className="modified-dot"></div>}
            <i className="codicon codicon-close close-icon" style={{ fontFamily: 'codicon', fontStyle: 'normal' }} onClick={(e) => { e.stopPropagation(); onClose(); }}></i>
        </div>
    );
};

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



const SidebarPane: React.FC<{ title: string; children: React.ReactNode; defaultCollapsed?: boolean; actions?: React.ReactNode }> = ({ title, children, defaultCollapsed = false, actions }) => {
    const [isCollapsed, setIsCollapsed] = useState(defaultCollapsed);
    return (
        <div className={`sidebar-pane${isCollapsed ? ' collapsed' : ''}`} style={{ flex: isCollapsed ? 0 : 1 }}>
            <div className={`pane-header${isCollapsed ? ' collapsed' : ''}`} onClick={() => setIsCollapsed(!isCollapsed)}>
                <i className="codicon codicon-chevron-down" style={{ fontFamily: 'codicon', fontStyle: 'normal' }}></i>
                <span style={{ flex: 1 }}>{title}</span>
                {actions && <div className="pane-actions" onClick={e => e.stopPropagation()}>{actions}</div>}
            </div>
            {!isCollapsed && <div className="pane-content" style={{ flex: 1, overflow: 'hidden' }}>{children}</div>}
        </div>
    );
};

const Sidebar: React.FC = () => {
    const activeView = useStore(state => state.activeSidebarView);
    const isOpen = useStore(state => state.isSidebarOpen);
    const { activeRoot, activeRootName, fileTree, refreshFileTree, setActiveRoot, closeFolder, iconThemeMapping, tabs, activeTabId, setActiveTab, closeTab } = useStore();

    const handleOpenFolder = async () => {
        try {
            const folder = await invoke<string | null>('open_folder');
            if (folder) {
                setActiveRoot(folder);
                await refreshFileTree();
            }
        } catch (error) {
            console.error('Open Folder Error:', error);
        }
    };

    useEffect(() => {
        const menu = document.getElementById('context-menu');
        if (!menu) return;

        const hideMenu = () => menu.classList.add('hidden');

        const handlers: Array<{ id: string; fn: () => void }> = [
            {
                id: 'cm-open', fn: () => {
                    const ctx = (window as any).__explorerContext;
                    if (!ctx) return;
                    if (!ctx.isDir) {
                        useStore.getState().openFile(ctx.path).catch(err => console.error(err));
                    }
                    hideMenu();
                }
            },
            {
                id: 'cm-new-file', fn: async () => {
                    const ctx = (window as any).__explorerContext;
                    if (!ctx) return;
                    const baseDir = ctx.isDir ? ctx.path : ctx.path.substring(0, ctx.path.lastIndexOf('/'));
                    const name = window.prompt('New file name:');
                    if (!name) return;
                    try {
                        await invoke('create_file', { path: `${baseDir}/${name}` });
                        await refreshFileTree();
                    } catch (e) { console.error(e); }
                    hideMenu();
                }
            },
            {
                id: 'cm-new-folder', fn: async () => {
                    const ctx = (window as any).__explorerContext;
                    if (!ctx) return;
                    const baseDir = ctx.isDir ? ctx.path : ctx.path.substring(0, ctx.path.lastIndexOf('/'));
                    const name = window.prompt('New folder name:');
                    if (!name) return;
                    try {
                        await invoke('create_directory', { path: `${baseDir}/${name}` });
                        await refreshFileTree();
                    } catch (e) { console.error(e); }
                    hideMenu();
                }
            },
            {
                id: 'cm-rename', fn: async () => {
                    const ctx = (window as any).__explorerContext;
                    if (!ctx) return;
                    const parent = ctx.path.includes('/') ? ctx.path.substring(0, ctx.path.lastIndexOf('/')) : '';
                    const name = window.prompt('Rename to:', ctx.name);
                    if (!name || name === ctx.name) return;
                    const newPath = parent ? `${parent}/${name}` : name;
                    try {
                        await invoke('rename_path', { oldPath: ctx.path, newPath });
                        await refreshFileTree();
                    } catch (e) { console.error(e); }
                    hideMenu();
                }
            },
            {
                id: 'cm-delete', fn: async () => {
                    const ctx = (window as any).__explorerContext;
                    if (!ctx) return;
                    const confirmDelete = window.confirm(`Delete '${ctx.name}'?`);
                    if (!confirmDelete) return;
                    try {
                        await invoke('delete_path', { path: ctx.path });
                        await refreshFileTree();
                    } catch (e) { console.error(e); }
                    hideMenu();
                }
            },
        ];

        handlers.forEach(({ id, fn }) => {
            const el = document.getElementById(id);
            if (el) el.onclick = (e) => { e.preventDefault(); e.stopPropagation(); fn(); };
        });

        const onGlobalClick = () => hideMenu();
        document.addEventListener('click', onGlobalClick);
        return () => document.removeEventListener('click', onGlobalClick);
    }, [refreshFileTree]);

    if (!isOpen) return null;

    const titles: Record<string, string> = {
        'explorer-view': 'EXPLORER',
        'search-view': 'SEARCH',
        'scm-view': 'SOURCE CONTROL',
        'debug-view': 'RUN AND DEBUG',
        'extensions-view': 'EXTENSIONS',
        'specs-view': 'SPECS',
        'agent-view': 'AGENT SETTINGS',
        'mobile-view': 'MOBILE EMULATORS'
    };

    const extensionContributions = useStore(state => state.extensionContributions);

    const isCoreView = titles[activeView] !== undefined;
    const extensionContainer = !isCoreView ? extensionContributions?.viewsContainers?.activitybar?.find((c: any) => c.id === activeView) : null;
    const extensionViews = extensionContainer ? (extensionContributions?.views?.[activeView] || []) : [];

    return (
        <aside className="sidebar" id="sidebar">
            {activeView !== 'extensions-view' && (
                <div className="sidebar-section-header" style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', padding: '0 12px', height: '35px', minHeight: '35px' }}>
                    <div style={{ fontSize: '11px', fontWeight: 600 }}>{titles[activeView] || (extensionContainer?.title?.toUpperCase() || activeView.toUpperCase())}</div>
                </div>
            )}

            <div className="sidebar-content-wrapper" style={{ flex: 1, overflow: 'hidden', display: 'flex', flexDirection: 'column' }}>
                {activeView === 'explorer-view' && (
                    <div className="sidebar-content" style={{ display: 'flex', flexDirection: 'column', overflowY: 'hidden', flex: 1 }}>

                        <SidebarPane
                            title={activeRootName || 'NO FOLDER OPENED'}
                            defaultCollapsed={false}
                            actions={activeRoot ? (
                                <div style={{ display: 'flex', gap: '8px', alignItems: 'center', paddingRight: '8px' }}>
                                    <i className="codicon codicon-new-file" onClick={() => (window as any).executeCommand('explorer.newFile')} style={{ cursor: 'pointer', fontSize: '14px', opacity: 0.8, fontFamily: 'codicon', fontStyle: 'normal' }} title="New File"></i>
                                    <i className="codicon codicon-new-folder" onClick={() => (window as any).executeCommand('explorer.newFolder')} style={{ cursor: 'pointer', fontSize: '14px', opacity: 0.8, fontFamily: 'codicon', fontStyle: 'normal' }} title="New Folder"></i>
                                    <i className="codicon codicon-close-all" onClick={() => (window as any).executeCommand('workbench.action.closeFolder')} style={{ cursor: 'pointer', fontSize: '14px', opacity: 0.8, fontFamily: 'codicon', fontStyle: 'normal' }} title="Close Folder"></i>
                                    <i className="codicon codicon-refresh" onClick={refreshFileTree} style={{ cursor: 'pointer', fontSize: '14px', opacity: 0.8, fontFamily: 'codicon', fontStyle: 'normal' }} title="Refresh"></i>
                                </div>
                            ) : null}
                        >
                            <div style={{ flex: 1, minHeight: '300px' }}>
                                {activeRoot ? (
                                    <div className="file-tree" style={{ height: '100%' }}>
                                        {fileTree.length > 0 ? (
                                            <React.Suspense fallback={<div style={{ padding: '20px', opacity: 0.5 }}>Loading...</div>}>
                                                <VirtualizedFileTree entries={fileTree} iconThemeMapping={iconThemeMapping} />
                                            </React.Suspense>
                                        ) : (
                                            <div style={{ padding: '10px 20px', fontSize: '12px', opacity: 0.5 }}>Empty Directory</div>
                                        )}
                                    </div>
                                ) : (
                                    <div style={{ padding: '20px', textAlign: 'center' }}>
                                        <p style={{ fontSize: '12px', opacity: 0.7, marginBottom: '12px' }}>You have not yet opened a folder.</p>
                                        <button className="primary-button" onClick={handleOpenFolder} style={{ width: '100%', padding: '6px', fontSize: '13px' }}>Open Folder</button>
                                    </div>
                                )}
                            </div>
                        </SidebarPane>

                        <SidebarPane title="OUTLINE" defaultCollapsed={true}>
                            <div style={{ padding: '20px', textAlign: 'center', opacity: 0.5, fontSize: '12px' }}>
                                No outline information found.
                            </div>
                        </SidebarPane>

                        <SidebarPane title="TIMELINE" defaultCollapsed={true}>
                            <div style={{ padding: '20px', textAlign: 'center', opacity: 0.5, fontSize: '12px' }}>
                                The timeline view is not yet available.
                            </div>
                        </SidebarPane>
                    </div>
                )}

                {activeView === 'search-view' && <SearchView />}
                {activeView === 'scm-view' && <ScmView />}
                {activeView === 'debug-view' && <DebugView />}
                {activeView === 'extensions-view' && <ExtensionsView />}
                {activeView === 'agent-view' && <AgentSettingsView />}
                {activeView === 'mobile-view' && <EmulatorPanel />}

                {/* Extension Contributed Views */}
                {extensionContainer && (
                    <div className="sidebar-content" style={{ display: 'flex', flexDirection: 'column', flex: 1, overflowY: 'auto' }}>
                        {extensionViews.length > 0 ? (
                            extensionViews.map((view: any) => (
                                <SidebarPane key={view.id} title={view.name.toUpperCase()} defaultCollapsed={false}>
                                    <div style={{ padding: '20px', textAlign: 'center', opacity: 0.7 }}>
                                        <div style={{ fontSize: '12px', marginBottom: '8px' }}>{view.name} View</div>
                                        <div style={{ fontSize: '10px', color: 'var(--text-secondary)' }}>ID: {view.id}</div>
                                        <div style={{ fontSize: '10px', marginTop: '12px', fontStyle: 'italic' }}>
                                            This view is provided by extension: <br /> {view.extensionId}
                                        </div>
                                        {/* In a real scenario, we'd render a webview or iframe here for the extension's UI */}
                                        <div style={{ marginTop: '20px', padding: '8px', border: '1px dashed var(--vscode-panel-border)', fontSize: '11px' }}>
                                            UI content from extension package would be rendered here via Webview API.
                                        </div>
                                    </div>
                                </SidebarPane>
                            ))
                        ) : (
                            <div style={{ padding: '20px', textAlign: 'center', opacity: 0.5, fontSize: '12px' }}>
                                No views registered for this container.
                            </div>
                        )}
                    </div>
                )}
            </div>
        </aside>
    );
};

export default Sidebar;
