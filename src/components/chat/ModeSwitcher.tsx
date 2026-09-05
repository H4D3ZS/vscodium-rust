/**
 * Cursor-style agent mode switcher — Ask / Agent / Code toggle pills.
 * Clean, minimal design matching Cursor's mode selector.
 */
import React from 'react';

export interface AgentModeOption {
    id: string;
    label: string;
    icon: string;
    description: string;
    color: string;
}

export const AGENT_MODES: AgentModeOption[] = [
    { id: 'Chat', label: 'Ask', icon: 'comment', description: 'Ask questions, discuss code', color: '#34d399' },
    { id: 'Agent', label: 'Agent', icon: 'zap', description: 'Autonomous coding agent', color: '#3b82f6' },
    { id: 'Planning', label: 'Plan', icon: 'list-ordered', description: 'Plan before execution', color: '#f59e0b' },
    { id: 'BugBounty', label: 'Security', icon: 'shield', description: 'Offensive security research', color: '#ef4444' },
    { id: 'Fast', label: 'Fast', icon: 'rocket', description: 'Quick single-shot fixes', color: '#a78bfa' },
    { id: 'Verification', label: 'Verify', icon: 'verified', description: 'Run tests & checks', color: '#06b6d4' },
    { id: 'Sentient', label: 'Sentient', icon: 'heart', description: 'Full autonomous AIRI mode', color: '#ec4899' },
];

interface ModeSwitcherProps {
    currentMode: string;
    onSelect: (mode: string) => void;
    compact?: boolean;
}

const ModeSwitcher: React.FC<ModeSwitcherProps> = ({ currentMode, onSelect, compact }) => {
    const current = AGENT_MODES.find(m => m.id === currentMode) || AGENT_MODES[0];

    if (compact) {
        return (
            <div style={{
                display: 'flex', alignItems: 'center', gap: '2px',
                padding: '2px', borderRadius: '8px',
                background: 'rgba(255,255,255,0.04)',
                border: '1px solid rgba(255,255,255,0.06)',
            }}>
                {AGENT_MODES.slice(0, 4).map(mode => (
                    <div
                        key={mode.id}
                        onClick={() => onSelect(mode.id)}
                        title={`${mode.label} — ${mode.description}`}
                        style={{
                            padding: '3px 8px', borderRadius: '6px',
                            fontSize: '10px', fontWeight: 600,
                            cursor: 'pointer', transition: 'all 0.1s',
                            color: currentMode === mode.id ? mode.color : 'rgba(255,255,255,0.45)',
                            background: currentMode === mode.id ? `${mode.color}15` : 'transparent',
                            border: currentMode === mode.id ? `1px solid ${mode.color}30` : '1px solid transparent',
                            whiteSpace: 'nowrap',
                        }}
                    >
                        {mode.label}
                    </div>
                ))}
            </div>
        );
    }

    return (
        <div
            style={{
                position: 'absolute', bottom: '100%', left: 0,
                background: 'var(--vscode-menu-background, #1e1e2e)',
                border: '1px solid var(--vscode-menu-border, rgba(255,255,255,0.12))',
                borderRadius: '10px', overflow: 'hidden',
                boxShadow: '0 -8px 30px rgba(0,0,0,0.5)',
                zIndex: 200, marginBottom: '8px',
                width: '240px',
            }}
            onClick={(e) => e.stopPropagation()}
        >
            <div style={{
                padding: '8px 10px 4px', fontSize: '9px', fontWeight: 700,
                textTransform: 'uppercase', letterSpacing: '0.08em',
                color: 'rgba(255,255,255,0.4)',
            }}>
                Agent Mode
            </div>
            {AGENT_MODES.map(mode => {
                const isActive = currentMode === mode.id;
                return (
                    <div
                        key={mode.id}
                        onClick={() => { onSelect(mode.id); }}
                        style={{
                            padding: '7px 10px', cursor: 'pointer',
                            display: 'flex', alignItems: 'center', gap: '8px',
                            background: isActive ? `${mode.color}12` : 'transparent',
                            borderLeft: isActive ? `2px solid ${mode.color}` : '2px solid transparent',
                            transition: 'all 0.08s',
                        }}
                    >
                        <i
                            className={`codicon codicon-${mode.icon}`}
                            style={{
                                fontFamily: 'codicon', fontStyle: 'normal', fontSize: '14px',
                                color: isActive ? mode.color : 'rgba(255,255,255,0.4)',
                            }}
                        />
                        <div style={{ flex: 1 }}>
                            <div style={{
                                fontSize: '12px', fontWeight: isActive ? 600 : 400,
                                color: isActive ? '#fff' : 'rgba(255,255,255,0.75)',
                            }}>
                                {mode.label}
                            </div>
                            <div style={{
                                fontSize: '10px', color: 'rgba(255,255,255,0.35)',
                                marginTop: '1px',
                            }}>
                                {mode.description}
                            </div>
                        </div>
                        {isActive && (
                            <i className="codicon codicon-check" style={{
                                fontFamily: 'codicon', fontStyle: 'normal',
                                fontSize: '12px', color: mode.color,
                            }} />
                        )}
                    </div>
                );
            })}
        </div>
    );
};

export default ModeSwitcher;
