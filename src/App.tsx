import React, { useEffect } from 'react';
import { invoke } from './tauri_bridge';
import TitleBar from './components/TitleBar';
import Workbench from './components/Workbench';
import StatusBar from './components/StatusBar';
import './styles.css';
import './panes.css';
import { TrustDialog } from './components/TrustDialog';
import { initSearch } from './search';
import { initStatusBar } from './status_bar';
import { initExtensions } from './extensions';
import { initSpecs } from './specs';
import { initMobile } from './mobile';
import { useStore } from './store.ts';
import { initCommands } from './commands.ts';
import { initScm } from './scm';
import { initDebugUI } from './debug_ui';
import { initTerminal } from './terminal';
import { initAgent } from './agent';
import { initTheme } from './theme_engine';
import CommandPalette from './components/CommandPalette';
import QuickOpen from './components/QuickOpen';
import RightSidebar from './components/RightSidebar';

const ContextMenu: React.FC = () => {
    const isOpen = useStore(state => state.isContextMenuOpen);
    const pos = useStore(state => state.contextMenuPosition);
    const setOpen = useStore(state => state.setContextMenuOpen);

    if (!isOpen) return null;

    return (
        <div
            id="context-menu"
            className="context-menu"
            style={{ position: 'fixed', left: pos.x, top: pos.y, zIndex: 10000 }}
            onMouseLeave={() => setOpen(false)}
        >
            <div className="menu-item" id="cm-open">Open</div>
            <div className="menu-item" id="cm-reveal">Reveal in Finder</div>
            <div className="menu-separator"></div>
            <div className="menu-item" id="cm-new-file">New File...</div>
            <div className="menu-item" id="cm-new-folder">New Folder...</div>
            <div className="menu-separator"></div>
            <div className="menu-item" id="cm-rename">Rename...</div>
            <div className="menu-item" id="cm-delete" style={{ color: '#f87171' }}>Delete</div>
            <div className="menu-separator"></div>
            <div className="menu-item" id="cm-palette" onClick={() => (window as any).executeCommand('workbench.action.showCommands')}>Command Palette...</div>
        </div>
    );
};

const App: React.FC = () => {
    const isDebugToolbarOpen = useStore(state => state.isDebugToolbarOpen);
    const ollamaUrl = useStore(state => state.ollamaUrl);
    const isWebDemo = !(window as any).__TAURI__;
    const isMixedContentBlocked = isWebDemo
        && window.location.protocol === 'https:'
        && ollamaUrl?.startsWith('http://');

    useEffect(() => {
        (window as any).useStore = useStore;
        // Initialize non-React behaviors once the shell is mounted.
        initCommands();
        initSearch();
        initStatusBar();
        initTheme();
        initExtensions().then(() => {
            initSpecs();
            initMobile();
            initScm();
            initDebugUI();
            initTerminal((shell) => useStore.getState().addTerminalGroup(shell));
            initAgent();
        });

        // --- Platform Detection for Native Feel ---
        const ua = navigator.userAgent.toLowerCase();
        if (ua.includes('mac')) document.body.classList.add('os-macos');
        else if (ua.includes('win')) document.body.classList.add('os-windows');
        else if (ua.includes('linux')) document.body.classList.add('os-linux');

        // Add desktop/web class
        if ((window as any).__TAURI__) document.body.classList.add('is-desktop');
        else document.body.classList.add('is-web');
        // ----------------------------------------

        const { refreshAvailableModels, setActiveRoot, activeRoot, refreshFileTree } = useStore.getState();
        refreshAvailableModels();

        // Restore the active project root on startup.
        // Priority: 1) localStorage (user's last session), 2) backend's cwd (first launch).
        if (activeRoot) {
            // We have a saved root — tell the backend and load the file tree
            invoke('set_active_root', { path: activeRoot })
                .then(() => refreshFileTree())
                .catch(console.error);
        } else {
            // No saved root — ask backend what it was initialized with (cwd)
            invoke<string | null>('get_active_root')
                .then((backendRoot) => {
                    if (backendRoot) {
                        setActiveRoot(backendRoot);
                    }
                })
                .catch(console.error);
        }

        // Listen for reload-window from backend — but DO NOT actually reload.
        // A full window.location.reload() destroys all Tauri IPC listeners, Monaco
        // editor state, and every Zustand subscription. If the backend emits this
        // event, it is a signal that it wants the frontend to reset, but we handle
        // that gracefully via store state resets instead of a hard browser reload.
        import('@tauri-apps/api/event').then(({ listen }) => {
            listen('reload-window', () => {
                console.warn('[STABILITY] Backend emitted reload-window — ignoring hard reload to preserve IPC state.');
                console.warn('[STABILITY] If you intended a full reset, use store.getState().clearAgentMessages() instead.');
                // Soft-reset: clear agent messages and refresh file tree
                const state = useStore.getState();
                state.clearAgentMessages?.();
                state.refreshFileTree?.();
            });
        });

        // DIAGNOSTIC: trace every isRightSidebarOpen change to find the auto-close culprit
        const unsubDiag = useStore.subscribe((state, prev) => {
            // Silenced diagnostic
        });

        // Expose ENTIRE store to window - all state and ALL actions
        (window as any).useStore = {
            getState: () => useStore.getState(),
            setState: (state: any) => useStore.setState(state),
            subscribe: useStore.subscribe,
            // Proxy to expose ALL store functions dynamically
            ...useStore.getState(),
        };

        return () => unsubDiag();
    }, []);

    return (
        <div id="vscodium-app-root" style={{ width: '100%', height: '100vh', display: 'flex', flexDirection: 'column', overflow: 'hidden' }}>
            <CommandPalette />
            <QuickOpen />

            <div className="body-backdrop"></div>
            <TitleBar />
            {isWebDemo && (
                <div className="web-demo-notice" role="status" aria-live="polite">
                    Running in web demo mode. Desktop-only features (native filesystem, terminal process control, emulators, and Tauri integrations) are limited or unavailable in GitHub Pages.
                </div>
            )}
            {isMixedContentBlocked && (
                <div className="web-demo-warning" role="alert">
                    AI calls are blocked in browser mode because this site is HTTPS but Ollama is set to HTTP (`{ollamaUrl}`). Use an HTTPS proxy endpoint or run the desktop app.
                </div>
            )}
            <Workbench />
            <StatusBar />

            {isDebugToolbarOpen && (
                <div id="debug-toolbar" className="debug-toolbar">
                    <div className="debug-tool-item" id="debug-continue" title="Continue (F5)"><i className="codicon codicon-debug-continue" /></div>
                    <div className="debug-tool-item" id="debug-step-over" title="Step Over (F10)"><i className="codicon codicon-debug-step-over" /></div>
                    <div className="debug-tool-item" id="debug-step-into" title="Step Into (F11)"><i className="codicon codicon-debug-step-into" /></div>
                    <div className="debug-tool-item" id="debug-step-out" title="Step Out (Shift+F11)"><i className="codicon codicon-debug-step-out" /></div>
                    <div className="debug-tool-item" id="debug-restart" title="Restart (Ctrl+Shift+F5)"><i className="codicon codicon-debug-restart" /></div>
                    <div className="debug-tool-item stop" id="debug-stop" title="Stop (Shift+F5)"><i className="codicon codicon-debug-stop" /></div>
                </div>
            )}

            <ContextMenu />
            <TrustDialog />
        </div>
    );
};

export default App;
