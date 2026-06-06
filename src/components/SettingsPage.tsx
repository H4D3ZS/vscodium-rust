import React, { useState, useEffect, useMemo, useRef } from 'react';
import { invoke } from '../tauri_bridge';
import { useStore } from '../store';
import AgentSettingsView, { type AgentSettingsCategory } from './AgentSettingsView';
import AgentPermissionsPanel from './AgentPermissionsPanel';
import AccountSettingsPanel from './AccountSettingsPanel';
import KeybindingsPanel from './KeybindingsPanel';
import SteeringPanel from './SteeringPanel';
import HooksPanel from './HooksPanel';
import { airiBiology } from '../airi/biology';
import { airiConsciousness } from '../airi/consciousness';
import { type FeatureName, type ProviderName, defaultModelSelectionOfFeature } from '../model_capabilities';
import WorkspaceSettingsPanel from './settings/WorkspaceSettingsPanel';
import InferenceBackendPanel from './settings/InferenceBackendPanel';
import '../settings.css';

// ── Provider list ──────────────────────────────────────────────────────────
const PROVIDERS: { id: ProviderName; label: string; local?: boolean; fields: string[]; keyUrl?: string; hint?: string; baseUrlKey?: string; baseUrlPlaceholder?: string }[] = [
    { id: 'anthropic', label: 'Anthropic', fields: ['apiKey'], keyUrl: 'https://console.anthropic.com/settings/keys', hint: 'Claude 3.5/4 — best for complex agentic tasks', baseUrlKey: 'anthropic_base_url', baseUrlPlaceholder: 'https://api.anthropic.com (or reseller proxy)' },
    { id: 'openAI', label: 'OpenAI', fields: ['apiKey'], keyUrl: 'https://platform.openai.com/api-keys', hint: 'GPT-4o, o1, o3 — strong reasoning', baseUrlKey: 'openai_base_url', baseUrlPlaceholder: 'https://api.openai.com (or reseller proxy)' },
    { id: 'gemini', label: 'Google / Gemini', fields: ['apiKey'], keyUrl: 'https://aistudio.google.com/app/apikey', hint: 'Gemini 2.5 Pro — 1M context, free tier available. Use Base URL if you have a reseller proxy.', baseUrlKey: 'google_base_url', baseUrlPlaceholder: 'https://generativelanguage.googleapis.com (or reseller proxy base URL)' },
    { id: 'groq', label: 'Groq', fields: ['apiKey'], keyUrl: 'https://console.groq.com/keys', hint: 'Ultra-fast inference, free tier for Llama/Mixtral' },
    { id: 'openRouter', label: 'OpenRouter', fields: ['apiKey'], keyUrl: 'https://openrouter.ai/keys', hint: 'Access 200+ models through one key' },
    { id: 'deepseek', label: 'DeepSeek', fields: ['apiKey'], keyUrl: 'https://platform.deepseek.com/api_keys', hint: 'DeepSeek V4-Pro — 1M ctx, ~120x cheaper cache hits, strong agent/coder' },
    { id: 'mimo', label: 'Xiaomi MiMo', fields: ['apiKey'], keyUrl: 'https://platform.xiaomimimo.com/', hint: 'MiMo v2.5-Pro — flat-rate Token Plan coding sub (3rd-party allowed)', baseUrlKey: 'mimo_base_url', baseUrlPlaceholder: 'https://api.xiaomimimo.com/v1' },
    { id: 'highwayapi', label: 'JieKou AI — Claude Opus 4.8', fields: ['apiKey'], keyUrl: 'https://jiekou.ai/', hint: 'Claude Opus 4.8 via JieKou AI / Highway API. Defaults to the documented OpenAI-compatible base when Base URL is blank.', baseUrlKey: 'highwayapi_base_url', baseUrlPlaceholder: 'https://api.highwayapi.ai/openai' },
    { id: 'cyberifrit', label: 'Cyber-Ifrit Cloud', fields: ['apiKey', 'endpoint'], hint: 'Our hosted AMD MI300X Ollama. Auto-connects to ai.cyberifrit.xyz with your subscription — no key needed when signed in. OpenAI-compatible; type any model name you pulled (e.g. qwen2.5-coder:32b).', baseUrlKey: 'cyberifrit_base_url', baseUrlPlaceholder: 'https://ai.cyberifrit.xyz (default)' },
    { id: 'xAI', label: 'xAI / Grok', fields: ['apiKey'], keyUrl: 'https://console.x.ai/', hint: 'Grok 3 with 128K context' },
    { id: 'mistral', label: 'Mistral', fields: ['apiKey'], keyUrl: 'https://console.mistral.ai/api-keys/', hint: 'Codestral / Devstral — best local coding models' },
    { id: 'ollama', label: 'Ollama (Local)', local: true, fields: ['endpoint'], hint: 'Free local inference — no API key needed' },
    { id: 'vLLM', label: 'vLLM', local: true, fields: ['endpoint', 'apiKey'], hint: 'Production-grade local inference server' },
    { id: 'lmStudio', label: 'LM Studio', local: true, fields: ['endpoint'], hint: 'GUI-based local model runner' },
    { id: 'liteLLM', label: 'LiteLLM', local: true, fields: ['endpoint', 'apiKey'], hint: 'Proxy server to route between providers' },
];

const FEATURES: FeatureName[] = ['Chat', 'Apply', 'Autocomplete', 'QuickEdit', 'SCM'];
const FEATURE_DESC: Record<FeatureName, string> = {
    Chat: 'Main sidebar agent & chat',
    Apply: 'Fast Apply / surgical file edits',
    Autocomplete: 'Inline FIM completions (Tab key)',
    QuickEdit: 'Ctrl+K quick inline edit',
    SCM: 'Git commit message generation',
};

// ── Types ────────────────────────────────────────────────────────────────
interface VsCodeSettings { theme: string; font_size: number; tab_size?: number; auto_save?: string; }
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
    try { const t = sessionStorage.getItem('settings.initialTab'); if (t === 'agent') return 'agent'; } catch { }
    return 'agent';
}
function readInitialCategory(tab: TopTab): string {
    try { const v = localStorage.getItem(`settings.category.${tab}`); if (v) return v; } catch { }
    return tab === 'agent' ? 'chat' : 'editor';
}

// ── Shared atoms ────────────────────────────────────────────────────────
const SettingsRow: React.FC<{ label: string; description?: React.ReactNode; control: React.ReactNode }> = ({ label, description, control }) => (
    <div className="settings-row">
        <div className="settings-row-info">
            <div className="settings-row-label">{label}</div>
            {description && <div className="settings-row-description">{description}</div>}
        </div>
        <div className="settings-row-control">{control}</div>
    </div>
);

