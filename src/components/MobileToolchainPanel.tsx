import React, { useCallback, useEffect, useState } from 'react';
import { invoke } from '../tauri_bridge';

type DoctorResult = {
    ok?: boolean;
    stdout?: string;
    stderr?: string;
    exit_code?: number;
    vphone_root?: string;
};

type PathsResult = {
    vphone_root?: string;
    toolchain_dir?: string;
    doctor_script?: string;
    flutter_on_path?: boolean;
    altstore_hint?: string;
};

const MobileToolchainPanel: React.FC = () => {
    const [paths, setPaths] = useState<PathsResult | null>(null);
    const [output, setOutput] = useState('');
    const [busy, setBusy] = useState(false);

    const refresh = useCallback(async () => {
        try {
            const p = await invoke<PathsResult>('resolve_mobile_toolchain_paths');
            setPaths(p);
        } catch (e) {
            setPaths(null);
            setOutput(String(e));
        }
    }, []);

    useEffect(() => {
        void refresh();
    }, [refresh]);

    const runDoctor = async () => {
        setBusy(true);
        setOutput('Running vphone-doctor…\n');
        try {
            const res = await invoke<DoctorResult>('run_vphone_doctor');
            setOutput(
                [res.stdout, res.stderr].filter(Boolean).join('\n') ||
                    (res.ok ? 'Doctor passed.' : `Doctor failed (exit ${res.exit_code})`),
            );
        } catch (e) {
            setOutput(String(e));
        } finally {
            setBusy(false);
        }
    };

    const runInstall = async () => {
        setBusy(true);
        setOutput('Installing phony Xcode shims…\n');
        try {
            const res = await invoke<DoctorResult>('install_vphone_toolchain');
            setOutput(
                [res.stdout, res.stderr].filter(Boolean).join('\n') ||
                    (res.ok ? 'Install complete. Restart terminal/IDE.' : `Install failed (exit ${res.exit_code})`),
            );
            await refresh();
        } catch (e) {
            setOutput(String(e));
        } finally {
            setBusy(false);
        }
    };

    return (
        <div style={{ padding: 12, overflow: 'auto', height: '100%', fontSize: 12 }}>
            <h3 style={{ fontSize: 11, textTransform: 'uppercase', margin: '0 0 12px', opacity: 0.8 }}>
                Mobile Toolchain
            </h3>

            <div style={{ marginBottom: 12, padding: 10, background: 'var(--vscode-textBlockQuote-background)', borderRadius: 4 }}>
                <div style={{ fontWeight: 600, marginBottom: 6 }}>vPhone (Flutter / React Native / phony Xcode)</div>
                <div style={{ fontSize: 11, opacity: 0.75, lineHeight: 1.5 }}>
                    Cross-platform shims so <code>flutter doctor</code> and RN tooling see Xcode + vPhone Bridge on Windows/Linux.
                    Requires <code>Virtual-iPhone-Emulator/toolchain</code> or <code>VPHONE_ROOT</code>.
                </div>
                {paths?.vphone_root ? (
                    <div style={{ marginTop: 8, fontSize: 10, fontFamily: 'monospace' }}>{paths.vphone_root}</div>
                ) : (
                    <div style={{ marginTop: 8, color: '#f87171', fontSize: 11 }}>
                        Virtual-iPhone-Emulator not found — set env <code>VPHONE_ROOT</code>
                    </div>
                )}
            </div>

            <div style={{ display: 'flex', gap: 6, flexWrap: 'wrap', marginBottom: 12 }}>
                <button type="button" disabled={busy} onClick={() => void runInstall()} style={btnStyle}>
                    Install shims
                </button>
                <button type="button" disabled={busy} onClick={() => void runDoctor()} style={btnStyle}>
                    Run doctor
                </button>
                <button type="button" disabled={busy} onClick={() => void refresh()} style={btnStyleSecondary}>
                    Refresh
                </button>
            </div>

            <div style={{ marginBottom: 12, padding: 10, background: 'var(--vscode-textBlockQuote-background)', borderRadius: 4 }}>
                <div style={{ fontWeight: 600, marginBottom: 6 }}>AltStore (deploy)</div>
                <div style={{ fontSize: 11, opacity: 0.75, lineHeight: 1.5 }}>
                    {paths?.altstore_hint ||
                        'Optional: install AltServer for sideloading .ipa to physical iPhones (AGPL — not bundled).'}
                    {' '}
                    <a href="https://github.com/altstoreio/AltStore" target="_blank" rel="noreferrer">
                        altstoreio/AltStore
                    </a>
                </div>
            </div>

            <div style={{ marginBottom: 8, fontSize: 10, opacity: 0.6 }}>
                Flutter on PATH: {paths?.flutter_on_path ? 'yes' : 'no'}
            </div>

            {output && (
                <pre
                    style={{
                        margin: 0,
                        padding: 10,
                        fontSize: 10,
                        lineHeight: 1.4,
                        whiteSpace: 'pre-wrap',
                        background: 'var(--vscode-editor-background)',
                        border: '1px solid var(--vscode-panel-border)',
                        borderRadius: 4,
                        maxHeight: 280,
                        overflow: 'auto',
                    }}
                >
                    {output}
                </pre>
            )}
        </div>
    );
};

const btnStyle: React.CSSProperties = {
    padding: '6px 12px',
    fontSize: 11,
    fontWeight: 600,
    background: 'var(--vscode-button-background)',
    color: 'var(--vscode-button-foreground)',
    border: 'none',
    borderRadius: 3,
    cursor: 'pointer',
};

const btnStyleSecondary: React.CSSProperties = {
    ...btnStyle,
    background: 'var(--vscode-button-secondaryBackground)',
    color: 'var(--vscode-button-secondaryForeground)',
};

export default MobileToolchainPanel;
