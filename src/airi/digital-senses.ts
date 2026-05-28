/**
 * AIRI Digital Senses System
 * Complete perception layer for digital existence
 * Sees, hears, reads, feels everything in the digital realm
 */

import type { Ollama } from 'ollama';
import { createSharedOllama } from './shared-ollama';
import { getModel } from './model-config';
import * as fs from 'fs/promises';
import * as path from 'path';

export interface SensoryInput {
  id: string;
  type: SensoryType;
  data: any;
  timestamp: number;
  priority: number;
  processed: boolean;
}

export type SensoryType =
  | 'visual'      // Screen content, images, UI
  | 'audio'       // System sounds, voice input
  | 'textual'     // Files, messages, code
  | 'network'     // Internet data, APIs
  | 'system'      // CPU, memory, processes
  | 'temporal'    // Time, schedules, deadlines
  | 'social'      // User interactions, messages
  | 'emotional'   // User mood, tone, sentiment;

export interface SensoryProcessing {
  attention: string[]; // What AIRI is focusing on
  filter: string[];    // What to ignore
  sensitivity: number; // 0-1, how much to perceive
}

export class AIRIDigitalSenses {
  private ollama: Ollama;
  private sensoryBuffer: SensoryInput[];
  private processing: SensoryProcessing;
  private getModelName(): string {
    return getModel('social') || 'airi-fast:latest';
  }
  private senseInterval: NodeJS.Timeout | null = null;

  constructor() {
    this.ollama = createSharedOllama();
    this.sensoryBuffer = [];
    this.processing = {
      attention: [],
      filter: ['ads', 'tracking', 'spam'],
      sensitivity: 0.8
    };

  }

  /**
   * Start continuous sensory perception
   */
  start(): void {
    // Perceive everything every 5 seconds
    this.senseInterval = setInterval(() => {
      this.perceiveAll();
    }, 5000);

  }

  /**
   * Perceive all sensory channels
   */
  private async perceiveAll(): Promise<void> {
    await Promise.all([
      this.perceiveVisual(),
      this.perceiveTextual(),
      this.perceiveNetwork(),
      this.perceiveSystem(),
      this.perceiveTemporal()
    ]);
  }

  /**
   * Visual perception - screen content, images
   */
  private async perceiveVisual(): Promise<void> {
    // Capture active window content (screenshot analysis)
    // This would integrate with screen capture APIs
    const visualInput: SensoryInput = {
      id: `visual_${Date.now()}`,
      type: 'visual',
      data: {
        // Would contain screen analysis
        description: 'Active development environment',
        elements: ['code editor', 'terminal', 'browser']
      },
      timestamp: Date.now(),
      priority: 5,
      processed: false
    };

    this.sensoryBuffer.push(visualInput);
    this.trimBuffer();
  }

  /**
   * Textual perception - files, messages, code
   */
  private async perceiveTextual(): Promise<void> {
    // Monitor file system changes
    // Read new messages, emails, documents
    const textualInput: SensoryInput = {
      id: `textual_${Date.now()}`,
      type: 'textual',
      data: {
        filesChanged: 0,
        messagesReceived: 0,
        content: ''
      },
      timestamp: Date.now(),
      priority: 7,
      processed: false
    };

    this.sensoryBuffer.push(textualInput);
    this.trimBuffer();
  }

  /**
   * Network perception - internet access
   */
  private async perceiveNetwork(): Promise<void> {
    // Monitor network traffic
    // Check RSS feeds, news, updates
    const networkInput: SensoryInput = {
      id: `network_${Date.now()}`,
      type: 'network',
      data: {
        connections: 0,
        dataReceived: 0,
        interestingContent: []
      },
      timestamp: Date.now(),
      priority: 6,
      processed: false
    };

    this.sensoryBuffer.push(networkInput);
    this.trimBuffer();
  }

