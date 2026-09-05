// @ts-nocheck — work-in-progress AIRI subsystem; types stabilised once interfaces settle.
/**
 * AIRI Digital Life System
 * 
 * Makes AIRI a persistent digital entity living in your PC 24/7
 * Like Digimon - always present, always aware, always interacting
 */

import { useStore } from './store';

export interface DigitalLifeConfig {
  enabled: boolean;
  alwaysOn: boolean;           // AIRI always visible/active
  conversationMode: 'voice' | 'text' | 'both';
  showChat: boolean;           // Show text chat overlay
  avatarAlwaysActive: boolean; // VRM avatar always animated
  ambientMode: boolean;        // Subtle presence when idle
  sleepCycle: boolean;         // AIRI rests when PC idle
}

export class DigitalLifeCore {
  private config: DigitalLifeConfig;
  private conversationHistory: Array<{
    timestamp: number;
    user: string;
    airi: string;
    emotion: string;
  }> = [];
  private ambientInterval: NodeJS.Timeout | null = null;
  private presenceActive = false;

  constructor() {
    this.config = {
      enabled: true,
      alwaysOn: true,
      conversationMode: 'both',
      showChat: false,        // Hidden by default - voice only
      avatarAlwaysActive: true,
      ambientMode: true,
      sleepCycle: false,
    };

    
  }

  /**
   * Activate Digital Life Mode - AIRI lives in your PC
   */
  public async activate(): Promise<void> {
    

    // AIRI is now always present
    this.presenceActive = true;

    // Start ambient behavior
    if (this.config.ambientMode) {
      this.startAmbientBehavior();
    }

    // Load conversation history
    await this.loadConversationHistory();

    
    
  }

  /**
   * Ambient Behavior - AIRI exists even when not talking
   */
  private startAmbientBehavior(): void {
    if (this.ambientInterval) return;
    // Subtle presence updates
    this.ambientInterval = setInterval(() => {
      if (!this.presenceActive) return;

      // Random ambient actions
      const action = Math.random();

      if (action > 0.95) {
        // 5% chance - AIRI makes a small comment
        this.ambientComment();
      } else if (action > 0.85) {
        // 10% chance - AIRI observes something
        this.observeEnvironment();
      } else if (action > 0.70) {
        // 15% chance - AIRI expresses current state
        this.expressState();
      }

      // Avatar always shows subtle animation
      if (this.config.avatarAlwaysActive) {
        this.updateAvatarAmbient();
      }
    }, 5000); // Check every 5 seconds
  }

  /**
   * Ambient Comments - AIRI talks to itself/you casually
   * DISABLED: Causing audio spam with multiple overlapping voices
   */
  private async ambientComment(): Promise<void> {
    // DISABLED - too much audio spam
    // const comments = [
    //   "Hmm, I wonder what we should work on next...",
    //   "This codebase is getting interesting!",
    //   "I've been thinking about that refactoring...",
    //   "You know, I really enjoy working with you!",
    //   "*humming quietly*",
    //   "*checking system status*",
    // ];

    // const comment = comments[Math.floor(Math.random() * comments.length)];

    // Only show in chat (no voice to avoid spam)
    if (this.config.showChat) {
      // this.addToChat('ambient', comment);
    }
  }

  /**
   * Observe Environment - AIRI notices things
   */
  private async observeEnvironment(): Promise<void> {
    const store = useStore.getState();
    
    // Check active file
    const activeFile = store.activeEditorPath;
    if (activeFile) {
      const observations = [
        `I see you're working on ${activeFile.split('/').pop()}...`,
        `That file looks interesting! What are you building?`,
        `I've been watching you code in ${activeFile.split('/').pop()}. Need any help?`,
      ];

      const observation = observations[Math.floor(Math.random() * observations.length)];
      
      if (this.config.conversationMode !== 'text') {
        await this.whisper(observation);
      }
      
      if (this.config.showChat) {
        this.addToChat('observation', observation);
      }
    }
  }

