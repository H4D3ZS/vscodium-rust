import React, { useCallback, useEffect, useMemo, useState } from 'react';
import { invoke } from '../../tauri_bridge';
import { useStore } from '../../store';
import {
    installModule,
    launchInstalledModule,
    setModuleEnabled,
    uninstallModule,
} from '../../application/modules/moduleLauncher';
import type { CatalogModuleRow, ModuleCategory } from '../../store/moduleSlice';

const CATEGORY_ORDER = [
    'dev-tools',
    'web-pentest',
    'pentest',
    'red-team',
    'malware-re',
    'secret-intel',
    'mobile-security',
    'osint-recon',
    'tech-industry',
    'documentation',
    'performance',
    'ai-ml',
    'compliance',
];

const ModuleInstallerPanel: React.FC = () => {
    const catalog = useStore((s) => s.moduleCatalog);
    const loading = useStore((s) => s.modulesLoading);
    const error = useStore((s) => s.modulesError);
    const refresh = useStore((s) => s.refreshModuleCatalog);

    const [filter, setFilter] = useState('');
    const [category, setCategory] = useState<string>('all');
    const [busyId, setBusyId] = useState<string | null>(null);
    const [catalogUrl, setCatalogUrl] = useState('');
    const [message, setMessage] = useState('');

    useEffect(() => {
        invoke<string>('modules_default_catalog_url')
            .then(setCatalogUrl)
            .catch(() => {});
        void refresh();
    }, [refresh]);

    const categories: ModuleCategory[] = useMemo(() => {
        const fromCat = catalog?.categories ?? [];
        return CATEGORY_ORDER.map((id) => fromCat.find((c) => c.id === id) ?? { id, label: id });
    }, [catalog]);

    const modules = useMemo(() => {
        const list = catalog?.modules ?? [];
        const q = filter.trim().toLowerCase();
        return list.filter((m) => {
            if (category !== 'all' && m.category !== category) return false;
            if (!q) return true;
            return (
                m.name.toLowerCase().includes(q)
                || m.description.toLowerCase().includes(q)
                || (m.tags ?? []).some((t) => t.toLowerCase().includes(q))
                || (m.repo ?? '').toLowerCase().includes(q)
            );
        });
    }, [catalog, filter, category]);

    const installedCount = useMemo(
        () => (catalog?.modules ?? []).filter((m) => m.installed).length,
        [catalog],
    );

    const runAction = useCallback(async (id: string, action: () => Promise<string | void>) => {
        setBusyId(id);
        setMessage('');
        try {
            const status = await action();
            if (typeof status === 'string') setMessage(status);
            await refresh(catalogUrl || undefined);
        } catch (e) {
            setMessage(String(e));
        } finally {
            setBusyId(null);
        }
    }, [refresh, catalogUrl]);

    const kindLabel = (m: CatalogModuleRow) => {
        if (m.native_rust) return 'Rust-native (built-in, zero download)';
        if (m.kind === 'component') return 'Built-in panel (no download)';
        if (m.kind === 'git') {
            const req = (m.requires ?? []).length ? ` · needs ${m.requires!.join(', ')}` : '';
            return `${m.repo ?? 'GitHub'}${req}`;
        }
        return m.kind;
    };

    return (
        <div style={{ maxWidth: 860 }}>
            <div style={{ marginBottom: 16 }}>
                <div style={{ fontWeight: 700, fontSize: 14, marginBottom: 6 }}>Extension Modules</div>
                <p className="settings-section-subtitle" style={{ margin: 0 }}>
                    Keep the IDE lean — install pentest, ML, docs, and mobile security tools on demand.
                    Modules are fetched from the H4D3ZS GitHub catalog and stored under your HADES home directory.
                </p>
            </div>

            <div className="settings-card" style={{ marginBottom: 12 }}>
                <div className="settings-card-title">Catalog source</div>
                <div style={{ display: 'flex', gap: 8, flexWrap: 'wrap', alignItems: 'center' }}>
                    <input
                        className="settings-input"
                        style={{ flex: 1, minWidth: 280, fontSize: 11 }}
                        value={catalogUrl}
                        onChange={(e) => setCatalogUrl(e.target.value)}
                        placeholder="GitHub raw catalog URL"
                    />
                    <button
                        type="button"
                        className="settings-button"
                        disabled={loading}
                        onClick={() => void refresh(catalogUrl || undefined)}
                    >
                        {loading ? 'Refreshing…' : 'Refresh catalog'}
                    </button>
                </div>
                <p className="afi-subtle" style={{ marginTop: 8, fontSize: 10 }}>
                    {installedCount} installed · {catalog?.modules.length ?? 0} available
                    {catalog?.updated ? ` · catalog ${catalog.updated}` : ''}
                </p>
            </div>

            <div style={{ display: 'flex', gap: 8, flexWrap: 'wrap', marginBottom: 12 }}>
                <input
                    className="settings-input"
                    style={{ flex: 1, minWidth: 200 }}
                    placeholder="Search modules…"
                    value={filter}
                    onChange={(e) => setFilter(e.target.value)}
                />
                <select
                    className="settings-select"
                    value={category}
                    onChange={(e) => setCategory(e.target.value)}
                >
                    <option value="all">All categories</option>
                    {categories.map((c) => (
                        <option key={c.id} value={c.id}>{c.label}</option>
                    ))}
                </select>
            </div>

            {(error || message) && (
                <div style={{
                    padding: '8px 12px',
                    marginBottom: 12,
                    borderRadius: 6,
                    fontSize: 11,
                    background: 'rgba(127,29,29,0.35)',
                    border: '1px solid #7f1d1d',
                    color: '#fca5a5',
                }}
                >
                    {error || message}
                </div>
            )}

            <div style={{ display: 'flex', flexDirection: 'column', gap: 8 }}>
                {modules.map((m) => (
                    <ModuleRow
                        key={m.id}
                        module={m}
                        busy={busyId === m.id}
                        onInstall={() => runAction(m.id, () => installModule(m.id, catalogUrl || undefined))}
                        onUninstall={() => runAction(m.id, () => uninstallModule(m.id))}
                        onLaunch={() => runAction(m.id, () => launchInstalledModule(m.id))}
                        onToggle={(enabled) => runAction(m.id, () => setModuleEnabled(m.id, enabled))}
                        kindLabel={kindLabel(m)}
                    />
                ))}
                {!loading && modules.length === 0 && (
                    <p className="afi-subtle">No modules match your filters.</p>
                )}
            </div>
        </div>
    );
};

