import React from 'react';
import { useStore } from '../store';

// Clean, native empty-editor welcome (shown when a folder is open but no file is
// selected). Replaces the old faint cube watermark. Original "agent orbit" mark,
// themed via --vscode tokens, with a "Code with Agent" shortcut that opens the
// agent panel.

const kbd: React.CSSProperties = {
    display: 'inline-flex',
    alignItems: 'center',
    justifyContent: 'center',
    minWidth: '20px',
    height: '20px',
    padding: '0 6px',
    fontSize: '11px',
    fontWeight: 600,
    lineHeight: 1,
    color: 'var(--vscode-keybindingLabel-foreground, var(--vscode-descriptionForeground))',
    background: 'var(--vscode-keybindingLabel-background, rgba(255,255,255,0.06))',
    border: '1px solid var(--vscode-keybindingLabel-border, rgba(255,255,255,0.12))',
    borderRadius: '4px',
};

const EmptyEditorWelcome: React.FC = () => (
    <div style={{
        flex: 1,
        height: '100%',
        width: '100%',
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        gap: '18px',
        userSelect: 'none',
    }}>
        <svg width="58" height="58" viewBox="0 0 48 48" fill="none" style={{ opacity: 0.85 }}>
            <circle cx="24" cy="24" r="17" stroke="var(--vscode-foreground)" strokeOpacity="0.32" strokeWidth="2" />
            <circle cx="24" cy="24" r="6" fill="var(--vscode-foreground)" fillOpacity="0.8" />
            <circle cx="24" cy="7" r="3.2" fill="var(--vscode-foreground)" fillOpacity="0.8" />
        </svg>
        <div style={{
            fontSize: '22px', fontWeight: 600, letterSpacing: '0.4px',
            color: 'var(--vscode-foreground)', opacity: 0.9,
        }}>
            VSCodium-Rust
        </div>
        <div
            onClick={() => { try { (useStore.getState() as any).setIsRightSidebarOpen?.(true); } catch { /* */ } }}
            style={{
                display: 'flex', alignItems: 'center', gap: '10px',
                fontSize: '13px', color: 'var(--vscode-descriptionForeground)', cursor: 'pointer',
            }}
        >
            <span>Code with Agent</span>
            <span style={kbd}>Ctrl</span>
            <span style={{ opacity: 0.5 }}>+</span>
            <span style={kbd}>L</span>
        </div>
    </div>
);

export default EmptyEditorWelcome;
