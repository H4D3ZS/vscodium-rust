import React, { useState, useCallback, useEffect, useRef } from 'react';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { invoke } from '../tauri_bridge';
import { useStore } from '../store';

// ---------------------------------------------------------------------------
// Tauri window helpers
// ---------------------------------------------------------------------------
// Window controls call native Rust commands (window_commands.rs) that operate
// on the AppHandle's "main" window directly. This bypasses the @tauri-apps/api
// window plugin — which was resolving "OK" but no-op'ing (likely JS/Rust
// version drift on a frameless window) — and does not depend on core:window
// capability grants. A JS fallback to getCurrentWindow() is kept for safety.
async function winMinimize() {
    console.log('[TitleBar] minimize clicked');
    try {
        await invoke('win_minimize');
        console.log('[TitleBar] minimize OK (native)');
    } catch (e) {
        console.error('[TitleBar] win_minimize failed, trying JS API:', e);
        try { await getCurrentWindow().minimize(); } catch (e2) { console.error('[TitleBar] JS minimize failed:', e2); }
    }
}

async function winMaximize() {
    console.log('[TitleBar] maximize clicked');
    try {
        const nowMax = await invoke<boolean>('win_toggle_maximize');
        console.log('[TitleBar] maximize toggled (native), maximized=', nowMax);
    } catch (e) {
        console.error('[TitleBar] win_toggle_maximize failed, trying JS API:', e);
        try { await getCurrentWindow().toggleMaximize(); } catch (e2) { console.error('[TitleBar] JS maximize failed:', e2); }
    }
}

