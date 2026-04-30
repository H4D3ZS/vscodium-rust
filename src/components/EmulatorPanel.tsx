import React, { useState, useEffect } from 'react';
import { useStore } from '../store';
import { EmulatorPreview } from './EmulatorPreview';

const EmulatorPanel: React.FC = () => {
    const activeDevice = useStore(state => state.activeDevice);
    const [streamStarted, setStreamStarted] = useState(false);

    useEffect(() => {
        if (activeDevice && !streamStarted) {
            console.log('📱 [EmulatorPanel] Device detected:', activeDevice);
            setStreamStarted(true);
        }
        
        if (!activeDevice) {
            setStreamStarted(false);
        }
    }, [activeDevice, streamStarted]);

    if (!activeDevice) {
        return (
            <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', height: '100%', color: 'var(--vscode-sideBar-foreground)', opacity: 0.6, fontSize: '13px', background: 'var(--vscode-editor-background)' }}>
                No active device detected. Start an emulator or connect a device via USB.
            </div>
        );
    }

    // Render live emulator stream
    return (
        <div style={{ width: '100%', height: '100%', display: 'flex', flexDirection: 'column', background: 'var(--vscode-editor-background)', overflow: 'hidden' }}>
            {/* Header */}
            <div style={{ width: '100%', padding: '8px 12px', background: 'var(--vscode-panel-background)', borderBottom: '1px solid var(--vscode-panel-border)', display: 'flex', alignItems: 'center', gap: '8px' }}>
                <div style={{ width: '8px', height: '8px', borderRadius: '50%', background: '#4ec9b0' }}></div>
                <span style={{ fontSize: '11px', fontWeight: 600, color: 'var(--vscode-sideBar-foreground)', opacity: 0.8 }}>{activeDevice}</span>
                <span style={{ fontSize: '10px', opacity: 0.5, marginLeft: 'auto' }}>Live Stream</span>
            </div>

            {/* Emulator Preview Component */}
            <div style={{ flex: 1, display: 'flex', alignItems: 'center', justifyContent: 'center', padding: '20px', overflow: 'auto' }}>
                <EmulatorPreview
                    streamUrl="ws://localhost:8989"
                    width={360}
                    height={640}
                    showFps={true}
                    showControls={true}
                />
            </div>

            {/* Footer with instructions */}
            <div style={{ padding: '8px 12px', fontSize: '10px', opacity: 0.5, borderTop: '1px solid var(--vscode-panel-border)' }}>
                Stream requires scrcpy backend. Run: scrcpy --no-display --tcpip=5555
            </div>
        </div>
    );
};

export default EmulatorPanel;
