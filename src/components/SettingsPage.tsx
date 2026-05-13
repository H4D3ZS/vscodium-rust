import React, { useState, useEffect, useMemo, useCallback } from 'react';
import { invoke } from '../tauri_bridge';
import { useStore } from '../store';
import AgentSettingsView, { type AgentSettingsCategory } from './AgentSettingsView';
import KeybindingsPanel from './KeybindingsPanel';
import { airiBiology } from '../airi/biology';
import { airiConsciousness } from '../airi/consciousness';
import '../settings.css';

// ─────────────────────────────────────────────────────────────────────────────
//  Settings page — Cursor IDE parity layout.
// ─────────────────────────────────────────────────────────────────────────────

interface VsCodeSettings {
    theme: string;
    font_size: number;
    tab_size?: number;
    auto_save?: string;
}

type TopTab = 'agent' | 'vscode';

interface CategoryDef {
    id: string;
    label: string;
    icon: string;
    agentCategory?: AgentSettingsCategory;
    customRender?: () => React.ReactNode;
    groupStart?: string;
}

function readInitialTab(): TopTab {
    try {
        const t = sessionStorage.getItem('settings.initialTab');
        if (t === 'agent') return 'agent';
        if (t === 'user' || t === 'workspace' || t === 'vscode') return 'vscode';
    } catch { }
    return 'agent';
}

function readInitialCategory(tab: TopTab): string {
    try {
        const k = `settings.category.${tab}`;
        const v = localStorage.getItem(k);
        if (v) return v;
    } catch { }
    return tab === 'agent' ? 'general' : 'editor';
}

const SettingsRow: React.FC<{
    label: string;
    description?: React.ReactNode;
    control: React.ReactNode;
}> = ({ label, description, control }) => (
    <div className="settings-row">
        <div className="settings-row-info">
            <div className="settings-row-label">{label}</div>
            {description && <div className="settings-row-description">{description}</div>}
        </div>
        <div className="settings-row-control">{control}</div>
    </div>
);

const SectionTitle: React.FC<{ children: React.ReactNode }> = ({ children }) => (
    <h2 className="settings-section-title">{children}</h2>
);

