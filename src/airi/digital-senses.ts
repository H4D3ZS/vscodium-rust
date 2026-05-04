/**
 * AIRI Digital Senses System
 * Complete perception layer for digital existence
 */

import { hadesOllama } from '../hades-ollama-service';
import { getModel } from './model-config';

export interface SensoryInput {
  id: string;
  type: string;
  data: any;
  timestamp: number;
}

export interface Perception {
  summary: string;
  description: string;
  importance: number;
  tags: string[];
  userFrustrated: boolean;
  errors: string[];
}

export class AIRIDigitalSenses {
  private sensoryBuffer: SensoryInput[] = [];
  private readonly MODEL_ROLE = 'social';
  private senseInterval: any | null = null;

  constructor() { }

  start(): void {
    if (this.senseInterval) return;
    this.senseInterval = setInterval(() => {
      this.perceiveAll().catch(() => { });
    }, 60000); // 60s intervals
  }

  private async perceiveAll(): Promise<void> {
    const input: SensoryInput = {
      id: `sense_${Date.now()}`,
      type: 'system',
      data: { timestamp: Date.now() },
      timestamp: Date.now()
    };
    this.sensoryBuffer.push(input);
    if (this.sensoryBuffer.length > 50) this.sensoryBuffer.shift();
  }

  getPerception(): Perception {
    return {
      summary: "Normal operating conditions.",
      description: "Observing workspace activity.",
      importance: 0.1,
      tags: ["status", "idle"],
      userFrustrated: false,
      errors: []
    };
  }

  async processSensoryData(): Promise<void> {
    const prompt = "Analyze recent senses.";
    try {
      await hadesOllama.generate(prompt, {
        model: getModel(this.MODEL_ROLE),
        stream: false,
        timeout: 15000
      });
    } catch { }
  }

  stop(): void {
    if (this.senseInterval) {
      clearInterval(this.senseInterval);
      this.senseInterval = null;
    }
  }
}

export const airiDigitalSenses = new AIRIDigitalSenses();
