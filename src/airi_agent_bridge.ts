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
    /** Enable Lemonade server integration */
    lemonadeEnabled: boolean;
}

export class AIRIAgentBridge {
    private config: AIRIAgentConfig;
    private initialized = false;
    private autonomyInterval: NodeJS.Timeout | null = null;
    private _onFileChanged: ((event: any) => void) | null = null;
    private _onBuildComplete: ((event: any) => void) | null = null;

    constructor(config: Partial<AIRIAgentConfig> = {}) {
        // Full autonomy (background loops that proactively hit the model + run
        // phase-wrap every 5 min) is OPT-IN. It was on by default, which made
        // AIRI churn the model and spam web fetches while the user was trying
        // to use the agent. Enable via localStorage 'airi.autonomous24x7'='1'.
        const autonomyOptIn = typeof localStorage !== 'undefined'
            && localStorage.getItem('airi.autonomous24x7') === '1';
        this.config = {
            fullAutonomy: config.fullAutonomy ?? autonomyOptIn,
            selfLearning: config.selfLearning ?? false,
            biology: config.biology ?? false,
            consciousness: config.consciousness ?? false,
            voice: config.voice ?? false,
            lemonadeEnabled: config.lemonadeEnabled ?? false,
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
            const rawOllama = useStore.getState().ollamaUrl || 'http://127.0.0.1:13305';
            let ollamaHost: string;
            try {
                ollamaHost = normalizeOllamaUrl(rawOllama);
            } catch {
                ollamaHost = 'http://127.0.0.1:13305';
            }
            try {
                await invoke('set_lemonade_url', { url: ollamaHost });
            } catch (e) {
                console.warn('[AIRI Bridge] set_lemonade_url failed:', e);
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
                memoryEnabled: true,
                voiceEnabled: this.config.voice,
                consciousnessEnabled: this.config.consciousness,
                biologyEnabled: this.config.biology,
                autonomousWorkEnabled: this.config.fullAutonomy,
                selfHealingEnabled: true,
                // selfEvolutionEnabled: true, // Removed duplicate or missing property
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
 console.error(' AIRI initialization failed:', error);
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
                    'conversation', // Validated event type
                    JSON.stringify({ message, response, context }),
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
                    JSON.stringify({ type: 'message_processing', error: String(error) }),
                    'failure'
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
            const result = await airi.actionSystemInstance.execute({
                type: action,
                args,
            });

            // Learn from action
            if (this.config.selfLearning) {
                await airiSelfLearning.learnFromEvent(
                    'success', // Use generic success for action result
                    JSON.stringify({ action, args, result }),
                    result.success ? 'success' : 'failure'
                );
            }

            return result;

        } catch (error) {
            console.error('[AIRI Bridge] Action processing failed:', error);

            // Learn from error
            if (this.config.selfLearning) {
                await airiSelfLearning.learnFromEvent(
                    'error',
                    JSON.stringify({ type: 'agent_action', action, args, error: String(error) }),
                    'failure'
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
        if (typeof window === 'undefined') return;

        // Store handler references so they can be removed on destroy
        this._onFileChanged = (event: any) => {
            if (this.config.selfLearning) {
                airiSelfLearning.learnFromEvent(
                    'observation',
                    JSON.stringify({ type: 'file_change', path: event.detail?.path }),
                    'neutral'
                );
            }
        };
        this._onBuildComplete = (event: any) => {
            if (this.config.selfLearning) {
                airiSelfLearning.learnFromEvent(
                    'success',
                    JSON.stringify({ type: 'build_result', success: event.detail?.success, errors: event.detail?.errors }),
                    event.detail?.success ? 'success' : 'failure'
                );
            }
        };

        window.addEventListener('file-changed', this._onFileChanged);
        window.addEventListener('build-complete', this._onBuildComplete);
    }

    /**
     * Remove event listeners — must be called on destroy to prevent OOM.
     */
    private removeEventListeners() {
        if (typeof window === 'undefined') return;
        if (this._onFileChanged) { window.removeEventListener('file-changed', this._onFileChanged); this._onFileChanged = null; }
        if (this._onBuildComplete) { window.removeEventListener('build-complete', this._onBuildComplete); this._onBuildComplete = null; }
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

                // Get AIRI's current motivation
                const thoughtState = airiConsciousness.getState();
                const primaryDrive = thoughtState.thoughts?.[0]?.content;

                if (primaryDrive) {
                    console.log(`[AIRI Autonomy] Working on: ${primaryDrive}`);

                    // AIRI autonomously decides what to do
                    const decision = await airi.decision.makeDecision('Autonomic workflow step', ['code_review', 'bug_fix', 'optimization']);
                    const actionName = decision.chosen.action;

                    console.log(`[AIRI Autonomy] Decision: ${actionName}`);
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
            if (biology.energy < 20 || biology.mood === 'tired') return false;
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
        this.removeEventListeners();
    }
}

// Export singleton instance
export const airiAgentBridge = new AIRIAgentBridge();

/**
 * Quick activation function
 */
export async function activateAIRIAgent(config?: Partial<AIRIAgentConfig>): Promise<AIRIAgentBridge> {
    const bridge = new AIRIAgentBridge(config);
    await bridge.initialize();
    return bridge;
}
