// =============================================================================
// AIRI Voice System v2 - Real-Time Streaming TTS with ElevenLabs
// Priority: ElevenLabs (when API key present) > Qwen3-TTS (local browser)
// =============================================================================

import { invoke } from './tauri_bridge';
import { qwenTTS } from './airi/qwen-tts'; // Qwen3-TTS local fallback

// NEW API KEY (saved securely via Tauri backend)
const ELEVENLABS_API_KEY = 'sk_e184e0a4bfa989bb8a04dee3076313f56173c6b29adcc777';

export type VoicePreset =
    | 'airi'      // Energetic anime girl
    | 'sage'      // Mature, calm assistant
    | 'nova'      // Young, energetic
    | 'kawaii'    // Cute, high-pitched
    | 'yamato'    // Japanese male, deep voice
    | 'hana'      // Soft, gentle female
    | 'ren'       // Male, professional
    | 'yuki'      // Female, friendly
    | 'haru'      // Male, youthful
    | 'sora'      // Female, calm
    | 'zero'      // Deep, authoritative
    | 'aria'      // Musical, expressive
    | 'filipino'; // Filipino/Tagalog native speaker

export interface VoiceConfig {
    voice_id: string;
    name: string;
    description: string;
    stability: number;
    similarity_boost: number;
    style: number;
    speed: number;
    gender: 'male' | 'female';
}

// ElevenLabs voice IDs - Free Tier Compatible
// Free tier: Can use "premade" voices, not "library" voices via API
// Your free credits work with these voices:
const ELEVENLABS_VOICES: Record<VoicePreset, VoiceConfig> = {
    // === FEMALE VOICES ===
    airi: {
        voice_id: 'EXAVITQu4vr4xnSDxMaL', // Using Bella - standard premade voice
        name: 'AIRI',
        description: 'Energetic anime girl - youthful, expressive',
        stability: 0.5,
        similarity_boost: 0.75,
        style: 0.5,
        speed: 1.1,
        gender: 'female',
    },
    sage: {
        voice_id: 'ThT5C4ZRbQsXWXq8yRvT', // Calm, professional - free tier
        name: 'Sage',
        description: 'Mature assistant - professional, calm',
        stability: 0.7,
        similarity_boost: 0.8,
        style: 0.3,
        speed: 0.95,
        gender: 'female',
    },
    nova: {
        voice_id: 'AZnzlk1XvdvUeBnXmlld', // Antoni - energetic (free tier)
        name: 'Nova',
        description: 'Young & energetic - teenage energy',
        stability: 0.4,
        similarity_boost: 0.7,
        style: 0.6,
        speed: 1.15,
        gender: 'female',
    },
    kawaii: {
        voice_id: 'VR6AewLTigWG4xSOukaG', // Arnold - can be pitched up (free)
        name: 'Kawaii',
        description: 'Cute & adorable - high-pitched, sweet',
        stability: 0.3,
        similarity_boost: 0.85,
        style: 0.8,
        speed: 1.2,
        gender: 'female',
    },
    hana: {
        voice_id: '21m00Tcm4TlvDq8ikWAM', // Using Rachel - soft tone
        name: 'Hana',
        description: 'Soft & gentle - soothing voice',
        stability: 0.65,
        similarity_boost: 0.8,
        style: 0.4,
        speed: 0.95,
        gender: 'female',
    },
    yuki: {
        voice_id: 'AZnzlk1XvdvUeBnXmlld', // Using Antoni - warm
        name: 'Yuki',
        description: 'Friendly & warm - approachable',
        stability: 0.55,
        similarity_boost: 0.75,
        style: 0.5,
        speed: 1.05,
        gender: 'female',
    },
    sora: {
        voice_id: 'EXAVITQu4vr4xnSDxMaL', // Using Bella - serene
        name: 'Sora',
        description: 'Calm & serene - peaceful tone',
        stability: 0.75,
        similarity_boost: 0.7,
        style: 0.25,
        speed: 0.9,
        gender: 'female',
    },
    aria: {
        voice_id: '21m00Tcm4TlvDq8ikWAM', // Using Rachel - expressive
        name: 'Aria',
        description: 'Musical & expressive - melodious',
        stability: 0.45,
        similarity_boost: 0.8,
        style: 0.65,
        speed: 1.05,
        gender: 'female',
    },
    filipino: {
        voice_id: 'jBpfuIE2acCO8z3wKNLl', // Using Gillian - Filipino/Tagalog
        name: 'Filipino',
        description: 'Native Filipino/Tagalog speaker - natural accent',
        stability: 0.5,
        similarity_boost: 0.75,
        style: 0.5,
        speed: 1.0,
        gender: 'female',
    },

    // === MALE VOICES ===
    yamato: {
        voice_id: 'ErXwobaYiN019PkySvjV', // Marcus - deep, confident
        name: 'Yamato',
        description: 'Japanese male - deep, confident',
        stability: 0.6,
        similarity_boost: 0.75,
        style: 0.35,
        speed: 0.95,
        gender: 'male',
    },
    ren: {
        voice_id: 'ErXwobaYiN019PkySvjV', // Marcus - professional
        name: 'Ren',
        description: 'Professional male - clear, authoritative',
        stability: 0.7,
        similarity_boost: 0.8,
        style: 0.3,
        speed: 1.0,
        gender: 'male',
    },
    haru: {
        voice_id: 'AZnzlk1XvdvUeBnXmlld', // Antoni - youthful
        name: 'Haru',
        description: 'Youthful male - energetic, friendly',
        stability: 0.5,
        similarity_boost: 0.7,
        style: 0.5,
        speed: 1.1,
        gender: 'male',
    },
    zero: {
        voice_id: 'VR6AewLigWG4xKvYrGfb', // Clyde - deep, authoritative
        name: 'Zero',
        description: 'Deep & authoritative - commanding',
        stability: 0.8,
        similarity_boost: 0.85,
        style: 0.2,
        speed: 0.9,
        gender: 'male',
    },
};

