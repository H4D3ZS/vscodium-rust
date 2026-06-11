// Agent status strips: phase roadmap + reasoning-budget toggle.
// Extracted from RightSidebar.tsx (A2 god-component split).
import React, { useEffect, useState, useRef, useMemo, useCallback, memo } from 'react';
import { useStore } from '../../store';
import { invoke } from '../../tauri_bridge';


const TaskRoadmap: React.FC = () => {
    const currentPhase = useStore(state => state.currentPhase);
    const status = useStore(state => state.currentPhaseStatus);
    const isThinking = useStore(state => state.isAgentThinking);

    if (currentPhase === 'IDLE' || !isThinking) return null;

    const phases = ['ANALYZE', 'PLAN', 'EXECUTE', 'VERIFY', 'REPORT'];
    const activeIndex = phases.indexOf(currentPhase);

    return (
        <div className="task-roadmap" style={{
            margin: '8px 10px',
            padding: '10px 12px',
            background: 'var(--vscode-list-hoverBackground, rgba(255,255,255,0.04))',
            border: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.12))',
            borderRadius: '6px',
            animation: 'fadeIn 0.3s ease-out'
        }}>
            <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '8px' }}>
                {phases.map((p, i) => (
                    <div key={p} style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', gap: '3px', flex: 1, position: 'relative' }}>
                        <div style={{
                            width: '18px',
                            height: '18px',
                            borderRadius: '50%',
                            background: i <= activeIndex ? 'var(--vscode-focusBorder, #007acc)' : 'var(--vscode-panel-border)',
                            color: i <= activeIndex ? '#000' : 'var(--vscode-foreground)',
                            display: 'flex',
                            alignItems: 'center',
                            justifyContent: 'center',
                            fontSize: '9px',
                            fontWeight: 700,
                            zIndex: 2,
                            boxShadow: i === activeIndex ? '0 0 6px var(--vscode-focusBorder, #007acc)' : 'none'
                        }}>
                            {i < activeIndex ? '✓' : i + 1}
                        </div>
                        <span style={{ fontSize: '7.5px', fontWeight: 600, opacity: i <= activeIndex ? 1 : 0.4 }}>{p}</span>
                        {i < phases.length - 1 && (
                            <div style={{
                                position: 'absolute',
                                left: '50%',
                                top: '9px',
                                width: '100%',
                                height: '2px',
                                background: i < activeIndex ? 'var(--vscode-focusBorder, #007acc)' : 'var(--vscode-panel-border)',
                                zIndex: 1
                            }} />
                        )}
                    </div>
                ))}
            </div>
            <div style={{ fontSize: '10px', display: 'flex', alignItems: 'center', gap: '6px' }}>
                <i className="codicon codicon-loading codicon-modifier-spin" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: 12, color: 'var(--vscode-focusBorder, #007acc)' }} />
                <span style={{ fontWeight: 600, fontSize: '9.5px', color: 'rgba(255,255,255,0.85)' }}>{status}</span>
            </div>
        </div>
    );
};


// ── Background agents tray ───────────────────────────────────────────────
// ── Void: Reasoning toggle + budget slider ────────────────────────────────
const ReasoningToggle: React.FC = () => {
    const isEnabled = useStore((s: any) => s.isReasoningEnabled ?? false);
    const setEnabled = useStore((s: any) => s.setIsReasoningEnabled);
    const budget = useStore((s: any) => s.currentReasoningBudget ?? 1024);
    const setBudget = useStore((s: any) => s.setCurrentReasoningBudget);
    const effort = useStore((s: any) => s.currentReasoningEffort ?? 'low');
    const setEffort = useStore((s: any) => s.setCurrentReasoningEffort);
    const model = useStore((s: any) => s.agentModel ?? '');
    const [open, setOpen] = React.useState(false);

    const ml = model.toLowerCase();
    const isThinkTag = ml.includes('qwen3') || ml.includes('qwq') || ml.includes('deepseek-r1') || ml.includes('r1:');
    const isAnthropicModel = ml.includes('anthropic') || ml.includes('claude');
    const isOpenAI = ml.includes('openai') || ml.includes('gpt') || ml.includes('o1') || ml.includes('o3');
    const supportsReasoning = isThinkTag || isAnthropicModel || isOpenAI;

    if (!supportsReasoning) return null;

    return (
        <div style={{ position: 'relative' }}>
            <span
                onClick={() => { setEnabled(!isEnabled); if (!isEnabled) setOpen(true); }}
                title={isEnabled ? 'Reasoning ON — click to toggle' : 'Enable reasoning / extended thinking'}
                style={{
                    fontSize: '10px', fontWeight: 600, cursor: 'pointer',
                    padding: '1px 6px', borderRadius: '4px',
                    color: isEnabled ? '#818cf8' : 'rgba(255,255,255,0.3)',
                    background: isEnabled ? 'rgba(99,102,241,0.15)' : 'transparent',
                    border: isEnabled ? '1px solid rgba(99,102,241,0.4)' : '1px solid rgba(255,255,255,0.08)',
                    transition: 'all 0.15s',
                    userSelect: 'none',
                }}
            >
                {isThinkTag ? '🧠 Think' : isAnthropicModel ? '💡 Thinking' : '⚡ Reason'}
            </span>
            {isEnabled && open && (
                <div style={{
                    position: 'absolute', bottom: '26px', left: 0, zIndex: 100,
                    background: '#1e1e2e', border: '1px solid rgba(99,102,241,0.3)',
                    borderRadius: '8px', padding: '10px 12px', minWidth: '200px',
                    boxShadow: '0 4px 20px rgba(0,0,0,0.4)',
                }}>
                    <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '8px' }}>
                        <span style={{ fontSize: '11px', fontWeight: 600, color: '#818cf8' }}>Reasoning Config</span>
                        <i className="codicon codicon-close" onClick={() => setOpen(false)} style={{ fontFamily: 'codicon', fontStyle: 'normal', cursor: 'pointer', opacity: 0.5, fontSize: '11px' }} />
                    </div>
                    {(isAnthropicModel) && (
                        <div style={{ marginBottom: '8px' }}>
                            <div style={{ fontSize: '10px', opacity: 0.6, marginBottom: '4px' }}>Budget tokens: {budget}</div>
                            <input type="range" min={1024} max={32000} step={1024} value={budget}
                                onChange={e => setBudget(parseInt(e.target.value))}
                                style={{ width: '100%', accentColor: '#818cf8' }} />
                        </div>
                    )}
                    {(isOpenAI) && (
                        <div style={{ marginBottom: '8px' }}>
                            <div style={{ fontSize: '10px', opacity: 0.6, marginBottom: '4px' }}>Effort</div>
                            {(['low', 'medium', 'high'] as const).map(e => (
                                <span key={e} onClick={() => setEffort(e)} style={{
                                    marginRight: '6px', fontSize: '10px', cursor: 'pointer', fontWeight: 600,
                                    padding: '1px 6px', borderRadius: '4px',
                                    color: effort === e ? '#818cf8' : 'rgba(255,255,255,0.5)',
                                    background: effort === e ? 'rgba(99,102,241,0.15)' : 'transparent',
                                    border: effort === e ? '1px solid rgba(99,102,241,0.4)' : '1px solid transparent',
                                }}>{e}</span>
                            ))}
                        </div>
                    )}
                    {isThinkTag && (
                        <div style={{ fontSize: '10px', opacity: 0.55 }}>Think-tag model — reasoning output will appear in the thinking trace above the response.</div>
                    )}
                </div>
            )}
        </div>
    );
};

export { TaskRoadmap };
export { ReasoningToggle };
