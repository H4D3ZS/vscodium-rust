import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useStore } from '../store';

interface HookDef {
    id: string;
    name: string;
    trigger: string;
    glob?: string;
    prompt: string;
    enabled: boolean;
}

const TRIGGERS = [
    { value: 'on_save', label: 'On File Save', icon: 'save', desc: 'Runs when a file is saved in the editor' },
    { value: 'on_commit', label: 'On Git Commit', icon: 'git-commit', desc: 'Runs before a git commit is finalized' },
    { value: 'on_file_create', label: 'On File Create', icon: 'new-file', desc: 'Runs when a new file is created' },
    { value: 'on_test_fail', label: 'On Test Fail', icon: 'testing-failed-icon', desc: 'Runs when tests fail' },
    { value: 'manual', label: 'Manual Only', icon: 'play', desc: 'Only runs when explicitly triggered' },
];

const BUILTIN_HOOKS: HookDef[] = [
    {
        id: 'doc-on-save',
        name: 'Auto-document on save',
        trigger: 'on_save',
        glob: '*.{ts,tsx,rs}',
        prompt: 'Review the file I just saved. If any exported functions/types lack JSDoc/doc comments, add minimal documentation (one-line summaries only). Do not modify logic.',
        enabled: false,
    },
    {
        id: 'commit-msg',
        name: 'Generate commit message',
        trigger: 'on_commit',
        glob: undefined,
        prompt: 'Look at the staged git diff and generate a concise, conventional commit message. Format: `type(scope): description`. Output only the commit message, nothing else.',
        enabled: false,
    },
    {
        id: 'test-fix',
        name: 'Auto-fix failing tests',
        trigger: 'on_test_fail',
        glob: undefined,
        prompt: 'Tests are failing. Read the test output, identify the root cause, and fix the minimal amount of code to make them pass. Do not change test logic.',
        enabled: false,
    },
];

const HOOKS_STORAGE_KEY = 'hades.hooks';

function loadHooks(): HookDef[] {
    try {
        const stored = localStorage.getItem(HOOKS_STORAGE_KEY);
        if (stored) return JSON.parse(stored);
    } catch { }
    return BUILTIN_HOOKS.map(h => ({ ...h }));
}

function saveHooks(hooks: HookDef[]) {
    try {
        localStorage.setItem(HOOKS_STORAGE_KEY, JSON.stringify(hooks));
    } catch { }
}

