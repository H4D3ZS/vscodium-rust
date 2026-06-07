import React, { useState } from 'react';
import { useStore } from '../store';
import { invoke } from '../tauri_bridge';
import { applyTheme, type VscodeTheme } from '../theme_engine';
import PyTorchLogo from './pytorch/PyTorchLogo';
import { togglePyTorchStudio } from '../application/pytorch/openPyTorchStudio';

// Stable sentinel so a missing viewsContainers entry never produces a fresh
// array reference; otherwise Zustand's selector identity check thrashes and
// React logs "The result of getSnapshot should be cached".
const EMPTY_EXTENSION_ITEMS: any[] = [];

const ActivityBar: React.FC = () => {
    const activeView = useStore(state => state.activeSidebarView);
    const layoutMode = useStore(state => state.layoutMode);
    const externalBrowserActive = useStore(state => state.externalBrowserActive);
    const setActiveView = useStore(state => state.setActiveSidebarView);

    const [isThemePickerOpen, setIsThemePickerOpen] = useState(false);
    const [installedThemes, setInstalledThemes] = useState<VscodeTheme[]>([]);
    const setTheme = useStore(state => state.setTheme);

    const extensionItems = useStore(
        state => state.extensionContributions?.viewsContainers?.activitybar ?? EMPTY_EXTENSION_ITEMS
    );

    const items = [
        { id: 'explorer-view', icon: 'files', title: 'Explorer (Ctrl+Shift+E)' },
        { id: 'search-view', icon: 'search', title: 'Search (Ctrl+Shift+F)' },
        { id: 'scm-view', icon: 'source-control', title: 'Source Control (Ctrl+Shift+G)' },
        { id: 'security-view', icon: 'shield', title: 'Security Review — click to audit codebase (Ctrl+Shift+Alt+R)' },
        { id: 'debug-view', icon: 'debug-alt', title: 'Run and Debug (Ctrl+Shift+D)' },
        { id: 'extensions-view', icon: 'extensions', title: 'Extensions (Ctrl+Shift+X)' },
        { id: 'browser-panel', icon: 'globe', title: 'External Browser — live Firefox (Ctrl+Shift+U)', isBrowser: true },
        { id: 'pytorch-studio', icon: 'beaker', title: 'PyTorch ML Studio — friend of AI engineers (click again to close)', isPyTorch: true },
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
        // Use the resolved absolutePath from get_installed_themes — `path` is the
        // extension-relative path (e.g. "./themes/palenight.json") which the backend
        // can't read, so themes silently failed to apply.
        const themeFile = (theme as any).absolutePath || theme.path;
        const monacoTheme = await applyTheme(themeFile);
        setTheme(monacoTheme);
        setIsThemePickerOpen(false);
    };

    return (
        <aside className="activity-bar" id="activity-bar">
            <div className="activity-bar-top">
                {items.map(item => (
                    <div
                        key={item.id}
                        className={`activity-item ${activeView === item.id || ((item as any).isBrowser && externalBrowserActive) || ((item as any).isPyTorch && layoutMode === 'ml-studio') ? 'active' : ''}`}
                        title={item.title}
                        onClick={() => {
                            if ((item as any).isBrowser) {
                                import('../application/browser/openBrowserPanel').then(m => m.toggleExternalBrowser());
                                return;
                            }
                            if ((item as any).isPyTorch) {
                                togglePyTorchStudio();
                                return;
                            }
                            const store = (window as any).useStore?.getState();
                            if (store && store.layoutMode !== 'editor') {
                                store.setLayoutMode('editor');
                            }
                            setActiveView(item.id);
                            invoke("check_activation_event", { event: `onView:${item.id}` });
                        }}
                    >
                        <div className="activity-item-icon">
                            {(item as any).isPyTorch ? (
                                <PyTorchLogo size={22} />
                            ) : item.base64_icon ? (
                                <img src={item.base64_icon} style={{ width: '24px', height: '24px', opacity: activeView === item.id ? 1 : 0.6 }} alt="" />
                            ) : (
                                <i className={`codicon codicon-${item.icon}`} style={{ fontFamily: 'codicon', fontStyle: 'normal' }}></i>
                            )}
                        </div>
                    </div>
                ))}
            </div>
            <div className="activity-bar-bottom">
                <div
                    className="activity-item"
                    title="Accounts & Subscription"
                    onClick={() => (window as any).useStore?.getState().openSettings?.('user')}
                >
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
                        <i className="codicon codicon-gear" style={{ fontFamily: 'codicon', fontStyle: 'normal' }} />
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
