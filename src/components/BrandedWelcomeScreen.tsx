import React, { useEffect, useState } from 'react';
import { openPyTorchStudio } from '../application/pytorch/openPyTorchStudio';
import { AI_ENGINEER_PILLARS, AI_ENGINEER_TAGLINE } from '../lib/aiEngineerManifesto';
import { useStore } from '../store';

const DISMISS_KEY = 'welcome.dismissed';
const APP_VERSION = '0.1.0';

function RustLogo({ size = 56 }: { size?: number }) {
    return (
        <svg width={size} height={size} viewBox="0 0 144 144" fill="none" aria-hidden>
            <circle cx="72" cy="72" r="66" fill="#1a1a1a" stroke="#dea584" strokeWidth="6" />
            <path
                d="M72 28c-24 0-44 20-44 44 0 8 2 15 6 22l12-6c-3-5-5-10-5-16 0-14 11-25 25-25s25 11 25 25c0 6-2 11-5 16l12 6c4-7 6-14 6-22 0-24-20-44-44-44z"
                fill="#dea584"
            />
            <path d="M58 78h28v12H58V78z" fill="#fff" />
            <path d="M64 54h16v18H64V54z" fill="#fff" />
        </svg>
    );
}

const PILLARS = AI_ENGINEER_PILLARS;

const START_ACTIONS = [
    { label: 'New File...', desc: 'Create in workspace', icon: 'new-file', cmd: 'explorer.newFile' },
    { label: 'Open Folder...', desc: 'Open from filesystem', icon: 'folder-opened', cmd: 'explorer.openFolder' },
    { label: 'PyTorch ML Studio', desc: 'Train & experiment locally', icon: 'beaker', cmd: 'pytorch.mlStudio' },
    { label: 'Clone Repository...', desc: 'Sync with Git', icon: 'source-control', cmd: 'git.clone' },
    { label: 'New AI Project...', desc: 'Specs-to-Code Pipeline', icon: 'sparkle', cmd: 'specs.newProject' },
] as const;

export interface BrandedWelcomeScreenProps {
    compact?: boolean;
}

