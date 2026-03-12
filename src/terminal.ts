import { invoke, listen } from './tauri_bridge.ts';
import { Terminal } from '@xterm/xterm';

export class TerminalManager {
    terminals: Map<string, any>;
    activeId: string | null;
    container: HTMLElement | null;
    tabsContainer: HTMLElement | null;
    idCounter: number;

    constructor() {
        this.terminals = new Map();
        this.activeId = null;
        this.container = null;
        this.tabsContainer = null;
        this.idCounter = 1;
        this.rebind();
    }

    rebind(): void {
        this.container = document.getElementById("terminal-container");
        this.tabsContainer = document.getElementById("terminal-tabs");
        
        const newBtn = document.getElementById("new-terminal");
        if (newBtn) {
            newBtn.onclick = () => this.createTerminal();
        }

        // If we have existing terminals, re-append their wrappers to the new container
        if (this.container && this.terminals.size > 0) {
            this.container.innerHTML = "";
            for (const [id, t] of this.terminals.entries()) {
                this.container.appendChild(t.wrapper);
                this.createTab(id);
            }
            if (this.activeId) {
                this.switchTo(this.activeId);
            }
        }
    }

    async createTerminal(): Promise<void> {
        const id = `term-${this.idCounter++}`;
        const wrapper = document.createElement("div");
        wrapper.className = "terminal-wrapper" + (this.activeId ? " hidden" : "");
        wrapper.id = `wrapper-${id}`;
        if (this.container) this.container.appendChild(wrapper);

        const TerminalKlass = (window as any).Terminal;
        const FitAddonKlass = (window as any).FitAddon ? (window as any).FitAddon.FitAddon : null;

        if (!TerminalKlass) {
            console.error("Terminal class not found.");
            return;
        }

        const term = new TerminalKlass({
            theme: { background: "#1e1e1e", foreground: "#cccccc" },
            fontSize: 12,
            fontFamily: 'var(--font-code)',
            cursorBlink: true,
        });

        const terminalData = { term, fitAddon: null, wrapper };
        if (FitAddonKlass) {
            const fitAddon = new FitAddonKlass();
            term.loadAddon(fitAddon);
            terminalData.fitAddon = fitAddon;
        }
        
        this.terminals.set(id, terminalData);
        term.open(wrapper);
        term.write("> Loading terminal backend...\r\n");

        if (terminalData.fitAddon) {
            setTimeout(() => terminalData.fitAddon.fit(), 100);
        }

        term.onData((data: string) => invoke("write_to_terminal", { termId: id, data }));
        term.onResize(({ cols, rows }: { cols: number, rows: number }) => invoke("resize_terminal", { termId: id, cols, rows }));

        this.createTab(id);

        try {
            await invoke("spawn_terminal", { termId: id });
        } catch (e) {
            term.write(`\r\n\x1b[31mError spawning terminal: ${e}\x1b[0m\r\n`);
        }
        this.switchTo(id);
    }

    createTab(id: string): void {
        if (!this.tabsContainer) return;
        
        // Remove existing tab if any
        const existing = Array.from(this.tabsContainer.querySelectorAll(".terminal-tab-btn")).find(el => (el as any).innerText.includes(`(${id.split('-')[1]})`));
        if (existing) existing.remove();

        const btn = document.createElement("button");
        btn.className = "terminal-tab-btn" + (this.activeId === id ? " active" : "");
        btn.innerText = `zsh (${id.split('-')[1]})`;
        btn.onclick = () => this.switchTo(id);
        
        const newTermBtn = document.getElementById("new-terminal");
        if (newTermBtn) {
            this.tabsContainer.insertBefore(btn, newTermBtn);
        } else {
            this.tabsContainer.appendChild(btn);
        }
    }

    switchTo(id: string): void {
        const t = this.terminals.get(id);
        if (!t) return;

        if (this.activeId) {
            const active = this.terminals.get(this.activeId);
            if (active) active.wrapper.classList.add("hidden");
        }
        this.activeId = id;
        t.wrapper.classList.remove("hidden");
        t.term.focus();
        if (t.fitAddon) t.fitAddon.fit();

        if (this.tabsContainer) {
            this.tabsContainer.querySelectorAll(".terminal-tab-btn").forEach((btn: any) => {
                btn.classList.toggle("active", btn.innerText.includes(`(${id.split('-')[1]})`));
            });
        }
    }

    handleData(termId: string, data: string): void {
        const t = this.terminals.get(termId);
        if (t) t.term.write(data);
    }
}

export let terminalManager: TerminalManager | null = null;

export async function initTerminal(): Promise<void> {
    terminalManager = new TerminalManager();

    listen("terminal-data", (event: any) => {
        const { term_id, data } = event.payload;
        if (terminalManager) {
            terminalManager.handleData(term_id, data);
        }
    });

    window.addEventListener("resize", () => {
        if (terminalManager) {
            for (const [id, t] of terminalManager.terminals.entries()) {
                if (t.fitAddon) {
                    try { t.fitAddon.fit(); } catch (e) { }
                }
            }
        }
    });

    (window as any).spawnTerminal = () => {
        if (terminalManager) {
            terminalManager.createTerminal();
        }
    };
    
    (window as any).rebindTerminal = () => {
        if (terminalManager) {
            terminalManager.rebind();
        }
    };
}
