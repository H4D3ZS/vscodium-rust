import React, { useEffect, Suspense, lazy } from 'react';
import { HeroUIProvider } from '@heroui/react';
import { invoke } from './tauri_bridge';
import TitleBar from './components/TitleBar';
import Workbench from './components/Workbench';
import StatusBar from './components/StatusBar';
import './styles.css';
import './panes.css';
import './settings.css';
import './heroui.css';
import { TrustDialog } from './components/TrustDialog';
import { useStore } from './store.ts';
import { initCommands } from './commands.ts';
import { initTheme } from './theme_engine';
import { scheduleDeferredInit, DEFERRED_INIT_MS } from './memory_budget';

// Lazy-load modal/overlay components — they only render on user trigger,
// keeping the initial bundle ~200KB smaller and saving renderer RAM.
const CommandPalette = lazy(() => import('./components/CommandPalette'));
const MultiFileReview = lazy(() => import('./components/agent/MultiFileReview'));
const AgentSetupWizard = lazy(() => import('./components/onboarding/AgentSetupWizard'));
const TrajectoryPanel = lazy(() => import('./components/agent/TrajectoryPanel'));
const QuickOpen = lazy(() => import('./components/QuickOpen'));
const ToolPermissionDialog = lazy(() => import('./components/ToolPermissionDialog'));
const LoginModal = lazy(() => import('./components/auth/LoginModal'));

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
            <div className="menu-item" id="cm-delete" style={{ color: 'var(--color-red)' }}>Delete</div>
            <div className="menu-separator"></div>
            <div className="menu-item" id="cm-palette" onClick={() => (window as any).executeCommand('workbench.action.showCommands')}>Command Palette...</div>
        </div>
    );
};

