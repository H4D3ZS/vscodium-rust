import React, { useCallback, useEffect, useRef, useState } from 'react';
import { listen } from '../../tauri_bridge';
import { logcatStatus, startLogcat, stopLogcat } from '../../application/android/logcatSession';
import { refreshAndroidDevices, setActiveAndroidDevice } from '../../application/android/refreshAndroidDevices';
import type { LogcatEntry } from '../../domain/android/ILogcatRepository';

const LogcatPanel: React.FC = () => {
    const [lines, setLines] = useState<LogcatEntry[]>([]);
    const [filter, setFilter] = useState('');
    const [running, setRunning] = useState(false);
    const [devices, setDevices] = useState<{ id: string; state: string }[]>([]);
    const [device, setDevice] = useState('');
    const endRef = useRef<HTMLDivElement>(null);

    const refreshDevices = useCallback(async () => {
        try {
            const list = await refreshAndroidDevices();
            setDevices(list);
            if (!device && list[0]) setDevice(list[0].id);
        } catch { /* adb offline */ }
    }, [device]);

    useEffect(() => {
        void refreshDevices();
        const id = window.setInterval(refreshDevices, 15000);
        return () => window.clearInterval(id);
    }, [refreshDevices]);

    useEffect(() => {
        const unlisten = listen<LogcatEntry>('logcat-line', (e) => {
            setLines((prev) => [...prev.slice(-2000), e.payload]);
        });
        const unlistenStop = listen('logcat-stopped', () => setRunning(false));
        return () => {
            unlisten.then((f) => f());
            unlistenStop.then((f) => f());
        };
    }, []);

    useEffect(() => {
        endRef.current?.scrollIntoView({ behavior: 'smooth' });
    }, [lines]);

    const onStart = async () => {
        if (device) await setActiveAndroidDevice(device);
        await startLogcat(device || undefined, filter || undefined);
        setRunning(true);
        setLines([]);
    };

    const onStop = async () => {
        await stopLogcat();
        setRunning(false);
        const st = await logcatStatus();
        setRunning(st.running);
    };

    const filtered = filter.trim()
        ? lines.filter((l) => l.raw.toLowerCase().includes(filter.toLowerCase()) || l.tag?.includes(filter))
        : lines;

    return (
        <div style={{ display: 'flex', flexDirection: 'column', height: '100%', fontSize: 11, fontFamily: 'Consolas, monospace' }}>
            <div style={{ display: 'flex', gap: 8, padding: '6px 8px', borderBottom: '1px solid var(--vscode-panel-border)', flexWrap: 'wrap' }}>
                <select value={device} onChange={(e) => setDevice(e.target.value)} style={{ minWidth: 140 }}>
                    <option value="">Default device</option>
                    {devices.map((d) => (
                        <option key={d.id} value={d.id}>{d.id} ({d.state})</option>
                    ))}
                </select>
                <input
                    placeholder="Filter (tag/text)"
                    value={filter}
                    onChange={(e) => setFilter(e.target.value)}
                    style={{ flex: 1, minWidth: 120, padding: '2px 6px' }}
                />
                {!running ? (
                    <button type="button" onClick={() => void onStart()}>Start Logcat</button>
                ) : (
                    <button type="button" onClick={() => void onStop()}>Stop</button>
                )}
                <button type="button" onClick={() => setLines([])}>Clear</button>
            </div>
            <pre style={{ flex: 1, overflow: 'auto', margin: 0, padding: 8, whiteSpace: 'pre-wrap' }}>
                {filtered.map((l, i) => (
                    <div key={`${i}-${l.raw.slice(0, 24)}`} style={{ opacity: l.level === 'E' ? 1 : 0.85, color: l.level === 'E' ? '#f85149' : l.level === 'W' ? '#d29922' : undefined }}>
                        {l.raw}
                    </div>
                ))}
                <div ref={endRef} />
            </pre>
        </div>
    );
};

export default LogcatPanel;
