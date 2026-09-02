import React, { useEffect, useMemo, useState, useCallback } from 'react';
import { useStore } from '../../store';
import { invoke } from '../../tauri_bridge';

// ─────────────────────────────────────────────────────────────────────────────
//  MultiFileReview — Cursor / Antigravity-style review carousel that pops
//  whenever the agent's last turn touched 2+ files. Each entry shows the
//  unified git diff for that file (vs the just-snapshotted checkpoint) and
//  exposes per-file Keep / Revert actions. A "Revert all" button rolls
//  the entire checkpoint back via the existing git_rollback_checkpoint
//  command if the user decides the whole turn was wrong.
//
//  Mounting strategy: we attach this once at the top of the app shell, it
//  becomes visible when `isMultiFileReviewOpen` flips true. The right
//  sidebar surfaces a banner button after each turn to open it.
// ─────────────────────────────────────────────────────────────────────────────

interface PerFileState {
    path: string;
    tool: string;
    diff: string;
    loading: boolean;
    decision: 'pending' | 'kept' | 'reverted';
    error?: string;
}

const MultiFileReview: React.FC = () => {
    const open = useStore(s => s.isMultiFileReviewOpen);
    const close = useStore(s => s.closeMultiFileReview);
    const edits = useStore(s => s.pendingAgentEdits);
    const lastCheckpoint = useStore(s => s.lastAgentCheckpoint);
    const rollbackAll = useStore(s => s.rollbackLastAgentCheckpoint);

    const [activeIdx, setActiveIdx] = useState(0);
    const [perFile, setPerFile] = useState<Record<string, PerFileState>>({});

    // Reset carousel position whenever a new turn populates the list.
    useEffect(() => {
        if (open) setActiveIdx(0);
    }, [open, edits.length]);

    // Lazy-load the diff for the file currently in view. We use
    // git_diff_file which compares working tree against HEAD; since the
    // snapshotCheckpoint commit lives at the top of the log this is also
    // the diff vs the checkpoint.
    useEffect(() => {
        if (!open) return;
        const current = edits[activeIdx];
        if (!current) return;
        if (perFile[current.path]?.diff) return;
        let cancelled = false;
        (async () => {
            setPerFile(s => ({
                ...s,
                [current.path]: {
                    path: current.path,
                    tool: current.tool,
                    diff: '',
                    loading: true,
                    decision: 'pending',
                },
            }));
            try {
                const diff = await invoke<string>('git_diff_file', { path: current.path }).catch(() => '');
                if (cancelled) return;
                setPerFile(s => ({
                    ...s,
                    [current.path]: {
                        ...s[current.path],
                        diff: diff || '(no diff available — file may be new or untracked)',
                        loading: false,
                    },
                }));
            } catch (e: any) {
                if (cancelled) return;
                setPerFile(s => ({
                    ...s,
                    [current.path]: {
                        ...s[current.path],
                        diff: '',
                        loading: false,
                        error: String(e?.message || e),
                    },
                }));
            }
        })();
        return () => { cancelled = true; };
    }, [open, activeIdx, edits, perFile]);

    const revertFile = useCallback(async (path: string) => {
        try {
            // The cleanest revert is `git checkout HEAD~1 -- <path>` since
            // the checkpoint commit is at the top of the log. We invoke
            // ai_execute_command which does shell escaping and returns
            // stdout/stderr for us to surface inline.
            const cmd = `git checkout HEAD~1 -- "${path}"`;
            const out = await invoke<string>('ai_execute_command', { command: cmd }).catch((e) => `ERR ${e}`);
            setPerFile(s => ({
                ...s,
                [path]: { ...s[path], decision: 'reverted', error: undefined, diff: s[path]?.diff || '', loading: false, path, tool: s[path]?.tool || '' },
            }));
            window.dispatchEvent(new CustomEvent('editor:reload-file', { detail: { path } }));
        } catch (e) {
            setPerFile(s => ({
                ...s,
                [path]: { ...s[path], error: String(e), path, tool: s[path]?.tool || '', diff: s[path]?.diff || '', loading: false, decision: s[path]?.decision || 'pending' },
            }));
        }
    }, []);

    const keepFile = useCallback((path: string) => {
        setPerFile(s => ({
            ...s,
            [path]: { ...s[path], decision: 'kept', path, tool: s[path]?.tool || '', diff: s[path]?.diff || '', loading: false },
        }));
    }, []);

    const onRevertAll = useCallback(async () => {
        if (!lastCheckpoint) return;
        const ok = window.confirm(
            `Roll back all ${edits.length} files to the checkpoint?\n\nThis cannot be undone.`,
        );
        if (!ok) return;
        const res = await rollbackAll();
        if (res.ok) {
            close();
            // Force open editors to reload from disk.
            edits.forEach(e => {
                window.dispatchEvent(new CustomEvent('editor:reload-file', { detail: { path: e.path } }));
            });
        } else {
            window.alert('Rollback failed: ' + res.message);
        }
    }, [close, edits, lastCheckpoint, rollbackAll]);

    const current = edits[activeIdx];
    const currentState = current ? perFile[current.path] : null;
    const stats = useMemo(() => {
        const kept = Object.values(perFile).filter(p => p.decision === 'kept').length;
        const reverted = Object.values(perFile).filter(p => p.decision === 'reverted').length;
        return { kept, reverted, total: edits.length };
    }, [edits.length, perFile]);

    if (!open) return null;

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
                    width: 'min(1100px, 92vw)',
                    height: 'min(720px, 90vh)',
                    background: 'var(--vscode-editor-background, #1e1e1e)',
                    color: 'var(--vscode-foreground, #ddd)',
                    border: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.08))',
                    borderRadius: 8,
                    display: 'flex',
                    flexDirection: 'column',
                    overflow: 'hidden',
                    boxShadow: '0 24px 48px rgba(0,0,0,0.45)',
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
                        <i className="codicon codicon-diff-multiple" style={{ fontFamily: 'codicon', fontStyle: 'normal' }} />
                        <span style={{ fontWeight: 600, fontSize: 13 }}>
                            Review agent changes — {edits.length} file{edits.length === 1 ? '' : 's'}
                        </span>
                        <span style={{ fontSize: 11, opacity: 0.6, marginLeft: 12 }}>
                            {stats.kept} kept · {stats.reverted} reverted
                        </span>
                    </div>
                    <div style={{ display: 'flex', gap: 8 }}>
                        {lastCheckpoint && (
                            <button onClick={onRevertAll} style={btnDanger}>
                                <i className="codicon codicon-discard" style={iconStyle} /> Revert all
                            </button>
                        )}
                        <button onClick={close} style={btnNeutral}>
                            <i className="codicon codicon-close" style={iconStyle} /> Done
                        </button>
                    </div>
                </div>

                {/* Body: file list + diff */}
                <div style={{ flex: '1 1 auto', display: 'flex', minHeight: 0 }}>
                    <div
                        style={{
                            flex: '0 0 240px',
                            borderRight: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.08))',
                            overflowY: 'auto',
                            background: 'var(--vscode-sideBar-background, rgba(0,0,0,0.2))',
                        }}
                    >
                        {edits.length === 0 && (
                            <div style={{ padding: 12, fontSize: 12, opacity: 0.55 }}>
                                No file changes this turn.
                            </div>
                        )}
                        {edits.map((e, i) => {
                            const decision = perFile[e.path]?.decision || 'pending';
                            const active = i === activeIdx;
                            return (
                                <div
                                    key={e.path}
                                    onClick={() => setActiveIdx(i)}
                                    style={{
                                        padding: '8px 12px',
                                        cursor: 'pointer',
                                        fontSize: 12,
                                        background: active ? 'rgba(124,58,237,0.18)' : 'transparent',
                                        borderLeft: active ? '2px solid #c084fc' : '2px solid transparent',
                                        display: 'flex',
                                        alignItems: 'center',
                                        gap: 6,
                                    }}
                                >
                                    <i
                                        className={`codicon codicon-${
                                            decision === 'kept'     ? 'check' :
                                            decision === 'reverted' ? 'discard' :
                                                                      'diff'
                                        }`}
                                        style={{
                                            ...iconStyle,
                                            color:
                                                decision === 'kept'     ? '#10b981' :
                                                decision === 'reverted' ? '#ef4444' :
                                                                          'rgba(255,255,255,0.5)',
                                        }}
                                    />
                                    <span
                                        style={{
                                            overflow: 'hidden',
                                            textOverflow: 'ellipsis',
                                            whiteSpace: 'nowrap',
                                        }}
                                        title={e.path}
                                    >
                                        {e.path.split(/[\\/]/).slice(-2).join('/')}
                                    </span>
                                </div>
                            );
                        })}
                    </div>

                    <div style={{ flex: '1 1 auto', display: 'flex', flexDirection: 'column', minWidth: 0 }}>
                        {/* File header */}
                        {current && (
                            <div
                                style={{
                                    padding: '6px 14px',
                                    borderBottom: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.08))',
                                    fontSize: 11,
                                    fontFamily: 'var(--font-mono, monospace)',
                                    display: 'flex',
                                    justifyContent: 'space-between',
                                    alignItems: 'center',
                                    gap: 12,
                                }}
                            >
                                <div style={{ overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                                    <span style={{ opacity: 0.55, marginRight: 8 }}>{current.tool}</span>
                                    {current.path}
                                </div>
                                <div style={{ display: 'flex', gap: 6 }}>
                                    <button
                                        onClick={() => keepFile(current.path)}
                                        disabled={currentState?.decision === 'kept'}
                                        style={btnSuccess}
                                    >
                                        <i className="codicon codicon-check" style={iconStyle} /> Keep
                                    </button>
                                    <button
                                        onClick={() => revertFile(current.path)}
                                        disabled={currentState?.decision === 'reverted'}
                                        style={btnDanger}
                                    >
                                        <i className="codicon codicon-discard" style={iconStyle} /> Revert
                                    </button>
                                    <button
                                        onClick={() => setActiveIdx((i) => Math.max(0, i - 1))}
                                        disabled={activeIdx === 0}
                                        style={btnNeutral}
                                    >
                                        <i className="codicon codicon-chevron-left" style={iconStyle} />
                                    </button>
                                    <span style={{ fontSize: 11, opacity: 0.6, alignSelf: 'center' }}>
                                        {activeIdx + 1}/{edits.length}
                                    </span>
                                    <button
                                        onClick={() => setActiveIdx((i) => Math.min(edits.length - 1, i + 1))}
                                        disabled={activeIdx >= edits.length - 1}
                                        style={btnNeutral}
                                    >
                                        <i className="codicon codicon-chevron-right" style={iconStyle} />
                                    </button>
                                </div>
                            </div>
                        )}

                        {/* Diff */}
                        <div style={{ flex: '1 1 auto', overflow: 'auto', padding: 14 }}>
                            {!current && (
                                <div style={{ opacity: 0.55, fontSize: 12 }}>Nothing selected.</div>
                            )}
                            {current && currentState?.loading && (
                                <div style={{ opacity: 0.6, fontSize: 12 }}>Loading diff…</div>
                            )}
                            {current && currentState && !currentState.loading && (
                                <pre
                                    style={{
                                        margin: 0,
                                        fontSize: 12,
                                        lineHeight: 1.5,
                                        fontFamily: 'var(--font-mono, monospace)',
                                        whiteSpace: 'pre',
                                    }}
                                >
                                    {currentState.diff
                                        .split('\n')
                                        .map((line, i) => {
                                            // Cheap colourisation — green +, red −, dim headers.
                                            let color = 'inherit';
                                            if (line.startsWith('+') && !line.startsWith('+++')) color = '#4ade80';
                                            else if (line.startsWith('-') && !line.startsWith('---')) color = '#f87171';
                                            else if (line.startsWith('@@')) color = '#c084fc';
                                            else if (line.startsWith('diff ') || line.startsWith('index ') || line.startsWith('+++') || line.startsWith('---')) color = 'rgba(255,255,255,0.4)';
                                            return (
                                                <div key={i} style={{ color }}>{line || '\u00a0'}</div>
                                            );
                                        })
                                    }
                                </pre>
                            )}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
};

const btnBase: React.CSSProperties = {
    display: 'inline-flex',
    alignItems: 'center',
    gap: 4,
    padding: '3px 10px',
    borderRadius: 4,
    fontSize: 11,
    cursor: 'pointer',
    fontFamily: 'inherit',
};

const btnNeutral: React.CSSProperties = {
    ...btnBase,
    background: 'rgba(255,255,255,0.06)',
    border: '1px solid rgba(255,255,255,0.1)',
    color: 'rgba(255,255,255,0.85)',
};

const btnSuccess: React.CSSProperties = {
    ...btnBase,
    background: 'rgba(16,185,129,0.16)',
    border: '1px solid rgba(16,185,129,0.4)',
    color: '#10b981',
};

const btnDanger: React.CSSProperties = {
    ...btnBase,
    background: 'rgba(239,68,68,0.14)',
    border: '1px solid rgba(239,68,68,0.4)',
    color: '#f87171',
};

const iconStyle: React.CSSProperties = {
    fontFamily: 'codicon',
    fontStyle: 'normal',
    fontSize: 11,
};

export default MultiFileReview;
