import React, { useEffect, useState } from 'react';
import { useStore } from '../store';
import { invoke } from '../tauri_bridge';

// ─────────────────────────────────────────────────────────────────────────────
//  WelcomePage — first-launch onboarding screen.
//
//  Shown when there are no open tabs AND the user hasn't dismissed it.
//  Mirrors the structure of VS Code / Cursor's "Get Started":
//    • Recent folders                    (click to open)
//    • Start (new file / open folder)
//    • Walkthroughs (quick tours of key features)
//    • Cybersecurity / agent quick links (red team, blue team, bug bounty)
//
//  The dismissal state is persisted in localStorage so closing it once
//  means it stays closed across reloads. Re-open from the Command Palette
//  via the "Welcome: Get Started" action (TODO future).
// ─────────────────────────────────────────────────────────────────────────────

const DISMISS_KEY = 'welcome.dismissed';

const WelcomePage: React.FC = () => {
    const tabs = useStore(s => s.tabs);
    const activeRoot = useStore(s => s.activeRoot);
    const setActiveRoot = useStore(s => s.setActiveRoot);

    const [dismissed, setDismissed] = useState<boolean>(() => {
        try { return localStorage.getItem(DISMISS_KEY) === '1'; } catch { return false; }
    });
    const [recent, setRecent] = useState<string[]>([]);
    const [neverShowOnStartup, setNeverShowOnStartup] = useState<boolean>(false);

    // Pull the recent-folders list from localStorage. The Sidebar's
    // open-folder action pushes onto this; we just consume it.
    useEffect(() => {
        try {
            const raw = localStorage.getItem('recentFolders');
            const arr = raw ? JSON.parse(raw) : [];
            if (Array.isArray(arr)) setRecent(arr.filter(s => typeof s === 'string').slice(0, 6));
        } catch { /* tracking prevention */ }
    }, []);

    if (dismissed || tabs.length > 0) return null;

    const onOpenFolder = async () => {
        try {
            const folder = await invoke<string | null>('open_folder');
            if (folder) setActiveRoot(folder);
        } catch (e) {
            console.error('open_folder failed', e);
        }
    };

    const dispatchAgent = async (prompt: string) => {
        try {
            // Show the right sidebar, then dispatch the prompt as a fresh
            // user message via the agent. Importing dynamically keeps the
            // Welcome page lightweight on first paint.
            const m = await import('../agent');
            const store = useStore.getState();
            (store as any).setIsRightSidebarOpen?.(true);
            await m.sendAgentMessage(prompt, () => {}, []);
        } catch (e) {
            console.error('Failed to dispatch welcome agent prompt:', e);
        }
    };

    const close = () => {
        if (neverShowOnStartup) {
            try { localStorage.setItem(DISMISS_KEY, '1'); } catch { /* ignore */ }
        }
        setDismissed(true);
    };

    return (
        <div
            style={{
                position: 'absolute',
                inset: 0,
                background: 'var(--vscode-editor-background)',
                color: 'var(--vscode-foreground)',
                display: 'flex',
                flexDirection: 'column',
                overflow: 'auto',
                zIndex: 1,
            }}
        >
            <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', padding: '24px 36px 0 36px' }}>
                <div>
                    <h1 style={{ fontSize: 26, fontWeight: 600, margin: 0 }}>Welcome to vscodium-rust ide</h1>
                    <p style={{ fontSize: 13, opacity: 0.7, marginTop: 4 }}>
                        The agent-native IDE — VS Code core, Cursor parity, Antigravity-style trajectory,
                        Windsurf-style flow, and built-in red/blue team playbooks.
                    </p>
                </div>
                <button onClick={close} style={btnNeutral} title="Dismiss">
                    <i className="codicon codicon-close" style={iconStyle} />
                </button>
            </div>

            <div style={{ display: 'grid', gridTemplateColumns: '1.1fr 0.9fr', gap: 36, padding: '24px 36px 36px 36px' }}>
                {/* Left column */}
                <div>
                    <Section title="Start">
                        <Action
                            icon="folder-opened"
                            label="Open Folder…"
                            description="Open a project on disk."
                            onClick={onOpenFolder}
                        />
                        <Action
                            icon="git-clone"
                            label="Clone Repository…"
                            description="Pull a repository from a Git remote."
                            onClick={async () => {
                                const url = window.prompt('Repository URL to clone:');
                                if (!url) return;
                                try { await invoke('git_clone', { url, into: '' }); } catch (e) { window.alert('Clone failed: ' + e); }
                            }}
                        />
                        <Action
                            icon="new-file"
                            label="New File"
                            description="Create an unsaved scratch buffer."
                            onClick={() => {
                                const s = useStore.getState() as any;
                                s.openTab?.({ id: `tab-${Date.now()}`, path: 'Untitled', content: '', language: 'plaintext', dirty: false });
                            }}
                        />
                    </Section>

                    <Section title="Recent">
                        {recent.length === 0 ? (
                            <div style={{ opacity: 0.5, fontSize: 12, padding: '6px 0' }}>
                                No recent folders.
                            </div>
                        ) : recent.map(p => (
                            <Action
                                key={p}
                                icon="folder"
                                label={p.split(/[\\/]/).slice(-1)[0]}
                                description={p}
                                onClick={() => setActiveRoot(p)}
                            />
                        ))}
                    </Section>
                </div>

                {/* Right column — agentic walkthroughs */}
                <div>
                    <Section title="Walkthroughs">
                        <Action
                            icon="comment-discussion"
                            label="Chat with @-mentions"
                            description="Stack @codebase, @web, @git, @symbol, @problems, @terminal context into your prompts."
                            onClick={() => dispatchAgent('Walk me through using @-mentions in chat. Don\'t do anything yet — just explain the 8 supported mentions and when to use each.')}
                        />
                        <Action
                            icon="diff-multiple"
                            label="Multi-file diff review"
                            description="Edit multiple files in one turn and review every diff before keeping it."
                            onClick={() => dispatchAgent('Demonstrate multi-file editing. Touch two harmless files (e.g. README.md and a comment in an existing source file) so I can see the review carousel.')}
                        />
                        <Action
                            icon="timeline-view-icon"
                            label="Agent trajectory timeline"
                            description="Replay every tool call the agent made step-by-step."
                            onClick={() => {
                                const s = useStore.getState() as any;
                                s.openTrajectory?.();
                            }}
                        />
                        <Action
                            icon="bug"
                            label="Bug Finder (/bugfind)"
                            description="Background pass that surfaces bugs, smells, and CVEs."
                            onClick={() => dispatchAgent('/bugfind')}
                        />
                    </Section>

                    <Section title="Security playbooks">
                        <Action
                            icon="shield"
                            label="Red Team (/redteam)"
                            description="Offensive: recon → classify → weaponize → pivot → report."
                            onClick={() => dispatchAgent('/redteam')}
                        />
                        <Action
                            icon="lock"
                            label="Blue Team (/blueteam)"
                            description="Defensive: inventory → threat model → harden → detect → verify."
                            onClick={() => dispatchAgent('/blueteam')}
                        />
                        <Action
                            icon="search"
                            label="Bug Bounty (/bounty)"
                            description="Scope → recon → PoC → writeup → disclosure."
                            onClick={() => dispatchAgent('/bounty')}
                        />
                    </Section>
                </div>
            </div>

            <div style={{ padding: '0 36px 24px 36px' }}>
                <label style={{ display: 'flex', alignItems: 'center', gap: 8, fontSize: 12, opacity: 0.65 }}>
                    <input
                        type="checkbox"
                        checked={neverShowOnStartup}
                        onChange={(e) => setNeverShowOnStartup(e.target.checked)}
                    />
                    Don't show this on startup
                </label>
            </div>
        </div>
    );
};

