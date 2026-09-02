/**
 * Cursor-style chat toolbar — mode selector, model picker, attach, plan/live toggles.
 * Clean pill-based design with consistent hover states.
 */
import React, { useState, useRef, useEffect, useMemo } from 'react';
import { createPortal } from 'react-dom';
import { useStore } from '../../store';
import { Volume2, VolumeX } from 'lucide-react';
import ScreenRecordingButton from '../agent/ScreenRecordingButton';
import ModeSwitcher, { AGENT_MODES } from './ModeSwitcher';
import ModelPicker from './ModelPicker';
import type { ModelInfo } from './ModelPicker';

/**
 * Fixed-position wrapper for a dropdown rendered in a body portal. Anchors the
 * picker to the pill's on-screen rect and opens upward (children use
 * `bottom: 100%`), clamped to stay inside the viewport. Escapes the toolbar's
 * overflow-scroll clip that otherwise hides the dropdown.
 */
function pickerPortalStyle(anchor: DOMRect, width: number): React.CSSProperties {
    const margin = 8;
    const left = Math.max(margin, Math.min(anchor.left, window.innerWidth - width - margin));
    return { position: 'fixed', left, top: anchor.top, width, zIndex: 1000 };
}

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
    mode, modeStyle, modelLabel, ttsEnabled, ttsPreset,
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
    const setAgentMode = useStore(state => state.setAgentMode);
    const agentModel = useStore(state => state.agentModel);
    const setAgentModel = useStore(state => state.setAgentModel);
    const refreshAvailableModels = useStore(state => state.refreshAvailableModels);
    const ollamaStatus = useStore(state => state.ollamaStatus);

    const [showModePicker, setShowModePicker] = useState(false);
    const [showModelPicker, setShowModelPicker] = useState(false);
    const [isRefreshingModels, setIsRefreshingModels] = useState(false);
    const modeRef = useRef<HTMLDivElement>(null);
    const modelRef = useRef<HTMLDivElement>(null);

    // Pull the live installed-model list from the active provider.
    // Called when the picker opens and via the manual refresh button.
    const doRefreshModels = React.useCallback(async () => {
        setIsRefreshingModels(true);
        try {
            const backend = useStore.getState().inferenceBackend || 'lemonade';
            await refreshAvailableModels?.(backend);
        }
        finally { setIsRefreshingModels(false); }
    }, [refreshAvailableModels]);

    // Pickers render in a fixed-position portal (below) so they escape the
    // toolbar's horizontal-scroll overflow clip. Capture the pill's screen rect
    // on open to anchor the portal.
    const [modelAnchor, setModelAnchor] = useState<DOMRect | null>(null);
    const [modeAnchor, setModeAnchor] = useState<DOMRect | null>(null);

    const openModelPicker = () => {
        setShowModelPicker(v => {
            const next = !v;
            if (next) {
                setModelAnchor(modelRef.current?.getBoundingClientRect() ?? null);
                void doRefreshModels();
            }
            return next;
        });
    };

    const openModePicker = () => {
        setShowModePicker(v => {
            const next = !v;
            if (next) setModeAnchor(modeRef.current?.getBoundingClientRect() ?? null);
            return next;
        });
    };

    const currentModeOption = AGENT_MODES.find(m => m.id === mode) || AGENT_MODES[0];

    // Build model list from available models
    const availableModels = useStore(state => state.availableModels);
    const modelList: ModelInfo[] = useMemo(() => {
        const models = availableModels || [];
        const mapped = models.map((m: any) => {
            const provider = typeof m === 'string' ? (m.includes('|') ? m.split('|')[0] : 'Lemonade') : m.provider || 'Lemonade';
            const rawId = typeof m === 'string' ? m : m.id || m.name;
            // Store model with provider prefix so provider detection works.
            // Format: "provider|modelId" (e.g. "lemonade|Qwythos-9B...")
            const id = rawId.includes('|') ? rawId : `${provider}|${rawId}`;
            return {
                id,
                name: typeof m === 'string' ? m.split('/').pop() || m : m.name || m.id,
                provider,
                contextWindow: m.contextWindow,
                capabilities: m.capabilities,
                isLocal: !m.provider || m.provider.toLowerCase() === 'lemonade',
            };
        });
        // Ollama-first: local installed models float to the top of the picker.
        return mapped.sort((a, b) => (a.isLocal === b.isLocal ? 0 : a.isLocal ? -1 : 1));
    }, [availableModels]);

    // Close dropdowns on outside click
    useEffect(() => {
        const handler = (e: MouseEvent) => {
            const t = e.target as HTMLElement;
            // Pickers live in a body portal — treat clicks inside them as "inside".
            const inModePortal = !!t.closest?.('[data-portal="mode-picker"]');
            const inModelPortal = !!t.closest?.('[data-portal="model-picker"]');
            if (modeRef.current && !modeRef.current.contains(t) && !inModePortal) setShowModePicker(false);
            if (modelRef.current && !modelRef.current.contains(t) && !inModelPortal) setShowModelPicker(false);
        };
        document.addEventListener('mousedown', handler);
        return () => document.removeEventListener('mousedown', handler);
    }, []);

    return (
        <div style={{ display: 'flex', alignItems: 'center', gap: '6px', marginTop: '6px', width: '100%', minWidth: 0 }}>
            <div className="vscr-toolbar-scroll" style={{ display: 'flex', gap: '6px', alignItems: 'center', flexWrap: 'nowrap', overflowX: 'auto', flex: 1, minWidth: 0 }}>
                {/* Attach */}
                <ToolButton icon="attach" onClick={onAttach} title="Attach File" />

                <ScreenRecordingButton />

                {/* Mode selector — Cursor-style pill */}
                <div ref={modeRef} style={{ position: 'relative' }}>
                    <div
                        onClick={(e) => { e.stopPropagation(); openModePicker(); }}
                        className="vscr-pill"
                        style={{
                            color: currentModeOption.color,
                            background: `${currentModeOption.color}12`,
                            border: `1px solid ${currentModeOption.color}30`,
                            fontWeight: 600, cursor: 'pointer',
                        }}
                        title={`${currentModeOption.label} — ${currentModeOption.description}`}
                    >
                        <i className={`codicon codicon-${currentModeOption.icon}`} style={{
                            fontFamily: 'codicon', fontStyle: 'normal', fontSize: '11px', marginRight: '3px',
                        }} />
                        {currentModeOption.label}
                        <i className="codicon codicon-chevron-down vscr-pill__caret" />
                    </div>
                    {showModePicker && modeAnchor && createPortal(
                        <div data-portal="mode-picker" style={pickerPortalStyle(modeAnchor, 240)}>
                            <ModeSwitcher
                                currentMode={mode}
                                onSelect={(m) => { setAgentMode(m); setShowModePicker(false); }}
                            />
                        </div>,
                        document.body,
                    )}
                </div>

                {/* Model selector — Cursor-style pill */}
                <div ref={modelRef} style={{ position: 'relative' }}>
                    <div
                        onClick={(e) => { e.stopPropagation(); openModelPicker(); }}
                        title={modelLabel}
                        className="vscr-pill vscr-pill--model"
                        style={{ cursor: 'pointer' }}
                    >
                        <span className="vscr-pill__label">{modelLabel}</span>
                        <i className="codicon codicon-chevron-down vscr-pill__caret" />
                    </div>
                    {showModelPicker && modelAnchor && createPortal(
                        <div data-portal="model-picker" style={pickerPortalStyle(modelAnchor, 300)}>
                            <ModelPicker
                                models={modelList}
                                selectedModel={agentModel}
                                onSelect={(id) => setAgentModel(id)}
                                onClose={() => setShowModelPicker(false)}
                                onRefresh={doRefreshModels}
                                isRefreshing={isRefreshingModels}
                                ollamaStatus={ollamaStatus}
                            />
                        </div>,
                        document.body,
                    )}
                </div>

                {reasoningToggle}

                {/* Live mode toggle */}
                <TogglePill
                    active={isCascadeWriteMode}
                    onClick={toggleCascadeWriteMode}
                    activeColor="#34d399"
                    title={isCascadeWriteMode ? 'Live mode ON — edits stream to editor' : 'Live mode OFF'}
                    label={isCascadeWriteMode ? 'Live' : undefined}
                    icon="zap"
                />

                {/* Plan mode toggle */}
                <TogglePill
                    active={isPlanMode}
                    onClick={togglePlanMode}
                    activeColor="#f59e0b"
                    title={isPlanMode ? 'Plan Mode ON' : 'Enable Plan Mode'}
                    label={isPlanMode ? 'Plan' : undefined}
                    icon="list-ordered"
                />

                {/* Spec mode */}
                <TogglePill
                    active={isSpecModeActive}
                    onClick={() => setSpecModeActive(!isSpecModeActive)}
                    activeColor="var(--terminator-accent, #00c6ff)"
                    title={isSpecModeActive ? 'Spec Mode ON' : 'Enable Spec Mode'}
                    icon="file-submodule"
                />

                {/* TTS */}
                <ToolButton
                    icon={ttsEnabled ? 'unmute' : 'mute'}
                    onClick={onToggleTts}
                    active={ttsEnabled}
                    activeColor="#c084fc"
                    title={ttsEnabled ? 'Vocal Mode ON' : 'Muted'}
                />

                {ttsEnabled && (
                    <select
                        value={ttsPreset}
                        onChange={e => onTtsPresetChange(e.target.value)}
                        style={{
                            height: '20px', background: 'rgba(255,255,255,0.04)',
                            border: '1px solid rgba(255,255,255,0.10)', borderRadius: '4px',
                            color: 'rgba(255,255,255,0.72)', fontSize: '9px', outline: 'none',
                        }}
                        title="Voice Preset"
                    >
                        {['airi', 'sage', 'nova', 'aria', 'kawaii', 'yamato', 'hana', 'ren', 'yuki', 'haru', 'sora', 'zero'].map(p => (
                            <option key={p} value={p}>{p.charAt(0).toUpperCase() + p.slice(1)}</option>
                        ))}
                    </select>
                )}

                {webUiControls}
            </div>

            <div style={{ display: 'flex', gap: '6px', alignItems: 'center', flexShrink: 0 }}>
                {/* Voice input */}
                <ToolButton
                    icon={isVoiceListening ? 'stop-circle' : 'mic'}
                    onClick={onToggleVoice}
                    active={isVoiceListening}
                    activeColor="#ef4444"
                    title={isVoiceListening ? 'Stop voice input' : 'Start voice input'}
                />

                {/* Send button */}
                <button
                    onClick={onSend}
                    disabled={isAgentThinking || inputEmpty}
                    style={{
                        background: isAgentThinking || inputEmpty ? 'rgba(255,255,255,0.06)' : '#3b82f6',
                        border: 'none', borderRadius: '6px',
                        color: isAgentThinking || inputEmpty ? 'rgba(255,255,255,0.3)' : '#fff',
                        padding: '4px 12px', fontSize: '11px', fontWeight: 600,
                        cursor: isAgentThinking || inputEmpty ? 'not-allowed' : 'pointer',
                        transition: 'all 0.15s',
                        display: 'flex', alignItems: 'center', gap: '4px',
                    }}
                >
                    {isAgentThinking ? (
                        <i className="codicon codicon-loading codicon-modifier-spin" style={{
                            fontFamily: 'codicon', fontStyle: 'normal', fontSize: '11px',
                        }} />
                    ) : (
                        <i className="codicon codicon-send" style={{
                            fontFamily: 'codicon', fontStyle: 'normal', fontSize: '11px',
                        }} />
                    )}
                    {isAgentThinking ? '' : 'Send'}
                </button>
            </div>
        </div>
    );
};