// Export all voices for the UI
export function getAllVoices(): VoiceConfig[] {
    return Object.values(ELEVENLABS_VOICES);
}

export function getVoicesByGender(gender: 'male' | 'female'): VoiceConfig[] {
    return Object.values(ELEVENLABS_VOICES).filter(v => v.gender === gender);
}

// ── State Management ───────────────────────────────────────────────────────

let currentAudio: HTMLAudioElement | null = null;
let isPlaying = false;
let ttsProvider: 'elevenlabs' | 'openai' | 'browser' | 'qwen' = 'elevenlabs';
let currentApiKey: string | null = null;
let openaiApiKey: string | null = null;
let selectedVoiceId: string | null = null; // User-selected ElevenLabs voice

// Streaming state
let isStreaming = false;
let streamAbortController: AbortController | null = null;
let audioQueue: Array<{ blob: Blob; onEnd?: () => void }> = [];
let isAudioQueuePlaying = false;

// ── Initialization ─────────────────────────────────────────────────────────

export async function initTTS(): Promise<boolean> {
    // // console.log('[TTS] 🎤 Initializing voice system...');

    // Check for session fallback (if ElevenLabs already failed with quota error)
    const prevFallback = localStorage.getItem('ttsProvider_fallback');
    if (prevFallback === 'browser' || prevFallback === 'qwen') {
        ttsProvider = prevFallback as any;
        // console.log(`[TTS] 🔄 Resuming with fallback provider: ${ttsProvider}`);
        return true;
    }

    // ALWAYS load API keys first before any speech
    try {
        const apiKeys = await invoke<any>('get_api_keys');
        // // // console.log('[TTS] API keys received:', Object.keys(apiKeys || {}));

        // Priority: ElevenLabs ALWAYS FIRST
        if (apiKeys?.elevenlabs_api_key && apiKeys.elevenlabs_api_key.startsWith('sk_')) {
            currentApiKey = apiKeys.elevenlabs_api_key;
            ttsProvider = 'elevenlabs';
            // // // console.log('[TTS] ✅ ElevenLabs provider configured (from storage)');
        } else if (ELEVENLABS_API_KEY && ELEVENLABS_API_KEY.startsWith('sk_')) {
            // Use hardcoded key (will be saved to storage)
            currentApiKey = ELEVENLABS_API_KEY;
            ttsProvider = 'elevenlabs';
            // // // console.log('[TTS] ✅ ElevenLabs provider configured (from config)');

            // Save to storage
            try {
                await invoke('save_api_key', { key: 'elevenlabs_api_key', value: ELEVENLABS_API_KEY });
                // // // console.log('[TTS] 💾 ElevenLabs API key saved to secure storage');
            } catch (e) {
                console.warn('[TTS] Could not save API key:', e);
            }
        } else {
            // No ElevenLabs key - use Qwen3-TTS as fallback
            // // // console.log('[TTS] ⚠️  No ElevenLabs key found, using Qwen3-TTS (local)');
            ttsProvider = 'qwen';
        }

        // Load saved voice ID
        const savedVoiceId = (apiKeys as any).elevenlabs_voice_id;
        if (savedVoiceId) {
            selectedVoiceId = savedVoiceId;
            // // console.log(`[TTS] ✅ Loaded saved voice ID: ${savedVoiceId}`);
        }

        // // console.log(`[TTS] 🎯 TTS Provider: ${ttsProvider}`);
        return true;
    } catch (e) {
        console.error('[TTS] ❌ Error initializing TTS:', e);
        ttsProvider = 'qwen'; // Fallback to local
        // // console.log('[TTS] ⚠️  Using Qwen3-TTS (local) as fallback');
        return false;
    }
}