async function winClose() {
    console.log('[TitleBar] close clicked');
    try {
        await invoke('win_close');
        console.log('[TitleBar] close OK (native)');
    } catch (e) {
        console.error('[TitleBar] win_close failed, trying JS API:', e);
        try { await getCurrentWindow().close(); } catch (e2) { console.error('[TitleBar] JS close failed:', e2); }
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
    submenu?: boolean;
}

interface Menu {
    label: string;
    items: MenuEntry[];
}

const menus: Menu[] = [
    {
        label: 'File',
        items: [
            { label: 'New Text File', shortcut: 'Ctrl+N' },
            { label: 'New Window', shortcut: 'Ctrl+Shift+N' },
            { label: 'New Agents Window' },
            { separator: true, label: '' },
            { label: 'Open Folder...', shortcut: 'Ctrl+K Ctrl+O' },
            { label: 'Add Folder to Workspace...' },
            { separator: true, label: '' },
            { label: 'Save', shortcut: 'Ctrl+S' },
            { label: 'Save As...', shortcut: 'Ctrl+Shift+S' },
            { label: 'Save All', shortcut: 'Ctrl+K S' },
            { label: 'Save Workspace As...' },
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
            { label: 'Codebase Search' },
            { label: 'Tasks & Specs' },
            { label: 'Steering & Hooks' },
            { separator: true, label: '' },
            { label: 'Toggle Chat Panel', shortcut: 'Ctrl+Alt+B' },
            { label: 'Open Chat' },
            { label: 'Mobile Emulators' },
            { label: 'Launch External Browser', shortcut: 'Ctrl+Shift+U' },
            { label: 'Security Review', shortcut: 'Ctrl+Shift+Alt+R' },
            { label: 'Visual Lab', shortcut: 'Ctrl+Shift+J' },
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
            { label: 'About' },
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
        case 'New Text File':
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
        case 'New Agents Window':
            store.openAiriPanel?.();
            void store.createNewSession?.();
            break;
        case 'Open Folder...':
        case 'Open...':
            exec?.('explorer.openFolder');
            break;
        case 'Add Folder to Workspace...':
        case 'Open Workspace...':
            exec?.('explorer.addFolderToWorkspace');
            break;
        case 'Save Workspace As...':
            exec?.('workbench.action.saveWorkspaceAs');
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
        case 'Codebase Search':
            store.setActiveSidebarView('vector-search-view');
            break;
        case 'Tasks & Specs':
            store.setActiveSidebarView('tasks-view');
            break;
        case 'Steering & Hooks':
            store.setActiveSidebarView('steering-view');
            break;
        case 'Toggle Chat Panel':
        case 'Toggle AIRI Panel':
            store.isAiriPanelOpen && store.isRightSidebarOpen ? store.closeAiriPanel() : store.openAiriPanel();
            break;
        case 'Open Chat':
            store.openAiriPanel();
            break;
        case 'Mobile Emulators':
            exec?.('workbench.action.openEmulators');
            break;
        case 'Launch External Browser':
            exec?.('workbench.action.openBrowser');
            break;
        case 'Security Review':
            exec?.('security.action.runCodebaseReview');
            break;
        case 'Visual Lab':
            exec?.('workbench.view.jsonVisualizer');
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
        case 'Toggle Breakpoint': {
            const path = store.activeEditorPath || store.tabs.find((t: any) => t.id === store.activeTabId)?.path;
            const ed = editor;
            const line = ed?.getPosition()?.lineNumber;
            if (path && line) store.toggleBreakpoint(path, line);
            else if (editor) editor.trigger('menu', 'editor.debug.action.toggleBreakpoint', null);
            break;
        }

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
        case 'About':
            alert('VSCodium-Rust\nVS Code–native shell · Agentic chat · Android & iPhone emulators\n\nCyber-Ifrit Cloud · Neural VFS · Local Ollama BYOB');
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
    const [gearOpen, setGearOpen] = useState(false);
    const gearRef = useRef<HTMLDivElement>(null);

    const isAiriPanelOpen = useStore(state => state.isAiriPanelOpen);
    const isAgentThinking = useStore(state => state.isAgentThinking);
    const activeRoot = useStore(state => state.activeRoot);
    // Native VSCode layout toggles (primary sidebar / panel / secondary sidebar).
    const isSidebarOpen = useStore(state => state.isSidebarOpen);
    const isBottomPanelOpen = useStore(state => state.isBottomPanelOpen);
    const isRightSidebarOpen = useStore(state => state.isRightSidebarOpen);
    const toggleSidebar = useStore(state => state.toggleSidebar);
    const toggleBottomPanel = useStore(state => state.toggleBottomPanel);
    // Browser preview (web-dev surface the agent can automate).
    const externalBrowserActive = useStore(state => state.externalBrowserActive);
    const setLayoutMode = useStore(state => state.setLayoutMode);
    const openSettings = useStore(state => state.openSettings);
    const setActiveSidebarView = useStore(state => state.setActiveSidebarView);

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

    // Close the gear (Manage) menu on outside click
    useEffect(() => {
        if (!gearOpen) return;
        const handler = (e: MouseEvent) => {
            if (gearRef.current && !gearRef.current.contains(e.target as Node)) setGearOpen(false);
        };
        document.addEventListener('mousedown', handler);
        return () => document.removeEventListener('mousedown', handler);
    }, [gearOpen]);

    // Gear (Manage) menu actions — VSCode/Antigravity entry points.
    const gearItems: { label: string; shortcut?: string; run: () => void; separatorBefore?: boolean }[] = [
        { label: 'Editor Settings', run: () => openSettings('user') },
        { label: 'Open IDE User Settings', shortcut: 'Ctrl+,', run: () => openSettings('user') },
        { label: 'Extensions', shortcut: 'Ctrl+Shift+X', separatorBefore: true, run: () => setActiveSidebarView('extensions-view') },
        { label: 'Keyboard Shortcuts', shortcut: 'Ctrl+K Ctrl+S', run: () => openSettings('user') },
        { label: 'Tasks', separatorBefore: true, run: () => { try { (window as any).executeCommand?.('workbench.action.tasks.runTask'); } catch { /* */ } } },
    ];

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

    // Programmatic drag — fire startDragging() on mousedown of the title bar
    // center (not on buttons). This is more reliable than data-tauri-drag-region
    // which captures OS events BEFORE React and blocks child button clicks.
    const handleTitleBarMouseDown = useCallback(async (e: React.MouseEvent<HTMLDivElement>) => {
        const target = e.target as HTMLElement;
        // Only drag if the click is on the bare title bar, NOT on interactive elements
        const isInteractive = target.closest('button, a, [role="button"], input, .menu-item-wrapper, .window-controls-right, .title-bar-right, .ide-logo');
        if (e.button === 0 && !isInteractive) {
            try {
                await invoke('win_start_drag');
            } catch {
                try { await getCurrentWindow().startDragging(); } catch { /* ignore */ }
            }
        }
    }, []);

    const isMac = typeof document !== 'undefined' && document.body.classList.contains('os-macos');

    return (
        <div
            id="title-bar"
            onMouseDown={handleTitleBarMouseDown}
            style={{
                background: 'var(--vscode-titleBar-activeBackground, #1e1e1e)',
                color: 'var(--vscode-titleBar-activeForeground, #cccccc)',
                borderBottom: '1px solid var(--vscode-panel-border, #2b2b2b)',
                zIndex: 100,
                position: 'relative',
            }}
        >
            {isMac && (
                <div className="window-controls-left" onMouseDown={(e) => e.stopPropagation()}>
                    <button type="button" className="wc-traffic wc-traffic-close" title="Close" aria-label="Close" onClick={(e) => { e.stopPropagation(); winClose(); }} />
                    <button type="button" className="wc-traffic wc-traffic-minimize" title="Minimize" aria-label="Minimize" onClick={(e) => { e.stopPropagation(); winMinimize(); }} />
                    <button type="button" className="wc-traffic wc-traffic-maximize" title="Maximize" aria-label="Maximize" onClick={(e) => { e.stopPropagation(); winMaximize(); }} />
                </div>
            )}
            {/* ── Left: logo + menus ── */}
            <div className="title-bar-left" ref={menuRef}>
                {/* IDE logo */}
                <div
                    className="ide-logo hoverable"
                    title={rootName}
                    style={{ display: 'flex', alignItems: 'center', marginRight: '4px', padding: '4px 8px', WebkitAppRegion: 'no-drag' } as React.CSSProperties}
                >
                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none">
                        <path d="M12 2L2 7l10 5 10-5-10-5z" fill="rgba(255,255,255,0.75)" />
                        <path d="M2 17l10 5 10-5M2 12l10 5 10-5" stroke="rgba(255,255,255,0.75)" strokeWidth="2" />
                    </svg>
                </div>

                {/* Menu bar */}
                <div className="menu-items-container">
                    {menus.map(menu => (
                        <div key={menu.label} className="menu-item-wrapper">
                            <div
                                className={`menu-label ${activeMenu === menu.label ? 'active' : ''}`}
                                onClick={() => handleMenuClick(menu.label)}
                                style={{ WebkitAppRegion: 'no-drag' } as React.CSSProperties}
                            >
                                {menu.label}
                            </div>

                            {activeMenu === menu.label && (
                                <div className="menu-dropdown" style={{ WebkitAppRegion: 'no-drag' } as React.CSSProperties}>
                                    {menu.items.map((item, idx) =>
                                        item.separator ? (
                                            <div key={idx} className="menu-dropdown-separator" />
                                        ) : (
                                            <div
                                                key={item.label}
                                                className={`menu-dropdown-item ${item.disabled ? 'disabled' : ''}`}
                                                onClick={() => handleItemClick(item)}
                                            >
                                                <span className="menu-dropdown-label">{item.label}</span>
                                                {item.submenu ? (
                                                    <span className="menu-submenu-arrow">›</span>
                                                ) : item.shortcut ? (
                                                    <span className="menu-shortcut">{item.shortcut}</span>
                                                ) : null}
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
                style={{
                    display: 'flex',
                    justifyContent: 'center',
                    alignItems: 'center',
                    flex: 1,
                }}
            >
                <div
                    className="command-box"
                    onClick={() => executeMenuAction('Command Palette...')}
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
                        WebkitAppRegion: 'no-drag',
                    } as React.CSSProperties}
                >
                    <i className="codicon codicon-search" style={{ fontSize: '11px', marginRight: '10px', opacity: 0.5 }} />
                    <div style={{ flex: 1 }}>{rootName}</div>
                </div>
            </div>

            {/* ── Right: layout toggles + AIRI button + window controls ── */}
            <div className="title-bar-right" style={{ WebkitAppRegion: 'no-drag' } as React.CSSProperties}>
                {/* Gear (Manage) menu — Antigravity-style settings entry points */}
                <div ref={gearRef} style={{ position: 'relative', marginRight: '4px' }}>
                    <i
                        className={`codicon codicon-settings-gear tb-layout-btn${gearOpen ? ' tb-layout-active' : ''}`}
                        title="Manage"
                        onClick={() => setGearOpen(o => !o)}
                    />
                    {gearOpen && (
                        <div className="vscode-native-menu" style={{
                            position: 'absolute', top: '100%', right: 0, marginTop: 0, minWidth: 250,
                            background: 'var(--vscode-menu-background, #252526)',
                            border: '1px solid var(--vscode-menu-border, #454545)',
                            zIndex: 99999,
                            padding: '0', WebkitAppRegion: 'no-drag',
                        } as React.CSSProperties}>
                            {gearItems.map((it) => (
                                <React.Fragment key={it.label}>
                                    {it.separatorBefore && <div style={{ height: 1, background: 'rgba(255,255,255,0.08)', margin: '4px 0' }} />}
                                    <div
                                        onClick={() => { it.run(); setGearOpen(false); }}
                                        style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', gap: 24, padding: '6px 14px', cursor: 'pointer', fontSize: 12, color: 'var(--vscode-menu-foreground, #ccc)' }}
                                        onMouseEnter={e => (e.currentTarget.style.background = 'var(--vscode-menu-selectionBackground, rgba(255,255,255,0.08))')}
                                        onMouseLeave={e => (e.currentTarget.style.background = 'transparent')}
                                    >
                                        <span>{it.label}</span>
                                        {it.shortcut && <span style={{ opacity: 0.5, fontSize: 11 }}>{it.shortcut}</span>}
                                    </div>
                                </React.Fragment>
                            ))}
                        </div>
                    )}
                </div>

                {/* Native VSCode layout-toggle cluster */}
                <div style={{ display: 'flex', alignItems: 'center', gap: '2px', marginRight: '6px' }}>
                    <i
                        className={`codicon codicon-globe tb-layout-btn${externalBrowserActive ? ' tb-layout-active' : ''}`}
                        title="External Bug Bounty Browser (Ctrl+Shift+U) — real Firefox window you watch live; agent drives it"
                        onClick={() => {
                            import('../application/browser/openBrowserPanel').then(m => m.toggleExternalBrowser());
                        }}
                    />
                    <span style={{ width: 1, height: 14, background: 'var(--vscode-panel-border, rgba(255,255,255,0.12))', margin: '0 4px' }} />
                    <i
                        className={`codicon codicon-layout-sidebar-left${isSidebarOpen ? '' : '-off'} tb-layout-btn`}
                        title="Toggle Primary Side Bar (Ctrl+B)"
                        onClick={() => toggleSidebar()}
                    />
                    <i
                        className={`codicon codicon-layout-panel${isBottomPanelOpen ? '' : '-off'} tb-layout-btn`}
                        title="Toggle Panel (Ctrl+J)"
                        onClick={() => toggleBottomPanel()}
                    />
                    <i
                        className={`codicon codicon-layout-sidebar-right${isRightSidebarOpen ? '' : '-off'} tb-layout-btn`}
                        title="Toggle Secondary Side Bar (Ctrl+Alt+B)"
                        onClick={() => useStore.getState().toggleRightSidebar()}
                    />
                </div>

                <div style={{ display: 'flex', alignItems: 'center', gap: '4px', marginRight: '12px' }}>

                    {/* Sentient Guard icon */}
                    <div
                        title="Sentient Guard"
                        style={{
                            display: 'flex',
                            alignItems: 'center',
                            padding: '4px',
                            opacity: isAiriPanelOpen ? 1 : 0.3,
                            color: isAiriPanelOpen ? 'var(--vscode-titleBar-activeForeground, #ccc)' : 'inherit',
                            transition: 'all 0.3s cubic-bezier(0.4, 0, 0.2, 1)',
                        }}
                    >
                        <i className="codicon codicon-shield" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '13px' }} />
                    </div>

                    {/* AIRI toggle button */}
                    <div
                        className={`title-toggle-btn ${isAiriPanelOpen ? 'active' : ''}`}
                        title={isAiriPanelOpen ? 'Close Chat Panel' : 'Open Chat Panel'}
                        style={{
                            display: 'flex',
                            alignItems: 'center',
                            gap: '6px',
                            padding: '4px 10px',
                            borderRadius: '2px',
                            fontSize: '11px',
                            fontWeight: 600,
                            cursor: 'pointer',
                            background: isAiriPanelOpen ? 'var(--vscode-toolbar-hoverBackground, rgba(255,255,255,0.08))' : 'transparent',
                            color: isAiriPanelOpen ? 'var(--vscode-titleBar-activeForeground, #ccc)' : 'rgba(255,255,255,0.5)',
                            border: '1px solid transparent',
                            WebkitAppRegion: 'no-drag',
                        } as React.CSSProperties}
                        onClick={handleAiriClick}
                    >
                        {isAgentThinking ? (
                            <i className="codicon codicon-loading codicon-modifier-spin" style={{ fontSize: '14px' }} />
                        ) : (
                            <i className="codicon codicon-comment-discussion" style={{ fontSize: '14px' }} />
                        )}
                        <span style={{ letterSpacing: '0.3px', whiteSpace: 'nowrap' }}>Chat</span>

                        {isAiriPanelOpen && (
                            <i
                                className="codicon codicon-add hoverable"
                                title="New Chat (Ctrl+T)"
                                onClick={handleNewChat}
                                style={{
                                    fontSize: '13px',
                                    paddingLeft: '6px',
                                    marginLeft: '6px',
                                    borderLeft: '1px solid var(--vscode-sideBar-border, rgba(255,255,255,0.12))',
                                    color: 'rgba(255,255,255,0.6)',
                                }}
                            />
                        )}
                    </div>
                </div>

                {/* Native window controls — pointer-events + no-drag so clicks always reach buttons */}
                <div
                    className="window-controls-right"
                    style={{ pointerEvents: 'all', userSelect: 'none', flexShrink: 0, WebkitAppRegion: 'no-drag' } as React.CSSProperties}
                    onMouseDown={(e) => e.stopPropagation()}
                    onClick={(e) => e.stopPropagation()}
                >
                    <button
                        className="wc-btn wc-minimize"
                        title="Minimize"
                        onClick={(e) => {
                            e.preventDefault();
                            e.stopPropagation();
                            winMinimize();
                        }}
                        onMouseDown={(e) => e.stopPropagation()}
                        aria-label="Minimize"
                    >
                        <svg width="10" height="1" viewBox="0 0 10 1">
                            <rect width="10" height="1" fill="currentColor" />
                        </svg>
                    </button>
                    <button
                        className="wc-btn wc-maximize"
                        title="Maximize / Restore"
                        onClick={(e) => {
                            e.preventDefault();
                            e.stopPropagation();
                            winMaximize();
                        }}
                        onMouseDown={(e) => e.stopPropagation()}
                        aria-label="Maximize"
                    >
                        <svg width="10" height="10" viewBox="0 0 10 10">
                            <rect x="0.5" y="0.5" width="9" height="9" fill="none" stroke="currentColor" strokeWidth="1" />
                        </svg>
                    </button>
                    <button
                        className="wc-btn wc-close"
                        title="Close"
                        onClick={(e) => {
                            e.preventDefault();
                            e.stopPropagation();
                            winClose();
                        }}
                        onMouseDown={(e) => e.stopPropagation()}
                        aria-label="Close"
                    >
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
