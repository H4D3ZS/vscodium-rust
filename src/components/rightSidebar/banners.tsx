// Chat banners: plan approval, checkpoint restore, multi-file review.
// Extracted from RightSidebar.tsx (A2 god-component split).
import React, { useEffect, useState, useRef, useMemo, useCallback, memo } from 'react';
import { useStore } from '../../store';
import { invoke } from '../../tauri_bridge';



// ── Restore-checkpoint banner ────────────────────────────────────────────
// Shows above the chat input whenever an agent turn just auto-snapshotted
// the workspace, giving the user a one-click "undo the AI's edits" path.
// ── Plan approval banner ──────────────────────────────────────────────────
// Appears when the agent outputs AWAITING_APPROVAL in plan mode.
// User reviews the plan and clicks Approve to resume execution.
const PlanApprovalBanner: React.FC = memo(() => {
    const [planData, setPlanData] = React.useState<{ plan: string; iteration: number } | null>(null);
    const [approving, setApproving] = React.useState(false);

    React.useEffect(() => {
        import('@tauri-apps/api/event').then(({ listen }) => {
            const unlisten = listen('plan-approval-required', (event: any) => {
                setPlanData(event.payload ?? null);
            });
            return () => { unlisten.then(f => f()); };
        });
    }, []);

    if (!planData) return null;

    const handleApprove = async () => {
        setApproving(true);
        try { await (await import('../../tauri_bridge')).invoke('resume_ai_agent'); } catch { /* ignore */ }
        setPlanData(null);
        setApproving(false);
    };

    const handleReject = async () => {
        try { await (await import('../../tauri_bridge')).invoke('stop_ai_agent'); } catch { /* ignore */ }
        setPlanData(null);
    };

    return (
        <div style={{
            display: 'flex', flexDirection: 'column', gap: 8,
            padding: '10px 12px', marginBottom: 6,
            background: 'rgba(245,158,11,0.07)',
            border: '1px solid rgba(245,158,11,0.3)',
            borderRadius: 8, fontSize: 11,
        }}>
            <div style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
                <span style={{ fontSize: 13 }}>📋</span>
                <strong style={{ color: '#fbbf24' }}>Plan ready — approve to execute</strong>
            </div>
            {planData.plan && (
                <pre style={{
                    margin: 0, padding: '6px 10px',
                    background: 'rgba(0,0,0,0.3)', borderRadius: 5,
                    fontSize: 10, color: 'rgba(255,255,255,0.75)',
                    maxHeight: 180, overflowY: 'auto', whiteSpace: 'pre-wrap',
                }}>
                    {planData.plan}
                </pre>
            )}
            <div style={{ display: 'flex', gap: 8 }}>
                <button
                    disabled={approving}
                    onClick={handleApprove}
                    style={{
                        flex: 1, padding: '5px 10px', fontSize: 11, fontWeight: 700, borderRadius: 5,
                        background: '#d97706', color: 'var(--vscode-editor-foreground, #fff)', border: 'none', cursor: 'pointer',
                    }}
                >
                    {approving ? '…' : '✓ Approve & Execute'}
                </button>
                <button
                    onClick={handleReject}
                    style={{
                        padding: '5px 10px', fontSize: 11, fontWeight: 700, borderRadius: 5,
                        background: 'rgba(248,113,113,0.15)', color: '#f87171',
                        border: '1px solid rgba(248,113,113,0.3)', cursor: 'pointer',
                    }}
                >
                    ✕ Cancel
                </button>
            </div>
        </div>
    );
});


