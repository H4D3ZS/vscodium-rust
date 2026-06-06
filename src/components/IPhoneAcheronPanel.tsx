import React, { useState, useEffect, useRef } from 'react';
import { invoke } from '../tauri_bridge';
import { listen } from '@tauri-apps/api/event';

interface ConsoleLine {
    text: string;
    stream: 'stdout' | 'stderr' | 'system';
    ts: number;
}

type ViewMode = 'console' | 'display';

const IPhoneAcheronPanel: React.FC = () => {
    const [isRunning, setIsRunning] = useState(false);
    const [status, setStatus] = useState<'idle' | 'launching' | 'running' | 'error'>('idle');
    const [consoleLines, setConsoleLines] = useState<ConsoleLine[]>([]);
    const [device, setDevice] = useState('iPhone13,2');
    const [diskPath, setDiskPath] = useState('');
    const [achronPath, setAchronPath] = useState('');
    const [binaryStatus, setBinaryStatus] = useState<string | null>(null);
    const [ipswPath, setIpswPath] = useState('');
    const [preparing, setPreparing] = useState(false);
    const consoleEndRef = useRef<HTMLDivElement>(null);
    const [autoScroll, setAutoScroll] = useState(true);
    // Xcode-Simulator UI: the device is always shown. Advanced setup + serial console
    // are toggled drawers (hidden by default) so the panel reads like the real simulator.
    const [showAdvanced, setShowAdvanced] = useState(false);
    const [showConsole, setShowConsole] = useState(false);
    const [frameDataUrl, setFrameDataUrl] = useState<string | null>(null);
    const displayImgRef = useRef<HTMLImageElement>(null);

    // iPhone13,2 (iPhone 12) native panel resolution in device pixels.
    const DEVICE_W = 1290;
    const DEVICE_H = 2796;

    // Translate a pointer event over the rendered <img> into device-pixel
    // coordinates, accounting for object-fit: contain letterboxing.
    const toDeviceCoords = (e: React.MouseEvent<HTMLImageElement>): { x: number; y: number } | null => {
        const img = displayImgRef.current;
        if (!img) return null;
        const rect = img.getBoundingClientRect();
        // Scale used by object-fit: contain
        const scale = Math.min(rect.width / DEVICE_W, rect.height / DEVICE_H);
        const drawnW = DEVICE_W * scale;
        const drawnH = DEVICE_H * scale;
        const offX = (rect.width - drawnW) / 2;
        const offY = (rect.height - drawnH) / 2;
        const px = (e.clientX - rect.left - offX) / scale;
        const py = (e.clientY - rect.top - offY) / scale;
        if (px < 0 || py < 0 || px >= DEVICE_W || py >= DEVICE_H) return null;
        return { x: Math.round(px), y: Math.round(py) };
    };

    const sendTouch = async (x: number, y: number, phase: number) => {
        try {
            await invoke('send_iphone_touch', { x, y, finger: 0, phase });
        } catch { /* emulator not running */ }
    };

    const handleDisplayPointerDown = (e: React.MouseEvent<HTMLImageElement>) => {
        const c = toDeviceCoords(e);
        if (c) sendTouch(c.x, c.y, 0 /* Began */);
    };
    const handleDisplayPointerMove = (e: React.MouseEvent<HTMLImageElement>) => {
        if (e.buttons !== 1) return;
        const c = toDeviceCoords(e);
        if (c) sendTouch(c.x, c.y, 1 /* Moved */);
    };
    const handleDisplayPointerUp = (e: React.MouseEvent<HTMLImageElement>) => {
        const c = toDeviceCoords(e);
        if (c) sendTouch(c.x, c.y, 3 /* Ended */);
    };

    useEffect(() => {
        import('../application/emulator/resolveEmulatorProject').then(async ({ resolveEmulatorProjectPath, probeEmulatorBinary }) => {
            const root = await resolveEmulatorProjectPath();
            setAchronPath(root);
            const bin = await probeEmulatorBinary(root);
            setBinaryStatus(bin ? `Found: ${bin.split(/[\\/]/).pop()}` : 'Build acheron first: cmake -B build && cmake --build build --config Release');
        }).catch(console.error);
    }, []);

    // Auto-scroll console
    useEffect(() => {
        if (autoScroll) consoleEndRef.current?.scrollIntoView({ behavior: 'smooth' });
    }, [consoleLines, autoScroll]);

    // Listen for framebuffer frames
    useEffect(() => {
        const unlistenFrame = listen('emulator-frame', (event: any) => {
            const { dataUrl } = event.payload ?? {};
            if (dataUrl) setFrameDataUrl(dataUrl);
        });
        return () => { unlistenFrame.then(f => f()); };
    }, []);

    // Listen for firmware-prepared → auto-fill the disk/ramdisk path
    useEffect(() => {
        const un = listen('ios-firmware-prepared', (event: any) => {
            const { ok, ramdisk } = event.payload ?? {};
            setPreparing(false);
            if (ok && ramdisk) {
                setDiskPath(ramdisk);
                setConsoleLines(prev => [...prev, { text: `Ramdisk ready → disk path set to ${ramdisk}`, stream: 'system', ts: Date.now() }]);
            }
        });
        return () => { un.then(f => f()); };
    }, []);

    // Listen for console output from emulator process
    useEffect(() => {
        const unlistenPromise = listen('emulator-console', (event: any) => {
            const { line, stream } = event.payload ?? {};
            if (!line) return;
            setConsoleLines(prev => [...prev.slice(-1000), { text: line, stream: stream ?? 'stdout', ts: Date.now() }]);
            if (stream === 'system' && line.includes('exited')) {
                setIsRunning(false);
                setStatus('idle');
            }
        });
        return () => { unlistenPromise.then(f => f()); };
    }, []);

    const handleCreateStubRamdisk = async () => {
        const outPath = (achronPath.trim() || 'C:\\Users\\HADES\\Desktop\\vscodium-rust\\Virtual-iPhone-Emulator') + '\\out\\raw\\initrd.bin';
        try {
            const result = await invoke<string>('create_stub_ramdisk', { outputPath: outPath });
            setConsoleLines(prev => [...prev, { text: result, stream: 'system', ts: Date.now() }]);
            setDiskPath(outPath); // auto-fill disk path
        } catch (e) {
            setConsoleLines(prev => [...prev, { text: `Failed: ${e}`, stream: 'system', ts: Date.now() }]);
        }
    };

    const handlePrepareFirmware = async () => {
        const projectPath = achronPath.trim() || 'C:\\Users\\HADES\\Desktop\\vscodium-rust\\Virtual-iPhone-Emulator';
        if (!ipswPath.trim()) {
            setConsoleLines(prev => [...prev, { text: 'Enter an IPSW path first.', stream: 'system', ts: Date.now() }]);
            return;
        }
        setPreparing(true);
        setShowConsole(true);
        setConsoleLines(prev => [...prev, { text: `Preparing firmware from ${ipswPath}…`, stream: 'system', ts: Date.now() }]);
        try {
            await invoke('prepare_ios_firmware', { projectPath, ipswPath: ipswPath.trim() });
        } catch (e) {
            setPreparing(false);
            setConsoleLines(prev => [...prev, { text: `Prepare failed: ${e}`, stream: 'system', ts: Date.now() }]);
        }
    };

    const handleLaunch = async () => {
        setStatus('launching');
        setConsoleLines([{ text: `Launching acheron for device ${device}…`, stream: 'system', ts: Date.now() }]);
        try {
            // Resolve project path: use the Virtual-iPhone-Emulator directory
            const projectPath = achronPath.trim() || 'C:\\Users\\HADES\\Desktop\\vscodium-rust\\Virtual-iPhone-Emulator';
            await invoke('launch_iphone_emulator', {
                projectPath,
                device: device || undefined,
                diskPath: diskPath.trim() || undefined,
            });
            setIsRunning(true);
            setStatus('running');
        } catch (e) {
            setStatus('error');
            setConsoleLines(prev => [...prev, { text: `Launch failed: ${e}`, stream: 'system', ts: Date.now() }]);
        }
    };

    const handleStop = async () => {
        try { await invoke('stop_iphone_emulator'); } catch { /* ignore */ }
        setIsRunning(false);
        setStatus('idle');
        setConsoleLines(prev => [...prev, { text: 'Emulator stopped.', stream: 'system', ts: Date.now() }]);
    };

    const lineColor = (stream: string) => {
        if (stream === 'stderr') return '#f87171';
        if (stream === 'system') return '#94a3b8';
        return '#d1fae5';
    };

    // ── Xcode-Simulator chrome ──────────────────────────────────────────────
    const IOS_VERSION = '26.1';
    const DEVICE_NAMES: Record<string, string> = {
        'iPhone13,2': 'iPhone 12', 'iPhone14,2': 'iPhone 13 Pro', 'iPhone15,2': 'iPhone 14 Pro',
        'iPhone16,1': 'iPhone 15 Pro', 'iPhone17,1': 'iPhone 16 Pro', 'iPhone17,3': 'iPhone 16',
    };
    const deviceDisplayName = DEVICE_NAMES[device.trim()] || (device.trim() || 'iPhone');

    const handleHome = async () => { try { await invoke('iphone_home_button'); } catch { /* best-effort */ } };
    const handleRotate = async () => { try { await invoke('iphone_rotate'); } catch { /* best-effort */ } };
    const handleScreenshot = () => {
        // Save the current framebuffer — a real device screenshot, no backend needed.
        if (!frameDataUrl) return;
        const a = document.createElement('a');
        a.href = frameDataUrl;
        a.download = `${deviceDisplayName.replace(/\s+/g, '_')}_${Date.now()}.png`;
        a.click();
    };
    const toolbarIcon: React.CSSProperties = { fontFamily: 'codicon', fontStyle: 'normal', fontSize: 14, cursor: 'pointer', opacity: 0.75, color: 'var(--vscode-foreground, #ddd)' };
    const inputStyle: React.CSSProperties = { flex: 1, fontSize: 11, padding: '3px 6px', background: 'var(--vscode-editor-background, #1e1e1e)', border: '1px solid var(--vscode-panel-border, #333)', borderRadius: 3, color: 'var(--vscode-editor-foreground, #fff)', outline: 'none' };

    return (
        <div style={{ display: 'flex', flexDirection: 'column', height: '100%', position: 'relative', background: 'var(--vscode-editor-background, #1e1e1e)', color: 'var(--vscode-foreground, #f0f0f0)', overflow: 'hidden' }}>

            {/* Xcode-Simulator toolbar: window dots · device/iOS · launch/home/shot/rotate/console/advanced */}
            <div style={{ display: 'flex', alignItems: 'center', gap: 10, padding: '6px 12px', background: 'var(--vscode-titleBar-activeBackground, #1c1c1e)', borderBottom: '1px solid #2a2a2e', flexShrink: 0, zIndex: 5 }}>
                <div style={{ display: 'flex', gap: 6 }}>
                    <span style={{ width: 11, height: 11, borderRadius: '50%', background: '#ff5f57' }} />
                    <span style={{ width: 11, height: 11, borderRadius: '50%', background: '#febc2e' }} />
                    <span style={{ width: 11, height: 11, borderRadius: '50%', background: '#28c840' }} />
                </div>
                <div style={{ flex: 1, textAlign: 'center', lineHeight: 1.05, fontFamily: 'var(--font-ui)' }}>
                    <div style={{ fontSize: 12, fontWeight: 700 }}>{deviceDisplayName}</div>
                    <div style={{ fontSize: 9, opacity: 0.55 }}>iOS {IOS_VERSION}</div>
                </div>
                <div style={{ display: 'flex', gap: 14, alignItems: 'center' }}>
                    {isRunning
                        ? <i className="codicon codicon-debug-stop" title="Stop" onClick={handleStop} style={{ ...toolbarIcon, color: '#ff5f57', opacity: 0.9 }} />
                        : <i className="codicon codicon-play" title="Launch" onClick={status === 'launching' ? undefined : handleLaunch} style={{ ...toolbarIcon, color: '#28c840', opacity: status === 'launching' ? 0.4 : 0.9 }} />}
                    <i className="codicon codicon-home" title="Home" onClick={handleHome} style={toolbarIcon} />
                    <i className="codicon codicon-device-camera" title="Screenshot" onClick={handleScreenshot} style={toolbarIcon} />
                    <i className="codicon codicon-screen-normal" title="Rotate" onClick={handleRotate} style={toolbarIcon} />
                    <i className="codicon codicon-terminal" title="Serial console" onClick={() => setShowConsole(v => !v)} style={{ ...toolbarIcon, color: showConsole ? '#63b3ed' : toolbarIcon.color }} />
                    <i className="codicon codicon-settings-gear" title="Advanced setup" onClick={() => setShowAdvanced(v => !v)} style={{ ...toolbarIcon, color: showAdvanced ? '#63b3ed' : toolbarIcon.color }} />
                </div>
            </div>

            {/* THE DEVICE — always centered, like the real Xcode Simulator */}
            <div style={{ flex: 1, display: 'flex', alignItems: 'center', justifyContent: 'center', minHeight: 0, padding: 16 }}>
                <div style={{
                    position: 'relative', height: '100%', aspectRatio: '1290 / 2796',
                    background: '#0b0b0d', borderRadius: '14% / 6.5%', padding: '2.2%',
                    boxShadow: '0 0 0 2px #2a2a2e, 0 18px 50px rgba(0,0,0,0.7)', boxSizing: 'border-box',
                }}>
                    {frameDataUrl ? (
                        <img
                            ref={displayImgRef}
                            src={frameDataUrl}
                            alt="iPhone display"
                            draggable={false}
                            onMouseDown={handleDisplayPointerDown}
                            onMouseMove={handleDisplayPointerMove}
                            onMouseUp={handleDisplayPointerUp}
                            style={{ width: '100%', height: '100%', objectFit: 'fill', borderRadius: '12% / 5.6%', cursor: 'pointer', userSelect: 'none', display: 'block' }}
                        />
                    ) : (
                        <div
                            onClick={status === 'launching' ? undefined : handleLaunch}
                            style={{ width: '100%', height: '100%', borderRadius: '12% / 5.6%', background: '#000', display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', gap: 12, cursor: status === 'launching' ? 'default' : 'pointer' }}
                        >
                            {status === 'launching' ? (
                                <>
                                    {/* Real iPhone boot screen — black + Apple logo + boot bar.
                                        Once the emulator streams real frames, iOS renders its own
                                        boot; this is the placeholder until the first frame. */}
                                    <style>{`@keyframes ios-boot{0%{width:8%}60%{width:72%}100%{width:96%}}`}</style>
                                    <svg viewBox="0 0 814 1000" width="56" height="69" fill="#f5f5f7" aria-label="Apple">
                                        <path d="M788.1 340.9c-5.8 4.5-108.2 62.2-108.2 190.5 0 148.4 130.3 200.9 134.2 202.2-.6 3.2-20.7 71.9-68.7 141.9-42.8 61.6-87.5 123.1-155.5 123.1s-85.5-39.5-164-39.5c-76.5 0-103.7 40.8-165.9 40.8s-105.6-57-155.5-127C46.7 790.7 0 663 0 541.8c0-194.4 126.4-297.5 250.8-297.5 66.1 0 121.2 43.4 162.7 43.4 39.5 0 101.1-46 176.3-46 28.5 0 130.9 2.6 198.3 99.2zm-234-181.5c31.1-36.9 53.1-88.1 53.1-139.3 0-7.1-.6-14.3-1.9-20.1-50.6 1.9-110.8 33.7-147.1 75.8-28.5 32.4-55.1 83.6-55.1 135.5 0 7.8 1.3 15.6 1.9 18.1 3.2.6 8.4 1.3 13.6 1.3 45.4 0 102.5-30.4 135.5-71.2z"/>
                                    </svg>
                                    <div style={{ position: 'absolute', bottom: '13%', width: '34%', height: 3, background: 'rgba(255,255,255,0.16)', borderRadius: 2, overflow: 'hidden' }}>
                                        <div style={{ height: '100%', background: '#fff', borderRadius: 2, animation: 'ios-boot 2.4s ease-in-out infinite' }} />
                                    </div>
                                </>
                            ) : isRunning ? (
                                <div style={{ color: '#94a3b8', fontSize: 12 }}>⟳ Waiting for first frame…</div>
                            ) : (
                                <>
                                    <i className="codicon codicon-play-circle" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: 46, color: '#3b82f6' }} />
                                    <div style={{ color: '#cbd5e1', fontSize: 13, fontWeight: 600 }}>Tap to launch {deviceDisplayName}</div>
                                    <div style={{ color: '#475569', fontSize: 10 }}>iOS {IOS_VERSION} · real-time emulator</div>
                                </>
                            )}
                        </div>
                    )}
                    {/* Dynamic Island */}
                    <div style={{ position: 'absolute', top: '3.2%', left: '50%', transform: 'translateX(-50%)', width: '30%', height: '3.4%', background: '#000', borderRadius: 999, pointerEvents: 'none' }} />
                </div>
            </div>

            {/* Advanced setup — top overlay drawer (gear toggle) */}
            {showAdvanced && (
                <div style={{ position: 'absolute', top: 42, left: 0, right: 0, zIndex: 20, padding: '10px 12px', background: 'rgba(17,17,19,0.97)', borderBottom: '1px solid #222', display: 'flex', flexDirection: 'column', gap: 6, boxShadow: '0 10px 28px rgba(0,0,0,0.55)' }}>
                    {binaryStatus && (
                        <div style={{ fontSize: 10, opacity: 0.65, padding: '2px 0' }}>{binaryStatus}</div>
                    )}
                    <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
                        <label style={{ fontSize: 10, opacity: 0.7, width: 70, flexShrink: 0 }}>Device</label>
                        <input value={device} onChange={e => setDevice(e.target.value)} placeholder="iPhone17,1" style={inputStyle} />
                    </div>
                    <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
                        <label style={{ fontSize: 10, opacity: 0.7, width: 70, flexShrink: 0 }}>Disk image</label>
                        <input value={diskPath} onChange={e => setDiskPath(e.target.value)} placeholder="Path to .img (optional)" style={inputStyle} />
                    </div>
                    <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
                        <label style={{ fontSize: 10, opacity: 0.7, width: 70, flexShrink: 0 }}>Acheron dir</label>
                        <input value={achronPath} onChange={e => setAchronPath(e.target.value)} placeholder="C:\...\Virtual-iPhone-Emulator" style={inputStyle} />
                    </div>
                    <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
                        <label style={{ fontSize: 10, opacity: 0.7, width: 70, flexShrink: 0 }}>IPSW</label>
                        <input value={ipswPath} onChange={e => setIpswPath(e.target.value)} placeholder="Path to .ipsw" style={inputStyle} />
                    </div>
                    <div style={{ display: 'flex', gap: 6, marginTop: 2 }}>
                        <button onClick={handlePrepareFirmware} disabled={preparing} style={{ flex: 2, padding: '5px 0', fontSize: 10, fontWeight: 600, background: preparing ? '#3a2f1e' : '#1e3a2f', color: preparing ? '#fbbf24' : '#4ade80', border: '1px solid #2b6c4f', borderRadius: 4, cursor: preparing ? 'wait' : 'pointer' }}>
                            {preparing ? '⟳ Preparing IPSW…' : '📦 Prepare Firmware'}
                        </button>
                        <button onClick={handleCreateStubRamdisk} style={{ flex: 1, padding: '5px 0', fontSize: 10, fontWeight: 600, background: '#1e3a5f', color: '#63b3ed', border: '1px solid #2b6cb0', borderRadius: 4, cursor: 'pointer' }}>
                            🧪 Stub Ramdisk
                        </button>
                    </div>
                </div>
            )}

            {/* Serial console — bottom overlay drawer (terminal toggle) */}
            {showConsole && (
                <div style={{ position: 'absolute', bottom: 0, left: 0, right: 0, height: '45%', zIndex: 20, display: 'flex', flexDirection: 'column', background: 'rgba(10,12,18,0.97)', borderTop: '1px solid #1e293b', boxShadow: '0 -10px 28px rgba(0,0,0,0.55)' }}>
                    <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', padding: '4px 12px', background: '#0f172a', borderBottom: '1px solid #1e293b' }}>
                        <span style={{ fontSize: 10, fontWeight: 600, color: '#64748b' }}>DARWIN SERIAL CONSOLE</span>
                        <div style={{ display: 'flex', gap: 8 }}>
                            <button onClick={() => setAutoScroll(v => !v)} style={{ fontSize: 9, padding: '1px 6px', background: autoScroll ? '#1e3a5f' : 'transparent', border: '1px solid #334155', borderRadius: 2, color: '#94a3b8', cursor: 'pointer' }}>{autoScroll ? '⬇ Auto' : 'Manual'}</button>
                            <button onClick={() => setConsoleLines([])} style={{ fontSize: 9, padding: '1px 6px', background: 'transparent', border: '1px solid #334155', borderRadius: 2, color: '#94a3b8', cursor: 'pointer' }}>Clear</button>
                            <button onClick={() => setShowConsole(false)} style={{ fontSize: 9, padding: '1px 6px', background: 'transparent', border: '1px solid #334155', borderRadius: 2, color: '#94a3b8', cursor: 'pointer' }}>✕</button>
                        </div>
                    </div>
                    <div style={{ flex: 1, overflowY: 'auto', padding: '6px 12px', lineHeight: '1.5', fontSize: 10, fontFamily: 'var(--font-mono, monospace)' }}>
                        {consoleLines.length === 0 && (
                            <div style={{ color: '#334155', marginTop: 20, textAlign: 'center' }}>No output yet. Launch to see Darwin boot logs.</div>
                        )}
                        {consoleLines.map((line, i) => (
                            <div key={i} style={{ color: lineColor(line.stream), whiteSpace: 'pre-wrap', wordBreak: 'break-all' }}>{line.text}</div>
                        ))}
                        <div ref={consoleEndRef} />
                    </div>
                </div>
            )}
        </div>
    );
};

export default IPhoneAcheronPanel;
