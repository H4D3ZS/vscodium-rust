// Background agents tray: spawn/track parallel background agent runs.
// Extracted from RightSidebar.tsx (A2 god-component split).
import React, { useEffect, useState, useRef, useMemo, useCallback, memo } from 'react';
import { useStore } from '../../store';
import { invoke } from '../../tauri_bridge';
import MissionControl from '../../presentation/supervisor/MissionControl';


// Compact strip listing any agent runs the user fired with `/bg <prompt>`
// or via `runBackgroundAgent`. Doesn't block the main chat.
const BackgroundAgentsTray: React.FC = memo(() => {
    const bgAgents = useStore(state => state.backgroundAgents);
    const remove = useStore(state => state.removeBackgroundAgent);
    const runBackground = useStore(state => state.runBackgroundAgent);
    const clearAll = useStore(state => state.clearBackgroundAgents);
    const [expandedId, setExpandedId] = useState<string | null>(null);
    const [spawnPrompt, setSpawnPrompt] = useState('');
    const [spawning, setSpawning] = useState(false);
    const [showSpawn, setShowSpawn] = useState(false);
    const [showMission, setShowMission] = useState(false);

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
    const PARALLEL_AGENTS_ENABLED = true;
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
                <i className="codicon codicon-pulse" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: 12, color: running > 0? '#60a5fa': 'rgba(255,255,255,0.4)' }} />
                <span style={{ fontWeight: 600, flex: 1, opacity: 0.7, fontSize: 11 }}>
                    Background Agents
                    {running > 0 && <span style={{ marginLeft: 4, color: '#60a5fa', fontSize: 9, fontWeight: 500 }}>{running} running</span>}
                </span>
                {done > 0 && (
                    <span onClick={clearAll} style={{ cursor: 'pointer', fontSize: 9, opacity: 0.4, padding: '1px 4px', borderRadius: 3 }} title="Clear finished">clear</span>
                )}
                <span
                    onClick={() => setShowMission(true)}
                    style={{ cursor: 'pointer', fontSize: 12, lineHeight: 1, marginRight: 2 }}
                    title="Open Mission Control (24/7 autonomous supervisor)"
                ></span>
                <span
                    onClick={() => setShowSpawn(v => !v)}
                    style={{
                        cursor: 'pointer', fontSize: 14, lineHeight: 1, color: '#60a5fa',
                        width: 20, height: 20, display: 'flex', alignItems: 'center', justifyContent: 'center',
                        borderRadius: 4, transition: 'background 0.1s',
                    }}
                    title="Spawn new background agent"
                    onMouseEnter={(e) => { (e.target as HTMLElement).style.background = 'rgba(96,165,250,0.12)'; }}
                    onMouseLeave={(e) => { (e.target as HTMLElement).style.background = 'transparent'; }}
                >+</span>
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
                    >{spawning? '…': ''}</button>
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
                    const isRunning = bg.status === 'running';
                    const isError = bg.status === 'error';
                    const isDone = bg.status === 'done';
                    const color = isDone? '#22c55e': isError? '#f87171': '#60a5fa';
                    return (
                        <div key={bg.id} style={{
                            display: 'flex', flexDirection: 'column', gap: 2,
                            padding: '5px 8px', background: 'rgba(0,0,0,0.2)', borderRadius: 6,
                            border: isRunning? '1px solid rgba(96,165,250,0.2)': '1px solid transparent',
                        }}>
                            <div style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
                                <span style={{
                                    width: 6, height: 6, borderRadius: '50%', background: color, flexShrink: 0,
                                    ...(isRunning? { animation: 'hubPulse 1s infinite' }: {}),
                                }} />
                                <span style={{
                                    flex: 1, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap',
                                    fontSize: 11, fontWeight: isRunning? 500: 400,
                                }} title={bg.prompt}>
                                    {bg.prompt.length > 45? bg.prompt.slice(0, 45) + '...': bg.prompt}
                                </span>
                                {(() => {
                                    const end = bg.finishedAt ?? Date.now();
                                    const secs = Math.max(0, Math.round((end - bg.startedAt) / 1000));
                                    const mins = Math.floor(secs / 60);
                                    const display = mins > 0? `${mins}m ${secs % 60}s`: `${secs}s`;
                                    return <span style={{ opacity: 0.4, fontSize: 9, fontVariantNumeric: 'tabular-nums' }}>{display}</span>;
                                })()}
                                <i
                                    className={`codicon codicon-${open? 'chevron-up': 'chevron-down'}`}
                                    onClick={() => setExpandedId(open? null: bg.id)}
                                    style={{ fontFamily: 'codicon', fontStyle: 'normal', cursor: 'pointer', fontSize: 10, opacity: 0.5 }}
                                />
                                <i
                                    className="codicon codicon-close"
                                    onClick={() => remove(bg.id)}
                                    style={{ fontFamily: 'codicon', fontStyle: 'normal', cursor: 'pointer', fontSize: 10, opacity: 0.4 }}
                                />
                            </div>
                            {/* Progress bar for running agents */}
                            {isRunning && (
                                <div style={{
                                    height: 2, borderRadius: 1, background: 'rgba(96,165,250,0.15)', overflow: 'hidden',
                                }}>
                                    <div style={{
                                        height: '100%', borderRadius: 1, width: '40%',
                                        background: 'linear-gradient(90deg, #3b82f6, #60a5fa, #3b82f6)',
                                        backgroundSize: '200% 100%',
                                        animation: 'progressSlide 2s ease-in-out infinite',
                                    }} />
                                </div>
                            )}
                            {open && bg.result && (
                                <pre style={{
                                    margin: 0, marginTop: 2, fontSize: 10, lineHeight: 1.4,
                                    fontFamily: 'var(--font-mono)',
                                    opacity: isError? 0.9: 0.7, maxHeight: 160, overflow: 'auto',
                                    whiteSpace: 'pre-wrap', wordBreak: 'break-word',
                                    color: isError? '#f87171': 'rgba(255,255,255,0.65)',
                                }}>
                                    {bg.result.slice(0, 4000)}
                                </pre>
                            )}
                        </div>
                    );
                })}
            </div>

            {/* Mission Control overlay — 24/7 autonomous supervisor cockpit. */}
            {showMission && (
                <div
                    onClick={e => { if (e.target === e.currentTarget) setShowMission(false); }}
                    style={{
                        position: 'fixed', inset: 0, background: 'rgba(0,0,0,0.6)',
                        display: 'flex', alignItems: 'center', justifyContent: 'center', zIndex: 9998,
                    }}
                >
                    <div style={{
                        width: 'min(880px, 92vw)', height: 'min(680px, 88vh)',
                        borderRadius: 10, overflow: 'hidden', border: '1px solid rgba(255,255,255,0.12)',
                        boxShadow: '0 12px 48px rgba(0,0,0,0.5)', position: 'relative',
                    }}>
                        <span
                            onClick={() => setShowMission(false)}
                            style={{ position: 'absolute', top: 8, right: 12, zIndex: 10000, cursor: 'pointer', color: '#8b949e', fontSize: 16 }}
                            title="Close"
                        ></span>
                        <MissionControl />
                    </div>
                </div>
            )}
        </div>
    );
});


// One-shot AIRI bootstrap latch. Module-scoped so it survives unmount/remount
// and (critically) React.StrictMode's deliberate double-invoke of effects.


export { BackgroundAgentsTray };
