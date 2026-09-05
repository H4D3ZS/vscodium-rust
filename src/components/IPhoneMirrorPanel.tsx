import React, { useCallback, useEffect, useRef, useState } from 'react';
import { invoke, listen } from '../tauri_bridge';
import { useStore } from '../store';
import IPhoneDeployPanel from './IPhoneDeployPanel';

/**
 * Physical iPhone mirroring over USB (go-ios).
 *
 * go-ios v1.2 serves the screen as an MJPEG HTTP stream
 * (`ios screenshot --stream --port`), which the WebView renders directly in an
 * <img> — the CSP already allows `img-src http://127.0.0.1:*`, so there's no
 * decode step on our side. On iOS 17+ this needs an active go-ios tunnel
 * (`ios tunnel start`, admin) and a mounted Developer Image; the backend
 * enforces the tunnel and streams go-ios's own output to the diagnostics log
 * so failures are visible instead of a silent black screen.
 */

interface IPhoneDevice {
    udid: string;
    name: string;
    product: string;
    ios_version: string;
    connection: string; // "usb" | "network"
}

interface MirrorStatus {
    go_ios_found: boolean;
    go_ios_path: string | null;
    streaming: boolean;
    syslog: boolean;
    tunnel: boolean;
    udid: string | null;
    stream_url: string;
    log_ws: string;
}

