import React, { useState } from 'react';
import { useStore } from '../store';
import { invoke } from '../tauri_bridge';
import { applyTheme, type VscodeTheme } from '../theme_engine';

const ActivityBar: React.FC = () => {
    const activeView = useStore(state => state.activeSidebarView);
    const setActiveView = useStore(state => state.setActiveSidebarView);
    
    const [isThemePickerOpen, setIsThemePickerOpen] = useState(false);
    const [installedThemes, setInstalledThemes] = useState<VscodeTheme[]>([]);
    const setTheme = useStore(state => state.setTheme);

    const items = [
        { id: 'explorer-view', icon: 'files', title: 'Explorer' },
        { id: 'search-view', icon: 'search', title: 'Search' },
        { id: 'scm-view', icon: 'source-control', title: 'Source Control' },
        { id: 'debug-view', icon: 'debug-alt', title: 'Run and Debug' },
        { id: 'extensions-view', icon: 'extensions', title: 'Extensions' },
        { id: 'specs-view', icon: 'book', title: 'Specs' },
        { id: 'agent-view', icon: 'sparkle', title: 'Agent' },
        { id: 'planning-view', icon: 'checklist', title: 'Workflow & Planning' },
        { id: 'mobile-view', icon: 'device-mobile', title: 'Mobile Emulators (Android & iOS)' },
    ];

    const openThemePicker = async () => {
        try {
            console.log("Fetching installed themes...");
            const themes = await invoke<VscodeTheme[]>('get_installed_themes');
            console.log("Found themes:", themes);
            setInstalledThemes(themes);
            setIsThemePickerOpen(true);
        } catch (e) {
            console.error("Failed to load themes:", e);
        }
    };

    const handleThemeSelect = async (theme: VscodeTheme) => {
        const monacoTheme = await applyTheme(theme.path);
        setTheme(monacoTheme);
        setIsThemePickerOpen(false);
    };

    return (
        <aside className="activity-bar" id="activity-bar">
            <div className="activity-bar-top">
                {items.map(item => (
                    <div
                        key={item.id}
                        className={`activity-item ${activeView === item.id ? 'active' : ''}`}
                        title={item.title}
                        onClick={() => setActiveView(item.id)}
                    >
                        <div className="activity-item-icon">
                            <i className={`codicon codicon-${item.icon}`}></i>
                        </div>
                    </div>
                ))}
            </div>
            <div className="activity-bar-bottom">
                <div className="activity-item" title="Accounts">
                    <div className="activity-item-icon">
                        <i className="codicon codicon-account"></i>
                    </div>
                </div>
                <div className="activity-item" title="Color Theme" onClick={openThemePicker}>
                    <div className="activity-item-icon">
                        <i className="codicon codicon-symbol-color"></i>
                    </div>
                </div>
                <div 
                    className="activity-item" 
                    title="Manage" 
                    id="activity-settings" 
                    onClick={() => (window as any).useStore?.getState().openSettings()}
                >
                    <div className="activity-item-icon">
                        <i className="codicon codicon-settings-gear"></i>
                    </div>
                </div>
            </div>

            {isThemePickerOpen && (
                <div className="theme-picker-overlay" onClick={() => setIsThemePickerOpen(false)}>
                    <div className="theme-picker" onClick={e => e.stopPropagation()}>
                        <div style={{ padding: '12px 16px', borderBottom: '1px solid var(--vscode-panel-border)', fontWeight: 'bold' }}>
                            Select Color Theme
                        </div>
                        {installedThemes.length === 0 && (
                            <div style={{ padding: '16px', fontSize: '12px', opacity: 0.7 }}>
                                No extension themes found. Try installing one from the marketplace.
                            </div>
                        )}
                        {installedThemes.map((theme, i) => (
                            <div key={i} className="theme-item" onClick={() => handleThemeSelect(theme)}>
                                <span className="theme-label">{theme.label}</span>
                                <span className="theme-ext">{theme.extension}</span>
                            </div>
                        ))}
                        <div style={{ padding: '12px 16px', borderTop: '1px solid var(--vscode-panel-border)', opacity: 0.5, fontSize: '11px' }}>
                            Predefined:
                        </div>
                        <div className="theme-item" onClick={() => { setTheme('vs-dark'); setIsThemePickerOpen(false); }}>
                             <span className="theme-label">Dark (Visual Studio)</span>
                        </div>
                        <div className="theme-item" onClick={() => { setTheme('vs'); setIsThemePickerOpen(false); }}>
                             <span className="theme-label">Light (Visual Studio)</span>
                        </div>
                    </div>
                </div>
            )}
        </aside>
    );
};

export default ActivityBar;
