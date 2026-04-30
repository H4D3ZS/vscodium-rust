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
    const setActiveDevice = useStore(state => state.setActiveDevice);
    
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
        }, 5000);

        return () => clearInterval(interval);
    }, []);

    // Auto-start stream when device is selected
    useEffect(() => {
        if (activeDevice && !streamStarted) {
            handleStartStream(activeDevice);
        }
    }, [activeDevice]);

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
                setActiveDevice(emulators[0].device_id);
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
            setActiveDevice(deviceId);
            setStreamStarted(true);
        } catch (err) {
            console.error('Failed to start stream:', err);
        }
    };

    // No device selected - show AVD list and running emulators
    if (!activeDevice) {
        return (
            <div style={{ 
                display: 'flex', 
                flexDirection: 'column', 
                height: '100%', 
                background: 'var(--vscode-editor-background)',
                overflow: 'auto',
                padding: '12px'
            }}>
                {/* Available AVDs Section */}
                <div style={{ marginBottom: '16px' }}>
                    <h3 style={{ 
                        fontSize: '11px', 
                        fontWeight: 600, 
                        textTransform: 'uppercase', 
                        marginBottom: '8px',
                        color: 'var(--vscode-sideBar-foreground)',
                        display: 'flex',
                        alignItems: 'center',
                        gap: '6px'
                    }}>
                        📱 Available Virtual Devices
                    </h3>
                    
                    {availableAvds.length === 0 ? (
                        <div style={{ 
                            fontSize: '11px', 
                            opacity: 0.5, 
                            padding: '12px',
                            background: 'var(--vscode-textBlockQuote-background)',
                            borderRadius: '4px'
                        }}>
                            No AVDs found. Create one first:
                            <br />
                            <code style={{ display: 'block', marginTop: '8px', padding: '8px', background: 'var(--vscode-editor-background)', borderRadius: '4px' }}>
                                avdmanager create avd -n "Pixel_4" -k "system-images;android-34;google_apis;x86_64" -d "pixel_4"
                            </code>
                        </div>
                    ) : (
                        <div style={{ display: 'flex', flexDirection: 'column', gap: '6px' }}>
                            {availableAvds.map((avd) => (
                                <div 
                                    key={avd.name}
                                    style={{ 
                                        padding: '10px', 
                                        background: 'var(--vscode-list-hoverBackground)', 
                                        borderRadius: '4px',
                                        border: '1px solid var(--vscode-panel-border)'
                                    }}
                                >
                                    <div style={{ fontWeight: 600, fontSize: '12px', marginBottom: '4px' }}>{avd.name}</div>
                                    <div style={{ fontSize: '10px', opacity: 0.6, marginBottom: '8px' }}>
                                        {avd.target || 'Unknown'} • {avd.abi || 'Unknown'}
                                    </div>
                                    <button
                                        onClick={() => handleSpawnEmulator(avd.name)}
                                        disabled={isLoading}
                                        style={{
                                            width: '100%',
                                            padding: '6px 10px',
                                            fontSize: '11px',
                                            fontWeight: 500,
                                            background: isLoading ? 'var(--vscode-button-secondaryBackground)' : 'var(--vscode-button-background)',
                                            color: 'var(--vscode-button-foreground)',
                                            border: 'none',
                                            borderRadius: '3px',
                                            cursor: isLoading ? 'not-allowed' : 'pointer',
                                            opacity: isLoading ? 0.6 : 1
                                        }}
                                    >
                                        {isLoading ? '⏳ Starting...' : '▶ Start Emulator'}
                                    </button>
                                </div>
                            ))}
                        </div>
                    )}
                    
                    {spawnStatus && (
                        <div style={{ 
                            marginTop: '12px', 
                            padding: '10px', 
                            fontSize: '11px', 
                            background: 'var(--vscode-textBlockQuote-background)',
                            borderRadius: '4px',
                            borderLeft: '3px solid var(--vscode-progressBar-background)'
                        }}>
                            {spawnStatus}
                        </div>
                    )}
                </div>

                {/* Running Emulators Section */}
                <div style={{ flex: 1 }}>
                    <h3 style={{ 
                        fontSize: '11px', 
                        fontWeight: 600, 
                        textTransform: 'uppercase', 
                        marginBottom: '8px',
                        color: 'var(--vscode-sideBar-foreground)',
                        display: 'flex',
                        alignItems: 'center',
                        gap: '6px'
                    }}>
                        🔄 Running Emulators
                    </h3>
                    
                    {runningEmulators.length === 0 ? (
                        <div style={{ 
                            fontSize: '11px', 
                            opacity: 0.5, 
                            padding: '12px',
                            background: 'var(--vscode-textBlockQuote-background)',
                            borderRadius: '4px'
                        }}>
                            No running emulators. Start one from the list above.
                        </div>
                    ) : (
                        <div style={{ display: 'flex', flexDirection: 'column', gap: '6px' }}>
                            {runningEmulators.map((emu) => (
                                <div 
                                    key={emu.device_id}
                                    style={{ 
                                        padding: '10px', 
                                        background: 'var(--vscode-list-hoverBackground)', 
                                        borderRadius: '4px',
                                        border: '1px solid var(--vscode-panel-border)',
                                        display: 'flex',
                                        alignItems: 'center',
                                        justifyContent: 'space-between'
                                    }}
                                >
                                    <div style={{ flex: 1 }}>
                                        <div style={{ fontWeight: 600, fontSize: '12px' }}>{emu.device_id}</div>
                                        <div style={{ fontSize: '10px', opacity: 0.6 }}>{emu.avd_name || 'Unknown AVD'}</div>
                                    </div>
                                    <button
                                        onClick={() => {
                                            setActiveDevice(emu.device_id);
                                        }}
                                        style={{
                                            padding: '6px 12px',
                                            fontSize: '11px',
                                            fontWeight: 500,
                                            background: 'var(--vscode-button-background)',
                                            color: 'var(--vscode-button-foreground)',
                                            border: 'none',
                                            borderRadius: '3px',
                                            cursor: 'pointer',
                                            marginLeft: '8px'
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
                    marginTop: 'auto',
                    padding: '10px', 
                    fontSize: '10px', 
                    opacity: 0.5, 
                    borderTop: '1px solid var(--vscode-panel-border)',
                    background: 'var(--vscode-editor-background)'
                }}>
                    💡 Tip: Install Android SDK Platform Tools to manage emulators
                </div>
            </div>
        );
    }

    // Device selected - show emulator stream
    return (
        <div style={{ width: '100%', height: '100%', display: 'flex', flexDirection: 'column', background: 'var(--vscode-editor-background)', overflow: 'hidden' }}>
            {/* Header */}
            <div style={{ width: '100%', padding: '10px 12px', background: 'var(--vscode-panel-background)', borderBottom: '1px solid var(--vscode-panel-border)', display: 'flex', alignItems: 'center', gap: '8px' }}>
                <div style={{ width: '8px', height: '8px', borderRadius: '50%', background: '#4ec9b0', boxShadow: '0 0 8px #4ec9b0' }}></div>
                <span style={{ fontSize: '11px', fontWeight: 600, color: 'var(--vscode-sideBar-foreground)' }}>{activeDevice}</span>
                <span style={{ fontSize: '10px', opacity: 0.5, marginLeft: 'auto', background: 'var(--vscode-badge-background)', padding: '2px 6px', borderRadius: '3px' }}>
                    📡 Live Stream
                </span>
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

            {/* Footer with device info */}
            <div style={{ 
                padding: '8px 12px', 
                fontSize: '10px', 
                opacity: 0.5, 
                borderTop: '1px solid var(--vscode-panel-border)',
                display: 'flex',
                justifyContent: 'space-between'
            }}>
                <span>Click on emulator screen to interact</span>
                <button
                    onClick={() => setActiveDevice('')}
                    style={{
                        padding: '4px 8px',
                        fontSize: '10px',
                        background: 'var(--vscode-button-secondaryBackground)',
                        color: 'var(--vscode-button-secondaryForeground)',
                        border: '1px solid var(--vscode-panel-border)',
                        borderRadius: '3px',
                        cursor: 'pointer'
                    }}
                >
                    ✕ Close Stream
                </button>
            </div>
        </div>
    );
};

export default EmulatorPanel;
