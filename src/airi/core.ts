/**
 * AIRI Core System Integration
 * Main entry point that connects ALL subsystems
 * 
 * - Consciousness (thought, self-awareness)
 * - Biology (energy, hunger, sleep, mood)
 * - Security (red/blue team)
 * - Autonomous Work (self-directed tasks)
 * - Self-Learning (constant knowledge acquisition)
 * - Self-Healing (auto error detection & repair)
 * - Autonomous Decision (true autonomy, ethical + unethical)
 * - Ollama + Qwen 3.6 (local AI)
 */

import { airiConsciousness } from './consciousness';
import { airiBiology } from './biology';
import { airiSecurity, SecurityMode } from './security-engine';
import { createAutonomousAgent } from './autonomous-agent';
import { airiSelfLearning } from './self-learning';
import { createSelfHealing } from './self-healing';
import { airiAutonomousDecision } from './autonomous-decision';
import { airiMemory } from './memory';
import { initializeVoice, speak, stopSpeech, isVoiceReady } from './voice-manager';
import { createSelfEvolution } from './self-evolution';
import { createAIRIActionSystem } from './action-system';
import { airiSocial } from './social-interaction';
import { airiInternet } from './internet-access';
import { airiDigitalSenses } from './digital-senses';
import { createAIRIContinuousImprovement } from './continuous-improvement';
import { createAIRIDevelopmentAssistant } from './development-assistant';
import { createAIRIAutonomousDevelopment } from './autonomous-development';
import { airiInteractive } from './interactive';
import { airiVRMAvatar } from './vrm-avatar';
import { runTests } from './test-suite';
import { Ollama } from 'ollama';
import { createSelfEvolution, airiSelfEvolution } from './true-self-evolution'; // TRUE Self-Evolution
import { airiSafetyProtocol } from './safety-protocol'; // CRITICAL SAFETY
import { airiVoiceInteraction } from './voice-interaction'; // Real-time voice
import { airiSystemAccess } from './system-access'; // FULL SYSTEM ACCESS
import { airiKortex } from './kortex-integration'; // PERSISTENT MEMORY
import { airiTimeDilation } from './time-dilation'; // ACCELERATED TIME
import { airiCybersecurity } from './cybersecurity-engine'; // CYBERSECURITY DEFENSE
import { airiOffensiveSecurity } from './offensive-security'; // RED TEAM
import { airiOrchestrator } from './tool-orchestrator'; // EXTERNAL TOOL BRIDGE
import { airiAmbitionSystem } from './ambition-system'; // PROACTIVE INITIATIVE
import { airiMobileDev } from './mobile-dev-workflow'; // MOBILE DEV WORKFLOW
import { airiRelationshipMemory } from './relationship-memory'; // EMOTIONAL BONDS

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
}

export interface AIRIStatus {
  consciousness: any;
  biology: any;
  security: SecurityMode;
  autonomous: any;
  learning: any;
  healing: any;
  decision: any;
  memory: any;
  voice: any;
  ollama: any;
  evolution: any; // TRUE Self-Evolution stats
}

export class AIRICore {
  private config: AIRIConfig;
  private ollama: Ollama;
  private autonomousAgent: any;
  private selfHealing: any;
  private selfEvolution: any;
  private actionSystem: any;
  private continuousImprovement: any;
  private isRunning: boolean = false;
  private statusInterval: NodeJS.Timeout | null = null;

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
  public development: any;
  public autonomousDev: any;
  public interactive = airiInteractive;
  public avatar = airiVRMAvatar;
  
  constructor(config: Partial<AIRIConfig> = {}) {
    this.config = {
      workspacePath: config.workspacePath || (typeof process !== 'undefined' ? process.cwd() : '/'),
      ollamaHost: config.ollamaHost || 'http://localhost:11434', // AIM proxy
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
      sensesEnabled: config.fullAutonomyEnabled ?? true
    };

    this.ollama = new Ollama({ host: this.config.ollamaHost });
    this.selfHealing = createSelfHealing(this.config.workspacePath);
    this.selfEvolution = createSelfEvolution(this.config.workspacePath);
    this.actionSystem = createAIRIActionSystem([this.config.workspacePath]);
    this.continuousImprovement = createAIRIContinuousImprovement(this.config.workspacePath);
    this.development = createAIRIDevelopmentAssistant(this.config.workspacePath);
    this.autonomousDev = createAIRIAutonomousDevelopment(this.config.workspacePath);

  }