const App: React.FC = () => {
    const isDebugToolbarOpen = useStore(state => state.isDebugToolbarOpen);

    useEffect(() => {
        (window as any).useStore = useStore;
        let unsubAgentRuntime: (() => void) | undefined;
        let unsubMemoryGov: (() => void) | undefined;
        // Critical path only — defer heavy subsystems until idle.
        initCommands();
        initTheme();
        import('./application/performance/ensureAgentRuntime').then(m => {
            unsubAgentRuntime = m.scheduleAgentRuntimeBootstrap();
        });
        import('./application/performance/memoryGovernor').then(m => {
            unsubMemoryGov = m.scheduleMemoryGovernor();
        });

        // Start periodic auto-save for agent conversations (every 30s)
        import('./application/agent/syncAgentMessages').then(m => {
            m.startAutoSave();
        });

        // Auto-start FCC sidecar if enabled
        if (localStorage.getItem('fcc.enabled') === 'true') {
            import('./tauri_bridge').then(({ invoke }) => {
                invoke('fcc_start').catch(() => {});
            });
        }

        // Save conversation on app close (beforeunload)
        const handleBeforeUnload = () => {
            import('./application/agent/syncAgentMessages').then(m => {
                m.syncAgentMessagesToBackend().catch(() => {});
            });
        };
        window.addEventListener('beforeunload', handleBeforeUnload);

        scheduleDeferredInit(() => {
            import('./application/debug/bootstrapDebugRuntime').then(m => m.bootstrapDebugRuntime());
            import('./application/workspace/multiRootWorkspace').then(m => m.initWorkspaceFoldersFromStorage());
        }, 2_000);

        scheduleDeferredInit(() => {
            // Legacy vanilla-JS init modules (search/status_bar/specs/mobile/scm/
            // debug_ui) were removed — they wired to getElementById() slots the
            // React app no longer renders, so they no-op'd at boot. React panels
            // (SearchView, StatusBar, EmulatorPanel, ScmView, DebugView, …)
            // replaced them. Only the extension host still needs an explicit init.
            import('./extensions').then(m => m.initExtensions());
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

        const { setActiveRoot, activeRoot, refreshFileTree } = useStore.getState();

        scheduleDeferredInit(async () => {
            const st = useStore.getState();
            import('./lib/agentAutonomy').then(({ ensureAgenticAutonomy }) => {
                void ensureAgenticAutonomy(st.agentMode);
            });
            import('./lib/localOllamaAgentDefaults').then(({ migrateLocalOllamaPlannerSettings }) => {
                migrateLocalOllamaPlannerSettings(useStore.getState());
            });
            void st.syncOllamaEndpoint?.()
                .then(() => st.refreshAvailableModels())
                .catch(() => st.refreshAvailableModels());
            // Push the configured Lemonade server URL into the Rust engine on
            // boot so get_endpoint("lemonade") / list_models("lemonade") hit the
            // user's actual port. Without this, a fresh boot with Lemonade as the
            // active backend leaves the engine on the default :13305 until the
            // user opens Settings or switches backends.
            try {
                const lemonadeUrl = st.lemonadeUrl || 'http://localhost:13305';
                const { invoke } = await import('./tauri_bridge');
                await invoke('set_lemonade_url', { url: lemonadeUrl }).catch(() => { });
            } catch { /* non-fatal */ }
        }, DEFERRED_INIT_MS);

        import('./application/workspace/restoreWorkspaceOnBoot').then(({ restoreWorkspaceOnBoot }) =>
            restoreWorkspaceOnBoot(activeRoot, {
                setActiveRoot,
                refreshFileTree,
                clearPersistedRoot: () => {
                    localStorage.removeItem('activeRoot');
                    localStorage.removeItem('activeRootName');
                },
            }),
        );

        // Listen for reload-window from backend
        const unlistens: (() => void)[] = [];
        let active = true;

        import('@tauri-apps/api/event').then(({ listen }) => {
            if (!active) return;
            listen('reload-window', () => {
                window.location.reload();
            }).then((unsub) => { if (active) unlistens.push(unsub); else unsub(); });

            // Wire backend task-phase-update → taskPlannerState in store (B10)
            listen('task-phase-update', (event: any) => {
                const { phase, status, iteration, max_iterations } = event.payload ?? {};
                useStore.setState((s: any) => ({
                    taskPlannerState: {
                        state: phase ?? s.taskPlannerState?.state ?? 'IDLE',
                        status: status ?? '',
                        iteration: iteration ?? 0,
                        maxIterations: max_iterations ?? 50,
                        steps: s.taskPlannerState?.steps ?? [],
                    },
                }));
            }).then((unsub) => { if (active) unlistens.push(unsub); else unsub(); });

            // Auto-open files created by the agent (Cursor-style). On low-end
            // machines this is suppressed: mounting Monaco for each written file
            // mid-generation is a 40–80 MB spike that OOMs a small (RAM-starved)
            // heap. The file is still written/visible in the tree; the user opens
            // it manually when memory allows.
            listen('editor_open_file', (event: any) => {
                if (document.body.classList.contains('low-end')) return;
                const path = event.payload?.path;
                if (path) {
                    const store = useStore.getState();
                    const existing = store.tabs?.find((t: any) => t.path === path);
                    if (existing) {
                        store.setActiveTab?.(existing.id);
                    } else {
                        store.openFile?.(path);
                    }
                }
            }).then((unsub) => { if (active) unlistens.push(unsub); else unsub(); });
        });

        // Expose store to window for debugging/automation (getState/setState/subscribe).
        // NOTE: do not spread getState() here — that snapshots state once and shadows
        // the live store; consumers should call window.useStore.getState().
        (window as any).useStore = {
            getState: () => useStore.getState(),
            setState: (state: any) => useStore.setState(state),
            subscribe: useStore.subscribe,
        };

        return () => {
            active = false;
            unsubAgentRuntime?.();
            unsubMemoryGov?.();
            unlistens.forEach((unsub) => unsub());
            window.removeEventListener('beforeunload', handleBeforeUnload);
            import('./application/agent/syncAgentMessages').then(m => m.stopAutoSave());
        };
    }, []);

    return (
        <HeroUIProvider>
        <div id="vscodium-app-root" style={{ width: '100%', height: '100vh', display: 'flex', flexDirection: 'column', overflow: 'hidden' }}>
            <Suspense fallback={null}>
                <CommandPalette />
                <QuickOpen />
            </Suspense>

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
            <Suspense fallback={null}>
                <LoginModal />
                <ToolPermissionDialog />
                <AgentSetupWizard />
                <MultiFileReview />
                <TrajectoryPanel />
            </Suspense>
        </div>
        </HeroUIProvider>
    );
};

export default App;
