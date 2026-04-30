import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useStore } from '../store';
import { EmulatorPreview } from './EmulatorPreview';

interface AVD {
    name: string;
    device: string;
    target: string;
    abi: string;
}

interface RunningEmulator {
    device_id: string;
    avd_name: string;
    port: number;
    status: string;
}

const EmulatorPanel: React.FC = () => {
    const activeDevice = useStore(state => state.activeDevice);
    const [availableAvds, setAvailableAvds] = useState<AVD[]>([]);
    const [runningEmulators, setRunningEmulators] = useState<RunningEmulator[]>([]);
    const [streamStarted, setStreamStarted] = useState(false);
    const [isLoading, setIsLoading] = useState(false);
    const [spawnStatus, setSpawnStatus] = useState<string>('');

    // Load available AVDs on mount
    useEffect(() => {
        loadAvailableAvds();
        loadRunningEmulators();
    }, []);

    // Auto-refresh running emulators
    useEffect(() => {
        const interval = setInterval(() => {
            loadRunningEmulators();
        }, 5000); // Check every 5 seconds

        return () => clearInterval(interval);
    }, []);

    const loadAvailableAvds = async () => {
        try {
            const avds = await invoke<AVD[]>('list_available_avds');
            setAvailableAvds(avds);
        } catch (err) {
            console.error('Failed to load AVDs:', err);
            setAvailableAvds([]);
        }
    };

    const loadRunningEmulators = async () => {
        try {
            const emulators = await invoke<RunningEmulator[]>('list_running_emulators');
            setRunningEmulators(emulators);
            
            // Auto-select first running emulator if none selected
            if (emulators.length > 0 && !activeDevice) {
                useStore.getState().setActiveDevice(emulators[0].device_id);
            }
        } catch (err) {
            console.error('Failed to load running emulators:', err);
        }
    };

    const handleSpawnEmulator = async (avdName: string) => {
        setIsLoading(true);
        setSpawnStatus(`Starting "${avdName}"... This may take 30-60 seconds`);
        
        try {
            const result = await invoke<string>('spawn_emulator_by_name', { avdName });
            setSpawnStatus(result);
            
            // Refresh emulator list after spawn
            setTimeout(() => {
                loadRunningEmulators();
            }, 5000);
        } catch (err) {
            setSpawnStatus(`Error: ${err}`);
        } finally {
            setIsLoading(false);
        }
    };

    const handleStartStream = async (deviceId: string) => {
        try {
            await invoke<string>('start_emulator_stream', { deviceId });
            useStore.getState().setActiveDevice(deviceId);
            setStreamStarted(true);
        } catch (err) {
            console.error('Failed to start stream:', err);
        }
    };

    if (!activeDevice) {
        return (
            <div style={{ display: 'flex', flexDirection: 'column', height: '100%', background: 'var(--vscode-editor-background)', overflow: 'auto' }}>
                {/* Available AVDs Section */}
                <div style={{ padding: '12px', borderBottom: '1px solid var(--vscode-panel-border)' }}>
                    <h3 style={{ fontSize: '11px', fontWeight: 600, textTransform: 'uppercase', marginBottom: '8px', color: 'var(--vscode-sideBar-foreground)' }}>
                        Available Virtual Devices
                    </h3>
                    
                    {availableAvds.length === 0 ? (
                        <div style={{ fontSize: '11px', opacity: 0.5, padding: '8px 0' }}>
                            No AVDs found. Create one in Android Studio or via avdmanager.
                        </div>
                    ) : (
                        <div style={{ display: 'flex', flexDirection: 'column', gap: '6px' }}>
                            {availableAvds.map((avd) => (
                                <div 
                                    key={avd.name}
                                    style={{ 
                                        padding: '8px', 
                                        background: 'var(--vscode-list-hoverBackground)', 
                                        borderRadius: '4px',
                                        border: '1px solid var(--vscode-panel-border)'
                                    }}
                                >
                                    <div style={{ fontWeight: 600, fontSize: '12px', marginBottom: '4px' }}>{avd.name}</div>
                                    <div style={{ fontSize: '10px', opacity: 0.6, marginBottom: '6px' }}>
                                        {avd.target} • {avd.abi} • {avd.device}
                                    </div>
                                    <button
                                        onClick={() => handleSpawnEmulator(avd.name)}
                                        disabled={isLoading}
                                        style={{
                                            width: '100%',
                                            padding: '4px 8px',
                                            fontSize: '11px',
                                            background: isLoading ? 'var(--vscode-button-secondaryBackground)' : 'var(--vscode-button-background)',
                                            color: 'var(--vscode-button-foreground)',
                                            border: 'none',
                                            borderRadius: '2px',
                                            cursor: isLoading ? 'not-allowed' : 'pointer',
                                            opacity: isLoading ? 0.6 : 1
                                        }}
                                    >
                                        {isLoading ? 'Starting...' : '▶ Start Emulator'}
                                    </button>
                                </div>
                            ))}
                        </div>
                    )}
                    
                    {spawnStatus && (
                        <div style={{ 
                            marginTop: '12px', 
                            padding: '8px', 
                            fontSize: '10px', 
                            background: 'var(--vscode-textBlockQuote-background)',
                            borderRadius: '4px',
                            borderLeft: '3px solid var(--vscode-progressBar-background)'
                        }}>
                            {spawnStatus}
                        </div>
                    )}
                </div>

                {/* Running Emulators Section */}
                <div style={{ padding: '12px', flex: 1 }}>
                    <h3 style={{ fontSize: '11px', fontWeight: 600, textTransform: 'uppercase', marginBottom: '8px', color: 'var(--vscode-sideBar-foreground)' }}>
                        Running Emulators
                    </h3>
                    
                    {runningEmulators.length === 0 ? (
                        <div style={{ fontSize: '11px', opacity: 0.5, padding: '8px 0' }}>
                            No running emulators. Start one from the list above.
                        </div>
                    ) : (
                        <div style={{ display: 'flex', flexDirection: 'column', gap: '6px' }}>
                            {runningEmulators.map((emu) => (
                                <div 
                                    key={emu.device_id}
                                    style={{ 
                                        padding: '8px', 
                                        background: 'var(--vscode-list-hoverBackground)', 
                                        borderRadius: '4px',
                                        border: '1px solid var(--vscode-panel-border)',
                                        display: 'flex',
                                        alignItems: 'center',
                                        justifyContent: 'space-between'
                                    }}
                                >
                                    <div>
                                        <div style={{ fontWeight: 600, fontSize: '12px' }}>{emu.device_id}</div>
                                        <div style={{ fontSize: '10px', opacity: 0.6 }}>{emu.avd_name || 'Unknown AVD'}</div>
                                    </div>
                                    <button
                                        onClick={() => handleStartStream(emu.device_id)}
                                        style={{
                                            padding: '4px 8px',
                                            fontSize: '10px',
                                            background: 'var(--vscode-button-background)',
                                            color: 'var(--vscode-button-foreground)',
                                            border: 'none',
                                            borderRadius: '2px',
                                            cursor: 'pointer'
                                        }}
                                    >
                                        📺 View
                                    </button>
                                </div>
                            ))}
                        </div>
                    )}
                </div>

                {/* Footer */}
                <div style={{ 
                    position: 'absolute', 
                    bottom: 0, 
                    left: 0, 
                    right: 0, 
                    padding: '8px 12px', 
                    fontSize: '10px', 
                    opacity: 0.5, 
                    borderTop: '1px solid var(--vscode-panel-border)',
                    background: 'var(--vscode-editor-background)'
                }}>
                    💡 Tip: Install Android SDK Platform Tools to spawn emulators directly
                </div>
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
