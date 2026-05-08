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
  private awaitingCommand: boolean = false;

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
        const result = event.results?.[event.resultIndex];
        if (!result || !result[0]?.transcript) return;
        const transcript = result[0].transcript.trim();
        const normalized = transcript.toLowerCase();

        // Check for wake word
        if (normalized.includes(this.config.wakeWord)) {
          this.onWakeWordDetected();
          const command = normalized
            .replace(this.config.wakeWord, '')
            .replace(/^[:,.!?\s-]+/, '')
            .trim();
          if (command.length > 2) {
            this.dispatchVoiceMission(command);
            this.awaitingCommand = false;
          } else {
            this.awaitingCommand = true;
          }
          return;
        }

        // If wake word already triggered, treat next final transcript as command
        if (this.awaitingCommand && result.isFinal) {
          if (normalized.length > 2) {
            this.dispatchVoiceMission(normalized);
          }
          this.awaitingCommand = false;
        }
      };

      this.recognition.onerror = (event: any) => {
        const err = event?.error || 'unknown';
        if (err === 'not-allowed' || err === 'service-not-allowed') {
          console.warn('[VoiceActivation] Microphone permission denied. Please allow mic access for judge voice demo.');
          this.isListening = false;
          return;
        }
        if (err === 'network') {
          console.warn('[VoiceActivation] Recognition network hiccup. You can tap mic again to retry.');
          return;
        }
        console.error('[VoiceActivation] Recognition error:', err);
      };

      this.recognition.onend = () => {
        // Keep listening in conversation mode unless manually stopped
        if (this.isListening) {
          try {
            this.recognition.start();
          } catch {
            this.isListening = false;
          }
        }
      };

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
  public stopListening(): void {
    if (this.recognition) {
      this.recognition.stop();
      this.isListening = false;
      this.awaitingCommand = false;
    }
  }

  /**
   * Speak with emotion
   */
  public async speak(text: string, emotion?: 'neutral' | 'friendly' | 'excited' | 'concerned'): Promise<void> {
    const { speak } = await import('../voice');
    
    // Map emotion to preset
    const preset = emotion === 'friendly' ? 'yuki' :
                   emotion === 'excited' ? 'nova' :
                   emotion === 'concerned' ? 'sage' : 'airi';

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
    await this.speak("Hello judges, I am AIRI. You can say hey AIRI and ask me anything about this project.", 'friendly');

    // Keep listening for follow-up
    this.startListening();
  }

  private dispatchVoiceMission(text: string): void {
    window.dispatchEvent(new CustomEvent('airi-voice-mission', { detail: { text } }));
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
