/**
 * Qwen3-TTS Integration - Free Local TTS
 * 
 * Uses browser SpeechSynthesis API (no external dependencies)
 * Free, fast, and works offline
 */

export interface QwenTTSConfig {
    speed: number;
    pitch: number;
    volume: number;
    voiceName?: string;
}

export class Qwen3TTS {
    private config: QwenTTSConfig;
    private isSpeaking = false;
    private audioContext: AudioContext | null = null;

    constructor(config?: Partial<QwenTTSConfig>) {
        this.config = {
            speed: config?.speed ?? 1.0,
            pitch: config?.pitch ?? 1.0,
            volume: config?.volume ?? 1.0,
            voiceName: config?.voiceName,
        };
    }

    /**
     * Initialize audio context
     */
    private initAudio(): void {
        if (!this.audioContext) {
            this.audioContext = new (window.AudioContext || (window as any).webkitAudioContext)();
        }
    }

    /**
     * Speak text using browser SpeechSynthesis
     */
    async speak(text: string, emotion?: string): Promise<void> {
        if (this.isSpeaking) {
            await this.stop();
        }


        return new Promise((resolve) => {
            const utterance = new SpeechSynthesisUtterance(text);

            // Configure voice settings
            utterance.rate = this.config.speed;
            utterance.pitch = this.config.pitch;
            utterance.volume = this.config.volume;

            // Apply emotion to voice
            if (emotion) {
                switch (emotion.toLowerCase()) {
                    case 'happy':
                    case 'excited':
                        utterance.pitch = 1.2;
                        utterance.rate = 1.1;
                        break;
                    case 'sad':
                        utterance.pitch = 0.8;
                        utterance.rate = 0.9;
                        break;
                    case 'angry':
                        utterance.pitch = 0.7;
                        utterance.rate = 1.2;
                        break;
                    case 'calm':
                    case 'gentle':
                        utterance.pitch = 1.0;
                        utterance.rate = 0.95;
                        break;
                }
            }

            // Voice selection - prefer English female voice for AIRI
            const voices = window.speechSynthesis.getVoices();
            const preferredVoice = voices.find(v => 
                v.name.includes('Google') || 
                v.name.includes('Female') ||
                v.name.includes('Zira') ||
                v.name.includes('Hazel') ||
                v.lang.startsWith('en')
            );
            
            if (preferredVoice) {
                utterance.voice = preferredVoice;
            }

            utterance.onend = () => {
                this.isSpeaking = false;
                resolve();
            };

            utterance.onerror = (event) => {
                this.isSpeaking = false;
 console.error('[Qwen3-TTS] Speech error:', event.error);
                resolve(); // Resolve anyway
            };

            this.isSpeaking = true;
            window.speechSynthesis.speak(utterance);
            
        });
    }

    /**
     * Stop speaking
     */
    async stop(): Promise<void> {
        window.speechSynthesis.cancel();
        this.isSpeaking = false;
    }

    /**
     * Check if currently speaking
     */
    isSpeakingNow(): boolean {
        return this.isSpeaking || window.speechSynthesis.speaking;
    }

    /**
     * Get available voices
     */
    getVoices(): SpeechSynthesisVoice[] {
        return window.speechSynthesis.getVoices();
    }

    /**
     * Update configuration
     */
    configure(config: Partial<QwenTTSConfig>): void {
        this.config = { ...this.config, ...config };
    }
}

// Export singleton
export const qwenTTS = new Qwen3TTS();

// Make globally accessible
if (typeof window !== 'undefined') {
    (window as any).__QWEN_TTS__ = qwenTTS;
    
    // Load voices on startup (Chrome needs this)
    if ('speechSynthesis' in window) {
        window.speechSynthesis.getVoices();
        // Also load on voiceschanged event
        window.speechSynthesis.onvoiceschanged = () => {
            window.speechSynthesis.getVoices();
        };
    }
}
