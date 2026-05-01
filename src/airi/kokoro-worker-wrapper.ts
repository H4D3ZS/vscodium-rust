/**
 * Kokoro TTS Worker Wrapper - Silences expected errors
 */

// Suppress Kokoro worker errors (expected when not configured)
const originalError = console.error;
console.error = function(...args: any[]) {
  const msg = args.join(' ');
  // Skip expected Kokoro errors
  if (msg.includes('KokoroWorker') || 
      msg.includes('Failed to fetch Kokoro voices') ||
      msg.includes('duckdb worker') ||
      msg.includes('Empty color reference')) {
    return; // Suppress expected errors
  }
  originalError.apply(console, args);
};