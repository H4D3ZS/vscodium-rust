import React, { useState, useEffect } from 'react';
import { useStore } from '../store';

function trustKey(root: string): string {
    return `workspace.trusted:${root}`;
}

const WorkspaceTrustBanner: React.FC = () => {
    const activeRoot = useStore(s => s.activeRoot);
    const [dismissed, setDismissed] = useState(true);

    useEffect(() => {
        if (!activeRoot) {
            setDismissed(true);
            return;
        }
        try {
            const trusted = localStorage.getItem(trustKey(activeRoot)) === '1';
            setDismissed(trusted);
        } catch {
            setDismissed(false);
        }
    }, [activeRoot]);

    if (!activeRoot || dismissed) return null;

    const trust = () => {
        try { localStorage.setItem(trustKey(activeRoot), '1'); } catch { }
        setDismissed(true);
    };

    const name = useStore.getState().activeRootName || activeRoot;

    return (
        <div style={{
            position: 'absolute',
            top: 0,
            left: 0,
            right: 0,
            zIndex: 50,
            background: 'var(--vscode-inputValidation-warningBackground, #352a05)',
            borderBottom: '1px solid var(--vscode-inputValidation-warningBorder, #b89500)',
            padding: '6px 12px',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'space-between',
            fontSize: 12,
            gap: 12,
        }}>
            <span>
                <i className="codicon codicon-shield" style={{ fontFamily: 'codicon', fontStyle: 'normal', marginRight: 6 }} />
                Restricted mode — <b>{name}</b> is not trusted. Tasks, extensions, and scripts may be limited until you trust this folder.
            </span>
            <div style={{ display: 'flex', gap: 8, flexShrink: 0 }}>
                <button type="button" onClick={trust} style={trustBtn}>Trust Folder</button>
                <button type="button" onClick={() => setDismissed(true)} style={dismissBtn}>Dismiss</button>
            </div>
        </div>
    );
};

const trustBtn: React.CSSProperties = {
    background: 'var(--vscode-button-background)',
    color: 'var(--vscode-button-foreground)',
    border: 'none',
    borderRadius: 3,
    padding: '4px 10px',
    cursor: 'pointer',
    fontSize: 11,
};

const dismissBtn: React.CSSProperties = {
    ...trustBtn,
    background: 'transparent',
    border: '1px solid var(--vscode-panel-border)',
};

export default WorkspaceTrustBanner;
