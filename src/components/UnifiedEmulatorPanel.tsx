import React from 'react';
import { useStore } from '../store';
import EmulatorPanel from './EmulatorPanel';
import IPhoneEmulatorPanel from './IPhoneEmulatorPanel';
import IPhoneMirrorPanel from './IPhoneMirrorPanel';
import MobileToolchainPanel from './MobileToolchainPanel';
import GradleToolsPanel from './android/GradleToolsPanel';

/**
 * Unified Emulator Panel — side-by-side mobile dev in the IDE.
 * iPhone uses one headless CoreSimulator session (pauses when another tab is selected).
 */
const UnifiedEmulatorPanel: React.FC = () => {
    const activeEmulator = useStore(state => state.emulatorPanelPosition);

    return (
        <div
            className="emulator-panel-container"
            style={{
                display: 'flex',
                flexDirection: 'column',
                flex: '1 1 auto',
                minHeight: 0,
                height: '100%',
                background: 'var(--vscode-editor-background)',
                overflow: 'hidden'
            }}
        >
            {/* Emulator type switcher */}
            <div style={{
                display: 'flex',
                borderBottom: '1px solid var(--vscode-panel-border)',
                background: 'var(--vscode-sideBar-background)',
                flexShrink: 0
            }}>
                <button
                    onClick={() => useStore.getState().setEmulatorPanelPosition('android')}
                    style={{
                        flex: 1,
                        padding: '6px 10px',
                        fontSize: '10px',
                        fontWeight: 600,
                        background: activeEmulator === 'android' 
? 'var(--vscode-button-background)' 
: 'transparent',
                        color: activeEmulator === 'android'
? 'var(--vscode-button-foreground)'
: 'var(--vscode-descriptionForeground)',
                        border: 'none',
                        borderBottom: activeEmulator === 'android'
? '2px solid var(--vscode-button-background)'
: '2px solid transparent',
                        cursor: 'pointer',
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'center',
                        gap: '4px'
                    }}
                >
                    Android
                </button>
                <button
                    onClick={() => useStore.getState().setEmulatorPanelPosition('iphone')}
                    style={{
                        flex: 1,
                        padding: '6px 10px',
                        fontSize: '10px',
                        fontWeight: 600,
                        background: activeEmulator === 'iphone'
? 'var(--vscode-button-background)'
: 'transparent',
                        color: activeEmulator === 'iphone'
? 'var(--vscode-button-foreground)'
: 'var(--vscode-descriptionForeground)',
                        border: 'none',
                        borderBottom: activeEmulator === 'iphone'
? '2px solid var(--vscode-button-background)'
: '2px solid transparent',
                        cursor: 'pointer',
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'center',
                        gap: '4px'
                    }}
                >
                     iPhone
                </button>
                <button
                    onClick={() => useStore.getState().setEmulatorPanelPosition('device')}
                    style={{
                        flex: 1,
                        padding: '6px 10px',
                        fontSize: '10px',
                        fontWeight: 600,
                        background: activeEmulator === 'device'
? 'var(--vscode-button-background)'
: 'transparent',
                        color: activeEmulator === 'device'
? 'var(--vscode-button-foreground)'
: 'var(--vscode-descriptionForeground)',
                        border: 'none',
                        borderBottom: activeEmulator === 'device'
? '2px solid var(--vscode-button-background)'
: '2px solid transparent',
                        cursor: 'pointer',
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'center',
                        gap: '4px'
                    }}
                >
                     Device
                </button>
                <button
                    onClick={() => useStore.getState().setEmulatorPanelPosition('toolchain')}
                    style={{
                        flex: 1,
                        padding: '6px 10px',
                        fontSize: '10px',
                        fontWeight: 600,
                        background: activeEmulator === 'toolchain'
? 'var(--vscode-button-background)'
: 'transparent',
                        color: activeEmulator === 'toolchain'
? 'var(--vscode-button-foreground)'
: 'var(--vscode-descriptionForeground)',
                        border: 'none',
                        borderBottom: activeEmulator === 'toolchain'
? '2px solid var(--vscode-button-background)'
: '2px solid transparent',
                        cursor: 'pointer',
                    }}
                >
                    Toolchain
                </button>
                <button
                    onClick={() => useStore.getState().setEmulatorPanelPosition('gradle')}
                    style={{
                        flex: 1,
                        padding: '6px 10px',
                        fontSize: '10px',
                        fontWeight: 600,
                        background: activeEmulator === 'gradle'
? 'var(--vscode-button-background)'
: 'transparent',
                        color: activeEmulator === 'gradle'
? 'var(--vscode-button-foreground)'
: 'var(--vscode-descriptionForeground)',
                        border: 'none',
                        borderBottom: activeEmulator === 'gradle'
? '2px solid var(--vscode-button-background)'
: '2px solid transparent',
                        cursor: 'pointer',
                    }}
                >
                    Gradle
                </button>
            </div>

            {/* Emulator content */}
            <div style={{ flex: '1 1 auto', minHeight: 0, overflow: 'hidden', display: 'flex', flexDirection: 'column' }}>
                {activeEmulator === 'android' && <EmulatorPanel />}
                {activeEmulator === 'iphone' && <IPhoneEmulatorPanel />}
                {activeEmulator === 'device' && <IPhoneMirrorPanel />}
                {activeEmulator === 'toolchain' && <MobileToolchainPanel />}
                {activeEmulator === 'gradle' && <GradleToolsPanel />}
            </div>
        </div>
    );
};

export default UnifiedEmulatorPanel;