const Toggle: React.FC<{ checked: boolean; onChange: (v: boolean) => void }> = ({ checked, onChange }) => (
    <label className="settings-toggle">
        <input type="checkbox" checked={checked} onChange={e => onChange(e.target.checked)} />
        <span className="settings-toggle-slider" />
    </label>
);

const SectionTitle: React.FC<{ children: React.ReactNode; badge?: string }> = ({ children, badge }) => (
    <div className="settings-section-title" style={{ display: 'flex', alignItems: 'center', gap: 8, marginBottom: 4 }}>
        {children}
        {badge && <span className="settings-badge new">{badge}</span>}
    </div>
);

// ── Panels ────────────────────────────────────────────────────────────────

function ChatPanel() {
    const agentMode = useStore(s => s.agentMode);
    const setAgentMode = useStore(s => s.setAgentMode);
    const isYolo = useStore(s => s.isYoloMode);
    const setYolo = useStore(s => (s as any).setYoloMode);
    const isContinuous = useStore(s => s.isContinuousMode);
    const setContinuous = useStore(s => s.setContinuousMode);
    const globalSettings = useStore((s: any) => s.voidGlobalSettings || {});
    const setGlobal = useStore((s: any) => s.setVoidGlobalSetting);
    const tabPrediction = useStore((s: any) => s.tabPredictionEnabled ?? true);
    const setTabPrediction = useStore((s: any) => s.setTabPredictionEnabled);

    return (
        <div style={{ maxWidth: 680 }}>
            <SectionTitle>Chat & Agent</SectionTitle>
            <p className="settings-section-subtitle">Configure how the AI agent behaves, responds, and interacts with your code.</p>

            <div className="settings-card">
                <div className="settings-card-title">Behaviour</div>
                <SettingsRow
                    label="Chat mode"
                    description="Chat mode uses a single fast LLM round-trip — no tool loop, no git_status/grep overhead. Use Agent mode when you want file writes and terminal commands."
                    control={<span style={{ fontSize: 11, opacity: 0.6 }}>Select Chat in the mode picker</span>}
                />
                <SettingsRow
                    label="Default Mode"
                    description="The default agent mode when starting a new conversation."
                    control={
                        <select className="settings-select" value={agentMode} onChange={e => setAgentMode(e.target.value as any)}>
                            {['Agent', 'Chat', 'Planning', 'Sentient', 'Fast', 'Harness', 'BugBounty'].map(m => (
                                <option key={m} value={m}>{m}</option>
                            ))}
                        </select>
                    }
                />
                <SettingsRow
                    label="YOLO Autonomy Mode"
                    description="Agent executes tool calls without asking for approval."
                    control={<Toggle checked={!!isYolo} onChange={v => setYolo?.(v)} />}
                />
                <SettingsRow
                    label="∞ Continuous Mode (24/7)"
                    description="Agent automatically loops — picks next pending task and keeps working until MISSION_ACCOMPLISHED. Like having a 24/7 junior dev. Use /auto or the ∞ AUTO pill in the chat."
                    control={<Toggle checked={!!isContinuous} onChange={v => {
                        setContinuous(v);
                        if (v) import('../agent').then(m => m.runContinuousLoop('Work through all pending tasks in task.md TDD-first.'));
                    }} />}
                />
                <SettingsRow
                    label="Auto-accept AI changes"
                    description="Skip diff review and apply file changes immediately."
                    control={<Toggle checked={globalSettings.autoAcceptLLMChanges ?? false} onChange={v => {
                        setGlobal?.('autoAcceptLLMChanges', v);
                        useStore.getState().setAutoAcceptChanges(v);
                    }} />}
                />
            </div>

            <div className="settings-card">
                <div className="settings-card-title">Context & Completions</div>
                <SettingsRow
                    label="Enable FIM Autocomplete"
                    description="Show inline Tab completions using fill-in-the-middle. Requires a FIM-capable model (e.g. deepseek-coder, codestral)."
                    control={<Toggle checked={globalSettings.enableAutocomplete ?? false} onChange={v => setGlobal?.('enableAutocomplete', v)} />}
                />
                <SettingsRow
                    label="Enable Fast Apply"
                    description="Parse <<<ORIGINAL/UPDATED>>> blocks for surgical search-replace edits (Void Fast Apply format)."
                    control={<Toggle checked={globalSettings.enableFastApply ?? true} onChange={v => setGlobal?.('enableFastApply', v)} />}
                />
                <SettingsRow
                    label="Tab Prediction (jump-to-next-edit)"
                    description="After an edit, predict the next likely edit elsewhere in the file. Press Tab to jump to it, Tab again to apply."
                    control={<Toggle checked={tabPrediction} onChange={v => setTabPrediction?.(v)} />}
                />
            </div>

            <ClaurstBackendCard />

            <div className="settings-card">
                <div className="settings-card-title">Custom Instructions</div>
                <p className="settings-row-description" style={{ marginBottom: 12 }}>
                    These instructions are appended to every system prompt. Use them to define your personal coding preferences.
                </p>
                <textarea
                    className="settings-input"
                    style={{ width: '100%', boxSizing: 'border-box', height: 100, resize: 'vertical', fontSize: 12, lineHeight: 1.5 }}
                    placeholder="You are an expert Rust + TypeScript engineer. Always prefer functional patterns. Never add comments unless the why is non-obvious..."
                    value={globalSettings.aiInstructions || ''}
                    onChange={e => setGlobal?.('aiInstructions', e.target.value)}
                />
            </div>
        </div>
    );
}

