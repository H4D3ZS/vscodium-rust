import type { TerminalProfile } from '../../domain/terminal/TerminalProfile';

/** Platform-default shell profiles (VSCode-like). */
export const DEFAULT_PROFILES: TerminalProfile[] = [
    {
        id: 'powershell',
        name: 'PowerShell',
        path: 'powershell.exe',
        args: ['-NoLogo'],
        icon: 'terminal-powershell',
        isDefault: true,
        platform: 'win32',
    },
    {
        id: 'pwsh',
        name: 'PowerShell 7',
        path: 'pwsh.exe',
        args: ['-NoLogo'],
        icon: 'terminal-powershell',
        isDefault: false,
        platform: 'win32',
    },
    {
        id: 'cmd',
        name: 'Command Prompt',
        path: 'cmd.exe',
        args: [],
        icon: 'terminal-cmd',
        isDefault: false,
        platform: 'win32',
    },
    {
        id: 'git-bash',
        name: 'Git Bash',
        path: 'C:\\Program Files\\Git\\bin\\bash.exe',
        args: ['--login', '-i'],
        icon: 'terminal-bash',
        isDefault: false,
        platform: 'win32',
    },
    {
        id: 'wsl',
        name: 'WSL',
        path: 'wsl.exe',
        args: [],
        icon: 'terminal-linux',
        isDefault: false,
        platform: 'win32',
    },
    {
        id: 'bash',
        name: 'Bash',
        path: 'bash',
        args: ['-l'],
        icon: 'terminal-bash',
        isDefault: true,
        platform: 'linux',
    },
    {
        id: 'zsh',
        name: 'ZSH',
        path: 'zsh',
        args: ['-l'],
        icon: 'terminal-bash',
        isDefault: false,
        platform: 'linux',
    },
    {
        id: 'zsh-mac',
        name: 'ZSH',
        path: 'zsh',
        args: ['-l'],
        icon: 'terminal-bash',
        isDefault: true,
        platform: 'darwin',
    },
    {
        id: 'bash-mac',
        name: 'Bash',
        path: 'bash',
        args: ['-l'],
        icon: 'terminal-bash',
        isDefault: false,
        platform: 'darwin',
    },
];

export function currentTerminalPlatform(): 'win32' | 'linux' | 'darwin' {
    const p = navigator.platform.toLowerCase();
    if (p.includes('win')) return 'win32';
    if (p.includes('mac')) return 'darwin';
    return 'linux';
}

/** Resolve profile from profile id, full shell path, or executable name. */
export function resolveTerminalProfile(shellOrProfileId?: string): TerminalProfile {
    const platform = currentTerminalPlatform();
    const platformProfiles = DEFAULT_PROFILES.filter((p) => !p.platform || p.platform === platform);
    const fallback = platformProfiles.find((p) => p.isDefault) || platformProfiles[0] || DEFAULT_PROFILES[0];

    if (!shellOrProfileId || !String(shellOrProfileId).trim()) {
        return fallback;
    }

    const key = String(shellOrProfileId).trim();
    const lower = key.toLowerCase();

    const byId = platformProfiles.find((p) => p.id === key);
    if (byId) return byId;

    const byExactPath = platformProfiles.find((p) => p.path.toLowerCase() === lower);
    if (byExactPath) return byExactPath;

    const base = key.split(/[/\\]/).pop() || key;
    const baseLower = base.toLowerCase();

    const byExe = platformProfiles.find((p) => {
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
        platform,
    };
}
