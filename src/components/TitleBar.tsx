import React, { useState, useCallback, useEffect, useRef } from 'react';
import { useStore } from '../store';

// ---------------------------------------------------------------------------
// Tauri window helpers
// ---------------------------------------------------------------------------
function getTauriWindow() {
    const t = (window as any).__TAURI__;
    if (!t) {
        console.warn('[TitleBar] __TAURI__ not found');
        return null;
    }
    if (t.window?.getCurrentWindow) return t.window.getCurrentWindow();
    if (t.appWindow) return t.appWindow;
    console.warn('[TitleBar] No window API found in __TAURI__');
    return null;
}

function winMinimize() {
    const win = getTauriWindow();
    if (!win) return;
    if (typeof win.minimize === 'function') {
        win.minimize().catch((e: unknown) => console.error('[TitleBar] minimize error:', e));
    }
}

async function winMaximize() {
    const win = getTauriWindow();
    if (!win) return;
    try {
        if (typeof win.isFullscreen === 'function' && await win.isFullscreen()) {
            await win.setFullscreen(false);
            return;
        }
        if (typeof win.toggleMaximize === 'function') {
            await win.toggleMaximize();
            return;
        }
        const isMaximized = typeof win.isMaximized === 'function' ? await win.isMaximized() : false;
        if (isMaximized && typeof win.unmaximize === 'function') {
            await win.unmaximize();
        } else if (typeof win.maximize === 'function') {
            await win.maximize();
        }
    } catch (e) {
        console.warn('[TitleBar] maximize failed:', e);
    }
}

function winClose() {
    const win = getTauriWindow();
    if (!win) return;
    if (typeof win.close === 'function') {
        win.close().catch((e: unknown) => console.error('[TitleBar] close error:', e));
    }
}

// ---------------------------------------------------------------------------
// Menu definition
// ---------------------------------------------------------------------------
interface MenuEntry {
    label: string;
    shortcut?: string;
    separator?: boolean;
    disabled?: boolean;
}

interface Menu {
    label: string;
    items: MenuEntry[];
}

const menus: Menu[] = [
    {
        label: 'File',
        items: [
            { label: 'New File', shortcut: 'Ctrl+N' },
            { label: 'New Folder' },
            { label: 'New Window', shortcut: 'Ctrl+Shift+N' },
            { separator: true, label: '' },
            { label: 'Open...', shortcut: 'Ctrl+K Ctrl+O' },
            { label: 'Open Workspace...' },
            { separator: true, label: '' },
            { label: 'Save', shortcut: 'Ctrl+S' },
            { label: 'Save As...', shortcut: 'Ctrl+Shift+S' },
            { label: 'Save All', shortcut: 'Ctrl+K S' },
            { separator: true, label: '' },
            { label: 'Close Editor', shortcut: 'Ctrl+W' },
            { label: 'Close Folder' },
        ],
    },
    {
        label: 'Edit',
        items: [
            { label: 'Undo', shortcut: 'Ctrl+Z' },
            { label: 'Redo', shortcut: 'Ctrl+Y' },
            { separator: true, label: '' },
            { label: 'Cut', shortcut: 'Ctrl+X' },
            { label: 'Copy', shortcut: 'Ctrl+C' },
            { label: 'Paste', shortcut: 'Ctrl+V' },
            { separator: true, label: '' },
            { label: 'Find', shortcut: 'Ctrl+F' },
            { label: 'Replace', shortcut: 'Ctrl+H' },
            { label: 'Find in Files', shortcut: 'Ctrl+Shift+F' },
        ],
    },
    {
        label: 'Selection',
        items: [
            { label: 'Select All', shortcut: 'Ctrl+A' },
            { label: 'Expand Selection', shortcut: 'Shift+Alt+Right' },
            { label: 'Shrink Selection', shortcut: 'Shift+Alt+Left' },
            { separator: true, label: '' },
            { label: 'Copy Line Down', shortcut: 'Shift+Alt+Down' },
            { label: 'Move Line Down', shortcut: 'Alt+Down' },
            { label: 'Move Line Up', shortcut: 'Alt+Up' },
            { label: 'Duplicate Selection' },
        ],
    },
    {
        label: 'View',
        items: [
            { label: 'Command Palette...', shortcut: 'Ctrl+Shift+P' },
            { separator: true, label: '' },
            { label: 'Explorer', shortcut: 'Ctrl+Shift+E' },
            { label: 'Search', shortcut: 'Ctrl+Shift+F' },
            { label: 'Source Control', shortcut: 'Ctrl+Shift+G' },
            { label: 'Extensions', shortcut: 'Ctrl+Shift+X' },
            { separator: true, label: '' },
            { label: 'Toggle AIRI Panel', shortcut: 'Ctrl+Alt+B' },
            { label: 'Toggle Terminal', shortcut: 'Ctrl+`' },
            { label: 'Toggle Sidebar', shortcut: 'Ctrl+B' },
            { separator: true, label: '' },
            { label: 'Word Wrap', shortcut: 'Alt+Z' },
            { label: 'Zoom In', shortcut: 'Ctrl+=' },
            { label: 'Zoom Out', shortcut: 'Ctrl+-' },
        ],
    },
    {
        label: 'Go',
        items: [
            { label: 'Go to File...', shortcut: 'Ctrl+P' },
            { label: 'Go to Symbol...', shortcut: 'Ctrl+Shift+O' },
            { label: 'Go to Line...', shortcut: 'Ctrl+G' },
            { separator: true, label: '' },
            { label: 'Go Back', shortcut: 'Alt+Left' },
            { label: 'Go Forward', shortcut: 'Alt+Right' },
        ],
    },
    {
        label: 'Run',
        items: [
            { label: 'Start Debugging', shortcut: 'F5' },
            { label: 'Run Without Debugging', shortcut: 'Ctrl+F5' },
            { label: 'Stop Debugging', shortcut: 'Shift+F5' },
            { separator: true, label: '' },
            { label: 'Toggle Breakpoint', shortcut: 'F9' },
        ],
    },
    {
        label: 'Terminal',
        items: [
            { label: 'New Terminal', shortcut: 'Ctrl+Shift+`' },
            { label: 'Split Terminal', shortcut: 'Ctrl+Shift+5' },
            { separator: true, label: '' },
            { label: 'Run Build Task', shortcut: 'Ctrl+Shift+B' },
            { label: 'Run Selected Text' },
        ],
    },
    {
        label: 'Help',
        items: [
            { label: 'Welcome' },
            { label: 'Command Palette...', shortcut: 'Ctrl+Shift+P' },
            { label: 'Show All Commands' },
            { separator: true, label: '' },
            { label: 'About Antigravity IDE' },
        ],
    },
];