const RestoreCheckpointBanner: React.FC = memo(() => {
    const checkpoint = useStore(state => state.lastAgentCheckpoint);
    const rollback = useStore(state => state.rollbackLastAgentCheckpoint);
    const dismiss = useStore(state => state.setLastAgentCheckpoint);
    const [busy, setBusy] = useState(false);
    const [msg, setMsg] = useState<string | null>(null);
    if (!checkpoint) return null;
    const age = Math.max(1, Math.round((Date.now() - checkpoint.timestamp) / 1000));
    return (
        <div style={{
            display: 'flex', alignItems: 'center', gap: 8,
            padding: '6px 10px', marginBottom: 6,
            background: 'var(--vscode-inputValidation-infoBackground, rgba(0,122,204,0.1))',
            border: '1px solid var(--vscode-inputValidation-infoBorder, rgba(0,122,204,0.35))',
            borderRadius: 8, fontSize: 11,
        }}>
            <i className="codicon codicon-discard" style={{ fontFamily: 'codicon', fontStyle: 'normal', color: 'var(--vscode-textLink-foreground, #3794ff)', fontSize: 13 }} />
            <span style={{ flex: 1, color: 'rgba(255,255,255,0.85)' }}>
                {msg ?? <>Checkpoint <code style={{ opacity: 0.7 }}>{checkpoint.description}</code> · {age}s ago</>}
            </span>
            <button
                disabled={busy}
                onClick={async () => {
                    setBusy(true);
                    const r = await rollback();
                    setBusy(false);
                    setMsg(r.ok ? 'Restored.' : r.message);
                    if (r.ok) setTimeout(() => setMsg(null), 1800);
                }}
                style={{ background: 'var(--vscode-button-background)', color: 'var(--vscode-button-foreground)', border: 'none', padding: '2px 8px', fontSize: 10, fontWeight: 600, borderRadius: 2, cursor: busy ? 'wait' : 'pointer' }}
            >
                {busy ? '…' : '↶ Restore'}
            </button>
            <i
                className="codicon codicon-close"
                onClick={() => dismiss(null)}
                style={{ fontFamily: 'codicon', fontStyle: 'normal', cursor: 'pointer', opacity: 0.5, fontSize: 11 }}
                title="Dismiss"
            />
        </div>
    );
});


// ── Multi-file review banner ──────────────────────────────────────────────
// Appears after the agent's turn whenever it touched 2+ files. Clicking
// opens the MultiFileReview carousel where the user can step through each
// diff and keep/revert per file.
const MultiFileReviewBanner: React.FC = memo(() => {
    const edits = useStore(s => s.pendingAgentEdits);
    const isThinking = useStore(s => s.isAgentThinking);
    const openReview = useStore(s => s.openMultiFileReview);
    // Only show after the turn finishes, with 2+ files. Single-file edits
    // are usually obvious and don't warrant a modal.
    if (isThinking) return null;
    if (edits.length < 2) return null;
    return (
        <div
            onClick={openReview}
            style={{
                display: 'flex', alignItems: 'center', gap: 8,
                padding: '7px 10px', marginBottom: 6,
                background: 'rgba(255,255,255,0.035)',
                border: '1px solid rgba(255,255,255,0.10)',
                borderRadius: 8, fontSize: 11,
                cursor: 'pointer',
            }}
        >
            <i className="codicon codicon-chevron-right" style={{ fontFamily: 'codicon', fontStyle: 'normal', opacity: 0.5, fontSize: 11 }} />
            <i className="codicon codicon-diff-multiple" style={{ fontFamily: 'codicon', fontStyle: 'normal', color: '#4ade80', fontSize: 12 }} />
            <span style={{ flex: 1, color: 'rgba(255,255,255,0.85)', fontWeight: 500 }}>
                {edits.length} {edits.length === 1 ? 'File' : 'Files'}
            </span>
            <span style={{
                background: 'rgba(255,255,255,0.08)', color: 'rgba(255,255,255,0.92)',
                padding: '2px 10px', fontSize: 10, fontWeight: 600, borderRadius: 5,
                border: '1px solid rgba(255,255,255,0.12)',
            }}>Review</span>
        </div>
    );
});

export { PlanApprovalBanner };
export { RestoreCheckpointBanner };
export { MultiFileReviewBanner };
