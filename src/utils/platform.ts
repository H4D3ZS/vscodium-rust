/**
 * Platform Detection Utility
 * Detects operating system and available emulator types
 */

export interface PlatformInfo {
  os: 'windows' | 'macos' | 'linux';
  arch: 'x64' | 'arm64' | 'x86';
  canRunIOSEmulator: boolean;
  canRunAndroidEmulator: boolean;
}

let cachedPlatform: PlatformInfo | null = null;

/**
 * Detect current platform
 */
export function detectPlatform(): PlatformInfo {
  if (cachedPlatform) {
    return cachedPlatform;
  }

  // Default to Windows for browser context
  let os: 'windows' | 'macos' | 'linux' = 'windows';
  let arch: 'x64' | 'arm64' | 'x86' = 'x64';

  // Browser detection
  if (typeof navigator !== 'undefined') {
    const ua = navigator.userAgent.toLowerCase();
    
    if (ua.includes('win')) {
      os = 'windows';
    } else if (ua.includes('mac')) {
      os = 'macos';
    } else if (ua.includes('linux')) {
      os = 'linux';
    }

    // Detect architecture
    if (ua.includes('arm') || ua.includes('aarch64')) {
      arch = 'arm64';
    } else if (ua.includes('x64') || ua.includes('x86_64')) {
      arch = 'x64';
    } else if (ua.includes('x86')) {
      arch = 'x86';
    }
  }

  // Determine emulator capabilities
  const canRunIOSEmulator = (os === 'macos'); // iOS simulators require macOS + Xcode
  const canRunAndroidEmulator = true; // All platforms can run Android emulators

  cachedPlatform = {
    os,
    arch,
    canRunIOSEmulator,
    canRunAndroidEmulator,
  };

  console.log(`[Platform] Detected: ${os} ${arch}, iOS: ${canRunIOSEmulator}, Android: ${canRunAndroidEmulator}`);
  
  return cachedPlatform;
}

/**
 * Check if running on macOS
 */
export function isMacOS(): boolean {
  return detectPlatform().os === 'macos';
}

/**
 * Check if running on Windows
 */
export function isWindows(): boolean {
  return detectPlatform().os === 'windows';
}

/**
 * Check if running on Linux
 */
export function isLinux(): boolean {
  return detectPlatform().os === 'linux';
}

/**
 * Check if iOS emulator is available
 */
export function canUseIOSEmulator(): boolean {
  return detectPlatform().canRunIOSEmulator;
}

/**
 * Check if Android emulator is available
 */
export function canUseAndroidEmulator(): boolean {
  return detectPlatform().canRunAndroidEmulator;
}

/**
 * Get platform summary for UI
 */
export function getPlatformSummary(): string {
  const platform = detectPlatform();
  const emulators: string[] = [];
  
  if (platform.canRunAndroidEmulator) {
    emulators.push('Android');
  }
  if (platform.canRunIOSEmulator) {
    emulators.push('iOS');
  }
  
  return `${platform.os} (${platform.arch}) - ${emulators.join(' + ')} emulators`;
}
