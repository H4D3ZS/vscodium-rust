/**
 * Scrollable agent message list — used by RightSidebar chat view.
 */
import React, { useRef, useEffect } from 'react';
import ChatMessage from './ChatMessage';
import type { AgentMessage } from '../../store';

interface ChatMessageListProps {
    messages: AgentMessage[];
    isAgentThinking: boolean;
    lastCopiedIdx: number | null;
    editingMsgIdx: number | null;
    editValue: string;
    onCopy: (content: string, idx: number) => void;
    onEditStart: (idx: number, content: string) => void;
    onEditChange: (v: string) => void;
    onEditSave: (idx: number) => void;
    onEditCancel: () => void;
    onRestoreCheckpoint?: (msg: AgentMessage) => void;
}

const ChatMessageList: React.FC<ChatMessageListProps> = ({
    messages,
    isAgentThinking,
    lastCopiedIdx,
    editingMsgIdx,
    editValue,
    onCopy,
    onEditStart,
    onEditChange,
    onEditSave,
    onEditCancel,
    onRestoreCheckpoint,
}) => {
    const endRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        endRef.current?.scrollIntoView({ behavior: 'smooth' });
    }, [messages.length, isAgentThinking]);

    if (messages.length === 0) return null;

    return (
        <div style={{ display: 'flex', flexDirection: 'column', padding: '6px 10px 10px', gap: '10px', flex: 1 }}>
            {messages.map((msg, idx) => (
                <ChatMessage
                    key={`${msg.timestamp ?? idx}-${msg.role}`}
                    msg={msg}
                    idx={idx}
                    isAgentThinking={isAgentThinking}
                    onCopy={onCopy}
                    onEditStart={onEditStart}
                    onRestoreCheckpoint={onRestoreCheckpoint}
                    lastCopiedIdx={lastCopiedIdx}
                    editingMsgIdx={editingMsgIdx}
                    editValue={editValue}
                    onEditChange={onEditChange}
                    onEditSave={onEditSave}
                    onEditCancel={onEditCancel}
                />
            ))}
            <div ref={endRef} />
        </div>
    );
};

export default ChatMessageList;