const Section: React.FC<{ title: string; children: React.ReactNode }> = ({ title, children }) => (
    <div style={{ marginBottom: 28 }}>
        <div style={{
            fontSize: 11, textTransform: 'uppercase', letterSpacing: 0.5,
            opacity: 0.45, marginBottom: 10, fontWeight: 600,
        }}>{title}</div>
        <div style={{ display: 'flex', flexDirection: 'column', gap: 4 }}>
            {children}
        </div>
    </div>
);

const Action: React.FC<{
    icon: string;
    label: string;
    description: string;
    onClick: () => void;
}> = ({ icon, label, description, onClick }) => (
    <div
        onClick={onClick}
        style={{
            display: 'flex',
            alignItems: 'center',
            gap: 12,
            padding: '8px 10px',
            cursor: 'pointer',
            borderRadius: 4,
        }}
        onMouseEnter={(e) => { e.currentTarget.style.background = 'rgba(255,255,255,0.04)'; }}
        onMouseLeave={(e) => { e.currentTarget.style.background = 'transparent'; }}
    >
        <i
            className={`codicon codicon-${icon}`}
            style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: 18, opacity: 0.85, color: '#c084fc' }}
        />
        <div style={{ flex: 1, minWidth: 0 }}>
            <div style={{ fontSize: 13, fontWeight: 600 }}>{label}</div>
            <div style={{ fontSize: 11, opacity: 0.55 }}>{description}</div>
        </div>
    </div>
);

const btnNeutral: React.CSSProperties = {
    background: 'transparent',
    border: 'none',
    color: 'inherit',
    cursor: 'pointer',
    padding: 4,
    fontSize: 12,
};

const iconStyle: React.CSSProperties = {
    fontFamily: 'codicon',
    fontStyle: 'normal',
    fontSize: 14,
};

export default WelcomePage;
