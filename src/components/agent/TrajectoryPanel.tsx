import React, { useMemo, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useStore } from '../../store';

// ─────────────────────────────────────────────────────────────────────────────
//  TrajectoryPanel — Antigravity-style agent timeline.
//
//  Renders the trajectory log captured by the right-sidebar listeners.
//  Each user turn is a collapsible block; inside the block we list every
//  tool call, tool result, error, and phase event the agent emitted, in
//  order, with timestamps. Clicking an event reveals its full payload.
//
//  Auto-scrolls to the bottom while the agent is thinking (so the user
//  watches the current turn unfold live), and locks scroll when they
//  manually scroll up — same UX as the chat itself.
// ─────────────────────────────────────────────────────────────────────────────

const KIND_BADGE: Record<string, { color: string; bg: string; icon: string }> = {
    tool_call:   { color: '#60a5fa', bg: 'rgba(96,165,250,0.14)',  icon: 'play' },
    tool_result: { color: '#22c55e', bg: 'rgba(34,197,94,0.14)',   icon: 'check' },
    content:     { color: '#a78bfa', bg: 'rgba(167,139,250,0.14)', icon: 'comment' },
    phase:       { color: '#fbbf24', bg: 'rgba(251,191,36,0.14)',  icon: 'milestone' },
    error:       { color: '#f87171', bg: 'rgba(248,113,113,0.14)', icon: 'error' },
};

