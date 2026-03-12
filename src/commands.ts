import { useStore } from './store';

type Command = {
    id: string;
    label: string;
    run: () => void;
};

let commands: Command[] = [];
let paletteInitialized = false;

function getStore(): any {
    // Zustand store can be accessed outside React via getState()
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
            id: 'workbench.action.showCommands',
            label: 'View: Show Command Palette',
            run: () => openCommandPalette(),
        },
    ];
}

function openCommandPalette() {
    const palette = document.getElementById('command-palette');
    const input = document.getElementById('command-input') as HTMLInputElement | null;
    const list = document.getElementById('command-list');
    if (!palette || !input || !list) return;

    palette.classList.remove('hidden');
    input.value = '';
    renderCommandList(commands, list, 0);
    input.focus();

    let selectedIndex = 0;

    const onKeyDown = (e: KeyboardEvent) => {
        if (e.key === 'Escape') {
            e.stopPropagation();
            palette.classList.add('hidden');
            document.removeEventListener('keydown', onKeyDown);
            return;
        }
        if (e.key === 'ArrowDown' || e.key === 'ArrowUp') {
            e.preventDefault();
            const delta = e.key === 'ArrowDown' ? 1 : -1;
            selectedIndex = (selectedIndex + delta + commands.length) % commands.length;
            renderCommandList(commands, list, selectedIndex);
            return;
        }
        if (e.key === 'Enter') {
            e.preventDefault();
            const cmd = commands[selectedIndex];
            if (cmd) {
                palette.classList.add('hidden');
                document.removeEventListener('keydown', onKeyDown);
                cmd.run();
            }
        }
    };

    const onInput = () => {
        const text = input.value.toLowerCase();
        const filtered = commands.filter(c => c.label.toLowerCase().includes(text));
        selectedIndex = 0;
        renderCommandList(filtered, list, selectedIndex);
        commands = filtered.length > 0 ? filtered : commands;
    };

    input.oninput = onInput;
    document.addEventListener('keydown', onKeyDown);
}

function renderCommandList(cmds: Command[], container: HTMLElement, selectedIndex: number) {
    container.innerHTML = '';
    cmds.forEach((cmd, idx) => {
        const div = document.createElement('div');
        div.className = 'command-item';
        div.style.padding = '4px 10px';
        div.style.fontSize = '13px';
        div.style.cursor = 'pointer';
        if (idx === selectedIndex) {
            div.style.backgroundColor = 'var(--vscode-list-activeSelectionBackground)';
            div.style.color = 'var(--vscode-list-activeSelectionForeground)';
        }
        div.innerText = cmd.label;
        div.onclick = () => cmd.run();
        container.appendChild(div);
    });
}

function handleGlobalKeydown(e: KeyboardEvent) {
    const isMac = navigator.platform.toLowerCase().includes('mac');
    const cmd = isMac ? e.metaKey : e.ctrlKey;

    // Cmd/Ctrl+Shift+P → Command Palette
    if (cmd && e.shiftKey && e.key.toLowerCase() === 'p') {
        e.preventDefault();
        openCommandPalette();
        return;
    }

    // Cmd/Ctrl+P → quick open placeholder (reuse command palette for now)
    if (cmd && !e.shiftKey && e.key.toLowerCase() === 'p') {
        e.preventDefault();
        openCommandPalette();
        return;
    }

    // Cmd/Ctrl+B → toggle sidebar
    if (cmd && !e.shiftKey && e.key.toLowerCase() === 'b') {
        e.preventDefault();
        getStore().toggleSidebar();
        return;
    }

    // Cmd/Ctrl+Alt+B → toggle right sidebar (Auxiliary Bar)
    if (cmd && e.altKey && !e.shiftKey && e.key.toLowerCase() === 'b') {
        e.preventDefault();
        getStore().toggleRightSidebar();
        return;
    }

    // Cmd/Ctrl+J → toggle panel
    if (cmd && !e.shiftKey && e.key.toLowerCase() === 'j') {
        e.preventDefault();
        getStore().toggleBottomPanel();
        return;
    }

    // Cmd/Ctrl+W → close active editor
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

    // Expose to title bar click
    (window as any).showCommandPalette = () => openCommandPalette();

    document.addEventListener('keydown', handleGlobalKeydown);
    paletteInitialized = true;
}

