import React, { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useStore } from '../store';
import type * as MonacoType from 'monaco-editor';

interface SteeringFile {
    name: string;
    path: string;
    content: string;
    description: string;
    alwaysOn: boolean;
}

const BUILTIN_TEMPLATES = [
    {
        name: 'coding-style',
        description: 'Code style & conventions',
        template: `# Coding Style\n\nAlways follow these conventions:\n\n- Use early returns\n- Prefer named exports\n- No trailing comments that narrate code\n- Errors should be explicit, not swallowed\n`,
    },
    {
        name: 'architecture',
        description: 'Architecture constraints',
        template: `# Architecture Rules\n\n- All state goes through the Zustand store\n- Tauri commands are the only backend boundary\n- No direct DOM manipulation outside utility hooks\n`,
    },
    {
        name: 'security',
        description: 'Security guidelines',
        template: `# Security\n\n- Validate all user inputs at system boundaries\n- Never log sensitive data\n- Use parameterized queries\n- Escape all HTML output\n`,
    },
];

const SteeringPanel: React.FC = () => {
    const [files, setFiles] = useState<SteeringFile[]>([]);
    const [selected, setSelected] = useState<SteeringFile | null>(null);
    const [editContent, setEditContent] = useState('');
    const [saving, setSaving] = useState(false);
    const [loading, setLoading] = useState(false);
    const editorRef = useRef<MonacoType.editor.IStandaloneCodeEditor | null>(null);
    const editorDivRef = useRef<HTMLDivElement>(null);
    const activeRoot = useStore(s => s.activeRoot);

    const steeringDir = activeRoot ? `${activeRoot}/.agent/steering` : null;

    const load = async () => {
        if (!steeringDir) return;
        setLoading(true);
        try {
            await invoke('create_directory', { path: steeringDir }).catch(() => null);
            const entries = await invoke<any[]>('list_directory', { path: steeringDir }).catch(() => []);
            const mdFiles = (entries || []).filter((e: any) => !e.is_dir && e.name.endsWith('.md'));

            const loaded: SteeringFile[] = await Promise.all(
                mdFiles.map(async (e: any) => {
                    const content = await invoke<string>('read_file', { path: e.path }).catch(() => '');
                    const descLine = content.split('\n').find((l: string) => l.startsWith('# '));
                    return {
                        name: e.name.replace('.md', ''),
                        path: e.path,
                        content,
                        description: descLine ? descLine.slice(2).trim() : e.name.replace('.md', ''),
                        alwaysOn: true,
                    };
                })
            );
            setFiles(loaded);
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => { load(); }, [activeRoot]);

    useEffect(() => {
        if (!selected || !editorDivRef.current) return;
        const monaco = (window as any).monaco as typeof MonacoType | undefined;
        if (!monaco) return;

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
        return () => { ed.dispose(); editorRef.current = null; };
    // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [selected?.path]);

    const selectFile = (f: SteeringFile) => {
        setSelected(f);
        setEditContent(f.content);
        setTimeout(() => { if (editorRef.current) editorRef.current.setValue(f.content); }, 50);
    };

    const saveFile = async () => {
        if (!selected) return;
        setSaving(true);
        try {
            await invoke('write_file_content', { path: selected.path, content: editContent });
            setFiles(prev => prev.map(f => f.path === selected.path ? { ...f, content: editContent } : f));
        } finally {
            setSaving(false);
        }
    };

    const createFromTemplate = async (tmpl: typeof BUILTIN_TEMPLATES[0]) => {
        if (!steeringDir) return;
        const path = `${steeringDir}/${tmpl.name}.md`;
        await invoke('write_file_content', { path, content: tmpl.template });
        await load();
    };

    const createBlank = async () => {
        if (!steeringDir) return;
        const name = prompt('Steering file name (no spaces):');
        if (!name) return;
        const safe = name.replace(/[^a-z0-9_-]/gi, '-').toLowerCase();
        const path = `${steeringDir}/${safe}.md`;
        await invoke('write_file_content', { path, content: `# ${safe}\n\nDescribe steering instructions here.\n` });
        await load();
    };

    const deleteFile = async (f: SteeringFile) => {
        if (!confirm(`Delete steering file "${f.name}"?`)) return;
        await invoke('delete_path', { path: f.path }).catch(() => null);
        if (selected?.path === f.path) setSelected(null);
        await load();
    };

    return (
        <div style={{ maxWidth: 860 }}>
            <div className="settings-section-title" style={{ display: 'flex', alignItems: 'center', gap: 10 }}>
                Steering
                <span className="settings-badge new">Agent</span>
            </div>
            <p className="settings-section-subtitle">
                Steering files in <code>.agent/steering/</code> are injected into every AI request. Use them to define project-wide coding standards, architecture rules, or domain context. Always active — no trigger needed.
            </p>

            <div style={{ display: 'flex', gap: 16, height: 480 }}>
                {/* Left: file list */}
                <div style={{ width: 220, flexShrink: 0, display: 'flex', flexDirection: 'column', gap: 8 }}>
                    <div style={{ display: 'flex', gap: 6 }}>
                        <button className="settings-button" style={{ flex: 1, fontSize: 11 }} onClick={createBlank}>
                            + New File
                        </button>
                        <button
                            className="settings-button"
                            style={{ padding: '5px 8px', fontSize: 11, opacity: 0.7 }}
                            title="Refresh"
                            onClick={load}
                        >
                            <i className={`codicon codicon-${loading ? 'loading~spin' : 'refresh'}`} />
                        </button>
                    </div>

                    {files.length === 0 ? (
                        <div style={{ fontSize: 11, opacity: 0.4, padding: '12px 0', textAlign: 'center' }}>
                            No steering files yet
                        </div>
                    ) : (
                        files.map(f => (
                            <div
                                key={f.path}
                                className={`steering-item ${selected?.path === f.path ? 'active' : ''}`}
                                onClick={() => selectFile(f)}
                            >
                                <i className="codicon codicon-symbol-file" style={{ fontSize: 14, marginTop: 1, color: '#c084fc' }} />
                                <div style={{ flex: 1, minWidth: 0 }}>
                                    <div style={{ fontSize: 12, fontWeight: 600, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{f.name}</div>
                                    <div style={{ fontSize: 10, opacity: 0.5, marginTop: 2, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{f.description}</div>
                                </div>
                                <button
                                    onClick={(e) => { e.stopPropagation(); deleteFile(f); }}
                                    style={{ background: 'none', border: 'none', color: 'rgba(248,81,73,0.5)', cursor: 'pointer', fontSize: 12, padding: 2 }}
                                    title="Delete"
                                >
                                    <i className="codicon codicon-trash" />
                                </button>
                            </div>
                        ))
                    )}

                    {/* Template suggestions */}
                    <div style={{ marginTop: 8, fontSize: 10, opacity: 0.4, textTransform: 'uppercase', letterSpacing: '0.5px', padding: '4px 0 2px' }}>
                        Templates
                    </div>
                    {BUILTIN_TEMPLATES.filter(t => !files.find(f => f.name === t.name)).map(t => (
                        <button
                            key={t.name}
                            onClick={() => createFromTemplate(t)}
                            style={{
                                background: 'rgba(255,255,255,0.02)',
                                border: '1px dashed rgba(255,255,255,0.12)',
                                borderRadius: 6,
                                padding: '7px 10px',
                                color: 'rgba(255,255,255,0.5)',
                                cursor: 'pointer',
                                fontSize: 11,
                                textAlign: 'left',
                                display: 'flex',
                                gap: 6,
                                alignItems: 'center',
                            }}
                        >
                            <i className="codicon codicon-add" style={{ fontSize: 12 }} />
                            {t.description}
                        </button>
                    ))}
                </div>

                {/* Right: editor */}
                <div style={{ flex: 1, display: 'flex', flexDirection: 'column', border: '1px solid rgba(255,255,255,0.08)', borderRadius: 8, overflow: 'hidden' }}>
                    {selected ? (
                        <>
                            <div style={{ padding: '8px 12px', borderBottom: '1px solid rgba(255,255,255,0.06)', display: 'flex', alignItems: 'center', gap: 8, background: 'rgba(255,255,255,0.02)' }}>
                                <i className="codicon codicon-symbol-file" style={{ color: '#c084fc', fontSize: 13 }} />
                                <span style={{ fontSize: 12, fontWeight: 600, flex: 1 }}>{selected.name}.md</span>
                                <span style={{ fontSize: 10, opacity: 0.4 }}>Always active</span>
                                <button
                                    className="settings-button"
                                    style={{ fontSize: 11, padding: '3px 10px' }}
                                    onClick={saveFile}
                                    disabled={saving}
                                >
                                    {saving ? 'Saving...' : 'Save'}
                                </button>
                            </div>
                            <div ref={editorDivRef} style={{ flex: 1 }} />
                        </>
                    ) : (
                        <div style={{ flex: 1, display: 'flex', alignItems: 'center', justifyContent: 'center', opacity: 0.3, flexDirection: 'column', gap: 8 }}>
                            <i className="codicon codicon-symbol-file" style={{ fontSize: 32 }} />
                            <span style={{ fontSize: 12 }}>Select a steering file to edit</span>
                        </div>
                    )}
                </div>
            </div>
        </div>
    );
};

export default SteeringPanel;
