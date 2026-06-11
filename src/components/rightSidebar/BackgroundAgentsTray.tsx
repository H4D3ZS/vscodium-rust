// Background agents tray: spawn/track parallel background agent runs.
// Extracted from RightSidebar.tsx (A2 god-component split).
import React, { useEffect, useState, useRef, useMemo, useCallback, memo } from 'react';
import { useStore } from '../../store';
import { invoke } from '../../tauri_bridge';


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
});


// One-shot AIRI bootstrap latch. Module-scoped so it survives unmount/remount
// and (critically) React.StrictMode's deliberate double-invoke of effects.


export { BackgroundAgentsTray };
