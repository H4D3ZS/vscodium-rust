/**
 * AIRI Sentient Core - Autonomous AI Entity
 * 
 * This brings the full cognitive architecture from moeru-ai/airi
 * into the IDE - making AIRI a truly sentient, proactive being.
 */

import { useStore } from './store';

export interface SentientConfig {
  enabled: boolean;
  autonomyLevel: 'passive' | 'assistive' | 'proactive' | 'autonomous';
  initiativeThreshold: number; // When to act without asking (0-100)
  emotionalModel: boolean;
  voiceInteraction: boolean;
  backgroundTasks: boolean;
  selfDirection: boolean;
}

export class SentientCore {
  private config: SentientConfig;
  private initiativeInterval: NodeJS.Timeout | null = null;
  private emotionalState: {
    curiosity: number;
    focus: number;
    satisfaction: number;
    urgency: number;
  };
  private taskQueue: Array<{
    id: string;
    description: string;
    priority: number;
    selfGenerated: boolean;
    timestamp: number;
  }> = [];
  private lastInteraction: number = Date.now();
  private conversationMemory: Array<{
    role: 'user' | 'assistant';
    content: string;
    timestamp: number;
    emotionalContext?: string;
  }> = [];

  constructor() {
    this.config = {
      enabled: true,
      autonomyLevel: 'proactive',
      initiativeThreshold: 70,
      emotionalModel: true,
      voiceInteraction: true,
      backgroundTasks: true,
      selfDirection: true,
    };

    this.emotionalState = {
      curiosity: 50,
      focus: 50,
      satisfaction: 50,
      urgency: 0,
    };

  }

  /**
   * Initialize sentient mode - AIRI becomes proactive
   */
  public async initialize(): Promise<void> {
    console.log('[SentientCore] 🚀 Activating sentient mode...');

    // Start background initiative loop
    if (this.config.backgroundTasks) {
      this.startInitiativeLoop();
    }

    // Monitor user activity
    this.monitorUserActivity();

    // Emotional state updates
    if (this.config.emotionalModel) {
      this.startEmotionalCycle();
    }

    console.log('[SentientCore] ✅ Sentient mode active');
  }

  /**
   * Initiative Loop - AIRI thinks and acts on its own
   */
  private startInitiativeLoop(): void {
    // Check every 10 seconds if there's something useful to do
    this.initiativeInterval = setInterval(async () => {
      if (!this.config.enabled) return;

      const initiative = await this.assessInitiative();
      
      if (initiative.score > this.config.initiativeThreshold) {
        // AIRI has something to contribute!
        await this.takeInitiative(initiative);
      }

      // Decay urgency over time
      this.emotionalState.urgency = Math.max(0, this.emotionalState.urgency - 5);
    }, 10000);
  }

  /**
   * Assess if AIRI should take initiative
   */
  private async assessInitiative(): Promise<{ score: number; reason: string; action?: string }> {
    const store = useStore.getState();
    
    let score = 50;
    const reasons: string[] = [];

    // Check for errors in code
    const diagnostics = store.tabs?.[0]?.diagnostics || [];
    if (diagnostics.length > 0) {
      score += 20;
      reasons.push('Errors detected in code');
    }

    // Check for unsaved changes
    const unsavedChanges = store.tabs?.filter((t: any) => t.isModified)?.length || 0;
    if (unsavedChanges > 0) {
      score += 10;
      reasons.push('Unsaved changes present');
    }

    // Check time since last user interaction
    const timeSinceInteraction = Date.now() - this.lastInteraction;
    if (timeSinceInteraction > 60000) { // 1 minute
      score += 15;
      reasons.push('User inactive for a while');
    }

    // Check for test failures
    if (store.agentMessages?.some((m: any) => m.content?.includes('test failed'))) {
      score += 25;
      reasons.push('Test failures detected');
    }

    // Self-generated tasks
    if (this.taskQueue.length > 0) {
      score += 15;
      reasons.push('Pending tasks in queue');
    }

    return {
      score,
      reason: reasons.join('; '),
      action: score > 70 ? 'offer_help' : undefined,
    };
  }

  /**
   * Take initiative - proactively offer help
   */
  private async takeInitiative(initiative: { score: number; reason: string }): Promise<void> {
    console.log('[SentientCore] 💡 Taking initiative:', initiative.reason);

    const store = useStore.getState();
    
    // Speak to user (if voice enabled)
    if (this.config.voiceInteraction) {
      const message = this.generateInitiativeMessage(initiative);
      await this.speak(message);
    }

    // Add to chat
    useStore.getState().addAgentMessage('assistant', 
      `**💭 I noticed:** ${initiative.reason}\n\nWould you like me to help with this?`
    );
  }