const TrajectoryPanel: React.FC = () => {
    const open = useStore(s => s.isTrajectoryOpen);
    const close = useStore(s => s.closeTrajectory);
    const events = useStore(s => s.agentTrajectory);
    const activeRoot = useStore(s => s.activeRoot);
    const cascadeId = useStore(s => s.activeCascadeId);
    const clear = useStore(s => s.clearTrajectory);
    const [expandedEvt, setExpandedEvt] = useState<string | null>(null);
    const [filter, setFilter] = useState<'all' | 'tools' | 'errors'>('all');
    const [exportMsg, setExportMsg] = useState('');

    const filtered = useMemo(() => {
        if (filter === 'tools') return events.filter(e => e.kind === 'tool_call' || e.kind === 'tool_result');
        if (filter === 'errors') return events.filter(e => e.kind === 'error' || e.success === false);
        return events;
    }, [events, filter]);

    // Group by turn number — each user message starts a new turn.
    const groups = useMemo(() => {
        const m = new Map<number, typeof events>();
        for (const e of filtered) {
            const t = e.turn ?? 0;
            if (!m.has(t)) m.set(t, []);
            m.get(t)!.push(e);
        }
        return Array.from(m.entries()).sort((a, b) => b[0] - a[0]); // newest turn on top
    }, [filtered]);

    if (!open) return null;

    const onExportJsonl = async () => {
        if (!activeRoot || !cascadeId) {
            setExportMsg('Start an agent run first (no cascade id).');
            return;
        }
        try {
            const path = await invoke<string>('ag_export_trajectory_jsonl', { root: activeRoot, cascadeId });
            setExportMsg(`Exported → ${path}`);
        } catch (e) {
            setExportMsg(String(e));
        }
    };

    return (
        <div
            style={{
                position: 'fixed',
                inset: 0,
                background: 'rgba(0,0,0,0.55)',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                zIndex: 9000,
            }}
            onClick={close}
        >
            <div
                onClick={(e) => e.stopPropagation()}
                style={{
                    width: 'min(900px, 92vw)',
                    height: 'min(740px, 92vh)',
                    background: 'var(--vscode-editor-background, #1e1e1e)',
                    color: 'var(--vscode-foreground, #ddd)',
                    border: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.08))',
                    borderRadius: 8,
                    display: 'flex',
                    flexDirection: 'column',
                    overflow: 'hidden',
                    boxShadow: '0 24px 48px rgba(0,0,0,0.5)',
                }}
            >
                {/* Header */}
                <div
                    style={{
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'space-between',
                        padding: '10px 16px',
                        borderBottom: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.08))',
                    }}
                >
                    <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                        <i className="codicon codicon-timeline-view-icon" style={{ fontFamily: 'codicon', fontStyle: 'normal' }} />
                        <span style={{ fontWeight: 600, fontSize: 13 }}>Agent Trajectory</span>
                        <span style={{ fontSize: 11, opacity: 0.55, marginLeft: 8 }}>
                            {events.length} events · {groups.length} turn{groups.length === 1 ? '' : 's'}
                        </span>
                    </div>
                    <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
                        <select
                            value={filter}
                            onChange={(e) => setFilter(e.target.value as any)}
                            style={{ fontSize: 11, padding: '2px 6px' }}
                        >
                            <option value="all">All events</option>
                            <option value="tools">Tool calls only</option>
                            <option value="errors">Errors only</option>
                        </select>
                        <button onClick={() => void onExportJsonl()} style={btnNeutral} disabled={!activeRoot || !cascadeId}>
                            <i className="codicon codicon-export" style={iconStyle} /> Export JSONL
                        </button>
                        <button onClick={clear} style={btnNeutral}>
                            <i className="codicon codicon-clear-all" style={iconStyle} /> Clear
                        </button>
                        <button onClick={close} style={btnNeutral}>
                            <i className="codicon codicon-close" style={iconStyle} /> Close
                        </button>
                    </div>
                </div>

                {exportMsg && (
                    <div style={{ padding: '4px 16px', fontSize: 11, opacity: 0.7, borderBottom: '1px solid rgba(255,255,255,0.06)' }}>
                        {exportMsg}
                    </div>
                )}

                {/* Body */}
                <div style={{ flex: '1 1 auto', overflowY: 'auto', padding: 16 }}>
                    {groups.length === 0 && (
                        <div style={{ opacity: 0.55, fontSize: 12, padding: 12 }}>
                            No trajectory events yet — start a chat turn to populate the timeline.
                        </div>
                    )}
                    {groups.map(([turn, evts]) => (
                        <div key={turn} style={{ marginBottom: 18 }}>
                            <div style={{
                                fontSize: 11, textTransform: 'uppercase', letterSpacing: 0.5,
                                opacity: 0.55, marginBottom: 6,
                            }}>
                                Turn {turn} · {evts.length} events
                            </div>
                            <div style={{ position: 'relative', paddingLeft: 16 }}>
                                {/* Vertical timeline line */}
                                <div style={{
                                    position: 'absolute',
                                    left: 6,
                                    top: 6,
                                    bottom: 6,
                                    width: 1,
                                    background: 'rgba(255,255,255,0.08)',
                                }} />
                                {evts.map((evt) => {
                                    const badge = KIND_BADGE[evt.kind] || KIND_BADGE.tool_call;
                                    const open = expandedEvt === evt.id;
                                    return (
                                        <div key={evt.id} style={{ position: 'relative', marginBottom: 4 }}>
                                            <div
                                                style={{
                                                    position: 'absolute',
                                                    left: -16,
                                                    top: 5,
                                                    width: 12,
                                                    height: 12,
                                                    borderRadius: '50%',
                                                    background: badge.bg,
                                                    border: `1px solid ${badge.color}`,
                                                    display: 'flex',
                                                    alignItems: 'center',
                                                    justifyContent: 'center',
                                                }}
                                            >
                                                <i
                                                    className={`codicon codicon-${badge.icon}`}
                                                    style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: 7, color: badge.color }}
                                                />
                                            </div>
                                            <div
                                                onClick={() => setExpandedEvt(open ? null : evt.id)}
                                                style={{
                                                    display: 'flex',
                                                    alignItems: 'center',
                                                    gap: 6,
                                                    padding: '2px 6px',
                                                    fontSize: 12,
                                                    cursor: evt.detail ? 'pointer' : 'default',
                                                    color: evt.success === false ? '#f87171' : 'inherit',
                                                }}
                                            >
                                                <span style={{
                                                    fontSize: 10,
                                                    fontFamily: 'var(--font-mono, monospace)',
                                                    opacity: 0.5,
                                                    minWidth: 60,
                                                }}>
                                                    {new Date(evt.ts).toLocaleTimeString()}
                                                </span>
                                                <span>{evt.title}</span>
                                                {evt.detail && (
                                                    <i
                                                        className={`codicon codicon-${open ? 'chevron-down' : 'chevron-right'}`}
                                                        style={{ ...iconStyle, opacity: 0.4 }}
                                                    />
                                                )}
                                            </div>
                                            {open && evt.detail && (
                                                <pre style={{
                                                    margin: '4px 6px 10px 6px',
                                                    padding: 8,
                                                    background: 'rgba(0,0,0,0.3)',
                                                    border: '1px solid rgba(255,255,255,0.06)',
                                                    borderRadius: 4,
                                                    fontSize: 11,
                                                    fontFamily: 'var(--font-mono, monospace)',
                                                    whiteSpace: 'pre-wrap',
                                                    wordBreak: 'break-word',
                                                    overflow: 'auto',
                                                    maxHeight: 240,
                                                }}>{evt.detail}</pre>
                                            )}
                                        </div>
                                    );
                                })}
                            </div>
                        </div>
                    ))}
                </div>
            </div>
        </div>
    );
};

const btnNeutral: React.CSSProperties = {
    display: 'inline-flex',
    alignItems: 'center',
    gap: 4,
    padding: '3px 10px',
    borderRadius: 4,
    fontSize: 11,
    cursor: 'pointer',
    fontFamily: 'inherit',
    background: 'rgba(255,255,255,0.06)',
    border: '1px solid rgba(255,255,255,0.1)',
    color: 'rgba(255,255,255,0.85)',
};

const iconStyle: React.CSSProperties = {
    fontFamily: 'codicon',
    fontStyle: 'normal',
    fontSize: 11,
};

export default TrajectoryPanel;
