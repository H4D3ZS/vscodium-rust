import type { TerminalTheme, TerminalThemeMode } from '../../domain/terminal/TerminalTheme';

const THEME_MODE_KEY = 'vscr.terminal.themeMode';

/** Read persisted theme preference (defaults to the native Cursor-style theme). */
export function getTerminalThemeMode(): TerminalThemeMode {
    try {
        const v = localStorage.getItem(THEME_MODE_KEY);
        return v === 'vscode' ? 'vscode' : 'native';
    } catch {
        return 'native';
    }
}

export function setTerminalThemeMode(mode: TerminalThemeMode): void {
    try {
        localStorage.setItem(THEME_MODE_KEY, mode);
    } catch { /* */ }
}

/** Follows `--vscode-terminal-*` CSS variables from the workbench theme. */
export function getVSCodeTheme(): TerminalTheme {
    const style = getComputedStyle(document.documentElement);
    const pick = (v: string, fb: string) => style.getPropertyValue(v).trim() || fb;
    return {
        background: pick('--vscode-terminal-background', '#1e1e1e'),
        foreground: pick('--vscode-terminal-foreground', '#cccccc'),
        cursor: pick('--vscode-terminalCursor-foreground', '#aeafad'),
        cursorAccent: pick('--vscode-terminalCursor-background', '#1e1e1e'),
        selectionBackground: pick('--vscode-terminal-selectionBackground', 'rgba(255, 255, 255, 0.15)'),
        black: pick('--vscode-terminal-ansiBlack', '#000000'),
        red: pick('--vscode-terminal-ansiRed', '#cd3131'),
        green: pick('--vscode-terminal-ansiGreen', '#0dbc79'),
        yellow: pick('--vscode-terminal-ansiYellow', '#e5e510'),
        blue: pick('--vscode-terminal-ansiBlue', '#2472c8'),
        magenta: pick('--vscode-terminal-ansiMagenta', '#bc3fbc'),
        cyan: pick('--vscode-terminal-ansiCyan', '#11a8cd'),
        white: pick('--vscode-terminal-ansiWhite', '#e5e5e5'),
        brightBlack: pick('--vscode-terminal-ansiBrightBlack', '#666666'),
        brightRed: pick('--vscode-terminal-ansiBrightRed', '#f14c4c'),
        brightGreen: pick('--vscode-terminal-ansiBrightGreen', '#23d18b'),
        brightYellow: pick('--vscode-terminal-ansiBrightYellow', '#f5f543'),
        brightBlue: pick('--vscode-terminal-ansiBrightBlue', '#3b8eea'),
        brightMagenta: pick('--vscode-terminal-ansiBrightMagenta', '#d670d6'),
        brightCyan: pick('--vscode-terminal-ansiBrightCyan', '#29b8db'),
        brightWhite: pick('--vscode-terminal-ansiBrightWhite', '#e5e5e5'),
    };
}

/**
 * Cursor-native terminal palette — clean neutral dark matching the Cursor IDE
 * look (and our neutral-dark agent chat): near-black surface, soft neutral
 * foreground, a single blue cursor/accent, standard balanced ANSI colors.
 */
export function getNativeTerminalTheme(): TerminalTheme {
    return {
        // Match the panel/editor background exactly so the terminal reads as one
        // seamless black surface (Cursor-style), not a boxed-in pane.
        background: '#1a1a1a',
        foreground: '#d4d4d6',
        cursor: '#3b82f6',
        cursorAccent: '#1a1a1a',
        selectionBackground: 'rgba(59,130,246,0.25)',
        black: '#1b1d20',
        red: '#f14c4c',
        green: '#3fb950',
        yellow: '#d6ad3c',
        blue: '#4c8ef7',
        magenta: '#bc7cf0',
        cyan: '#3dc9d6',
        white: '#cfd1d4',
        brightBlack: '#5a5f66',
        brightRed: '#ff6a6a',
        brightGreen: '#56d364',
        brightYellow: '#e7c662',
        brightBlue: '#6ba5ff',
        brightMagenta: '#d2a8ff',
        brightCyan: '#5fdce8',
        brightWhite: '#f0f1f3',
    };
}

export function resolveTerminalTheme(mode?: TerminalThemeMode): TerminalTheme {
    const m = mode ?? getTerminalThemeMode();
    return m === 'native' ? getNativeTerminalTheme() : getVSCodeTheme();
}