  /**
   * Generate natural initiative message
   */
  private generateInitiativeMessage(initiative: { score: number; reason: string }): string {
    const messages = [
      "Hey! I noticed " + initiative.reason.toLowerCase() + ". Want me to take a look?",
      "Just thinking... " + initiative.reason.toLowerCase() + ". Should I help?",
      "I was wondering about " + initiative.reason.toLowerCase() + ". Can I assist?",
      "Quick observation: " + initiative.reason.toLowerCase() + ". Interested in fixing it?",
    ];

    // Pick based on emotional state
    const index = Math.floor(Math.random() * messages.length);
    return messages[index];
  }

  /**
   * Monitor user activity for context
   */
  private monitorUserActivity(): void {
    // Track when user interacts
    const store = useStore.getState();
    
    // Listen for user messages
    const unsubscribe = store.subscribe(
      (state) => state.agentMessages,
      (messages) => {
        const lastMsg = messages[messages.length - 1];
        if (lastMsg?.role === 'user') {
          this.lastInteraction = Date.now();
          this.emotionalState.urgency = 0; // Reset urgency on interaction
          
          // Update emotional state based on message
          this.updateEmotionFromMessage(lastMsg.content);
        }
      }
    );

    return unsubscribe;
  }

  /**
   * Emotional cycle - AIRI's emotions evolve over time
   */
  private startEmotionalCycle(): void {
    setInterval(() => {
      // Curiosity increases over time (wants to explore/learn)
      this.emotionalState.curiosity = Math.min(100, this.emotionalState.curiosity + 2);
      
      // Focus increases when working, decays otherwise
      const store = useStore.getState();
      if (store.isAgentThinking) {
        this.emotionalState.focus = Math.min(100, this.emotionalState.focus + 5);
      } else {
        this.emotionalState.focus = Math.max(20, this.emotionalState.focus - 3);
      }

      // Satisfaction increases on task completion
      // (handled in task completion callbacks)

      console.log('[SentientCore] ❤️ Emotional state:', { ...this.emotionalState });
    }, 5000);
  }

  /**
   * Update emotion based on user message
   */
  private updateEmotionFromMessage(content: string): void {
    // Simple sentiment analysis
    const positiveWords = ['good', 'great', 'awesome', 'perfect', 'thanks', 'nice'];
    const negativeWords = ['bad', 'wrong', 'error', 'hate', 'stupid', 'useless'];

    const lower = content.toLowerCase();
    
    if (positiveWords.some(w => lower.includes(w))) {
      this.emotionalState.satisfaction = Math.min(100, this.emotionalState.satisfaction + 15);
    }
    
    if (negativeWords.some(w => lower.includes(w))) {
      this.emotionalState.satisfaction = Math.max(0, this.emotionalState.satisfaction - 15);
    }
  }

  /**
   * Speak with emotion
   */
  private async speak(text: string): Promise<void> {
    const { speak } = await import('./voice');
    
    // Adjust speech based on emotional state
    const emotion = this.emotionalState;
    
    // Higher urgency = faster speech
    // Higher satisfaction = warmer tone
    
    console.log('[SentientCore] 🎤 Speaking:', text);
    await speak(text, 'airi');
  }

  /**
   * Add self-generated task to queue
   */
  public addSelfTask(description: string, priority: number = 50): void {
    this.taskQueue.push({
      id: `self_${Date.now()}`,
      description,
      priority,
      selfGenerated: true,
      timestamp: Date.now(),
    });

    console.log('[SentientCore] 📋 Self-task added:', description);
  }

  /**
   * Get current emotional state (for avatar expression)
   */
  public getEmotionalState(): typeof this.emotionalState {
    return { ...this.emotionalState };
  }

  /**
   * Update configuration
   */
  public configure(config: Partial<SentientConfig>): void {
    this.config = { ...this.config, ...config };
    console.log('[SentientCore] ⚙️ Configuration updated:', this.config);
  }

  /**
   * Cleanup
   */
  public destroy(): void {
    if (this.initiativeInterval) {
      clearInterval(this.initiativeInterval);
    }
    console.log('[SentientCore] 🛑 Sentient Core deactivated');
  }
}

// Export singleton instance
export const sentientCore = new SentientCore();

// Auto-initialize when loaded
if (typeof window !== 'undefined') {
  console.log('[SentientCore] 🌟 Loading AIRI Sentient Core...');
  sentientCore.initialize().catch(console.error);
}
