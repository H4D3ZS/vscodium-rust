import React, { useState, useEffect } from 'react';
import { detectCrash, clearCrashMarker, formatElapsed, type CrashRecoveryInfo } from '../application/agent/syncAgentMessages';

/**
 * CrashRecoveryBanner — shown at the top of the chat panel when the app
 * detects that the previous session crashed mid-conversation. Offers to
 * restore the conversation from the backend.
 */
const CrashRecoveryBanner: React.FC = () => {
    const [info, setInfo] = useState<CrashRecoveryInfo | null>(null);
    const [dismissed, setDismissed] = useState(false);

    useEffect(() => {
        // Delay detection slightly so the store has time to hydrate
        const timer = setTimeout(() => {
            const crash = detectCrash();
            if (crash) setInfo(crash);
        }, 1500);
        return () => clearTimeout(timer);
    }, []);

    const handleRestore = async () => {
        try {
            // Load the last conversation from backend
            const { loadChatSession } = (await import('../store')).useStore.getState();
            const { invoke } = await import('../tauri_bridge');
            const sessions = await invoke<{ path: string }[]>('list_chat_sessions').catch(() => []);
            if (sessions?.length > 0) {
                await loadChatSession(sessions[0].path);
            }
        } catch (err) {
            console.warn('[CrashRecovery] Failed to restore session:', err);
        }
        clearCrashMarker();
        setDismissed(true);
    };

    const handleDismiss = () => {
        clearCrashMarker();
        setDismissed(true);
    };

    if (!info || dismissed) return null;

    return (
        <div style={{
            display: 'flex',
            alignItems: 'center',
            gap: '8px',
            padding: '8px 12px',
            background: 'rgba(251, 191, 36, 0.1)',
            borderBottom: '1px solid rgba(251, 191, 36, 0.25)',
            fontSize: '12px',
            color: 'var(--vscode-foreground, #e2e8f0)',
            flexShrink: 0,
        }}>
            <i className="codicon codicon-warning" style={{
                fontFamily: 'codicon', fontStyle: 'normal', fontSize: '14px',
                color: '#f59e0b', flexShrink: 0,
            }} />
            <div style={{ flex: 1, minWidth: 0 }}>
                <span>
                    Previous session interrupted {formatElapsed(info.elapsedMs)}
                    {info.messageCount > 0 && ` with ${info.messageCount} messages`}
                </span>
            </div>
            <button
                onClick={handleRestore}
                style={{
                    padding: '4px 10px',
                    background: 'rgba(251, 191, 36, 0.15)',
                    border: '1px solid rgba(251, 191, 36, 0.3)',
                    borderRadius: '4px',
                    color: '#fbbf24',
                    fontSize: '11px',
                    cursor: 'pointer',
                    whiteSpace: 'nowrap',
                }}
            >
                Restore
            </button>
            <button
                onClick={handleDismiss}
                style={{
                    padding: '4px',
                    background: 'none',
                    border: 'none',
                    color: 'inherit',
                    opacity: 0.5,
                    cursor: 'pointer',
                    fontSize: '14px',
                    lineHeight: 1,
                }}
                title="Dismiss"
            >
                ×
            </button>
        </div>
    );
};

export default CrashRecoveryBanner;