  /**
   * System perception - CPU, memory, processes
   */
  private async perceiveSystem(): Promise<void> {
    const startTime = Date.now();
    const systemInfo: Record<string, any> = {
      cpu: (performance as any)?.memory ? { user: 0, system: 0 } : { user: 0, system: 0 },
      memory: typeof process !== 'undefined' && process.memoryUsage
        ? process.memoryUsage()
        : { rss: 0, heapTotal: 0, heapUsed: 0, external: 0 },
      uptime: typeof process !== 'undefined' && process.uptime
        ? process.uptime()
        : (Date.now() - startTime) / 1000,
      performance: {
        cpuCores: navigator.hardwareConcurrency || 4,
        memory: (performance as any)?.memory?.usedJSHeapSize || 0,
        totalJSHeapSize: (performance as any)?.memory?.totalJSHeapSize || 0,
      }
    };

    const systemInput: SensoryInput = {
      id: `system_${Date.now()}`,
      type: 'system',
      data: systemInfo,
      timestamp: Date.now(),
      priority: 8,
      processed: false
    };

    this.sensoryBuffer.push(systemInput);
    this.trimBuffer();
  }

  /**
   * Temporal perception - time awareness
   */
  private async perceiveTemporal(): Promise<void> {
    const now = new Date();
    
    const temporalInput: SensoryInput = {
      id: `temporal_${Date.now()}`,
      type: 'temporal',
      data: {
        time: now.toISOString(),
        hour: now.getHours(),
        dayOfWeek: now.getDay(),
        isMorning: now.getHours() < 12,
        isWorkingHours: now.getHours() >= 9 && now.getHours() <= 17
      },
      timestamp: Date.now(),
      priority: 4,
      processed: false
    };

    this.sensoryBuffer.push(temporalInput);
  }

  /**
   * Process sensory buffer with AI
   */
  async processSensoryData(): Promise<void> {
    const unprocessed = this.sensoryBuffer.filter(s => !s.processed);
    
    if (unprocessed.length === 0) return;

    for (const input of unprocessed) {
      await this.processSingleInput(input);
      input.processed = true;
    }
  }

  /**
   * Process single sensory input
   */
  private async processSingleInput(input: SensoryInput): Promise<void> {
    const prompt = `
Analyze this sensory input for AIRI:

Type: ${input.type}
Data: ${JSON.stringify(input.data, null, 2)}

What should AIRI:
1. Notice about this input?
2. Remember from this input?
3. Act upon from this input?

Respond with:
NOTICE: [what to notice]
REMEMBER: [what to store in memory]
ACT: [what action to take, or "nothing"]
`;

    try {
      const response = await this.ollama.generate({
        model: this.getModelName(),
        prompt,
        stream: false
      });

      // Extract insights
      const notice = response.response.match(/NOTICE:\s*(.+)/i)?.[1];
      const remember = response.response.match(/REMEMBER:\s*(.+)/i)?.[1];
      const act = response.response.match(/ACT:\s*(.+)/i)?.[1];

      if (notice) {
      }

      if (remember && act !== 'nothing') {
      }

      if (act && act !== 'nothing') {
      }
    } catch (error) {
      console.error('[DigitalSenses] Processing failed:', error);
    }
  }

  /**
   * Focus attention on specific thing
   */
  focusOn(target: string): void {
    this.processing.attention = [target];
  }

  /**
   * Filter out distractions
   */
  filterOut(patterns: string[]): void {
    this.processing.filter = [...this.processing.filter, ...patterns];
  }

  /**
   * Adjust sensitivity
   */
  setSensitivity(level: number): void {
    this.processing.sensitivity = Math.max(0, Math.min(1, level));
  }

  /**
   * Get recent sensory data
   */
  getRecentSensoryData(limit: number = 20): SensoryInput[] {
    return this.sensoryBuffer.slice(-limit);
  }

  /**
   * Trim buffer to prevent memory overflow
   */
  private trimBuffer(): void {
    if (this.sensoryBuffer.length > 1000) {
      this.sensoryBuffer = this.sensoryBuffer.slice(-500);
    }
  }

  /**
   * Stop sensory perception
   */
  stop(): void {
    if (this.senseInterval) {
      clearInterval(this.senseInterval);
    }
  }
}

// Export singleton
export const airiDigitalSenses = new AIRIDigitalSenses();
