import React, { useState } from 'react';
import { useStore } from '../store';
import { invoke } from '../tauri_bridge';
import { applyTheme, type VscodeTheme } from '../theme_engine';
import { Beaker, Layout, Bot, Globe, Braces, Settings } from 'lucide-react';

// Stable sentinel so a missing viewsContainers entry never produces a fresh
// array reference; otherwise Zustand's selector identity check thrashes and
// React logs "The result of getSnapshot should be cached".
const EMPTY_EXTENSION_ITEMS: any[] = [];

const ActivityBar: React.FC = () => {
    const activeView = useStore(state => state.activeSidebarView);
    const setActiveView = useStore(state => state.setActiveSidebarView);

    const [isThemePickerOpen, setIsThemePickerOpen] = useState(false);
    const [installedThemes, setInstalledThemes] = useState<VscodeTheme[]>([]);
    const setTheme = useStore(state => state.setTheme);

    const extensionItems = useStore(
        state => state.extensionContributions?.viewsContainers?.activitybar ?? EMPTY_EXTENSION_ITEMS
    );

    const items = [
        // AI/agent lives in the RIGHT sidebar (standard for AI IDEs) — removed the
        // redundant left activity-bar entry. Toggle AIRI from the title-bar button.
        { id: 'explorer-view', icon: 'files', title: 'Explorer' },
        { id: 'search-view', icon: 'search', title: 'Search' },
        { id: 'scm-view', icon: 'source-control', title: 'Source Control' },
        { id: 'debug-view', icon: 'debug-alt', title: 'Run and Debug' },
        { id: 'test-view', icon: 'beaker', title: 'Test Explorer' },
        { id: 'extensions-view', icon: 'extensions', title: 'Extensions' },
        { id: 'vector-search-view', icon: 'search-fuzzy', title: 'Codebase Search' },
        { id: 'tasks-view', icon: 'tasklist', title: 'Tasks & Specs (Antigravity)' },
        { id: 'steering-view', icon: 'symbol-ruler', title: 'Steering & Hooks (Kiro)' },
        { id: 'visual-lab', icon: 'json', title: 'JSON Visualizer & Flow (Visual Lab)' },
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
                            const store = (window as any).useStore?.getState();
                            if (item.id === 'agent-manager') {
                                if (store) {
                                    const isCurrentlyOpen = store.isRightSidebarOpen && store.isAiriPanelOpen;
                                    if (isCurrentlyOpen) {
                                        store.toggleRightSidebar();
                                    } else {
                                        store.openAiriPanel();
                                        setTimeout(() => {
                                            window.dispatchEvent(new CustomEvent('right-sidebar:set-view', { detail: { view: 'chat' } }));
                                        }, 10);
                                    }
                                }
                                return;
                            }
                            if (item.id === 'visual-lab') {
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
                            
                            // For regular views, ensure we are in editor mode
                            if (store && store.layoutMode !== 'editor') {
                                store.setLayoutMode('editor');
                            }
                            setActiveView(item.id);
                            invoke("check_activation_event", { event: `onView:${item.id}` });
                        }}
                    >
                        <div className="activity-item-icon">
                            {item.id === 'agent-manager' ? (
                                <Bot size={24} style={{ opacity: ((window as any).useStore?.getState().isRightSidebarOpen && (window as any).useStore?.getState().isAiriPanelOpen) ? 1 : 0.6, color: 'var(--terminator-accent)' }} />
                            ) : item.id === 'visual-lab' ? (
                                <Braces size={22} strokeWidth={2.25} style={{ opacity: activeView === item.id ? 1 : 0.6 }} />
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
                    title="Settings (Manage)"
                    id="activity-settings"
                    onClick={() => (window as any).useStore?.getState().openSettings()}
                >
                    <div className="activity-item-icon">
                        <Settings size={22} strokeWidth={1.9} />
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
                        <div className="theme-list">
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
                </div>
            )}
        </aside>
    );
};

export default ActivityBar;
