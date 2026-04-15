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
    const avatarCustomConfig = useStore(state => state.avatarCustomConfig);
    const setAvatarCustomConfig = useStore(state => state.setAvatarCustomConfig);
    const avatar3dConfig = useStore(state => state.avatar3dConfig);
    const setAvatar3dConfig = useStore(state => state.setAvatar3dConfig);
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
    const [realApiKey, setRealApiKey] = useState(''); // Store real ElevenLabs key separately
    const [savingKeys, setSavingKeys] = useState(false);
    const [keyStatus, setKeyStatus] = useState<Record<string, string>>({});
    const [showKeys, setShowKeys] = useState<Record<string, boolean>>({});
    
    // ElevenLabs voice selection state
    const [selectedElevenLabsVoice, setSelectedElevenLabsVoice] = useState<string | undefined>(undefined);
    
    // Custom avatar configuration state
    const [customStickerUrl, setCustomStickerUrl] = useState('');
    const [customWallpaperUrl, setCustomWallpaperUrl] = useState('');
    const [isCustomAvatarEnabled, setIsCustomAvatarEnabled] = useState(false);
    
    // 3D VRM avatar configuration state
    const [vrmModelUrl, setVrmModelUrl] = useState('');
    const [vrmModelId, setVrmModelId] = useState('');
    const [customVrmModels, setCustomVrmModels] = useState<Array<{ id: string; name: string; url: string }>>([]);

    const [newMcpName, setNewMcpName] = useState('');
    const [newMcpType, setNewMcpType] = useState<'command' | 'http'>('command');
    const [newMcpCommand, setNewMcpCommand] = useState('');
    const [newMcpArgs, setNewMcpArgs] = useState('');
    const [newMcpUrl, setNewMcpUrl] = useState('');
    const [isAddingMcp, setIsAddingMcp] = useState(false);

    useEffect(() => {
        console.log('[Settings] === Loading API keys and voice configuration ===');
        
        listMcpServers().catch(console.error);
        
        // Load existing saved keys (masked) and voice ID
        invoke<Record<string, string>>('get_api_keys')
            .then(keys => {
                console.log('[Settings] ✅ API keys loaded:', Object.keys(keys));
                console.log('[Settings] Keys:', {
                    elevenlabs_api_key: (keys as any).elevenlabs_api_key ? 'present' : 'missing',
                    elevenlabs_voice_id: (keys as any).elevenlabs_voice_id || 'NOT SET'
                });
                
                setApiKeys(prev => {
                    const newKeys = {
                        anthropic: keys.anthropic ? '••••••••' + (keys.anthropic.slice(-4)) : '',
                        google: keys.google ? '••••••••' + (keys.google.slice(-4)) : '',
                        openai: keys.openai ? '••••••••' + (keys.openai.slice(-4)) : '',
                        groq: (keys as any).groq ? '••••••••' + ((keys as any).groq.slice(-4)) : '',
                        openrouter: (keys as any).openrouter ? '••••••••' + ((keys as any).openrouter.slice(-4)) : '',
                        elevenlabs: (keys as any).elevenlabs_api_key ? '••••••••' + ((keys as any).elevenlabs_api_key.slice(-4)) : '',
                    };
                    console.log('[Settings] Setting apiKeys state:', {
                        elevenlabs: newKeys.elevenlabs ? `${newKeys.elevenlabs.substring(0, 8)}...` : 'EMPTY',
                        elevenlabs_length: newKeys.elevenlabs?.length || 0
                    });
                    
                    // Store REAL key for API calls
                    if ((keys as any).elevenlabs_api_key) {
                        setRealApiKey((keys as any).elevenlabs_api_key);
                        console.log('[Settings] ✅ Real API key stored (length:', (keys as any).elevenlabs_api_key.length, ')');
                    }
                    
                    return newKeys;
                });

                // Load saved ElevenLabs voice ID
                const savedVoiceId = (keys as any).elevenlabs_voice_id;
                console.log('[Settings] 🎤 Saved voice ID:', savedVoiceId);
                
                if (savedVoiceId) {
                    setSelectedElevenLabsVoice(savedVoiceId);
                    console.log('[Settings] ✅ Voice ID set in component state:', savedVoiceId);
                    
                    // Also set it in voice.ts module
                    import('../voice').then(({ setSelectedVoice }) => {
                        setSelectedVoice(savedVoiceId);
                        console.log('[Settings] ✅ Voice ID set in voice.ts:', savedVoiceId);
                    }).catch(err => {
                        console.error('[Settings] ❌ Failed to set voice in voice.ts:', err);
                    });
                } else {
                    console.log('[Settings] ⚠️ No saved voice ID found in api_keys.json');
                }
            })
            .catch(err => {
                console.error('[Settings] ❌ Failed to load API keys:', err);
            });

        // Load custom avatar configuration
        if (avatarCustomConfig?.stickerUrl) setCustomStickerUrl(avatarCustomConfig.stickerUrl);
        if (avatarCustomConfig?.wallpaperUrl) setCustomWallpaperUrl(avatarCustomConfig.wallpaperUrl);
        if (avatarCustomConfig?.enabled) setIsCustomAvatarEnabled(true);

        // Load 3D VRM avatar configuration
        if (avatar3dConfig?.modelUrl) setVrmModelUrl(avatar3dConfig.modelUrl);
        if (avatar3dConfig?.modelId) setVrmModelId(avatar3dConfig.modelId);
        if (avatar3dConfig?.customModels) setCustomVrmModels(avatar3dConfig.customModels);
        
        console.log('[Settings] === Loading complete ===');
    }, []);
    
    // Force re-render when apiKeys are loaded (fixes input not updating)
    useEffect(() => {
        if (apiKeys.elevenlabs && apiKeys.elevenlabs.length > 0) {
            console.log('[Settings] ✅ apiKeys.elevenlabs updated, length:', apiKeys.elevenlabs.length);
        }
    }, [apiKeys.elevenlabs]);

    const handleSaveKeys = async () => {
        console.log('[Settings] === handleSaveKeys CALLED ===');
        console.log('[Settings] apiKeys.elevenlabs BEFORE save:', apiKeys.elevenlabs ? `${apiKeys.elevenlabs.substring(0, 10)}... (length: ${apiKeys.elevenlabs.length})` : 'EMPTY');
        
        setSavingKeys(true);
        setKeyStatus({});
        try {
            // Only send non-masked (newly entered) keys
            const keysToSave: Record<string, string> = {};
            if (apiKeys.anthropic && !apiKeys.anthropic.startsWith('•')) {
                keysToSave.anthropic = apiKeys.anthropic;
                console.log('[Settings] Adding anthropic key to save');
            }
            if (apiKeys.google && !apiKeys.google.startsWith('•')) {
                keysToSave.google = apiKeys.google;
                console.log('[Settings] Adding google key to save');
            }
            if (apiKeys.openai && !apiKeys.openai.startsWith('•')) {
                keysToSave.openai = apiKeys.openai;
                console.log('[Settings] Adding openai key to save');
            }
            if ((apiKeys as any).groq && !(apiKeys as any).groq.startsWith('•')) {
                keysToSave.groq = (apiKeys as any).groq;
                console.log('[Settings] Adding groq key to save');
            }
            if ((apiKeys as any).openrouter && !(apiKeys as any).openrouter.startsWith('•')) {
                keysToSave.openrouter = (apiKeys as any).openrouter;
                console.log('[Settings] Adding openrouter key to save');
            }
            if ((apiKeys as any).elevenlabs && !(apiKeys as any).elevenlabs.startsWith('•')) {
                keysToSave.elevenlabs_api_key = (apiKeys as any).elevenlabs;
                console.log('[Settings] ✅ Adding elevenlabs key to save:', (apiKeys as any).elevenlabs.substring(0, 10) + `... (length: ${(apiKeys as any).elevenlabs.length})`);
            } else {
                console.log('[Settings] ⚠️ Elevenlabs key NOT added to save:', {
                    exists: !!(apiKeys as any).elevenlabs,
                    starts_with_bullet: (apiKeys as any).elevenlabs?.startsWith('•'),
                    value: (apiKeys as any).elevenlabs
                });
            }

            console.log('[Settings] Keys to save:', Object.keys(keysToSave), 'elevenlabs_api_key in payload:', !!keysToSave.elevenlabs_api_key);
            
            const results = await invoke<Record<string, string>>('save_api_keys', { keys: keysToSave });
            console.log('[Settings] ✅ Save result:', results);
            
            setKeyStatus(results);
            // Refresh models after saving
            for (const provider of Object.keys(results)) {
                if (!results[provider].startsWith('Dead')) {
                    refreshModels(provider).catch(() => {});
                }
            }
            
            // Reload keys to confirm they were saved
            console.log('[Settings] Reloading keys to verify save...');
            const reloadedKeys = await invoke<Record<string, string>>('get_api_keys');
            console.log('[Settings] Reloaded keys:', {
                elevenlabs_api_key: (reloadedKeys as any).elevenlabs_api_key ? `present (length: ${(reloadedKeys as any).elevenlabs_api_key.length})` : 'missing',
                elevenlabs_voice_id: (reloadedKeys as any).elevenlabs_voice_id || 'NOT SET'
            });
        } catch (err) {
            console.error('[Settings] ❌ Save error:', err);
            setKeyStatus({ error: String(err) });
        } finally {
            setSavingKeys(false);
        }
        console.log('[Settings] === handleSaveKeys COMPLETE ===');
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
                
                {/* Custom Avatar Configuration */}
                <div style={{
                    marginTop: '16px',
                    padding: '12px',
                    background: 'rgba(255,255,255,0.02)',
                    borderRadius: '8px',
                    border: '1px solid rgba(255,255,255,0.08)',
                }}>
                    <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '10px' }}>
                        <div style={{ fontSize: '10px', fontWeight: 600, color: '#fff', textTransform: 'uppercase' }}>
                            <i className="codicon codicon-image" style={{ marginRight: '4px', fontFamily: 'codicon', fontStyle: 'normal' }}></i>
                            Custom Avatar URLs
                        </div>
                        <label style={{ display: 'flex', alignItems: 'center', gap: '6px', fontSize: '10px', cursor: 'pointer' }}>
                            <input
                                type="checkbox"
                                checked={isCustomAvatarEnabled}
                                onChange={(e) => {
                                    setIsCustomAvatarEnabled(e.target.checked);
                                    setAvatarCustomConfig({ enabled: e.target.checked });
                                }}
                                style={{ accentColor: 'var(--vscode-button-background)' }}
                            />
                            Enable Custom
                        </label>
                    </div>
                    
                    <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                        <div>
                            <label style={{ fontSize: '9px', opacity: 0.7, display: 'block', marginBottom: '4px' }}>
                                Sticker URL (avatar overlay)
                            </label>
                            <input
                                type="text"
                                value={customStickerUrl}
                                onChange={(e) => setCustomStickerUrl(e.target.value)}
                                placeholder="https://example.com/avatar.png"
                                disabled={!isCustomAvatarEnabled}
                                style={{
                                    width: '100%',
                                    background: 'var(--vscode-input-background)',
                                    color: 'var(--vscode-input-foreground)',
                                    border: '1px solid var(--vscode-input-border)',
                                    padding: '6px 10px',
                                    fontSize: '11px',
                                    borderRadius: '4px',
                                    opacity: isCustomAvatarEnabled ? 1 : 0.5,
                                }}
                            />
                        </div>
                        
                        <div>
                            <label style={{ fontSize: '9px', opacity: 0.7, display: 'block', marginBottom: '4px' }}>
                                Wallpaper URL (background)
                            </label>
                            <input
                                type="text"
                                value={customWallpaperUrl}
                                onChange={(e) => setCustomWallpaperUrl(e.target.value)}
                                placeholder="https://example.com/background.png"
                                disabled={!isCustomAvatarEnabled}
                                style={{
                                    width: '100%',
                                    background: 'var(--vscode-input-background)',
                                    color: 'var(--vscode-input-foreground)',
                                    border: '1px solid var(--vscode-input-border)',
                                    padding: '6px 10px',
                                    fontSize: '11px',
                                    borderRadius: '4px',
                                    opacity: isCustomAvatarEnabled ? 1 : 0.5,
                                }}
                            />
                        </div>
                        
                        <button
                            onClick={() => {
                                setAvatarCustomConfig({
                                    stickerUrl: customStickerUrl,
                                    wallpaperUrl: customWallpaperUrl,
                                    enabled: isCustomAvatarEnabled,
                                });
                            }}
                            disabled={!isCustomAvatarEnabled || (!customStickerUrl && !customWallpaperUrl)}
                            style={{
                                background: isCustomAvatarEnabled && (customStickerUrl || customWallpaperUrl)
                                    ? 'var(--vscode-button-background)'
                                    : 'rgba(255,255,255,0.1)',
                                color: isCustomAvatarEnabled && (customStickerUrl || customWallpaperUrl)
                                    ? 'var(--vscode-button-foreground)'
                                    : 'rgba(255,255,255,0.3)',
                                border: 'none',
                                padding: '6px 12px',
                                fontSize: '10px',
                                borderRadius: '4px',
                                cursor: isCustomAvatarEnabled && (customStickerUrl || customWallpaperUrl) ? 'pointer' : 'not-allowed',
                                alignSelf: 'flex-start',
                                marginTop: '4px',
                            }}
                        >
                            Save Custom Avatar
                        </button>
                        
                        <div style={{ fontSize: '9px', opacity: 0.6, marginTop: '4px' }}>
                            <i className="codicon codicon-info" style={{ marginRight: '4px', fontFamily: 'codicon', fontStyle: 'normal' }}></i>
                            Use PNG URLs for best quality. Avatars persist across reloads.
                        </div>
                    </div>
                </div>
                
                {/* 3D VRM Avatar Configuration */}
                <div style={{
                    marginTop: '16px',
                    padding: '12px',
                    background: 'rgba(168,85,247,0.05)',
                    borderRadius: '8px',
                    border: '1px solid rgba(168,85,247,0.2)',
                }}>
                    <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '10px' }}>
                        <div style={{ fontSize: '10px', fontWeight: 600, color: '#c084fc', textTransform: 'uppercase', display: 'flex', alignItems: 'center', gap: '6px' }}>
                            <i className="codicon codicon-preview" style={{ fontFamily: 'codicon', fontStyle: 'normal' }}></i>
                            3D VRM Avatar (Airi Panel)
                        </div>
                    </div>
                    
                    <div style={{ fontSize: '9px', opacity: 0.7, marginBottom: '10px', lineHeight: 1.4 }}>
                        Configure the 3D anime avatar that appears in the AIRI panel. Supports VRM 0.x/1.0 models.
                    </div>
                    
                    <div style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
                        {/* Pre-loaded Models */}
                        <div>
                            <label style={{ fontSize: '9px', opacity: 0.7, display: 'block', marginBottom: '6px' }}>
                                Pre-loaded Models (from AIRI cache)
                            </label>
                            <div style={{ display: 'grid', gridTemplateColumns: 'repeat(3, 1fr)', gap: '6px' }}>
                                {[
                                    { id: 'hiyori_pro', name: 'Hiyori Pro', desc: 'Professional Live2D' },
                                    { id: 'hiyori_free', name: 'Hiyori Free', desc: 'Free version' },
                                    { id: 'avatar_a', name: 'Avatar Sample A', desc: 'VRM sample' },
                                    { id: 'avatar_b', name: 'Avatar Sample B', desc: 'VRM sample' },
                                    { id: 'airi', name: 'AIRI Default', desc: 'Default avatar' },
                                    { id: 'sage', name: 'Sage', desc: 'Mature assistant' },
                                ].map(model => (
                                    <button
                                        key={model.id}
                                        onClick={() => {
                                            setVrmModelId(model.id);
                                            setVrmModelUrl('');
                                            setAvatar3dConfig({ modelId: model.id, modelUrl: undefined });
                                        }}
                                        style={{
                                            background: vrmModelId === model.id && !vrmModelUrl
                                                ? 'rgba(168,85,247,0.3)'
                                                : 'rgba(255,255,255,0.05)',
                                            border: `1px solid ${vrmModelId === model.id && !vrmModelUrl ? '#c084fc' : 'rgba(255,255,255,0.1)'}`,
                                            padding: '6px 8px',
                                            borderRadius: '4px',
                                            color: 'var(--vscode-foreground)',
                                            fontSize: '9px',
                                            cursor: 'pointer',
                                            textAlign: 'left',
                                        }}
                                    >
                                        <div style={{ fontWeight: 600, marginBottom: '2px' }}>{model.name}</div>
                                        <div style={{ fontSize: '7px', opacity: 0.6 }}>{model.desc}</div>
                                    </button>
                                ))}
                            </div>
                            <div style={{ fontSize: '8px', opacity: 0.5, marginTop: '4px' }}>
                                <i className="codicon codicon-check" style={{ marginRight: '4px', fontFamily: 'codicon', fontStyle: 'normal' }}></i>
                                These models are already cached locally - no download needed!
                            </div>
                        </div>
                        
                        {/* Custom VRM URL */}
                        <div>
                            <label style={{ fontSize: '9px', opacity: 0.7, display: 'block', marginBottom: '4px' }}>
                                Custom VRM Model URL
                            </label>
                            <input
                                type="text"
                                value={vrmModelUrl}
                                onChange={(e) => setVrmModelUrl(e.target.value)}
                                placeholder="https://example.com/model.vrm"
                                style={{
                                    width: '100%',
                                    background: 'var(--vscode-input-background)',
                                    color: 'var(--vscode-input-foreground)',
                                    border: '1px solid var(--vscode-input-border)',
                                    padding: '6px 10px',
                                    fontSize: '11px',
                                    borderRadius: '4px',
                                }}
                            />
                            <div style={{ fontSize: '8px', opacity: 0.6, marginTop: '4px' }}>
                                Enter a direct URL to a .vrm file (must be publicly accessible or local server)
                            </div>
                        </div>
                        
                        {/* Custom Models List */}
                        {customVrmModels.length > 0 && (
                            <div>
                                <label style={{ fontSize: '9px', opacity: 0.7, display: 'block', marginBottom: '6px' }}>
                                    Saved Custom Models
                                </label>
                                <div style={{ display: 'flex', flexDirection: 'column', gap: '4px' }}>
                                    {customVrmModels.map(model => (
                                        <div
                                            key={model.id}
                                            style={{
                                                display: 'flex',
                                                justifyContent: 'space-between',
                                                alignItems: 'center',
                                                padding: '6px 8px',
                                                background: vrmModelUrl === model.url
                                                    ? 'rgba(168,85,247,0.2)'
                                                    : 'rgba(255,255,255,0.02)',
                                                border: `1px solid ${vrmModelUrl === model.url ? '#c084fc' : 'rgba(255,255,255,0.1)'}`,
                                                borderRadius: '4px',
                                            }}
                                        >
                                            <div style={{ fontSize: '9px' }}>
                                                <div style={{ fontWeight: 600 }}>{model.name}</div>
                                                <div style={{ opacity: 0.6, fontSize: '8px' }}>{model.url}</div>
                                            </div>
                                            <div style={{ display: 'flex', gap: '4px' }}>
                                                <button
                                                    onClick={() => {
                                                        setVrmModelUrl(model.url);
                                                        setVrmModelId(undefined);
                                                        setAvatar3dConfig({ modelUrl: model.url, modelId: undefined });
                                                    }}
                                                    style={{
                                                        background: vrmModelUrl === model.url
                                                            ? 'var(--vscode-button-background)'
                                                            : 'rgba(255,255,255,0.1)',
                                                        color: 'var(--vscode-button-foreground)',
                                                        border: 'none',
                                                        padding: '3px 8px',
                                                        fontSize: '8px',
                                                        borderRadius: '2px',
                                                        cursor: 'pointer',
                                                    }}
                                                >
                                                    {vrmModelUrl === model.url ? '✓ Active' : 'Use'}
                                                </button>
                                                <button
                                                    onClick={() => {
                                                        const updated = customVrmModels.filter(m => m.id !== model.id);
                                                        setCustomVrmModels(updated);
                                                        setAvatar3dConfig({ customModels: updated });
                                                    }}
                                                    style={{
                                                        background: 'rgba(239,68,68,0.2)',
                                                        color: '#f87171',
                                                        border: 'none',
                                                        padding: '3px 8px',
                                                        fontSize: '8px',
                                                        borderRadius: '2px',
                                                        cursor: 'pointer',
                                                    }}
                                                    title="Delete"
                                                >
                                                    <i className="codicon codicon-trash" style={{ fontFamily: 'codicon', fontStyle: 'normal' }}></i>
                                                </button>
                                            </div>
                                        </div>
                                    ))}
                                </div>
                            </div>
                        )}
                        
                        {/* Add Custom Model */}
                        <div style={{
                            display: 'flex',
                            gap: '6px',
                            alignItems: 'flex-end',
                            marginTop: '4px',
                        }}>
                            <div style={{ flex: 1 }}>
                                <label style={{ fontSize: '8px', opacity: 0.6, display: 'block', marginBottom: '2px' }}>
                                    Model Name
                                </label>
                                <input
                                    type="text"
                                    id="newModelName"
                                    placeholder="My Custom Model"
                                    style={{
                                        width: '100%',
                                        background: 'var(--vscode-input-background)',
                                        color: 'var(--vscode-input-foreground)',
                                        border: '1px solid var(--vscode-input-border)',
                                        padding: '4px 8px',
                                        fontSize: '10px',
                                        borderRadius: '3px',
                                    }}
                                />
                            </div>
                            <div style={{ flex: 2 }}>
                                <label style={{ fontSize: '8px', opacity: 0.6, display: 'block', marginBottom: '2px' }}>
                                    VRM URL
                                </label>
                                <input
                                    type="text"
                                    id="newModelUrl"
                                    placeholder="https://example.com/model.vrm"
                                    style={{
                                        width: '100%',
                                        background: 'var(--vscode-input-background)',
                                        color: 'var(--vscode-input-foreground)',
                                        border: '1px solid var(--vscode-input-border)',
                                        padding: '4px 8px',
                                        fontSize: '10px',
                                        borderRadius: '3px',
                                    }}
                                />
                            </div>
                            <button
                                onClick={() => {
                                    const nameInput = document.getElementById('newModelName') as HTMLInputElement;
                                    const urlInput = document.getElementById('newModelUrl') as HTMLInputElement;
                                    if (nameInput.value && urlInput.value) {
                                        const newModel = {
                                            id: `custom_${Date.now()}`,
                                            name: nameInput.value,
                                            url: urlInput.value,
                                        };
                                        const updated = [...customVrmModels, newModel];
                                        setCustomVrmModels(updated);
                                        setAvatar3dConfig({ customModels: updated });
                                        nameInput.value = '';
                                        urlInput.value = '';
                                    }
                                }}
                                style={{
                                    background: 'var(--vscode-button-background)',
                                    color: 'var(--vscode-button-foreground)',
                                    border: 'none',
                                    padding: '5px 12px',
                                    fontSize: '9px',
                                    borderRadius: '3px',
                                    cursor: 'pointer',
                                    whiteSpace: 'nowrap',
                                }}
                            >
                                Add Model
                            </button>
                        </div>
                        
                        <div style={{ fontSize: '9px', opacity: 0.6 }}>
                            <i className="codicon codicon-info" style={{ marginRight: '4px', fontFamily: 'codicon', fontStyle: 'normal' }}></i>
                            3D avatar requires AIRI panel running at localhost:5174
                        </div>
                    </div>
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
                        // Removed: elevenlabs - now only in VOICE & TTS section below
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
                                key={`elevenlabs-input-${apiKeys.elevenlabs?.length || 0}`}
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
                            <button
                                onClick={async () => {
                                    console.log('[ElevenLabs] 🔄 Force replacing API key...');
                                    const apiKey = (apiKeys as any).elevenlabs;
                                    console.log('[ElevenLabs] Current key:', apiKey ? `${apiKey.substring(0, 8)}...` : 'empty', 'length:', apiKey?.length);
                                    
                                    // Always save, even if masked (force replace)
                                    if (apiKey && apiKey.length > 10) {
                                        try {
                                            const result = await invoke('save_api_keys', {
                                                keys: { elevenlabs_api_key: apiKey }
                                            });
                                            console.log('[ElevenLabs] ✅ Force replace result:', result);
                                            setKeyStatus(prev => ({ ...prev, ...result }));
                                            
                                            // Verify save
                                            const reloaded = await invoke('get_api_keys');
                                            console.log('[ElevenLabs] 🔍 Verified save:', {
                                                elevenlabs_api_key: (reloaded as any).elevenlabs_api_key ? '✓ REPLACED' : '✗ FAILED',
                                                elevenlabs_voice_id: (reloaded as any).elevenlabs_voice_id || 'NOT SET'
                                            });
                                            
                                            alert('✅ ElevenLabs API key REPLACED successfully!\n\nOld key has been overwritten with new key.\nCheck console for details.');
                                        } catch (err: any) {
                                            console.error('[ElevenLabs] ❌ Force replace failed:', err);
                                            alert('❌ Failed to replace: ' + (err.message || err));
                                        }
                                    } else {
                                        console.log('[ElevenLabs] ⚠️ Key too short');
                                        alert('⚠️ Please enter a valid API key (starts with sk_, min 10 chars)');
                                    }
                                }}
                                style={{
                                    background: 'var(--vscode-button-background)',
                                    color: 'var(--vscode-button-foreground)',
                                    border: 'none',
                                    padding: '6px 12px',
                                    fontSize: '11px',
                                    cursor: 'pointer',
                                    borderRadius: '4px',
                                    marginLeft: '6px',
                                }}
                                title="Force replace API key (overwrites old key)"
                            >
                                <i className="codicon codicon-refresh" style={{ fontFamily: 'codicon', fontStyle: 'normal', marginRight: '4px' }}></i>
                                Replace
                            </button>
                        </div>
                    </div>

                    {/* ElevenLabs Voice Picker */}
                    <ElevenLabsVoicePicker
                        apiKey={realApiKey || apiKeys.elevenlabs}
                        selectedVoiceId={selectedElevenLabsVoice}
                        onVoiceSelect={(voiceId) => {
                            // Update local state
                            setSelectedElevenLabsVoice(voiceId);
                            
                            // Import voice module dynamically to set selected voice in voice.ts
                            import('../voice').then(({ setSelectedVoice }) => {
                                setSelectedVoice(voiceId);
                                console.log('[Settings] ElevenLabs voice set:', voiceId);
                            });
                            
                            // Save to persistent storage
                            invoke('save_api_keys', { 
                                keys: { elevenlabs_voice_id: voiceId } 
                            }).then(() => {
                                console.log('[Settings] ElevenLabs voice ID saved');
                            }).catch(err => {
                                console.error('[Settings] Failed to save voice ID:', err);
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
