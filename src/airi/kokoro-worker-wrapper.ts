/**
 * Kokoro TTS Worker Wrapper - Silences expected errors
 */

// Suppress Kokoro and 3D engine errors (expected in some envs)
const originalError = console.error;
const originalWarn = console.warn;

const NOISE_PATTERNS = [
  'KokoroWorker',
  'Failed to fetch Kokoro voices',
  'duckdb worker',
  'Empty color reference',
  'colorSpace',
  'Tracking Prevention',
  'GLTFLoader',
  'Unsafe attempt to load URL',
  'chromewebdata',
  'favicon.ico',
  'no-speech'
];

function isNoise(msg: string): boolean {
  return NOISE_PATTERNS.some(pattern => msg.includes(pattern));
}

console.error = function (...args: any[]) {
  const msg = args.join(' ');
  if (isNoise(msg)) return;
  originalError.apply(console, args);
};

console.warn = function (...args: any[]) {
  const msg = args.join(' ');
  if (msg.includes('Aim cache empty') || isNoise(msg)) return;
  originalWarn.apply(console, args);
};

// Global Catch-all for unhandled rejections (most VRM errors are here)
window.addEventListener('unhandledrejection', (event) => {
  const msg = event.reason?.message || String(event.reason);
  if (isNoise(msg)) {
    event.preventDefault(); // Stop console spam
  }
});

// Global catch-all for window errors
window.addEventListener('error', (event) => {
  const msg = event.message || '';
  if (isNoise(msg)) {
    event.preventDefault(); // Stop console spam
  }
}, true);