import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import RunConfigsPanel from './RunConfigsPanel';

// ─────────────────────────────────────────────────────────────────────────────
//  DebugView — the panel mounted under the activity bar's "Run and Debug"
//  entry. It now hosts the RunConfigsPanel (.vscode/tasks.json +
//  launch.json launcher) up top, then a debug control strip and the
//  classic VARIABLES / WATCH / CALL STACK / BREAKPOINTS placeholders.
//
//  We kept the placeholders because future debugger integrations will
//  populate them; today they make the panel feel feature-complete instead
//  of empty.
// ─────────────────────────────────────────────────────────────────────────────

const DebugView: React.FC = () => {
    const [isDebugging, setIsDebugging] = useState(false);

    const handleStop = async () => {
        try {
            await invoke('debug_stop');
        } catch (e) {
            console.error('debug_stop failed', e);
        }
        setIsDebugging(false);
    };

    return (
        <div className="debug-view" style={{ display: 'flex', flexDirection: 'column', height: '100%' }}>
            {/* Run & Debug launchers — drives both .vscode/tasks.json and
                .vscode/launch.json. The user picks an entry; we route
                tasks through a terminal and launches through debug_start. */}
            <div style={{ flex: '0 0 auto', maxHeight: '50%', overflow: 'hidden', borderBottom: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.06))' }}>
                <RunConfigsPanel />
            </div>

            {/* Debug session controls */}
            <div style={{ padding: 10 }}>
                {isDebugging ? (
                    <div style={{ display: 'flex', gap: 4 }}>
                        <button style={ctrl}><i className="codicon codicon-debug-continue" style={icon} /></button>
                        <button style={ctrl}><i className="codicon codicon-debug-step-over" style={icon} /></button>
                        <button style={ctrl}><i className="codicon codicon-debug-step-into" style={icon} /></button>
                        <button onClick={handleStop} style={ctrlStop}><i className="codicon codicon-debug-stop" style={icon} /></button>
                    </div>
                ) : (
                    <div style={{ fontSize: 11, opacity: 0.55 }}>
                        Pick a launch configuration above to start debugging, or run a task to spawn a terminal.
                    </div>
                )}
            </div>

            {/* Classic debugger sub-panels */}
            <div style={{ flex: 1, overflowY: 'auto' }}>
                <Group title="VARIABLES" emptyHint="Not debugging" />
                <Group title="WATCH" emptyHint="No expressions" />
                <Group title="CALL STACK" emptyHint="Not debugging" />
                <Group title="BREAKPOINTS" emptyHint="No breakpoints" />
            </div>
        </div>
    );
};

const Group: React.FC<{ title: string; emptyHint: string }> = ({ title, emptyHint }) => (
    <div style={{ marginBottom: 12 }}>
        <div style={{ fontSize: 11, fontWeight: 'bold', opacity: 0.8, background: 'rgba(255,255,255,0.05)', padding: '4px 8px' }}>
            {title}
        </div>
        <div style={{ padding: '4px 10px', opacity: 0.5, fontSize: 11 }}>{emptyHint}</div>
    </div>
);

const ctrl: React.CSSProperties = {
    flex: 1, background: 'var(--vscode-button-background, #444)', color: 'white',
    border: 'none', padding: 6, cursor: 'pointer', borderRadius: 2,
};

const ctrlStop: React.CSSProperties = { ...ctrl, background: '#a1260d' };

const icon: React.CSSProperties = { fontFamily: 'codicon', fontStyle: 'normal' };

export default DebugView;
