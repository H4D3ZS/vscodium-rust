import React, { useState, useEffect, useMemo, useCallback, useRef } from 'react';
import { useShallow } from 'zustand/react/shallow';
import { useStore, type FileEntry } from '../store';
import { invoke } from '@tauri-apps/api/core';
import { List } from 'react-window';
import ErrorBoundary from './ErrorBoundary';
import SearchView from './SearchView';
import ExtensionsView from './ExtensionsView';
import ScmView from './ScmView';
import DebugView from './DebugView';
import TestExplorer from './TestExplorer';
import ProjectSpecsSidebar from './ProjectSpecsSidebar';
import VectorSearchPanel from './vectorSearchPanel';
import AgTasksView from './AgTasksView';
import AgSteeringView from './AgSteeringView';

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

type TreeIcon = { type: 'img' | 'icon'; value: string; color?: string };

const TreeItemIcon: React.FC<{ icon: TreeIcon }> = ({ icon }) => {
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
        opacity: 0.95,
        color: icon.color || 'inherit',
        fontStyle: 'normal',
        fontWeight: 'normal',
        lineHeight: 1,
        WebkitFontSmoothing: 'antialiased'
    }}></i>;
};

// Lightweight native-style file-type icons: a colored codicon per extension /
// special filename. Avoids shipping a full icon-theme asset pack while giving
// the explorer the typed, colored look of real VSCode.
function fileCodicon(name: string): { cls: string; color: string } {
    const lc = name.toLowerCase();
    const ext = lc.includes('.') ? lc.split('.').pop() || '' : '';
    if (lc === 'package.json') return { cls: 'codicon-json', color: '#cb3837' };
    if (lc.endsWith('.lock') || lc === 'package-lock.json' || lc === 'pnpm-lock.yaml') return { cls: 'codicon-lock', color: '#8a8a8a' };
    if (lc === '.gitignore' || lc === '.gitattributes') return { cls: 'codicon-source-control', color: '#e8633a' };
    if (lc === 'dockerfile') return { cls: 'codicon-file-code', color: '#0db7ed' };
    if (lc === 'readme.md') return { cls: 'codicon-book', color: '#519aba' };
    if (lc === 'license' || lc === 'licence' || lc.startsWith('license')) return { cls: 'codicon-law', color: '#cdba6f' };
    const map: Record<string, { cls: string; color: string }> = {
        ts: { cls: 'codicon-file-code', color: '#3178c6' }, tsx: { cls: 'codicon-file-code', color: '#3178c6' },
        js: { cls: 'codicon-file-code', color: '#e8d44d' }, jsx: { cls: 'codicon-file-code', color: '#e8d44d' },
        mjs: { cls: 'codicon-file-code', color: '#e8d44d' },
        rs: { cls: 'codicon-file-code', color: '#dea584' }, py: { cls: 'codicon-file-code', color: '#4b8bbe' },
        go: { cls: 'codicon-file-code', color: '#00add8' }, java: { cls: 'codicon-file-code', color: '#e76f00' },
        c: { cls: 'codicon-file-code', color: '#5c6bc0' }, cpp: { cls: 'codicon-file-code', color: '#5c6bc0' },
        cs: { cls: 'codicon-file-code', color: '#9b59b6' }, rb: { cls: 'codicon-file-code', color: '#cc342d' },
        php: { cls: 'codicon-file-code', color: '#777bb3' }, swift: { cls: 'codicon-file-code', color: '#f05138' },
        kt: { cls: 'codicon-file-code', color: '#a97bff' }, astro: { cls: 'codicon-file-code', color: '#ff5d01' },
        vue: { cls: 'codicon-file-code', color: '#41b883' }, svelte: { cls: 'codicon-file-code', color: '#ff3e00' },
        html: { cls: 'codicon-file-code', color: '#e34c26' }, css: { cls: 'codicon-file-code', color: '#519aba' },
        scss: { cls: 'codicon-file-code', color: '#cd6799' }, json: { cls: 'codicon-json', color: '#cbcb41' },
        md: { cls: 'codicon-markdown', color: '#519aba' }, toml: { cls: 'codicon-settings-gear', color: '#9c9c9c' },
        yaml: { cls: 'codicon-settings-gear', color: '#cb171e' }, yml: { cls: 'codicon-settings-gear', color: '#cb171e' },
        xml: { cls: 'codicon-file-code', color: '#e37933' }, sql: { cls: 'codicon-database', color: '#dad8d8' },
        sh: { cls: 'codicon-terminal', color: '#89e051' }, ps1: { cls: 'codicon-terminal', color: '#4b8bbe' },
        bat: { cls: 'codicon-terminal', color: '#89e051' },
        png: { cls: 'codicon-file-media', color: '#a074c4' }, jpg: { cls: 'codicon-file-media', color: '#a074c4' },
        jpeg: { cls: 'codicon-file-media', color: '#a074c4' }, gif: { cls: 'codicon-file-media', color: '#a074c4' },
        svg: { cls: 'codicon-file-media', color: '#ffb13b' }, ico: { cls: 'codicon-file-media', color: '#a074c4' },
        webp: { cls: 'codicon-file-media', color: '#a074c4' },
        pdf: { cls: 'codicon-file-pdf', color: '#e5252a' }, zip: { cls: 'codicon-file-zip', color: '#f5c518' },
        gz: { cls: 'codicon-file-zip', color: '#f5c518' }, exe: { cls: 'codicon-file-binary', color: '#9c9c9c' },
        dll: { cls: 'codicon-file-binary', color: '#9c9c9c' }, aim: { cls: 'codicon-chip', color: '#c084fc' },
    };
    return map[ext] || { cls: 'codicon-file', color: '#9aa0a6' };
}