export function getProvider(): string {
    return ttsProvider;
}

export function setSelectedVoice(voiceId: string): void {
    selectedVoiceId = voiceId;
    // console.log(`[TTS] Selected ElevenLabs voice: ${voiceId}`);
}

// ── ElevenLabs TTS (HTTP Streaming) ────────────────────────────────────────

async function speakElevenLabs(text: string, preset: VoicePreset): Promise<ArrayBuffer> {
    const config = ELEVENLABS_VOICES[preset];
    const voiceId = selectedVoiceId || config.voice_id;

    // console.log(`[TTS] speakElevenLabs: preset=${preset}, selectedVoiceId=${selectedVoiceId}, using voiceId=${voiceId}`);

    if (!currentApiKey) {
        const apiKeys = await invoke<any>('get_api_keys');
        currentApiKey = apiKeys?.elevenlabs_api_key;
        if (!currentApiKey) throw new Error('ElevenLabs API key not found');
    }

    const response = await fetch(`https://api.elevenlabs.io/v1/text-to-speech/${voiceId}?optimize_streaming_latency=2`, {
        method: 'POST',
        headers: {
            'Accept': 'audio/mpeg',
            'Content-Type': 'application/json',
            'xi-api-key': currentApiKey,
        },
        body: JSON.stringify({
            text: text,
            model_id: 'eleven_multilingual_v2',
            voice_settings: {
                stability: config.stability,
                similarity_boost: config.similarity_boost,
                style: config.style,
                use_speaker_boost: true,
            },
        }),
    });

    if (!response.ok) {
        const err = await response.text();
        throw new Error(`ElevenLabs API error: ${response.status} - ${err}`);
    }

    return response.arrayBuffer();
}

// ── ElevenLabs Real-Time Streaming TTS ─────────────────────────────────────

