/**
 * AIRI Voice Manager - Fixed Overlap Issues
 * Prevents multiple voices speaking at once
 * Ensures TTS only works after proper initialization
 * Manages voice state across all AIRI systems
 */

import { airiBiology } from './biology';
import { airiConsciousness } from './consciousness';
import { qwenTTS } from './qwen-tts'; // Free Qwen3-TTS fallback

// Voice state
let isInitialized = false;
let isSpeaking = false;
let speechQueue: SpeechRequest[] = [];
let currentRequest: SpeechRequest | null = null;
let voiceLock = false; // Prevent overlapping

interface SpeechRequest {
  id: string;
  text: string;
  preset: string;
  priority: number; // 1-10, higher = more urgent
  timestamp: number;
  onStart?: () => void;
  onEnd?: () => void;
}

/**
 * Initialize voice system (call once on startup)
 */
export async function initializeVoice(): Promise<boolean> {
  if (isInitialized) {
    console.log('[VoiceManager] ✅ Already initialized');
    return true;
  }

  console.log('[VoiceManager] 🎤 Initializing voice system...');

  try {
    // Use existing ElevenLabs voice system from src/voice.ts
    const { initTTS } = await import('../voice');
    const initialized = await initTTS();
    
    if (initialized) {
      isInitialized = true;
      console.log('[VoiceManager] ✅ Voice system initialized');
      
      // Start queue processor
      processSpeechQueue();
      
      return true;
    } else {
      console.error('[VoiceManager] ❌ TTS initialization failed');
      return false;
    }
  } catch (error) {
    console.error('[VoiceManager] ❌ Initialization error:', error);
    return false;
  }
}

/**
 * Check if voice system is ready
 */
export function isVoiceReady(): boolean {
  return isInitialized;
}

/**
 * Speak with proper queue management (prevents overlap)
 */
export async function speak(
  text: string,
  preset: string = 'airi',
  priority: number = 5,
  onEnd?: () => void
): Promise<boolean> {
  if (!isInitialized) {
    console.warn('[VoiceManager] ⚠️ Voice not initialized, queuing request');
    // Queue the request for when initialization completes
    queueSpeech(text, preset, priority, onEnd);
    return false;
  }

  // Don't speak if AIRI is sleeping
  const biology = airiBiology.getState();
  if (biology.isSleeping) {
    console.log('[VoiceManager] 😴 AIRI is sleeping, skipping speech');
    onEnd?.();
    return false;
  }

  // Create speech request
  const request: SpeechRequest = {
    id: `speech_${Date.now()}_${Math.random()}`,
    text,
    preset,
    priority,
    timestamp: Date.now(),
    onEnd
  };

  // Add to queue
  queueSpeechRequest(request);

  return true;
}

/**
 * Queue speech request
 */
function queueSpeechRequest(request: SpeechRequest): void {
  speechQueue.push(request);
  
  // Sort by priority (higher priority first)
  speechQueue.sort((a, b) => b.priority - a.priority);
  
  console.log(`[VoiceManager] 📝 Queued: "${request.text.substring(0, 30)}..." (priority: ${request.priority})`);
  
  // Process queue if not already processing
  if (!voiceLock) {
    processSpeechQueue();
  }
}

/**
 * Queue speech (public API)
 */
function queueSpeech(
  text: string,
  preset: string = 'airi',
  priority: number = 5,
  onEnd?: () => void
): void {
  const request: SpeechRequest = {
    id: `speech_${Date.now()}_${Math.random()}`,
    text,
    preset,
    priority,
    timestamp: Date.now(),
    onEnd
  };
  
  speechQueue.push(request);
}

/**
 * Process speech queue (prevents overlap)
 */
async function processSpeechQueue(): Promise<void> {
  if (voiceLock || speechQueue.length === 0 || !isInitialized) {
    return;
  }

  voiceLock = true;
  currentRequest = speechQueue.shift() || null;

  if (!currentRequest) {
    voiceLock = false;
    return;
  }

  try {
    console.log(`[VoiceManager] 🎤 Speaking: "${currentRequest.text.substring(0, 50)}..."`);
    isSpeaking = true;

    // Load voice module
    const { speak } = await import('../voice');

    // Speak with callbacks
    const success = await speak(
      currentRequest.text,
      currentRequest.preset as any,
      () => {
        // On end callback
        console.log('[VoiceManager] ✅ Speech completed');
        isSpeaking = false;
        voiceLock = false;
        currentRequest?.onEnd?.();
        
        // Process next in queue
        setTimeout(() => processSpeechQueue(), 100);
      },
      () => {
        // On start callback
        console.log('[VoiceManager] ▶️ Speech started');
      }
    );

    if (!success) {
      console.warn('[VoiceManager] ⚠️ Speech failed');
      isSpeaking = false;
      voiceLock = false;
      currentRequest?.onEnd?.();
      
      // Try next in queue
      setTimeout(() => processSpeechQueue(), 100);
    }

  } catch (error) {
    console.error('[VoiceManager] ❌ Speech error:', error);
    isSpeaking = false;
    voiceLock = false;
    currentRequest?.onEnd?.();
    
    // Try next in queue
    setTimeout(() => processSpeechQueue(), 100);
  }
}

/**
 * Stop all speech immediately
 */
export async function stopSpeech(): Promise<void> {
  if (!isInitialized) return;

  console.log('[VoiceManager] ⏹️ Stopping all speech');

  try {
    const { stop, clearTtsQueue } = await import('../voice');
    stop();
    clearTtsQueue();

    isSpeaking = false;
    voiceLock = false;
    speechQueue = [];
    currentRequest = null;
  } catch (error) {
    console.error('[VoiceManager] Stop error:', error);
  }
}

/**
 * Clear speech queue
 */
export function clearSpeechQueue(): void {
  speechQueue = [];
  console.log('[VoiceManager] 🗑️ Queue cleared');
}

/**
 * Get queue status
 */
export function getQueueStatus(): {
  queueLength: number;
  isSpeaking: boolean;
  isLocked: boolean;
  currentRequest: string | null;
} {
  return {
    queueLength: speechQueue.length,
    isSpeaking,
    isLocked: voiceLock,
    currentRequest: currentRequest ? currentRequest.text : null
  };
}

/**
 * Emergency shutdown (on unload)
 */
export async function shutdown(): Promise<void> {
  console.log('[VoiceManager] 🔇 Shutting down');
  await stopSpeech();
  isInitialized = false;
}

// Register shutdown handler
if (typeof window !== 'undefined') {
  window.addEventListener('beforeunload', () => {
    shutdown();
  });
}

console.log('[VoiceManager] ✅ Voice Manager loaded (overlap prevention enabled)');
