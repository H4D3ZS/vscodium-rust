import React, { useState, useEffect, useRef } from 'react';
import { invoke } from '../tauri_bridge';
import { listen } from '@tauri-apps/api/event';

interface ConsoleLine {
    text: string;
    stream: 'stdout' | 'stderr' | 'system';
    ts: number;
}

type ViewMode = 'console' | 'display';

const IPhoneEmulatorPanel: React.FC = () => {
    const [isRunning, setIsRunning] = useState(false);
    const [status, setStatus] = useState<'idle' | 'launching' | 'running' | 'error'>('idle');
    const [consoleLines, setConsoleLines] = useState<ConsoleLine[]>([]);
    const [device, setDevice] = useState('iPhone13,2');
    const [diskPath, setDiskPath] = useState('');
    const [achronPath, setAchronPath] = useState('');
    const [ipswPath, setIpswPath] = useState('');
    const [preparing, setPreparing] = useState(false);
    const consoleEndRef = useRef<HTMLDivElement>(null);
    const [autoScroll, setAutoScroll] = useState(true);
    const [viewMode, setViewMode] = useState<ViewMode>('console');
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
        setViewMode('console');
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

    return (
        <div style={{ display: 'flex', flexDirection: 'column', height: '100%', background: 'var(--vscode-editor-background, #0a0a0a)', color: 'var(--vscode-editor-foreground, #f0f0f0)', fontFamily: 'monospace' }}>

            {/* Header */}
            <div style={{ display: 'flex', alignItems: 'center', gap: 8, padding: '8px 12px', background: 'var(--vscode-editor-background, #111)', borderBottom: '1px solid #222' }}>
                <span style={{ fontSize: 14 }}>🍎</span>
                <span style={{ fontSize: 11, fontWeight: 700, flex: 1 }}>iPhone Emulator (acheron)</span>
                <span style={{
                    fontSize: 9, padding: '2px 6px', borderRadius: 3, fontWeight: 600,
                    background: status === 'running' ? '#166534' : status === 'launching' ? '#92400e' : status === 'error' ? '#7f1d1d' : '#1e293b',
                    color: status === 'running' ? '#4ade80' : status === 'launching' ? '#fbbf24' : status === 'error' ? '#f87171' : '#94a3b8',
                }}>
                    {status === 'idle' ? 'IDLE' : status === 'launching' ? 'LAUNCHING…' : status === 'running' ? 'RUNNING' : 'ERROR'}
                </span>
            </div>

            {/* Config (only when idle) */}
            {!isRunning && (
                <div style={{ padding: '10px 12px', background: 'var(--vscode-editor-background, #111)', borderBottom: '1px solid #222', display: 'flex', flexDirection: 'column', gap: 6 }}>
                    <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
                        <label style={{ fontSize: 10, opacity: 0.7, width: 70, flexShrink: 0 }}>Device</label>
                        <input
                            value={device}
                            onChange={e => setDevice(e.target.value)}
                            placeholder="iPhone13,2"
                            style={{ flex: 1, fontSize: 11, padding: '3px 6px', background: 'var(--vscode-editor-background, #1e1e1e)', border: '1px solid var(--vscode-panel-border, #333)', borderRadius: 3, color: 'var(--vscode-editor-foreground, #fff)', outline: 'none' }}
                        />
                    </div>
                    <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
                        <label style={{ fontSize: 10, opacity: 0.7, width: 70, flexShrink: 0 }}>Disk image</label>
                        <input
                            value={diskPath}
                            onChange={e => setDiskPath(e.target.value)}
                            placeholder="Path to .img file (optional)"
                            style={{ flex: 1, fontSize: 11, padding: '3px 6px', background: 'var(--vscode-editor-background, #1e1e1e)', border: '1px solid var(--vscode-panel-border, #333)', borderRadius: 3, color: 'var(--vscode-editor-foreground, #fff)', outline: 'none' }}
                        />
                    </div>
                    <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
                        <label style={{ fontSize: 10, opacity: 0.7, width: 70, flexShrink: 0 }}>Acheron dir</label>
                        <input
                            value={achronPath}
                            onChange={e => setAchronPath(e.target.value)}
                                    placeholder="C:\...\Virtual-iPhone-Emulator"
                            style={{ flex: 1, fontSize: 11, padding: '3px 6px', background: 'var(--vscode-editor-background, #1e1e1e)', border: '1px solid var(--vscode-panel-border, #333)', borderRadius: 3, color: 'var(--vscode-editor-foreground, #fff)', outline: 'none' }}
                        />
                    </div>
                    <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
                        <label style={{ fontSize: 10, opacity: 0.7, width: 70, flexShrink: 0 }}>IPSW</label>
                        <input
                            value={ipswPath}
                            onChange={e => setIpswPath(e.target.value)}
                            placeholder="Path to .ipsw (real firmware → real ramdisk)"
                            style={{ flex: 1, fontSize: 11, padding: '3px 6px', background: 'var(--vscode-editor-background, #1e1e1e)', border: '1px solid var(--vscode-panel-border, #333)', borderRadius: 3, color: 'var(--vscode-editor-foreground, #fff)', outline: 'none' }}
                        />
                    </div>
                    <div style={{ display: 'flex', gap: 6, marginTop: 2 }}>
                        <button
                            onClick={handlePrepareFirmware}
                            disabled={preparing}
                            style={{
                                flex: 2, padding: '5px 0', fontSize: 10, fontWeight: 600,
                                background: preparing ? '#3a2f1e' : '#1e3a2f', color: preparing ? '#fbbf24' : '#4ade80',
                                border: '1px solid #2b6c4f', borderRadius: 4, cursor: preparing ? 'wait' : 'pointer',
                            }}
                            title="Extract a real IPSW into a prepared firmware dir (kernelcache + devicetree + real ramdisk). The real path to genuine userspace boot."
                        >
                            {preparing ? '⟳ Preparing IPSW…' : '📦 Prepare Firmware (IPSW → ramdisk)'}
                        </button>
                        <button
                            onClick={handleCreateStubRamdisk}
                            style={{
                                flex: 1, padding: '5px 0', fontSize: 10, fontWeight: 600,
                                background: '#1e3a5f', color: '#63b3ed', border: '1px solid #2b6cb0', borderRadius: 4, cursor: 'pointer',
                            }}
                            title="Generate a minimal test ramdisk so the kernel can reach userspace (rd=md0)"
                        >
                            🧪 Stub Ramdisk
                        </button>
                    </div>
                    <button
                        onClick={handleLaunch}
                        disabled={status === 'launching'}
                        style={{
                            marginTop: 4, padding: '6px 0', fontSize: 11, fontWeight: 700,
                            background: '#166534', color: '#4ade80', border: '1px solid #14532d',
                            borderRadius: 4, cursor: 'pointer',
                        }}
                    >
                        {status === 'launching' ? '⟳ Launching…' : '▶ Launch Emulator'}
                    </button>
                </div>
            )}

            {/* View toggle */}
            <div style={{ display: 'flex', background: 'var(--vscode-editor-background, #0a0a0a)', borderBottom: '1px solid #1e293b' }}>
                {(['console', 'display'] as ViewMode[]).map(m => (
                    <button key={m} onClick={() => setViewMode(m)} style={{
                        flex: 1, padding: '4px', fontSize: 10, fontWeight: 600, border: 'none', cursor: 'pointer',
                        background: viewMode === m ? '#1e3a5f' : 'transparent',
                        color: viewMode === m ? '#63b3ed' : '#64748b',
                        borderBottom: viewMode === m ? '2px solid #63b3ed' : '2px solid transparent',
                    }}>
                        {m === 'console' ? '📟 Serial Console' : '📺 Display'}
                    </button>
                ))}
            </div>

            {/* Framebuffer display — wrapped in an iPhone device bezel for
                Xcode-Simulator-style presentation. */}
            {viewMode === 'display' && (
                <div style={{ flex: 1, display: 'flex', alignItems: 'center', justifyContent: 'center', background: 'radial-gradient(circle at 50% 30%, #15151b, #050507)', minHeight: 0, padding: 16 }}>
                    {frameDataUrl ? (
                        <div
                            style={{
                                position: 'relative',
                                height: '100%',
                                aspectRatio: '1290 / 2796',
                                background: '#000',
                                borderRadius: '14% / 6.5%',
                                padding: '2.2%',
                                boxShadow: '0 0 0 2px #2a2a2e, 0 18px 50px rgba(0,0,0,0.7)',
                                boxSizing: 'border-box',
                            }}
                        >
                            <img
                                ref={displayImgRef}
                                src={frameDataUrl}
                                alt="Emulator display"
                                draggable={false}
                                onMouseDown={handleDisplayPointerDown}
                                onMouseMove={handleDisplayPointerMove}
                                onMouseUp={handleDisplayPointerUp}
                                style={{
                                    width: '100%', height: '100%', objectFit: 'fill',
                                    borderRadius: '12% / 5.6%',
                                    cursor: isRunning ? 'pointer' : 'default', userSelect: 'none',
                                    display: 'block',
                                }}
                            />
                            {/* Dynamic Island */}
                            <div style={{
                                position: 'absolute', top: '3.2%', left: '50%', transform: 'translateX(-50%)',
                                width: '30%', height: '3.4%', background: '#000', borderRadius: 999,
                                pointerEvents: 'none',
                            }} />
                        </div>
                    ) : (
                        <div style={{ color: '#334155', textAlign: 'center', fontSize: 11 }}>
                            {isRunning ? '⟳ Waiting for first frame…' : 'Launch emulator to see display output'}
                        </div>
                    )}
                </div>
            )}

            {/* Serial Console Output */}
            <div style={{ flex: 1, flexDirection: 'column', minHeight: 0, display: viewMode === 'console' ? 'flex' : 'none' }}>
                <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', padding: '4px 12px', background: '#0f172a', borderBottom: '1px solid #1e293b' }}>
                    <span style={{ fontSize: 10, fontWeight: 600, color: '#64748b' }}>DARWIN SERIAL CONSOLE</span>
                    <div style={{ display: 'flex', gap: 8 }}>
                        <button
                            onClick={() => setAutoScroll(v => !v)}
                            style={{ fontSize: 9, padding: '1px 6px', background: autoScroll ? '#1e3a5f' : 'transparent', border: '1px solid #334155', borderRadius: 2, color: '#94a3b8', cursor: 'pointer' }}
                        >
                            {autoScroll ? '⬇ Auto' : 'Manual'}
                        </button>
                        <button
                            onClick={() => setConsoleLines([])}
                            style={{ fontSize: 9, padding: '1px 6px', background: 'transparent', border: '1px solid #334155', borderRadius: 2, color: '#94a3b8', cursor: 'pointer' }}
                        >
                            Clear
                        </button>
                        {isRunning && (
                            <button
                                onClick={handleStop}
                                style={{ fontSize: 9, padding: '1px 8px', background: '#7f1d1d', border: '1px solid #991b1b', borderRadius: 2, color: '#fca5a5', cursor: 'pointer', fontWeight: 600 }}
                            >
                                ■ Stop
                            </button>
                        )}
                    </div>
                </div>
                <div style={{ flex: 1, overflowY: 'auto', padding: '6px 12px', lineHeight: '1.5', fontSize: 10 }}>
                    {consoleLines.length === 0 && (
                        <div style={{ color: '#334155', marginTop: 20, textAlign: 'center' }}>
                            No output yet. Launch the emulator to see Darwin boot logs.
                        </div>
                    )}
                    {consoleLines.map((line, i) => (
                        <div key={i} style={{ color: lineColor(line.stream), whiteSpace: 'pre-wrap', wordBreak: 'break-all' }}>
                            {line.text}
                        </div>
                    ))}
                    <div ref={consoleEndRef} />
                </div>
            </div>
        </div>
    );
};

export default IPhoneEmulatorPanel;
