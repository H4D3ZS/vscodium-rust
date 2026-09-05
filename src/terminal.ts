/**
 * VSCode-Authentic Terminal Implementation
 * Full-featured terminal matching VSCode/VSCodium behavior
 */

import { invoke } from './tauri_bridge.ts';
import { Terminal } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import { SearchAddon } from '@xterm/addon-search';
import { WebLinksAddon } from '@xterm/addon-web-links';
import { WebglAddon } from '@xterm/addon-webgl';
import { Unicode11Addon } from '@xterm/addon-unicode11';
import { CanvasAddon } from '@xterm/addon-canvas';
import { CommandBlockTracker } from './terminalBlocks.ts';
import {
  resolveTerminalProfile,
  DEFAULT_PROFILES,
} from './infrastructure/terminal/terminalProfiles.ts';
import {
  getVSCodeTheme,
  getNativeTerminalTheme,
  resolveTerminalTheme,
} from './infrastructure/terminal/terminalThemes.ts';
import '@xterm/xterm/css/xterm.css';
import { summarizeToolResult } from './domain/agent/cleanAgentContent';

// Re-export domain types for legacy imports.
export type { TerminalProfile } from './domain/terminal/TerminalProfile.ts';
export type { TerminalTheme } from './domain/terminal/TerminalTheme.ts';

export interface ISearchOptions {
  regex?: boolean;
  wholeWord?: boolean;
  caseSensitive?: boolean;
  incremental?: boolean;
}

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

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
  /**Warp-style command-block tracker (OSC 133 driven). PTY terminals only. */
  blocks?: CommandBlockTracker;
}

export interface TerminalGroup {
  id: string;
  instances: TerminalInstance[];
  activeInstanceId: string | null;
  layout: 'single' | 'split-horizontal' | 'split-vertical';
}

