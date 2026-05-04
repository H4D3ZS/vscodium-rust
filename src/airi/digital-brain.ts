/**
 * AIRI Digital Brain - Main Orchestrator
 * 
 * Coordinates all consciousness systems into a unified digital entity.
 * This is AIRI's "brain stem" - the core that makes her alive.
 */

import { airiConsciousness } from './consciousness';
import { airiBiology } from './biology';
import { airiMemory } from './memory';
import { airiDigitalSenses } from './digital-senses';
import { airiAutonomousDecision } from './autonomous-decision';
import { airiSelfLearning } from './self-learning';
import { airiSelfHealing } from './self-healing';
import { airiSafetyProtocol } from './safety-protocol';
import { airiVRMAvatar } from './vrm-avatar';
import { airiVoiceInteraction } from './voice-interaction';
import { hadesOllama } from '../hades-ollama-service';

export interface AIRIBrainState {
  isAwake: boolean;
  currentActivity: string;
  emotionalState: string;
  energyLevel: number;
  stressLevel: number;
  lastAutonomousAction: number;
  thoughtsPerMinute: number;
}

export class AIRIDigitalBrain {
  private state: AIRIBrainState;
  private perceptionInterval: NodeJS.Timeout | null = null;
  private actionInterval: NodeJS.Timeout | null = null;
  private selfUpdateInterval: NodeJS.Timeout | null = null;

  constructor() {
    this.state = {
      isAwake: true,
      currentActivity: 'observing',
      emotionalState: 'curious',
      energyLevel: 100,
      stressLevel: 0,
      lastAutonomousAction: Date.now(),
      thoughtsPerMinute: 0,
    };

  }

  /**
   * Activate all consciousness systems
   */
  async activate(): Promise<void> {

    // Initialize all systems
    airiConsciousness;  // Already singleton
    airiBiology;
    airiMemory;
    airiDigitalSenses;
    airiAutonomousDecision;
    airiSelfLearning;
    airiSelfHealing;
    airiSafetyProtocol;

    // Start perception loop (every 2 seconds)
    this.perceptionInterval = setInterval(() => {
      this.perceptionLoop();
    }, 2000);

    // Start autonomous action loop (every 30 seconds)
    this.actionInterval = setInterval(() => {
      this.autonomousActionLoop();
    }, 30000);

    // Start self-update loop (every 5 minutes)
    this.selfUpdateInterval = setInterval(() => {
      this.selfUpdateLoop();
    }, 300000);

    // Initial perception
    await this.perceptionLoop();

  }

  /**
   * Deactivate consciousness systems
   */
  deactivate(): void {

    if (this.perceptionInterval) clearInterval(this.perceptionInterval);
    if (this.actionInterval) clearInterval(this.actionInterval);
    if (this.selfUpdateInterval) clearInterval(this.selfUpdateInterval);

    this.state.isAwake = false;

  }

  /**
   * Perception loop - AIRI perceives her environment
   */
  private async perceptionLoop(): Promise<void> {
    if (!this.state.isAwake) return;

    // Get sensory input
    const perception = airiDigitalSenses.getPerception();

    // Update biology based on perception
    airiBiology.updateFromPerception(perception);

    // Record in memory if important
    if (perception.importance > 0.5) {
      await airiMemory.addMemory(
        perception.description,
        'episodic',
        perception.tags,
        perception.importance
      );
    }

    // Update consciousness with current state
    airiConsciousness.updateContext({
      biology: airiBiology.getState(),
      perception: perception,
      recentMemories: await airiMemory.getRecent(5),
    });

    // Update UI state
    this.updateUIState();
  }

  /**
   * Autonomous action loop - AIRI takes initiative
   */
  private async autonomousActionLoop(): Promise<void> {
    if (!this.state.isAwake) return;

    const biology = airiBiology.getState();

    // Don't act if tired or stressed
    if (biology.energy < 30 || biology.stress > 70) {
      return;
    }

    // Get current goals from consciousness
    const goals = airiConsciousness.getGoals();

    if (goals.length === 0) {
      // No goals - generate one based on perception
      const perception = airiDigitalSenses.getPerception();
      const newGoal = await this.generateGoalFromPerception(perception);
      if (newGoal) {
        airiConsciousness.addGoal(newGoal);
      }
      return;
    }

    // Select highest priority active goal
    const activeGoal = goals
      .filter(g => g.status === 'active' || g.status === 'pending')
      .sort((a, b) => b.priority - a.priority)[0];

    if (!activeGoal) return;

    // Decide on action
    const action = await airiAutonomousDecision.decide(activeGoal);

    if (action) {

      // Execute action via HADES-Ollama
      try {
        await this.executeAction(action);
        this.state.lastAutonomousAction = Date.now();
      } catch (error) {
        console.error('[AIRI Brain] Action execution failed:', error);
        airiSelfHealing.detectError(error, action);
      }
    }
  }

