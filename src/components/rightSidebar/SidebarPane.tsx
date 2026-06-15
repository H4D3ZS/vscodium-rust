// Collapsible sidebar pane wrapper shared by RightSidebar sections.
// Extracted from RightSidebar.tsx (A2 god-component split).
import React, { useEffect, useState, useRef, useMemo, useCallback, memo } from 'react';
import { useStore } from '../../store';


const SidebarPane: React.FC<{ title: string; children: React.ReactNode; defaultCollapsed?: boolean; actions?: React.ReactNode }> = ({ title, children, defaultCollapsed = false, actions }) => {
    const [isCollapsed, setIsCollapsed] = useState(defaultCollapsed);
    return (
        <div className="sidebar-pane" style={{ display: 'flex', flexDirection: 'column', flexShrink: 0, borderBottom: '1px solid var(--vscode-sideBar-border, rgba(255,255,255,0.05))' }}>
            <div
                className={`pane-header${isCollapsed ? ' collapsed' : ''}`}
                onClick={() => setIsCollapsed(!isCollapsed)}
                style={{
                    padding: '6px 10px',
                    display: 'flex',
                    alignItems: 'center',
                    cursor: 'pointer',
                    background: 'var(--vscode-sideBarSectionHeader-background, rgba(255,255,255,0.02))',
                    fontSize: '11px',
                    fontWeight: 600,
                    textTransform: 'uppercase',
                    letterSpacing: '0.05em',
                    color: 'var(--vscode-sideBar-foreground)',
                    opacity: 0.8
                }}
            >
                <i className={`codicon codicon-chevron-${isCollapsed ? 'right' : 'down'}`} style={{ fontFamily: 'codicon', fontStyle: 'normal', marginRight: '8px', fontSize: '12px' }}></i>
                <span style={{ flex: 1 }}>{title}</span>
                {actions && <div className="pane-actions" onClick={e => e.stopPropagation()}>{actions}</div>}
            </div>
            {!isCollapsed && <div className="pane-content" style={{ padding: '8px 0' }}>{children}</div>}
        </div>
    );
};


export { SidebarPane };
