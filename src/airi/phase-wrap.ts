/**
 * AIRI Phase-Wrap Autonomic Loop
 * Continuous self-awareness feedback loop
 */

import { airiConsciousness } from './consciousness';
import { airiMemory } from './memory';
import { airiBiology } from './biology';
import { airiVRMAvatar } from './vrm-avatar';
import { invoke } from '../tauri_bridge';
import { getModel } from './model-config';
import { hadesOllama } from '../hades-ollama-service';

export interface PhaseWrapReport {
  timestamp: number;
  phase: 'reflect' | 'suture' | 'sync' | 'signal';
  summary: string;
  memoryUpdated?: boolean;
}

export class AIRIPhaseWrap {
  private reports: PhaseWrapReport[] = [];
  private isEnabled: boolean = true;
  private wrapInterval: any | null = null;
  private recentEvents: Array<{ type: string; data: any; time: number }> = [];

  constructor() { }

  start(): void {
    if (this.wrapInterval) return;
    this.wrapInterval = setInterval(() => {
      this.executeWrap().catch(() => { });
    }, 15 * 60 * 1000); // 15 min intervals
  }

  stop(): void {
    if (this.wrapInterval) {
      clearInterval(this.wrapInterval);
      this.wrapInterval = null;
    }
  }

  recordEvent(type: string, data: any): void {
    this.recentEvents.unshift({ type, data, time: Date.now() });
    if (this.recentEvents.length > 10) this.recentEvents.pop();
  }

  async executeWrap(): Promise<void> {
    if (!this.isEnabled) return;
    try {
      const reflection = await this.reflect();
      this.recordReport('reflect', reflection);
      await this.suture(reflection);
      await this.signal();
    } catch (error) {
      // console.error('[PhaseWrap] ❌ Cycle failed:', error);
    }
  }

  private async reflect(): Promise<string> {
    const summary = `Events: ${this.recentEvents.map(e => e.type).join(', ')}`;
    const prompt = `Reflect on recent events: ${summary}. What did you learn? Respond in 1 sentence.`;

    try {
      const response = await hadesOllama.generate(prompt, {
        model: getModel('self_learning'),
        stream: false,
        timeout: 20000
      });
      return response.response?.trim() || 'Nothing significant.';
    } catch {
      return 'Reflection failed.';
    }
  }

  private async suture(reflection: string): Promise<void> {
    try {
      await airiMemory.addMemory(reflection, 'semantic', ['reflection'], 0.7);
    } catch { }
  }

  private async signal(): Promise<void> {
    try {
      airiVRMAvatar.setEmotion('happy');
      setTimeout(() => airiVRMAvatar.setEmotion('neutral'), 1000);
      invoke('airi_event', { event: 'phase_wrap', payload: { reports: this.reports.slice(0, 4) } }).catch(() => { });
    } catch { }
  }

  getReports(limit: number = 10): PhaseWrapReport[] {
    return this.reports.slice(0, limit);
  }

  private recordReport(phase: PhaseWrapReport['phase'], summary: string): void {
    this.reports.unshift({ timestamp: Date.now(), phase, summary });
    if (this.reports.length > 50) this.reports.pop();
  }
}

export const airiPhaseWrap = new AIRIPhaseWrap();
