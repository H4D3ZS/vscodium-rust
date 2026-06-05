import React, { useState, useEffect, useRef } from 'react';
import { Send, X, Bot, Sparkles } from 'lucide-react';

interface InlineEditOverlayProps {
    position: { top: number, left: number };
    onClose: () => void;
    onSubmit: (prompt: string) => void;
}

const InlineEditOverlay: React.FC<InlineEditOverlayProps> = ({ position, onClose, onSubmit }) => {
    const [prompt, setPrompt] = useState('');
    const inputRef = useRef<HTMLInputElement>(null);

    useEffect(() => {
        inputRef.current?.focus();
    }, []);

    const handleKeyDown = (e: React.KeyboardEvent) => {
        if (e.key === 'Enter' && !e.shiftKey) {
            e.preventDefault();
            onSubmit(prompt);
        } else if (e.key === 'Escape') {
            onClose();
        }
    };

    return (
        <div
            className="inline-edit-overlay"
            style={{
                position: 'absolute',
                top: position.top,
                left: position.left,
                zIndex: 1000,
                width: '400px',
                background: 'var(--vscode-sideBar-background)',
                border: '1px solid var(--terminator-accent)',
                borderRadius: '8px',
                boxShadow: '0 8px 32px rgba(0,0,0,0.5)',
                padding: '12px',
                animation: 'slideInUp 0.2s ease-out'
            }}
        >
            <div style={{ display: 'flex', alignItems: 'center', gap: '8px', marginBottom: '10px' }}>
                <div style={{ background: 'var(--terminator-accent)', borderRadius: '4px', padding: '4px' }}>
                    <Bot size={14} color="white" />
                </div>
                <span style={{ fontSize: '12px', fontWeight: 600, flex: 1 }}>Inline Agent</span>
                <X size={14} style={{ cursor: 'pointer', opacity: 0.5 }} onClick={onClose} />
            </div>

            <div style={{ position: 'relative' }}>
                <input
                    ref={inputRef}
                    type="text"
                    value={prompt}
                    onChange={(e) => setPrompt(e.target.value)}
                    onKeyDown={handleKeyDown}
                    placeholder="Ask the agent to edit or generate code..."
                    style={{
                        width: '100%',
                        background: 'var(--vscode-editor-background)',
                        color: 'inherit',
                        border: '1px solid var(--vscode-panel-border)',
                        borderRadius: '4px',
                        padding: '8px 32px 8px 10px',
                        fontSize: '13px',
                        outline: 'none'
                    }}
                />
                <div style={{ position: 'absolute', right: '8px', top: '50%', transform: 'translateY(-50%)', display: 'flex', gap: '8px', alignItems: 'center' }}>
                    <Sparkles size={14} className="spinning" style={{ color: 'var(--terminator-accent)', opacity: prompt ? 1 : 0.2 }} />
                    <Send
                        size={14}
                        style={{ cursor: prompt ? 'pointer' : 'default', opacity: prompt ? 1 : 0.2 }}
                        onClick={() => prompt && onSubmit(prompt)}
                    />
                </div>
            </div>
            <div style={{ marginTop: '8px', fontSize: '10px', opacity: 0.4 }}>
                Press <b>Enter</b> to submit, <b>Esc</b> to cancel.
            </div>
        </div>
    );
};

export default InlineEditOverlay;
