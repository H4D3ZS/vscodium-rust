/**
 * AIRI Core System Integration
 * Main entry point that connects ALL subsystems
 */

import {
  airiConsciousness,
  airiBiology,
  airiSecurity,
  airiSelfLearning,
  airiMemory,
  airiVRMAvatar,
  airiDigitalSenses,
  airiAutonomousDecision,
  AIRIConsciousness,
} from './core-exports';
import {
  createAutonomousAgent,
  AIRIAutonomousAgent,
} from './autonomous-agent';
import {
  createSelfHealing,
  AIRISelfHealing,
} from './self-healing';
import {
  createSelfEvolution,
  AIRISelfEvolution,
} from './true-self-evolution';
import {
  createAIRIActionSystem,
  AIRIActionSystem,
} from './action-system';
import {
  airiSocial,
  AIRISocialInteraction,
} from './social-interaction';
import {
  airiInternet,
  AIRIInternetAccess,
} from './internet-access';
import {
  airiDigitalSenses as airiSenses,
  AIRIDigitalSenses,
} from './digital-senses';
import {
  createAIRIContinuousImprovement,
  AIRIContinuousImprovement,
} from './continuous-improvement';
import {
  createAIRIDevelopmentAssistant,
  AIRIDevelopmentAssistant,
} from './development-assistant';
import {
  createAIRIAutonomousDevelopment,
  AIRIAutonomousDevelopment,
} from './autonomous-development';
import {
  airiInteractive,
  AIRIInteractive,
} from './interactive';
import {
  airiPhaseWrap,
  AIRIPhaseWrap,
} from './phase-wrap';
import {
  airiVision,
  AIRIVisionSystem,
} from './vision-system';
import {
  airiSurgicalEditor,
  AIRISurgicalEditor,
} from './surgical-editor';
import {
  airiKortex,
  AIRIKortexIntegration,
} from './kortex-integration';
import {
  airiTimeDilation,
  AIRITimeDilation,
} from './time-dilation';
import {
  airiCybersecurity,
  AIRICybersecurityEngine,
} from './cybersecurity-engine';
import {
  airiOffensiveSecurity,
  AIRIOffensiveSecurity,
} from './offensive-security';
import {
  airiOrchestrator,
  AIRIToolOrchestrator,
} from './tool-orchestrator';
import {
  airiAmbitionSystem,
  AIRIAmbitionSystem,
} from './ambition-system';
import {
  airiMobileDev,
  AIRIMobileDevelopment,
} from './mobile-dev-workflow';
import {
  airiRelationshipMemory,
  AIRIRelationshipMemory,
} from './relationship-memory';
import { airiSystemAccess, AIRISystemAccess } from './system-access';
import { hadesOllama } from '../hades-ollama-service';
import { getModel } from './model-config';
import { invoke } from '../tauri_bridge';
import { initializeVoice, isVoiceReady, speak } from './voice-manager';
import { airiSelfEvolution } from './true-self-evolution';
import { runTests } from './test-suite';
import { airiSafetyProtocol, AIRISafetyProtocol } from './safety-protocol';
import { airiVoiceInteraction, AIRIVoiceInteraction } from './voice-interaction';
import type { SecurityMode, Mood } from './types';
import type { BiologyState } from './biology';

export interface AIRIConfig {
  workspacePath: string;
  ollamaHost: string;
  consciousnessEnabled: boolean;
  biologyEnabled: boolean;
  autonomousWorkEnabled: boolean;
  securityEnabled: boolean;
  voiceEnabled: boolean;
  selfLearningEnabled: boolean;
  selfHealingEnabled: boolean;
  fullAutonomyEnabled: boolean;
  memoryEnabled: boolean;
  selfEvolutionEnabled: boolean;
  actionSystemEnabled: boolean;
  socialEnabled: boolean;
  internetEnabled: boolean;
  sensesEnabled: boolean;
}

export interface AIRIStatus {
  consciousness: { thoughts: any[], state: any };
  biology: BiologyState;
  security: SecurityMode;
  autonomous: { isRunning: boolean, lastScan: number };
  learning: { knowledgeBaseSize: number, learnedConcepts: number };
  healing: { healthScore: number, issuesFixed: number };
  decision: { totalDecisions: number, successRate: number };
  memory: { memoriesCount: number, recentActivity: any[] };
  voice: { isReady: boolean, currentVoice: string };
  ollama: { connected: boolean, model: string };
  evolution: { generation: number, fitness: number };
}

export class AIRICore {
  private config: AIRIConfig;
  private autonomousAgent: any;
  private selfHealing: any;
  private selfEvolution: any;
  public actionSystem: any;
  private continuousImprovement: any;
  private isRunning: boolean = false;
  private statusInterval: any | null = null;
  private eventListeners: Map<string, Array<(data: any) => void>> = new Map();

