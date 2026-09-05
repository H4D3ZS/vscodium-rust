import React, { useEffect, useState } from 'react';
import { useShallow } from 'zustand/react/shallow';
import { useStore } from '../../store';
import { sanitizeHtml } from '../../lib/markdown';
// Emojis for icons since heroicons are not available
const PuzzlePieceIcon = ({ className }: { className?: string }) => <span className={className}></span>;
const ArrowLeftIcon = ({ className }: { className?: string }) => <span className={className}>⬅</span>;
const GlobeAltIcon = ({ className }: { className?: string }) => <span className={className}></span>;
const ShieldCheckIcon = ({ className }: { className?: string }) => <span className={className}></span>;
const ArrowDownTrayIcon = ({ className }: { className?: string }) => <span className={className}>📥</span>;
const TrashIcon = ({ className }: { className?: string }) => <span className={className}>🗑</span>;
const CheckIcon = ({ className }: { className?: string }) => <span className={className}></span>;
const ExclamationTriangleIcon = ({ className }: { className?: string }) => <span className={className}></span>;


export const ExtensionDetails: React.FC = () => {
  const {
    selectedExtensionId,
    setSelectedExtensionId,
    extensionDetails,
    fetchExtensionDetails,
    installedExtensions,
    installExtension,
    uninstallExtension,
    requestExtensionTrust
  } = useStore(useShallow(s => ({
    selectedExtensionId: s.selectedExtensionId,
    setSelectedExtensionId: s.setSelectedExtensionId,
    extensionDetails: s.extensionDetails,
    fetchExtensionDetails: s.fetchExtensionDetails,
    installedExtensions: s.installedExtensions,
    installExtension: s.installExtension,
    uninstallExtension: s.uninstallExtension,
    requestExtensionTrust: s.requestExtensionTrust,
  })));

  const [isInstalling, setIsInstalling] = useState(false);

  const extension = selectedExtensionId ? extensionDetails[selectedExtensionId] : null;
  const isInstalled = installedExtensions.some(e => e.id === selectedExtensionId);
  const marketplaceExtension = useStore(s => s.marketExtensions.find(e => e.id === selectedExtensionId)) ||
    useStore(s => s.popularExtensions.find(e => e.id === selectedExtensionId));

  const displayExtension = extension || marketplaceExtension;

  useEffect(() => {
    if (selectedExtensionId && !extensionDetails[selectedExtensionId]) {
      fetchExtensionDetails(selectedExtensionId);
    }
  }, [selectedExtensionId, extensionDetails, fetchExtensionDetails]);

  if (!selectedExtensionId) return null;

  if (!displayExtension) {
    return (
      <div className="flex-1 flex flex-col items-center justify-center text-zinc-500 animate-pulse">
        <PuzzlePieceIcon className="w-12 h-12 mb-4 opacity-20" />
        <p>Loading extension details...</p>
      </div>
    );
  }

  const handleInstall = async () => {
    if (displayExtension) {
      setIsInstalling(true);
      try {
        const publisher = displayExtension.namespace || displayExtension.publisher;
        const name = displayExtension.name;
        const version = displayExtension.version;

        const trusted = await requestExtensionTrust(publisher, name, version);
        if (trusted) {
          await installExtension(publisher, name, version);
        }
      } catch (error: any) {
        console.error("Installation failed:", error);
      } finally {
        setIsInstalling(false);
      }
    }
  };

  const handleUninstall = async () => {
    if (displayExtension) {
      await uninstallExtension(displayExtension.namespace || displayExtension.publisher, displayExtension.name, displayExtension.version);
    }
  };

  return (
    <div className="flex-1 flex flex-col h-full bg-[#1e1e1e] overflow-y-auto custom-scrollbar animate-in fade-in slide-in-from-right-4 duration-300">
      {/* Header */}
      <div className="sticky top-0 z-10 bg-[#1e1e1e]/80 backdrop-blur-md border-b border-white/5 p-4 flex items-center gap-4">
        <button
          onClick={() => setSelectedExtensionId(null)}
          className="p-2 hover:bg-white/5 rounded-full transition-colors group"
        >
          <ArrowLeftIcon className="w-5 h-5 text-zinc-400 group-hover:text-white transition-colors" />
        </button>
        <span className="text-sm font-medium text-zinc-300">Extension Details</span>
      </div>

      <div className="p-8 max-w-4xl mx-auto w-full">
        {/* Main Info */}
        <div className="flex gap-8 items-start mb-12">
          <div className="w-32 h-32 flex-shrink-0 bg-white/5 rounded-2xl flex items-center justify-center border border-white/10 shadow-2xl relative group overflow-hidden">
            {displayExtension.iconUrl ? (
              <img src={displayExtension.iconUrl} alt="" className="w-24 h-24 object-contain group-hover:scale-110 transition-transform duration-500" />
            ) : (
              <PuzzlePieceIcon className="w-16 h-16 text-zinc-600 group-hover:text-zinc-400 transition-colors duration-500" />
            )}
            <div className="absolute inset-0 bg-gradient-to-br from-blue-500/10 to-transparent opacity-0 group-hover:opacity-100 transition-opacity" />
          </div>

          <div className="flex-1 space-y-4">
            <div className="flex flex-col gap-1">
              <h1 className="text-3xl font-bold text-white tracking-tight leading-tight">
                {displayExtension.displayName || displayExtension.name}
              </h1>
              <div className="flex items-center gap-2 text-zinc-400 font-medium">
                <span className="text-blue-400 hover:underline cursor-pointer transition-all">{displayExtension.namespace || displayExtension.publisher}</span>
                <span className="w-1 h-1 rounded-full bg-zinc-700" />
                <div className="flex items-center gap-1.5 bg-zinc-800/50 px-2 py-0.5 rounded text-xs border border-white/5">
                  <ShieldCheckIcon className="w-3.5 h-3.5 text-emerald-400" />
                  <span>Trusted</span>
                </div>
              </div>
            </div>

            <p className="text-lg text-zinc-400 leading-relaxed font-light">
              {displayExtension.description}
            </p>

            <div className="flex items-center gap-3 pt-2">
              {isInstalled ? (
                <>
                  <button
                    onClick={handleUninstall}
                    className="px-6 py-2.5 bg-zinc-800 hover:bg-zinc-700 text-white rounded-lg font-medium transition-all border border-white/5 hover:border-white/10 flex items-center gap-2"
                  >
                    <TrashIcon className="w-4 h-4" />
                    Uninstall
                  </button>
                  <button className="px-6 py-2.5 bg-blue-600 hover:bg-blue-50 text-white rounded-lg font-medium transition-all shadow-lg shadow-blue-500/20 flex items-center gap-2">
                    <CheckIcon className="w-4 h-4" />
                    Installed
                  </button>
                </>
              ) : (
                <button
                  onClick={handleInstall}
                  className="px-8 py-2.5 bg-blue-600 hover:bg-blue-500 text-white rounded-lg font-medium transition-all shadow-lg shadow-blue-500/30 flex items-center gap-2 group"
                >
                  <ArrowDownTrayIcon className="w-4 h-4 group-hover:translate-y-0.5 transition-transform" />
                  Install
                </button>
              )}
            </div>
          </div>
        </div>

        {/* Categories/Stats grid */}
        <div className="grid grid-cols-4 gap-4 mb-12">
          {[
            { label: 'Rating', value: `${displayExtension.averageRating?.toFixed(1) || '0.0'} ★`, color: 'text-amber-400' },
            { label: 'Installs', value: displayExtension.downloadCount?.toLocaleString() || '0', color: 'text-zinc-300' },
            { label: 'Version', value: displayExtension.version || '1.0.0', color: 'text-zinc-300' },
            { label: 'Type', value: 'Extension', color: 'text-zinc-300' },
          ].map((stat, i) => (
            <div key={i} className="bg-white/5 border border-white/5 p-4 rounded-xl backdrop-blur-sm">
              <div className="text-[10px] uppercase tracking-widest text-zinc-500 mb-1 font-bold">{stat.label}</div>
              <div className={`text-lg font-semibold ${stat.color}`}>{stat.value}</div>
            </div>
          ))}
        </div>

        {/* Content Tabs TBD - Simplified for now */}
        <div className="space-y-12">
          <section>
            <div className="flex items-center gap-3 mb-6">
              <div className="h-6 w-1 bg-blue-500 rounded-full" />
              <h2 className="text-xl font-bold text-white">Overview</h2>
            </div>
            <div className="prose prose-invert max-w-none text-zinc-400 leading-7">
              {extension?.readme ? (
                <div className="markdown-body bg-transparent p-0" dangerouslySetInnerHTML={{ __html: sanitizeHtml(extension.readme) }} />
              ) : (
                <div className="bg-zinc-900/50 p-6 rounded-2xl border border-white/5 italic flex items-center gap-3">
                  <PuzzlePieceIcon className="w-5 h-5 opacity-50" />
                  Detailed overview not available for this extension.
                </div>
              )}
            </div>
          </section>

          <section>
            <div className="flex items-center gap-3 mb-6">
              <div className="h-6 w-1 bg-blue-500 rounded-full" />
              <h2 className="text-xl font-bold text-white">Metadata</h2>
            </div>
            <div className="bg-white/5 rounded-2xl border border-white/5 overflow-hidden">
              {[
                { label: 'Publisher', value: displayExtension.namespace || displayExtension.publisher },
                { label: 'Identifier', value: `${displayExtension.namespace || displayExtension.publisher}.${displayExtension.name}` },
                { label: 'Last Updated', value: displayExtension.timestamp || 'N/A' },
                { label: 'Repository', value: displayExtension.repository || 'N/A' },
                { label: 'License', value: displayExtension.license || 'N/A' },
              ].map((row, i) => (
                <div key={i} className={`flex items-center p-4 ${i !== 0 ? 'border-t border-white/5' : ''}`}>
                  <div className="w-1/3 text-sm text-zinc-500 font-medium">{row.label}</div>
                  <div className="w-2/3 text-sm text-zinc-300">{row.value}</div>
                </div>
              ))}
            </div>
          </section>
        </div>
      </div>
    </div>
  );
};
