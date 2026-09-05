// ════════════════════════════════════════════════════════════════════════════
// Warp-style command blocks for xterm.js — ORIGINAL implementation.
//
// Not a copy of Warp (which is a native Rust/GPU app, source-available, NOT MIT).
// This is our own take on the idea: shell integration via OSC 133 (the FinalTerm /
// VS Code standard) drives a per-command "block" model. Each command gets:
//   • a colored gutter mark (green = exit 0, red = failed, blue = running)
//   • a hover toolbar (copy command, copy output, re-run, clear-to-here)
//   • prev/next command navigation
//
// The shell emits the markers; xterm parses them; we draw the decorations. No
// terminal-renderer replacement, so it composes with the existing DOM-rendered
// xterm and the Tauri PTY transport untouched.
// ════════════════════════════════════════════════════════════════════════════

import type { Terminal, IMarker, IDecoration, IDisposable } from '@xterm/xterm';

export interface CommandBlock {
  id: number;
  /**Marker on the prompt/command line — anchors the gutter decoration. */
  promptMarker?: IMarker;
  /**Marker at output start (OSC 133;C). */
  outputStartMarker?: IMarker;
  /**Marker at output end (OSC 133;D). */
  outputEndMarker?: IMarker;
  commandLine: string;
  exitCode: number | null;
  startTime: number | null;
  endTime: number | null;
  cwd: string;
  running: boolean;
  decoration?: IDecoration;
  /**Full-width Warp-style header band at the command line. */
  bandDecoration?: IDecoration;
}

/**
 * PowerShell shell-integration script. Emits OSC 133 sequences so we can detect
 * command boundaries, exit codes, and the command line itself. BEL-terminated
 * (`\a`) OSC — xterm parses it cleanly and it's trivial to embed in PS strings.
 *
 *   133;A            prompt start
 *   133;B            command input start
 *   133;C            pre-exec (output starts)
 *   133;D;<exit>     command finished + exit code
 *   133;E;<cmdline>  the command line (our extension, consumed below)
 *   133;P;Cwd=<dir>  working directory
 *
 * Designed to be sent as ONE line over stdin after spawn. Guards re-injection and
 * degrades gracefully if PSReadLine is absent (loses only pre-exec timing + the
 * command line, still gets per-command success/fail marks).
 */
export const POWERSHELL_SHELL_INTEGRATION: string = [
  'if (-not $global:__vscr_si) {',
  '  $global:__vscr_si = $true;',
  '  $global:__vscr_orig_prompt = $function:prompt;',
  '  function global:prompt {',
  '    $code = if ($?) { 0 } else { if ($LASTEXITCODE) { $LASTEXITCODE } else { 1 } };',
  '    $e = [char]27; $b = [char]7;',
  '    $cwd = (Get-Location).Path;',
  '    [Console]::Write("$e]133;D;$code$b$e]133;A$b$e]133;P;Cwd=$cwd$b");',
  '    $orig = & $global:__vscr_orig_prompt;',
  '    [Console]::Write("$e]133;B$b");',
  '    return $orig;',
  '  };',
  '  if (Get-Module -ListAvailable PSReadLine) {',
  '    Set-PSReadLineKeyHandler -Key Enter -ScriptBlock {',
  '      $line = $null; $cur = $null;',
  '      [Microsoft.PowerShell.PSConsoleReadLine]::GetBufferState([ref]$line, [ref]$cur);',
  '      $e = [char]27; $b = [char]7;',
  '      [Console]::Write("$e]133;E;$line$b$e]133;C$b");',
  '      [Microsoft.PowerShell.PSConsoleReadLine]::AcceptLine();',
  '    };',
  '  };',
  '}',
].join(' ');

type SendInput = (data: string) => void;

export class CommandBlockTracker {
  private term: Terminal;
  private sendInput: SendInput;
  private disposables: IDisposable[] = [];
  private blocks: CommandBlock[] = [];
  private current: CommandBlock | null = null;
  private nextId = 1;
  private lastCwd = '';
  private toolbar: HTMLDivElement | null = null;
  private hideTimer: any = null;
  private styleInjected = false;

