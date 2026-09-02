import React, { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { invoke } from '../tauri_bridge';
import IPhoneAcheronPanel from './IPhoneAcheronPanel';

type SimDevice = {
    udid: string;
    name: string;
    runtime: string;
    state: string;
    booted: boolean;
};

type Preflight = {
    ok?: boolean;
    hint?: string;
    developer_dir?: string;
    profile?: string;
    mode?: 'embed' | 'stream';
    accessibility_hint?: string;
};

type SessionState = {
    running?: boolean;
    paused?: boolean;
    udid?: string | null;
    profile?: string;
};

type StreamStatus = {
    stream_url?: string;
    width?: number;
    height?: number;
    device_name?: string;
    running?: boolean;
};

function parseIosVersion(runtime: string): string {
    const m = runtime.match(/iOS[- ](\d+)[-_.](\d+)/i);
    if (m) return `${m[1]}.${m[2]}`;
    const dotted = runtime.match(/(\d+\.\d+)/);
    return dotted ? dotted[1] : '—';
}

const toolbarIcon: React.CSSProperties = {
    fontFamily: 'codicon',
    fontStyle: 'normal',
    fontSize: 14,
    cursor: 'pointer',
    opacity: 0.75,
    color: 'var(--vscode-foreground, #ddd)',
};

const inputStyle: React.CSSProperties = {
    flex: 1,
    fontSize: 11,
    padding: '3px 6px',
    background: 'var(--vscode-editor-background, #1e1e1e)',
    border: '1px solid var(--vscode-panel-border, #333)',
    borderRadius: 3,
    color: 'var(--vscode-editor-foreground, #fff)',
    outline: 'none',
};

const HOT_ATTACH_TIMEOUT_MS = 25_000;
const COLD_BOOT_TIMEOUT_MS = 90_000;

/** Headless native NSView (default), optional embed/stream fallbacks via env. */
const MacIOSSimulatorPanel: React.FC = () => {
    const streamImgRef = useRef<HTMLImageElement>(null);
    const viewportRef = useRef<HTMLDivElement>(null);
    const layoutRafRef = useRef<number | null>(null);
    const mirroringRef = useRef(false);
    const hasFrameRef = useRef(false);
    const autoStartedRef = useRef(false);
    const frameTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);
    const [devices, setDevices] = useState<SimDevice[]>([]);
    const [selectedUdid, setSelectedUdid] = useState('');
    const [streamUrl, setStreamUrl] = useState('');
    const [panelMode, setPanelMode] = useState<'native' | 'embed' | 'stream'>('native');
    const [hasDisplay, setHasDisplay] = useState(false);
    const [frameSize, setFrameSize] = useState({ w: 393, h: 852 });
    const [preflight, setPreflight] = useState<Preflight | null>(null);
    const [profile, setProfile] = useState('balanced');
    const [running, setRunning] = useState(false);
    const [connecting, setConnecting] = useState(false);
    const [coldBoot, setColdBoot] = useState(false);
    const [error, setError] = useState('');
    const [showAdvanced, setShowAdvanced] = useState(false);
    const [showLegacy, setShowLegacy] = useState(false);

    const selectedDevice = useMemo(
        () => devices.find((d) => d.udid === selectedUdid) ?? devices[0],
        [devices, selectedUdid],
    );

    const deviceDisplayName = selectedDevice?.name ?? 'iPhone';
    const iosVersion = selectedDevice ? parseIosVersion(selectedDevice.runtime) : '—';
    const aspectRatio = `${frameSize.w} / ${frameSize.h}`;

    const clearFrameTimeout = () => {
        if (frameTimeoutRef.current) {
            clearTimeout(frameTimeoutRef.current);
            frameTimeoutRef.current = null;
        }
    };

    const armFrameTimeout = (cold: boolean) => {
        clearFrameTimeout();
        frameTimeoutRef.current = setTimeout(() => {
            if (mirroringRef.current && !hasFrameRef.current) {
                setError('Stream timed out — tap Run to retry.');
                mirroringRef.current = false;
                setRunning(false);
                setConnecting(false);
                setColdBoot(false);
                setStreamUrl('');
                void invoke('ios_sim_stop_mirror').catch(() => {});
            }
        }, cold ? COLD_BOOT_TIMEOUT_MS : HOT_ATTACH_TIMEOUT_MS);
    };

    const syncPanelLayout = useCallback(() => {
        if (layoutRafRef.current != null) return;
        layoutRafRef.current = requestAnimationFrame(() => {
            layoutRafRef.current = null;
            const el = viewportRef.current;
            if (!el || !running || panelMode === 'stream') return;
            const r = el.getBoundingClientRect();
            if (r.width < 8 || r.height < 8) return;
            void invoke('ios_sim_embed_layout', {
                x: r.left,
                y: r.top,
                width: r.width,
                height: r.height,
            }).catch(() => {});
        });
    }, [panelMode, running]);

    const applyDimensions = useCallback((w?: number, h?: number) => {
        if (w && h && w > 0 && h > 0) {
            setFrameSize({ w, h });
        }
    }, []);

    const bindStream = useCallback((url: string) => {
        if (!url) return;
        setStreamUrl(url);
    }, []);

    const onStreamLive = useCallback(() => {
        hasFrameRef.current = true;
        setHasDisplay(true);
        setConnecting(false);
        setColdBoot(false);
        clearFrameTimeout();
    }, []);

    const refreshDevices = useCallback(async () => {
        try {
            const list = await invoke<SimDevice[]>('ios_sim_list_devices');
            setDevices(list);
            if (!selectedUdid && list.length > 0) {
                const booted = list.find((d) => d.booted);
                setSelectedUdid(booted?.udid ?? list[0].udid);
            }
        } catch (e) {
            setError(String(e));
        }
    }, [selectedUdid]);

    useEffect(() => {
        const warmupId = window.setTimeout(() => {
            void invoke('ios_sim_warmup').catch(() => {});
        }, 3_000);
        invoke<Preflight>('ios_sim_preflight').then((p) => {
            setPreflight(p);
            if (p.profile) setProfile(p.profile);
            if (p.mode) setPanelMode(p.mode === 'stream' ? 'stream' : p.mode === 'embed' ? 'embed' : 'native');
        }).catch(() => {});
        void refreshDevices();
        return () => clearTimeout(warmupId);
    }, [refreshDevices]);

    const startSession = useCallback(async (udidOverride?: string) => {
        if (mirroringRef.current || connecting) return;
        const udid = udidOverride ?? selectedUdid;
        const device = devices.find((d) => d.udid === udid) ?? selectedDevice;
        const wasBooted = device?.booted ?? false;
        setConnecting(true);
        setColdBoot(!wasBooted);
        setError('');
        hasFrameRef.current = false;
        setHasDisplay(false);
        mirroringRef.current = true;
        setRunning(true);
        armFrameTimeout(!wasBooted);
        try {
            const result = await invoke<{
                ok?: boolean;
                reused?: boolean;
                mode?: 'native' | 'embed' | 'stream';
                profile?: string;
                stream_url?: string;
                width?: number;
                height?: number;
            }>('ios_sim_start_mirror', { udid: udid || null, auto_boot: true });
            const mode = result?.mode ?? 'native';
            setPanelMode(mode);
            if (result?.profile) setProfile(result.profile);
            applyDimensions(result?.width, result?.height);
            if (mode === 'native' || mode === 'embed') {
                hasFrameRef.current = true;
                setHasDisplay(true);
                setConnecting(false);
                setColdBoot(false);
                clearFrameTimeout();
                requestAnimationFrame(() => syncPanelLayout());
            } else if (result?.stream_url) {
                bindStream(result.stream_url);
            } else {
                const url = await invoke<string | null>('ios_sim_stream_url');
                if (url) bindStream(url);
            }
            await refreshDevices();
        } catch (e) {
            setError(String(e));
            mirroringRef.current = false;
            setRunning(false);
            setConnecting(false);
            setColdBoot(false);
            setStreamUrl('');
            clearFrameTimeout();
        }
    }, [applyDimensions, bindStream, connecting, devices, selectedDevice, selectedUdid, refreshDevices, syncPanelLayout]);

    useEffect(() => {
        if (panelMode === 'stream' || !running) return undefined;
        syncPanelLayout();
        const el = viewportRef.current;
        if (!el) return undefined;
        const ro = new ResizeObserver(() => syncPanelLayout());
        ro.observe(el);
        const onWin = () => syncPanelLayout();
        window.addEventListener('resize', onWin);
        window.addEventListener('scroll', onWin, true);
        // Burst sync after attach — ResizeObserver can lag one frame on sidebar open.
        let burst = 0;
        const burstId = window.setInterval(() => {
            syncPanelLayout();
            burst += 1;
            if (burst >= 12) window.clearInterval(burstId);
        }, 80);
        return () => {
            ro.disconnect();
            window.removeEventListener('resize', onWin);
            window.removeEventListener('scroll', onWin, true);
            window.clearInterval(burstId);
            if (layoutRafRef.current != null) {
                cancelAnimationFrame(layoutRafRef.current);
                layoutRafRef.current = null;
            }
        };
    }, [panelMode, running, syncPanelLayout, frameSize.w, frameSize.h]);

    const stopSession = async () => {
        mirroringRef.current = false;
        clearFrameTimeout();
        await invoke('ios_sim_stop_mirror').catch(() => {});
        setRunning(false);
        setConnecting(false);
        setColdBoot(false);
        hasFrameRef.current = false;
        setHasDisplay(false);
        setStreamUrl('');
    };

    useEffect(() => {
        invoke('ios_sim_resume').catch(() => {});
        return () => {
            invoke('ios_sim_pause').catch(() => {});
        };
    }, []);

    // Pause IOSurface capture when the IDE tab/window is hidden to save GPU/RAM.
    useEffect(() => {
        if (!running) return undefined;
        const onVis = () => {
            if (document.hidden) {
                void invoke('ios_sim_pause').catch(() => {});
            } else {
                void invoke('ios_sim_resume').catch(() => {});
                syncPanelLayout();
            }
        };
        document.addEventListener('visibilitychange', onVis);
        return () => document.removeEventListener('visibilitychange', onVis);
    }, [running, syncPanelLayout]);

    useEffect(() => {
        if (!running) {
            const id = setInterval(() => void refreshDevices(), 25_000);
            return () => clearInterval(id);
        }
        return undefined;
    }, [running, refreshDevices]);

    // Native IOSurface reports real pixel size once capture starts — keep bezel aspect in sync.
    useEffect(() => {
        if (!running || panelMode === 'stream') return undefined;
        const poll = () => {
            void invoke<SessionState & { width?: number; height?: number }>('ios_sim_session_state')
                .then((s) => applyDimensions(s.width, s.height))
                .catch(() => {});
        };
        poll();
        const id = window.setInterval(poll, 5000);
        return () => clearInterval(id);
    }, [running, panelMode, applyDimensions]);

    useEffect(() => {
        if (!selectedUdid || autoStartedRef.current) return;
        autoStartedRef.current = true;

        invoke<SessionState>('ios_sim_session_state')
            .then(async (session) => {
                if (session.profile) setProfile(session.profile);
                const mode = (session as SessionState & { mode?: string }).mode ?? 'native';
                setPanelMode(mode === 'stream' ? 'stream' : mode === 'embed' ? 'embed' : 'native');
                if (mode !== 'stream') {
                    if (session.running) {
                        mirroringRef.current = true;
                        setRunning(true);
                        hasFrameRef.current = true;
                        setHasDisplay(true);
                        setConnecting(false);
                        requestAnimationFrame(() => syncPanelLayout());
                        return;
                    }
                    return startSession();
                }
                const status = await invoke<StreamStatus>('ios_sim_stream_status');
                applyDimensions(status.width, status.height);
                const url = status.stream_url || (await invoke<string | null>('ios_sim_stream_url'));
                if (session.running && url) {
                    mirroringRef.current = true;
                    setRunning(true);
                    bindStream(url);
                    setConnecting(true);
                    armFrameTimeout(false);
                    return;
                }
                return startSession();
            })
            .catch(() => startSession());
    }, [selectedUdid, startSession, bindStream, applyDimensions, syncPanelLayout]);

    const mapTouch = (e: React.MouseEvent, phase: string) => {
        if (!running || !hasDisplay) return;
        const target = panelMode === 'stream' ? streamImgRef.current : viewportRef.current;
        if (!target) return;
        const rect = target.getBoundingClientRect();
        if (rect.width < 1 || rect.height < 1) return;
        const x = (e.clientX - rect.left) / rect.width;
        const y = (e.clientY - rect.top) / rect.height;
        if (x < 0 || y < 0 || x > 1 || y > 1) return;
        void invoke('ios_sim_send_touch', { x_ratio: x, y_ratio: y, phase }).catch(() => {});
    };

    const touchActiveRef = useRef(false);

    const onTouchDown = (e: React.MouseEvent) => {
        touchActiveRef.current = true;
        mapTouch(e, 'down');
    };

    const onTouchMove = (e: React.MouseEvent) => {
        if (!touchActiveRef.current) return;
        mapTouch(e, 'move');
    };

    const onTouchUp = (e: React.MouseEvent) => {
        if (!touchActiveRef.current) return;
        touchActiveRef.current = false;
        mapTouch(e, 'up');
    };

    const bootSelected = async () => {
        if (!selectedUdid) return;
        if (selectedDevice?.booted) {
            autoStartedRef.current = true;
            void startSession();
            return;
        }
        setError('');
        try {
            await invoke('ios_sim_boot_device', { udid: selectedUdid });
            await refreshDevices();
            autoStartedRef.current = true;
            void startSession();
        } catch (e) {
            setError(String(e));
        }
    };

    const handleHome = () => { void invoke('ios_sim_send_home').catch(() => {}); };

    const handleScreenshot = async () => {
        if (panelMode === 'stream') {
            const img = streamImgRef.current;
            if (!img?.src) return;
            const a = document.createElement('a');
            a.href = img.src;
            a.download = `${deviceDisplayName.replace(/\s+/g, '_')}_${Date.now()}.jpg`;
            a.click();
            return;
        }
        try {
            const b64 = await invoke<string>('ios_sim_capture_screenshot');
            const a = document.createElement('a');
            a.href = `data:image/png;base64,${b64}`;
            a.download = `${deviceDisplayName.replace(/\s+/g, '_')}_${Date.now()}.png`;
            a.click();
        } catch (e) {
            setError(String(e));
        }
    };

    if (showLegacy) {
        return (
            <div style={{ display: 'flex', flexDirection: 'column', height: '100%', minHeight: 0 }}>
                <div style={{ padding: '6px 12px', borderBottom: '1px solid var(--vscode-panel-border)', background: 'var(--vscode-titleBar-activeBackground, #1c1c1e)' }}>
                    <button type="button" onClick={() => setShowLegacy(false)} style={{ fontSize: 10, background: 'transparent', border: 'none', color: 'inherit', cursor: 'pointer' }}>← Back</button>
                </div>
                <IPhoneAcheronPanel />
            </div>
        );
    }

    const showIdlePrompt = !running && !connecting;
    const showStreamAttach = running && connecting && !hasDisplay && !coldBoot;
    const showColdBoot = running && connecting && !hasDisplay && coldBoot;

    return (
        <div style={{ display: 'flex', flexDirection: 'column', height: '100%', position: 'relative', background: 'var(--vscode-editor-background, #1e1e1e)', color: 'var(--vscode-foreground, #f0f0f0)', overflow: 'hidden' }}>
            <div style={{ display: 'flex', alignItems: 'center', gap: 10, padding: '6px 12px', background: 'var(--vscode-titleBar-activeBackground, #1c1c1e)', borderBottom: '1px solid #2a2a2e', flexShrink: 0, zIndex: 5 }}>
                <div style={{ display: 'flex', gap: 6 }}>
                    <span style={{ width: 11, height: 11, borderRadius: '50%', background: '#ff5f57' }} />
                    <span style={{ width: 11, height: 11, borderRadius: '50%', background: '#febc2e' }} />
                    <span style={{ width: 11, height: 11, borderRadius: '50%', background: '#28c840' }} />
                </div>
                <div style={{ flex: 1, textAlign: 'center', lineHeight: 1.05 }}>
                    <div style={{ fontSize: 12, fontWeight: 700 }}>{deviceDisplayName}</div>
                    <div style={{ fontSize: 9, opacity: 0.55 }}>
                        iOS {iosVersion}
                        {selectedDevice?.booted ? ' · Booted' : selectedDevice ? ' · Shutdown' : ''}
                        {profile ? ` · ${profile}` : ''}
                    </div>
                </div>
                <div style={{ display: 'flex', gap: 14, alignItems: 'center' }}>
                    {running ? (
                        <i className="codicon codicon-debug-stop" title="Stop" onClick={() => void stopSession()} style={{ ...toolbarIcon, color: '#ff5f57', opacity: 0.9 }} />
                    ) : (
                        <i className="codicon codicon-play" title="Run" onClick={connecting ? undefined : () => void startSession()} style={{ ...toolbarIcon, color: '#28c840', opacity: connecting ? 0.4 : 0.9 }} />
                    )}
                    <i className="codicon codicon-home" title="Home" onClick={handleHome} style={toolbarIcon} />
                    <i className="codicon codicon-device-camera" title="Screenshot" onClick={handleScreenshot} style={{ ...toolbarIcon, opacity: hasDisplay ? 0.75 : 0.35 }} />
                    <i className="codicon codicon-settings-gear" title="Devices" onClick={() => setShowAdvanced((v) => !v)} style={{ ...toolbarIcon, color: showAdvanced ? '#63b3ed' : toolbarIcon.color }} />
                </div>
            </div>

            {preflight && !preflight.ok && preflight.hint && (
                <div style={{ padding: '6px 12px', fontSize: 10, color: '#fbbf24', background: 'rgba(66, 32, 6, 0.85)', borderBottom: '1px solid #78350f' }}>{preflight.hint}</div>
            )}

            {preflight?.accessibility_hint && (
                <div style={{ padding: '4px 12px', fontSize: 9, color: '#94a3b8', background: 'rgba(15,23,42,0.85)', borderBottom: '1px solid #334155' }}>
                    {preflight.accessibility_hint}
                </div>
            )}

            <div style={{ flex: 1, display: 'flex', alignItems: 'center', justifyContent: 'center', minHeight: 0, padding: 12 }}>
                <div style={{ position: 'relative', height: '100%', maxWidth: '100%', aspectRatio, background: panelMode === 'stream' ? 'linear-gradient(145deg, #1a1a1e 0%, #0b0b0d 100%)' : 'transparent', borderRadius: panelMode === 'stream' ? 44 : 0, padding: panelMode === 'stream' ? 10 : 0, boxShadow: panelMode === 'stream' ? 'inset 0 0 0 2px #3a3a40, inset 0 0 0 4px #0b0b0d, 0 12px 40px rgba(0,0,0,0.55)' : 'none', boxSizing: 'border-box' }}>
                    <div
                        ref={viewportRef}
                        style={{ position: 'relative', width: '100%', height: '100%', borderRadius: panelMode === 'stream' ? 36 : 0, overflow: 'hidden', background: 'transparent' }}
                    >
                        {panelMode === 'stream' && streamUrl ? (
                            <img
                                ref={streamImgRef}
                                src={streamUrl}
                                alt={deviceDisplayName}
                                draggable={false}
                                onLoad={onStreamLive}
                                onError={() => setError('MJPEG stream failed — tap Run to reconnect.')}
                                onMouseDown={onTouchDown}
                                onMouseMove={onTouchMove}
                                onMouseUp={onTouchUp}
                                onMouseLeave={onTouchUp}
                                style={{
                                    width: '100%',
                                    height: '100%',
                                    objectFit: 'fill',
                                    cursor: hasDisplay ? 'pointer' : 'default',
                                    userSelect: 'none',
                                    display: hasDisplay ? 'block' : 'none',
                                }}
                            />
                        ) : running && hasDisplay && panelMode !== 'native' ? (
                            <div
                                aria-hidden
                                onMouseDown={onTouchDown}
                                onMouseMove={onTouchMove}
                                onMouseUp={onTouchUp}
                                onMouseLeave={onTouchUp}
                                style={{
                                    position: 'absolute',
                                    inset: 0,
                                    zIndex: 2,
                                    cursor: 'pointer',
                                    userSelect: 'none',
                                    touchAction: 'none',
                                    background: 'transparent',
                                }}
                            />
                        ) : running && hasDisplay && panelMode === 'native' ? (
                            /* Native SimDisplayView sits above WKWebView in AppKit — receives HID touch directly */
                            null
                        ) : (
                            <div style={{ width: '100%', height: '100%', pointerEvents: 'none' }} />
                        )}

                        {!hasDisplay && (
                            <div
                                onClick={showIdlePrompt ? () => void startSession() : undefined}
                                style={{ position: 'absolute', inset: 0, display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', gap: 12, cursor: showIdlePrompt ? 'pointer' : 'default', pointerEvents: panelMode !== 'stream' && running ? 'none' : 'auto' }}
                            >
                                {showColdBoot ? (
                                    <div style={{ color: '#64748b', fontSize: 11 }}>Booting simulator…</div>
                                ) : showStreamAttach ? (
                                    <div style={{ color: '#64748b', fontSize: 11 }}>Starting stream…</div>
                                ) : showIdlePrompt ? (
                                    <>
                                        <i className="codicon codicon-play-circle" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: 40, color: '#3b82f6' }} />
                                        <div style={{ color: '#cbd5e1', fontSize: 12, fontWeight: 600 }}>Tap to run {deviceDisplayName}</div>
                                    </>
                                ) : null}
                            </div>
                        )}

                        {panelMode === 'stream' && (
                            <div style={{ position: 'absolute', top: 10, left: '50%', transform: 'translateX(-50%)', width: 90, height: 26, background: '#000', borderRadius: 20, pointerEvents: 'none' }} />
                        )}
                    </div>
                </div>
            </div>

            {error && (
                <div style={{ position: 'absolute', bottom: 12, left: '50%', transform: 'translateX(-50%)', maxWidth: '90%', padding: '6px 12px', fontSize: 10, borderRadius: 6, background: 'rgba(17,17,19,0.92)', border: '1px solid #7f1d1d', color: '#f87171', zIndex: 10 }}>{error}</div>
            )}

            {showAdvanced && (
                <div style={{ position: 'absolute', top: 42, left: 0, right: 0, zIndex: 20, padding: '10px 12px', background: 'rgba(17,17,19,0.97)', borderBottom: '1px solid #222', display: 'flex', flexDirection: 'column', gap: 8 }}>
                    <div style={{ fontSize: 9, opacity: 0.5 }}>Headless native panel · CoreSimulator IOSurface · pauses when tab hidden</div>
                    <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
                        <label style={{ fontSize: 10, opacity: 0.7, width: 56 }}>Device</label>
                        <select value={selectedUdid} onChange={(e) => { const n = e.target.value; autoStartedRef.current = true; setSelectedUdid(n); void stopSession().then(() => startSession(n)); }} style={inputStyle}>
                            {devices.map((d) => (
                                <option key={d.udid} value={d.udid}>{d.name} · iOS {parseIosVersion(d.runtime)} {d.booted ? '(booted)' : ''}</option>
                            ))}
                        </select>
                        <button type="button" onClick={() => void bootSelected()} style={{ padding: '4px 10px', fontSize: 10, background: 'transparent', border: '1px solid var(--vscode-panel-border)', borderRadius: 3, cursor: 'pointer' }}>Boot</button>
                    </div>
                </div>
            )}
        </div>
    );
};

export default MacIOSSimulatorPanel;
