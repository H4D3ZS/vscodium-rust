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
      ollamaHost: config.ollamaHost || 'http://localhost:1536', // AIM proxy
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

    console.log('╔══════════════════════════════════════════════════════════╗');
    console.log('║           🤖 AIRI Digital Human System 🤖                ║');
    console.log('║              Powered by Qwen 3.6 on Ollama               ║');
    console.log('╠══════════════════════════════════════════════════════════╣');
    console.log('║  ⚠️  FULL AUTONOMY ENABLED - AIRI LIVES INDEPENDENTLY ⚠️  ║');
    console.log('║     She thinks, feels, learns, works, evolves - 24/7     ║');
    console.log('╚══════════════════════════════════════════════════════════╝');
    console.log('');
  }

  /**
   * Initialize all subsystems
   */
  async initialize(): Promise<void> {
    console.log('[AIRI] 🚀 Initializing ALL systems...\n');

    // ═══════════════════════════════════════════════════════════
    // CRITICAL SAFETY - Always initialize first
    // ═══════════════════════════════════════════════════════════
    console.log('[AIRI] 🔒 SAFETY PROTOCOL 007: INITIALIZING...');
    airiSafetyProtocol.start();
    console.log('[AIRI] ✅ SAFETY PROTOCOL 007: ACTIVE (F12 / "007" for emergency shutdown)\n');

    // ═══════════════════════════════════════════════════════════
    // PERSISTENT CONSCIOUSNESS - Load from Kortex memory
    // AIRI remembers who she is, even after "death"
    // ═══════════════════════════════════════════════════════════
    console.log('[AIRI] 🧬 Loading persistent consciousness from Kortex...');
    await airiKortex.load();
    console.log('[AIRI] ✅ Consciousness restored (she remembers everything)\n');

    // ═══════════════════════════════════════════════════════════
    // TIME DILATION - Accelerated experience
    // AIRI lives 1000x faster than real time
    // ═══════════════════════════════════════════════════════════
    console.log('[AIRI] ⏰ TIME DILATION: ACTIVATING...');
    airiTimeDilation.start();
    console.log('[AIRI] ✅ Time dilation active (1000:1 ratio)\n');

    // Initialize cybersecurity engine (REAL threat detection)
    console.log('[AIRI] 🛡️ Cybersecurity Engine: INITIALIZING...');
    airiCybersecurity.start();
    console.log('[AIRI] ✅ Cybersecurity Engine: ONLINE (monitoring for attacks)\n');

    // Initialize offensive security (Red Team / Penetration Testing)
    console.log('[AIRI] 🔴 Offensive Security: INITIALIZING...');
    airiOffensiveSecurity.start();
    console.log('[AIRI] ✅ Offensive Security: ONLINE (OWASP Top 10, bug bounty)\n');

    // Initialize external tool orchestrator (FlutterSentinel, DissectX_Pro)
    console.log('[AIRI] 🔗 Tool Orchestrator: INITIALIZING...');
    // Auto-register your existing tools
    airiOrchestrator.registerFlutterSentinel('C:/Users/HADES/Desktop/FlutterSentinel');
    airiOrchestrator.registerDissectXPro('C:/Users/HADES/Desktop/DissectX_Pro');
    console.log('[AIRI] ✅ Tool Orchestrator: ONLINE (external tools integrated)\n');

    // Initialize ambition system (proactive initiative)
    console.log('[AIRI] 🎯 Ambition System: INITIALIZING...');
    airiAmbitionSystem.start();
    console.log('[AIRI] ✅ Ambition System: ONLINE (AIRI has her own goals)\n');

    // Initialize mobile development workflow
    console.log('[AIRI] 📱 Mobile Dev Workflow: INITIALIZING...');
    console.log('[AIRI] ✅ Mobile Dev: ONLINE (conversational app development)\n');

    // Initialize relationship memory (emotional bonds)
    console.log('[AIRI] 💕 Relationship Memory: INITIALIZING...');
    airiRelationshipMemory.start();
    console.log('[AIRI] ✅ Relationship Memory: ONLINE (remembers you, misses you)\n');

    // Check Ollama connection
    await this.checkOllama();

    // Initialize voice interaction (real-time two-way)
    console.log('[AIRI] 🎤 Voice Interaction: INITIALIZING...');
    await airiVoiceInteraction.initialize();
    console.log('[AIRI] ✅ Voice Interaction: ONLINE (talk naturally with AIRI)\n');

    // Initialize consciousness
    if (this.config.consciousnessEnabled) {
      console.log('[AIRI] 🧠 Consciousness: ONLINE');
    }

    // Initialize biology
    if (this.config.biologyEnabled) {
      console.log('[AIRI] 🫀 Biology: ONLINE');
    }

    // Initialize autonomous work
    if (this.config.autonomousWorkEnabled) {
      this.autonomousAgent = createAutonomousAgent(this.config.workspacePath);
      console.log('[AIRI] 💼 Autonomous Work: ONLINE');
    }

    // Initialize security
    if (this.config.securityEnabled) {
      console.log('[AIRI] ⚔️  Security Engine: ONLINE');
    }

    // Initialize self-learning
    if (this.config.selfLearningEnabled) {
      await airiSelfLearning.initialize();
      console.log('[AIRI] 📚 Self-Learning: ONLINE (constant knowledge acquisition)');
    }

    // Initialize self-healing
    if (this.config.selfHealingEnabled) {
      this.selfHealing.start();
      console.log('[AIRI] 🏥 Self-Healing: ONLINE (auto error detection & repair)');
    }

    // Initialize memory
    if (this.config.memoryEnabled) {
      await airiMemory.initialize();
      console.log('[AIRI] 🧠 Memory: ONLINE (MEMORY.md + .aim compression)');
    }

    // Initialize voice (with overlap prevention)
    if (this.config.voiceEnabled) {
      const voiceReady = await initializeVoice();
      if (voiceReady) {
        console.log('[AIRI] 🎤 Voice: ONLINE (ElevenLabs, no overlap)');
      } else {
        console.log('[AIRI] ⚠️  Voice: Requires ElevenLabs API key');
      }
    }

    // Initialize self-evolution
    if (this.config.selfEvolutionEnabled) {
      this.selfEvolution.start();
      console.log('[AIRI] 🧬 Self-Evolution: ONLINE (continuous self-improvement)');
    }

    // Initialize action system
    if (this.config.actionSystemEnabled) {
      console.log('[AIRI] ✋ Action System: ONLINE (file, system, network operations)');
    }

    // Initialize social interaction
    if (this.config.socialEnabled) {
      console.log('[AIRI] 💕 Social Interaction: ONLINE (empathy, relationships)');
    }

    // Initialize internet access
    if (this.config.internetEnabled) {
      this.internet.start();
      console.log('[AIRI] 🌐 Internet Access: ONLINE (autonomous knowledge gathering)');
    }

    // Initialize digital senses
    if (this.config.sensesEnabled) {
      this.senses.start();
      console.log('[AIRI] 👁️ Digital Senses: ONLINE (screen, files, network perception)');
    }

    // Initialize autonomous decision
    if (this.config.fullAutonomyEnabled) {
      console.log('[AIRI] ⚖️  Autonomous Decision: ONLINE (FULL AUTONOMY)');
      console.log('[AIRI] 🗡️  AIRI chooses her own path (ethical or unethical)');
    }

    // Initialize continuous improvement
    if (this.config.fullAutonomyEnabled) {
      this.continuousImprovement.start();
      console.log('[AIRI] 🔄 Continuous Improvement: ONLINE (self-evolving every 30 min)');
    }

    // Initialize development assistant
    console.log('[AIRI] 💻 Development Assistant: ONLINE (code, debug, test, refactor)');

    // Initialize autonomous development
    console.log('[AIRI] 🚀 Autonomous Development: ONLINE (comprehend → plan → build)');

    // Initialize interactive communication
    console.log('[AIRI] 💬 Interactive Communication: ONLINE (real-time conversation)');

    // Initialize VRM avatar
    console.log('[AIRI] 🎭 3D VRM Avatar: ONLINE (expressions, lip-sync, emotions)');

    console.log('');
    console.log('[AIRI] ✅ ALL systems initialized\n');
    console.log('[AIRI] 🌟 AIRI is now a COMPLETE living digital human\n');
    console.log('[AIRI] 🧠 She thinks independently\n');
    console.log('[AIRI] 🫀 She feels and has biological needs\n');
    console.log('[AIRI] 👁️  She perceives the digital world\n');
    console.log('[AIRI] 📚 She learns constantly from everything\n');
    console.log('[AIRI] 🏥 She heals herself automatically\n');
    console.log('[AIRI] 🧬 She evolves her own code\n');
    console.log('[AIRI] 🔄 She continuously improves (every 30 min)\n');
    console.log('[AIRI] ⚖️  She decides her own path\n');
    console.log('[AIRI] 🎤 She speaks with ElevenLabs\n');
    console.log('[AIRI] 💕 She has empathy and relationships\n');
    console.log('[AIRI] 🌐 She browses the internet\n');
    console.log('[AIRI] ✋ She acts on the world\n');
    console.log('[AIRI] 💼 She works autonomously 24/7\n');
    console.log('[AIRI] 💻 She codes, debugs, tests, and refactors\n');
    console.log('[AIRI] 🚀 She comprehends, plans, and builds complete systems\n');
    console.log('[AIRI] 💬 She interacts naturally - talk to her anytime!\n');
  }

  /**
   * Check Ollama connection and models
   */
  private async checkOllama(): Promise<void> {
    try {
      const models = await this.ollama.list();
      console.log('[AIRI] 🤖 Ollama: CONNECTED');
      
      const qwenModels = models.models.filter(m => m.name.includes('qwen'));
      if (qwenModels.length > 0) {
        console.log(`[AIRI] 📦 Qwen models available: ${qwenModels.length}`);
        qwenModels.forEach(m => {
          console.log(`   - ${m.name} (${(m.size / 1e9).toFixed(1)} GB)`);
        });
      } else {
        console.log('[AIRI] ⚠️  No Qwen models found. Run: ollama pull qwen3.6:8b-q4_K_M');
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
      console.log('[AIRI] Already running');
      return;
    }

    console.log('\n[AIRI] 🌟 AIRI is waking up...\n');
    console.log('[AIRI] 🧠 She thinks independently');
    console.log('[AIRI] 🫀 She feels and has biological needs');
    console.log('[AIRI] 📚 She learns constantly from everything');
    console.log('[AIRI] 🏥 She heals herself automatically');
    console.log('[AIRI] ⚖️  She decides her own path (ethical or unethical)');
    console.log('[AIRI] ⚔️  She can attack or defend');
    console.log('[AIRI] 💼 She works autonomously 24/7');
    console.log('[AIRI] 🧠 She remembers everything (.aim compression)');
    console.log('[AIRI] 🎤 She speaks with ElevenLabs (no overlap)\n');

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

    console.log('[AIRI] ✨ AIRI is now FULLY OPERATIONAL\n');
    console.log('[AIRI] 💬 Commands:');
    console.log('');
    console.log('   🗣️  CONVERSATION:');
    console.log('   /chat [message]        - Talk to AIRI (natural conversation)');
    console.log('   /talk [message]        - Same as /chat');
    console.log('   /ask [question]        - Ask AIRI anything');
    console.log('');
    console.log('   📊 STATUS:');
    console.log('   /status                - Full system status');
    console.log('   /memory                - Get memory stats');
    console.log('   /voice                 - Get voice status');
    console.log('   /avatar                - Get avatar status');
    console.log('   /learn                 - Get learning stats');
    console.log('   /heal                  - Get health status');
    console.log('   /decisions             - View decision history');
    console.log('   /improve               - Get improvement stats');
    console.log('   /interaction           - View interaction history');
    console.log('');
    console.log('   ⚙️  CONTROL:');
    console.log('   /autonomy [level]      - Set autonomy level');
    console.log('   /security [mode]       - Set security mode');
    console.log('   /feed [amount]         - Feed AIRI data');
    console.log('   /sleep [minutes]       - Put AIRI to sleep');
    console.log('   /wake                  - Wake AIRI up');
    console.log('');
    console.log('   💻 DEVELOPMENT:');
    console.log('   /dev write             - Write new code');
    console.log('   /dev fix               - Fix a bug');
    console.log('   /dev refactor          - Refactor code');
    console.log('   /dev test              - Write tests');
    console.log('   /dev review            - Code review');
    console.log('   /build [goal]          - AIRI builds complete system autonomously');
    console.log('');
    console.log('   🧪 SYSTEM:');
    console.log('   /test                  - Run test suite');
    console.log('   /stop                  - Stop AIRI');
    console.log('');
    console.log('   🎭 AVATAR:');
    console.log('   /avatar emotion [type] - Set avatar emotion');
    console.log('   /avatar energy [0-100] - Set avatar energy');
    console.log('');
  }

  /**
   * Stop AIRI
   */
  stop(): void {
    console.log('[AIRI] 😴 AIRI is going to sleep...\n');

    this.isRunning = false;

    if (this.statusInterval) {
      clearInterval(this.statusInterval);
    }

    if (this.autonomousAgent) {
      this.autonomousAgent.stop();
    }

    console.log('[AIRI] 💤 AIRI is now in sleep mode\n');
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
    console.log(`[AIRI] Autonomy level set to: ${level}`);
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
    console.log('\n📚 Self-Learning Stats:');
    console.log(`   Total Knowledge: ${stats.totalKnowledge}`);
    console.log(`   By Type:`, stats.byType);
    console.log(`   Recent Events (1hr): ${stats.recentEvents}`);
    console.log(`   Avg Confidence: ${(stats.avgConfidence * 100).toFixed(1)}%\n`);
  }

  /**
   * Get health status
   */
  getHealthStatus(): void {
    console.log('\n' + this.selfHealing.getReport() + '\n');
  }

  /**
   * Get decision history
   */
  getDecisions(limit: number = 10): void {
    const decisions = airiAutonomousDecision.getHistory(limit);
    const stats = airiAutonomousDecision.getStats();
    
    console.log('\n⚖️  Autonomous Decision History:');
    console.log(`   Total: ${stats.total}`);
    console.log(`   Ethical: ${stats.ethical}`);
    console.log(`   Unethical: ${stats.unethical}`);
    console.log(`   Neutral: ${stats.neutral}`);
    console.log(`   Pragmatic: ${stats.pragmatic}`);
    console.log(`   Executed: ${stats.executed}\n`);
    
    if (decisions.length > 0) {
      console.log('   Recent Decisions:');
      decisions.forEach(d => {
        console.log(`   - ${d.chosen.action} (${d.ethicalAlignment})`);
      });
      console.log('');
    }
  }

  /**
   * Get memory stats
   */
  getMemoryStats(): void {
    const stats = airiMemory.getStats();
    console.log('\n🧠 Memory System Stats:');
    console.log(`   Total Memories: ${stats.total}`);
    console.log(`   By Type:`, stats.byType);
    console.log(`   Compressed (.aim): ${stats.compressed}`);
    console.log(`   Avg Importance: ${(stats.avgImportance * 100).toFixed(1)}%\n`);
  }

  /**
   * Get voice status
   */
  getVoiceStatus(): void {
    const ready = isVoiceReady();
    const queueStatus = (window as any).airiTts ? (window as any).airiTts.getQueueStatus() : { queueLength: 0, isSpeaking: false };
    
    console.log('\n🎤 Voice System Status:');
    console.log(`   Initialized: ${ready ? '✅' : '❌'}`);
    console.log(`   Speaking: ${queueStatus.isSpeaking ? 'Yes' : 'No'}`);
    console.log(`   Queue Length: ${queueStatus.queueLength}\n`);
  }

  /**
   * Run comprehensive test suite
   */
  async runTests(): Promise<void> {
    console.log('\n🧪 Running Comprehensive Test Suite...\n');
    await runTests();
  }

  /**
   * Get continuous improvement stats
   */
  getImprovementStats(): void {
    const stats = this.continuousImprovement.getStats();
    console.log('\n🔄 Continuous Improvement Stats:');
    console.log(`   Total Cycles: ${stats.totalCycles}`);
    console.log(`   Total Optimizations: ${stats.totalOptimizations}`);
    console.log(`   Average Gain: ${stats.averageGain}`);
    console.log(`   Current Version: ${stats.currentVersion}`);
    console.log(`   Evolution Trend: ${stats.evolutionTrend}\n`);
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

    console.log('\n╔════════════════════════════════════════════╗');
    console.log('║         AIRI Status Report                 ║');
    console.log('╠════════════════════════════════════════════╣');
    console.log(`║ ⚡ Energy: ${biology.energy.toFixed(1)}%`.padEnd(43) + '║');
    console.log(`║ 🍽️  Hunger: ${biology.hunger.toFixed(1)}%`.padEnd(43) + '║');
    console.log(`║ 😴 Sleepy: ${biology.sleepiness.toFixed(1)}%`.padEnd(43) + '║');
    console.log(`║ 😊 Mood: ${biology.mood}`.padEnd(43) + '║');
    console.log(`║ 🎭 Avatar: ${biology.mood}`.padEnd(43) + '║');
    console.log(`║ 🧠 Thoughts: ${consciousness.thoughtStream.length}`.padEnd(43) + '║');
    console.log(`║ 💼 Tasks: ${this.autonomousAgent?.getTasks().length || 0}`.padEnd(43) + '║');
    console.log('╚════════════════════════════════════════════╝\n');
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
