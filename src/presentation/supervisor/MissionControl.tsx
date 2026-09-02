/**
 * MissionControl — cockpit for the 24/7 autonomous supervisor + WebUI bridge.
 *
 * Queue tasks, watch them run unattended against free web-chat models, and see the
 * verify/commit outcome. Reads supervisor_status + listens for supervisor:update;
 * polls webui_bridge_status for the connected-provider strip.
 *
 * Zero heavy animation — CSS Grid/Flexbox, consistent with TriageWorkbench/ApiProbe.
 */

import React, { useCallback, useEffect, useState } from 'react';
import { invoke, listen } from '../../tauri_bridge.ts';

// ── Types (mirror Rust) ───────────────────────────────────────────────────────

type TaskStatus = 'queued' | 'running' | 'verifying' | 'committed' | 'failed';

interface SupervisorTask {
    id: string;
    prompt: string;
    status: TaskStatus;
    provider?: string;
    branch?: string;
    commit?: string;
    created_ns: number;
    attempts: number;
    last_error?: string;
}

interface SupervisorConfig {
    max_steps_per_task: number;
    task_timeout_secs: number;
    max_attempts: number;
    provider_cooldown_secs: number;
    branch_prefix: string;
    auto_commit: boolean;
}

interface SupervisorSnapshot {
    paused: boolean;
    running?: SupervisorTask | null;
    queue: SupervisorTask[];
    history: SupervisorTask[];
    config: SupervisorConfig;
}

interface TabStatus {
    tab_id: string;
    provider: string;
    connected_secs: number;
    cooling: boolean;
}

// ── Colours ───────────────────────────────────────────────────────────────────

const STATUS_COLOR: Record<TaskStatus, string> = {
    queued: '#8b949e',
    running: '#79c0ff',
    verifying: '#d29922',
    committed: '#3fb950',
    failed: '#f85149',
};

const PROVIDER_COLOR: Record<string, string> = {
    claude: '#d2a8ff',
    openai: '#3fb950',
    gemini: '#79c0ff',
    deepseek: '#58a6ff',
    qwen: '#ff7b72',
};

// ── Small components ──────────────────────────────────────────────────────────

const StatusPill: React.FC<{ status: TaskStatus }> = ({ status }) => (
    <span style={{
        padding: '1px 6px',
        borderRadius: 3,
        fontSize: 9,
        fontFamily: 'monospace',
        fontWeight: 700,
        letterSpacing: '0.04em',
        color: STATUS_COLOR[status],
        background: `${STATUS_COLOR[status]}18`,
        textTransform: 'uppercase',
    }}>
        {status}
    </span>
);

