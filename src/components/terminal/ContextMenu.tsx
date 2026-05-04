import React, { useEffect, useRef } from 'react';

interface ContextMenuOption {
    label: string;
    icon?: string;
    onClick: () => void;
    danger?: boolean;
}

interface ContextMenuProps {
    x: number;
    y: number;
    options: ContextMenuOption[];
    onClose: () => void;
    visible: boolean;
}

const ContextMenu: React.FC<ContextMenuProps> = ({ x, y, options, onClose, visible }) => {
    const menuRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        const handleClickOutside = (e: MouseEvent) => {
            if (menuRef.current && !menuRef.current.contains(e.target as Node)) {
                onClose();
            }
        };

        if (visible) {
            document.addEventListener('mousedown', handleClickOutside);
        }
        return () => document.removeEventListener('mousedown', handleClickOutside);
    }, [visible, onClose]);

    if (!visible) return null;

    return (
        <div 
            ref={menuRef}
            className="context-menu"
            style={{
                position: 'fixed',
                top: y,
                left: x,
                zIndex: 10000,
                minWidth: '200px',
                background: 'rgba(35, 35, 35, 0.85)',
                backdropFilter: 'blur(15px)',
                color: '#fff',
                border: '1px solid rgba(255, 255, 255, 0.1)',
                boxShadow: '0 10px 30px rgba(0, 0, 0, 0.5), 0 0 0 1px rgba(255, 255, 255, 0.05)',
                padding: '6px',
                borderRadius: '10px',
                animation: 'menuIn 0.15s ease-out'
            }}
        >
            {options.map((option, index) => (
                <div 
                    key={index}
                    onClick={(e) => {
                        e.stopPropagation();
                        option.onClick();
                        onClose();
                    }}
                    style={{
                        display: 'flex',
                        alignItems: 'center',
                        gap: '10px',
                        padding: '8px 12px',
                        cursor: 'pointer',
                        fontSize: '13px',
                        fontWeight: 500,
                        borderRadius: '6px',
                        color: option.danger ? '#f87171' : 'rgba(255, 255, 255, 0.85)',
                        transition: 'all 0.1s'
                    }}
                    onMouseEnter={(e) => {
                        e.currentTarget.style.background = option.danger ? 'rgba(248, 113, 113, 0.1)' : 'rgba(255, 255, 255, 0.08)';
                        e.currentTarget.style.color = option.danger ? '#f87171' : '#fff';
                        e.currentTarget.style.transform = 'translateX(2px)';
                    }}
                    onMouseLeave={(e) => {
                        e.currentTarget.style.background = 'transparent';
                        e.currentTarget.style.color = option.danger ? '#f87171' : 'rgba(255, 255, 255, 0.85)';
                        e.currentTarget.style.transform = 'translateX(0)';
                    }}
                >
                    {option.icon && <i className={`codicon ${option.icon}`} style={{ fontSize: '14px', opacity: 0.8 }}></i>}
                    <span style={{ flex: 1 }}>{option.label}</span>
                </div>
            ))}
            <style>{`
                @keyframes menuIn {
                    from { opacity: 0; transform: translateY(5px) scale(0.98); }
                    to { opacity: 1; transform: translateY(0) scale(1); }
                }
            `}</style>
        </div>
    );
};

export default ContextMenu;

