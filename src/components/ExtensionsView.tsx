import React, { useState, useEffect } from 'react';
import { useShallow } from 'zustand/react/shallow';
import { useStore } from '../store';
import { invoke } from '@tauri-apps/api/core';
import ExtensionDetails from './ExtensionDetails';

interface ExtensionItemProps {
    ext: any;
    isInstalled?: boolean;
    onInstall?: () => void;
    onClick?: () => void;
}

const ExtensionItem: React.FC<ExtensionItemProps> = ({ ext, isInstalled, onInstall, onClick }) => {
    const [installing, setInstalling] = useState(false);
    const [installed, setInstalled] = useState(isInstalled);
    const [error, setError] = useState<string | null>(null);

    const displayName = ext.displayName || ext.name;
    const publisher = ext.publisher || ext.namespace || ext.publisherName;
    const name = ext.name;
    const version = ext.version;
    const description = ext.description || "No description provided.";

    // Icon resolution: prefer local/base64, then remote. The final fallback
    // is null — the render shows a codicon placeholder when no icon URL is
    // available, keeping the extensions view fully functional offline.
    const icon = ext.iconUrl ||
        ext.icon_url ||
        ext.base64_icon ||
        (publisher && name ? `https://${publisher}.gallery.vsassets.io/_apis/public/gallery/publisher/${publisher}/extension/${name}/latest/assetbyname/Microsoft.VisualStudio.Services.Icons.Default` : null) ||
        (publisher && name ? `https://open-vsx.org/api/${publisher}/${name}/icon` : null) ||
        null;

    // Format stats
    const downloads = ext.downloadCount ? (ext.downloadCount > 1000 ? (ext.downloadCount / 1000).toFixed(1) + "k" : ext.downloadCount) : null;
    const rating = ext.averageRating ? ext.averageRating.toFixed(1) : null;

    const addInstalledExtension = useStore(state => state.addInstalledExtension);
    const requestTrust = useStore(state => state.requestExtensionTrust);

    const handleInstall = async (e: React.MouseEvent) => {
        e.stopPropagation();
        if (installed || installing) return;

        const trusted = await requestTrust(publisher, ext.name, version);
        if (!trusted) return;

        setInstalling(true);
        setError(null);
        try {
            const meta = await invoke("install_extension", {
                publisher,
                name: ext.name,
                version
            });
            setInstalled(true);
            addInstalledExtension(meta);
            if (onInstall) onInstall();
        } catch (err: any) {
            console.error("Installation failed:", err);
            // Backend errors are now actionable (timeouts/offline/rate-limit) — show them.
            setError(String(err).slice(0, 120) || 'Failed');
            setTimeout(() => setError(null), 8000);
        } finally {
            setInstalling(false);
        }
    };

    return (
        <div className={`extension-item interactive-hover ${installed ? 'installed' : ''}`}
            onClick={onClick}
            style={{
                display: 'flex',
                padding: '10px 12px',
                borderBottom: '1px solid var(--glass-border)',
                gap: '12px',
                fontSize: '13px',
                cursor: 'pointer',
                position: 'relative',
                transition: 'background 0.2s ease'
            }}>
            <div className="extension-icon glass-card" style={{
                flexShrink: 0,
                width: '42px',
                height: '42px',
                padding: '4px',
                background: 'rgba(255, 255, 255, 0.03)'
            }}>
                {icon ? (
                    <img
                        src={icon}
                        alt={displayName}
                        style={{ width: '100%', height: '100%', borderRadius: '4px', objectFit: 'contain' }}
                        onError={(e) => { (e.target as HTMLImageElement).style.display = 'none'; }}
                    />
                ) : (
                    <i className="codicon codicon-extension" style={{
                        fontFamily: 'codicon', fontStyle: 'normal', fontSize: '24px',
                        display: 'flex', alignItems: 'center', justifyContent: 'center',
                        width: '100%', height: '100%', opacity: 0.4,
                    }} />
                )}
            </div>
            <div className="extension-details" style={{ flex: 1, minWidth: 0 }}>
                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'baseline', gap: '8px' }}>
                    <span style={{ fontWeight: 600, color: 'var(--vscode-foreground)', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{displayName}</span>
                    <span style={{ fontSize: '11px', opacity: 0.6 }}>v{version}</span>
                </div>
                <div style={{ color: 'var(--vscode-textLink-foreground)', fontSize: '12px', marginBottom: '2px' }}>{publisher}</div>
                <div style={{ opacity: 0.5, fontSize: '12px', display: '-webkit-box', WebkitLineClamp: 2, WebkitBoxOrient: 'vertical', overflow: 'hidden', lineHeight: '1.3' }}>{description}</div>

                <div style={{ display: 'flex', gap: '12px', marginTop: '6px', fontSize: '11px', opacity: 0.5, alignItems: 'center' }}>
                    {downloads && <span><i className="codicon codicon-cloud-download" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '11px', marginRight: '4px' }}></i>{downloads}</span>}
                    {rating && <span><i className="codicon codicon-star-full" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '11px', marginRight: '4px', color: '#f1c40f' }}></i>{rating}</span>}
                    {ext.categories && ext.categories.length > 0 && (
                        <span style={{ padding: '1px 6px', background: 'rgba(255,255,255,0.05)', borderRadius: '4px' }}>
                            {ext.categories[0]}
                        </span>
                    )}
                </div>

                {installed && (
                    <div className="extension-actions" style={{ display: 'flex', gap: '12px', marginTop: '8px', fontSize: '14px', opacity: 0.7 }}>
                        <i className="codicon codicon-settings-gear interactive-hover" style={{ fontFamily: 'codicon', fontStyle: 'normal', cursor: 'pointer' }} title="Extension Settings"></i>
                        <i className="codicon codicon-debug-pause interactive-hover" style={{ fontFamily: 'codicon', fontStyle: 'normal', cursor: 'pointer' }} title="Disable"></i>
                        <i className="codicon codicon-trash interactive-hover" style={{ fontFamily: 'codicon', fontStyle: 'normal', cursor: 'pointer', color: 'var(--vscode-errorForeground)' }} title="Uninstall"></i>
                    </div>
                )}
            </div>

            {!installed && (
                <button
                    className="glass-button primary"
                    onClick={handleInstall}
                    disabled={installing}
                    style={{
                        padding: '4px 10px',
                        fontSize: '11px',
                        fontWeight: 600,
                        alignSelf: 'center',
                        flexShrink: 0,
                        marginLeft: 'auto'
                    }}
                >
                    {installing ? 'Installing...' : (error || 'Install')}
                </button>
            )}
        </div>
    );
};


