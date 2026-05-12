import React, { useState, useCallback } from 'react';
import { useStore } from '../store';

// Window control helpers via Tauri global (withGlobalTauri: true)
function getTauriWindow() {
    const t = (window as any).__TAURI__;
    if (!t) {
        console.warn('[TitleBar] __TAURI__ not found');
        return null;
    }
    // Tauri v2: use the window namespace
    if (t.window?.getCurrentWindow) {
        return t.window.getCurrentWindow();
    }
    // Try appWindow for older setups
    if (t.appWindow) return t.appWindow;
    console.warn('[TitleBar] No window API found in __TAURI__');
    return null;
}

function winMinimize() {
    const win = getTauriWindow();
    if (!win) {
        console.warn('[TitleBar] Cannot minimize - no window');
        return;
    }
    // Tauri v2 uses minimize()
    if (typeof win.minimize === 'function') {
        win.minimize().catch(e => console.error('[TitleBar] minimize error:', e));
    } else {
        console.warn('[TitleBar] minimize not available');
    }
}

function winMaximize() {
    const win = getTauriWindow();
    if (!win) {
        console.warn('[TitleBar] Cannot maximize - no window');
        return;
    }
    // Tauri v2 uses toggleMaximize()
    if (typeof win.toggleMaximize === 'function') {
        win.toggleMaximize().catch(e => console.error('[TitleBar] toggleMaximize error:', e));
    } else {
        console.warn('[TitleBar] toggleMaximize not available');
    }
}

function winClose() {
    const win = getTauriWindow();
    if (!win) {
        console.warn('[TitleBar] Cannot close - no window');
        return;
    }
    // Tauri v2 uses close()
    if (typeof win.close === 'function') {
        win.close().catch(e => console.error('[TitleBar] close error:', e));
    } else {
        console.warn('[TitleBar] close not available');
    }
}

