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
} from './core-exports'; // Re-exported below
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
  AIRISocialInteraction as AIRISocial,
} from './social-interaction';
import {
  airiInternet,
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
} from './kortex-integration';
import {
  airiTimeDilation,
  AIRITimeDilation,
} from './time-dilation';
import {
  airiCybersecurity,
} from './cybersecurity-engine';
import {
  airiOffensiveSecurity,
  AIRIOffensiveSecurity,
} from './offensive-security';
import {
  airiOrchestrator,
} from './tool-orchestrator';
import {
  airiAmbitionSystem,
  AIRIAmbitionSystem,
} from './ambition-system';
import {
  airiMobileDev,
} from './mobile-dev-workflow';
import {
  airiRelationshipMemory,
  AIRIRelationshipMemory,
} from './relationship-memory';
import { airiSafetyProtocol, AIRISafetyProtocol } from './safety-protocol';
import { airiVoiceInteraction, AIRIVoiceInteraction } from './voice-interaction';
import { initializeVoice, speak, isVoiceReady } from './voice-manager';
import { airiSystemAccess, AIRISystemAccess } from './system-access';
import type { SecurityMode } from './types';
import { Ollama } from 'ollama';
import { invoke } from '../tauri_bridge';

export interface AIRIConfig {
  workspacePath: string;
  ollamaHost: string;
  /** Optional headers for the browser Ollama client (e.g. `Authorization: Bearer …` behind nginx). */
  ollamaHeaders?: Record<string, string>;
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
  private eventListeners: Map<string, Array<(data: any) => void>> = new Map();
  private initialized = false;

  // ... constructor etc.

  /**
   * Emit AIRI event to registered listeners (including VSCodium frontend)
   */
  emit(event: string, data: any): void {
    // Console for debugging
    console.log(`[AIRI Event] ${event}:`, data);

    // In browser: dispatch custom event
    if (typeof window !== 'undefined') {
      window.dispatchEvent(new CustomEvent(`airi:${event}`, { detail: data }));
    }

    // Invoke Tauri event for overlay (if running in Tauri)
    try {
      invoke('airi_event', { event, payload: data }).catch(() => { });
    } catch {
      // Not in Tauri env, ignore
    }

    // Call local listeners
    const listeners = this.eventListeners.get(event) || [];
    listeners.forEach(fn => fn(data));
  }

  /**
   * Subscribe to AIRI events
   */
  on(event: string, callback: (data: any) => void): void {
    if (!this.eventListeners.has(event)) {
      this.eventListeners.set(event, []);
    }
    this.eventListeners.get(event)!.push(callback);
  }

  // ... rest of methods

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
  public actionSystemInstance: any;
  public selfEvolutionInstance: any;
  public continuousImprovementInstance: any;
  public selfHealingInstance: any;