const ExtensionsView: React.FC = () => {
    const {
        installedExtensions,
        marketExtensions,
        popularExtensions,
        isSearchingExtensions,
        searchExtensions,
        refreshInstalledExtensions,
        refreshPopularExtensions,
        selectedExtensionId,
        setSelectedExtensionId
    } = useStore(useShallow(s => ({
        installedExtensions: s.installedExtensions,
        marketExtensions: s.marketExtensions,
        popularExtensions: s.popularExtensions,
        isSearchingExtensions: s.isSearchingExtensions,
        searchExtensions: s.searchExtensions,
        refreshInstalledExtensions: s.refreshInstalledExtensions,
        refreshPopularExtensions: s.refreshPopularExtensions,
        selectedExtensionId: s.selectedExtensionId,
        setSelectedExtensionId: s.setSelectedExtensionId,
    })));

    const [searchQuery, setSearchQuery] = useState('');
    const [activeAccordion, setActiveAccordion] = useState<string | null>('marketplace');

    useEffect(() => {
        // Start the extension host (Node sidecar, ~34 MB) on first open of this
        // view instead of at app boot — most sessions never touch Extensions.
        // initExtensions() also does refreshInstalled/refreshPopular internally.
        void import('../extensions').then(m => m.initExtensions()).catch(() => {
            refreshInstalledExtensions();
            refreshPopularExtensions();
        });
    }, []);

    const handleSearch = (e: React.FormEvent) => {
        e.preventDefault();
        searchExtensions(searchQuery);
        if (searchQuery) setActiveAccordion('marketplace');
    };

    const toggleAccordion = (id: string) => {
        setActiveAccordion(activeAccordion === id ? null : id);
    };

    const isInstalled = (ext: any) => {
        const id = `${ext.publisher || ext.namespace || ext.publisherName}.${ext.name}`;
        return installedExtensions.some(i => `${i.publisher}.${i.name}` === id);
    };

    if (selectedExtensionId) {
        return <ExtensionDetails extensionId={selectedExtensionId} onBack={() => setSelectedExtensionId(null)} />;
    }

    return (
        <div className="extensions-view glass-panel" style={{ display: 'flex', flexDirection: 'column', height: '100%' }}>
            <div className="extensions-header" style={{ padding: '12px', borderBottom: '1px solid var(--glass-border)' }}>
                <form onSubmit={handleSearch} style={{ position: 'relative' }}>
                    <i className="codicon codicon-search" style={{
                        fontFamily: 'codicon',
                        fontStyle: 'normal',
                        position: 'absolute',
                        left: '10px',
                        top: '50%',
                        transform: 'translateY(-50%)',
                        fontSize: '14px',
                        opacity: 0.5,
                        zIndex: 1
                    }}></i>
                    <input
                        type="text"
                        placeholder="Search Extensions in Marketplace..."
                        value={searchQuery}
                        onChange={(e) => setSearchQuery(e.target.value)}
                        className="glass-input"
                        style={{
                            width: '100%',
                            padding: '8px 32px 8px 32px',
                            fontSize: '12px'
                        }}
                    />
                    {isSearchingExtensions && (
                        <div style={{ position: 'absolute', right: '10px', top: '50%', transform: 'translateY(-50%)' }}>
                            <i className="codicon codicon-loading codicon-modifier-spin" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '14px', opacity: 0.6 }}></i>
                        </div>
                    )}
                </form>
            </div>

            <div className="extensions-content" style={{ flex: 1, overflowY: 'auto' }}>
                {/* Installed Accordion */}
                <div className="accordion-section">
                    <div
                        className="accordion-header interactive-hover"
                        onClick={() => toggleAccordion('installed')}
                        style={{
                            display: 'flex',
                            alignItems: 'center',
                            padding: '8px 12px',
                            fontWeight: 600,
                            cursor: 'pointer',
                            background: 'rgba(255,255,255,0.03)',
                            fontSize: '11px',
                            color: 'var(--vscode-sideBar-foreground)',
                            textTransform: 'uppercase',
                            letterSpacing: '0.5px'
                        }}
                    >
                        <i className={`codicon codicon-chevron-${activeAccordion === 'installed' ? 'down' : 'right'}`} style={{ fontFamily: 'codicon', fontStyle: 'normal', marginRight: '8px', opacity: 0.7 }}></i>
                        <span>Installed</span>
                        <span style={{
                            marginLeft: 'auto',
                            background: 'rgba(255,255,255,0.1)',
                            color: 'var(--vscode-foreground)',
                            padding: '1px 8px',
                            borderRadius: '10px',
                            fontSize: '10px',
                            fontWeight: 700
                        }}>{installedExtensions.length}</span>
                    </div>
                    {activeAccordion === 'installed' && (
                        <div className="accordion-content">
                            {installedExtensions.length > 0 ? (
                                installedExtensions.map(ext => (
                                    <ExtensionItem
                                        key={`${ext.publisher}.${ext.name}`}
                                        ext={ext}
                                        isInstalled={true}
                                        onClick={() => setSelectedExtensionId(`${ext.publisher}.${ext.name}`)}
                                    />
                                ))
                            ) : (
                                <div style={{ padding: '24px', textAlign: 'center', opacity: 0.4, fontSize: '12px' }}>No extensions installed.</div>
                            )}
                        </div>
                    )}
                </div>

                {/* Marketplace Results / Popular Accordion */}
                <div className="accordion-section" style={{ marginTop: '1px' }}>
                    <div
                        className="accordion-header interactive-hover"
                        onClick={() => toggleAccordion('marketplace')}
                        style={{
                            display: 'flex',
                            alignItems: 'center',
                            padding: '8px 12px',
                            fontWeight: 600,
                            cursor: 'pointer',
                            background: 'rgba(255,255,255,0.03)',
                            fontSize: '11px',
                            color: 'var(--vscode-sideBar-foreground)',
                            textTransform: 'uppercase',
                            letterSpacing: '0.5px'
                        }}
                    >
                        <i className={`codicon codicon-chevron-${activeAccordion === 'marketplace' ? 'down' : 'right'}`} style={{ fontFamily: 'codicon', fontStyle: 'normal', marginRight: '8px', opacity: 0.7 }}></i>
                        <span>{searchQuery && marketExtensions.length > 0 ? 'Marketplace' : 'Popular'}</span>
                    </div>
                    {activeAccordion === 'marketplace' && (
                        <div className="accordion-content">
                            {searchQuery ? (
                                marketExtensions.length > 0 ? (
                                    marketExtensions.map(ext => (
                                        <ExtensionItem
                                            key={`${ext.publisher || ext.namespace || ext.publisherName}.${ext.name}`}
                                            ext={ext}
                                            isInstalled={isInstalled(ext)}
                                            onInstall={refreshInstalledExtensions}
                                            onClick={() => setSelectedExtensionId(`${ext.publisher || ext.namespace || ext.publisherName}.${ext.name}`)}
                                        />
                                    ))
                                ) : (
                                    !isSearchingExtensions && <div style={{ padding: '24px', textAlign: 'center', opacity: 0.4, fontSize: '12px' }}>No results found for "{searchQuery}".</div>
                                )
                            ) : (
                                popularExtensions.length > 0 ? (
                                    popularExtensions.map(ext => (
                                        <ExtensionItem
                                            key={`${ext.publisher || ext.namespace || ext.publisherName}.${ext.name}`}
                                            ext={ext}
                                            isInstalled={isInstalled(ext)}
                                            onInstall={refreshInstalledExtensions}
                                            onClick={() => setSelectedExtensionId(`${ext.publisher || ext.namespace || ext.publisherName}.${ext.name}`)}
                                        />
                                    ))
                                ) : (
                                    <div style={{ padding: '24px', textAlign: 'center', opacity: 0.4, fontSize: '12px' }}>Loading popular extensions...</div>
                                )
                            )}
                        </div>
                    )}
                </div>
            </div>
            <style>{`
                .extension-item:hover {
                    background: rgba(255, 255, 255, 0.05) !important;
                }
                .codicon-modifier-spin {
                    animation: codicon-spin 1s linear infinite;
                }
                @keyframes codicon-spin {
                    from { transform: rotate(0deg); }
                    to { transform: rotate(360deg); }
                }
            `}</style>
        </div>
    );
};

export default ExtensionsView;
