// Settings shell (Milestone C): 8 sections, registry-driven search, panels
// resolved from src/domain/settings/registry.ts. Keep this file a thin
// router — settings content belongs in components/settings/* panels and
// new generic rows belong in the registry.

import React, { useState, useEffect, useMemo, lazy, Suspense } from 'react';
import { invoke } from '../tauri_bridge';
import {
    SETTINGS_SECTIONS,
    SETTINGS_ITEMS,
    LEGACY_CATEGORY_MAP,
    searchSettings,
    itemsForSection,
    findItem,
    type SettingsSectionId,
    type SettingsItem,
} from '../domain/settings/registry';
import AgentSettingsView, { type AgentSettingsCategory } from './AgentSettingsView';
import AgentPermissionsPanel from './AgentPermissionsPanel';
const AccountSettingsPanel = lazy(() => import('./AccountSettingsPanel'));
import EnterprisePanel from './EnterprisePanel';
import KeybindingsPanel from './KeybindingsPanel';
import SteeringPanel from './SteeringPanel';
import HooksPanel from './HooksPanel';
import AntigravityHooksPanel from './AntigravityHooksPanel';
import WorkspaceSettingsPanel from './settings/WorkspaceSettingsPanel';
import InferenceBackendPanel from './settings/InferenceBackendPanel';
import LemonadeSettingsPanel from './settings/LemonadeSettingsPanel';
import FccSettingsPanel from './settings/FccSettingsPanel';
import PlatformHubPanel from './platform/PlatformHubPanel';
import { openPyTorchStudio } from '../application/pytorch/openPyTorchStudio';
import PyTorchLogo from './pytorch/PyTorchLogo';
import SkillStorePanel from './skills/SkillStorePanel';
import LspLanguagesPanel from './settings/LspLanguagesPanel';
import ModuleInstallerPanel from './settings/ModuleInstallerPanel';
import AneAccelerationPanel from './AneAccelerationPanel';
import ModelSelectorPanel from './ModelSelectorPanel';
import {
    ChatPanel, ModelsPanel, ProvidersPanel, AIRICorePanel, HadesIntelligencePanel,
    ApexSettingsPanel, PrivacyPanel, WorkflowPanel, EditorPanel, ThemePanel,
    type VsCodeSettings,
} from './settings/panels';
import '../settings.css';

const PyTorchLaunchCard: React.FC = () => (
    <div style={{ maxWidth: 680 }}>
        <div className="settings-card" style={{ textAlign: 'center', padding: 32 }}>
            <PyTorchLogo size={48} />
            <h3 style={{ margin: '16px 0 8px' }}>PyTorch ML Studio</h3>
            <p style={{ opacity: 0.7, marginBottom: 16 }}>
                Train, fine-tune, and monitor models with thermal awareness.
            </p>
            <button type="button" className="settings-button" onClick={openPyTorchStudio}>
                Open ML Studio
            </button>
        </div>
    </div>
);

/** panel key (registry) → component. `agent-view:<cat>` routes to AgentSettingsView. */
function renderPanel(
    key: string,
    vsSettings: VsCodeSettings,
    onVsChange: (k: keyof VsCodeSettings, v: unknown) => void,
): React.ReactNode {
    if (key.startsWith('agent-view:')) {
        return <AgentSettingsView category={key.slice('agent-view:'.length) as AgentSettingsCategory} hideHeader />;
    }
    switch (key) {
        case 'editor': return <EditorPanel vsSettings={vsSettings} handleVsSettingChange={onVsChange} />;
        case 'theme': return <ThemePanel vsSettings={vsSettings} handleVsSettingChange={onVsChange} />;
        case 'workspace': return <WorkspaceSettingsPanel />;
        case 'models': return <><ModelsPanel /><ProvidersPanel /></>;
        case 'model-selection': return <ModelSelectorPanel />;
        case 'inference-backend': return <InferenceBackendPanel />;
        case 'lemonade': return <LemonadeSettingsPanel />;
        case 'fcc': return <FccSettingsPanel />;
        case 'chat': return <ChatPanel />;
        case 'permissions': return <AgentPermissionsPanel />;
        case 'skill-store': return <SkillStorePanel />;
        case 'workflow': return <WorkflowPanel />;
        case 'modules': return <ModuleInstallerPanel />;
        case 'platform': return <PlatformHubPanel />;
        case 'account': return <AccountSettingsPanel />;
        case 'enterprise': return <EnterprisePanel />;
        case 'privacy': return <PrivacyPanel />;
        case 'keybindings': return <KeybindingsPanel />;
        case 'apex': return <ApexSettingsPanel />;
        case 'airi-core': return <AIRICorePanel />;
        case 'ane': return <AneAccelerationPanel />;
        case 'kortex': return <HadesIntelligencePanel />;
        case 'steering': return <SteeringPanel />;
        case 'hooks': return <HooksPanel />;
        case 'ag-hooks': return <AntigravityHooksPanel />;
        case 'lsp': return <LspLanguagesPanel />;
        case 'pytorch': return <PyTorchLaunchCard />;
        default: return <div style={{ opacity: 0.6, padding: 16 }}>Unknown panel: {key}</div>;
    }
}

