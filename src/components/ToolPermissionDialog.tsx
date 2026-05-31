import React, { useEffect } from 'react';
import { listen } from '../tauri_bridge';
import { useStore } from '../store';
import { AlertTriangle, Terminal, Trash2, GitCommit, X, Check } from 'lucide-react';

const LEVEL_ICONS: Record<string, React.ReactNode> = {
    dangerous: <AlertTriangle size={16} style={{ color: '#f87171' }} />,
    caution: <Terminal size={16} style={{ color: '#fbbf24' }} />,
};

const TOOL_ICONS: Record<string, React.ReactNode> = {
    remove_item: <Trash2 size={14} />,
    run_command: <Terminal size={14} />,
    git_commit: <GitCommit size={14} />,
};

export const ToolPermissionDialog: React.FC = () => {
    const setPendingToolPermission = useStore(s => s.setPendingToolPermission);
    const pending = useStore(s => s.pendingToolPermission);
    const respondToolPermission = useStore(s => s.respondToolPermission);

    useEffect(() => {
        const unlisten = listen('tool_permission_request', (event: any) => {
            const { id, tool, args, level } = event.payload ?? {};
            setPendingToolPermission({ id, tool, args, level });
        });
        return () => { unlisten.then(f => f()); };
    }, [setPendingToolPermission]);

    if (!pending) return null;

    const icon = TOOL_ICONS[pending.tool] ?? LEVEL_ICONS[pending.level] ?? <AlertTriangle size={16} />;
    const argsStr = pending.args ? JSON.stringify(pending.args, null, 2) : '';

    return (
        <div style={{
            position: 'fixed', inset: 0, zIndex: 99999,
            display: 'flex', alignItems: 'center', justifyContent: 'center',
            background: 'rgba(0,0,0,0.6)', backdropFilter: 'blur(4px)',
        }}>
            <div style={{
                background: 'var(--vscode-editor-background, #1e1e1e)',
                border: '1px solid rgba(248,113,113,0.4)',
                borderRadius: 12, padding: '20px 24px',
                width: 420, maxWidth: '90vw',
                boxShadow: '0 24px 64px rgba(0,0,0,0.6)',
            }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: 10, marginBottom: 16 }}>
                    {LEVEL_ICONS[pending.level] ?? <AlertTriangle size={18} style={{ color: '#f87171' }} />}
                    <span style={{ fontSize: 14, fontWeight: 700, color: '#f87171' }}>
                        Tool Permission Required
                    </span>
                </div>

                <p style={{ fontSize: 12, color: 'rgba(255,255,255,0.7)', marginBottom: 12 }}>
                    The agent wants to execute a <strong style={{ color: '#fbbf24' }}>{pending.level}</strong> operation:
                </p>

                <div style={{
                    background: 'rgba(255,255,255,0.05)', borderRadius: 8,
                    padding: '10px 14px', marginBottom: 16,
                    border: '1px solid rgba(255,255,255,0.08)',
                }}>
                    <div style={{ display: 'flex', alignItems: 'center', gap: 8, marginBottom: 6 }}>
                        {icon}
                        <code style={{ fontSize: 13, fontWeight: 600, color: '#e2e8f0' }}>{pending.tool}</code>
                    </div>
                    {argsStr && (
                        <pre style={{
                            fontSize: 10, color: 'rgba(255,255,255,0.5)',
                            margin: 0, maxHeight: 120, overflowY: 'auto',
                            whiteSpace: 'pre-wrap', wordBreak: 'break-all',
                        }}>
                            {argsStr.slice(0, 500)}{argsStr.length > 500 ? '…' : ''}
                        </pre>
                    )}
                </div>

                <div style={{ display: 'flex', gap: 8, justifyContent: 'flex-end' }}>
                    <button
                        onClick={() => respondToolPermission(pending.id, false)}
                        style={{
                            display: 'flex', alignItems: 'center', gap: 6,
                            padding: '6px 14px', borderRadius: 6, fontSize: 12, fontWeight: 600,
                            background: 'rgba(248,113,113,0.1)', border: '1px solid rgba(248,113,113,0.3)',
                            color: '#f87171', cursor: 'pointer',
                        }}
                    >
                        <X size={12} /> Deny
                    </button>
                    <button
                        onClick={() => respondToolPermission(pending.id, true)}
                        style={{
                            display: 'flex', alignItems: 'center', gap: 6,
                            padding: '6px 14px', borderRadius: 6, fontSize: 12, fontWeight: 600,
                            background: 'rgba(52,211,153,0.15)', border: '1px solid rgba(52,211,153,0.3)',
                            color: '#34d399', cursor: 'pointer',
                        }}
                    >
                        <Check size={12} /> Allow
                    </button>
                </div>
            </div>
        </div>
    );
};

export default ToolPermissionDialog;
