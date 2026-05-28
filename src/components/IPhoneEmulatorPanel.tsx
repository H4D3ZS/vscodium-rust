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
    const consoleEndRef = useRef<HTMLDivElement>(null);
    const [autoScroll, setAutoScroll] = useState(true);
    const [viewMode, setViewMode] = useState<ViewMode>('console');
    const [frameDataUrl, setFrameDataUrl] = useState<string | null>(null);

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
        <div style={{ display: 'flex', flexDirection: 'column', height: '100%', background: '#0a0a0a', color: '#f0f0f0', fontFamily: 'monospace' }}>

            {/* Header */}
            <div style={{ display: 'flex', alignItems: 'center', gap: 8, padding: '8px 12px', background: '#111', borderBottom: '1px solid #222' }}>
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
                <div style={{ padding: '10px 12px', background: '#111', borderBottom: '1px solid #222', display: 'flex', flexDirection: 'column', gap: 6 }}>
                    <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
                        <label style={{ fontSize: 10, opacity: 0.7, width: 70, flexShrink: 0 }}>Device</label>
                        <input
                            value={device}
                            onChange={e => setDevice(e.target.value)}
                            placeholder="iPhone13,2"
                            style={{ flex: 1, fontSize: 11, padding: '3px 6px', background: '#1e1e1e', border: '1px solid #333', borderRadius: 3, color: '#fff', outline: 'none' }}
                        />
                    </div>
                    <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
                        <label style={{ fontSize: 10, opacity: 0.7, width: 70, flexShrink: 0 }}>Disk image</label>
                        <input
                            value={diskPath}
                            onChange={e => setDiskPath(e.target.value)}
                            placeholder="Path to .img file (optional)"
                            style={{ flex: 1, fontSize: 11, padding: '3px 6px', background: '#1e1e1e', border: '1px solid #333', borderRadius: 3, color: '#fff', outline: 'none' }}
                        />
                    </div>
                    <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
                        <label style={{ fontSize: 10, opacity: 0.7, width: 70, flexShrink: 0 }}>Acheron dir</label>
                        <input
                            value={achronPath}
                            onChange={e => setAchronPath(e.target.value)}
                                    placeholder="C:\...\Virtual-iPhone-Emulator"
                            style={{ flex: 1, fontSize: 11, padding: '3px 6px', background: '#1e1e1e', border: '1px solid #333', borderRadius: 3, color: '#fff', outline: 'none' }}
                        />
                    </div>
                    <div style={{ display: 'flex', gap: 6, marginTop: 2 }}>
                        <button
                            onClick={handleCreateStubRamdisk}
                            style={{
                                flex: 1, padding: '5px 0', fontSize: 10, fontWeight: 600,
                                background: '#1e3a5f', color: '#63b3ed', border: '1px solid #2b6cb0', borderRadius: 4, cursor: 'pointer',
                            }}
                            title="Generate a minimal test ramdisk so the kernel can reach userspace (rd=md0)"
                        >
                            🧪 Create Stub Ramdisk
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
            <div style={{ display: 'flex', background: '#0a0a0a', borderBottom: '1px solid #1e293b' }}>
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

            {/* Framebuffer display */}
            {viewMode === 'display' && (
                <div style={{ flex: 1, display: 'flex', alignItems: 'center', justifyContent: 'center', background: '#000', minHeight: 0 }}>
                    {frameDataUrl ? (
                        <img src={frameDataUrl} alt="Emulator display" style={{ maxWidth: '100%', maxHeight: '100%', objectFit: 'contain' }} />
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
