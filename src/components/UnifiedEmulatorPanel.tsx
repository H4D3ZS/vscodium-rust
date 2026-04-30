import React, { useState } from 'react';
import { useStore } from '../store';
import EmulatorPanel from './EmulatorPanel';
import IPhoneEmulatorPanel from './IPhoneEmulatorPanel';

/**
 * Unified Emulator Panel
 * Supports both Android and iPhone emulators
 * Can be shown/hidden independently from AIRI panel
 */
const UnifiedEmulatorPanel: React.FC = () => {
    const [activeEmulator, setActiveEmulator] = useState<'android' | 'iphone'>('android');
    const isEmulatorOpen = useStore(state => state.isEmulatorPanelOpen);
    const closeEmulatorPanel = useStore(state => state.closeEmulatorPanel);

    if (!isEmulatorOpen) {
        return null;
    }

    return (
        <div
            className="emulator-panel-container"
            style={{
                display: 'flex',
                flexDirection: 'column',
                height: '100%',
                background: 'var(--vscode-editor-background)',
                overflow: 'hidden'
            }}
        >
            {/* Header with emulator switcher */}
            <div style={{
                display: 'flex',
                justifyContent: 'space-between',
                alignItems: 'center',
                padding: '8px 12px',
                borderBottom: '1px solid var(--vscode-panel-border)',
                background: 'var(--vscode-sideBar-background)'
            }}>
                <div style={{ display: 'flex', gap: '8px' }}>
                    <button
                        onClick={() => setActiveEmulator('android')}
                        style={{
                            padding: '4px 12px',
                            fontSize: '11px',
                            fontWeight: 600,
                            background: activeEmulator === 'android' 
                                ? 'var(--vscode-button-background)' 
                                : 'var(--vscode-button-secondaryBackground)',
                            color: activeEmulator === 'android'
                                ? 'var(--vscode-button-foreground)'
                                : 'var(--vscode-descriptionForeground)',
                            border: '1px solid var(--vscode-panel-border)',
                            borderRadius: '3px',
                            cursor: 'pointer',
                            display: 'flex',
                            alignItems: 'center',
                            gap: '6px'
                        }}
                    >
                        🤖 Android
                    </button>
                    <button
                        onClick={() => setActiveEmulator('iphone')}
                        style={{
                            padding: '4px 12px',
                            fontSize: '11px',
                            fontWeight: 600,
                            background: activeEmulator === 'iphone'
                                ? 'var(--vscode-button-background)'
                                : 'var(--vscode-button-secondaryBackground)',
                            color: activeEmulator === 'iphone'
                                ? 'var(--vscode-button-foreground)'
                                : 'var(--vscode-descriptionForeground)',
                            border: '1px solid var(--vscode-panel-border)',
                            borderRadius: '3px',
                            cursor: 'pointer',
                            display: 'flex',
                            alignItems: 'center',
                            gap: '6px'
                        }}
                    >
                        🍎 iPhone
                    </button>
                </div>

                <button
                    onClick={closeEmulatorPanel}
                    style={{
                        padding: '2px 6px',
                        fontSize: '10px',
                        background: 'transparent',
                        color: 'var(--vscode-descriptionForeground)',
                        border: '1px solid var(--vscode-panel-border)',
                        borderRadius: '3px',
                        cursor: 'pointer'
                    }}
                    title="Close emulator panel"
                >
                    ✕
                </button>
            </div>

            {/* Emulator content */}
            <div style={{ flex: 1, overflow: 'hidden' }}>
                {activeEmulator === 'android' && <EmulatorPanel />}
                {activeEmulator === 'iphone' && <IPhoneEmulatorPanel />}
            </div>
        </div>
    );
};

export default UnifiedEmulatorPanel;