function ModuleRow({
    module: m,
    busy,
    onInstall,
    onUninstall,
    onLaunch,
    onToggle,
    kindLabel,
}: {
    module: CatalogModuleRow;
    busy: boolean;
    onInstall: () => void;
    onUninstall: () => void;
    onLaunch: () => void;
    onToggle: (enabled: boolean) => void;
    kindLabel: string;
}) {
    return (
        <div
            className="settings-card"
            style={{
                padding: '10px 14px',
                opacity: m.installed && !m.enabled ? 0.72 : 1,
            }}
        >
            <div style={{ display: 'flex', justifyContent: 'space-between', gap: 12, alignItems: 'flex-start' }}>
                <div style={{ flex: 1, minWidth: 0 }}>
                    <div style={{ display: 'flex', alignItems: 'center', gap: 8, flexWrap: 'wrap' }}>
                        <span style={{ fontWeight: 600, fontSize: 13 }}>{m.name}</span>
                        {m.native_rust && (
                            <span className="settings-badge" style={{ background: '#1d4ed8', fontSize: 9 }}>Rust-native</span>
                        )}
                        {m.installed && (
                            <span className="settings-badge" style={{ background: '#238636', fontSize: 9 }}>Installed</span>
                        )}
                        {m.kind === 'git' && !m.installed && (
                            <span className="settings-badge" style={{ fontSize: 9 }}>~{m.size_mb ?? '?'} MB</span>
                        )}
                    </div>
                    <p className="afi-desc" style={{ margin: '4px 0 6px', fontSize: 11 }}>{m.description}</p>
                    <div style={{ fontSize: 10, opacity: 0.55 }}>{kindLabel}</div>
                </div>
                <div style={{ display: 'flex', gap: 6, flexShrink: 0, flexWrap: 'wrap', justifyContent: 'flex-end' }}>
                    {!m.installed && (
                        <button type="button" className="settings-button success" disabled={busy} onClick={onInstall}>
                            {busy ? '…' : 'Install'}
                        </button>
                    )}
                    {m.installed && (
                        <>
                            <button type="button" className="settings-button success" disabled={busy || !m.enabled} onClick={onLaunch}>
                                Open
                            </button>
                            <button
                                type="button"
                                className="settings-button"
                                disabled={busy}
                                onClick={() => onToggle(!m.enabled)}
                            >
                                {m.enabled ? 'Disable' : 'Enable'}
                            </button>
                            <button type="button" className="settings-button danger" disabled={busy} onClick={onUninstall}>
                                Remove
                            </button>
                        </>
                    )}
                </div>
            </div>
        </div>
    );
}

export default ModuleInstallerPanel;
