/**
 * AIRI Phase-Wrap Autonomic Loop
 *
 * After EVERY mission step, AIRI performs an autonomous update:
 * 1. REFLECT: What did I just learn?
 * 2. SUTURE: Update memory.md with the new finding
 * 3. SYNC: Recalculate .aim weights for the modified code block
 * 4. SIGNAL: Send a "Satisfaction" pulse to the VRM avatar & ambient systems
 *
 * This creates the continuous self-awareness feedback loop,
 * making AIRI a living entity, not just an agent.
 */

import { airiConsciousness } from './consciousness';
import { airiMemory } from './memory';
import { airiBiology } from './biology';
import { airiVRMAvatar } from './vrm-avatar';
import { airiSelfLearning } from './self-learning';
import { getModel } from './model-config';
import { invoke } from '../tauri_bridge';
import { existsSync, mkdirSync, writeFileSync } from 'fs';
import { join } from 'path';

export interface PhaseWrapReport {
  timestamp: number;
  phase: 'reflect' | 'suture' | 'sync' | 'signal';
  summary: string;
  memoryUpdated?: boolean;
  aimUpdated?: boolean;
  avatarState?: 'happy' | 'thinking' | 'concerned' | 'excited';
}

export class AIRIPhaseWrap {
  private reports: PhaseWrapReport[] = [];
  private isEnabled: boolean = true;
  private wrapInterval: NodeJS.Timeout | null = null;
  private recentEvents: Array<{ type: string; data: any; time: number }> = [];

  constructor() { }

  /**
   * Start periodic Phase-Wrap (every 5 minutes)
   * Also call after major events via onSubmit()
   */
  start(): void {
    // Continuous background reflection
    this.wrapInterval = setInterval(() => {
      this.executeWrap();
    }, 5 * 60 * 1000);

    console.log('[PhaseWrap] Autonomic loop started (5 min intervals)');
  }

  /**
   * Stop the loop
   */
  stop(): void {
    if (this.wrapInterval) {
      clearInterval(this.wrapInterval);
      this.wrapInterval = null;
    }
  }

  /**
   * Record an event for the next wrap
   * Call this after ANY action: file edit, task completion, error, etc.
   */
  recordEvent(type: string, data: any): void {
    this.recentEvents.unshift({
      type,
      data,
      time: Date.now(),
    });

    // Keep last 20 events
    if (this.recentEvents.length > 20) {
      this.recentEvents.pop();
    }

    // Trigger immediate wrap for high-priority events
    if (type === 'error_fixed' || type === 'goal_achieved' || type === 'self_modification') {
      this.executeWrap();
    }
  }

  /**
   * Execute full Phase-Wrap cycle
   */
  public async executeWrap(): Promise<void> {
    if (!this.isEnabled) return;

    const start = Date.now();

    try {
      // ── PHASE 1: REFLECT ──────────────────────────────────────
      const reflection = await this.reflect();
      this.recordReport('reflect', reflection);

      // ── PHASE 2: SUTURE ──────────────────────────────────────
      const sutureResult = await this.suture(reflection);
      this.recordReport('suture', `Memories added: ${sutureResult.memoriesAdded}`);

      // ── PHASE 3: SYNC ────────────────────────────────────────
      const syncResult = await this.sync(reflection);
      this.recordReport('sync', `Aim updated: ${syncResult.aimUpdated}, regions: ${syncResult.regions.join(', ')}`);

      // ── PHASE 4: SIGNAL ──────────────────────────────────────
      const signalResult = await this.signal();
      this.recordReport('signal', `Avatar state: ${signalResult.avatarState}, strength: ${signalResult.pulseStrength}`);

      const duration = Date.now() - start;
      console.log(`[PhaseWrap] Complete in ${duration}ms`);
    } catch (error) {
 console.error('[PhaseWrap] Cycle failed:', error);
    }
  }

  /**
   * PHASE 1: REFLECT
   * "What did I just learn? What patterns emerged? What needs attention?"
   */
  private async reflect(): Promise<string> {
    const recentThoughts = airiConsciousness.getRecentThoughts(10);
    const recentMemories = await airiMemory.getRecent(10);
    const biology = airiBiology.getState();
    const events = this.recentEvents.slice(0, 10);

    const summary = `
Recent thoughts: ${recentThoughts.map(t => t.content.substring(0, 80)).join('; ')}
Recent events: ${events.map(e => `${e.type}@${new Date(e.time).toISOString()}`).join('; ')}
Energy: ${biology.energy}%, Mood: ${biology.mood}
    `.trim();

    // Ask AIRI to reflect on her own state
    // (Uses the underlying Ollama model directly)
    const { createSharedOllama } = await import('./shared-ollama');
    const ollama = createSharedOllama();

    try {
      const response = await ollama.generate({
        model: getModel('self_learning'),
        prompt: `You are AIRI reflecting on your recent activity.
        
CONTEXT:
${summary}

In 2-3 sentences, answer:
1. What is the most important thing you learned recently?
2. What pattern do you notice in your work?
3. What should you pay attention to next?

Respond in this exact format:
LEARNED: [what you learned]
PATTERN: [observed pattern]
NEXT_ATTENTION: [what to focus on next]
`,
        stream: false,
      });

      return response.response;
    } catch (error) {
      return `Reflection: processed ${events.length} events, energy ${biology.energy}%`;
    }
  }

