/**
 * Cursor-style Activity Panel — shows ALL tool calls, their status, and output
 * in a separate panel below the chat. Chat messages stay clean (text only).
 *
 * This replaces the old inline tool blocks in ChatMessage.
 */
import React, { useState, useMemo } from 'react';
import { useStore } from '../../store';
import { formatCursorActivityLine, getToolLabel } from '../../domain/agent/cleanAgentContent';
import type { AgentToolBlock } from '../../domain/agent/agentToolBlocks';
import { collapseToolBlocksForDisplay, type DisplayToolBlock } from '../../domain/agent/agentToolBlocks';
import InlineDiffPreview from './InlineDiffPreview';

interface ActivityPanelProps {
    isAgentThinking: boolean;
}

const ActivityPanel: React.FC<ActivityPanelProps> = ({ isAgentThinking }) => {
    const liveBlocks = useStore((s) => s.agentToolBlocks);
    const trajectory = useStore((s) => s.agentTrajectory);
    const currentAction = useStore((s) => s.agentCurrentAction);
    const agentMessages = useStore((s) => s.agentMessages);
    const [expanded, setExpanded] = useState(true);
    const [expandedBlocks, setExpandedBlocks] = useState<Set<string>>(new Set());

    // Collect all finalized tool blocks from messages
    const historicalBlocks = useMemo(() => {
        const blocks: AgentToolBlock[] = [];
        for (const msg of agentMessages) {
            if (msg.toolBlocks) {
                blocks.push(...msg.toolBlocks);
            }
        }
        return blocks;
    }, [agentMessages]);

    // Merge live + historical, most recent first
    const allBlocks = useMemo(() => {
        const combined = [...liveBlocks, ...historicalBlocks];
        return combined.slice(-50); // Keep last 50
    }, [liveBlocks, historicalBlocks]);

    const displayItems = useMemo(() => collapseToolBlocksForDisplay(allBlocks), [allBlocks]);

    const runningCount = liveBlocks.filter(b => b.status === 'running').length;
    const totalCount = displayItems.length;

    if (totalCount === 0 && !isAgentThinking) return null;

    const toggleBlock = (id: string) => {
        setExpandedBlocks(prev => {
            const next = new Set(prev);
            if (next.has(id)) next.delete(id);
            else next.add(id);
            return next;
        });
    };

    return (
        <div style={{
            margin: '0 10px 6px',
            borderRadius: '8px',
            border: '1px solid rgba(255,255,255,0.06)',
            background: 'rgba(255,255,255,0.02)',
            overflow: 'hidden',
        }}>
            {/* Header — clickable to collapse */}
            <div
                onClick={() => setExpanded(v => !v)}
                style={{
                    display: 'flex', alignItems: 'center', gap: '6px',
                    padding: '5px 10px', cursor: 'pointer',
                    borderBottom: expanded ? '1px solid rgba(255,255,255,0.04)' : 'none',
                    fontSize: '10px', fontWeight: 600, color: 'rgba(255,255,255,0.5)',
                }}
            >
                {isAgentThinking && runningCount > 0 ? (
                    <span style={{
                        width: 5, height: 5, borderRadius: '50%',
                        background: '#3794ff', animation: 'hubPulse 1s infinite',
                    }} />
                ) : (
                    <i className="codicon codicon-pulse" style={{
                        fontFamily: 'codicon', fontStyle: 'normal', fontSize: '11px', opacity: 0.5,
                    }} />
                )}
                <span>Activity</span>
                {runningCount > 0 && (
                    <span style={{
                        fontSize: '9px', padding: '0 4px', borderRadius: '6px',
                        background: 'rgba(55,148,255,0.12)', color: '#3794ff',
                    }}>
                        {runningCount} running
                    </span>
                )}
                <span style={{ flex: 1 }} />
                <span style={{ fontSize: '9px', opacity: 0.4 }}>{totalCount} steps</span>
                <i className={`codicon codicon-chevron-${expanded ? 'up' : 'down'}`} style={{
                    fontFamily: 'codicon', fontStyle: 'normal', fontSize: '9px', opacity: 0.3,
                }} />
            </div>

            {/* Activity lines — compact, one per tool call */}
            {expanded && (
                <div style={{ maxHeight: '200px', overflowY: 'auto' }}>
                    {/* Live running action */}
                    {isAgentThinking && currentAction && (
                        <div style={{
                            display: 'flex', alignItems: 'center', gap: '6px',
                            padding: '4px 10px', fontSize: '10px',
                            color: 'rgba(255,255,255,0.7)',
                        }}>
                            <span style={{
                                width: 4, height: 4, borderRadius: '50%',
                                background: '#3794ff', animation: 'hubPulse 1s infinite',
                            }} />
                            <span>{typeof currentAction === 'string' ? currentAction : ''}</span>
                        </div>
                    )}

                    {/* Finalized blocks — each is a compact one-liner */}
                    {displayItems.map((item) => {
                        if ('type' in item && item.type === 'recon-summary') {
                            return (
                                <div key={item.id} style={{
                                    padding: '3px 10px', fontSize: '10px',
                                    color: 'rgba(255,255,255,0.35)',
                                }}>
                                    {item.count} recon steps (collapsed)
                                </div>
                            );
                        }

                        const block = item as AgentToolBlock;
                        const isExpanded = expandedBlocks.has(block.id);
                        const isRunning = block.status === 'running';
                        const isError = block.status === 'error';

                        const statusColor = isRunning ? '#3794ff' : isError ? '#f48771' : 'rgba(255,255,255,0.2)';

                        // Build one-line summary
                        const oneLine = buildOneLineSummary(block);

                        return (
                            <div key={block.id}>
                                <div
                                    onClick={() => {
                                        if (block.outputLines.length > 0 || block.kind === 'terminal' || block.kind === 'edit') {
                                            toggleBlock(block.id);
                                        }
                                    }}
                                    style={{
                                        display: 'flex', alignItems: 'center', gap: '6px',
                                        padding: '3px 10px', fontSize: '10px',
                                        cursor: block.outputLines.length > 0 ? 'pointer' : 'default',
                                        color: isError ? '#f48771' : isRunning ? 'rgba(255,255,255,0.7)' : 'rgba(255,255,255,0.4)',
                                    }}
                                >
                                    <span style={{
                                        width: 4, height: 4, borderRadius: '50%', flexShrink: 0,
                                        background: statusColor,
                                        animation: isRunning ? 'hubPulse 1s infinite' : undefined,
                                    }} />
                                    <span style={{
                                        flex: 1, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap',
                                        fontFamily: block.kind === 'terminal' ? 'var(--font-mono)' : undefined,
                                    }}>
                                        {oneLine}
                                    </span>
                                    {isRunning && (
                                        <i className="codicon codicon-loading codicon-modifier-spin" style={{
                                            fontFamily: 'codicon', fontStyle: 'normal', fontSize: '9px', opacity: 0.4,
                                        }} />
                                    )}
                                    {block.kind === 'edit' && block.path && (
                                        <span style={{
                                            fontSize: '9px', opacity: 0.3,
                                            fontFamily: 'var(--font-mono)',
                                        }}>
                                            {block.path.split(/[\\/]/).pop()}
                                        </span>
                                    )}
                                    {(block.outputLines.length > 0 || block.kind === 'terminal' || block.kind === 'edit') && (
                                        <i className={`codicon codicon-chevron-${isExpanded ? 'up' : 'down'}`} style={{
                                            fontFamily: 'codicon', fontStyle: 'normal', fontSize: '8px', opacity: 0.25,
                                        }} />
                                    )}
                                </div>

                                {/* Expanded output */}
                                {isExpanded && block.outputLines.length > 0 && (
                                    <pre style={{
                                        margin: 0, padding: '4px 10px 4px 20px',
                                        fontSize: '9px', lineHeight: 1.3,
                                        fontFamily: 'var(--font-mono)',
                                        color: isError ? '#f48771' : 'rgba(255,255,255,0.45)',
                                        maxHeight: '100px', overflow: 'auto',
                                        whiteSpace: 'pre-wrap', wordBreak: 'break-word',
                                    }}>
                                        {block.outputLines.join('\n')}
                                    </pre>
                                )}

                                {/* Edit diff */}
                                {isExpanded && block.kind === 'edit' && block.oldText && block.newText && (
                                    <div style={{ padding: '4px 10px 4px 20px', maxHeight: '150px', overflow: 'auto' }}>
                                        <InlineDiffPreview oldText={block.oldText} newText={block.newText} path={block.path} hideMeta />
                                    </div>
                                )}
                            </div>
                        );
                    })}
                </div>
            )}
        </div>
    );
};

/** Build a compact one-line summary for an activity entry */
function buildOneLineSummary(block: AgentToolBlock): string {
    const label = getToolLabel(block.tool || block.kind);

    switch (block.kind) {
        case 'terminal':
            return block.command
                ? `$ ${block.command.length > 60 ? block.command.slice(0, 60) + '...' : block.command}`
                : label;
        case 'edit':
            if (block.path) {
                const file = block.path.split(/[\\/]/).pop() || block.path;
                return `${label} ${file}`;
            }
            return label;
        case 'read':
            return block.title || label;
        case 'search':
            return block.title || label;
        case 'todo':
            return block.title || 'Task list';
        case 'canvas':
            return block.canvasTitle || 'Canvas';
        default:
            return block.title || label;
    }
}

export default ActivityPanel;