  constructor(term: Terminal, sendInput: SendInput) {
    this.term = term;
    this.sendInput = sendInput;
  }

  attach(): void {
    this.injectStyle();
    // xterm's OSC handler: the dispatcher strips `133;`, handing us the payload.
    const anyTerm = this.term as any;
    if (anyTerm.parser?.registerOscHandler) {
      this.disposables.push(
        anyTerm.parser.registerOscHandler(133, (data: string) => this.onOsc(data)),
      );
    }
  }

  dispose(): void {
    for (const d of this.disposables) {
      try { d.dispose(); } catch { /* */ }
    }
    this.disposables = [];
    for (const b of this.blocks) {
      try { b.decoration?.dispose(); } catch { /* */ }
      try { b.bandDecoration?.dispose(); } catch { /* */ }
    }
    this.blocks = [];
    if (this.toolbar) { try { this.toolbar.remove(); } catch { /* */ } this.toolbar = null; }
  }

  // ── OSC 133 dispatch ──────────────────────────────────────────────────────
  private onOsc(data: string): boolean {
    const semi = data.indexOf(';');
    const kind = semi === -1? data: data.slice(0, semi);
    const rest = semi === -1? '': data.slice(semi + 1);

    switch (kind) {
      case 'A':
        // Prompt start — open a fresh block anchored at the prompt line.
        this.startBlock();
        break;
      case 'B':
        // Command input start — nothing to do; command text arrives via E.
        break;
      case 'E':
        if (this.current) this.current.commandLine = stripWeird(rest);
        break;
      case 'C':
        // Pre-exec: output begins. Mark it and stamp the start time.
        if (this.current) {
          this.current.running = true;
          this.current.startTime = Date.now();
          this.current.outputStartMarker = this.term.registerMarker(0) || undefined;
        }
        break;
      case 'D':
        this.endBlock(rest);
        break;
      case 'P':
        if (rest.startsWith('Cwd=')) {
          this.lastCwd = rest.slice(4);
          if (this.current) this.current.cwd = this.lastCwd;
        }
        break;
      default:
        return false;
    }
    return true;
  }

  private startBlock(): void {
    // A `D` without a following command (e.g. the very first prompt) leaves a
    // half-open block; drop it so we don't accumulate empties.
    if (this.current && this.current.startTime === null && !this.current.commandLine) {
      this.current = null;
    }
    const block: CommandBlock = {
      id: this.nextId++,
      promptMarker: this.term.registerMarker(0) || undefined,
      commandLine: '',
      exitCode: null,
      startTime: null,
      endTime: null,
      cwd: this.lastCwd,
      running: false,
    };
    this.current = block;
  }

  private endBlock(exitRaw: string): void {
    const block = this.current;
    if (!block || block.startTime === null) {
      // `D` for a prompt that never ran a command (empty Enter) — discard.
      this.current = null;
      return;
    }
    const exit = parseInt(exitRaw, 10);
    block.exitCode = Number.isFinite(exit)? exit: 0;
    block.endTime = Date.now();
    block.running = false;
    block.outputEndMarker = this.term.registerMarker(0) || undefined;
    this.decorate(block);
    this.blocks.push(block);
    if (this.blocks.length > 500) {
      const dropped = this.blocks.shift();
      try { dropped?.decoration?.dispose(); } catch { /* */ }
      try { dropped?.bandDecoration?.dispose(); } catch { /* */ }
    }
    this.current = null;
  }

