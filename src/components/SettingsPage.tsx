import React, { useState, useEffect, useMemo, useCallback } from 'react';
import { invoke } from '../tauri_bridge';
import { useStore } from '../store';
import AgentSettingsView, { type AgentSettingsCategory } from './AgentSettingsView';
import KeybindingsPanel from './KeybindingsPanel';

// ─────────────────────────────────────────────────────────────────────────────
//  Settings page — Cursor IDE parity layout.
//
//  Top tabs:
//    • vscodium-rust Settings  — the Cursor-equivalent agentic / IDE
//                                feature surfaces (General, Models, Tab,
//                                Agents, Cloud Agents, Plan & Usage,
//                                Rules/Skills/Subagents, Tools & MCPs,
//                                Hooks, Indexing & Docs, Memory, Voice,
//                                Avatar, Network, Beta, Plugins,
//                                Marketplace, Docs).
//    • VS Code Settings        — classic editor settings (font, theme).
//
//  Categories under each top tab populate a vertical sidebar. The sidebar
//  is filterable by the search input at the top. Each agent category
//  either delegates to <AgentSettingsView category="…"> (which filters its
//  sections via `visibleInCategory`) or renders an inline panel defined
//  locally in this file. Inline panels are kept here on purpose because
//  they're small and live entirely off the existing store/IPC surface.
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
    /** Visual divider above this entry (matches Cursor's grouped sidebar). */
    groupStart?: string;
}

