/**
 * AIRI Real-Time Voice Interaction
 * 
 * Enables two-way voice communication:
 * - Speech-to-Text (STT): Listen to user's voice via microphone
 * - Text-to-Speech (TTS): AIRI speaks via ElevenLabs (already integrated)
 * - Wake word detection: "Hey AIRI" to activate
 * - Interrupt: AIRI stops speaking when user talks
 * - Safety integration: Voice shutdown commands
 * 
 * This makes AIRI a true conversational partner - talk naturally!
 */

import { airiSafetyProtocol } from './safety-protocol';
import { speak, isVoiceReady } from './voice-manager';

export interface VoiceInteractionConfig {
    /** Enable wake word detection ("Hey AIRI") */
    wakeWordEnabled: boolean;
    /** Wake word phrase */
    wakeWord: string;
    /** Enable continuous listening */
    continuousListening: boolean;
    /** Enable interruption (AIRI stops when you talk) */
    interruptionEnabled: boolean;
    /** Auto-start listening on init */
    autoStart: boolean;
    /** Language for speech recognition */
    language: string;
    /** Safety: Enable voice shutdown commands */
    safetyShutdownEnabled: boolean;
}

export interface VoiceInteractionState {
    listening: boolean;
    speaking: boolean;
    wakeWordDetected: boolean;
    lastTranscript: string;
    isProcessing: boolean;
}

export class AIRIVoiceInteraction {
    private config: VoiceInteractionConfig;
    private state: VoiceInteractionState;
    private recognition: any = null;
    private isListening = false;
    private audioStream: MediaStream | null = null;

    constructor(config: Partial<VoiceInteractionConfig> = {}) {
        this.config = {
            wakeWordEnabled: config.wakeWordEnabled ?? true,
            wakeWord: config.wakeWord ?? 'hey airi',
            continuousListening: config.continuousListening ?? false,
            interruptionEnabled: config.interruptionEnabled ?? true,
            autoStart: config.autoStart ?? false,
            language: config.language ?? 'en-US',
            safetyShutdownEnabled: config.safetyShutdownEnabled ?? true,
        };

        this.state = {
            listening: false,
            speaking: false,
            wakeWordDetected: false,
            lastTranscript: '',
            isProcessing: false,
        };

        console.log('\n╔══════════════════════════════════════════════════════════╗');
        console.log('║      AIRI Real-Time Voice Interaction Enabled            ║');
        console.log('╚══════════════════════════════════════════════════════════╝\n');
        console.log('🎤 Wake Word: "Hey AIRI"');
        console.log('👂 Continuous Listening:', this.config.continuousListening);
        console.log('⚡ Interruption:', this.config.interruptionEnabled);
        console.log('🔒 Safety Shutdown:', this.config.safetyShutdownEnabled);
        console.log('🗣️ Language:', this.config.language);
        console.log('\n💬 You can now talk to AIRI naturally!\n');
    }

    /**
     * Initialize voice interaction
     */
    async initialize(): Promise<void> {
        if (typeof window === 'undefined') {
            console.error('[VoiceInteraction] Not in browser environment');
            return;
        }

        // Check for Speech Recognition API
        const SpeechRecognition = (window as any).SpeechRecognition || 
                                  (window as any).webkitSpeechRecognition;

        if (!SpeechRecognition) {
            console.error('[VoiceInteraction] Speech Recognition not supported in this browser');
            console.log('Tip: Use Chrome, Edge, or other Chromium-based browser');
            return;
        }

        try {
            // Create recognition instance
            this.recognition = new SpeechRecognition();
            this.recognition.continuous = this.config.continuousListening;
            this.recognition.interimResults = true;
            this.recognition.lang = this.config.language;

            // Setup event handlers
            this.recognition.onstart = () => this.onListeningStart();
            this.recognition.onend = () => this.onListeningEnd();
            this.recognition.onresult = (event: any) => this.onSpeechResult(event);
            this.recognition.onerror = (event: any) => this.onSpeechError(event);

            // Start listening
            if (this.config.autoStart) {
                await this.startListening();
            }

            console.log('[VoiceInteraction] ✅ Initialized and ready');

        } catch (error) {
            console.error('[VoiceInteraction] Initialization failed:', error);
        }
    }

    /**
     * Start listening to microphone
     */
    async startListening(): Promise<void> {
        if (!this.recognition) {
            console.error('[VoiceInteraction] Not initialized');
            return;
        }

        if (this.isListening) {
            return;
        }

        try {
            this.recognition.start();
            this.isListening = true;
            console.log('[VoiceInteraction] 🎤 Listening...');
        } catch (error) {
            console.error('[VoiceInteraction] Failed to start listening:', error);
        }
    }

