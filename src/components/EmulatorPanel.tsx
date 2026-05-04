import React, { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useStore } from '../store';
import { ScrcpyEmbed, getScrcpyForDevice, stopAllScrcpyInstances } from '../utils/scrcpy-embed';

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
    const emulatorPosition = useStore(state => state.emulatorPanelPosition);
    const setEmulatorPosition = useStore(state => state.setEmulatorPanelPosition);
    
    const scrcpyContainerRef = useRef<HTMLDivElement>(null);
    const scrcpyInstance = useRef<ScrcpyEmbed | null>(null);

    const [availableAvds, setAvailableAvds] = useState<AVD[]>([]);
    const [runningEmulators, setRunningEmulators] = useState<RunningEmulator[]>([]);
    const [streamStarted, setStreamStarted] = useState(false);
    const [isLoading, setIsLoading] = useState(false);
    const [spawnStatus, setSpawnStatus] = useState<string>('');

    // Load available AVDs on mount
    useEffect(() => {
        loadAvailableAvds();
        if (isMacOS()) {
            loadAvailableIOSSimulators();
        }
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

    const loadAvailableIOSSimulators = async () => {
        // iOS simulators only on macOS
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

    const handleSpawnAndEmbed = async (avdName: string) => {
        setIsLoading(true);
        setSpawnStatus(`Spawning "${avdName}" and embedding in IDE...`);

        try {
            // First check if any emulator is already running
            const emulators = await invoke<RunningEmulator[]>('list_running_emulators');
            
            let deviceId: string;
            
            if (emulators.length > 0) {
                // Use existing running emulator
                deviceId = emulators[0].device_id;
                setSpawnStatus(`Using existing emulator: ${deviceId}`);
                setActiveDevice(deviceId);
            } else {
                // Spawn new emulator
                const result = await invoke<string>('spawn_emulator_headless', {
                    avdName,
                    port: 8989
                });

                setSpawnStatus(result);
                
                // Wait for emulator to boot
                setSpawnStatus('Waiting for emulator to boot (30-60 seconds)...');
                await new Promise(resolve => setTimeout(resolve, 30000));
                
                // Reload running emulators
                await loadRunningEmulators();
                
                if (activeDevice) {
                    deviceId = activeDevice;
                } else {
                    const updated = await invoke<RunningEmulator[]>('list_running_emulators');
                    deviceId = updated.length > 0 ? updated[0].device_id : 'emulator-5554';
                }
                
                setActiveDevice(deviceId);
            }

            setIsLoading(false);
            setSpawnStatus('Emulator ready!');
            
        } catch (err: any) {
            setSpawnStatus(`Error: ${err.message || err}`);
            setIsLoading(false);
        }
    };

    const handleSpawnEmulator = async (avdName: string) => {
        setIsLoading(true);
        setSpawnStatus(`Starting "${avdName}"... This may take 30-60 seconds`);
        
        try {
            const result = await invoke<string>('spawn_emulator_by_name', { avdName });
            setSpawnStatus(result);
            
            // Refresh emulator list after spawn
            setTimeout(async () => {
                await loadRunningEmulators();
            }, 5000);
        } catch (err) {
            setSpawnStatus(`Error: ${err}`);
        } finally {
            setIsLoading(false);
        }
    };

    const handleStartStream = async (deviceId: string) => {
        try {
            setActiveDevice(deviceId);
            setStreamStarted(true);
            
            // Start embedded scrcpy stream
            if (scrcpyContainerRef.current) {
                scrcpyInstance.current = getScrcpyForDevice(deviceId);
                await scrcpyInstance.current.start(scrcpyContainerRef.current);
            }
        } catch (err) {
            console.error('Failed to start stream:', err);
        }
    };

    const handleLaunchEmulator = async (avdName: string) => {
        setIsLoading(true);
        setSpawnStatus(`Launching "${avdName}"...`);

        try {
            // First check if emulator is already running
            const emulators = await invoke<RunningEmulator[]>('list_running_emulators');
            
            if (emulators.length === 0) {
                // Spawn emulator
                const result = await invoke<string>('spawn_emulator_by_name', { avdName });
                setSpawnStatus(result);
                
                // Wait for boot
                setSpawnStatus('Waiting for emulator to boot (30-60 seconds)...');
                await new Promise(resolve => setTimeout(resolve, 30000));
                
                // Reload running emulators
                await loadRunningEmulators();
            }
            
            // Get device ID
            const updated = await invoke<RunningEmulator[]>('list_running_emulators');
            const deviceId = updated.length > 0 ? updated[0].device_id : 'emulator-5554';
            
            setSpawnStatus(`Starting screen capture: ${deviceId}...`);
            setActiveDevice(deviceId);
            
            // Start scrcpy stream
            await handleStartStream(deviceId);
            
            setIsLoading(false);
            setSpawnStatus('Emulator screen active!');
            
        } catch (err: any) {
            setSpawnStatus(`Error: ${err.message || err}`);
            setIsLoading(false);
        }
    };

    const handleStopStream = async () => {
        if (scrcpyInstance.current) {
            await scrcpyInstance.current.stop();
            scrcpyInstance.current = null;
        }
        setStreamStarted(false);
        setActiveDevice('');
    };

    const togglePosition = () => {
        const newPosition = emulatorPosition === 'left' ? 'right' : 
                           emulatorPosition === 'right' ? 'hidden' : 'left';
        setEmulatorPosition(newPosition);
    };

    const moveToRightSidebar = () => {
        setEmulatorPosition('right');
    };

    // Helper functions
    function isMacOS(): boolean {
        return navigator.userAgent.toLowerCase().includes('mac');
    }

    // Don't render if hidden
    if (emulatorPosition === 'hidden') {
        return (
            <div 
                onClick={togglePosition}
                style={{
                    position: 'fixed',
                    bottom: '20px',
                    right: '20px',
                    padding: '10px 15px',
                    background: 'var(--vscode-button-background)',
                    color: 'var(--vscode-button-foreground)',
                    borderRadius: '5px',
                    cursor: 'pointer',
                    fontSize: '12px',
                    zIndex: 1000,
                    boxShadow: '0 2px 8px rgba(0,0,0,0.3)'
                }}
                title="Show emulator panel"
            >
                📱 Show Emulator
            </div>
        );
    }

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
                {/* Header with position toggle */}
                <div style={{
                    display: 'flex',
                    justifyContent: 'space-between',
                    alignItems: 'center',
                    marginBottom: '16px',
                    paddingBottom: '8px',
                    borderBottom: '1px solid var(--vscode-panel-border)'
                }}>
                    <h3 style={{
                        fontSize: '11px',
                        fontWeight: 600,
                        textTransform: 'uppercase',
                        color: 'var(--vscode-sideBar-foreground)',
                        margin: 0
                    }}>
                        📱 Mobile Emulators
                    </h3>
                    <button
                        onClick={togglePosition}
                        style={{
                            padding: '4px 8px',
                            fontSize: '10px',
                            background: 'var(--vscode-button-secondaryBackground)',
                            color: 'var(--vscode-button-secondaryForeground)',
                            border: '1px solid var(--vscode-panel-border)',
                            borderRadius: '3px',
                            cursor: 'pointer'
                        }}
                        title={`Move panel to ${emulatorPosition === 'left' ? 'right' : 'hide'}`}
                    >
                        {emulatorPosition === 'left' ? '➡️ Right' : emulatorPosition === 'right' ? '❌ Hide' : '📱 Show'}
                    </button>
                </div>

                {/* Available AVDs Section */}
                <div style={{ marginBottom: '16px' }}>
                    <h4 style={{
                        fontSize: '10px',
                        fontWeight: 600,
                        textTransform: 'uppercase',
                        marginBottom: '8px',
                        color: 'var(--vscode-sideBar-foreground)',
                        display: 'flex',
                        alignItems: 'center',
                        gap: '6px'
                    }}>
                        🟢 Available Virtual Devices
                    </h4>

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
                            <code style={{ display: 'block', marginTop: '8px', padding: '8px', background: 'var(--vscode-editor-background)', borderRadius: '4px', fontSize: '10px' }}>
                                avdmanager create avd -n "Pixel_4" -k "system-images;android-34;google_apis_playstore;x86_64" -d "pixel_4"
                            </code>
                        </div>
                    ) : (
                        <div style={{ display: 'flex', flexDirection: 'column', gap: '6px' }}>
                            {availableAvds.map((avd) => (
                                <div key={avd.name} style={{
                                    padding: '10px',
                                    background: 'var(--vscode-list-hoverBackground)',
                                    borderRadius: '4px',
                                    border: '1px solid var(--vscode-panel-border)'
                                }}>
                                    <div style={{ fontWeight: 600, fontSize: '12px', marginBottom: '4px' }}>{avd.name}</div>
                                    <div style={{ fontSize: '10px', opacity: 0.6, marginBottom: '8px' }}>
                                        {avd.target || 'Unknown'} • {avd.abi || 'Unknown'}
                                    </div>
                                    <div style={{ display: 'flex', gap: '6px', marginBottom: '8px' }}>
                                        <button
                                            onClick={() => handleLaunchEmulator(avd.name)}
                                            disabled={isLoading}
                                            style={{
                                                flex: 1,
                                                padding: '6px 10px',
                                                fontSize: '11px',
                                                fontWeight: 600,
                                                background: isLoading ? 'var(--vscode-button-secondaryBackground)' : '#0e639c',
                                                color: 'white',
                                                border: 'none',
                                                borderRadius: '3px',
                                                cursor: isLoading ? 'not-allowed' : 'pointer',
                                                opacity: isLoading ? 0.6 : 1
                                            }}
                                            title="Launch emulator and embed in IDE"
                                        >
                                            {isLoading ? '⏳ Launching...' : '🚀 Launch & Embed'}
                                        </button>
                                        <button
                                            onClick={() => handleSpawnEmulator(avd.name)}
                                            disabled={isLoading}
                                            style={{
                                                flex: 1,
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
                                            title="Start emulator in external window"
                                        >
                                            {isLoading ? '⏳ Starting...' : '▶ External'}
                                        </button>
                                    </div>
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
                    <h4 style={{
                        fontSize: '10px',
                        fontWeight: 600,
                        textTransform: 'uppercase',
                        marginBottom: '8px',
                        color: 'var(--vscode-sideBar-foreground)',
                        display: 'flex',
                        alignItems: 'center',
                        gap: '6px'
                    }}>
                        🔵 Running Emulators
                    </h4>

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

    // Device selected - show embedded scrcpy stream
    return (
        <div style={{ width: '100%', height: '100%', display: 'flex', flexDirection: 'column', background: 'var(--vscode-editor-background)', overflow: 'hidden' }}>
            {/* Header */}
            <div style={{ width: '100%', padding: '10px 12px', background: 'var(--vscode-panel-background)', borderBottom: '1px solid var(--vscode-panel-border)', display: 'flex', alignItems: 'center', gap: '8px' }}>
                <div style={{ width: '8px', height: '8px', borderRadius: '50%', background: '#4ec9b0', boxShadow: '0 0 8px #4ec9b0' }}></div>
                <span style={{ fontSize: '11px', fontWeight: 600, color: 'var(--vscode-sideBar-foreground)' }}>{activeDevice}</span>
                <span style={{ fontSize: '10px', opacity: 0.5, marginLeft: 'auto', background: 'var(--vscode-badge-background)', padding: '2px 6px', borderRadius: '3px' }}>
                    📡 Embedded
                </span>
                <button
                    onClick={togglePosition}
                    style={{
                        padding: '2px 6px',
                        fontSize: '10px',
                        background: 'transparent',
                        color: 'var(--vscode-descriptionForeground)',
                        border: '1px solid var(--vscode-panel-border)',
                        borderRadius: '3px',
                        cursor: 'pointer',
                        marginLeft: '4px'
                    }}
                    title="Move panel"
                >
                    {emulatorPosition === 'left' ? '➡️' : '⬅️'}
                </button>
            </div>

            {/* Embedded scrcpy Stream */}
            <div 
                ref={scrcpyContainerRef}
                style={{ 
                    flex: 1, 
                    background: '#000',
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center'
                }}
            >
                <div style={{ color: '#888', fontSize: '12px' }}>
                    {streamStarted ? 'Loading emulator stream...' : 'Starting stream...'}
                </div>
            </div>

            {/* Footer with controls */}
            <div style={{ 
                padding: '8px 12px', 
                fontSize: '10px', 
                opacity: 0.5, 
                borderTop: '1px solid var(--vscode-panel-border)',
                display: 'flex',
                justifyContent: 'space-between',
                alignItems: 'center'
            }}>
                <span>Click on emulator to interact</span>
                <button
                    onClick={handleStopStream}
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
