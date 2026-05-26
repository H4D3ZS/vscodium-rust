import React, { useState, useEffect, useRef } from 'react';
import { useStore } from '../store';
import { invoke } from '@tauri-apps/api/core';
import type * as MonacoType from 'monaco-editor';

type Tab = 'workflows' | 'rules' | 'tasks';

interface WorkflowInfo { name: string; description: string; path: string; content: string; }
interface RuleInfo { name: string; description: string; trigger: string | null; path: string; content: string; }
interface AgTask { task_id: string; description: string; file_ref: string | null; done: boolean; phase: string; tasks_path: string; spec_dir: string; }

const TRIGGER_OPTIONS = ['always', 'on_file_create', 'model_decision', 'glob', 'on_save', 'on_commit'];

const WorkflowPanel: React.FC = () => {
    const [tab, setTab] = useState<Tab>('tasks');
    const [workflows, setWorkflows] = useState<WorkflowInfo[]>([]);
    const [rules, setRules] = useState<RuleInfo[]>([]);
    const [tasks, setTasks] = useState<AgTask[]>([]);
    const [selected, setSelected] = useState<WorkflowInfo | RuleInfo | null>(null);
    const [editContent, setEditContent] = useState('');
    const [editTrigger, setEditTrigger] = useState('always');
    const [saving, setSaving] = useState(false);
    const [loading, setLoading] = useState(false);
    const editorRef = useRef<MonacoType.editor.IStandaloneCodeEditor | null>(null);
    const editorDivRef = useRef<HTMLDivElement>(null);
    const activeRoot = useStore(s => s.activeRoot);

    const load = async () => {
        if (!activeRoot) return;
        setLoading(true);
        try {
            const [wfs, rs, ts] = await Promise.allSettled([
                invoke<WorkflowInfo[]>('ag_get_workflows', { root: activeRoot }),
                invoke<RuleInfo[]>('ag_get_rules', { root: activeRoot }),
                invoke<AgTask[]>('ag_list_all_tasks', { root: activeRoot }),
            ]);
            if (wfs.status === 'fulfilled') setWorkflows(wfs.value || []);
            if (rs.status === 'fulfilled') setRules(rs.value || []);
            if (ts.status === 'fulfilled') setTasks(ts.value || []);
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => { load(); }, [activeRoot]);

    // Mount Monaco editor when an item is selected (keyed on path+tab so switching tabs rebuilds)
    useEffect(() => {
        if (!selected || !editorDivRef.current) return;
        const monaco = (window as any).monaco as typeof MonacoType | undefined;
        if (!monaco) return;

        // Dispose old editor before creating new one
        if (editorRef.current) {
            editorRef.current.dispose();
            editorRef.current = null;
        }

        const ed = monaco.editor.create(editorDivRef.current, {
            value: editContent,
            language: 'markdown',
            theme: 'vs-dark',
            fontSize: 12,
            lineNumbers: 'on',
            minimap: { enabled: false },
            wordWrap: 'on',
            scrollBeyondLastLine: false,
            automaticLayout: true,
        });
        ed.onDidChangeModelContent(() => setEditContent(ed.getValue()));
        editorRef.current = ed;

        return () => {
            ed.dispose();
            editorRef.current = null;
        };
    // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [selected?.path, tab]);

    const selectItem = (item: WorkflowInfo | RuleInfo) => {
        setSelected(item);
        setEditContent(item.content);
        if ('trigger' in item) setEditTrigger(item.trigger || 'always');
        setTimeout(() => {
            if (editorRef.current) editorRef.current.setValue(item.content);
        }, 50);
    };

    const saveSelected = async () => {
        if (!selected) return;
        setSaving(true);
        try {
            let finalContent = editContent;
            // For rules, update/inject trigger in frontmatter
            if (tab === 'rules' && editTrigger !== 'always') {
                if (finalContent.startsWith('---')) {
                    const end = finalContent.indexOf('\n---', 3);
                    if (end > 0) {
                        const fm = finalContent.slice(3, end);
                        const hasTrigger = fm.includes('trigger:');
                        const newFm = hasTrigger
                            ? fm.replace(/trigger:.*/, `trigger: ${editTrigger}`)
                            : fm + `\ntrigger: ${editTrigger}`;
                        finalContent = `---${newFm}\n---${finalContent.slice(end + 4)}`;
                    }
                } else {
                    finalContent = `---\ntrigger: ${editTrigger}\n---\n\n${finalContent}`;
                }
            }
            await invoke('write_file_content', { path: selected.path, content: finalContent });
            await load();
        } finally {
            setSaving(false);
        }
    };

    const createNew = async (type: 'workflow' | 'rule') => {
        if (!activeRoot) return;
        const name = prompt(`Name for new ${type} (no spaces, no .md):`);
        if (!name) return;
        const safeName = name.replace(/[^a-z0-9_-]/gi, '-').toLowerCase();
        const dir = type === 'workflow'
            ? `${activeRoot}/.agent/workflows`
            : `${activeRoot}/.agent/rules`;
        try {
            await invoke('create_directory', { path: dir }).catch(() => null);
            const stub = type === 'workflow'
                ? `---\ndescription: ${safeName}\n---\n\n# ${safeName}\n\n## Steps\n\n1. TODO\n`
                : `---\nname: ${safeName}\ndescription: TODO\ntrigger: always\n---\n\n# Rule: ${safeName}\n\nTODO\n`;
            await invoke('write_file_content', { path: `${dir}/${safeName}.md`, content: stub });
            await load();
        } catch (e) {
            alert(`Failed to create ${type}: ${e}`);
        }
    };

    const runWorkflow = (wf: WorkflowInfo) => {
        const input = document.querySelector('.agent-input-section textarea') as HTMLTextAreaElement;
        if (input) {
            input.value = `Execute the workflow at: ${wf.path}\n\n${wf.content.slice(0, 500)}`;
            input.dispatchEvent(new Event('input', { bubbles: true }));
        }
    };

    const markDone = async (task: AgTask) => {
        try {
            await invoke('ag_mark_task_done', { tasksPath: task.tasks_path, taskId: task.task_id });
            await load();
        } catch (e) { alert(`Failed: ${e}`); }
    };

    const pendingTasks = tasks.filter(t => !t.done);
    const doneTasks = tasks.filter(t => t.done);

    const tabBtn = (t: Tab, label: string, count?: number) => (
        <button
            onClick={() => { setTab(t); setSelected(null); }}
            style={{
                flex: 1,
                padding: '6px 4px',
                background: tab === t ? 'rgba(99, 102, 241, 0.2)' : 'transparent',
                border: 'none',
                borderBottom: tab === t ? '2px solid #6366f1' : '2px solid transparent',
                color: tab === t ? '#a5b4fc' : 'var(--vscode-foreground)',
                cursor: 'pointer',
                fontSize: '11px',
                fontWeight: tab === t ? 600 : 400,
            }}
        >
            {label}{count !== undefined ? ` (${count})` : ''}
        </button>
    );

    const listItemStyle: React.CSSProperties = {
        padding: '8px 10px',
        background: 'rgba(255,255,255,0.02)',
        border: '1px solid var(--vscode-panel-border)',
        borderRadius: '4px',
        cursor: 'pointer',
        marginBottom: '4px',
        transition: 'background 0.15s',
    };

    return (
        <div style={{ display: 'flex', flexDirection: 'column', height: '100%', fontFamily: 'var(--vscode-font-family)' }}>
            {/* Tab Bar */}
            <div style={{ display: 'flex', borderBottom: '1px solid var(--vscode-panel-border)', flexShrink: 0 }}>
                {tabBtn('tasks', 'Tasks', pendingTasks.length || undefined)}
                {tabBtn('workflows', 'Workflows', workflows.length || undefined)}
                {tabBtn('rules', 'Rules', rules.length || undefined)}
                <button
                    onClick={load}
                    title="Refresh"
                    style={{ background: 'transparent', border: 'none', color: 'var(--vscode-foreground)', opacity: 0.5, cursor: 'pointer', padding: '0 8px' }}
                >
                    <i className={`codicon codicon-${loading ? 'loading~spin' : 'refresh'}`} />
                </button>
            </div>

            <div style={{ flex: 1, overflow: 'hidden', display: 'flex', flexDirection: 'column' }}>
                {/* TASKS TAB */}
                {tab === 'tasks' && (
                    <div style={{ flex: 1, overflowY: 'auto', padding: '8px' }}>
                        <div style={{ display: 'flex', gap: '6px', marginBottom: '8px' }}>
                            <button
                                onClick={() => {
                                    const input = document.querySelector('.agent-input-section textarea') as HTMLTextAreaElement;
                                    if (input) { input.value = '/next'; input.dispatchEvent(new Event('input', { bubbles: true })); }
                                }}
                                style={{ flex: 1, padding: '5px', background: 'rgba(99,102,241,0.15)', border: '1px solid rgba(99,102,241,0.3)', borderRadius: '4px', color: '#a5b4fc', cursor: 'pointer', fontSize: '11px' }}
                            >
                                <i className="codicon codicon-play" /> Execute Next Task
                            </button>
                            <button
                                onClick={async () => {
                                    if (!activeRoot) return;
                                    const name = prompt('Spec name (e.g. "User Auth"):');
                                    if (!name) return;
                                    const desc = prompt('Short description:') || name;
                                    try {
                                        await invoke('ag_create_spec', { root: activeRoot, slug: name, description: desc });
                                        await load();
                                    } catch (e) { alert(`Failed: ${e}`); }
                                }}
                                style={{ padding: '5px 8px', background: 'rgba(16,185,129,0.1)', border: '1px solid rgba(16,185,129,0.2)', borderRadius: '4px', color: '#34d399', cursor: 'pointer', fontSize: '11px' }}
                                title="New Spec"
                            >
                                <i className="codicon codicon-add" />
                            </button>
                        </div>

                        {pendingTasks.length === 0 && doneTasks.length === 0 && (
                            <div style={{ padding: '20px', textAlign: 'center', opacity: 0.5, fontSize: '12px' }}>
                                No tasks found in <code>specs/*/tasks.md</code>.<br />
                                Use <code>/spec</code> to create one.
                            </div>
                        )}

                        {pendingTasks.length > 0 && (
                            <>
                                <div style={{ fontSize: '10px', opacity: 0.5, textTransform: 'uppercase', marginBottom: '4px', padding: '0 2px' }}>
                                    Pending ({pendingTasks.length})
                                </div>
                                {pendingTasks.map((t, i) => (
                                    <div key={i} style={{ ...listItemStyle, borderLeft: '3px solid #6366f1' }}>
                                        <div style={{ display: 'flex', alignItems: 'flex-start', gap: '6px' }}>
                                            <button
                                                onClick={() => markDone(t)}
                                                title="Mark complete"
                                                style={{ background: 'none', border: '1px solid rgba(255,255,255,0.2)', borderRadius: '2px', width: '14px', height: '14px', cursor: 'pointer', marginTop: '2px', flexShrink: 0, display: 'flex', alignItems: 'center', justifyContent: 'center' }}
                                            />
                                            <div style={{ flex: 1, minWidth: 0 }}>
                                                <div style={{ fontSize: '11px', fontWeight: 600, color: '#a5b4fc' }}>{t.task_id}</div>
                                                <div style={{ fontSize: '12px', lineHeight: 1.4 }}>{t.description}</div>
                                                {t.file_ref && <div style={{ fontSize: '10px', opacity: 0.5, marginTop: '2px' }}>{t.file_ref}</div>}
                                                <div style={{ fontSize: '10px', opacity: 0.4, marginTop: '2px' }}>{t.phase}</div>
                                            </div>
                                        </div>
                                    </div>
                                ))}
                            </>
                        )}

                        {doneTasks.length > 0 && (
                            <>
                                <div style={{ fontSize: '10px', opacity: 0.4, textTransform: 'uppercase', margin: '8px 0 4px', padding: '0 2px' }}>
                                    Completed ({doneTasks.length})
                                </div>
                                {doneTasks.slice(0, 10).map((t, i) => (
                                    <div key={i} style={{ ...listItemStyle, opacity: 0.5, borderLeft: '3px solid #10b981' }}>
                                        <div style={{ display: 'flex', alignItems: 'flex-start', gap: '6px' }}>
                                            <i className="codicon codicon-check" style={{ color: '#10b981', marginTop: '2px', fontSize: '12px' }} />
                                            <div>
                                                <span style={{ fontSize: '11px', fontWeight: 600 }}>{t.task_id}</span>
                                                <span style={{ fontSize: '11px', marginLeft: '6px' }}>{t.description}</span>
                                            </div>
                                        </div>
                                    </div>
                                ))}
                            </>
                        )}
                    </div>
                )}

                {/* WORKFLOWS TAB */}
                {tab === 'workflows' && (
                    <div style={{ flex: 1, display: 'flex', overflow: 'hidden' }}>
                        <div style={{ width: '160px', flexShrink: 0, borderRight: '1px solid var(--vscode-panel-border)', overflowY: 'auto', padding: '8px' }}>
                            <button
                                onClick={() => createNew('workflow')}
                                style={{ width: '100%', marginBottom: '8px', padding: '4px', background: 'rgba(99,102,241,0.1)', border: '1px dashed rgba(99,102,241,0.4)', borderRadius: '4px', color: '#a5b4fc', cursor: 'pointer', fontSize: '11px' }}
                            >
                                + New Workflow
                            </button>
                            {workflows.length === 0 && (
                                <div style={{ fontSize: '11px', opacity: 0.4, textAlign: 'center', padding: '8px 0' }}>
                                    No workflows in<br /><code>.agent/workflows</code>
                                </div>
                            )}
                            {workflows.map((wf, i) => (
                                <div
                                    key={i}
                                    onClick={() => selectItem(wf)}
                                    style={{
                                        ...listItemStyle,
                                        background: selected?.path === wf.path ? 'rgba(99,102,241,0.15)' : 'rgba(255,255,255,0.02)',
                                        borderColor: selected?.path === wf.path ? 'rgba(99,102,241,0.5)' : 'var(--vscode-panel-border)',
                                    }}
                                >
                                    <div style={{ fontSize: '11px', fontWeight: 600, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{wf.name}</div>
                                    <div style={{ fontSize: '10px', opacity: 0.5, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap', marginTop: '2px' }}>{wf.description}</div>
                                </div>
                            ))}
                        </div>

                        <div style={{ flex: 1, display: 'flex', flexDirection: 'column', overflow: 'hidden' }}>
                            {selected && tab === 'workflows' ? (
                                <>
                                    <div style={{ padding: '6px 8px', borderBottom: '1px solid var(--vscode-panel-border)', display: 'flex', alignItems: 'center', gap: '8px', flexShrink: 0 }}>
                                        <span style={{ fontSize: '12px', fontWeight: 600, flex: 1 }}>{selected.name}.md</span>
                                        <button
                                            onClick={() => runWorkflow(selected as WorkflowInfo)}
                                            style={{ padding: '3px 8px', background: 'rgba(74,222,128,0.1)', border: '1px solid rgba(74,222,128,0.3)', borderRadius: '4px', color: '#4ade80', cursor: 'pointer', fontSize: '11px' }}
                                        >
                                            <i className="codicon codicon-play" /> Run
                                        </button>
                                        <button
                                            onClick={saveSelected}
                                            disabled={saving}
                                            style={{ padding: '3px 8px', background: 'rgba(99,102,241,0.1)', border: '1px solid rgba(99,102,241,0.3)', borderRadius: '4px', color: '#a5b4fc', cursor: 'pointer', fontSize: '11px' }}
                                        >
                                            {saving ? '...' : 'Save'}
                                        </button>
                                    </div>
                                    <div ref={editorDivRef} style={{ flex: 1 }} />
                                </>
                            ) : (
                                <div style={{ flex: 1, display: 'flex', alignItems: 'center', justifyContent: 'center', opacity: 0.4, fontSize: '12px' }}>
                                    Select a workflow to edit
                                </div>
                            )}
                        </div>
                    </div>
                )}

                {/* RULES TAB */}
                {tab === 'rules' && (
                    <div style={{ flex: 1, display: 'flex', overflow: 'hidden' }}>
                        <div style={{ width: '160px', flexShrink: 0, borderRight: '1px solid var(--vscode-panel-border)', overflowY: 'auto', padding: '8px' }}>
                            <button
                                onClick={() => createNew('rule')}
                                style={{ width: '100%', marginBottom: '8px', padding: '4px', background: 'rgba(245,158,11,0.1)', border: '1px dashed rgba(245,158,11,0.4)', borderRadius: '4px', color: '#fbbf24', cursor: 'pointer', fontSize: '11px' }}
                            >
                                + New Rule
                            </button>
                            {rules.length === 0 && (
                                <div style={{ fontSize: '11px', opacity: 0.4, textAlign: 'center', padding: '8px 0' }}>
                                    No rules in<br /><code>.agent/rules</code>
                                </div>
                            )}
                            {rules.map((rule, i) => (
                                <div
                                    key={i}
                                    onClick={() => selectItem(rule)}
                                    style={{
                                        ...listItemStyle,
                                        background: selected?.path === rule.path ? 'rgba(245,158,11,0.1)' : 'rgba(255,255,255,0.02)',
                                        borderColor: selected?.path === rule.path ? 'rgba(245,158,11,0.4)' : 'var(--vscode-panel-border)',
                                    }}
                                >
                                    <div style={{ fontSize: '11px', fontWeight: 600, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{rule.name}</div>
                                    {rule.trigger && (
                                        <div style={{ fontSize: '9px', background: 'rgba(245,158,11,0.15)', borderRadius: '3px', padding: '1px 4px', display: 'inline-block', marginTop: '2px', color: '#fbbf24' }}>
                                            {rule.trigger}
                                        </div>
                                    )}
                                </div>
                            ))}
                        </div>

                        <div style={{ flex: 1, display: 'flex', flexDirection: 'column', overflow: 'hidden' }}>
                            {selected && tab === 'rules' ? (
                                <>
                                    <div style={{ padding: '6px 8px', borderBottom: '1px solid var(--vscode-panel-border)', display: 'flex', alignItems: 'center', gap: '6px', flexShrink: 0, flexWrap: 'wrap' }}>
                                        <span style={{ fontSize: '12px', fontWeight: 600, flex: 1 }}>{selected.name}.md</span>
                                        <select
                                            value={editTrigger}
                                            onChange={e => setEditTrigger(e.target.value)}
                                            style={{ fontSize: '11px', background: 'var(--vscode-input-background)', color: 'var(--vscode-input-foreground)', border: '1px solid var(--vscode-input-border)', borderRadius: '4px', padding: '2px 4px' }}
                                        >
                                            {TRIGGER_OPTIONS.map(t => <option key={t} value={t}>{t}</option>)}
                                        </select>
                                        <button
                                            onClick={saveSelected}
                                            disabled={saving}
                                            style={{ padding: '3px 8px', background: 'rgba(245,158,11,0.1)', border: '1px solid rgba(245,158,11,0.3)', borderRadius: '4px', color: '#fbbf24', cursor: 'pointer', fontSize: '11px' }}
                                        >
                                            {saving ? '...' : 'Save'}
                                        </button>
                                    </div>
                                    <div ref={editorDivRef} style={{ flex: 1 }} />
                                </>
                            ) : (
                                <div style={{ flex: 1, display: 'flex', alignItems: 'center', justifyContent: 'center', opacity: 0.4, fontSize: '12px' }}>
                                    Select a rule to edit
                                </div>
                            )}
                        </div>
                    </div>
                )}
            </div>
        </div>
    );
};

export default WorkflowPanel;