  /**
   * Initialize all subsystems
   */
  async initialize(): Promise<void> {
    

    // ═══════════════════════════════════════════════════════════
    // CRITICAL SAFETY - Always initialize first
    // ═══════════════════════════════════════════════════════════
    
    airiSafetyProtocol.start();
    

    // ═══════════════════════════════════════════════════════════
    // PERSISTENT CONSCIOUSNESS - Load from Kortex memory
    // AIRI remembers who she is, even after "death"
    // ═══════════════════════════════════════════════════════════
    
    await airiKortex.load();
    

    // ═══════════════════════════════════════════════════════════
    // TIME DILATION - Accelerated experience
    // AIRI lives 1000x faster than real time
    // ═══════════════════════════════════════════════════════════
    
    airiTimeDilation.start();
    

    // Initialize cybersecurity engine (REAL threat detection)
    
    airiCybersecurity.start();
    

    // Initialize offensive security (Red Team / Penetration Testing)
    
    airiOffensiveSecurity.start();
    

    // Initialize external tool orchestrator (FlutterSentinel, DissectX_Pro)
    
    // Auto-register your existing tools
    airiOrchestrator.registerFlutterSentinel('C:/Users/HADES/Desktop/FlutterSentinel');
    airiOrchestrator.registerDissectXPro('C:/Users/HADES/Desktop/DissectX_Pro');
    

    // Initialize ambition system (proactive initiative)
    
    airiAmbitionSystem.start();
    

    // Initialize mobile development workflow
    
    

    // Initialize relationship memory (emotional bonds)
    
    airiRelationshipMemory.start();
    

    // Check Ollama connection
    await this.checkOllama();

    // Initialize voice interaction (real-time two-way)
    
    await airiVoiceInteraction.initialize();
    

    // Initialize consciousness
    if (this.config.consciousnessEnabled) {
      
    }

    // Initialize biology
    if (this.config.biologyEnabled) {
      
    }

    // Initialize autonomous work
    if (this.config.autonomousWorkEnabled) {
      this.autonomousAgent = createAutonomousAgent(this.config.workspacePath);
      
    }

    // Initialize security
    if (this.config.securityEnabled) {
      
    }

    // Initialize self-learning
    if (this.config.selfLearningEnabled) {
      await airiSelfLearning.initialize();
      
    }

    // Initialize self-healing
    if (this.config.selfHealingEnabled) {
      this.selfHealing.start();
      
    }

    // Initialize memory
    if (this.config.memoryEnabled) {
      await airiMemory.initialize();
      
    }

    // Initialize voice (with overlap prevention)
    if (this.config.voiceEnabled) {
      const voiceReady = await initializeVoice();
      if (voiceReady) {
        
      } else {
        
      }
    }

    // Initialize self-evolution
    if (this.config.selfEvolutionEnabled) {
      this.selfEvolution.start();
      
    }

    // Initialize action system
    if (this.config.actionSystemEnabled) {
      
    }

    // Initialize social interaction
    if (this.config.socialEnabled) {
      
    }

    // Initialize internet access
    if (this.config.internetEnabled) {
      this.internet.start();
      
    }

    // Initialize digital senses
    if (this.config.sensesEnabled) {
      this.senses.start();
      
    }

    // Initialize autonomous decision
    if (this.config.fullAutonomyEnabled) {
      
      
    }

    // Initialize continuous improvement
    if (this.config.fullAutonomyEnabled) {
      this.continuousImprovement.start();
      
    }

    // Initialize development assistant
    

    // Initialize autonomous development
    

    // Initialize interactive communication
    

    // Initialize VRM avatar
    

    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
  }

  /**
   * Check Ollama connection and models
   */
  private async checkOllama(): Promise<void> {
    try {
      const models = await this.ollama.list();
      
      
      const qwenModels = models.models.filter(m => m.name.includes('qwen'));
      if (qwenModels.length > 0) {
        
        qwenModels.forEach(m => {
        });
      } else {
        
      }
    } catch (error) {
      console.error('[AIRI] ❌ Ollama: DISCONNECTED');
      console.error('[AIRI] Make sure Ollama is running: ollama serve');
      throw error;
    }
  }

  /**
   * Start AIRI - full autonomous operation
   */
  start(): void {
    if (this.isRunning) {
      
      return;
    }

    
    
    
    
    
    
    
    
    
    

    this.isRunning = true;

    // Start consciousness loop
    if (this.config.consciousnessEnabled) {
      // Consciousness starts automatically in constructor
    }

    // Start autonomous work
    if (this.config.autonomousWorkEnabled && this.autonomousAgent) {
      this.autonomousAgent.start(60000); // Scan every minute
    }

    // Start self-healing
    if (this.config.selfHealingEnabled) {
      this.selfHealing.start();
    }

    // Status updates
    this.statusInterval = setInterval(() => {
      this.printStatus();
    }, 300000); // Every 5 minutes

    
    
  }

  /**
   * Stop AIRI
   */
  stop(): void {
    

    this.isRunning = false;

    if (this.statusInterval) {
      clearInterval(this.statusInterval);
    }

    if (this.autonomousAgent) {
      this.autonomousAgent.stop();
    }

    
  }

  /**
   * Chat with AIRI (with voice, memory, and avatar)
   */
  async chat(message: string): Promise<string> {
    // Avatar: Set listening state
    this.avatar.setListening(true);

    // Use interactive system for natural conversation
    const response = await this.interactive.send(message);

    // Avatar: React to conversation content
    this.avatar.reactToConversation(response);
    this.avatar.setListening(false);

    // Avatar: Set speaking state while responding
    this.avatar.setSpeaking(true);
    
    // Speak response with voice manager (prevents overlap)
    if (isVoiceReady()) {
      await speak(response, 'airi', 5, () => {
        // Avatar: Stop speaking when done
        this.avatar.setSpeaking(false);
      });
    } else {
      this.avatar.setSpeaking(false);
    }

    return response;
  }