/** Reusable toolbar icon button */
const ToolButton: React.FC<{
    icon: string;
    onClick: () => void;
    active?: boolean;
    activeColor?: string;
    title?: string;
}> = ({ icon, onClick, active, activeColor, title }) => (
    <div
        onClick={onClick}
        style={{
            cursor: 'pointer',
            opacity: active ? 1 : 0.45,
            display: 'flex', alignItems: 'center',
            color: active && activeColor ? activeColor : undefined,
            padding: '2px', borderRadius: '4px',
            transition: 'opacity 0.1s',
        }}
        title={title}
        onMouseEnter={(e) => { (e.currentTarget as HTMLDivElement).style.opacity = '1'; }}
        onMouseLeave={(e) => { if (!active) (e.currentTarget as HTMLDivElement).style.opacity = '0.45'; }}
    >
        <i className={`codicon codicon-${icon}`} style={{
            fontFamily: 'codicon', fontStyle: 'normal', fontSize: '13px',
        }} />
    </div>
);

/** Toggle pill — Cursor-style compact toggle */
const TogglePill: React.FC<{
    active: boolean;
    onClick: () => void;
    activeColor: string;
    title: string;
    label?: string;
    icon: string;
}> = ({ active, onClick, activeColor, title, label, icon }) => (
    <div
        onClick={onClick}
        title={title}
        style={{
            cursor: 'pointer', display: 'flex', alignItems: 'center', gap: '3px',
            fontSize: '10px', fontWeight: 600,
            padding: '2px 6px', borderRadius: '5px',
            color: active ? activeColor : 'rgba(255,255,255,0.35)',
            background: active ? `${activeColor}12` : 'transparent',
            border: active ? `1px solid ${activeColor}30` : '1px solid transparent',
            transition: 'all 0.1s',
        }}
    >
        <i className={`codicon codicon-${icon}`} style={{
            fontFamily: 'codicon', fontStyle: 'normal', fontSize: '11px',
        }} />
        {label && <span>{label}</span>}
    </div>
);

export default ChatToolbar;
