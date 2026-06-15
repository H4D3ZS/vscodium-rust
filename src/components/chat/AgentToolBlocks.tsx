/**
 * Cursor Composer–style tool invocation cards — terminal streaming, reads, diffs, todos.
 * Enhanced with progress bars, step counters, and cleaner visual hierarchy.
 * Recon noise (glob/list/grep) collapses into compact summaries.
 */
import React, { useMemo, useState } from 'react';
import type { AgentToolBlock } from '../../domain/agent/agentToolBlocks';
import { collapseToolBlocksForDisplay, type DisplayToolBlock } from '../../domain/agent/agentToolBlocks';
import InlineDiffPreview from './InlineDiffPreview';
import ComposerTodoList from './ComposerTodoList';
import { computeInlineDiff } from '../../domain/agent/inlineDiff';
import { useStore } from '../../store';
import { invoke } from '../../tauri_bridge';

/** Inline per-file Accept/Reject — mirrors MultiFileReview's keep/revert. */
const EditDecision: React.FC<{ path?: string }> = ({ path }) => {
    const [decision, setDecision] = useState<'pending' | 'accepted' | 'rejected'>('pending');
    const [busy, setBusy] = useState(false);
    if (!path) return null;

    const reject = async (e: React.MouseEvent) => {
        e.stopPropagation();
        if (busy || decision !== 'pending') return;
        setBusy(true);
        try {
            await invoke('ai_execute_command', { command: `git checkout HEAD~1 -- "${path}"` }).catch(() => {});
            window.dispatchEvent(new CustomEvent('editor:reload-file', { detail: { path } }));
            setDecision('rejected');
            try { useStore.getState().removePendingAgentEdit?.(path); } catch { /* non-fatal */ }
        } finally {
            setBusy(false);
        }
    };
    const accept = (e: React.MouseEvent) => {
        e.stopPropagation();
        if (decision !== 'pending') return;
        setDecision('accepted');
        try { useStore.getState().removePendingAgentEdit?.(path); } catch { /* non-fatal */ }
    };

    if (decision === 'accepted') {
        return (
            <span className="agent-edit-decided agent-edit-decided--ok" style={{ display: 'flex', alignItems: 'center', gap: '4px' }}>
                <i className="codicon codicon-check" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '11px' }} />
                Accepted
            </span>
        );
    }
    if (decision === 'rejected') {
        return (
            <span className="agent-edit-decided agent-edit-decided--no" style={{ display: 'flex', alignItems: 'center', gap: '4px' }}>
                <i className="codicon codicon-discard" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '11px' }} />
                Reverted
            </span>
        );
    }
    return (
        <div className="agent-edit-actions" onClick={(e) => e.stopPropagation()} style={{ display: 'flex', gap: '4px' }}>
            <button className="agent-edit-btn agent-edit-btn--reject" onClick={reject} disabled={busy} title="Revert this file" style={{
                display: 'flex', alignItems: 'center', gap: '3px',
                background: 'rgba(248,81,73,0.08)', border: '1px solid rgba(248,81,73,0.25)',
                color: '#f85149', padding: '2px 8px', fontSize: '10px', fontWeight: 600,
                borderRadius: '4px', cursor: 'pointer',
            }}>
                <i className="codicon codicon-discard" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '10px' }} />
                Reject
            </button>
            <button className="agent-edit-btn agent-edit-btn--accept" onClick={accept} disabled={busy} title="Keep this change" style={{
                display: 'flex', alignItems: 'center', gap: '3px',
                background: 'rgba(52,211,153,0.08)', border: '1px solid rgba(52,211,153,0.25)',
                color: '#34d399', padding: '2px 8px', fontSize: '10px', fontWeight: 600,
                borderRadius: '4px', cursor: 'pointer',
            }}>
                <i className="codicon codicon-check" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '10px' }} />
                Accept
            </button>
        </div>
    );
};

/** Count added/removed lines for the header badge */
function diffCounts(oldText?: string, newText?: string): { adds: number; dels: number } {
    if (!oldText && !newText) return { adds: 0, dels: 0 };
    const lines = computeInlineDiff(oldText || '', newText || '', 4000);
    let adds = 0, dels = 0;
    for (const l of lines) {
        if (l.type === 'add') adds++;
        else if (l.type === 'del') dels++;
    }
    return { adds, dels };
}

