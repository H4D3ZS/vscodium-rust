import React, { useState, useEffect } from 'react';
import { useShallow } from 'zustand/react/shallow';
import { useStore } from '../store';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';

interface ExtensionDetailsProps {
    extensionId: string;
    onBack: () => void;
}

const ExtensionDetails: React.FC<ExtensionDetailsProps> = ({ extensionId, onBack }) => {
    const {
        extensionDetails,
        fetchExtensionDetails,
        installExtension,
        uninstallExtension,
        installedExtensions,
        requestExtensionTrust,
        addInstalledExtension
    } = useStore(useShallow(s => ({
        extensionDetails: s.extensionDetails,
        fetchExtensionDetails: s.fetchExtensionDetails,
        installExtension: s.installExtension,
        uninstallExtension: s.uninstallExtension,
        installedExtensions: s.installedExtensions,
        requestExtensionTrust: s.requestExtensionTrust,
        addInstalledExtension: s.addInstalledExtension,
    })));

    const details = extensionDetails[extensionId];
    const isInstalled = installedExtensions.some(ext => `${ext.publisher}.${ext.name}` === extensionId || ext.id === extensionId);

    useEffect(() => {
        if (!details) {
            fetchExtensionDetails(extensionId);
        }
    }, [extensionId, details, fetchExtensionDetails]);

    const [isInstalling, setIsInstalling] = useState(false);
    const [error, setError] = useState<string | null>(null);

    if (!details) {
        return (
            <div className="flex items-center justify-center h-full bg-[#0d1117] text-slate-400">
                <div className="flex flex-col items-center gap-6 animate-pulse">
                    <div className="w-20 h-20 rounded-3xl bg-slate-800/40 border border-white/5 shadow-2xl" />
                    <div className="h-2 w-32 bg-slate-800/40 rounded-full" />
                    <div className="h-2 w-48 bg-slate-800/40 rounded-full opacity-60" />
                </div>
            </div>
        );
    }

    const displayName = details.displayName || details.name || details.extensionName;
    const publisher = details.publisher?.publisherName || details.publisher?.displayName || details.publisher || details.namespace || "Unknown Publisher";
    const version = details.version || (details.versions && details.versions[0]?.version) || '0.0.1';
    const description = details.shortDescription || details.description || "No description provided.";

    const icon = details.iconUrl ||
        details.icon_url ||
        details.base64_icon ||
        (details.publisher && details.name ? `https://open-vsx.org/api/${details.publisher}/${details.name}/icon` : null) ||
        (details.versions && details.versions[0]?.files?.find((f: any) => f.assetType === 'Microsoft.VisualStudio.Services.Icons.Default')?.source) ||
        null;

    const handleInstall = async () => {
        setIsInstalling(true);
        setError(null);
        try {
            const trusted = await requestExtensionTrust(publisher, details.name || details.extensionName, version);
            if (trusted) {
                const meta = await installExtension(publisher, details.name || details.extensionName, version);
                if (meta) addInstalledExtension(meta);
            }
        } catch (err: any) {
            console.error("Installation failed:", err);
            setError("Installation failed. Please try again.");
        } finally {
            setIsInstalling(false);
        }
    };

    const handleUninstall = async () => {
        setIsInstalling(true);
        try {
            await uninstallExtension(publisher, details.name || details.extensionName, version);
        } catch (err) {
            console.error("Uninstall failed:", err);
        } finally {
            setIsInstalling(false);
        }
    };

    const statistics = details.statistics || [];
    const installCount = statistics.find((s: any) => s.statisticName === 'install')?.value || details.downloadCount || 0;
    const rating = statistics.find((s: any) => s.statisticName === 'averagerating')?.value || details.averageRating || 0;

    return (
        <div className="extension-details-page" style={{
            display: 'flex',
            flexDirection: 'column',
            height: '100%',
            background: 'var(--vscode-sideBar-background)',
            color: 'var(--vscode-foreground)'
        }}>
            {/* Standard Header */}
            <div className="details-header" style={{
                padding: '10px 16px',
                borderBottom: '1px solid var(--vscode-widget-border, rgba(128,128,128,0.2))',
                display: 'flex',
                alignItems: 'center',
                gap: '12px',
                background: 'var(--vscode-editor-background)',
                zIndex: 10
            }}>
                <button onClick={onBack} className="icon-button" style={{
                    padding: '4px',
                    borderRadius: '4px',
                    background: 'transparent',
                    border: 'none',
                    color: 'inherit',
                    cursor: 'pointer',
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center'
                }}>
                    <i className="codicon codicon-arrow-left" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '16px' }}></i>
                </button>
                <div style={{ flex: 1, minWidth: 0 }}>
                    <h2 style={{ margin: 0, fontSize: '14px', fontWeight: 600, whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis' }}>{displayName}</h2>
                    <div style={{ display: 'flex', alignItems: 'center', gap: '6px', fontSize: '11px', opacity: 0.7 }}>
                        <span style={{ color: 'var(--vscode-textLink-foreground)' }}>{publisher}</span>
                        <span style={{ width: '2px', height: '2px', borderRadius: '50%', background: 'currentColor' }}></span>
                        <span>v{version}</span>
                    </div>
                </div>
            </div>

            <div className="details-scrollable" style={{ flex: 1, overflowY: 'auto', padding: '24px' }}>
                <div style={{ maxWidth: '1000px', margin: '0 auto' }}>
                    {/* Hero Area */}
                    <div style={{ display: 'flex', gap: '24px', marginBottom: '32px', alignItems: 'flex-start' }}>
                        <div className="hero-icon" style={{
                            flexShrink: 0,
                            width: '96px',
                            height: '96px',
                            padding: '8px',
                            background: 'var(--vscode-editor-background)',
                            border: '1px solid var(--vscode-widget-border, rgba(128,128,128,0.2))',
                            borderRadius: '12px'
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
                                    fontFamily: 'codicon', fontStyle: 'normal', fontSize: '32px',
                                    display: 'flex', alignItems: 'center', justifyContent: 'center',
                                    width: '100%', height: '100%', opacity: 0.4,
                                }} />
                            )}
                        </div>
                        <div style={{ flex: 1 }}>
                            <h1 style={{ fontSize: '24px', fontWeight: 600, marginBottom: '8px', color: 'var(--vscode-foreground)' }}>{displayName}</h1>
                            <p style={{ fontSize: '13px', lineHeight: '1.5', opacity: 0.8, marginBottom: '20px' }}>{description}</p>

                            <div style={{ display: 'flex', gap: '12px', alignItems: 'center', flexWrap: 'wrap' }}>
                                {isInstalled ? (
                                    <div style={{ display: 'flex', gap: '8px' }}>
                                        <button className="vscode-button secondary" disabled style={{ opacity: 0.8, cursor: 'default', background: 'var(--vscode-button-secondaryBackground)', color: 'var(--vscode-button-secondaryForeground)', border: 'none', padding: '4px 12px', borderRadius: '2px', display: 'flex', alignItems: 'center' }}>
                                            <i className="codicon codicon-check" style={{ fontFamily: 'codicon', fontStyle: 'normal', marginRight: '6px' }}></i> Installed
                                        </button>
                                        <button onClick={handleUninstall} className="vscode-button" style={{ background: 'transparent', color: 'var(--vscode-errorForeground)', border: '1px solid var(--vscode-errorForeground)', padding: '4px 12px', borderRadius: '2px', cursor: 'pointer' }}>
                                            Uninstall
                                        </button>
                                        <button className="vscode-button secondary" style={{ background: 'var(--vscode-button-secondaryBackground)', color: 'var(--vscode-button-secondaryForeground)', border: 'none', padding: '4px 8px', borderRadius: '2px', cursor: 'pointer' }}>
                                            <i className="codicon codicon-gear" style={{ fontFamily: 'codicon', fontStyle: 'normal' }}></i>
                                        </button>
                                    </div>
                                ) : (
                                    <button
                                        onClick={handleInstall}
                                        disabled={isInstalling}
                                        className="vscode-button primary"
                                        style={{
                                            padding: '6px 20px',
                                            fontSize: '13px',
                                            background: 'var(--vscode-button-background)',
                                            color: 'var(--vscode-button-foreground)',
                                            border: 'none',
                                            borderRadius: '2px',
                                            cursor: 'pointer'
                                        }}
                                    >
                                        {isInstalling ? (
                                            <>
                                                <i className="codicon codicon-loading codicon-modifier-spin" style={{ fontFamily: 'codicon', fontStyle: 'normal', marginRight: '8px' }}></i>
                                                Installing...
                                            </>
                                        ) : 'Install'}
                                    </button>
                                )}

                                <div style={{ display: 'flex', gap: '16px', marginLeft: '12px', paddingLeft: '16px', borderLeft: '1px solid var(--vscode-widget-border, rgba(128,128,128,0.2))' }}>
                                    <div style={{ textAlign: 'center' }}>
                                        <div style={{ fontSize: '10px', textTransform: 'uppercase', opacity: 0.5, fontWeight: 600 }}>Installs</div>
                                        <div style={{ fontWeight: 600, fontSize: '13px' }}>{installCount.toLocaleString()}</div>
                                    </div>
                                    <div style={{ textAlign: 'center' }}>
                                        <div style={{ fontSize: '10px', textTransform: 'uppercase', opacity: 0.5, fontWeight: 600 }}>Rating</div>
                                        <div style={{ fontWeight: 600, fontSize: '13px', display: 'flex', alignItems: 'center', gap: '4px' }}>
                                            {rating > 0 ? rating.toFixed(1) : '—'}
                                            <i className="codicon codicon-star-full" style={{ fontFamily: 'codicon', fontStyle: 'normal', color: 'var(--vscode-terminal-ansiYellow, #f1c40f)', fontSize: '10px' }}></i>
                                        </div>
                                    </div>
                                </div>
                            </div>
                            {error && <div style={{ color: 'var(--vscode-errorForeground)', fontSize: '11px', marginTop: '8px' }}>{error}</div>}
                        </div>
                    </div>

                    {/* Content Grid */}
                    <div style={{ display: 'grid', gridTemplateColumns: '1fr 240px', gap: '32px' }}>
                        <div className="main-content">
                            <div className="content-card" style={{
                                padding: '24px',
                                border: '1px solid var(--vscode-widget-border, rgba(128,128,128,0.1))',
                                background: 'var(--vscode-editor-background)',
                                borderRadius: '4px'
                            }}>
                                <h3 style={{ marginTop: 0, marginBottom: '16px', fontSize: '14px', display: 'flex', alignItems: 'center', gap: '8px' }}>
                                    <i className="codicon codicon-book" style={{ fontFamily: 'codicon', fontStyle: 'normal', color: 'var(--vscode-textLink-foreground)', fontSize: '14px' }}></i>
                                    Extension Details
                                </h3>
                                <div className="extension-markdown-content" style={{ opacity: 0.9, lineHeight: '1.6', fontSize: '13px' }}>
                                    <ReactMarkdown remarkPlugins={[remarkGfm]}>
                                        {details.readme || details.description || "No overview available."}
                                    </ReactMarkdown>
                                </div>
                            </div>
                        </div>

                        <div className="sidebar-content">
                            <div className="info-card" style={{
                                padding: '16px',
                                background: 'var(--vscode-editor-background)',
                                border: '1px solid var(--vscode-widget-border, rgba(128,128,128,0.1))',
                                borderRadius: '4px',
                                position: 'sticky',
                                top: '0'
                            }}>
                                <h4 style={{ marginTop: 0, fontSize: '10px', textTransform: 'uppercase', opacity: 0.6, fontWeight: 700, marginBottom: '16px' }}>Information</h4>

                                <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
                                    <div>
                                        <div style={{ fontSize: '10px', opacity: 0.6, marginBottom: '2px' }}>Identifier</div>
                                        <code style={{ fontSize: '11px', background: 'var(--vscode-badge-background)', color: 'var(--vscode-badge-foreground)', padding: '2px 4px', borderRadius: '2px' }}>{extensionId}</code>
                                    </div>
                                    <div>
                                        <div style={{ fontSize: '10px', opacity: 0.6, marginBottom: '2px' }}>Publisher</div>
                                        <div style={{ fontSize: '12px', fontWeight: 500 }}>{publisher}</div>
                                    </div>
                                    <div>
                                        <div style={{ fontSize: '10px', opacity: 0.6, marginBottom: '2px' }}>License</div>
                                        <div style={{ fontSize: '12px' }}>{details.license || 'Proprietary'}</div>
                                    </div>
                                    <div>
                                        <div style={{ fontSize: '10px', opacity: 0.6, marginBottom: '2px' }}>Updated</div>
                                        <div style={{ fontSize: '12px' }}>
                                            {details.lastUpdated ? new Date(details.lastUpdated).toLocaleDateString() : 'N/A'}
                                        </div>
                                    </div>
                                </div>

                                <div style={{ marginTop: '20px', paddingTop: '16px', borderTop: '1px solid var(--vscode-widget-border, rgba(128,128,128,0.1))' }}>
                                    <div style={{ fontSize: '10px', opacity: 0.6, marginBottom: '10px' }}>Links</div>
                                    <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                                        <a href={details.homepage} style={{ fontSize: '12px', color: 'var(--vscode-textLink-foreground)', textDecoration: 'none', display: 'flex', alignItems: 'center', gap: '6px' }}>
                                            <i className="codicon codicon-home" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px' }}></i> Homepage
                                        </a>
                                        <a href={details.repository?.url} style={{ fontSize: '12px', color: 'var(--vscode-textLink-foreground)', textDecoration: 'none', display: 'flex', alignItems: 'center', gap: '6px' }}>
                                            <i className="codicon codicon-github" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px' }}></i> Repository
                                        </a>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <style>{`
281:         .extension-markdown-content h1, 
282:         .extension-markdown-content h2, 
283:         .extension-markdown-content h3 {
284:             border-bottom: 1px solid var(--vscode-widget-border, rgba(128,128,128,0.1));
285:             padding-bottom: 4px;
286:             margin-top: 24px;
287:             color: var(--vscode-foreground);
288:             font-size: 1.1em;
289:         }
290:         .extension-markdown-content code {
291:             background: var(--vscode-textCodeBlock-background, rgba(128,128,128,0.1));
292:             padding: 1px 4px;
293:             border-radius: 3px;
294:             font-family: var(--vscode-editor-font-family);
295:         }
296:         .extension-markdown-content pre {
297:             background: var(--vscode-textCodeBlock-background, rgba(0,0,0,0.1));
298:             padding: 12px;
299:             border-radius: 4px;
300:             overflow-x: auto;
301:             border: 1px solid var(--vscode-widget-border, rgba(128,128,128,0.1));
302:         }
303:         .extension-markdown-content img {
304:             max-width: 100%;
305:             border-radius: 4px;
306:         }
307:         .vscode-button.primary:hover {
308:             filter: brightness(1.1);
309:         }
310:         .vscode-button.secondary:hover {
311:             background: var(--vscode-button-secondaryHoverBackground) !important;
312:         }
      `}</style>
        </div>
    );
};

export default ExtensionDetails;
