import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

const DebugView: React.FC = () => {
    const [isDebugging, setIsDebugging] = useState(false);

    const handleStart = async () => {
        try {
            // In a real scenario, we'd prompt for adapter path or use a default
            await invoke('debug_start', { adapterPath: 'lldb-vscode' });
            setIsDebugging(true);
        } catch (e) {
            alert(`Debug start failed: ${e}`);
        }
    };

    const handleStop = async () => {
        await invoke('debug_stop');
        setIsDebugging(false);
    };

    return (
        <div className="debug-view" style={{ display: 'flex', flexDirection: 'column', height: '100%', padding: '10px' }}>
            <div style={{ marginBottom: '15px' }}>
                {!isDebugging ? (
                    <button 
                        onClick={handleStart}
                        style={{ width: '100%', background: 'var(--vscode-button-background)', color: 'white', border: 'none', padding: '6px', cursor: 'pointer', borderRadius: '2px', fontSize: '12px' }}>
                        Start Debugging
                    </button>
                ) : (
                    <div style={{ display: 'flex', gap: '4px' }}>
                        <button style={{ flex: 1, background: '#388a34', color: 'white', border: 'none', padding: '6px', cursor: 'pointer', borderRadius: '2px' }}><i className="codicon codicon-debug-continue"></i></button>
                        <button style={{ flex: 1, background: '#444', color: 'white', border: 'none', padding: '6px', cursor: 'pointer', borderRadius: '2px' }}><i className="codicon codicon-debug-step-over"></i></button>
                        <button style={{ flex: 1, background: '#444', color: 'white', border: 'none', padding: '6px', cursor: 'pointer', borderRadius: '2px' }}><i className="codicon codicon-debug-step-into"></i></button>
                        <button onClick={handleStop} style={{ flex: 1, background: '#a1260d', color: 'white', border: 'none', padding: '6px', cursor: 'pointer', borderRadius: '2px' }}><i className="codicon codicon-debug-stop"></i></button>
                    </div>
                )}
            </div>

            <div style={{ flex: 1, overflowY: 'auto' }}>
                <div style={{ marginBottom: '12px' }}>
                    <div style={{ fontSize: '11px', fontWeight: 'bold', opacity: 0.8, background: 'rgba(255,255,255,0.05)', padding: '4px' }}>VARIABLES</div>
                    <div style={{ padding: '4px', opacity: 0.5, fontSize: '11px' }}>Not debugging</div>
                </div>
                <div style={{ marginBottom: '12px' }}>
                    <div style={{ fontSize: '11px', fontWeight: 'bold', opacity: 0.8, background: 'rgba(255,255,255,0.05)', padding: '4px' }}>WATCH</div>
                    <div style={{ padding: '4px', opacity: 0.5, fontSize: '11px' }}>No expressions</div>
                </div>
                <div style={{ marginBottom: '12px' }}>
                    <div style={{ fontSize: '11px', fontWeight: 'bold', opacity: 0.8, background: 'rgba(255,255,255,0.05)', padding: '4px' }}>CALL STACK</div>
                    <div style={{ padding: '4px', opacity: 0.5, fontSize: '11px' }}>Not debugging</div>
                </div>
                <div>
                    <div style={{ fontSize: '11px', fontWeight: 'bold', opacity: 0.8, background: 'rgba(255,255,255,0.05)', padding: '4px' }}>BREAKPOINTS</div>
                    <div style={{ padding: '4px', opacity: 0.5, fontSize: '11px' }}>No breakpoints</div>
                </div>
            </div>
        </div>
    );
};

export default DebugView;