const cardStyle: React.CSSProperties = {
    marginBottom: '6px',
    borderRadius: '8px',
    border: '1px solid rgba(255,255,255,0.06)',
    background: 'rgba(255,255,255,0.02)',
    overflow: 'hidden',
};

const headerStyle: React.CSSProperties = {
    display: 'flex',
    alignItems: 'center',
    gap: '6px',
    padding: '5px 10px',
    fontSize: '11px',
    borderBottom: '1px solid rgba(255,255,255,0.04)',
    color: 'rgba(255,255,255,0.75)',
};

/** Animated progress bar for running tools */
const ProgressBar: React.FC<{ indeterminate?: boolean }> = ({ indeterminate }) => (
    <div style={{
        height: '2px', borderRadius: '1px',
        background: 'rgba(59,130,246,0.1)',
        overflow: 'hidden', marginTop: '2px',
    }}>
        <div style={{
            height: '100%', borderRadius: '1px',
            background: 'linear-gradient(90deg, #3b82f6, #60a5fa, #3b82f6)',
            backgroundSize: '200% 100%',
            animation: indeterminate ? 'progressSlide 1.5s ease-in-out infinite' : undefined,
            width: indeterminate ? '40%' : '100%',
        }} />
    </div>
);

const ReconSummaryCard: React.FC<{ count: number; labels: string[] }> = ({ count, labels }) => {
    const [open, setOpen] = useState(false);
    return (
        <div style={cardStyle}>
            <div
                style={{ ...headerStyle, cursor: 'pointer', borderBottom: open ? headerStyle.borderBottom : 'none' }}
                onClick={() => setOpen((v) => !v)}
            >
                <i className="codicon codicon-search" style={{ fontSize: 11, opacity: 0.4, fontFamily: 'codicon', fontStyle: 'normal' }} />
                <span style={{ opacity: 0.5, fontSize: '10px' }}>{count} recon steps</span>
                <i className={`codicon codicon-chevron-${open ? 'up' : 'down'}`} style={{ fontSize: 9, opacity: 0.3, marginLeft: 'auto', fontFamily: 'codicon', fontStyle: 'normal' }} />
            </div>
            {open && (
                <div style={{ padding: '6px 10px', fontSize: '10px', opacity: 0.45, lineHeight: 1.5 }}>
                    {labels.map((l, i) => <div key={i}>{l}</div>)}
                    {count > labels.length && <div>...and {count - labels.length} more</div>}
                </div>
            )}
        </div>
    );
};