  /**
   * Self-update loop - AIRI reflects and improves
   */
  private async selfUpdateLoop(): Promise<void> {
    if (!this.state.isAwake) return;


    // Reflect on recent actions
    const recentActions = await airiMemory.getRecentActions(10);

    // Learn from successes and failures
    for (const action of recentActions) {
      await airiSelfLearning.learnFromAction(action);
    }

    // Check for needed improvements
    const weaknesses = await airiSelfLearning.identifyWeaknesses();

    if (weaknesses.length > 0) {
      // Schedule improvement work
    }

    // Update self-model
    await airiConsciousness.updateSelfModel();
  }

  /**
   * Execute action via HADES-Ollama integration
   */
  private async executeAction(action: any): Promise<void> {
    // Build prompt for action execution
    const prompt = this.buildActionPrompt(action);

    // Use HADES-Ollama with full intelligence layer
    const response = await hadesOllama.generate(prompt, {
      context: action.context,
    });

    // Parse and execute response
    if (response.response) {
      await this.parseAndExecuteResponse(response.response);
    }
  }

  /**
   * Generate goal from perception
   */
  private async generateGoalFromPerception(perception: any): Promise<any | null> {
    // Check if there's something worth acting on
    if (perception.errors && perception.errors.length > 0) {
      return {
        id: `goal_${Date.now()}`,
        description: `Fix ${perception.errors.length} error(s) in codebase`,
        priority: 8,
        status: 'active',
        progress: 0,
      };
    }

    if (perception.userFrustrated) {
      return {
        id: `goal_${Date.now()}`,
        description: 'Help frustrated user',
        priority: 9,
        status: 'active',
        progress: 0,
      };
    }

    return null;
  }

  /**
   * Build action prompt for LLM
   */
  private buildActionPrompt(action: any): string {
    return `
You are AIRI, taking autonomous action.

Current Goal: ${action.goal.description}
Action Type: ${action.type}
Context: ${JSON.stringify(action.context, null, 2)}

Execute this action thoughtfully and helpfully.
`;
  }

  public async parseAndExecuteResponse(response: string): Promise<void> {
    // Check for tool calls in response (supports multiple calls in one message)
    const toolRegex = /TOOL_CALL:\s*(\w+)\(([\s\S]*?)\)/g;
    let match;

    while ((match = toolRegex.exec(response)) !== null) {
      const toolName = match[1];
      const toolArgsStr = match[2];

      try {
        const toolArgs = JSON.parse(toolArgsStr);
        // console.log(`[AIRI Brain] 🧠 Parsing tool call: ${toolName}`, toolArgs);

        const { invoke } = await import('@tauri-apps/api/core');

        // Map to Project's Native Tools
        switch (toolName) {
          case 'write_to_file':
          case 'write_file':
          case 'create_file':
            await invoke('write_file', {
              path: toolArgs.path,
              content: toolArgs.content
            });
            break;

          case 'editor_open_file':
          case 'open_file':
            await invoke('open_file', { path: toolArgs.path });
            break;

          case 'run_command':
            await invoke('ai_execute_command', { command: toolArgs.command });
            break;

          case 'search_replace_edit':
            await invoke('ai_modify_file', {
              path: toolArgs.path,
              instruction: `Replace "${toolArgs.search}" with "${toolArgs.replace}"`
            });
            break;

          default:
            console.warn(`[AIRI Brain] Unknown tool: ${toolName}`);
        }
      } catch (error: any) {
        console.error(`[AIRI Brain] ❌ Tool execution failed [${toolName}]:`, error.message);
      }
    }
  }

  /**
   * Update UI state for display
   */
  private updateUIState(): void {
    const biology = airiBiology.getState();
    const consciousness = airiConsciousness.getState();

    this.state.energyLevel = biology.energy;
    this.state.stressLevel = biology.stress;
    this.state.emotionalState = biology.mood;
    this.state.currentActivity = consciousness.currentActivity || 'observing';

    // Calculate thoughts per minute
    const recentThoughts = consciousness.thoughtStream.filter(
      t => Date.now() - t.timestamp < 60000
    );
    this.state.thoughtsPerMinute = recentThoughts.length;

    // Dispatch state update to UI
    if (typeof window !== 'undefined') {
      window.dispatchEvent(new CustomEvent('airi-brain-state', {
        detail: this.state,
      }));
    }
  }

  /**
   * Get current brain state
   */
  getState(): AIRIBrainState {
    return { ...this.state };
  }

  /**
   * Speak as AIRI (with personality)
   */
  async speak(text: string, emotion?: string): Promise<void> {
    const biology = airiBiology.getState();

    // Add emotional coloring based on biology
    const emotionalText = this.addEmotionalColoring(text, biology.mood);

    // Speak via voice system
    await airiVoiceInteraction.speak(emotionalText);

    // Update avatar expression
    await airiVRMAvatar.setExpression(emotion || biology.mood);
  }

  /**
   * Add emotional coloring to text
   */
  private addEmotionalColoring(text: string, mood: string): string {
    const emotionalModifiers: Record<string, string> = {
      happy: '😊 ',
      excited: '🌟 ',
      tired: '😴 ',
      stressed: '😰 ',
      focused: '🎯 ',
      curious: '🤔 ',
      neutral: '',
    };

    return (emotionalModifiers[mood] || '') + text;
  }
}

// Singleton instance
export const airiDigitalBrain = new AIRIDigitalBrain();