const IPhoneMirrorPanel: React.FC = () => {
    const [status, setStatus] = useState<MirrorStatus | null>(null);
    const [devices, setDevices] = useState<IPhoneDevice[]>([]);
    const [selected, setSelected] = useState<string>('');
    const [busy, setBusy] = useState(false);
    const [error, setError] = useState<string>('');
    const [logs, setLogs] = useState<string[]>([]);
    const [streamSrc, setStreamSrc] = useState<string>('');
    const [control, setControl] = useState(false);
    const [devSize, setDevSize] = useState<{ w: number; h: number }>({ w: 0, h: 0 });
    const [typeBuf, setTypeBuf] = useState('');

    const logWs = useRef<WebSocket | null>(null);
    const logEndRef = useRef<HTMLDivElement>(null);
    const dragRef = useRef<{ x: number; y: number; t: number; rect: DOMRect } | null>(null);

    const pushLog = useCallback((line: string) => {
        setLogs(prev => {
            const next = [...prev, line];
            return next.length > 500? next.slice(-500): next; // bounded
        });
    }, []);

    const refreshStatus = useCallback(async () => {
        try { setStatus(await invoke<MirrorStatus>('iphone_mirror_status')); } catch { /* ignore */ }
    }, []);

    const refreshDevices = useCallback(async () => {
        setError('');
        try {
            const d = await invoke<IPhoneDevice[]>('iphone_list_devices');
            setDevices(d);
            if (d.length && !d.some(x => x.udid === selected)) setSelected(d[0].udid);
        } catch (e) { setError(String(e)); }
    }, [selected]);

    useEffect(() => {
        refreshStatus();
        refreshDevices();
        const tStatus = setInterval(refreshStatus, 3000);
        const tDevices = setInterval(refreshDevices, 5000);
        return () => { clearInterval(tStatus); clearInterval(tDevices); };
    }, [refreshStatus, refreshDevices]);

    // go-ios diagnostics (tunnel / screenshot / image-mount output).
    //
    // `listen` resolves asynchronously, so React StrictMode's double-invoke
    // registered a second listener before the first cleanup could run — every
    // line was logged twice. Guard with a cancelled flag and unsubscribe
    // immediately if the effect was torn down while the promise was in flight.
    useEffect(() => {
        let cancelled = false;
        let un: (() => void) | undefined;
        listen<{ stream: string; line: string }>('iphone:mirror-log', (e) => {
            if (cancelled) return;
            pushLog(`[${e.payload.stream}] ${e.payload.line}`);
        }).then(u => {
            if (cancelled) { u(); return; }
            un = u;
        });
        return () => { cancelled = true; un?.(); };
    }, [pushLog]);

    useEffect(() => { logEndRef.current?.scrollIntoView({ block: 'end' }); }, [logs]);

    const [copied, setCopied] = useState(false);
    /**Copy the WHOLE buffer, not the rendered tail — the useful error is often
     *  scrolled off the top. Falls back to a textarea+execCommand because the
     *  async clipboard API is unavailable on some WebView2 builds. */
    const copyLogs = useCallback(async () => {
        const text = logs.join('\n');
        try {
            await navigator.clipboard.writeText(text);
        } catch {
            const ta = document.createElement('textarea');
            ta.value = text;
            ta.style.position = 'fixed';
            ta.style.opacity = '0';
            document.body.appendChild(ta);
            ta.select();
            try { document.execCommand('copy'); } catch { /* clipboard blocked */ }
            document.body.removeChild(ta);
        }
        setCopied(true);
        setTimeout(() => setCopied(false), 1500);
    }, [logs]);

    /**Full pairing reset. Clears the stale host record that makes every lockdown
     *  call fail with InvalidHostID, and revokes on the device so it re-prompts
     *  for trust. Needs admin; the backend says so plainly if it isn't. */
    const repairPairing = useCallback(async () => {
        if (!selected) { setError('Select a device first.'); return; }
        setBusy(true); setError('');
        pushLog('[repair] resetting pairing — this takes ~10s and the phone will ask you to trust this PC…');
        try {
            await invoke('iphone_repair_pairing', { udid: selected });
            pushLog('[repair] done — press Start Tunnel.');
            await refreshStatus();
        } catch (e) { setError(String(e)); }
        finally { setBusy(false); }
    }, [selected, pushLog, refreshStatus]);

    /**Android-Studio-style ▶ Run: compile → package → sign → install → launch.
     *  Each stage is reported separately, because a failure at 'sign' and one at
     *  'install' need completely different fixes. */
    const runOnDevice = useCallback(async () => {
        if (!selected) { setError('Select a device first.'); return; }
        const projectDir = (useStore.getState() as any).activeRoot;
        if (!projectDir) { setError('Open a project folder first.'); return; }
        setBusy(true); setError('');
        pushLog('[run] building for the device…');
        try {
            const r = await invoke<any>('ios_run', {
                req: {
                    project_dir: projectDir,
                    app_name: String(projectDir).split(/[\/]/).filter(Boolean).pop() || 'App',
                    bundle_id: `com.vscodium.${(String(projectDir).split(/[\/]/).filter(Boolean).pop() || 'app').toLowerCase()}`,
                    udid: selected,
                },
            });
            for (const st of r.stages ?? []) pushLog(`[run] ${st.ok? '': ''} ${st.stage}: ${st.detail}`);
            if (!r.ok) setError(`Run failed at: ${(r.stages ?? []).filter((s: any) => !s.ok).map((s: any) => s.stage).join(', ')}`);
        } catch (e) { setError(String(e)); }
        finally { setBusy(false); }
    }, [selected, pushLog]);

    const startTunnel = useCallback(async () => {
        setBusy(true); setError('');
        // go-ios runs with --userspace, which needs no elevation.
        pushLog('[meta] starting tunnel (userspace — no admin needed)…');
        try {
            const msg = await invoke<string>('iphone_tunnel_start');
            pushLog(`[meta] ${msg}`);
            await refreshStatus();
        } catch (e) { setError(String(e)); }
        finally { setBusy(false); }
    }, [pushLog, refreshStatus]);

    const stopTunnel = useCallback(async () => {
        await invoke('iphone_tunnel_stop').catch(() => {});
        await refreshStatus();
    }, [refreshStatus]);

    const startMirror = useCallback(async () => {
        if (!selected) { setError('Select a device first.'); return; }
        setBusy(true); setError('');
        try {
            const res = await invoke<{ stream_url: string }>('iphone_start_mirror', { udid: selected });
            // Cache-bust so the <img> opens a fresh MJPEG connection.
            const base = res.stream_url.replace(/\/+$/, '');
            setStreamSrc(`${base}/?t=${Date.now()}`);
            await refreshStatus();
        } catch (e) {
            setError(String(e));
            setStreamSrc('');
        } finally {
            setBusy(false);
        }
    }, [selected, refreshStatus]);

    const stopMirror = useCallback(async () => {
        setBusy(true);
        try {
            setStreamSrc('');
            await invoke('iphone_stop_mirror').catch(() => {});
            await refreshStatus();
        } finally { setBusy(false); }
    }, [refreshStatus]);

    const toggleLogs = useCallback(async () => {
        if (logWs.current) {
            logWs.current.close(); logWs.current = null;
            await invoke('iphone_stop_syslog').catch(() => {});
            await refreshStatus();
            return;
        }
        if (!selected) { setError('Select a device first.'); return; }
        try {
            const url = await invoke<string>('iphone_start_syslog', { udid: selected });
            const ws = new WebSocket(url);
            ws.onmessage = (ev) => pushLog(String(ev.data));
            logWs.current = ws;
            await refreshStatus();
        } catch (e) { setError(String(e)); }
    }, [selected, refreshStatus, pushLog]);

    const prepare = useCallback(async () => {
        if (!selected) { setError('Select a device first.'); return; }
        setBusy(true); setError('');
        try {
            const res = await invoke<any>('iphone_prepare', { udid: selected });
            pushLog(`[prepare] pair=${res?.pair?.ok} mount=${res?.mount?.ok}${res?.repaired? ' (re-paired)': ''}`);
            // `pair=false` on its own is unactionable — surface go-ios's reason,
            // including the unpair/retry detail when the first attempt failed.
            const p = res?.pair ?? {};
            const reasons = [p.stderr, p.error, p.retry?.stderr, p.retry?.error, p.first_attempt?.stderr]
                .filter((x: unknown): x is string => typeof x === 'string' && x.trim().length > 0);
            for (const r of Array.from(new Set(reasons))) pushLog(`[pair] ${r}`);
            if (res?.hint) pushLog(`[prepare] ${res.hint}`);
            if (res?.needs_admin) {
                setError(
                    'Pairing needs Administrator on Windows: the pair record is written through '
                    + 'Apple Mobile Device Service, which refuses an unelevated process. Restart the IDE '
                    + 'as Administrator, or pair once with the Apple Devices / iTunes app (connect, unlock, '
                    + 'tap Trust) — go-ios reuses that record. The tunnel itself does not need admin.',
                );
            } else if (p.ok === false) {
                setError(
                    'Pairing failed. Unlock the iPhone, keep it plugged in, then tap "Trust This Computer" '
                    + 'on the device and press Pair + Mount again. The tunnel cannot start without a valid pair record.',
                );
            }
        } catch (e) { setError(String(e)); }
        finally { setBusy(false); }
    }, [selected, pushLog]);

    useEffect(() => () => { logWs.current?.close(); }, []);

    // ── Interactive control (WebDriverAgent) ──
    const startControl = useCallback(async () => {
        if (!selected) { setError('Select a device first.'); return; }
        setBusy(true); setError('');
        try {
            const r = await invoke<{ width: number; height: number }>('iphone_control_start', { udid: selected });
            setDevSize({ w: r.width, h: r.height });
            setControl(true);
        } catch (e) { setError(String(e)); }
        finally { setBusy(false); }
    }, [selected]);

    const stopControl = useCallback(async () => {
        await invoke('iphone_control_stop').catch(() => {});
        setControl(false);
    }, []);

    // One-time signing setup. After this, Control auto-installs WDA with no prompts.
    const configureSigning = useCallback(async () => {
        const method = (window.prompt('Signing method — type "free" (Sideloadly/AltServer, 7-day cert), "manual" (.p12 + profile), or "asc" (App Store Connect API key):', 'free') || '').trim();
        let config: any;
        if (method === 'free') {
            const signer_path = window.prompt('Path to Sideloadly.exe or AltServer.exe (blank = auto-detect):') || '';
            config = { method: 'free', signer_path: signer_path || undefined };
        } else if (method === 'asc') {
            const asc_key_id = window.prompt('ASC Key ID:'); if (!asc_key_id) return;
            const asc_issuer_id = window.prompt('ASC Issuer ID:'); if (!asc_issuer_id) return;
            const asc_p8 = window.prompt('Path to ASC private key (.p8):'); if (!asc_p8) return;
            const bundle_id = window.prompt('WDA bundle id (blank = default):') || '';
            const p12_password = window.prompt('Password for the generated .p12 (blank = none):') || '';
            config = { method: 'asc', asc_key_id, asc_issuer_id, asc_p8, bundle_id, p12_password };
        } else if (method === 'manual') {
            const p12 = window.prompt('Path to your .p12 certificate:'); if (!p12) return;
            const p12_password = window.prompt('.p12 password (blank if none):') || '';
            const profile = window.prompt('Path to your .mobileprovision profile:'); if (!profile) return;
            config = { method: 'manual', p12, p12_password, profile };
        } else { return; }
        setBusy(true); setError('');
        try {
            pushLog(`[meta] ${await invoke<string>('iphone_set_signing', { config })}`);
            pushLog('[meta] signing saved — click Control and WDA installs automatically.');
        } catch (e) { setError(String(e)); }
        finally { setBusy(false); }
    }, [pushLog]);

    const home = useCallback(() => { invoke('iphone_wda_home').catch(e => setError(String(e))); }, []);

    const sendType = useCallback(() => {
        if (!typeBuf) return;
        invoke('iphone_wda_type', { text: typeBuf }).then(() => setTypeBuf('')).catch(e => setError(String(e)));
    }, [typeBuf]);

    const onScreenDown = useCallback((e: React.MouseEvent<HTMLImageElement>) => {
        if (!control) return;
        dragRef.current = { x: e.clientX, y: e.clientY, t: performance.now(), rect: e.currentTarget.getBoundingClientRect() };
    }, [control]);

    const onScreenUp = useCallback((e: React.MouseEvent<HTMLImageElement>) => {
        if (!control || !dragRef.current) return;
        const s = dragRef.current; dragRef.current = null;
        const map = (cx: number, cy: number) => ({
            x: Math.max(0, Math.min(1, (cx - s.rect.left) / s.rect.width)) * devSize.w,
            y: Math.max(0, Math.min(1, (cy - s.rect.top) / s.rect.height)) * devSize.h,
        });
        const from = map(s.x, s.y);
        const dist = Math.hypot(e.clientX - s.x, e.clientY - s.y);
        if (dist < 8) {
            invoke('iphone_wda_tap', { x: from.x, y: from.y }).catch(err => setError(String(err)));
        } else {
            const to = map(e.clientX, e.clientY);
            const duration = Math.max(0.05, (performance.now() - s.t) / 1000);
            invoke('iphone_wda_swipe', { fromX: from.x, fromY: from.y, toX: to.x, toY: to.y, duration }).catch(err => setError(String(err)));
        }
    }, [control, devSize]);

    const selDev = devices.find(d => d.udid === selected);
    const isNetwork = selDev?.connection === 'network';

    const btn: React.CSSProperties = {
        padding: '4px 10px', fontSize: '11px', fontWeight: 600, cursor: 'pointer',
        border: '1px solid var(--vscode-button-border, transparent)', borderRadius: 3,
        background: 'var(--vscode-button-background)', color: 'var(--vscode-button-foreground)',
    };
    const ghost: React.CSSProperties = {
        ...btn, background: 'transparent', color: 'var(--vscode-descriptionForeground)',
        border: '1px solid var(--vscode-panel-border)',
    };

    return (
        <div style={{ display: 'flex', flexDirection: 'column', height: '100%', minHeight: 0, padding: 8, gap: 8, color: 'var(--vscode-foreground)', overflowY: 'auto' }}>
            {/* Controls */}
            <div style={{ display: 'flex', flexWrap: 'wrap', gap: 6, alignItems: 'center' }}>
                <select
                    value={selected}
                    onChange={e => setSelected(e.target.value)}
                    style={{ fontSize: 11, padding: '3px 6px', background: 'var(--vscode-dropdown-background)', color: 'var(--vscode-dropdown-foreground)', border: '1px solid var(--vscode-dropdown-border)', borderRadius: 3, maxWidth: 220 }}
                >
                    {devices.length === 0 && <option value="">No device</option>}
                    {devices.map(d => (
                        <option key={d.udid} value={d.udid}>
                            {d.connection === 'network'? '': ''} {(d.name || 'iPhone')}{d.ios_version? ` (iOS ${d.ios_version})`: ''} — {d.udid.slice(0, 8)}…
                        </option>
                    ))}
                </select>
                <button style={ghost} onClick={refreshDevices} disabled={busy}>Refresh</button>
                <button
                    style={btn}
                    onClick={runOnDevice}
                    disabled={busy || !selected}
                    title="Build the open project for iOS, sign it, install and launch on the device"
                >
                    ▶ Run
                </button>
                <button style={ghost} onClick={prepare} disabled={busy || !selected}>Pair + Mount</button>
                <button
                    style={ghost}
                    onClick={repairPairing}
                    disabled={busy || !selected}
                    title="Revoke trust, clear the stale pair record, restart the USB service and pair again. Fixes InvalidHostID / 'Saving the PairRecord to usbmux failed'. Needs Administrator."
                >
                    Repair Pairing
                </button>
                {status?.tunnel
? <button style={ghost} onClick={stopTunnel} title="Tunnel is up"> Tunnel</button>
: <button style={ghost} onClick={startTunnel} disabled={busy} title="iOS 17+ needs this (userspace — no admin)">Start Tunnel</button>}
                {status?.streaming
                    // Accent styling means "running", matching the tunnel's .
? <button style={btn} onClick={stopMirror} disabled={busy}>■ Stop</button>
                    // Idle uses the same neutral style as every other button —
                    // the accent style made Mirror look already-active before it
                    // had been clicked.
: <button style={ghost} onClick={startMirror} disabled={busy || !selected || isNetwork} title={isNetwork? 'Screen mirror needs a USB connection': 'Start the screen mirror'}>▶ Mirror</button>}
                <button style={ghost} onClick={toggleLogs} disabled={!selected}>
                    {status?.syslog? 'Stop Logs': 'Live Logs'}
                </button>
                {control
? <button style={btn} onClick={stopControl}> Control On</button>
: <button style={ghost} onClick={startControl} disabled={busy || !selected} title="Interactive tap/swipe via WebDriverAgent"> Control</button>}
                {control && <button style={ghost} onClick={home} title="Home button"></button>}
                <button style={ghost} onClick={configureSigning} disabled={busy} title="One-time signing setup — free (Sideloadly), manual (.p12), or ASC (App Store Connect)"> Signing</button>
            </div>

            {error && <div style={{ fontSize: 11, color: 'var(--vscode-errorForeground)' }}>{error}</div>}
            {isNetwork && (
                <div style={{ fontSize: 10, color: 'var(--vscode-descriptionForeground)' }}>
                     Wi-Fi device: logs, install &amp; hot-reload deploy work, but the live screen mirror needs a USB cable (it's a USB-only AV stream).
                </div>
            )}

            {/* Screen — MJPEG stream rendered directly in an <img> */}
            <div style={{ position: 'relative', flex: '1 1 60%', minHeight: 160, display: 'flex', alignItems: 'center', justifyContent: 'center', background: '#000', borderRadius: 14, overflow: 'hidden', border: '1px solid var(--vscode-panel-border)' }}>
                {streamSrc? (
                    <img
                        src={streamSrc}
                        alt="iPhone screen"
                        draggable={false}
                        onMouseDown={onScreenDown}
                        onMouseUp={onScreenUp}
                        onContextMenu={e => e.preventDefault()}
                        style={{ maxWidth: '100%', maxHeight: '100%', objectFit: 'contain', cursor: control? 'crosshair': 'default', userSelect: 'none' }}
                        onError={() => setError('Stream failed to load — check the diagnostics log below.')}
                    />
                ): (
                    <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', gap: 8, color: '#8a8a8a' }}>
                        <div style={{ fontSize: 46, lineHeight: 1 }}></div>
                        <div style={{ fontSize: 12, fontWeight: 600 }}>
                            {devices.length? (busy? 'Connecting…': 'Press ▶ Mirror to view the device'): 'Connect an iPhone via USB'}
                        </div>
                        <div style={{ fontSize: 10, opacity: 0.7 }}>
                            {devices.length
? 'iOS 17+: click Start Tunnel first, and unlock the phone'
: 'Enable Developer Mode (iOS 16+) and trust this computer'}
                        </div>
                    </div>
                )}
            </div>

            {/* Text injection (WDA types into the focused field on the device) */}
            {control && (
                <div style={{ display: 'flex', gap: 6 }}>
                    <input
                        value={typeBuf}
                        onChange={e => setTypeBuf(e.target.value)}
                        onKeyDown={e => { if (e.key === 'Enter') sendType(); }}
                        placeholder="Type into the focused field on the phone…"
                        style={{ flex: 1, fontSize: 11, padding: '3px 6px', borderRadius: 3, background: 'var(--vscode-input-background)', color: 'var(--vscode-input-foreground)', border: '1px solid var(--vscode-input-border, var(--vscode-panel-border))' }}
                    />
                    <button style={ghost} onClick={sendType}>Send</button>
                </div>
            )}

            {/* Diagnostics + syslog */}
            {logs.length > 0 && (
                <div style={{ flex: '0 0 28%', minHeight: 60, display: 'flex', flexDirection: 'column', background: 'var(--vscode-editorWidget-background)', border: '1px solid var(--vscode-panel-border)', borderRadius: 4 }}>
                    <div style={{ display: 'flex', alignItems: 'center', gap: 6, padding: '3px 6px', borderBottom: '1px solid var(--vscode-panel-border)' }}>
                        <span style={{ fontSize: 10, opacity: 0.6 }}>Diagnostics ({logs.length})</span>
                        <button
                            style={{ ...ghost, marginLeft: 'auto', fontSize: 10, padding: '1px 8px' }}
                            title="Copy the full log to the clipboard"
                            onClick={copyLogs}
                        >
                            {copied? 'Copied ': 'Copy'}
                        </button>
                        <button style={{ ...ghost, fontSize: 10, padding: '1px 8px' }} onClick={() => setLogs([])}>Clear</button>
                    </div>
                    <div style={{ flex: 1, overflow: 'auto', fontFamily: 'var(--vscode-editor-font-family, monospace)', fontSize: 10, padding: 6 }}>
                        {logs.slice(-200).map((l, i) => <div key={i} style={{ whiteSpace: 'pre-wrap', opacity: 0.85 }}>{l}</div>)}
                        <div ref={logEndRef} />
                    </div>
                </div>
            )}

            {/* Build & Deploy bridge (uses the device selected above) */}
            <IPhoneDeployPanel udid={selected} />
        </div>
    );
};

export default IPhoneMirrorPanel;
