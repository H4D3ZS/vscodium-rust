import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useStore } from '../store';

interface AgTask {
    task_id: string;
    description: string;
    file_ref: string | null;
    done: boolean;
    phase: string;
    spec_dir: string;
}

const AgTasksView: React.FC = () => {
    const [tasks, setTasks] = useState<AgTask[]>([]);
    const [loading, setLoading] = useState(false);
    const [filter, setFilter] = useState<'all' | 'pending' | 'done'>('pending');
    const activeRoot = useStore(s => s.activeRoot);

    const load = async () => {
        if (!activeRoot) return;
        setLoading(true);
        try {
            const all = await invoke<AgTask[]>('ag_list_all_tasks', { root: activeRoot });
            setTasks(all || []);
        } catch (_) {
            setTasks([]);
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => { load(); }, [activeRoot]);

    const markDone = async (task: AgTask) => {
        if (!activeRoot) return;
        const tasksPath = `${task.spec_dir}/tasks.md`;
        try {
            await invoke('ag_mark_task_done', { tasksPath, taskId: task.task_id });
            await load();
        } catch { /* non-fatal */ }
    };

    const executeTask = (task: AgTask) => {
        const input = document.querySelector<HTMLTextAreaElement>('#agent-input');
        if (input) {
            const nativeInputValueSetter = Object.getOwnPropertyDescriptor(window.HTMLTextAreaElement.prototype, 'value')?.set;
            nativeInputValueSetter?.call(input, `/next`);
            input.dispatchEvent(new Event('input', { bubbles: true }));
            input.dispatchEvent(new KeyboardEvent('keydown', { key: 'Enter', bubbles: true }));
        }
    };

    const filtered = tasks.filter(t =>
        filter === 'all' ? true : filter === 'pending' ? !t.done : t.done
    );

    const pending = tasks.filter(t => !t.done).length;
    const done = tasks.filter(t => t.done).length;

    if (!activeRoot) {
        return (
            <div style={{ padding: '20px', textAlign: 'center', opacity: 0.5, fontSize: '12px' }}>
                Open a folder to view tasks
            </div>
        );
    }

    return (
        <div style={{ display: 'flex', flexDirection: 'column', height: '100%', overflow: 'hidden' }}>
            {/* Stats bar */}
            <div style={{ padding: '8px 12px', borderBottom: '1px solid rgba(255,255,255,0.06)', display: 'flex', gap: 8, alignItems: 'center' }}>
                <span
                    style={{ fontSize: 10, padding: '2px 7px', borderRadius: 10, cursor: 'pointer',
                        background: filter === 'pending' ? 'rgba(79,142,247,0.2)' : 'rgba(255,255,255,0.05)',
                        color: filter === 'pending' ? '#4f8ef7' : 'rgba(255,255,255,0.5)',
                        border: `1px solid ${filter === 'pending' ? 'rgba(79,142,247,0.4)' : 'rgba(255,255,255,0.1)'}`,
                    }}
                    onClick={() => setFilter('pending')}
                >
                    {pending} pending
                </span>
                <span
                    style={{ fontSize: 10, padding: '2px 7px', borderRadius: 10, cursor: 'pointer',
                        background: filter === 'done' ? 'rgba(63,185,80,0.15)' : 'rgba(255,255,255,0.05)',
                        color: filter === 'done' ? '#3fb950' : 'rgba(255,255,255,0.5)',
                        border: `1px solid ${filter === 'done' ? 'rgba(63,185,80,0.3)' : 'rgba(255,255,255,0.1)'}`,
                    }}
                    onClick={() => setFilter('done')}
                >
                    {done} done
                </span>
                <span
                    style={{ fontSize: 10, padding: '2px 7px', borderRadius: 10, cursor: 'pointer', marginLeft: 'auto',
                        background: filter === 'all' ? 'rgba(255,255,255,0.1)' : 'rgba(255,255,255,0.03)',
                        color: 'rgba(255,255,255,0.5)',
                        border: '1px solid rgba(255,255,255,0.1)',
                    }}
                    onClick={() => setFilter('all')}
                >
                    all
                </span>
                <button
                    onClick={load}
                    style={{ background: 'none', border: 'none', color: 'rgba(255,255,255,0.4)', cursor: 'pointer', padding: 2, fontSize: 12 }}
                    title="Refresh"
                >
                    <i className={`codicon codicon-${loading ? 'loading~spin' : 'refresh'}`} />
                </button>
            </div>

            {/* Task list */}
            <div style={{ flex: 1, overflowY: 'auto', padding: '6px 0' }}>
                {filtered.length === 0 ? (
                    <div style={{ padding: '20px', textAlign: 'center', opacity: 0.4, fontSize: '11px' }}>
                        {loading ? 'Loading...' : tasks.length === 0 ? 'No tasks found.\nCreate specs in .kiro/specs/' : 'No tasks in this filter.'}
                    </div>
                ) : (
                    filtered.map(task => (
                        <div
                            key={task.task_id}
                            style={{
                                display: 'flex', alignItems: 'flex-start', gap: 8, padding: '6px 12px',
                                borderBottom: '1px solid rgba(255,255,255,0.04)',
                                opacity: task.done ? 0.45 : 1,
                            }}
                        >
                            <input
                                type="checkbox"
                                checked={task.done}
                                onChange={() => !task.done && markDone(task)}
                                style={{ marginTop: 2, cursor: task.done ? 'default' : 'pointer', accentColor: '#4f8ef7' }}
                            />
                            <div style={{ flex: 1, minWidth: 0 }}>
                                <div style={{ fontSize: 11, fontWeight: 600, color: task.done ? 'rgba(255,255,255,0.4)' : 'var(--vscode-foreground)', textDecoration: task.done ? 'line-through' : 'none', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                                    {task.task_id}
                                </div>
                                <div style={{ fontSize: 11, opacity: 0.65, lineHeight: 1.4, marginTop: 1 }}>
                                    {task.description}
                                </div>
                                {task.file_ref && (
                                    <div style={{ fontSize: 10, opacity: 0.4, fontFamily: 'monospace', marginTop: 2, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                                        → {task.file_ref}
                                    </div>
                                )}
                                <div style={{ fontSize: 10, opacity: 0.35, marginTop: 2 }}>
                                    {task.spec_dir.split(/[\/\\]/).slice(-2).join('/')} · {task.phase}
                                </div>
                            </div>
                            {!task.done && (
                                <button
                                    onClick={() => executeTask(task)}
                                    title="Execute with AI"
                                    style={{ background: 'rgba(79,142,247,0.1)', border: '1px solid rgba(79,142,247,0.25)', borderRadius: 4, color: '#4f8ef7', cursor: 'pointer', padding: '2px 5px', fontSize: 11, flexShrink: 0 }}
                                >
                                    <i className="codicon codicon-play" />
                                </button>
                            )}
                        </div>
                    ))
                )}
            </div>

            {/* Footer: new spec button */}
            <div style={{ padding: '8px 12px', borderTop: '1px solid rgba(255,255,255,0.06)' }}>
                <button
                    style={{ width: '100%', fontSize: 11, padding: '5px', background: 'rgba(255,255,255,0.04)', border: '1px dashed rgba(255,255,255,0.12)', borderRadius: 5, color: 'rgba(255,255,255,0.5)', cursor: 'pointer' }}
                    onClick={() => {
                        const input = document.querySelector<HTMLTextAreaElement>('#agent-input');
                        if (input) {
                            const nativeInputValueSetter = Object.getOwnPropertyDescriptor(window.HTMLTextAreaElement.prototype, 'value')?.set;
                            nativeInputValueSetter?.call(input, '/spec ');
                            input.dispatchEvent(new Event('input', { bubbles: true }));
                            input.focus();
                        }
                    }}
                >
                    + New Spec
                </button>
            </div>
        </div>
    );
};

export default AgTasksView;
