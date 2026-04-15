import React, { useEffect, useState } from 'react';
import { invoke } from '../tauri_bridge';
import { useStore } from '../store';
import { getThemes, applyTheme } from '../theme_engine';
import type { VscodeTheme } from '../theme_engine';
import ElevenLabsVoicePicker from './ElevenLabsVoicePicker';

const AgentSettingsView: React.FC = () => {
    const ollamaUrl = useStore(state => state.ollamaUrl);
    const setOllamaUrl = useStore(state => state.setOllamaUrl);
    const ollamaStatus = useStore(state => state.ollamaStatus);
    const refreshModels = useStore(state => state.refreshAvailableModels);
    const agentModel = useStore(state => state.agentModel);
    const setAgentModel = useStore(state => state.setAgentModel);
    const availableModels = useStore(state => state.availableModels);
    const setTheme = useStore(state => state.setTheme);
    const mcpServers = useStore(state => state.mcpServers);
    const addMcpServer = useStore(state => state.addMcpServer);
    const removeMcpServer = useStore(state => state.removeMcpServer);
    const listMcpServers = useStore(state => state.listMcpServers);
    const isPullingModel = useStore(state => state.isPullingModel);
    const pullOllamaModel = useStore(state => state.pullOllamaModel);
    const avatarCharacter = useStore(state => state.avatarCharacter);
    const setAvatarCharacter = useStore(state => state.setAvatarCharacter);
    const [pullInput, setPullInput] = useState('');

    // AI Avatar Characters
    const avatarCharacters = [
        { id: 'airi', name: 'AIRI', desc: 'Primary avatar - energetic anime AI', color: '#c084fc' },
        { id: 'sage', name: 'Sage', desc: 'Mature assistant - calm & wise', color: '#60a5fa' },
        { id: 'nova', name: 'Nova', desc: 'Young & energetic - futuristic', color: '#f472b6' },
        { id: 'kawaii', name: 'Kawaii', desc: 'Cute & friendly - adorable', color: '#f472b6' },
        { id: 'sentinel', name: 'Sentinel', desc: 'Security-focused - protective', color: '#22c55e' },
        { id: 'oracle', name: 'Oracle', desc: 'Knowledge-focused - all-knowing', color: '#eab308' },
        { id: 'phantom', name: 'Phantom', desc: 'Stealthy - mysterious & quiet', color: '#a855f7' },
        { id: 'titan', name: 'Titan', desc: 'Powerful - strong & reliable', color: '#ef4444' },
    ];

    // API Key state
    const [apiKeys, setApiKeys] = useState({ anthropic: '', google: '', openai: '', groq: '', openrouter: '', elevenlabs: '' });
    const [savingKeys, setSavingKeys] = useState(false);
    const [keyStatus, setKeyStatus] = useState<Record<string, string>>({});
    const [showKeys, setShowKeys] = useState<Record<string, boolean>>({});

    const [newMcpName, setNewMcpName] = useState('');
    const [newMcpType, setNewMcpType] = useState<'command' | 'http'>('command');
    const [newMcpCommand, setNewMcpCommand] = useState('');
    const [newMcpArgs, setNewMcpArgs] = useState('');
    const [newMcpUrl, setNewMcpUrl] = useState('');
    const [isAddingMcp, setIsAddingMcp] = useState(false);

    useEffect(() => {
        listMcpServers().catch(console.error);
        // Load existing saved keys (masked)
        invoke<Record<string, string>>('get_api_keys').then(keys => {
            setApiKeys(prev => ({
                anthropic: keys.anthropic ? '••••••••' + (keys.anthropic.slice(-4)) : '',
                google: keys.google ? '••••••••' + (keys.google.slice(-4)) : '',
                openai: keys.openai ? '••••••••' + (keys.openai.slice(-4)) : '',
                groq: (keys as any).groq ? '••••••••' + ((keys as any).groq.slice(-4)) : '',
                openrouter: (keys as any).openrouter ? '••••••••' + ((keys as any).openrouter.slice(-4)) : '',
                elevenlabs: (keys as any).elevenlabs_api_key ? '••••••••' + ((keys as any).elevenlabs_api_key.slice(-4)) : '',
            }));
        }).catch(console.error);
    }, []);

    const handleSaveKeys = async () => {
        setSavingKeys(true);
        setKeyStatus({});
        try {
            // Only send non-masked (newly entered) keys
            const keysToSave: Record<string, string> = {};
            if (apiKeys.anthropic && !apiKeys.anthropic.startsWith('•')) keysToSave.anthropic = apiKeys.anthropic;
            if (apiKeys.google && !apiKeys.google.startsWith('•')) keysToSave.google = apiKeys.google;
            if (apiKeys.openai && !apiKeys.openai.startsWith('•')) keysToSave.openai = apiKeys.openai;
            if ((apiKeys as any).groq && !(apiKeys as any).groq.startsWith('•')) keysToSave.groq = (apiKeys as any).groq;
            if ((apiKeys as any).openrouter && !(apiKeys as any).openrouter.startsWith('•')) keysToSave.openrouter = (apiKeys as any).openrouter;
            if ((apiKeys as any).elevenlabs && !(apiKeys as any).elevenlabs.startsWith('•')) keysToSave.elevenlabs_api_key = (apiKeys as any).elevenlabs;

            const results = await invoke<Record<string, string>>('save_api_keys', { keys: keysToSave });
            setKeyStatus(results);
            // Refresh models after saving
            for (const provider of Object.keys(results)) {
                if (!results[provider].startsWith('Dead')) {
                    refreshModels(provider).catch(() => {});
                }
            }
        } catch (err) {
            setKeyStatus({ error: String(err) });
        } finally {
            setSavingKeys(false);
        }
    };

    return (
        <div className="agent-settings-view" style={{ padding: '12px', display: 'flex', flexDirection: 'column', gap: '20px', height: '100%', overflowY: 'auto' }}>
            <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', marginBottom: '10px', padding: '20px 0', background: 'rgba(255,255,255,0.02)', borderRadius: '12px', border: '1px solid rgba(255,255,255,0.05)' }}>
                <div style={{ position: 'relative', marginBottom: '12px' }}>
                    <div style={{
                        width: '64px', height: '64px',
                        background: 'linear-gradient(135deg, #3b82f6 0%, #2563eb 100%)',
                        borderRadius: '20px',
                        display: 'flex', alignItems: 'center', justifyContent: 'center',
                        boxShadow: '0 8px 24px rgba(37, 99, 235, 0.4)'
                    }}>
                        <i className="codicon codicon-sparkle" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '32px', color: '#fff' }}></i>
                    </div>
                    {ollamaStatus === 'running' && (
                        <div style={{
                            position: 'absolute', bottom: '-4px', right: '-4px',
                            width: '16px', height: '16px', borderRadius: '50%',
                            background: '#4ade80',
                            border: '3px solid var(--vscode-sideBar-background)',
                            boxShadow: '0 0 12px rgba(74, 222, 128, 0.6)'
                        }}></div>
                    )}
                </div>
                <div style={{ fontSize: '18px', fontWeight: 800, color: '#fff', letterSpacing: '-0.5px', display: 'flex', alignItems: 'center', gap: '8px' }}>
                    AIRI Neural Engine
                    <div style={{
                        padding: '2px 8px',
                        background: 'rgba(255,255,255,0.05)',
                        borderRadius: '100px',
                        fontSize: '9px',
                        fontWeight: 700,
                        border: '1px solid rgba(255,255,255,0.1)',
                        color: ollamaStatus === 'running' ? '#4ade80' : 'rgba(255,255,255,0.6)'
                    }}>
                        {(agentModel.split('|').pop() || '').split(':')[0].toUpperCase()}
                    </div>
                </div>
                <div style={{ fontSize: '11px', opacity: 0.5, fontWeight: 600, textTransform: 'uppercase', letterSpacing: '0.1em' }}>Core Configuration</div>
            </div>

            {/* ── AI Character Selection ── */}
            <section>
                <div style={{ fontSize: '11px', fontWeight: 600, color: 'var(--vscode-sideBarSectionHeader-foreground)', marginBottom: '12px', textTransform: 'uppercase' }}>
                    AI Character
                </div>
                <div style={{ display: 'grid', gridTemplateColumns: 'repeat(4, 1fr)', gap: '8px' }}>
                    {avatarCharacters.map((char) => (
                        <div
                            key={char.id}
                            onClick={() => setAvatarCharacter(char.id)}
                            style={{
                                padding: '10px 8px',
                                background: avatarCharacter === char.id ? `${char.color}20` : 'rgba(255,255,255,0.02)',
                                border: `1px solid ${avatarCharacter === char.id ? char.color : 'rgba(255,255,255,0.08)'}`,
                                borderRadius: '8px',
                                cursor: 'pointer',
                                textAlign: 'center',
                                transition: 'all 0.2s',
                                opacity: avatarCharacter === char.id ? 1 : 0.7,
                            }}
                        >
                            <div style={{
                                width: '32px', height: '32px',
                                background: `linear-gradient(135deg, ${char.color} 0%, ${char.color}80 100%)`,
                                borderRadius: '50%',
                                margin: '0 auto 6px',
                                display: 'flex', alignItems: 'center', justifyContent: 'center',
                                fontSize: '14px', fontWeight: 800, color: '#fff',
                            }}>
                                {char.name[0]}
                            </div>
                            <div style={{ fontSize: '10px', fontWeight: 600, color: '#fff' }}>{char.name}</div>
                            <div style={{ fontSize: '8px', opacity: 0.5, marginTop: '2px' }}>{char.desc}</div>
                        </div>
                    ))}
                </div>
                <div style={{ marginTop: '12px' }}>
                    <input
                        type="text"
                        placeholder="Or add custom character ID..."
                        onKeyDown={(e) => {
                            if (e.key === 'Enter' && e.currentTarget.value) {
                                setAvatarCharacter(e.currentTarget.value);
                                e.currentTarget.value = '';
                            }
                        }}
                        style={{
                            width: '100%',
                            background: 'var(--vscode-input-background)',
                            color: 'var(--vscode-input-foreground)',
                            border: '1px solid var(--vscode-input-border)',
                            padding: '6px 10px', fontSize: '11px', borderRadius: '4px'
                        }}
                    />
                </div>
            </section>

            <section>
                <div style={{ fontSize: '11px', fontWeight: 600, color: 'var(--vscode-sideBarSectionHeader-foreground)', marginBottom: '12px', textTransform: 'uppercase' }}>
                    Model Configuration
                </div>
                <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                    <label style={{ fontSize: '11px', opacity: 0.8 }}>Active Model</label>
                    <select
                        value={agentModel}
                        onChange={(e) => setAgentModel(e.target.value)}
                        style={{ background: 'var(--vscode-dropdown-background)', color: 'var(--vscode-dropdown-foreground)', border: '1px solid var(--vscode-dropdown-border)', padding: '4px', fontSize: '12px', cursor: 'pointer', position: 'relative', zIndex: 1 }}
                    >
                        {availableModels.map(m => (
                            <option key={`${m.provider}|${m.id}`} value={`${m.provider}|${m.id}`}>
                                {m.provider.toUpperCase()} - {m.id}
                            </option>
                        ))}
                    </select>
                </div>
            </section>

            {/* ── Cloud API Keys ── */}
            <section>
                <div style={{ fontSize: '11px', fontWeight: 600, color: 'var(--vscode-sideBarSectionHeader-foreground)', marginBottom: '12px', textTransform: 'uppercase' }}>
                    Cloud API Keys
                </div>
                <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                    {([
                        { key: 'anthropic', label: 'Anthropic (Claude)', placeholder: 'sk-ant-...' },
                        { key: 'google', label: 'Google (Gemini)', placeholder: 'AIza...' },
                        { key: 'openai', label: 'OpenAI', placeholder: 'sk-...' },
                        { key: 'groq', label: 'Groq', placeholder: 'gsk_...' },
                        { key: 'openrouter', label: 'OpenRouter', placeholder: 'sk-or-...' },
                        { key: 'elevenlabs', label: 'ElevenLabs (TTS)', placeholder: 'sk_...' },
                    ] as { key: string; label: string; placeholder: string }[]).map(({ key, label, placeholder }) => (
                        <div key={key} style={{ display: 'flex', flexDirection: 'column', gap: '4px' }}>
                            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                                <label style={{ fontSize: '11px', opacity: 0.8 }}>{label}</label>
                                {keyStatus[key] && (
                                    <span style={{
                                        fontSize: '10px', fontWeight: 600,
                                        color: keyStatus[key].startsWith('Dead') ? '#f87171' : '#4ade80'
                                    }}>
                                        {keyStatus[key].startsWith('Dead') ? 'Invalid' : 'Valid'}
                                    </span>
                                )}
                            </div>
                            <div style={{ display: 'flex', gap: '4px' }}>
                                <input
                                    type={showKeys[key] ? 'text' : 'password'}
                                    value={(apiKeys as any)[key]}
                                    onChange={e => setApiKeys(prev => ({ ...prev, [key]: e.target.value }))}
                                    placeholder={placeholder}
                                    style={{
                                        flex: 1,
                                        background: 'var(--vscode-input-background)',
                                        color: 'var(--vscode-input-foreground)',
                                        border: `1px solid ${keyStatus[key]?.startsWith('Dead') ? '#f87171' : keyStatus[key] ? '#4ade80' : 'var(--vscode-input-border)'}`,
                                        padding: '4px 8px', fontSize: '11px', borderRadius: '2px', fontFamily: 'monospace'
                                    }}
                                />
                                <button
                                    onClick={() => setShowKeys(prev => ({ ...prev, [key]: !prev[key] }))}
                                    style={{ background: 'var(--vscode-button-secondaryBackground)', color: 'var(--vscode-button-secondaryForeground)', border: 'none', padding: '4px 6px', fontSize: '11px', cursor: 'pointer', borderRadius: '2px' }}
                                    title={showKeys[key] ? 'Hide' : 'Show'}
                                >
                                    <i className={`codicon codicon-${showKeys[key] ? 'eye-closed' : 'eye'}`} style={{ fontFamily: 'codicon', fontStyle: 'normal' }}></i>
                                </button>
                            </div>
                        </div>
                    ))}
                    <button
                        onClick={handleSaveKeys}
                        disabled={savingKeys}
                        style={{
                            marginTop: '6px',
                            background: savingKeys ? 'var(--vscode-button-secondaryBackground)' : 'var(--vscode-button-background)',
                            color: 'var(--vscode-button-foreground)',
                            border: 'none', padding: '6px 12px', fontSize: '12px',
                            cursor: savingKeys ? 'wait' : 'pointer', borderRadius: '4px',
                            fontWeight: 600, display: 'flex', alignItems: 'center', gap: '6px', justifyContent: 'center'
                        }}
                    >
                        <i className="codicon codicon-save" style={{ fontFamily: 'codicon', fontStyle: 'normal' }}></i>
                        {savingKeys ? 'Validating & Saving...' : 'Save & Validate Keys'}
                    </button>
                    {keyStatus.error && (
                        <div style={{ fontSize: '11px', color: '#f87171', padding: '4px' }}>{keyStatus.error}</div>
                    )}
                </div>
            </section>

            <section>
                <div style={{ fontSize: '11px', fontWeight: 600, color: 'var(--vscode-sideBarSectionHeader-foreground)', marginBottom: '12px', textTransform: 'uppercase' }}>
                    Ollama Integration
                </div>
                <div style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
                    <div style={{ display: 'flex', flexDirection: 'column', gap: '4px' }}>
                        <label style={{ fontSize: '11px', opacity: 0.8 }}>Self-Hosted URL</label>
                        <input
                            type="text"
                            value={ollamaUrl}
                            onChange={(e) => setOllamaUrl(e.target.value)}
                            style={{ background: 'var(--vscode-input-background)', color: 'var(--vscode-input-foreground)', border: '1px solid var(--vscode-input-border)', padding: '4px 8px', fontSize: '12px' }}
                            placeholder="http://localhost:1536"
                        />
                    </div>

                    <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                        <div style={{
                            width: '8px',
                            height: '8px',
                            borderRadius: '50%',
                            background: ollamaStatus === 'running' ? '#4ade80' : ollamaStatus === 'error' ? '#f87171' : '#fbbf24'
                        }}></div>
                        <span style={{ fontSize: '12px' }}>
                            {ollamaStatus === 'running' ? 'Connected' : ollamaStatus === 'error' ? 'Error' : 'Checking...'}
                        </span>
                        <button
                            onClick={() => refreshModels('ollama')}
                            style={{ marginLeft: 'auto', background: 'var(--vscode-button-secondaryBackground)', color: 'var(--vscode-button-secondaryForeground)', border: 'none', padding: '2px 8px', fontSize: '10px', cursor: 'pointer', borderRadius: '4px' }}
                        >
                            Reconnect
                        </button>
                    </div>

                    <div style={{ display: 'flex', flexDirection: 'column', gap: '8px', marginTop: '4px', padding: '10px', background: 'var(--vscode-sideBar-background)', border: '1px solid var(--vscode-panel-border)', borderRadius: '2px' }}>
                        <label style={{ fontSize: '11px', fontWeight: 600, opacity: 0.7 }}>Pull New Model</label>
                        <div style={{ display: 'flex', gap: '8px' }}>
                            <input
                                type="text"
                                placeholder="e.g. deepseek-v3"
                                value={pullInput}
                                onChange={(e) => setPullInput(e.target.value)}
                                style={{ flex: 1, background: 'var(--vscode-input-background)', color: 'var(--vscode-input-foreground)', border: '1px solid var(--vscode-input-border)', padding: '4px 8px', fontSize: '11px' }}
                                disabled={isPullingModel}
                            />
                            <button
                                onClick={() => {
                                    if (pullInput) {
                                        pullOllamaModel(pullInput);
                                        setPullInput('');
                                    }
                                }}
                                disabled={isPullingModel || !pullInput}
                                style={{
                                    background: isPullingModel ? 'var(--vscode-button-secondaryBackground)' : 'var(--vscode-button-background)',
                                    color: 'var(--vscode-button-foreground)',
                                    border: 'none',
                                    padding: '4px 12px',
                                    fontSize: '11px',
                                    cursor: isPullingModel ? 'wait' : 'pointer',
                                    borderRadius: '2px',
                                    fontWeight: 600
                                }}
                            >
                                {isPullingModel ? 'Pulling...' : 'Pull'}
                            </button>
                        </div>
                        {isPullingModel && (
                            <div style={{ height: '2px', background: 'rgba(255,255,255,0.1)', borderRadius: '1px', marginTop: '4px', overflow: 'hidden' }}>
                                <div style={{ height: '100%', width: '100%', background: '#3b82f6', animation: 'progressIndeterminate 1.5s infinite linear' }}></div>
                            </div>
                        )}
                    </div>
                </div>
            </section>

            {/* ── Voice / TTS Settings ── */}
            <section>
                <div style={{ fontSize: '11px', fontWeight: 600, color: 'var(--vscode-sideBarSectionHeader-foreground)', marginBottom: '12px', textTransform: 'uppercase' }}>
                    Voice & TTS (AIRI Speech)
                </div>
                <div style={{ display: 'flex', flexDirection: 'column', gap: '12px', padding: '12px', background: 'rgba(192,132,252,0.05)', borderRadius: '6px', border: '1px solid rgba(192,132,252,0.15)' }}>
                    <div style={{ fontSize: '11px', opacity: 0.7, display: 'flex', alignItems: 'center', gap: '6px' }}>
                        <i className="codicon codicon-mic" style={{ fontFamily: 'codicon', fontStyle: 'normal' }}></i>
                        Enable anime-style voice synthesis for AIRI responses
                    </div>
                    <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                            <label style={{ fontSize: '11px', opacity: 0.8 }}>ElevenLabs API Key</label>
                            {keyStatus['elevenlabs'] && (
                                <span style={{ fontSize: '10px', fontWeight: 600, color: keyStatus['elevenlabs'].startsWith('Dead') ? '#f87171' : '#4ade80' }}>
                                    {keyStatus['elevenlabs'].startsWith('Dead') ? 'Invalid' : 'Valid'}
                                </span>
                            )}
                        </div>
                        <div style={{ display: 'flex', gap: '4px' }}>
                            <input
                                type={showKeys['elevenlabs'] ? 'text' : 'password'}
                                value={apiKeys.elevenlabs}
                                onChange={e => setApiKeys(prev => ({ ...prev, elevenlabs: e.target.value }))}
                                placeholder="sk_... (from elevenlabs.io)"
                                style={{
                                    flex: 1,
                                    background: 'var(--vscode-input-background)',
                                    color: 'var(--vscode-input-foreground)',
                                    border: `1px solid ${keyStatus['elevenlabs']?.startsWith('Dead') ? '#f87171' : keyStatus['elevenlabs'] ? '#4ade80' : 'var(--vscode-input-border)'}`,
                                    padding: '6px 10px', fontSize: '12px', borderRadius: '4px', fontFamily: 'monospace'
                                }}
                            />
                            <button
                                onClick={() => setShowKeys(prev => ({ ...prev, elevenlabs: !prev.elevenlabs }))}
                                style={{ background: 'var(--vscode-button-secondaryBackground)', color: 'var(--vscode-button-secondaryForeground)', border: 'none', padding: '6px 8px', fontSize: '11px', cursor: 'pointer', borderRadius: '4px' }}
                                title={showKeys['elevenlabs'] ? 'Hide' : 'Show'}
                            >
                                <i className={`codicon codicon-${showKeys['elevenlabs'] ? 'eye-closed' : 'eye'}`} style={{ fontFamily: 'codicon', fontStyle: 'normal' }}></i>
                            </button>
                        </div>
                    </div>

                    {/* ElevenLabs Voice Picker */}
                    <ElevenLabsVoicePicker
                        apiKey={apiKeys.elevenlabs}
                        onVoiceSelect={(voiceId) => {
                            // Import voice module dynamically to set selected voice
                            import('../voice').then(({ setSelectedVoice }) => {
                                setSelectedVoice(voiceId);
                                console.log('[Settings] ElevenLabs voice set:', voiceId);
                            });
                        }}
                    />

                    <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                            <label style={{ fontSize: '11px', opacity: 0.8 }}>OpenAI API Key (for TTS)</label>
                            {keyStatus['openai'] && (
                                <span style={{ fontSize: '10px', fontWeight: 600, color: keyStatus['openai'].startsWith('Dead') ? '#f87171' : '#4ade80' }}>
                                    {keyStatus['openai'].startsWith('Dead') ? 'Invalid' : 'Valid'}
                                </span>
                            )}
                        </div>
                        <input
                            type={showKeys['openai_tts'] ? 'text' : 'password'}
                            value={apiKeys.openai}
                            onChange={e => setApiKeys(prev => ({ ...prev, openai: e.target.value }))}
                            placeholder="sk-... (also used for GPT-4o TTS)"
                            style={{
                                width: '100%',
                                background: 'var(--vscode-input-background)',
                                color: 'var(--vscode-input-foreground)',
                                border: `1px solid var(--vscode-input-border)`,
                                padding: '6px 10px', fontSize: '12px', borderRadius: '4px', fontFamily: 'monospace'
                            }}
                        />
                    </div>

                    <div style={{ fontSize: '10px', opacity: 0.5, paddingTop: '8px', borderTop: '1px solid rgba(255,255,255,0.1)' }}>
                        Voice presets: AIRI (energetic), Sage (calm), Nova (young), Kawaii (cute)
                    </div>
                </div>
            </section>

            <section>
                <div style={{ fontSize: '11px', fontWeight: 600, color: 'var(--vscode-sideBarSectionHeader-foreground)', marginBottom: '12px', textTransform: 'uppercase', display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                    MCP Servers
                    <button
                        onClick={() => setIsAddingMcp(!isAddingMcp)}
                        style={{ background: 'transparent', border: 'none', color: '#3b82f6', cursor: 'pointer', fontSize: '10px' }}
                    >
                        {isAddingMcp ? 'Cancel' : '+ Add Server'}
                    </button>
                </div>

                {isAddingMcp && (
                    <div style={{ marginBottom: '16px', padding: '10px', background: 'var(--vscode-sideBar-background)', border: '1px solid var(--vscode-panel-border)', borderRadius: '2px', display: 'flex', flexDirection: 'column', gap: '8px' }}>
                        <input
                            placeholder="Server Name (e.g. filesystem)"
                            value={newMcpName}
                            onChange={e => setNewMcpName(e.target.value)}
                            style={{ background: 'var(--vscode-input-background)', color: 'var(--vscode-input-foreground)', border: '1px solid var(--vscode-input-border)', padding: '4px 8px', fontSize: '11px' }}
                        />
                        <select
                            value={newMcpType}
                            onChange={e => setNewMcpType(e.target.value as any)}
                            style={{ background: 'rgba(0,0,0,0.2)', color: '#fff', border: '1px solid rgba(255,255,255,0.1)', padding: '4px 8px', fontSize: '11px', borderRadius: '4px' }}
                        >
                            <option value="command">Stdio Command</option>
                            <option value="http">HTTP Transport</option>
                        </select>
                        {newMcpType === 'command' ? (
                            <>
                                <input
                                    placeholder="Command (e.g. npx)"
                                    value={newMcpCommand}
                                    onChange={e => setNewMcpCommand(e.target.value)}
                                    style={{ background: 'rgba(0,0,0,0.2)', color: '#fff', border: '1px solid rgba(255,255,255,0.1)', padding: '4px 8px', fontSize: '11px', borderRadius: '4px' }}
                                />
                                <input
                                    placeholder="Args (comma separated)"
                                    value={newMcpArgs}
                                    onChange={e => setNewMcpArgs(e.target.value)}
                                    style={{ background: 'rgba(0,0,0,0.2)', color: '#fff', border: '1px solid rgba(255,255,255,0.1)', padding: '4px 8px', fontSize: '11px', borderRadius: '4px' }}
                                />
                            </>
                        ) : (
                            <input
                                placeholder="URL (e.g. http://localhost:3000/sse)"
                                value={newMcpUrl}
                                onChange={e => setNewMcpUrl(e.target.value)}
                                style={{ background: 'rgba(0,0,0,0.2)', color: '#fff', border: '1px solid rgba(255,255,255,0.1)', padding: '4px 8px', fontSize: '11px', borderRadius: '4px' }}
                            />
                        )}
                        <button
                            onClick={async () => {
                                let config: any = {};
                                if (newMcpType === 'command') {
                                    config = { command: newMcpCommand, args: newMcpArgs.split(',').map(a => a.trim()).filter(Boolean) };
                                } else {
                                    config = { url: newMcpUrl };
                                }
                                await addMcpServer(newMcpName, config);
                                setIsAddingMcp(false);
                                setNewMcpName(''); setNewMcpCommand(''); setNewMcpArgs(''); setNewMcpUrl('');
                            }}
                            style={{ background: '#3b82f6', color: '#fff', border: 'none', padding: '6px', fontSize: '11px', borderRadius: '4px', cursor: 'pointer', fontWeight: 600 }}
                        >
                            Connect Server
                        </button>
                    </div>
                )}

                <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                    {mcpServers.length === 0 && !isAddingMcp && (
                        <div style={{ fontSize: '11px', opacity: 0.4, fontStyle: 'italic', textAlign: 'center', padding: '10px' }}>No MCP servers configured.</div>
                    )}
                    {mcpServers.map(server => (
                        <div key={server.name} style={{ display: 'flex', alignItems: 'center', gap: '8px', padding: '6px 10px', background: 'var(--vscode-sideBar-background)', border: '1px solid var(--vscode-panel-border)', borderRadius: '2px' }}>
                            <i className="codicon codicon-server" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '14px', color: '#89d185', opacity: 0.8 }}></i>
                            <div style={{ display: 'flex', flexDirection: 'column', flex: 1, overflow: 'hidden' }}>
                                <span style={{ fontSize: '12px', fontWeight: 600 }}>{server.name}</span>
                                <span style={{ fontSize: '10px', opacity: 0.5, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                                    {server.config.command ? `${server.config.command} ${server.config.args.join(' ')}` : server.config.url}
                                </span>
                            </div>
                            <i
                                className="codicon codicon-trash"
                                onClick={() => removeMcpServer(server.name)}
                                style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '14px', opacity: 0.4, cursor: 'pointer' }}
                                title="Remove Server"
                            ></i>
                        </div>
                    ))}
                </div>
            </section>

        </div>
    );
};

export default AgentSettingsView;