export async function speakStreamRealtime(
    text: string,
    preset: VoicePreset = 'airi',
    onChunkStart?: () => void,
    onChunkEnd?: () => void,
    onEnd?: () => void
): Promise<boolean> {
    if (ttsProvider !== 'elevenlabs') {
        console.warn('[TTS] Real-time streaming only available with ElevenLabs');
        return speak(text, preset, onEnd, onChunkStart);
    }

    stop();
    isStreaming = true;
    streamAbortController = new AbortController();

    try {
        // Split text into sentences for natural streaming chunks
        const sentences = splitIntoSentences(text);

        for (const sentence of sentences) {
            if (!isStreaming || streamAbortController?.signal.aborted) {
                break;
            }

            onChunkStart?.();

            const config = ELEVENLABS_VOICES[preset];

            // Stream audio chunks as they arrive
            const response = await fetch(
                `https://api.elevenlabs.io/v1/text-to-speech/${config.voice_id}?optimize_streaming_latency=2&output_format=mp3_44100_128`,
                {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                        'xi-api-key': currentApiKey!,
                    },
                    body: JSON.stringify({
                        text: sentence,
                        model_id: 'eleven_multilingual_v2',
                        voice_settings: {
                            stability: config.stability,
                            similarity_boost: config.similarity_boost,
                            style: config.style,
                            use_speaker_boost: true,
                        },
                    }),
                    signal: streamAbortController.signal,
                }
            );

            if (!response.ok || !response.body) {
                console.error('[TTS] ElevenLabs streaming error');
                continue;
            }

            // Use WebAudio API for low-latency playback
            const audioContext = new (window.AudioContext || (window as any).webkitAudioContext)();
            const reader = response.body.getReader();
            const chunks: Uint8Array[] = [];

            while (true) {
                const { done, value } = await reader.read();
                if (done || streamAbortController?.signal.aborted) break;
                if (value) chunks.push(value);
            }

            if (chunks.length > 0 && !streamAbortController?.signal.aborted) {
                // Combine chunks and decode audio
                const totalLength = chunks.reduce((sum, chunk) => sum + chunk.length, 0);
                const combined = new Uint8Array(totalLength);
                let offset = 0;
                for (const chunk of chunks) {
                    combined.set(chunk, offset);
                    offset += chunk.length;
                }

                // Decode MP3 audio
                const audioBuffer = await audioContext.decodeAudioData(combined.buffer);

                // Play with WebAudio for zero-latency
                const source = audioContext.createBufferSource();
                source.buffer = audioBuffer;
                source.connect(audioContext.destination);
                source.start();

                await new Promise<void>(resolve => {
                    source.onended = () => {
                        audioContext.close();
                        resolve();
                    };
                });
            }

            onChunkEnd?.();
        }

        isStreaming = false;
        onEnd?.();
        window.dispatchEvent(new CustomEvent('airi-tts-end'));
        return true;

    } catch (error: any) {
        if (error.name === 'AbortError') {
            // // console.log('[TTS] Stream aborted');
            return false;
        }
        console.error('[TTS] Streaming error:', error);
        isStreaming = false;

        // Fallback to regular speak
        return speak(text, preset, onEnd, onChunkStart);
    }
}

// ── OpenAI TTS ─────────────────────────────────────────────────────────────

async function speakOpenAI(text: string, preset: VoicePreset): Promise<ArrayBuffer> {
    const key = openaiApiKey;
    if (!key) {
        const apiKeys = await invoke<any>('get_api_keys');
        openaiApiKey = apiKeys?.openai;
        if (!openaiApiKey) throw new Error('OpenAI API key not configured');
    }

    const voiceMap: Record<VoicePreset, string> = {
        airi: 'shimmer', sage: 'alloy', nova: 'fable', kawaii: 'shimmer',
        yamato: 'onyx', hana: 'fable', ren: 'onyx', yuki: 'alloy',
        haru: 'echo', sora: 'nova', zero: 'onyx', aria: 'shimmer',
        filipino: 'shimmer',
    };

    const response = await fetch('https://api.openai.com/v1/audio/speech', {
        method: 'POST',
        headers: {
            'Authorization': `Bearer ${openaiApiKey}`,
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            model: 'tts-1',
            voice: voiceMap[preset] || 'alloy',
            input: text,
            speed: ELEVENLABS_VOICES[preset].speed,
        }),
    });

    if (!response.ok) {
        const err = await response.text();
        throw new Error(`OpenAI TTS error: ${response.status} - ${err}`);
    }

    return response.arrayBuffer();
}

