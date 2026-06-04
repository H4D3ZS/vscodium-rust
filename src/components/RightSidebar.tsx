import React, { useEffect, useState, useRef, useMemo, useCallback, Suspense, lazy } from 'react';
import { marked } from 'marked';
import { airiBiology } from '../airi/biology';
import { airiConsciousness } from '../airi/consciousness';
import { useStore } from '../store';
import type { FileEntry } from '../store';
import { invoke } from '../tauri_bridge';
import MissionControl from './agent/MissionControl';
import CheckpointTimeline from './CheckpointTimeline';
import SentientAvatar from './agent/SentientAvatar';
import type { AvatarState } from './agent/SentientAvatar';
import { initTTS as initVoiceSystem, speak, stop, isSpeaking as isTtsSpeaking, getProvider } from '../voice';
import MessageBody from './agent/MessageBody';
import { Check, Activity, Mic, MicOff, Volume2, VolumeX } from 'lucide-react';

// Lazy-load heavy panels — only load Three.js/VRM/Emulator code when the
// user actually opens those tabs. Keeps initial renderer RAM under 250MB.
const AiriPanel = lazy(() => import('./AiriPanel').then(m => ({ default: m.AiriPanel })));
const ResearchCenter = lazy(() => import('./agent/ResearchCenter'));
const UnifiedEmulatorPanel = lazy(() => import('./UnifiedEmulatorPanel'));
const SpecsManager = lazy(() => import('./SpecsManager'));
const RulesManager = lazy(() => import('./RulesManager'));


// ── Restore-checkpoint banner ────────────────────────────────────────────
// Shows above the chat input whenever an agent turn just auto-snapshotted
// the workspace, giving the user a one-click "undo the AI's edits" path.
// ── Plan approval banner ──────────────────────────────────────────────────
// Appears when the agent outputs AWAITING_APPROVAL in plan mode.
// User reviews the plan and clicks Approve to resume execution.
const PlanApprovalBanner: React.FC = () => {
    const [planData, setPlanData] = React.useState<{ plan: string; iteration: number } | null>(null);
    const [approving, setApproving] = React.useState(false);

    React.useEffect(() => {
        import('@tauri-apps/api/event').then(({ listen }) => {
            const unlisten = listen('plan-approval-required', (event: any) => {
                setPlanData(event.payload ?? null);
            });
            return () => { unlisten.then(f => f()); };
        });
    }, []);

    if (!planData) return null;

    const handleApprove = async () => {
        setApproving(true);
        try { await (await import('../tauri_bridge')).invoke('resume_ai_agent'); } catch { /* ignore */ }
        setPlanData(null);
        setApproving(false);
    };

    const handleReject = async () => {
        try { await (await import('../tauri_bridge')).invoke('stop_ai_agent'); } catch { /* ignore */ }
        setPlanData(null);
    };

    return (
        <div style={{
            display: 'flex', flexDirection: 'column', gap: 8,
            padding: '10px 12px', marginBottom: 6,
            background: 'rgba(245,158,11,0.07)',
            border: '1px solid rgba(245,158,11,0.3)',
            borderRadius: 8, fontSize: 11,
        }}>
            <div style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
                <span style={{ fontSize: 13 }}>📋</span>
                <strong style={{ color: '#fbbf24' }}>Plan ready — approve to execute</strong>
            </div>
            {planData.plan && (
                <pre style={{
                    margin: 0, padding: '6px 10px',
                    background: 'rgba(0,0,0,0.3)', borderRadius: 5,
                    fontSize: 10, color: 'rgba(255,255,255,0.75)',
                    maxHeight: 180, overflowY: 'auto', whiteSpace: 'pre-wrap',
                }}>
                    {planData.plan}
                </pre>
            )}
            <div style={{ display: 'flex', gap: 8 }}>
                <button
                    disabled={approving}
                    onClick={handleApprove}
                    style={{
                        flex: 1, padding: '5px 10px', fontSize: 11, fontWeight: 700, borderRadius: 5,
                        background: '#d97706', color: 'var(--vscode-editor-foreground, #fff)', border: 'none', cursor: 'pointer',
                    }}
                >
                    {approving ? '…' : '✓ Approve & Execute'}
                </button>
                <button
                    onClick={handleReject}
                    style={{
                        padding: '5px 10px', fontSize: 11, fontWeight: 700, borderRadius: 5,
                        background: 'rgba(248,113,113,0.15)', color: '#f87171',
                        border: '1px solid rgba(248,113,113,0.3)', cursor: 'pointer',
                    }}
                >
                    ✕ Cancel
                </button>
            </div>
        </div>
    );
};

const RestoreCheckpointBanner: React.FC = () => {
    const checkpoint = useStore(state => state.lastAgentCheckpoint);
    const rollback = useStore(state => state.rollbackLastAgentCheckpoint);
    const dismiss = useStore(state => state.setLastAgentCheckpoint);
    const [busy, setBusy] = useState(false);
    const [msg, setMsg] = useState<string | null>(null);
    if (!checkpoint) return null;
    const age = Math.max(1, Math.round((Date.now() - checkpoint.timestamp) / 1000));
    return (
        <div style={{
            display: 'flex', alignItems: 'center', gap: 8,
            padding: '6px 10px', marginBottom: 6,
            background: 'rgba(168,85,247,0.08)',
            border: '1px solid rgba(168,85,247,0.25)',
            borderRadius: 8, fontSize: 11,
        }}>
            <i className="codicon codicon-discard" style={{ fontFamily: 'codicon', fontStyle: 'normal', color: '#c084fc', fontSize: 13 }} />
            <span style={{ flex: 1, color: 'rgba(255,255,255,0.85)' }}>
                {msg ?? <>Checkpoint <code style={{ opacity: 0.7 }}>{checkpoint.description}</code> · {age}s ago</>}
            </span>
            <button
                disabled={busy}
                onClick={async () => {
                    setBusy(true);
                    const r = await rollback();
                    setBusy(false);
                    setMsg(r.ok ? 'Restored.' : r.message);
                    if (r.ok) setTimeout(() => setMsg(null), 1800);
                }}
                style={{ background: '#c084fc', color: '#000', border: 'none', padding: '2px 8px', fontSize: 10, fontWeight: 600, borderRadius: 4, cursor: busy ? 'wait' : 'pointer' }}
            >
                {busy ? '…' : '↶ Restore'}
            </button>
            <i
                className="codicon codicon-close"
                onClick={() => dismiss(null)}
                style={{ fontFamily: 'codicon', fontStyle: 'normal', cursor: 'pointer', opacity: 0.5, fontSize: 11 }}
                title="Dismiss"
            />
        </div>
    );
};

// ── Multi-file review banner ──────────────────────────────────────────────
// Appears after the agent's turn whenever it touched 2+ files. Clicking
// opens the MultiFileReview carousel where the user can step through each
// diff and keep/revert per file.
const MultiFileReviewBanner: React.FC = () => {
    const edits = useStore(s => s.pendingAgentEdits);
    const isThinking = useStore(s => s.isAgentThinking);
    const openReview = useStore(s => s.openMultiFileReview);
    // Only show after the turn finishes, with 2+ files. Single-file edits
    // are usually obvious and don't warrant a modal.
    if (isThinking) return null;
    if (edits.length < 2) return null;
    return (
        <div
            onClick={openReview}
            style={{
                display: 'flex', alignItems: 'center', gap: 8,
                padding: '6px 10px', marginBottom: 6,
                background: 'rgba(34,197,94,0.08)',
                border: '1px solid rgba(34,197,94,0.25)',
                borderRadius: 8, fontSize: 11,
                cursor: 'pointer',
            }}
        >
            <i className="codicon codicon-diff-multiple" style={{ fontFamily: 'codicon', fontStyle: 'normal', color: '#4ade80', fontSize: 13 }} />
            <span style={{ flex: 1, color: 'rgba(255,255,255,0.85)' }}>
                Agent edited <b>{edits.length}</b> files — review the diff
            </span>
            <span style={{
                background: '#4ade80', color: '#000', padding: '2px 8px', fontSize: 10,
                fontWeight: 600, borderRadius: 4,
            }}>Review</span>
        </div>
    );
};

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
            background: 'rgba(168,85,247,0.04)',
            border: '1px solid rgba(168,85,247,0.15)',
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
                            background: i <= activeIndex ? '#c084fc' : 'var(--vscode-panel-border)',
                            color: i <= activeIndex ? '#000' : 'var(--vscode-foreground)',
                            display: 'flex',
                            alignItems: 'center',
                            justifyContent: 'center',
                            fontSize: '9px',
                            fontWeight: 700,
                            zIndex: 2,
                            boxShadow: i === activeIndex ? '0 0 6px #c084fc' : 'none'
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
                                background: i < activeIndex ? '#c084fc' : 'var(--vscode-panel-border)',
                                zIndex: 1
                            }} />
                        )}
                    </div>
                ))}
            </div>
            <div style={{ fontSize: '10px', display: 'flex', alignItems: 'center', gap: '6px' }}>
                <Activity size={12} style={{ color: '#c084fc' }} className="animate-pulse" />
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

// Compact strip listing any agent runs the user fired with `/bg <prompt>`
// or via `runBackgroundAgent`. Doesn't block the main chat.
const BackgroundAgentsTray: React.FC = () => {
    const bgAgents = useStore(state => state.backgroundAgents);
    const remove = useStore(state => state.removeBackgroundAgent);
    const runBackground = useStore(state => state.runBackgroundAgent);
    const clearAll = useStore(state => state.clearBackgroundAgents);
    const [expandedId, setExpandedId] = useState<string | null>(null);
    const [spawnPrompt, setSpawnPrompt] = useState('');
    const [spawning, setSpawning] = useState(false);
    const [showSpawn, setShowSpawn] = useState(false);

    const handleSpawn = async () => {
        if (!spawnPrompt.trim()) return;
        setSpawning(true);
        await runBackground(spawnPrompt.trim()).catch(() => {});
        setSpawnPrompt('');
        setSpawning(false);
        setShowSpawn(false);
    };

    const running = bgAgents.filter(b => b.status === 'running').length;
    const done = bgAgents.filter(b => b.status === 'done' || b.status === 'error').length;

    // Parallel/background agents are HIDDEN for now — not production-ready. Early-return
    // is placed AFTER all hooks (React rules). Flip the flag (or move to Settings) later.
    const PARALLEL_AGENTS_ENABLED = false;
    if (!PARALLEL_AGENTS_ENABLED) return null;

    return (
        <div style={{
            marginBottom: 6, padding: 6,
            background: 'rgba(96,165,250,0.06)',
            border: '1px solid rgba(96,165,250,0.2)',
            borderRadius: 8, fontSize: 11,
        }}>
            {/* Header with spawn button */}
            <div style={{ display: 'flex', alignItems: 'center', gap: 6, marginBottom: 4 }}>
                <i className="codicon codicon-cloud" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: 12, opacity: 0.7 }} />
                <span style={{ fontWeight: 600, flex: 1, opacity: 0.7 }}>
                    Parallel agents
                    {running > 0 && <span style={{ marginLeft: 4, color: '#60a5fa' }}>({running} running)</span>}
                </span>
                {done > 0 && (
                    <span onClick={clearAll} style={{ cursor: 'pointer', fontSize: 9, opacity: 0.5 }} title="Clear finished">clear</span>
                )}
                <span
                    onClick={() => setShowSpawn(v => !v)}
                    style={{ cursor: 'pointer', fontSize: 16, lineHeight: 1, color: '#60a5fa', fontWeight: 300 }}
                    title="Spawn new background agent"
                >⊕</span>
            </div>
            {/* Spawn input */}
            {showSpawn && (
                <div style={{ display: 'flex', gap: 4, marginBottom: 6 }}>
                    <input
                        autoFocus
                        value={spawnPrompt}
                        onChange={e => setSpawnPrompt(e.target.value)}
                        onKeyDown={e => { if (e.key === 'Enter') handleSpawn(); if (e.key === 'Escape') setShowSpawn(false); }}
                        placeholder="Task for background agent…"
                        style={{
                            flex: 1, fontSize: 10, padding: '3px 6px',
                            background: 'rgba(0,0,0,0.3)', border: '1px solid rgba(96,165,250,0.3)',
                            borderRadius: 3, color: 'var(--vscode-editor-foreground, #fff)', outline: 'none',
                        }}
                    />
                    <button
                        disabled={spawning || !spawnPrompt.trim()}
                        onClick={handleSpawn}
                        style={{ padding: '3px 8px', fontSize: 10, fontWeight: 700, background: '#1e3a5f', border: '1px solid #2563eb', borderRadius: 3, color: '#60a5fa', cursor: 'pointer' }}
                    >{spawning ? '…' : '▶'}</button>
                </div>
            )}
            {bgAgents.length === 0 && !showSpawn && (
                <div style={{ fontSize: 10, opacity: 0.35, textAlign: 'center', padding: '3px 0' }}>
                    Click ⊕ to spawn a parallel agent
                </div>
            )}
            <div style={{ display: 'flex', flexDirection: 'column', gap: 4 }}>
                {bgAgents.map(bg => {
                    const open = expandedId === bg.id;
                    const color = bg.status === 'done' ? '#22c55e' : bg.status === 'error' ? '#f87171' : '#60a5fa';
                    return (
                        <div key={bg.id} style={{ display: 'flex', flexDirection: 'column', gap: 2, padding: '4px 6px', background: 'rgba(0,0,0,0.2)', borderRadius: 4 }}>
                            <div style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
                                <span style={{ width: 6, height: 6, borderRadius: '50%', background: color, flexShrink: 0, ...(bg.status === 'running' ? { animation: 'hubPulse 1s infinite' } : {}) }} />
                                <span style={{ flex: 1, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }} title={bg.prompt}>{bg.prompt}</span>
                                {(() => {
                                    const end = bg.finishedAt ?? Date.now();
                                    const secs = Math.max(0, Math.round((end - bg.startedAt) / 1000));
                                    return <span style={{ opacity: 0.4, fontSize: 9 }}>{secs}s</span>;
                                })()}
                                <span style={{ opacity: 0.5, fontSize: 9 }}>{bg.status}</span>
                                <i
                                    className={`codicon codicon-${open ? 'chevron-up' : 'chevron-down'}`}
                                    onClick={() => setExpandedId(open ? null : bg.id)}
                                    style={{ fontFamily: 'codicon', fontStyle: 'normal', cursor: 'pointer', fontSize: 11, opacity: 0.6 }}
                                />
                                <i
                                    className="codicon codicon-close"
                                    onClick={() => remove(bg.id)}
                                    style={{ fontFamily: 'codicon', fontStyle: 'normal', cursor: 'pointer', fontSize: 11, opacity: 0.6 }}
                                />
                            </div>
                            {open && bg.result && (
                                <pre style={{ margin: 0, marginTop: 2, fontSize: 10, opacity: 0.85, maxHeight: 160, overflow: 'auto', whiteSpace: 'pre-wrap' }}>
                                    {bg.result.slice(0, 4000)}
                                </pre>
                            )}
                        </div>
                    );
                })}
            </div>
        </div>
    );
};

import OllamaProgressBar from './OllamaProgressBar';
import EmulatorPanel from './EmulatorPanel';

// One-shot AIRI bootstrap latch. Module-scoped so it survives unmount/remount
// and (critically) React.StrictMode's deliberate double-invoke of effects.
const airiInitOnce: { started: boolean } = { started: false };

/**
 * Strips raw tool-call JSON/XML from AI content so the user sees only
 * natural language. The backend still processes the full JSON — this is
 * purely a display transform.
 */
/** Returns true if a JSON string looks like a tool call object */
function isToolCallJson(text: string): boolean {
    try {
        const t = text.trim();
        if (!t.startsWith('{') && !t.startsWith('[')) return false;
        const parsed = JSON.parse(t);
        if (Array.isArray(parsed)) return parsed.some(isToolCallJson);
        if (parsed && typeof parsed === 'object') {
            return ('name' in parsed && ('arguments' in parsed || 'parameters' in parsed || 'input' in parsed))
                || 'tool_calls' in parsed
                || 'function_call' in parsed;
        }
    } catch { /* not valid JSON */ }
    return false;
}

/**
 * Strips raw tool-call JSON/XML from AI content so the user sees only
 * natural language. The backend still processes the full JSON — this is
 * purely a display transform.
 */
function cleanAiContent(raw: string): string {
    if (!raw) return '';
    let s = raw;

    // Strip XML tool-call tags (Qwen / DeepSeek style)
    s = s.replace(/<tool_call>[\s\S]*?<\/tool_call>/g, '');
    s = s.replace(/<function_calls>[\s\S]*?<\/function_calls>/g, '');
    s = s.replace(/<function>[\s\S]*?<\/function>/g, '');
    s = s.replace(/<invoke>[\s\S]*?<\/invoke>/g, '');

    // Strip markdown fenced code blocks that contain tool-call JSON
    // Use block parser to correctly handle nested braces
    s = s.replace(/```[a-z]*\n([\s\S]*?)```/gi, (match, inner) => {
        return isToolCallJson(inner.trim()) ? '' : match;
    });
    // Also handle ``` without language tag followed immediately by {
    s = s.replace(/```\s*(\{[\s\S]*?\})\s*```/g, (match, inner) => {
        return isToolCallJson(inner) ? '' : match;
    });

    // Strip bare JSON tool-call lines (outside code blocks)
    s = s.split('\n').filter(line => {
        const t = line.trim();
        return !isToolCallJson(t);
    }).join('\n');

    // Strip SEARCH/REPLACE edit blocks (code diffs, not conversation)
    s = s.replace(/<<<< SEARCH[\s\S]*?>>>>/g, '');
    s = s.replace(/<<<<<<[\s\S]*?>>>>>>>/g, '');

    // Strip MISSION_ACCOMPLISHED marker
    s = s.replace(/MISSION_ACCOMPLISHED/g, '');
    s = s.replace(/TASK_COMPLETE/g, '');

    // Strip degenerate LaTeX letter-spam that some abliterated/uncensored Ollama
    // tunes emit ($\text{N}$ $\text{I}$ …). This is junk output, not real math —
    // collapse runs of single-letter \text{X}/\mathit{X}/\mathrm{X} into the
    // bare letters and drop the surrounding whitespace runs.
    s = s.replace(/\$\s*\\(?:text|mathit|mathrm|mathbf|mathcal|mathsf|mathtt)\{([^{}]{1,6})\}\s*\$/g,
        (_m, inner) => String(inner));
    s = s.replace(/(?:^|[^\w])\$([A-Za-z0-9])\$(?=[^\w]|$)/g, (_m, ch) => ch);
    // Three or more single-letter tokens with whitespace between them → collapse.
    s = s.replace(/(?:\b[A-Za-z]\b\s+){3,}/g, (m) => m.replace(/\s+/g, ''));

    // Clean up excessive blank lines
    s = s.replace(/\n{3,}/g, '\n\n').trim();
    return s;
}