const SettingsPage: React.FC = () => {
    const [topTab, setTopTab] = useState<TopTab>(readInitialTab);
    const [activeCategory, setActiveCategory] = useState<string>(() => readInitialCategory(readInitialTab()));
    const [searchQuery, setSearchQuery] = useState('');
    const setTheme = useStore(state => state.setTheme);

    const [vsSettings, setVsSettings] = useState<VsCodeSettings>({
        theme: 'vs-dark',
        font_size: 14,
        tab_size: 4,
        auto_save: 'off',
    });

    useEffect(() => {
        invoke<VsCodeSettings>('get_settings')
            .then(setVsSettings)
            .catch(err => console.error('[Settings] get_settings failed:', err));
        try { sessionStorage.removeItem('settings.initialTab'); } catch { }
    }, []);

    useEffect(() => {
        try { localStorage.setItem(`settings.category.${topTab}`, activeCategory); }
        catch { }
    }, [topTab, activeCategory]);

    const handleVsSettingChange = async (key: keyof VsCodeSettings, value: any) => {
        const next = { ...vsSettings, [key]: value };
        setVsSettings(next);
        try {
            await invoke('update_settings', { newSettings: next });
            if (key === 'theme') setTheme(value);
        } catch (err) { }
    };

    // ── Panel Definitions ──

    function AIRICorePanel() {
        const [bio, setBio] = useState(airiBiology.getState());
        const [cons, setCons] = useState(airiConsciousness.getState());
        const visionEnabled = useStore(s => s.airiVisionEnabled);
        const setVisionEnabled = useStore(s => s.setAiriVisionEnabled);

        useEffect(() => {
            const timer = setInterval(() => {
                setBio(airiBiology.getState());
                setCons(airiConsciousness.getState());
            }, 2000);
            return () => clearInterval(timer);
        }, []);

        return (
            <div style={{ maxWidth: 720 }}>
                <SectionTitle>AIRI Sentient Core</SectionTitle>
                <p className="settings-row-description" style={{ marginBottom: 20 }}>
                    Manage the biological state and cognitive parameters of the AIRI digital entity.
                </p>

                <div style={{ display: 'grid', gridTemplateColumns: 'repeat(3, 1fr)', gap: 16, marginBottom: 24 }}>
                    <div className="solid-card" style={{ padding: 16, textAlign: 'center' }}>
                        <div style={{ fontSize: 10, opacity: 0.6, marginBottom: 4 }}>ENERGY</div>
                        <div style={{ fontSize: 24, fontWeight: 300, color: bio.energy > 30 ? '#4ade80' : '#f87171' }}>
                            {bio.energy}%
                        </div>
                        <div style={{ height: 4, background: 'rgba(255,255,255,0.1)', borderRadius: 2, marginTop: 8 }}>
                            <div style={{ height: '100%', width: `${bio.energy}%`, background: 'currentColor', borderRadius: 2 }} />
                        </div>
                    </div>
                    <div className="solid-card" style={{ padding: 16, textAlign: 'center' }}>
                        <div style={{ fontSize: 10, opacity: 0.6, marginBottom: 4 }}>MOOD</div>
                        <div style={{ fontSize: 18, fontWeight: 400, textTransform: 'capitalize' }}>
                            {bio.mood}
                        </div>
                        <div style={{ fontSize: 10, opacity: 0.5, marginTop: 4 }}>Biological State</div>
                    </div>
                    <div className="solid-card" style={{ padding: 16, textAlign: 'center' }}>
                        <div style={{ fontSize: 10, opacity: 0.6, marginBottom: 4 }}>CONSCIOUSNESS</div>
                        <div style={{ fontSize: 18, fontWeight: 400, color: '#c084fc' }}>
                            Active
                        </div>
                        <div style={{ fontSize: 10, opacity: 0.5, marginTop: 4 }}>Sentient Thread</div>
                    </div>
                </div>

                <SettingsRow
                    label="Vision System"
                    description="Allow AIRI to see your screen in real-time to provide context-aware help."
                    control={
                        <div className="settings-checkbox-wrapper">
                            <input
                                type="checkbox"
                                className="settings-checkbox"
                                checked={visionEnabled}
                                onChange={(e) => setVisionEnabled(e.target.checked)}
                            />
                            <span style={{ fontSize: 12 }}>Enable Vision</span>
                        </div>
                    }
                />

                <SettingsRow
                    label="Metabolic Needs"
                    description="When enabled, AIRI consumes energy during complex tasks and performs better when well-rested."
                    control={
                        <button className="settings-button" onClick={() => airiBiology.rest(15)} style={{ background: '#c084fc' }}>
                            Restore Energy (Rest)
                        </button>
                    }
                />

                <div style={{ marginTop: 24 }}>
                    <div style={{ fontSize: 13, fontWeight: 600, marginBottom: 8 }}>Active Thoughts</div>
                    <div className="solid-card" style={{ padding: 12, fontSize: 12, background: 'rgba(0,0,0,0.2)', minHeight: 60, border: '1px solid rgba(255,255,255,0.05)' }}>
                        <div style={{ opacity: 0.8, fontStyle: 'italic', lineHeight: 1.6 }}>
                            {cons.thoughts[cons.thoughts.length - 1]?.content || "AIRI is focusing on the current workspace architecture..."}
                        </div>
                    </div>
                </div>
            </div>
        );
    }

    function AgentGeneralPanel() {
        return (
            <div style={{ maxWidth: 720 }}>
                <SectionTitle>General</SectionTitle>
                <p style={{ opacity: 0.7, fontSize: 12, lineHeight: 1.55 }}>
                    Antigravity IDE — The cognitive workspace. AIRI is the sentient engine powering your development.
                </p>
                <div style={{ marginTop: 20 }}>
                    {[
                        { label: 'Sentient Core', desc: 'Biological state, consciousness, and vision.' },
                        { label: 'Ollama Integration', desc: 'Local model serving and performance.' },
                        { label: 'Agent Workspace', desc: 'Rules, skills, and subagent permissions.' },
                        { label: 'Cognitive Memory', desc: 'Persistent .aim long-term storage.' }
                    ].map(item => (
                        <div key={item.label} style={{ marginBottom: 12 }}>
                            <div style={{ fontSize: 13, fontWeight: 600 }}>{item.label}</div>
                            <div style={{ fontSize: 12, opacity: 0.6 }}>{item.desc}</div>
                        </div>
                    ))}
                </div>
            </div>
        );
    }

    function AgentBehaviourPanel() {
        const agentMode = useStore(s => s.agentMode);
        const setAgentMode = useStore(s => s.setAgentMode);
        const isYolo = useStore(s => s.isYoloMode);
        const setYolo = useStore(state => (state as any).setYoloMode);

        return (
            <div style={{ maxWidth: 720, display: 'flex', flexDirection: 'column', gap: 16 }}>
                <SectionTitle>Agents</SectionTitle>
                <SettingsRow
                    label="Default Agent Mode"
                    description="Standard 'Agent' mode allows file writes. 'Sentient' uses AIRI core."
                    control={
                        <select className="settings-select" value={agentMode} onChange={(e) => setAgentMode(e.target.value as any)}>
                            {['Agent', 'Chat', 'Plan', 'Sentient'].map(m => (
                                <option key={m} value={m}>{m}</option>
                            ))}
                        </select>
                    }
                />
                <SettingsRow
                    label="YOLO Autonomy"
                    description="Allow agents to execute tool calls without manual approval."
                    control={
                        <div className="settings-checkbox-wrapper">
                            <input
                                type="checkbox"
                                className="settings-checkbox"
                                checked={!!isYolo}
                                onChange={(e) => setYolo?.(e.target.checked)}
                            />
                            <span style={{ fontSize: 12 }}>Enable YOLO</span>
                        </div>
                    }
                />
            </div>
        );
    }

    function HadesIntelligencePanel() {
        const aimVfs = useStore(s => s.aimVfsEnabled);
        const setAimVfs = useStore(s => (s as any).setAimVfsEnabled);
        const thermal = useStore(s => s.thermalGovernorEnabled);
        const setThermal = useStore(s => (s as any).setThermalGovernorEnabled);
        const jit = useStore(s => s.jitDecompressionEnabled);
        const setJit = useStore(s => (s as any).setJitDecompressionEnabled);
        const jitThreshold = useStore(s => s.jitThreshold);
        const setJitThreshold = useStore(s => (s as any).setJitThreshold);

        return (
            <div style={{ maxWidth: 720, display: 'flex', flexDirection: 'column', gap: 16 }}>
                <SectionTitle>HADES Intelligence Layer</SectionTitle>
                <p className="settings-row-description">
                    Configure the low-level optimization and context injection layer powering Antigravity.
                </p>

                <SettingsRow
                    label="AIM VFS Context"
                    description="Enable neural-aware virtual file system for real-time context injection."
                    control={
                        <input type="checkbox" className="settings-checkbox" checked={aimVfs} onChange={(e) => setAimVfs?.(e.target.checked)} />
                    }
                />

                <SettingsRow
                    label="Thermal Governor"
                    description="Monitor RX 580 / Ryzen 9 thermals and throttle execution during heavy inference."
                    control={
                        <input type="checkbox" className="settings-checkbox" checked={thermal} onChange={(e) => setThermal?.(e.target.checked)} />
                    }
                />

                <SettingsRow
                    label="JIT Code Decompression"
                    description="Dynamically inflate compressed code gists before processing."
                    control={
                        <input type="checkbox" className="settings-checkbox" checked={jit} onChange={(e) => setJit?.(e.target.checked)} />
                    }
                />

                <SettingsRow
                    label="Attention Threshold"
                    description="Sensitivity level for JIT decompression triggers (0.5 - 1.0)."
                    control={
                        <input
                            type="range" min="0.5" max="1.0" step="0.01"
                            style={{ width: 200 }}
                            value={jitThreshold}
                            onChange={(e) => setJitThreshold?.(parseFloat(e.target.value))}
                        />
                    }
                />
            </div>
        );
    }

    const agentCategories: CategoryDef[] = [
        { id: 'general', label: 'General', icon: 'gear', customRender: () => <AgentGeneralPanel /> },
        { id: 'airi', label: 'Sentient Core', icon: 'beaker', customRender: () => <AIRICorePanel />, groupStart: 'Sentient' },
        { id: 'agents', label: 'Agents', icon: 'robot', customRender: () => <AgentBehaviourPanel /> },
        { id: 'hades', label: 'HADES Layer', icon: 'zap', customRender: () => <HadesIntelligencePanel /> },
        { id: 'models', label: 'Models', icon: 'symbol-misc', agentCategory: 'models', groupStart: 'Backend' },
        { id: 'mcps', label: 'Tools & MCPs', icon: 'plug', agentCategory: 'mcps' },
        { id: 'ollama', label: 'Ollama', icon: 'server-environment', agentCategory: 'ollama' },
        { id: 'memory', label: 'Memory', icon: 'database', agentCategory: 'memory', groupStart: 'Data' },
        { id: 'voice', label: 'Voice & TTS', icon: 'unmute', agentCategory: 'voice' },
        { id: 'avatar', label: 'AI Avatar', icon: 'person', agentCategory: 'avatar' },
    ];

    const vsCategories: CategoryDef[] = [
        { id: 'editor', label: 'Editor', icon: 'edit' },
        { id: 'theme', label: 'Theme', icon: 'symbol-color' },
    ];

    const activeList = topTab === 'agent' ? agentCategories : vsCategories;
    const filteredCategories = useMemo(() => {
        if (!searchQuery.trim()) return activeList;
        const q = searchQuery.trim().toLowerCase();
        return activeList.filter(c => c.label.toLowerCase().includes(q));
    }, [activeList, searchQuery]);

    const renderMainPane = () => {
        const cat = filteredCategories.find(c => c.id === activeCategory) || filteredCategories[0];
        if (!cat) return null;
        if (cat.customRender) return cat.customRender();
        if (cat.agentCategory) return <AgentSettingsView category={cat.agentCategory} hideHeader />;

        // Editor fallbacks
        if (cat.id === 'editor') {
            return (
                <div style={{ maxWidth: 720 }}>
                    <SectionTitle>Editor</SectionTitle>
                    <SettingsRow
                        label="Font Size"
                        description="Controls the font size in pixels."
                        control={<input type="number" className="settings-input" style={{ width: 80 }} value={vsSettings.font_size} onChange={e => handleVsSettingChange('font_size', parseInt(e.target.value))} />}
                    />
                    <SettingsRow
                        label="Tab Size"
                        description="The number of spaces a tab is equal to."
                        control={<input type="number" className="settings-input" style={{ width: 80 }} value={vsSettings.tab_size || 4} onChange={e => handleVsSettingChange('tab_size', parseInt(e.target.value))} />}
                    />
                    <SettingsRow
                        label="Auto Save"
                        description="Controls auto save of modified files."
                        control={
                            <select className="settings-select" value={vsSettings.auto_save || 'off'} onChange={e => handleVsSettingChange('auto_save', e.target.value)}>
                                <option value="off">Off</option>
                                <option value="afterDelay">After Delay</option>
                                <option value="onFocusChange">On Focus Change</option>
                                <option value="onWindowChange">On Window Change</option>
                            </select>
                        }
                    />
                </div>
            );
        }
        if (cat.id === 'theme') {
            return (
                <div style={{ maxWidth: 720 }}>
                    <SectionTitle>Theme</SectionTitle>
                    <SettingsRow
                        label="Color Theme"
                        control={
                            <select className="settings-select" value={vsSettings.theme} onChange={e => handleVsSettingChange('theme', e.target.value)}>
                                <option value="vs-dark">Visual Studio Dark</option>
                                <option value="Monokai">Monokai</option>
                                <option value="Airi Purple">AIRI Purple</option>
                            </select>
                        }
                    />
                </div>
            );
        }
        return null;
    };

    return (
        <div className="settings-container">
            <div className="settings-header">
                <div className="settings-title-row">
                    <h1 className="settings-title">Settings</h1>
                    <div className="settings-search-wrapper" style={{ position: 'relative' }}>
                        <i className="codicon codicon-search" style={{ position: 'absolute', left: 10, top: '50%', transform: 'translateY(-50%)', opacity: 0.5, fontSize: 14 }}></i>
                        <input
                            type="text"
                            className="settings-search-input"
                            placeholder="Search settings..."
                            value={searchQuery}
                            onChange={(e) => setSearchQuery(e.target.value)}
                            style={{ paddingLeft: 32 }}
                        />
                    </div>
                </div>

                <div className="settings-tabs">
                    <div className={`settings-tab ${topTab === 'agent' ? 'active' : ''}`} onClick={() => setTopTab('agent')}>IDE Settings</div>
                    <div className={`settings-tab ${topTab === 'vscode' ? 'active' : ''}`} onClick={() => setTopTab('vscode')}>Editor Settings</div>
                </div>
            </div>

            <div className="settings-body">
                <div className="settings-sidebar">
                    {filteredCategories.map(c => (
                        <div key={c.id}>
                            {c.groupStart && !searchQuery && <div className="settings-sidebar-group">{c.groupStart}</div>}
                            <div className={`settings-sidebar-item ${activeCategory === c.id ? 'active' : ''}`} onClick={() => setActiveCategory(c.id)}>
                                <i className={`codicon codicon-${c.icon}`}></i>
                                {c.label}
                            </div>
                        </div>
                    ))}
                </div>
                <div className="settings-content">
                    {renderMainPane()}
                </div>
            </div>
        </div>
    );
};

export default SettingsPage;
