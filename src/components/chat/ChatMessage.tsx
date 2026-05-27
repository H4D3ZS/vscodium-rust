import React, { useState } from 'react';
import MessageBody from '../agent/MessageBody';
import type { AgentMessage } from '../../store';

function cleanAiContent(raw: string): string {
    if (!raw) return '';
    let s = raw;
    s = s.replace(/<tool_call>[\s\S]*?<\/tool_call>/g, '');
    s = s.replace(/<function_calls>[\s\S]*?<\/function_calls>/g, '');
    s = s.replace(/<function>[\s\S]*?<\/function>/g, '');
    s = s.replace(/<invoke>[\s\S]*?<\/invoke>/g, '');
    s = s.replace(/```[a-z]*\n([\s\S]*?)```/gi, (match, inner) => {
        try {
            const t = inner.trim();
            if ((t.startsWith('{') || t.startsWith('[')) && JSON.parse(t) && typeof JSON.parse(t) === 'object') {
                const p = JSON.parse(t);
                if ('name' in p && ('arguments' in p || 'input' in p)) return '';
            }
        } catch { /* not JSON */ }
        return match;
    });
    s = s.replace(/MISSION_ACCOMPLISHED|TASK_COMPLETE/g, '');
    s = s.replace(/\n{3,}/g, '\n\n').trim();
    return s;
}

interface ChatMessageProps {
    msg: AgentMessage;
    idx: number;
    isAgentThinking: boolean;
    onCopy: (content: string, idx: number) => void;
    onEditStart: (idx: number, content: string) => void;
    onRestore?: (timestamp: number) => void;
    lastCopiedIdx: number | null;
    editingMsgIdx: number | null;
    editValue: string;
    onEditChange: (v: string) => void;
    onEditSave: (idx: number) => void;
    onEditCancel: () => void;
}

const ChatMessage: React.FC<ChatMessageProps> = ({
    msg, idx, isAgentThinking,
    onCopy, onEditStart, onRestore,
    lastCopiedIdx, editingMsgIdx, editValue,
    onEditChange, onEditSave, onEditCancel,
}) => {
    const cleaned = cleanAiContent(msg.content || '');
    if (!cleaned && msg.role === 'assistant') return null;

    return (
        <div className="agent-message-container" style={{ padding: '6px 10px', position: 'relative' }}>
            <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: '4px' }}>
                <span style={{ fontSize: '10px', fontWeight: 700, opacity: 0.5, textTransform: 'uppercase', letterSpacing: '0.05em' }}>
                    {msg.role === 'user' ? 'You' : 'AIRI'}
                </span>
                <div className="message-actions" style={{ display: 'flex', gap: '6px', opacity: 0, transition: 'opacity 0.15s' }}>
                    {msg.role === 'user' && (
                        <i
                            className="codicon codicon-edit"
                            style={{ cursor: 'pointer', fontSize: '11px' }}
                            onClick={() => onEditStart(idx, msg.content)}
                        />
                    )}
                    {msg.role === 'user' && onRestore && msg.timestamp && (
                        <i
                            className="codicon codicon-discard"
                            title="Restore to this point"
                            style={{ cursor: 'pointer', fontSize: '11px' }}
                            onClick={() => onRestore(msg.timestamp!)}
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
                            style={{ background: 'rgba(0,0,0,0.2)', border: '1px solid var(--vscode-focusBorder)', color: '#fff', padding: '8px', borderRadius: '6px', fontSize: '13px', resize: 'vertical', minHeight: '60px', outline: 'none' }}
                        />
                        <div style={{ display: 'flex', gap: '8px', justifyContent: 'flex-end' }}>
                            <button onClick={onEditCancel} style={{ background: 'transparent', border: '1px solid rgba(255,255,255,0.2)', color: '#fff', padding: '4px 8px', borderRadius: '4px', fontSize: '11px', cursor: 'pointer' }}>Cancel</button>
                            <button onClick={() => onEditSave(idx)} style={{ background: '#3b82f6', border: 'none', color: '#fff', padding: '4px 12px', borderRadius: '4px', fontSize: '11px', cursor: 'pointer', fontWeight: 600 }}>Resend</button>
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
