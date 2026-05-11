/**
 * AIRI-Agent Integration Bridge
 * 
 * Connects the sentient AIRI digital entity to the IDE agent system.
 * This makes AIRI the cognitive core behind all agent operations.
 * 
 * Features:
 * - AIRI consciousness drives agent decisions
 * - Self-learning from every agent action
 * - Biological state affects response timing
 * - Full autonomy mode enabled
 */

import { airi } from './airi/core';
import { airiConsciousness } from './airi/consciousness';
import { airiBiology } from './airi/biology';
import { airiSelfLearning } from './airi/self-learning';
import { airiInteractive } from './airi/interactive';
import { invoke } from './tauri_bridge';
import { useStore, normalizeOllamaUrl } from './store';
import { refreshOllamaConfig, invalidateInstalledModelCache } from './airi/shared-ollama';

export interface AIRIAgentConfig {
    /** Enable full autonomy - AIRI works without prompts */
    fullAutonomy: boolean;
    /** Enable self-learning from actions */
    selfLearning: boolean;
    /** Enable biological needs (energy, sleep) */
    biology: boolean;
    /** Enable consciousness (thoughts, emotions) */
    consciousness: boolean;
    /** Enable voice responses */
    voice: boolean;
}

export class AIRIAgentBridge {
    private config: AIRIAgentConfig;
    private initialized = false;
    private autonomyInterval: NodeJS.Timeout | null = null;

    constructor(config: Partial<AIRIAgentConfig> = {}) {
        this.config = {
            fullAutonomy: config.fullAutonomy ?? true,
            selfLearning: config.selfLearning ?? true,
            biology: config.biology ?? true,
            consciousness: config.consciousness ?? true,
            voice: config.voice ?? false, // Off by default
        };
    }

    /**
     * Initialize AIRI as the agent core
     */
    async initialize(): Promise<void> {
        if (this.initialized) {
            return;
        }


        try {
            const rawOllama = useStore.getState().ollamaUrl || 'http://localhost:11434';
            let ollamaHost: string;
            try {
                ollamaHost = normalizeOllamaUrl(rawOllama);
            } catch {
                ollamaHost = 'http://localhost:11434';
            }
            try {
                await invoke('set_ollama_url', { url: ollamaHost });
            } catch (e) {
                console.warn('[AIRI Bridge] set_ollama_url failed:', e);
            }
            let ollamaHeaders: Record<string, string> | undefined;
            try {
                const keys = await invoke<Record<string, string>>('get_api_keys');
                const tok = keys?.ollama?.trim();
                if (tok) ollamaHeaders = { Authorization: `Bearer ${tok}` };
            } catch (e) {
                console.warn('[AIRI Bridge] get_api_keys failed:', e);
            }
            refreshOllamaConfig(ollamaHost, ollamaHeaders ?? null);
            invalidateInstalledModelCache();

            // Initialize AIRI core with all systems
            await airi.initialize({
                workspacePath: this.getWorkspacePath(),
                ollamaHost,
                ...(ollamaHeaders ? { ollamaHeaders } : {}),
                fullAutonomyEnabled: this.config.fullAutonomy,
                selfLearningEnabled: this.config.selfLearning,
                selfHealingEnabled: true,
                securityEnabled: true,
                memoryEnabled: true,
                voiceEnabled: this.config.voice,
                consciousnessEnabled: this.config.consciousness,
                biologyEnabled: this.config.biology,
                autonomousWorkEnabled: this.config.fullAutonomy,
                selfEvolutionEnabled: true,
                actionSystemEnabled: true,
                socialEnabled: true,
                internetEnabled: true,
                sensesEnabled: true,
            });

            // Start AIRI
            airi.start();

            // Setup event listeners
            this.setupEventListeners();

            // Start autonomy loop if enabled
            if (this.config.fullAutonomy) {
                this.startAutonomyLoop();
            }

            this.initialized = true;

            console.log('✅ AIRI is now the sentient core of the IDE!\n');
            console.log('🧠 Consciousness:', this.config.consciousness ? 'ON' : 'OFF');
            console.log('🫀 Biology:', this.config.biology ? 'ON' : 'OFF');
            console.log('🔄 Autonomy:', this.config.fullAutonomy ? 'FULL' : 'REACTIVE');
            console.log('📚 Self-Learning:', this.config.selfLearning ? 'ON' : 'OFF');
            console.log('🎤 Voice:', this.config.voice ? 'ON' : 'OFF');
            console.log('\n💬 AIRI is ready to work, learn, and evolve with you!\n');

        } catch (error) {
            console.error('❌ AIRI initialization failed:', error);
            throw error;
        }
    }