function readInitialItem(): string {
    // Legacy deep links: other components still set settings.category.agent.
    try {
        sessionStorage.removeItem('settings.initialTab');
        const legacy = localStorage.getItem('settings.category.agent')
            ?? localStorage.getItem('settings.category.vscode');
        if (legacy) {
            localStorage.removeItem('settings.category.agent');
            localStorage.removeItem('settings.category.vscode');
            const mapped = LEGACY_CATEGORY_MAP[legacy];
            if (mapped && findItem(mapped)) return mapped;
        }
        const saved = localStorage.getItem('settings.item');
        if (saved && findItem(saved)) return saved;
    } catch { /* storage unavailable */ }
    return 'chat';
}

const SettingsPage: React.FC = () => {
    const [activeItemId, setActiveItemId] = useState<string>(readInitialItem);
    const [searchQuery, setSearchQuery] = useState('');
    const [vsSettings, setVsSettings] = useState<VsCodeSettings>({
        theme: 'vs-dark', font_size: 14, tab_size: 4, auto_save: 'off',
    });

    const activeItem = findItem(activeItemId) ?? SETTINGS_ITEMS[0];
    const activeSection = activeItem.section;

    useEffect(() => {
        invoke<VsCodeSettings>('get_settings').then(setVsSettings).catch(() => {});
    }, []);

    useEffect(() => {
        try { localStorage.setItem('settings.item', activeItemId); } catch { /* ignore */ }
    }, [activeItemId]);

    const handleVsSettingChange = async (key: keyof VsCodeSettings, value: unknown) => {
        const next = { ...vsSettings, [key]: value };
        setVsSettings(next);
        try {
            await invoke('update_settings', {
                settings: { theme: next.theme, font_size: next.font_size },
            });
        } catch { /* backend persists best-effort */ }
    };

    const searchResults = useMemo(() => searchSettings(searchQuery), [searchQuery]);
    const sectionItems = itemsForSection(activeSection);

    const selectSection = (id: SettingsSectionId) => {
        const first = itemsForSection(id)[0];
        if (first) setActiveItemId(first.id);
    };

    const selectItem = (it: SettingsItem) => {
        setSearchQuery('');
        setActiveItemId(it.id);
    };

    return (
        <div className="settings-container">
            <div className="settings-header">
                <div className="settings-title-row">
                    <h1 className="settings-title">Settings</h1>
                    <div className="settings-search-wrapper">
                        <i className="codicon codicon-search settings-search-icon" />
                        <input
                            type="text"
                            className="settings-search-input"
                            placeholder="Search settings..."
                            value={searchQuery}
                            onChange={(e) => setSearchQuery(e.target.value)}
                        />
                    </div>
                </div>
            </div>

            <div className="settings-body">
                <div className="settings-sidebar">
                    {searchQuery.trim() ? (
                        <>
                            {searchResults.length === 0 && (
                                <div style={{ padding: '8px 12px', opacity: 0.6, fontSize: 12 }}>No matching settings</div>
                            )}
                            {searchResults.map((it) => {
                                const section = SETTINGS_SECTIONS.find((s) => s.id === it.section);
                                return (
                                    <div
                                        key={it.id}
                                        className={`settings-sidebar-item ${it.id === activeItemId ? 'active' : ''}`}
                                        onClick={() => selectItem(it)}
                                    >
                                        <i className={`codicon codicon-${it.icon}`} />
                                        <span>
                                            {it.label}
                                            <span style={{ opacity: 0.5, fontSize: 10, display: 'block' }}>{section?.label}</span>
                                        </span>
                                    </div>
                                );
                            })}
                        </>
                    ) : (
                        SETTINGS_SECTIONS.map((s) => (
                            <div
                                key={s.id}
                                className={`settings-sidebar-item ${s.id === activeSection ? 'active' : ''}`}
                                onClick={() => selectSection(s.id)}
                            >
                                <i className={`codicon codicon-${s.icon}`} />
                                {s.label}
                            </div>
                        ))
                    )}
                </div>

                <div className="settings-content">
                    {sectionItems.length > 1 && (
                        <div className="settings-subnav">
                            {sectionItems.map((it) => (
                                <button
                                    type="button"
                                    key={it.id}
                                    className={`settings-chip${it.id === activeItemId ? ' active' : ''}`}
                                    onClick={() => setActiveItemId(it.id)}
                                >
                                    {it.label}
                                </button>
                            ))}
                        </div>
                    )}
                    {renderPanel(activeItem.panel, vsSettings, handleVsSettingChange)}
                </div>
            </div>
        </div>
    );
};

export default SettingsPage;
