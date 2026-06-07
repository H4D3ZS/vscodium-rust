/**
 * Scrollable agent message list — used by RightSidebar chat view.
 * Auto-scrolls only when the user is already near the bottom (Cursor-style).
 */
import React, { useRef, useEffect, useCallback, useState } from 'react';
import ChatMessage from './ChatMessage';
import AgentToolBlocks from './AgentToolBlocks';
import ComposerThinkingBlock from './ComposerThinkingBlock';
import type { AgentMessage } from '../../store';
import { useStore } from '../../store';

const NEAR_BOTTOM_PX = 120;

function findScrollParent(el: HTMLElement | null): HTMLElement | null {
    let node: HTMLElement | null = el?.parentElement ?? null;
    while (node && node !== document.body) {
        const style = window.getComputedStyle(node);
        const scrollable =
            /(auto|scroll)/.test(style.overflowY)
            || node.classList.contains('right-sidebar-scroll');
        if (scrollable && node.scrollHeight > node.clientHeight + 1) return node;
        node = node.parentElement;
    }
    return null;
}

function isNearBottom(el: HTMLElement): boolean {
    return el.scrollHeight - el.scrollTop - el.clientHeight < NEAR_BOTTOM_PX;
}

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
    scrollContainerRef?: React.RefObject<HTMLElement | null>;
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
    scrollContainerRef,
}) => {
    const endRef = useRef<HTMLDivElement>(null);
    const stickToBottomRef = useRef(true);
    const prevMsgCountRef = useRef(messages.length);
    const [showJumpToBottom, setShowJumpToBottom] = useState(false);
    const liveBlocks = useStore((s) => s.agentToolBlocks);
    const lastAssistantThoughts = useStore((s) => {
        const last = s.agentMessages[s.agentMessages.length - 1];
        return last?.role === 'assistant' ? (last.thoughts || s.currentThought?.logic || '') : '';
    });
    const streamTick = useStore((s) => {
        const last = s.agentMessages[s.agentMessages.length - 1];
        const blocks = s.agentToolBlocks?.length ?? 0;
        if (last?.role !== 'assistant') return `0:0:${blocks}`;
        return `${last.content?.length ?? 0}:${last.thoughts?.length ?? 0}:${blocks}`;
    });

    const scrollToBottom = useCallback((behavior: ScrollBehavior = 'auto') => {
        const container = scrollContainerRef?.current ?? findScrollParent(endRef.current);
        if (container) {
            if (behavior === 'smooth') {
                container.scrollTo({ top: container.scrollHeight, behavior: 'smooth' });
            } else {
                container.scrollTop = container.scrollHeight;
            }
            return;
        }
        endRef.current?.scrollIntoView({ behavior });
    }, [scrollContainerRef]);

    useEffect(() => {
        const container = scrollContainerRef?.current ?? findScrollParent(endRef.current);
        if (!container) return;

        const onScroll = () => {
            const pinned = isNearBottom(container);
            stickToBottomRef.current = pinned;
            setShowJumpToBottom(!pinned && (isAgentThinking || container.scrollHeight > container.clientHeight + 40));
        };

        onScroll();
        container.addEventListener('scroll', onScroll, { passive: true });
        return () => container.removeEventListener('scroll', onScroll);
    }, [scrollContainerRef, isAgentThinking, messages.length]);

    useEffect(() => {
        const grew = messages.length > prevMsgCountRef.current;
        const last = messages[messages.length - 1];
        if (grew && last?.role === 'user') {
            stickToBottomRef.current = true;
            setShowJumpToBottom(false);
        }
        prevMsgCountRef.current = messages.length;

        if (stickToBottomRef.current) {
            scrollToBottom('auto');
        }
    }, [messages, isAgentThinking, liveBlocks.length, streamTick, lastAssistantThoughts, scrollToBottom]);

    useEffect(() => {
        if (!isAgentThinking && liveBlocks.length > 0) {
            useStore.getState().finalizeAgentToolBlocks?.();
        }
    }, [isAgentThinking, liveBlocks.length]);

    if (messages.length === 0 && !isAgentThinking) return null;

    return (
        <div style={{ display: 'flex', flexDirection: 'column', padding: '6px 10px 10px', gap: '10px', flex: 1, position: 'relative' }}>
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
            {showJumpToBottom && (
                <button
                    type="button"
                    onClick={() => {
                        stickToBottomRef.current = true;
                        setShowJumpToBottom(false);
                        scrollToBottom('smooth');
                    }}
                    style={{
                        position: 'sticky',
                        bottom: 8,
                        alignSelf: 'center',
                        zIndex: 5,
                        padding: '6px 12px',
                        borderRadius: 999,
                        border: '1px solid rgba(255,255,255,0.12)',
                        background: 'rgba(30,30,30,0.92)',
                        color: 'var(--vscode-foreground)',
                        fontSize: 11,
                        cursor: 'pointer',
                        boxShadow: '0 4px 12px rgba(0,0,0,0.35)',
                    }}
                >
                    ↓ Jump to latest
                </button>
            )}
            <div ref={endRef} />
        </div>
    );
};

export default ChatMessageList;