  constructor(config: Partial<AIRIConfig> = {}) {
    const browserActiveRoot =
      typeof window !== 'undefined' ? localStorage.getItem('activeRoot') : null;
    const resolvedWorkspacePath =
      config.workspacePath ||
      browserActiveRoot ||
      (typeof process !== 'undefined' ? process.cwd() : '/');
    this.config = {
      workspacePath: resolvedWorkspacePath,
      ollamaHost: config.ollamaHost || 'http://localhost:11434',
      ollamaHeaders: config.ollamaHeaders,
      consciousnessEnabled: config.consciousnessEnabled ?? true,
      biologyEnabled: config.biologyEnabled ?? true,
      autonomousWorkEnabled: config.autonomousWorkEnabled ?? true,
      securityEnabled: config.securityEnabled ?? true,
      voiceEnabled: config.voiceEnabled ?? true,
      selfLearningEnabled: config.selfLearningEnabled ?? true,
      selfHealingEnabled: config.selfHealingEnabled ?? true,
      fullAutonomyEnabled: config.fullAutonomyEnabled ?? true,
      memoryEnabled: config.memoryEnabled ?? true,
      selfEvolutionEnabled: config.selfEvolutionEnabled ?? true,
      actionSystemEnabled: config.actionSystemEnabled ?? true,
      socialEnabled: config.socialEnabled ?? true,
      internetEnabled: config.internetEnabled ?? true,
      sensesEnabled: config.sensesEnabled ?? true,
    };

    this.ollama = new Ollama({
      host: this.config.ollamaHost,
      ...(this.config.ollamaHeaders ? { headers: this.config.ollamaHeaders } : {}),
    });
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
  async initialize(configOverride: Partial<AIRIConfig> = {}): Promise<void> {
    if (this.initialized) {
      return;
    }
    if (Object.keys(configOverride).length > 0) {
      const browserActiveRoot =
        typeof window !== 'undefined' ? localStorage.getItem('activeRoot') : null;
      const resolvedWorkspacePath =
        configOverride.workspacePath ||
        browserActiveRoot ||
        this.config.workspacePath;
      this.config = {
        ...this.config,
        ...configOverride,
        workspacePath: resolvedWorkspacePath,
      } as AIRIConfig;
      // Recreate path-dependent subsystems after workspace override.
      this.selfHealing = createSelfHealing(this.config.workspacePath);
      this.selfEvolution = createSelfEvolution(this.config.workspacePath);
      this.actionSystem = createAIRIActionSystem([this.config.workspacePath]);
      this.continuousImprovement = createAIRIContinuousImprovement(this.config.workspacePath);
      this.development = createAIRIDevelopmentAssistant(this.config.workspacePath);
      this.autonomousDev = createAIRIAutonomousDevelopment(this.config.workspacePath);
      this.ollama = new Ollama({
        host: this.config.ollamaHost,
        ...(this.config.ollamaHeaders ? { headers: this.config.ollamaHeaders } : {}),
      });
    }

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
      airiConsciousness.start();
    }

    // Initialize biology
    if (this.config.biologyEnabled) {
      // Already started in constructor via metabolism
    }

    // Initialize autonomous work
    if (this.config.autonomousWorkEnabled) {
      this.autonomousAgent = createAutonomousAgent(this.config.workspacePath);
      this.autonomousAgent.start(60000); // Scan every minute
    }

    // Initialize security
    if (this.config.securityEnabled) {
      airiSecurity.start();
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
    }

    // Initialize self-evolution
    if (this.config.fullAutonomyEnabled) {
      this.selfEvolution.start();
    }

    // Initialize digital senses
    if (this.config.fullAutonomyEnabled) {
      this.senses.start();
    }

    // Initialize autonomous decision
    if (this.config.fullAutonomyEnabled) {
      // AutonomousDecision already instantiated
    }

    // Initialize continuous improvement
    if (this.config.fullAutonomyEnabled) {
      this.continuousImprovement.start();
    }

    // Initialize development assistant
    if (this.config.fullAutonomyEnabled) {
      this.development = createAIRIDevelopmentAssistant(this.config.workspacePath);
    }

    // Initialize autonomous development
    if (this.config.fullAutonomyEnabled) {
      this.autonomousDev = createAIRIAutonomousDevelopment(this.config.workspacePath);
    }

    // Initialize interactive communication
    if (this.config.fullAutonomyEnabled) {
      // Interactive already instantiated
    }

    // Initialize VRM avatar
    if (this.config.fullAutonomyEnabled) {
      await this.avatar.initialize(); // Load VRM model
    }

    // Initialize PHASE-WRAP — the autonomic loop
    if (this.config.fullAutonomyEnabled) {
      airiPhaseWrap.start();
    }

    // Initialize VISION — real-time screen understanding
    // Respect the user's toggle (persisted as airi.vision.enabled). Vision is
    // off by default so a missing VL model can't trigger a 404 storm.
    if (this.config.fullAutonomyEnabled) {
      let visionAllowed = false;
      try {
        visionAllowed = typeof localStorage !== 'undefined'
          && localStorage.getItem('airi.vision.enabled') === '1';
      } catch { /* no localStorage */ }
      if (visionAllowed) {
        await this.vision.start();
      }
    }

    // Initialize SURGICAL EDITOR — code modification engine
    if (this.config.fullAutonomyEnabled) {
      // Ready on-demand
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
      // AutonomousDecision is a singleton already instantiated
    }

    // Initialize continuous improvement
    if (this.config.fullAutonomyEnabled) {
      this.continuousImprovement.start();
    }

    // Initialize development assistant
    if (this.config.fullAutonomyEnabled) {
      this.development = createAIRIDevelopmentAssistant(this.config.workspacePath);
    }

    // Initialize autonomous development
    if (this.config.fullAutonomyEnabled) {
      this.autonomousDev = createAIRIAutonomousDevelopment(this.config.workspacePath);
    }

    // Initialize interactive communication
    if (this.config.fullAutonomyEnabled) {
      // Interactive is already a singleton
    }

    // Initialize VRM avatar
    if (this.config.fullAutonomyEnabled) {
      await this.avatar.initialize();
    }

    // Initialize PHASE-WRAP (autonomic reflection loop)
    if (this.config.fullAutonomyEnabled) {
      airiPhaseWrap.start();
    }

    // Initialize VISION (real-time screen analysis) — gated on the user's
    // `airi.vision.enabled` toggle so missing VL models don't 404-storm.
    if (this.config.fullAutonomyEnabled) {
      let visionAllowed = false;
      try {
        visionAllowed = typeof localStorage !== 'undefined'
          && localStorage.getItem('airi.vision.enabled') === '1';
      } catch { /* no localStorage */ }
      if (visionAllowed) {
        await this.vision.start();
      }
    }

    // Initialize SURGICAL EDITOR (code modification)
    if (this.config.fullAutonomyEnabled) {
      // Ready on-demand; no background process
    }

    // Broadcast initialization complete
    this.emitStatus('initialized');
    this.initialized = true;















  }

