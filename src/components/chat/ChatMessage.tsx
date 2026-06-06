import React, { useState } from 'react';
import MessageBody from '../agent/MessageBody';
import type { AgentMessage } from '../../store';
import { useStore } from '../../store';
import { cleanAgentContent, getToolLabel } from '../../domain/agent/cleanAgentContent';

interface ChatMessageProps {
    msg: AgentMessage;
    idx: number;
    isAgentThinking: boolean;
    onCopy: (content: string, idx: number) => void;
    onEditStart: (idx: number, content: string) => void;
    onRestoreCheckpoint?: (msg: AgentMessage) => void;
    lastCopiedIdx: number | null;
    editingMsgIdx: number | null;
    editValue: string;
    onEditChange: (v: string) => void;
    onEditSave: (idx: number) => void;
    onEditCancel: () => void;
}

const AgentToolSteps: React.FC<{ steps: NonNullable<AgentMessage['steps']>; cleanUi: boolean }> = ({ steps, cleanUi }) => {
    const [expanded, setExpanded] = useState(false);
    if (!steps.length) return null;

    const running = steps.filter((s) => s.status === 'running').length;
    const failed = steps.filter((s) => s.status === 'error').length;
    const done = steps.filter((s) => s.status === 'success').length;

    if (cleanUi) {
        const label = running > 0
            ? `Working… (${running} active)`
            : failed > 0
                ? `Used ${steps.length} tools · ${failed} failed`
                : `Used ${steps.length} tool${steps.length === 1 ? '' : 's'}`;

        return (
            <details
                open={expanded}
                onToggle={(e) => setExpanded((e.target as HTMLDetailsElement).open)}
                style={{ marginBottom: '8px' }}
            >
                <summary style={{
                    fontSize: '11px',
                    cursor: 'pointer',
                    opacity: 0.55,
                    listStyle: 'none',
                    display: 'flex',
                    alignItems: 'center',
                    gap: '6px',
                    userSelect: 'none',
                }}>
                    <i className="codicon codicon-tools" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '11px' }} />
                    <span>{label}</span>
                    {running > 0 && (
                        <span style={{ width: '5px', height: '5px', borderRadius: '50%', background: '#3794ff', animation: 'hubPulse 1s infinite' }} />
                    )}
                </summary>
                <div style={{ marginTop: '6px', paddingLeft: '4px', display: 'flex', flexDirection: 'column', gap: '4px' }}>
                    {steps.map((step, sIdx) => {
                        const statusColor = step.status === 'running' ? '#3794ff' : step.status === 'success' ? '#89d185' : step.status === 'error' ? '#f48771' : '#555';
                        const text = step.summary || getToolLabel(step.name);
                        return (
                            <div key={sIdx} style={{ display: 'flex', alignItems: 'flex-start', gap: '6px', fontSize: '10px', opacity: 0.75 }}>
                                <span style={{ color: statusColor, fontSize: '8px', marginTop: '3px' }}>●</span>
                                <span>{text}</span>
                            </div>
                        );
                    })}
                </div>
            </details>
        );
    }

    return (
        <div style={{ display: 'flex', flexDirection: 'column', gap: '3px', marginBottom: '8px' }}>
            {steps.map((step, sIdx) => {
                const statusColor = step.status === 'running' ? '#3794ff' : step.status === 'success' ? '#89d185' : step.status === 'error' ? '#f48771' : '#555';
                return (
                    <div key={sIdx} style={{ display: 'flex', alignItems: 'center', gap: '6px', fontSize: '10px', opacity: 0.7 }}>
                        <span style={{ color: statusColor, fontSize: '8px' }}>●</span>
                        <span style={{ fontFamily: 'var(--font-mono)' }}>{step.summary || step.name}</span>
                    </div>
                );
            })}
        </div>
    );
};

