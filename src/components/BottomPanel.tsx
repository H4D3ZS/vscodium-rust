import React, { useState, useEffect } from 'react';
import { useStore } from '../store';
import EmulatorPanel from './EmulatorPanel';

const BottomPanel: React.FC = () => {
    const isOpen = useStore(state => state.isBottomPanelOpen);
    const activeTab = useStore(state => state.activePanelTab);
    const setActiveTab = useStore(state => state.setActivePanelTab);
    const backendPing = useStore(state => state.backendPing);
    const [pingResult, setPingResult] = useState<string>('');
    const [isPending, setIsPending] = useState(false);

    useEffect(() => {
        if (isOpen && activeTab === 'TERMINAL') {
            // Short delay to ensure DOM is ready
            setTimeout(() => {
                if ((window as any).rebindTerminal) {
                    (window as any).rebindTerminal();
                }
                
                const container = document.getElementById("terminal-container");
                if (container && container.childNodes.length === 0) {
                    if ((window as any).spawnTerminal) {
                        (window as any).spawnTerminal();
                    }
                }
            }, 50);
        }
    }, [isOpen, activeTab]);

    if (!isOpen) return null;

    const handlePing = async () => {
        setIsPending(true);
        const result = await backendPing();
        setPingResult(result);
        setIsPending(false);
        // Clear result after 3 seconds
        setTimeout(() => setPingResult(''), 3000);
    };

    return (
        <footer className="bottom-panel" id="bottom-panel" style={{ height: '300px', display: 'flex', flexDirection: 'column', background: 'var(--vscode-panel-background)', borderTop: '1px solid var(--vscode-panel-border)', zIndex: 10 }}>
            <div className="panel-header" style={{ display: 'flex', background: 'var(--vscode-panel-background)', borderBottom: '1px solid var(--vscode-panel-border)', alignItems: 'center', height: '35px' }}>
                <div className="panel-tabs" style={{ display: 'flex', height: '100%' }}>
                    <div 
                        className={`panel-tab ${activeTab === 'TERMINAL' ? 'active' : ''}`} 
                        onClick={() => setActiveTab('TERMINAL')}
                        style={{ 
                            padding: '0 16px', 
                            display: 'flex', 
                            alignItems: 'center', 
                            cursor: 'pointer',
                            fontSize: '11px',
                            borderBottom: activeTab === 'TERMINAL' ? '1px solid var(--vscode-panelTitle-activeBorder)' : 'none',
                            color: activeTab === 'TERMINAL' ? 'var(--vscode-panelTitle-activeForeground)' : 'var(--vscode-panelTitle-inactiveForeground)',
                            opacity: activeTab === 'TERMINAL' ? 1 : 0.7
                        }}
                    >
                        TERMINAL
                    </div>
                    <div 
                        className={`panel-tab ${activeTab === 'EMULATOR' ? 'active' : ''}`} 
                        onClick={() => setActiveTab('EMULATOR')}
                        style={{ 
                            padding: '0 16px', 
                            display: 'flex', 
                            alignItems: 'center', 
                            cursor: 'pointer',
                            fontSize: '11px',
                            borderBottom: activeTab === 'EMULATOR' ? '1px solid var(--vscode-panelTitle-activeBorder)' : 'none',
                            color: activeTab === 'EMULATOR' ? 'var(--vscode-panelTitle-activeForeground)' : 'var(--vscode-panelTitle-inactiveForeground)',
                            opacity: activeTab === 'EMULATOR' ? 1 : 0.7
                        }}
                    >
                        EMULATOR
                    </div>
                    <div className="panel-tab" style={{ padding: '0 16px', display: 'flex', alignItems: 'center', opacity: 0.5, fontSize: '11px' }}>OUTPUT</div>
                </div>

                <div style={{ marginLeft: '20px', fontSize: '11px', color: 'var(--vscode-sideBar-foreground)', opacity: 0.8, display: 'flex', alignItems: 'center', gap: '8px' }}>
                    <button
                        onClick={handlePing}
                        disabled={isPending}
                        style={{
                            background: 'var(--vscode-statusBar-background)',
                            color: 'white',
                            border: 'none',
                            padding: '2px 8px',
                            borderRadius: '2px',
                            cursor: 'pointer',
                            fontSize: '10px'
                        }}
                    >
                        {isPending ? 'Pinging...' : 'Ping'}
                    </button>
                    {pingResult && <span style={{ color: '#4ec9b0' }}>{pingResult}</span>}
                </div>

                <div id="terminal-tabs" style={{ display: 'flex', gap: '4px', marginLeft: 'auto', padding: '0 10px', alignItems: 'center' }}>
                    <button className="icon-btn" style={{ background: 'none', border: 'none', color: 'var(--vscode-sideBar-foreground)', cursor: 'pointer', opacity: 0.8 }}>
                        <i className="codicon codicon-add"></i>
                    </button>
                </div>
            </div>
            <div className="panel-content" id="panel-content" style={{ flex: 1, position: 'relative', overflow: 'hidden', background: 'var(--vscode-editor-background)', padding: 0 }}>
                <div 
                    id="terminal-container" 
                    style={{ 
                        width: '100%', 
                        height: '100%', 
                        display: activeTab === 'TERMINAL' ? 'block' : 'none' 
                    }}
                ></div>
                {activeTab === 'EMULATOR' && (
                    <div style={{ width: '100%', height: '100%' }}>
                        <EmulatorPanel />
                    </div>
                )}
            </div>
        </footer>
    );
};

export default BottomPanel;
