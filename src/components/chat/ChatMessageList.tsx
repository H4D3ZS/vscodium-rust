/**
 * Scrollable agent message list — used by RightSidebar chat view.
 */
import React, { useRef, useEffect } from 'react';
import ChatMessage from './ChatMessage';
import AgentToolBlocks from './AgentToolBlocks';
import ComposerThinkingBlock from './ComposerThinkingBlock';
import type { AgentMessage } from '../../store';
import { useStore } from '../../store';

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
    const liveBlocks = useStore((s) => s.agentToolBlocks);
    const lastAssistantThoughts = useStore((s) => {
        const last = s.agentMessages[s.agentMessages.length - 1];
        return last?.role === 'assistant' ? (last.thoughts || s.currentThought?.logic || '') : '';
    });

    useEffect(() => {
        endRef.current?.scrollIntoView({ behavior: 'smooth' });
    }, [messages.length, isAgentThinking, liveBlocks.length]);

    useEffect(() => {
        if (!isAgentThinking && liveBlocks.length > 0) {
            useStore.getState().finalizeAgentToolBlocks?.();
        }
    }, [isAgentThinking, liveBlocks.length]);

    if (messages.length === 0 && !isAgentThinking) return null;

    return (
        <div style={{ display: 'flex', flexDirection: 'column', padding: '6px 10px 10px', gap: '10px', flex: 1 }}>
            {messages.map((msg, idx) => (
                <ChatMessage
                    key={`${msg.timestamp ?? idx}-${msg.role}`}
                    msg={msg}
                    idx={idx}
                    isLastMessage={idx === messages.length - 1}
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
            {(isAgentThinking || liveBlocks.length > 0) && (
                <>
                    {isAgentThinking && lastAssistantThoughts && (
                        <ComposerThinkingBlock thoughts={lastAssistantThoughts} isStreaming />
                    )}
                    <AgentToolBlocks blocks={liveBlocks} />
                </>
            )}
            <div ref={endRef} />
        </div>
    );
};

export default ChatMessageList;
