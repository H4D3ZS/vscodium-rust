/**
 * ChatMessage — Cursor-style clean message rendering.
 * Chat messages show ONLY the assistant's text (and thinking block).
 * Tool activity is rendered separately in ActivityPanel.
 */
import React from 'react';
import MessageBody from '../agent/MessageBody';
import ComposerThinkingBlock from './ComposerThinkingBlock';
import type { AgentMessage } from '../../store';
import { useStore } from '../../store';
import { cleanAgentContent } from '../../domain/agent/cleanAgentContent';
import { shouldAutoAcceptEverything } from '../../lib/agentAutonomy';

interface ChatMessageProps {
    msg: AgentMessage;
    idx: number;
    isAgentThinking: boolean;
    isLastMessage?: boolean;
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

const ChatMessage: React.FC<ChatMessageProps> = ({
    msg, idx, isAgentThinking, isLastMessage,
    onCopy, onEditStart, onRestoreCheckpoint,
    lastCopiedIdx, editingMsgIdx, editValue,
    onEditChange, onEditSave, onEditCancel,
}) => {
    const agentMode = useStore((s) => s.agentMode);
    const cleaned = cleanAgentContent(msg.content || '');
    const hasContent = !!cleaned;
    const hasThoughts = !!msg.thoughts;
    const hasContext = msg.context && msg.context.length > 0;
    // Only render if there's actual text content or thoughts — no tool blocks inline
    if (!hasContent && !hasThoughts && !hasContext) return null;

    return (
        <div className="agent-message-container" style={{ padding: '6px 10px', position: 'relative' }}>
            {/* Header: role label + actions */}
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

            {/* Message body — CLEAN: only text + thinking, no tool blocks */}
            <div style={{
                background: msg.role === 'user'
                    ? 'rgba(59,130,246,0.06)'
                    : msg.isSubAgentResponse ? 'rgba(59,130,246,0.03)' : 'transparent',
                padding: msg.role === 'user' ? '9px 12px' : '2px 0',
                borderRadius: msg.role === 'user' ? '8px' : 0,
                border: msg.role === 'user'
                    ? '1px solid rgba(59,130,246,0.15)'
                    : msg.isSubAgentResponse ? '1px solid rgba(59,130,246,0.10)' : 'none',
            }}>
                {editingMsgIdx === idx ? (
                    <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                        <textarea
                            value={editValue}
                            onChange={e => onEditChange(e.target.value)}
                            autoFocus
                            style={{
                                background: 'rgba(0,0,0,0.2)', border: '1px solid var(--vscode-focusBorder)',
                                color: 'var(--vscode-editor-foreground, #fff)', padding: '8px', borderRadius: '6px',
                                fontSize: '13px', resize: 'vertical', minHeight: '60px', outline: 'none',
                            }}
                        />
                        <div style={{ display: 'flex', gap: '8px', justifyContent: 'flex-end' }}>
                            <button onClick={onEditCancel} style={{
                                background: 'transparent', border: '1px solid rgba(255,255,255,0.2)',
                                color: 'var(--vscode-editor-foreground, #fff)', padding: '4px 8px',
                                borderRadius: '4px', fontSize: '11px', cursor: 'pointer',
                            }}>Cancel</button>
                            <button onClick={() => onEditSave(idx)} style={{
                                background: 'var(--vscode-button-background, #0e639c)', border: 'none',
                                color: 'var(--vscode-button-foreground, #fff)', padding: '4px 12px',
                                borderRadius: '2px', fontSize: '11px', cursor: 'pointer', fontWeight: 600,
                            }}>Resend</button>
                        </div>
                    </div>
                ) : (
                    <>
                        {/* Thinking block — collapsible, shown above text */}
                        {msg.thoughts && (
                            <ComposerThinkingBlock
                                thoughts={msg.thoughts}
                                durationMs={msg.thoughtDurationMs}
                                isStreaming={isAgentThinking && isLastMessage}
                            />
                        )}

                        {/* Clean text content — NO tool blocks, NO steps */}
                        {hasContent && (
                            <MessageBody
                                content={cleaned}
                                allowApply={msg.role === 'assistant' && !isAgentThinking && !shouldAutoAcceptEverything(agentMode)}
                            />
                        )}

                        {/* Attached context chips */}
                        {hasContext && (
                            <div style={{ display: 'flex', flexWrap: 'wrap', gap: '6px', marginTop: '6px', opacity: 0.8 }}>
                                {msg.context.map((item: any, i: number) => (
                                    <div key={i} style={{
                                        display: 'flex', alignItems: 'center', gap: '4px',
                                        padding: '2px 6px', background: 'rgba(255,255,255,0.06)',
                                        borderRadius: '4px', fontSize: '10px',
                                        border: '1px solid rgba(255,255,255,0.1)', color: 'rgba(255,255,255,0.7)',
                                    }}>
                                        {item.thumbnail
                                            ? <img src={item.thumbnail} style={{ width: '14px', height: '14px', borderRadius: '2px', objectFit: 'cover' }} alt="" />
                                            : <i className="codicon codicon-files" style={{ fontSize: '10px' }} />
                                        }
                                        <span>{item.name}</span>
                                    </div>
                                ))}
                            </div>
                        )}
                    </>
                )}
            </div>
        </div>
    );
};

export default ChatMessage;