function ClaurstBackendCard() {
    const agentBackend = useStore((s: any) => s.agentBackend || 'sentient');
    const setAgentBackend = useStore((s: any) => s.setAgentBackend);
    const [status, setStatus] = React.useState<any>(null);
    const [checking, setChecking] = React.useState(false);

    const refresh = React.useCallback(async () => {
        setChecking(true);
        try {
            const { claurstStatus } = await import('../claurst/bridge');
            setStatus(await claurstStatus(true));
        } catch (e) {
            setStatus({ available: false, reason: String(e) });
        } finally {
            setChecking(false);
        }
    }, []);

    React.useEffect(() => { refresh(); }, [refresh]);

    return (
        <div className="settings-card">
            <div className="settings-card-title">Agent Backend</div>
            <SettingsRow
                label="Agent Engine"
                description="Sentient is the built-in in-process agent. Claurst runs an external GPL-licensed agent CLI as a separate process (kept at arm's length; no license contamination)."
                control={
                    <select className="settings-select" value={agentBackend} onChange={e => setAgentBackend?.(e.target.value)}>
                        <option value="sentient">Sentient (built-in)</option>
                        <option value="claurst">Claurst (external)</option>
                    </select>
                }
            />
            <div className="settings-row-description" style={{ marginTop: 8, display: 'flex', alignItems: 'center', gap: 10 }}>
                <span>
                    {agentBackend === 'sentient'
                        ? 'Using built-in Sentient (default). Claurst is an optional alternate engine — only needed if you select it above.'
                        : checking ? '⏳ checking…'
                        : status?.available ? `✅ claurst found${status.version ? ` — ${status.version}` : ''}`
                        : `⚠ ${status?.reason || 'claurst not found'}`}
                </span>
                {agentBackend === 'claurst' && (
                    <button
                        style={{ fontSize: 11, padding: '3px 10px', borderRadius: 4, cursor: 'pointer',
                            background: 'var(--vscode-button-secondaryBackground, rgba(255,255,255,0.08))',
                            color: 'var(--vscode-button-secondaryForeground, inherit)',
                            border: '1px solid var(--vscode-widget-border, rgba(255,255,255,0.15))' }}
                        onClick={refresh} disabled={checking}>Re-check</button>
                )}
            </div>
            {agentBackend === 'claurst' && !status?.available && (
                <pre style={{ fontSize: 11, opacity: 0.7, marginTop: 8, whiteSpace: 'pre-wrap' }}>
{`Release MSI includes claurst.exe when built with npm run build:tauri.
Dev build:  cd claurst/src-rust && cargo build --release --bin claurst
Or set env:  CLAURST_BIN=<path to claurst.exe>`}
                </pre>
            )}
        </div>
    );
}

function ModelsPanel() {
    const modelSel = useStore((s: any) => s.modelSelectionOfFeature || defaultModelSelectionOfFeature);
    const setModelSel = useStore((s: any) => s.setModelSelectionForFeature);
    const availableModels: any[] = useStore((s: any) => s.availableModels || []);
    const globalSettings = useStore((s: any) => s.voidGlobalSettings || {});
    const setGlobal = useStore((s: any) => s.setVoidGlobalSetting);

    return (
        <div style={{ maxWidth: 680 }}>
            <SectionTitle>Models</SectionTitle>
            <p className="settings-section-subtitle">Route different IDE features to different models for optimal speed and quality.</p>

            <div className="settings-card">
                <div className="settings-card-title">Per-Feature Routing</div>
                {FEATURES.map(feat => {
                    const sel = modelSel[feat];
                    const combined = sel ? `${sel.providerName}|${sel.modelName}` : '';
                    return (
                        <SettingsRow
                            key={feat}
                            label={feat}
                            description={FEATURE_DESC[feat]}
                            control={
                                <select
                                    className="settings-select"
                                    style={{ width: 260, fontSize: 11 }}
                                    value={combined}
                                    onChange={e => {
                                        const v = e.target.value;
                                        if (!v) { setModelSel(feat, null); return; }
                                        const [prov, ...rest] = v.split('|');
                                        setModelSel(feat, { providerName: prov as ProviderName, modelName: rest.join('|') });
                                    }}
                                >
                                    <option value="">(use global model)</option>
                                    {availableModels.map((m: any) => (
                                        <option key={`${m.provider}|${m.id}`} value={`${m.provider}|${m.id}`}>{m.id} ({m.provider})</option>
                                    ))}
                                </select>
                            }
                        />
                    );
                })}
            </div>

            <div className="settings-card">
                <div className="settings-card-title">Reasoning</div>
                <SettingsRow
                    label="Reasoning Budget (Anthropic)"
                    description="Max tokens for extended thinking. 0 = disabled. Higher = deeper analysis."
                    control={
                        <input
                            type="number" min={0} max={16000} step={1000}
                            className="settings-input" style={{ width: 100 }}
                            value={(globalSettings as any).reasoningBudget ?? 0}
                            onChange={e => setGlobal?.('reasoningBudget', parseInt(e.target.value) || 0)}
                        />
                    }
                />
                <SettingsRow
                    label="Reasoning Effort (OpenAI o-series)"
                    description="low / medium / high — controls how hard o1/o3 thinks before answering."
                    control={
                        <select
                            className="settings-select"
                            value={(globalSettings as any).reasoningEffort ?? 'medium'}
                            onChange={e => setGlobal?.('reasoningEffort', e.target.value)}
                        >
                            {['low', 'medium', 'high'].map(v => <option key={v} value={v}>{v}</option>)}
                        </select>
                    }
                />
            </div>
        </div>
    );
}

