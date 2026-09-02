/** ANSI palette passed to xterm.js `theme` option. */
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

/** `vscode` follows IDE CSS vars; `native` uses the Tokyo-night-inspired palette. */
export type TerminalThemeMode = 'vscode' | 'native';