  // Public access to core systems
  public consciousness = airiConsciousness;
  public biology = airiBiology;
  public security = airiSecurity;
  public learning = airiSelfLearning;
  public decision = airiAutonomousDecision;
  public memory = airiMemory;
  public social = airiSocial;
  public internet = airiInternet;
  public senses = airiDigitalSenses;
  public development: AIRIDevelopmentAssistant;
  public autonomousDev: AIRIAutonomousDevelopment;
  public interactive = airiInteractive;
  public avatar = airiVRMAvatar;
  // Advanced systems
  public phaseWrap = airiPhaseWrap;
  public vision = airiVision;
  public surgicalEditor = airiSurgicalEditor;
  public kortex = airiKortex;
  public timeDilation = airiTimeDilation;
  public cybersecurity = airiCybersecurity;
  public offensiveSecurity = airiOffensiveSecurity;
  public orchestrator = airiOrchestrator;
  public ambition = airiAmbitionSystem;
  public mobileDev = airiMobileDev;
  public relationships = airiRelationshipMemory;
  public safety = airiSafetyProtocol;
  public voiceInteraction = airiVoiceInteraction;
  public systemAccess = airiSystemAccess;

  constructor(config: Partial<AIRIConfig> = {}) {
    this.config = {
      workspacePath: config.workspacePath || 'c:/Users/HADES/Desktop/vscodium-rust',
      ollamaHost: config.ollamaHost || 'http://localhost:11434',
      consciousnessEnabled: config.consciousnessEnabled ?? true,
      biologyEnabled: config.biologyEnabled ?? true,
      autonomousWorkEnabled: config.autonomousWorkEnabled ?? true,
      securityEnabled: config.securityEnabled ?? true,
      voiceEnabled: config.voiceEnabled ?? true,
      selfLearningEnabled: config.selfLearningEnabled ?? true,
      selfHealingEnabled: config.selfHealingEnabled ?? true,
      fullAutonomyEnabled: config.fullAutonomyEnabled ?? true,
      memoryEnabled: config.memoryEnabled ?? true,
      selfEvolutionEnabled: config.fullAutonomyEnabled ?? true,
      actionSystemEnabled: config.fullAutonomyEnabled ?? true,
      socialEnabled: config.fullAutonomyEnabled ?? true,
      internetEnabled: config.fullAutonomyEnabled ?? true,
      sensesEnabled: config.fullAutonomyEnabled ?? true,
    };

    this.selfHealing = createSelfHealing(this.config.workspacePath);
    this.selfEvolution = createSelfEvolution(this.config.workspacePath);
    this.actionSystem = createAIRIActionSystem([this.config.workspacePath]);
    this.continuousImprovement = createAIRIContinuousImprovement(this.config.workspacePath);
    this.development = createAIRIDevelopmentAssistant(this.config.workspacePath);
    this.autonomousDev = createAIRIAutonomousDevelopment(this.config.workspacePath);
  }

  emit(event: string, data: any): void {
    if (typeof window !== 'undefined') {
      window.dispatchEvent(new CustomEvent(`airi:${event}`, { detail: data }));
    }
    try {
      invoke('airi_event', { event, payload: data }).catch(() => { });
    } catch { }
    const listeners = this.eventListeners.get(event) || [];
    listeners.forEach(fn => fn(data));
  }

  on(event: string, callback: (data: any) => void): void {
    if (!this.eventListeners.has(event)) {
      this.eventListeners.set(event, []);
    }
    this.eventListeners.get(event)!.push(callback);
  }

  async initialize(config?: Partial<AIRIConfig>): Promise<void> {
    if (config) {
      this.config = { ...this.config, ...config };
    }

    airiSafetyProtocol.start();
    const corePromises = [
      airiKortex.load().catch(e => console.warn('[AIRI] Kortex failed to load:', e)),
      airiSelfLearning.initialize().catch(e => console.warn('[AIRI] Self-learning failed:', e)),
      airiMemory.initialize().catch(e => console.warn('[AIRI] Memory failed:', e))
    ];

    airiTimeDilation.start();
    airiConsciousness.start();
    airiRelationshipMemory.start();
    airiAmbitionSystem.start();

    const connectivityPromises = [
      this.checkOllama().catch(e => console.warn('[AIRI] Ollama connection failed:', e)),
      airiVoiceInteraction.initialize().catch(e => console.warn('[AIRI] Voice failed:', e))
    ];

    const sensoryPromises = [
      this.avatar.initialize().catch(e => console.error('[AIRI] Avatar init failed:', e)),
      this.vision.start().catch(e => console.error('[AIRI] Vision init failed:', e))
    ];

    airiCybersecurity.start();
    airiOffensiveSecurity.start();
    airiPhaseWrap.start();

    if (this.config.autonomousWorkEnabled) {
      this.autonomousAgent = createAutonomousAgent(this.config.workspacePath);
      this.autonomousAgent.start(60000);
    }

    if (this.config.selfHealingEnabled) {
      this.selfHealing.start();
    }

    this.selfEvolution.start();
    this.internet.start();
    this.senses.start();
    this.continuousImprovement.start();

    Promise.all(corePromises).then(() => {
      this.emitStatus('conscious');
    });

    Promise.all([...connectivityPromises, ...sensoryPromises]).then(() => {
      this.emitStatus('initialized');
    });

    this.emitStatus('initialized');
  }

