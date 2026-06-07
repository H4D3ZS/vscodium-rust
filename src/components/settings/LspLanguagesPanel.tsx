import React, { useCallback, useEffect, useState } from 'react';
import { invoke } from '../../tauri_bridge';
import { useStore } from '../../store';
import { bootstrapLanguageServer } from '../../application/lsp/bootstrapLanguageServer';
import {
    lspStoreCatalog,
    lspStoreInstallNpm,
    lspStoreInstallPath,
    lspStoreInstallPreset,
    lspStoreScanPath,
    lspStoreSetEnabled,
    lspStoreUninstall,
    type LspPreset,
    type PathImportRow,
    type UserLspRecord,
} from '../../lib/lspStore';

interface LspServerRow {
    id: string;
    label: string;
    stacks: string[];
    installed: boolean;
    launchable: boolean;
    path?: string;
}

interface DetectCandidate {
    id: string;
    label: string;
    stacks: string[];
    score: number;
    reason: string;
    installed: boolean;
}

const rowStyle: React.CSSProperties = {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    gap: 8,
    padding: '6px 8px',
    border: '1px solid var(--vscode-widget-border, rgba(255,255,255,0.08))',
    borderRadius: 4,
};

const LspLanguagesPanel: React.FC = () => {
    const workspaceFolders = useStore((s) => s.workspaceFolders);
    const workspacePath = workspaceFolders[0]?.path ?? '';
    const lspRunning = useStore((s) => s.lspRunning);
    const lspServerId = useStore((s) => s.lspServerId);
    const lspError = useStore((s) => s.lspError);
    const [servers, setServers] = useState<LspServerRow[]>([]);
    const [userServers, setUserServers] = useState<UserLspRecord[]>([]);
    const [presets, setPresets] = useState<LspPreset[]>([]);
    const [pathImports, setPathImports] = useState<PathImportRow[]>([]);
    const [detect, setDetect] = useState<{ primary?: DetectCandidate; candidates: DetectCandidate[] } | null>(null);
    const [busy, setBusy] = useState(false);
    const [manualName, setManualName] = useState('');
    const [manualCommand, setManualCommand] = useState('');
    const [manualArgs, setManualArgs] = useState('--stdio');
    const [manualLanguages, setManualLanguages] = useState('');
    const [npmPackage, setNpmPackage] = useState('');
    const [npmName, setNpmName] = useState('');
    const [storeError, setStoreError] = useState<string | null>(null);

    const refresh = useCallback(async () => {
        try {
            const status = await invoke<{ servers: LspServerRow[]; userServers?: UserLspRecord[] }>('lsp_bundle_status');
            setServers(status.servers ?? []);
            setUserServers(status.userServers ?? []);
            if (workspacePath) {
                const d = await invoke<{ primary?: DetectCandidate; candidates: DetectCandidate[] }>(
                    'lsp_detect_workspace',
                    { root: workspacePath },
                );
                setDetect(d);
            }
            const catalog = await lspStoreCatalog();
            setPresets(catalog.presets ?? []);
            const scan = await lspStoreScanPath();
            setPathImports(scan.imports ?? []);
        } catch (e) {
            console.warn('[LspLanguagesPanel]', e);
        }
    }, [workspacePath]);

    useEffect(() => {
        void refresh();
    }, [refresh]);

    const onAutoStart = async () => {
        if (!workspacePath) return;
        setBusy(true);
        await bootstrapLanguageServer(workspacePath);
        await refresh();
        setBusy(false);
    };

    const onStartServer = async (serverId: string) => {
        if (!workspacePath) return;
        setBusy(true);
        setStoreError(null);
        try {
            await invoke('lsp_start_server', { root: workspacePath, serverId });
            useStore.getState().setLspStatus({ running: true, serverId, error: null });
            await refresh();
        } catch (e) {
            const msg = String(e);
            useStore.getState().setLspStatus({ error: msg });
            setStoreError(msg);
        } finally {
            setBusy(false);
        }
    };

    const onFetch = async () => {
        setBusy(true);
        try {
            if (workspacePath) {
                await invoke('lsp_ensure_bundle', { root: workspacePath });
            }
            await refresh();
        } catch (e) {
            useStore.getState().setLspStatus({ error: String(e) });
        } finally {
            setBusy(false);
        }
    };

    const onInstallPreset = async (presetId: string) => {
        setBusy(true);
        setStoreError(null);
        try {
            await lspStoreInstallPreset(presetId);
            await refresh();
        } catch (e) {
            setStoreError(String(e));
        } finally {
            setBusy(false);
        }
    };

    const onToggleUser = async (id: string, enabled: boolean) => {
        setBusy(true);
        try {
            await lspStoreSetEnabled(id, enabled);
            await refresh();
        } catch (e) {
            setStoreError(String(e));
        } finally {
            setBusy(false);
        }
    };

    const onUninstallUser = async (id: string) => {
        setBusy(true);
        try {
            await lspStoreUninstall(id);
            await refresh();
        } catch (e) {
            setStoreError(String(e));
        } finally {
            setBusy(false);
        }
    };

    const onManualInstall = async () => {
        if (!manualName.trim() || !manualCommand.trim()) return;
        setBusy(true);
        setStoreError(null);
        try {
            await lspStoreInstallPath({
                name: manualName.trim(),
                command: manualCommand.trim(),
                args: manualArgs.split(/\s+/).filter(Boolean),
                languages: manualLanguages.split(/[,;\s]+/).filter(Boolean),
                file_extensions: manualLanguages.split(/[,;\s]+/).filter(Boolean),
            });
            setManualName('');
            setManualCommand('');
            await refresh();
        } catch (e) {
            setStoreError(String(e));
        } finally {
            setBusy(false);
        }
    };

    const onNpmInstall = async () => {
        if (!npmPackage.trim()) return;
        setBusy(true);
        setStoreError(null);
        try {
            await lspStoreInstallNpm({
                package: npmPackage.trim(),
                name: npmName.trim() || undefined,
                languages: manualLanguages.split(/[,;\s]+/).filter(Boolean),
                file_extensions: manualLanguages.split(/[,;\s]+/).filter(Boolean),
            });
            setNpmPackage('');
            setNpmName('');
            await refresh();
        } catch (e) {
            setStoreError(String(e));
        } finally {
            setBusy(false);
        }
    };

    const installedPresetIds = new Set(userServers.map((s) => s.preset_id ?? s.id));

    return (
        <div style={{ maxWidth: 820, fontSize: 12, lineHeight: 1.55 }}>
            <div className="settings-card" style={{ marginBottom: 16 }}>
                <div className="settings-card-title">Language servers</div>
                <p style={{ opacity: 0.75, margin: '0 0 12px' }}>
                    Bundled IntelliSense for the main stacks (TS/JS, Rust, Python, Go, mobile, etc.).
                    Install additional servers below — toggle on/off or uninstall anytime.
                    Anything not in the catalog: install the binary globally and use{' '}
                    <strong>Register from PATH</strong> or point at the executable manually.
                </p>
                <div style={{ display: 'flex', gap: 8, flexWrap: 'wrap', marginBottom: 12 }}>
                    <button type="button" disabled={busy || !workspacePath} onClick={onAutoStart}>
                        Auto-start for workspace
                    </button>
                    <button type="button" disabled={busy} onClick={onFetch}>
                        Ensure bundle
                    </button>
                    <button type="button" disabled={busy} onClick={refresh}>
                        Refresh
                    </button>
                </div>
                <div style={{ opacity: 0.85 }}>
                    Status: {lspRunning ? `running (${lspServerId})` : 'stopped'}
                    {workspacePath && <span> · workspace: {workspacePath}</span>}
                </div>
                {(lspError || storeError) && (
                    <pre style={{ color: '#f85149', marginTop: 8, fontSize: 11, whiteSpace: 'pre-wrap' }}>
                        {lspError || storeError}
                    </pre>
                )}
                {!workspacePath && (
                    <p style={{ opacity: 0.65, marginTop: 8 }}>Open a folder to detect which language server fits.</p>
                )}
            </div>

            {detect && detect.candidates.length > 0 && (
                <div className="settings-card" style={{ marginBottom: 16 }}>
                    <div className="settings-card-title">Detected for this workspace</div>
                    {detect.primary && (
                        <p style={{ margin: '0 0 10px' }}>
                            Primary: <strong>{detect.primary.label}</strong> — {detect.primary.reason}
                        </p>
                    )}
                    <div style={{ display: 'flex', flexDirection: 'column', gap: 6 }}>
                        {detect.candidates.map((c) => (
                            <div key={c.id} style={rowStyle}>
                                <div>
                                    <strong>{c.label}</strong>
                                    <span style={{ opacity: 0.6 }}> — {c.reason}</span>
                                    <div style={{ opacity: 0.55, fontSize: 10 }}>{c.stacks.join(', ')}</div>
                                </div>
                                <button type="button" disabled={busy || !c.installed} onClick={() => onStartServer(c.id)}>
                                    {c.installed ? 'Start' : 'Not bundled'}
                                </button>
                            </div>
                        ))}
                    </div>
                </div>
            )}

            <div className="settings-card" style={{ marginBottom: 16 }}>
                <div className="settings-card-title">My language servers</div>
                <p style={{ opacity: 0.65, margin: '0 0 10px', fontSize: 11 }}>
                    User-installed servers stored under AppData. Disabled servers stay installed but won&apos;t start.
                </p>
                {userServers.length === 0 ? (
                    <p style={{ opacity: 0.55, margin: 0 }}>None installed yet — use the catalog or manual install below.</p>
                ) : (
                    <div style={{ display: 'flex', flexDirection: 'column', gap: 6 }}>
                        {userServers.map((s) => (
                            <div key={s.id} style={rowStyle}>
                                <div style={{ minWidth: 0 }}>
                                    <strong>{s.name}</strong>
                                    {!s.enabled && <span style={{ opacity: 0.6 }}> (disabled)</span>}
                                    <div style={{ opacity: 0.55, fontSize: 10 }}>
                                        {s.languages.join(', ')} · {s.source}
                                    </div>
                                    <div style={{ opacity: 0.45, fontSize: 10, wordBreak: 'break-all' }}>{s.command}</div>
                                </div>
                                <div style={{ display: 'flex', gap: 6, flexShrink: 0 }}>
                                    <label style={{ display: 'flex', alignItems: 'center', gap: 4 }}>
                                        <input
                                            type="checkbox"
                                            checked={s.enabled}
                                            disabled={busy}
                                            onChange={(e) => void onToggleUser(s.id, e.target.checked)}
                                        />
                                        On
                                    </label>
                                    <button type="button" disabled={busy || !s.enabled || !workspacePath} onClick={() => onStartServer(s.id)}>
                                        Start
                                    </button>
                                    <button type="button" disabled={busy} onClick={() => void onUninstallUser(s.id)}>
                                        Uninstall
                                    </button>
                                </div>
                            </div>
                        ))}
                    </div>
                )}
            </div>

            {pathImports.length > 0 && (
                <div className="settings-card" style={{ marginBottom: 16 }}>
                    <div className="settings-card-title">Register from PATH</div>
                    <p style={{ opacity: 0.65, margin: '0 0 10px', fontSize: 11 }}>
                        Found on your system PATH — one click to add to My language servers.
                    </p>
                    <div style={{ display: 'flex', flexDirection: 'column', gap: 6 }}>
                        {pathImports.map((row) => (
                            <div key={row.presetId} style={rowStyle}>
                                <div>
                                    <strong>{row.name}</strong>
                                    <div style={{ opacity: 0.55, fontSize: 10 }}>{row.command}</div>
                                </div>
                                <button
                                    type="button"
                                    disabled={busy || row.alreadyInstalled}
                                    onClick={() => void onInstallPreset(row.presetId)}
                                >
                                    {row.alreadyInstalled ? 'Added' : 'Import'}
                                </button>
                            </div>
                        ))}
                    </div>
                </div>
            )}

            <div className="settings-card" style={{ marginBottom: 16 }}>
                <div className="settings-card-title">Install from catalog</div>
                <p style={{ opacity: 0.65, margin: '0 0 10px', fontSize: 11 }}>
                    Popular VS Code–style language servers. npm presets use the bundled Node runtime.
                </p>
                <div style={{ display: 'flex', flexDirection: 'column', gap: 6 }}>
                    {presets.map((p) => (
                        <div key={p.id} style={rowStyle}>
                            <div>
                                <strong>{p.name}</strong>
                                <span style={{ opacity: 0.55 }}> ({p.install_kind})</span>
                                <div style={{ opacity: 0.55, fontSize: 10 }}>{p.languages.join(', ')} — {p.note}</div>
                            </div>
                            <button
                                type="button"
                                disabled={busy || installedPresetIds.has(p.id)}
                                onClick={() => void onInstallPreset(p.id)}
                            >
                                {installedPresetIds.has(p.id) ? 'Installed' : 'Install'}
                            </button>
                        </div>
                    ))}
                </div>
            </div>

            <div className="settings-card" style={{ marginBottom: 16 }}>
                <div className="settings-card-title">Install npm package</div>
                <p style={{ opacity: 0.65, margin: '0 0 10px', fontSize: 11 }}>
                    Any npm LSP package (e.g. <code>yaml-language-server</code>, <code>@styled/typescript-styled-plugin</code>).
                </p>
                <div style={{ display: 'grid', gap: 8, maxWidth: 520 }}>
                    <input
                        placeholder="npm package name"
                        value={npmPackage}
                        onChange={(e) => setNpmPackage(e.target.value)}
                        style={{ padding: '4px 8px' }}
                    />
                    <input
                        placeholder="Display name (optional)"
                        value={npmName}
                        onChange={(e) => setNpmName(e.target.value)}
                        style={{ padding: '4px 8px' }}
                    />
                    <input
                        placeholder="Languages / extensions (comma-separated)"
                        value={manualLanguages}
                        onChange={(e) => setManualLanguages(e.target.value)}
                        style={{ padding: '4px 8px' }}
                    />
                    <button type="button" disabled={busy || !npmPackage.trim()} onClick={() => void onNpmInstall()}>
                        Install npm LSP
                    </button>
                </div>
            </div>

            <div className="settings-card" style={{ marginBottom: 16 }}>
                <div className="settings-card-title">Manual install</div>
                <p style={{ opacity: 0.65, margin: '0 0 10px', fontSize: 11 }}>
                    Point at any LSP executable already on disk (global install, custom build, etc.).
                </p>
                <div style={{ display: 'grid', gap: 8, maxWidth: 520 }}>
                    <input
                        placeholder="Display name"
                        value={manualName}
                        onChange={(e) => setManualName(e.target.value)}
                        style={{ padding: '4px 8px' }}
                    />
                    <input
                        placeholder="Path to executable (e.g. C:\tools\my-lsp.exe)"
                        value={manualCommand}
                        onChange={(e) => setManualCommand(e.target.value)}
                        style={{ padding: '4px 8px' }}
                    />
                    <input
                        placeholder="Args (default: --stdio)"
                        value={manualArgs}
                        onChange={(e) => setManualArgs(e.target.value)}
                        style={{ padding: '4px 8px' }}
                    />
                    <input
                        placeholder="Languages (comma-separated, e.g. yaml,yml)"
                        value={manualLanguages}
                        onChange={(e) => setManualLanguages(e.target.value)}
                        style={{ padding: '4px 8px' }}
                    />
                    <button type="button" disabled={busy || !manualName.trim() || !manualCommand.trim()} onClick={() => void onManualInstall()}>
                        Register server
                    </button>
                </div>
            </div>

            <div className="settings-card">
                <div className="settings-card-title">Bundled servers</div>
                <p style={{ opacity: 0.65, margin: '0 0 10px', fontSize: 11 }}>
                    Dev/build: <code>powershell -File scripts/fetch-lsp-binaries.ps1</code>
                </p>
                <div style={{ display: 'flex', flexDirection: 'column', gap: 6 }}>
                    {servers.map((s) => (
                        <div key={s.id} style={rowStyle}>
                            <div>
                                <strong>{s.label}</strong>
                                <div style={{ opacity: 0.55, fontSize: 10 }}>{s.stacks?.join(', ')}</div>
                            </div>
                            <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
                                <span style={{ color: s.launchable ? '#3fb950' : '#d29922' }}>
                                    {s.launchable ? 'ready' : 'missing'}
                                </span>
                                <button type="button" disabled={busy || !s.launchable || !workspacePath} onClick={() => onStartServer(s.id)}>
                                    Start
                                </button>
                            </div>
                        </div>
                    ))}
                </div>
            </div>
        </div>
    );
};

export default LspLanguagesPanel;