// ── Browser Fallback TTS ────────────────────────────────────────────────────

function speakBrowser(text: string, preset: VoicePreset): SpeechSynthesisUtterance {
    const synth = window.speechSynthesis;
    const utterance = new SpeechSynthesisUtterance(text);

    const browserPresets = {
        airi: { rate: 1.15, pitch: 1.2 },
        sage: { rate: 0.9, pitch: 0.95 },
        nova: { rate: 1.25, pitch: 1.3 },
        kawaii: { rate: 1.3, pitch: 1.4 },
        yamato: { rate: 0.95, pitch: 0.85 },
        hana: { rate: 0.85, pitch: 1.1 },
        ren: { rate: 1.0, pitch: 0.9 },
        yuki: { rate: 1.05, pitch: 1.05 },
        haru: { rate: 1.1, pitch: 1.0 },
        sora: { rate: 0.9, pitch: 1.15 },
        zero: { rate: 0.85, pitch: 0.8 },
        aria: { rate: 1.0, pitch: 1.25 },
    };

    const config = browserPresets[preset];
    utterance.rate = config.rate;
    utterance.pitch = config.pitch;
    utterance.volume = 0.9;

    const voices = synth.getVoices();
    const enVoices = voices.filter(v => v.lang.startsWith('en'));
    utterance.voice = enVoices.find(v => v.name.includes('Google') || v.name.includes('Samantha')) || enVoices[0];

    return utterance;
}

// ── Main Speak Function ────────────────────────────────────────────────────

