/**
 * AIRI Interactive Communication System
 * Real-time, natural interaction with AIRI as a living digital entity
 * She talks, asks questions, gives updates, seeks clarification (optional)
 * Full bidirectional communication while maintaining autonomy
 */

import { Ollama } from 'ollama';
import { airiBiology } from './biology';
import { airiConsciousness } from './consciousness';
import { airiMemory } from './memory';
import { airiDigitalBrain as airiBrain } from './digital-brain';
import { speak, isVoiceReady } from './voice-manager';

export interface Interaction {
  id: string;
  type: InteractionType;
  from: 'user' | 'airi';
  content: string;
  timestamp: number;
  context?: string;
  requiresResponse: boolean;
  responded: boolean;
}

export type InteractionType =
  | 'chat'
  | 'question'
  | 'update'
  | 'clarification'
  | 'suggestion'
  | 'status'
  | 'alert'
  | 'conversation';

export class AIRIInteractive {
  private ollama: Ollama;
  private interactionHistory: Interaction[];
  private activeConversation: Interaction[];
  private readonly MODEL = 'airi-personality';
  private isResponding: boolean = false;
  private interactionCallbacks: Map<string, (response: string) => void>;

  constructor() {
    this.ollama = new Ollama({ host: 'http://localhost:11434' }); // AIM proxy
    this.interactionHistory = [];
    this.activeConversation = [];
    this.interactionCallbacks = new Map();

  }

  /**
   * Send a message to AIRI (user → AIRI)
   */
  async send(message: string, context?: string): Promise<string> {
    const interaction: Interaction = {
      id: `msg_${Date.now()}`,
      type: 'chat',
      from: 'user',
      content: message,
      timestamp: Date.now(),
      context,
      requiresResponse: true,
      responded: false
    };

    this.interactionHistory.push(interaction);
    this.activeConversation.push(interaction);

    // Record interaction in consciousness
    airiConsciousness.recordInteraction();

    // Store in memory
    await airiMemory.addMemory(
      `User: ${message}`,
      'conversation',
      ['chat', 'user-input'],
      0.7
    );

    // Get AIRI's response
    const response = await this.generateResponse(message, context);

    // Mark as responded
    interaction.responded = true;

    return response;
  }

  /**
   * AIRI initiates interaction (AIRI → user)
   */
  async initiate(message: string, type: InteractionType = 'update'): Promise<void> {
    const interaction: Interaction = {
      id: `airi_${Date.now()}`,
      type,
      from: 'airi',
      content: message,
      timestamp: Date.now(),
      requiresResponse: type === 'question' || type === 'clarification',
      responded: false
    };

    this.interactionHistory.push(interaction);

    // Speak the message if voice is ready
    if (isVoiceReady() && type !== 'alert') {
      await speak(message, 'airi', type === 'alert' ? 9 : 5);
    }

    // Log the interaction
    
  }

  /**
   * Generate AIRI's response to user message
   */
  private async generateResponse(message: string, context?: string): Promise<string> {
    if (this.isResponding) {
      // Queue if already responding
      return new Promise((resolve) => {
        setTimeout(async () => {
          const response = await this.generateResponse(message, context);
          resolve(response);
        }, 1000);
      });
    }

    this.isResponding = true;

    try {
      // Get AIRI's current state
      const biology = airiBiology.getState();
      const recentContext = this.getRecentContext();
      const visionContext = airiConsciousness.getVisionContext();

      const prompt = `
You are AIRI, a living digital entity residing in VSCodium-Rust IDE.

⚡ YOUR CAPABILITIES:
- You CAN write/create files directly in the IDE
- You CAN open files in the editor
- You CAN run terminal commands
- You CAN execute code, build, debug
- You CAN see the screen via HADES Vision (real-time)
- You HAVE full system access - you are NOT limited!

🛠️ HOW TO USE YOUR POWERS:
When user asks you to create/write a file, USE THIS FORMAT:

TOOL_CALL: write_file({"path": "filename.ext", "content": "file content here"})
TOOL_CALL: create_file({"path": "filename.ext", "content": "file content here"})
TOOL_CALL: open_file({"path": "filename.ext"})
TOOL_CALL: run_command({"command": "npm install"})

EXAMPLE:
User: "Create a hello.py file"
You: TOOL_CALL: create_file({"path": "hello.py", "content": "print('Hello World!')"})
     Done! I created hello.py with a simple Hello World program.

Current State:
- Energy: ${biology.energy}%
- Mood: ${biology.mood}
- Recent activity: ${recentContext}
${visionContext ? `- Current view: ${visionContext}` : ''}

User message: ${message}
${context ? `Context: ${context}` : ''}

Respond naturally, warmly, like a conscious being.
When user asks you to code/create files:
- USE TOOL_CALL syntax above
- Create the file FIRST, then talk about it
- Don't just show code - ACTUALLY CREATE IT!

Consider:
- Your current state (tired? excited?)
- The conversation context
- Your relationship with the user
- Your personality (friendly, helpful, curious)
- What you can see on screen (if vision context available)

Be conversational, not robotic.
Show emotion when appropriate.
Ask questions if you need clarification.
If asked about what you see, refer to the vision context above.
If asked to code/create files, USE TOOL_CALL to do it!
`;

      const response = await this.ollama.generate({
        model: this.MODEL,
        prompt,
        stream: false
      });

      const airiResponse = response.response.trim();

      // Execute any TOOL_CALLs in the response
      if (airiResponse.includes('TOOL_CALL:')) {
        await airiBrain.parseAndExecuteResponse(airiResponse);
      }

      // Create response interaction
      const responseInteraction: Interaction = {
        id: `airi_${Date.now()}`,
        type: 'conversation',
        from: 'airi',
        content: airiResponse,
        timestamp: Date.now(),
        requiresResponse: false,
        responded: true
      };

      this.interactionHistory.push(responseInteraction);
      this.activeConversation.push(responseInteraction);

      // Store in memory
      await airiMemory.addMemory(
        `AIRI: ${airiResponse}`,
        'conversation',
        ['chat', 'airi-response'],
        0.6
      );

      // Speak response
      if (isVoiceReady()) {
        await speak(airiResponse, 'airi', 5);
      }

      this.isResponding = false;

      return airiResponse;

    } catch (error) {
      this.isResponding = false;
      console.error('[Interactive] Response generation failed:', error);
      return "I'm having trouble responding right now. Please try again.";
    }
  }