const TitleBar: React.FC = () => {
    const [activeMenu, setActiveMenu] = useState<string | null>(null);
    const agentModel = useStore(state => state.agentModel);

    const menus = [
        { label: 'File',      items: ['New File', 'New Window', 'Open...', 'Save', 'Close Editor'] },
        { label: 'Edit',      items: ['Undo', 'Redo', 'Cut', 'Copy', 'Paste', 'Find', 'Replace'] },
        { label: 'Selection', items: ['Select All', 'Expand Selection', 'Shrink Selection'] },
        { label: 'View',      items: ['Command Palette...', 'Explorer', 'Search', 'Source Control', 'Run', 'Extensions'] },
        { label: 'Go',        items: ['Back', 'Forward', 'Go to File...', 'Go to Symbol...'] },
        { label: 'Run',       items: ['Start Debugging', 'Run Without Debugging', 'Stop Debugging'] },
        { label: 'Terminal',  items: ['New Terminal', 'Split Terminal', 'Run Build Task...', 'Run Selected Text'] },
        { label: 'Help',      items: ['Welcome', 'Documentation', 'Show All Commands', 'About'] }
    ];

    const handleMenuClick = (menu: string) => {
        setActiveMenu(activeMenu === menu ? null : menu);
    };

    const handleItemClick = (item: string) => {
        const execute = (window as any).executeCommand;
        if (!execute) { console.error('Command system not initialized'); return; }

        switch (item) {
            case 'New File':           execute('explorer.newFile'); break;
            case 'New Folder':         execute('explorer.newFolder'); break;
            case 'Open...':            execute('explorer.openFolder'); break;
            case 'Save':               execute('workbench.action.files.save'); break;
            case 'Command Palette...': execute('workbench.action.showCommands'); break;
            case 'Welcome':            execute('workbench.action.showWelcome'); break;
            case 'New Terminal':       (window as any).spawnTerminal?.(); break;
            default:                   console.log(`Menu item clicked: ${item}`);
        }
        setActiveMenu(null);
    };

    return (
        <div id="title-bar" data-tauri-drag-region>
            {/* Left: nav + menus */}
            <div className="title-bar-left">
                {/* IDE Icon instead of nav arrows */}
                <div className="ide-logo hoverable" title="vscodium-rust ide" style={{ display: 'flex', alignItems: 'center', marginRight: '8px', padding: '4px 8px' }}>
                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" style={{ marginRight: '6px' }}>
                        <path d="M12 2L2 7l10 5 10-5-10-5z" fill="#c084fc"/>
                        <path d="M2 17l10 5 10-5M2 12l10 5 10-5" stroke="#c084fc" strokeWidth="2"/>
                    </svg>
                    <span style={{ fontSize: '11px', fontWeight: 600, color: 'rgba(255,255,255,0.7)' }}>vscodium-rust ide</span>
                </div>

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
                                <div className="menu-dropdown">
                                    {menu.items.map(item => (
                                        <div
                                            key={item}
                                            className="menu-dropdown-item"
                                            onClick={() => handleItemClick(item)}
                                        >
                                            {item}
                                        </div>
                                    ))}
                                </div>
                            )}
                        </div>
                    ))}
                </div>
            </div>

            {/* Center: command palette / title */}
            <div
                className="command-center"
                onClick={() => (window as any).showCommandPalette?.()}
                data-tauri-drag-region="false"
            >
                <div className="command-box">
                    <i className="codicon codicon-search" style={{ fontFamily: 'codicon', fontStyle: 'normal' }}></i>
                    <div className="text">
                        {(window as any).activeRootName || 'vscodium-rust'}
                    </div>
                </div>
            </div>

            {/* Right: layout toggles + window controls */}
            <div className="title-bar-right" data-tauri-drag-region="false">
                <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                    <div title="Privacy Guard Active" style={{ display: 'flex', alignItems: 'center', padding: '4px', opacity: 0.4, cursor: 'help' }}>
                        <i className="codicon codicon-shield" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px' }}></i>
                    </div>
                    <i
                        className="codicon codicon-robot hoverable"
                        style={{ fontFamily: 'codicon', fontStyle: 'normal' }}
                        title="Toggle AIRI Panel"
                        onClick={() => (window as any).useStore?.getState().toggleAiriPanel()}
                    ></i>
                    <i
                        className="codicon codicon-device-mobile hoverable"
                        style={{ fontFamily: 'codicon', fontStyle: 'normal' }}
                        title="Toggle Emulator Panel"
                        onClick={() => (window as any).useStore?.getState().toggleEmulatorPanel()}
                    ></i>
                </div>

                {/* Native window controls — always visible, no-drag region */}
                <div className="window-controls-right">
                    <button
                        className="wc-btn wc-minimize"
                        title="Minimize"
                        onClick={winMinimize}
                        aria-label="Minimize"
                    >
                        <svg width="10" height="1" viewBox="0 0 10 1">
                            <rect width="10" height="1" fill="currentColor" />
                        </svg>
                    </button>
                    <button
                        className="wc-btn wc-maximize"
                        title="Maximize / Restore"
                        onClick={winMaximize}
                        aria-label="Maximize"
                    >
                        <svg width="10" height="10" viewBox="0 0 10 10">
                            <rect x="0.5" y="0.5" width="9" height="9" fill="none" stroke="currentColor" strokeWidth="1" />
                        </svg>
                    </button>
                    <button
                        className="wc-btn wc-close"
                        title="Close"
                        onClick={winClose}
                        aria-label="Close"
                    >
                        <svg width="10" height="10" viewBox="0 0 10 10">
                            <line x1="0" y1="0" x2="10" y2="10" stroke="currentColor" strokeWidth="1.2" />
                            <line x1="10" y1="0" x2="0" y2="10" stroke="currentColor" strokeWidth="1.2" />
                        </svg>
                    </button>
                </div>
            </div>

            {activeMenu && (
                <div className="menu-overlay" onClick={() => setActiveMenu(null)}></div>
            )}
        </div>
    );
};

export default TitleBar;
