import React, { useState, useEffect } from 'react';
import { invoke } from '../tauri_bridge';
import { useStore } from '../store';
import McpManager from './McpManager';

interface Settings {
    theme: string;
    font_size: number;
    tab_size?: number;
    auto_save?: string;
}

const SettingsPage: React.FC = () => {
    const [activeTab, setActiveTab] = useState<'user' | 'workspace' | 'agent'>('user');
    const setTheme = useStore(state => state.setTheme);
    const ollamaStatus = useStore(state => state.ollamaStatus);
    const refreshAvailableModels = useStore(state => state.refreshAvailableModels);
    
    const [settings, setSettings] = useState<Settings>({
        theme: 'vs-dark',
        font_size: 14,
        tab_size: 4,
        auto_save: 'off'
    });
    const [searchQuery, setSearchQuery] = useState('');

    useEffect(() => {
        loadSettings();
    }, []);

    const loadSettings = async () => {
        try {
            const currentSettings = await invoke<Settings>('get_settings');
            setSettings(currentSettings);
        } catch (e) {
            console.error("Failed to load settings:", e);
        }
    };

    const handleSettingChange = async (key: keyof Settings, value: any) => {
        const newSettings = { ...settings, [key]: value };
        setSettings(newSettings);
        try {
            await invoke('update_settings', { newSettings });
            if (key === 'theme') {
                setTheme(value);
            }
        } catch (e) {
            console.error("Failed to update settings:", e);
        }
    };

    const renderSettingItem = (label: string, description: string, control: React.ReactNode) => (
        <div className="settings-item">
            <div className="settings-item-header">
                <div className="settings-item-label">{label}</div>
                <div className="settings-item-description">{description}</div>
            </div>
            <div className="settings-item-control">
                {control}
            </div>
        </div>
    );

    const renderAgentSettings = () => (
        <div className="agent-settings">
            <div className="settings-section">
                <div className="settings-section-title">AI Engine</div>
                {renderSettingItem(
                    "Ollama Status",
                    "Status of the local Ollama instance.",
                    <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                        <div style={{ 
                            width: '10px', 
                            height: '10px', 
                            borderRadius: '50%', 
                            background: ollamaStatus === 'running' ? '#4ade80' : '#f87171' 
                        }}></div>
                        <span>{ollamaStatus === 'running' ? 'Connected' : 'Disconnected'}</span>
                        <button 
                            className="secondary-button" 
                            style={{ marginLeft: '12px', padding: '4px 8px', fontSize: '11px' }}
                            onClick={() => refreshAvailableModels('ollama')}
                        >
                            Reconnect
                        </button>
                    </div>
                )}
            </div>

            <McpManager />
        </div>
    );

    return (
        <div className="settings-container">
            <div className="settings-header">
                <h1>Settings</h1>
                <div style={{ position: 'relative', maxWidth: '800px' }}>
                    <input 
                        type="text" 
                        placeholder="Search settings" 
                        value={searchQuery}
                        onChange={(e) => setSearchQuery(e.target.value)}
                        style={{ paddingLeft: '30px' }}
                    />
                    <i className="codicon codicon-search" style={{ position: 'absolute', left: '8px', top: '50%', transform: 'translateY(-50%)', opacity: 0.5 }}></i>
                </div>
            </div>

            <div className="settings-tabs">
                <div className={`settings-tab ${activeTab === 'user' ? 'active' : ''}`} onClick={() => setActiveTab('user')}>User</div>
                <div className={`settings-tab ${activeTab === 'workspace' ? 'active' : ''}`} onClick={() => setActiveTab('workspace')}>Workspace</div>
                <div className={`settings-tab ${activeTab === 'agent' ? 'active' : ''}`} onClick={() => setActiveTab('agent')}>AI Agent Settings</div>
            </div>

            <div className="settings-content">
                {activeTab === 'agent' ? (
                    renderAgentSettings()
                ) : (
                    <div className="settings-section">
                        <div className="settings-section-title">Commonly Used</div>
                        
                        {renderSettingItem(
                            "Files: Auto Save",
                            "Controls auto save of editors that have unsaved changes.",
                            <select 
                                value={settings.auto_save || 'off'} 
                                onChange={(e) => handleSettingChange('auto_save', e.target.value)}
                            >
                                <option value="off">off</option>
                                <option value="afterDelay">afterDelay</option>
                                <option value="onFocusChange">onFocusChange</option>
                                <option value="onWindowChange">onWindowChange</option>
                            </select>
                        )}

                        {renderSettingItem(
                            "Editor: Font Size",
                            "Controls the font size in pixels.",
                            <input 
                                type="number" 
                                value={settings.font_size} 
                                onChange={(e) => handleSettingChange('font_size', parseInt(e.target.value))}
                            />
                        )}

                        {renderSettingItem(
                            "Editor: Tab Size",
                            "The number of spaces a tab is equal to.",
                            <input 
                                type="number" 
                                value={settings.tab_size || 4} 
                                onChange={(e) => handleSettingChange('tab_size', parseInt(e.target.value))}
                            />
                        )}

                        {renderSettingItem(
                            "Workbench: Color Theme",
                            "Specifies the color theme used in the workbench.",
                            <select 
                                value={settings.theme} 
                                onChange={(e) => handleSettingChange('theme', e.target.value)}
                            >
                                <option value="vs-dark">Dark (Visual Studio)</option>
                                <option value="vs">Light (Visual Studio)</option>
                                <option value="Darcula">Darcula</option>
                                <option value="Monokai">Monokai</option>
                                <option value="Solarized Dark">Solarized Dark</option>
                            </select>
                        )}
                    </div>
                )}
            </div>
        </div>
    );
};

export default SettingsPage;
