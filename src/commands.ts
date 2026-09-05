import { useStore } from './store';
import { invoke } from './tauri_bridge';

export type Command = {
    id: string;
    label: string;
    run: () => void;
};

let commands: Command[] = [];
let paletteInitialized = false;

function getStore(): any {
    return (useStore as any).getState();
}

function registerCoreCommands() {
    const store = getStore();

    commands = [
        {
            id: 'workbench.action.toggleSidebarVisibility',
            label: 'View: Toggle Side Bar Visibility',
            run: () => store.toggleSidebar(),
        },
        {
            id: 'workbench.action.togglePanel',
            label: 'View: Toggle Panel',
            run: () => store.toggleBottomPanel(),
        },
        {
            id: 'workbench.action.toggleAuxiliaryBar',
            label: 'View: Toggle Auxiliary Bar',
            run: () => store.toggleRightSidebar(),
        },
        {
            id: 'agent.action.stop',
            label: 'Agent: Stop',
            run: () => { import('./application/agent/stopAgent').then(m => m.stopAgent()).catch(console.error); },
        },
        {
            id: 'agent.action.newChat',
            label: 'Agent: New Chat',
            run: () => {
                store.clearAgentMessages?.();
                import('./tauri_bridge').then(({ invoke }) => invoke('clear_ai_memory').catch(() => {}));
            },
        },
        {
            id: 'agent.action.openComposer',
            label: 'Agent: Open Composer',
            run: () => store.toggleComposer?.(true),
        },
        {
            id: 'agent.action.togglePlanMode',
            label: 'Agent: Toggle Plan Mode',
            run: () => store.togglePlanMode?.(),
        },
        {
            id: 'agent.action.toggleLiveEdits',
            label: 'Agent: Toggle Live Edits',
            run: () => store.toggleCascadeWriteMode?.(),
        },
        {
            id: 'agent.action.addSelectionToChat',
            label: 'Agent: Add Selection to Chat',
            run: () => {
                const ed = (window as any).activeEditor;
                const sel = ed?.getSelection?.();
                const model = ed?.getModel?.();
                if (!sel || !model || sel.isEmpty?.()) return;
                const text = model.getValueInRange(sel);
                const path = store.activeEditorPath || 'selection';
                store.attachFile?.({ id: `sel-${Date.now()}`, path, name: path.split(/[\\/]/).pop() || 'selection', type: 'file', gist: text });
                store.openAiriPanel?.();
            },
        },
        {
            id: 'workbench.action.openChat',
            label: 'View: Open Chat',
            run: () => store.openAiriPanel?.(),
        },
        {
            id: 'workbench.action.openMcpStore',
            label: 'View: Open MCP Store',
            run: () => store.openMcpStore?.('store'),
        },
        {
            id: 'workbench.action.manageMcpServers',
            label: 'MCP: Manage Servers',
            run: () => store.openMcpStore?.('manage'),
        },
        {
            id: 'workbench.action.openEmulators',
            label: 'View: Mobile Emulators',
            run: () => store.openEmulatorPanel?.(),
        },
        {
            id: 'workbench.action.openBrowser',
            label: 'View: Launch External Browser (Bug Bounty)',
            run: () => {
                import('./application/browser/openBrowserPanel').then(m => m.launchExternalBrowser());
            },
        },
        {
            id: 'workbench.action.toggleBrowser',
            label: 'View: Toggle External Browser',
            run: () => {
                import('./application/browser/openBrowserPanel').then(m => m.toggleExternalBrowser());
            },
        },
        {
            id: 'workbench.action.toggleChat',
            label: 'View: Toggle Chat Panel',
            run: () => {
                const s = getStore();
                if (s.isAiriPanelOpen && s.isRightSidebarOpen) s.closeAiriPanel?.();
                else s.openAiriPanel?.();
            },
        },
        {
            id: 'workbench.view.explorer',
            label: 'View: Show Explorer',
            run: () => store.setActiveSidebarView('explorer-view'),
        },
        {
            id: 'workbench.view.search',
            label: 'View: Show Search',
            run: () => store.setActiveSidebarView('search-view'),
        },
        {
            id: 'workbench.view.securityReview',
            label: 'View: Open Security Review',
            run: () => {
                import('./application/security/runCodebaseSecurityReview').then(m => m.openSecurityReviewPanel());
            },
        },
        {
            id: 'workbench.action.restartLanguageServer',
            label: 'Developer: Restart Language Server',
            run: () => {
                const root = getStore().activeRoot;
                import('./application/lsp/bootstrapLanguageServer').then(async m => {
                    await m.stopLanguageServer();
                    if (root) await m.bootstrapLanguageServer(root);
                });
            },
        },
        {
            id: 'workbench.view.ports',
            label: 'View: Show Ports',
            run: () => {
                getStore().setActivePanelTab('PORTS');
                getStore().toggleBottomPanel();
                if (!getStore().isBottomPanelOpen) getStore().toggleBottomPanel();
            },
        },
        {
            id: 'security.action.runCodebaseReview',
            label: 'Security: Run Codebase Review',
            run: () => {
                import('./application/security/runCodebaseSecurityReview').then(async m => {
                    m.openSecurityReviewPanel();
                    const depth = getStore().securityReviewDepth ?? 'deep';
                    try {
                        await m.runCodebaseSecurityReview({ depth });
                    } catch { /* shown in panel */ }
                });
            },
        },
        {
            id: 'workbench.view.scm',
            label: 'View: Show Source Control',
            run: () => store.setActiveSidebarView('scm-view'),
        },
        {
            id: 'workbench.view.extensions',
            label: 'View: Show Extensions',
            run: () => store.setActiveSidebarView('extensions-view'),
        },
        {
            id: 'workbench.view.vectorSearch',
            label: 'View: Codebase Search',
            run: () => store.setActiveSidebarView('vector-search-view'),
        },
        {
            id: 'workbench.view.tasks',
            label: 'View: Tasks & Specs',
            run: () => store.setActiveSidebarView('tasks-view'),
        },
        {
            id: 'workbench.view.steering',
            label: 'View: Steering & Hooks',
            run: () => store.setActiveSidebarView('steering-view'),
        },
        {
            id: 'workbench.view.jsonVisualizer',
            label: 'View: JSON Visualizer',
            run: () => {
                const s = getStore();
                const activeTab = s.tabs?.find((t: any) => t.id === s.activeTabId);
                if (activeTab && (activeTab.path?.endsWith('.json') || activeTab.language === 'json')) {
                    s.setVisualLabData?.(activeTab.content);
                    s.setVisualLabMode?.('json');
                }
                s.toggleVisualLab?.(true);
            },
        },
        {
            id: 'workbench.action.openVisualLab',
            label: 'View: Open Visual Lab',
            run: () => {
                const s = getStore();
                const activeTab = s.tabs?.find((t: any) => t.id === s.activeTabId);
                if (activeTab?.content) s.setVisualLabData?.(activeTab.content);
                if (activeTab?.path?.endsWith('.sql')) {
                    s.setVisualLabMode?.('erd');
                } else if (activeTab?.path?.endsWith('.json') || activeTab?.language === 'json') {
                    s.setVisualLabMode?.('json');
                }
                s.toggleVisualLab?.(true);
            },
        },
        {
            id: 'workbench.action.closeActiveEditor',
            label: 'File: Close Editor',
            run: () => {
                const { activeTabId, closeTab } = getStore();
                if (activeTabId) closeTab(activeTabId);
            },
        },
        {
            id: 'workbench.action.files.save',
            label: 'File: Save',
            run: () => {
                getStore().saveActiveFile();
            },
        },
        {
            id: 'workbench.action.files.saveAs',
            label: 'File: Save As...',
            run: () => {
                getStore().saveActiveFile();
            },
        },
        {
            id: 'workbench.action.files.saveAll',
            label: 'File: Save All',
            run: () => {
                getStore().saveActiveFile();
            },
        },
        {
            id: 'workbench.action.showCommands',
            label: 'View: Show Command Palette',
            run: () => openCommandPalette(),
        },
        {
            id: 'explorer.openFolder',
            label: 'File: Open Folder...',
            run: async () => {
                const result = await invoke<string | null>('open_folder');
                if (result) {
                    store.setActiveRoot(result);
                    await store.addWorkspaceFolder(result);
                }
            },
        },
        {
            id: 'explorer.addFolderToWorkspace',
            label: 'File: Add Folder to Workspace...',
            run: async () => {
                const result = await invoke<string | null>('open_folder');
                if (!result) return;
                await store.addWorkspaceFolder(result);
                if (!store.activeRoot) {
                    store.setActiveRoot(result);
                } else {
                    await store.refreshFileTree();
                }
            },
        },
        {
            id: 'workbench.action.showWelcome',
            label: 'Help: Welcome',
            run: () => store.showWelcomeTab(),
        },
        {
            id: 'explorer.newFile',
            label: 'File: New File...',
            run: async () => {
                const path = getStore().activeRoot;
                if (!path) return;
                const name = window.prompt('Enter file name', 'untitled.txt');
                if (!name) return;
                await invoke('create_file', { path: `${path}/${name}` });
                await store.refreshFileTree();
            },
        },
        {
            id: 'explorer.newFolder',
            label: 'File: New Folder...',
            run: async () => {
                const path = getStore().activeRoot;
                if (!path) return;
                const name = window.prompt('Enter folder name', 'new_folder');
                if (!name) return;
                await invoke('create_directory', { path: `${path}/${name}` });
                await store.refreshFileTree();
            },
        },
        {
            id: 'workbench.action.closeFolder',
            label: 'File: Close Folder',
            run: () => {
                store.closeFolder();
            },
        },
        {
            id: 'workbench.action.saveWorkspaceAs',
            label: 'File: Save Workspace As...',
            run: async () => {
                const { getWorkspaceFolders } = await import('./application/workspace/multiRootWorkspace');
                const folders = getWorkspaceFolders();
                if (folders.length === 0) {
                    alert('Open a folder before saving a workspace.');
                    return;
                }
                const defaultName = `${folders[0]?.name || 'workspace'}.code-workspace`;
                const target = window.prompt('Save workspace as (full path):', defaultName);
                if (!target) return;
                const payload = {
                    folders: folders.map((f) => ({ path: f.path, name: f.name })),
                };
                await invoke('write_file_content', {
                    path: target,
                    content: JSON.stringify(payload, null, 2),
                });
            },
        },
        {
            id: 'security.action.generateExploitArtifact',
            label: 'Security: Generate Exploit Artifact (BugTrace CORE-Ultra)',
            run: async () => {
                const task = window.prompt(
                    'Describe the security artifact to generate (Nuclei template, CVE PoC, JWT cracker, bypass, kernel exploit):',
                );
                if (!task) return;
                try {
                    const res = await invoke<{ artifact?: string }>('apex_exploit_tooling', { task });
                    const artifact = (res && res.artifact) || '';
                    if (!artifact.trim()) {
                        alert('No artifact returned. Is Lemonade running with CORE-Ultra loaded on :13305?');
                        return;
                    }
                    const target = window.prompt('Save artifact to path:', 'exploit_artifact.md');
                    if (!target) return;
                    await invoke('write_file_content', { path: target, content: artifact });
                    await store.openFile(target);
                } catch (e) {
                    alert(`Exploit tooling failed: ${e}`);
                }
            },
        },
        {
            id: 'mobile.mirrorPhysicalIphone',
            label: 'Cyber-Ifrit: Mirror Physical iPhone (USB)',
            run: async () => {
                const s = getStore();
                s.setEmulatorPanelPosition('device');
                s.openEmulatorPanel();
            },
        },
        {
            id: 'mobile.deployFlutterIphone',
            label: 'Cyber-Ifrit: Build & Deploy Flutter to iPhone',
            run: async () => {
                const s = getStore();
                s.setEmulatorPanelPosition('device');
                s.openEmulatorPanel();
                try {
                    const pre = await invoke<{ ready_flutter: boolean; notes: string[] }>('iphone_deploy_preflight');
                    if (!pre.ready_flutter) {
                        alert('Flutter deploy toolchain incomplete:\n\n' + pre.notes.join('\n') +
                            '\n\nOpen the 📱 Device tab → Build & Deploy to configure and run.');
                    }
                } catch { /* panel still opens */ }
            },
        },
        {
            id: 'mobile.deployReactNativeIphone',
            label: 'Cyber-Ifrit: Build & Deploy React Native to iPhone',
            run: async () => {
                const s = getStore();
                s.setEmulatorPanelPosition('device');
                s.openEmulatorPanel();
                try {
                    const pre = await invoke<{ ready_react_native: boolean; notes: string[] }>('iphone_deploy_preflight');
                    if (!pre.ready_react_native) {
                        alert('React Native deploy toolchain incomplete:\n\n' + pre.notes.join('\n') +
                            '\n\nOpen the 📱 Device tab → Build & Deploy to configure and run.');
                    }
                } catch { /* panel still opens */ }
            },
        },
        {
            id: 'git.clone',
            label: 'Git: Clone Repository...',
            run: async () => {
                const url = window.prompt('Enter Repository URL');
                if (!url) return;

                const result = await invoke<string | null>('open_folder');
                if (!result) return;

                try {
                    await invoke('git_clone', { url, path: result });
                    store.setActiveRoot(result);
                } catch (e) {
                    alert(`Clone failed: ${e}`);
                }
            },
        },
        {
            id: 'terminal.new',
            label: 'Terminal: New Terminal',
            run: () => {
                getStore().addTerminalGroup();
            },
        },
        {
            id: 'workbench.action.tasks.build',
            label: 'Terminal: Run Build Task',
            run: () => {
                const runInTerminal = (window as any).runInTerminal;
                if (!runInTerminal) {
                    alert('No active terminal to run build task.');
                    return;
                }
                const isRust = getStore().activeRoot && getStore().activeRoot.includes('rust');
                const cmd = isRust ? 'cargo build' : 'npm run build';
                runInTerminal(cmd);
            },
        },
        {
            id: 'editor.action.wordWrap',
            label: 'View: Toggle Word Wrap',
            run: () => {
                getStore().toggleEditorWordWrap?.();
            },
        },
        {
            id: 'workbench.action.zoomIn',
            label: 'View: Zoom In',
            run: () => {
                try {
                    const el = document.documentElement;
                    el.style.fontSize = `${parseFloat(getComputedStyle(el).fontSize) + 1}px`;
                } catch {}
            },
        },
        {
            id: 'workbench.action.zoomOut',
            label: 'View: Zoom Out',
            run: () => {
                try {
                    const el = document.documentElement;
                    el.style.fontSize = `${parseFloat(getComputedStyle(el).fontSize) - 1}px`;
                } catch {}
            },
        },
        {
            id: 'editor.action.gotoLine',
            label: 'Go: Go to Line...',
            run: () => {
                const editor = (window as any).activeEditor;
                if (editor) {
                    editor.focus();
                    editor.trigger('menu', 'editor.action.gotoLine', null);
                }
            },
        },
        {
            id: 'workbench.action.gotoSymbol',
            label: 'Go: Go to Symbol...',
            run: () => {
                const editor = (window as any).activeEditor;
                if (editor) {
                    editor.focus();
                    editor.trigger('menu', 'workbench.action.gotoSymbol', null);
                }
            },
        },
    ];

    // Expose the command registry so the React CommandPalette component can access it
    (window as any).commandRegistry = commands;
}

