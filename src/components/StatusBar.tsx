import React from 'react';
import { useStore } from '../store';

const StatusBar: React.FC = () => {
    const theme = useStore(state => state.theme);
    const setTheme = useStore(state => state.setTheme);
    const setActiveSidebarView = useStore(state => state.setActiveSidebarView);
    const toggleBottomPanel = useStore(state => state.toggleBottomPanel);
    const agentModel = useStore(state => state.agentModel);

    const toggleTheme = () => {
        const themes = ['vs', 'vs-dark', 'hc-black'];
        const next = themes[(themes.indexOf(theme) + 1) % themes.length];
        setTheme(next);
    };

    return (
        <footer className="status-bar" style={{
            backgroundColor: 'var(--vscode-statusBar-background)',
            color: 'var(--vscode-statusBar-foreground)',
            height: 'var(--status-bar-height, 22px)',
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center',
            padding: '0 8px',
            fontSize: '12px',
            fontFamily: 'var(--font-ui)',
            userSelect: 'none'
        }}>
            <div className="status-left" style={{ display: 'flex', alignItems: 'center', height: '100%' }}>
                <div className="status-item hoverable" onClick={() => setActiveSidebarView('scm-view')} style={{ cursor: 'pointer', display: 'flex', alignItems: 'center', height: '100%', padding: '0 8px' }}>
                    <i className="codicon codicon-source-control" style={{ fontSize: '13px', marginRight: '4px' }}></i>main*
                </div>
                <div className="status-item hoverable" onClick={() => {}} style={{ cursor: 'pointer', display: 'flex', alignItems: 'center', height: '100%', padding: '0 8px' }}>
                    <i className="codicon codicon-sync" style={{ fontSize: '13px' }}></i>
                </div>
                <div className="status-item hoverable" onClick={() => {}} style={{ cursor: 'pointer', display: 'flex', alignItems: 'center', height: '100%', padding: '0 8px' }}>
                    <i className="codicon codicon-error" style={{ fontSize: '13px', marginRight: '3px' }}></i>0
                    <i className="codicon codicon-warning" style={{ fontSize: '13px', marginLeft: '6px', marginRight: '3px' }}></i>0
                </div>
                <div className="status-item hoverable" onClick={() => setActiveSidebarView('agent-view')} style={{ cursor: 'pointer', display: 'flex', alignItems: 'center', height: '100%', padding: '0 8px' }}>
                    <i className="codicon codicon-sparkle" style={{ fontSize: '13px', marginRight: '4px' }}></i>
                    <span>{agentModel.split('|').pop() || 'GPT-4o'}</span>
                </div>
                <div className="status-item hoverable" onClick={() => setActiveSidebarView('mobile-view')} style={{ cursor: 'pointer', display: 'flex', alignItems: 'center', height: '100%', padding: '0 8px' }}>
                    <i className="codicon codicon-device-mobile" style={{ fontSize: '13px', marginRight: '4px' }}></i>
                    <span>No Device</span>
                </div>
            </div>
            <div className="status-right" style={{ display: 'flex', alignItems: 'center', height: '100%' }}>
                <div className="status-item hoverable" style={{ cursor: 'pointer', height: '100%', display: 'flex', alignItems: 'center', padding: '0 8px' }}>Ln 1, Col 1</div>
                <div className="status-item hoverable" style={{ cursor: 'pointer', height: '100%', display: 'flex', alignItems: 'center', padding: '0 8px' }}>Spaces: 4</div>
                <div className="status-item hoverable" style={{ cursor: 'pointer', height: '100%', display: 'flex', alignItems: 'center', padding: '0 8px' }}>UTF-8</div>
                <div className="status-item hoverable" style={{ cursor: 'pointer', height: '100%', display: 'flex', alignItems: 'center', padding: '0 8px' }}>Rust</div>
                <div className="status-item hoverable" onClick={toggleTheme} style={{ cursor: 'pointer', display: 'flex', alignItems: 'center', height: '100%', padding: '0 8px' }}>
                    <i className="codicon codicon-color-mode" style={{ marginRight: '4px' }}></i>
                    <span>{theme}</span>
                </div>
                <div className="status-item hoverable" onClick={toggleBottomPanel} title="Toggle Terminal" style={{ cursor: 'pointer', display: 'flex', alignItems: 'center', height: '100%', padding: '0 8px' }}>
                    <i className="codicon codicon-terminal" style={{ fontSize: '14px' }}></i>
                </div>
            </div>
        </footer>
    );
};

export default StatusBar;