const HooksPanel: React.FC = () => {
    const [hooks, setHooks] = useState<HookDef[]>(loadHooks);
    const [editing, setEditing] = useState<HookDef | null>(null);
    const [editDraft, setEditDraft] = useState<HookDef | null>(null);
    const activeRoot = useStore(s => s.activeRoot);

    const updateHooks = (next: HookDef[]) => {
        setHooks(next);
        saveHooks(next);
    };

    const toggleEnabled = (id: string) => {
        updateHooks(hooks.map(h => h.id === id ? { ...h, enabled: !h.enabled } : h));
    };

    const deleteHook = (id: string) => {
        if (!confirm('Delete this hook?')) return;
        updateHooks(hooks.filter(h => h.id !== id));
        if (editing?.id === id) setEditing(null);
    };

    const startEdit = (h: HookDef) => {
        setEditing(h);
        setEditDraft({ ...h });
    };

    const saveEdit = () => {
        if (!editDraft) return;
        updateHooks(hooks.map(h => h.id === editDraft.id ? editDraft : h));
        setEditing(null);
    };

    const addNew = () => {
        const newHook: HookDef = {
            id: `hook-${Date.now()}`,
            name: 'New Hook',
            trigger: 'manual',
            prompt: '',
            enabled: true,
        };
        updateHooks([...hooks, newHook]);
        startEdit(newHook);
    };

    const runManual = async (h: HookDef) => {
        const store = (window as any).useStore?.getState();
        if (!store) return;
        store.addAgentMessage?.('user', `[HOOK: ${h.name}]\n\n${h.prompt}`);
        store.addAgentMessage?.('assistant', '');
        store.setIsAgentThinking?.(true);
        try {
            const { sendAgentMessage } = await import('../agent');
            await sendAgentMessage(h.prompt, () => {});
        } catch (e) {
            console.error('Hook run failed:', e);
        }
    };

    const getTriggerInfo = (trigger: string) => TRIGGERS.find(t => t.value === trigger) || TRIGGERS[0];

    return (
        <div style={{ maxWidth: 780 }}>
            <div className="settings-section-title" style={{ display: 'flex', alignItems: 'center', gap: 10 }}>
                Hooks
                <span className="settings-badge new">Agent</span>
            </div>
            <p className="settings-section-subtitle">
                Hooks trigger AI actions automatically on IDE events. Enable hooks to automate repetitive tasks like documentation, commit messages, and test fixes.
            </p>

            <div style={{ display: 'flex', justifyContent: 'flex-end', marginBottom: 12 }}>
                <button className="settings-button" onClick={addNew} style={{ fontSize: 12 }}>
                    + New Hook
                </button>
            </div>

            {/* Hook list */}
            {hooks.length === 0 && (
                <div style={{ textAlign: 'center', opacity: 0.4, padding: '32px 0', fontSize: 12 }}>
                    No hooks configured. Click "New Hook" to create one.
                </div>
            )}

            {hooks.map(h => {
                const trigInfo = getTriggerInfo(h.trigger);
                return (
                    <div key={h.id} className="hook-item" style={{ flexDirection: 'column', alignItems: 'stretch', gap: 0 }}>
                        <div style={{ display: 'flex', alignItems: 'center', gap: 10 }}>
                            {/* Enable toggle */}
                            <label className="settings-toggle" title={h.enabled ? 'Enabled' : 'Disabled'}>
                                <input
                                    type="checkbox"
                                    checked={h.enabled}
                                    onChange={() => toggleEnabled(h.id)}
                                />
                                <span className="settings-toggle-slider" />
                            </label>

                            {/* Trigger icon + name */}
                            <div style={{ flex: 1, minWidth: 0 }}>
                                <div style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
                                    <span style={{ fontSize: 13, fontWeight: 600, color: h.enabled ? 'var(--vscode-foreground)' : 'rgba(255,255,255,0.4)' }}>
                                        {h.name}
                                    </span>
                                    <span style={{
                                        fontSize: 10,
                                        padding: '1px 6px',
                                        borderRadius: 4,
                                        background: 'rgba(99,102,241,0.15)',
                                        color: '#818cf8',
                                        border: '1px solid rgba(99,102,241,0.3)',
                                    }}>
                                        <i className={`codicon codicon-${trigInfo.icon}`} style={{ marginRight: 3, fontSize: 10 }} />
                                        {trigInfo.label}
                                    </span>
                                    {h.glob && (
                                        <span style={{ fontSize: 10, opacity: 0.4, fontFamily: 'monospace' }}>{h.glob}</span>
                                    )}
                                </div>
                                {h.prompt && (
                                    <div style={{ fontSize: 11, opacity: 0.45, marginTop: 2, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap', maxWidth: 500 }}>
                                        {h.prompt.slice(0, 100)}{h.prompt.length > 100 ? '…' : ''}
                                    </div>
                                )}
                            </div>

                            {/* Actions */}
                            <div style={{ display: 'flex', gap: 4 }}>
                                {h.trigger === 'manual' && (
                                    <button
                                        className="settings-button success"
                                        style={{ fontSize: 11, padding: '3px 8px' }}
                                        onClick={() => runManual(h)}
                                        title="Run now"
                                    >
                                        <i className="codicon codicon-play" />
                                    </button>
                                )}
                                <button
                                    className="settings-button"
                                    style={{ fontSize: 11, padding: '3px 8px', opacity: 0.7 }}
                                    onClick={() => startEdit(h)}
                                    title="Edit"
                                >
                                    <i className="codicon codicon-edit" />
                                </button>
                                <button
                                    className="settings-button danger"
                                    style={{ fontSize: 11, padding: '3px 8px' }}
                                    onClick={() => deleteHook(h.id)}
                                    title="Delete"
                                >
                                    <i className="codicon codicon-trash" />
                                </button>
                            </div>
                        </div>
                    </div>
                );
            })}

            {/* Edit modal */}
            {editing && editDraft && (
                <div style={{
                    position: 'fixed', inset: 0, zIndex: 9999,
                    background: 'rgba(0,0,0,0.6)', backdropFilter: 'blur(4px)',
                    display: 'flex', alignItems: 'center', justifyContent: 'center',
                }}>
                    <div style={{
                        background: 'var(--vscode-editor-background, #0d1117)',
                        border: '1px solid rgba(255,255,255,0.12)',
                        borderRadius: 12,
                        padding: 24,
                        width: 560,
                        maxWidth: '95vw',
                    }}>
                        <div style={{ fontSize: 15, fontWeight: 600, marginBottom: 16 }}>
                            {editing.id === editDraft.id && hooks.find(h => h.id === editing.id) ? 'Edit Hook' : 'New Hook'}
                        </div>

                        <div style={{ display: 'flex', flexDirection: 'column', gap: 12 }}>
                            <div>
                                <div style={{ fontSize: 11, opacity: 0.5, marginBottom: 4 }}>Name</div>
                                <input
                                    className="settings-input"
                                    style={{ width: '100%', boxSizing: 'border-box' }}
                                    value={editDraft.name}
                                    onChange={e => setEditDraft({ ...editDraft, name: e.target.value })}
                                />
                            </div>

                            <div>
                                <div style={{ fontSize: 11, opacity: 0.5, marginBottom: 4 }}>Trigger</div>
                                <select
                                    className="settings-select"
                                    style={{ width: '100%' }}
                                    value={editDraft.trigger}
                                    onChange={e => setEditDraft({ ...editDraft, trigger: e.target.value })}
                                >
                                    {TRIGGERS.map(t => (
                                        <option key={t.value} value={t.value}>{t.label} — {t.desc}</option>
                                    ))}
                                </select>
                            </div>

                            {(editDraft.trigger === 'on_save' || editDraft.trigger === 'on_file_create') && (
                                <div>
                                    <div style={{ fontSize: 11, opacity: 0.5, marginBottom: 4 }}>File glob (optional, e.g. <code>*.ts</code>)</div>
                                    <input
                                        className="settings-input"
                                        style={{ width: '100%', boxSizing: 'border-box', fontFamily: 'monospace' }}
                                        placeholder="*.{ts,tsx,rs}"
                                        value={editDraft.glob || ''}
                                        onChange={e => setEditDraft({ ...editDraft, glob: e.target.value || undefined })}
                                    />
                                </div>
                            )}

                            <div>
                                <div style={{ fontSize: 11, opacity: 0.5, marginBottom: 4 }}>Prompt (what to ask the AI)</div>
                                <textarea
                                    className="settings-input"
                                    style={{ width: '100%', boxSizing: 'border-box', height: 120, resize: 'vertical', fontSize: 12 }}
                                    placeholder="Describe what the AI should do when this hook fires..."
                                    value={editDraft.prompt}
                                    onChange={e => setEditDraft({ ...editDraft, prompt: e.target.value })}
                                />
                            </div>
                        </div>

                        <div style={{ display: 'flex', justifyContent: 'flex-end', gap: 8, marginTop: 20 }}>
                            <button className="settings-button" style={{ opacity: 0.6 }} onClick={() => setEditing(null)}>Cancel</button>
                            <button className="settings-button" onClick={saveEdit}>Save Hook</button>
                        </div>
                    </div>
                </div>
            )}
        </div>
    );
};

export default HooksPanel;
