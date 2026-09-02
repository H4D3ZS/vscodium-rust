/**
 * ChatMessage — Cursor-style clean message rendering.
 * Chat messages show ONLY the assistant's text (and thinking block).
 * Tool activity is rendered separately in ActivityPanel.
 */
import React, { lazy, Suspense } from 'react';
import MessageBody from '../agent/MessageBody';
import ComposerThinkingBlock from './ComposerThinkingBlock';
import type { AgentMessage } from '../../store';
import { useStore } from '../../store';
import { cleanAgentContent } from '../../domain/agent/cleanAgentContent';
import { shouldAutoAcceptEverything } from '../../lib/agentAutonomy';
import { messageHasPlanGate } from './PlanProceedCard';
import { messageIsCompletion } from './WalkthroughCard';
const PlanProceedCard = lazy(() => import('./PlanProceedCard'));
const WalkthroughCard = lazy(() => import('./WalkthroughCard'));

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

    const isStreamingHere = !!isAgentThinking && !!isLastMessage && msg.role === 'assistant';

    return (
        <div className="agent-message-container ac-msg">
            {/* Header: role label + actions */}
            <div className="ac-msg__head">
                <span className="ac-msg__role">
                    {msg.role === 'user'
                        ? 'You'
                        : msg.isSubAgentResponse
                            ? 'Partner'
                            : 'Agent'}
                </span>
                <div className="ac-msg__actions">
                    {msg.role === 'user' && (
                        <button className="ac-icon-btn" title="Edit" onClick={() => onEditStart(idx, msg.content)}>
                            <i className="codicon codicon-edit" />
                        </button>
                    )}
                    {msg.role === 'user' && onRestoreCheckpoint && msg.checkpointId && !isAgentThinking && (
                        <button className="ac-icon-btn" title="Restore workspace to this turn" onClick={() => onRestoreCheckpoint(msg)}>
                            <i className="codicon codicon-discard" style={{ color: 'var(--ac-amber)' }} />
                        </button>
                    )}
                    {msg.role === 'assistant' && (
                        <button className="ac-icon-btn" title="Copy" onClick={() => onCopy(msg.content, idx)}>
                            <i className={`codicon codicon-${lastCopiedIdx === idx ? 'check' : 'copy'}`}
                               style={lastCopiedIdx === idx ? { color: 'var(--ac-green)' } : undefined} />
                        </button>
                    )}
                </div>
            </div>

            {/* Message body — CLEAN: only text + thinking, no tool blocks */}
            <div className={`ac-msg__body${msg.role === 'user' ? ' ac-user-bubble' : ''}`}>
                {editingMsgIdx === idx ? (
                    <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                        <textarea
                            value={editValue}
                            onChange={e => onEditChange(e.target.value)}
                            autoFocus
                            style={{
                                background: 'var(--ac-bg)', border: '1px solid var(--ac-accent-border)',
                                color: 'var(--ac-fg)', padding: '8px', borderRadius: 'var(--ac-radius-sm)',
                                fontSize: '13px', resize: 'vertical', minHeight: '60px', outline: 'none',
                            }}
                        />
                        <div style={{ display: 'flex', gap: '8px', justifyContent: 'flex-end' }}>
                            <button className="ac-btn ac-btn--ghost" onClick={onEditCancel}>Cancel</button>
                            <button className="ac-btn ac-btn--primary" onClick={() => onEditSave(idx)}>Resend</button>
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
                            <div className={isStreamingHere ? 'ac-caret' : undefined}>
                                <MessageBody
                                    content={cleaned}
                                    allowApply={msg.role === 'assistant' && !isAgentThinking && !shouldAutoAcceptEverything(agentMode)}
                                />
                            </div>
                        )}

                        {/* Antigravity-style flow cards (assistant only). The plan card
                            reads the RAW content (gate tokens are stripped from `cleaned`). */}
                        {msg.role === 'assistant' && !isAgentThinking && messageHasPlanGate(msg.content || '') && (
                            <PlanProceedCard content={msg.content || ''} />
                        )}
                        {msg.role === 'assistant' && !isAgentThinking && isLastMessage && messageIsCompletion(msg.content || '') && (
                            <WalkthroughCard />
                        )}

                        {/* Attached context chips */}
                        {hasContext && (
                            <div style={{ display: 'flex', flexWrap: 'wrap', gap: '6px', marginTop: '6px' }}>
                                {msg.context.map((item: any, i: number) => (
                                    <div key={i} className="ac-context-chip">
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