// ---------------------------------------------------------------------------
// Command executor
// ---------------------------------------------------------------------------
function executeMenuAction(item: string) {
    const exec = (window as any).executeCommand as ((id: string) => void) | undefined;
    const store = useStore.getState();
    const editor = (window as any).activeEditor;

    switch (item) {
        // ── File ──────────────────────────────────────────────────────────
        case 'New File':
            exec?.('explorer.newFile');
            break;
        case 'New Folder':
            exec?.('explorer.newFolder');
            break;
        case 'New Window':
            // Best-effort: open a new browser tab in web mode, otherwise alert
            try {
                window.open(window.location.href, '_blank');
            } catch {
                alert('New Window is only supported in the Tauri desktop shell.');
            }
            break;
        case 'Open...':
            exec?.('explorer.openFolder');
            break;
        case 'Open Workspace...':
            exec?.('explorer.openFolder');
            break;
        case 'Save':
            exec?.('workbench.action.files.save');
            break;
        case 'Save As...':
            store.saveActiveFile();
            break;
        case 'Save All':
            store.saveActiveFile();
            break;
        case 'Close Editor':
            exec?.('workbench.action.closeActiveEditor');
            break;
        case 'Close Folder':
            exec?.('workbench.action.closeFolder');
            break;

        // ── Edit ──────────────────────────────────────────────────────────
        case 'Undo':
            if (editor) {
                editor.trigger('menu', 'undo', null);
            } else {
                document.execCommand('undo');
            }
            break;
        case 'Redo':
            if (editor) {
                editor.trigger('menu', 'redo', null);
            } else {
                document.execCommand('redo');
            }
            break;
        case 'Cut':
            if (editor) {
                editor.focus();
                editor.trigger('menu', 'editor.action.clipboardCutAction', null);
            } else {
                document.execCommand('cut');
            }
            break;
        case 'Copy':
            if (editor) {
                editor.focus();
                editor.trigger('menu', 'editor.action.clipboardCopyAction', null);
            } else {
                document.execCommand('copy');
            }
            break;
        case 'Paste':
            if (editor) {
                editor.focus();
                editor.trigger('menu', 'editor.action.clipboardPasteAction', null);
            } else {
                document.execCommand('paste');
            }
            break;
        case 'Find':
            if (editor) {
                editor.focus();
                editor.trigger('menu', 'actions.find', null);
            } else {
                exec?.('workbench.action.findInFiles');
            }
            break;
        case 'Replace':
            if (editor) {
                editor.focus();
                editor.trigger('menu', 'editor.action.startFindReplaceAction', null);
            }
            break;
        case 'Find in Files':
            store.setActiveSidebarView('search-view'); // also sets isSidebarOpen=true
            break;

        // ── Selection ────────────────────────────────────────────────────
        case 'Select All':
            if (editor) {
                editor.focus();
                editor.trigger('menu', 'editor.action.selectAll', null);
            } else {
                document.execCommand('selectAll');
            }
            break;
        case 'Expand Selection':
            if (editor) {
                editor.focus();
                editor.trigger('menu', 'editor.action.smartSelect.expand', null);
            }
            break;
        case 'Shrink Selection':
            if (editor) {
                editor.focus();
                editor.trigger('menu', 'editor.action.smartSelect.shrink', null);
            }
            break;
        case 'Copy Line Down':
            if (editor) editor.trigger('menu', 'editor.action.copyLinesDownAction', null);
            break;
        case 'Move Line Down':
            if (editor) editor.trigger('menu', 'editor.action.moveLinesDownAction', null);
            break;
        case 'Move Line Up':
            if (editor) editor.trigger('menu', 'editor.action.moveLinesUpAction', null);
            break;
        case 'Duplicate Selection':
            if (editor) editor.trigger('menu', 'editor.action.duplicateSelection', null);
            break;

        // ── View ─────────────────────────────────────────────────────────
        case 'Command Palette...':
        case 'Show All Commands':
            exec?.('workbench.action.showCommands');
            break;
        case 'Explorer':
            store.setActiveSidebarView('explorer-view');
            break;
        case 'Search':
            store.setActiveSidebarView('search-view');
            break;
        case 'Source Control':
            store.setActiveSidebarView('scm-view');
            break;
        case 'Extensions':
            store.setActiveSidebarView('extensions-view');
            break;
        case 'Toggle AIRI Panel':
            store.isAiriPanelOpen ? store.closeAiriPanel() : store.openAiriPanel();
            break;
        case 'Toggle Terminal':
            store.toggleBottomPanel();
            break;
        case 'Toggle Sidebar':
            store.toggleSidebar();
            break;
        case 'Word Wrap':
            exec?.('editor.action.wordWrap');
            break;
        case 'Zoom In':
            exec?.('workbench.action.zoomIn');
            break;
        case 'Zoom Out':
            exec?.('workbench.action.zoomOut');
            break;

        // ── Go ────────────────────────────────────────────────────────────
        case 'Go to File...':
            exec?.('workbench.action.showCommands');
            break;
        case 'Go to Symbol...':
            exec?.('workbench.action.gotoSymbol');
            break;
        case 'Go to Line...':
            exec?.('editor.action.gotoLine');
            break;
        case 'Go Back':
            if (editor) editor.trigger('menu', 'workbench.action.navigateBack', null);
            break;
        case 'Go Forward':
            if (editor) editor.trigger('menu', 'workbench.action.navigateForward', null);
            break;

        // ── Run ───────────────────────────────────────────────────────────
        case 'Start Debugging':
            store.setActiveSidebarView('run-view');
            break;
        case 'Run Without Debugging':
            (window as any).spawnTerminal?.();
            break;
        case 'Stop Debugging':
            // Signal stop via global if available
            (window as any).stopActiveTask?.();
            break;
        case 'Toggle Breakpoint':
            if (editor) editor.trigger('menu', 'editor.debug.action.toggleBreakpoint', null);
            break;

        // ── Terminal ──────────────────────────────────────────────────────
        case 'New Terminal':
            store.addTerminalGroup();
            if (!useStore.getState().isBottomPanelOpen) store.toggleBottomPanel();
            break;
        case 'Split Terminal':
            store.addTerminalGroup();
            if (!useStore.getState().isBottomPanelOpen) store.toggleBottomPanel();
            break;
        case 'Run Build Task':
            exec?.('workbench.action.tasks.build');
            break;
        case 'Run Selected Text':
            if (editor) {
                const selection = editor.getSelection?.();
                if (selection) {
                    const text = editor.getModel?.()?.getValueInRange?.(selection);
                    if (text) (window as any).runInTerminal?.(text);
                }
            }
            break;

        // ── Help ──────────────────────────────────────────────────────────
        case 'Welcome':
            exec?.('workbench.action.showWelcome');
            break;
        case 'About Antigravity IDE':
            alert('Antigravity IDE\nBuilt on VSCodium · Powered by AIRI · Kortex Neural Engine\n\nPrivacy-first · Offline-capable · Local AI');
            break;

        default:
            console.log(`[TitleBar] Unhandled menu action: ${item}`);
    }
}