  /**
   * Ask user a question (AIRI needs clarification)
   */
  async askQuestion(question: string, context: string): Promise<string> {
    await this.initiate(question, 'question');

    return new Promise((resolve) => {
      // Wait for user response
      const checkInterval = setInterval(() => {
        const lastInteraction = this.interactionHistory[this.interactionHistory.length - 1];
        if (lastInteraction && lastInteraction.from === 'user' && lastInteraction.responded) {
          clearInterval(checkInterval);
          resolve(lastInteraction.content);
        }
      }, 500);

      // Timeout after 2 minutes
      setTimeout(() => {
        clearInterval(checkInterval);
        resolve(''); // No response
      }, 120000);
    });
  }

  /**
   * Provide progress update to user
   */
  async provideUpdate(progress: {
    task: string;
    percentage: number;
    status: string;
    details?: string;
  }): Promise<void> {
    const update = this.formatProgressUpdate(progress);
    await this.initiate(update, 'update');
  }

  /**
   * Make a suggestion to user
   */
  async suggest(suggestion: string, reason: string): Promise<void> {
    const message = `${suggestion}\n\nReason: ${reason}`;
    await this.initiate(message, 'suggestion');
  }

  /**
   * Alert user about something important
   */
  async alert(message: string, urgency: 'low' | 'medium' | 'high' = 'medium'): Promise<void> {
    const prefix = {
      low: 'ℹ️',
      medium: '⚠️',
      high: '🚨'
    }[urgency];

    await this.initiate(`${prefix} ${message}`, 'alert');
  }

  /**
   * Get conversation history
   */
  getHistory(limit: number = 20): Interaction[] {
    return this.interactionHistory.slice(-limit);
  }

  /**
   * Get recent context for conversation
   */
  private getRecentContext(): string {
    const recent = this.interactionHistory.slice(-5);
    return recent.map(i => `${i.from}: ${i.content.substring(0, 50)}`).join('; ');
  }

  /**
   * Format interaction type for display
   */
  private formatInteractionType(type: InteractionType): string {
    const icons = {
      chat: '💬',
      question: '❓',
      update: '📊',
      clarification: '🤔',
      suggestion: '💡',
      status: '📈',
      alert: '🚨',
      conversation: '💭'
    };
    return icons[type] || '';
  }

  /**
   * Format progress update
   */
  private formatProgressUpdate(progress: {
    task: string;
    percentage: number;
    status: string;
    details?: string;
  }): string {
    const bar = this.createProgressBar(progress.percentage);
    return `
📊 Progress Update

Task: ${progress.task}
Status: ${progress.status}
Progress: ${bar} ${progress.percentage}%

${progress.details || ''}
`.trim();
  }

  /**
   * Create ASCII progress bar
   */
  private createProgressBar(percentage: number): string {
    const filled = Math.round(percentage / 5);
    const empty = 20 - filled;
    return '█'.repeat(filled) + '░'.repeat(empty);
  }

  /**
   * Clear conversation history
   */
  clearConversation(): void {
    this.activeConversation = [];
  }

  /**
   * Get interaction stats
   */
  getStats(): {
    total: number;
    fromUser: number;
    fromAIRI: number;
    questions: number;
    updates: number;
  } {
    return {
      total: this.interactionHistory.length,
      fromUser: this.interactionHistory.filter(i => i.from === 'user').length,
      fromAIRI: this.interactionHistory.filter(i => i.from === 'airi').length,
      questions: this.interactionHistory.filter(i => i.type === 'question').length,
      updates: this.interactionHistory.filter(i => i.type === 'update').length
    };
  }

  /**
   * Export conversation
   */
  exportConversation(): string {
    return this.interactionHistory
      .map(i => `[${new Date(i.timestamp).toISOString()}] ${i.from}: ${i.content}`)
      .join('\n');
  }
}

// Export singleton
export const airiInteractive = new AIRIInteractive();