  // ── Gutter decoration + hover toolbar ─────────────────────────────────────
  private decorate(block: CommandBlock): void {
    const marker = block.promptMarker;
    if (!marker) return;
    const anyTerm = this.term as any;
    if (!anyTerm.registerDecoration) return;
    const ok = block.exitCode === 0;

    // Warp-style full-width header band, registered FIRST so the interactive
    // gutter mark (below) stacks on top and still receives hover. The band is a
    // decorative one-row strip that separates each command into its own "card".
    const band: IDecoration | undefined = anyTerm.registerDecoration({ marker, x: 0, width: this.term.cols || 80 });
    if (band) {
      block.bandDecoration = band;
      band.onRender((el: HTMLElement) => {
        el.classList.add('vscr-block-band');
        el.classList.toggle('vscr-band-ok', ok);
        el.classList.toggle('vscr-band-fail', !ok);
        // Stretch to the full terminal width regardless of column count.
        const screen = this.term.element?.querySelector('.xterm-screen') as HTMLElement | null;
        if (screen) el.style.width = `${screen.clientWidth}px`;
      });
    }

    const deco: IDecoration | undefined = anyTerm.registerDecoration({ marker, x: 0, width: 1 });
    if (!deco) return;
    block.decoration = deco;

    deco.onRender((el: HTMLElement) => {
      el.classList.add('vscr-block-mark');
      el.classList.toggle('vscr-block-ok', ok);
      el.classList.toggle('vscr-block-fail', !ok);
      el.title = this.blockSummary(block);
      el.onmouseenter = (ev) => this.showToolbar(block, el, ev);
      el.onmouseleave = () => this.scheduleHide();
      el.onclick = (ev) => this.showToolbar(block, el, ev);
    });
  }

  /**Render a block as portable Markdown (command + output + exit) for sharing. */
  private blockAsMarkdown(block: CommandBlock): string {
    const out = this.readOutput(block);
    const code = block.exitCode === 0? 'exit 0': `exit ${block.exitCode}`;
    const dur = block.startTime && block.endTime
? ` · ${formatDuration(block.endTime - block.startTime)}`
: '';
    let md = '```sh\n' + (block.commandLine || '') + '\n```\n';
    if (out) md += '\n```\n' + out + '\n```\n';
    md += `\n_(${code}${dur})_\n`;
    return md;
  }

  private blockSummary(block: CommandBlock): string {
    const dur = block.startTime && block.endTime
? ` · ${formatDuration(block.endTime - block.startTime)}`
: '';
    const code = block.exitCode === 0? 'exit 0': `exit ${block.exitCode}`;
    const cmd = block.commandLine || '(command)';
    return `${cmd}\n${code}${dur}`;
  }

  // ── Block output extraction ───────────────────────────────────────────────
  private readOutput(block: CommandBlock): string {
    const start = block.outputStartMarker?.line ?? -1;
    const end = block.outputEndMarker?.line ?? -1;
    if (start < 0 || end < 0 || end < start) return '';
    const buf = this.term.buffer.active;
    const lines: string[] = [];
    for (let i = start + 1; i < end; i++) {
      const line = buf.getLine(i);
      if (line) lines.push(line.translateToString(true));
    }
    return lines.join('\n').replace(/\s+$/, '');
  }

  // ── Floating toolbar ──────────────────────────────────────────────────────
  private ensureToolbar(): HTMLDivElement {
    if (this.toolbar) return this.toolbar;
    const bar = document.createElement('div');
    bar.className = 'vscr-block-toolbar';
    bar.onmouseenter = () => { if (this.hideTimer) clearTimeout(this.hideTimer); };
    bar.onmouseleave = () => this.scheduleHide();
    const host = this.term.element?.parentElement || document.body;
    host.appendChild(bar);
    this.toolbar = bar;
    return bar;
  }

