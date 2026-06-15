import { invoke } from '../../tauri_bridge';
import { desktopProjectPath, isWindowsHost, toHostPath } from '../../lib/hostPaths';

const CANDIDATE_NAMES = ['Virtual-iPhone-Emulator', 'virtual-iphone-emulator'];

/** Resolve the acheron project root (Virtual-iPhone-Emulator). */
export async function resolveEmulatorProjectPath(): Promise<string> {
    const activeRoot = (await import('../../store')).useStore.getState().activeRoot;

    const candidates: string[] = [];
    const legacyDefault = await desktopProjectPath('vscodium-rust/Virtual-iPhone-Emulator');
    if (legacyDefault) candidates.push(legacyDefault);
    if (activeRoot) {
        for (const name of CANDIDATE_NAMES) {
            candidates.push(`${activeRoot.replace(/\\/g, '/')}/${name}`);
            candidates.push(`${activeRoot.replace(/\\/g, '/')}/../${name}`);
        }
    }

    // Repo-relative when IDE is opened from vscodium-rust root
    try {
        const cwd = await invoke<string>('get_active_root').catch(() => '');
        if (cwd) {
            for (const name of CANDIDATE_NAMES) {
                candidates.push(`${cwd.replace(/\\/g, '/')}/${name}`);
            }
        }
    } catch { /* */ }

    for (const c of candidates) {
        const normalized = toHostPath(c);
        try {
            const exists = await invoke<boolean>('path_exists', { path: normalized });
            if (exists) return normalized;
        } catch { /* try next */ }
    }

    return toHostPath(candidates[candidates.length - 1] ?? 'Virtual-iPhone-Emulator');
}

export async function probeEmulatorBinary(projectPath: string): Promise<string | null> {
    const sep = isWindowsHost() ? '\\' : '/';
    const checks = [
        `${projectPath}${sep}build${sep}Release${sep}acheron.exe`,
        `${projectPath}${sep}build${sep}acheron.exe`,
        `${projectPath}${sep}acheron.exe`,
        `${projectPath}${sep}acheron-native`,
        `${projectPath}${sep}acheron-signed`,
    ];
    for (const p of checks) {
        try {
            if (await invoke<boolean>('path_exists', { path: p })) return p;
        } catch { /* */ }
    }
    return null;
}
