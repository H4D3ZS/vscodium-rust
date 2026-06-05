/** Lazy-load the xterm runtime (~2 MB) only when a terminal is opened. */
export async function getTerminalManager() {
    const { terminalManager } = await import('../../terminal');
    return terminalManager;
}
