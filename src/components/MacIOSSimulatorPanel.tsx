import React, { useCallback, useEffect, useRef, useState } from 'react';
import { listen } from '@tauri-apps/api/event';
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
};

/**
 * macOS-only Xcode iOS Simulator panel (Codex++-style headless mirror).
 * @see https://github.com/b-nnett/codex-plusplus-ios-simulator
 */
const MacIOSSimulatorPanel: React.FC = () => {
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const imgRef = useRef<HTMLImageElement | null>(null);
    const [devices, setDevices] = useState<SimDevice[]>([]);
    const [selectedUdid, setSelectedUdid] = useState('');
    const [deviceName, setDeviceName] = useState('iPhone');
    const [preflight, setPreflight] = useState<Preflight | null>(null);
    const [running, setRunning] = useState(false);
    const [connecting, setConnecting] = useState(false);
    const [fps, setFps] = useState(0);
    const [error, setError] = useState('');
    const [showLegacy, setShowLegacy] = useState(false);
    const frameTimes = useRef<number[]>([]);
    const viewport = useRef({ w: 1170, h: 2532 });

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
        invoke<Preflight>('ios_sim_preflight').then(setPreflight).catch(() => {});
        void refreshDevices();
    }, [refreshDevices]);

    const startMirror = async () => {
        setConnecting(true);
        setError('');
        try {
            await invoke('ios_sim_start_mirror', {
                udid: selectedUdid || null,
                autoBoot: true,
            });
            setRunning(true);
        } catch (e) {
            setError(String(e));
        } finally {
            setConnecting(false);
        }
    };

    const stopMirror = async () => {
        await invoke('ios_sim_stop_mirror').catch(() => {});
        setRunning(false);
    };

    useEffect(() => {
        let unlisten: (() => void) | undefined;
        listen<{ base64: string; width: number; height: number; deviceName?: string }>(
            'ios-simulator:frame',
            (ev) => {
                const { base64: b64, width, height, deviceName: dn } = ev.payload;
                if (width && height) viewport.current = { w: width, h: height };
                if (dn) setDeviceName(dn);
                const canvas = canvasRef.current;
                if (!canvas) return;
                const ctx = canvas.getContext('2d');
                if (!ctx) return;
                const img = imgRef.current ?? new Image();
                imgRef.current = img;
                img.onload = () => {
                    canvas.width = img.width;
                    canvas.height = img.height;
                    ctx.drawImage(img, 0, 0);
                    setConnecting(false);
                    const now = Date.now();
                    frameTimes.current.push(now);
                    frameTimes.current = frameTimes.current.filter((t) => t > now - 1000);
                    setFps(frameTimes.current.length);
                };
                img.src = `data:image/jpeg;base64,${b64}`;
            },
        ).then((fn) => {
            unlisten = fn;
        });
        return () => {
            unlisten?.();
            invoke('ios_sim_stop_mirror').catch(() => {});
        };
    }, []);

    const mapTouch = (e: React.MouseEvent<HTMLCanvasElement>, phase: string) => {
        const canvas = canvasRef.current;
        if (!canvas || canvas.width === 0) return;
        const rect = canvas.getBoundingClientRect();
        const x = (e.clientX - rect.left) / rect.width;
        const y = (e.clientY - rect.top) / rect.height;
        if (x < 0 || y < 0 || x > 1 || y > 1) return;
        void invoke('ios_sim_send_touch', { xRatio: x, yRatio: y, phase }).catch(() => {});
    };

    if (showLegacy) {
        return (
            <div style={{ display: 'flex', flexDirection: 'column', height: '100%', minHeight: 0 }}>
                <div style={{ padding: '6px 12px', borderBottom: '1px solid var(--vscode-panel-border)' }}>
                    <button type="button" onClick={() => setShowLegacy(false)} style={{ fontSize: 10 }}>
                        ← Back to Xcode Simulator
                    </button>
                </div>
                <IPhoneAcheronPanel />
            </div>
        );
    }

    return (
        <div style={{ display: 'flex', flexDirection: 'column', height: '100%', minHeight: 0 }}>
            <div
                style={{
                    padding: '6px 12px',
                    borderBottom: '1px solid var(--vscode-panel-border)',
                    display: 'flex',
                    alignItems: 'center',
                    gap: 8,
                    flexShrink: 0,
                    flexWrap: 'wrap',
                }}
            >
                <span style={{ fontSize: 11, fontWeight: 600 }}>Xcode Simulator</span>
                <select
                    value={selectedUdid}
                    onChange={(e) => setSelectedUdid(e.target.value)}
                    style={{ flex: 1, minWidth: 120, fontSize: 10, padding: '2px 4px' }}
                >
                    {devices.map((d) => (
                        <option key={d.udid} value={d.udid}>
                            {d.name} {d.booted ? '●' : ''}
                        </option>
                    ))}
                </select>
                {!running ? (
                    <button type="button" disabled={connecting} onClick={() => void startMirror()} style={btn}>
                        {connecting ? 'Starting…' : '▶ Mirror'}
                    </button>
                ) : (
                    <button type="button" onClick={() => void stopMirror()} style={btn}>
                        ■ Stop
                    </button>
                )}
                <button type="button" onClick={() => void invoke('ios_sim_send_home')} style={btn} title="Home">
                    ⌂
                </button>
                <button type="button" onClick={() => setShowLegacy(true)} style={btnSecondary} title="Experimental acheron/vPhone">
                    Legacy
                </button>
                {running && <span style={{ fontSize: 10, opacity: 0.5 }}>{fps} fps</span>}
            </div>

            {preflight && !preflight.ok && preflight.hint && (
                <div style={{ padding: '8px 12px', fontSize: 10, color: '#fbbf24', background: '#422006' }}>
                    {preflight.hint}
                </div>
            )}
            {error && (
                <div style={{ padding: '8px 12px', fontSize: 10, color: '#f87171' }}>{error}</div>
            )}

            <div
                style={{
                    flex: 1,
                    minHeight: 0,
                    background: '#000',
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    position: 'relative',
                }}
            >
                {!running && (
                    <div style={{ position: 'absolute', textAlign: 'center', color: '#888', fontSize: 12, padding: 16 }}>
                        <div style={{ fontWeight: 600, marginBottom: 8 }}>{deviceName}</div>
                        <div style={{ fontSize: 11, opacity: 0.7 }}>
                            Headless mirror via CoreSimulator (no Simulator.app window)
                        </div>
                        <div style={{ fontSize: 10, marginTop: 12, opacity: 0.5 }}>
                            Apple Silicon · Intel · Hackintosh with Xcode
                        </div>
                    </div>
                )}
                {running && connecting && (
                    <div style={{ position: 'absolute', color: '#888', fontSize: 12 }}>Waiting for booted simulator…</div>
                )}
                <canvas
                    ref={canvasRef}
                    onMouseDown={(e) => mapTouch(e, 'down')}
                    onMouseMove={(e) => e.buttons === 1 && mapTouch(e, 'move')}
                    onMouseUp={(e) => mapTouch(e, 'up')}
                    style={{
                        maxWidth: '100%',
                        maxHeight: '100%',
                        objectFit: 'contain',
                        cursor: running ? 'pointer' : 'default',
                        opacity: running ? 1 : 0,
                    }}
                />
            </div>

            <div style={{ padding: '6px 12px', fontSize: 9, opacity: 0.45, borderTop: '1px solid var(--vscode-panel-border)' }}>
                Based on{' '}
                <a href="https://github.com/b-nnett/codex-plusplus-ios-simulator" target="_blank" rel="noreferrer">
                    codex-plusplus-ios-simulator
                </a>{' '}
                (MIT) · temporary until vPhone emulator ships
            </div>
        </div>
    );
};

const btn: React.CSSProperties = {
    padding: '4px 10px',
    fontSize: 10,
    fontWeight: 600,
    background: 'var(--vscode-button-background)',
    color: 'var(--vscode-button-foreground)',
    border: 'none',
    borderRadius: 3,
    cursor: 'pointer',
};

const btnSecondary: React.CSSProperties = {
    ...btn,
    background: 'transparent',
    border: '1px solid var(--vscode-panel-border)',
    color: 'inherit',
};

export default MacIOSSimulatorPanel;
