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
import { CommandBlockTracker } from './terminalBlocks.ts';
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
  /** Warp-style command-block tracker (OSC 133 driven). PTY terminals only. */
  blocks?: CommandBlockTracker;
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
    // Windows PowerShell 5.1 — ALWAYS present on Win10/11. Must be the default:
    // `pwsh.exe` (PowerShell 7) is NOT installed by default, and on Windows
    // ConPTY's spawn returns Ok for a missing exe (deferred process creation),
    // so the PTY silently produces no output → blank terminal. Use the exe that
    // is guaranteed to exist.
    id: 'powershell',
    name: 'PowerShell',
    path: 'powershell.exe',
    args: ['-NoLogo'],
    icon: 'terminal-powershell',
    isDefault: true,
    platform: 'win32'
  },
  {
    // PowerShell 7+ (only if the user installed it). Not default.
    id: 'pwsh',
    name: 'PowerShell 7',
    path: 'pwsh.exe',
    args: ['-NoLogo'],
    icon: 'terminal-powershell',
    isDefault: false,
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

/**
 * Warp/cmder-inspired terminal palette (Tokyo-Night-ish): deep indigo
 * background, soft-but-vivid ANSI, a bright blue bar cursor. Gives the panel a
 * modern native feel distinct from the editor's VSCode theme. Original colors —
 * not copied from Warp or cmder.
 */
export const getCmderWarpTheme = (): TerminalTheme => ({
  background: '#171922',
  foreground: '#d5d8e0',
  cursor: '#7aa2f7',
  cursorAccent: '#171922',
  selectionBackground: 'rgba(122,162,247,0.25)',
  black: '#15161e',
  red: '#f7768e',
  green: '#9ece6a',
  yellow: '#e0af68',
  blue: '#7aa2f7',
  magenta: '#bb9af7',
  cyan: '#7dcfff',
  white: '#a9b1d6',
  brightBlack: '#414868',
  brightRed: '#ff899d',
  brightGreen: '#b9f27c',
  brightYellow: '#ff9e64',
  brightBlue: '#8db0ff',
  brightMagenta: '#d7b4f3',
  brightCyan: '#a4daff',
  brightWhite: '#e6e9f0',
});

function currentTerminalPlatform(): 'win32' | 'linux' | 'darwin' {
  const p = navigator.platform.toLowerCase();
  if (p.includes('win')) return 'win32';
  if (p.includes('mac')) return 'darwin';
  return 'linux';
}

/** Resolve profile from profile id, full shell path, or executable name. */
function resolveTerminalProfile(shellOrProfileId?: string): TerminalProfile {
  const platform = currentTerminalPlatform();
  const platformProfiles = DEFAULT_PROFILES.filter(p => !p.platform || p.platform === platform);
  const fallback = platformProfiles.find(p => p.isDefault) || platformProfiles[0] || DEFAULT_PROFILES[0];

  if (!shellOrProfileId || !String(shellOrProfileId).trim()) {
    return fallback;
  }

  const key = String(shellOrProfileId).trim();
  const lower = key.toLowerCase();

  const byId = platformProfiles.find(p => p.id === key);
  if (byId) return byId;

  const byExactPath = platformProfiles.find(p => p.path.toLowerCase() === lower);
  if (byExactPath) return byExactPath;

  const base = key.split(/[/\\]/).pop() || key;
  const baseLower = base.toLowerCase();

  const byExe = platformProfiles.find(p => {
    const tail = p.path.split(/[/\\]/).pop() || p.path;
    return tail.toLowerCase() === baseLower;
  });
  if (byExe) return byExe;

  return {
    id: 'custom-shell',
    name: base,
    path: key,
    args: [],
    icon: 'terminal-bash',
    isDefault: false,
    platform
  };
}

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
  // Global `terminal-data` listener — set up exactly once and routes payloads
  // to whichever terminal instance owns the term_id. Without this listener
  // the xterm display stayed blank (PTY output was emitted into the void).
  private dataListenerInstalled = false;
  // Per-terminal write buffers used until the underlying xterm has been
  // attached and is ready to receive `term.write`.
  private pendingWrites: Map<string, string[]> = new Map();
  // Ids that have received at least one byte of PTY output. Used by the
  // post-spawn watchdog to tell "shell is alive and talking" apart from
  // "spawned but silent" (the chronic blank-terminal failure mode).
  private firstDataIds: Set<string> = new Set();
  // Poll timers per terminal. PTY output is pulled via `terminal_take_pending`
  // (invoke) because the `terminal-data` event stream doesn't reach the webview.
  private pollTimers: Map<string, any> = new Map();
  // Tracks which terminals have had `term.open()` called. We defer open() until
  // the element is attached to a sized container (in `attach()`) so xterm's
  // renderer doesn't initialize blank on a detached 0×0 element.
  private openedIds: Set<string> = new Set();

  constructor() {
    this.profilesReady = this.loadProfiles();
    void this.ensureDataListener();
  }

  /** Subscribe once to the Tauri `terminal-data` event stream. */
  private async ensureDataListener(): Promise<void> {
    if (this.dataListenerInstalled) return;
    this.dataListenerInstalled = true;
    try {
      await listen('terminal-data', (_event: any) => {
        // Intentionally a no-op. PTY output is rendered via polling
        // (`terminal_take_pending`) because this event stream does not reliably
        // reach the webview. The backend still emits it as a best-effort
        // secondary, but writing here too would double-render the output on
        // hosts where the event DOES fire. Kept subscribed for diagnostics.
      });
    } catch (err) {
      console.warn('[terminal] Failed to subscribe to terminal-data:', err);
      this.dataListenerInstalled = false;
    }
  }

  /** Begin polling the backend pending buffer for this terminal's PTY output. */
  private startPolling(id: string): void {
    if (this.pollTimers.has(id)) return;
    const tick = async () => {
      if (!this.terminals.has(id)) { this.stopPolling(id); return; }
      try {
        const chunk = await invoke<string>('terminal_take_pending', { id });
        if (chunk) {
          const inst = this.terminals.get(id);
          if (inst) {
            this.firstDataIds.add(id);
            inst.term.write(chunk);
            inst.lastOutput = chunk;
          }
        }
      } catch {
        /* backend not ready / terminal gone — keep ticking */
      }
    };
    const timer = setInterval(tick, 50);
    this.pollTimers.set(id, timer);
    void tick(); // immediate first pull (don't wait a full interval)
  }

  private stopPolling(id: string): void {
    const t = this.pollTimers.get(id);
    if (t) { clearInterval(t); this.pollTimers.delete(id); }
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
    cwd?: string,
    /** Must match React/store `instanceId` so `attach()` can find this terminal. */
    explicitId?: string
  ): Promise<string> {
    await this.profilesReady;
    const id =
      explicitId && explicitId.trim().length > 0
        ? explicitId.trim()
        : `term-${this.nextId++}`;
    const profile = resolveTerminalProfile(profileId || this.defaultProfileId);

    // Create persistent element
    const element = document.createElement('div');
    element.className = 'terminal-instance-element';
    element.style.width = '100%';
    element.style.height = '100%';

    // Create terminal with VSCode-like settings
    const term = new Terminal({
      // Match the IDE's global VSCode theme (reads --vscode-terminal-* vars).
      theme: getVSCodeTheme(),
      fontSize: 13,
      fontFamily: '"Cascadia Mono", "Cascadia Code", "JetBrains Mono", Consolas, "Courier New", monospace',
      fontWeight: 'normal',
      fontWeightBold: 600,
      lineHeight: 1.3,
      letterSpacing: 0,
      cursorBlink: true,
      cursorStyle: 'bar',
      cursorWidth: 2,
      allowProposedApi: true,
      scrollback: 2000,
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
    
    // Use xterm's default DOM renderer (no WebGL/Canvas). The GPU renderers
    // initialize BLANK when the terminal is opened on a detached / 0-size element
    // and don't reliably recover after attach — that was the chronic "empty
    // terminal" bug. The DOM renderer always paints and reflows naturally; for a
    // shell its perf is more than enough.

    term.unicode.activeVersion = '11';

    // NOTE: term.open() is deferred to attach() — opening on this still-detached
    // element renders blank (WebGL/Canvas init at 0×0). Writes before open() are
    // buffered by xterm and flushed on open.

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

    // Warp-style command blocks: OSC 133 shell integration → gutter marks +
    // hover toolbar + navigation. Parses sequences arriving through the normal
    // write path; the PS integration script is injected after spawn below.
    try {
      const tracker = new CommandBlockTracker(term, (data: string) => {
        if (this.activityIds.has(id)) return; // virtual terminal has no PTY
        void invoke('terminal_send_data', { id, data }).catch(() => {});
      });
      tracker.attach();
      instance.blocks = tracker;
    } catch { /* decorations are best-effort */ }

    // Visible boot line. xterm buffers this until open() (in attach). If you SEE it,
    // the renderer works and any missing shell prompt is a PTY issue; if the pane is
    // totally blank, the element never got opened/attached.
    try { term.writeln('\x1b[90m✓ terminal ready — ' + profile.name + '\x1b[0m'); } catch { /* */ }

    // If data arrived before this instance was registered, flush it now.
    const pending = this.pendingWrites.get(id);
    if (pending && pending.length) {
      for (const chunk of pending) term.write(chunk);
      this.pendingWrites.delete(id);
    }

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

    // Spawn shell (Rust `spawn_terminal` only accepts `id` + optional `shell`)
    try {
      const result = await invoke<{ id?: string; status?: string; pid?: number }>('spawn_terminal', {
        id,
        shell: profile.path
      });

      if (result && typeof result === 'object' && 'pid' in result) {
        instance.pid = (result as any).pid;
      }

      // ── PRIMARY transport: poll the backend pending buffer ─────────────────
      // The `terminal-data` event stream does not reach the webview, so we pull
      // PTY output via invoke (proven to work). ~35ms feels instant for a shell
      // and returns an empty string when idle (cheap). Stopped in closeTerminal.
      this.startPolling(id);

      // ── Shell integration (Warp-style blocks) ──────────────────────────────
      // The OSC 133 prompt hooks are injected at shell STARTUP by the backend
      // (`spawn_terminal` launches PowerShell with `-Command ". <temp>.ps1"`), so
      // there's nothing to type here. Injecting over stdin made ConPTY echo the
      // whole multi-line script as visible garbage — that's why it moved to spawn.

      // ── Post-spawn watchdog ────────────────────────────────────────────────
      // spawn_terminal returned without throwing, so the backend accepted the
      // request. If NO PTY bytes arrive shortly after, the shell either died on
      // launch or the event stream is broken — surface exactly which, and nudge
      // the PTY with a newline (wakes a stalled ConPTY prompt) instead of
      // leaving the user staring at a blank pane.
      setTimeout(async () => {
        if (this.firstDataIds.has(id)) return; // healthy — shell is talking
        try {
          await invoke('terminal_send_data', { id, data: '\r' });
        } catch { /* writer gone */ }
        setTimeout(async () => {
          if (this.firstDataIds.has(id)) return;
          let statusStr = 'unknown';
          try {
            const status = await invoke<any>('terminal_get_status', { id });
            statusStr = JSON.stringify(status);
          } catch (e: any) {
            statusStr = `status check failed: ${e?.message || e}`;
          }
          if (this.firstDataIds.has(id)) return;
          term.writeln(`\r\n\x1b[33m[diagnostic] no shell output after ~2s.\x1b[0m`);
          term.writeln(`\x1b[33m[diagnostic] shell=${instance.shell}  backend-status=${statusStr}\x1b[0m`);
          term.writeln(`\x1b[90mIf status shows active:false the shell exited on launch (bad exe/cwd).`);
          term.writeln(`If active:true but still no prompt, the terminal-data event stream isn't reaching the UI.\x1b[0m`);
        }, 700);
      }, 1500);

      const fitNow = () => {
        try {
          fitAddon.fit();
          const { cols, rows } = term;
          if (cols > 0 && rows > 0) {
            void invoke('resize_terminal', { id, cols, rows }).catch(() => {});
          }
        } catch {
          /* container may still be 0×0 */
        }
      };
      setTimeout(fitNow, 0);
      setTimeout(fitNow, 50);
      setTimeout(fitNow, 200);
      term.focus();
    } catch (e: any) {
      term.writeln(`\r\n\x1b[31mError spawning terminal: ${e.message || e}\x1b[0m\r\n`);
      term.writeln(`\x1b[33mProfile: ${profile.name} (${profile.path})\x1b[0m\r\n`);
    }

    return id;
  }

  private setupTerminalEvents(instance: TerminalInstance) {
    const { term, id, element } = instance;

    // Keystrokes — pipe to the backend PTY via terminal_send_data.
    // The previous `write_to_terminal` invocation silently no-op'd because
    // no such Tauri command was registered. Activity terminals have no PTY,
    // so swallow input there.
    term.onData((data: string) => {
      if (this.activityIds.has(id)) return;
      invoke('terminal_send_data', { id, data }).catch((e) => {
        console.warn('[terminal] terminal_send_data failed:', e);
      });
    });

    term.onResize(({ cols, rows }) => {
      if (this.activityIds.has(id)) return;
      invoke('resize_terminal', { id, cols, rows }).catch(() => {});
    });

    term.onTitleChange((title: string) => {
      if (title && title !== instance.name) {
        instance.name = title;
        this.notifyTitleChange(instance);
      }
    });

    term.onBell(() => {
      instance.element.style.boxShadow = 'inset 0 0 20px rgba(255, 255, 255, 0.3)';
      setTimeout(() => {
        instance.element.style.boxShadow = '';
      }, 100);
    });

    // Right-click for context menu — `Terminal#onContextMenu` does not exist
    // in xterm.js, so we listen on the DOM element instead.
    element.addEventListener('contextmenu', (event: MouseEvent) => {
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
      // Open xterm NOW that the element is in a sized container (deferred from
      // createTerminal). xterm buffers any pre-open writes and renders them here.
      if (!this.openedIds.has(id)) {
        try {
          instance.term.open(instance.element);
          this.openedIds.add(id);
        } catch { /* element not ready; a later attach will retry */ }
      }
      const fitNow = () => {
        try {
          instance.fitAddon.fit();
          const { cols, rows } = instance.term;
          if (cols > 0 && rows > 0) {
            void invoke('resize_terminal', { id, cols, rows }).catch(() => {});
          }
        } catch {
          /* layout not ready */
        }
      };
      setTimeout(fitNow, 0);
      requestAnimationFrame(() => {
        fitNow();
        requestAnimationFrame(fitNow);
      });
      setTimeout(fitNow, 50);
      setTimeout(fitNow, 200);
    }
  }

  detach(id: string) {
    const instance = this.terminals.get(id);
    if (instance && instance.element.parentNode) {
      instance.element.parentNode.removeChild(instance.element);
    }
  }

  /**
   * Refit & resync a terminal. `TerminalInstance` calls this on activation
   * and on ResizeObserver ticks; the missing method was a silent no-op
   * before and left xterm with stale cols/rows after layout changes.
   */
  resize(id: string): void {
    const instance = this.terminals.get(id);
    if (!instance) return;
    try {
      instance.fitAddon.fit();
      const { cols, rows } = instance.term;
      // Activity terminals have no backing PTY, so don't ping the Rust side.
      if (!this.activityIds.has(id)) {
        invoke('resize_terminal', { id, cols, rows }).catch(() => {});
      }
    } catch {
      /* xterm not yet measured */
    }
  }

  // ═══════════════════════════════════════════════════════════════════════
  // AIRI ACTIVITY TERMINAL
  //
  // A virtual terminal (no PTY) that mirrors every AI tool invocation in
  // real time. Subscribed to the same `ai-tool-call` / `ai-tool-result` /
  // `ai-action` events the right-sidebar agent panel uses, so the user can
  // *watch* AIRI work instead of only seeing a finished card in the UI.
  // ═══════════════════════════════════════════════════════════════════════

  private activityIds: Set<string> = new Set();
  private activityUnsubs: Map<string, Array<() => void>> = new Map();
  /** Tool calls keyed by call_id, so the result can render on the same block. */
  private activityCalls: Map<string, Map<string, { name: string; argPreview: string }>> = new Map();

  /**
   * Create the activity terminal if it does not exist, otherwise return the
   * existing id. Idempotent — safe to wire to a toolbar button.
   */
  async createAiriActivityTerminal(explicitId?: string): Promise<string> {
    // Reuse the existing activity terminal so the button doesn't spawn many.
    if (!explicitId) {
      for (const id of this.activityIds) {
        if (this.terminals.has(id)) return id;
      }
    }
    const id = explicitId && explicitId.trim().length > 0
      ? explicitId.trim()
      : `airi-activity-${Date.now()}`;

    const element = document.createElement('div');
    element.className = 'terminal-instance-element';
    element.style.width = '100%';
    element.style.height = '100%';

    const term = new Terminal({
      theme: getVSCodeTheme(),
      fontSize: 13,
      fontFamily: 'Consolas, "Courier New", monospace',
      cursorBlink: false,
      cursorStyle: 'underline',
      allowProposedApi: true,
      scrollback: 2000,
      disableStdin: true,
      convertEol: true,
      drawBoldTextInBrightColors: true,
    });

    const fitAddon = new FitAddon();
    const searchAddon = new SearchAddon();
    const webLinksAddon = new WebLinksAddon();
    term.loadAddon(fitAddon);
    term.loadAddon(searchAddon);
    term.loadAddon(webLinksAddon);
    try { term.loadAddon(new WebglAddon()); } catch { try { term.loadAddon(new CanvasAddon()); } catch { /* */ } }
    term.open(element);

    const instance: TerminalInstance = {
      id,
      name: 'AIRI',
      term,
      fitAddon,
      searchAddon,
      webLinksAddon,
      element,
      shell: '<virtual:airi>',
      isBusy: false,
    };
    this.terminals.set(id, instance);
    this.activityIds.add(id);
    this.activityCalls.set(id, new Map());

    // Banner
    term.writeln('\x1b[1;36m╔════════════════════════════════════════════════════════════╗\x1b[0m');
    term.writeln('\x1b[1;36m║  AIRI LIVE ACTIVITY · tool calls + actions stream below   ║\x1b[0m');
    term.writeln('\x1b[1;36m╚════════════════════════════════════════════════════════════╝\x1b[0m');
    term.writeln('\x1b[2mNo PTY attached. Read-only feed.\x1b[0m');
    term.writeln('');

    // Flush any pending writes (none expected for activity, but cheap).
    const pending = this.pendingWrites.get(id);
    if (pending && pending.length) {
      for (const chunk of pending) term.write(chunk);
      this.pendingWrites.delete(id);
    }

    // Subscribe to backend events. We hold the unlistens so we can tear
    // them down when the terminal is closed.
    const unsubs: Array<() => void> = [];

    const ts = () => {
      const d = new Date();
      const hh = String(d.getHours()).padStart(2, '0');
      const mm = String(d.getMinutes()).padStart(2, '0');
      const ss = String(d.getSeconds()).padStart(2, '0');
      return `\x1b[2m${hh}:${mm}:${ss}\x1b[0m`;
    };

    const truncate = (s: string, n: number) => (s.length > n ? s.slice(0, n - 1) + '…' : s);

    const writeToolCall = (payload: any) => {
      const name = String(payload?.name ?? '');
      if (!name) return;
      let argPreview = '';
      try {
        const raw = payload?.args;
        const parsed = typeof raw === 'string' ? JSON.parse(raw) : raw;
        argPreview = parsed ? truncate(JSON.stringify(parsed), 240) : '';
      } catch {
        argPreview = typeof payload?.args === 'string' ? truncate(payload.args, 240) : '';
      }
      const calls = this.activityCalls.get(id);
      if (calls && payload?.call_id) {
        calls.set(String(payload.call_id), { name, argPreview });
      }
      term.writeln(`${ts()} \x1b[1;33m▶\x1b[0m \x1b[1m${name}\x1b[0m \x1b[2m${argPreview}\x1b[0m`);
    };

    const writeToolResult = (payload: any) => {
      const name = String(payload?.name ?? '');
      const result = String(payload?.result ?? '');
      const blocked = !!payload?.blocked;
      const callId = payload?.call_id ? String(payload.call_id) : '';
      const calls = this.activityCalls.get(id);
      const prior = callId ? calls?.get(callId) : undefined;
      const tag = prior?.name || name || 'tool';
      const lines = result.split(/\r?\n/);
      const head = truncate(lines[0] ?? '', 320);
      const extra = lines.length > 1 ? ` \x1b[2m(+${lines.length - 1} more lines)\x1b[0m` : '';
      const marker = blocked ? '\x1b[1;35m⏸\x1b[0m' : '\x1b[1;32m✔\x1b[0m';
      term.writeln(`${ts()} ${marker} \x1b[1m${tag}\x1b[0m \x1b[2m→\x1b[0m ${head}${extra}`);
      if (callId && calls) calls.delete(callId);
    };

    const writeAction = (payload: any) => {
      const action = String(payload?.action ?? '').trim();
      const tool = payload?.tool ? ` \x1b[2m[${payload.tool}]\x1b[0m` : '';
      if (action) term.writeln(`${ts()} \x1b[1;34m•\x1b[0m ${action}${tool}`);
    };

    const writeChat = (payload: any) => {
      const chunk = typeof payload === 'string' ? payload : String(payload?.content ?? '');
      if (chunk) term.write(chunk);
    };

    // ── Live `run_command` streaming ─────────────────────────────────────
    // Backend now emits `ai-tool-stdout-start` / `ai-tool-stdout` (per
    // line) / `ai-tool-stdout-end` so we can paint long-running command
    // output as it happens instead of waiting for the final tool_result.
    const writeStdoutStart = (payload: any) => {
      const cmd = String(payload?.command ?? '').trim();
      if (cmd) {
        term.writeln(
          `${ts()} \x1b[1;36m$\x1b[0m \x1b[1m${cmd}\x1b[0m`,
        );
      }
    };
    const writeStdoutLine = (payload: any) => {
      const line = String(payload?.line ?? '');
      const isErr = payload?.stream === 'stderr';
      if (isErr) {
        term.writeln(`  \x1b[31m${line}\x1b[0m`);
      } else {
        term.writeln(`  ${line}`);
      }
    };
    const writeStdoutEnd = (payload: any) => {
      const code = payload?.exit_code ?? null;
      const ok = !!payload?.success;
      const marker = ok ? '\x1b[1;32m✔\x1b[0m' : '\x1b[1;31m✖\x1b[0m';
      term.writeln(
        `${ts()} ${marker} \x1b[2mexit ${code === null ? '?' : code}\x1b[0m`,
      );
    };

    // ── Activity transport: INVOKE-POLL, not events ─────────────────────
    // The Tauri global event stream is dead in this webview (emit→listen never
    // delivers), so we poll the backend's activity buffer instead. Each drained
    // line is `{kind, payload}` JSON; dispatch to the matching writer above.
    const dispatch: Record<string, (p: any) => void> = {
      'ai-tool-call': writeToolCall,
      'ai-tool-result': writeToolResult,
      'ai-action': writeAction,
      'ai-tool-stdout-start': writeStdoutStart,
      'ai-tool-stdout': writeStdoutLine,
      'ai-tool-stdout-end': writeStdoutEnd,
    };
    let draining = false;
    const drainTick = async () => {
      if (draining) return;
      draining = true;
      try {
        const lines = await invoke<string[]>('agent_activity_drain');
        for (const raw of lines) {
          try {
            const evt = JSON.parse(raw);
            const fn = dispatch[evt?.kind];
            if (fn) fn(evt.payload);
          } catch { /* skip malformed line */ }
        }
      } catch { /* backend not ready yet */ }
      finally { draining = false; }
    };
    const activityTimer = setInterval(drainTick, 120);
    unsubs.push(() => clearInterval(activityTimer));
    this.activityUnsubs.set(id, unsubs);
    void drainTick();
    // `writeChat` is reserved for a future "stream model output" wire-up; reference it
    // so esbuild doesn't yell about an unused local.
    void writeChat;

    const group = this.groups.get(this.activeGroupId || '') || null;
    const actualGroupId = group?.id || this.createGroupId();
    if (!this.groups.has(actualGroupId)) {
      this.groups.set(actualGroupId, {
        id: actualGroupId,
        instances: [],
        activeInstanceId: null,
        layout: 'single',
      });
    }
    const g = this.groups.get(actualGroupId)!;
    g.instances.push(instance);
    g.activeInstanceId = id;
    this.activeGroupId = actualGroupId;

    // Resize on the next paint so xterm actually measures the container.
    setTimeout(() => { try { fitAddon.fit(); } catch { /* */ } }, 0);

    return id;
  }

  /**
   * True if this terminal is an AIRI activity feed (no PTY backing).
   * Used by keystroke/resize hooks to avoid posting to a non-existent PTY.
   */
  isActivityTerminal(id: string): boolean {
    return this.activityIds.has(id);
  }

  // ── Warp-style command-block navigation (bind to keys if desired) ──────────
  scrollToPreviousCommand(id: string): void {
    this.terminals.get(id)?.blocks?.scrollToPreviousCommand();
  }
  scrollToNextCommand(id: string): void {
    this.terminals.get(id)?.blocks?.scrollToNextCommand();
  }
  rerunLastCommand(id: string): void {
    this.terminals.get(id)?.blocks?.rerunLastCommand();
  }

  // ═══════════════════════════════════════════════════════════════════════
  // TERMINAL OPERATIONS
  // ═══════════════════════════════════════════════════════════════════════

  async closeTerminal(id: string): Promise<void> {
    const instance = this.terminals.get(id);
    if (instance) {
      this.stopPolling(id);
      const isActivity = this.activityIds.has(id);
      if (!isActivity) {
        try {
          await invoke('close_terminal', { id });
        } catch (e) {}
      }

      // Tear down activity-feed subscriptions before disposing xterm.
      if (isActivity) {
        const unsubs = this.activityUnsubs.get(id) || [];
        for (const u of unsubs) {
          try { u(); } catch { /* */ }
        }
        this.activityUnsubs.delete(id);
        this.activityCalls.delete(id);
        this.activityIds.delete(id);
      }

      try { instance.blocks?.dispose(); } catch { /* */ }
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
    const newId = await this.createTerminal(instance.shell, group.id, instance.cwd);
    
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

    // Capture once so all AI menu items see the same snapshot of buffer/
    // selection; otherwise reading at click time can race with the user
    // scrolling or selecting new text.
    const hasSelection = instance.term.hasSelection();
    const selectedText = hasSelection ? instance.term.getSelection() : '';
    // Pull a chunky tail of the visible buffer for the "Explain last
    // output" / "Fix last error" paths. We grab the active terminal's
    // ring buffer through the dedicated IPC so we get raw command output
    // (not just what's currently scrolled into view).
    const askAgent = async (prompt: string) => {
      try {
        const m = await import('./agent');
        // Dispatch a chat-style message so the user can see the agent's
        // response inline. The right sidebar will auto-open if it's not
        // already because addAgentMessage triggers a state change.
        await m.sendAgentMessage(prompt, () => {}, []);
      } catch (e) {
        console.error('[AI Terminal] failed to dispatch prompt:', e);
      }
    };

    const buildTail = async (): Promise<string> => {
      try {
        const { invoke } = await import('./tauri_bridge');
        const out = await invoke<string>('terminal_read_output', { id: instance.id });
        return (out || '').slice(-4000);
      } catch {
        // Fallback to the xterm buffer if the IPC failed (e.g. backend
        // restarted but the xterm still has scrollback in memory).
        try {
          const buf = instance.term.buffer.active;
          const lines: string[] = [];
          for (let i = Math.max(0, buf.length - 200); i < buf.length; i++) {
            lines.push(buf.getLine(i)?.translateToString(true) || '');
          }
          return lines.join('\n').slice(-4000);
        } catch { return ''; }
      }
    };

    const menuItems: any[] = [
      { label: 'New Terminal', icon: 'add', action: () => this.createTerminal() },
      { label: 'Split Terminal', icon: 'split-horizontal', action: () => this.splitTerminal(instance.id, 'horizontal') },
      { type: 'separator' },
      { label: 'Copy', icon: 'copy', action: () => this.copySelection(instance) },
      { label: 'Paste', icon: 'paste', action: () => this.paste(instance) },
      { label: 'Select All', icon: 'selection', action: () => this.selectAll(instance) },
      { type: 'separator' },
      // ── AI Terminal actions (Windsurf-style) ───────────────────────
      // Each item ships a different framing to the agent so the result
      // is targeted: explanation, root-cause + fix proposal, or a
      // direct command suggestion. Selection-aware: if the user
      // highlighted text we use that; otherwise we fall back to the
      // last 4KB of terminal output.
      {
        label: hasSelection ? 'Explain selection' : 'Explain last output',
        icon: 'sparkle',
        action: async () => {
          const body = hasSelection ? selectedText : await buildTail();
          if (!body.trim()) return;
          await askAgent(
            `[INTENT: TERMINAL EXPLAIN]\nExplain this terminal output in plain English. Identify what command produced it, what the output means, and call out anything noteworthy.\n\n\`\`\`\n${body}\n\`\`\``
          );
        },
      },
      {
        label: hasSelection ? 'Fix error in selection' : 'Fix last error',
        icon: 'wand',
        action: async () => {
          const body = hasSelection ? selectedText : await buildTail();
          if (!body.trim()) return;
          await askAgent(
            `[INTENT: TERMINAL FIX]\nThe following terminal output contains an error. Diagnose the root cause and propose a concrete fix. If a file edit would resolve it, use the appropriate edit tool; if a command would resolve it, suggest the exact command.\n\n\`\`\`\n${body}\n\`\`\``
          );
        },
      },
      {
        label: 'Suggest next command',
        icon: 'arrow-right',
        action: async () => {
          const body = await buildTail();
          await askAgent(
            `[INTENT: TERMINAL NEXT]\nGiven this recent terminal history, suggest the next 1-3 useful commands the user might run. Reply with code-fenced commands only, no narration.\n\n\`\`\`\n${body}\n\`\`\``
          );
        },
      },
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

  /** The active PTY terminal instance (for the command palette / workflows). */
  getActiveInstance(): TerminalInstance | undefined {
    const g = this.getActiveGroup();
    if (!g || !g.activeInstanceId) return undefined;
    return this.getTerminal(g.activeInstanceId);
  }

  /** Recent unique command history of the active terminal (newest first). */
  getActiveCommandHistory(): string[] {
    return this.getActiveInstance()?.blocks?.getCommandHistory() ?? [];
  }

  /** Send raw data to the active terminal's PTY. */
  sendToActive(data: string): void {
    const inst = this.getActiveInstance();
    if (inst) {
      void invoke('terminal_send_data', { id: inst.id, data });
      try { inst.term.focus(); } catch { /* */ }
    }
  }

  /** Type a command at the prompt WITHOUT running it (palette default). */
  insertInActive(cmd: string): void { this.sendToActive(cmd); }

  /** Type a command and run it (palette Ctrl+Enter). */
  runInActive(cmd: string): void { this.sendToActive(cmd + '\r'); }

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

    // Ctrl+Shift+R - Terminal command palette (history + workflows). Pure,
    // non-AI recall: dispatch an event the React palette listens for.
    if ((e.key === 'r' || e.key === 'R') && e.ctrlKey && e.shiftKey) {
      e.preventDefault();
      window.dispatchEvent(new CustomEvent('vscr:open-terminal-palette'));
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

let _terminalBootstrapped = false;
export const initTerminal = async (addTerminalGroup: () => void | Promise<void>) => {
  // Guard against double-spawn: React 18 StrictMode runs init effects TWICE in
  // dev (mount→unmount→mount), which created two terminals. The module-level flag
  // persists across the double-invoke so exactly ONE initial terminal is created.
  if (_terminalBootstrapped) return;
  _terminalBootstrapped = true;
  try {
    await addTerminalGroup();
  } catch (e: any) {
    const msg = e?.message || e?.toString?.() || String(e) || 'unknown error';
    console.warn('[terminal] Failed to create initial terminal:', msg, e);
  }
};
