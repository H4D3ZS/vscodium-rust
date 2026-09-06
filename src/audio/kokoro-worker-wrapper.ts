/**
 * Kokoro TTS Worker Wrapper - Silences expected errors and patches
 * known-broken third-party hot spots before any other module imports them.
 */

const originalError = console.error;
console.error = function(...args: any[]) {
  const msg = args.join(' ');
  if (msg.includes('KokoroWorker') ||
      msg.includes('Failed to fetch Kokoro voices') ||
      msg.includes('duckdb worker') ||
      msg.includes('Empty color reference') ||
      msg.includes("THREE.GLTFLoader: Couldn't load texture blob")) {
    return;
  }
  originalError.apply(console, args);
};

// ---------------------------------------------------------------------------
// `@pixiv/three-vrm` 3.5.2 + Three.js 0.183 mismatch
// ---------------------------------------------------------------------------
// `GLTFMToonMaterialParamsAssignHelper` occasionally hands `undefined` to
// `setTextureColorSpace`, which then crashes with:
//   "Cannot set properties of undefined (setting 'colorSpace')"
//
// Swallow that specific unhandled-rejection so the IDE keeps running. The
// avatar simply renders without that texture instead of bringing the whole
// page down. Other errors still bubble through unchanged.
if (typeof window !== 'undefined') {
  window.addEventListener('unhandledrejection', (ev: PromiseRejectionEvent) => {
    const reason: any = ev.reason;
    const msg = String(reason?.message || reason || '');
    const stack = String(reason?.stack || '');
    if (
      msg.includes("Cannot set properties of undefined (setting 'colorSpace')") ||
      stack.includes('setTextureColorSpace') ||
      stack.includes('GLTFMToonMaterialParamsAssignHelper')
    ) {
      ev.preventDefault();
    }
  });
}