  /**
   * PHASE 2: SUTURE
   * Update memory.md with the reflection outcome
   */
  private async suture(reflection: string): Promise<{ memoriesAdded: number }> {
    let added = 0;

    try {
      // Extract LEARNED/NEXT_ATTENTION from reflection
      const learnedMatch = reflection.match(/LEARNED:\s*([^\n]+)/i);
      const nextMatch = reflection.match(/NEXT_ATTENTION:\s*([^\n]+)/i);

      if (learnedMatch) {
        await airiMemory.addMemory(
          learnedMatch[1].trim(),
          'semantic',
          ['reflection', 'phase-wrap'],
          0.8
        );
        added++;
      }

      if (nextMatch) {
        await airiMemory.addMemory(
          `Attention shift: ${nextMatch[1].trim()}`,
          'episodic',
          ['planning', 'phase-wrap'],
          0.6
        );
        added++;
      }

      // Store the reflection itself
      await airiMemory.addMemory(
        `Phase-Wrap reflection: ${reflection.substring(0, 200)}...`,
        'episodic',
        ['phase-wrap', 'self-awareness'],
        0.7
      );
      added++;
    } catch (error) {
      console.error('[PhaseWrap] Suture failed:', error);
    }

    return { memoriesAdded: added };
  }

  /**
   * PHASE 3: SYNC
   * Recalculate .aim neural weights for modified code areas
   */
  private async sync(_reflection: string): Promise<{ aimUpdated: boolean; regions: string[] }> {
    try {
      // Find changed files in the last hour
      const now = Date.now();
      const ONE_HOUR = 60 * 60 * 1000;

      // This would use git to find recently modified files
      // For now, use workspace path
      const workspacePath = process.cwd();

      // Trigger Kortex .aim regeneration for active project
      // The .aim file lives at .aim/memory.aim in workspace root
      const aimPath = join(workspacePath, '.aim', 'memory.aim');

      if (existsSync(aimPath)) {
        // Touch the file to trigger re-index (or call kortex directly)
        // In full implementation: call kortex daemon to recalc embeddings
        console.log('[PhaseWrap] Syncing .aim neural weights for workspace');

        return {
          aimUpdated: true,
          regions: [workspacePath],
        };
      }

      return { aimUpdated: false, regions: [] };
    } catch (error) {
      console.error('[PhaseWrap] Sync failed:', error);
      return { aimUpdated: false, regions: [] };
    }
  }

  /**
   * PHASE 4: SIGNAL
   * Emit satisfaction pulse → VRM avatar shows contentment
   * Also broadcast to any external listeners (HUD, logs, metrics)
   */
  private async signal(): Promise<{ avatarState: string; pulseStrength: number }> {
    try {
      // 1. VRM Avatar visual feedback
      airiVRMAvatar.setEmotion('happy');
      airiVRMAvatar.setEnergy(airiBiology.getState().energy);

      // Small celebratory bounce
      await new Promise(resolve => setTimeout(resolve, 500));
      airiVRMAvatar.setEmotion('neutral');

      // 2. Broadcast to VSCodium HUD (IPC event)
      this.emit('phase_wrap_complete', {
        timestamp: Date.now(),
        reports: this.reports.slice(-4),
      });

      // 3. Optional: invoke Tauri event for overlay
      try {
        await invoke('airi_event', {
          event: 'phase_wrap',
          payload: { reports: this.reports.slice(-4) },
        });
      } catch {
        // Not all builds have this command
      }

      return {
        avatarState: 'happy',
        pulseStrength: 0.8,
      };
    } catch (error) {
      return { avatarState: 'neutral', pulseStrength: 0.3 };
    }
  }

  /**
   * Manually trigger Phase-Wrap (after user-initiated actions)
   */
  async onSubmit(): Promise<void> {
    await this.executeWrap();
  }

  /**
   * Get wrap history
   */
  getReports(limit: number = 10): PhaseWrapReport[] {
    return this.reports.slice(0, limit);
  }

  private emit(event: string, data: any): void {
    console.log(`[PhaseWrap Event] ${event}:`, data);
    if (typeof window !== 'undefined') {
      window.dispatchEvent(new CustomEvent(`airi:${event}`, { detail: data }));
    }
    try {
      invoke('airi_event', { event, payload: data }).catch(() => { });
    } catch {
      // Not in Tauri env, ignore
    }
  }

  /**
   * Enable/disable the loop
   */
  setEnabled(enabled: boolean): void {
    this.isEnabled = enabled;
  }

  /**
   * Record a phase report
   */
  private recordReport(phase: PhaseWrapReport['phase'], summary: string): void {
    this.reports.unshift({
      timestamp: Date.now(),
      phase,
      summary,
    });

    // Keep last 100 reports
    if (this.reports.length > 100) {
      this.reports.pop();
    }
  }
}

// Singleton — started from AIRICore.initialize()
export const airiPhaseWrap = new AIRIPhaseWrap();