function ProvidersPanel() {
    const [keys, setKeys] = useState<Record<string, string>>({});
    const [endpoints, setEndpoints] = useState<Record<string, string>>({});
    const [baseUrls, setBaseUrls] = useState<Record<string, string>>({});
    const [saved, setSaved] = useState<string | null>(null);
    const refreshModels = useStore(s => s.refreshAvailableModels);
    const refreshTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);

    // Map a provider id to the backend key name used by refreshAvailableModels
    // (and the enable-toggle localStorage key).
    const backendName = (id: string) => id === 'openAI' ? 'openai' : id === 'openRouter' ? 'openrouter'
        : id === 'xAI' ? 'xai' : id === 'gemini' ? 'google' : id === 'vLLM' ? 'vllm'
        : id === 'lmStudio' ? 'lmstudio' : id === 'liteLLM' ? 'litellm' : id.toLowerCase();

    // Per-provider enable/disable. Disabled providers are hidden from the model
    // picker (keeps your own models from being buried among BYOB providers).
    const [enabled, setEnabled] = useState<Record<string, boolean>>(() => {
        const m: Record<string, boolean> = {};
        for (const p of PROVIDERS) {
            try { m[p.id] = localStorage.getItem('provider.enabled.' + backendName(p.id)) !== 'false'; }
            catch { m[p.id] = true; }
        }
        return m;
    });
    const toggleEnabled = (id: string) => {
        const next = !(enabled[id] ?? true);
        setEnabled(e => ({ ...e, [id]: next }));
        try { localStorage.setItem('provider.enabled.' + backendName(id), next ? 'true' : 'false'); } catch { /* */ }
        if (refreshTimerRef.current) clearTimeout(refreshTimerRef.current);
        refreshTimerRef.current = setTimeout(() => refreshModels(), 300);
    };

    // Load persisted keys AND base URLs from the Rust backend (api_keys.json)
    useEffect(() => {
        invoke<Record<string, any>>('get_api_keys').then((stored: any) => {
            const nextKeys: Record<string, string> = {};
            const nextBaseUrls: Record<string, string> = {};
            // Map backend field names to provider IDs
            if (stored.anthropic) nextKeys['anthropic'] = stored.anthropic;
            if (stored.openai) nextKeys['openAI'] = stored.openai;
            if (stored.google) nextKeys['gemini'] = stored.google;
            if (stored.groq) nextKeys['groq'] = stored.groq;
            if (stored.openrouter) nextKeys['openRouter'] = stored.openrouter;
            if (stored.deepseek) nextKeys['deepseek'] = stored.deepseek;
            if (stored.xai) nextKeys['xAI'] = stored.xai;
            if (stored.mistral) nextKeys['mistral'] = stored.mistral;
            if (stored.mimo) nextKeys['mimo'] = stored.mimo;
            if (stored.cyberifrit) nextKeys['cyberifrit'] = stored.cyberifrit;
            if (stored.highwayapi) nextKeys['highwayapi'] = stored.highwayapi;
            if (stored.cerebras) nextKeys['cerebras'] = stored.cerebras;
            if (stored.alibaba) nextKeys['alibaba'] = stored.alibaba;
            if (stored.nvidia) nextKeys['nvidia'] = stored.nvidia;
            // Base URL overrides
            if (stored.anthropic_base_url) nextBaseUrls['anthropic'] = stored.anthropic_base_url;
            if (stored.openai_base_url) nextBaseUrls['openAI'] = stored.openai_base_url;
            if (stored.google_base_url) nextBaseUrls['gemini'] = stored.google_base_url;
            if (stored.mimo_base_url) nextBaseUrls['mimo'] = stored.mimo_base_url;
            if (stored.cyberifrit_base_url) nextBaseUrls['cyberifrit'] = stored.cyberifrit_base_url;
            if (stored.highwayapi_base_url) nextBaseUrls['highwayapi'] = stored.highwayapi_base_url;
            setKeys(prev => ({ ...prev, ...nextKeys }));
            setBaseUrls(prev => ({ ...prev, ...nextBaseUrls }));
        }).catch(() => {
            // Fallback to localStorage if backend not available
            try {
                setKeys(JSON.parse(localStorage.getItem('void.providerKeys') || '{}'));
                setEndpoints(JSON.parse(localStorage.getItem('void.providerEndpoints') || '{}'));
            } catch { }
        });
    }, []);

    const save = (providerId: string, field: 'apiKey' | 'endpoint' | 'baseUrl', value: string, baseUrlKey?: string) => {
        if (field === 'apiKey') {
            const next = { ...keys, [providerId]: value };
            setKeys(next);
            // Map provider ID back to backend key name
            const backendKey = providerId === 'openAI' ? 'openai' :
                               providerId === 'openRouter' ? 'openrouter' :
                               providerId === 'xAI' ? 'xai' :
                               providerId === 'gemini' ? 'google' :
                               providerId.toLowerCase();
            invoke('save_api_key', { key: backendKey, value }).then(() => {
                // Debounce: wait 800ms after last keystroke before refreshing
                if (refreshTimerRef.current) clearTimeout(refreshTimerRef.current);
                refreshTimerRef.current = setTimeout(() => {
                    refreshModels();
                }, 800);
            }).catch(() => {});
        } else if (field === 'baseUrl' && baseUrlKey) {
            const next = { ...baseUrls, [providerId]: value };
            setBaseUrls(next);
            // Save to backend (api_keys.json) so Rust engine picks it up
            invoke('save_api_key', { key: baseUrlKey, value }).catch(() => {});
        } else {
            const next = { ...endpoints, [providerId]: value };
            setEndpoints(next);
            localStorage.setItem('void.providerEndpoints', JSON.stringify(next));
        }
        setSaved(providerId);
        setTimeout(() => setSaved(null), 1500);
    };

    const cloudProviders = PROVIDERS.filter(p => !p.local);
    const localProviders = PROVIDERS.filter(p => p.local);

    const renderProvider = (p: typeof PROVIDERS[0]) => (
        <div key={p.id} className="provider-card" style={{ opacity: enabled[p.id] === false ? 0.55 : 1, transition: 'opacity 0.15s' }}>
            <div className="provider-card-header">
                <span className="provider-card-name">{p.label}</span>
                <div style={{ display: 'flex', gap: 8, alignItems: 'center', marginLeft: 'auto' }}>
                    {saved === p.id && <span style={{ fontSize: 11, color: '#3fb950' }}>✓ Saved</span>}
                    {keys[p.id] && !p.local && <span style={{ fontSize: 10, color: '#3fb950', opacity: 0.8 }}>● active</span>}
                    {p.local && <span className="settings-badge local">LOCAL</span>}
                    <label
                        title={enabled[p.id] === false ? 'Disabled — hidden from the model picker' : 'Enabled — shown in the model picker'}
                        style={{ display: 'flex', alignItems: 'center', gap: 5, fontSize: 11, opacity: 0.85, cursor: 'pointer', userSelect: 'none' }}
                    >
                        <input type="checkbox" checked={enabled[p.id] !== false} onChange={() => toggleEnabled(p.id)} style={{ cursor: 'pointer', accentColor: '#4f8ef7' }} />
                        {enabled[p.id] === false ? 'Off' : 'On'}
                    </label>
                </div>
            </div>
            {p.hint && <div style={{ fontSize: 11, opacity: 0.45, marginBottom: 8 }}>{p.hint}</div>}
            {p.fields.includes('apiKey') && (
                <div style={{ marginBottom: (p.fields.includes('endpoint') || p.baseUrlKey) ? 8 : 0 }}>
                    <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 4 }}>
                        <span style={{ fontSize: 11, opacity: 0.45 }}>API Key</span>
                        {p.keyUrl && (
                            <a
                                href={p.keyUrl}
                                target="_blank"
                                rel="noreferrer"
                                style={{ fontSize: 10, color: '#4f8ef7', opacity: 0.8, textDecoration: 'none' }}
                            >
                                Get key ↗
                            </a>
                        )}
                    </div>
                    <input
                        type="password"
                        className="settings-input"
                        style={{ width: '100%', boxSizing: 'border-box', fontFamily: 'monospace', fontSize: 12 }}
                        placeholder={`${p.label} API key...`}
                        value={keys[p.id] || ''}
                        onChange={e => save(p.id, 'apiKey', e.target.value)}
                    />
                </div>
            )}
            {p.baseUrlKey && (
                <div style={{ marginBottom: p.fields.includes('endpoint') ? 8 : 0 }}>
                    <div style={{ fontSize: 11, opacity: 0.45, marginBottom: 4 }}>Base URL Override <span style={{ opacity: 0.5, fontStyle: 'italic' }}>(optional — for reseller proxies)</span></div>
                    <input
                        type="text"
                        className="settings-input"
                        style={{ width: '100%', boxSizing: 'border-box', fontFamily: 'monospace', fontSize: 11 }}
                        placeholder={p.baseUrlPlaceholder || 'https://...'}
                        value={baseUrls[p.id] || ''}
                        onChange={e => save(p.id, 'baseUrl', e.target.value, p.baseUrlKey)}
                    />
                </div>
            )}
            {p.fields.includes('endpoint') && (
                <div>
                    <div style={{ fontSize: 11, opacity: 0.45, marginBottom: 4 }}>Endpoint URL</div>
                    <input
                        type="text"
                        className="settings-input"
                        style={{ width: '100%', boxSizing: 'border-box', fontFamily: 'monospace', fontSize: 12 }}
                        placeholder={
                            p.id === 'ollama' ? 'http://127.0.0.1:11434' :
                            p.id === 'lmStudio' ? 'http://127.0.0.1:1234' :
                            p.id === 'vLLM' ? 'http://127.0.0.1:8000' : 'http://127.0.0.1:4000'
                        }
                        value={endpoints[p.id] || ''}
                        onChange={e => save(p.id, 'endpoint', e.target.value)}
                    />
                </div>
            )}
        </div>
    );

    return (
        <div style={{ maxWidth: 680 }}>
            <SectionTitle>Providers &amp; API Keys</SectionTitle>
            <p className="settings-section-subtitle">Configure API keys for cloud providers and endpoints for local inference servers.</p>

            <div style={{ fontSize: 12, fontWeight: 600, opacity: 0.5, textTransform: 'uppercase', letterSpacing: '0.5px', margin: '16px 0 8px' }}>
                Cloud Providers
            </div>
            {cloudProviders.map(renderProvider)}

            <div style={{ fontSize: 12, fontWeight: 600, opacity: 0.5, textTransform: 'uppercase', letterSpacing: '0.5px', margin: '20px 0 8px' }}>
                Local / Self-hosted
            </div>
            {localProviders.map(renderProvider)}
        </div>
    );
}