  private showToolbar(block: CommandBlock, anchor: HTMLElement, ev: MouseEvent): void {
    if (this.hideTimer) { clearTimeout(this.hideTimer); this.hideTimer = null; }
    const bar = this.ensureToolbar();
    bar.innerHTML = '';

    const okMark = block.exitCode === 0? '': '';
    const dur = block.startTime && block.endTime? formatDuration(block.endTime - block.startTime): '';
    const status = document.createElement('span');
    status.className = `vscr-tb-status ${block.exitCode === 0? 'ok': 'fail'}`;
    status.textContent = `${okMark} exit ${block.exitCode}${dur? ' · ' + dur: ''}`;
    bar.appendChild(status);

    const addBtn = (label: string, fn: () => void) => {
      const b = document.createElement('button');
      b.className = 'vscr-tb-btn';
      b.textContent = label;
      b.onclick = (e) => { e.stopPropagation(); fn(); this.flash(b); };
      bar.appendChild(b);
    };

    addBtn('Copy cmd', () => copyText(block.commandLine));
    addBtn('Copy out', () => copyText(this.readOutput(block)));
    addBtn('Re-run', () => {
      if (block.commandLine) this.sendInput(block.commandLine + '\r');
    });
    addBtn('Share', () => copyText(this.blockAsMarkdown(block)));
    addBtn('Save', () => {
      // Persist as a workflow (cmder task / Warp workflow). Dynamic import keeps
      // terminalBlocks decoupled from the workflows module.
      if (block.commandLine) {
        import('./terminalWorkflows.ts')
          .then((m) => m.saveWorkflowFromCommand(block.commandLine))
          .catch(() => { /* workflows module optional */ });
      }
    });
    addBtn('Clear ↑', () => this.clearToBlock(block));

    // Position under the anchor cell, clamped into the terminal viewport.
    const host = this.term.element?.parentElement;
    const hostRect = host?.getBoundingClientRect();
    const aRect = anchor.getBoundingClientRect();
    bar.style.display = 'flex';
    const top = (hostRect? aRect.top - hostRect.top: aRect.top) + anchor.offsetHeight + 2;
    let left = hostRect? aRect.left - hostRect.left: aRect.left;
    left = Math.max(4, left);
    bar.style.top = `${top}px`;
    bar.style.left = `${left}px`;
    void ev;
  }

  private scheduleHide(): void {
    if (this.hideTimer) clearTimeout(this.hideTimer);
    this.hideTimer = setTimeout(() => {
      if (this.toolbar) this.toolbar.style.display = 'none';
    }, 280);
  }

  private flash(btn: HTMLElement): void {
    btn.classList.add('vscr-tb-flash');
    setTimeout(() => btn.classList.remove('vscr-tb-flash'), 350);
  }

  /**Scroll the viewport so the line above this block's prompt is at the top. */
  private clearToBlock(block: CommandBlock): void {
    const line = block.promptMarker?.line;
    if (typeof line === 'number') this.term.scrollToLine(Math.max(0, line));
  }

  // ── Navigation ────────────────────────────────────────────────────────────
  scrollToPreviousCommand(): void { this.scrollRelative(-1); }
  scrollToNextCommand(): void { this.scrollRelative(1); }

  private scrollRelative(dir: -1 | 1): void {
    if (!this.blocks.length) return;
    const viewportTop = this.term.buffer.active.viewportY;
    let target: number | null = null;
    if (dir < 0) {
      for (let i = this.blocks.length - 1; i >= 0; i--) {
        const l = this.blocks[i].promptMarker?.line;
        if (typeof l === 'number' && l < viewportTop) { target = l; break; }
      }
    } else {
      for (let i = 0; i < this.blocks.length; i++) {
        const l = this.blocks[i].promptMarker?.line;
        if (typeof l === 'number' && l > viewportTop) { target = l; break; }
      }
    }
    if (target !== null) this.term.scrollToLine(target);
  }

  /**Re-run the most recent command (Warp's ⌘↑↵ feel). */
  rerunLastCommand(): void {
    const last = this.blocks[this.blocks.length - 1];
    if (last?.commandLine) this.sendInput(last.commandLine + '\r');
  }

  /**Recent unique command lines (most recent first) — feeds the palette. */
  getCommandHistory(): string[] {
    const seen = new Set<string>();
    const out: string[] = [];
    for (let i = this.blocks.length - 1; i >= 0; i--) {
      const c = this.blocks[i].commandLine?.trim();
      if (c && !seen.has(c)) {
        seen.add(c);
        out.push(c);
      }
    }
    return out;
  }

