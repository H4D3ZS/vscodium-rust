import React, { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useStore } from '../store';

const EmulatorPanel: React.FC = () => {
    const activeDevice = useStore(state => state.activeDevice);
    const [screenshot, setScreenshot] = useState<string | null>(null);
    const [error, setError] = useState<string | null>(null);
    const containerRef = useRef<HTMLDivElement>(null);
    const imgRef = useRef<HTMLImageElement>(null);

    useEffect(() => {
        if (!activeDevice) {
            setScreenshot(null);
            return;
        }

        let isMounted = true;
        const poll = async () => {
            if (!isMounted || !activeDevice) return;
            try {
                const b64 = await invoke<string>('get_emulator_screenshot', { deviceId: activeDevice });
                if (isMounted) {
                    setScreenshot(b64);
                    setError(null);
                }
            } catch (err) {
                if (isMounted) {
                    setError(String(err));
                }
            }
            if (isMounted) {
                setTimeout(poll, 250); // Poll every 250ms for performance/smoothness balance
            }
        };

        poll();
        return () => { isMounted = false; };
    }, [activeDevice]);

    const handleInteraction = async (e: React.MouseEvent) => {
        if (!activeDevice || !imgRef.current) return;

        const rect = imgRef.current.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = e.clientY - rect.top;

        // Scale coordinates to actual device resolution
        const scaleX = imgRef.current.naturalWidth / imgRef.current.clientWidth;
        const scaleY = imgRef.current.naturalHeight / imgRef.current.clientHeight;

        try {
            await invoke('emulator_tap', {
                deviceId: activeDevice,
                x: Math.round(x * scaleX),
                y: Math.round(y * scaleY)
            });
        } catch (err) {
            console.error('Tap failed:', err);
        }
    };

    if (!activeDevice) {
        return (
            <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', height: '100%', color: 'var(--vscode-sideBar-foreground)', opacity: 0.6, fontSize: '13px', background: 'var(--vscode-editor-background)' }}>
                No active device detected. Start an emulator or connect a device via USB.
            </div>
        );
    }

    const isIPhone = activeDevice?.includes("iPhone") || activeDevice?.includes("iOS");

    return (
        <div ref={containerRef} style={{ width: '100%', height: '100%', display: 'flex', flexDirection: 'column', alignItems: 'center', background: '#1a1a1a', overflow: 'hidden' }}>
            <div className="emulator-controls" style={{ width: '100%', padding: '4px 12px', background: 'var(--vscode-panel-background)', borderBottom: '1px solid var(--vscode-panel-border)', display: 'flex', alignItems: 'center', gap: '12px' }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: '6px' }}>
                    <div style={{ width: '8px', height: '8px', borderRadius: '50%', background: isIPhone ? '#3498db' : '#4ec9b0' }}></div>
                    <span style={{ fontSize: '11px', fontWeight: 600, color: 'var(--vscode-sideBar-foreground)' }}>{activeDevice}</span>
                </div>
                <div style={{ flex: 1 }}></div>
                <div style={{ display: 'flex', gap: '10px' }}>
                     {!isIPhone && <i className="codicon codicon-refresh" style={{ fontSize: '12px', cursor: 'pointer', opacity: 0.8 }} title="Refresh Stream" onClick={() => setScreenshot(null)}></i>}
                     <i className="codicon codicon-screen-full" style={{ fontSize: '12px', cursor: 'pointer', opacity: 0.8 }} title="Fit to Screen"></i>
                </div>
            </div>
            <div style={{ flex: 1, display: 'flex', alignItems: 'center', justifyContent: 'center', width: '100%', padding: '15px', position: 'relative', overflow: 'auto' }}>
                {isIPhone ? (
                     <div style={{ 
                        width: '280px', 
                        height: '580px', 
                        background: '#000', 
                        borderRadius: '40px', 
                        border: '8px solid #222', 
                        position: 'relative',
                        boxShadow: '0 0 50px rgba(0,0,0,0.8)',
                        display: 'flex',
                        flexDirection: 'column',
                        alignItems: 'center',
                        justifyContent: 'center',
                        gap: '20px'
                     }}>
                        <div style={{ width: '120px', height: '24px', background: '#222', borderRadius: '0 0 15px 15px', position: 'absolute', top: 0 }}></div>
                        <i className="codicon codicon-device-mobile" style={{ fontSize: '64px', color: '#fff', opacity: 0.2 }}></i>
                        <div style={{ color: '#fff', textAlign: 'center', padding: '0 40px' }}>
                            <div style={{ fontWeight: 600, fontSize: '16px', marginBottom: '8px' }}>Virtual iPhone (JB)</div>
                            <div style={{ fontSize: '12px', opacity: 0.6, color: '#3498db', fontWeight: 700 }}>READY</div>
                            <div style={{ fontSize: '11px', opacity: 0.4, marginTop: '20px' }}>Upload app builds via ADB/Bridge to initiate stream.</div>
                        </div>
                     </div>
                ) : screenshot ? (
                    <img 
                        ref={imgRef}
                        src={screenshot} 
                        onClick={handleInteraction}
                        style={{ 
                            maxHeight: '100%', 
                            maxWidth: '100%', 
                            cursor: 'pointer', 
                            boxShadow: '0 0 30px rgba(0,0,0,0.7)',
                            borderRadius: '8px',
                            border: '4px solid #333'
                        }} 
                        alt="emulator"
                    />
                ) : (
                    <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', gap: '10px' }}>
                        <div className="codicon codicon-loading codicon-modifier-spin" style={{ fontSize: '24px', color: 'var(--vscode-progressBar-background)' }}></div>
                        <div style={{ color: 'var(--vscode-sideBar-foreground)', opacity: 0.8, fontSize: '12px' }}>{error ? `Connection lost: ${error}` : 'Connecting to device stream...'}</div>
                    </div>
                )}
            </div>
        </div>
    );
};

export default EmulatorPanel;
