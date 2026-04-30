import React, { useState } from 'react';
import { useStore } from '../store';
import EmulatorPanel from './EmulatorPanel';
import IPhoneEmulatorPanel from './IPhoneEmulatorPanel';

/**
 * Unified Emulator Panel
 * Supports both Android and iPhone emulators
 * Integrated in right sidebar with AIRI
 */
const UnifiedEmulatorPanel: React.FC = () => {
    const [activeEmulator, setActiveEmulator] = useState<'android' | 'iphone'>('android');

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
                padding: '6px 10px',
                borderBottom: '1px solid var(--vscode-panel-border)',
                background: 'var(--vscode-sideBar-background)',
                flexShrink: 0
            }}>
                <div style={{ display: 'flex', gap: '6px' }}>
                    <button
                        onClick={() => setActiveEmulator('android')}
                        style={{
                            padding: '3px 10px',
                            fontSize: '10px',
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
                            gap: '4px'
                        }}
                    >
                        🤖 Android
                    </button>
                    <button
                        onClick={() => setActiveEmulator('iphone')}
                        style={{
                            padding: '3px 10px',
                            fontSize: '10px',
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
                            gap: '4px'
                        }}
                    >
                        🍎 iPhone
                    </button>
                </div>
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