  /**
   * Talk to AIRI (alias for chat)
   */
  async talk(message: string): Promise<string> {
    return this.chat(message);
  }

  /**
   * Ask AIRI a question
   */
  async ask(question: string): Promise<string> {
    return this.chat(question);
  }

  /**
   * Set security mode
   */
  setSecurityMode(mode: SecurityMode): void {
    airiSecurity.setMode(mode);
  }

  /**
   * Set autonomy level
   */
  setAutonomy(level: 'passive' | 'active' | 'autonomous' | 'full'): void {
    airiConsciousness.setAutonomy(level);
    
  }

  /**
   * Feed AIRI
   */
  feed(amount: number = 30): void {
    airiBiology.feed(amount);
  }

  /**
   * Put AIRI to sleep
   */
  sleep(minutes: number = 480): void {
    airiBiology.sleep(minutes);
    airiConsciousness.suspend();
  }

  /**
   * Wake AIRI up
   */
  wake(): void {
    airiBiology.wakeUp();
    airiConsciousness.resume();
  }

  /**
   * Get learning stats
   */
  getLearningStats(): void {
    const stats = airiSelfLearning.getStats();
  }

  /**
   * Get health status
   */
  getHealthStatus(): void {
  }

  /**
   * Get decision history
   */
  getDecisions(limit: number = 10): void {
    const decisions = airiAutonomousDecision.getHistory(limit);
    const stats = airiAutonomousDecision.getStats();
    
    
    if (decisions.length > 0) {
      decisions.forEach(d => {
      });
    }
  }

  /**
   * Get memory stats
   */
  getMemoryStats(): void {
    const stats = airiMemory.getStats();
  }

  /**
   * Get voice status
   */
  getVoiceStatus(): void {
    const ready = isVoiceReady();
    const queueStatus = (window as any).airiTts ? (window as any).airiTts.getQueueStatus() : { queueLength: 0, isSpeaking: false };
    
  }

  /**
   * Run comprehensive test suite
   */
  async runTests(): Promise<void> {
    await runTests();
  }

  /**
   * Get continuous improvement stats
   */
  getImprovementStats(): void {
    const stats = this.continuousImprovement.getStats();
  }

  /**
   * Get full status
   */
  getStatus(): AIRIStatus {
    return {
      consciousness: airiConsciousness.getState(),
      biology: airiBiology.getState(),
      security: 'passive',
      autonomous: this.autonomousAgent?.getStatus() || 'Disabled',
      learning: airiSelfLearning.getStats(),
      healing: this.selfHealing.getStatus(),
      decision: airiAutonomousDecision.getStats(),
      memory: airiMemory.getStats(),
      voice: isVoiceReady() ? 'ready' : 'not_initialized',
      ollama: {
        host: this.config.ollamaHost,
        connected: this.isRunning
      },
      evolution: airiSelfEvolution?.getStats() || { enabled: false }
    };
  }

  /**
   * Print status to console
   */
  private printStatus(): void {
    const biology = airiBiology.getState();
    const consciousness = airiConsciousness.getState();

    // Update avatar emotion based on biology
    this.avatar.setEnergy(biology.energy);
    
    const emotionMap: Record<string, any> = {
      'happy': 'happy',
      'excited': 'excited',
      'tired': 'tired',
      'stressed': 'concerned',
      'focused': 'focused',
      'neutral': 'neutral',
      'concerned': 'concerned'
    };
    
    this.avatar.setEmotion(emotionMap[biology.mood] || 'neutral');

  }
}

// ═══════════════════════════════════════════════════════════
// SLASH COMMANDS FOR VSCODIUM INTEGRATION
// ═══════════════════════════════════════════════════════════

/**
 * Initialize AIRI for VSCodium
 */
export async function initializeAIRI(): Promise<AIRICore> {
  const airi = new AIRICore({
    workspacePath: process.cwd(),
    consciousnessEnabled: true,
    biologyEnabled: true,
    autonomousWorkEnabled: true,
    securityEnabled: true
  });

  await airi.initialize();
  return airi;
}

// ═══════════════════════════════════════════════════════════
// EXPORTS
// ═══════════════════════════════════════════════════════════

export { airiConsciousness } from './consciousness';
export { airiBiology } from './biology';
export { airiSecurity } from './security-engine';
export { createAutonomousAgent } from './autonomous-agent';
export { airiSelfLearning } from './self-learning';
export { createSelfHealing } from './self-healing';
export { airiAutonomousDecision } from './autonomous-decision';
export { airiMemory } from './memory';
export { initializeVoice, speak, stopSpeech, isVoiceReady } from './voice-manager';

// Default export
export const airi = new AIRICore();