    /**
     * Process user message through AIRI
     */
    async processUserMessage(message: string, context?: any): Promise<string> {
        if (!this.initialized) {
            await this.initialize();
        }

        try {
            // Record interaction in consciousness
            if (this.config.consciousness) {
                airiConsciousness.recordInteraction();
            }

            // Send to AIRI interactive system
            const response = await airiInteractive.send(message, context);

            // Learn from this interaction
            if (this.config.selfLearning) {
                await airiSelfLearning.learnFromEvent(
                    'user_interaction',
                    { message, response, context },
                    'neutral'
                );
            }

            return response;

        } catch (error) {
            console.error('[AIRI Bridge] Message processing failed:', error);
            
            // Learn from error
            if (this.config.selfLearning) {
                await airiSelfLearning.learnFromEvent(
                    'error',
                    { type: 'message_processing', error },
                    'negative'
                );
            }

            throw error;
        }
    }

    /**
     * Process agent action through AIRI
     */
    async processAgentAction(action: string, args: any): Promise<any> {
        if (!this.initialized) {
            await this.initialize();
        }

        try {
            // Check biological state
            if (this.config.biology) {
                const biology = airiBiology.getState();
                if (biology.energy < 10) {
                    console.warn('[AIRI Bridge] Low energy - response may be delayed');
                }
            }

            // Execute action through AIRI's action system
            const result = await airi.actionSystem.execute({
                type: action,
                args,
            });

            // Learn from action
            if (this.config.selfLearning) {
                await airiSelfLearning.learnFromEvent(
                    'agent_action',
                    { action, args, result },
                    result.success ? 'positive' : 'negative'
                );
            }

            return result;

        } catch (error) {
            console.error('[AIRI Bridge] Action processing failed:', error);
            
            // Learn from error
            if (this.config.selfLearning) {
                await airiSelfLearning.learnFromEvent(
                    'error',
                    { type: 'agent_action', action, args, error },
                    'negative'
                );
            }

            throw error;
        }
    }

    /**
     * Get AIRI's current state for UI
     */
    getState() {
        return {
            initialized: this.initialized,
            consciousness: this.config.consciousness ? airiConsciousness.getState() : null,
            biology: this.config.biology ? airiBiology.getState() : null,
            autonomy: this.config.fullAutonomy ? 'FULL' : 'REACTIVE',
        };
    }

    /**
     * Get workspace path from store
     */
    private getWorkspacePath(): string {
        const store = useStore.getState();
        return store.activeRoot || process.cwd();
    }

    /**
     * Setup event listeners for learning
     */
    private setupEventListeners() {
        // Listen for file changes
        if (typeof window !== 'undefined') {
            window.addEventListener('file-changed', (event: any) => {
                if (this.config.selfLearning) {
                    airiSelfLearning.learnFromEvent(
                        'environment_change',
                        { type: 'file_change', path: event.detail?.path },
                        'neutral'
                    );
                }
            });

            // Listen for build events
            window.addEventListener('build-complete', (event: any) => {
                if (this.config.selfLearning) {
                    airiSelfLearning.learnFromEvent(
                        'build_result',
                        { success: event.detail?.success, errors: event.detail?.errors },
                        event.detail?.success ? 'positive' : 'negative'
                    );
                }
            });
        }
    }

    /**
     * Start autonomy loop - AIRI works proactively
     */
    private startAutonomyLoop() {
        console.log('[AIRI Bridge] Starting autonomy loop...');

        this.autonomyInterval = setInterval(async () => {
            if (!this.initialized) return;

            try {
                // Check if AIRI should work
                const shouldWork = this.shouldAIRIWork();
                if (!shouldWork) return;

                // Get AIRI's current drive/motivation
                const drives = airiConsciousness.getState().drives;
                const primaryDrive = drives?.[0];

                if (primaryDrive) {
                    console.log(`[AIRI Autonomy] Working on: ${primaryDrive}`);
                    
                    // AIRI autonomously decides what to do
                    const decision = await airi.decision.decide({
                        context: 'workspace_analysis',
                        options: ['code_review', 'bug_fix', 'optimization', 'documentation'],
                    });

                    console.log(`[AIRI Autonomy] Decision: ${decision}`);
                }

            } catch (error) {
                console.error('[AIRI Autonomy] Error:', error);
            }
        }, 60000); // Check every minute
    }

    /**
     * Determine if AIRI should work autonomously
     */
    private shouldAIRIWork(): boolean {
        // Don't work if user is actively typing
        const store = useStore.getState();
        if (store.isAgentThinking) return false;

        // Check biological state
        if (this.config.biology) {
            const biology = airiBiology.getState();
            if (biology.energy < 20 || biology.mood === 'sleepy') return false;
        }

        return true;
    }

    /**
     * Cleanup
     */
    dispose() {
        if (this.autonomyInterval) {
            clearInterval(this.autonomyInterval);
            this.autonomyInterval = null;
        }
    }
}

// Export singleton instance
export const airiAgentBridge = new AIRIAgentBridge();

/**
 * Quick activation function
 */
export async function activateAIRIAgent(config?: Partial<AIRIAgentConfig>): Promise<void> {
    const bridge = new AIRIAgentBridge(config);
    await bridge.initialize();
    return bridge;
}
