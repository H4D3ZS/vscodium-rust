/**
 * Antigravity IDE parity: Agent panel "..." menu → MCP Servers / Manage MCP Servers.
 * @see docs/MCP_STORE.md
 */
import React, { useEffect, useRef, useState } from 'react';
import { useStore } from '../../store';

const AgentMcpMenu: React.FC = () => {
    const [open, setOpen] = useState(false);
    const ref = useRef<HTMLDivElement>(null);

    useEffect(() => {
        const onDoc = (e: MouseEvent) => {
            if (ref.current && !ref.current.contains(e.target as Node)) setOpen(false);
        };
        document.addEventListener('mousedown', onDoc);
        return () => document.removeEventListener('mousedown', onDoc);
    }, []);

    const openStore = (view: 'store' | 'manage') => {
        setOpen(false);
        useStore.getState().openMcpStore?.(view);
    };

    return (
        <div ref={ref} style={{ position: 'relative', display: 'flex', alignItems: 'center' }}>
            <button
                type="button"
                onClick={() => setOpen(v => !v)}
                title="MCP Servers (Antigravity-style menu)"
                aria-expanded={open}
                style={{
                    cursor: 'pointer',
                    opacity: open ? 1 : 0.7,
                    display: 'flex',
                    alignItems: 'center',
                    background: 'none',
                    border: 'none',
                    color: 'inherit',
                    padding: '2px 4px',
                }}
            >
                <i className="codicon codicon-kebab-vertical" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '14px' }} />
            </button>
            {open && (
                <div
                    role="menu"
                    style={{
                        position: 'absolute',
                        top: '100%',
                        right: 0,
                        marginTop: 4,
                        minWidth: 200,
                        zIndex: 50,
                        background: 'var(--vscode-menu-background, #252526)',
                        border: '1px solid var(--vscode-menu-border, rgba(255,255,255,0.1))',
                        borderRadius: 4,
                        boxShadow: '0 4px 16px rgba(0,0,0,0.35)',
                        padding: '4px 0',
                    }}
                >
                    <button
                        type="button"
                        role="menuitem"
                        className="agent-mcp-menu-item"
                        onClick={() => openStore('store')}
                    >
                        MCP Servers
                    </button>
                    <button
                        type="button"
                        role="menuitem"
                        className="agent-mcp-menu-item"
                        onClick={() => openStore('manage')}
                    >
                        Manage MCP Servers
                    </button>
                </div>
            )}
            <style>{`
                .agent-mcp-menu-item {
                    display: block;
                    width: 100%;
                    text-align: left;
                    padding: 6px 14px;
                    font-size: 12px;
                    background: transparent;
                    border: none;
                    color: var(--vscode-menu-foreground, #ccc);
                    cursor: pointer;
                }
                .agent-mcp-menu-item:hover {
                    background: var(--vscode-menu-selectionBackground, rgba(255,255,255,0.08));
                }
            `}</style>
        </div>
    );
};

export default AgentMcpMenu;
