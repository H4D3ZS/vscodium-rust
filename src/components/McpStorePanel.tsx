import React, { useCallback, useEffect, useMemo, useState } from 'react';
import { useShallow } from 'zustand/react/shallow';
import { invoke } from '../tauri_bridge';
import { MCP_CATALOG, type McpCatalogEntry } from '../domain/mcp/mcpCatalog';
import { useStore } from '../store';

/**
 * MCP Store editor tab — UX parity with Google Antigravity IDE.
 * Antigravity: Agent "..." → MCP Servers → browse/install → Manage → View raw config.
 * @see docs/MCP_STORE.md
 */

type View = 'store' | 'manage' | 'raw';

const McpStorePanel: React.FC = () => {
    const { mcpServers, addMcpServer, removeMcpServer, listMcpServers, setMcpServerEnabled, setVisualLabData, setVisualLabMode, toggleVisualLab } = useStore(useShallow(s => ({
        mcpServers: s.mcpServers,
        addMcpServer: s.addMcpServer,
        removeMcpServer: s.removeMcpServer,
        listMcpServers: s.listMcpServers,
        setMcpServerEnabled: s.setMcpServerEnabled,
        setVisualLabData: s.setVisualLabData,
        setVisualLabMode: s.setVisualLabMode,
        toggleVisualLab: s.toggleVisualLab,
    })));

    const [view, setView] = useState<View>('store');
    const [search, setSearch] = useState('');
    const [installing, setInstalling] = useState<string | null>(null);
    const [configPath, setConfigPath] = useState('');
    const [installTarget, setInstallTarget] = useState<McpCatalogEntry | null>(null);
    const [envDraft, setEnvDraft] = useState<Record<string, string>>({});
    const [customName, setCustomName] = useState('');
    const [customCommand, setCustomCommand] = useState('npx');
    const [customArgs, setCustomArgs] = useState('');
    const [msg, setMsg] = useState('');
    const [toolCount, setToolCount] = useState<number | null>(null);
    const [refreshing, setRefreshing] = useState(false);
    const [rawText, setRawText] = useState('');
    const [rawLoading, setRawLoading] = useState(false);
    const [rawSaving, setRawSaving] = useState(false);
    const [rawError, setRawError] = useState('');

    useEffect(() => {
        listMcpServers().catch(() => {});
        invoke<string>('get_mcp_config_path').then(setConfigPath).catch(() => {});
        try {
            const v = sessionStorage.getItem('mcpStore.view');
            if (v === 'manage' || v === 'store' || v === 'raw') setView(v);
        } catch { /* */ }
    }, [listMcpServers]);

    const installedNames = useMemo(
        () => new Set(mcpServers.map(s => s.name.toLowerCase())),
        [mcpServers],
    );

    const filtered = useMemo(() => {
        const q = search.trim().toLowerCase();
        const base = !q
            ? [...MCP_CATALOG].sort((a, b) => a.name.localeCompare(b.name))
            : MCP_CATALOG.filter(e =>
                e.name.toLowerCase().includes(q) ||
                e.description.toLowerCase().includes(q) ||
                e.category.includes(q) ||
                (e.tags || []).some(t => t.includes(q)),
            );
        return base;
    }, [search]);

    const loadRawConfig = useCallback(async () => {
        setRawLoading(true);
        setRawError('');
        try {
            const res = await invoke<{ path?: string; text?: string }>('read_mcp_config');
            if (res?.path) setConfigPath(res.path);
            setRawText(res?.text ?? '{\n  "mcpServers": {}\n}');
        } catch (e) {
            setRawError(String(e));
            setRawText('{\n  "mcpServers": {}\n}');
        } finally {
            setRawLoading(false);
        }
    }, []);

    const refreshTools = useCallback(async () => {
        setRefreshing(true);
        setMsg('');
        try {
            await listMcpServers();
            const servers = await invoke<Array<{ config?: { enabled?: boolean } }>>('list_mcp_servers');
            const res = await invoke<{ count?: number }>('list_mcp_tools');
            const count = res?.count ?? 0;
            setToolCount(count);
            const enabled = servers.filter(s => s.config?.enabled !== false).length;
            setMsg(`${count} tool${count === 1 ? '' : 's'} across ${enabled} enabled server${enabled === 1 ? '' : 's'}. Toggle off/on after editing raw config (Antigravity parity).`);
        } catch {
            setMsg('Refresh failed — ensure enabled servers can start (npx/node on PATH, valid env).');
        } finally {
            setRefreshing(false);
        }
    }, [listMcpServers]);

    useEffect(() => {
        if (view === 'manage') void refreshTools();
        if (view === 'raw') void loadRawConfig();
    }, [view, refreshTools, loadRawConfig]);

    const isInstalled = (entry: McpCatalogEntry) => installedNames.has(entry.name.toLowerCase());

    const runInstall = useCallback(async (entry: McpCatalogEntry, env: Record<string, string>) => {
        setInstalling(entry.id);
        setMsg('');
        try {
            const cleanEnv = Object.fromEntries(
                Object.entries(env).filter(([, v]) => (v || '').trim().length > 0),
            );

            if (cleanEnv.GHIDRA_INSTALL_DIR) {
                const det = await invoke<{ ok?: boolean; path?: string; error?: string }>(
                    'detect_ghidra_install_dir',
                    { path: cleanEnv.GHIDRA_INSTALL_DIR },
                );
                if (!det?.ok) {
                    throw new Error(det?.error || 'Invalid Ghidra path');
                }
                if (det.path) cleanEnv.GHIDRA_INSTALL_DIR = det.path;
            }

            if (cleanEnv.IDA_INSTALL_DIR) {
                const det = await invoke<{ ok?: boolean; path?: string; error?: string }>(
                    'detect_ida_install_dir',
                    { path: cleanEnv.IDA_INSTALL_DIR },
                );
                if (!det?.ok) {
                    throw new Error(det?.error || 'Invalid IDA Pro path');
                }
                if (det.path) cleanEnv.IDA_INSTALL_DIR = det.path;
            }

            if (entry.type === 'http' && entry.serverUrl) {
                await addMcpServer(entry.name, { type: 'http', serverUrl: entry.serverUrl });
            } else {
                if (!entry.command) {
                    throw new Error('Catalog entry missing command');
                }
                const config: Record<string, unknown> = {
                    command: entry.command,
                    args: [...(entry.args ?? [])],
                    enabled: true,
                };
                if (Object.keys(cleanEnv).length) config.env = cleanEnv;
                await addMcpServer(entry.name, config);
            }
            await listMcpServers();
            const res = await invoke<{ count?: number }>('list_mcp_tools');
            const count = res?.count ?? 0;
            setMsg(`Installed ${entry.name} — ${count} MCP tool${count === 1 ? '' : 's'} available to the agent.`);
            setInstallTarget(null);
            setEnvDraft({});
        } catch (e) {
            setMsg(`Install failed: ${e}`);
        } finally {
            setInstalling(null);
        }
    }, [addMcpServer, listMcpServers]);

    const startInstall = (entry: McpCatalogEntry) => {
        if (isInstalled(entry)) return;
        if (entry.envFields?.length) {
            setInstallTarget(entry);
            const defaults: Record<string, string> = {};
            for (const f of entry.envFields) {
                if (f.key === 'GHIDRA_INSTALL_DIR') defaults[f.key] = 'E:\\Ghidra';
                else if (f.key === 'IDA_INSTALL_DIR') defaults[f.key] = 'E:\\IDA Professional 9.1';
                else defaults[f.key] = '';
            }
            setEnvDraft(defaults);
            return;
        }
        void runInstall(entry, {});
    };

    const openRawConfig = async () => {
        setView('raw');
        try { sessionStorage.setItem('mcpStore.view', 'raw'); } catch { /* */ }
        await loadRawConfig();
    };

    const saveRawConfig = async () => {
        setRawSaving(true);
        setRawError('');
        setMsg('');
        try {
            JSON.parse(rawText);
            await invoke('write_mcp_config', { text: rawText });
            await listMcpServers();
            await refreshTools();
            setMsg('Config saved and MCP servers reloaded.');
        } catch (e) {
            setRawError(String(e));
        } finally {
            setRawSaving(false);
        }
    };

    const formatRawConfig = () => {
        try {
            setRawText(JSON.stringify(JSON.parse(rawText), null, 2));
            setRawError('');
        } catch (e) {
            setRawError(`Invalid JSON: ${e}`);
        }
    };

    const visualizeRawConfig = () => {
        try {
            JSON.parse(rawText);
            setVisualLabData(rawText);
            setVisualLabMode('json');
            toggleVisualLab(true);
        } catch (e) {
            setRawError(`Fix JSON before visualizing: ${e}`);
        }
    };

    const addCustom = async () => {
        const name = customName.trim();
        if (!name || !customCommand.trim()) return;
        await addMcpServer(name, {
            command: customCommand.trim(),
            args: customArgs.split(',').map(a => a.trim()).filter(Boolean),
            enabled: true,
        });
        setCustomName('');
        setCustomArgs('');
        await listMcpServers();
        setMsg(`Added ${name}.`);
    };

    return (
        <div className="mcp-store-root">
            <header className="mcp-store-header">
                <div>
                    <h1 className="mcp-store-title">
                        {view === 'store' ? 'MCP Store' : view === 'manage' ? 'Manage MCP Servers' : 'Raw MCP Config'}
                    </h1>
                    <p className="mcp-store-sub">
                        {view === 'store'
                            ? 'Same model as Antigravity MCP Store: Install registers config; the IDE spawns an external MCP process.'
                            : view === 'manage'
                            ? 'Toggle servers, refresh tools, or edit raw JSON (Antigravity: Manage MCP Servers → View raw config).'
                            : 'Edit mcp_servers.json here (Antigravity opens mcp_config.json inline). Save reloads all MCP servers.'}
                    </p>
                </div>
                <div className="mcp-store-header-actions">
                    {view === 'raw' ? (
                        <button type="button" className="mcp-store-link" onClick={() => { setView('manage'); try { sessionStorage.setItem('mcpStore.view', 'manage'); } catch { /* */ } }}>
                            ← Back to Manage
                        </button>
                    ) : view === 'store' ? (
                        <button type="button" className="mcp-store-link" onClick={() => { setView('manage'); try { sessionStorage.setItem('mcpStore.view', 'manage'); } catch { /* */ } }}>
                            Manage MCP Servers
                        </button>
                    ) : (
                        <button type="button" className="mcp-store-link" onClick={() => { setView('store'); try { sessionStorage.setItem('mcpStore.view', 'store'); } catch { /* */ } }}>
                            ← Back to Store
                        </button>
                    )}
                    {view !== 'raw' && (
                        <button type="button" className="mcp-store-link" onClick={() => void openRawConfig()} title={configPath}>
                            View raw config
                        </button>
                    )}
                </div>
            </header>

            {msg && <div className="mcp-store-msg">{msg}</div>}

            {view === 'store' && (
                <>
                    <div className="mcp-store-search-wrap">
                        <i className="codicon codicon-search" />
                        <input
                            type="search"
                            className="mcp-store-search"
                            placeholder="Search MCP servers"
                            value={search}
                            onChange={e => setSearch(e.target.value)}
                        />
                    </div>

                    <div className="mcp-store-list">
                        {filtered.map(entry => {
                            const installed = isInstalled(entry);
                            const busy = installing === entry.id;
                            return (
                                <div key={entry.id} className="mcp-store-row">
                                    <div className="mcp-store-row-body">
                                        <div className="mcp-store-row-title">{entry.name}</div>
                                        <div className="mcp-store-row-desc">{entry.description}</div>
                                        {entry.needsConfig && (
                                            <div className="mcp-store-row-hint">{entry.needsConfig}</div>
                                        )}
                                    </div>
                                    <button
                                        type="button"
                                        className={`mcp-store-install${installed ? ' installed' : ''}`}
                                        disabled={installed || busy}
                                        onClick={() => startInstall(entry)}
                                        title={installed ? 'Installed' : 'Install'}
                                    >
                                        {installed ? (
                                            <i className="codicon codicon-check" />
                                        ) : busy ? (
                                            <i className="codicon codicon-loading codicon-modifier-spin" />
                                        ) : (
                                            <i className="codicon codicon-cloud-download" />
                                        )}
                                    </button>
                                </div>
                            );
                        })}
                        {filtered.length === 0 && (
                            <div className="mcp-store-empty">No servers match your search.</div>
                        )}
                    </div>
                    <footer className="mcp-store-footer">
                        Catalog follows Antigravity&apos;s curated MCP Store pattern. See{' '}
                        <a href="https://cloud.google.com/bigquery/docs/pre-built-tools-with-mcp-toolbox" target="_blank" rel="noreferrer">Google Cloud MCP + Antigravity</a>
                        {' '}and <code>docs/MCP_STORE.md</code> in this repo.
                    </footer>
                </>
            )}

            {view === 'manage' && (
                <div className="mcp-store-manage">
                    <div className="mcp-store-manage-toolbar">
                        <button type="button" className="mcp-store-refresh" onClick={() => void refreshTools()} disabled={refreshing}>
                            {refreshing ? 'Refreshing…' : 'Refresh'}
                        </button>
                        {toolCount !== null && (
                            <span className="mcp-store-tool-count">{toolCount} tools registered</span>
                        )}
                    </div>
                    {configPath && (
                        <div className="mcp-store-config-path">
                            <span className="label">Config</span>
                            <code>{configPath}</code>
                        </div>
                    )}

                    {mcpServers.length === 0 ? (
                        <div className="mcp-store-empty">No MCP servers installed yet. Browse the store to add one.</div>
                    ) : (
                        <div className="mcp-store-installed">
                            {mcpServers.map(server => {
                                const cfg = (server as { config?: Record<string, unknown> }).config || {};
                                const enabled = cfg.enabled !== false;
                                const isHttp = !!cfg.serverUrl || cfg.type === 'http';
                                const args = Array.isArray(cfg.args) ? (cfg.args as string[]).join(' ') : '';
                                const subtitle = isHttp
                                    ? String(cfg.serverUrl || '')
                                    : `${cfg.command || ''} ${args}`.trim();
                                return (
                                    <div key={server.name} className="mcp-store-installed-row">
                                        <div className="mcp-store-installed-info">
                                            <span className="name">{server.name}</span>
                                            <span className="badge">{isHttp ? 'http' : 'stdio'}</span>
                                            {!enabled && <span className="badge off">disabled</span>}
                                            <div className="sub">{subtitle}</div>
                                        </div>
                                        <button
                                            type="button"
                                            className={`mcp-toggle${enabled ? ' on' : ''}`}
                                            onClick={() => setMcpServerEnabled(server.name, !enabled)}
                                        >
                                            {enabled ? 'on' : 'off'}
                                        </button>
                                        <button
                                            type="button"
                                            className="mcp-remove"
                                            onClick={() => removeMcpServer(server.name)}
                                            title="Remove"
                                        >
                                            <i className="codicon codicon-trash" />
                                        </button>
                                    </div>
                                );
                            })}
                        </div>
                    )}

                    <div className="mcp-store-custom">
                        <h3>Add custom server</h3>
                        <div className="mcp-store-custom-grid">
                            <input placeholder="Name" value={customName} onChange={e => setCustomName(e.target.value)} />
                            <input placeholder="Command (e.g. npx)" value={customCommand} onChange={e => setCustomCommand(e.target.value)} />
                            <input placeholder="Args (comma separated)" value={customArgs} onChange={e => setCustomArgs(e.target.value)} className="wide" />
                            <button type="button" onClick={() => void addCustom()} disabled={!customName.trim()}>
                                Add server
                            </button>
                        </div>
                    </div>
                </div>
            )}

            {view === 'raw' && (
                <div className="mcp-store-raw">
                    {configPath && (
                        <div className="mcp-store-config-path">
                            <span className="label">File</span>
                            <code>{configPath}</code>
                        </div>
                    )}
                    {rawError && <div className="mcp-store-raw-error">{rawError}</div>}
                    {rawLoading ? (
                        <div className="mcp-store-empty">Loading config…</div>
                    ) : (
                        <textarea
                            className="mcp-store-raw-editor"
                            spellCheck={false}
                            value={rawText}
                            onChange={e => setRawText(e.target.value)}
                            placeholder={'{\n  "mcpServers": {}\n}'}
                        />
                    )}
                    <div className="mcp-store-raw-actions">
                        <button type="button" onClick={() => void loadRawConfig()} disabled={rawLoading}>
                            Reload
                        </button>
                        <button type="button" onClick={formatRawConfig} disabled={rawLoading}>
                            Format JSON
                        </button>
                        <button type="button" onClick={visualizeRawConfig} disabled={rawLoading} title="Open Visual Lab graph view">
                            Visualize JSON
                        </button>
                        <button type="button" className="primary" onClick={() => void saveRawConfig()} disabled={rawSaving || rawLoading}>
                            {rawSaving ? 'Saving…' : 'Save & reload servers'}
                        </button>
                    </div>
                    <p className="mcp-store-raw-hint">
                        Use the <code>mcpServers</code> root key (same as Antigravity <code>mcp_config.json</code>).
                        HTTP servers need <code>type</code> + <code>serverUrl</code>.
                    </p>
                </div>
            )}

            {installTarget && (
                <div className="mcp-store-modal-backdrop" onClick={() => setInstallTarget(null)}>
                    <div className="mcp-store-modal" onClick={e => e.stopPropagation()}>
                        <h3>Install {installTarget.name}</h3>
                        <p className="mcp-store-modal-desc">{installTarget.description}</p>
                        {(installTarget.envFields || []).map(field => (
                            <label key={field.key} className="mcp-store-field">
                                <span>{field.label}</span>
                                <input
                                    type={field.secret ? 'password' : 'text'}
                                    placeholder={field.placeholder || field.key}
                                    value={envDraft[field.key] || ''}
                                    onChange={e => setEnvDraft(d => ({ ...d, [field.key]: e.target.value }))}
                                />
                            </label>
                        ))}
                        <div className="mcp-store-modal-actions">
                            <button type="button" onClick={() => setInstallTarget(null)}>Cancel</button>
                            <button
                                type="button"
                                className="primary"
                                disabled={!!installing}
                                onClick={() => void runInstall(installTarget, envDraft)}
                            >
                                {installing ? 'Installing…' : 'Install'}
                            </button>
                        </div>
                    </div>
                </div>
            )}

            <style>{`
                .mcp-store-root {
                    height: 100%;
                    overflow: auto;
                    background: var(--vscode-editor-background);
                    color: var(--vscode-foreground);
                    padding: 24px 32px 48px;
                    box-sizing: border-box;
                }
                .mcp-store-header {
                    display: flex;
                    justify-content: space-between;
                    align-items: flex-start;
                    gap: 16px;
                    margin-bottom: 20px;
                    flex-wrap: wrap;
                }
                .mcp-store-title {
                    margin: 0 0 6px;
                    font-size: 22px;
                    font-weight: 600;
                }
                .mcp-store-sub {
                    margin: 0;
                    font-size: 12px;
                    opacity: 0.55;
                    max-width: 520px;
                    line-height: 1.5;
                }
                .mcp-store-header-actions {
                    display: flex;
                    gap: 12px;
                    align-items: center;
                    flex-shrink: 0;
                }
                .mcp-store-link {
                    background: none;
                    border: none;
                    color: var(--vscode-textLink-foreground, #3794ff);
                    font-size: 12px;
                    cursor: pointer;
                    padding: 0;
                    text-decoration: underline;
                }
                .mcp-store-msg {
                    margin-bottom: 12px;
                    padding: 8px 12px;
                    border-radius: 4px;
                    font-size: 12px;
                    background: rgba(56, 189, 248, 0.12);
                    border: 1px solid rgba(56, 189, 248, 0.25);
                }
                .mcp-store-search-wrap {
                    display: flex;
                    align-items: center;
                    gap: 8px;
                    padding: 8px 12px;
                    border: 1px solid var(--vscode-panel-border, rgba(255,255,255,0.12));
                    border-radius: 6px;
                    margin-bottom: 20px;
                    max-width: 480px;
                }
                .mcp-store-search {
                    flex: 1;
                    border: none;
                    background: transparent;
                    color: inherit;
                    font-size: 13px;
                    outline: none;
                }
                .mcp-store-section { margin-bottom: 24px; }
                .mcp-store-footer {
                    margin-top: 24px;
                    padding-top: 16px;
                    border-top: 1px solid var(--vscode-panel-border, rgba(255,255,255,0.06));
                    font-size: 11px;
                    opacity: 0.45;
                    line-height: 1.5;
                }
                .mcp-store-footer a { color: var(--vscode-textLink-foreground, #3794ff); }
                .mcp-store-manage-toolbar {
                    display: flex;
                    align-items: center;
                    gap: 12px;
                    margin-bottom: 12px;
                }
                .mcp-store-refresh {
                    padding: 4px 12px;
                    font-size: 12px;
                    cursor: pointer;
                    border-radius: 4px;
                    border: 1px solid var(--vscode-panel-border);
                    background: var(--vscode-button-secondaryBackground, rgba(255,255,255,0.05));
                    color: inherit;
                }
                .mcp-store-refresh:disabled { opacity: 0.5; cursor: default; }
                .mcp-store-tool-count { font-size: 11px; opacity: 0.55; }
                .mcp-store-raw { display: flex; flex-direction: column; gap: 10px; flex: 1; min-height: 0; }
                .mcp-store-raw-editor {
                    flex: 1;
                    min-height: 360px;
                    width: 100%;
                    box-sizing: border-box;
                    padding: 12px;
                    font-family: var(--font-mono, 'Consolas', monospace);
                    font-size: 12px;
                    line-height: 1.5;
                    background: var(--vscode-editor-background);
                    color: var(--vscode-editor-foreground);
                    border: 1px solid var(--vscode-panel-border, rgba(255,255,255,0.12));
                    border-radius: 6px;
                    resize: vertical;
                }
                .mcp-store-raw-actions {
                    display: flex;
                    gap: 8px;
                    flex-wrap: wrap;
                }
                .mcp-store-raw-actions button {
                    padding: 6px 14px;
                    font-size: 12px;
                    cursor: pointer;
                    border-radius: 4px;
                    border: 1px solid var(--vscode-panel-border);
                    background: transparent;
                    color: inherit;
                }
                .mcp-store-raw-actions button.primary {
                    background: var(--vscode-button-background, #0e639c);
                    color: var(--vscode-button-foreground, #fff);
                    border: none;
                }
                .mcp-store-raw-actions button:disabled { opacity: 0.5; cursor: default; }
                .mcp-store-raw-error {
                    padding: 8px 12px;
                    border-radius: 4px;
                    font-size: 12px;
                    background: rgba(247, 118, 142, 0.12);
                    border: 1px solid rgba(247, 118, 142, 0.35);
                    color: #f7768e;
                }
                .mcp-store-raw-hint {
                    font-size: 11px;
                    opacity: 0.45;
                    margin: 0;
                    line-height: 1.5;
                }
                .mcp-store-cat {
                    font-size: 11px;
                    text-transform: uppercase;
                    letter-spacing: 0.06em;
                    opacity: 0.45;
                    margin: 0 0 8px;
                    font-weight: 600;
                }
                .mcp-store-row {
                    display: flex;
                    align-items: center;
                    gap: 12px;
                    padding: 14px 4px;
                    border-bottom: 1px solid var(--vscode-panel-border, rgba(255,255,255,0.06));
                }
                .mcp-store-row-body { flex: 1; min-width: 0; }
                .mcp-store-row-title { font-size: 14px; font-weight: 600; margin-bottom: 4px; }
                .mcp-store-row-desc { font-size: 12px; opacity: 0.55; line-height: 1.45; }
                .mcp-store-row-hint { font-size: 10px; opacity: 0.4; margin-top: 4px; font-style: italic; }
                .mcp-store-install {
                    width: 36px;
                    height: 36px;
                    border-radius: 6px;
                    border: 1px solid var(--vscode-panel-border, rgba(255,255,255,0.15));
                    background: var(--vscode-button-secondaryBackground, rgba(255,255,255,0.05));
                    color: var(--vscode-foreground);
                    cursor: pointer;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    flex-shrink: 0;
                }
                .mcp-store-install:hover:not(:disabled) {
                    background: var(--vscode-list-hoverBackground);
                }
                .mcp-store-install.installed {
                    border-color: rgba(74, 222, 128, 0.4);
                    color: #4ade80;
                    cursor: default;
                }
                .mcp-store-install:disabled { opacity: 0.5; cursor: default; }
                .mcp-store-empty {
                    padding: 32px;
                    text-align: center;
                    opacity: 0.45;
                    font-size: 13px;
                }
                .mcp-store-config-path {
                    font-size: 11px;
                    margin-bottom: 16px;
                    opacity: 0.6;
                    word-break: break-all;
                }
                .mcp-store-config-path .label { margin-right: 8px; font-weight: 600; }
                .mcp-store-installed-row {
                    display: flex;
                    align-items: center;
                    gap: 10px;
                    padding: 10px 12px;
                    border: 1px solid var(--vscode-panel-border, rgba(255,255,255,0.08));
                    border-radius: 4px;
                    margin-bottom: 8px;
                }
                .mcp-store-installed-info { flex: 1; min-width: 0; }
                .mcp-store-installed-info .name { font-weight: 600; font-size: 13px; }
                .mcp-store-installed-info .badge {
                    margin-left: 6px;
                    font-size: 9px;
                    padding: 1px 5px;
                    border: 1px solid rgba(255,255,255,0.15);
                    border-radius: 2px;
                    opacity: 0.6;
                }
                .mcp-store-installed-info .badge.off { color: #f87171; border-color: #f87171; }
                .mcp-store-installed-info .sub {
                    font-size: 10px;
                    opacity: 0.45;
                    margin-top: 2px;
                    overflow: hidden;
                    text-overflow: ellipsis;
                    white-space: nowrap;
                }
                .mcp-toggle {
                    font-size: 10px;
                    font-weight: 700;
                    padding: 2px 10px;
                    border-radius: 10px;
                    cursor: pointer;
                    border: 1px solid rgba(255,255,255,0.2);
                    background: transparent;
                    color: inherit;
                }
                .mcp-toggle.on { background: #89d185; color: #000; border-color: #89d185; }
                .mcp-remove {
                    background: none;
                    border: none;
                    opacity: 0.45;
                    cursor: pointer;
                    color: inherit;
                    padding: 4px;
                }
                .mcp-store-custom {
                    margin-top: 28px;
                    padding-top: 20px;
                    border-top: 1px solid var(--vscode-panel-border, rgba(255,255,255,0.08));
                }
                .mcp-store-custom h3 { margin: 0 0 12px; font-size: 13px; }
                .mcp-store-custom-grid {
                    display: grid;
                    grid-template-columns: 1fr 1fr auto;
                    gap: 8px;
                    align-items: center;
                }
                .mcp-store-custom-grid .wide { grid-column: 1 / -2; }
                .mcp-store-custom-grid input {
                    padding: 6px 8px;
                    font-size: 12px;
                    background: var(--vscode-input-background);
                    color: var(--vscode-input-foreground);
                    border: 1px solid var(--vscode-input-border, rgba(255,255,255,0.12));
                    border-radius: 4px;
                }
                .mcp-store-custom-grid button {
                    padding: 6px 12px;
                    font-size: 12px;
                    cursor: pointer;
                    background: var(--vscode-button-background, #0e639c);
                    color: var(--vscode-button-foreground, #fff);
                    border: none;
                    border-radius: 4px;
                }
                .mcp-store-modal-backdrop {
                    position: fixed;
                    inset: 0;
                    background: rgba(0,0,0,0.55);
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    z-index: 1000;
                    padding: 16px;
                }
                .mcp-store-modal {
                    background: var(--vscode-editor-background);
                    border: 1px solid var(--vscode-panel-border);
                    border-radius: 8px;
                    padding: 20px;
                    max-width: 420px;
                    width: 100%;
                }
                .mcp-store-modal h3 { margin: 0 0 8px; font-size: 15px; }
                .mcp-store-modal-desc { font-size: 12px; opacity: 0.55; margin: 0 0 16px; }
                .mcp-store-field { display: block; margin-bottom: 12px; }
                .mcp-store-field span { display: block; font-size: 11px; margin-bottom: 4px; opacity: 0.7; }
                .mcp-store-field input {
                    width: 100%;
                    box-sizing: border-box;
                    padding: 6px 8px;
                    font-size: 12px;
                    background: var(--vscode-input-background);
                    color: var(--vscode-input-foreground);
                    border: 1px solid var(--vscode-input-border);
                    border-radius: 4px;
                }
                .mcp-store-modal-actions {
                    display: flex;
                    justify-content: flex-end;
                    gap: 8px;
                    margin-top: 8px;
                }
                .mcp-store-modal-actions button {
                    padding: 6px 14px;
                    font-size: 12px;
                    cursor: pointer;
                    border-radius: 4px;
                    border: 1px solid var(--vscode-panel-border);
                    background: transparent;
                    color: inherit;
                }
                .mcp-store-modal-actions button.primary {
                    background: var(--vscode-button-background, #0e639c);
                    color: var(--vscode-button-foreground, #fff);
                    border: none;
                }
            `}</style>
        </div>
    );
};

export default McpStorePanel;
