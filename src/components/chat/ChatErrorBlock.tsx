import React from 'react';

export interface ChatErrorBlockProps {
    message: string;
    onRetry?: () => void;
    /** Error category for styling and icon selection. */
    kind?: 'connection' | 'timeout' | 'model' | 'permission' | 'generic';
}

const KIND_CONFIG: Record<string, { icon: string; bg: string; border: string }> = {
    connection: { icon: 'codicon-error', bg: 'rgba(239,68,68,0.08)', border: 'rgba(239,68,68,0.25)' },
    timeout:    { icon: 'codicon-clock', bg: 'rgba(251,191,36,0.08)', border: 'rgba(251,191,36,0.25)' },
    model:      { icon: 'codicon-warning', bg: 'rgba(168,85,247,0.08)', border: 'rgba(168,85,247,0.25)' },
    permission: { icon: 'codicon-lock', bg: 'rgba(59,130,246,0.08)', border: 'rgba(59,130,246,0.25)' },
    generic:    { icon: 'codicon-error', bg: 'rgba(239,68,68,0.06)', border: 'rgba(239,68,68,0.2)' },
};

function detectKind(message: string): string {
    const m = message.toLowerCase();
    if (m.includes('not responding') || m.includes('connection failed') || m.includes('econnrefused') || m.includes('fetch failed')) return 'connection';
    if (m.includes('timeout') || m.includes('timed out')) return 'timeout';
    if (m.includes('model') && (m.includes('not found') || m.includes('not loaded') || m.includes('not available'))) return 'model';
    if (m.includes('permission') || m.includes('denied') || m.includes('unauthorized')) return 'permission';
    return 'generic';
}

/**
 * ChatErrorBlock — styled error container for agent chat messages. Shows a
 * distinct visual treatment with optional retry action.
 */
const ChatErrorBlock: React.FC<ChatErrorBlockProps> = ({ message, onRetry, kind: explicitKind }) => {
    const kind = explicitKind || detectKind(message);
    const config = KIND_CONFIG[kind] || KIND_CONFIG.generic;

    return (
        <div style={{
            display: 'flex',
            alignItems: 'flex-start',
            gap: '8px',
            padding: '10px 14px',
            margin: '4px 0',
            background: config.bg,
            border: `1px solid ${config.border}`,
            borderRadius: '6px',
            fontSize: '12px',
            lineHeight: '1.5',
            color: 'var(--vscode-foreground, #e2e8f0)',
        }}>
            <i
                className={`codicon ${config.icon}`}
                style={{
                    fontFamily: 'codicon',
                    fontStyle: 'normal',
                    fontSize: '14px',
                    marginTop: '2px',
                    flexShrink: 0,
                    opacity: 0.8,
                }}
            />
            <div style={{ flex: 1, minWidth: 0 }}>
                <div style={{ fontWeight: 500, marginBottom: '2px' }}>Agent Error</div>
                <div style={{ opacity: 0.8, wordBreak: 'break-word' }}>{message}</div>
                {onRetry && (
                    <button
                        onClick={onRetry}
                        style={{
                            marginTop: '6px',
                            padding: '3px 10px',
                            background: 'rgba(255,255,255,0.08)',
                            border: '1px solid rgba(255,255,255,0.12)',
                            borderRadius: '4px',
                            color: 'inherit',
                            fontSize: '11px',
                            cursor: 'pointer',
                        }}
                    >
                        Retry
                    </button>
                )}
            </div>
        </div>
    );
};

export default ChatErrorBlock;
