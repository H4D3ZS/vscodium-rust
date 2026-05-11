/**
 * VSCode-Authentic Terminal Implementation
 * Full-featured terminal matching VSCode/VSCodium behavior
 */

import { invoke, listen } from './tauri_bridge.ts';
import { Terminal } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import { SearchAddon } from '@xterm/addon-search';
import { WebLinksAddon } from '@xterm/addon-web-links';
import { WebglAddon } from '@xterm/addon-webgl';
import { Unicode11Addon } from '@xterm/addon-unicode11';
import { CanvasAddon } from '@xterm/addon-canvas';
import '@xterm/xterm/css/xterm.css';

export interface ISearchOptions {
  regex?: boolean;
  wholeWord?: boolean;
  caseSensitive?: boolean;
  incremental?: boolean;
}

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

export interface TerminalProfile {
  id: string;
  name: string;
  path: string;
  args?: string[];
  icon: string;
  isDefault: boolean;
  platform?: 'win32' | 'linux' | 'darwin';
}

export interface TerminalInstance {
  id: string;
  name: string;
  term: Terminal;
  fitAddon: FitAddon;
  searchAddon: SearchAddon;
  webLinksAddon: WebLinksAddon;
  element: HTMLDivElement;
  shell: string;
  cwd?: string;
  pid?: number;
  isBusy: boolean;
  lastOutput?: string;
}

export interface TerminalGroup {
  id: string;
  instances: TerminalInstance[];
  activeInstanceId: string | null;
  layout: 'single' | 'split-horizontal' | 'split-vertical';
}

