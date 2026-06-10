/**
 * Cursor Composer–style tool invocation cards — terminal streaming, reads, diffs, todos.
 * Recon noise (glob/list/grep) collapses into compact summaries.
 */
import React, { useMemo, useState } from 'react';
import type { AgentToolBlock } from '../../domain/agent/agentToolBlocks';
import { collapseToolBlocksForDisplay, type DisplayToolBlock } from '../../domain/agent/agentToolBlocks';
import InlineDiffPreview from './InlineDiffPreview';
import ComposerTodoList from './ComposerTodoList';
import { useStore } from '../../store';

const cardStyle: React.CSSProperties = {
    marginBottom: '8px',
    borderRadius: '8px',
    border: '1px solid rgba(255,255,255,0.08)',
    background: 'rgba(255,255,255,0.03)',
    overflow: 'hidden',
};

const headerStyle: React.CSSProperties = {
    display: 'flex',
    alignItems: 'center',
    gap: '8px',
    padding: '6px 10px',
    fontSize: '11px',
    borderBottom: '1px solid rgba(255,255,255,0.05)',
    color: 'rgba(255,255,255,0.75)',
};

const ReconSummaryCard: React.FC<{ count: number; labels: string[] }> = ({ count, labels }) => {
    const [open, setOpen] = useState(false);
    return (
        <div style={cardStyle}>
            <div
                style={{ ...headerStyle, cursor: 'pointer', borderBottom: open ? headerStyle.borderBottom : 'none' }}
                onClick={() => setOpen((v) => !v)}
            >
                <i className="codicon codicon-search" style={{ fontSize: 12, opacity: 0.45 }} />
                <span style={{ opacity: 0.55 }}>{count} recon steps</span>
                <i className={`codicon codicon-chevron-${open ? 'up' : 'down'}`} style={{ fontSize: 10, opacity: 0.35, marginLeft: 'auto' }} />
            </div>
            {open && (
                <div style={{ padding: '6px 10px', fontSize: 10, opacity: 0.45, lineHeight: 1.5 }}>
                    {labels.map((l, i) => <div key={i}>{l}</div>)}
                    {count > labels.length && <div>…and {count - labels.length} more</div>}
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

    const icon =
        block.kind === 'terminal' ? 'terminal'
            : block.kind === 'read' ? 'file'
                : block.kind === 'edit' ? 'edit'
                    : block.kind === 'todo' ? 'tasklist'
                        : block.kind === 'search' ? 'search'
                            : 'tools';

    const statusDot = (
        <span style={{
            width: 6, height: 6, borderRadius: '50%', flexShrink: 0,
            background: isRunning ? '#3794ff' : isError ? '#f48771' : 'rgba(255,255,255,0.3)',
            animation: isRunning ? 'hubPulse 1s infinite' : undefined,
        }} />
    );

    if (block.kind === 'canvas') {
        const canOpen = block.status === 'done' && !!block.canvasId;
        return (
            <div
                style={{
                    ...headerStyle,
                    ...cardStyle,
                    borderBottom: 'none',
                    cursor: canOpen ? 'pointer' : 'default',
                    opacity: isError ? 0.6 : 1,
                }}
                onClick={() => {
                    if (canOpen && block.canvasId) {
                        useStore.getState().openCanvasTab(block.canvasId);
                    }
                }}
                title={canOpen ? 'Open canvas tab' : undefined}
            >
                {statusDot}
                <span style={{ fontSize: 12 }}>📊</span>
                <span style={{ flex: 1, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                    {isError ? `Canvas failed${block.canvasTitle ? ` · ${block.canvasTitle}` : ''}`
                        : isRunning ? `Rendering canvas${block.canvasTitle ? ` · ${block.canvasTitle}` : ''}…`
                            : `Open canvas${block.canvasTitle ? ` · ${block.canvasTitle}` : ''}`}
                </span>
                {canOpen && <i className="codicon codicon-link-external" style={{ fontSize: 10, opacity: 0.45 }} />}
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
            <div style={{ ...headerStyle, ...cardStyle, borderBottom: 'none', opacity: 0.5 }}>
                {statusDot}
                <i className={`codicon codicon-${icon}`} style={{ fontSize: 11, opacity: 0.5 }} />
                <span style={{ fontFamily: 'var(--font-mono)', fontSize: 10 }}>{block.title}</span>
                {block.lineRange && <span style={{ opacity: 0.4, marginLeft: 'auto' }}>{block.lineRange}</span>}
            </div>
        );
    }

    if (block.kind === 'edit') {
        const hasDiff = !!(block.oldText || block.newText);
        return (
            <div style={cardStyle}>
                <div
                    style={{ ...headerStyle, cursor: 'pointer', borderBottom: open ? headerStyle.borderBottom : 'none' }}
                    onClick={() => setOpen((v) => !v)}
                >
                    {statusDot}
                    <i className="codicon codicon-diff" style={{ fontSize: 12, opacity: 0.6 }} />
                    <span style={{ flex: 1 }}>{block.title}</span>
                    <i className={`codicon codicon-chevron-${open ? 'up' : 'down'}`} style={{ fontSize: 10, opacity: 0.4 }} />
                </div>
                {open && hasDiff && (
                    <InlineDiffPreview oldText={block.oldText} newText={block.newText} path={block.path} />
                )}
                {open && !hasDiff && block.preview && (
                    <pre style={{
                        margin: 0, padding: '8px 10px', fontSize: 10, lineHeight: 1.45,
                        fontFamily: 'var(--font-mono)', opacity: 0.7, maxHeight: 120, overflow: 'auto',
                        whiteSpace: 'pre-wrap', wordBreak: 'break-word',
                    }}>
                        {block.preview.slice(0, 600)}{block.preview.length > 600 ? '\n…' : ''}
                    </pre>
                )}
            </div>
        );
    }

    const canExpand = block.kind === 'terminal' || block.outputLines.length > 0 || isError;

    return (
        <div style={cardStyle}>
            <div
                style={{ ...headerStyle, cursor: canExpand ? 'pointer' : 'default', borderBottom: open ? headerStyle.borderBottom : 'none' }}
                onClick={() => canExpand && setOpen((v) => !v)}
            >
                {statusDot}
                <i className={`codicon codicon-${icon}`} style={{ fontSize: 12, opacity: 0.6 }} />
                <span style={{ flex: 1, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                    {block.kind === 'terminal' && block.command ? block.command.slice(0, 80) : block.title}
                </span>
                {isError && <span style={{ color: '#f48771', fontSize: 10 }}>failed</span>}
                {canExpand && (
                    <i className={`codicon codicon-chevron-${open ? 'up' : 'down'}`} style={{ fontSize: 10, opacity: 0.4 }} />
                )}
            </div>
            {open && block.command && (
                <div style={{
                    padding: '6px 10px', fontFamily: 'var(--font-mono)', fontSize: 10,
                    color: 'rgba(255,255,255,0.55)', borderBottom: '1px solid rgba(255,255,255,0.04)',
                }}>
                    <span style={{ color: '#89d185', marginRight: 6 }}>$</span>
                    {block.command}
                </div>
            )}
            {open && block.outputLines.length > 0 && (
                <pre style={{
                    margin: 0, padding: '8px 10px', fontSize: 10, lineHeight: 1.4,
                    fontFamily: 'var(--font-mono)', maxHeight: 200, overflow: 'auto',
                    whiteSpace: 'pre-wrap', wordBreak: 'break-word',
                    color: isError ? '#f48771' : 'rgba(255,255,255,0.65)',
                }}>
                    {block.outputLines.join('\n')}
                </pre>
            )}
            {open && block.status === 'done' && block.exitCode != null && (
                <div style={{ padding: '4px 10px', fontSize: 10, opacity: 0.4 }}>exit {block.exitCode}</div>
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
        <div className="agent-tool-blocks" style={{ margin: compact ? '4px 0 8px' : '4px 10px 8px' }}>
            {display.map((item) => (
                <RenderBlock key={'type' in item ? item.id : item.id} item={item} />
            ))}
        </div>
    );
};

export default AgentToolBlocks;
