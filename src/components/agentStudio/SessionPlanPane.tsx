import React, { useCallback, useEffect, useState } from 'react';
import { useStore } from '../../store';
import type { SessionPlanFile, SessionPlanFileId } from '../../domain/research/SessionPlanFile';
import { ensureSessionPlanFiles, loadSessionPlanFiles } from '../../application/research/loadSessionPlanFiles';
import { fileRepository } from '../../infrastructure/editor/TauriFileRepository';
import { openFile } from '../../application/editor/openFile';

const TAB_ORDER: SessionPlanFileId[] = ['task_plan', 'findings', 'progress'];

const SessionPlanPane: React.FC = () => {
    const activeRoot = useStore(s => s.activeRoot);
    const [files, setFiles] = useState<SessionPlanFile[]>([]);
    const [activeId, setActiveId] = useState<SessionPlanFileId>('task_plan');
    const [loading, setLoading] = useState(false);
    const [saving, setSaving] = useState(false);

    const refresh = useCallback(async () => {
        if (!activeRoot) {
            setFiles([]);
            return;
        }
        setLoading(true);
        try {
            setFiles(await loadSessionPlanFiles(activeRoot));
        } finally {
            setLoading(false);
        }
    }, [activeRoot]);

    useEffect(() => { void refresh(); }, [refresh]);

    const active = files.find(f => f.id === activeId);
    const fullPath = activeRoot && active ? `${activeRoot.replace(/\//g, '\\')}\\${active.filename}` : '';

    const save = async () => {
        if (!activeRoot || !active) return;
        setSaving(true);
        try {
            await fileRepository.write(fullPath, active.content);
            await refresh();
        } finally {
            setSaving(false);
        }
    };

    const initFiles = async () => {
        if (!activeRoot) return;
        setLoading(true);
        try {
            setFiles(await ensureSessionPlanFiles(activeRoot));
        } finally {
            setLoading(false);
        }
    };

    if (!activeRoot) {
        return (
            <div style={{ padding: 16, fontSize: 12, opacity: 0.6 }}>
                Open a workspace folder to use session planning files.
            </div>
        );
    }

    return (
        <div style={{ display: 'flex', flexDirection: 'column', height: '100%' }}>
            <div style={{ padding: '10px 12px', borderBottom: '1px solid rgba(255,255,255,0.06)', fontSize: 11, opacity: 0.75 }}>
                Persistent session notes — <code>task_plan.md</code>, <code>findings.md</code>, <code>progress.md</code>
            </div>
            <div style={{ display: 'flex', gap: 4, padding: '8px 12px', borderBottom: '1px solid rgba(255,255,255,0.06)' }}>
                {TAB_ORDER.map(id => {
                    const f = files.find(x => x.id === id);
                    return (
                        <button key={id} onClick={() => setActiveId(id)} style={tabBtn(activeId === id)}>
                            {f?.filename || id}
                        </button>
                    );
                })}
                <div style={{ flex: 1 }} />
                <button onClick={() => void initFiles()} disabled={loading} style={toolBtn}>Init</button>
                <button onClick={() => fullPath && openFile(fullPath)} disabled={!fullPath} style={toolBtn}>Open in editor</button>
                <button onClick={() => void save()} disabled={saving || !active} style={toolBtn}>{saving ? 'Saving…' : 'Save'}</button>
            </div>
            <textarea
                value={active?.content || ''}
                onChange={e => setFiles(prev => prev.map(f => f.id === activeId ? { ...f, content: e.target.value } : f))}
                spellCheck={false}
                style={{
                    flex: 1, resize: 'none', border: 'none', outline: 'none',
                    background: 'var(--vscode-editor-background, #1e1e1e)',
                    color: 'var(--vscode-editor-foreground, #d4d4d4)',
                    fontFamily: 'var(--vscode-editor-font-family, Consolas, monospace)',
                    fontSize: 12, lineHeight: 1.5, padding: 12,
                }}
            />
        </div>
    );
};

const tabBtn = (active: boolean): React.CSSProperties => ({
    border: 'none',
    borderBottom: active ? '2px solid var(--vscode-focusBorder, #007acc)' : '2px solid transparent',
    background: 'transparent',
    color: active ? '#e7e7e7' : 'rgba(231,231,231,0.55)',
    padding: '4px 8px', fontSize: 11, cursor: 'pointer',
});

const toolBtn: React.CSSProperties = {
    border: '1px solid rgba(255,255,255,0.12)',
    background: 'rgba(255,255,255,0.04)',
    color: 'rgba(255,255,255,0.8)',
    borderRadius: 4, padding: '3px 8px', fontSize: 10, cursor: 'pointer',
};

export default SessionPlanPane;
