import React, { useEffect, Suspense, lazy } from 'react';
import { invoke } from './tauri_bridge';
import TitleBar from './components/TitleBar';
import Workbench from './components/Workbench';
import StatusBar from './components/StatusBar';
import './styles.css';
import './panes.css';
import './settings.css';
import { TrustDialog } from './components/TrustDialog';
import { useStore } from './store.ts';
import { initCommands } from './commands.ts';
import { initTheme } from './theme_engine';
import { scheduleDeferredInit, DEFERRED_INIT_MS } from './memory_budget';

// Lazy-load modal/overlay components — they only render on user trigger,
// keeping the initial bundle ~200KB smaller and saving renderer RAM.
const CommandPalette = lazy(() => import('./components/CommandPalette'));
const MultiFileReview = lazy(() => import('./components/agent/MultiFileReview'));
const OllamaFirstLaunchWizard = lazy(() => import('./components/onboarding/OllamaFirstLaunchWizard'));
const AgentSetupWizard = lazy(() => import('./components/onboarding/AgentSetupWizard'));
const TrajectoryPanel = lazy(() => import('./components/agent/TrajectoryPanel'));
const QuickOpen = lazy(() => import('./components/QuickOpen'));
const ToolPermissionDialog = lazy(() => import('./components/ToolPermissionDialog'));

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

        scheduleDeferredInit(() => {
            import('./application/debug/bootstrapDebugRuntime').then(m => m.bootstrapDebugRuntime());
            import('./application/workspace/multiRootWorkspace').then(m => m.initWorkspaceFoldersFromStorage());
        }, 2_000);

        scheduleDeferredInit(() => {
            import('./search').then(m => m.initSearch());
            import('./status_bar').then(m => m.initStatusBar());
            import('./extensions').then(m => m.initExtensions()).then(() => {
                import('./specs').then(m => m.initSpecs());
                import('./mobile').then(m => m.initMobile());
                import('./scm').then(m => m.initScm());
                import('./debug_ui').then(m => m.initDebugUI());
            });
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

        scheduleDeferredInit(() => {
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
        import('@tauri-apps/api/event').then(({ listen }) => {
            listen('reload-window', () => {
                window.location.reload();
            });

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
            });

            // Auto-open files created by the agent (Cursor-style)
            listen('editor_open_file', (event: any) => {
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
            });
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
            unsubAgentRuntime?.();
            unsubMemoryGov?.();
        };
    }, []);

    return (
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
                <ToolPermissionDialog />
                <OllamaFirstLaunchWizard />
                <AgentSetupWizard />
                <MultiFileReview />
                <TrajectoryPanel />
            </Suspense>
        </div>
    );
};

export default App;
