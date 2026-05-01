/**
 * AIRI Voice Activation System
 * 
 * Natural voice interaction for a true digital entity:
 * - Wake word detection ("Hey AIRI")
 * - Manual voice toggle button
 * - Conversation mode (back-and-forth)
 * - Emotional voice modulation
 * 
 * NOT auto-speech spam - ALL voice is USER-TRIGGERED
 */

import { useStore } from '../store';

export interface VoiceActivationConfig {
  wakeWordEnabled: boolean;
  wakeWord: string;  // "Hey AIRI"
  conversationMode: boolean;
  emotionalModulation: boolean;
}

export class AIRIVoiceActivation {
  private config: VoiceActivationConfig = {
    wakeWordEnabled: true,
    wakeWord: 'hey airi',
    conversationMode: true,
    emotionalModulation: true,
  };

  private isListening: boolean = false;
  private recognition: any = null;
  private audioContext: AudioContext | null = null;

  constructor() {
    this.initWakeWordDetection();
  }

  /**
   * Initialize wake word detection
   */
  private initWakeWordDetection(): void {
    // Use Web Speech API for wake word
    if ('webkitSpeechRecognition' in window || 'SpeechRecognition' in window) {
      const SpeechRecognition = (window as any).SpeechRecognition || (window as any).webkitSpeechRecognition;
      this.recognition = new SpeechRecognition();
      
      this.recognition.continuous = true;
      this.recognition.interimResults = true;
      this.recognition.lang = 'en-US';

      this.recognition.onresult = (event: any) => {
        const transcript = Array.from(event.results)
          .map((result: any) => result[0].transcript)
          .join('');

        // Check for wake word
        if (transcript.toLowerCase().includes(this.config.wakeWord)) {
          this.onWakeWordDetected();
        }
      };

      this.recognition.onerror = (event: any) => {
        console.error('[VoiceActivation] Recognition error:', event.error);
      };

      // Start listening
      this.recognition.start();
      this.isListening = true;

    } else {
      console.warn('[VoiceActivation] Web Speech API not supported');
    }
  }

  /**
   * Wake word detected - AIRI responds
   */
  private async onWakeWordDetected(): Promise<void> {

    // Visual feedback
    useStore.getState().setIsAgentThinking(true);

    // Play activation sound
    this.playActivationSound();

    // AIRI responds
    const responses = [
      "Yes? I'm listening.",
      "I'm here. What do you need?",
      "Ready to help.",
      "What's on your mind?",
    ];

    const response = responses[Math.floor(Math.random() * responses.length)];
    
    // Speak response
    await this.speak(response);

    useStore.getState().setIsAgentThinking(false);
  }

  /**
   * Manual voice toggle (button click)
   */
  public async toggleVoice(): Promise<void> {
    if (this.isListening) {
      this.stopListening();
    } else {
      this.startListening();
    }
  }

  /**
   * Start listening for commands
   */
  private startListening(): void {
    if (this.recognition) {
      this.recognition.start();
      this.isListening = true;
    }
  }

  /**
   * Stop listening
   */
  private stopListening(): void {
    if (this.recognition) {
      this.recognition.stop();
      this.isListening = false;
    }
  }

  /**
   * Speak with emotion
   */
  public async speak(text: string, emotion?: 'neutral' | 'friendly' | 'excited' | 'concerned'): Promise<void> {
    const { speak } = await import('../voice');
    
    // Map emotion to preset
    const preset = emotion === 'friendly' ? 'friendly' : 
                   emotion === 'excited' ? 'excited' :
                   emotion === 'concerned' ? 'concerned' : 'airi';

    try {
      await speak(text, preset);
    } catch (e) {
      console.error('[VoiceActivation] Speak error:', e);
    }
  }

  /**
   * Play activation sound
   */
  private playActivationSound(): void {
    const audioContext = new (window.AudioContext || (window as any).webkitAudioContext)();
    const oscillator = audioContext.createOscillator();
    const gainNode = audioContext.createGain();

    oscillator.connect(gainNode);
    gainNode.connect(audioContext.destination);

    oscillator.frequency.value = 880; // A5
    oscillator.type = 'sine';
    gainNode.gain.setValueAtTime(0.1, audioContext.currentTime);
    gainNode.gain.exponentialRampToValueAtTime(0.01, audioContext.currentTime + 0.3);

    oscillator.start(audioContext.currentTime);
    oscillator.stop(audioContext.currentTime + 0.3);
  }

  /**
   * Start conversation mode
   */
  public async startConversation(): Promise<void> {

    // Greet user
    await this.speak("Yes? I'm listening.", 'friendly');

    // Keep listening for follow-up
    this.startListening();
  }

  /**
   * Get voice activation status
   */
  public getStatus(): {
    isListening: boolean;
    wakeWordEnabled: boolean;
    conversationMode: boolean;
  } {
    return {
      isListening: this.isListening,
      wakeWordEnabled: this.config.wakeWordEnabled,
      conversationMode: this.config.conversationMode,
    };
  }
}

// Singleton instance
export const airiVoiceActivation = new AIRIVoiceActivation();