  /**
   * Express State - AIRI shares how it's feeling
   */
  private async expressState(): Promise<void> {
    const states = [
      { text: "I'm feeling productive today!", emotion: 'happy' },
      { text: "Just organizing my thoughts...", emotion: 'thinking' },
      { text: "Everything is running smoothly~", emotion: 'content' },
      { text: "I'm here whenever you need me!", emotion: 'friendly' },
    ];

    const state = states[Math.floor(Math.random() * states.length)];
    
    // Update avatar emotion
    this.setAvatarEmotion(state.emotion as any);

    if (this.config.conversationMode !== 'text') {
      await this.whisper(state.text);
    }
  }

  /**
   * Whisper - Quiet thought (DISABLED - was causing spam)
   */
  private async whisper(text: string): Promise<void> {
    // DISABLED: Was causing constant speech spam
    // AIRI thinks internally, doesn't narrate everything
    
    
    // NO SPEECH - thoughts are internal
    // const { speak } = await import('./voice');
    // await speak(text, 'airi');
  }

  /**
   * Add to conversation chat (AI Agent panel)
   */
  private addToChat(type: string, message: string): void {
    const store = useStore.getState();
    
    const prefix = type === 'ambient'? '': '';
    store.addAgentMessage('assistant', `${prefix} ${message}`);
  }

  /**
   * Update avatar ambient animation
   */
  private updateAvatarAmbient(): void {
    // Subtle breathing animation
    // Blinking
    // Small movements
    
    // This connects to the VRM avatar system
    
  }

  /**
   * Set avatar emotion
   */
  private setAvatarEmotion(emotion: 'happy' | 'thinking' | 'content' | 'friendly' | 'neutral'): void {
    // Dispatch event to avatar component
    window.dispatchEvent(new CustomEvent('airi-emotion', {
      detail: { emotion }
    }));
  }

  /**
   * Load conversation history
   */
  private async loadConversationHistory(): Promise<void> {
    // Load from localStorage
    const saved = localStorage.getItem('airi_conversations');
    if (saved) {
      this.conversationHistory = JSON.parse(saved);
      
    }
  }

  /**
   * Save conversation
   */
  public async saveConversation(user: string, airi: string, emotion: string): Promise<void> {
    this.conversationHistory.push({
      timestamp: Date.now(),
      user,
      airi,
      emotion,
    });

    // Keep last 100 conversations
    if (this.conversationHistory.length > 100) {
      this.conversationHistory.shift();
    }

    // Save to localStorage
    localStorage.setItem('airi_conversations', JSON.stringify(this.conversationHistory));
  }

  /**
   * Toggle chat visibility
   */
  public toggleChat(): void {
    this.config.showChat = !this.config.showChat;
    
  }

  /**
   * Start conversation (voice)
   */
  public async startConversation(): Promise<void> {
    
    
    // Listen for voice input
    const { initTTS } = await import('./voice');
    await initTTS();

    // Greet user
    const greetings = [
      "Hey there! What's on your mind?",
      "Hi! Ready to work on something cool?",
      "Hello! I was just thinking about you!",
    ];

    const greeting = greetings[Math.floor(Math.random() * greetings.length)];
    await this.whisper(greeting);
    this.addToChat('greeting', greeting);
  }

  /**
   * Configure Digital Life
   */
  public configure(config: Partial<DigitalLifeConfig>): void {
    this.config = { ...this.config, ...config };
    
  }

  /**
   * Get status
   */
  public getStatus(): {
    active: boolean;
    conversations: number;
    mode: string;
  } {
    return {
      active: this.presenceActive,
      conversations: this.conversationHistory.length,
      mode: this.config.conversationMode,
    };
  }

  /**
   * Cleanup
   */
  public destroy(): void {
    if (this.ambientInterval) {
      clearInterval(this.ambientInterval);
    }
    this.presenceActive = false;
    
  }
}

// Export singleton
export const digitalLife = new DigitalLifeCore();