function readInitialTab(): TopTab {
    try {
        const t = sessionStorage.getItem('settings.initialTab');
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

// ── Small shared building block ──────────────────────────────────────────────
// Cursor's settings UI is a stream of "label / description / control" rows.
// Pull that into one component so every panel below has the same shape.
const SettingsRow: React.FC<{
    label: string;
    description?: React.ReactNode;
    control: React.ReactNode;
}> = ({ label, description, control }) => (
    <div className="settings-item">
        <div className="settings-item-header">
            <div className="settings-item-label">{label}</div>
            {description && <div className="settings-item-description">{description}</div>}
        </div>
        <div className="settings-item-control">{control}</div>
    </div>
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

    // ── load editor settings once ────────────────────────────────────────
    useEffect(() => {
        invoke<VsCodeSettings>('get_settings')
            .then(setVsSettings)
            .catch(err => console.error('[Settings] get_settings failed:', err));
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
    // Order mirrors Cursor's sidebar, grouped into sections for visual
    // parity. `groupStart` adds a small label above an entry to act as
    // the divider Cursor uses between e.g. "Models" and "Tools".
    const agentCategories: CategoryDef[] = [
        { id: 'general',     label: 'General',                icon: 'gear',          customRender: () => <AgentGeneralPanel /> },
        { id: 'plan',        label: 'Plan & Usage',           icon: 'graph-line',    customRender: () => <PlanUsagePanel /> },
        { id: 'agents',      label: 'Agents',                 icon: 'robot',         customRender: () => <AgentBehaviourPanel />, groupStart: 'Agentic' },
        { id: 'tab',         label: 'Tab',                    icon: 'symbol-keyword', customRender: () => <CursorTabPanel /> },
        { id: 'keybindings', label: 'Keybindings',            icon: 'keyboard',      customRender: () => <KeybindingsPanel /> },
        { id: 'models',      label: 'Models',                 icon: 'symbol-misc',   agentCategory: 'models' },
        { id: 'cloud',       label: 'Cloud Agents',           icon: 'cloud',         customRender: () => <CloudAgentsPanel /> },
        { id: 'plugins',     label: 'Plugins',                icon: 'extensions',    customRender: () => <PluginsPanel /> },
        { id: 'rules',       label: 'Rules, Skills, Subagents', icon: 'book',        customRender: () => <RulesSkillsPanel />, groupStart: 'Customization' },
        { id: 'mcps',        label: 'Tools & MCPs',           icon: 'plug',          agentCategory: 'mcps' },
        { id: 'hooks',       label: 'Hooks',                  icon: 'symbol-event',  customRender: () => <HooksPanel /> },
        { id: 'indexing',    label: 'Indexing & Docs',        icon: 'search',        customRender: () => <IndexingDocsPanel />, groupStart: 'Context' },
        { id: 'ollama',      label: 'Ollama',                 icon: 'server-environment', agentCategory: 'ollama' },
        { id: 'memory',      label: 'Memory (.aim)',          icon: 'database',      agentCategory: 'memory' },
        { id: 'voice',       label: 'Voice & TTS',            icon: 'unmute',        agentCategory: 'voice' },
        { id: 'avatar',      label: 'AI Avatar',              icon: 'person',        agentCategory: 'avatar' },
        { id: 'network',     label: 'Network',                icon: 'globe',         customRender: () => <NetworkPanel />, groupStart: 'System' },
        { id: 'beta',        label: 'Beta',                   icon: 'beaker',        customRender: () => <BetaPanel /> },
        { id: 'marketplace', label: 'Marketplace',            icon: 'extensions',    customRender: () => <MarketplacePanel />, groupStart: 'Resources' },
        { id: 'docs',        label: 'Docs',                   icon: 'book',          customRender: () => <DocsPanel /> },
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

    // Snap to the first visible category when the query changes.
    useEffect(() => {
        if (filteredCategories.length === 0) return;
        if (!filteredCategories.some(c => c.id === activeCategory)) {
            setActiveCategory(filteredCategories[0].id);
        }
    }, [filteredCategories, activeCategory]);

    // ═════════════════════════════════════════════════════════════════════
    // Sub-panels for categories that aren't AgentSettingsView slices.
    // Each is a tiny component reading directly from the store / IPC.
    // ═════════════════════════════════════════════════════════════════════

    function AgentGeneralPanel() {
        return (
            <div className="settings-section" style={{ maxWidth: 720 }}>
                <div className="settings-section-title">General</div>
                <p style={{ opacity: 0.7, fontSize: 12, lineHeight: 1.55 }}>
                    vscodium-rust IDE — Cursor-style settings. Use the sidebar to navigate every
                    agentic surface in the IDE.
                </p>
                <ul style={{ fontSize: 12, opacity: 0.85, lineHeight: 1.8, paddingLeft: '1.2em' }}>
                    <li><b>Plan &amp; Usage</b> — token throughput, model bindings, .aim telemetry.</li>
                    <li><b>Agents</b> — autonomy mode, YOLO escalation, completion gate.</li>
                    <li><b>Tab</b> — inline AI completions (Cursor Tab).</li>
                    <li><b>Models</b> — default model, cloud provider API keys, DeepSeek-ANE.</li>
                    <li><b>Cloud Agents</b> — background agents started with <code>/bg</code>.</li>
                    <li><b>Rules / Skills / Subagents</b> — workspace rules from <code>.cursor/rules</code>, <code>AGENTS.md</code>, <code>CLAUDE.md</code>.</li>
                    <li><b>Tools &amp; MCPs</b> — Model Context Protocol servers (stdio / http).</li>
                    <li><b>Hooks</b> — lifecycle hooks and event scripts.</li>
                    <li><b>Indexing &amp; Docs</b> — workspace indexer + documentation URLs.</li>
                    <li><b>Ollama</b> — Local / Auto / Remote toggles, bearer token, model picker.</li>
                    <li><b>Network</b> — proxy + self-signed TLS for self-hosted endpoints.</li>
                    <li><b>Beta</b> — experimental tool toggles (fast_apply, semantic_search, shadow VFS).</li>
                </ul>
            </div>
        );
    }

    function AgentBehaviourPanel() {
        const agentMode = useStore(s => s.agentMode);
        const setAgentMode = useStore(s => s.setAgentMode);
        const isYolo = useStore(s => s.isYoloMode);
        const setYolo = useStore(s => (s as any).setYoloMode);

        return (
            <div className="settings-section" style={{ maxWidth: 720, display: 'flex', flexDirection: 'column', gap: 16 }}>
                <div className="settings-section-title">Agents</div>

                <SettingsRow
                    label="Default Agent Mode"
                    description="Mode the right-sidebar chat opens with. Chat is read-only; Agent & Plan can write files and run commands. Bug Bounty / Sentient bias the system prompt toward offensive security workflows."
                    control={
                        <select value={agentMode} onChange={(e) => setAgentMode(e.target.value as any)}>
                            {['Agent', 'Chat', 'Plan', 'Bug Bounty', 'Sentient', 'Verification'].map(m => (
                                <option key={m} value={m}>{m}</option>
                            ))}
                        </select>
                    }
                />

                <SettingsRow
                    label="YOLO Mode"
                    description="Auto-escalate Chat → Agent when the prompt clearly asks for action (write/run/build/etc.) instead of nagging you to switch modes."
                    control={
                        <label style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                            <input
                                type="checkbox"
                                checked={!!isYolo}
                                onChange={(e) => setYolo?.(e.target.checked)}
                            />
                            <span style={{ fontSize: 12, opacity: 0.85 }}>Enable</span>
                        </label>
                    }
                />
            </div>
        );
    }

    // ─────────────────────────────────────────────────────────────────────
    // Cursor Tab — inline AI completion master switches.
    //
    // Currently wired to the store flags only; the editor consults the
    // same flags at completion time so disabling here actually silences
    // inline suggestions.
    // ─────────────────────────────────────────────────────────────────────
    function CursorTabPanel() {
        const enabled = useStore(s => s.tabPredictionEnabled);
        const multi = useStore(s => s.tabMultilineSuggestions);
        const acceptKey = useStore(s => s.tabAcceptKey);
        const setEnabled = useStore(s => s.setTabPredictionEnabled);
        const setMulti = useStore(s => s.setTabMultilineSuggestions);
        const setAcceptKey = useStore(s => s.setTabAcceptKey);

        return (
            <div className="settings-section" style={{ maxWidth: 720, display: 'flex', flexDirection: 'column', gap: 16 }}>
                <div className="settings-section-title">Tab (inline AI completions)</div>
                <p style={{ fontSize: 12, opacity: 0.7, marginTop: -4 }}>
                    Cursor Tab predicts the rest of the line (or block) you're typing using the
                    selected default model. Hit the accept key to apply.
                </p>
                <SettingsRow
                    label="Enable Cursor Tab"
                    description="Master switch. Disabling stops inline AI suggestions everywhere."
                    control={
                        <label style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                            <input type="checkbox" checked={enabled} onChange={(e) => setEnabled(e.target.checked)} />
                            <span style={{ fontSize: 12, opacity: 0.85 }}>Enable</span>
                        </label>
                    }
                />
                <SettingsRow
                    label="Multi-line suggestions"
                    description="Let predictions span more than one line. Off keeps suggestions to a single line."
                    control={
                        <label style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                            <input
                                type="checkbox"
                                checked={multi}
                                onChange={(e) => setMulti(e.target.checked)}
                                disabled={!enabled}
                            />
                            <span style={{ fontSize: 12, opacity: 0.85 }}>Enable</span>
                        </label>
                    }
                />
                <SettingsRow
                    label="Accept key"
                    description="Key used to commit the visible suggestion."
                    control={
                        <select
                            value={acceptKey}
                            onChange={(e) => setAcceptKey(e.target.value as 'Tab' | 'Enter')}
                            disabled={!enabled}
                        >
                            <option value="Tab">Tab</option>
                            <option value="Enter">Enter</option>
                        </select>
                    }
                />
            </div>
        );
    }

    // ─────────────────────────────────────────────────────────────────────
    // Cloud Agents — surfaces the background-agent registry the store
    // already maintains (started via `/bg` in chat). We let the user
    // cancel an in-flight one or clear the whole list.
    // ─────────────────────────────────────────────────────────────────────
    function CloudAgentsPanel() {
        const bgAgents = useStore(s => s.backgroundAgents);
        // Store exposes `removeBackgroundAgent`; we expose it as the
        // panel's "Cancel" affordance for running tasks. There's no
        // dedicated cancel command yet, so removing it from the list is
        // the closest thing to abort the UI can express.
        const removeBackgroundAgent = useStore(s => (s as any).removeBackgroundAgent);
        const clearBackgroundAgents = useStore(s => (s as any).clearBackgroundAgents);
        return (
            <div className="settings-section" style={{ maxWidth: 720 }}>
                <div className="settings-section-title">Cloud Agents</div>
                <p style={{ fontSize: 12, opacity: 0.7 }}>
                    Background agents started with <code>/bg &lt;prompt&gt;</code> in chat. Each one runs
                    detached against the configured default model.
                </p>
                {bgAgents.length === 0 && (
                    <div style={{ fontSize: 12, opacity: 0.55, padding: '12px 0' }}>
                        No background agents yet. Type <code>/bg refactor src/auth</code> in chat.
                    </div>
                )}
                {bgAgents.length > 0 && (
                    <div style={{ display: 'flex', flexDirection: 'column', gap: 10 }}>
                        {bgAgents.map(b => (
                            <div
                                key={b.id}
                                style={{
                                    border: '1px solid var(--vscode-panel-border)',
                                    padding: 10,
                                    borderRadius: 4,
                                    fontSize: 12,
                                }}
                            >
                                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 4 }}>
                                    <code style={{ opacity: 0.85 }}>{b.id.slice(0, 8)}</code>
                                    <span style={{
                                        fontSize: 11,
                                        padding: '1px 8px',
                                        borderRadius: 10,
                                        background:
                                            b.status === 'done'    ? 'rgba(0,180,0,0.18)' :
                                            b.status === 'running' ? 'rgba(0,120,200,0.22)' :
                                            b.status === 'error'   ? 'rgba(200,40,40,0.22)' :
                                                                     'rgba(255,255,255,0.08)',
                                    }}>{b.status}</span>
                                </div>
                                <div style={{ opacity: 0.85, marginBottom: 6 }}>
                                    {b.prompt.length > 220 ? b.prompt.slice(0, 220) + '…' : b.prompt}
                                </div>
                                {b.result && b.status !== 'running' && (
                                    <pre style={{
                                        margin: 0,
                                        fontSize: 11,
                                        opacity: 0.75,
                                        maxHeight: 120,
                                        overflow: 'auto',
                                        whiteSpace: 'pre-wrap',
                                    }}>{b.result.slice(0, 1000)}</pre>
                                )}
                                {b.status === 'running' && removeBackgroundAgent && (
                                    <button onClick={() => removeBackgroundAgent(b.id)} style={{ fontSize: 11 }}>Cancel</button>
                                )}
                            </div>
                        ))}
                        {clearBackgroundAgents && (
                            <button onClick={() => clearBackgroundAgents()} style={{ fontSize: 12, alignSelf: 'flex-start' }}>
                                Clear all
                            </button>
                        )}
                    </div>
                )}
            </div>
        );
    }

    // ─────────────────────────────────────────────────────────────────────
    // Plan & Usage — Kortex .aim telemetry snapshot. We poll once per
    // panel mount + give the user a manual refresh button.
    // ─────────────────────────────────────────────────────────────────────
    function PlanUsagePanel() {
        const [snapshot, setSnapshot] = useState<any>(null);
        const [error, setError] = useState<string | null>(null);
        const refresh = useCallback(async () => {
            try {
                setError(null);
                const snap = await invoke<any>('aim_telemetry_snapshot');
                setSnapshot(snap);
            } catch (e: any) {
                setError(String(e?.message || e));
            }
        }, []);
        useEffect(() => { refresh(); }, [refresh]);

        const samples = (snapshot?.samples || []) as Array<any>;
        const total = samples.length;
        const tokenSum = samples.reduce((acc, s) => acc + (s?.tokens_in || 0) + (s?.tokens_out || 0), 0);
        const lastModel = (snapshot?.bound_model_id as string) || 'none';

        return (
            <div className="settings-section" style={{ maxWidth: 720 }}>
                <div className="settings-section-title">Plan &amp; Usage</div>
                <p style={{ fontSize: 12, opacity: 0.7 }}>
                    Token throughput and model bindings tracked in the local <code>.aim</code> telemetry
                    file. This is your local plan — no cloud subscription required.
                </p>
                {error && (
                    <div style={{ fontSize: 12, color: '#e88', marginBottom: 8 }}>
                        Couldn't load telemetry: {error}
                    </div>
                )}
                <SettingsRow label="Bound model" description="The model id last attached to the .aim weight-map." control={<code style={{ fontSize: 12 }}>{lastModel}</code>} />
                <SettingsRow label="Samples recorded" description="Total telemetry samples in the in-memory ring buffer." control={<code style={{ fontSize: 12 }}>{total}</code>} />
                <SettingsRow label="Tokens (in + out)" description="Summed across the visible sample window." control={<code style={{ fontSize: 12 }}>{tokenSum.toLocaleString()}</code>} />
                <button onClick={refresh} style={{ fontSize: 12, marginTop: 8 }}>Refresh</button>
            </div>
        );
    }

    // ─────────────────────────────────────────────────────────────────────
    // Rules, Skills, Subagents — calls the new `list_workspace_rules`
    // Tauri command which surfaces .cursor/rules, .agents/rules,
    // .cursorrules, AGENTS.md, CLAUDE.md. We render the list and let the
    // user expand any one of them to inspect the injected content.
    // ─────────────────────────────────────────────────────────────────────
    function RulesSkillsPanel() {
        const [rules, setRules] = useState<Array<{ name: string; content: string; file_path: string }>>([]);
        const [error, setError] = useState<string | null>(null);
        const [expanded, setExpanded] = useState<Set<string>>(new Set());

        const refresh = useCallback(async () => {
            try {
                setError(null);
                const out = await invoke<{ count: number; rules: any[] }>('list_workspace_rules');
                setRules(out?.rules || []);
            } catch (e: any) {
                setError(String(e?.message || e));
            }
        }, []);
        useEffect(() => { refresh(); }, [refresh]);

        const toggle = (key: string) => {
            const next = new Set(expanded);
            if (next.has(key)) next.delete(key); else next.add(key);
            setExpanded(next);
        };

        return (
            <div className="settings-section" style={{ maxWidth: 820 }}>
                <div className="settings-section-title">Rules, Skills, Subagents</div>
                <p style={{ fontSize: 12, opacity: 0.7 }}>
                    Workspace rules injected into every system prompt. The agent reads from
                    <code> .cursor/rules</code>, <code>.agents/rules</code>, <code>.cursorrules</code>,
                    <code> AGENTS.md</code>, and <code>CLAUDE.md</code> at the repo root.
                </p>
                {error && (
                    <div style={{ fontSize: 12, color: '#e88', marginBottom: 8 }}>
                        Couldn't load rules: {error}
                    </div>
                )}
                {rules.length === 0 && !error && (
                    <div style={{ fontSize: 12, opacity: 0.55, padding: '12px 0' }}>
                        No rules detected. Create a file at <code>.cursor/rules/my-rule.mdc</code> or
                        edit <code>AGENTS.md</code> in the repo root to add one.
                    </div>
                )}
                <div style={{ display: 'flex', flexDirection: 'column', gap: 8 }}>
                    {rules.map(r => (
                        <div
                            key={r.file_path}
                            style={{
                                border: '1px solid var(--vscode-panel-border)',
                                borderRadius: 4,
                                padding: 8,
                                fontSize: 12,
                            }}
                        >
                            <div
                                onClick={() => toggle(r.file_path)}
                                style={{ display: 'flex', justifyContent: 'space-between', cursor: 'pointer' }}
                            >
                                <div>
                                    <b>{r.name}</b>
                                    <span style={{ opacity: 0.55, marginLeft: 8 }}>{r.file_path}</span>
                                </div>
                                <span style={{ opacity: 0.7 }}>
                                    {expanded.has(r.file_path) ? '▾' : '▸'}
                                </span>
                            </div>
                            {expanded.has(r.file_path) && (
                                <pre style={{
                                    marginTop: 8,
                                    background: 'var(--vscode-textCodeBlock-background, rgba(0,0,0,0.25))',
                                    padding: 8,
                                    borderRadius: 3,
                                    maxHeight: 320,
                                    overflow: 'auto',
                                    whiteSpace: 'pre-wrap',
                                    fontSize: 11,
                                }}>{r.content.slice(0, 8000)}{r.content.length > 8000 ? '\n…' : ''}</pre>
                            )}
                        </div>
                    ))}
                </div>
                <button onClick={refresh} style={{ fontSize: 12, marginTop: 10 }}>Refresh</button>
            </div>
        );
    }

    // ─────────────────────────────────────────────────────────────────────
    // Hooks — Cursor hooks live in `.cursor/hooks.json`. We just point the
    // user at the file and offer to open it. Real event execution is owned
    // by the hooks engine (.agents/hooks if present), the panel is a
    // settings discovery surface, not the runner.
    // ─────────────────────────────────────────────────────────────────────
    function HooksPanel() {
        const [present, setPresent] = useState<{ cursor: boolean; agents: boolean } | null>(null);
        useEffect(() => {
            // `list_directory` returns Vec<FileEntry>{ name, path, is_dir }
            // (see src-tauri/src/file_commands.rs). It errors when the
            // directory doesn't exist, which is exactly the signal we
            // want for "present / absent".
            (async () => {
                const cursorPresent = await invoke<any[]>('list_directory', { path: '.cursor' })
                    .then(entries => Array.isArray(entries) && entries.some(e => (e?.name || '') === 'hooks.json'))
                    .catch(() => false);
                const agentsPresent = await invoke<any[]>('list_directory', { path: '.agents/hooks' })
                    .then(() => true)
                    .catch(() => false);
                setPresent({ cursor: cursorPresent, agents: agentsPresent });
            })();
        }, []);
        return (
            <div className="settings-section" style={{ maxWidth: 720 }}>
                <div className="settings-section-title">Hooks</div>
                <p style={{ fontSize: 12, opacity: 0.7 }}>
                    Hooks fire scripts on agent lifecycle events (pre/post tool, before commit, etc).
                    vscodium-rust reads the same shape Cursor does — <code>.cursor/hooks.json</code> at the
                    workspace root.
                </p>
                <SettingsRow
                    label=".cursor/hooks.json"
                    description={present?.cursor ? 'Detected in this workspace.' : 'Not found. Create the file to register hooks.'}
                    control={<code style={{ fontSize: 11 }}>{present?.cursor ? 'present' : 'absent'}</code>}
                />
                <SettingsRow
                    label=".agents/hooks/"
                    description={present?.agents ? 'Native hook scripts directory detected.' : 'Optional — drop executable scripts here for native hooks.'}
                    control={<code style={{ fontSize: 11 }}>{present?.agents ? 'present' : 'absent'}</code>}
                />
                <p style={{ fontSize: 11, opacity: 0.55, marginTop: 12 }}>
                    See <code>~/.agents/skills/create-hook/SKILL.md</code> for the full hooks spec.
                </p>
            </div>
        );
    }

    // ─────────────────────────────────────────────────────────────────────
    // Indexing & Docs — reindex the workspace + manage docs URLs the agent
    // can pull into context.
    // ─────────────────────────────────────────────────────────────────────
    function IndexingDocsPanel() {
        const indexingEnabled = useStore(s => s.indexingEnabled);
        const setIndexingEnabled = useStore(s => s.setIndexingEnabled);
        const docsUrls = useStore(s => s.indexingDocsUrls);
        const setDocsUrls = useStore(s => s.setIndexingDocsUrls);
        const [newUrl, setNewUrl] = useState('');
        const [reindexBusy, setReindexBusy] = useState(false);
        const [reindexResult, setReindexResult] = useState<string | null>(null);

        const reindex = async () => {
            setReindexBusy(true);
            setReindexResult(null);
            try {
                // Trigger the context indexer directly. reindex_if_needed
                // skips work when nothing changed, so this is safe to spam.
                const out = await invoke<any>('reindex_workspace');
                setReindexResult(typeof out === 'string' ? out : JSON.stringify(out, null, 2));
            } catch (e: any) {
                setReindexResult(`Error: ${String(e?.message || e)}`);
            } finally {
                setReindexBusy(false);
            }
        };

        const addUrl = () => {
            const u = newUrl.trim();
            if (!u) return;
            if (docsUrls.includes(u)) { setNewUrl(''); return; }
            setDocsUrls([...docsUrls, u]);
            setNewUrl('');
        };
        const removeUrl = (u: string) => setDocsUrls(docsUrls.filter(x => x !== u));

        return (
            <div className="settings-section" style={{ maxWidth: 720, display: 'flex', flexDirection: 'column', gap: 16 }}>
                <div className="settings-section-title">Indexing &amp; Docs</div>

                <SettingsRow
                    label="Index workspace"
                    description="Let the context_indexer scan the workspace for semantic search and codebase tools."
                    control={
                        <label style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                            <input type="checkbox" checked={indexingEnabled} onChange={(e) => setIndexingEnabled(e.target.checked)} />
                            <span style={{ fontSize: 12, opacity: 0.85 }}>Enable</span>
                        </label>
                    }
                />

                <div className="settings-item">
                    <div className="settings-item-header">
                        <div className="settings-item-label">Re-index now</div>
                        <div className="settings-item-description">
                            Force the indexer to rebuild. Useful after large branch swaps or pulling
                            a fresh checkout.
                        </div>
                    </div>
                    <div className="settings-item-control">
                        <button onClick={reindex} disabled={reindexBusy} style={{ fontSize: 12 }}>
                            {reindexBusy ? 'Indexing…' : 'Re-index'}
                        </button>
                    </div>
                </div>
                {reindexResult && (
                    <pre style={{
                        margin: 0,
                        background: 'var(--vscode-textCodeBlock-background, rgba(0,0,0,0.25))',
                        padding: 10,
                        borderRadius: 3,
                        fontSize: 11,
                        maxHeight: 200,
                        overflow: 'auto',
                        whiteSpace: 'pre-wrap',
                    }}>{reindexResult}</pre>
                )}

                <div className="settings-item">
                    <div className="settings-item-header">
                        <div className="settings-item-label">Documentation URLs</div>
                        <div className="settings-item-description">
                            URLs the agent's <code>web_search</code> / <code>browse</code> tools can pull into context
                            when answering questions about a specific framework or vendor.
                        </div>
                    </div>
                    <div className="settings-item-control" style={{ display: 'flex', flexDirection: 'column', gap: 6, alignItems: 'stretch', minWidth: 320 }}>
                        <div style={{ display: 'flex', gap: 6 }}>
                            <input
                                type="text"
                                value={newUrl}
                                placeholder="https://docs.example.com/"
                                onChange={(e) => setNewUrl(e.target.value)}
                                onKeyDown={(e) => { if (e.key === 'Enter') addUrl(); }}
                                style={{ flex: 1, fontSize: 12 }}
                            />
                            <button onClick={addUrl} style={{ fontSize: 12 }}>Add</button>
                        </div>
                        {docsUrls.length === 0 && (
                            <div style={{ fontSize: 11, opacity: 0.55 }}>No documentation URLs added yet.</div>
                        )}
                        {docsUrls.map(u => (
                            <div key={u} style={{ display: 'flex', gap: 6, alignItems: 'center', justifyContent: 'space-between', fontSize: 12 }}>
                                <code style={{ opacity: 0.85, wordBreak: 'break-all' }}>{u}</code>
                                <button onClick={() => removeUrl(u)} style={{ fontSize: 11 }}>Remove</button>
                            </div>
                        ))}
                    </div>
                </div>
            </div>
        );
    }

    // ─────────────────────────────────────────────────────────────────────
    // Network — proxy, insecure TLS, basic connectivity probe.
    // ─────────────────────────────────────────────────────────────────────
    function NetworkPanel() {
        const proxy = useStore(s => s.networkProxyUrl);
        const setProxy = useStore(s => s.setNetworkProxyUrl);
        const insecure = useStore(s => s.networkAllowInsecureTls);
        const setInsecure = useStore(s => s.setNetworkAllowInsecureTls);
        const [probe, setProbe] = useState<string | null>(null);
        const runProbe = async () => {
            setProbe('…probing');
            try {
                const status = await invoke<any>('check_ollama_status');
                setProbe(`Ollama: ${typeof status === 'string' ? status : JSON.stringify(status)}`);
            } catch (e: any) {
                setProbe(`Ollama: ${String(e?.message || e)}`);
            }
        };
        return (
            <div className="settings-section" style={{ maxWidth: 720, display: 'flex', flexDirection: 'column', gap: 16 }}>
                <div className="settings-section-title">Network</div>
                <SettingsRow
                    label="HTTP / HTTPS proxy"
                    description="Outbound proxy URL (e.g. http://corp-proxy:8080). Leave blank for direct."
                    control={
                        <input
                            type="text"
                            value={proxy}
                            onChange={(e) => setProxy(e.target.value)}
                            placeholder="http://host:port"
                            style={{ fontSize: 12, minWidth: 280 }}
                        />
                    }
                />
                <SettingsRow
                    label="Allow insecure TLS"
                    description="Accept self-signed certificates. Useful for self-hosted Ollama / DeepSeek-ANE servers behind a private CA."
                    control={
                        <label style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                            <input type="checkbox" checked={insecure} onChange={(e) => setInsecure(e.target.checked)} />
                            <span style={{ fontSize: 12, opacity: 0.85 }}>Enable</span>
                        </label>
                    }
                />
                <SettingsRow
                    label="Connection diagnostics"
                    description="Ping the active Ollama endpoint to verify reachability."
                    control={
                        <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
                            <button onClick={runProbe} style={{ fontSize: 12 }}>Run probe</button>
                            {probe && <code style={{ fontSize: 11, opacity: 0.8 }}>{probe.slice(0, 120)}</code>}
                        </div>
                    }
                />
            </div>
        );
    }

    // ─────────────────────────────────────────────────────────────────────
    // Beta — experimental flags. Each one maps to a runtime feature.
    // ─────────────────────────────────────────────────────────────────────
    function BetaPanel() {
        const fast = useStore(s => s.betaFastApply);
        const sem = useStore(s => s.betaSemanticSearch);
        const shadow = useStore(s => s.betaShadowWorkspace);
        const setFast = useStore(s => s.setBetaFastApply);
        const setSem = useStore(s => s.setBetaSemanticSearch);
        const setShadow = useStore(s => s.setBetaShadowWorkspace);
        return (
            <div className="settings-section" style={{ maxWidth: 720, display: 'flex', flexDirection: 'column', gap: 16 }}>
                <div className="settings-section-title">Beta</div>
                <p style={{ fontSize: 12, opacity: 0.7, marginTop: -6 }}>
                    Experimental features. Turning them off removes the matching tool from the
                    model's catalog at runtime.
                </p>
                <SettingsRow
                    label="fast_apply tool"
                    description="Lets the agent apply patches via diff-style search/replace instead of writing full files. Faster on large files."
                    control={<input type="checkbox" checked={fast} onChange={(e) => setFast(e.target.checked)} />}
                />
                <SettingsRow
                    label="semantic_search tool"
                    description="Embedding-backed semantic search over the workspace. Requires indexing to be enabled."
                    control={<input type="checkbox" checked={sem} onChange={(e) => setSem(e.target.checked)} />}
                />
                <SettingsRow
                    label="Shadow workspace preview"
                    description="Apply edits to a shadow VFS and render a green/red diff before committing to disk."
                    control={<input type="checkbox" checked={shadow} onChange={(e) => setShadow(e.target.checked)} />}
                />
            </div>
        );
    }

    // ─────────────────────────────────────────────────────────────────────
    // Plugins — VS Code extension surface. We don't manage installs from
    // here yet (that lives in the Extensions view); the panel exists so
    // the sidebar matches Cursor's layout and so we have one place to add
    // toggle wiring later.
    // ─────────────────────────────────────────────────────────────────────
    function PluginsPanel() {
        return (
            <div className="settings-section" style={{ maxWidth: 720 }}>
                <div className="settings-section-title">Plugins</div>
                <p style={{ fontSize: 12, opacity: 0.7 }}>
                    VS Code extensions install and manage in the dedicated Extensions panel
                    (Activity Bar → Extensions). This page lists configuration that affects all
                    extensions globally.
                </p>
                <SettingsRow
                    label="Open Extensions view"
                    description="Switch the workbench to the Extensions panel."
                    control={
                        <button onClick={() => {
                            window.dispatchEvent(new CustomEvent('workbench:open-view', { detail: { view: 'extensions' } }));
                        }} style={{ fontSize: 12 }}>Open</button>
                    }
                />
            </div>
        );
    }

    function MarketplacePanel() {
        return (
            <div className="settings-section" style={{ maxWidth: 720 }}>
                <div className="settings-section-title">Marketplace</div>
                <p style={{ fontSize: 12, opacity: 0.7 }}>
                    Browse skills, hooks, MCP servers and rule packs shared by the community.
                </p>
                <SettingsRow
                    label="Skills (Agent SKILL.md)"
                    description="Drop a SKILL.md under ~/.agents/skills/&lt;name&gt;/SKILL.md to install a new skill the agent can auto-discover."
                    control={<code style={{ fontSize: 11 }}>~/.agents/skills/</code>}
                />
                <SettingsRow
                    label="MCP registry"
                    description="Public registry of Model Context Protocol servers."
                    control={
                        <a href="https://github.com/modelcontextprotocol/servers" target="_blank" rel="noreferrer" style={{ fontSize: 12 }}>
                            github.com/modelcontextprotocol/servers
                        </a>
                    }
                />
            </div>
        );
    }

    function DocsPanel() {
        return (
            <div className="settings-section" style={{ maxWidth: 720 }}>
                <div className="settings-section-title">Docs</div>
                <p style={{ fontSize: 12, opacity: 0.7 }}>
                    Reference documentation for the agentic surfaces in vscodium-rust IDE.
                </p>
                <SettingsRow
                    label="Slash commands"
                    description="Type /help in the right-sidebar chat for the live list (init, bg, redteam, weaponize, …)."
                    control={<code style={{ fontSize: 11 }}>/help</code>}
                />
                <SettingsRow
                    label="Agent rules spec"
                    description="See AGENTS.md at the repo root."
                    control={<code style={{ fontSize: 11 }}>AGENTS.md</code>}
                />
                <SettingsRow
                    label="Hooks spec"
                    description="~/.agents/skills/create-hook/SKILL.md"
                    control={<code style={{ fontSize: 11 }}>SKILL.md</code>}
                />
            </div>
        );
    }

    // ── VS Code (editor) panels ──────────────────────────────────────────
    function renderVsCodePanel() {
        if (activeCategory === 'theme') {
            return (
                <div className="settings-section" style={{ maxWidth: 720 }}>
                    <div className="settings-section-title">Theme</div>
                    <SettingsRow
                        label="Workbench: Color Theme"
                        description="Specifies the color theme used in the workbench."
                        control={
                            <select value={vsSettings.theme} onChange={(e) => handleVsSettingChange('theme', e.target.value)}>
                                <option value="vs-dark">Dark (Visual Studio)</option>
                                <option value="vs">Light (Visual Studio)</option>
                                <option value="Darcula">Darcula</option>
                                <option value="Monokai">Monokai</option>
                                <option value="Solarized Dark">Solarized Dark</option>
                            </select>
                        }
                    />
                </div>
            );
        }

        if (activeCategory === 'files') {
            return (
                <div className="settings-section" style={{ maxWidth: 720 }}>
                    <div className="settings-section-title">Files</div>
                    <SettingsRow
                        label="Files: Auto Save"
                        description="Controls auto save of editors that have unsaved changes."
                        control={
                            <select
                                value={vsSettings.auto_save || 'off'}
                                onChange={(e) => handleVsSettingChange('auto_save', e.target.value)}
                            >
                                <option value="off">off</option>
                                <option value="afterDelay">afterDelay</option>
                                <option value="onFocusChange">onFocusChange</option>
                                <option value="onWindowChange">onWindowChange</option>
                            </select>
                        }
                    />
                </div>
            );
        }

        return (
            <div className="settings-section" style={{ maxWidth: 720 }}>
                <div className="settings-section-title">Editor</div>
                <SettingsRow
                    label="Editor: Font Size"
                    description="Controls the font size in pixels."
                    control={
                        <input
                            type="number"
                            value={vsSettings.font_size}
                            onChange={(e) => handleVsSettingChange('font_size', parseInt(e.target.value || '14', 10))}
                        />
                    }
                />
                <SettingsRow
                    label="Editor: Tab Size"
                    description="The number of spaces a tab is equal to."
                    control={
                        <input
                            type="number"
                            value={vsSettings.tab_size || 4}
                            onChange={(e) => handleVsSettingChange('tab_size', parseInt(e.target.value || '4', 10))}
                        />
                    }
                />
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
                        flex: '0 0 230px',
                        borderRight: '1px solid var(--vscode-panel-border)',
                        padding: '12px 0',
                        overflowY: 'auto',
                        background: 'var(--vscode-sideBar-background)',
                    }}
                >
                    {filteredCategories.map(c => (
                        <React.Fragment key={c.id}>
                            {/* Cursor groups sidebar entries under tiny captioned dividers. We
                                only emit the divider when not actively searching so the user
                                doesn't see stray section labels with one item under them. */}
                            {c.groupStart && !searchQuery.trim() && (
                                <div
                                    style={{
                                        fontSize: 10,
                                        textTransform: 'uppercase',
                                        letterSpacing: 0.5,
                                        opacity: 0.45,
                                        padding: '12px 16px 4px 16px',
                                    }}
                                >
                                    {c.groupStart}
                                </div>
                            )}
                            <div
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
                        </React.Fragment>
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