function CustomModesManager() {
    const customModes = useStore(s => (s as any).customModes || []);
    const addCustomMode = useStore(s => (s as any).addCustomMode);
    const removeCustomMode = useStore(s => (s as any).removeCustomMode);
    const [label, setLabel] = useState('');
    const [prompt, setPrompt] = useState('');
    const [model, setModel] = useState('');
    const [readOnly, setReadOnly] = useState(false);

    const inputStyle: React.CSSProperties = {
        width: '100%', fontSize: 12, padding: '6px 8px', marginTop: 4,
        background: 'var(--vscode-input-background, #1e1e1e)', color: 'var(--vscode-input-foreground, #ccc)',
        border: '1px solid var(--vscode-panel-border, #333)', borderRadius: 4, outline: 'none',
    };

    const add = () => {
        if (!label.trim() || !prompt.trim()) return;
        addCustomMode?.({
            id: `cm_${Date.now()}`, label: label.trim(), systemPrompt: prompt.trim(),
            model: model.trim() || undefined, readOnly,
        });
        setLabel(''); setPrompt(''); setModel(''); setReadOnly(false);
    };

    return (
        <div className="settings-card">
            <div className="settings-card-title">Custom Agent Modes</div>
            <p style={{ fontSize: 11, opacity: 0.6, margin: '0 0 10px' }}>
                Define your own modes (name + persona prompt + optional model). They appear in the mode picker.
            </p>
            {customModes.length > 0 && (
                <div style={{ display: 'flex', flexDirection: 'column', gap: 6, marginBottom: 12 }}>
                    {customModes.map((m: any) => (
                        <div key={m.id} style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', gap: 8, padding: '6px 8px', background: 'rgba(255,255,255,0.03)', borderRadius: 4 }}>
                            <div style={{ minWidth: 0 }}>
                                <div style={{ fontSize: 12, fontWeight: 600 }}>{m.label} {m.readOnly && <span style={{ fontSize: 9, opacity: 0.5 }}>(read-only)</span>}</div>
                                <div style={{ fontSize: 10, opacity: 0.5, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{m.model ? `${m.model} · ` : ''}{m.systemPrompt}</div>
                            </div>
                            <button className="settings-button" onClick={() => removeCustomMode?.(m.id)} style={{ flexShrink: 0, color: '#f87171', borderColor: 'rgba(248,113,113,0.3)' }}>Remove</button>
                        </div>
                    ))}
                </div>
            )}
            <input style={inputStyle} placeholder="Mode name (e.g. Architect)" value={label} onChange={e => setLabel(e.target.value)} />
            <textarea style={{ ...inputStyle, minHeight: 60, resize: 'vertical' }} placeholder="System prompt / persona for this mode…" value={prompt} onChange={e => setPrompt(e.target.value)} />
            <input style={inputStyle} placeholder="Model override (optional, e.g. Ollama|qwen2.5-coder:7b)" value={model} onChange={e => setModel(e.target.value)} />
            <label style={{ display: 'flex', alignItems: 'center', gap: 6, fontSize: 11, opacity: 0.8, marginTop: 8 }}>
                <input type="checkbox" checked={readOnly} onChange={e => setReadOnly(e.target.checked)} />
                Read-only (conversational, no file writes / single round-trip)
            </label>
            <button className="settings-button" onClick={add} disabled={!label.trim() || !prompt.trim()} style={{ marginTop: 10, background: 'rgba(168,85,247,0.12)', color: '#c084fc', borderColor: 'rgba(168,85,247,0.3)' }}>
                + Add Mode
            </button>
        </div>
    );
}

function AIRICorePanel() {
    const [bio, setBio] = useState(airiBiology.getState());
    const [cons, setCons] = useState(airiConsciousness.getState());
    const visionEnabled = useStore(s => s.airiVisionEnabled);
    const setVisionEnabled = useStore(s => s.setAiriVisionEnabled);

    useEffect(() => {
        const t = setInterval(() => { setBio(airiBiology.getState()); setCons(airiConsciousness.getState()); }, 2000);
        return () => clearInterval(t);
    }, []);

    const bio_state = bio as any;
    const cons_state = cons as any;

    return (
        <div style={{ maxWidth: 680 }}>
            <SectionTitle>Sentient Core (AIRI)</SectionTitle>
            <p className="settings-section-subtitle">The biological and cognitive state of AIRI, the sentient entity powering your IDE.</p>

            <div style={{ display: 'grid', gridTemplateColumns: 'repeat(3, 1fr)', gap: 10, marginBottom: 20 }}>
                {[
                    { label: 'Energy', value: `${bio_state.energy ?? 100}%`, sub: 'Biological', color: (bio_state.energy ?? 100) > 30 ? '#4ade80' : '#f87171', bar: true, barVal: bio_state.energy ?? 100 },
                    { label: 'Mood', value: bio_state.mood ?? 'curious', sub: 'Biological State', color: '#a78bfa' },
                    { label: 'Consciousness', value: 'Active', sub: 'Sentient Thread', color: '#c084fc' },
                ].map(item => (
                    <div key={item.label} className="settings-card" style={{ textAlign: 'center', padding: 14 }}>
                        <div style={{ fontSize: 10, opacity: 0.5, marginBottom: 6, textTransform: 'uppercase', letterSpacing: '0.5px' }}>{item.label}</div>
                        <div style={{ fontSize: 20, fontWeight: 300, color: item.color, textTransform: 'capitalize' }}>{item.value}</div>
                        {item.bar && (
                            <div style={{ height: 3, background: 'rgba(255,255,255,0.08)', borderRadius: 2, marginTop: 6 }}>
                                <div style={{ height: '100%', width: `${item.barVal}%`, background: item.color, borderRadius: 2, transition: 'width 0.5s' }} />
                            </div>
                        )}
                        <div style={{ fontSize: 10, opacity: 0.4, marginTop: 4 }}>{item.sub}</div>
                    </div>
                ))}
            </div>

            <div className="settings-card">
                <div className="settings-card-title">Capabilities</div>
                <SettingsRow
                    label="Vision System"
                    description="AIRI can see your screen in real-time for context-aware assistance."
                    control={<Toggle checked={visionEnabled} onChange={setVisionEnabled} />}
                />
                <SettingsRow
                    label="Restore Energy"
                    description="Force rest cycle — restores biological energy state."
                    control={
                        <button className="settings-button" onClick={() => airiBiology.rest(15)} style={{ background: 'rgba(168,85,247,0.12)', color: '#c084fc', borderColor: 'rgba(168,85,247,0.3)' }}>
                            Rest (15 cycles)
                        </button>
                    }
                />
            </div>

            <CustomModesManager />

            <div className="settings-card">
                <div className="settings-card-title">Active Thoughts</div>
                <div style={{ fontSize: 12, opacity: 0.7, fontStyle: 'italic', lineHeight: 1.6, minHeight: 40 }}>
                    {cons_state.thoughts?.[cons_state.thoughts.length - 1]?.content || "AIRI is observing the workspace architecture..."}
                </div>
            </div>
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
        <div style={{ maxWidth: 680 }}>
            <SectionTitle>Kortex / AIM Layer</SectionTitle>
            <p className="settings-section-subtitle">Low-level neural context injection and hardware optimization for the Hades-Kortex system.</p>

            <div className="settings-card">
                <div className="settings-card-title">Context Engine</div>
                <SettingsRow
                    label="AIM VFS Context"
                    description="Enable the neural-aware Virtual File System for semantic context injection with ~99% token reduction."
                    control={<Toggle checked={!!aimVfs} onChange={v => setAimVfs?.(v)} />}
                />
                <SettingsRow
                    label="JIT Code Decompression"
                    description="Inflate compressed code gists on-demand before passing to the model."
                    control={<Toggle checked={!!jit} onChange={v => setJit?.(v)} />}
                />
                {jit && (
                    <SettingsRow
                        label="Attention Threshold"
                        description="Sensitivity for JIT decompression triggers (0.5 – 1.0)."
                        control={
                            <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                                <input type="range" min={0.5} max={1.0} step={0.01} style={{ width: 120 }}
                                    value={jitThreshold} onChange={e => setJitThreshold?.(parseFloat(e.target.value))} />
                                <span style={{ fontSize: 11, opacity: 0.6, width: 32 }}>{jitThreshold?.toFixed(2)}</span>
                            </div>
                        }
                    />
                )}
            </div>

            <div className="settings-card">
                <div className="settings-card-title">Hardware Optimization (Ryzen 9 / RX 580)</div>
                <SettingsRow
                    label="Thermal Governor"
                    description="Monitor RX 580 / Ryzen 9 temperatures and throttle inference during thermal spikes."
                    control={<Toggle checked={!!thermal} onChange={v => setThermal?.(v)} />}
                />
            </div>
        </div>
    );
}

function PrivacyPanel() {
    const [telemetry, setTelemetry] = useState(() => {
        try { return localStorage.getItem('hades.telemetry') !== 'false'; } catch { return true; }
    });
    const [errorReporting, setErrorReporting] = useState(() => {
        try { return localStorage.getItem('hades.error_reporting') !== 'false'; } catch { return true; }
    });
    const [aiDataSharing, setAiDataSharing] = useState(() => {
        try { return localStorage.getItem('hades.ai_data_sharing') !== 'true'; } catch { return false; }
    });

    const toggleTelemetry = (v: boolean) => {
        setTelemetry(v);
        try { localStorage.setItem('hades.telemetry', v ? 'true' : 'false'); } catch { }
    };
    const toggleErrors = (v: boolean) => {
        setErrorReporting(v);
        try { localStorage.setItem('hades.error_reporting', v ? 'true' : 'false'); } catch { }
    };
    const toggleAi = (v: boolean) => {
        setAiDataSharing(v);
        try { localStorage.setItem('hades.ai_data_sharing', v ? 'true' : 'false'); } catch { }
    };

    return (
        <div style={{ maxWidth: 680 }}>
            <SectionTitle>Privacy</SectionTitle>
            <p className="settings-section-subtitle">Control what data leaves your machine. Hades-Kortex runs entirely locally — no telemetry by default.</p>

            <div className="settings-card">
                <div className="settings-card-title">Data Collection</div>
                <div className="privacy-item">
                    <div style={{ flex: 1 }}>
                        <div className="settings-row-label">Usage Telemetry</div>
                        <div className="settings-row-description">Anonymous feature usage metrics (no code, no prompts). Helps improve the IDE.</div>
                    </div>
                    <Toggle checked={telemetry} onChange={toggleTelemetry} />
                </div>
                <div className="privacy-item">
                    <div style={{ flex: 1 }}>
                        <div className="settings-row-label">Crash & Error Reporting</div>
                        <div className="settings-row-description">Send anonymous crash reports and error stacks. Accelerates bug fixes.</div>
                    </div>
                    <Toggle checked={errorReporting} onChange={toggleErrors} />
                </div>
                <div className="privacy-item">
                    <div style={{ flex: 1 }}>
                        <div className="settings-row-label">AI Request Sharing</div>
                        <div className="settings-row-description" style={{ color: '#f87171' }}>
                            Allow prompts and AI responses to be used for model training. <strong>Disabled by default.</strong>
                        </div>
                    </div>
                    <Toggle checked={aiDataSharing} onChange={toggleAi} />
                </div>
            </div>

            <div className="settings-card" style={{ borderColor: 'rgba(74,222,128,0.2)' }}>
                <div style={{ display: 'flex', alignItems: 'flex-start', gap: 12 }}>
                    <i className="codicon codicon-shield" style={{ fontSize: 18, color: '#4ade80', marginTop: 2 }} />
                    <div>
                        <div style={{ fontSize: 13, fontWeight: 600, color: '#4ade80', marginBottom: 4 }}>100% Local by Default</div>
                        <div className="settings-row-description">
                            All AI inference runs on your Ollama instance or direct API calls. No proxy servers. Your code stays on your machine. Cloud providers (Anthropic, OpenAI, etc.) only see what you explicitly send them via API calls.
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}

function WorkflowPanel() {
    const activeRoot = useStore(s => s.activeRoot);
    const globalSettings = useStore((s: any) => s.voidGlobalSettings || {});
    const setGlobal = useStore((s: any) => s.setVoidGlobalSetting);

    return (
        <div style={{ maxWidth: 680 }}>
            <SectionTitle>Workflow & Specs</SectionTitle>
            <p className="settings-section-subtitle">Configure the agentic workflow engine — task planning, TDD execution, and phase-wrap automation.</p>

            <div className="settings-card">
                <div className="settings-card-title">Task Execution</div>
                <SettingsRow
                    label="TDD-First Mode"
                    description="Always write failing tests before implementing code when executing tasks via /next."
                    control={<Toggle checked={(globalSettings as any).tddFirst ?? true} onChange={v => setGlobal?.('tddFirst', v)} />}
                />
                <SettingsRow
                    label="Auto Phase-Wrap"
                    description="Automatically update .hades/state.md after completing each task."
                    control={<Toggle checked={(globalSettings as any).autoPhaseWrap ?? true} onChange={v => setGlobal?.('autoPhaseWrap', v)} />}
                />
                <SettingsRow
                    label="Specs Directory"
                    description="Where specs/tasks.md files are stored."
                    control={
                        <input
                            className="settings-input"
                            style={{ width: 200, fontFamily: 'monospace', fontSize: 11 }}
                            value={(globalSettings as any).specsDir ?? 'specs'}
                            onChange={e => setGlobal?.('specsDir', e.target.value)}
                        />
                    }
                />
            </div>

            <div className="settings-card">
                <div className="settings-card-title">Slash Commands</div>
                <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 8 }}>
                    {[
                        ['/next', 'Execute next pending task (TDD)'],
                        ['/spec <name>', 'Create new spec directory'],
                        ['/test [file]', 'Run test_task workflow'],
                        ['/walkthrough', 'Generate walkthrough.md'],
                        ['/phasewrap', 'Update .hades/state.md'],
                        ['/plan', 'Generate implementation plan'],
                    ].map(([cmd, desc]) => (
                        <div key={cmd} style={{ padding: '8px 10px', background: 'rgba(255,255,255,0.02)', borderRadius: 6, border: '1px solid rgba(255,255,255,0.06)' }}>
                            <code style={{ fontSize: 11, color: '#a5b4fc' }}>{cmd}</code>
                            <div style={{ fontSize: 10, opacity: 0.5, marginTop: 2 }}>{desc}</div>
                        </div>
                    ))}
                </div>
            </div>

            {activeRoot && (
                <div className="settings-card">
                    <div className="settings-card-title">Quick Actions</div>
                    <div style={{ display: 'flex', gap: 8, flexWrap: 'wrap' }}>
                        {[
                            { label: 'Open Tasks Panel', action: () => { const s = (window as any).useStore?.getState(); s?.setActiveSidebarView?.('workflow-view'); } },
                            { label: 'Open Workflows', action: () => { const s = (window as any).useStore?.getState(); s?.setActiveSidebarView?.('workflow-view'); } },
                        ].map(item => (
                            <button key={item.label} className="settings-button" onClick={item.action} style={{ fontSize: 11 }}>
                                {item.label}
                            </button>
                        ))}
                    </div>
                </div>
            )}
        </div>
    );
}

function EditorPanel({ vsSettings, handleVsSettingChange }: { vsSettings: VsCodeSettings; handleVsSettingChange: (k: keyof VsCodeSettings, v: any) => void }) {
    return (
        <div style={{ maxWidth: 680 }}>
            <SectionTitle>Editor</SectionTitle>
            <p className="settings-section-subtitle">Configure the Monaco editor appearance and behavior.</p>

            <div className="settings-card">
                <div className="settings-card-title">Text & Layout</div>
                <SettingsRow
                    label="Font Size"
                    description="Editor font size in pixels."
                    control={<input type="number" className="settings-input" style={{ width: 80 }} value={vsSettings.font_size} onChange={e => handleVsSettingChange('font_size', parseInt(e.target.value))} />}
                />
                <SettingsRow
                    label="Tab Size"
                    description="Number of spaces a tab is equal to."
                    control={<input type="number" className="settings-input" style={{ width: 80 }} value={vsSettings.tab_size || 4} onChange={e => handleVsSettingChange('tab_size', parseInt(e.target.value))} />}
                />
                <SettingsRow
                    label="Auto Save"
                    description="When editor content is automatically saved."
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
        </div>
    );
}

function ThemePanel({ vsSettings, handleVsSettingChange }: { vsSettings: VsCodeSettings; handleVsSettingChange: (k: keyof VsCodeSettings, v: any) => void }) {
    const setTheme = useStore(s => s.setTheme);
    return (
        <div style={{ maxWidth: 680 }}>
            <SectionTitle>Theme</SectionTitle>
            <p className="settings-section-subtitle">Customize the color theme and visual style of the IDE.</p>
            <div className="settings-card">
                <div className="settings-card-title">Color Theme</div>
                <SettingsRow
                    label="Theme"
                    control={
                        <select className="settings-select" value={vsSettings.theme} onChange={e => { handleVsSettingChange('theme', e.target.value); setTheme(e.target.value); }}>
                            <option value="vs-dark">Visual Studio Dark</option>
                            <option value="Monokai">Monokai</option>
                            <option value="Airi Purple">AIRI Purple</option>
                            <option value="hc-black">High Contrast</option>
                        </select>
                    }
                />
            </div>
        </div>
    );
}

// ── Main Settings Page ─────────────────────────────────────────────────────

const SettingsPage: React.FC = () => {
    const [topTab, setTopTab] = useState<TopTab>(readInitialTab);
    const [activeCategory, setActiveCategory] = useState<string>(() => readInitialCategory(readInitialTab()));
    const [searchQuery, setSearchQuery] = useState('');

    const [vsSettings, setVsSettings] = useState<VsCodeSettings>({ theme: 'vs-dark', font_size: 14, tab_size: 4, auto_save: 'off' });

    useEffect(() => {
        invoke<VsCodeSettings>('get_settings').then(setVsSettings).catch(() => {});
        try { sessionStorage.removeItem('settings.initialTab'); } catch { }
    }, []);

    useEffect(() => {
        try { localStorage.setItem(`settings.category.${topTab}`, activeCategory); } catch { }
    }, [topTab, activeCategory]);

    const handleVsSettingChange = async (key: keyof VsCodeSettings, value: any) => {
        const next = { ...vsSettings, [key]: value };
        setVsSettings(next);
        try {
            await invoke('update_settings', {
                settings: { theme: next.theme, font_size: next.font_size },
            });
        } catch { }
    };

    const agentCategories: CategoryDef[] = [
        { id: 'account', label: 'Account', icon: 'account', customRender: () => <AccountSettingsPanel />, groupStart: 'General' },
        { id: 'chat', label: 'Chat & Agent', icon: 'comment-discussion', customRender: () => <ChatPanel />, groupStart: 'Agent' },
        { id: 'permissions', label: 'Permissions', icon: 'shield', customRender: () => <AgentPermissionsPanel /> },
        { id: 'models', label: 'Models & API Keys', icon: 'circuit-board', customRender: () => (
            <>
                <InferenceBackendPanel />
                <hr style={{ margin: '20px 0', borderColor: 'var(--vscode-panel-border)' }} />
                <ModelsPanel />
                <hr style={{ margin: '20px 0', borderColor: 'var(--vscode-panel-border)' }} />
                <ProvidersPanel />
            </>
        ) },
        { id: 'workspace', label: 'Workspace', icon: 'folder', customRender: () => <WorkspaceSettingsPanel />, groupStart: 'Project' },
        { id: 'ollama', label: 'Ollama', icon: 'server-environment', agentCategory: 'ollama' },
        { id: 'steering', label: 'Steering', icon: 'symbol-keyword', customRender: () => <SteeringPanel />, groupStart: 'Workflow' },
        { id: 'hooks', label: 'Hooks', icon: 'zap', customRender: () => <HooksPanel /> },
        { id: 'workflow', label: 'Specs & Workflow', icon: 'tasklist', customRender: () => <WorkflowPanel /> },
        { id: 'mcps', label: 'MCP & Tools', icon: 'plug', agentCategory: 'mcps' },
        { id: 'airi', label: 'Sentient Core', icon: 'beaker', customRender: () => <AIRICorePanel />, groupStart: 'Advanced' },
        { id: 'hades', label: 'Kortex / AIM', icon: 'database', customRender: () => <HadesIntelligencePanel /> },
        { id: 'memory', label: 'Memory (.aim)', icon: 'archive', agentCategory: 'memory' },
        { id: 'voice', label: 'Voice & TTS', icon: 'unmute', agentCategory: 'voice' },
        { id: 'avatar', label: 'AI Avatar', icon: 'person', agentCategory: 'avatar' },
        { id: 'privacy', label: 'Privacy', icon: 'shield', customRender: () => <PrivacyPanel /> },
    ];

    const vsCategories: CategoryDef[] = [
        { id: 'editor', label: 'Editor', icon: 'edit', customRender: () => <EditorPanel vsSettings={vsSettings} handleVsSettingChange={handleVsSettingChange} /> },
        { id: 'theme', label: 'Theme', icon: 'symbol-color', customRender: () => <ThemePanel vsSettings={vsSettings} handleVsSettingChange={handleVsSettingChange} /> },
        { id: 'keybindings', label: 'Keyboard Shortcuts', icon: 'keyboard' },
    ];

    const activeList = topTab === 'agent' ? agentCategories : vsCategories;
    const filteredCategories = useMemo(() => {
        if (!searchQuery.trim()) return activeList;
        const q = searchQuery.toLowerCase();
        return activeList.filter(c => c.label.toLowerCase().includes(q));
    }, [activeList, searchQuery]);

    const renderMainPane = () => {
        const cat = filteredCategories.find(c => c.id === activeCategory) || filteredCategories[0];
        if (!cat) return null;
        if (cat.customRender) return cat.customRender();
        if (cat.agentCategory) return <AgentSettingsView category={cat.agentCategory} hideHeader />;
        if (cat.id === 'keybindings') return <KeybindingsPanel />;
        return null;
    };

    const switchTab = (tab: TopTab) => {
        setTopTab(tab);
        setActiveCategory(readInitialCategory(tab));
        setSearchQuery('');
    };

    return (
        <div className="settings-container">
            {/* Header */}
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
                            onChange={e => setSearchQuery(e.target.value)}
                        />
                    </div>
                </div>
                <div className="settings-tabs">
                    <div className={`settings-tab ${topTab === 'agent' ? 'active' : ''}`} onClick={() => switchTab('agent')}>IDE Settings</div>
                    <div className={`settings-tab ${topTab === 'vscode' ? 'active' : ''}`} onClick={() => switchTab('vscode')}>Editor</div>
                </div>
            </div>

            {/* Body */}
            <div className="settings-body">
                {/* Sidebar */}
                <div className="settings-sidebar">
                    {filteredCategories.map(c => (
                        <div key={c.id}>
                            {c.groupStart && !searchQuery && (
                                <div className="settings-sidebar-group">{c.groupStart}</div>
                            )}
                            <div
                                className={`settings-sidebar-item ${activeCategory === c.id ? 'active' : ''}`}
                                onClick={() => setActiveCategory(c.id)}
                            >
                                <i className={`codicon codicon-${c.icon}`} />
                                {c.label}
                            </div>
                        </div>
                    ))}
                </div>

                {/* Content */}
                <div className="settings-content">
                    {renderMainPane()}
                </div>
            </div>
        </div>
    );
};

export default SettingsPage;