const ChatMessage: React.FC<ChatMessageProps> = ({
    msg, idx, isAgentThinking,
    onCopy, onEditStart, onRestoreCheckpoint,
    lastCopiedIdx, editingMsgIdx, editValue,
    onEditChange, onEditSave, onEditCancel,
}) => {
    const agentCleanUi = useStore((s) => s.agentCleanUi);
    const cleaned = cleanAgentContent(msg.content || '');
    const hasContent = !!cleaned;
    const hasThoughts = !!msg.thoughts;
    const hasSteps = msg.steps && msg.steps.length > 0;
    const hasContext = msg.context && msg.context.length > 0;
    const shouldRender = hasContent || hasThoughts || hasSteps || hasContext;
    if (!shouldRender) return null;

    return (
        <div className="agent-message-container" style={{ padding: '6px 10px', position: 'relative' }}>
            <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: '4px' }}>
                <span style={{ fontSize: '10px', fontWeight: 700, opacity: 0.5, textTransform: 'uppercase', letterSpacing: '0.05em' }}>
                    {msg.role === 'user'
                        ? 'You'
                        : msg.isSubAgentResponse
                            ? 'Partner'
                            : 'Agent'}
                </span>
                <div className="message-actions" style={{ display: 'flex', gap: '6px', opacity: 0, transition: 'opacity 0.15s' }}>
                    {msg.role === 'user' && (
                        <i
                            className="codicon codicon-edit"
                            style={{ cursor: 'pointer', fontSize: '11px' }}
                            onClick={() => onEditStart(idx, msg.content)}
                        />
                    )}
                    {msg.role === 'user' && onRestoreCheckpoint && msg.checkpointId && !isAgentThinking && (
                        <i
                            className="codicon codicon-discard"
                            title="Restore workspace to this turn"
                            style={{ cursor: 'pointer', fontSize: '11px', color: '#e0af68' }}
                            onClick={() => onRestoreCheckpoint(msg)}
                        />
                    )}
                    {msg.role === 'assistant' && (
                        <i
                            className={`codicon codicon-${lastCopiedIdx === idx ? 'check' : 'copy'}`}
                            style={{ cursor: 'pointer', fontSize: '11px', color: lastCopiedIdx === idx ? '#10b981' : 'inherit' }}
                            onClick={() => onCopy(msg.content, idx)}
                        />
                    )}
                </div>
            </div>

            <div style={{
                background: msg.role === 'user'
                    ? 'rgba(59,130,246,0.06)'
                    : msg.isSubAgentResponse ? 'rgba(59,130,246,0.03)' : 'rgba(255,255,255,0.01)',
                padding: '9px 12px',
                borderRadius: '8px',
                border: msg.role === 'user'
                    ? '1px solid rgba(59,130,246,0.15)'
                    : '1px solid rgba(255,255,255,0.04)',
            }}>
                {editingMsgIdx === idx ? (
                    <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                        <textarea
                            value={editValue}
                            onChange={e => onEditChange(e.target.value)}
                            autoFocus
                            style={{ background: 'rgba(0,0,0,0.2)', border: '1px solid var(--vscode-focusBorder)', color: 'var(--vscode-editor-foreground, #fff)', padding: '8px', borderRadius: '6px', fontSize: '13px', resize: 'vertical', minHeight: '60px', outline: 'none' }}
                        />
                        <div style={{ display: 'flex', gap: '8px', justifyContent: 'flex-end' }}>
                            <button onClick={onEditCancel} style={{ background: 'transparent', border: '1px solid rgba(255,255,255,0.2)', color: 'var(--vscode-editor-foreground, #fff)', padding: '4px 8px', borderRadius: '4px', fontSize: '11px', cursor: 'pointer' }}>Cancel</button>
                            <button onClick={() => onEditSave(idx)} style={{ background: 'var(--vscode-button-background, #0e639c)', border: 'none', color: 'var(--vscode-button-foreground, #fff)', padding: '4px 12px', borderRadius: '2px', fontSize: '11px', cursor: 'pointer', fontWeight: 600 }}>Resend</button>
                        </div>
                    </div>
                ) : (
                    <>
                        {msg.thoughts && (
                            <details style={{ marginBottom: '8px', opacity: 0.6 }}>
                                <summary style={{ fontSize: '10px', cursor: 'pointer', fontWeight: 600 }}>Cognitive trace...</summary>
                                <div style={{ fontSize: '10px', padding: '8px', background: 'rgba(0,0,0,0.2)', borderRadius: '6px', marginTop: '4px', fontFamily: 'var(--font-mono)' }}>{msg.thoughts}</div>
                            </details>
                        )}
                        {hasSteps && (
                            <AgentToolSteps steps={msg.steps!} cleanUi={agentCleanUi && msg.role === 'assistant'} />
                        )}
                        {msg.context && msg.context.length > 0 && (
                            <div style={{ display: 'flex', flexWrap: 'wrap', gap: '6px', marginBottom: '8px', opacity: 0.8 }}>
                                {msg.context.map((item: any, i: number) => (
                                    <div key={i} style={{ display: 'flex', alignItems: 'center', gap: '4px', padding: '2px 6px', background: 'rgba(255,255,255,0.06)', borderRadius: '4px', fontSize: '10px', border: '1px solid rgba(255,255,255,0.1)', color: 'rgba(255,255,255,0.7)' }}>
                                        {item.thumbnail
                                            ? <img src={item.thumbnail} style={{ width: '14px', height: '14px', borderRadius: '2px', objectFit: 'cover' }} alt="" />
                                            : <i className="codicon codicon-files" style={{ fontSize: '10px' }} />
                                        }
                                        <span>{item.name}</span>
                                    </div>
                                ))}
                            </div>
                        )}
                        <MessageBody content={cleaned} allowApply={msg.role === 'assistant' && !isAgentThinking} />
                    </>
                )}
            </div>
        </div>
    );
};

export default ChatMessage;
