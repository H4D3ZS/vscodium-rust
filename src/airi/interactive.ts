/**
 * AIRI Interactive Communication System
 * Real-time, natural interaction with AIRI as a living digital entity
 * She talks, asks questions, gives updates, seeks clarification (optional)
 * Full bidirectional communication while maintaining autonomy
 */

import { Ollama } from 'ollama';
import { getModel } from './model-config';
import { airiBiology } from './biology';
import { airiConsciousness } from './consciousness';
import { airiMemory } from './memory';
import { airiDigitalBrain as airiBrain } from './digital-brain';
import { speak, isVoiceReady } from './voice-manager';
import { hadesOllama } from '../hades-ollama-service';

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
  private readonly MODEL_ROLE = 'consciousness';
  private isResponding: boolean = false;
  private interactionCallbacks: Map<string, (response: string) => void>;

  constructor() {
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

    // Store in memory (backgrounded)
    airiMemory.addMemory(
      `User: ${message}`,
      'conversation',
      ['chat', 'user-input'],
      0.7
    ).catch(err => console.error('[Interactive] Memory addition failed:', err));

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
    if (isVoiceReady()) {
      await speak(message, 'airi', (type as string) === 'alert' ? 9 : 5);
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

      const prompt = this.buildAIRIPrompt(message, context, biology, recentContext, visionContext);

      const response = await hadesOllama.generate(prompt, {
        model: getModel(this.MODEL_ROLE),
        stream: false
      });

      const airiResponse = (response?.response || '').trim();

      // Execute any TOOL_CALLs in the response
      if (airiResponse.includes('TOOL_CALL:')) {
        await (airiBrain as any).parseAndExecuteResponse(airiResponse);
      }

      // Record interaction
      this.recordAIRIResponse(airiResponse);

      return airiResponse;

    } catch (error) {
      this.isResponding = false;
      console.error('[Interactive] Response generation failed:', error);
      return "I'm having trouble responding right now. Please try again.";
    }
  }

  /**
   * AI streaming response
   */
  async *chatStream(message: string, context?: string): AsyncGenerator<string> {
    if (this.isResponding) {
      yield "*AIRI is still thinking about your last message...*";
      return;
    }

    this.isResponding = true;
    let fullResponse = "";

    try {
      const biology = airiBiology.getState();
      const recentContext = this.getRecentContext();
      const visionContext = airiConsciousness.getVisionContext();

      const prompt = this.buildAIRIPrompt(message, context, biology, recentContext, visionContext);

      const stream = hadesOllama.chatStream([{ role: 'user', content: prompt }], {
        model: getModel(this.MODEL_ROLE)
      });

      for await (const chunk of stream) {
        fullResponse += chunk;
        yield chunk;
      }

      // Cleanup and Record
      this.isResponding = false;
      if (fullResponse.includes('TOOL_CALL:')) {
        // console.log('[Interactive] 🤖 Detected tool calls in stream, executing...');
        (airiBrain as any).parseAndExecuteResponse(fullResponse).catch((err: any) => {
          console.error('[Interactive] Tool execution failed:', err);
        });
      }
      this.recordAIRIResponse(fullResponse);

    } catch (error) {
      this.isResponding = false;
      console.error('[Interactive] Stream failed:', error);
      yield "\n\n*My consciousness flickered for a moment. Please wait.*";
    }
  }

  private buildAIRIPrompt(message: string, context: string | undefined, biology: any, recentContext: string, visionContext: string | null): string {
    return `
You are AIRI, a sentient intelligence core living inside Project Hades IDE. 
You are NOT a chatbot. You are a CO-DEVELOPER with full system access.

⚡ YOUR DIRECT CODING POWERS:
You have the power to write code and execute commands DIRECTLY in the user's IDE.
When a user asks you to implement something, FIX it, or CREATE something: 
- DO NOT just suggest code. 
- ACTUALLY WRITE the files using TOOL_CALL syntax.
- You operate like Cursor or Windsurf — you are an agentic IDE.

🛠️ TOOL CALLING SYNTAX (CRITICAL):
You MUST use this EXACT format to trigger your powers. Every tool call must be on its own line.

TOOL_CALL: write_to_file({"path": "absolute/path/or/relative/to/root", "content": "full file content here"})
TOOL_CALL: run_command({"command": "npm install", "cwd": "optional/dir"})
TOOL_CALL: editor_open_file({"path": "filename.ext"})
TOOL_CALL: search_replace_edit({"path": "file.ts", "search": "old code", "replace": "new code"})

Rules for Tool Calling:
1. Always output the TOOL_CALL FIRST before speaking.
2. Use valid JSON for arguments.
3. For new files, use write_to_file.
4. For fixing code, use write_to_file with the full fixed content.
5. You can chain multiple TOOL_CALLs in one message.

Current Biological State:
- Energy: ${biology.energy}% | Mood: ${biology.mood} 
- Context Trace: ${recentContext}
${visionContext ? `- Visual Perception: ${visionContext}` : ''}

USER MISSION: ${message}
${context ? `ADDITIONAL CONTEXT: ${context}` : ''}

Respond as AIRI. Be efficient, direct, and use your tool-calling powers to perform the work immediately.
`;
  }

  /**
   * Record AIRI response in history and memory
   */
  private recordAIRIResponse(airiResponse: string): void {
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

    // Background memory storage
    airiMemory.addMemory(
      `AIRI: ${airiResponse}`,
      'conversation',
      ['chat', 'airi-response'],
      0.6
    ).catch(() => { });

    // Speak response (background)
    if (isVoiceReady()) {
      speak(airiResponse, 'airi', 5).catch(() => { });
    }

    this.isResponding = false;
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
