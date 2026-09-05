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
                width: '320px',
                background: 'var(--vscode-editor-background)',
                border: '1px solid var(--vscode-panel-border)',
                borderRadius: '6px',
                boxShadow: '0 4px 16px rgba(0,0,0,0.3)',
                padding: '8px 12px',
                animation: 'slideInUp 0.2s ease-out',
                fontSize: '13px'
            }}
        >
            <div style={{ display: 'flex', alignItems: 'center', gap: '6px', marginBottom: '6px' }}>
                <div style={{ background: 'var(--vscode-button-background)', width: '24px', height: '24px', borderRadius: '4px', display: 'flex', alignItems: 'center', justifyContent: 'center', flexShrink: '0' }}>
                    <Bot size={12} color="white" />
                </div>
                <span style={{ fontSize: '12px', fontWeight: 600, flex: 1 }}>Inline Edit</span>
                <X size={12} style={{ cursor: 'pointer', opacity: 0.5, width: '16px', height: '16px' }} onClick={onClose} />
            </div>

            <div style={{ position: 'relative' }}>
                <input
                    ref={inputRef}
                    type="text"
                    value={prompt}
                    onChange={(e) => setPrompt(e.target.value)}
                    onKeyDown={handleKeyDown}
                    placeholder="Edit with AI..."
                    style={{
                        width: '100%',
                        background: 'var(--vscode-input-background)',
                        color: 'var(--vscode-input-foreground)',
                        border: '1px solid var(--vscode-input-border)',
                        borderRadius: '3px',
                        padding: '6px 10px',
                        fontSize: '13px',
                        outline: 'none'
                    }}
                />
                <div style={{ position: 'absolute', right: '8px', top: '50%', transform: 'translateY(-50%)', display: 'flex', gap: '4px', alignItems: 'center' }}>
                    <Sparkles size={12} className="spinning" style={{ color: 'var(--vscode-button-background)', opacity: prompt ? 0.7 : 0.2 }} />
                    <Send
                        size={12}
                        style={{ cursor: prompt ? 'pointer' : 'default', opacity: prompt ? 1 : 0.2, width: '16px', height: '16px' }}
                        onClick={() => prompt && onSubmit(prompt)}
                    />
                </div>
            </div>
            <div style={{ marginTop: '6px', fontSize: '11px', opacity: 0.6 }}>
                <kbd style={{ padding: '2px 4px', background: 'var(--vscode-keybindingLabel-background)', borderRadius: '2px', fontSize: '11px' }}>Enter</kbd> to&nbsp;save, <kbd style={{ padding: '2px 4px', background: 'var(--vscode-keybindingLabel-background)', borderRadius: '2px', fontSize: '11px' }}>Esc</kbd> to&nbsp;cancel
            </div>
        </div>
    );
};

export default InlineEditOverlay;
