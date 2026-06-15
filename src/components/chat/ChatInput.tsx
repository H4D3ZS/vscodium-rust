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
        <div className="composer" style={isSpecModeActive ? {
            borderColor: 'var(--terminator-accent, #00c6ff)',
            boxShadow: '0 0 0 1px var(--terminator-accent, #00c6ff)',
        } : undefined}>
            <textarea
                ref={inputRef}
                value={inputValue}
                onChange={onChange}
                onKeyDown={onKeyDown}
                onPaste={onPaste}
                className="composer__textarea"
                placeholder={
                    isAgentThinking ? 'Agent executing...' :
                    isSpecModeActive ? 'Describe the feature to auto-generate requirements & tasks...' :
                    'Ask anything...  (type @ to mention a file)'
                }
                disabled={isAgentThinking}
                rows={1}
            />

            {(attachedFiles.length > 0 || isAttaching) && (
                <div style={{ display: 'flex', flexWrap: 'wrap', gap: '4px', marginTop: '6px', paddingTop: '6px', borderTop: '1px solid rgba(255,255,255,0.04)' }}>
                    {attachedFiles.map((item: any, i: number) => (
                        <div key={item.id || i} style={{
                            display: 'flex', alignItems: 'center', gap: '4px', padding: '2px 6px',
                            background: 'rgba(59,130,246,0.08)', borderRadius: '4px', fontSize: '10px',
                            border: '1px solid rgba(59,130,246,0.15)', color: 'rgba(255,255,255,0.8)',
                        }}>
                            {item.thumbnail
                                ? <img src={item.thumbnail} style={{ width: '14px', height: '14px', borderRadius: '2px', objectFit: 'cover' }} alt="" />
                                : <i className="codicon codicon-file" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '10px', opacity: 0.6 }} />
                            }
                            <span>{item.name}</span>
                            <i
                                className="codicon codicon-close"
                                onClick={() => onRemoveFile(item.path)}
                                style={{ fontFamily: 'codicon', fontStyle: 'normal', cursor: 'pointer', opacity: 0.4, fontSize: '9px' }}
                            />
                        </div>
                    ))}
                    {isAttaching && (
                        <div style={{
                            display: 'flex', alignItems: 'center', gap: '4px', padding: '2px 6px',
                            borderRadius: '4px', fontSize: '10px',
                            border: '1px dashed rgba(255,255,255,0.15)', color: 'rgba(255,255,255,0.4)',
                        }}>
                            <i className="codicon codicon-loading codicon-modifier-spin" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '9px' }} />
                            <span>Loading...</span>
                        </div>
                    )}
                </div>
            )}

            {toolbar}
        </div>
    );
};

export default ChatInput;
