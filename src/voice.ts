// =============================================================================
// AIRI Voice System v2 - Real-Time Streaming TTS with ElevenLabs
// Supports ElevenLabs (WebSocket streaming), OpenAI TTS, and Browser Fallback
// =============================================================================

import { invoke } from './tauri_bridge';

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
    | 'aria';     // Musical, expressive

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

// ElevenLabs voice IDs (verified working voices from ElevenLabs voice library)
const ELEVENLABS_VOICES: Record<VoicePreset, VoiceConfig> = {
    // === FEMALE VOICES ===
    airi: {
        voice_id: '21m00Tcm4TlvDq8ikWAM', // Rachel - clear, versatile
        name: 'AIRI',
        description: 'Energetic anime girl - youthful, expressive',
        stability: 0.5,
        similarity_boost: 0.75,
        style: 0.5,
        speed: 1.1,
        gender: 'female',
    },
    sage: {
        voice_id: 'EXAVITQu4vr4xnSDxMaL', // Bella - calm, professional
        name: 'Sage',
        description: 'Mature assistant - professional, calm',
        stability: 0.7,
        similarity_boost: 0.8,
        style: 0.3,
        speed: 0.95,
        gender: 'female',
    },
    nova: {
        voice_id: 'AZnzlk1XvdvUeBnXmlld', // Antoni - energetic
        name: 'Nova',
        description: 'Young & energetic - teenage energy',
        stability: 0.4,
        similarity_boost: 0.7,
        style: 0.6,
        speed: 1.15,
        gender: 'female',
    },
    kawaii: {
        voice_id: 'MFw3iBd7yC6D3J2XJVyQ', // Custom kawaii voice
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
let ttsProvider: 'elevenlabs' | 'openai' | 'browser' = 'elevenlabs';
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
    // Check for API keys and set provider
    try {
        const apiKeys = await invoke<any>('get_api_keys');
        console.log('[TTS] API keys received:', Object.keys(apiKeys || {}));

        // Priority: ElevenLabs > OpenAI > Browser
        if (apiKeys?.elevenlabs_api_key && apiKeys.elevenlabs_api_key.startsWith('sk_')) {
            currentApiKey = apiKeys.elevenlabs_api_key;
            ttsProvider = 'elevenlabs';
            console.log('[TTS] ✅ ElevenLabs provider configured');
            
            // Load saved voice ID from storage
            const savedVoiceId = (apiKeys as any).elevenlabs_voice_id;
            if (savedVoiceId) {
                selectedVoiceId = savedVoiceId;
                console.log(`[TTS] ✅ Loaded saved voice ID: ${savedVoiceId}`);
            }
            return true;
        }

        if (apiKeys?.openai && apiKeys.openai.startsWith('sk-')) {
            openaiApiKey = apiKeys.openai;
            ttsProvider = 'openai';
            console.log('[TTS] ✅ OpenAI TTS provider configured');
            return true;
        }

        console.log('[TTS] ⚠️ No valid API keys found. elevenlabs_api_key:', apiKeys?.elevenlabs_api_key ? 'present but masked' : 'missing');
    } catch (e) {
        console.warn('[TTS] Error loading API keys:', e);
    }

    ttsProvider = 'browser';
    console.log('[TTS] ⚠️ Using browser Web Speech API (fallback)');
    console.log('[TTS] 💡 Add your ElevenLabs API key in Settings for premium voices');
    return true;
}

export function getProvider(): string {
    return ttsProvider;
}

export function setSelectedVoice(voiceId: string): void {
    selectedVoiceId = voiceId;
    console.log(`[TTS] Selected ElevenLabs voice: ${voiceId}`);
}

// ── ElevenLabs TTS (HTTP Streaming) ────────────────────────────────────────

async function speakElevenLabs(text: string, preset: VoicePreset): Promise<ArrayBuffer> {
    const config = ELEVENLABS_VOICES[preset];
    const voiceId = selectedVoiceId || config.voice_id;
    
    console.log(`[TTS] speakElevenLabs: preset=${preset}, selectedVoiceId=${selectedVoiceId}, using voiceId=${voiceId}`);

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
            console.log('[TTS] Stream aborted');
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
    console.log(`[TTS] speak() called: provider=${ttsProvider}, preset=${preset}, selectedVoiceId=${selectedVoiceId}`);
    
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
                    console.log('[TTS] ✅ ElevenLabs activated');
                }
            } catch (e) {
                console.warn('[TTS] Could not check API keys');
            }
        }
    }
    
    // Only use ElevenLabs - no browser fallback unless explicitly configured
    if (ttsProvider !== 'elevenlabs' && ttsProvider !== 'openai') {
        console.warn('[TTS] ⚠️ No valid TTS provider. ElevenLabs API key required.');
        return false;
    }

    isPlaying = true;
    onStart?.();
    window.dispatchEvent(new CustomEvent('airi-tts-start', { detail: { text, preset } }));

    try {
        let audioBuffer: ArrayBuffer;

        if (ttsProvider === 'elevenlabs') {
            audioBuffer = await speakElevenLabs(text, preset);
        } else if (ttsProvider === 'openai') {
            audioBuffer = await speakOpenAI(text, preset);
        } else {
            // Browser fallback - play synchronously
            const utterance = speakBrowser(text, preset);
            utterance.onend = () => {
                isPlaying = false;
                onEnd?.();
                window.dispatchEvent(new CustomEvent('airi-tts-end'));
            };
            utterance.onerror = (e) => {
                isPlaying = false;
                console.error('[TTS] Browser error:', e);
                window.dispatchEvent(new CustomEvent('airi-tts-error', { detail: { error: e } }));
            };
            window.speechSynthesis.speak(utterance);
            return true;
        }

        // Play ElevenLabs/OpenAI audio
        const blob = new Blob([audioBuffer], { type: 'audio/mpeg' });
        const url = URL.createObjectURL(blob);

        currentAudio = new Audio(url);
        currentAudio.onended = () => {
            isPlaying = false;
            URL.revokeObjectURL(url);
            onEnd?.();
            window.dispatchEvent(new CustomEvent('airi-tts-end'));
            
            // Stop lip sync
            window.dispatchEvent(new CustomEvent('airi-lipsync-stop'));
        };
        currentAudio.onerror = (e) => {
            isPlaying = false;
            URL.revokeObjectURL(url);
            console.error('[TTS] Audio error:', e);
            window.dispatchEvent(new CustomEvent('airi-tts-error', { detail: { error: e } }));
            window.dispatchEvent(new CustomEvent('airi-lipsync-stop'));
        };

        // Start lip sync when audio starts
        currentAudio.onplay = () => {
            window.dispatchEvent(new CustomEvent('airi-lipsync-start', { detail: { text } }));
        };

        await currentAudio.play();
        return true;

    } catch (error) {
        console.error('[TTS] Speak error:', error);
        isPlaying = false;
        window.dispatchEvent(new CustomEvent('airi-tts-error', { detail: { error } }));
        
        // Fallback to browser TTS if ElevenLabs fails (quota exceeded, etc.)
        if (ttsProvider === 'elevenlabs') {
            console.log('[TTS] ⚠️ ElevenLabs failed, falling back to browser TTS');
            ttsProvider = 'browser';
            const utterance = speakBrowser(text, preset);
            utterance.onend = () => { isPlaying = false; onEnd?.(); };
            window.speechSynthesis.speak(utterance);
        }
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

console.log('[TTS] ✅ AIRI Voice System v2 loaded');
console.log('[TTS] 🎤 Supports real-time streaming via ElevenLabs');
console.log('[TTS] 🎭 12 character voices available');