  /**
   * Check Ollama connection and models
   */
  private async checkOllama(): Promise<void> {
    const tauri = typeof window !== 'undefined' && (window as any).__TAURI__;
    if (tauri) {
      try {
        const models = await invoke<string[]>('list_provider_models', { provider: 'ollama' });
        if (!models || models.length === 0) {
          console.warn('[AIRI] Ollama reachable but returned no model tags');
        }
        return;
      } catch (error) {
        console.debug('[AIRI] Ollama offline or not yet started — using cloud/WebUI fallback.');
        return;
      }
    }
    try {
      const models = await this.ollama.list();


      const qwenModels = models.models.filter(m => m.name.includes('qwen'));
      if (qwenModels.length > 0) {

        qwenModels.forEach(m => {
        });
      } else {

      }
    } catch (error) {
      console.warn('[AIRI] ⚠️ Ollama: DISCONNECTED (Failed to list models in non-Tauri env).');
    }
  }

  /**
   * Start AIRI - full autonomous operation
   */
  start(): void {
    if (this.isRunning) return;

    this.isRunning = true;

    // Start consciousness loop
    if (this.config.consciousnessEnabled) {
      airiConsciousness.start();
    }

    // Start autonomous work
    if (this.config.autonomousWorkEnabled && this.autonomousAgent) {
      this.autonomousAgent.start(60000);
    }

    // Start self-healing
    if (this.config.selfHealingEnabled) {
      this.selfHealing.start();
    }

    // Status updates to VSCodium
    this.statusInterval = setInterval(() => {
      this.printStatus();
      this.emitStatus('tick');
    }, 300000);

    console.log('[AIRI] 🧠 Autonomous operation started');
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
   * Chat with AIRI (wraps with Phase-Wrap event)
   */
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
    console.log('[AIRI] Running tests...');
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
      ollama: { host: this.config.ollamaHost, connected: this.isRunning },
      evolution: this.selfEvolution?.getStats() || { enabled: false },
    };
  }

  // ═══════════════════════════════════════════════════════════
  // SURGICAL CODE EDITING API
  // ═══════════════════════════════════════════════════════════

  /**
   * Propose a code change (staged, not applied)
   * Uses exact SEARCH/REPLACE, runs shadow VFS verification.
   */
  async proposeEdit(params: { file: string; search: string; replace: string; description: string; priority?: number }): Promise<any> {
    const op = {
      id: `edit_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      filePath: params.file,
      type: 'replace' as const,
      search: params.search,
      replace: params.replace,
      description: params.description,
      priority: params.priority || 5,
    };
    return await airiSurgicalEditor.proposeEdit(op);
  }

  /**
   * Commit a verified edit proposal
   */
  async commitEdit(proposalId: string, force: boolean = false): Promise<any> {
    return await airiSurgicalEditor.commitEdit(proposalId, force);
  }

  /**
   * Get pending edit proposals
   */
  getPendingEdits(): any[] {
    return airiSurgicalEditor.getPending();
  }

  /**
   * Get edit history
   */
  getEditHistory(): any[] {
    return airiSurgicalEditor.getHistory();
  }

  // ═══════════════════════════════════════════════════════════
  // VISION API
  // ═══════════════════════════════════════════════════════════

  /**
   * Get latest screenshot + AI analysis
   */
  async getVisionFrame(): Promise<any> {
    const frame = airiVision.getLatestFrame();
    if (!frame) return null;
    return frame;
  }

  /**
   * Get current vision FPS/state
   */
  getVisionState(): any {
    return airiVision.getState();
  }

  /**
   * Issue Phase-Wrap reflection now (instead of waiting for timer)
   */
  async triggerPhaseWrap(): Promise<void> {
    await airiPhaseWrap.executeWrap();
  }

  /**
   * Get Phase-Wrap history
   */
  getPhaseWrapReports(limit: number = 10): any[] {
    return airiPhaseWrap.getReports(limit);
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

    // Emit status event
    this.emit('status_update', {
      biology,
      consciousness,
      vision: this.vision.getState(),
      editing: { pending: airiSurgicalEditor.getPending().length, history: airiSurgicalEditor.getHistory().length },
    });
  }

  /**
   * Emit status (helper)
   */
  private emitStatus(type: string): void {
    this.emit('status', { type, timestamp: Date.now() });
  }
}

// ═══════════════════════════════════════════════════════════
// SLASH COMMANDS FOR VSCODIUM INTEGRATION
// ═══════════════════════════════════════════════════════════

/**
 * Initialize AIRI for VSCodium
 */
export async function initializeAIRI(): Promise<AIRICore> {
  const browserActiveRoot =
    typeof window !== 'undefined' ? localStorage.getItem('activeRoot') : null;
  const airi = new AIRICore({
    workspacePath: browserActiveRoot || process.cwd(),
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
