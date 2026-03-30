import React, { useState } from 'react';
import { useStore } from '../store';
import { invoke } from '../tauri_bridge';

const TitleBar: React.FC = () => {
    const [activeMenu, setActiveMenu] = useState<string | null>(null);
    const agentModel = useStore(state => state.agentModel);
    const ollamaStatus = useStore(state => state.ollamaStatus);

    const menus = [
        { label: 'File', items: ['New File', 'New Window', 'Open...', 'Save', 'Close Editor'] },
        { label: 'Edit', items: ['Undo', 'Redo', 'Cut', 'Copy', 'Paste', 'Find', 'Replace'] },
        { label: 'Selection', items: ['Select All', 'Expand Selection', 'Shrink Selection'] },
        { label: 'View', items: ['Command Palette...', 'Explorer', 'Search', 'Source Control', 'Run', 'Extensions'] },
        { label: 'Go', items: ['Back', 'Forward', 'Go to File...', 'Go to Symbol...'] },
        { label: 'Run', items: ['Start Debugging', 'Run Without Debugging', 'Stop Debugging'] },
        { label: 'Terminal', items: ['New Terminal', 'Split Terminal', 'Run Build Task...', 'Run Selected Text'] },
        { label: 'Help', items: ['Welcome', 'Documentation', 'Show All Commands', 'About'] }
    ];

    const handleMenuClick = (menu: string) => {
        setActiveMenu(activeMenu === menu ? null : menu);
    };

    const handleItemClick = (item: string) => {
        const execute = (window as any).executeCommand;
        if (!execute) {
            console.error('Command system not initialized');
            return;
        }

        switch (item) {
            case 'New File': execute('explorer.newFile'); break;
            case 'New Folder': execute('explorer.newFolder'); break;
            case 'Open...': execute('explorer.openFolder'); break;
            case 'Save': execute('workbench.action.files.save'); break;
            case 'Command Palette...': execute('workbench.action.showCommands'); break;
            case 'Welcome': execute('workbench.action.showWelcome'); break;
            case 'New Terminal': (window as any).spawnTerminal?.(); break;
            default: console.log(`Menu item clicked: ${item}`);
        }
        setActiveMenu(null);
    };

    return (
        <div id="title-bar" data-tauri-drag-region>
            <div className="title-bar-left">
                <div className="window-controls-spacer"></div>
                <div className="navigation-controls" style={{ display: 'flex', gap: '2px', marginRight: '8px' }}>
                    <div className="nav-btn hoverable" title="Go Back">
                        <i className="codicon codicon-arrow-left"></i>
                    </div>
                    <div className="nav-btn hoverable" title="Go Forward">
                        <i className="codicon codicon-arrow-right"></i>
                    </div>
                </div>
                <div className="menu-items-container">
                    {menus.map(menu => (
                        <div key={menu.label} className="menu-item-wrapper">
                            <div
                                className={`menu-label ${activeMenu === menu.label ? 'active' : ''}`}
                                onClick={() => handleMenuClick(menu.label)}
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

            <div className="command-center" onClick={() => (window as any).showCommandPalette?.()}>
                <div className="command-box">
                    <i className="codicon codicon-search"></i>
                    <div className="text">
                        {(window as any).activeRootName || 'vscodium-rust'} — {(window as any).useStore?.getState().activeTabId ? ((window as any).useStore?.getState().tabs.find((t: any) => t.id === (window as any).useStore?.getState().activeTabId)?.filename || 'Welcome') : 'Welcome'}
                    </div>
                </div>
            </div>

            <div className="title-bar-right" style={{ display: 'flex', alignItems: 'center', gap: '8px', paddingRight: '12px' }}>
                <div
                    title="Privacy Guard Active"
                    style={{
                        display: 'flex',
                        alignItems: 'center',
                        padding: '4px',
                        opacity: 0.4,
                        cursor: 'help'
                    }}
                >
                    <i className="codicon codicon-shield" style={{ fontSize: '12px' }}></i>
                </div>

                <i className="codicon codicon-layout-centered-single hoverable" title="Toggle Layout"></i>
                <i
                    className="codicon codicon-layout-sidebar-right hoverable"
                    title="Toggle Agent (⌥⌘B)"
                    onClick={() => (window as any).useStore?.getState().toggleRightSidebar()}
                ></i>
            </div>

            {activeMenu && (
                <div className="menu-overlay" onClick={() => setActiveMenu(null)}></div>
            )}
        </div>

    );
};

export default TitleBar;
