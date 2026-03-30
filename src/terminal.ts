import { invoke, listen } from './tauri_bridge.ts';
import { Terminal } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import { SearchAddon } from '@xterm/addon-search';
import '@xterm/xterm/css/xterm.css';

export interface TerminalData {
    id: string;
    term: Terminal;
    fitAddon: FitAddon;
    shell: string;
    element: HTMLElement; // Persistent xterm element
    searchAddon: SearchAddon;
}

export class TerminalManager {
    terminals: Map<string, TerminalData>;
    activeId: string | null;
    idCounter: number;

    constructor() {
        this.terminals = new Map();
        this.activeId = null;
        this.idCounter = 1;
    }

    async createTerminal(shell?: string, theme?: any, providedId?: string): Promise<string> {
        const id = providedId || `term-${Date.now()}`;

        // Create a persistent hidden container for this terminal instance
        const element = document.createElement('div');
        element.style.width = '100%';
        element.style.height = '100%';
        element.style.position = 'relative';
        element.className = 'terminal-instance-element';

        const term = new Terminal({
            theme: theme || getVSCodeTheme(),
            fontSize: 12,
            fontFamily: 'Menlo, Monaco, "Courier New", monospace',
            cursorBlink: true,
            allowProposedApi: true
        });

        const fitAddon = new FitAddon();
        const searchAddon = new SearchAddon();
        term.loadAddon(fitAddon);
        term.loadAddon(searchAddon);

        term.open(element);

        const terminalData: TerminalData = {
            id,
            term,
            fitAddon,
            searchAddon,
            element,
            shell: shell || ""
        };

        this.terminals.set(id, terminalData);
        this.activeId = id;

        term.onData((data: string) => invoke("write_to_terminal", { id, data }));
        term.onResize(({ cols, rows }: { cols: number, rows: number }) => {
            invoke("resize_terminal", { id, cols, rows });
        });

        try {
            await invoke("spawn_terminal", { id, shell: terminalData.shell });
            // Defer fit until attached
        } catch (e) {
            term.write(`\r\n\x1b[31mError spawning terminal: ${e}\x1b[0m\r\n`);
        }

        return id;
    }

    attach(id: string, container: HTMLElement) {
        const t = this.terminals.get(id);
        if (t && container) {
            // Append the persistent element to the new container
            container.appendChild(t.element);
            setTimeout(() => {
                try {
                    t.fitAddon.fit();
                    const { cols, rows } = t.term;
                    invoke("resize_terminal", { id, cols, rows });
                } catch (e) { }
            }, 50);
        }
    }

    async closeTerminal(id: string): Promise<void> {
        const t = this.terminals.get(id);
        if (t) {
            await invoke("close_terminal", { id });
            t.term.dispose();
            if (t.element.parentNode) {
                t.element.parentNode.removeChild(t.element);
            }
            this.terminals.delete(id);
            if (this.activeId === id) this.activeId = null;
        }
    }

    async getAvailableShells(): Promise<string[]> {
        return await invoke<string[]>("get_available_shells");
    }

    handleData(termId: string, data: string): void {
        const t = this.terminals.get(termId);
        if (t) t.term.write(data);
    }

    resize(id: string) {
        const t = this.terminals.get(id);
        if (t && t.element.offsetParent) { // Only fit if visible
            try {
                t.fitAddon.fit();
                const { cols, rows } = t.term;
                invoke("resize_terminal", { id, cols, rows });
            } catch (e) { }
        }
    }

    setTheme(id: string, theme: any) {
        const t = this.terminals.get(id);
        if (t) {
            t.term.options.theme = theme;
        }
    }

    updateAllThemes() {
        const theme = getVSCodeTheme();
        for (const data of this.terminals.values()) {
            data.term.options.theme = theme;
        }
    }
}

export const getVSCodeTheme = () => {
    const style = getComputedStyle(document.documentElement);
    return {
        background: style.getPropertyValue('--vscode-terminal-background').trim() || '#1e1e1e',
        foreground: style.getPropertyValue('--vscode-terminal-foreground').trim() || '#cccccc',
        cursor: style.getPropertyValue('--vscode-terminalCursor-foreground').trim() || '#cccccc',
        cursorAccent: style.getPropertyValue('--vscode-terminalCursor-background').trim() || '#1e1e1e',
        selectionBackground: style.getPropertyValue('--vscode-terminal-selectionBackground').trim() || 'rgba(255, 255, 255, 0.1)',
        black: style.getPropertyValue('--vscode-terminal-ansiBlack').trim() || '#000000',
        red: style.getPropertyValue('--vscode-terminal-ansiRed').trim() || '#cd3131',
        green: style.getPropertyValue('--vscode-terminal-ansiGreen').trim() || '#0dbc79',
        yellow: style.getPropertyValue('--vscode-terminal-ansiYellow').trim() || '#e5e510',
        blue: style.getPropertyValue('--vscode-terminal-ansiBlue').trim() || '#2472c8',
        magenta: style.getPropertyValue('--vscode-terminal-ansiMagenta').trim() || '#bc3fbc',
        cyan: style.getPropertyValue('--vscode-terminal-ansiCyan').trim() || '#11a8cd',
        white: style.getPropertyValue('--vscode-terminal-ansiWhite').trim() || '#e5e5e5',
        brightBlack: style.getPropertyValue('--vscode-terminal-ansiBrightBlack').trim() || '#666666',
        brightRed: style.getPropertyValue('--vscode-terminal-ansiBrightRed').trim() || '#f14c4c',
        brightGreen: style.getPropertyValue('--vscode-terminal-ansiBrightGreen').trim() || '#23d18b',
        brightYellow: style.getPropertyValue('--vscode-terminal-ansiBrightYellow').trim() || '#f5f543',
        brightBlue: style.getPropertyValue('--vscode-terminal-ansiBrightBlue').trim() || '#3b8eea',
        brightMagenta: style.getPropertyValue('--vscode-terminal-ansiBrightMagenta').trim() || '#d670d6',
        brightCyan: style.getPropertyValue('--vscode-terminal-ansiBrightCyan').trim() || '#29b8db',
        brightWhite: style.getPropertyValue('--vscode-terminal-ansiBrightWhite').trim() || '#e5e5e5'
    };
};

export const terminalManager = new TerminalManager();

let isInitialized = false;

export async function initTerminal(onTerminalCreate?: (shell?: string) => void): Promise<void> {
    if (isInitialized) return;
    isInitialized = true;

    listen("terminal-data", (event: any) => {
        const { term_id, data } = event.payload;
        terminalManager.handleData(term_id, data);
    });

    listen("terminal-create", (event: any) => {
        const { shell } = event.payload || {};
        if (onTerminalCreate) onTerminalCreate(shell);
    });

    window.addEventListener("resize", () => {
        for (const id of terminalManager.terminals.keys()) {
            terminalManager.resize(id);
        }
    });

    listen("theme-changed", () => {
        terminalManager.updateAllThemes();
    });

    // Expose a global way to spawn/manage terminals for the TitleBar
    (window as any).spawnTerminal = () => {
        if (onTerminalCreate) onTerminalCreate();
    };
}
