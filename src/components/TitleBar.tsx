import React, { useState } from 'react';

const TitleBar: React.FC = () => {
    const [activeMenu, setActiveMenu] = useState<string | null>(null);

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
        console.log(`Menu item clicked: ${item}`);
        if (item === 'New Terminal') {
            (window as any).spawnTerminal?.();
        }
        if (item === 'Command Palette...') {
            (window as any).showCommandPalette?.();
        }
        setActiveMenu(null);
    };

    return (
        <div id="title-bar" data-tauri-drag-region>
            <div className="title-bar-left">
                <div className="window-controls-spacer"></div>
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

            <div className="title-bar-right">
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
