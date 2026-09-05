import React, { useCallback, useEffect, useRef, useState } from 'react';
import { invoke, listen } from '../tauri_bridge';

/**
 * Cyber-Ifrit deploy bridge UI — build → sign → install → tunnel → launch a
 * Flutter or React Native app onto the connected iPhone from Windows, no Mac.
 * The backend streams every step as `iphone:deploy-log` events, rendered live
 * in the console below.
 */

interface Preflight {
    go_ios: string | null;
    zsign: string | null;
    ideviceiproxy: string | null;
    idevice_id: string | null;
    flutter: string | null;
    npx: string | null;
    ready_flutter: boolean;
    ready_react_native: boolean;
    notes: string[];
}

interface DeployLog { phase: string; stream: string; line: string; }

const PHASE_COLOR: Record<string, string> = {
    meta: 'var(--vscode-descriptionForeground)',
    stderr: 'var(--vscode-errorForeground)',
    stdout: 'var(--vscode-foreground)',
};

const IPhoneDeployPanel: React.FC<{ udid: string }> = ({ udid }) => {
    const [framework, setFramework] = useState<'flutter' | 'react-native'>('flutter');
    const [projectDir, setProjectDir] = useState('');
    const [bundleId, setBundleId] = useState('');
    const [certP12, setCertP12] = useState('');
    const [certPassword, setCertPassword] = useState('');
    const [mobileprovision, setMobileprovision] = useState('');
    const [hotReloadPort, setHotReloadPort] = useState(8081);
    const [skipBuild, setSkipBuild] = useState(false);

    const [pre, setPre] = useState<Preflight | null>(null);
    const [deploying, setDeploying] = useState(false);
    const [logs, setLogs] = useState<DeployLog[]>([]);
    const [error, setError] = useState('');
    const [open, setOpen] = useState(false);
    const logEndRef = useRef<HTMLDivElement>(null);

    const refreshPreflight = useCallback(async () => {
        try { setPre(await invoke<Preflight>('iphone_deploy_preflight')); } catch { /* ignore */ }
    }, []);

    useEffect(() => { if (open && !pre) refreshPreflight(); }, [open, pre, refreshPreflight]);

    // Live build log.
    useEffect(() => {
        let un: (() => void) | undefined;
        listen<DeployLog>('iphone:deploy-log', (e) => {
            setLogs(prev => {
                const next = [...prev, e.payload];
                return next.length > 1000? next.slice(-1000): next; // bounded
            });
        }).then(u => { un = u; });
        return () => { un?.(); };
    }, []);

    useEffect(() => { logEndRef.current?.scrollIntoView({ block: 'end' }); }, [logs]);

    const deploy = useCallback(async () => {
        setError('');
        if (!udid) { setError('No device selected (pick one in the mirror controls above).'); return; }
        if (!projectDir) { setError('Set the project directory.'); return; }
        if (!bundleId) { setError('Set the app bundle id.'); return; }
        if (!certP12) { setError('A signing cert (.p12) is required.'); return; }
        setDeploying(true);
        setLogs([]);
        try {
            await invoke('iphone_deploy', {
                config: {
                    framework,
                    project_dir: projectDir,
                    udid,
                    bundle_id: bundleId,
                    cert_p12: certP12,
                    cert_password: certPassword || null,
                    mobileprovision: mobileprovision || null,
                    hot_reload_port: hotReloadPort,
                    skip_build: skipBuild,
                },
            });
        } catch (e) {
            setError(String(e));
        } finally {
            setDeploying(false);
        }
    }, [udid, framework, projectDir, bundleId, certP12, certPassword, mobileprovision, hotReloadPort, skipBuild]);

    const stopTunnel = useCallback(async () => {
        await invoke('iphone_stop_tunnel').catch(() => {});
    }, []);

    const ready = framework === 'flutter'? pre?.ready_flutter: pre?.ready_react_native;

    const input: React.CSSProperties = {
        fontSize: 11, padding: '3px 6px', borderRadius: 3, flex: 1, minWidth: 120,
        background: 'var(--vscode-input-background)', color: 'var(--vscode-input-foreground)',
        border: '1px solid var(--vscode-input-border, var(--vscode-panel-border))',
    };
    const btn: React.CSSProperties = {
        padding: '4px 10px', fontSize: 11, fontWeight: 600, cursor: 'pointer', borderRadius: 3,
        border: '1px solid transparent',
        background: 'var(--vscode-button-background)', color: 'var(--vscode-button-foreground)',
    };
    const ghost: React.CSSProperties = {
        ...btn, background: 'transparent', color: 'var(--vscode-descriptionForeground)',
        border: '1px solid var(--vscode-panel-border)',
    };
    const row: React.CSSProperties = { display: 'flex', gap: 6, alignItems: 'center', flexWrap: 'wrap' };

    return (
        <div style={{ borderTop: '1px solid var(--vscode-panel-border)', padding: 8, display: 'flex', flexDirection: 'column', gap: 6 }}>
            <button
                onClick={() => setOpen(o => !o)}
                style={{ ...ghost, alignSelf: 'flex-start' }}
            >
                {open? '▾': '▸'} Build &amp; Deploy (Cyber-Ifrit)
            </button>

            {open && (
                <div style={{ display: 'flex', flexDirection: 'column', gap: 6 }}>
                    {pre && pre.notes.length > 0 && (
                        <div style={{ fontSize: 10, padding: 6, borderRadius: 4, background: 'var(--vscode-inputValidation-warningBackground, #3a2d00)', border: '1px solid var(--vscode-inputValidation-warningBorder, #ad8b00)' }}>
                            {pre.notes.map((n, i) => <div key={i}>• {n}</div>)}
                        </div>
                    )}

                    <div style={row}>
                        <select value={framework} onChange={e => setFramework(e.target.value as any)} style={{ ...input, flex: '0 0 auto', minWidth: 120 }}>
                            <option value="flutter">Flutter</option>
                            <option value="react-native">React Native</option>
                        </select>
                        <button style={ghost} onClick={refreshPreflight}>Preflight</button>
                        {pre && <span style={{ fontSize: 10, color: ready? 'var(--vscode-testing-iconPassed, #3fb950)': 'var(--vscode-errorForeground)' }}>{ready? 'toolchain ready': 'toolchain incomplete'}</span>}
                    </div>

                    <input style={input} placeholder="Project directory (absolute path)" value={projectDir} onChange={e => setProjectDir(e.target.value)} />
                    <input style={input} placeholder="Bundle id (com.example.app)" value={bundleId} onChange={e => setBundleId(e.target.value)} />
                    <div style={row}>
                        <input style={input} placeholder="Signing cert .p12 path" value={certP12} onChange={e => setCertP12(e.target.value)} />
                        <input style={{ ...input, flex: '0 0 130px', minWidth: 90 }} type="password" placeholder="Cert password" value={certPassword} onChange={e => setCertPassword(e.target.value)} />
                    </div>
                    <input style={input} placeholder=".mobileprovision path (optional)" value={mobileprovision} onChange={e => setMobileprovision(e.target.value)} />
                    <div style={row}>
                        <label style={{ fontSize: 10, color: 'var(--vscode-descriptionForeground)' }}>
                            Hot-reload port&nbsp;
                            <input style={{ ...input, flex: '0 0 70px', minWidth: 60 }} type="number" value={hotReloadPort} onChange={e => setHotReloadPort(Number(e.target.value) || 8081)} />
                        </label>
                        <label style={{ fontSize: 10, color: 'var(--vscode-descriptionForeground)', display: 'flex', alignItems: 'center', gap: 4 }}>
                            <input type="checkbox" checked={skipBuild} onChange={e => setSkipBuild(e.target.checked)} />
                            Skip build (reuse last)
                        </label>
                    </div>

                    <div style={row}>
                        <button style={btn} onClick={deploy} disabled={deploying}>
                            {deploying? 'Deploying…': ' Build & Deploy'}
                        </button>
                        <button style={ghost} onClick={stopTunnel}>Stop Tunnel</button>
                    </div>

                    {error && <div style={{ fontSize: 11, color: 'var(--vscode-errorForeground)' }}>{error}</div>}

                    {logs.length > 0 && (
                        <div style={{ maxHeight: 220, overflow: 'auto', fontFamily: 'var(--vscode-editor-font-family, monospace)', fontSize: 10, background: 'var(--vscode-editorWidget-background)', border: '1px solid var(--vscode-panel-border)', borderRadius: 4, padding: 6 }}>
                            {logs.map((l, i) => (
                                <div key={i} style={{ whiteSpace: 'pre-wrap', color: PHASE_COLOR[l.stream] ?? 'var(--vscode-foreground)' }}>
                                    <span style={{ opacity: 0.5 }}>[{l.phase}]</span> {l.line}
                                </div>
                            ))}
                            <div ref={logEndRef} />
                        </div>
                    )}
                </div>
            )}
        </div>
    );
};

export default IPhoneDeployPanel;
