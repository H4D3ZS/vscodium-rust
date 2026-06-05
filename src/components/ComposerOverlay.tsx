import React, { useEffect, useRef } from 'react';
import { useStore } from '../store';
import Composer from './Composer';
import { X, Maximize2, Minimize2 } from 'lucide-react';

const ComposerOverlay: React.FC = () => {
    const isComposerOpen = useStore(state => state.isComposerOpen);
    const toggleComposer = useStore(state => state.toggleComposer);
    const [isFullScreen, setIsFullScreen] = React.useState(false);
    
    useEffect(() => {
        const handler = (e: KeyboardEvent) => {
            if (e.key === 'Escape' && isComposerOpen) toggleComposer(false);
        };
        window.addEventListener('keydown', handler);
        return () => window.removeEventListener('keydown', handler);
    }, [isComposerOpen, toggleComposer]);

    if (!isComposerOpen) return null;

    return (
        <div style={{
            position: 'absolute',
            top: 0, left: 0, right: 0, bottom: 0,
            zIndex: 9999,
            backgroundColor: 'rgba(0,0,0,0.35)',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            pointerEvents: 'auto' // capture clicks to close if needed, but let's just make it a pure overlay
        }}>
            <div style={{
                width: isFullScreen ? '95vw' : '800px',
                height: isFullScreen ? '95vh' : '600px',
                backgroundColor: 'var(--vscode-editor-background)',
                borderRadius: '2px',
                boxShadow: '0 8px 24px rgba(0, 0, 0, 0.45)',
                border: '1px solid var(--vscode-panel-border)',
                display: 'flex',
                flexDirection: 'column',
                overflow: 'hidden',
                transition: 'all 0.2s cubic-bezier(0.16, 1, 0.3, 1)'
            }}>
                {/* Title bar for dragging / controls */}
                <div style={{
                    height: '40px',
                    display: 'flex',
                    alignItems: 'center',
                    padding: '0 16px',
                    background: 'var(--vscode-titleBar-activeBackground)',
                    borderBottom: '1px solid var(--vscode-panel-border)',
                    userSelect: 'none',
                    justifyContent: 'space-between'
                }}>
                    <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                        <i className="codicon codicon-composer" style={{ fontSize: '14px', color: 'var(--terminator-accent)' }} />
                        <span style={{ fontSize: '12px', fontWeight: 600, opacity: 0.9 }}>Composer (Ctrl+I)</span>
                    </div>
                    <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
                        <div onClick={() => setIsFullScreen(!isFullScreen)} style={{ cursor: 'pointer', opacity: 0.6, display: 'flex' }} title="Toggle Fullscreen">
                            {isFullScreen ? <Minimize2 size={14} /> : <Maximize2 size={14} />}
                        </div>
                        <div onClick={() => toggleComposer(false)} style={{ cursor: 'pointer', opacity: 0.6, display: 'flex' }} title="Close (Esc)">
                            <X size={16} />
                        </div>
                    </div>
                </div>
                
                {/* Embedded Composer component */}
                <div style={{ flex: 1, overflow: 'hidden', position: 'relative' }}>
                    <Composer />
                </div>
            </div>
        </div>
    );
};

export default ComposerOverlay;