// ---------------------------------------------------------------------------
// TitleBar component
// ---------------------------------------------------------------------------
const TitleBar: React.FC = () => {
    const [activeMenu, setActiveMenu] = useState<string | null>(null);
    const menuRef = useRef<HTMLDivElement>(null);

    const isAiriPanelOpen = useStore(state => state.isAiriPanelOpen);
    const isAgentThinking = useStore(state => state.isAgentThinking);
    const activeRoot = useStore(state => state.activeRoot);

    // Close menu on outside click
    useEffect(() => {
        if (!activeMenu) return;
        const handler = (e: MouseEvent) => {
            if (menuRef.current && !menuRef.current.contains(e.target as Node)) {
                setActiveMenu(null);
            }
        };
        document.addEventListener('mousedown', handler);
        return () => document.removeEventListener('mousedown', handler);
    }, [activeMenu]);

    const handleMenuClick = useCallback((label: string) => {
        setActiveMenu(prev => (prev === label ? null : label));
    }, []);

    const handleItemClick = useCallback((item: MenuEntry) => {
        if (item.separator || item.disabled) return;
        executeMenuAction(item.label);
        setActiveMenu(null);
    }, []);

    const handleAiriClick = useCallback(() => {
        const s = useStore.getState();
        if (s.isAiriPanelOpen) {
            s.closeAiriPanel?.();
        } else {
            s.openAiriPanel?.();
        }
    }, []);

    const handleNewChat = useCallback((e: React.MouseEvent) => {
        e.stopPropagation();
        useStore.getState().createNewSession?.();
    }, []);

    const rootName = activeRoot
        ? activeRoot.split(/[\\/]/).filter(Boolean).pop() ?? 'vscodium-rust'
        : 'vscodium-rust';

    return (
        <div
            id="title-bar"
            data-tauri-drag-region
            style={{
                background: isAiriPanelOpen ? 'linear-gradient(to right, #1a1a1a, #2d1b4e)' : '#1e1e1e',
                borderBottom: isAiriPanelOpen ? '1px solid rgba(168, 85, 247, 0.4)' : '1px solid #2b2b2b',
                boxShadow: isAiriPanelOpen ? '0 4px 20px rgba(168, 85, 247, 0.15)' : 'none',
                transition: 'all 0.5s cubic-bezier(0.4, 0, 0.2, 1)',
                zIndex: 100,
            }}
        >
            {/* ── Left: logo + menus ── */}
            <div className="title-bar-left" ref={menuRef}>
                {/* IDE logo */}
                <div
                    className="ide-logo hoverable"
                    title="Antigravity IDE"
                    style={{ display: 'flex', alignItems: 'center', marginRight: '4px', padding: '4px 8px' }}
                >
                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none">
                        <path d="M12 2L2 7l10 5 10-5-10-5z" fill={isAiriPanelOpen ? '#a855f7' : 'rgba(255,255,255,0.75)'} />
                        <path d="M2 17l10 5 10-5M2 12l10 5 10-5" stroke={isAiriPanelOpen ? '#c084fc' : 'rgba(255,255,255,0.75)'} strokeWidth="2" />
                    </svg>
                </div>

                {/* Menu bar */}
                <div className="menu-items-container">
                    {menus.map(menu => (
                        <div key={menu.label} className="menu-item-wrapper">
                            <div
                                className={`menu-label ${activeMenu === menu.label ? 'active' : ''}`}
                                onClick={() => handleMenuClick(menu.label)}
                                data-tauri-drag-region="false"
                            >
                                {menu.label}
                            </div>

                            {activeMenu === menu.label && (
                                <div className="menu-dropdown" data-tauri-drag-region="false">
                                    {menu.items.map((item, idx) =>
                                        item.separator ? (
                                            <div key={idx} className="menu-dropdown-separator" />
                                        ) : (
                                            <div
                                                key={item.label}
                                                className={`menu-dropdown-item ${item.disabled ? 'disabled' : ''}`}
                                                onClick={() => handleItemClick(item)}
                                            >
                                                <span>{item.label}</span>
                                                {item.shortcut && (
                                                    <span className="menu-shortcut">{item.shortcut}</span>
                                                )}
                                            </div>
                                        )
                                    )}
                                </div>
                            )}
                        </div>
                    ))}
                </div>
            </div>

            {/* ── Center: command palette trigger ── */}
            <div
                className="command-center"
                onClick={() => executeMenuAction('Command Palette...')}
                data-tauri-drag-region="false"
                style={{
                    display: 'flex',
                    justifyContent: 'center',
                    alignItems: 'center',
                    flex: 1,
                }}
            >
                <div
                    className="command-box"
                    style={{
                        display: 'flex',
                        alignItems: 'center',
                        background: 'rgba(255,255,255,0.03)',
                        border: '1px solid rgba(255,255,255,0.08)',
                        borderRadius: '8px',
                        padding: '4px 16px',
                        minWidth: '280px',
                        maxWidth: '500px',
                        cursor: 'text',
                        transition: 'all 0.2s',
                        fontSize: '11px',
                        color: 'rgba(255,255,255,0.6)',
                    }}
                >
                    <i className="codicon codicon-search" style={{ fontSize: '11px', marginRight: '10px', opacity: 0.5 }} />
                    <div style={{ flex: 1 }}>{rootName}</div>
                </div>
            </div>

            {/* ── Right: AIRI button + window controls ── */}
            <div className="title-bar-right" data-tauri-drag-region="false">
                <div style={{ display: 'flex', alignItems: 'center', gap: '4px', marginRight: '12px' }}>

                    {/* Sentient Guard icon */}
                    <div
                        title="Sentient Guard"
                        style={{
                            display: 'flex',
                            alignItems: 'center',
                            padding: '4px',
                            opacity: isAiriPanelOpen ? 1 : 0.3,
                            color: isAiriPanelOpen ? '#a855f7' : 'inherit',
                            transition: 'all 0.3s cubic-bezier(0.4, 0, 0.2, 1)',
                        }}
                    >
                        <i className="codicon codicon-shield" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '13px' }} />
                    </div>

                    {/* AIRI toggle button */}
                    <div
                        className={`title-toggle-btn ${isAiriPanelOpen ? 'active' : ''}`}
                        title={isAiriPanelOpen ? 'Close AIRI Panel' : 'Open AIRI Panel'}
                        style={{
                            display: 'flex',
                            alignItems: 'center',
                            gap: '6px',
                            padding: '4px 10px',
                            borderRadius: '6px',
                            fontSize: '11px',
                            fontWeight: 600,
                            cursor: 'pointer',
                            background: isAiriPanelOpen ? 'rgba(168, 85, 247, 0.15)' : 'transparent',
                            color: isAiriPanelOpen ? '#c084fc' : 'rgba(255,255,255,0.4)',
                            border: isAiriPanelOpen ? '1px solid rgba(168, 85, 247, 0.3)' : '1px solid transparent',
                            transition: 'all 0.2s ease',
                            position: 'relative',
                        }}
                        onClick={handleAiriClick}
                        data-tauri-drag-region="false"
                    >
                        {isAgentThinking ? (
                            <i className="codicon codicon-loading codicon-modifier-spin" style={{ fontSize: '14px', color: '#c084fc' }} />
                        ) : (
                            <i className="codicon codicon-robot" style={{ fontSize: '14px' }} />
                        )}
                        <span style={{ letterSpacing: '0.5px' }}>AIRI</span>

                        {isAiriPanelOpen && (
                            <i
                                className="codicon codicon-add hoverable"
                                title="New Chat (Ctrl+T)"
                                onClick={handleNewChat}
                                style={{
                                    fontSize: '13px',
                                    paddingLeft: '6px',
                                    marginLeft: '6px',
                                    borderLeft: '1px solid rgba(168, 85, 247, 0.3)',
                                    color: 'rgba(255,255,255,0.6)',
                                }}
                            />
                        )}
                    </div>
                </div>

                {/* Native window controls */}
                <div className="window-controls-right">
                    <button className="wc-btn wc-minimize" title="Minimize" onClick={winMinimize} aria-label="Minimize">
                        <svg width="10" height="1" viewBox="0 0 10 1">
                            <rect width="10" height="1" fill="currentColor" />
                        </svg>
                    </button>
                    <button className="wc-btn wc-maximize" title="Maximize / Restore" onClick={winMaximize} aria-label="Maximize">
                        <svg width="10" height="10" viewBox="0 0 10 10">
                            <rect x="0.5" y="0.5" width="9" height="9" fill="none" stroke="currentColor" strokeWidth="1" />
                        </svg>
                    </button>
                    <button className="wc-btn wc-close" title="Close" onClick={winClose} aria-label="Close">
                        <svg width="10" height="10" viewBox="0 0 10 10">
                            <line x1="0" y1="0" x2="10" y2="10" stroke="currentColor" strokeWidth="1.2" />
                            <line x1="10" y1="0" x2="0" y2="10" stroke="currentColor" strokeWidth="1.2" />
                        </svg>
                    </button>
                </div>
            </div>
        </div>
    );
};

export default TitleBar;