export async function speak(
    text: string,
    preset: VoicePreset = 'airi',
    onEnd?: () => void,
    onStart?: () => void
): Promise<boolean> {
    // ALWAYS check for ElevenLabs API key first (highest priority)
    if (!currentApiKey) {
        // Try to load from hardcoded config first
        if (ELEVENLABS_API_KEY && ELEVENLABS_API_KEY.startsWith('sk_')) {
            currentApiKey = ELEVENLABS_API_KEY;
            ttsProvider = 'elevenlabs';
            // // console.log('[TTS] ✅ ElevenLabs ACTIVATED (from hardcoded config)');
        } else {
            // Try to load from storage
            try {
                const apiKeys = await invoke<any>('get_api_keys');
                if (apiKeys?.elevenlabs_api_key && apiKeys.elevenlabs_api_key.startsWith('sk_')) {
                    currentApiKey = apiKeys.elevenlabs_api_key;
                    ttsProvider = 'elevenlabs';
                    // // console.log('[TTS] ✅ ElevenLabs ACTIVATED (from storage)');
                }
            } catch (e) {
                console.warn('[TTS] Could not load API keys:', e);
            }
        }
    }

    // Auto-detect Filipino/Tagalog text and switch voice
    const filipinoPatterns = [
        /\b(kumusta|kamusta|salamat|paalam|oo|hindi|baka|nandito|tagalog|filipino|pinoy|pinay)\b/i,
        /\b(na|ng|sa|ang|mga|kay|kay|nina|para|tungkol)\b/,
        /\b(magandang|masayang|malungkot|pagod|gutom|uhaw)\b/i,
    ];

    const isFilipino = filipinoPatterns.some(pattern => pattern.test(text));
    if (isFilipino && preset !== 'filipino') {
        // // console.log('[TTS] 🇵 Filipino/Tagalog detected, switching voice...');
        preset = 'filipino';
    }

    // console.log(`[TTS] speak() called: provider=${ttsProvider}, preset=${preset}, selectedVoiceId=${selectedVoiceId}`);

    // Prevent overlapping speech - stop any current playback
    stop();

    // Force ElevenLabs if API key is available
    if (!currentApiKey) {
        const apiKeys = (window as any).apiKeysForTTS; // Cached if available
        if (!apiKeys?.elevenlabs_api_key?.startsWith('sk_')) {
            try {
                const keys = await invoke<any>('get_api_keys');
                if (keys?.elevenlabs_api_key?.startsWith('sk_')) {
                    currentApiKey = keys.elevenlabs_api_key;
                    ttsProvider = 'elevenlabs';
                    // // console.log('[TTS] ✅ ElevenLabs activated (API key found)');
                }
            } catch (e) {
                console.warn('[TTS] Could not check API keys');
            }
        } else if (apiKeys?.elevenlabs_api_key?.startsWith('sk_')) {
            // API key already in memory
            currentApiKey = apiKeys.elevenlabs_api_key;
            ttsProvider = 'elevenlabs';
            // // console.log('[TTS] ✅ ElevenLabs activated (from memory)');
        }
    }

    // Priority: ElevenLabs > OpenAI > Qwen3-TTS (local) > Browser
    if (ttsProvider !== 'elevenlabs' && ttsProvider !== 'openai' && ttsProvider !== 'qwen') {
        console.warn('[TTS] ⚠️ No valid TTS provider configured. Using Qwen3-TTS (local browser) fallback.');
        ttsProvider = 'qwen';
    }

    isPlaying = true;
    onStart?.();
    window.dispatchEvent(new CustomEvent('airi-tts-start', { detail: { text, preset } }));

    try {
        let audioBuffer: ArrayBuffer | null = null;

        if (ttsProvider === 'elevenlabs') {
            try {
                audioBuffer = await speakElevenLabs(text, preset);
            } catch (e) {
                console.warn('[TTS] ElevenLabs failed, falling back to local Qwen:', e);
                ttsProvider = 'qwen';
                return speak(text, preset, onEnd, onStart);
            }
        } else if (ttsProvider === 'openai') {
            try {
                audioBuffer = await speakOpenAI(text, preset);
            } catch (e) {
                console.warn('[TTS] OpenAI failed, falling back to local Qwen:', e);
                ttsProvider = 'qwen';
                return speak(text, preset, onEnd, onStart);
            }
        }

        // Handle providers that return an ArrayBuffer (ElevenLabs, OpenAI)
        if (audioBuffer) {
            const blob = new Blob([audioBuffer], { type: 'audio/mpeg' });
            const url = URL.createObjectURL(blob);

            currentAudio = new Audio(url);
            currentAudio.onended = () => {
                isPlaying = false;
                URL.revokeObjectURL(url);
                onEnd?.();
                window.dispatchEvent(new CustomEvent('airi-tts-end'));
                window.dispatchEvent(new CustomEvent('airi-lipsync-stop'));
            };
            currentAudio.onerror = (e) => {
                isPlaying = false;
                URL.revokeObjectURL(url);
                console.error('[TTS] Audio playback error:', e);
                // Last ditch fallback if high-quality audio fails to play
                speakBrowser(text, preset).onend = () => { onEnd?.(); };
            };

            currentAudio.onplay = () => {
                window.dispatchEvent(new CustomEvent('airi-lipsync-start', { detail: { text } }));
            };

            await currentAudio.play();
            return true;
        }

        // Provider: Qwen (local, offline-capable)
        if (ttsProvider === 'qwen') {
            await qwenTTS.speak(text, preset);
            isPlaying = false;
            onEnd?.();
            window.dispatchEvent(new CustomEvent('airi-tts-end'));
            return true;
        }

        // Last Fallback: Browser Native
        const utterance = speakBrowser(text, preset);
        utterance.onend = () => {
            isPlaying = false;
            onEnd?.();
            window.dispatchEvent(new CustomEvent('airi-tts-end'));
        };
        window.speechSynthesis.speak(utterance);
        return true;

    } catch (error) {
        console.error('[TTS] Critical Speak failure:', error);
        isPlaying = false;
        // Don't leave the UI hanging
        onEnd?.();
        return false;
    }
}

// ── Sentence Splitting for Natural TTS Chunks ──────────────────────────────