const SidebarPane: React.FC<{ title: string; children: React.ReactNode; defaultCollapsed?: boolean; actions?: React.ReactNode }> = ({ title, children, defaultCollapsed = false, actions }) => {
    const [isCollapsed, setIsCollapsed] = useState(defaultCollapsed);
    return (
        <div className="sidebar-pane" style={{ display: 'flex', flexDirection: 'column', flexShrink: 0, borderBottom: '1px solid var(--vscode-sideBar-border, rgba(255,255,255,0.05))' }}>
            <div
                className={`pane-header${isCollapsed ? ' collapsed' : ''}`}
                onClick={() => setIsCollapsed(!isCollapsed)}
                style={{
                    padding: '6px 10px',
                    display: 'flex',
                    alignItems: 'center',
                    cursor: 'pointer',
                    background: 'var(--vscode-sideBarSectionHeader-background, rgba(255,255,255,0.02))',
                    fontSize: '11px',
                    fontWeight: 600,
                    textTransform: 'uppercase',
                    letterSpacing: '0.05em',
                    color: 'var(--vscode-sideBar-foreground)',
                    opacity: 0.8
                }}
            >
                <i className={`codicon codicon-chevron-${isCollapsed ? 'right' : 'down'}`} style={{ fontFamily: 'codicon', fontStyle: 'normal', marginRight: '8px', fontSize: '12px' }}></i>
                <span style={{ flex: 1 }}>{title}</span>
                {actions && <div className="pane-actions" onClick={e => e.stopPropagation()}>{actions}</div>}
            </div>
            {!isCollapsed && <div className="pane-content" style={{ padding: '8px 0' }}>{children}</div>}
        </div>
    );
};

marked.setOptions({
    gfm: true,
    breaks: true,
    silent: true
});