  private async checkOllama(): Promise<void> {
    try {
      const models = await hadesOllama.list();
      const qwenModels = models.models.filter((m: any) => m.name.includes('qwen'));
      if (qwenModels.length === 0) {
        console.warn('[AIRI] No Qwen models found in Ollama.');
      }
    } catch (error) {
      console.error('[AIRI] ❌ Ollama: DISCONNECTED');
      throw error;
    }
  }

  start(): void {
    if (this.isRunning) return;
    this.isRunning = true;

    if (this.config.consciousnessEnabled) {
      airiConsciousness.start();
    }

    if (this.config.autonomousWorkEnabled && this.autonomousAgent) {
      this.autonomousAgent.start(60000);
    }

    if (this.config.selfHealingEnabled) {
      this.selfHealing.start();
    }

    this.statusInterval = setInterval(() => {
      this.emitStatus('tick');
    }, 300000);
  }

  stop(): void {
    this.isRunning = false;
    if (this.statusInterval) {
      clearInterval(this.statusInterval);
    }
    if (this.autonomousAgent) {
      this.autonomousAgent.stop();
    }
  }

  async chat(message: string): Promise<string> {
    airiPhaseWrap.recordEvent('user_message', { message: message.substring(0, 100) });
    this.avatar.setListening(true);
    const response = await this.interactive.send(message);
    this.avatar.reactToConversation(response);
    this.avatar.setListening(false);
    this.avatar.setSpeaking(true);

    if (isVoiceReady()) {
      await speak(response, 'airi', 5, () => {
        this.avatar.setSpeaking(false);
      });
    } else {
      this.avatar.setSpeaking(false);
    }

    airiPhaseWrap.recordEvent('assistant_reply', { length: response.length });
    return response;
  }

  async talk(message: string): Promise<string> {
    return this.chat(message);
  }

  async ask(question: string): Promise<string> {
    return this.chat(question);
  }

  setSecurityMode(mode: SecurityMode): void {
    airiSecurity.setMode(mode);
  }

  setAutonomy(level: 'passive' | 'active' | 'autonomous' | 'full'): void {
    airiConsciousness.setAutonomy(level);
  }

  getStatus(): AIRIStatus {
    const consciousnessState = airiConsciousness.getState();
    const learningStats = airiSelfLearning.getStats() as any;
    const decisionStats = airiAutonomousDecision.getStats() as any;
    const memoryStats = airiMemory.getStats() as any;
    const evolutionStats = (airiSelfEvolution as any)?.getStats() || { generation: 0, fitness: 0 };

    return {
      consciousness: { thoughts: (consciousnessState as any).thoughts || [], state: consciousnessState },
      biology: airiBiology.getState(),
      security: 'passive',
      autonomous: { isRunning: !!this.autonomousAgent, lastScan: Date.now() },
      learning: { knowledgeBaseSize: learningStats.totalKnowledge || 0, learnedConcepts: learningStats.recentEvents || 0 },
      healing: { healthScore: 100, issuesFixed: 0 },
      decision: { totalDecisions: decisionStats.total || 0, successRate: decisionStats.executed / (decisionStats.total || 1) },
      memory: { memoriesCount: memoryStats.total || 0, recentActivity: [] },
      voice: { isReady: isVoiceReady(), currentVoice: 'airi' },
      ollama: { connected: this.isRunning, model: getModel('consciousness') },
      evolution: { generation: evolutionStats.generation || 1, fitness: evolutionStats.fitness || 0.8 },
    };
  }

  emitStatus(status: string): void {
    this.emit('status', { status, timestamp: Date.now() });
  }

  private printStatus(): void {
    // legacy or debug
  }
}

export const airiCore = new AIRICore();
export const airi = airiCore;

// Re-export singletons for system-wide access
export { airiConsciousness, airiBiology, airiSecurity, airiSelfLearning, airiMemory, airiVRMAvatar, airiDigitalSenses, airiAutonomousDecision };
export { airiSocial } from './social-interaction';
export { airiInternet } from './internet-access';
export { airiPhaseWrap } from './phase-wrap';
export { airiVision } from './vision-system';
export { airiSurgicalEditor } from './surgical-editor';
export { airiKortex } from './kortex-integration';
export { airiTimeDilation } from './time-dilation';
export { airiCybersecurity } from './cybersecurity-engine';
export { airiOffensiveSecurity } from './offensive-security';
export { airiAmbitionSystem } from './ambition-system';
export { airiRelationshipMemory } from './relationship-memory';
export { airiSafetyProtocol } from './safety-protocol';
export { airiVoiceInteraction } from './voice-interaction';
export { airiSystemAccess } from './system-access';
export { airiDigitalBrain } from './digital-brain';
export { airiSelfHealing } from './self-healing';
export { airiAutonomousAgent } from './autonomous-agent';
export { airiDevelopmentAssistant } from './development-assistant';
export { airiAutonomousDevelopment } from './autonomous-development';
