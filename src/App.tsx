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
            initTerminal(() => useStore.getState().addTerminalGroup());
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
        // Priority: 1) localStorage (user's last session, if the folder still
        // exists on disk), 2) backend's cwd (first launch / stale entry).
        // The path_exists probe is what stops a deleted workspace (e.g.
        // `manus_source_code`) from leaking into terminal cwd and breaking
        // every new PTY on launch.
        const fallbackToBackendRoot = () => {
            invoke<string | null>('get_active_root')
                .then((backendRoot) => {
                    if (backendRoot) setActiveRoot(backendRoot);
                })
                .catch(console.error);
        };

        if (activeRoot) {
            const cleaned = activeRoot.split('\0')[0].trim();
            invoke<boolean>('path_exists', { path: cleaned })
                .then((exists) => {
                    if (!exists) {
                        console.warn('[App] saved activeRoot no longer exists — clearing:', cleaned);
                        localStorage.removeItem('activeRoot');
                        localStorage.removeItem('activeRootName');
                        useStore.setState({ activeRoot: null, activeRootName: null, fileTree: [] });
                        fallbackToBackendRoot();
                        return;
                    }
                    invoke('set_active_root', { path: cleaned })
                        .then(() => refreshFileTree())
                        .catch((err) => {
                            console.warn('[App] set_active_root rejected — falling back:', err);
                            localStorage.removeItem('activeRoot');
                            localStorage.removeItem('activeRootName');
                            useStore.setState({ activeRoot: null, activeRootName: null, fileTree: [] });
                            fallbackToBackendRoot();
                        });
                })
                .catch((err) => {
                    console.warn('[App] path_exists failed — falling back:', err);
                    fallbackToBackendRoot();
                });
        } else {
            fallbackToBackendRoot();
        }

        // Listen for reload-window from backend
        import('@tauri-apps/api/event').then(({ listen }) => {
            listen('reload-window', () => {
                window.location.reload();
            });
        });

        // DIAGNOSTIC: trace every isRightSidebarOpen change to find the auto-close culprit
        const unsubDiag = useStore.subscribe((state, prev) => {
            if (state.isRightSidebarOpen !== prev.isRightSidebarOpen) {
                console.trace(`[DIAG] isRightSidebarOpen changed: ${prev.isRightSidebarOpen} → ${state.isRightSidebarOpen}`);
            }
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
