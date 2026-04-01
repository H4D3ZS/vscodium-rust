import React, { useState } from 'react';
import { useStore } from '../store';
import { invoke } from '../tauri_bridge';
import { applyTheme, type VscodeTheme } from '../theme_engine';
import { Beaker } from 'lucide-react';

const ActivityBar: React.FC = () => {
    const activeView = useStore(state => state.activeSidebarView);
    const setActiveView = useStore(state => state.setActiveSidebarView);

    const [isThemePickerOpen, setIsThemePickerOpen] = useState(false);
    const [installedThemes, setInstalledThemes] = useState<VscodeTheme[]>([]);
    const setTheme = useStore(state => state.setTheme);

    const extensionItems = useStore(state => state.extensionContributions?.viewsContainers?.activitybar || []);

    const coreItems = [
        { id: 'explorer-view', icon: 'files', title: 'Explorer' },
        { id: 'search-view', icon: 'search', title: 'Search' },
        { id: 'scm-view', icon: 'source-control', title: 'Source Control' },
        { id: 'debug-view', icon: 'debug-alt', title: 'Run and Debug' },
        { id: 'extensions-view', icon: 'extensions', title: 'Extensions' },
        { id: 'mobile-view', icon: 'device-mobile', title: 'Mobile Emulators (Android & iOS)' },
        { id: 'visual-lab', icon: 'beaker', title: 'Visual Lab (JSON & Flow)' },
    ];

    const items = [
        ...coreItems,
        ...extensionItems
            .filter((ext: any) => {
                const id = String(ext.id || '').toLowerCase();
                const title = String(ext.title || '').toLowerCase();
                const icon = String(ext.icon || '').toLowerCase();
                // Strictly exclude any agentic/specs/chat views to consolidate in the Right Sidebar
                // This targets robot, sparkle, bot, specs, and AI icons
                const isAgentic = id.includes('agent') || id.includes('specs') || id.includes('ai') || id.includes('chat') ||
                    title.includes('agent') || title.includes('specs') || title.includes('ai') || title.includes('chat') ||
                    icon.includes('robot') || icon.includes('sparkle') || icon.includes('bot') || icon.includes('stars');
                return !isAgentic;
            })
            .map((ext: any) => ({
                id: ext.id,
                icon: ext.icon,
                title: ext.title,
                base64_icon: ext.base64_icon,
                isExtension: true
            }))
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
                        onClick={() => {
                            if (item.id === 'visual-lab') {
                                const store = (window as any).useStore?.getState();
                                if (store) {
                                    const activeTab = store.tabs.find((t: any) => t.id === store.activeTabId);
                                    if (activeTab && (activeTab.path.endsWith('.json') || activeTab.language === 'json')) {
                                        store.setVisualLabData(activeTab.content);
                                    }
                                    store.toggleVisualLab(true);
                                    store.setVisualLabMode('json');
                                }
                                return;
                            }
                            setActiveView(item.id);
                            invoke("check_activation_event", { event: `onView:${item.id}` });
                        }}
                    >
                        <div className="activity-item-icon">
                            {item.id === 'visual-lab' ? (
                                <Beaker size={24} style={{ opacity: activeView === item.id ? 1 : 0.6 }} />
                            ) : item.base64_icon ? (
                                <img src={item.base64_icon} style={{ width: '24px', height: '24px', opacity: activeView === item.id ? 1 : 0.6 }} />
                            ) : (
                                <i className={`codicon codicon-${item.icon}`} style={{ fontFamily: 'codicon', fontStyle: 'normal' }}></i>
                            )}
                        </div>
                        {item.id === 'scm-view' && <div className="badge dot"></div>}
                        {item.id === 'extensions-view' && false && <div className="badge">12</div>}
                    </div>
                ))}
            </div>
            <div className="activity-bar-bottom">
                <div className="activity-item" title="Accounts">
                    <div className="activity-item-icon">
                        <i className="codicon codicon-account" style={{ fontFamily: 'codicon', fontStyle: 'normal' }}></i>
                    </div>
                </div>
                <div className="activity-item" title="Color Theme" onClick={openThemePicker}>
                    <div className="activity-item-icon">
                        <i className="codicon codicon-paintcan" style={{ fontFamily: 'codicon', fontStyle: 'normal' }}></i>
                    </div>
                </div>
                <div
                    className="activity-item"
                    title="Manage"
                    id="activity-settings"
                    onClick={() => (window as any).useStore?.getState().openSettings()}
                >
                    <div className="activity-item-icon">
                        <i className="codicon codicon-settings-gear" style={{ fontFamily: 'codicon', fontStyle: 'normal' }}></i>
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
                            <div style={{ padding: '20px', fontSize: '12px', opacity: 0.7, textAlign: 'center' }}>
                                <i className="codicon codicon-info" style={{ fontSize: '24px', display: 'block', marginBottom: '8px', fontFamily: 'codicon', fontStyle: 'normal' }}></i>
                                No extension themes found.<br />
                                Scanning standard VS Code paths...
                            </div>
                        )}
                        {installedThemes.map((theme, i) => (
                            <div key={i} className="theme-item" onClick={() => handleThemeSelect(theme)}>
                                <span className="theme-label">{theme.label}</span>
                                <span className="theme-ext">{theme.extensionName}</span>
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