const BrandedWelcomeScreen: React.FC<BrandedWelcomeScreenProps> = ({ compact = false }) => {
    const tabs = useStore(s => s.tabs);
    const activeRoot = useStore(s => s.activeRoot);
    const setActiveRoot = useStore(s => s.setActiveRoot);
    const recentWorkspaces = useStore(s => s.recentWorkspaces);
    const welcomeForceVisible = useStore(s => s.welcomeForceVisible);
    const setWelcomeForceVisible = useStore(s => s.setWelcomeForceVisible);

    const [dismissed, setDismissed] = useState<boolean>(() => {
        try { return localStorage.getItem(DISMISS_KEY) === '1'; } catch { return false; }
    });
    const [neverShowOnStartup, setNeverShowOnStartup] = useState(false);
    const [recent, setRecent] = useState<string[]>([]);

    useEffect(() => {
        try {
            const raw = localStorage.getItem('recentFolders');
            const arr = raw ? JSON.parse(raw) : [];
            if (Array.isArray(arr)) setRecent(arr.filter(s => typeof s === 'string').slice(0, 6));
        } catch { /* ignore */ }
    }, []);

    useEffect(() => {
        const onShow = () => {
            try { localStorage.removeItem(DISMISS_KEY); } catch { /* */ }
            setDismissed(false);
            setWelcomeForceVisible(true);
        };
        window.addEventListener('welcome:show', onShow);
        return () => window.removeEventListener('welcome:show', onShow);
    }, [setWelcomeForceVisible]);

    const runAction = (cmd: string) => {
        if (cmd === 'specs.newProject') {
            const s = useStore.getState();
            s.setSpecsWizardStep?.('generator');
            s.setSpecsWizardOpen?.(true);
            return;
        }
        if (cmd === 'pytorch.mlStudio') {
            openPyTorchStudio();
            return;
        }
        (window as any).executeCommand?.(cmd);
    };

    const onDismissChange = (checked: boolean) => {
        setNeverShowOnStartup(checked);
        if (checked) {
            try { localStorage.setItem(DISMISS_KEY, '1'); } catch { /* */ }
            setDismissed(true);
            setWelcomeForceVisible(false);
        } else {
            try { localStorage.removeItem(DISMISS_KEY); } catch { /* */ }
            setDismissed(false);
        }
    };

    if (compact) {
        return (
            <div className="welcome-compact">
                <RustLogo size={48} />
                <h2 className="welcome-compact-title">VSCODIUM-RUST IDE</h2>
                <p className="welcome-compact-sub">{AI_ENGINEER_TAGLINE}</p>
                <button type="button" className="welcome-compact-link" onClick={() => useStore.getState().openChatSidebar?.()}>
                    Code with Agent · Ctrl+L
                </button>
            </div>
        );
    }

    const visible = welcomeForceVisible || (!dismissed && tabs.length === 0 && !activeRoot);
    if (!visible) return null;

    const recentList = recentWorkspaces.length > 0
        ? recentWorkspaces
        : recent.map(p => ({ path: p, name: p.split(/[\\/]/).slice(-1)[0], openedAt: 0 }));

    return (
        <div className="welcome-simple welcome-screen-overlay">
            <div className="welcome-simple-inner">
                <header className="welcome-simple-header">
                    <RustLogo />
                    <div className="welcome-simple-heading">
                        <div className="welcome-simple-title-row">
                            <h1>VSCODIUM-RUST IDE</h1>
                            <span className="welcome-simple-version">AIRI CORE v{APP_VERSION}</span>
                        </div>
                        <p className="welcome-simple-tagline">
                            {AI_ENGINEER_TAGLINE}
                        </p>
                    </div>
                </header>

                <div className="welcome-simple-pillars">
                    {PILLARS.map(p => (
                        <div key={p.title} className="welcome-simple-pillar">
                            <div className="welcome-simple-pillar-title">{p.title}</div>
                            <div className="welcome-simple-pillar-desc">{p.desc}</div>
                        </div>
                    ))}
                </div>

                <div className="welcome-simple-columns">
                    <section className="welcome-simple-section">
                        <h2 className="welcome-simple-section-label">Get started</h2>
                        <div className="welcome-simple-actions">
                            {START_ACTIONS.map(item => (
                                <button
                                    key={item.cmd}
                                    type="button"
                                    className="welcome-simple-action"
                                    onClick={() => runAction(item.cmd)}
                                >
                                    <i className={`codicon codicon-${item.icon}`} />
                                    <span>
                                        <span className="welcome-simple-action-label">{item.label}</span>
                                        <span className="welcome-simple-action-desc">{item.desc}</span>
                                    </span>
                                </button>
                            ))}
                        </div>
                    </section>

                    <section className="welcome-simple-section">
                        <h2 className="welcome-simple-section-label">Recent workspaces</h2>
                        {recentList.length === 0 ? (
                            <div className="welcome-simple-empty">
                                <i className="codicon codicon-history" />
                                <span>No recent folders found.</span>
                            </div>
                        ) : (
                            <div className="welcome-simple-recents">
                                {recentList.map(ws => (
                                    <button
                                        key={ws.path}
                                        type="button"
                                        className="welcome-simple-recent"
                                        onClick={() => setActiveRoot(ws.path)}
                                    >
                                        <i className="codicon codicon-folder" />
                                        <span>
                                            <span className="welcome-simple-action-label">{ws.name}</span>
                                            <span className="welcome-simple-action-desc">{ws.path}</span>
                                        </span>
                                    </button>
                                ))}
                            </div>
                        )}
                    </section>
                </div>

                <label className="welcome-simple-dismiss">
                    <input
                        type="checkbox"
                        checked={neverShowOnStartup}
                        onChange={(e) => onDismissChange(e.target.checked)}
                    />
                    Don&apos;t show this on startup
                </label>
            </div>
        </div>
    );
};

export default BrandedWelcomeScreen;