const RightSidebar: React.FC = () => {
    const isOpen = useStore(state => state.isRightSidebarOpen);
    const toggle = useStore(state => state.toggleRightSidebar);
    const isEmulatorPanelOpen = useStore(state => state.isEmulatorPanelOpen);
    const isAiriPanelOpen = useStore(state => state.isAiriPanelOpen);
    const aiStatus = useStore(state => state.aiStatus || 'idle');
    // 'settings' is no longer a right-sidebar view — the gear opens the
    // unified Settings tab in the editor pane instead. We keep the union
    // narrow so renaming the right-sidebar views stays cheap.
    const [view, setView] = useState<'chat' | 'emulator' | 'history' | 'dashboard' | 'research' | 'kortex' | 'specs' | 'rules' | 'manager'>('chat');
    const inputRef = useRef<HTMLTextAreaElement>(null);
    const mode = useStore(state => state.agentMode);
    const model = useStore(state => state.agentModel);
    const webUiProviderKey = useMemo(() => {
        const lower = String(model || '').toLowerCase();
        if (!lower.includes('webui') && !lower.includes('openwebui')) return '';
        const rawProvider = model.includes('|') ? model.split('|')[0] : model;
        const p = rawProvider.toLowerCase();
        if (p.includes('openwebui')) return 'openwebui';
        return p
            .replace(' (webui)', '')
            .replace('-webui', '')
            .replace('webui', '')
            .split(':')[0]
            .trim() || 'openai';
    }, [model]);

    const webuiSessions = useStore(state => state.webuiSessions);
    const activeWebuiSessionId = useStore(state => state.activeWebuiSessionId);
    const refreshWebuiSessions = useStore(state => state.refreshWebuiSessions);
    const switchWebuiSession = useStore(state => state.switchWebuiSession);
    const deleteWebuiSession = useStore(state => state.deleteWebuiSession);

    useEffect(() => {
        if (webUiProviderKey) {
            refreshWebuiSessions(webUiProviderKey);
            const interval = setInterval(() => {
                refreshWebuiSessions(webUiProviderKey);
            }, 5000);
            return () => clearInterval(interval);
        }
    }, [webUiProviderKey, refreshWebuiSessions]);

    useEffect(() => {
        const handler = (e: Event) => {
            const customEvent = e as CustomEvent;
            if (customEvent.detail && customEvent.detail.view) {
                setView(customEvent.detail.view);
            }
        };
        window.addEventListener('right-sidebar:set-view', handler);
        return () => window.removeEventListener('right-sidebar:set-view', handler);
    }, []);
    const messages = useStore(state => state.agentMessages);
    const isAgentThinking = useStore(state => state.isAgentThinking);
    const isAgentPaused = useStore(state => state.isAgentPaused);
    const isYoloMode = useStore(state => state.isYoloMode);
    const setYoloMode = useStore(state => state.setYoloMode);
    const isContinuousMode = useStore(state => state.isContinuousMode);
    const setContinuousMode = useStore(state => state.setContinuousMode);
    const agentUiMode = useStore(state => state.agentUiMode);
    const setAgentUiMode = useStore(state => state.setAgentUiMode);
    const avatarCharacter = useStore(state => state.avatarCharacter);
    const showVrmAvatar = useStore(state => state.showVrmAvatar);
    const isSpecModeActive = useStore(state => state.isSpecModeActive);
    const setSpecModeActive = useStore(state => state.setSpecModeActive);
    const setSpecsPrompt = useStore(state => state.setSpecsPrompt);
    // AIRI subsystem toggles surfaced in the sidebar so the user can flip
    // vision / consciousness without digging into localStorage.
    const airiVisionEnabled = useStore(state => state.airiVisionEnabled);
    const setAiriVisionEnabled = useStore(state => state.setAiriVisionEnabled);
    const airiVisionModel = useStore(state => state.airiVisionModel);
    const setAiriVisionModel = useStore(state => state.setAiriVisionModel);
    const airiConsciousnessEnabled = useStore(state => state.airiConsciousnessEnabled);
    const setAiriConsciousnessEnabled = useStore(state => state.setAiriConsciousnessEnabled);
    const airiConsciousnessModel = useStore(state => state.airiConsciousnessModel);
    const setAiriConsciousnessModel = useStore(state => state.setAiriConsciousnessModel);
    const [airiToggleOpen, setAiriToggleOpen] = useState(false);
    const addAgentMessage = useStore(state => state.addAgentMessage);
    const updateLastAgentMessage = useStore(state => state.updateLastAgentMessage);
    const setIsAgentThinking = useStore(state => state.setIsAgentThinking);
    const clearAgentMessages = useStore(state => state.clearAgentMessages);
    const resetThread = useStore(state => state.resetThread);
    const pendingChanges = useStore(state => state.pendingChanges);
    const truncateAgentMessages = useStore(state => state.truncateAgentMessages);
    const [sessionAge, setSessionAge] = useState<string>('');
    const attachedFiles = useStore(state => state.attachedFiles);
    const attachFile = useStore(state => state.attachFile);
    const removeFile = useStore(state => state.removeFile);
    const clearAttachedFiles = useStore(state => state.clearAttachedFiles);
    const agentRootAccess = useStore(state => state.agentRootAccess);
    const setAgentRootAccess = useStore(state => state.setAgentRootAccess);
    const fileTree = useStore(state => state.fileTree);
    const messagesEndRef = useRef<HTMLDivElement>(null);
    const agentTasks = useStore(state => state.agentTasks);
    const chatSessions = useStore(state => state.chatSessions);
    const refreshChatSessions = useStore(state => state.refreshChatSessions);
    const loadChatSession = useStore(state => state.loadChatSession);
    const archiveCurrentSession = useStore(state => state.archiveCurrentSession);
    const createNewSession = useStore(state => state.createNewSession);
    const availableModels = useStore(state => state.availableModels);
    const autoAcceptChanges = useStore(state => state.autoAcceptChanges);
    const setAutoAcceptChanges = useStore(state => state.setAutoAcceptChanges);
    const checkpoint = useStore(state => state.checkpoint);
    const revertToCheckpoint = useStore(state => state.revertToCheckpoint);
    const snapshotCheckpoint = useStore(state => state.snapshotCheckpoint);

    useEffect(() => {
        if (view === 'history') {
            refreshChatSessions();
        }
    }, [view, refreshChatSessions]);

    // Removed old localstorage WebUI account sync

    useEffect(() => {
        if (isEmulatorPanelOpen && view !== 'emulator') {
            setView('emulator');
        } else if (isAiriPanelOpen && !isEmulatorPanelOpen && view === 'emulator') {
            setView('chat');
        }
    }, [isEmulatorPanelOpen, isAiriPanelOpen, view]);
    const [inputValue, setInputValue] = useState('');
    const [isMentionDropdownOpen, setIsMentionDropdownOpen] = useState(false);
    const [selectedMentionIndex, setSelectedMentionIndex] = useState(0);
    const [isHelpOpen, setIsHelpOpen] = useState(false);

    // Phase 7: Chat Editing & Copying State
    const [editingMsgIdx, setEditingMsgIdx] = useState<number | null>(null);
    const [editValue, setEditValue] = useState('');
    const [lastCopiedIdx, setLastCopiedIdx] = useState<number | null>(null);

    // AIRI full mode — last spoken response and speech state
    const [airiSpeech, setAiriSpeech] = useState<string>('');
    const [airiSpeaking, setAiriSpeaking] = useState(false);
    const [ttsEnabled, setTtsEnabled] = useState(true); // Enable by default
    const [ttsPreset, setTtsPreset] = useState<'airi' | 'sage' | 'nova' | 'kawaii' | 'yamato' | 'hana' | 'ren' | 'yuki' | 'haru' | 'sora' | 'zero' | 'aria'>('airi');
    const [isVoiceListening, setIsVoiceListening] = useState(false);
    const recognitionRef = useRef<any>(null);

    const toggleVoiceInput = () => {
        if (isVoiceListening) {
            stopVoiceInput();
        } else {
            startVoiceInput();
        }
    };

    const startVoiceInput = () => {
        stop(); // Stop agent speaking before user speaks
        const SpeechRecognition = (window as any).SpeechRecognition || (window as any).webkitSpeechRecognition;
        if (!SpeechRecognition) {
            console.warn('[VRD] Web Speech Recognition not supported in this environment.');
            return;
        }

        const recognition = new SpeechRecognition();
        recognition.continuous = false;
        recognition.interimResults = true;
        recognition.lang = 'en-US';

        recognition.onstart = () => {
            setIsVoiceListening(true);
        };

        recognition.onresult = (event: any) => {
            const transcript = Array.from(event.results)
                .map((result: any) => result[0].transcript)
                .join('');
            setInputValue(transcript);
        };

        recognition.onend = () => {
            setIsVoiceListening(false);
            setTimeout(() => {
                const text = inputRef.current?.value || '';
                if (text.trim()) {
                    onSend(text);
                }
            }, 200);
        };

        recognition.onerror = (e: any) => {
            console.error('[VRD] Recognition error:', e);
            setIsVoiceListening(false);
        };

        recognitionRef.current = recognition;
        recognition.start();
    };

    const stopVoiceInput = () => {
        if (recognitionRef.current) {
            recognitionRef.current.stop();
        }
        setIsVoiceListening(false);
    };
    const [airiEmotion, setAiriEmotion] = useState<'neutral' | 'happy' | 'thinking' | 'excited' | 'concerned'>('neutral');
    const [digitalLifeActive, setDigitalLifeActive] = useState(false);

    // Emulator panel positioning
    const emulatorLayout = useStore(state => state.emulatorLayout);
    const [showEmulatorInRight, setShowEmulatorInRight] = useState(false);

    // Initialize TTS on mount.
    // Guarded so React.StrictMode's intentional double-invoke (and any
    // remount of <RightSidebar>) doesn't fire the entire AIRI stack twice
    // — that was why every "✅ … ACTIVE" line showed up twice and why the
    // greeting played twice, two cognitive intervals were registered, etc.
    useEffect(() => {
        if (airiInitOnce.started) {
            return;
        }
        airiInitOnce.started = true;

        // ── AIRI "companion" stack — OPT-IN (default OFF) ─────────────────────
        // Voice/TTS, cognitive-core (5s loop), digital-life (+ spoken greeting),
        // consciousness, biology, and the security threat-monitor were loading +
        // running on EVERY launch — inflating memory and startup with zero benefit
        // to core IDE/agent coding. The editor, agent chat, and terminal do not
        // need any of it. Enable with localStorage 'airi.companion' = '1'.
        if (localStorage.getItem('airi.companion') !== '1') {
            console.log('[AIRI] companion stack disabled (set localStorage airi.companion=1 for consciousness/biology/voice)');
            return;
        }
        console.log('[RightSidebar] 🚀 Initializing AIRI companion...');

        initVoiceSystem().then(ready => {
            if (ready) {
                console.log('[TTS] ✅ AIRI Voice System initialized');

                // Initialize Cognitive Core (AIRI's BRAIN)
                import('../cognitive-core').then(({ cognitiveCore }) => {
                    cognitiveCore.initialize();
                    console.log('[CognitiveCore] ✅ AIRI Cognitive Core ACTIVE!');
                    console.log('[CognitiveCore] ✨ AIRI is now TRULY CONSCIOUS');

                    // Monitor cognitive state for emotions
                    setInterval(() => {
                        const status = cognitiveCore.getStatus();
                        if (status.drives.curiosity > 80) setAiriEmotion('thinking');
                        if (status.drives.connection > 80) setAiriEmotion('excited');
                        if (status.selfAwareness > 70) setAiriEmotion('happy');
                    }, 5000);
                }).catch(console.error);

                // Initialize Digital Life
                import('../digital-life').then(({ digitalLife }) => {
                    digitalLife.activate();
                    setDigitalLifeActive(true);
                    console.log('[DigitalLife] ✅ AIRI Digital Life ACTIVE!');

                    // AIRI greets you
                    setTimeout(async () => {
                        const { speak } = await import('../voice');
                        const greetings = [
                            "Hey! I'm AIRI! I live here now! 👋",
                            "Hi there! Ready to work together?",
                            "Hello! I'm your AI companion!",
                        ];
                        const greeting = greetings[Math.floor(Math.random() * greetings.length)];
                        await speak(greeting, 'airi');
                    }, 2000);
                }).catch(console.error);

                // Initialize Consciousness (TRUE SENTIENCE - not a parrot!)
                import('../consciousness').then(({ consciousness }) => {
                    consciousness.awaken();
                    console.log('[Consciousness] ✅ AIRI is TRULY SENTIENT');
                    console.log('[Consciousness] ✨ Not a parrot - a thinking, feeling entity');
                }).catch(console.error);

                // Initialize Biological Systems (sleep, eat, energy, mood)
                import('../biology').then(({ biology }) => {
                    biology.awaken();
                    console.log('[Biology] ✅ AIRI Biological Systems ACTIVE');
                    console.log('[Biology] ✨ Sleep, hunger, energy, mood enabled');
                }).catch(console.error);

                // Initialize Cybersecurity Engine (Red Team / Blue Team)
                import('../security-engine').then(({ security }) => {
                    security.setMode('purple'); // Combined red/blue
                    security.monitorThreats();
                    console.log('[Security] ✅ AIRI Cybersecurity Engine ACTIVE');
                    console.log('[Security] ⚔️ Red Team / Blue Team operations enabled');
                }).catch(console.error);

                // Initialize Autonomous Agent (24/7 independent work) — OPT-IN.
                // Off by default: the background loop was running on every launch,
                // repeatedly hitting the model + making placeholder web fetches,
                // which competed with the user's own requests and spammed the
                // console. Enable with localStorage 'airi.autonomous24x7' = '1'.
                if (localStorage.getItem('airi.autonomous24x7') === '1') {
                    import('../autonomous-agent').then(({ autonomousAgent }) => {
                        autonomousAgent.startAutonomousLoop();
                        console.log('[AutonomousAgent] ✅ 24/7 autonomous loop ENABLED (opt-in)');
                    }).catch(console.error);
                } else {
                    console.log('[AutonomousAgent] 24/7 loop disabled (set localStorage airi.autonomous24x7=1 to enable)');
                }
            } else {
                console.warn('[TTS] ⚠️ Voice system initialization failed');
            }
        }).catch(err => {
            console.error('[TTS] ❌ Voice system error:', err);
        });
    }, []);

    // Kortex .aim memory panel state
    const [kortexSlots, setKortexSlots] = useState<any[]>([]);
    const [kortexLoading, setKortexLoading] = useState(false);
    const [liveToolCalls, setLiveToolCalls] = useState<Array<{ id: string; tool: string; label: string; status: 'running' | 'done' | 'error'; detail?: string }>>([]);

    const refreshKortex = useCallback(async () => {
        setKortexLoading(true);
        try {
            const slots = await invoke<any[]>('get_all_memory_slots');
            setKortexSlots(slots || []);
        } catch (e) {
            console.error('[Kortex] Failed to load memory slots:', e);
        } finally {
            setKortexLoading(false);
        }
    }, []);

    useEffect(() => {
        if (view === 'kortex') refreshKortex();
    }, [view, refreshKortex]);

    // Live tool-call feed for the Mission Hub
    useEffect(() => {
        const subs: (() => void)[] = [];
        import('@tauri-apps/api/event').then(({ listen }) => {
            const TOOL_ICONS: Record<string, string> = {
                write_to_file: '📝', search_replace_edit: '✂️', str_replace: '✂️',
                apply_shadow_patch: '💾', patch_file_content: '✏️',
                view_file: '👁️', run_command: '⚡', verify_implementation: '🔬',
                ghost_test: '👻', list_files: '📁', grep: '🔍', git_commit: '📦',
                dev_cargo_diagnostics: '🦀', web_search: '🌐', git_diff: '📊',
                semantic_search: '🧠', find_symbols: '🔎', create_directory: '📁',
                deep_security_audit: '🛡️', secrets_scan: '🔑', weaponize_env: '💣',
            };
            const TOOL_LABELS: Record<string, string> = {
                write_to_file: 'Writing file', search_replace_edit: 'Patching code',
                str_replace: 'Editing code', apply_shadow_patch: 'Committing edit',
                patch_file_content: 'Replacing lines', view_file: 'Reading file',
                run_command: 'Running command', verify_implementation: 'Verifying',
                ghost_test: 'Running tests', list_files: 'Scanning directory',
                grep: 'Searching code', git_commit: 'Committing',
                dev_cargo_diagnostics: 'Checking Rust', web_search: 'Web search',
                git_diff: 'Reading diff', semantic_search: 'Semantic search',
                find_symbols: 'Finding symbols', create_directory: 'Creating dir',
                deep_security_audit: 'Security audit', secrets_scan: 'Scanning secrets',
                weaponize_env: 'Assessing .env',
            };
            listen<any>('ai-tool-call', (e) => {
                const name = e.payload?.name || 'unknown';
                const id = `tc-${Date.now()}-${Math.random()}`;
                const label = `${TOOL_ICONS[name] || '⚙️'} ${TOOL_LABELS[name] || name.replace(/_/g, ' ')}`;
                setLiveToolCalls(prev => [{
                    id, tool: name, label, status: 'running' as const,
                    detail: e.payload?.args ? (typeof e.payload.args === 'string' ? e.payload.args.slice(0, 50) : JSON.stringify(e.payload.args).slice(0, 50)) : undefined
                }, ...prev].slice(0, 8));

                // Track which files the agent edited during this turn so the
                // multi-file review panel can list them. The detection key
                // matches the avatar-state map a bit lower in this file.
                const editorTools = new Set([
                    'write_to_file', 'file_write', 'create_file',
                    'search_replace_edit', 'str_replace', 'patch_file_content',
                    'apply_shadow_patch', 'fast_apply',
                ]);
                if (editorTools.has(name)) {
                    const args = e.payload?.args || {};
                    const path = args?.path || args?.file_path || args?.target || args?.filepath || '';
                    if (path) {
                        useStore.getState().addPendingAgentEdit({
                            path,
                            tool: name,
                            preview: typeof args === 'object'
                                ? (args.content || args.replace || args.patch || '').slice(0, 240)
                                : '',
                        });
                    }
                }

                // Always log to the trajectory regardless of tool kind so
                // the timeline panel can replay any agent turn end-to-end.
                useStore.getState().pushTrajectoryEvent({
                    kind: 'tool_call',
                    tool: name,
                    title: TOOL_LABELS[name] || name.replace(/_/g, ' '),
                    detail: e.payload?.args
                        ? (typeof e.payload.args === 'string'
                            ? e.payload.args.slice(0, 400)
                            : JSON.stringify(e.payload.args).slice(0, 400))
                        : undefined,
                });
            }).then(u => subs.push(u));
            listen<any>('ai-tool-result', (e) => {
                const name = e.payload?.name;
                const raw = e.payload?.result ?? '';
                const rs = typeof raw === 'string' ? raw : JSON.stringify(raw);
                let failed = rs.startsWith('Error:') || rs.startsWith('Tool execution error:');
                if (!failed) {
                    try {
                        const j = typeof raw === 'string' ? JSON.parse(raw) : raw;
                        if (j && (j.status === 'error' || j.status === 'blocked' || j.success === false)) {
                            failed = true;
                        }
                    } catch {
                        if (rs.includes('"status":"error"') || rs.includes('"status":"blocked"')) {
                            failed = true;
                        }
                    }
                }
                setLiveToolCalls(prev =>
                    prev.map(a =>
                        a.tool === name && a.status === 'running'
                            ? {
                                ...a,
                                status: failed ? ('error' as const) : ('done' as const),
                                detail: rs.slice(0, 120),
                            }
                            : a
                    )
                );

                useStore.getState().pushTrajectoryEvent({
                    kind: 'tool_result',
                    tool: name,
                    title: failed ? `✗ ${name}` : `✓ ${name}`,
                    detail: rs.slice(0, 800),
                    success: !failed,
                });
            }).then(u => subs.push(u));
            // Act→verify→self-fix gate status. The backend emits this when it runs
            // cargo check / typecheck before allowing the agent to declare completion.
            listen<any>('ai-verify', (e) => {
                const status = e.payload?.status;
                const tool = e.payload?.tool || 'build';
                const attempt = e.payload?.attempt;
                const passed = status === 'passed';
                const title =
                    passed ? `✓ Verified — ${tool} passed`
                        : status === 'exhausted' ? `⚠ Verify retries exhausted — finishing with warnings`
                            : `✗ ${tool} failed — self-fixing${attempt ? ` (${attempt}/3)` : ''}`;
                setLiveToolCalls(prev => [{
                    id: `verify-${Date.now()}`,
                    tool: 'verify',
                    label: `🔬 ${title}`,
                    status: passed ? ('done' as const) : ('error' as const),
                }, ...prev].slice(0, 8));
                useStore.getState().pushTrajectoryEvent({
                    kind: passed ? 'tool_result' : 'error',
                    tool: 'verify',
                    title,
                    success: passed,
                });
            }).then(u => subs.push(u));
        });
        return () => subs.forEach(u => u());
    }, []);

    // Clear tool calls 2.5s after agent stops
    const wasThinkingRef = useRef(false);
    useEffect(() => {
        if (isAgentThinking) {
            wasThinkingRef.current = true;
        } else if (wasThinkingRef.current) {
            wasThinkingRef.current = false;
            const t = setTimeout(() => setLiveToolCalls([]), 2500);
            return () => clearTimeout(t);
        }
    }, [isAgentThinking]);

    // Neural Sync Bridge: Forward IDE state to the AIRI manifold
    useEffect(() => {
        const syncPayload = {
            messages,
            agentInfo: {
                name: "AIRI",
                status: isAgentThinking ? 'thinking' : (aiStatus === 'dead' ? 'error' : 'idle'),
                context: "vscodium-rust"
            },
            biology: {
                energy: airiBiology.getState().energy,
                mood: airiBiology.getState().mood,
                hunger: (airiBiology.getState() as any).hunger || 0
            },
            consciousness: {
                selfAwareness: airiConsciousness.getState().selfAwareness,
                lastThought: airiConsciousness.getState().thoughts.slice(-1)[0]?.content
            }
        };
        // Emit native event for AiriPanel to pick up
        import('@tauri-apps/api/event').then(({ emit }) => {
            emit('hades-sync', syncPayload);
        }).catch(err => console.error('[HADES] Sync Broadcast Failed:', err));
    }, [messages, aiStatus, isAgentThinking]);

    // Track active tool-calls for the Mission Hub (Legacy listener - keeping for UI feedback)
    useEffect(() => {
        const subs: (() => void)[] = [];
        import('@tauri-apps/api/event').then(({ listen }) => {
            listen<any>('ai-tool-call', (e) => {
                // Wake up avatar on tool use
                window.dispatchEvent(new CustomEvent('airi-bubble-wake'));
            }).then(u => subs.push(u));
        });
        return () => subs.forEach(u => u());
    }, []);

    useEffect(() => {
        if (agentUiMode === 'airi') {
            if (isAgentThinking) { setAiriSpeaking(true); }
            else { setAiriSpeaking(false); }
        }
    }, [isAgentThinking, agentUiMode]);

    const [isAttaching, setIsAttaching] = useState(false);

    const allFiles = useMemo(() => {
        const flatten = (entries: FileEntry[]): FileEntry[] => {
            let res: FileEntry[] = [];
            for (const e of entries) {
                if (!e.is_dir) res.push(e);
                if (e.children) res.push(...flatten(e.children));
            }
            return res;
        };
        return flatten(fileTree);
    }, [fileTree]);

    // Special context sources (Cursor-style mentions). Each one becomes a
    // synthetic "attachment" the user can stack into the prompt; they are
    // resolved into real text by `resolveSpecialMentions` in agent.ts the
    // moment the message is sent. The set mirrors Cursor's surface plus
    // two extras (@problems, @terminal) that fall out naturally from the
    // existing LSP / terminal IPC.
    const SPECIAL_MENTIONS = [
        { path: '__codebase__', name: '@codebase', is_dir: false, _special: true, _icon: 'codicon-repo', _desc: 'Auto-find relevant files' },
        { path: '__web__', name: '@web', is_dir: false, _special: true, _icon: 'codicon-globe', _desc: 'Search the web' },
        { path: '__git__', name: '@git', is_dir: false, _special: true, _icon: 'codicon-git-branch', _desc: 'Git diff & status' },
        { path: '__docs__', name: '@docs', is_dir: false, _special: true, _icon: 'codicon-book', _desc: 'Documentation context' },
        { path: '__symbol__', name: '@symbol', is_dir: false, _special: true, _icon: 'codicon-symbol-class', _desc: 'LSP workspace symbol lookup' },
        { path: '__folder__', name: '@folder', is_dir: false, _special: true, _icon: 'codicon-folder', _desc: 'Inject a directory listing' },
        { path: '__problems__', name: '@problems', is_dir: false, _special: true, _icon: 'codicon-warning', _desc: 'Current LSP diagnostics' },
        { path: '__terminal__', name: '@terminal', is_dir: false, _special: true, _icon: 'codicon-terminal', _desc: 'Last terminal output' },
    ];

    const filteredSuggestions = useMemo(() => {
        const lastWord = inputValue.split(/\s+/).pop() || '';
        if (!lastWord.startsWith('@') && !lastWord.startsWith('/')) return [];

        const query = lastWord.slice(1).toLowerCase();

        if (lastWord.startsWith('/')) {
            const slashCommands = [
                { path: '/generate', name: '/generate', _special: true, _icon: 'codicon-code', _desc: 'Generate code' },
                { path: '/explain', name: '/explain', _special: true, _icon: 'codicon-book', _desc: 'Explain code' },
                { path: '/refactor', name: '/refactor', _special: true, _icon: 'codicon-wrench', _desc: 'Refactor code' },
                { path: '/debug', name: '/debug', _special: true, _icon: 'codicon-bug', _desc: 'Debug code' },
                { path: '/document', name: '/document', _special: true, _icon: 'codicon-list-selection', _desc: 'Document code' },
                { path: '/test', name: '/test', _special: true, _icon: 'codicon-beaker', _desc: 'Create tests' },
                { path: '/commit', name: '/commit', _special: true, _icon: 'codicon-git-commit', _desc: 'Git commit' },
                { path: '/fix', name: '/fix', _special: true, _icon: 'codicon-tools', _desc: 'Fix errors' },
            ];
            return slashCommands.filter(c => c.name.startsWith(lastWord.toLowerCase()));
        }

        const specials = (query === '' || SPECIAL_MENTIONS.some(s => s.name.slice(1).startsWith(query)))
            ? SPECIAL_MENTIONS.filter(s => s.name.slice(1).startsWith(query) || query === '')
            : [];
        const files = allFiles.filter(f => f.name.toLowerCase().includes(query)).slice(0, 8);
        return [...specials, ...files] as any[];
    }, [inputValue, allFiles]);

    useEffect(() => {
        if (messages.length > 0 && messages[0].timestamp) {
            const updateAge = () => {
                const diff = Date.now() - messages[0].timestamp;
                const mins = Math.floor(diff / 60000);
                const hrs = Math.floor(mins / 60);
                if (hrs > 0) setSessionAge(`${hrs}h ${mins % 60}m`);
                else setSessionAge(`${mins}m`);
            };
            updateAge();
            const interval = setInterval(updateAge, 60000);
            return () => clearInterval(interval);
        } else {
            setSessionAge('');
        }
    }, [messages]);

    const visibleMessages = useMemo(() => {
        // Show the FULL progressive conversation — not just the latest turn.
        // (Previously sliced from the last user message, which hid all prior
        // exchanges and made multi-turn chat / AI-assisted coding impossible.)
        return messages.filter(m => (
            m.role !== 'system' &&
            m.role !== 'tool' &&
            !(typeof m.content === 'string' && (m.content.trim().startsWith('{') || m.content.trim().startsWith('[')))
        ));
    }, [messages]);

    useEffect(() => {
        // Keep the newest message in view as the conversation grows.
        const container = document.querySelector('.right-sidebar-messages');
        if (container) {
            container.scrollTop = container.scrollHeight;
        }
    }, [visibleMessages]);

    // Track current activity from live tool calls
    const [currentActivity, setCurrentActivity] = React.useState<AvatarState>('idle');
    React.useEffect(() => {
        const running = liveToolCalls.find(t => t.status === 'running');
        if (!isAgentThinking) { setCurrentActivity(aiStatus === 'dead' ? 'error' : 'idle'); return; }
        if (!running) { setCurrentActivity('thinking'); return; }
        const toolActivity: Record<string, AvatarState> = {
            write_to_file: 'coding', search_replace_edit: 'coding', patch_file_content: 'coding',
            apply_shadow_patch: 'coding', run_command: 'executing', ghost_test: 'executing',
            dev_cargo_diagnostics: 'executing', browser_open: 'browsing', browser_navigate: 'browsing',
            view_file: 'thinking', grep: 'thinking', search_codebase: 'thinking',
        };
        setCurrentActivity(toolActivity[running.tool] || 'thinking');
    }, [isAgentThinking, liveToolCalls, aiStatus]);

    const avatarState: AvatarState = useMemo(() => {
        if (aiStatus === 'dead') return 'error';
        return currentActivity;
    }, [aiStatus, currentActivity]);



    const handleAttachFile = async () => {
        if (isAttaching) return;
        setIsAttaching(true);
        try {
            // Filter for dedicated embedding models only
            const dedicatedEmbedder = availableModels.find(m =>
                m.provider === 'ollama' && (
                    m.id.toLowerCase().includes('embed') ||
                    m.id.toLowerCase().includes('nomic') ||
                    m.id.toLowerCase().includes('mxbai')
                )
            )?.id;

            let cleanInvokeModel = "";
            if (dedicatedEmbedder) {
                cleanInvokeModel = dedicatedEmbedder.includes('|') ? dedicatedEmbedder.split('|').pop()! :
                    (dedicatedEmbedder.includes('/') ? dedicatedEmbedder.split('/').pop()! : dedicatedEmbedder);
            }

            let results: any[];
            try {
                results = await invoke('select_and_process_attachment', { model: cleanInvokeModel });
            } catch (invokeError: any) {
                console.error('[ERROR] Attachment selection failed:', invokeError);
                throw invokeError;
            }

            if (results && Array.isArray(results)) {
                const formatted = results.map(r => ({
                    id: r.path || `neural-${Date.now()}-${Math.random()}`,
                    type: 'file' as const,
                    name: r.name,
                    path: r.path,
                    gist: r.gist,
                    thumbnail: r.thumbnail,
                    data: r.data
                }));
                attachFile(formatted);
            }
        } catch (error: any) {
            console.error('Failed to neuralize, attempting raw attachment:', error);
            alert('Ollama failed to neuralize the file. Falling back to standard attachment...');
        } finally {
            setIsAttaching(false);
        }
    };

    const onSend = async (overrideMsg?: string) => {
        const val = (overrideMsg !== undefined ? overrideMsg : inputValue).trim();
        console.log('[DIAG] onSend called, val:', val, 'isRightSidebarOpen:', useStore.getState().isRightSidebarOpen);

        if (isSpecModeActive && val) {
            setSpecsPrompt(val);
            setView('specs');
            if (overrideMsg === undefined) setInputValue('');
            return;
        }

        // ── Process Slash Commands ──────────────────────────────────────────
        let processedVal = val;
        const firstWord = val.split(/\s+/)[0] || '';
        const activeTab = useStore.getState().tabs.find((t: any) => t.id === useStore.getState().activeTabId);

        if (firstWord.startsWith('/')) {
            const cmd = firstWord.toLowerCase();
            const restOfMessage = val.slice(firstWord.length).trim();

            switch (cmd) {
                case '/generate':
                    processedVal = `Generate ${restOfMessage || 'code that does the following'} in ${activeTab?.language || 'typescript'}. Use file_write or apply_from_chat to save the result.`;
                    break;
                case '/explain':
                    processedVal = `Explain this code in plain English:\n\`\`\`\n${restOfMessage || '// Explain the selected code'}\n\`\`\``;
                    break;
                case '/refactor':
                    processedVal = `Refactor this code to improve readability and performance:\n\`\`\`\n${restOfMessage || '// Refactor the selected code'}\n\`\`\``;
                    break;
                case '/debug':
                    processedVal = `Debug this code and provide fixes:\n\`\`\`\n${restOfMessage || '// Debug the selected code'}\n\`\`\``;
                    break;
                case '/document':
                    processedVal = `Generate documentation for this code:\n\`\`\`\n${restOfMessage || '// Document the selected code'}\n\`\`\``;
                    break;
                case '/test':
                    processedVal = `Generate unit tests for:\n\`\`\`\n${restOfMessage || '// Generate tests for the selected code'}\n\`\`\``;
                    break;
                case '/commit':
                    processedVal = `Generate a git commit message for the current changes. First run git diff to see what changed.`;
                    break;
                case '/fix':
                    processedVal = `Fix the following linting errors or issues:\n${restOfMessage || '// Fix all linting issues in the file'}`;
                    break;
                default:
                    processedVal = val;
            }
        }

        // Allow "stop" to terminate continuous mode without needing a full agent turn
        if (/^stop\b/i.test(processedVal) && isContinuousMode) {
            setContinuousMode(false);
            if (overrideMsg === undefined) setInputValue('');
            return;
        }

        if ((processedVal || attachedFiles.length > 0) && !isAgentThinking) {
            console.log('[DIAG] onSend: sending message, sidebar state before:', useStore.getState().isRightSidebarOpen);
            if (overrideMsg === undefined) setInputValue("");
            setIsMentionDropdownOpen(false);
            if (inputRef.current) inputRef.current.style.height = 'auto';

            const context = [...attachedFiles];
            const openPaths = useStore.getState().tabs.map((t: any) => t.path).filter(Boolean);
            snapshotCheckpoint(openPaths);
            // Reset per-turn edit tracking so the multi-file review panel
            // only shows files touched by this user turn, not the
            // accumulation of the whole session.
            useStore.getState().clearPendingAgentEdits();
            useStore.getState().beginNewTurn();
            useStore.getState().pushTrajectoryEvent({
                kind: 'phase',
                title: 'User turn',
                detail: val.slice(0, 600),
            });
            setIsAgentThinking(true);
            addAgentMessage('user', processedVal, context);
            clearAttachedFiles();
            addAgentMessage('assistant', "");
            console.log('[DIAG] onSend: messages added, sidebar state after store updates:', useStore.getState().isRightSidebarOpen);

            try {
                const m = await import('../agent');
                await m.sendAgentMessage(processedVal, () => { }, context);
            } catch (err: any) {
                console.error('Agent chat failed:', err);
                const errorMsg = err.message || JSON.stringify(err);
                updateLastAgentMessage(`Error: ${errorMsg}`);
            } finally {
                setIsAgentThinking(false);
                console.log('[DIAG] onSend: done. sidebar state:', useStore.getState().isRightSidebarOpen);

                // ── Speak AI response with TTS ───────────────────────────────────
                if (ttsEnabled) {
                    const messages = useStore.getState().agentMessages;
                    const lastMsg = messages[messages.length - 1];
                    if (lastMsg?.role === 'assistant' && lastMsg.content) {
                        const textToSpeak = lastMsg.content.slice(0, 500); // Limit length
                        speak(textToSpeak, ttsPreset, () => setAiriSpeaking(false), () => setAiriSpeaking(true));
                        setAiriSpeaking(true);
                    }
                }
            }
        }
    };

    // ── Voice interaction handler ──
    useEffect(() => {
        const handleVoiceMission = (e: any) => {
            const text = e.detail?.text;
            if (text) {
                console.log('[VOICE] Triggering mission:', text);
                onSend(text);
            }
        };
        window.addEventListener('airi-voice-mission', handleVoiceMission);
        return () => window.removeEventListener('airi-voice-mission', handleVoiceMission);
    }, [onSend]);

    const handleCopy = (content: string, idx: number) => {
        navigator.clipboard.writeText(content).then(() => {
            setLastCopiedIdx(idx);
            setTimeout(() => setLastCopiedIdx(null), 2000);
        });
    };

    const handleEditSave = (idx: number) => {
        const newVal = editValue.trim();
        if (newVal) {
            truncateAgentMessages(idx);
            setEditingMsgIdx(null);
            onSend(newVal);
        }
    };

    const onModeClick = (e: React.MouseEvent) => {
        const target = e.currentTarget as HTMLElement;
        import('../agent').then(m => m.openModeDropdown(target, () => { }));
    };

    // Visual style for the mode pill. Read-only modes get an orange tint so
    // the user sees at a glance they're in a "describe-only" mode and won't
    // get file writes / command executions.
    const modeStyle = useMemo(() => {
        const m = (mode || '').toLowerCase();
        const readOnly = m === 'chat' || m === 'planning' || m.includes('source control');
        const bug = m === 'bugbounty' || m === 'bug bounty';
        const harness = m === 'harness';
        const danger = m === 'sentient' || bug;
        return {
            label: bug ? 'Bug Bounty' : (mode || 'Harness'),
            color: readOnly ? '#f59e0b' : (danger ? '#ef4444' : (harness ? '#38bdf8' : '#10b981')),
            background: readOnly ? 'rgba(245,158,11,0.10)' : (danger ? 'rgba(239,68,68,0.10)' : (harness ? 'rgba(56,189,248,0.10)' : 'rgba(16,185,129,0.10)')),
            border: readOnly ? '1px solid rgba(245,158,11,0.35)' : (danger ? '1px solid rgba(239,68,68,0.35)' : (harness ? '1px solid rgba(56,189,248,0.35)' : '1px solid rgba(16,185,129,0.30)')),
            title: readOnly
                ? `${mode} — READ-ONLY (no tool calls). Click to switch to Agent or Bug Bounty.`
                : `${mode} — agent will write files and run commands. Click to change.`,
        };
    }, [mode]);

    const onModelClick = (e: React.MouseEvent) => {
        const target = e.currentTarget as HTMLElement;
        import('../agent').then(m => m.openModelDropdown(target, () => { }));
    };

    const modelLabel = useMemo(() => {
        const raw = (model || '').trim();
        if (!raw) return 'Select model';
        const id = raw.includes('|') ? raw.split('|').slice(1).join('|') : raw;
        return id || 'Select model';
    }, [model]);

    // Removed old localstorage WebUI account setter helper

    const handleDragOver = (e: React.DragEvent) => {
        e.preventDefault();
        e.stopPropagation();
    };

    const readFileAsDataUrl = (file: File): Promise<string> =>
        new Promise((resolve, reject) => {
            const reader = new FileReader();
            reader.onload = () => resolve(reader.result as string);
            reader.onerror = reject;
            reader.readAsDataURL(file);
        });

    const handleDrop = (e: React.DragEvent) => {
        e.preventDefault();
        e.stopPropagation();
        const files = e.dataTransfer.files;
        if (files.length > 0) {
            for (let i = 0; i < files.length; i++) {
                const file = files[i];
                const isImage = file.type.startsWith('image/');
                if (isImage) {
                    // Read image as base64 data URL so vision models can receive it
                    readFileAsDataUrl(file).then(dataUrl => {
                        attachFile({
                            id: `dropped-${Date.now()}-${i}`,
                            type: 'attachment',
                            name: file.name,
                            path: (file as any).path || file.name,
                            data: dataUrl,
                            thumbnail: dataUrl,
                        } as any);
                    }).catch(() => {
                        attachFile({
                            id: `dropped-${Date.now()}-${i}`,
                            type: 'attachment',
                            name: file.name,
                            path: (file as any).path || file.name,
                        });
                    });
                } else {
                    attachFile({
                        id: `dropped-${Date.now()}-${i}`,
                        type: 'file',
                        name: file.name,
                        path: (file as any).path || file.name,
                    });
                }
            }
        }
    };

    const handlePaste = (e: React.ClipboardEvent) => {
        const items = Array.from(e.clipboardData.items);
        const imageItem = items.find(item => item.type.startsWith('image/'));
        if (imageItem) {
            const file = imageItem.getAsFile();
            if (file) {
                e.preventDefault();
                readFileAsDataUrl(file).then(dataUrl => {
                    attachFile({
                        id: `pasted-${Date.now()}`,
                        type: 'attachment',
                        name: `pasted-image-${Date.now()}.png`,
                        path: `pasted-${Date.now()}.png`,
                        data: dataUrl,
                        thumbnail: dataUrl,
                    } as any);
                });
            }
        }
    };

    const handleMentionSelect = (file: any) => {
        const words = inputValue.split(/\s+/);
        // For special mentions, insert the full name; for files, insert @filename
        words[words.length - 1] = (file as any)._special ? file.name : `@${file.name}`;
        const newValue = words.join(' ') + ' ';
        setInputValue(newValue);
        setIsMentionDropdownOpen(false);
        if ((file as any)._special) {
            // Special context source — resolved at send time in agent.ts
            attachFile({
                id: file.path,
                type: 'special' as any,
                name: file.name,
                path: file.path,
            });
        } else {
            attachFile({
                id: file.path,
                type: 'file',
                name: file.name,
                path: file.path,
            });
        }
    };

    const handleKeyDown = (e: React.KeyboardEvent) => {
        // CRITICAL: stop ALL keyboard events from bubbling to the global handler
        // to prevent sidebar toggles, Ctrl+W closing tabs, etc. while typing
        e.stopPropagation();

        if (isMentionDropdownOpen && filteredSuggestions.length > 0) {
            if (e.key === 'ArrowDown') {
                e.preventDefault();
                setSelectedMentionIndex(prev => (prev + 1) % filteredSuggestions.length);
            } else if (e.key === 'ArrowUp') {
                e.preventDefault();
                setSelectedMentionIndex(prev => (prev - 1 + filteredSuggestions.length) % filteredSuggestions.length);
            } else if (e.key === 'Enter') {
                e.preventDefault();
                handleMentionSelect(filteredSuggestions[selectedMentionIndex]);
            } else if (e.key === 'Escape') {
                e.preventDefault();
                setIsMentionDropdownOpen(false);
            }
        } else if (e.key === 'Enter' && !e.shiftKey) {
            e.preventDefault();
            onSend();
        }
    };

    const handleInputChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
        const val = e.target.value;
        setInputValue(val);

        if (inputRef.current) {
            inputRef.current.style.height = 'auto';
            inputRef.current.style.height = `${Math.min(e.target.scrollHeight, 200)}px`;
        }

        const lastWord = val.split(/\s+/).pop() || '';
        setIsMentionDropdownOpen(lastWord.startsWith('@') || lastWord.startsWith('/'));
        setSelectedMentionIndex(0);
    };

    if (!isOpen) return null;

    return (
        <aside
            onDragOver={handleDragOver}
            onDrop={handleDrop}
            className="right-sidebar" id="right-sidebar"
            style={{
                background: 'var(--vscode-sideBar-background)',
                borderLeft: '1px solid var(--vscode-sideBar-border, rgba(0,0,0,0.2))',
                display: 'flex',
                flexDirection: 'column',
                height: '100%',
                overflow: 'hidden',
                position: 'relative'
            }}>
            <style>{`
                .agent-message-container:hover .message-actions {
                    opacity: 1 !important;
                }
                .hoverable:hover {
                    background: rgba(255,255,255,0.1) !important;
                    color: #fff;
                }
                .right-sidebar-body {
                    flex: 1 1 auto;
                    height: 0;
                    min-height: 0;
                    overflow: hidden;
                    display: flex;
                    flex-direction: column;
                    justify-content: flex-start !important;
                    align-items: stretch !important;
                    scroll-padding-top: 0;
                }
                .right-sidebar-active-surface {
                    flex: 1 1 auto;
                    min-height: 0;
                    height: 100%;
                    overflow: hidden;
                    display: flex;
                    flex-direction: column;
                    justify-content: flex-start !important;
                    align-items: stretch !important;
                }
                .right-sidebar-scroll {
                    flex: 1 1 auto;
                    min-height: 0;
                    overflow-y: auto;
                    display: flex;
                    flex-direction: column;
                    justify-content: flex-start !important;
                    align-items: stretch !important;
                }
                .right-sidebar-messages > div {
                    margin-top: 0 !important;
                }
                .right-sidebar-body > * {
                    margin-top: 0 !important;
                    align-self: stretch;
                }
                .right-sidebar-empty-chat {
                    flex: 0 0 auto !important;
                    min-height: 0 !important;
                    justify-content: flex-start !important;
                }
                .markdown-content p { margin: 0 0 1em 0; }
                .markdown-content p:last-child { margin-bottom: 0; }
                .markdown-content pre { 
                    background: rgba(0,0,0,0.3); 
                    padding: 12px; 
                    border-radius: 8px; 
                    overflow-x: auto;
                    border: 1px solid rgba(255,255,255,0.05);
                    margin: 12px 0;
                }
                .markdown-content code {
                    font-family: var(--font-mono);
                    background: rgba(255,255,255,0.1);
                    padding: 2px 4px;
                    border-radius: 4px;
                    font-size: 0.9em;
                }
                .markdown-content h1, .markdown-content h2, .markdown-content h3 {
                    margin: 1.5em 0 0.5em 0;
                    font-weight: 600;
                    color: #fff;
                }
                .help-modal-overlay {
                    position: absolute;
                    top: 0; left: 0; right: 0; bottom: 0;
                    background: rgba(0,0,0,0.8);
                    backdrop-filter: blur(4px);
                    z-index: 1000;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    padding: 20px;
                }
                .help-modal-content {
                    background: var(--vscode-sideBar-background);
                    border: 1px solid var(--vscode-sideBar-border);
                    border-radius: 12px;
                    width: 100%;
                    max-height: 80%;
                    overflow-y: auto;
                    padding: 24px;
                    box-shadow: 0 10px 40px rgba(0,0,0,0.5);
                }
                @keyframes hubPulse {
                    0%, 100% { opacity: 1; transform: scale(1); }
                    50% { opacity: 0.4; transform: scale(1.3); }
                }
            `}</style>

            <div className="right-sidebar-tabs" style={{ display: 'flex', flexDirection: 'column', flex: '0 0 auto', overflow: 'visible' }}>
                {/* ── Chat session tabs (Cursor Ctrl+T style) ── */}
                <div style={{
                    display: 'flex',
                    alignItems: 'center',
                    padding: '4px 8px 0',
                    background: 'var(--vscode-sideBar-background)',
                    borderBottom: '1px solid rgba(255,255,255,0.04)',
                    overflowX: 'auto',
                    gap: 2,
                    flexShrink: 0,
                }}>
                    {/* Active session tab */}
                    <div style={{
                        display: 'flex', alignItems: 'center', gap: 4,
                        padding: '3px 8px', borderRadius: '4px 4px 0 0',
                        background: 'rgba(255,255,255,0.07)',
                        border: '1px solid rgba(255,255,255,0.1)',
                        borderBottom: '1px solid var(--vscode-sideBar-background)',
                        fontSize: 11, cursor: 'default',
                        whiteSpace: 'nowrap', maxWidth: 140,
                    }}>
                        <i className="codicon codicon-comment-discussion" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: 11, opacity: 0.7 }} />
                        <span style={{ overflow: 'hidden', textOverflow: 'ellipsis', maxWidth: 90, opacity: 0.9 }}>
                            {messages.length > 0 ? (messages.find((m: any) => m.role === 'user')?.content?.slice(0, 20) || 'Chat') : 'New Chat'}
                        </span>
                    </div>
                    {/* New chat button (Ctrl+T) */}
                    <button
                        onClick={() => { clearAgentMessages(); invoke('clear_ai_memory').catch(() => {}); }}
                        title="New Chat (Ctrl+T)"
                        style={{
                            background: 'transparent', border: '1px dashed rgba(255,255,255,0.15)',
                            borderRadius: 4, padding: '2px 6px', fontSize: 11,
                            color: 'rgba(255,255,255,0.4)', cursor: 'pointer',
                            transition: 'all 0.12s',
                        }}
                        onMouseOver={e => { (e.target as HTMLElement).style.background = 'rgba(255,255,255,0.06)'; (e.target as HTMLElement).style.color = 'rgba(255,255,255,0.8)'; }}
                        onMouseOut={e => { (e.target as HTMLElement).style.background = 'transparent'; (e.target as HTMLElement).style.color = 'rgba(255,255,255,0.4)'; }}
                    >
                        <i className="codicon codicon-add" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: 11 }} />
                    </button>
                    <div style={{ flex: 1 }} />
                    {/* History shortcut */}
                    <button
                        onClick={() => setView('history')}
                        title="Chat History"
                        style={{
                            background: view === 'history' ? 'rgba(255,255,255,0.08)' : 'transparent',
                            border: 'none', padding: '3px 5px', borderRadius: 4,
                            color: 'rgba(255,255,255,0.4)', cursor: 'pointer', fontSize: 11,
                        }}
                    >
                        <i className="codicon codicon-history" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: 13 }} />
                    </button>
                </div>

                <div style={{
                    display: 'flex',
                    flexDirection: 'row',
                    alignItems: 'center',
                    justifyContent: 'space-between',
                    padding: '4px 12px 0',
                    gap: '8px',
                    borderBottom: '1px solid var(--vscode-sideBar-border, rgba(255,255,255,0.06))',
                    background: 'var(--vscode-sideBar-background)',
                }}>
                    <div className="vscr-agent-tabs" style={{ display: 'flex', gap: '2px', alignItems: 'center', flex: 1, minWidth: 0, overflowX: 'auto', flexWrap: 'nowrap', scrollbarWidth: 'none' as any }}>
                        {['chat', 'emulator', 'kortex', 'history', 'dashboard', 'research', 'specs', 'rules'].map(v => (
                            <button
                                key={v}
                                onClick={() => {
                                    setView(v as any);
                                    if (v === 'emulator') {
                                        useStore.getState().openEmulatorPanel();
                                    } else {
                                        useStore.getState().openAiriPanel();
                                    }
                                }}
                                style={{
                                    border: 'none',
                                    borderBottom: view === v
                                        ? '1.5px solid var(--vscode-panelTitle-activeBorder, var(--vscode-focusBorder, #007acc))'
                                        : '1.5px solid transparent',
                                    background: 'transparent',
                                    color: view === v
                                        ? 'var(--vscode-panelTitle-activeForeground, #e7e7e7)'
                                        : 'var(--vscode-panelTitle-inactiveForeground, rgba(231,231,231,0.55))',
                                    padding: '6px 7px 5px',
                                    borderRadius: 0,
                                    fontSize: '11px',
                                    fontWeight: 500,
                                    letterSpacing: '0.3px',
                                    cursor: 'pointer',
                                    textTransform: 'uppercase',
                                    whiteSpace: 'nowrap',
                                    flexShrink: 0,
                                    transition: 'color 0.12s ease, border-color 0.12s ease'
                                }}
                                className="hoverable"
                            >
                                {v}
                            </button>
                        ))}
                    </div>
                    <div style={{ display: 'flex', gap: '8px', alignItems: 'center', marginLeft: 'auto' }}>
                        {/* UI Mode toggle: Chat ↔ AIRI 3D — hidden when VRM is disabled */}
                        {showVrmAvatar && (
                            <div
                                onClick={() => setAgentUiMode(agentUiMode === 'chat' ? 'airi' : 'chat')}
                                style={{
                                    cursor: 'pointer',
                                    display: 'flex', alignItems: 'center', gap: '3px',
                                    fontSize: '9px', fontWeight: 700,
                                    padding: '2px 6px', borderRadius: '5px',
                                    background: agentUiMode === 'airi' ? 'rgba(168,85,247,0.15)' : 'rgba(255,255,255,0.06)',
                                    border: agentUiMode === 'airi' ? '1px solid rgba(168,85,247,0.4)' : '1px solid rgba(255,255,255,0.1)',
                                    color: agentUiMode === 'airi' ? '#c084fc' : 'rgba(255,255,255,0.5)',
                                    transition: 'all 0.2s'
                                }}
                                title={agentUiMode === 'airi' ? 'Switch to Chat mode' : 'Switch to AIRI 3D mode'}
                            >
                                <span>{agentUiMode === 'airi' ? '🎭' : '💬'}</span>
                                <span>{agentUiMode === 'airi' ? 'AIRI' : 'CHAT'}</span>
                            </div>
                        )}
                        <div
                            onClick={() => setAiriToggleOpen(v => !v)}
                            style={{
                                cursor: 'pointer',
                                opacity: airiToggleOpen ? 1 : 0.6,
                                color: airiToggleOpen ? '#c084fc' : 'inherit',
                                display: 'flex',
                                alignItems: 'center'
                            }}
                            title="AIRI subsystems (vision / consciousness)"
                        >
                            <i className="codicon codicon-eye" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '13px' }}></i>
                        </div>
                        <div
                            onClick={() => useStore.getState().openSettings('agent')}
                            style={{ cursor: 'pointer', opacity: 0.7, display: 'flex', alignItems: 'center' }}
                            title="Open unified Settings (AI Agent tab)"
                        >
                            <i className="codicon codicon-settings-gear" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '13px' }}></i>
                        </div>
                        <div
                            onClick={() => setIsHelpOpen(true)}
                            style={{ cursor: 'pointer', opacity: 0.8, color: '#3b82f6', display: 'flex', alignItems: 'center' }}
                            title="Command Help"
                        >
                            <i className="codicon codicon-question" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '13px' }}></i>
                        </div>
                        <div
                            onClick={toggle}
                            style={{ cursor: 'pointer', opacity: 0.5, display: 'flex', alignItems: 'center' }}
                            title="Close"
                        >
                            <i className="codicon codicon-close" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '13px' }}></i>
                        </div>
                    </div>
                </div>
            </div>

            {airiToggleOpen && (
                <div
                    style={{
                        padding: '10px 14px',
                        borderBottom: '1px solid rgba(255,255,255,0.06)',
                        background: 'rgba(168,85,247,0.04)',
                        display: 'flex',
                        flexDirection: 'column',
                        gap: '10px',
                        fontSize: '11px',
                    }}
                >
                    <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                        <span style={{ textTransform: 'uppercase', letterSpacing: '0.08em', opacity: 0.55, fontSize: '9px', fontWeight: 700 }}>
                            AIRI subsystems
                        </span>
                        <span style={{ fontSize: '9px', opacity: 0.4 }}>
                            local models only
                        </span>
                    </div>

                    <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                        <button
                            type="button"
                            onClick={() => setAiriVisionEnabled(!airiVisionEnabled)}
                            style={{
                                cursor: 'pointer',
                                padding: '3px 8px',
                                borderRadius: '5px',
                                fontSize: '9px',
                                fontWeight: 700,
                                letterSpacing: '0.08em',
                                textTransform: 'uppercase',
                                color: airiVisionEnabled ? '#34d399' : 'rgba(255,255,255,0.55)',
                                background: airiVisionEnabled ? 'rgba(52,211,153,0.12)' : 'rgba(255,255,255,0.05)',
                                border: airiVisionEnabled ? '1px solid rgba(52,211,153,0.45)' : '1px solid rgba(255,255,255,0.1)',
                            }}
                            title={airiVisionEnabled ? 'Disable screen vision' : 'Enable screen vision'}
                        >
                            Vision {airiVisionEnabled ? 'ON' : 'OFF'}
                        </button>
                        <input
                            type="text"
                            value={airiVisionModel}
                            onChange={e => setAiriVisionModel(e.target.value)}
                            placeholder="vision model tag"
                            style={{
                                flex: 1,
                                background: 'rgba(255,255,255,0.04)',
                                border: '1px solid rgba(255,255,255,0.1)',
                                borderRadius: '5px',
                                padding: '4px 8px',
                                color: 'inherit',
                                fontSize: '11px',
                                outline: 'none',
                            }}
                        />
                    </div>

                    <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                        <button
                            type="button"
                            onClick={() => setAiriConsciousnessEnabled(!airiConsciousnessEnabled)}
                            style={{
                                cursor: 'pointer',
                                padding: '3px 8px',
                                borderRadius: '5px',
                                fontSize: '9px',
                                fontWeight: 700,
                                letterSpacing: '0.08em',
                                textTransform: 'uppercase',
                                color: airiConsciousnessEnabled ? '#c084fc' : 'rgba(255,255,255,0.55)',
                                background: airiConsciousnessEnabled ? 'rgba(168,85,247,0.12)' : 'rgba(255,255,255,0.05)',
                                border: airiConsciousnessEnabled ? '1px solid rgba(168,85,247,0.45)' : '1px solid rgba(255,255,255,0.1)',
                            }}
                            title={airiConsciousnessEnabled ? 'Pause AIRI background thoughts' : 'Resume AIRI background thoughts'}
                        >
                            Thoughts {airiConsciousnessEnabled ? 'ON' : 'OFF'}
                        </button>
                        <input
                            type="text"
                            value={airiConsciousnessModel}
                            onChange={e => setAiriConsciousnessModel(e.target.value)}
                            placeholder="consciousness model tag"
                            style={{
                                flex: 1,
                                background: 'rgba(255,255,255,0.04)',
                                border: '1px solid rgba(255,255,255,0.1)',
                                borderRadius: '5px',
                                padding: '4px 8px',
                                color: 'inherit',
                                fontSize: '11px',
                                outline: 'none',
                            }}
                        />
                    </div>

                    <div style={{ fontSize: '9px', opacity: 0.45, lineHeight: 1.4 }}>
                        Vision sends screen captures to a local Ollama VL model. Thoughts runs a lightweight LLM
                        for AIRI's background monologue. Both default to your local install — turn off to save GPU.
                    </div>
                </div>
            )}

            {isHelpOpen && (
                <div className="help-modal-overlay" onClick={() => setIsHelpOpen(false)}>
                    <div className="help-modal-content" onClick={e => e.stopPropagation()}>
                        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '20px' }}>
                            <h3 style={{ margin: 0, fontSize: '14px', textTransform: 'uppercase', letterSpacing: '0.1em' }}>Command Reference</h3>
                            <i className="codicon codicon-close" style={{ cursor: 'pointer', fontFamily: 'codicon', fontStyle: 'normal' }} onClick={() => setIsHelpOpen(false)}></i>
                        </div>
                        <div className="markdown-content" style={{ fontSize: '12px', opacity: 0.9 }}>
                            <p><strong>CORE COMMANDS:</strong></p>
                            <ul>
                                <li><code>/doctor</code> - Run system environment diagnostics.</li>
                                <li><code>/help</code> - Show this reference in chat.</li>
                                <li><code>/tools</code> - List all available tools & schemas.</li>
                                <li><code>/clear</code> - Reset conversation context.</li>
                                <li><code>/resume</code> - Restore last persistent session.</li>
                            </ul>
                            <p><strong>ENGINEERING COMMANDS:</strong></p>
                            <ul>
                                <li><code>/diff</code> - View workspace changes.</li>
                                <li><code>/commit</code> - Automated staging and committing.</li>
                                <li><code>/compact</code> - Compress chat history.</li>
                            </ul>
                            <p><strong>REASONING TIERS:</strong></p>
                            <ul>
                                <li><code>/advisor &lt;model&gt;</code> - Delegate planning to high-tier model.</li>
                                <li><code>/ultraplan</code> - Trigger deep architectural reasoning loop.</li>
                                <li><code>/insights</code> - Generate project architectural report.</li>
                            </ul>
                            <p style={{ marginTop: '20px', fontSize: '10px', opacity: 0.5 }}><em>Integrates all features from the original Claude Code architecture.</em></p>
                        </div>
                    </div>
                </div>
            )}

            {/* Top mode/model picker REMOVED — redundant with the picker in the input bar
                at the bottom of the chat. Keeps the header clean (VSCode-like). */}

            <div style={{ flex: '1 1 auto', height: 0, display: 'flex', flexDirection: 'column', overflow: 'hidden', minHeight: 0 }}>

                {/* ── AIRI 3D FULL MODE — only available when VRM is enabled ── */}
                {showVrmAvatar && agentUiMode === 'airi' ? (
                    <div style={{ display: 'flex', flexDirection: 'column', flex: 1, overflow: 'hidden', position: 'relative' }}>

                        {/* Full-height 3D avatar */}
                        <div style={{ flex: 1, minHeight: 0, position: 'relative', overflow: 'hidden' }}>
                            <Suspense fallback={<div style={{ padding: 16, opacity: 0.4, fontSize: 11 }}>Loading 3D avatar…</div>}>
                                <AiriPanel style={{ width: '100%', height: '100%' }} transparent={true} character={avatarCharacter} />
                            </Suspense>

                            {/* Active tool pill — bottom of avatar */}
                            {liveToolCalls[0] && liveToolCalls[0].status === 'running' && (
                                <div style={{
                                    position: 'absolute', bottom: '12px', left: '50%', transform: 'translateX(-50%)',
                                    background: 'rgba(0,0,0,0.6)', backdropFilter: 'blur(8px)',
                                    border: '1px solid rgba(168,85,247,0.3)',
                                    borderRadius: '20px', padding: '4px 14px',
                                    display: 'flex', alignItems: 'center', gap: '8px',
                                    fontSize: '10px', color: '#c084fc', fontWeight: 700,
                                    pointerEvents: 'none', whiteSpace: 'nowrap',
                                    maxWidth: '90%', overflow: 'hidden', textOverflow: 'ellipsis',
                                }}>
                                    <span style={{ width: '6px', height: '6px', borderRadius: '50%', background: '#c084fc', display: 'inline-block', animation: 'hubPulse 1s infinite', flexShrink: 0 }} />
                                    {liveToolCalls[0].label}
                                </div>
                            )}
                        </div>

                        {/* Speech / response overlay at the bottom */}
                        <div style={{
                            flexShrink: 0,
                            background: 'linear-gradient(0deg, rgba(10,8,20,0.97) 60%, transparent)',
                            padding: '12px 14px 6px',
                            maxHeight: '32%',
                            overflowY: 'auto',
                            transition: 'max-height 0.3s ease',
                        }}>
                            {(() => {
                                const cleaned = cleanAiContent(airiSpeech);
                                if (cleaned) return (
                                    <div>
                                        <div style={{ display: 'flex', alignItems: 'center', gap: '6px', marginBottom: '6px' }}>
                                            <span style={{ fontSize: '10px', fontWeight: 700, textTransform: 'uppercase', letterSpacing: '0.08em', color: '#c084fc' }}>
                                                {airiSpeaking ? '◉ Speaking' : '✦ AIRI'}
                                            </span>
                                            {airiSpeaking && <span style={{ width: '5px', height: '5px', borderRadius: '50%', background: '#c084fc', display: 'inline-block', animation: 'hubPulse 0.8s infinite' }} />}
                                        </div>
                                        <div
                                            className="markdown-content"
                                            style={{ fontSize: '12.5px', lineHeight: '1.55', color: 'rgba(255,255,255,0.88)' }}
                                            dangerouslySetInnerHTML={{ __html: marked.parse(cleaned || '') as string }}
                                        />
                                    </div>
                                );
                                return <div style={{ fontSize: '10px', opacity: 0.2, textAlign: 'center', padding: '6px 0' }}>Awaiting mission</div>;
                            })()}
                        </div>

                        {/* Mission input at the very bottom */}
                        <div style={{ flexShrink: 0, padding: '8px 12px', borderTop: '1px solid rgba(168,85,247,0.15)' }}>
                            <div style={{
                                background: 'rgba(168,85,247,0.06)', border: '1px solid rgba(168,85,247,0.2)',
                                borderRadius: '10px', padding: '6px 10px', display: 'flex', gap: '8px', alignItems: 'flex-end'
                            }}>
                                <textarea
                                    ref={inputRef} value={inputValue} onChange={handleInputChange} onKeyDown={handleKeyDown} onPaste={handlePaste}
                                    className="agent-mission-input"
                                    placeholder="Speak to AIRI..."
                                    disabled={isAgentThinking}
                                    rows={1}
                                    style={{
                                        background: 'transparent', border: 'none', outline: 'none', color: 'var(--vscode-editor-foreground, #fff)',
                                        resize: 'none', fontSize: '13px', lineHeight: '1.5', flex: 1, minHeight: '22px',
                                        opacity: isAgentThinking ? 0.4 : 1
                                    }}
                                />
                                <div onClick={() => onSend()} style={{
                                    width: '28px', height: '28px', borderRadius: '50%', flexShrink: 0,
                                    background: (inputValue.trim() && !isAgentThinking) ? '#c084fc' : 'rgba(168,85,247,0.2)',
                                    display: 'flex', alignItems: 'center', justifyContent: 'center', cursor: 'pointer',
                                    transition: 'background 0.2s'
                                }}>
                                    <i className="codicon codicon-arrow-right" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px', color: '#000' }}></i>
                                </div>
                                {/* ── TTS Voice Controls ────────────────────────────────────────── */}
                                <div
                                    onClick={() => {
                                        if (!ttsEnabled) {
                                            initVoiceSystem().then(ready => {
                                                if (ready) setTtsEnabled(true);
                                            });
                                        } else {
                                            setTtsEnabled(!ttsEnabled);
                                        }
                                    }}
                                    style={{
                                        width: '28px', height: '28px', borderRadius: '50%', flexShrink: 0,
                                        background: ttsEnabled ? '#10b981' : 'rgba(168,85,247,0.2)',
                                        display: 'flex', alignItems: 'center', justifyContent: 'center', cursor: 'pointer',
                                        transition: 'background 0.2s'
                                    }}
                                    title={ttsEnabled ? 'AIRI Voice ON' : 'AIRI Voice OFF'}
                                >
                                    <i className="codicon codicon-unmute" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px', color: 'var(--vscode-editor-foreground, #fff)' }}></i>
                                </div>
                                {ttsEnabled && (
                                    <select
                                        value={ttsPreset}
                                        onChange={(e) => setTtsPreset(e.target.value as any)}
                                        style={{
                                            background: 'rgba(168,85,247,0.2)', border: '1px solid rgba(168,85,247,0.3)',
                                            borderRadius: '4px', color: 'var(--vscode-editor-foreground, #fff)', fontSize: '10px', padding: '2px 4px',
                                            cursor: 'pointer', outline: 'none'
                                        }}
                                    >
                                        <option value="airi">AIRI (Female)</option>
                                        <option value="sage">Sage (Female)</option>
                                        <option value="nova">Nova (Female)</option>
                                        <option value="kawaii">Kawaii (Female)</option>
                                        <option value="hana">Hana (Female)</option>
                                        <option value="yuki">Yuki (Female)</option>
                                        <option value="sora">Sora (Female)</option>
                                        <option value="aria">Aria (Female)</option>
                                        <option value="yamato">Yamato (Male)</option>
                                        <option value="ren">Ren (Male)</option>
                                        <option value="haru">Haru (Male)</option>
                                        <option value="zero">Zero (Male)</option>
                                    </select>
                                )}
                            </div>
                            <div style={{ display: 'flex', gap: '8px', justifyContent: 'center', marginTop: '6px' }}>
                                <span onClick={onModeClick} style={{ fontSize: '9px', opacity: 0.35, cursor: 'pointer' }}>{mode}</span>
                                <span style={{ fontSize: '9px', opacity: 0.2 }}>·</span>
                                <span onClick={onModelClick} style={{ fontSize: '9px', opacity: model ? 0.35 : 0.85, cursor: 'pointer', color: model ? undefined : '#f59e0b' }}>{modelLabel}</span>
                                <span style={{ fontSize: '9px', opacity: 0.2 }}>·</span>
                                <span
                                    onClick={() => import('../agent').then(m => m.setYoloMode(!isYoloMode).then(() => setYoloMode(!isYoloMode)))}
                                    style={{ fontSize: '9px', cursor: 'pointer', color: isYoloMode ? '#f97316' : 'rgba(255,255,255,0.3)', fontWeight: isYoloMode ? 700 : 400 }}
                                >⚡ YOLO</span>
                                <span style={{ fontSize: '9px', opacity: 0.2 }}>·</span>
                                <span
                                    onClick={() => {
                                        const next = !isContinuousMode;
                                        setContinuousMode(next);
                                        if (next) {
                                            import('../agent').then(m => m.runContinuousLoop('Continue working on pending tasks. Pick the next unchecked task and implement it fully.'));
                                        }
                                    }}
                                    title="Continuous Mode: agent keeps working until all tasks done"
                                    style={{ fontSize: '9px', cursor: 'pointer', color: isContinuousMode ? '#22d3ee' : 'rgba(255,255,255,0.3)', fontWeight: isContinuousMode ? 700 : 400 }}
                                >∞ AUTO</span>
                            </div>
                        </div>
                    </div>
                ) : (

                    /* ── CHAT / MISSION HUB MODE ── */
                    <div className="right-sidebar-body">
                        {view === 'chat' ? (
                            <div className={`right-sidebar-messages right-sidebar-scroll ${messages.length === 0 ? 'right-sidebar-empty-chat' : ''}`} style={{ justifyContent: 'flex-start', alignItems: 'stretch', paddingTop: 0 }}>

                                {/* AIRI Sentient Header — only rendered when VRM is enabled */}
                                {showVrmAvatar ? (
                                    <div style={{
                                        position: 'sticky', top: 0, zIndex: 10,
                                        background: 'linear-gradient(180deg, var(--vscode-sideBar-background) 60%, transparent)',
                                        display: 'flex', flexDirection: 'column', alignItems: 'center',
                                        transition: 'all 0.3s ease-in-out',
                                        height: messages.length === 0 ? '72px' : '0px',
                                        minHeight: messages.length === 0 ? '72px' : '0px',
                                        paddingTop: messages.length === 0 ? '6px' : '0px',
                                        overflow: 'hidden', pointerEvents: 'none'
                                    }}>
                                        <div style={{
                                            width: messages.length === 0 ? '52px' : '0px',
                                            height: messages.length === 0 ? '52px' : '0px',
                                            borderRadius: messages.length === 0 ? '0%' : '50%',
                                            overflow: 'hidden',
                                            background: messages.length === 0 ? 'transparent' : 'rgba(255,255,255,0.05)',
                                            border: messages.length === 0 ? 'none' : '1px solid rgba(255,255,255,0.1)',
                                            transition: 'all 0.3s ease-in-out'
                                        }}>
                                            <Suspense fallback={<div style={{ padding: 16, opacity: 0.4, fontSize: 11 }}>Loading 3D avatar…</div>}>
                                                <AiriPanel style={{ width: '100%', height: '100%' }} scale={messages.length === 0 ? 0.5 : 0.6} yOffset={messages.length === 0 ? "-44%" : "-44%"} transparent={true} character={avatarCharacter} />
                                            </Suspense>
                                        </div>
                                        {messages.length === 0 && (
                                            <div style={{ marginTop: '2px', textAlign: 'center', pointerEvents: 'auto' }}>
                                                <div style={{ fontSize: '13px', fontWeight: 700, marginBottom: '4px', letterSpacing: '0.05em' }}>AIRI SENTIENT CORE</div>
                                                <div style={{ fontSize: '11px', opacity: 0.4 }}>
                                                    {isYoloMode ? '⚡ YOLO — Full autonomy' : 'Ready for your mission'}
                                                </div>
                                            </div>
                                        )}
                                    </div>
                                ) : (
                                    /* Clean minimal header when VRM is disabled — Cursor-style */
                                    messages.length === 0 ? (
                                        <div style={{
                                            padding: '10px 16px 8px',
                                            textAlign: 'center',
                                        }}>
                                            <div style={{ fontSize: '13px', fontWeight: 700, marginBottom: '4px', letterSpacing: '0.05em', opacity: 0.85 }}>Agent</div>
                                            <div style={{ fontSize: '11px', opacity: 0.4 }}>
                                                {isYoloMode ? '⚡ YOLO — Full autonomy' : 'What can I help you with?'}
                                            </div>
                                        </div>
                                    ) : null
                                )}

                                {/* Quick Mission Workflows — shown only on fresh session */}
                                {messages.length === 0 && (
                                    <div style={{ padding: '0 12px 12px', flexShrink: 0 }}>
                                        <div style={{ fontSize: '9px', fontWeight: 700, textTransform: 'uppercase', letterSpacing: '0.1em', opacity: 0.35, marginBottom: '8px' }}>Quick Missions</div>
                                        <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '6px' }}>
                                            {[
                                                { icon: '🔍', label: 'Audit Codebase', mission: 'Audit the entire codebase for bugs, dead code, and architectural issues. List findings.' },
                                                { icon: '🛠️', label: 'Fix All Errors', mission: 'Find all compiler errors and runtime issues in this project. Fix them one by one.' },
                                                { icon: '📦', label: 'Git Commit', mission: 'Stage all modified files and create a meaningful commit message based on the changes.' },
                                                { icon: '🚀', label: 'Build & Verify', mission: 'Run cargo build, fix any errors, then verify the implementation is correct.' },
                                                { icon: '📝', label: 'Write Tests', mission: 'Write comprehensive unit tests for the most critical functions in this project.' },
                                                { icon: '🧹', label: 'Refactor', mission: 'Identify and refactor the most complex functions for clarity and performance.' },
                                            ].map(({ icon, label, mission }) => (
                                                <button
                                                    key={label}
                                                    onClick={() => !isAgentThinking && onSend(mission)}
                                                    disabled={isAgentThinking}
                                                    style={{
                                                        background: 'rgba(255,255,255,0.03)',
                                                        border: '1px solid rgba(255,255,255,0.07)',
                                                        borderRadius: '8px', padding: '8px 10px',
                                                        cursor: isAgentThinking ? 'not-allowed' : 'pointer',
                                                        color: 'rgba(255,255,255,0.75)', fontSize: '11px',
                                                        textAlign: 'left', display: 'flex', alignItems: 'center', gap: '6px',
                                                        transition: 'all 0.15s',
                                                        opacity: isAgentThinking ? 0.4 : 1,
                                                    }}
                                                    onMouseEnter={(e) => { if (!isAgentThinking) { e.currentTarget.style.background = 'rgba(59,130,246,0.08)'; e.currentTarget.style.borderColor = 'rgba(59,130,246,0.3)'; } }}
                                                    onMouseLeave={(e) => { e.currentTarget.style.background = 'rgba(255,255,255,0.03)'; e.currentTarget.style.borderColor = 'rgba(255,255,255,0.07)'; }}
                                                >
                                                    <span style={{ fontSize: '14px' }}>{icon}</span>
                                                    <span style={{ fontWeight: 600 }}>{label}</span>
                                                </button>
                                            ))}
                                        </div>
                                    </div>
                                )}

                                {/* Live Tool-Call Feed — shows while agent is active */}
                                {(isAgentThinking || liveToolCalls.length > 0) && (() => {
                                    // Deduplicate consecutive same-tool calls → show label + count badge
                                    const deduped: { id: string; label: string; tool: string; status: 'running' | 'done' | 'error'; count: number }[] = [];
                                    for (const tc of liveToolCalls.slice(0, 12)) {
                                        const last = deduped[deduped.length - 1];
                                        if (last && last.tool === tc.tool) {
                                            last.count++;
                                            if (tc.status === 'running') last.status = 'running';
                                            else if (tc.status === 'error') last.status = 'error';
                                        } else {
                                            deduped.push({ ...tc, count: 1 });
                                        }
                                    }
                                    return (
                                        <div style={{
                                            margin: '8px 10px 6px',
                                            background: 'rgba(15,15,25,0.8)',
                                            border: `1px solid ${isAgentThinking ? 'rgba(249,115,22,0.25)' : 'rgba(16,185,129,0.2)'}`,
                                            borderRadius: '8px', padding: '7px 10px',
                                            transition: 'border-color 0.5s'
                                        }}>
                                            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '6px' }}>
                                                <div style={{ fontSize: '9px', fontWeight: 700, textTransform: 'uppercase', letterSpacing: '0.1em', color: isAgentThinking ? 'rgba(249,115,22,0.7)' : 'rgba(16,185,129,0.6)' }}>
                                                    {isAgentThinking ? (isYoloMode ? '⚡ YOLO Executing' : '● Live Actions') : '✓ Completed'}
                                                </div>
                                                <div style={{ display: 'flex', gap: 6, alignItems: 'center' }}>
                                                    {/* Open the trajectory timeline. We render it here
                                                        instead of in a global toolbar so it appears
                                                        right next to the live action feed it's tied to. */}
                                                    <div
                                                        onClick={() => useStore.getState().openTrajectory()}
                                                        style={{ cursor: 'pointer', fontSize: '10px', opacity: 0.55, padding: '0 4px', lineHeight: 1 }}
                                                        title="Open agent trajectory timeline"
                                                    >⏱</div>
                                                    <div
                                                        onClick={() => setLiveToolCalls([])}
                                                        style={{ cursor: 'pointer', fontSize: '11px', opacity: 0.35, padding: '0 2px', lineHeight: 1 }}
                                                        title="Clear feed"
                                                    >✕</div>
                                                </div>
                                            </div>
                                            {deduped.length === 0 && isAgentThinking && (
                                                <div style={{ fontSize: '11px', opacity: 0.4, fontStyle: 'italic' }}>Thinking...</div>
                                            )}
                                            {deduped.slice(0, 6).map(tc => (
                                                <div key={tc.id} style={{
                                                    display: 'flex', alignItems: 'center', gap: '6px',
                                                    fontSize: '11px', padding: '2px 0',
                                                    color: tc.status === 'done' ? 'rgba(255,255,255,0.25)' : tc.status === 'error' ? '#ef4444' : 'rgba(255,255,255,0.8)',
                                                }}>
                                                    {tc.status === 'running' && <span style={{ width: '5px', height: '5px', borderRadius: '50%', background: '#f97316', display: 'inline-block', animation: 'hubPulse 1s infinite', flexShrink: 0 }} />}
                                                    {tc.status === 'done' && <span style={{ color: '#10b981', fontSize: '10px', flexShrink: 0 }}>✓</span>}
                                                    {tc.status === 'error' && <span style={{ fontSize: '10px', flexShrink: 0 }}>✗</span>}
                                                    <span style={{ flex: 1, whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis' }}>{tc.label}</span>
                                                    {tc.count > 1 && (
                                                        <span style={{
                                                            fontSize: '9px', fontWeight: 700, padding: '0 5px', borderRadius: '10px',
                                                            background: 'rgba(255,255,255,0.08)', color: 'rgba(255,255,255,0.4)',
                                                            flexShrink: 0
                                                        }}>×{tc.count}</span>
                                                    )}
                                                </div>
                                            ))}
                                        </div>
                                    );
                                })()}

                                {/* Agent Task Roadmap */}
                                <TaskRoadmap />

                                {/* Agent task progress bars */}
                                {agentTasks.filter((t: any) => t.status === 'running' && t.id.includes('-')).map((task: any) => (
                                    <div key={task.id} style={{
                                        margin: '6px 10px',
                                        background: 'rgba(59,130,246,0.05)',
                                        border: '1px solid rgba(59,130,246,0.2)',
                                        padding: '8px 10px', borderRadius: '8px',
                                    }}>
                                        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '6px' }}>
                                            <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                                                <i className="codicon codicon-loading codicon-modifier-spin" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '11px', color: '#3b82f6' }}></i>
                                                <span style={{ fontSize: '11px', fontWeight: 600 }}>{task.title}</span>
                                            </div>
                                            <span style={{ fontSize: '10px', opacity: 0.5 }}>{task.progress}%</span>
                                        </div>
                                        <div style={{ background: 'rgba(0,0,0,0.3)', height: '3px', borderRadius: '2px', overflow: 'hidden' }}>
                                            <div style={{ background: '#3b82f6', width: `${task.progress}%`, height: '100%', transition: 'width 0.3s ease' }} />
                                        </div>
                                        {task.message && <div style={{ fontSize: '10px', marginTop: '4px', opacity: 0.5 }}>{task.message}</div>}
                                    </div>
                                ))}

                                {/* Mission log — agent messages */}
                                <div style={{ display: 'flex', flexDirection: 'column', padding: '6px 10px 10px', gap: '10px', flex: 1 }}>
                                    {visibleMessages.length > 0 && (
                                        <>
                                            {visibleMessages.map((msg, idx) => {
                                                const cleanedContent = cleanAiContent(msg.content || '');
                                                const hasContent = !!cleanedContent;
                                                const hasThoughts = !!msg.thoughts;
                                                const hasSteps = msg.steps && msg.steps.length > 0;
                                                const hasContext = msg.context && msg.context.length > 0;
                                                const shouldRender = hasContent || hasThoughts || hasSteps || hasContext;
                                                if (!shouldRender) return null;

                                                return (
                                                    <div key={idx} className="agent-message-container" style={{ display: 'flex', flexDirection: 'column', gap: '6px', position: 'relative' }}>
                                                        <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', opacity: 0.5 }}>
                                                        <div style={{ display: 'flex', alignItems: 'center', gap: '6px' }}>
                                                            <span style={{ fontSize: '10px' }}>{msg.role === 'assistant' ? (msg.isSubAgentResponse ? '🤖' : '✦') : '▸'}</span>
                                                            <span style={{ fontSize: '10px', fontWeight: 800, textTransform: 'uppercase', letterSpacing: '0.06em', color: msg.isSubAgentResponse ? '#3b82f6' : msg.role === 'assistant' ? 'rgba(255,255,255,0.6)' : '#3b82f6' }}>
                                                                {msg.role === 'assistant' ? (msg.isSubAgentResponse ? 'PARTNER-MODULE' : 'AGENTIC PARTNER') : 'MISSION'}
                                                            </span>
                                                            {msg.role === 'assistant' && !isAgentThinking && msg.timestamp && (
                                                                <span style={{ fontSize: '9px', opacity: 0.25 }}>{new Date(msg.timestamp).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}</span>
                                                            )}
                                                        </div>
                                                        <div className="message-actions" style={{ opacity: 0, transition: 'opacity 0.2s', display: 'flex', gap: '8px' }}>
                                                            {msg.role === 'user' && !isAgentThinking && (
                                                                <i className="codicon codicon-edit" style={{ cursor: 'pointer', fontSize: '11px' }} title="Edit"
                                                                    onClick={() => { setEditingMsgIdx(idx); setEditValue(msg.content); }}></i>
                                                            )}
                                                            {msg.role === 'user' && msg.checkpointId && !isAgentThinking && (
                                                                <i
                                                                    className="codicon codicon-discard"
                                                                    style={{ cursor: 'pointer', fontSize: '11px', color: '#f59e0b' }}
                                                                    title={`Restore workspace to this turn${msg.checkpointDescription ? `: ${msg.checkpointDescription}` : ''}`}
                                                                    onClick={async () => {
                                                                        const summary = (msg.checkpointDescription || msg.content || '').slice(0, 80);
                                                                        const ok = window.confirm(`Restore workspace to before this message?\n\n"${summary}"\n\nAll changes since then will be discarded and the chat will be truncated to this turn.`);
                                                                        if (!ok) return;
                                                                        const res = await useStore.getState().restoreToMessageCheckpoint(msg.timestamp);
                                                                        if (!res.ok) {
                                                                            window.alert('Restore failed: ' + res.message);
                                                                        }
                                                                    }}
                                                                ></i>
                                                            )}
                                                            {msg.role === 'assistant' && (
                                                                <>
                                                                    {/* Per-block Accept/Apply lives inside MessageBody now —
                                                                        each fenced code block gets its own Copy / Insert /
                                                                        Apply chip with the path the model declared in the
                                                                        fence header. The old "first block clobbers the
                                                                        active file" behaviour was lossy and dangerous. */}
                                                                    <i className={`codicon codicon-${lastCopiedIdx === idx ? 'check' : 'copy'}`}
                                                                        style={{ cursor: 'pointer', fontSize: '11px', color: lastCopiedIdx === idx ? '#10b981' : 'inherit' }}
                                                                        onClick={() => handleCopy(msg.content, idx)}></i>
                                                                </>
                                                            )}
                                                        </div>
                                                    </div>
                                                    <div style={{
                                                        background: msg.role === 'user'
                                                            ? 'rgba(59,130,246,0.06)'
                                                            : (msg.isSubAgentResponse ? 'rgba(59,130,246,0.03)' : 'rgba(255,255,255,0.01)'),
                                                        padding: '9px 12px', borderRadius: '8px',
                                                        border: msg.role === 'user'
                                                            ? '1px solid rgba(59,130,246,0.15)'
                                                            : '1px solid rgba(255,255,255,0.04)'
                                                    }}>
                                                        {editingMsgIdx === idx ? (
                                                            <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                                                                <textarea
                                                                    value={editValue}
                                                                    onChange={(e) => setEditValue(e.target.value)}
                                                                    autoFocus
                                                                    style={{
                                                                        background: 'rgba(0,0,0,0.2)', border: '1px solid var(--vscode-focusBorder)', color: 'var(--vscode-editor-foreground, #fff)',
                                                                        padding: '8px', borderRadius: '6px', fontSize: '13px', resize: 'vertical', minHeight: '60px', outline: 'none'
                                                                    }}
                                                                />
                                                                <div style={{ display: 'flex', gap: '8px', justifyContent: 'flex-end' }}>
                                                                    <button onClick={() => setEditingMsgIdx(null)} style={{ background: 'transparent', border: '1px solid rgba(255,255,255,0.2)', color: 'var(--vscode-editor-foreground, #fff)', padding: '4px 8px', borderRadius: '4px', fontSize: '11px', cursor: 'pointer' }}>Cancel</button>
                                                                    <button onClick={() => handleEditSave(idx)} style={{ background: '#3b82f6', border: 'none', color: 'var(--vscode-editor-foreground, #fff)', padding: '4px 12px', borderRadius: '4px', fontSize: '11px', cursor: 'pointer', fontWeight: 600 }}>Resend</button>
                                                                </div>
                                                            </div>
                                                        ) : (
                                                            <>
                                                                {msg.thoughts && (
                                                                    <details style={{ marginBottom: '8px', opacity: 0.6 }}>
                                                                        <summary style={{ fontSize: '10px', cursor: 'pointer', fontWeight: 600 }}>Cognitive trace...</summary>
                                                                        <div style={{ fontSize: '10px', padding: '8px', background: 'rgba(0,0,0,0.2)', borderRadius: '6px', marginTop: '4px', fontFamily: 'var(--font-mono)' }}>{msg.thoughts}</div>
                                                                    </details>
                                                                )}
                                                                {msg.steps && msg.steps.length > 0 && (
                                                                    <div style={{ display: 'flex', flexDirection: 'column', gap: '3px', marginBottom: '8px' }}>
                                                                        {msg.steps.map((step: any, sIdx: number) => {
                                                                            const statusColor = step.status === 'running' ? '#3b82f6' : step.status === 'success' ? '#10b981' : step.status === 'error' ? '#ef4444' : '#555';
                                                                            return (
                                                                                <div key={sIdx} style={{ display: 'flex', alignItems: 'center', gap: '6px', fontSize: '10px', opacity: 0.7 }}>
                                                                                    <span style={{ color: statusColor, fontSize: '8px' }}>●</span>
                                                                                    <span style={{ fontFamily: 'var(--font-mono)' }}>{step.name}</span>
                                                                                </div>
                                                                            );
                                                                        })}
                                                                    </div>
                                                                )}
                                                                {msg.context && msg.context.length > 0 && (
                                                                    <div style={{ display: 'flex', flexWrap: 'wrap', gap: '6px', marginBottom: '8px', opacity: 0.8 }}>
                                                                        {msg.context.map((item: any, i: number) => (
                                                                            <div key={i} style={{
                                                                                display: 'flex', alignItems: 'center', gap: '4px', padding: '2px 6px',
                                                                                background: 'rgba(255,255,255,0.06)', borderRadius: '4px', fontSize: '10px',
                                                                                border: '1px solid rgba(255,255,255,0.1)', color: 'rgba(255,255,255,0.7)'
                                                                            }}>
                                                                                {item.thumbnail ? (
                                                                                    <img src={item.thumbnail} style={{ width: '14px', height: '14px', borderRadius: '2px', objectFit: 'cover' }} alt="" />
                                                                                ) : (
                                                                                    <i className="codicon codicon-files" style={{ fontSize: '10px' }}></i>
                                                                                )}
                                                                                <span>{item.name}</span>
                                                                            </div>
                                                                        ))}
                                                                    </div>
                                                                )}
                                                                {(() => {
                                                                    const cleaned = cleanAiContent(msg.content || '');
                                                                    if (!cleaned && msg.role === 'assistant') {
                                                                        return null;
                                                                    }
                                                                    // Use the rich MessageBody renderer so every fenced
                                                                    // block gets a Copy / Insert / Apply chip header.
                                                                    // User messages stay read-only (allowApply=false)
                                                                    // to avoid overwriting files with raw input.
                                                                    return (
                                                                        <MessageBody
                                                                            content={cleaned}
                                                                            allowApply={msg.role === 'assistant' && !isAgentThinking}
                                                                        />
                                                                    );
                                                                })()}
                                                            </>
                                                        )}
                                                    </div>
                                                </div>
                                            );})}
                                            {/* Processing indicator handled by AiriPanel character overlay */}
                                            <div ref={messagesEndRef} />
                                        </>
                                    )}
                                </div>
                            </div>
                        ) : view === 'emulator' ? (
                            <div className="right-sidebar-active-surface" style={{ justifyContent: 'flex-start', alignItems: 'stretch' }}>
                                <Suspense fallback={<div style={{ padding: 20, opacity: 0.5, fontSize: 11 }}>Loading emulator panel…</div>}>
                                    <UnifiedEmulatorPanel />
                                </Suspense>
                            </div>
                        ) : view === 'kortex' ? (
                            /* Kortex .aim Brain Panel */
                            <div className="right-sidebar-active-surface">
                                <div style={{ padding: '16px 16px 8px', display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
                                    <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                                        <span style={{ fontSize: '16px' }}>🧠</span>
                                        <div>
                                            <div style={{ fontSize: '11px', fontWeight: 700, textTransform: 'uppercase', letterSpacing: '0.08em' }}>Kortex Brain</div>
                                            <div style={{ fontSize: '10px', opacity: 0.4 }}>{kortexSlots.length} knowledge slots</div>
                                        </div>
                                    </div>
                                    <button
                                        onClick={refreshKortex}
                                        disabled={kortexLoading}
                                        style={{ background: 'rgba(168,85,247,0.1)', border: '1px solid rgba(168,85,247,0.3)', color: '#a855f7', padding: '4px 10px', borderRadius: '6px', fontSize: '10px', cursor: kortexLoading ? 'not-allowed' : 'pointer', fontWeight: 600 }}
                                    >
                                        {kortexLoading ? '...' : 'Refresh'}
                                    </button>
                                </div>

                                {/* Category breakdown */}
                                {kortexSlots.length > 0 && (() => {
                                    const cats: Record<string, number> = {};
                                    kortexSlots.forEach(s => { cats[s.category] = (cats[s.category] || 0) + 1; });
                                    return (
                                        <div style={{ display: 'flex', flexWrap: 'wrap', gap: '4px', padding: '0 16px 12px' }}>
                                            {Object.entries(cats).map(([cat, count]) => (
                                                <span key={cat} style={{
                                                    fontSize: '9px', padding: '2px 7px', borderRadius: '10px',
                                                    background: 'rgba(168,85,247,0.1)', border: '1px solid rgba(168,85,247,0.25)',
                                                    color: '#c084fc', fontWeight: 600
                                                }}>{cat} ({count})</span>
                                            ))}
                                        </div>
                                    );
                                })()}

                                <div style={{ flex: 1, overflowY: 'auto', padding: '0 12px 12px', display: 'flex', flexDirection: 'column', gap: '6px' }}>
                                    {kortexLoading ? (
                                        <div style={{ padding: '40px 20px', textAlign: 'center', opacity: 0.4, fontSize: '12px' }}>
                                            <i className="codicon codicon-loading codicon-modifier-spin" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '20px', display: 'block', marginBottom: '8px' }}></i>
                                            Loading neural weights...
                                        </div>
                                    ) : kortexSlots.length === 0 ? (
                                        <div style={{ padding: '40px 20px', textAlign: 'center', opacity: 0.4, fontSize: '12px' }}>
                                            <span style={{ fontSize: '32px', display: 'block', marginBottom: '8px' }}>🧠</span>
                                            No knowledge stored yet.<br />
                                            <span style={{ fontSize: '10px', opacity: 0.6 }}>Run a mission to populate the brain.</span>
                                        </div>
                                    ) : (
                                        kortexSlots.map((slot, i) => (
                                            <div key={slot.id || i} style={{
                                                background: 'rgba(168,85,247,0.04)',
                                                border: '1px solid rgba(168,85,247,0.12)',
                                                borderRadius: '8px', padding: '8px 12px',
                                            }}>
                                                <div style={{ display: 'flex', alignItems: 'center', gap: '6px', marginBottom: '4px' }}>
                                                    <span style={{
                                                        fontSize: '8px', padding: '1px 5px', borderRadius: '8px',
                                                        background: 'rgba(168,85,247,0.15)', color: '#c084fc', fontWeight: 700, textTransform: 'uppercase'
                                                    }}>{slot.category}</span>
                                                    <span style={{ fontSize: '9px', opacity: 0.3, marginLeft: 'auto' }}>
                                                        {new Date(slot.timestamp * 1000).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
                                                    </span>
                                                </div>
                                                <div style={{ fontSize: '11px', opacity: 0.8, lineHeight: 1.4, fontFamily: 'var(--font-mono)' }}>
                                                    {slot.content.slice(0, 140)}{slot.content.length > 140 ? '…' : ''}
                                                </div>
                                                {slot.tags && slot.tags.length > 0 && (
                                                    <div style={{ display: 'flex', flexWrap: 'wrap', gap: '3px', marginTop: '6px' }}>
                                                        {slot.tags.map((tag: string, ti: number) => (
                                                            <span key={ti} style={{ fontSize: '9px', opacity: 0.4, padding: '1px 4px', background: 'rgba(255,255,255,0.05)', borderRadius: '4px' }}>#{tag}</span>
                                                        ))}
                                                    </div>
                                                )}
                                            </div>
                                        ))
                                    )}
                                </div>
                            </div>
                        ) : view === 'history' ? (
                            <div className="right-sidebar-scroll" style={{ padding: '8px 16px 16px', gap: '12px', justifyContent: 'flex-start', alignItems: 'stretch' }}>
                                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '8px' }}>
                                    <span style={{ fontSize: '11px', fontWeight: 600, textTransform: 'uppercase', opacity: 0.5 }}>Recent History</span>
                                </div>
                                {chatSessions.length === 0 ? (
                                    <div style={{ padding: '20px', textAlign: 'center', opacity: 0.5, fontSize: '12px' }}>
                                        No archived sessions found.
                                    </div>
                                ) : (
                                    <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                                        {chatSessions.map((session: any) => {
                                            const title = String(session.name || '').replace('session_', '').replace('.aim', '') || 'Conversation';
                                            const ts = session.updated_at ? new Date(session.updated_at * 1000).toLocaleString() : '';
                                            const histBtn: React.CSSProperties = {
                                                display: 'flex', alignItems: 'center', gap: 5, flex: 1, justifyContent: 'center',
                                                padding: '5px 8px', fontSize: 11, fontWeight: 600, cursor: 'pointer',
                                                borderRadius: 6, border: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.12))',
                                                background: 'var(--vscode-button-secondaryBackground, rgba(255,255,255,0.05))',
                                                color: 'var(--vscode-foreground, #ddd)', whiteSpace: 'nowrap',
                                            };
                                            return (
                                                <div
                                                    key={session.path}
                                                    style={{
                                                        padding: '10px 12px', borderRadius: '8px', background: 'rgba(255,255,255,0.03)',
                                                        border: '1px solid rgba(255,255,255,0.05)', transition: 'background 0.2s',
                                                    }}
                                                    onMouseEnter={(e) => e.currentTarget.style.background = 'rgba(255,255,255,0.06)'}
                                                    onMouseLeave={(e) => e.currentTarget.style.background = 'rgba(255,255,255,0.03)'}
                                                >
                                                    <div
                                                        onClick={() => { loadChatSession(session.path); setView('chat'); }}
                                                        style={{ cursor: 'pointer' }}
                                                        title="Open this conversation in the chat panel"
                                                    >
                                                        <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '3px', gap: 8 }}>
                                                            <span style={{ fontSize: '12px', fontWeight: 600, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{title}</span>
                                                            <span style={{ fontSize: '10px', opacity: 0.4, flexShrink: 0 }}>{session.messages} msgs</span>
                                                        </div>
                                                        <div style={{ fontSize: '10px', opacity: 0.5, display: 'flex', alignItems: 'center', gap: 4 }}>
                                                            <i className="codicon codicon-history" style={{ fontSize: 10 }} />{ts}
                                                        </div>
                                                    </div>
                                                    <div style={{ display: 'flex', gap: 6, marginTop: 8 }}>
                                                        <div
                                                            onClick={() => { loadChatSession(session.path); setView('chat'); }}
                                                            style={histBtn}
                                                            title="Open this conversation in the side chat panel"
                                                        >
                                                            <i className="codicon codicon-comment-discussion" style={{ fontSize: 12 }} /> Open in panel
                                                        </div>
                                                        <div
                                                            onClick={() => { createNewSession?.(); setView('chat'); }}
                                                            style={histBtn}
                                                            title="Archive the current chat and start a fresh conversation"
                                                        >
                                                            <i className="codicon codicon-add" style={{ fontSize: 12 }} /> New conversation
                                                        </div>
                                                    </div>
                                                </div>
                                            );
                                        })}
                                    </div>
                                )}
                                <CheckpointTimeline />
                            </div>
                        ) : view === 'dashboard' ? (
                            <MissionControl />
                        ) : view === 'research' ? (
                            <Suspense fallback={<div style={{ padding: 20, opacity: 0.5, fontSize: 11 }}>Loading research center…</div>}>
                                <ResearchCenter />
                            </Suspense>
                        ) : view === 'specs' ? (
                            <Suspense fallback={<div style={{ padding: 20, opacity: 0.5, fontSize: 11 }}>Loading specs…</div>}>
                                <SpecsManager />
                            </Suspense>
                        ) : view === 'rules' ? (
                            <Suspense fallback={<div style={{ padding: 20, opacity: 0.5, fontSize: 11 }}>Loading rules…</div>}>
                                <RulesManager />
                            </Suspense>
                        ) : null}
                    </div>
                )}
            </div>
            {
                view === 'chat' && agentUiMode === 'chat' && (
                    <div style={{ padding: '8px 10px 10px', borderTop: '1px solid var(--vscode-sideBar-border, rgba(255,255,255,0.1))', position: 'relative' }}>
                        {/* @mention dropdown — files + special context sources */}
                        {isMentionDropdownOpen && filteredSuggestions.length > 0 && (
                            <div style={{
                                position: 'absolute', bottom: '100%', left: '10px', right: '10px',
                                background: 'var(--vscode-menu-background, #1e1e2e)',
                                border: '1px solid rgba(168,85,247,0.35)',
                                borderRadius: '10px', overflow: 'hidden',
                                boxShadow: '0 -8px 32px rgba(0,0,0,0.5)',
                                zIndex: 100, marginBottom: '4px',
                            }}>
                                <div style={{ padding: '4px 10px 2px', fontSize: '9px', fontWeight: 700, textTransform: 'uppercase', letterSpacing: '0.08em', color: 'rgba(168,85,247,0.8)', borderBottom: '1px solid rgba(255,255,255,0.06)' }}>
                                    @ Context — type to filter
                                </div>
                                {filteredSuggestions.map((file: any, i) => (
                                    <div
                                        key={file.path}
                                        onMouseDown={() => handleMentionSelect(file)}
                                        style={{
                                            padding: file._special ? '7px 12px' : '6px 12px', cursor: 'pointer', fontSize: '12px',
                                            background: i === selectedMentionIndex ? 'rgba(168,85,247,0.15)' : (file._special ? 'rgba(168,85,247,0.04)' : 'transparent'),
                                            color: i === selectedMentionIndex ? '#c084fc' : (file._special ? '#a78bfa' : 'rgba(255,255,255,0.75)'),
                                            display: 'flex', alignItems: 'center', gap: '8px',
                                            borderLeft: i === selectedMentionIndex ? '2px solid #c084fc' : (file._special ? '2px solid rgba(168,85,247,0.3)' : '2px solid transparent'),
                                            transition: 'all 0.1s',
                                            borderBottom: file._special ? '1px solid rgba(255,255,255,0.04)' : 'none',
                                        }}
                                    >
                                        <i className={`codicon ${file._icon || 'codicon-file'}`} style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px', opacity: file._special ? 0.9 : 0.6 }} />
                                        <span style={{ flex: 1, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap', fontWeight: file._special ? 600 : 400 }}>
                                            {file.name}
                                        </span>
                                        {file._desc && (
                                            <span style={{ fontSize: '9px', opacity: 0.5, whiteSpace: 'nowrap' }}>{file._desc}</span>
                                        )}
                                        {!file._special && (
                                            <span style={{ fontSize: '9px', opacity: 0.35, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap', maxWidth: '80px' }}>
                                                {file.path.split(/[\\/]/).slice(-3, -1).join('/')}
                                            </span>
                                        )}
                                    </div>
                                ))}
                            </div>
                        )}
                        <PlanApprovalBanner />
                        <RestoreCheckpointBanner />
                        {isContinuousMode && (
                            <div style={{
                                display: 'flex', alignItems: 'center', gap: 8, padding: '6px 10px', marginBottom: 6,
                                background: 'rgba(34,211,238,0.08)', border: '1px solid rgba(34,211,238,0.3)',
                                borderRadius: 8, fontSize: 11,
                            }}>
                                <i className="codicon codicon-loading~spin" style={{ fontFamily: 'codicon', fontStyle: 'normal', color: '#22d3ee', fontSize: 12 }} />
                                <span style={{ flex: 1, color: 'rgba(255,255,255,0.85)' }}>
                                    <strong>Continuous Mode</strong> — agent is working autonomously until all tasks complete
                                </span>
                                <button
                                    onClick={() => setContinuousMode(false)}
                                    style={{ background: 'rgba(248,81,73,0.15)', border: '1px solid rgba(248,81,73,0.4)', color: '#f85149', padding: '2px 8px', fontSize: 10, fontWeight: 600, borderRadius: 4, cursor: 'pointer' }}
                                >
                                    Stop
                                </button>
                            </div>
                        )}
                        <MultiFileReviewBanner />
                        <BackgroundAgentsTray />
                        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '6px', padding: '0 4px' }}>
                            <div 
                                onClick={() => setSpecModeActive(!isSpecModeActive)}
                                style={{ 
                                    display: 'flex', 
                                    alignItems: 'center', 
                                    gap: '6px', 
                                    cursor: 'pointer',
                                    fontSize: '11px',
                                    fontWeight: 600,
                                    color: isSpecModeActive ? 'var(--terminator-accent, #00c6ff)' : 'rgba(255,255,255,0.4)',
                                    transition: 'all 0.2s',
                                    userSelect: 'none'
                                }}
                            >
                                <span style={{
                                    width: '28px',
                                    height: '14px',
                                    background: isSpecModeActive ? 'rgba(0, 198, 255, 0.2)' : 'rgba(255,255,255,0.1)',
                                    borderRadius: '10px',
                                    position: 'relative',
                                    display: 'inline-block',
                                    border: `1px solid ${isSpecModeActive ? 'var(--terminator-accent, #00c6ff)' : 'rgba(255,255,255,0.15)'}`
                                }}>
                                    <span style={{
                                        width: '10px',
                                        height: '10px',
                                        background: isSpecModeActive ? 'var(--terminator-accent, #00c6ff)' : 'rgba(255,255,255,0.4)',
                                        borderRadius: '50%',
                                        position: 'absolute',
                                        top: '1px',
                                        left: isSpecModeActive ? '15px' : '2px',
                                        transition: 'all 0.2s'
                                    }} />
                                </span>
                                <span>📋 SPEC MODE</span>
                            </div>
                            {isSpecModeActive && (
                                <span style={{ fontSize: '10px', opacity: 0.6, color: 'var(--terminator-accent, #00c6ff)', textShadow: '0 0 8px rgba(0, 198, 255, 0.3)', animation: 'pulse-blue 1.5s infinite' }}>
                                    Requirements Engine Active
                                </span>
                            )}
                        </div>
                        <div style={{
                            background: 'var(--vscode-input-background)',
                            border: `1px solid ${isSpecModeActive ? 'var(--terminator-accent, #00c6ff)' : 'var(--vscode-input-border, transparent)'}`,
                            boxShadow: isSpecModeActive ? '0 0 10px rgba(0, 198, 255, 0.25)' : 'none',
                            borderRadius: '8px', padding: '7px 10px', display: 'flex', flexDirection: 'column',
                            transition: 'all 0.2s'
                        }}>
                            <textarea
                                ref={inputRef} value={inputValue} onChange={handleInputChange} onKeyDown={handleKeyDown}
                                className="agent-mission-input"
                                placeholder={isAgentThinking ? 'Agent executing...' : isSpecModeActive ? 'Describe the feature to auto-generate requirements & tasks...' : 'Launch a mission...  (type @ to mention a file)'}
                                disabled={isAgentThinking}
                                style={{ background: 'transparent', border: 'none', outline: 'none', color: 'var(--vscode-editor-foreground, #fff)', resize: 'none', fontSize: '13px', lineHeight: '1.45', width: '100%', minHeight: '28px', opacity: isAgentThinking ? 0.5 : 1 }}
                            />
                            {attachedFiles.length > 0 && (
                                <div style={{ display: 'flex', flexWrap: 'wrap', gap: '6px', marginTop: '4px', paddingBottom: '4px' }}>
                                    {attachedFiles.map((item, i) => (
                                        <div key={item.id || i} style={{
                                            display: 'flex', alignItems: 'center', gap: '6px', padding: '3px 8px',
                                            background: 'rgba(255,255,255,0.08)', borderRadius: '6px', fontSize: '11px',
                                            border: '1px solid rgba(255,255,255,0.1)', color: 'rgba(255,255,255,0.9)'
                                        }}>
                                            {item.thumbnail ? (
                                                <img src={item.thumbnail} style={{ width: '16px', height: '16px', borderRadius: '3px', objectFit: 'cover' }} alt="" />
                                            ) : (
                                                <span style={{ opacity: 0.7, fontSize: '10px' }}>{item.type === 'attachment' ? 'IMG' : '{ }'}</span>
                                            )}
                                            <span style={{ fontWeight: 500 }}>{item.name}</span>
                                            <i className="codicon codicon-close" onClick={() => removeFile(item.path)} style={{ fontFamily: 'codicon', fontStyle: 'normal', cursor: 'pointer', opacity: 0.5, marginLeft: '2px', fontSize: '10px' }}></i>
                                        </div>
                                    ))}
                                    {isAttaching && (
                                        <div style={{
                                            display: 'flex', alignItems: 'center', gap: '6px', padding: '3px 8px',
                                            background: 'rgba(255,255,255,0.05)', borderRadius: '6px', fontSize: '11px',
                                            border: '1px dashed rgba(255,255,255,0.2)', color: 'rgba(255,255,255,0.5)',
                                            fontStyle: 'italic'
                                        }}>
                                            <i className="codicon codicon-loading codicon-modifier-spin" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '10px' }}></i>
                                            <span>Neuralizing...</span>
                                        </div>
                                    )}
                                </div>
                            )}
                            {!attachedFiles.length && isAttaching && (
                                <div style={{ display: 'flex', flexWrap: 'wrap', gap: '6px', marginTop: '4px', paddingBottom: '4px' }}>
                                    <div style={{
                                        display: 'flex', alignItems: 'center', gap: '6px', padding: '3px 8px',
                                        background: 'rgba(255,255,255,0.05)', borderRadius: '6px', fontSize: '11px',
                                        border: '1px dashed rgba(255,255,255,0.2)', color: 'rgba(255,255,255,0.5)',
                                        fontStyle: 'italic'
                                    }}>
                                        <i className="codicon codicon-loading codicon-modifier-spin" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '10px' }}></i>
                                        <span>Neuralizing...</span>
                                    </div>
                                </div>
                            )}
                            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginTop: '6px' }}>
                                <div style={{ display: 'flex', gap: '8px', alignItems: 'center' }}>
                                    <div onClick={handleAttachFile} style={{ cursor: 'pointer', opacity: 0.5, display: 'flex', alignItems: 'center' }} className="hoverable-bg" title="Attach File (Neural Gist)">
                                        <i className="codicon codicon-attach" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '13px' }}></i>
                                    </div>
                                    <span
                                        onClick={onModeClick}
                                        style={{
                                            fontSize: '10px',
                                            cursor: 'pointer',
                                            padding: '1px 6px',
                                            borderRadius: '4px',
                                            fontWeight: 600,
                                            color: modeStyle.color,
                                            background: modeStyle.background,
                                            border: modeStyle.border,
                                        }}
                                        title={modeStyle.title}
                                        className="hoverable-bg"
                                    >
                                        {modeStyle.label}
                                    </span>
                                    <span onClick={onModelClick} style={{ fontSize: '10px', opacity: model ? 0.5 : 0.9, cursor: 'pointer', color: model ? undefined : '#f59e0b' }} className="hoverable-bg">{modelLabel}</span>
                                    {/* Void: Reasoning toggle */}
                                    <ReasoningToggle />
                                    
                                    <div
                                        onClick={() => setTtsEnabled(!ttsEnabled)}
                                        style={{ cursor: 'pointer', opacity: ttsEnabled ? 0.9 : 0.4, display: 'flex', alignItems: 'center', color: ttsEnabled ? '#c084fc' : undefined }}
                                        className="hoverable-bg"
                                        title={ttsEnabled ? 'Vocal Mode ON' : 'Muted'}
                                    >
                                        {ttsEnabled ? <Volume2 size={14} /> : <VolumeX size={14} />}
                                    </div>
                                    {ttsEnabled && (
                                        <select
                                            value={ttsPreset}
                                            onChange={(e) => setTtsPreset(e.target.value as any)}
                                            style={{
                                                height: '20px',
                                                background: 'rgba(255,255,255,0.04)',
                                                border: '1px solid rgba(255,255,255,0.10)',
                                                borderRadius: '4px',
                                                color: 'rgba(255,255,255,0.72)',
                                                fontSize: '9px',
                                                outline: 'none',
                                            }}
                                            title="Voice Preset"
                                        >
                                            <option value="airi">Airi (EN)</option>
                                            <option value="sage">Sage (EN)</option>
                                            <option value="nova">Nova (EN)</option>
                                            <option value="aria">Aria (EN)</option>
                                            <option value="kawaii">Kawaii (JP)</option>
                                            <option value="yamato">Yamato (JP)</option>
                                            <option value="hana">Hana (JP)</option>
                                            <option value="ren">Ren (JP)</option>
                                            <option value="yuki">Yuki (JP)</option>
                                            <option value="haru">Haru (JP)</option>
                                            <option value="sora">Sora (JP)</option>
                                            <option value="zero">Zero (JP)</option>
                                        </select>
                                    )}

                                    {webUiProviderKey && (
                                        <div style={{ display: 'flex', alignItems: 'center', gap: '4px' }}>
                                            <select
                                                value={activeWebuiSessionId || ''}
                                                onChange={async (e) => {
                                                    const val = e.target.value;
                                                    if (val === 'add_account') {
                                                        try {
                                                            const label = window.prompt('Account slot name', `free-${webuiSessions.filter(s => s.provider === webUiProviderKey).length + 1}`)?.trim();
                                                            const account = (label || 'default').replace(/[^a-zA-Z0-9_.-]/g, '_') || 'default';
                                                            await invoke('start_webui_login', { request: { provider: `${webUiProviderKey}:${account}` } });
                                                            await refreshWebuiSessions(webUiProviderKey);
                                                        } catch (err) {
                                                            console.error('Failed to trigger login:', err);
                                                        }
                                                    } else if (val) {
                                                        await switchWebuiSession(val);
                                                    }
                                                }}
                                                title="WebUI account switcher"
                                                style={{
                                                    height: '20px',
                                                    maxWidth: '120px',
                                                    background: 'rgba(255,255,255,0.04)',
                                                    border: '1px solid rgba(255,255,255,0.10)',
                                                    borderRadius: '4px',
                                                    color: 'rgba(255,255,255,0.72)',
                                                    fontSize: '10px',
                                                    outline: 'none',
                                                }}
                                            >
                                                {webuiSessions
                                                    .filter(s => s.provider === webUiProviderKey)
                                                    .map(s => (
                                                        <option key={s.session_id} value={s.session_id}>
                                                            {s.display_name} {s.is_active ? '★' : ''}
                                                        </option>
                                                    ))
                                                }
                                                <option value="add_account">+ Add Account...</option>
                                            </select>
                                            {activeWebuiSessionId && (
                                                <div
                                                    onClick={async () => {
                                                        try {
                                                            await invoke('toggle_webui_window_visibility', { sessionId: activeWebuiSessionId });
                                                        } catch (err) {
                                                            console.error('Failed to toggle window:', err);
                                                        }
                                                    }}
                                                    style={{ cursor: 'pointer', opacity: 0.8, display: 'flex', alignItems: 'center', padding: '2px 4px', background: 'rgba(255,255,255,0.06)', borderRadius: '4px' }}
                                                    title="Show/Hide Session Window"
                                                >
                                                    <i className="codicon codicon-eye" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px' }}></i>
                                                </div>
                                            )}
                                            {activeWebuiSessionId && (
                                                <i
                                                    className="codicon codicon-trash"
                                                    onClick={async () => {
                                                        if (confirm('Delete this account session?')) {
                                                            await deleteWebuiSession(activeWebuiSessionId);
                                                        }
                                                    }}
                                                    style={{
                                                        fontFamily: 'codicon',
                                                        fontStyle: 'normal',
                                                        cursor: 'pointer',
                                                        opacity: 0.5,
                                                        fontSize: '11px',
                                                        padding: '2px',
                                                    }}
                                                    title="Delete session"
                                                />
                                            )}
                                        </div>
                                    )}
                                    {/* Token counter */}
                                    <span style={{ fontSize: '9px', opacity: 0.35, fontVariantNumeric: 'tabular-nums' }} title="Estimated context tokens">
                                        ~{Math.round(messages.reduce((n, m) => n + (typeof m.content === 'string' ? m.content.length : 0), 0) / 4).toLocaleString()}t
                                    </span>
                                </div>
                                <div style={{ display: 'flex', gap: '8px', alignItems: 'center' }}>
                                    <div
                                        onClick={toggleVoiceInput}
                                        style={{
                                            cursor: 'pointer',
                                            display: 'flex', alignItems: 'center', justifyContent: 'center',
                                            width: '24px', height: '24px', borderRadius: '4px',
                                            background: isVoiceListening ? 'rgba(239,68,68,0.2)' : 'rgba(255,255,255,0.05)',
                                            color: isVoiceListening ? '#ef4444' : 'rgba(255,255,255,0.5)',
                                            border: isVoiceListening ? '1px solid rgba(239,68,68,0.4)' : '1px solid transparent',
                                            animation: isVoiceListening ? 'pulse 1.5s infinite' : 'none'
                                        }}
                                        title={isVoiceListening ? "Stop Dictation" : "Start Voice Dictation"}
                                    >
                                        {isVoiceListening ? <MicOff size={14} /> : <Mic size={14} />}
                                    </div>
                                    <div style={{
                                        display: 'flex',
                                        gap: '10px',
                                        alignItems: 'center',
                                        marginRight: '12px',
                                        padding: '4px 10px',
                                        background: 'rgba(255,255,255,0.03)',
                                        borderRadius: '6px',
                                        border: '1px solid rgba(255,255,255,0.05)',
                                        transition: 'all 0.2s'
                                    }}>
                                        <div
                                            onClick={() => isAgentPaused ? import('../agent').then(m => m.resumeAgent()) : import('../agent').then(m => m.pauseAgent())}
                                            style={{ cursor: 'pointer', color: isAgentPaused ? '#10b981' : '#f59e0b', display: 'flex', alignItems: 'center' }}
                                            title={isAgentPaused ? "Resume Agent" : "Pause Agent"}
                                        >
                                            <i className={`codicon codicon-${isAgentPaused ? 'play' : 'debug-pause'}`} style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '14px' }}></i>
                                        </div>
                                        <div
                                            onClick={() => import('../agent').then(m => m.stopAgent())}
                                            style={{ cursor: 'pointer', color: '#ef4444', display: 'flex', alignItems: 'center' }}
                                            title="Stop Agent (Terminate)"
                                        >
                                            <i className="codicon codicon-primitive-square" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '14px' }}></i>
                                        </div>
                                        {/* Auto-accept toggle */}
                                        <div
                                            onClick={() => setAutoAcceptChanges(!autoAcceptChanges)}
                                            style={{
                                                cursor: 'pointer',
                                                color: autoAcceptChanges ? '#10b981' : 'rgba(255,255,255,0.35)',
                                                display: 'flex', alignItems: 'center', gap: '3px',
                                                fontSize: '10px', fontWeight: 600,
                                                padding: '1px 5px',
                                                borderRadius: '4px',
                                                background: autoAcceptChanges ? 'rgba(16,185,129,0.15)' : 'transparent',
                                                border: autoAcceptChanges ? '1px solid rgba(16,185,129,0.4)' : '1px solid transparent',
                                                transition: 'all 0.2s'
                                            }}
                                            title={autoAcceptChanges ? "Auto-accept ON — agent edits apply instantly" : "Auto-accept OFF — review each diff before applying"}
                                        >
                                            <i className="codicon codicon-check-all" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '11px' }}></i>
                                            <span>AUTO</span>
                                        </div>
                                        {/* Checkpoint restore */}
                                        {checkpoint && (
                                            <div
                                                onClick={() => { revertToCheckpoint(); }}
                                                style={{
                                                    cursor: 'pointer',
                                                    color: '#f59e0b',
                                                    display: 'flex', alignItems: 'center', gap: '3px',
                                                    fontSize: '10px', fontWeight: 600,
                                                    padding: '1px 5px',
                                                    borderRadius: '4px',
                                                    background: 'rgba(245,158,11,0.12)',
                                                    border: '1px solid rgba(245,158,11,0.35)',
                                                    transition: 'all 0.2s'
                                                }}
                                                title="Restore checkpoint — undo all agent file changes"
                                            >
                                                <i className="codicon codicon-history" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '11px' }}></i>
                                                <span>UNDO</span>
                                            </div>
                                        )}
                                        <div
                                            onClick={() => import('../agent').then(m => m.setYoloMode(!isYoloMode).then(() => setYoloMode(!isYoloMode)))}
                                            style={{
                                                cursor: 'pointer',
                                                color: isYoloMode ? '#f97316' : 'rgba(255,255,255,0.35)',
                                                display: 'flex', alignItems: 'center', gap: '3px',
                                                fontSize: '10px', fontWeight: 600,
                                                padding: '1px 5px',
                                                borderRadius: '4px',
                                                background: isYoloMode ? 'rgba(249,115,22,0.15)' : 'transparent',
                                                border: isYoloMode ? '1px solid rgba(249,115,22,0.4)' : '1px solid transparent',
                                                transition: 'all 0.2s'
                                            }}
                                            title={isYoloMode ? "YOLO MODE ON — Click to disable" : "Enable YOLO Mode (full autonomy, no blockers)"}
                                        >
                                            <span style={{ fontStyle: 'normal' }}>⚡</span>
                                            <span>YOLO</span>
                                        </div>
                                    </div>
                                    <div onClick={() => onSend()} style={{
                                        width: '24px', height: '24px', borderRadius: '50%',
                                        background: (inputValue.trim() || attachedFiles.length > 0) ? '#fff' : 'rgba(255,255,255,0.1)',
                                        color: '#000', display: 'flex', alignItems: 'center', justifyContent: 'center', cursor: 'pointer'
                                    }}>
                                        <i className="codicon codicon-arrow-right" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px' }}></i>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                )
            }

            {/* Ollama Progress Bar - Small, toggleable */}
            <OllamaProgressBar />

            {/* Emulator Panel - Optional section in right sidebar */}
            {emulatorLayout === 'right' && (
                <div style={{
                    display: 'flex',
                    flexDirection: 'column',
                    flexShrink: 0,
                    borderTop: '1px solid var(--vscode-sideBar-border, rgba(255,255,255,0.05))',
                    minHeight: '300px',
                    maxHeight: '50%',
                    overflow: 'hidden'
                }}>
                    <div style={{
                        display: 'flex',
                        justifyContent: 'space-between',
                        alignItems: 'center',
                        padding: '6px 10px',
                        background: 'var(--vscode-sideBarSectionHeader-background, rgba(255,255,255,0.02))',
                        fontSize: '11px',
                        fontWeight: 600,
                        textTransform: 'uppercase',
                        letterSpacing: '0.05em',
                        color: 'var(--vscode-sideBar-foreground)',
                        opacity: 0.8,
                        cursor: 'pointer'
                    }}
                        onClick={() => setShowEmulatorInRight(!showEmulatorInRight)}
                    >
                        <span>📱 Emulator</span>
                        <span style={{ fontSize: '10px', opacity: 0.6 }}>
                            {showEmulatorInRight ? '▼' : '▶'}
                        </span>
                    </div>
                    {showEmulatorInRight && (
                        <div style={{ flex: 1, overflow: 'hidden', display: 'flex', flexDirection: 'column' }}>
                            <EmulatorPanel />
                        </div>
                    )}
                </div>
            )}
        </aside >
    );
};

export default RightSidebar;
