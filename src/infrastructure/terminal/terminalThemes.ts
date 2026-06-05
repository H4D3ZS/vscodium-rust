import type { TerminalTheme, TerminalThemeMode } from '../../domain/terminal/TerminalTheme';

const THEME_MODE_KEY = 'vscr.terminal.themeMode';

/** Read persisted theme preference (defaults to VSCode IDE theme). */
export function getTerminalThemeMode(): TerminalThemeMode {
    try {
        const v = localStorage.getItem(THEME_MODE_KEY);
        return v === 'native' ? 'native' : 'vscode';
    } catch {
        return 'vscode';
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
 * Warp/cmder-inspired palette (Tokyo-night-ish). Original colors — not copied
 * from either product. Gives the panel a distinct native-terminal feel.
 */
export function getNativeTerminalTheme(): TerminalTheme {
    return {
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
    };
}

export function resolveTerminalTheme(mode?: TerminalThemeMode): TerminalTheme {
    const m = mode ?? getTerminalThemeMode();
    return m === 'native' ? getNativeTerminalTheme() : getVSCodeTheme();
}
