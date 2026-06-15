/**
 * Cross-platform host path helpers — replaces the legacy hardcoded
 * `C:\Users\HADES\...` defaults so the IDE works for any user on
 * Windows, macOS, and Linux.
 */
import { homeDir } from '@tauri-apps/api/path';

let cachedHome: string | null = null;

/** User home directory with trailing separators stripped ('' if unavailable). */
export async function userHome(): Promise<string> {
    if (cachedHome !== null) return cachedHome;
    try {
        cachedHome = (await homeDir()).replace(/[\\/]+$/, '');
    } catch {
        cachedHome = '';
    }
    return cachedHome;
}

/** True when the IDE is running on Windows (path separator / drive letters). */
export function isWindowsHost(): boolean {
    return navigator.userAgent.includes('Windows');
}

/** Normalize separators for the host OS. */
export function toHostPath(p: string): string {
    return isWindowsHost() ? p.replace(/\//g, '\\') : p.replace(/\\/g, '/');
}

/**
 * Legacy desktop-project default: `<home>/Desktop/<name>`.
 * Returns '' when the home dir can't be resolved — callers should treat
 * that as "no default" rather than guessing.
 */
export async function desktopProjectPath(name: string): Promise<string> {
    const home = await userHome();
    return home ? toHostPath(`${home}/Desktop/${name}`) : '';
}
