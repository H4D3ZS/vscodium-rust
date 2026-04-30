import React from 'react';
import { useStore } from '../store';
import EmulatorPanel from './EmulatorPanel';
import AiriPanel from './AiriPanel';

/**
 * Right Sidebar - Can contain emulator panel or be hidden
 * Used when emulator is positioned on the right side
 */
const RightSidebar: React.FC = () => {
    const emulatorPosition = useStore(state => state.emulatorPanelPosition);
    const isRightSidebarOpen = useStore(state => state.isRightSidebarOpen);

    // Don't render if emulator is on left or hidden
    if (emulatorPosition !== 'right') {
        return null;
    }

    return (
        <aside
            className="sidebar right-sidebar"
            style={{
                width: '350px',
                minWidth: '300px',
                maxWidth: '500px',
                background: 'var(--vscode-sideBar-background)',
                borderLeft: '1px solid var(--vscode-panel-border)',
                display: 'flex',
                flexDirection: 'column',
                overflow: 'hidden'
            }}
        >
            <EmulatorPanel />
        </aside>
    );
};

export default RightSidebar;