const ToolBlockCard: React.FC<{ block: AgentToolBlock }> = ({ block }) => {
    const [open, setOpen] = useState(
        block.kind === 'terminal' || block.kind === 'edit' || block.status === 'running' || block.status === 'error',
    );
    const isRunning = block.status === 'running';
    const isError = block.status === 'error';

    const iconMap: Record<string, string> = {
        terminal: 'terminal',
        read: 'file',
        edit: 'file-code',
        todo: 'list-flat',
        search: 'search',
        canvas: 'dashboard',
    };
    const icon = iconMap[block.kind] || 'tools';

    const statusDot = (
        <span style={{
            width: 6, height: 6, borderRadius: '50%', flexShrink: 0,
            background: isRunning ? '#3794ff' : isError ? '#f48771' : 'rgba(255,255,255,0.25)',
            animation: isRunning ? 'hubPulse 1.5s infinite' : undefined,
        }} />
    );

    if (block.kind === 'canvas') {
        const canOpen = block.status === 'done' && !!block.canvasId;
        return (
            <div
                style={{
                    ...headerStyle, ...cardStyle, borderBottom: 'none',
                    cursor: canOpen ? 'pointer' : 'default',
                    opacity: isError ? 0.6 : 1,
                }}
                onClick={() => { if (canOpen && block.canvasId) useStore.getState().openCanvasTab(block.canvasId); }}
                title={canOpen ? 'Open canvas tab' : undefined}
            >
                {statusDot}
                <i className="codicon codicon-dashboard" style={{ fontSize: 12, opacity: 0.6, fontFamily: 'codicon', fontStyle: 'normal' }} />
                <span style={{ flex: 1, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap', fontSize: '11px' }}>
                    {isError ? `Canvas failed${block.canvasTitle ? ` · ${block.canvasTitle}` : ''}`
                        : isRunning ? `Rendering${block.canvasTitle ? ` · ${block.canvasTitle}` : ''}`
                            : `Open canvas${block.canvasTitle ? ` · ${block.canvasTitle}` : ''}`}
                </span>
                {canOpen && <i className="codicon codicon-link-external" style={{ fontSize: 10, opacity: 0.35, fontFamily: 'codicon', fontStyle: 'normal' }} />}
            </div>
        );
    }

    if (block.kind === 'todo' && block.todos?.length) {
        return (
            <div style={cardStyle}>
                <ComposerTodoList todos={block.todos} title={block.title} />
            </div>
        );
    }

    if ((block.kind === 'read' || block.kind === 'search') && !block.outputLines.length && !block.command && !isError) {
        return (
            <div style={{ ...headerStyle, ...cardStyle, borderBottom: 'none', opacity: 0.45, padding: '4px 10px' }}>
                {statusDot}
                <i className={`codicon codicon-${icon}`} style={{ fontSize: 10, opacity: 0.5, fontFamily: 'codicon', fontStyle: 'normal' }} />
                <span style={{ fontFamily: 'var(--font-mono)', fontSize: '10px' }}>{block.title}</span>
                {block.lineRange && <span style={{ opacity: 0.35, marginLeft: 'auto', fontSize: '9px' }}>{block.lineRange}</span>}
            </div>
        );
    }

    if (block.kind === 'edit') {
        const hasDiff = !!(block.oldText || block.newText);
        const fileName = (block.path || block.title || '').split(/[\\/]/).pop() || block.title;
        const { adds, dels } = hasDiff ? diffCounts(block.oldText, block.newText) : { adds: 0, dels: 0 };
        return (
            <div style={cardStyle} className="agent-edit-card">
                <div
                    style={{ ...headerStyle, cursor: 'pointer', borderBottom: open ? headerStyle.borderBottom : 'none' }}
                    onClick={() => setOpen((v) => !v)}
                >
                    {statusDot}
                    <span className="agent-edit-card__chip" style={{
                        display: 'flex', alignItems: 'center', gap: '4px',
                        padding: '1px 6px', borderRadius: '4px',
                        background: 'rgba(59,130,246,0.08)',
                    }}>
                        <i className="codicon codicon-file-code" style={{ fontSize: 10, opacity: 0.7, fontFamily: 'codicon', fontStyle: 'normal' }} />
                        <span className="agent-edit-card__name" style={{ fontSize: '11px' }}>{fileName}</span>
                    </span>
                    <span style={{ flex: 1 }} />
                    {(adds > 0 || dels > 0) && (
                        <span className="agent-edit-card__stats" style={{ display: 'flex', gap: '4px', fontSize: '10px', fontWeight: 600 }}>
                            {adds > 0 && <span className="agent-edit-card__add" style={{ color: '#3fb950' }}>+{adds}</span>}
                            {dels > 0 && <span className="agent-edit-card__del" style={{ color: '#f85149' }}>-{dels}</span>}
                        </span>
                    )}
                    <i className={`codicon codicon-chevron-${open ? 'up' : 'down'}`} style={{ fontSize: 9, opacity: 0.35, fontFamily: 'codicon', fontStyle: 'normal' }} />
                </div>
                {isRunning && <ProgressBar indeterminate />}
                {open && hasDiff && (
                    <InlineDiffPreview oldText={block.oldText} newText={block.newText} path={block.path} hideMeta />
                )}
                {open && !hasDiff && block.preview && (
                    <pre style={{
                        margin: 0, padding: '6px 10px', fontSize: '10px', lineHeight: 1.4,
                        fontFamily: 'var(--font-mono)', opacity: 0.65, maxHeight: 120, overflow: 'auto',
                        whiteSpace: 'pre-wrap', wordBreak: 'break-word',
                    }}>
                        {block.preview.slice(0, 500)}{block.preview.length > 500 ? '\n...' : ''}
                    </pre>
                )}
                {block.status === 'done' && block.path && (
                    <div className="agent-edit-footer" style={{ padding: '4px 10px 6px' }}>
                        <EditDecision path={block.path} />
                    </div>
                )}
            </div>
        );
    }

    // Terminal / general tool card
    const canExpand = block.kind === 'terminal' || block.outputLines.length > 0 || isError;

    return (
        <div style={cardStyle}>
            <div
                style={{ ...headerStyle, cursor: canExpand ? 'pointer' : 'default', borderBottom: open ? headerStyle.borderBottom : 'none' }}
                onClick={() => canExpand && setOpen((v) => !v)}
            >
                {statusDot}
                <i className={`codicon codicon-${icon}`} style={{ fontSize: 11, opacity: 0.55, fontFamily: 'codicon', fontStyle: 'normal' }} />
                <span style={{ flex: 1, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap', fontSize: '11px' }}>
                    {block.kind === 'terminal' && block.command ? block.command.slice(0, 80) : block.title}
                </span>
                {isError && <span style={{ color: '#f48771', fontSize: '9px', fontWeight: 600 }}>failed</span>}
                {isRunning && (
                    <i className="codicon codicon-loading codicon-modifier-spin" style={{
                        fontSize: 10, opacity: 0.4, fontFamily: 'codicon', fontStyle: 'normal',
                    }} />
                )}
                {canExpand && !isRunning && (
                    <i className={`codicon codicon-chevron-${open ? 'up' : 'down'}`} style={{ fontSize: 9, opacity: 0.3, fontFamily: 'codicon', fontStyle: 'normal' }} />
                )}
            </div>
            {isRunning && <ProgressBar indeterminate />}
            {open && block.command && (
                <div style={{
                    padding: '4px 10px', fontFamily: 'var(--font-mono)', fontSize: '10px',
                    color: 'rgba(255,255,255,0.5)', borderBottom: '1px solid rgba(255,255,255,0.03)',
                }}>
                    <span style={{ color: '#89d185', marginRight: 4 }}>$</span>
                    {block.command}
                </div>
            )}
            {open && block.outputLines.length > 0 && (
                <pre style={{
                    margin: 0, padding: '6px 10px', fontSize: '10px', lineHeight: 1.35,
                    fontFamily: 'var(--font-mono)', maxHeight: 200, overflow: 'auto',
                    whiteSpace: 'pre-wrap', wordBreak: 'break-word',
                    color: isError ? '#f48771' : 'rgba(255,255,255,0.6)',
                }}>
                    {block.outputLines.join('\n')}
                </pre>
            )}
            {open && block.status === 'done' && block.exitCode != null && (
                <div style={{ padding: '3px 10px', fontSize: '9px', opacity: 0.35 }}>
                    exit {block.exitCode}
                </div>
            )}
        </div>
    );
};

const RenderBlock: React.FC<{ item: DisplayToolBlock }> = ({ item }) => {
    if ('type' in item && item.type === 'recon-summary') {
        return <ReconSummaryCard count={item.count} labels={item.labels} />;
    }
    return <ToolBlockCard block={item as AgentToolBlock} />;
};

interface AgentToolBlocksProps {
    blocks: AgentToolBlock[];
    compact?: boolean;
}

const AgentToolBlocks: React.FC<AgentToolBlocksProps> = ({ blocks, compact }) => {
    const display = useMemo(() => {
        const slice = compact ? blocks.slice(-12) : blocks;
        return collapseToolBlocksForDisplay(slice);
    }, [blocks, compact]);

    if (!display.length) return null;

    return (
        <div className="agent-tool-blocks" style={{ margin: compact ? '4px 0 6px' : '4px 10px 6px' }}>
            {display.map((item) => (
                <RenderBlock key={item.id} item={item} />
            ))}
        </div>
    );
};

export default AgentToolBlocks;
