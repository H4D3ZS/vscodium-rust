import React, { useState, useEffect, useMemo } from 'react';
import { invoke } from '../tauri_bridge';
import { useStore } from '../store';
import AgentSettingsView, { AgentSettingsCategory } from './AgentSettingsView';

// ─────────────────────────────────────────────────────────────────────────────
//  Settings page — Cursor-style two-tab + left-sidebar layout
//
//  Top tabs:
//    • vscodium-rust Settings  — agentic / AI / MCP / Ollama / Voice / Avatar
//    • VS Code Settings        — classic editor settings (font, theme, etc.)
//
//  Inside each top tab, a vertical category sidebar (left) drives which
//  panel is shown in the main pane (right). This mirrors the layout the
//  user pointed at in their reference screenshot of Cursor IDE Settings.
//
//  The agentic categories below render `<AgentSettingsView category="…">`
//  which we refactored to filter its sections by category. Categories
//  the user hasn't created yet (Tools/Hooks/Beta) get inline placeholder
//  panels so the navigation is consistent and we can add real surfaces
//  later without re-touching this file's structure.
// ─────────────────────────────────────────────────────────────────────────────

interface VsCodeSettings {
    theme: string;
    font_size: number;
    tab_size?: number;
    auto_save?: string;
}

type TopTab = 'agent' | 'vscode';

// Categories shown under each top tab.
interface CategoryDef {
    id: string;
    label: string;
    icon: string;   // codicon name
    /** When set, the page hands this off to AgentSettingsView. */
    agentCategory?: AgentSettingsCategory;
    /** When set, page renders a custom node instead of AgentSettingsView. */
    customRender?: () => React.ReactNode;
}

function readInitialTab(): TopTab {
    try {
        const t = sessionStorage.getItem('settings.initialTab');
        // Legacy values mapped to the new shape:
        //   'agent'      → vscodium-rust Settings tab
        //   'user'/'workspace' → VS Code Settings tab
        if (t === 'agent') return 'agent';
        if (t === 'user' || t === 'workspace' || t === 'vscode') return 'vscode';
    } catch { /* sessionStorage unavailable */ }
    return 'agent';
}

