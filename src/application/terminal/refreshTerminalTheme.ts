import {
    getTerminalThemeMode,
    resolveTerminalTheme,
    setTerminalThemeMode,
} from '../../infrastructure/terminal/terminalThemes';
import type { TerminalThemeMode } from '../../domain/terminal/TerminalTheme';
import { getTerminalManager } from './getTerminalManager';

export function getActiveTerminalThemeMode(): TerminalThemeMode {
    return getTerminalThemeMode();
}

/** Toggle between IDE theme and native Tokyo-night palette. */
export async function setActiveTerminalThemeMode(mode: TerminalThemeMode): Promise<void> {
    setTerminalThemeMode(mode);
    const mgr = await getTerminalManager();
    const theme = resolveTerminalTheme(mode);
    for (const instance of mgr.getAllTerminals()) {
        instance.term.options.theme = theme;
    }
}

export async function refreshAllTerminalThemes(): Promise<void> {
    const mgr = await getTerminalManager();
    const theme = resolveTerminalTheme();
    for (const instance of mgr.getAllTerminals()) {
        instance.term.options.theme = theme;
    }
}
