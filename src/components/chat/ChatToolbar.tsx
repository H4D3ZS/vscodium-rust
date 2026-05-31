import React from 'react';
import { useStore } from '../../store';
import { Volume2, VolumeX } from 'lucide-react';

interface ChatToolbarProps {
    mode: string;
    model: string;
    modeStyle: { label: string; color: string; background: string; border: string; title: string };
    modelLabel: string;
    ttsEnabled: boolean;
    ttsPreset: string;
    isAgentThinking: boolean;
    onModeClick: (e: React.MouseEvent) => void;
    onModelClick: (e: React.MouseEvent) => void;
    onAttach: () => void;
    onToggleTts: () => void;
    onTtsPresetChange: (preset: string) => void;
    onToggleVoice: () => void;
    isVoiceListening: boolean;
    reasoningToggle?: React.ReactNode;
    webUiControls?: React.ReactNode;
    onSend: () => void;
    inputEmpty: boolean;
}

const ChatToolbar: React.FC<ChatToolbarProps> = ({
    modeStyle, modelLabel, ttsEnabled, ttsPreset,
    isAgentThinking,
    onModeClick, onModelClick, onAttach,
    onToggleTts, onTtsPresetChange,
    onToggleVoice, isVoiceListening,
    reasoningToggle, webUiControls,
    onSend, inputEmpty,
}) => {
    const isSpecModeActive = useStore(state => state.isSpecModeActive);
    const setSpecModeActive = useStore(state => state.setSpecModeActive);
    const isPlanMode = useStore(state => state.isPlanMode);
    const togglePlanMode = useStore(state => state.togglePlanMode);
    const isCascadeWriteMode = useStore(state => state.isCascadeWriteMode);
    const toggleCascadeWriteMode = useStore(state => state.toggleCascadeWriteMode);

    return (
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginTop: '6px' }}>
            <div style={{ display: 'flex', gap: '8px', alignItems: 'center', flexWrap: 'wrap' }}>
                <div onClick={onAttach} style={{ cursor: 'pointer', opacity: 0.5, display: 'flex', alignItems: 'center' }} title="Attach File (Neural Gist)">
                    <i className="codicon codicon-attach" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '13px' }} />
                </div>

                <span
                    onClick={onModeClick}
                    style={{ fontSize: '10px', cursor: 'pointer', padding: '1px 6px', borderRadius: '4px', fontWeight: 600, color: modeStyle.color, background: modeStyle.background, border: modeStyle.border }}
                    title={modeStyle.title}
                >
                    {modeStyle.label}
                </span>

                <span onClick={onModelClick} style={{ fontSize: '10px', opacity: 0.5, cursor: 'pointer' }}>{modelLabel}</span>

                {reasoningToggle}

                {/* Cascade Write Mode toggle (Windsurf-style — live streaming edits) */}
                <div
                    onClick={toggleCascadeWriteMode}
                    title={isCascadeWriteMode ? 'Live mode ON — edits stream directly to editor' : 'Live mode OFF — review diffs manually'}
                    style={{
                        cursor: 'pointer',
                        display: 'flex', alignItems: 'center', gap: '4px',
                        fontSize: '10px', fontWeight: 600,
                        padding: '1px 6px', borderRadius: '4px',
                        color: isCascadeWriteMode ? '#34d399' : 'rgba(255,255,255,0.35)',
                        background: isCascadeWriteMode ? 'rgba(52,211,153,0.08)' : 'transparent',
                        border: isCascadeWriteMode ? '1px solid rgba(52,211,153,0.3)' : '1px solid rgba(255,255,255,0.1)',
                        transition: 'all 0.15s',
                    }}
                >
                    {isCascadeWriteMode ? '⚡ Live' : '⚡'}
                </div>

                {/* Plan Mode toggle (Claude Code style — plan before execute) */}
                <div
                    onClick={togglePlanMode}
                    title={isPlanMode ? 'Plan Mode ON — agent generates task list before editing' : 'Enable Plan Mode'}
                    style={{
                        cursor: 'pointer',
                        display: 'flex', alignItems: 'center', gap: '4px',
                        fontSize: '10px', fontWeight: 600,
                        padding: '1px 6px', borderRadius: '4px',
                        color: isPlanMode ? '#f59e0b' : 'rgba(255,255,255,0.35)',
                        background: isPlanMode ? 'rgba(245,158,11,0.08)' : 'transparent',
                        border: isPlanMode ? '1px solid rgba(245,158,11,0.3)' : '1px solid rgba(255,255,255,0.1)',
                        transition: 'all 0.15s',
                    }}
                >
                    {isPlanMode ? '📋 Plan' : '📋'}
                </div>

                {/* Spec Mode toggle */}
                <div
                    onClick={() => setSpecModeActive(!isSpecModeActive)}
                    title={isSpecModeActive ? 'Spec Mode ON — click to disable' : 'Enable Spec Mode'}
                    style={{
                        cursor: 'pointer',
                        display: 'flex', alignItems: 'center', gap: '4px',
                        fontSize: '10px', fontWeight: 600,
                        padding: '1px 6px', borderRadius: '4px',
                        color: isSpecModeActive ? 'var(--terminator-accent, #00c6ff)' : 'rgba(255,255,255,0.35)',
                        background: isSpecModeActive ? 'rgba(0,198,255,0.08)' : 'transparent',
                        border: isSpecModeActive ? '1px solid rgba(0,198,255,0.3)' : '1px solid rgba(255,255,255,0.1)',
                        transition: 'all 0.15s',
                    }}
                >
                    📋
                </div>

                <div
                    onClick={onToggleTts}
                    style={{ cursor: 'pointer', opacity: ttsEnabled ? 0.9 : 0.4, display: 'flex', alignItems: 'center', color: ttsEnabled ? '#c084fc' : undefined }}
                    title={ttsEnabled ? 'Vocal Mode ON' : 'Muted'}
                >
                    {ttsEnabled ? <Volume2 size={14} /> : <VolumeX size={14} />}
                </div>

                {ttsEnabled && (
                    <select
                        value={ttsPreset}
                        onChange={e => onTtsPresetChange(e.target.value)}
                        style={{ height: '20px', background: 'rgba(255,255,255,0.04)', border: '1px solid rgba(255,255,255,0.10)', borderRadius: '4px', color: 'rgba(255,255,255,0.72)', fontSize: '9px', outline: 'none' }}
                        title="Voice Preset"
                    >
                        {['airi', 'sage', 'nova', 'aria', 'kawaii', 'yamato', 'hana', 'ren', 'yuki', 'haru', 'sora', 'zero'].map(p => (
                            <option key={p} value={p}>{p.charAt(0).toUpperCase() + p.slice(1)}</option>
                        ))}
                    </select>
                )}

                {webUiControls}
            </div>

            <div style={{ display: 'flex', gap: '6px', alignItems: 'center' }}>
                <div
                    onClick={onToggleVoice}
                    style={{ cursor: 'pointer', opacity: isVoiceListening ? 1 : 0.4, display: 'flex', alignItems: 'center', color: isVoiceListening ? '#ef4444' : undefined }}
                    title={isVoiceListening ? 'Stop voice input' : 'Start voice input'}
                >
                    <i className={`codicon codicon-${isVoiceListening ? 'stop-circle' : 'mic'}`} style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '13px' }} />
                </div>

                <button
                    onClick={onSend}
                    disabled={isAgentThinking || inputEmpty}
                    style={{
                        background: isAgentThinking || inputEmpty ? 'rgba(255,255,255,0.06)' : '#3b82f6',
                        border: 'none', borderRadius: '6px',
                        color: isAgentThinking || inputEmpty ? 'rgba(255,255,255,0.3)' : '#fff',
                        padding: '4px 10px', fontSize: '11px', fontWeight: 600,
                        cursor: isAgentThinking || inputEmpty ? 'not-allowed' : 'pointer',
                        transition: 'all 0.15s',
                    }}
                >
                    {isAgentThinking ? '...' : 'Send'}
                </button>
            </div>
        </div>
    );
};

export default ChatToolbar;