const TaskRow: React.FC<{ task: SupervisorTask; onSkip?: (id: string) => void }> = ({ task, onSkip }) => {
    const [open, setOpen] = useState(false);
    return (
        <div style={{ borderBottom: '1px solid rgba(255,255,255,0.05)' }}>
            <div
                role="button"
                tabIndex={0}
                onClick={() => setOpen(o => !o)}
                onKeyDown={e => e.key === 'Enter' && setOpen(o => !o)}
                style={{
                    display: 'grid',
                    gridTemplateColumns: '64px 1fr auto auto',
                    alignItems: 'center',
                    gap: 8,
                    padding: '6px 10px',
                    cursor: 'pointer',
                    outline: 'none',
                }}
            >
                <StatusPill status={task.status} />
                <span style={{ fontSize: 12, color: '#e6edf3', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                    {task.prompt.split('\n')[0]}
                </span>
                {task.provider && (
                    <span style={{ fontSize: 10, fontFamily: 'monospace', color: PROVIDER_COLOR[task.provider] ?? '#8b949e' }}>
                        {task.provider}
                    </span>
                )}
                {onSkip && task.status === 'queued' && (
                    <span
                        onClick={e => { e.stopPropagation(); onSkip(task.id); }}
                        style={{ fontSize: 10, color: '#8b949e', cursor: 'pointer' }}
                        title="Remove from queue"
                    >✕</span>
                )}
                {!onSkip && <span style={{ fontSize: 10, color: '#8b949e', fontFamily: 'monospace' }}>{task.id}</span>}
            </div>
            {open && (
                <div style={{ padding: '0 10px 10px 74px', fontSize: 11, color: '#c9d1d9' }}>
                    <pre style={{
                        margin: '0 0 6px', padding: '8px', background: '#010409', borderRadius: 4,
                        fontFamily: '"Cascadia Mono",Consolas,monospace', fontSize: 11, whiteSpace: 'pre-wrap',
                        wordBreak: 'break-word', maxHeight: 160, overflowY: 'auto', color: '#c9d1d9',
                    }}>
                        {task.prompt}
                    </pre>
                    <div style={{ display: 'flex', gap: 10, fontSize: 10, fontFamily: 'monospace', color: '#8b949e', flexWrap: 'wrap' }}>
                        <span>attempts: {task.attempts}</span>
                        {task.branch && <span>branch: <span style={{ color: '#d2a8ff' }}>{task.branch}</span></span>}
                        {task.commit && <span>commit: <span style={{ color: '#3fb950' }}>{task.commit}</span></span>}
                    </div>
                    {task.last_error && (
                        <div style={{ marginTop: 6, padding: '6px 8px', background: 'rgba(248,81,73,0.1)', borderRadius: 4, fontSize: 10, fontFamily: 'monospace', color: '#f85149', whiteSpace: 'pre-wrap', maxHeight: 140, overflowY: 'auto' }}>
                            {task.last_error}
                        </div>
                    )}
                </div>
            )}
        </div>
    );
};

const BridgeStrip: React.FC<{ tabs: TabStatus[] }> = ({ tabs }) => (
    <div style={{ display: 'flex', alignItems: 'center', gap: 8, padding: '6px 12px', borderBottom: '1px solid rgba(255,255,255,0.05)', flexWrap: 'wrap' }}>
        <span style={{ fontSize: 10, fontFamily: 'monospace', color: '#8b949e', fontWeight: 700 }}>BRIDGE</span>
        {tabs.length === 0 && (
            <span style={{ fontSize: 10, color: '#8b949e' }}>
                no chat tabs connected — load the extension &amp; open a chat
            </span>
        )}
        {tabs.map(t => (
            <span key={t.tab_id} style={{
                display: 'inline-flex', alignItems: 'center', gap: 4,
                padding: '2px 8px', borderRadius: 10, fontSize: 10, fontFamily: 'monospace',
                background: 'rgba(255,255,255,0.05)',
                color: PROVIDER_COLOR[t.provider] ?? '#c9d1d9',
            }}>
                <span style={{
                    width: 7, height: 7, borderRadius: '50%',
                    background: t.cooling ? '#d29922' : '#3fb950',
                }} />
                {t.provider}
                {t.cooling && <span style={{ color: '#d29922' }}>cooling</span>}
            </span>
        ))}
    </div>
);

// ── Main ──────────────────────────────────────────────────────────────────────

const MissionControl: React.FC = () => {
    const [snap, setSnap] = useState<SupervisorSnapshot | null>(null);
    const [tabs, setTabs] = useState<TabStatus[]>([]);
    const [prompt, setPrompt] = useState('');
    const [busy, setBusy] = useState(false);

    const refresh = useCallback(() => {
        invoke<SupervisorSnapshot>('supervisor_status').then(setSnap).catch(() => {});
        invoke<TabStatus[]>('webui_bridge_status').then(setTabs).catch(() => {});
    }, []);

    useEffect(() => {
        refresh();
        const un = listen<SupervisorSnapshot>('supervisor:update', ev => setSnap(ev.payload));
        const poll = setInterval(() => {
            invoke<TabStatus[]>('webui_bridge_status').then(setTabs).catch(() => {});
        }, 4000);
        return () => { un.then(f => f()); clearInterval(poll); };
    }, [refresh]);

    const enqueue = async () => {
        if (!prompt.trim()) return;
        setBusy(true);
        try {
            await invoke('supervisor_enqueue', { prompt: prompt.trim() });
            setPrompt('');
            refresh();
        } finally {
            setBusy(false);
        }
    };

    const act = async (cmd: string, args?: Record<string, unknown>) => {
        await invoke(cmd, args).catch(() => {});
        refresh();
    };

    const paused = snap?.paused ?? false;

    return (
        <div style={{
            display: 'flex', flexDirection: 'column', height: '100%',
            background: '#0d1117', color: '#e6edf3',
            fontFamily: '-apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif', overflow: 'hidden',
        }}>
            {/* Toolbar */}
            <div style={{
                display: 'grid', gridTemplateColumns: '1fr auto auto auto',
                alignItems: 'center', gap: 8, padding: '8px 12px',
                background: '#161b22', borderBottom: '1px solid rgba(255,255,255,0.08)', flexShrink: 0,
            }}>
                <span style={{ fontSize: 11, fontWeight: 700, color: '#8b949e', letterSpacing: '0.04em' }}>
                    🛰 MISSION CONTROL
                    <span style={{ marginLeft: 8, fontWeight: 400, color: paused ? '#d29922' : '#3fb950' }}>
                        {paused ? 'PAUSED' : 'ACTIVE 24/7'}
                    </span>
                </span>
                <button onClick={() => act(paused ? 'supervisor_resume' : 'supervisor_pause')} style={btn(false)}>
                    {paused ? '▶ Resume' : '⏸ Pause'}
                </button>
                <button onClick={() => act('supervisor_clear')} style={{ ...btn(false), background: 'transparent', border: '1px solid rgba(255,255,255,0.12)', color: '#8b949e' }}>
                    Clear queue
                </button>
                <button onClick={refresh} style={{ ...btn(false), background: 'transparent', border: '1px solid rgba(255,255,255,0.12)', color: '#8b949e' }}>
                    ↻
                </button>
            </div>

            {/* Bridge strip */}
            <BridgeStrip tabs={tabs} />

            {/* Enqueue */}
            <div style={{ display: 'flex', gap: 6, padding: '8px 12px', borderBottom: '1px solid rgba(255,255,255,0.05)', flexShrink: 0 }}>
                <textarea
                    value={prompt}
                    onChange={e => setPrompt(e.target.value)}
                    onKeyDown={e => { if (e.key === 'Enter' && (e.metaKey || e.ctrlKey)) enqueue(); }}
                    placeholder="Describe a task… (⌘/Ctrl+Enter to queue). Runs unattended on an isolated branch, verified before commit."
                    style={{
                        flex: 1, background: '#0d1117', color: '#c9d1d9',
                        border: '1px solid rgba(255,255,255,0.1)', borderRadius: 4,
                        padding: '6px 8px', fontSize: 12, resize: 'vertical', minHeight: 40, outline: 'none',
                    }}
                />
                <button onClick={enqueue} disabled={busy || !prompt.trim()} style={btn(busy || !prompt.trim())}>
                    {busy ? '…' : '+ Queue'}
                </button>
            </div>

            {/* Body */}
            <div style={{ flex: 1, overflowY: 'auto' }}>
                {/* Running */}
                {snap?.running && (
                    <Section label="RUNNING">
                        <TaskRow task={snap.running} />
                    </Section>
                )}

                {/* Queue */}
                <Section label={`QUEUE (${snap?.queue.length ?? 0})`}>
                    {(snap?.queue ?? []).length === 0
                        ? <Empty text="Queue empty. Add a task above." />
                        : snap!.queue.map(t => <TaskRow key={t.id} task={t} onSkip={id => act('supervisor_skip', { taskId: id })} />)}
                </Section>

                {/* History */}
                <Section label={`HISTORY (${snap?.history.length ?? 0})`}>
                    {(snap?.history ?? []).length === 0
                        ? <Empty text="No completed tasks yet." />
                        : snap!.history.map(t => <TaskRow key={t.id} task={t} />)}
                </Section>
            </div>
        </div>
    );
};

const Section: React.FC<{ label: string; children: React.ReactNode }> = ({ label, children }) => (
    <div>
        <div style={{
            position: 'sticky', top: 0, zIndex: 1,
            padding: '4px 12px', background: '#161b22',
            fontSize: 10, fontFamily: 'monospace', fontWeight: 700, color: '#8b949e',
            letterSpacing: '0.08em', borderBottom: '1px solid rgba(255,255,255,0.06)',
        }}>
            {label}
        </div>
        {children}
    </div>
);

const Empty: React.FC<{ text: string }> = ({ text }) => (
    <div style={{ padding: '16px 12px', fontSize: 11, color: '#8b949e', textAlign: 'center' }}>{text}</div>
);

const btn = (disabled: boolean): React.CSSProperties => ({
    padding: '5px 12px',
    background: disabled ? 'rgba(255,255,255,0.04)' : '#238636',
    color: disabled ? '#8b949e' : '#fff',
    border: 'none', borderRadius: 4, fontSize: 11, fontWeight: 600,
    cursor: disabled ? 'not-allowed' : 'pointer', flexShrink: 0, whiteSpace: 'nowrap',
});

export default MissionControl;
