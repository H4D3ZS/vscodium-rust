import React from 'react';
import type { AttachedContext } from '../../store';

interface ChatInputProps {
    inputRef: React.RefObject<HTMLTextAreaElement>;
    inputValue: string;
    isAgentThinking: boolean;
    isSpecModeActive: boolean;
    attachedFiles: AttachedContext[];
    isAttaching: boolean;
    onChange: (e: React.ChangeEvent<HTMLTextAreaElement>) => void;
    onKeyDown: (e: React.KeyboardEvent<HTMLTextAreaElement>) => void;
    onPaste: (e: React.ClipboardEvent<HTMLTextAreaElement>) => void;
    onRemoveFile: (path: string) => void;
    toolbar: React.ReactNode;
}

const ChatInput: React.FC<ChatInputProps> = ({
    inputRef, inputValue, isAgentThinking, isSpecModeActive,
    attachedFiles, isAttaching,
    onChange, onKeyDown, onPaste, onRemoveFile,
    toolbar,
}) => {
    return (
        <div style={{
            background: 'var(--vscode-input-background)',
            border: `1px solid ${isSpecModeActive ? 'var(--terminator-accent, #00c6ff)' : 'var(--vscode-input-border, transparent)'}`,
            boxShadow: isSpecModeActive ? '0 0 10px rgba(0,198,255,0.25)' : 'none',
            borderRadius: '8px', padding: '7px 10px',
            display: 'flex', flexDirection: 'column',
            transition: 'all 0.2s',
        }}>
            <textarea
                ref={inputRef}
                value={inputValue}
                onChange={onChange}
                onKeyDown={onKeyDown}
                onPaste={onPaste}
                className="agent-mission-input"
                placeholder={
                    isAgentThinking ? 'Agent executing...' :
                    isSpecModeActive ? 'Describe the feature to auto-generate requirements & tasks...' :
                    'Launch a mission...  (type @ to mention a file)'
                }
                disabled={isAgentThinking}
                style={{
                    background: 'transparent', border: 'none', outline: 'none',
                    color: 'var(--vscode-editor-foreground, #fff)', resize: 'none', fontSize: '13px', lineHeight: '1.45',
                    width: '100%', minHeight: '28px',
                    opacity: isAgentThinking ? 0.5 : 1,
                }}
            />

            {(attachedFiles.length > 0 || isAttaching) && (
                <div style={{ display: 'flex', flexWrap: 'wrap', gap: '6px', marginTop: '4px', paddingBottom: '4px' }}>
                    {attachedFiles.map((item: any, i: number) => (
                        <div key={item.id || i} style={{
                            display: 'flex', alignItems: 'center', gap: '6px', padding: '3px 8px',
                            background: 'rgba(255,255,255,0.08)', borderRadius: '6px', fontSize: '11px',
                            border: '1px solid rgba(255,255,255,0.1)', color: 'rgba(255,255,255,0.9)',
                        }}>
                            {item.thumbnail
                                ? <img src={item.thumbnail} style={{ width: '16px', height: '16px', borderRadius: '3px', objectFit: 'cover' }} alt="" />
                                : <span style={{ opacity: 0.7, fontSize: '10px' }}>{item.type === 'attachment' ? 'IMG' : '{ }'}</span>
                            }
                            <span style={{ fontWeight: 500 }}>{item.name}</span>
                            <i
                                className="codicon codicon-close"
                                onClick={() => onRemoveFile(item.path)}
                                style={{ fontFamily: 'codicon', fontStyle: 'normal', cursor: 'pointer', opacity: 0.5, marginLeft: '2px', fontSize: '10px' }}
                            />
                        </div>
                    ))}
                    {isAttaching && (
                        <div style={{
                            display: 'flex', alignItems: 'center', gap: '6px', padding: '3px 8px',
                            background: 'rgba(255,255,255,0.05)', borderRadius: '6px', fontSize: '11px',
                            border: '1px dashed rgba(255,255,255,0.2)', color: 'rgba(255,255,255,0.5)',
                            fontStyle: 'italic',
                        }}>
                            <i className="codicon codicon-loading codicon-modifier-spin" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '10px' }} />
                            <span>Neuralizing...</span>
                        </div>
                    )}
                </div>
            )}

            {toolbar}
        </div>
    );
};

export default ChatInput;
