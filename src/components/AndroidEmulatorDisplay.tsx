/**
 * Embedded Android emulator display — uses backend ADB screencap stream (emulator:frame events).
 */
import React, { useCallback, useEffect, useRef, useState } from 'react';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '../tauri_bridge';

interface AndroidEmulatorDisplayProps {
    deviceId: string;
    onClose?: () => void;
}

export const AndroidEmulatorDisplay: React.FC<AndroidEmulatorDisplayProps> = ({
    deviceId,
    onClose,
}) => {
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const imgSize = useRef({ w: 1080, h: 1920 });
    const [connected, setConnected] = useState(false);
    const [fps, setFps] = useState(0);
    const [error, setError] = useState('');
    const frameTimes = useRef<number[]>([]);

    const mapCoords = useCallback((e: React.MouseEvent<HTMLCanvasElement>) => {
        const canvas = canvasRef.current;
        if (!canvas || canvas.width === 0) return null;
        const rect = canvas.getBoundingClientRect();
        const x = Math.round(((e.clientX - rect.left) / rect.width) * canvas.width);
        const y = Math.round(((e.clientY - rect.top) / rect.height) * canvas.height);
        if (x < 0 || y < 0 || x >= canvas.width || y >= canvas.height) return null;
        return { x, y };
    }, []);

    useEffect(() => {
        let unsubs: (() => void)[] = [];
        let cancelled = false;

        const boot = async () => {
            setError('');
            try {
                await invoke('start_emulator_stream', { deviceId });
            } catch (e) {
                if (!cancelled) setError(String(e));
            }
        };

        void boot();

        listen<{
            base64: string;
            width: number;
            height: number;
        }>('emulator:frame', (event) => {
            const { base64: b64, width: fw, height: fh } = event.payload;
            if (fw && fh) {
                imgSize.current = { w: fw, h: fh };
            }
            const canvas = canvasRef.current;
            if (!canvas) return;
            const ctx = canvas.getContext('2d');
            if (!ctx) return;
            const img = new Image();
            img.onload = () => {
                canvas.width = img.width;
                canvas.height = img.height;
                ctx.drawImage(img, 0, 0);
                setConnected(true);
                const now = Date.now();
                frameTimes.current.push(now);
                frameTimes.current = frameTimes.current.filter((t) => t > now - 1000);
                setFps(frameTimes.current.length);
            };
            img.src = `data:image/png;base64,${b64}`;
        }).then((u) => unsubs.push(u));

        return () => {
            cancelled = true;
            unsubs.forEach((u) => u());
            invoke('stop_emulator_stream').catch(() => {});
        };
    }, [deviceId]);

    const onPointerDown = async (e: React.MouseEvent<HTMLCanvasElement>) => {
        const c = mapCoords(e);
        if (c) await invoke('send_emulator_tap', { deviceId, x: c.x, y: c.y }).catch(() => {});
    };

    return (
        <div style={{ display: 'flex', flexDirection: 'column', height: '100%', minHeight: 0 }}>
            <div
                style={{
                    padding: '8px 12px',
                    borderBottom: '1px solid var(--vscode-panel-border)',
                    display: 'flex',
                    alignItems: 'center',
                    gap: 8,
                    flexShrink: 0,
                }}
            >
                <span
                    style={{
                        width: 8,
                        height: 8,
                        borderRadius: '50%',
                        background: connected ? '#4ec9b0' : '#888',
                        boxShadow: connected ? '0 0 8px #4ec9b0' : undefined,
                    }}
                />
                <span style={{ fontSize: 11, fontWeight: 600 }}>{deviceId}</span>
                <span style={{ fontSize: 10, opacity: 0.5, marginLeft: 'auto' }}>
                    {connected ? `${fps} fps` : 'Connecting…'}
                </span>
                {onClose && (
                    <button
                        type="button"
                        onClick={onClose}
                        style={{
                            padding: '2px 8px',
                            fontSize: 10,
                            cursor: 'pointer',
                            border: '1px solid var(--vscode-panel-border)',
                            background: 'transparent',
                            color: 'inherit',
                        }}
                    >
                        ✕
                    </button>
                )}
            </div>
            <div
                style={{
                    flex: 1,
                    minHeight: 0,
                    background: '#000',
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    overflow: 'hidden',
                }}
            >
                {!connected && !error && (
                    <div style={{ position: 'absolute', color: '#888', fontSize: 12, textAlign: 'center' }}>
                        Connecting to emulator…
                        <br />
                        <span style={{ fontSize: 11, opacity: 0.7 }}>Wait for boot if you just launched</span>
                    </div>
                )}
                {error && (
                    <div style={{ color: '#f87171', fontSize: 11, padding: 12, textAlign: 'center' }}>{error}</div>
                )}
                <canvas
                    ref={canvasRef}
                    onMouseDown={onPointerDown}
                    style={{ maxWidth: '100%', maxHeight: '100%', objectFit: 'contain', cursor: 'pointer' }}
                />
            </div>
        </div>
    );
};

export default AndroidEmulatorDisplay;
