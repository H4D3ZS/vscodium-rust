/**
 * ChatPanel — composition root for the agent chat surface.
 * Manages message list + input area; delegates to ChatMessage / ChatInput / ChatToolbar.
 * State stays in the calling component (RightSidebar); this handles pure layout.
 */
import React, { useRef, useEffect } from 'react';
import ChatMessage from './ChatMessage';
import ChatInput from './ChatInput';
import ChatToolbar from './ChatToolbar';
import type { AgentMessage, AttachedContext } from '../../store';

interface ChatPanelProps {
    messages: AgentMessage[];
    isAgentThinking: boolean;
    isSpecModeActive: boolean;
    attachedFiles: AttachedContext[];
    isAttaching: boolean;
    inputRef: React.RefObject<HTMLTextAreaElement>;
    inputValue: string;
    editingMsgIdx: number | null;
    editValue: string;
    lastCopiedIdx: number | null;
    /* toolbar passthrough */
    mode: string;
    model: string;
    modeStyle: any;
    modelLabel: string;
    ttsEnabled: boolean;
    ttsPreset: string;
    isVoiceListening: boolean;
    reasoningToggle?: React.ReactNode;
    webUiControls?: React.ReactNode;
    /* handlers */
    onInputChange: (e: React.ChangeEvent<HTMLTextAreaElement>) => void;
    onKeyDown: (e: React.KeyboardEvent<HTMLTextAreaElement>) => void;
    onPaste: (e: React.ClipboardEvent<HTMLTextAreaElement>) => void;
    onSend: () => void;
    onRemoveFile: (path: string) => void;
    onCopy: (content: string, idx: number) => void;
    onEditStart: (idx: number, content: string) => void;
    onEditChange: (v: string) => void;
    onEditSave: (idx: number) => void;
    onEditCancel: () => void;
    onRestore?: (timestamp: number) => void;
    onModeClick: (e: React.MouseEvent) => void;
    onModelClick: (e: React.MouseEvent) => void;
    onAttach: () => void;
    onToggleTts: () => void;
    onTtsPresetChange: (p: string) => void;
    onToggleVoice: () => void;
}

const ChatPanel: React.FC<ChatPanelProps> = (props) => {
    const messagesEndRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
    }, [props.messages.length]);

    return (
        <div style={{ display: 'flex', flexDirection: 'column', flex: 1, minHeight: 0, overflow: 'hidden' }}>
            {/* Message list */}
            <div className="right-sidebar-scroll" style={{ flex: 1, display: 'flex', flexDirection: 'column' }}>
                {props.messages
                    .filter(m => m.role !== 'system' && m.role !== 'tool')
                    .map((msg, idx) => (
                        <ChatMessage
                            key={idx}
                            msg={msg}
                            idx={idx}
                            isAgentThinking={props.isAgentThinking}
                            onCopy={props.onCopy}
                            onEditStart={props.onEditStart}
                            onRestore={props.onRestore}
                            lastCopiedIdx={props.lastCopiedIdx}
                            editingMsgIdx={props.editingMsgIdx}
                            editValue={props.editValue}
                            onEditChange={props.onEditChange}
                            onEditSave={props.onEditSave}
                            onEditCancel={props.onEditCancel}
                        />
                    ))
                }
                <div ref={messagesEndRef} />
            </div>

            {/* Input area */}
            <div style={{ flexShrink: 0, padding: '8px 10px' }}>
                <ChatInput
                    inputRef={props.inputRef}
                    inputValue={props.inputValue}
                    isAgentThinking={props.isAgentThinking}
                    isSpecModeActive={props.isSpecModeActive}
                    attachedFiles={props.attachedFiles}
                    isAttaching={props.isAttaching}
                    onChange={props.onInputChange}
                    onKeyDown={props.onKeyDown}
                    onPaste={props.onPaste}
                    onRemoveFile={props.onRemoveFile}
                    toolbar={
                        <ChatToolbar
                            mode={props.mode}
                            model={props.model}
                            modeStyle={props.modeStyle}
                            modelLabel={props.modelLabel}
                            ttsEnabled={props.ttsEnabled}
                            ttsPreset={props.ttsPreset}
                            isAgentThinking={props.isAgentThinking}
                            onModeClick={props.onModeClick}
                            onModelClick={props.onModelClick}
                            onAttach={props.onAttach}
                            onToggleTts={props.onToggleTts}
                            onTtsPresetChange={props.onTtsPresetChange}
                            onToggleVoice={props.onToggleVoice}
                            isVoiceListening={props.isVoiceListening}
                            reasoningToggle={props.reasoningToggle}
                            webUiControls={props.webUiControls}
                            onSend={props.onSend}
                            inputEmpty={!props.inputValue.trim() && props.attachedFiles.length === 0}
                        />
                    }
                />
            </div>
        </div>
    );
};

export default ChatPanel;
