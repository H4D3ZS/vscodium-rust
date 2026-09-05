import React from 'react';
import { useStore } from '../store';
import { Globe, Shield, Lock, RefreshCw, ChevronLeft, ChevronRight, Search, Activity } from 'lucide-react';
import { invoke } from '@tauri-apps/api/core';
import { closeCenterWorkbench } from '../application/layout/closeCenterWorkbench';

// True when the URL points at THIS IDE's own origin. In `tauri dev` the IDE's
// frontend is served by Vite (e.g. localhost:5173), so previewing that URL would
// just load the IDE inside itself (infinite IDE-in-IDE). We block that.
const isSelfOrigin = (u: string): boolean => {
    try {
        return new URL(u).host === window.location.host;
    } catch {
        return false;
    }
};

const BrowserSurface: React.FC = () => {
    // Start blank — show a start screen with dev-server quick-picks instead of
    // auto-loading a guessed port (5173 is the IDE's own dev server in dev mode).
    const [url, setUrl] = React.useState('');
    const [screenshot, setScreenshot] = React.useState<string | null>(null);
    const [isNavigating, setIsNavigating] = React.useState(false);
    const [isDesignMode, setIsDesignMode] = React.useState(false);
    const [selectedElement, setSelectedElement] = React.useState<{x: number, y: number} | null>(null);
    // 'vision' = mirror of the REAL stealth-Firefox the agent drives (screenshots;
    // works on ANY site). 'live' = interactive <iframe> for quick localhost dev
    // preview (limited by CSP/X-Frame-Options). Default to vision since the real
    // browser is the Firefox window.
    const [mode, setMode] = React.useState<'live' | 'vision'>('vision');
    const [iframeKey, setIframeKey] = React.useState(0);
    const isAgentThinking = useStore(state => state.isAgentThinking);
    const agentMode = useStore(state => state.agentMode);
    // Live screenshot polling is OFF by default (memory/CPU heavy). Enable in
    // Settings → Permissions → Browser only if the machine can handle it.
    const visionEnabled = useStore(state => state.isAgentVisionEnabled);
    const browserHidden = useStore(state => state.browserStealthHidden);

    const openStealthBrowser = React.useCallback(() => {
        invoke('browser_open', { headless: browserHidden }).catch(() => { /* Playwright optional for LIVE iframe */ });
    }, [browserHidden]);

    // Panel opened — ensure stealth Firefox sidecar is up for VISION mode.
    React.useEffect(() => {
        if (mode !== 'vision') return;
        openStealthBrowser();
    }, [mode, openStealthBrowser]);

    // Live mirror: while the agent is working in VISION mode, poll the real
    // browser for a fresh screenshot so the panel shows what the agent sees/does.
    React.useEffect(() => {
        let interval: any;
        let busy = false;
        if (visionEnabled && isAgentThinking && mode === 'vision') {
            interval = setInterval(async () => {
                if (busy) return;
                busy = true;
                try {
                    const b64 = await invoke<string>('browser_screenshot');
                    if (b64) setScreenshot(`data:image/jpeg;base64,${b64}`);
                } catch { /* browser not up yet */ } finally { busy = false; }
            }, 3000);
        }
        return () => clearInterval(interval);
    }, [visionEnabled, isAgentThinking, mode]);

    const normalizeUrl = (u: string): string => {
        const t = u.trim();
        if (!t) return t;
        if (/^https?:\/\//i.test(t)) return t;
        if (/^localhost(:\d+)?(\/|$)/i.test(t) || /^\d+\.\d+\.\d+\.\d+/.test(t)) return `http://${t}`;
        return `https://${t}`;
    };

    const handleNavigate = async (rawUrl: string) => {
        const newUrl = normalizeUrl(rawUrl);
        setUrl(newUrl);
        if (mode === 'live') {
            // Interactive preview — just (re)load the iframe.
            setIframeKey(k => k + 1);
            return;
        }
        // Vision mode — drive the stealth browser + grab a screenshot.
        setIsNavigating(true);
        try {
            await invoke('browser_open', { headless: browserHidden });
            await invoke('browser_navigate', { url: newUrl });
            const b64 = await invoke<string>('browser_screenshot');
            if (b64) setScreenshot(`data:image/jpeg;base64,${b64}`);
        } catch (e) {
            console.error(e);
        } finally {
            setIsNavigating(false);
        }
    };

    return (
        <div className="browser-surface" style={{ display: 'flex', flex: 1, flexDirection: 'column', height: '100%', background: '#fff', color: '#000' }}>
            {/* Browser Navigation Bar */}
            <div className="browser-navbar" style={{
                height: '48px',
                background: 'var(--vscode-editor-background)',
                borderBottom: '1px solid var(--vscode-panel-border)',
                display: 'flex',
                alignItems: 'center',
                padding: '0 12px',
                gap: '8px',
                color: 'var(--vscode-foreground)'
            }}>
                <div style={{ display: 'flex', gap: '4px' }}>
                    <button style={navButtonStyle}><ChevronLeft size={16} /></button>
                    <button style={navButtonStyle}><ChevronRight size={16} /></button>
                    <button style={navButtonStyle} onClick={() => handleNavigate(url)} disabled={isNavigating}>
                        <RefreshCw size={16} className={isNavigating? 'animate-spin': ''} />
                    </button>
                </div>

                <div className="url-bar" style={{
                    flex: 1,
                    height: '28px',
                    background: 'var(--vscode-sideBar-background)',
                    borderRadius: '4px',
                    border: '1px solid var(--vscode-panel-border)',
                    display: 'flex',
                    alignItems: 'center',
                    padding: '0 10px',
                    fontSize: '12px',
                    gap: '8px'
                }}>
                    <Lock size={12} style={{ opacity: 0.5 }} />
                    <input
                        value={url}
                        onChange={(e) => setUrl(e.target.value)}
                        onKeyDown={(e) => e.key === 'Enter' && handleNavigate(url)}
                        style={{
                            background: 'transparent',
                            border: 'none',
                            color: 'inherit',
                            width: '100%',
                            outline: 'none',
                            fontSize: '11px'
                        }}
                    />
                    <Shield size={12} color="var(--termainator-success)" />
                </div>

                {isAgentThinking && (
                    <div style={{
                        padding: '4px 10px',
                        background: 'rgba(59, 130, 246, 0.1)',
                        borderRadius: '20px',
                        fontSize: '10px',
                        fontWeight: 700,
                        color: '#3b82f6',
                        display: 'flex',
                        alignItems: 'center',
                        gap: '6px'
                    }}>
                        <Activity size={10} className="animate-pulse" />
                        AGENT ACTUATING
                    </div>
                )}
                {/* Live (iframe) ↔ Vision (agent screenshot) toggle */}
                <div style={{ display: 'flex', borderRadius: '6px', overflow: 'hidden', border: '1px solid var(--vscode-panel-border)' }}>
                    {(['live', 'vision'] as const).map(m => (
                        <button
                            key={m}
                            onClick={() => setMode(m)}
                            style={{
                                ...navButtonStyle,
                                width: 'auto',
                                padding: '0 10px',
                                borderRadius: 0,
                                fontSize: '10px',
                                fontWeight: 700,
                                textTransform: 'uppercase',
                                letterSpacing: '0.4px',
                                opacity: 1,
                                background: mode === m? 'var(--vscode-button-background, #0e639c)': 'transparent',
                                color: mode === m? 'var(--vscode-button-foreground, #fff)': 'var(--vscode-foreground)',
                            }}
                            title={m === 'live'? 'Live preview (interactive iframe)': 'Agent vision (stealth browser screenshot)'}
                        >
                            {m}
                        </button>
                    ))}
                </div>
                <button
                    onClick={() => setIsDesignMode(!isDesignMode)}
                    style={{
                        ...navButtonStyle,
                        background: isDesignMode? 'rgba(168, 85, 247, 0.2)': 'transparent',
                        color: isDesignMode? '#c084fc': 'inherit',
                        border: isDesignMode? '1px solid rgba(168, 85, 247, 0.5)': 'none'
                    }}
                    title="Toggle Design Mode"
                >
                    <i className="codicon codicon-symbol-ruler" style={{ fontSize: '14px' }} />
                </button>
                <button
                    onClick={() => closeCenterWorkbench()}
                    style={navButtonStyle}
                    title="Close preview (back to editor)"
                >
                    <i className="codicon codicon-close" style={{ fontSize: '14px' }} />
                </button>
            </div>

            {/* Browser Content Area (Live Vision) */}
            <div className="browser-content" style={{
                flex: 1,
                display: 'flex',
                overflow: 'auto',
                background: '#f1f5f9',
                position: 'relative'
            }}>
                {mode === 'live'? (
                    !url? (
                        <BrowserStart onPick={(u) => { setUrl(u); setIframeKey(k => k + 1); }} />
                    ): isSelfOrigin(url)? (
                        <BrowserSelfWarning url={url} onClear={() => setUrl('')} />
                    ): (
                        <iframe
                            key={iframeKey}
                            src={url}
                            title="Live preview"
                            style={{ flex: 1, width: '100%', height: '100%', border: 'none', background: '#fff' }}
                            sandbox="allow-scripts allow-same-origin allow-forms allow-popups allow-modals"
                        />
                    )
                ): screenshot? (
                    <div style={{ position: 'relative', width: '100%', display: 'flex', justifyContent: 'center' }}>
                        <img
                            src={screenshot}
                            style={{
                                maxWidth: '100%',
                                height: 'auto',
                                objectFit: 'contain',
                                background: '#fff',
                                cursor: isDesignMode? 'crosshair': 'default'
                            }}
                            alt="Agent Vision"
                            onClick={(e) => {
                                if (isDesignMode) {
                                    const rect = e.currentTarget.getBoundingClientRect();
                                    setSelectedElement({ x: e.clientX - rect.left, y: e.clientY - rect.top });
                                }
                            }}
                        />
                        {isDesignMode && (
                            <div style={{
                                position: 'absolute',
                                top: 0, left: 0, right: 0, bottom: 0,
                                background: 'url("data:image/svg+xml,%3Csvg width=\'20\' height=\'20\' xmlns=\'http://www.w3.org/2000/svg\'%3E%3Cpath d=\'M 20 0 L 0 0 0 20\' fill=\'none\' stroke=\'rgba(168, 85, 247, 0.2)\' stroke-width=\'1\'/%3E%3C/svg%3E")',
                                pointerEvents: 'none'
                            }} />
                        )}
                        {isDesignMode && selectedElement && (
                            <div style={{
                                position: 'absolute',
                                left: selectedElement.x - 4,
                                top: selectedElement.y - 4,
                                width: '8px',
                                height: '8px',
                                background: '#c084fc',
                                borderRadius: '50%',
                                boxShadow: '0 0 0 4px rgba(168,85,247,0.3)'
                            }} />
                        )}
                        {isDesignMode && selectedElement && (
                            <div style={{
                                position: 'absolute',
                                left: Math.min(selectedElement.x + 12, 600),
                                top: selectedElement.y - 20,
                                background: 'var(--vscode-editor-background)',
                                border: '1px solid var(--vscode-widget-border)',
                                borderRadius: '6px',
                                padding: '8px 12px',
                                display: 'flex',
                                flexDirection: 'column',
                                gap: '8px',
                                boxShadow: '0 4px 12px rgba(0,0,0,0.5)',
                                zIndex: 10
                            }}>
                                <span style={{ fontSize: '11px', fontWeight: 600, color: 'var(--vscode-foreground)' }}>Element Selected</span>
                                <input 
                                    autoFocus
                                    placeholder="Rewrite, resize, or move..." 
                                    style={{
                                        background: 'rgba(0,0,0,0.2)',
                                        border: '1px solid rgba(255,255,255,0.1)',
                                        color: 'var(--vscode-editor-foreground, #fff)',
                                        padding: '4px 8px',
                                        borderRadius: '4px',
                                        fontSize: '11px',
                                        outline: 'none',
                                        width: '200px'
                                    }}
                                    onKeyDown={(e) => {
                                        if (e.key === 'Enter') {
                                            setSelectedElement(null);
                                            setIsDesignMode(false);
                                            // Real app would dispatch to agent here
                                        }
                                    }}
                                />
                                <div style={{ display: 'flex', alignItems: 'center', gap: '4px', fontSize: '9px', opacity: 0.6 }}>
                                    <i className="codicon codicon-sparkle" /> Ask Agent to Edit
                                </div>
                            </div>
                        )}
                    </div>
                ): (
                    <BrowserStart onPick={(u) => void handleNavigate(u)} />
                )}
            </div>
        </div>
    );
};

const navButtonStyle: React.CSSProperties = {
    background: 'none',
    border: 'none',
    color: 'inherit',
    cursor: 'pointer',
    width: '28px',
    height: '28px',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    borderRadius: '4px',
    opacity: 0.7
};

// Quick-pick dev servers. 5173 is intentionally omitted — that's the IDE's own
// dev server in `tauri dev`. (Vite bumps a second app to 5174.)
const QUICK_PORTS: { label: string; url: string }[] = [
    { label: 'Vite (5174)', url: 'http://localhost:5174' },
    { label: 'Astro (4321)', url: 'http://localhost:4321' },
    { label: 'Next/CRA (3000)', url: 'http://localhost:3000' },
    { label: 'Dev (8080)', url: 'http://localhost:8080' },
    { label: 'Live Server (5500)', url: 'http://localhost:5500' },
];

const BrowserStart: React.FC<{ onPick: (u: string) => void }> = ({ onPick }) => (
    <div style={{ flex: 1, display: 'flex', alignItems: 'center', justifyContent: 'center', background: '#f8fafc' }}>
        <div style={{ textAlign: 'center', maxWidth: 460, padding: 24 }}>
            <div style={{ fontSize: 44, marginBottom: 12 }}></div>
            <h2 style={{ color: '#0f172a', margin: '0 0 6px', fontSize: 20 }}>Browser Preview</h2>
            <p style={{ color: '#64748b', fontSize: 13, lineHeight: 1.5, margin: '0 0 20px' }}>
                Preview your project's dev server here while the agent edits code.
                <b>LIVE</b> = interactive iframe · <b>VISION</b> = mirror of the stealth Firefox the agent drives.
            </p>
            <div style={{ display: 'flex', flexWrap: 'wrap', gap: 8, justifyContent: 'center' }}>
                {QUICK_PORTS.map(p => (
                    <button
                        key={p.url}
                        onClick={() => onPick(p.url)}
                        style={{
                            background: '#fff', color: '#0f172a',
                            border: '1px solid #cbd5e1', borderRadius: 8,
                            padding: '7px 12px', fontSize: 12, cursor: 'pointer', fontWeight: 600,
                        }}
                    >
                        {p.label}
                    </button>
                ))}
            </div>
            <p style={{ color: '#94a3b8', fontSize: 11, marginTop: 18 }}>
                Tip: port 5173 is this IDE's own dev server — use your project's port.
                Toggle <b>visible vs hidden</b> browser in Settings → Permissions.
            </p>
        </div>
    </div>
);

const BrowserSelfWarning: React.FC<{ url: string; onClear: () => void }> = ({ url, onClear }) => (
    <div style={{ flex: 1, display: 'flex', alignItems: 'center', justifyContent: 'center', background: '#fff7ed' }}>
        <div style={{ textAlign: 'center', maxWidth: 460, padding: 24 }}>
            <div style={{ fontSize: 40, marginBottom: 12 }}></div>
            <h2 style={{ color: '#9a3412', margin: '0 0 6px', fontSize: 18 }}>That's this IDE</h2>
            <p style={{ color: '#7c2d12', fontSize: 13, lineHeight: 1.5, margin: '0 0 18px' }}>
                <code>{url}</code> is this IDE's own dev server — loading it here would just
                show the IDE inside itself. Enter your <b>project's</b> dev-server URL instead.
            </p>
            <button
                onClick={onClear}
                style={{
                    background: '#ea580c', color: '#fff', border: 'none', borderRadius: 8,
                    padding: '8px 16px', fontSize: 12, cursor: 'pointer', fontWeight: 600,
                }}
            >
                Pick a different URL
            </button>
        </div>
    </div>
);

export default BrowserSurface;
