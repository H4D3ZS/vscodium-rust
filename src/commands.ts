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
                    await new Promise(r => setTimeout(r, 50));
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
                const editor = (window as any).activeEditor;
                if (editor) {
                    const current = editor.getRawOptions?.()?.wordWrap ?? 'off';
                    editor.updateOptions?.({ wordWrap: current === 'off' ? 'on' : 'off' });
                }
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
}

export function initCommands() {
    if (paletteInitialized) return;
    registerCoreCommands();

    (window as any).showCommandPalette = () => openCommandPalette();
    (window as any).executeCommand = (id: string) => {
        const cmd = commands.find(c => c.id === id);
        if (cmd) cmd.run();
    };

    document.addEventListener('keydown', handleGlobalKeydown);
    paletteInitialized = true;
}