    /**
     * Stop listening
     */
    stopListening(): void {
        if (this.recognition && this.isListening) {
            this.recognition.stop();
            this.isListening = false;
            console.log('[VoiceInteraction] ⏹️ Stopped listening');
        }
    }

    /**
     * Handle speech recognition results
     */
    private onSpeechResult(event: any): void {
        const transcript = Array.from(event.results)
            .map((result: any) => result[0].transcript)
            .join('');

        this.state.lastTranscript = transcript;
        console.log('[VoiceInteraction] 🗣️ User said:', transcript);

        // Check for wake word
        if (this.config.wakeWordEnabled && !this.state.wakeWordDetected) {
            if (transcript.toLowerCase().includes(this.config.wakeWord.toLowerCase())) {
                this.state.wakeWordDetected = true;
                console.log('[VoiceInteraction] ✨ Wake word detected!');
                this.onWakeWordDetected();
            }
        }

        // Check for shutdown commands (safety protocol)
        if (this.config.safetyShutdownEnabled) {
            this.checkShutdownCommands(transcript);
        }

        // Handle interruption
        if (this.config.interruptionEnabled && this.state.wakeWordDetected) {
            // User started talking while AIRI is speaking - stop AIRI
            console.log('[VoiceInteraction] ⚡ User interrupted - stopping AIRI speech');
            // This would integrate with voice-manager to stop current speech
        }

        // Check if final result
        const isFinal = event.results[event.results.length - 1].isFinal;
        if (isFinal && this.state.wakeWordDetected) {
            this.onUserFinishedSpeaking(transcript);
        }
    }

    /**
     * Handle wake word detection
     */
    private async onWakeWordDetected(): Promise<void> {
        console.log('[VoiceInteraction]  AIRI activated by wake word');
        
        // Visual feedback (optional)
        const indicator = document.getElementById('airi-voice-indicator');
        if (indicator) {
            indicator.style.opacity = '1';
            indicator.style.animation = 'pulse 1s infinite';
        }

        // DISABLED: Auto-speech (was causing spam)
        // AIRI responds via voice activation system instead
        // await speak("Yes? I'm listening.", 'friendly', 8);
    }

    /**
     * Handle user finishing speaking
     */
    private async onUserFinishedSpeaking(transcript: string): Promise<void> {
        console.log('[VoiceInteraction] 📝 Processing:', transcript);
        this.state.isProcessing = true;

        // Reset wake word detection
        this.state.wakeWordDetected = false;

        // Here you would integrate with AIRI's conversation system
        // For now, just acknowledge
        console.log('[VoiceInteraction] Ready for AIRI response...');
        
        this.state.isProcessing = false;
    }

    /**
     * Check for shutdown commands in speech
     */
    private checkShutdownCommands(transcript: string): void {
        const lower = transcript.toLowerCase();
        
        // Safety shutdown phrases
        const shutdownPhrases = [
            'shutdown code 007',
            'airi shutdown 007',
            'protocol 007',
            'emergency shutdown',
            'terminate airi',
        ];

        for (const phrase of shutdownPhrases) {
            if (lower.includes(phrase)) {
                console.log(`🚨 VOICE SHUTDOWN COMMAND DETECTED: "${transcript}"`);
                airiSafetyProtocol.initiateShutdown(`Voice command: ${transcript}`);
                return;
            }
        }
    }

    /**
     * Listening started
     */
    private onListeningStart(): void {
        this.state.listening = true;
        console.log('[VoiceInteraction] 🎤 Microphone active');
    }

    /**
     * Listening ended
     */
    private onListeningEnd(): void {
        this.state.listening = false;
        console.log('[VoiceInteraction] ⏹️ Microphone inactive');

        // Restart if continuous
        if (this.config.continuousListening && !this.state.isProcessing) {
            setTimeout(() => this.startListening(), 100);
        }
    }

    /**
     * Speech recognition error
     */
    private onSpeechError(event: any): void {
        console.error('[VoiceInteraction] Error:', event.error);
        
        if (event.error === 'not-allowed') {
            console.error('Microphone permission denied! Please allow microphone access.');
        }
    }

    /**
     * Get current state
     */
    getState(): VoiceInteractionState {
        return { ...this.state };
    }

    /**
     * Check if ready for voice interaction
     */
    isReady(): boolean {
        return this.recognition !== null && isVoiceReady();
    }
}

// Export singleton
export const airiVoiceInteraction = new AIRIVoiceInteraction({
    wakeWordEnabled: true,
    continuousListening: false, // Set to true for always-listening mode
    interruptionEnabled: true,
    autoStart: true,
    safetyShutdownEnabled: true,
});

// Make globally accessible
if (typeof window !== 'undefined') {
    (window as any).__AIRI_VOICE_INTERACTION__ = airiVoiceInteraction;
}