  // ── Styles ────────────────────────────────────────────────────────────────
  private injectStyle(): void {
    if (this.styleInjected) return;
    if (document.getElementById('vscr-block-style')) { this.styleInjected = true; return; }
    const css = `
.vscr-block-mark {
  width: 3px !important;
  margin-left: 0;
  border-radius: 2px;
  cursor: pointer;
  z-index: 6;
  transition: width .12s ease, box-shadow .12s ease;
}
.vscr-block-mark:hover { width: 5px !important; }
.vscr-block-ok   { background: #2ea043; box-shadow: 0 0 6px rgba(46,160,67,.55); }
.vscr-block-fail { background: #f85149; box-shadow: 0 0 6px rgba(248,81,73,.55); }
/* Warp-style per-command header band — a subtle full-width strip that turns
   each command + its output into a visually distinct "block". Decorative only
   (pointer-events disabled so the gutter mark keeps its hover). */
.vscr-block-band {
  pointer-events: none;
  height: 100% !important;
  z-index: 1;
  border-top: 1px solid rgba(255,255,255,.06);
  background: linear-gradient(90deg, rgba(255,255,255,.035), rgba(255,255,255,.012) 45%, transparent 80%);
}
.vscr-band-ok   { box-shadow: inset 3px 0 0 rgba(46,160,67,.45); }
.vscr-band-fail {
  box-shadow: inset 3px 0 0 rgba(248,81,73,.6);
  background: linear-gradient(90deg, rgba(248,81,73,.07), transparent 70%);
}
.vscr-block-toolbar {
  position: absolute;
  z-index: 40;
  display: none;
  align-items: center;
  gap: 4px;
  padding: 4px 6px;
  background: rgba(22,24,30,.97);
  border: 1px solid rgba(255,255,255,.10);
  border-radius: 8px;
  box-shadow: 0 8px 24px rgba(0,0,0,.45);
  backdrop-filter: blur(8px);
  font: 11px/1.4 "Cascadia Mono", Consolas, monospace;
  color: #d7dae0;
  user-select: none;
}
.vscr-tb-status { padding: 0 4px; font-weight: 600; }
.vscr-tb-status.ok   { color: #3fb950; }
.vscr-tb-status.fail { color: #ff7b72; }
.vscr-tb-btn {
  background: rgba(255,255,255,.06);
  border: 1px solid rgba(255,255,255,.08);
  color: #d7dae0;
  padding: 2px 8px;
  border-radius: 6px;
  cursor: pointer;
  transition: background .12s ease, transform .08s ease;
}
.vscr-tb-btn:hover { background: rgba(88,166,255,.20); border-color: rgba(88,166,255,.45); }
.vscr-tb-btn:active { transform: translateY(1px); }
.vscr-tb-flash { background: #2ea043 !important; color: #fff !important; }
`;
    const style = document.createElement('style');
    style.id = 'vscr-block-style';
    style.textContent = css;
    document.head.appendChild(style);
    this.styleInjected = true;
  }
}

// ── helpers ──────────────────────────────────────────────────────────────────
function formatDuration(ms: number): string {
  if (ms < 1000) return `${ms}ms`;
  const s = ms / 1000;
  if (s < 60) return `${s.toFixed(s < 10? 1: 0)}s`;
  const m = Math.floor(s / 60);
  return `${m}m ${Math.round(s % 60)}s`;
}

function stripWeird(s: string): string {
  // PSReadLine can hand back the raw buffer; trim control chars + trailing space.
  return s.replace(/[\x00-\x08\x0b\x0c\x0e-\x1f]/g, '').trim();
}

function copyText(text: string): void {
  if (!text) return;
  try {
    if (navigator.clipboard?.writeText) { void navigator.clipboard.writeText(text); return; }
  } catch { /* fall through */ }
  try {
    const ta = document.createElement('textarea');
    ta.value = text;
    ta.style.position = 'fixed';
    ta.style.opacity = '0';
    document.body.appendChild(ta);
    ta.select();
    document.execCommand('copy');
    ta.remove();
  } catch { /* */ }
}
