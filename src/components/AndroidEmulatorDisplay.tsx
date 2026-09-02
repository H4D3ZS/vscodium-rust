/**
 * Embedded Android emulator display — ADB screencap stream (`emulator:frame`) with
 * FULL interaction forwarded to the device via the scrcpy backend commands:
 *   tap, drag→swipe, wheel→scroll, keyboard (text + special keys), and the
 *   hardware Back / Home / Recents buttons. Behaves like Android Studio's
 *   embedded device rather than a passive video.
 */
import React, { useCallback, useEffect, useRef, useState } from 'react';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '../tauri_bridge';

interface AndroidEmulatorDisplayProps {
    deviceId: string;
    onClose?: () => void;
}

// Android KEYCODE_* values.
const KEY = { HOME: 3, BACK: 4, APP_SWITCH: 187, ENTER: 66, DEL: 67, TAB: 61, ESCAPE: 111 } as const;
const TAP_MAX_DIST = 12;   // px in device space below which a press is a tap
const TAP_MAX_MS = 350;    // press shorter than this (and small move) = tap

export const AndroidEmulatorDisplay: React.FC<AndroidEmulatorDisplayProps> = ({ deviceId, onClose }) => {
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const wrapRef = useRef<HTMLDivElement>(null);
    const [connected, setConnected] = useState(false);
    const [fps, setFps] = useState(0);
    const [error, setError] = useState('');
    const frameTimes = useRef<number[]>([]);
    const press = useRef<{ x: number; y: number; t: number } | null>(null);

    /** Map a mouse event to device pixel coords (canvas resolution). */
    const mapCoords = useCallback((e: { clientX: number; clientY: number }) => {
        const canvas = canvasRef.current;
        if (!canvas || canvas.width === 0) return null;
        const rect = canvas.getBoundingClientRect();
        // The canvas uses object-fit: contain, so compute the letterboxed draw rect.
        const scale = Math.min(rect.width / canvas.width, rect.height / canvas.height);
        const drawW = canvas.width * scale, drawH = canvas.height * scale;
        const offX = (rect.width - drawW) / 2, offY = (rect.height - drawH) / 2;
        const x = Math.round((e.clientX - rect.left - offX) / scale);
        const y = Math.round((e.clientY - rect.top - offY) / scale);
        if (x < 0 || y < 0 || x >= canvas.width || y >= canvas.height) return null;
        return { x, y };
    }, []);

    useEffect(() => {
        let unsubs: (() => void)[] = [];
        let cancelled = false;

        (async () => {
            setError('');
            try { await invoke('start_emulator_stream', { deviceId }); }
            catch (e) { if (!cancelled) setError(String(e)); }
        })();

        listen<{ base64: string; width: number; height: number }>('emulator:frame', async (event) => {
            const { base64: b64 } = event.payload;
            const canvas = canvasRef.current;
            if (!canvas || cancelled) return;
            const ctx = canvas.getContext('2d');
            if (!ctx) return;
            try {
                // createImageBitmap decodes off the main thread and frees promptly —
                // far cheaper than `new Image()` + data URL per frame (less GC churn).
                const blob = await (await fetch(`data:image/png;base64,${b64}`)).blob();
                const bitmap = await createImageBitmap(blob);
                if (cancelled) { bitmap.close(); return; }
                if (canvas.width !== bitmap.width) canvas.width = bitmap.width;
                if (canvas.height !== bitmap.height) canvas.height = bitmap.height;
                ctx.drawImage(bitmap, 0, 0);
                bitmap.close();
                setConnected(true);
                const now = Date.now();
                frameTimes.current.push(now);
                frameTimes.current = frameTimes.current.filter((t) => t > now - 1000);
                setFps(frameTimes.current.length);
            } catch { /* drop bad frame */ }
        }).then((u) => unsubs.push(u));

        return () => {
            cancelled = true;
            unsubs.forEach((u) => u());
            invoke('stop_emulator_stream').catch(() => {});
        };
    }, [deviceId]);

    // ── Pointer: press → (tap | swipe) ──────────────────────────────────────
    const onMouseDown = (e: React.MouseEvent<HTMLCanvasElement>) => {
        const c = mapCoords(e);
        if (c) press.current = { x: c.x, y: c.y, t: Date.now() };
        canvasRef.current?.focus();
    };
    const onMouseUp = async (e: React.MouseEvent<HTMLCanvasElement>) => {
        const start = press.current;
        press.current = null;
        const end = mapCoords(e);
        if (!start || !end) return;
        const dist = Math.hypot(end.x - start.x, end.y - start.y);
        const dt = Date.now() - start.t;
        if (dist <= TAP_MAX_DIST && dt <= TAP_MAX_MS) {
            await invoke('send_emulator_tap', { deviceId, x: end.x, y: end.y }).catch(() => {});
        } else {
            const duration = Math.max(50, Math.min(800, dt));
            await invoke('send_emulator_swipe', { deviceId, x1: start.x, y1: start.y, x2: end.x, y2: end.y, duration }).catch(() => {});
        }
    };

    // ── Wheel → scroll (vertical/horizontal swipe at the cursor) ────────────
    const onWheel = async (e: React.WheelEvent<HTMLCanvasElement>) => {
        const c = mapCoords(e);
        if (!c) return;
        const dy = Math.abs(e.deltaY) >= Math.abs(e.deltaX);
        const amt = dy ? Math.sign(e.deltaY) * 300 : Math.sign(e.deltaX) * 300;
        const x2 = dy ? c.x : c.x - amt;
        const y2 = dy ? c.y - amt : c.y;
        await invoke('send_emulator_swipe', { deviceId, x1: c.x, y1: c.y, x2, y2, duration: 120 }).catch(() => {});
    };

    // ── Keyboard → text + special keys ──────────────────────────────────────
    const onKeyDown = async (e: React.KeyboardEvent) => {
        if (e.metaKey || e.ctrlKey || e.altKey) return;
        const map: Record<string, number> = {
            Enter: KEY.ENTER, Backspace: KEY.DEL, Tab: KEY.TAB, Escape: KEY.ESCAPE,
        };
        if (map[e.key] != null) {
            e.preventDefault();
            await invoke('send_emulator_key', { deviceId, keycode: map[e.key] }).catch(() => {});
        } else if (e.key.length === 1) {
            e.preventDefault();
            await invoke('send_emulator_text', { deviceId, text: e.key }).catch(() => {});
        }
    };

    const hw = (keycode: number) => () => invoke('send_emulator_key', { deviceId, keycode }).catch(() => {});

    const btn: React.CSSProperties = {
        padding: '3px 10px', fontSize: 11, cursor: 'pointer', borderRadius: 6,
        border: '1px solid var(--vscode-panel-border)', background: 'transparent', color: 'inherit',
    };

    return (
        <div style={{ display: 'flex', flexDirection: 'column', height: '100%', minHeight: 0 }}>
            <div style={{ padding: '8px 12px', borderBottom: '1px solid var(--vscode-panel-border)', display: 'flex', alignItems: 'center', gap: 8, flexShrink: 0 }}>
                <span style={{ width: 8, height: 8, borderRadius: '50%', background: connected ? '#3fb950' : '#888', boxShadow: connected ? '0 0 8px #3fb950' : undefined }} />
                <span style={{ fontSize: 11, fontWeight: 600 }}>{deviceId}</span>
                <span style={{ fontSize: 10, opacity: 0.5 }}>{connected ? `${fps} fps` : 'Connecting…'}</span>
                <div style={{ flex: 1 }} />
                {onClose && <button type="button" onClick={onClose} style={btn}>Close</button>}
            </div>

            <div ref={wrapRef} style={{ flex: 1, minHeight: 0, background: '#000', display: 'flex', alignItems: 'center', justifyContent: 'center', overflow: 'hidden', position: 'relative' }}>
                {!connected && !error && (
                    <div style={{ position: 'absolute', color: '#888', fontSize: 12, textAlign: 'center' }}>
                        Connecting to emulator…<br /><span style={{ fontSize: 11, opacity: 0.7 }}>Wait for boot if you just launched</span>
                    </div>
                )}
                {error && <div style={{ color: '#f87171', fontSize: 11, padding: 12, textAlign: 'center' }}>{error}</div>}
                <canvas
                    ref={canvasRef}
                    tabIndex={0}
                    onMouseDown={onMouseDown}
                    onMouseUp={onMouseUp}
                    onWheel={onWheel}
                    onKeyDown={onKeyDown}
                    onContextMenu={(e) => { e.preventDefault(); hw(KEY.BACK)(); }}
                    style={{ maxWidth: '100%', maxHeight: '100%', objectFit: 'contain', cursor: 'pointer', outline: 'none' }}
                />
            </div>

            {/* Hardware buttons (Android Studio-style) */}
            <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', gap: 10, padding: '6px 12px', borderTop: '1px solid var(--vscode-panel-border)', flexShrink: 0 }}>
                <button type="button" title="Back (or right-click)" style={btn} onClick={hw(KEY.BACK)}><i className="codicon codicon-arrow-left" style={{ fontFamily: 'codicon', fontStyle: 'normal' }} /></button>
                <button type="button" title="Home" style={btn} onClick={hw(KEY.HOME)}><i className="codicon codicon-circle-large-outline" style={{ fontFamily: 'codicon', fontStyle: 'normal' }} /></button>
                <button type="button" title="Recents" style={btn} onClick={hw(KEY.APP_SWITCH)}><i className="codicon codicon-multiple-windows" style={{ fontFamily: 'codicon', fontStyle: 'normal' }} /></button>
            </div>
        </div>
    );
};

export default AndroidEmulatorDisplay;