const FileTreeItem: React.FC<{ entry: FileEntry; depth: number; iconThemeMapping: any; style: React.CSSProperties }> = ({ entry, depth, iconThemeMapping, style }) => {
    const openFile = useStore(state => state.openFile);
    const toggleDirectory = useStore(state => state.toggleDirectory);
    const activeTabId = useStore(state => state.activeTabId);
    const tabs = useStore(state => state.tabs);
    const setContextMenuOpen = useStore(state => state.setContextMenuOpen);

    const isExpanded = entry.is_expanded ?? false;
    const isActive = tabs.find(t => t.id === activeTabId)?.path === entry.path;

    const getIcon = (): TreeIcon => {
        // FORCE SVG for Folders to bypass ALL font issues
        const folderSvg = `data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9IiM3OWI4ZmYiIHN0cm9rZS13aWR0aD0iMiIgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIiBzdHJva2UtbGluZWpvaW49InJvdW5kIj48cGF0aCBkPSJNMjIgMTlhMiAyIDAgMCAxLTIgMkg0YTIgMiAwIDAgMS0yLTJWN2EyIDIgMCAwIDEgMi0yaDVsMiAyaDlhMiAyIDAgMCAxIDIgMnYxMHoiPjwvcGF0aD48L3N2Zz4=`;

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

        // Native-style fallback: a colored, typed codicon per extension.
        const fc = fileCodicon(entry.name);
        return { type: 'icon', value: `codicon ${fc.cls}`, color: fc.color };
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
            <div ref={containerRef} style={{ flex: 1, width: '100%', overflow: 'hidden', minHeight: 0 }}>
                <List
                    className="file-explorer-list"
                    rowCount={flattenedNodes.length}
                    rowHeight={22}
                    rowComponent={Row as any}
                    rowProps={{}}
                    overscanCount={10}
                    style={{ height: containerHeight > 0 ? containerHeight : 400, width: '100%' }}
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



const SidebarPane: React.FC<{ title: string; children: React.ReactNode; defaultCollapsed?: boolean; actions?: React.ReactNode; flexGrow?: number }> = ({ title, children, defaultCollapsed = false, actions, flexGrow = 1 }) => {
    const [isCollapsed, setIsCollapsed] = useState(defaultCollapsed);
    return (
        <div
            className={`sidebar-pane${isCollapsed ? ' collapsed' : ''}`}
            style={{
                display: 'flex',
                flexDirection: 'column',
                flex: isCollapsed ? '0 0 auto' : `${flexGrow} ${flexGrow} 0`,
                minHeight: isCollapsed ? 0 : undefined,
                overflow: 'hidden',
            }}
        >
            <div className={`pane-header${isCollapsed ? ' collapsed' : ''}`} onClick={() => setIsCollapsed(!isCollapsed)}>
                <i
                    className="codicon codicon-chevron-down"
                    style={{ fontFamily: 'codicon', fontStyle: 'normal', transform: isCollapsed ? 'rotate(-90deg)' : 'none', transition: 'transform 0.15s' }}
                ></i>
                <span style={{ flex: 1, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{title}</span>
                {actions && <div className="pane-actions" onClick={e => e.stopPropagation()}>{actions}</div>}
            </div>
            {!isCollapsed && (
                <div className="pane-content" style={{ flex: 1, display: 'flex', flexDirection: 'column', overflow: 'hidden', minHeight: 0 }}>
                    {children}
                </div>
            )}
        </div>
    );
};

// ── Symbol Outline Pane ──────────────────────────────────────────────────────
const SYMBOL_KIND_ICONS: Record<number, string> = {
    1: 'codicon-symbol-file', 2: 'codicon-symbol-namespace', 3: 'codicon-symbol-namespace',
    4: 'codicon-symbol-namespace', 5: 'codicon-symbol-class', 6: 'codicon-symbol-method',
    7: 'codicon-symbol-property', 8: 'codicon-symbol-field', 9: 'codicon-symbol-enum-member',
    10: 'codicon-symbol-interface', 11: 'codicon-symbol-function', 12: 'codicon-symbol-variable',
    13: 'codicon-symbol-constant', 14: 'codicon-symbol-string', 15: 'codicon-symbol-numeric',
    16: 'codicon-symbol-boolean', 17: 'codicon-symbol-array', 18: 'codicon-symbol-object',
    19: 'codicon-symbol-key', 20: 'codicon-symbol-null', 21: 'codicon-symbol-enum',
    22: 'codicon-symbol-struct', 23: 'codicon-symbol-event', 24: 'codicon-symbol-operator',
    25: 'codicon-symbol-type-parameter',
};

const SymbolItem: React.FC<{ sym: any; depth: number }> = ({ sym, depth }) => {
    const [open, setOpen] = useState(true);
    const icon = SYMBOL_KIND_ICONS[sym.kind] || 'codicon-symbol-misc';
    const line = (sym.selectionRange?.start?.line ?? sym.range?.start?.line ?? 0) + 1;
    const activeEditorPath = useStore(state => state.activeEditorPath);
    return (
        <div>
            <div
                style={{ display: 'flex', alignItems: 'center', padding: `2px 8px 2px ${8 + depth * 12}px`, cursor: 'pointer', fontSize: '12px' }}
                onMouseEnter={e => (e.currentTarget.style.background = 'var(--vscode-list-hoverBackground)')}
                onMouseLeave={e => (e.currentTarget.style.background = 'transparent')}
                onClick={() => {
                    window.dispatchEvent(new CustomEvent('editor:jump-to-line', {
                        detail: { path: activeEditorPath, line, column: 1 }
                    }));
                    if (sym.children?.length) setOpen(o => !o);
                }}
            >
                {sym.children?.length > 0 && (
                    <i className={`codicon codicon-chevron-${open ? 'down' : 'right'}`} style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '10px', marginRight: '2px', opacity: 0.6 }} />
                )}
                {!sym.children?.length && <span style={{ width: '14px', display: 'inline-block' }} />}
                <i className={`codicon ${icon}`} style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '13px', marginRight: '6px', opacity: 0.8 }} />
                <span style={{ overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{sym.name}</span>
                <span style={{ marginLeft: '6px', opacity: 0.35, fontSize: '10px' }}>{line}</span>
            </div>
            {open && sym.children?.map((child: any, i: number) => (
                <SymbolItem key={i} sym={child} depth={depth + 1} />
            ))}
        </div>
    );
};

const SymbolOutlinePane: React.FC = () => {
    const activeEditorPath = useStore(state => state.activeEditorPath);
    const [symbols, setSymbols] = useState<any[]>([]);
    const [loading, setLoading] = useState(false);
    const fetchRef = useRef<string>('');

    useEffect(() => {
        if (!activeEditorPath) { setSymbols([]); return; }
        const normalized = activeEditorPath.replace(/\\/g, '/');
        const uri = normalized.startsWith('/') ? `file://${normalized}` : `file:///${normalized}`;
        if (fetchRef.current === uri) return;
        fetchRef.current = uri;
        setLoading(true);
        invoke<any>('lsp_document_symbols', { uri })
            .then(res => {
                if (fetchRef.current !== uri) return;
                setSymbols(Array.isArray(res) ? res : []);
            })
            .catch(() => setSymbols([]))
            .finally(() => setLoading(false));
    }, [activeEditorPath]);

    if (loading) return <div style={{ padding: '8px 12px', fontSize: '11px', opacity: 0.5 }}>Loading symbols…</div>;
    if (!activeEditorPath) return <div style={{ padding: '12px', fontSize: '11px', opacity: 0.4, textAlign: 'center' }}>Open a file to see its symbols.</div>;
    if (symbols.length === 0) return <div style={{ padding: '12px', fontSize: '11px', opacity: 0.4, textAlign: 'center' }}>No symbols found.</div>;
    return (
        <div style={{ overflowY: 'auto', flex: 1 }}>
            {symbols.map((sym, i) => <SymbolItem key={i} sym={sym} depth={0} />)}
        </div>
    );
};

const Sidebar: React.FC = () => {
    const activeView = useStore(state => state.activeSidebarView);
    const isOpen = useStore(state => state.isSidebarOpen);
    const { activeRoot, activeRootName, fileTree, refreshFileTree, setActiveRoot, closeFolder, iconThemeMapping, tabs, activeTabId, setActiveTab, closeTab } = useStore(useShallow(s => ({
        activeRoot: s.activeRoot, activeRootName: s.activeRootName, fileTree: s.fileTree,
        refreshFileTree: s.refreshFileTree, setActiveRoot: s.setActiveRoot, closeFolder: s.closeFolder,
        iconThemeMapping: s.iconThemeMapping, tabs: s.tabs, activeTabId: s.activeTabId,
        setActiveTab: s.setActiveTab, closeTab: s.closeTab,
    })));

    const handleOpenFolder = async () => {
        try {
            const folder = await invoke<string | null>('open_folder');
            if (folder) {
                // setActiveRoot calls set_active_root + refreshFileTree internally.
                // Call refreshFileTree once explicitly after so the tree is loaded
                // immediately without waiting for the async chain inside setActiveRoot.
                setActiveRoot(folder);
                // Small yield so the state update lands before the refresh
                await new Promise(r => setTimeout(r, 50));
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
        'test-view': 'TEST EXPLORER',
        'extensions-view': 'EXTENSIONS',
        'specs-view': 'SPECS',
        'tasks-view': 'TASKS & SPECS',
        'steering-view': 'STEERING & HOOKS',
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
                    <div className="sidebar-content" style={{ display: 'flex', flexDirection: 'column', overflow: 'hidden', flex: 1, minHeight: 0 }}>

                        {/* FILE TREE — takes all available space */}
                        <SidebarPane
                            title={activeRootName || 'NO FOLDER OPENED'}
                            defaultCollapsed={false}
                            flexGrow={4}
                            actions={activeRoot ? (
                                <div style={{ display: 'flex', gap: '8px', alignItems: 'center', paddingRight: '8px' }}>
                                    <i className="codicon codicon-new-file" onClick={(e) => { e.stopPropagation(); (window as any).executeCommand('explorer.newFile'); }} style={{ cursor: 'pointer', fontSize: '14px', opacity: 0.8, fontFamily: 'codicon', fontStyle: 'normal' }} title="New File"></i>
                                    <i className="codicon codicon-new-folder" onClick={(e) => { e.stopPropagation(); (window as any).executeCommand('explorer.newFolder'); }} style={{ cursor: 'pointer', fontSize: '14px', opacity: 0.8, fontFamily: 'codicon', fontStyle: 'normal' }} title="New Folder"></i>
                                    <i className="codicon codicon-close-all" onClick={(e) => { e.stopPropagation(); (window as any).executeCommand('workbench.action.closeFolder'); }} style={{ cursor: 'pointer', fontSize: '14px', opacity: 0.8, fontFamily: 'codicon', fontStyle: 'normal' }} title="Close Folder"></i>
                                    <i className="codicon codicon-refresh" onClick={(e) => { e.stopPropagation(); refreshFileTree(); }} style={{ cursor: 'pointer', fontSize: '14px', opacity: 0.8, fontFamily: 'codicon', fontStyle: 'normal' }} title="Refresh"></i>
                                </div>
                            ) : null}
                        >
                            {/* This div fills the pane-content flex container */}
                            <div style={{ display: 'flex', flexDirection: 'column', flex: 1, overflow: 'hidden', minHeight: 0 }}>
                                {activeRoot ? (
                                    fileTree.length > 0 ? (
                                        <VirtualizedFileTree entries={fileTree} iconThemeMapping={iconThemeMapping} />
                                    ) : (
                                        <div style={{ padding: '10px 20px', fontSize: '12px', opacity: 0.5 }}>Empty Directory</div>
                                    )
                                ) : (
                                    <div style={{ padding: '20px', textAlign: 'center' }}>
                                        <p style={{ fontSize: '12px', opacity: 0.7, marginBottom: '12px' }}>You have not yet opened a folder.</p>
                                        <button className="primary-button" onClick={handleOpenFolder} style={{ width: '100%', padding: '6px', fontSize: '13px' }}>Open Folder</button>
                                    </div>
                                )}
                            </div>
                        </SidebarPane>

                        <SidebarPane title="AI PROJECT SPECS" defaultCollapsed={true} flexGrow={1}>
                            <ProjectSpecsSidebar />
                        </SidebarPane>

                        <SidebarPane title="OUTLINE" defaultCollapsed={false} flexGrow={2}>
                            <SymbolOutlinePane />
                        </SidebarPane>

                        <SidebarPane title="TIMELINE" defaultCollapsed={true} flexGrow={1}>
                            <div style={{ padding: '20px', textAlign: 'center', opacity: 0.5, fontSize: '12px' }}>
                                The timeline view is not yet available.
                            </div>
                        </SidebarPane>
                    </div>
                )}

                {activeView === 'search-view' && <SearchView />}
                {activeView === 'scm-view' && <ScmView />}
                {activeView === 'debug-view' && <DebugView />}
                {activeView === 'test-view' && <TestExplorer />}
                {activeView === 'extensions-view' && <ExtensionsView />}
                {activeView === 'vector-search-view' && <VectorSearchPanel />}
                {activeView === 'tasks-view' && <AgTasksView />}
                {activeView === 'steering-view' && <AgSteeringView />}

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
