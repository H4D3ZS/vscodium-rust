import React, { useState, useEffect } from 'react';
import { useStore, type FileEntry } from '../store';
import { invoke } from '@tauri-apps/api/core';
import GitGraph from './GitGraph';
import EmulatorPanel from './EmulatorPanel';

const FileTreeItem: React.FC<{ entry: FileEntry; depth: number }> = ({ entry, depth }) => {
    const [isCollapsed, setIsCollapsed] = useState(true);
    const openFile = useStore(state => state.openFile);
    const iconThemeMapping = useStore(state => state.iconThemeMapping);

    const getIcon = () => {
        if (entry.is_dir) {
            return { type: 'icon', value: `codicon codicon-${isCollapsed ? 'chevron-right' : 'chevron-down'}` };
        }
        
        if (iconThemeMapping) {
            const ext = entry.name.split('.').pop()?.toLowerCase();
            let iconId = null;
            
            if (ext && iconThemeMapping.fileExtensions && iconThemeMapping.fileExtensions[ext]) {
                iconId = iconThemeMapping.fileExtensions[ext];
            } else if (iconThemeMapping.file) {
                iconId = iconThemeMapping.file;
            }

            if (iconId && iconThemeMapping.iconDefinitions && iconThemeMapping.iconDefinitions[iconId]) {
                const def = iconThemeMapping.iconDefinitions[iconId];
                if (def.iconPath) {
                    return { type: 'img', value: def.iconPath };
                }
            }
        }
        
        return { type: 'icon', value: entry.is_dir ? 'codicon codicon-folder' : 'codicon codicon-file' };
    };

    // Auto-open if it's the root (rel_path is empty)
    useEffect(() => {
        if (depth === 0) setIsCollapsed(false);
    }, [depth]);

    const handleToggle = (e: React.MouseEvent) => {
        e.stopPropagation();
        if (entry.is_dir) {
            setIsCollapsed(!isCollapsed);
        } else {
            openFile(entry.path).catch(err => console.error(err));
        }
    };

    const handleContextMenu = (e: React.MouseEvent) => {
        e.preventDefault();
        e.stopPropagation();

        // Store context globally for menu handlers.
        (window as any).__explorerContext = {
            path: entry.path,
            name: entry.name,
            isDir: entry.is_dir,
        };

        const menu = document.getElementById('context-menu') as HTMLElement | null;
        if (menu) {
            menu.style.left = `${e.clientX}px`;
            menu.style.top = `${e.clientY}px`;
            menu.classList.remove('hidden');
        }
    };

    return (
        <div className="file-tree-item" style={{ userSelect: 'none' }}>
            <div
                className="tree-row"
                onClick={handleToggle}
                onContextMenu={handleContextMenu}
                style={{
                    display: 'flex',
                    alignItems: 'center',
                    height: '22px',
                    paddingLeft: `${depth * 12 + 12}px`,
                    cursor: 'pointer',
                    fontSize: '13px',
                    color: 'var(--vscode-sideBar-foreground)',
                    whiteSpace: 'nowrap'
                }}
            >
                {(() => {
                    const icon = getIcon();
                    if (icon.type === 'img') {
                        return <img src={icon.value} style={{ marginRight: '6px', width: '16px', height: '16px' }} />;
                    } else {
                        return <i className={icon.value} style={{ marginRight: '6px', fontSize: '14px', width: '16px', textAlign: 'center' }}></i>;
                    }
                })()}
                <span>{entry.name}</span>
            </div>
            {!isCollapsed && entry.children && (
                <div className="tree-children">
                    {entry.children.map(child => (
                        <FileTreeItem key={child.path} entry={child} depth={depth + 1} />
                    ))}
                </div>
            )}
        </div>
    );
};