export { getVSCodeTheme, getNativeTerminalTheme as getCmderWarpTheme };

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
    // NOTE: no `terminal-data` event subscription — PTY output is rendered
    // exclusively via `terminal_take_pending` polling, and the backend no
    // longer emits the event (it was pure IPC overhead).
  }

  /**Begin adaptive polling: 50ms when active, backs off to 500ms when idle. */
  private startPolling(id: string): void {
    if (this.pollTimers.has(id)) return;

    // Adaptive backoff: emptyStreak counts consecutive empty polls.
    // Active output → 50ms. Quiet for a while → step up to 150ms, then 500ms.
    let emptyStreak = 0;
    let currentInterval = 50;
    let timerId: ReturnType<typeof setTimeout>;

    const tick = async () => {
      if (!this.terminals.has(id)) { this.stopPolling(id); return; }
      try {
        const chunk = await invoke<string>('terminal_take_pending', { id });
        if (chunk) {
          emptyStreak = 0;
          currentInterval = 50; // snap back to fast poll on activity
          const inst = this.terminals.get(id);
          if (inst) {
            this.firstDataIds.add(id);
            inst.term.write(chunk);
            inst.lastOutput = chunk;
          }
        } else {
          emptyStreak++;
          // Back off: 5 empty → 150ms, 25 empty → 500ms
          if (emptyStreak >= 25) currentInterval = 500;
          else if (emptyStreak >= 5) currentInterval = 150;
        }
      } catch {
        /* backend not ready / terminal gone — keep ticking */
      }
      // Re-schedule only if not yet stopped
      if (this.pollTimers.has(id)) {
        timerId = setTimeout(tick, currentInterval);
        this.pollTimers.set(id, timerId);
      }
    };

    timerId = setTimeout(tick, 0); // immediate first pull
    this.pollTimers.set(id, timerId);
  }

  private stopPolling(id: string): void {
    const t = this.pollTimers.get(id);
    if (t != null) { clearTimeout(t); this.pollTimers.delete(id); }
  }

  // ═══════════════════════════════════════════════════════════════════════
  // INITIALIZATION
  // ═══════════════════════════════════════════════════════════════════════

  private async loadProfiles() {
    // Load default profiles based on platform
    const platform = navigator.platform.toLowerCase().includes('win')? 'win32': 
                     navigator.platform.toLowerCase().includes('mac')? 'darwin': 'linux';
    
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
    /**Must match React/store `instanceId` so `attach()` can find this terminal. */
    explicitId?: string,
    /**Override the Tauri spawn command. When set, invoked as `{ id }` only (no shell arg). */
    spawnCommand?: string,
    /**Extra args merged into the spawn payload. Only used with `spawnCommand`. */
    spawnArgs?: Record<string, unknown>
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
      theme: resolveTerminalTheme(),
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

    // Clean start — no boot banner (Cursor-style: just the shell prompt). The
    // diagnostic "no shell output after ~2s" watchdog below still catches a dead
    // PTY, so we don't need a cosmetic ready line cluttering the terminal.

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
      const result = await invoke<{ id?: string; status?: string; pid?: number }>(
        spawnCommand || 'spawn_terminal',
        spawnCommand? { id, ...(spawnArgs || {}) }: { id, shell: profile.path }
      );

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

  /**Spawn an OpenCode TUI session. Passes provider env vars via the dedicated Tauri command. */
  async createOpenCodeTerminal(explicitId?: string, groupId?: string): Promise<string> {
    return this.createTerminal(undefined, groupId, undefined, explicitId, 'spawn_opencode_terminal');
  }

  /**
   * Spawn a Claude Code session wired to the local Lemonade server.
   *
   * The backend applies the measured per-model `ctx_size`/`llamacpp.args` and
   * reloads if they changed, maps every model alias to the local model, and
   * defaults to skip-permissions + airgapped. Omitting `model` uses whatever
   * the IDE currently has selected.
   */
  async createClaudeCodeTerminal(
    opts: {
      explicitId?: string;
      groupId?: string;
      model?: string;
      skipPermissions?: boolean;
      allowNet?: boolean;
      extraArgs?: string[];
    } = {}
  ): Promise<string> {
    const { explicitId, groupId, model, skipPermissions, allowNet, extraArgs } = opts;
    return this.createTerminal(
      undefined,
      groupId,
      undefined,
      explicitId,
      'spawn_claude_terminal',
      // Tauri maps camelCase args onto the Rust snake_case params.
      {
        ...(model? { model }: {}),
        ...(skipPermissions !== undefined? { skipPermissions }: {}),
        ...(allowNet !== undefined? { allowNet }: {}),
        ...(extraArgs? { extraArgs }: {}),
      }
    );
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
  /**Tool calls keyed by call_id, so the result can render on the same block. */
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
    element.className = 'terminal-instance-element terminal-activity-feed';
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
    // DOM renderer only — AIRI is a virtual output terminal; WebGL keeps the GPU
    // in a high-power state even when the panel is hidden.
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
    term.writeln('\x1b[2mRead-only feed (no shell). Drag to select · Ctrl+C copy · right-click Copy all.\x1b[0m');
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

    const truncate = (s: string, n: number) => (s.length > n? s.slice(0, n - 1) + '…': s);

    const writeToolCall = (payload: any) => {
      const name = String(payload?.name ?? '');
      if (!name) return;
      let argPreview = '';
      try {
        const raw = payload?.args;
        const parsed = typeof raw === 'string'? JSON.parse(raw): raw;
        argPreview = parsed? truncate(JSON.stringify(parsed), 240): '';
      } catch {
        argPreview = typeof payload?.args === 'string'? truncate(payload.args, 240): '';
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
      const callId = payload?.call_id? String(payload.call_id): '';
      const calls = this.activityCalls.get(id);
      const prior = callId? calls?.get(callId): undefined;
      const tag = prior?.name || name || 'tool';
      let parsedArgs: Record<string, unknown> = {};
      try {
        if (prior?.argPreview) parsedArgs = JSON.parse(prior.argPreview);
      } catch { /* ignore */ }
      const summary = summarizeToolResult(tag, result, parsedArgs);
      const marker = blocked? '\x1b[1;35m⏸\x1b[0m': '\x1b[1;32m\x1b[0m';
      term.writeln(`${ts()} ${marker} \x1b[1m${tag}\x1b[0m \x1b[2m→\x1b[0m ${summary}`);
      if (callId && calls) calls.delete(callId);
    };

    const writeAction = (_payload: any) => {
      // Skipped — ai-tool-call already logs ▶ name; avoids duplicate "Executing tool:" lines.
    };

    const writeChat = (payload: any) => {
      const chunk = typeof payload === 'string'? payload: String(payload?.content ?? '');
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
      const marker = ok? '\x1b[1;32m\x1b[0m': '\x1b[1;31m\x1b[0m';
      term.writeln(
        `${ts()} ${marker} \x1b[2mexit ${code === null? '?': code}\x1b[0m`,
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
      // Don't burn 8 invokes/sec while the window is hidden — the buffer is
      // drained in one shot when the tab becomes visible again.
      if (typeof document !== 'undefined' && document.hidden) return;
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

    // Context menu + Ctrl+C copy (activity terminals skip this in createTerminal).
    this.setupTerminalEvents(instance);

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
      this.pendingWrites.delete(id);
      this.firstDataIds.delete(id);
      this.openedIds.delete(id);
      this.pollTimers.delete(id);
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
    group.layout = direction === 'horizontal'? 'split-horizontal': 'split-vertical';
    
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

    const isActivity = this.activityIds.has(instance.id);
    const menuItems: any[] = isActivity
? [
          { label: 'Copy', icon: 'copy', action: () => this.copySelection(instance) },
          { label: 'Copy all', icon: 'copy', action: () => this.copyAll(instance) },
          { label: 'Select all', icon: 'selection', action: () => this.selectAll(instance) },
          { type: 'separator' },
          { label: 'Clear', icon: 'clear-all', action: () => instance.term.clear() },
        ]
: [
          { label: 'New Terminal', icon: 'add', action: () => this.createTerminal() },
          { label: 'Split Right', icon: 'split-horizontal', action: () => this.splitTerminal(instance.id, 'horizontal') },
          { label: 'Split Down', icon: 'split-vertical', action: () => this.splitTerminal(instance.id, 'vertical') },
          { type: 'separator' },
          { label: 'Copy', icon: 'copy', action: () => this.copySelection(instance) },
          { label: 'Copy all', icon: 'copy', action: () => this.copyAll(instance) },
          { label: 'Paste', icon: 'paste', action: () => this.paste(instance) },
          { label: 'Select All', icon: 'selection', action: () => this.selectAll(instance) },
          { type: 'separator' },
          { label: 'Previous Command', icon: 'arrow-up', action: () => this.scrollToPreviousCommand(instance.id) },
          { label: 'Next Command', icon: 'arrow-down', action: () => this.scrollToNextCommand(instance.id) },
          { label: 'Re-run Last Command', icon: 'debug-restart', action: () => this.rerunLastCommand(instance.id) },
          { type: 'separator' },
          { label: 'Clear', icon: 'clear-all', action: () => instance.term.clear() },
          { label: 'Kill Terminal', icon: 'trash', action: () => this.closeTerminal(instance.id) },
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
      void navigator.clipboard.writeText(selectedText);
    }
  }

  /**Select entire scrollback and copy to clipboard (activity log export). */
  copyAll(instance: TerminalInstance) {
    instance.term.selectAll();
    const text = instance.term.getSelection();
    if (text) {
      void navigator.clipboard.writeText(text);
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
    return this.activeGroupId? this.groups.get(this.activeGroupId): undefined;
  }

  getAllTerminals(): TerminalInstance[] {
    return Array.from(this.terminals.values());
  }

  /**The active PTY terminal instance (for the command palette / workflows). */
  getActiveInstance(): TerminalInstance | undefined {
    const g = this.getActiveGroup();
    if (!g || !g.activeInstanceId) return undefined;
    return this.getTerminal(g.activeInstanceId);
  }

  /**Recent unique command history of the active terminal (newest first). */
  getActiveCommandHistory(): string[] {
    return this.getActiveInstance()?.blocks?.getCommandHistory() ?? [];
  }

  /**Send raw data to the active terminal's PTY. */
  sendToActive(data: string): void {
    const inst = this.getActiveInstance();
    if (inst) {
      void invoke('terminal_send_data', { id: inst.id, data });
      try { inst.term.focus(); } catch { /* */ }
    }
  }

  /**Type a command at the prompt WITHOUT running it (palette default). */
  insertInActive(cmd: string): void { this.sendToActive(cmd); }

  /**Type a command and run it (palette Ctrl+Enter). */
  runInActive(cmd: string): void { this.sendToActive(cmd + '\r'); }

  updateAllThemes() {
    const theme = resolveTerminalTheme();
    for (const instance of this.terminals.values()) {
      instance.term.options.theme = theme;
    }
  }
}

/**@deprecated Use `application/terminal/bootstrapTerminalRuntime` instead. */
export const registerTerminalShortcuts = (_manager: TerminalManager) => {
  void import('./application/terminal/bootstrapTerminalRuntime').then((m) => m.bootstrapTerminalRuntime());
};

// ═══════════════════════════════════════════════════════════════════════════
// EXPORT SINGLETON
// ═══════════════════════════════════════════════════════════════════════════

export const terminalManager = new TerminalManager();

// ═══════════════════════════════════════════════════════════════════════════
// LEGACY INIT FUNCTION (for App.tsx compatibility)
// ═══════════════════════════════════════════════════════════════════════════

/**@deprecated Use `application/terminal/initDefaultTerminal` instead. */
export const initTerminal = async (_addTerminalGroup?: () => void | Promise<void>) => {
  const { initDefaultTerminal } = await import('./application/terminal/bootstrapTerminalRuntime');
  await initDefaultTerminal();
};
