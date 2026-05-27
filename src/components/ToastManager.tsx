import React, { useEffect, useState, useCallback } from 'react';
import { listen } from '@tauri-apps/api/event';

interface Toast {
    id: string;
    message: string;
    type: 'info' | 'success' | 'warning' | 'error';
    duration?: number;
}

let _addToast: ((t: Omit<Toast, 'id'>) => void) | null = null;

export function showToast(message: string, type: Toast['type'] = 'info', duration = 3500) {
    _addToast?.({ message, type, duration });
}

const ICONS: Record<Toast['type'], string> = {
    info: 'codicon-info',
    success: 'codicon-check',
    warning: 'codicon-warning',
    error: 'codicon-error',
};

const COLORS: Record<Toast['type'], string> = {
    info: '#60a5fa',
    success: '#4ade80',
    warning: '#fbbf24',
    error: '#f87171',
};

const ToastManager: React.FC = () => {
    const [toasts, setToasts] = useState<Toast[]>([]);

    const addToast = useCallback((t: Omit<Toast, 'id'>) => {
        const id = `toast-${Date.now()}-${Math.random()}`;
        setToasts(prev => [...prev, { ...t, id }]);
        setTimeout(() => setToasts(prev => prev.filter(x => x.id !== id)), t.duration ?? 3500);
    }, []);

    useEffect(() => {
        _addToast = addToast;
        return () => { _addToast = null; };
    }, [addToast]);

    // Listen for Tauri toast events from backend
    useEffect(() => {
        let unlisten: (() => void) | null = null;
        listen<{ message: string; type?: Toast['type'] }>('app-toast', e => {
            addToast({ message: e.payload.message, type: e.payload.type ?? 'info' });
        }).then(u => { unlisten = u; });
        return () => unlisten?.();
    }, [addToast]);

    if (toasts.length === 0) return null;

    return (
        <div style={{
            position: 'fixed',
            bottom: '30px',
            left: '50%',
            transform: 'translateX(-50%)',
            display: 'flex',
            flexDirection: 'column',
            gap: '8px',
            zIndex: 9999,
            pointerEvents: 'none',
        }}>
            {toasts.map(t => (
                <div
                    key={t.id}
                    style={{
                        display: 'flex',
                        alignItems: 'center',
                        gap: '8px',
                        padding: '8px 14px',
                        background: 'var(--color-surface-raised, #252526)',
                        border: `1px solid ${COLORS[t.type]}40`,
                        borderRadius: '8px',
                        boxShadow: '0 4px 16px rgba(0,0,0,0.5)',
                        fontSize: '12px',
                        color: 'var(--color-text, #cccccc)',
                        animation: 'fade-in 0.15s ease',
                        pointerEvents: 'auto',
                        whiteSpace: 'nowrap',
                        maxWidth: '480px',
                    }}
                >
                    <i
                        className={`codicon ${ICONS[t.type]}`}
                        style={{ fontFamily: 'codicon', fontStyle: 'normal', color: COLORS[t.type], fontSize: '14px', flexShrink: 0 }}
                    />
                    <span style={{ overflow: 'hidden', textOverflow: 'ellipsis' }}>{t.message}</span>
                </div>
            ))}
        </div>
    );
};

export default ToastManager;