const Sidebar: React.FC = () => {
    const activeView = useStore(state => state.activeSidebarView);
    const isOpen = useStore(state => state.isSidebarOpen);
    const fileTree = useStore(state => state.fileTree);
    const refreshFileTree = useStore(state => state.refreshFileTree);
    const setActiveRoot = useStore(state => state.setActiveRoot);
    const activeRoot = useStore(state => state.activeRoot);
    const activeRootName = useStore(state => state.activeRootName);
    const activeDevice = useStore(state => state.activeDevice);
    const setView = useStore(state => state.setActiveSidebarView);
    const openFile = useStore(state => state.openFile);

    // API Keys state
    const [openAIKey, setOpenAIKey] = useState('');
    const [anthropicKey, setAnthropicKey] = useState('');
    const [googleKey, setGoogleKey] = useState('');
    const [statusMessage, setStatusMessage] = useState('');

    useEffect(() => {
        invoke('get_api_keys').then((keys: any) => {
            if (keys.openai) setOpenAIKey(keys.openai);
            if (keys.anthropic) setAnthropicKey(keys.anthropic);
            if (keys.google) setGoogleKey(keys.google);
        }).catch(err => console.error("Failed to load keys", err));
    }, []);

    const handleOpenFolder = async () => {
        try {
            const result = await invoke<string | null>('open_folder');
            if (result) {
                (window as any).activeRoot = result;
                setActiveRoot(result);
                await refreshFileTree();
            }
        } catch (err) {
            console.error(err);
        }
    };

    const saveKeys = async () => {
        setStatusMessage('Saving...');
        try {
            await invoke('save_api_keys', {
                keys: {
                    openai: openAIKey,
                    anthropic: anthropicKey,
                    google: googleKey
                }
            });
            setStatusMessage('API Keys Saved Successfully.');
            setTimeout(() => setStatusMessage(''), 3000);
        } catch (err) {
            console.error(err);
            setStatusMessage('Error saving keys.');
        }
    };

    useEffect(() => {
        // Wire context menu actions once.
        const menu = document.getElementById('context-menu');
        if (!menu) return;

        const hideMenu = () => menu.classList.add('hidden');

        const handlers: Array<{ id: string; fn: () => void }> = [
            {
                id: 'cm-open',
                fn: () => {
                    const ctx = (window as any).__explorerContext;
                    if (!ctx) return;
                    if (!ctx.isDir) {
                        openFile(ctx.path).catch((err: any) => console.error(err));
                    }
                    hideMenu();
                },
            },
            {
                id: 'cm-new-file',
                fn: async () => {
                    const ctx = (window as any).__explorerContext;
                    if (!ctx) return;
                    const baseDir = ctx.isDir ? ctx.path : ctx.path.substring(0, ctx.path.lastIndexOf('/'));
                    const name = window.prompt('New file name:');
                    if (!name) return;
                    try {
                        await invoke('create_file', { path: `${baseDir}/${name}` });
                        await refreshFileTree();
                    } catch (e) {
                        console.error('Create file failed:', e);
                    }
                    hideMenu();
                },
            },
            {
                id: 'cm-new-folder',
                fn: async () => {
                    const ctx = (window as any).__explorerContext;
                    if (!ctx) return;
                    const baseDir = ctx.isDir ? ctx.path : ctx.path.substring(0, ctx.path.lastIndexOf('/'));
                    const name = window.prompt('New folder name:');
                    if (!name) return;
                    try {
                        await invoke('create_directory', { path: `${baseDir}/${name}` });
                        await refreshFileTree();
                    } catch (e) {
                        console.error('Create folder failed:', e);
                    }
                    hideMenu();
                },
            },
            {
                id: 'cm-rename',
                fn: async () => {
                    const ctx = (window as any).__explorerContext;
                    if (!ctx) return;
                    const parent = ctx.path.includes('/') ? ctx.path.substring(0, ctx.path.lastIndexOf('/')) : '';
                    const name = window.prompt('Rename to:', ctx.name);
                    if (!name || name === ctx.name) return;
                    const newPath = parent ? `${parent}/${name}` : name;
                    try {
                        await invoke('rename_path', { oldPath: ctx.path, newPath });
                        await refreshFileTree();
                    } catch (e) {
                        console.error('Rename failed:', e);
                    }
                    hideMenu();
                },
            },
            {
                id: 'cm-delete',
                fn: async () => {
                    const ctx = (window as any).__explorerContext;
                    if (!ctx) return;
                    const confirmDelete = window.confirm(`Delete '${ctx.name}'? This cannot be undone.`);
                    if (!confirmDelete) return;
                    try {
                        await invoke('delete_path', { path: ctx.path });
                        await refreshFileTree();
                    } catch (e) {
                        console.error('Delete failed:', e);
                    }
                    hideMenu();
                },
            },
            {
                id: 'cm-palette',
                fn: () => {
                    hideMenu();
                    (window as any).showCommandPalette?.();
                },
            },
        ];

        handlers.forEach(({ id, fn }) => {
            const el = document.getElementById(id);
            if (el) {
                el.onclick = (e) => {
                    e.preventDefault();
                    e.stopPropagation();
                    fn();
                };
            }
        });

        const onGlobalClick = (e: MouseEvent) => {
            if (!(e.target as HTMLElement).closest('#context-menu')) {
                hideMenu();
            }
        };
        document.addEventListener('click', onGlobalClick);
        document.addEventListener('contextmenu', onGlobalClick);

        return () => {
            document.removeEventListener('click', onGlobalClick);
            document.removeEventListener('contextmenu', onGlobalClick);
        };
    }, [openFile]);

    const handleNewFile = async (e: React.MouseEvent) => {
        e.stopPropagation();
        const root = activeRoot;
        if (!root) return;
        const name = window.prompt('New file name:');
        if (!name) return;
        try {
            await invoke('create_file', { path: `${root}/${name}` });
            await refreshFileTree();
        } catch (e) {
            console.error('Create file failed:', e);
        }
    };

    const handleNewFolder = async (e: React.MouseEvent) => {
        e.stopPropagation();
        const root = activeRoot;
        if (!root) return;
        const name = window.prompt('New folder name:');
        if (!name) return;
        try {
            await invoke('create_directory', { path: `${root}/${name}` });
            await refreshFileTree();
        } catch (e) {
            console.error('Create folder failed:', e);
        }
    };

    const handleRefresh = async (e: React.MouseEvent) => {
        e.stopPropagation();
        await refreshFileTree();
    };

    if (!isOpen) return null;

    const titles: Record<string, string> = {
        'explorer-view': 'Explorer',
        'search-view': 'Search',
        'scm-view': 'Source Control (Graph)',
        'debug-view': 'Run and Debug',
        'extensions-view': 'Extensions',
        'specs-view': 'Specs',
        'agent-view': 'Agent',
        'planning-view': 'Planning & History',
        'mobile-view': 'Mobile (Android & iOS)'
    };

    return (
        <aside className="sidebar" id="sidebar" style={{ background: 'var(--vscode-sideBar-background)', color: 'var(--vscode-sideBar-foreground)', display: 'flex', flexDirection: 'column', width: 'var(--sidebar-width)', borderRight: '1px solid var(--vscode-panel-border)' }}>
            <div id="explorer-view" className={`sidebar-section ${activeView === 'explorer-view' ? '' : 'hidden'}`} style={{ flex: 1, display: 'flex', flexDirection: 'column', overflow: 'hidden' }}>
                <div className="sidebar-header" style={{ display: 'flex', alignItems: 'center', padding: '0 12px', height: '35px', minHeight: '35px', borderBottom: '1px solid var(--vscode-panel-border)', background: 'var(--vscode-sideBar-background)' }}>
                    <span style={{ fontSize: '11px', fontWeight: 600, textTransform: 'uppercase', flex: 1, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap', color: 'var(--vscode-sideBar-foreground)', opacity: 0.8 }}>
                        {activeRootName || 'EXPLORER'}
                    </span>
                    {activeRoot && (
                        <div style={{ display: 'flex', gap: '8px', alignItems: 'center' }}>
                            <i className="codicon codicon-new-file" onClick={handleNewFile} style={{ cursor: 'pointer', fontSize: '14px', opacity: 0.8 }} title="New File"></i>
                            <i className="codicon codicon-new-folder" onClick={handleNewFolder} style={{ cursor: 'pointer', fontSize: '14px', opacity: 0.8 }} title="New Folder"></i>
                            <i className="codicon codicon-refresh" onClick={handleRefresh} style={{ cursor: 'pointer', fontSize: '14px', opacity: 0.8 }} title="Refresh Explorer"></i>
                        </div>
                    )}
                </div>
                <div id="explorer-content" className="sidebar-content" style={{ display: 'flex', flexDirection: 'column', padding: '0 0 10px 0', overflowY: 'auto', flex: 1 }}>
                    {fileTree.length > 0 ? (
                        <div className="file-tree" style={{ width: '100%' }}>
                            {fileTree.map(entry => (
                                <FileTreeItem key={entry.path} entry={entry} depth={0} />
                            ))}
                        </div>
                    ) : (
                        <>
                            <div style={{ padding: '20px', textAlign: 'center', opacity: 0.6, fontSize: '12px' }}>
                                You have not yet opened a folder.
                            </div>
                            <button
                                className="primary-button"
                                id="explorer-open-folder"
                                onClick={handleOpenFolder}
                                style={{ background: 'var(--vscode-button-background)', color: 'var(--vscode-button-foreground)', border: 'none', padding: '6px 10px', borderRadius: '2px', cursor: 'pointer', fontSize: '13px', margin: '0 20px' }}
                            >Open Folder</button>
                        </>
                    )}
                </div>
            </div>

            <div id="agent-view" className={`sidebar-section ${activeView === 'agent-view' ? '' : 'hidden'}`}>
                <div className="sidebar-section-header">AGENT SETTINGS</div>
                <div className="sidebar-content" style={{ padding: '15px' }}>
                    <div style={{ marginBottom: '10px' }}>
                        <label style={{ display: 'block', marginBottom: '5px', color: 'var(--vscode-sideBar-foreground)', opacity: 0.8, fontSize: '11px' }}>OpenAI API Key</label>
                        <input type="password" value={openAIKey} onChange={(e) => setOpenAIKey(e.target.value)} id="openai-api-key" placeholder="sk-..." style={{ width: '100%', boxSizing: 'border-box', background: 'var(--vscode-input-background)', color: 'var(--vscode-sideBar-foreground)', border: '1px solid transparent', padding: '4px 6px', fontSize: '12px', outline: 'none' }} />
                    </div>
                    <div style={{ marginBottom: '10px' }}>
                        <label style={{ display: 'block', marginBottom: '5px', color: 'var(--vscode-sideBar-foreground)', opacity: 0.8, fontSize: '11px' }}>Anthropic API Key</label>
                        <input type="password" value={anthropicKey} onChange={(e) => setAnthropicKey(e.target.value)} id="anthropic-api-key" placeholder="sk-ant-..." style={{ width: '100%', boxSizing: 'border-box', background: 'var(--vscode-input-background)', color: 'var(--vscode-sideBar-foreground)', border: '1px solid transparent', padding: '4px 6px', fontSize: '12px', outline: 'none' }} />
                    </div>
                    <div style={{ marginBottom: '15px' }}>
                        <label style={{ display: 'block', marginBottom: '5px', color: 'var(--vscode-sideBar-foreground)', opacity: 0.8, fontSize: '11px' }}>Google API Key</label>
                        <input type="password" value={googleKey} onChange={(e) => setGoogleKey(e.target.value)} id="google-api-key" placeholder="AIza..." style={{ width: '100%', boxSizing: 'border-box', background: 'var(--vscode-input-background)', color: 'var(--vscode-sideBar-foreground)', border: '1px solid transparent', padding: '4px 6px', fontSize: '12px', outline: 'none' }} />
                    </div>
                    <button className="primary-button" id="save-api-keys" onClick={saveKeys} style={{ width: '100%', marginTop: '10px', background: 'var(--vscode-statusBar-background)', color: '#fff', border: 'none', padding: '6px', cursor: 'pointer', fontSize: '12px' }}>Save Keys</button>
                    {statusMessage && <div id="api-key-status" style={{ marginTop: '10px', fontSize: '11px', textAlign: 'center', color: '#89d185' }}>{statusMessage}</div>}
                </div>
            </div>

            <div id="search-view" className={`sidebar-section ${activeView === 'search-view' ? '' : 'hidden'}`}>
                <div className="sidebar-section-header">SEARCH</div>
                <div className="sidebar-search-container">
                    <input type="text" id="search-input" placeholder="Search" />
                </div>
                <div id="search-results" className="sidebar-content"></div>
            </div>

            <div id="extensions-view" className={`sidebar-section ${activeView === 'extensions-view' ? '' : 'hidden'}`}>
                <div className="sidebar-section-header" style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                    <div>EXTENSIONS</div>
                    <div className="extensions-actions" style={{ marginRight: '10px' }}>
                        <i className="codicon codicon-cloud-upload" id="install-vsix-btn" title="Install from VSIX..." style={{ cursor: 'pointer', fontSize: '14px' }}></i>
                    </div>
                </div>
                <div className="sidebar-search-container" style={{ padding: '10px' }}>
                    <input type="text" id="extensions-search-input" placeholder="Search Extensions in Marketplace" style={{ width: '100%', boxSizing: 'border-box', background: 'var(--vscode-input-background)', color: 'var(--vscode-sideBar-foreground)', border: '1px solid transparent', padding: '4px 6px', fontSize: '13px', outline: 'none' }} />
                </div>
                <div id="extensions-content" className="sidebar-content sidebar-accordion">
                    <div className="accordion-header" id="installed-accordion-header">
                        <i className="codicon codicon-chevron-down accordion-icon"></i>
                        <span className="accordion-title">Installed</span>
                    </div>
                    <div id="installed-extensions-list" className="accordion-content"></div>
                    <div className="accordion-header" id="marketplace-accordion-header">
                        <i className="codicon codicon-chevron-down accordion-icon"></i>
                        <span className="accordion-title">Marketplace</span>
                    </div>
                    <div id="marketplace-extensions-list" className="accordion-content"></div>
                </div>
            </div>

            <div id="specs-view" className={`sidebar-section ${activeView === 'specs-view' ? '' : 'hidden'}`}>
                <div className="sidebar-section-header" style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                    <div><i className="codicon codicon-chevron-down" style={{ marginRight: '4px' }}></i> SPECS</div>
                    <div className="specs-actions" style={{ marginRight: '10px' }}>
                        <i className="codicon codicon-add" id="new-spec" onClick={() => import('../specs').then(m => (m as any).createNewSpec())} title="New Spec" style={{ cursor: 'pointer' }}></i>
                    </div>
                </div>
                <div id="specs-content" className="sidebar-content" style={{ padding: '10px' }}>
                    <div className="specs-empty" id="specs-empty" style={{ fontSize: '12px', opacity: 0.6, fontStyle: 'italic' }}>No active specs. Create one to start a structured workflow.</div>
                    <div id="specs-list" className="specs-list"></div>
                </div>
            </div>

            <div id="mobile-view" className={`sidebar-section ${activeView === 'mobile-view' ? '' : 'hidden'}`} style={{ display: 'flex', flexDirection: 'column', height: '100%', overflow: 'hidden' }}>
                <div className="sidebar-header" style={{ display: 'flex', alignItems: 'center', padding: '0 12px', height: '35px', minHeight: '35px', borderBottom: '1px solid var(--vscode-panel-border)', background: 'var(--vscode-sideBar-background)' }}>
                    <span style={{ fontSize: '11px', fontWeight: 600, textTransform: 'uppercase', flex: 1, color: 'var(--vscode-sideBar-foreground)', opacity: 0.8 }}>MOBILE EMULATORS</span>
                    <i className="codicon codicon-refresh" id="refresh-mobile" title="Refresh Devices" style={{ cursor: 'pointer', fontSize: '14px', opacity: 0.8 }}></i>
                </div>
                <div style={{ flex: 1, overflow: 'hidden', display: 'flex', flexDirection: 'column' }}>
                    {activeDevice ? (
                        <EmulatorPanel />
                    ) : (
                        <div id="mobile-content" className="sidebar-content" style={{ padding: '0', height: '100%', overflowY: 'auto' }}>
                            <div style={{ fontSize: '12px', opacity: 0.6, textAlign: 'center', marginTop: '20px' }}>No devices connected.</div>
                        </div>
                    )}
                </div>
            </div>

            <div id="source-control-view" className={`sidebar-section ${activeView === 'scm-view' ? '' : 'hidden'}`} style={{ display: 'flex', flexDirection: 'column', height: '100%', overflow: 'hidden' }}>
                <div className="sidebar-header" style={{ display: 'flex', alignItems: 'center', padding: '0 12px', height: '35px', minHeight: '35px', borderBottom: '1px solid var(--vscode-panel-border)', background: 'var(--vscode-sideBar-background)' }}>
                    <span style={{ fontSize: '11px', fontWeight: 600, textTransform: 'uppercase', flex: 1, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap', color: 'var(--vscode-sideBar-foreground)', opacity: 0.8 }}>
                        GIT HISTORY
                    </span>
                    <div style={{ display: 'flex', gap: '8px', alignItems: 'center' }}>
                         <i className="codicon codicon-refresh" onClick={() => (window as any).refreshGitHistory?.()} style={{ cursor: 'pointer', fontSize: '14px', opacity: 0.8 }} title="Refresh History"></i>
                    </div>
                </div>
                <div style={{ flex: 1, overflow: 'hidden', display: 'flex', flexDirection: 'column' }}>
                    <div style={{ flex: 1, overflow: 'hidden' }}>
                        <GitGraph />
                    </div>
                </div>
            </div>

            <div id="debug-view" className={`sidebar-section ${activeView === 'debug-view' ? '' : 'hidden'}`}>
                <div className="sidebar-section-header">RUN AND DEBUG</div>
                <div className="debug-controls">
                    <button id="start-debug" className="debug-btn primary">Start Debugging</button>
                </div>
                <div id="debug-content" className="sidebar-content">
                    <div className="debug-section">
                        <div className="debug-section-header"><i className="codicon codicon-chevron-right" style={{ marginRight: '4px' }}></i> VARIABLES</div>
                        <div id="debug-variables" className="debug-section-content"></div>
                    </div>
                    <div className="debug-section">
                        <div className="debug-section-header"><i className="codicon codicon-chevron-right" style={{ marginRight: '4px' }}></i> WATCH</div>
                        <div id="debug-watch" className="debug-section-content"></div>
                    </div>
                    <div className="debug-section">
                        <div className="debug-section-header"><i className="codicon codicon-chevron-down" style={{ marginRight: '4px' }}></i> CALL STACK</div>
                        <div id="debug-callstack" className="debug-section-content"></div>
                    </div>
                    <div className="debug-section">
                        <div className="debug-section-header"><i className="codicon codicon-chevron-down" style={{ marginRight: '4px' }}></i> BREAKPOINTS</div>
                        <div id="debug-breakpoints" className="debug-section-content"></div>
                    </div>
                </div>
            </div>

            <div id="planning-view" className={`sidebar-section ${activeView === 'planning-view' ? '' : 'hidden'}`} style={{ display: 'flex', flexDirection: 'column', height: '100%' }}>
                <div className="sidebar-section-header">GIT PLANNING & WORKFLOW</div>
                <div style={{ flex: 1, overflow: 'hidden' }}>
                    <GitGraph />
                </div>
                <div id="planning-content" className="sidebar-content" style={{ padding: '15px', borderTop: '1px solid var(--vscode-panel-border)' }}>
                    <div style={{ color: 'var(--vscode-sideBar-foreground)', opacity: 0.6, fontSize: '11px', marginBottom: '10px' }}>ACTIVE TASK BOUNDARY</div>
                    <div style={{ background: 'var(--vscode-editor-background)', border: '1px solid var(--vscode-panel-border)', borderRadius: '4px', padding: '10px' }}>
                        <h4 style={{ color: '#fff', fontSize: '13px', marginBottom: '5px' }}>Workflow Strategy</h4>
                        <p style={{ color: 'var(--vscode-sideBar-foreground)', opacity: 0.8, fontSize: '12px' }}>Review git history to plan your next architectural move.</p>
                        <button id="plan-generate" style={{ width: '100%', marginTop: '10px', background: 'var(--vscode-button-background)', color: 'var(--vscode-button-foreground)', border: 'none', padding: '6px 10px', borderRadius: '2px', cursor: 'pointer', fontSize: '12px' }}>Generate Implementation Plan</button>
                    </div>
                </div>
            </div>
        </aside>
    );
};

export default Sidebar;