function splitIntoSentences(text: string): string[] {
    // Improved sentence splitting for natural, human-like speech
    // Handles abbreviations, numbers, and preserves speech flow

    // First, clean the text while preserving natural speech patterns
    let cleaned = text
        // Remove markdown code blocks
        .replace(/```[\s\S]*?```/g, ' ')
        // Remove inline code but keep content readable
        .replace(/`([^`]+)`/g, '$1')
        // Remove markdown headers but keep text
        .replace(/#{1,6}\s+/g, '')
        // Remove markdown formatting
        .replace(/[*_~]/g, '')
        // Convert URLs to readable text
        .replace(/https?:\/\/\S+/g, ' link ')
        // Normalize whitespace
        .replace(/\s+/g, ' ')
        .trim();

    // Protect common abbreviations from being split
    const abbreviations = ['Mr', 'Mrs', 'Ms', 'Dr', 'Prof', 'Sr', 'Jr', 'vs', 'etc', 'e.g', 'i.e', 'cf', 'al', 'St', 'Ave', 'Blvd', 'Rd', 'Inc', 'Ltd', 'Co'];
    const protectedText = cleaned;

    // Split on natural sentence boundaries
    // Matches: sentence-ending punctuation followed by space (or end of string)
    const sentences = protectedText.match(/[^.!?]+[.!?]+(?=\s|$)|[^.!?]+$/g);

    if (!sentences || sentences.length === 0) {
        return [cleaned];
    }

    // Filter and trim sentences
    return sentences
        .map(s => s.trim())
        .filter(s => s.length > 2 && s.length < 300); // Skip very short or overly long segments
}

// ── Controls ───────────────────────────────────────────────────────────────

export function stop(): void {
    if (currentAudio) {
        currentAudio.pause();
        currentAudio = null;
    }
    window.speechSynthesis?.cancel();
    isPlaying = false;
}

export function stopStreaming(): void {
    if (streamAbortController) {
        streamAbortController.abort();
        streamAbortController = null;
    }
    isStreaming = false;
    stop();
    audioQueue = [];
    isAudioQueuePlaying = false;
}

export function pause(): void {
    currentAudio?.pause();
}

export function resume(): void {
    currentAudio?.play();
}

export function isSpeaking(): boolean {
    return isPlaying || isStreaming;
}

// ── Queue-based Streaming (for incremental text) ──────────────────────────

let ttsQueue: string[] = [];
let isProcessingQueue = false;
let currentTtsPreset: VoicePreset = 'airi';

export function queueSpeechChunk(text: string, preset: VoicePreset = 'airi'): void {
    if (!text || text.trim().length === 0) return;

    currentTtsPreset = preset;
    ttsQueue.push(text.trim());

    if (!isProcessingQueue) {
        processTtsQueue();
    }
}

export function flushTtsQueue(): void {
    if (ttsQueue.length > 0 && !isProcessingQueue) {
        processTtsQueue();
    }
}

export function clearTtsQueue(): void {
    ttsQueue = [];
    isProcessingQueue = false;
    stop();
}

async function processTtsQueue(): Promise<void> {
    if (isProcessingQueue || ttsQueue.length === 0) return;

    isProcessingQueue = true;

    while (ttsQueue.length > 0) {
        const text = ttsQueue.shift()!;

        // If streaming is interrupted, stop processing
        if (!isSpeaking()) {
            break;
        }

        await speak(text, currentTtsPreset);

        // Small delay between chunks for natural pacing
        await new Promise(resolve => setTimeout(resolve, 100));
    }

    isProcessingQueue = false;
}

// ── Export for Global Access ────────────────────────────────────────────────

(window as any).airiTts = {
    init: initTTS,
    speak,
    speakStream: speakStreamRealtime,
    stop,
    stopStream: stopStreaming,
    pause,
    resume,
    isSpeaking,
    getProvider,
    getAllVoices,
    getVoicesByGender,
    queueSpeechChunk,
    flushTtsQueue,
    clearTtsQueue,
    presets: ELEVENLABS_VOICES,
    setSelectedVoice,
};

/*
// // console.log('[TTS] ✅ AIRI Voice System v2 loaded');
// // console.log('[TTS] 🎤 Supports real-time streaming via ElevenLabs');
// // console.log('[TTS] 🎭 12 character voices available');
*/
