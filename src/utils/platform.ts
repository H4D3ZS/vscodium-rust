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

  // Detect OS from user agent (browser) or process (Node.js/Tauri)
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

  // Tauri/Node.js detection (more accurate)
  if (typeof window !== 'undefined' && (window as any).__TAURI__) {
    try {
      const platform = (window as any).__TAURI__.os?.platform();
      if (platform === 'win32') os = 'windows';
      else if (platform === 'darwin') os = 'macos';
      else if (platform === 'linux') os = 'linux';

      const archType = (window as any).__TAURI__.os?.arch();
      if (archType === 'arm64' || archType === 'aarch64') arch = 'arm64';
      else if (archType === 'x86_64') arch = 'x64';
      else if (archType === 'x86') arch = 'x86';
    } catch (e) {
      // Fallback to browser detection
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