function readInitialCategory(tab: TopTab): string {
    try {
        const k = `settings.category.${tab}`;
        const v = localStorage.getItem(k);
        if (v) return v;
    } catch { /* tracking prevention / private mode */ }
    return tab === 'agent' ? 'general' : 'editor';
}

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

    // ── load editor settings once ────────────────────────────────────────
    useEffect(() => {
        invoke<VsCodeSettings>('get_settings')
            .then(setVsSettings)
            .catch(err => console.error('[Settings] get_settings failed:', err));
        // Consume the one-shot initial-tab hint so a later File→Settings
        // open doesn't get stuck on a stale destination.
        try { sessionStorage.removeItem('settings.initialTab'); } catch { /* no-op */ }
    }, []);

    // ── persist active category whenever the user clicks one ─────────────
    useEffect(() => {
        try { localStorage.setItem(`settings.category.${topTab}`, activeCategory); }
        catch { /* tracking prevention / private mode */ }
    }, [topTab, activeCategory]);

    // ── External focus events (e.g. right-sidebar gear click) ────────────
    useEffect(() => {
        const onFocus = (e: Event) => {
            const detail = ((e as CustomEvent).detail || {}) as { tab?: string; category?: string };
            if (detail.tab === 'agent') setTopTab('agent');
            if (detail.tab === 'vscode' || detail.tab === 'user' || detail.tab === 'workspace') setTopTab('vscode');
            if (detail.category) setActiveCategory(detail.category);
        };
        window.addEventListener('settings:focus-tab', onFocus as EventListener);
        return () => window.removeEventListener('settings:focus-tab', onFocus as EventListener);
    }, []);

    // ── Save handler for the editor (VS Code) settings ───────────────────
    const handleVsSettingChange = async (key: keyof VsCodeSettings, value: any) => {
        const next = { ...vsSettings, [key]: value };
        setVsSettings(next);
        try {
            await invoke('update_settings', { newSettings: next });
            if (key === 'theme') setTheme(value);
        } catch (err) {
            console.error('[Settings] update_settings failed:', err);
        }
    };

    // ── Category definitions ─────────────────────────────────────────────
    // Keep these arrays in render scope so the sidebar can group/filter
    // them by search query at render time.
    const agentCategories: CategoryDef[] = [
        { id: 'general',   label: 'General',          icon: 'gear',        agentCategory: undefined,    customRender: () => <AgentGeneralPanel /> },
        { id: 'models',    label: 'Models & API Keys', icon: 'symbol-misc', agentCategory: 'models' },
        { id: 'ollama',    label: 'Ollama (Local & Remote)', icon: 'server', agentCategory: 'ollama' },
        { id: 'agents',    label: 'Agents',           icon: 'robot',       customRender: () => <AgentBehaviourPanel /> },
        { id: 'mcps',      label: 'Tools & MCPs',     icon: 'plug',        agentCategory: 'mcps' },
        { id: 'memory',    label: 'Memory (.aim)',    icon: 'database',    agentCategory: 'memory' },
        { id: 'voice',     label: 'Voice & TTS',      icon: 'unmute',      agentCategory: 'voice' },
        { id: 'avatar',    label: 'AI Avatar',        icon: 'person',      agentCategory: 'avatar' },
    ];

    const vsCategories: CategoryDef[] = [
        { id: 'editor', label: 'Editor', icon: 'edit' },
        { id: 'files',  label: 'Files',  icon: 'files' },
        { id: 'theme',  label: 'Theme',  icon: 'symbol-color' },
    ];

    const activeList = topTab === 'agent' ? agentCategories : vsCategories;
    const filteredCategories = useMemo(() => {
        if (!searchQuery.trim()) return activeList;
        const q = searchQuery.trim().toLowerCase();
        return activeList.filter(c => c.label.toLowerCase().includes(q));
    }, [activeList, searchQuery]);

    // Snap to the first visible category when the query changes
    useEffect(() => {
        if (filteredCategories.length === 0) return;
        if (!filteredCategories.some(c => c.id === activeCategory)) {
            setActiveCategory(filteredCategories[0].id);
        }
    }, [filteredCategories, activeCategory]);

    // ── Sub-panels for categories that aren't AgentSettingsView slices ───

    function AgentGeneralPanel() {
        return (
            <div className="settings-section" style={{ maxWidth: 720 }}>
                <div className="settings-section-title">General</div>
                <p style={{ opacity: 0.7, fontSize: 12, lineHeight: 1.55 }}>
                    Welcome to the AI Agent settings. Use the sidebar to navigate.
                </p>
                <ul style={{ fontSize: 12, opacity: 0.85, lineHeight: 1.8, paddingLeft: '1.2em' }}>
                    <li><b>Models & API Keys</b> — pick a default model, add cloud provider keys.</li>
                    <li><b>Ollama</b> — Local / Auto / Remote server, override base URL, bearer token.</li>
                    <li><b>Agents</b> — autonomy mode, YOLO, completion gate behaviour.</li>
                    <li><b>Tools &amp; MCPs</b> — enable / add Model Context Protocol servers.</li>
                    <li><b>Memory (.aim)</b> — inspect Kortex persistent memory slots and telemetry.</li>
                    <li><b>Voice &amp; TTS</b> — ElevenLabs / browser TTS, preset voices.</li>
                    <li><b>AI Avatar</b> — VRM 3D avatar character + custom URLs.</li>
                </ul>
            </div>
        );
    }

    function AgentBehaviourPanel() {
        // Tiny inline editor for behaviour toggles. We bind directly to the
        // store so we don't fan out the agent-config IPC just for this panel.
        const agentMode = useStore(s => s.agentMode);
        const setAgentMode = useStore(s => s.setAgentMode);
        const isYolo = useStore(s => s.isYoloMode);
        // Store exposes the setter as `setYoloMode` (see store.ts ~L1762).
        const setYolo = useStore(s => (s as any).setYoloMode);

        return (
            <div className="settings-section" style={{ maxWidth: 720, display: 'flex', flexDirection: 'column', gap: 16 }}>
                <div className="settings-section-title">Agents</div>

                <div className="settings-item">
                    <div className="settings-item-header">
                        <div className="settings-item-label">Default Agent Mode</div>
                        <div className="settings-item-description">
                            Mode the right-sidebar chat opens with. Chat is read-only; Agent &amp;
                            Plan can write files and run commands. Bug Bounty / Sentient bias the
                            system prompt toward offensive security workflows.
                        </div>
                    </div>
                    <div className="settings-item-control">
                        <select
                            value={agentMode}
                            onChange={(e) => setAgentMode(e.target.value as any)}
                        >
                            {['Agent', 'Chat', 'Plan', 'Bug Bounty', 'Sentient', 'Verification'].map(m => (
                                <option key={m} value={m}>{m}</option>
                            ))}
                        </select>
                    </div>
                </div>

                <div className="settings-item">
                    <div className="settings-item-header">
                        <div className="settings-item-label">YOLO Mode</div>
                        <div className="settings-item-description">
                            Auto-escalate Chat → Agent when the prompt clearly asks for action
                            (write/run/build/etc.) instead of nagging you to switch modes.
                        </div>
                    </div>
                    <div className="settings-item-control">
                        <label style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                            <input
                                type="checkbox"
                                checked={!!isYolo}
                                onChange={(e) => setYolo?.(e.target.checked)}
                            />
                            <span style={{ fontSize: 12, opacity: 0.85 }}>Enable</span>
                        </label>
                    </div>
                </div>
            </div>
        );
    }

    // ── VS Code (editor) panels ──────────────────────────────────────────
    function renderVsCodePanel() {
        const item = (label: string, description: string, control: React.ReactNode) => (
            <div className="settings-item" key={label}>
                <div className="settings-item-header">
                    <div className="settings-item-label">{label}</div>
                    <div className="settings-item-description">{description}</div>
                </div>
                <div className="settings-item-control">{control}</div>
            </div>
        );

        if (activeCategory === 'theme') {
            return (
                <div className="settings-section" style={{ maxWidth: 720 }}>
                    <div className="settings-section-title">Theme</div>
                    {item(
                        'Workbench: Color Theme',
                        'Specifies the color theme used in the workbench.',
                        <select
                            value={vsSettings.theme}
                            onChange={(e) => handleVsSettingChange('theme', e.target.value)}
                        >
                            <option value="vs-dark">Dark (Visual Studio)</option>
                            <option value="vs">Light (Visual Studio)</option>
                            <option value="Darcula">Darcula</option>
                            <option value="Monokai">Monokai</option>
                            <option value="Solarized Dark">Solarized Dark</option>
                        </select>
                    )}
                </div>
            );
        }

        if (activeCategory === 'files') {
            return (
                <div className="settings-section" style={{ maxWidth: 720 }}>
                    <div className="settings-section-title">Files</div>
                    {item(
                        'Files: Auto Save',
                        'Controls auto save of editors that have unsaved changes.',
                        <select
                            value={vsSettings.auto_save || 'off'}
                            onChange={(e) => handleVsSettingChange('auto_save', e.target.value)}
                        >
                            <option value="off">off</option>
                            <option value="afterDelay">afterDelay</option>
                            <option value="onFocusChange">onFocusChange</option>
                            <option value="onWindowChange">onWindowChange</option>
                        </select>
                    )}
                </div>
            );
        }

        // Default: editor
        return (
            <div className="settings-section" style={{ maxWidth: 720 }}>
                <div className="settings-section-title">Editor</div>
                {item(
                    'Editor: Font Size',
                    'Controls the font size in pixels.',
                    <input
                        type="number"
                        value={vsSettings.font_size}
                        onChange={(e) => handleVsSettingChange('font_size', parseInt(e.target.value || '14', 10))}
                    />
                )}
                {item(
                    'Editor: Tab Size',
                    'The number of spaces a tab is equal to.',
                    <input
                        type="number"
                        value={vsSettings.tab_size || 4}
                        onChange={(e) => handleVsSettingChange('tab_size', parseInt(e.target.value || '4', 10))}
                    />
                )}
            </div>
        );
    }

    // ── Main pane: pick the right renderer based on the active category ─
    function renderMainPane() {
        if (topTab === 'vscode') return renderVsCodePanel();

        const def = agentCategories.find(c => c.id === activeCategory) || agentCategories[0];
        if (def.customRender) return def.customRender();
        if (def.agentCategory) {
            return (
                <div style={{ padding: 0, height: '100%', overflow: 'hidden' }}>
                    <AgentSettingsView category={def.agentCategory} hideHeader />
                </div>
            );
        }
        return <AgentGeneralPanel />;
    }

    return (
        <div
            className="settings-container"
            style={{
                display: 'flex',
                flexDirection: 'column',
                height: '100%',
                background: 'var(--vscode-editor-background)',
                color: 'var(--vscode-foreground)',
            }}
        >
            {/* ── Top header: title + search + top-level tabs ─────────── */}
            <div
                style={{
                    flex: '0 0 auto',
                    padding: '16px 24px 0 24px',
                    borderBottom: '1px solid var(--vscode-panel-border)',
                }}
            >
                <h1 style={{ fontSize: 22, fontWeight: 600, margin: 0 }}>Settings</h1>
                <div style={{ position: 'relative', maxWidth: 520, marginTop: 10 }}>
                    <input
                        type="text"
                        placeholder="Search settings"
                        value={searchQuery}
                        onChange={(e) => setSearchQuery(e.target.value)}
                        style={{
                            width: '100%',
                            paddingLeft: 30,
                            padding: '6px 30px',
                            fontSize: 12,
                            background: 'var(--vscode-input-background)',
                            color: 'var(--vscode-input-foreground)',
                            border: '1px solid var(--vscode-input-border)',
                            borderRadius: 4,
                            boxSizing: 'border-box',
                        }}
                    />
                    <i
                        className="codicon codicon-search"
                        style={{
                            position: 'absolute',
                            left: 8,
                            top: '50%',
                            transform: 'translateY(-50%)',
                            opacity: 0.5,
                            fontFamily: 'codicon',
                            fontStyle: 'normal',
                        }}
                    />
                </div>

                <div style={{ display: 'flex', gap: 24, marginTop: 14 }}>
                    {(
                        [
                            { id: 'agent',  label: 'vscodium-rust Settings' },
                            { id: 'vscode', label: 'VS Code Settings' },
                        ] as { id: TopTab; label: string }[]
                    ).map(t => (
                        <div
                            key={t.id}
                            onClick={() => {
                                setTopTab(t.id);
                                // When swapping tabs, restore that tab's last
                                // category instead of stranding the user on a
                                // category that doesn't exist in the new tab.
                                setActiveCategory(readInitialCategory(t.id));
                            }}
                            style={{
                                padding: '8px 0',
                                fontSize: 13,
                                cursor: 'pointer',
                                fontWeight: topTab === t.id ? 600 : 400,
                                opacity: topTab === t.id ? 1 : 0.55,
                                borderBottom: topTab === t.id ? '2px solid var(--vscode-focusBorder, #0e639c)' : '2px solid transparent',
                                marginBottom: -1,
                            }}
                        >
                            {t.label}
                        </div>
                    ))}
                </div>
            </div>

            {/* ── Body: sidebar + main pane ──────────────────────────── */}
            <div style={{ flex: '1 1 auto', display: 'flex', minHeight: 0 }}>
                <div
                    style={{
                        flex: '0 0 220px',
                        borderRight: '1px solid var(--vscode-panel-border)',
                        padding: '12px 0',
                        overflowY: 'auto',
                        background: 'var(--vscode-sideBar-background)',
                    }}
                >
                    {filteredCategories.map(c => (
                        <div
                            key={c.id}
                            onClick={() => setActiveCategory(c.id)}
                            style={{
                                display: 'flex',
                                alignItems: 'center',
                                gap: 10,
                                padding: '6px 16px',
                                cursor: 'pointer',
                                fontSize: 12,
                                background: activeCategory === c.id ? 'var(--vscode-list-activeSelectionBackground, rgba(14,99,156,0.4))' : 'transparent',
                                color: activeCategory === c.id ? 'var(--vscode-list-activeSelectionForeground, #fff)' : 'inherit',
                                borderLeft: activeCategory === c.id ? '2px solid var(--vscode-focusBorder, #0e639c)' : '2px solid transparent',
                            }}
                            onMouseEnter={(e) => {
                                if (activeCategory !== c.id) e.currentTarget.style.background = 'rgba(255,255,255,0.04)';
                            }}
                            onMouseLeave={(e) => {
                                if (activeCategory !== c.id) e.currentTarget.style.background = 'transparent';
                            }}
                        >
                            <i
                                className={`codicon codicon-${c.icon}`}
                                style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: 14, opacity: 0.8 }}
                            />
                            {c.label}
                        </div>
                    ))}
                    {filteredCategories.length === 0 && (
                        <div style={{ padding: '8px 16px', fontSize: 11, opacity: 0.5 }}>
                            No settings match "{searchQuery}".
                        </div>
                    )}
                </div>

                <div
                    style={{
                        flex: '1 1 auto',
                        overflowY: 'auto',
                        padding: '20px 28px',
                    }}
                >
                    {renderMainPane()}
                </div>
            </div>
        </div>
    );
};

export default SettingsPage;
