import React, { useState, useEffect, useCallback } from 'react';
import { invoke } from '../tauri_bridge';

export interface ElevenLabsVoice {
    voice_id: string;
    name: string;
    labels: any;
    preview_url: string | null;
    category: string | null;
    gender: string | null;
    age: string | null;
    accent: string | null;
}

interface ElevenLabsVoicePickerProps {
    onVoiceSelect: (voiceId: string) => void;
    selectedVoiceId?: string;
    apiKey: string;
}

const ElevenLabsVoicePicker: React.FC<ElevenLabsVoicePickerProps> = ({ onVoiceSelect, selectedVoiceId, apiKey }) => {
    const [voices, setVoices] = useState<ElevenLabsVoice[]>([]);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [playingId, setPlayingId] = useState<string | null>(null);
    const [filter, setFilter] = useState<'all' | 'premade' | 'cloned' | 'generated'>('premade');
    const [saveStatus, setSaveStatus] = useState<'idle' | 'saving' | 'saved'>('idle');

    const fetchVoices = useCallback(async () => {
        console.log('[Voice Picker] fetchVoices called');
        console.log('[Voice Picker] apiKey:', apiKey ? `${apiKey.substring(0, 8)}... (length: ${apiKey.length})` : 'MISSING');
        console.log('[Voice Picker] apiKey starts with sk_:', apiKey?.startsWith('sk_'));
        
        if (!apiKey || !apiKey.startsWith('sk_')) {
            console.warn('[Voice Picker] API key not configured or invalid');
            setError('ElevenLabs API key not configured');
            return;
        }

        setLoading(true);
        setError(null);
        try {
            console.log('[Voice Picker] Calling elevenlabs_get_voices...');
            const fetchedVoices = await invoke<ElevenLabsVoice[]>('elevenlabs_get_voices');
            console.log('[Voice Picker] ✅ Voices fetched:', fetchedVoices.length);
            setVoices(fetchedVoices);
        } catch (e: any) {
            console.error('[Voice Picker] ❌ Error fetching voices:', e);
            setError(e?.message || e?.toString() || 'Failed to fetch voices');
        } finally {
            setLoading(false);
        }
    }, [apiKey]);

    const handleSaveVoice = useCallback(async (voiceId: string, e: React.MouseEvent) => {
        e.stopPropagation();
        setSaveStatus('saving');
        console.log('[Voice Picker] 💾 Saving voice:', voiceId);
        
        try {
            // Save voice ID to persistent storage
            const result = await invoke('save_api_keys', { 
                keys: { elevenlabs_voice_id: voiceId } 
            });
            
            console.log('[Voice Picker] ✅ Save result:', result);
            console.log('[Voice Picker] ✅ Voice ID saved to api_keys.json');
            
            // Notify parent component to update local state
            onVoiceSelect(voiceId);
            console.log('[Voice Picker] ✅ Parent component notified');
            
            // Also update voice.ts module directly
            try {
                const voiceModule = await import('../voice');
                if (voiceModule.setSelectedVoice) {
                    voiceModule.setSelectedVoice(voiceId);
                    console.log('[Voice Picker] ✅ Voice set in voice.ts:', voiceId);
                }
            } catch (err) {
                console.warn('[Voice Picker] ⚠️ Could not update voice.ts:', err);
            }
            
            // Show saved confirmation
            setSaveStatus('saved');
            console.log('[Voice Picker] 🎉 Save complete!');
            setTimeout(() => setSaveStatus('idle'), 2000);
        } catch (err) {
            console.error('[Voice Picker] ❌ Failed to save voice:', err);
            setError('Failed to save voice selection');
            setSaveStatus('idle');
        }
    }, [onVoiceSelect]);

    useEffect(() => {
        fetchVoices();
    }, [fetchVoices]);

    const playPreview = (voiceId: string, previewUrl: string | null) => {
        if (!previewUrl) return;
        setPlayingId(voiceId);
        const audio = new Audio(previewUrl);
        audio.onended = () => setPlayingId(null);
        audio.onerror = () => setPlayingId(null);
        audio.play();
    };

    const filteredVoices = filter === 'all' ? voices : voices.filter(v => v.category === filter);

    const genderIcon = (gender: string | null) => {
        if (gender === 'male') return '♂';
        if (gender === 'female') return '♀';
        return '';
    };

    if (!apiKey || !apiKey.startsWith('sk_')) {
        return (
            <div style={{
                padding: '12px',
                background: 'var(--vscode-input-background)',
                borderRadius: '4px',
                border: '1px solid var(--vscode-input-border)',
                fontSize: '12px',
                color: 'var(--vscode-foreground)',
                opacity: 0.7,
            }}>
                <i className="codicon codicon-warning" style={{ marginRight: '4px', fontFamily: 'codicon', fontStyle: 'normal' }}></i>
                Configure your ElevenLabs API key first
            </div>
        );
    }

    return (
        <div style={{
            background: 'var(--vscode-input-background)',
            borderRadius: '4px',
            border: '1px solid var(--vscode-input-border)',
            overflow: 'hidden',
        }}>
            {/* Header */}
            <div style={{
                padding: '8px 12px',
                borderBottom: '1px solid var(--vscode-panel-border)',
                display: 'flex',
                justifyContent: 'space-between',
                alignItems: 'center',
            }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                    <i className="codicon codicon-mic" style={{ fontSize: '14px', fontFamily: 'codicon', fontStyle: 'normal' }}></i>
                    <span style={{ fontSize: '11px', fontWeight: 600, textTransform: 'uppercase' }}>
                        ElevenLabs Voices
                    </span>
                    <span style={{
                        background: 'var(--vscode-badge-background)',
                        color: 'var(--vscode-badge-foreground)',
                        padding: '1px 6px',
                        borderRadius: '10px',
                        fontSize: '10px',
                    }}>{voices.length}</span>
                </div>
                <button
                    onClick={fetchVoices}
                    style={{
                        background: 'transparent',
                        border: 'none',
                        color: 'var(--vscode-foreground)',
                        cursor: loading ? 'wait' : 'pointer',
                        padding: '4px',
                        borderRadius: '2px',
                    }}
                    title="Refresh voices"
                >
                    <i className={`codicon codicon-${loading ? 'loading' : 'refresh'}`} style={{
                        fontSize: '14px',
                        fontFamily: 'codicon',
                        fontStyle: 'normal',
                        animation: loading ? 'spin 1s linear infinite' : 'none',
                    }}></i>
                </button>
            </div>

            {/* Error */}
            {error && (
                <div style={{
                    padding: '8px 12px',
                    fontSize: '11px',
                    color: '#f87171',
                    borderBottom: '1px solid var(--vscode-panel-border)',
                }}>
                    {error}
                </div>
            )}

            {/* Filter Tabs */}
            <div style={{
                display: 'flex',
                gap: '2px',
                padding: '6px 8px',
                borderBottom: '1px solid var(--vscode-panel-border)',
            }}>
                {(['premade', 'cloned', 'generated', 'all'] as const).map(f => (
                    <button
                        key={f}
                        onClick={() => setFilter(f)}
                        style={{
                            background: filter === f ? 'var(--vscode-button-background)' : 'transparent',
                            color: filter === f ? 'var(--vscode-button-foreground)' : 'var(--vscode-foreground)',
                            border: '1px solid var(--vscode-button-background)',
                            padding: '2px 8px',
                            fontSize: '10px',
                            borderRadius: '2px',
                            cursor: 'pointer',
                            textTransform: 'capitalize',
                        }}
                    >
                        {f}
                    </button>
                ))}
            </div>

            {/* Voice List */}
            <div style={{
                maxHeight: '300px',
                overflowY: 'auto',
                padding: '4px 0',
            }}>
                {loading && voices.length === 0 ? (
                    <div style={{
                        padding: '20px',
                        textAlign: 'center',
                        fontSize: '12px',
                        opacity: 0.5,
                    }}>
                        <i className="codicon codicon-loading" style={{
                            animation: 'spin 1s linear infinite',
                            marginRight: '4px',
                            fontFamily: 'codicon',
                            fontStyle: 'normal',
                        }}></i>
                        Loading voices...
                    </div>
                ) : filteredVoices.length === 0 ? (
                    <div style={{
                        padding: '20px',
                        textAlign: 'center',
                        fontSize: '12px',
                        opacity: 0.5,
                    }}>
                        No voices found
                    </div>
                ) : (
                    filteredVoices.map(voice => (
                        <div
                            key={voice.voice_id}
                            style={{
                                padding: '8px 12px',
                                cursor: 'pointer',
                                background: selectedVoiceId === voice.voice_id
                                    ? 'var(--vscode-list-activeSelectionBackground)'
                                    : 'transparent',
                                color: selectedVoiceId === voice.voice_id
                                    ? 'var(--vscode-list-activeSelectionForeground)'
                                    : 'var(--vscode-foreground)',
                                borderBottom: '1px solid var(--vscode-panel-border)',
                                display: 'flex',
                                alignItems: 'center',
                                gap: '8px',
                                fontSize: '12px',
                            }}
                        >
                            {/* Gender icon */}
                            <span style={{
                                opacity: 0.5,
                                fontSize: '14px',
                                width: '16px',
                                textAlign: 'center',
                            }}>
                                {genderIcon(voice.gender)}
                            </span>

                            {/* Voice name - click to select */}
                            <div 
                                onClick={() => onVoiceSelect(voice.voice_id)}
                                style={{ flex: 1 }}
                            >
                                <span style={{ fontWeight: selectedVoiceId === voice.voice_id ? 600 : 400 }}>
                                    {voice.name}
                                </span>
                                {selectedVoiceId === voice.voice_id && (
                                    <span style={{
                                        marginLeft: '6px',
                                        fontSize: '9px',
                                        color: '#4ade80',
                                        fontWeight: 600,
                                    }}>✓ SELECTED</span>
                                )}
                                {/* Metadata chips */}
                                <div style={{ display: 'flex', gap: '4px', flexWrap: 'wrap', marginTop: '3px' }}>
                                    {voice.age && (
                                        <span style={{
                                            background: 'var(--vscode-badge-background)',
                                            color: 'var(--vscode-badge-foreground)',
                                            padding: '1px 4px',
                                            borderRadius: '3px',
                                            fontSize: '9px',
                                            textTransform: 'capitalize',
                                        }}>{voice.age}</span>
                                    )}
                                    {voice.accent && (
                                        <span style={{
                                            background: 'var(--vscode-badge-background)',
                                            color: 'var(--vscode-badge-foreground)',
                                            padding: '1px 4px',
                                            borderRadius: '3px',
                                            fontSize: '9px',
                                        }}>{voice.accent}</span>
                                    )}
                                </div>
                            </div>

                            {/* Play preview */}
                            {voice.preview_url && (
                                <button
                                    onClick={(e) => {
                                        e.stopPropagation();
                                        playPreview(voice.voice_id, voice.preview_url);
                                    }}
                                    title="Play preview"
                                    style={{
                                        background: 'transparent',
                                        border: 'none',
                                        color: playingId === voice.voice_id ? 'var(--vscode-button-background)' : 'var(--vscode-foreground)',
                                        cursor: 'pointer',
                                        padding: '4px',
                                        display: 'flex',
                                        alignItems: 'center',
                                        borderRadius: '3px',
                                    }}
                                >
                                    <i className={`codicon codicon-${playingId === voice.voice_id ? 'loading' : 'play-circle'}`} style={{
                                        fontSize: '16px',
                                        fontFamily: 'codicon',
                                        fontStyle: 'normal',
                                        animation: playingId === voice.voice_id ? 'spin 1s linear infinite' : 'none',
                                    }}></i>
                                </button>
                            )}

                            {/* Save voice button */}
                            <button
                                onClick={(e) => handleSaveVoice(voice.voice_id, e)}
                                title={selectedVoiceId === voice.voice_id ? 'Selected voice' : 'Select and save this voice'}
                                disabled={saveStatus === 'saving'}
                                style={{
                                    background: selectedVoiceId === voice.voice_id
                                        ? 'var(--vscode-button-background)'
                                        : saveStatus === 'saving' && selectedVoiceId !== voice.voice_id
                                        ? 'rgba(255,255,255,0.1)'
                                        : 'rgba(255,255,255,0.05)',
                                    color: selectedVoiceId === voice.voice_id
                                        ? 'var(--vscode-button-foreground)'
                                        : 'var(--vscode-foreground)',
                                    border: selectedVoiceId === voice.voice_id
                                        ? 'none'
                                        : '1px solid var(--vscode-button-background)',
                                    padding: '4px 10px',
                                    fontSize: '9px',
                                    borderRadius: '3px',
                                    cursor: saveStatus === 'saving' ? 'wait' : 'pointer',
                                    display: 'flex',
                                    alignItems: 'center',
                                    gap: '4px',
                                    minWidth: '70px',
                                    justifyContent: 'center',
                                }}
                            >
                                {saveStatus === 'saving' && selectedVoiceId !== voice.voice_id ? (
                                    <>
                                        <i className="codicon codicon-loading" style={{
                                            fontSize: '12px',
                                            fontFamily: 'codicon',
                                            fontStyle: 'normal',
                                            animation: 'spin 1s linear infinite',
                                        }}></i>
                                        Saving...
                                    </>
                                ) : selectedVoiceId === voice.voice_id ? (
                                    <>
                                        <i className="codicon codicon-check" style={{ fontSize: '12px', fontFamily: 'codicon', fontStyle: 'normal' }}></i>
                                        Saved
                                    </>
                                ) : (
                                    <>
                                        <i className="codicon codicon-save" style={{ fontSize: '12px', fontFamily: 'codicon', fontStyle: 'normal' }}></i>
                                        Save
                                    </>
                                )}
                            </button>
                        </div>
                    ))
                )}
            </div>
        </div>
    );
};

export default ElevenLabsVoicePicker;