export interface TerminalTheme {
  background: string;
  foreground: string;
  cursor: string;
  cursorAccent: string;
  selectionBackground: string;
  black: string;
  red: string;
  green: string;
  yellow: string;
  blue: string;
  magenta: string;
  cyan: string;
  white: string;
  brightBlack: string;
  brightRed: string;
  brightGreen: string;
  brightYellow: string;
  brightBlue: string;
  brightMagenta: string;
  brightCyan: string;
  brightWhite: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// DEFAULT PROFILES (VSCode-like)
// ═══════════════════════════════════════════════════════════════════════════

const DEFAULT_PROFILES: TerminalProfile[] = [
  // Windows
  {
    id: 'powershell',
    name: 'PowerShell',
    path: 'pwsh.exe',
    args: ['-NoLogo'],
    icon: 'terminal-powershell',
    isDefault: true,
    platform: 'win32'
  },
  {
    id: 'cmd',
    name: 'Command Prompt',
    path: 'cmd.exe',
    args: [],
    icon: 'terminal-cmd',
    isDefault: false,
    platform: 'win32'
  },
  {
    id: 'git-bash',
    name: 'Git Bash',
    path: 'C:\\Program Files\\Git\\bin\\bash.exe',
    args: ['--login', '-i'],
    icon: 'terminal-bash',
    isDefault: false,
    platform: 'win32'
  },
  {
    id: 'wsl',
    name: 'WSL',
    path: 'wsl.exe',
    args: [],
    icon: 'terminal-linux',
    isDefault: false,
    platform: 'win32'
  },
  // Linux
  {
    id: 'bash',
    name: 'Bash',
    path: 'bash',
    args: ['-l'],
    icon: 'terminal-bash',
    isDefault: true,
    platform: 'linux'
  },
  {
    id: 'zsh',
    name: 'ZSH',
    path: 'zsh',
    args: ['-l'],
    icon: 'terminal-bash',
    isDefault: false,
    platform: 'linux'
  },
  // macOS
  {
    id: 'zsh-mac',
    name: 'ZSH',
    path: 'zsh',
    args: ['-l'],
    icon: 'terminal-bash',
    isDefault: true,
    platform: 'darwin'
  },
  {
    id: 'bash-mac',
    name: 'Bash',
    path: 'bash',
    args: ['-l'],
    icon: 'terminal-bash',
    isDefault: false,
    platform: 'darwin'
  }
];

// ═══════════════════════════════════════════════════════════════════════════
// VSCode THEME
// ═══════════════════════════════════════════════════════════════════════════

export const getVSCodeTheme = (): TerminalTheme => {
  const style = getComputedStyle(document.documentElement);
  
  return {
    background: style.getPropertyValue('--vscode-terminal-background').trim() || '#1e1e1e',
    foreground: style.getPropertyValue('--vscode-terminal-foreground').trim() || '#cccccc',
    cursor: style.getPropertyValue('--vscode-terminalCursor-foreground').trim() || '#aeafad',
    cursorAccent: style.getPropertyValue('--vscode-terminalCursor-background').trim() || '#1e1e1e',
    selectionBackground: style.getPropertyValue('--vscode-terminal-selectionBackground').trim() || 'rgba(255, 255, 255, 0.15)',
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

// ═══════════════════════════════════════════════════════════════════════════
// TERMINAL MANAGER
// ═══════════════════════════════════════════════════════════════════════════

export class TerminalManager {
  private terminals: Map<string, TerminalInstance> = new Map();
  private groups: Map<string, TerminalGroup> = new Map();
  private activeGroupId: string | null = null;
  private nextId = 1;
  private defaultProfileId: string = 'powershell';
  private linkProvider: any = null;
  private profilesReady: Promise<void>;

  constructor() {
    this.profilesReady = this.loadProfiles();
  }

  // ═══════════════════════════════════════════════════════════════════════
  // INITIALIZATION
  // ═══════════════════════════════════════════════════════════════════════

  private async loadProfiles() {
    // Load default profiles based on platform
    const platform = navigator.platform.toLowerCase().includes('win') ? 'win32' : 
                     navigator.platform.toLowerCase().includes('mac') ? 'darwin' : 'linux';
    
    const platformProfiles = DEFAULT_PROFILES.filter(p => !p.platform || p.platform === platform);
    
    // Try to detect available shells
    try {
      const shells = await invoke<string[]>('get_available_shells');
      // Update profiles based on detected shells
      platformProfiles.forEach(profile => {
        if (shells.some(s => s.includes(profile.path))) {
          profile.isDefault = true;
        }
      });
    } catch (e) {
      console.warn('Could not detect available shells, using defaults');
    }

    this.defaultProfileId = platformProfiles.find(p => p.isDefault)?.id || platformProfiles[0]?.id || 'powershell';
  }

  /**
   * Return the list of shell executables this host knows about. Used by
   * `TerminalView` to populate the shell picker. Falls back to the static
   * platform profiles when the Tauri backend doesn't expose detection.
   */
  async getAvailableShells(): Promise<string[]> {
    await this.profilesReady;
    try {
      const shells = await invoke<string[]>('get_available_shells');
      if (Array.isArray(shells) && shells.length > 0) {
        return shells;
      }
    } catch {
      // Backend command unavailable — fall back to defaults.
    }
    const platform = navigator.platform.toLowerCase().includes('win')
      ? 'win32'
      : navigator.platform.toLowerCase().includes('mac')
        ? 'darwin'
        : 'linux';
    return DEFAULT_PROFILES
      .filter(p => !p.platform || p.platform === platform)
      .map(p => p.path);
  }

  // ═══════════════════════════════════════════════════════════════════════
  // TERMINAL CREATION
  // ═══════════════════════════════════════════════════════════════════════

  async createTerminal(
    profileId?: string,
    groupId?: string,
    cwd?: string
  ): Promise<string> {
    await this.profilesReady;
    const id = `term-${this.nextId++}`;
    const profile = DEFAULT_PROFILES.find(p => p.id === (profileId || this.defaultProfileId)) 
                  || DEFAULT_PROFILES[0];

    // Create persistent element
    const element = document.createElement('div');
    element.className = 'terminal-instance-element';
    element.style.width = '100%';
    element.style.height = '100%';

    // Create terminal with VSCode-like settings
    const term = new Terminal({
      theme: getVSCodeTheme(),
      fontSize: 14,
      fontFamily: 'Consolas, "Courier New", monospace',
      fontWeight: 'normal',
      fontWeightBold: 'bold',
      cursorBlink: true,
      cursorStyle: 'block',
      cursorWidth: 2,
      allowProposedApi: true,
      scrollback: 5000,
      tabStopWidth: 8,
      rightClickSelectsWord: true,
      fastScrollSensitivity: 5,
      scrollSensitivity: 3,
      macOptionIsMeta: true,
      macOptionClickForcesSelection: false,
      minimumContrastRatio: 1,
      drawBoldTextInBrightColors: true,
      convertEol: false,
      allowTransparency: false,
      disableStdin: false,
      screenReaderMode: false
    });

    // Load addons
    const fitAddon = new FitAddon();
    const searchAddon = new SearchAddon();
    const webLinksAddon = new WebLinksAddon();
    const unicodeAddon = new Unicode11Addon();
    
    term.loadAddon(fitAddon);
    term.loadAddon(searchAddon);
    term.loadAddon(webLinksAddon);
    term.loadAddon(unicodeAddon);
    
    // Try WebGL for hardware acceleration
    try {
      const webglAddon = new WebglAddon();
      webglAddon.onContextLoss(() => {
        webglAddon.dispose();
        // Fallback to canvas
        try {
          term.loadAddon(new CanvasAddon());
        } catch (e) {}
      });
      term.loadAddon(webglAddon);
    } catch (e) {
      // Fallback to canvas
      try {
        term.loadAddon(new CanvasAddon());
      } catch (e) {}
    }

    term.unicode.activeVersion = '11';

    // Open terminal
    term.open(element);

    // Create instance
    const instance: TerminalInstance = {
      id,
      name: `${profile.name} ${this.nextId - 1}`,
      term,
      fitAddon,
      searchAddon,
      webLinksAddon,
      element,
      shell: profile.path,
      cwd,
      isBusy: false
    };

    this.terminals.set(id, instance);

    // Create group if needed
    const actualGroupId = groupId || this.createGroupId();
    if (!this.groups.has(actualGroupId)) {
      this.groups.set(actualGroupId, {
        id: actualGroupId,
        instances: [],
        activeInstanceId: null,
        layout: 'single'
      });
    }

    const group = this.groups.get(actualGroupId)!;
    group.instances.push(instance);
    group.activeInstanceId = id;
    this.activeGroupId = actualGroupId;

    // Setup terminal events
    this.setupTerminalEvents(instance);

    // Spawn shell
    try {
      const result = await invoke<{ id?: string; status?: string; pid?: number }>('spawn_terminal', { 
        id, 
        shell: profile.path,
        args: profile.args || [],
        cwd 
      });
      
      if (result && typeof result === 'object' && 'pid' in result) {
        instance.pid = (result as any).pid;
      }
      
      // Initial fit
      setTimeout(() => {
        fitAddon.fit();
        term.focus();
      }, 50);
    } catch (e: any) {
      term.writeln(`\r\n\x1b[31mError spawning terminal: ${e.message || e}\x1b[0m\r\n`);
      term.writeln(`\x1b[33mProfile: ${profile.name} (${profile.path})\x1b[0m\r\n`);
    }

    return id;
  }

  private setupTerminalEvents(instance: TerminalInstance) {
    const { term, id } = instance;

    // Data event - send to backend
    term.onData((data: string) => {
      invoke('write_to_terminal', { id, data });
    });

    // Resize event
    term.onResize(({ cols, rows }) => {
      invoke('resize_terminal', { id, cols, rows });
    });

    // Title event
    term.onTitleChange((title: string) => {
      if (title && title !== instance.name) {
        instance.name = title;
        this.notifyTitleChange(instance);
      }
    });

    // Bell event
    term.onBell(() => {
      // Visual bell effect
      instance.element.style.boxShadow = 'inset 0 0 20px rgba(255, 255, 255, 0.3)';
      setTimeout(() => {
        instance.element.style.boxShadow = '';
      }, 100);
    });

    // Selection event
    term.onSelectionChange(() => {
      // Could trigger context menu update
    });

    // Right-click for context menu
    term.onContextMenu((event: MouseEvent) => {
      event.preventDefault();
      this.showContextMenu(instance, event);
    });
  }

  // ═══════════════════════════════════════════════════════════════════════
  // TERMINAL ATTACHMENT
  // ═══════════════════════════════════════════════════════════════════════

  attach(id: string, container: HTMLElement) {
    const instance = this.terminals.get(id);
    if (instance && container) {
      container.appendChild(instance.element);
      setTimeout(() => {
        try {
          instance.fitAddon.fit();
          const { cols, rows } = instance.term;
          invoke('resize_terminal', { id, cols, rows });
        } catch (e) {}
      }, 50);
    }
  }

  detach(id: string) {
    const instance = this.terminals.get(id);
    if (instance && instance.element.parentNode) {
      instance.element.parentNode.removeChild(instance.element);
    }
  }

  // ═══════════════════════════════════════════════════════════════════════
  // TERMINAL OPERATIONS
  // ═══════════════════════════════════════════════════════════════════════

  async closeTerminal(id: string): Promise<void> {
    const instance = this.terminals.get(id);
    if (instance) {
      try {
        await invoke('close_terminal', { id });
      } catch (e) {}
      
      instance.term.dispose();
      if (instance.element.parentNode) {
        instance.element.parentNode.removeChild(instance.element);
      }
      
      this.terminals.delete(id);
      
      // Remove from group
      for (const group of this.groups.values()) {
        const idx = group.instances.findIndex(i => i.id === id);
        if (idx >= 0) {
          group.instances.splice(idx, 1);
          if (group.activeInstanceId === id) {
            group.activeInstanceId = group.instances[0]?.id || null;
          }
        }
        if (group.instances.length === 0) {
          this.groups.delete(group.id);
        }
      }
    }
  }

  async renameTerminal(id: string, name: string): Promise<void> {
    const instance = this.terminals.get(id);
    if (instance) {
      instance.name = name;
      this.notifyTitleChange(instance);
    }
  }

  async splitTerminal(id: string, direction: 'horizontal' | 'vertical'): Promise<string> {
    const instance = this.terminals.get(id);
    if (!instance) throw new Error('Terminal not found');

    // Find group
    let group: TerminalGroup | undefined;
    for (const g of this.groups.values()) {
      if (g.instances.some(i => i.id === id)) {
        group = g;
        break;
      }
    }

    if (!group) throw new Error('Group not found');

    // Create new terminal with same profile
    const newId = await this.createTerminal(undefined, group.id, instance.cwd);
    
    // Update layout
    group.layout = direction === 'horizontal' ? 'split-horizontal' : 'split-vertical';
    
    return newId;
  }

  setActiveTerminal(groupId: string, instanceId: string): void {
    const group = this.groups.get(groupId);
    if (group) {
      group.activeInstanceId = instanceId;
      this.activeGroupId = groupId;
      
      const instance = this.terminals.get(instanceId);
      if (instance) {
        instance.term.focus();
      }
    }
  }

  // ═══════════════════════════════════════════════════════════════════════
  // SEARCH
  // ═══════════════════════════════════════════════════════════════════════

  findNext(instanceId: string, term: string, options?: ISearchOptions): boolean {
    const instance = this.terminals.get(instanceId);
    if (!instance) return false;
    return instance.searchAddon.findNext(term, options);
  }

  findPrevious(instanceId: string, term: string, options?: ISearchOptions): boolean {
    const instance = this.terminals.get(instanceId);
    if (!instance) return false;
    return instance.searchAddon.findPrevious(term, options);
  }

  // ═══════════════════════════════════════════════════════════════════════
  // CONTEXT MENU
  // ═══════════════════════════════════════════════════════════════════════

  private showContextMenu(instance: TerminalInstance, event: MouseEvent) {
    const menu = document.createElement('div');
    menu.className = 'terminal-context-menu';
    menu.style.cssText = `
      position: fixed;
      left: ${event.clientX}px;
      top: ${event.clientY}px;
      background: var(--vscode-menu-background);
      border: 1px solid var(--vscode-menu-border);
      border-radius: 4px;
      padding: 4px 0;
      min-width: 180px;
      z-index: 10000;
      box-shadow: 0 4px 12px rgba(0,0,0,0.3);
    `;

    const menuItems = [
      { label: 'New Terminal', icon: 'add', action: () => this.createTerminal() },
      { label: 'Split Terminal', icon: 'split-horizontal', action: () => this.splitTerminal(instance.id, 'horizontal') },
      { type: 'separator' },
      { label: 'Copy', icon: 'copy', action: () => this.copySelection(instance) },
      { label: 'Paste', icon: 'paste', action: () => this.paste(instance) },
      { label: 'Select All', icon: 'selection', action: () => this.selectAll(instance) },
      { type: 'separator' },
      { label: 'Clear', icon: 'clear-all', action: () => instance.term.clear() },
      { label: 'Kill Terminal', icon: 'trash', action: () => this.closeTerminal(instance.id) }
    ];

    menuItems.forEach(item => {
      if ('type' in item && item.type === 'separator') {
        const separator = document.createElement('div');
        separator.style.cssText = 'height: 1px; background: var(--vscode-menu-separatorBackground); margin: 4px 0;';
        menu.appendChild(separator);
      } else {
        const menuItem = document.createElement('div');
        menuItem.style.cssText = `
          padding: 4px 12px;
          cursor: pointer;
          display: flex;
          align-items: center;
          gap: 8px;
          font-size: 12px;
          color: var(--vscode-menu-foreground);
        `;
        menuItem.onmouseover = () => menuItem.style.background = 'var(--vscode-menu-selectionBackground)';
        menuItem.onmouseout = () => menuItem.style.background = '';
        menuItem.onclick = () => {
          (item as any).action();
          menu.remove();
        };
        
        const icon = document.createElement('i');
        icon.className = `codicon codicon-${(item as any).icon}`;
        icon.style.fontSize = '14px';
        
        const label = document.createElement('span');
        label.textContent = item.label;
        
        menuItem.appendChild(icon);
        menuItem.appendChild(label);
        menu.appendChild(menuItem);
      }
    });

    document.body.appendChild(menu);

    // Close on click outside
    const closeMenu = () => {
      menu.remove();
      document.removeEventListener('click', closeMenu);
    };
    setTimeout(() => document.addEventListener('click', closeMenu), 100);
  }

  copySelection(instance: TerminalInstance) {
    const selectedText = instance.term.getSelection();
    if (selectedText) {
      navigator.clipboard.writeText(selectedText);
    }
  }

  async paste(instance: TerminalInstance) {
    try {
      const text = await navigator.clipboard.readText();
      instance.term.paste(text);
    } catch (e) {}
  }

  selectAll(instance: TerminalInstance) {
    instance.term.selectAll();
  }

  // ═══════════════════════════════════════════════════════════════════════
  // UTILITIES
  // ═══════════════════════════════════════════════════════════════════════

  private createGroupId(): string {
    return `group-${Date.now()}`;
  }

  private notifyTitleChange(instance: TerminalInstance) {
    // Could emit event for UI update
    console.log('Terminal title changed:', instance.name);
  }

  getTerminal(id: string): TerminalInstance | undefined {
    return this.terminals.get(id);
  }

  getGroup(groupId: string): TerminalGroup | undefined {
    return this.groups.get(groupId);
  }

  getActiveGroup(): TerminalGroup | undefined {
    return this.activeGroupId ? this.groups.get(this.activeGroupId) : undefined;
  }

  getAllTerminals(): TerminalInstance[] {
    return Array.from(this.terminals.values());
  }

  updateAllThemes() {
    const theme = getVSCodeTheme();
    for (const instance of this.terminals.values()) {
      instance.term.options.theme = theme;
    }
  }
}

// ═══════════════════════════════════════════════════════════════════════════
// KEYBOARD SHORTCUTS (VSCode-like)
// ═══════════════════════════════════════════════════════════════════════════

export const registerTerminalShortcuts = (manager: TerminalManager) => {
  window.addEventListener('keydown', (e: KeyboardEvent) => {
    const group = manager.getActiveGroup();
    if (!group || !group.activeInstanceId) return;
    
    const instance = manager.getTerminal(group.activeInstanceId);
    if (!instance) return;

    // Ctrl+` - Toggle terminal
    if (e.key === '`' && (e.ctrlKey || e.metaKey)) {
      e.preventDefault();
      // Toggle terminal panel visibility
      return;
    }

    // Ctrl+Shift+` - New terminal
    if (e.key === '`' && e.ctrlKey && e.shiftKey) {
      e.preventDefault();
      manager.createTerminal();
      return;
    }

    // Ctrl+Shift+5 - Split terminal
    if (e.key === '5' && e.ctrlKey && e.shiftKey) {
      e.preventDefault();
      manager.splitTerminal(instance.id, 'horizontal');
      return;
    }

    // Ctrl+Shift+T - New terminal (alternative)
    if (e.key === 't' && e.ctrlKey && e.shiftKey) {
      e.preventDefault();
      manager.createTerminal();
      return;
    }

    // Ctrl+Shift+W - Close terminal
    if (e.key === 'w' && e.ctrlKey && e.shiftKey) {
      e.preventDefault();
      manager.closeTerminal(instance.id);
      return;
    }

    // Ctrl+Shift+P - Find widget
    if (e.key === 'p' && e.ctrlKey && e.shiftKey && document.activeElement?.closest('.terminal')) {
      e.preventDefault();
      // Show find widget
      return;
    }

    // Ctrl+F - Find in terminal
    if (e.key === 'f' && e.ctrlKey && document.activeElement?.closest('.terminal')) {
      e.preventDefault();
      // Show find widget
      return;
    }

    // Ctrl+C - Copy (when text selected)
    if (e.key === 'c' && e.ctrlKey && instance.term.hasSelection()) {
      e.preventDefault();
      manager.copySelection(instance);
      return;
    }

    // Ctrl+V - Paste
    if (e.key === 'v' && e.ctrlKey && document.activeElement?.closest('.terminal')) {
      e.preventDefault();
      manager.paste(instance);
      return;
    }

    // Ctrl+A - Select all
    if (e.key === 'a' && e.ctrlKey && document.activeElement?.closest('.terminal')) {
      e.preventDefault();
      manager.selectAll(instance);
      return;
    }

    // Page Up/Down
    if (e.key === 'PageUp' && e.shiftKey) {
      e.preventDefault();
      instance.term.scrollPages(-1);
      return;
    }
    if (e.key === 'PageDown' && e.shiftKey) {
      e.preventDefault();
      instance.term.scrollPages(1);
      return;
    }
  });
};

// ═══════════════════════════════════════════════════════════════════════════
// EXPORT SINGLETON
// ═══════════════════════════════════════════════════════════════════════════

export const terminalManager = new TerminalManager();

// ═══════════════════════════════════════════════════════════════════════════
// LEGACY INIT FUNCTION (for App.tsx compatibility)
// ═══════════════════════════════════════════════════════════════════════════

export const initTerminal = async (addTerminalGroup: (shell: string) => void) => {
  // Create initial terminal with default profile
  try {
    const id = await terminalManager.createTerminal();
    const instance = terminalManager.getTerminal(id);
    if (instance) {
      addTerminalGroup(instance.shell);
    }
  } catch (e: any) {
    const msg = e?.message || e?.toString?.() || String(e) || 'unknown error';
    console.warn('[terminal] Failed to create initial terminal:', msg, e);
  }
};