export function openCommandPalette() {
    const store = getStore();
    store.setCommandPaletteOpen(true);
    store.setCommandPaletteQuery('');
}

function isTypingInInput(): boolean {
    const el = document.activeElement;
    if (!el) return false;
    const tag = el.tagName.toLowerCase();
    if (tag === 'input' || tag === 'textarea') return true;
    if ((el as HTMLElement).isContentEditable) return true;
    return false;
}

function handleGlobalKeydown(e: KeyboardEvent) {
    const isMac = navigator.platform.toLowerCase().includes('mac');
    const cmd = isMac ? e.metaKey : e.ctrlKey;

    // Always allow command palette (Ctrl+Shift+P) even when typing
    if (cmd && e.shiftKey && e.key.toLowerCase() === 'p') {
        e.preventDefault();
        openCommandPalette();
        return;
    }

    if (cmd && !e.shiftKey && e.key.toLowerCase() === 'p') {
        e.preventDefault();
        openCommandPalette();
        return;
    }

    // Skip ALL other shortcuts when user is typing in an input/textarea
    if (isTypingInInput()) return;

    if (cmd && !e.shiftKey && e.key.toLowerCase() === 'b') {
        e.preventDefault();
        getStore().toggleSidebar();
        return;
    }

    if (cmd && e.altKey && !e.shiftKey && e.key.toLowerCase() === 'b') {
        e.preventDefault();
        getStore().toggleRightSidebar();
        return;
    }

    if (cmd && !e.shiftKey && e.key.toLowerCase() === 'j') {
        e.preventDefault();
        getStore().toggleBottomPanel();
        return;
    }

    if (cmd && !e.shiftKey && e.key.toLowerCase() === 'w') {
        e.preventDefault();
        const { activeTabId, closeTab } = getStore();
        if (activeTabId) closeTab(activeTabId);
        return;
    }

    // Agent: Composer (Ctrl+I) — Cursor-style multi-file edit overlay
    if (cmd && !e.shiftKey && e.key.toLowerCase() === 'i') {
        e.preventDefault();
        getStore().toggleComposer?.();
        return;
    }

    // Agent: New chat (Ctrl+T) — matches title bar hint
    if (cmd && !e.shiftKey && e.key.toLowerCase() === 't') {
        e.preventDefault();
        const s = getStore();
        s.createNewSession?.();
        s.openAiriPanel?.();
        return;
    }

    // Agent: Add selection to chat (Ctrl+Shift+L)
    if (cmd && e.shiftKey && e.key.toLowerCase() === 'l') {
        e.preventDefault();
        (window as any).executeCommand?.('agent.action.addSelectionToChat');
        return;
    }

    // Bug-bounty browser panel (Ctrl+Shift+U — URL)
    if (cmd && e.shiftKey && e.key.toLowerCase() === 'u') {
        e.preventDefault();
        (window as any).executeCommand?.('workbench.action.toggleBrowser');
        return;
    }

    // JSON Visualizer sidebar (Ctrl+Shift+J)
    if (cmd && e.shiftKey && e.key.toLowerCase() === 'j') {
        e.preventDefault();
        (window as any).executeCommand?.('workbench.view.jsonVisualizer');
        return;
    }

    // Security review (Ctrl+Shift+Alt+R)
    if (cmd && e.shiftKey && e.altKey && e.key.toLowerCase() === 'r') {
        e.preventDefault();
        (window as any).executeCommand?.('security.action.runCodebaseReview');
        return;
    }

    // Word wrap (Alt+Z) — VS Code parity
    if (e.altKey && !cmd && !e.shiftKey && e.key.toLowerCase() === 'z') {
        e.preventDefault();
        (window as any).executeCommand?.('editor.action.wordWrap');
        return;
    }
}

export function initCommands() {
    if (paletteInitialized) return;
    registerCoreCommands();

    (window as any).showCommandPalette = () => openCommandPalette();
    (window as any).executeCommand = (id: string, ...args: unknown[]) => {
        const cmd = commands.find(c => c.id === id);
        if (cmd) {
            cmd.run();
            return;
        }
        const extCommands: Map<string, string> | undefined = (window as any).__extensionCommands;
        if (extCommands?.has(id)) {
            void import('./application/extensions/ExtHostBridge').then(({ executeExtensionCommand }) =>
                executeExtensionCommand(id, args),
            );
        }
    };

    document.addEventListener('keydown', handleGlobalKeydown);
    paletteInitialized = true;
}

export function destroyCommands(): void {
    document.removeEventListener('keydown', handleGlobalKeydown);
}
