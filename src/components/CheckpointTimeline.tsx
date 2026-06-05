import React, { useEffect, useState, useCallback } from 'react';
import { invoke } from '../tauri_bridge';

interface Checkpoint {
    id: string;
    name: string;
    description: string;
    timestamp: number;
    datetime: string;
    files_changed: number;
    is_ai_generated: boolean;
    can_rollback: boolean;
}

interface FileDiff { path: string; status: string; additions: number; deletions: number; patch?: string }
interface CheckpointDiff { files: FileDiff[]; total_additions: number; total_deletions: number }

const statusColor: Record<string, string> = {
    added: '#22c55e', modified: '#fbbf24', deleted: '#f87171', renamed: '#60a5fa',
};

/**
 * Visual timeline of code checkpoints (Void-style). Lists restore points, expands
 * to a real per-file diff, and restores/deletes via the git_checkpoints backend.
 */
const CheckpointTimeline: React.FC<{ embedded?: boolean }> = ({ embedded = false }) => {
    const [checkpoints, setCheckpoints] = useState<Checkpoint[]>([]);
    const [loading, setLoading] = useState(false);
    const [expandedId, setExpandedId] = useState<string | null>(null);
    const [diff, setDiff] = useState<CheckpointDiff | null>(null);
    const [diffLoading, setDiffLoading] = useState(false);
    const [busy, setBusy] = useState<string | null>(null);

    const refresh = useCallback(async () => {
        setLoading(true);
        try {
            const res = await invoke<Checkpoint[]>('git_list_checkpoints', { limit: 50 });
            setCheckpoints(Array.isArray(res) ? res : []);
        } catch (e) {
            console.error('[Checkpoints] list failed:', e);
            setCheckpoints([]);
        } finally {
            setLoading(false);
        }
    }, []);

    useEffect(() => { refresh(); }, [refresh]);

    const toggle = async (cp: Checkpoint) => {
        if (expandedId === cp.id) { setExpandedId(null); setDiff(null); return; }
        setExpandedId(cp.id);
        setDiff(null);
        setDiffLoading(true);
        try {
            const d = await invoke<CheckpointDiff>('git_get_checkpoint_diff', { checkpointId: cp.id });
            setDiff(d);
        } catch (e) {
            console.error('[Checkpoints] diff failed:', e);
        } finally {
            setDiffLoading(false);
        }
    };

    const restore = async (cp: Checkpoint) => {
        if (!window.confirm(`Restore the workspace to this checkpoint?\n\n"${cp.description || cp.name}"\n\nUncommitted changes since then will be discarded.`)) return;
        setBusy(cp.id);
        try {
            await invoke('git_rollback_checkpoint', { checkpointId: cp.id });
            window.dispatchEvent(new CustomEvent('reload-window'));
        } catch (e) {
            window.alert('Restore failed: ' + e);
        } finally {
            setBusy(null);
        }
    };

    const remove = async (cp: Checkpoint) => {
        setBusy(cp.id);
        try {
            await invoke('git_delete_checkpoint', { checkpointId: cp.id });
            await refresh();
            if (expandedId === cp.id) { setExpandedId(null); setDiff(null); }
        } catch (e) {
            console.error('[Checkpoints] delete failed:', e);
        } finally {
            setBusy(null);
        }
    };

    return (
        <div style={{ marginTop: embedded ? 12 : 20 }}>
            {!embedded && (
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 8 }}>
                <span style={{ fontSize: 11, fontWeight: 600, textTransform: 'uppercase', opacity: 0.5 }}>Code Checkpoints</span>
                <i className="codicon codicon-refresh" onClick={refresh} title="Refresh" style={{ cursor: 'pointer', fontSize: 12, opacity: 0.6 }} />
            </div>
            )}
            {embedded && (
                <div style={{ display: 'flex', justifyContent: 'flex-end', marginBottom: 8 }}>
                    <i className="codicon codicon-refresh" onClick={refresh} title="Refresh checkpoints" style={{ cursor: 'pointer', fontSize: 12, opacity: 0.6 }} />
                </div>
            )}

            {loading ? (
                <div style={{ padding: 20, textAlign: 'center', opacity: 0.5, fontSize: 12 }}>Loading checkpoints…</div>
            ) : checkpoints.length === 0 ? (
                <div style={{ padding: 20, textAlign: 'center', opacity: 0.5, fontSize: 12 }}>No checkpoints yet. They're created automatically before agent edits.</div>
            ) : (
                <div style={{ position: 'relative', display: 'flex', flexDirection: 'column', gap: 0 }}>
                    {checkpoints.map((cp, idx) => {
                        const open = expandedId === cp.id;
                        const last = idx === checkpoints.length - 1;
                        return (
                            <div key={cp.id} style={{ display: 'flex', gap: 10 }}>
                                {/* timeline rail */}
                                <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', width: 14, flexShrink: 0 }}>
                                    <span style={{ width: 9, height: 9, borderRadius: '50%', marginTop: 14, background: cp.is_ai_generated ? '#a855f7' : '#60a5fa', flexShrink: 0 }} />
                                    {!last && <span style={{ flex: 1, width: 2, background: 'rgba(255,255,255,0.08)' }} />}
                                </div>
                                <div style={{ flex: 1, paddingBottom: 10, minWidth: 0 }}>
                                    <div
                                        onClick={() => toggle(cp)}
                                        style={{ cursor: 'pointer', padding: '8px 10px', background: open ? 'rgba(255,255,255,0.05)' : 'rgba(255,255,255,0.02)', borderRadius: 6, border: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.06))' }}
                                    >
                                        <div style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
                                            <span style={{ fontSize: 12, fontWeight: 600, flex: 1, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{cp.description || cp.name}</span>
                                            {cp.is_ai_generated && <span style={{ fontSize: 8, padding: '1px 5px', borderRadius: 3, background: 'rgba(168,85,247,0.15)', color: '#c084fc' }}>AI</span>}
                                            <span style={{ fontSize: 9, opacity: 0.4 }}>{cp.files_changed} files</span>
                                        </div>
                                        <div style={{ fontSize: 10, opacity: 0.45, marginTop: 2 }}>{cp.datetime}</div>
                                    </div>
                                    {open && (
                                        <div style={{ marginTop: 6, padding: '8px 10px', background: 'rgba(0,0,0,0.2)', borderRadius: 6 }}>
                                            {diffLoading ? (
                                                <div style={{ fontSize: 11, opacity: 0.5 }}>Loading diff…</div>
                                            ) : diff ? (
                                                <>
                                                    <div style={{ fontSize: 10, marginBottom: 6 }}>
                                                        <span style={{ color: '#22c55e' }}>+{diff.total_additions}</span>{' '}
                                                        <span style={{ color: '#f87171' }}>-{diff.total_deletions}</span>
                                                    </div>
                                                    {diff.files.map((f, i) => (
                                                        <div key={i} style={{ fontSize: 11, display: 'flex', alignItems: 'center', gap: 6, padding: '2px 0' }}>
                                                            <span style={{ width: 6, height: 6, borderRadius: '50%', background: statusColor[f.status] || '#888', flexShrink: 0 }} />
                                                            <span style={{ flex: 1, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap', fontFamily: 'var(--font-mono)' }} title={f.path}>{f.path}</span>
                                                            <span style={{ color: '#22c55e', fontSize: 10 }}>+{f.additions}</span>
                                                            <span style={{ color: '#f87171', fontSize: 10 }}>-{f.deletions}</span>
                                                        </div>
                                                    ))}
                                                </>
                                            ) : (
                                                <div style={{ fontSize: 11, opacity: 0.5 }}>No diff available.</div>
                                            )}
                                            <div style={{ display: 'flex', gap: 8, marginTop: 10 }}>
                                                {cp.can_rollback && (
                                                    <button onClick={(e) => { e.stopPropagation(); restore(cp); }} disabled={busy === cp.id}
                                                        style={{ fontSize: 11, fontWeight: 600, padding: '4px 10px', borderRadius: 4, border: 'none', cursor: 'pointer', background: 'rgba(168,85,247,0.15)', color: '#c084fc' }}>
                                                        {busy === cp.id ? '…' : '↶ Restore'}
                                                    </button>
                                                )}
                                                <button onClick={(e) => { e.stopPropagation(); remove(cp); }} disabled={busy === cp.id}
                                                    style={{ fontSize: 11, padding: '4px 10px', borderRadius: 4, border: '1px solid rgba(248,113,113,0.3)', cursor: 'pointer', background: 'transparent', color: '#f87171' }}>
                                                    Delete
                                                </button>
                                            </div>
                                        </div>
                                    )}
                                </div>
                            </div>
                        );
                    })}
                </div>
            )}
        </div>
    );
};

export default CheckpointTimeline;
