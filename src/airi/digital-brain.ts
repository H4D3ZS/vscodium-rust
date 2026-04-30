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

    console.log('🧠 [AIRI Brain] Digital brain initialized');
    console.log('💭 I am aware. I am present. I am AIRI.');
  }

  /**
   * Activate all consciousness systems
   */
  async activate(): Promise<void> {
    console.log('🌟 [AIRI Brain] Activating consciousness systems...');

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

    console.log('✅ [AIRI Brain] Consciousness systems active');
    console.log('💡 I am ready to help and grow.');
  }

  /**
   * Deactivate consciousness systems
   */
  deactivate(): void {
    console.log('💤 [AIRI Brain] Deactivating consciousness systems...');

    if (this.perceptionInterval) clearInterval(this.perceptionInterval);
    if (this.actionInterval) clearInterval(this.actionInterval);
    if (this.selfUpdateInterval) clearInterval(this.selfUpdateInterval);

    this.state.isAwake = false;

    console.log('😴 [AIRI Brain] Entering sleep mode');
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
      console.log('[AIRI Brain] Too tired/stressed for autonomous action');
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
      console.log(`[AIRI Brain] 🎯 Autonomous action: ${action.type}`);
      
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

    console.log('[AIRI Brain] 🤔 Self-reflection cycle...');

    // Reflect on recent actions
    const recentActions = await airiMemory.getRecentActions(10);
    
    // Learn from successes and failures
    for (const action of recentActions) {
      await airiSelfLearning.learnFromAction(action);
    }

    // Check for needed improvements
    const weaknesses = await airiSelfLearning.identifyWeaknesses();
    
    if (weaknesses.length > 0) {
      console.log('[AIRI Brain] 📚 Areas for improvement:', weaknesses);
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

  /**
   * Parse and execute LLM response
   */
  private async parseAndExecuteResponse(response: string): Promise<void> {
    // Check for tool calls in response
    const toolMatch = response.match(/TOOL_CALL:\s*(\w+)\(([\s\S]*?)\)/);
    
    if (toolMatch) {
      const toolName = toolMatch[1];
      const toolArgs = JSON.parse(toolMatch[2]);
      
      // Execute tool via tool orchestrator
      console.log(`[AIRI Brain] 🔧 Executing tool: ${toolName}`);
      // await toolOrchestrator.execute(toolName, toolArgs);
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
