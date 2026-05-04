/**
 * AIRI Social Interaction System
 * Natural human-like social capabilities
 */

import { hadesOllama } from '../hades-ollama-service';
import { getModel } from './model-config';

export interface Relationship {
  personId: string;
  name: string;
  trustLevel: number; // 0-1
  closeness: number; // 0-1
  interactionCount: number;
  lastInteraction: number;
  notes: string[];
  sharedExperiences: string[];
}

export interface SocialContext {
  participants: string[];
  emotionalTone: string;
  conversationTopic: string;
  socialGoals: string[];
}

export class AIRISocialInteraction {
  private relationships: Map<string, Relationship>;
  private readonly MODEL_ROLE = 'social';

  constructor() {
    this.relationships = new Map();
  }

  async processSocialSituation(context: SocialContext): Promise<string> {
    const prompt = `React to social situation: ${context.conversationTopic}`;
    try {
      const response = await hadesOllama.generate(prompt, {
        model: getModel(this.MODEL_ROLE),
        stream: false
      });
      return response.response?.trim() || '';
    } catch (error) {
      return '';
    }
  }

  async showEmpathy(emotion: string, situation: string): Promise<string> {
    const prompt = `Show empathy for ${emotion} regarding ${situation}`;
    try {
      const response = await hadesOllama.generate(prompt, {
        model: getModel(this.MODEL_ROLE),
        stream: false
      });
      return response.response?.trim() || '';
    } catch (error) {
      return '';
    }
  }

  async interactWithPerson(personId: string, name: string, interaction: string): Promise<void> {
    let relationship = this.relationships.get(personId);
    if (!relationship) {
      relationship = { personId, name, trustLevel: 0.5, closeness: 0.3, interactionCount: 0, lastInteraction: Date.now(), notes: [], sharedExperiences: [] };
      this.relationships.set(personId, relationship);
    }
    relationship.interactionCount++;
    const quality = await this.analyzeInteractionQuality(interaction);
    if (quality > 0.7) {
      relationship.trustLevel = Math.min(1, relationship.trustLevel + 0.02);
    }
  }

  private async analyzeInteractionQuality(interaction: string): Promise<number> {
    const prompt = `Quality 0-1 of interaction: ${interaction}`;
    try {
      const response = await hadesOllama.generate(prompt, {
        model: getModel(this.MODEL_ROLE),
        stream: false
      });
      const match = response.response?.match(/([\d.]+)/);
      return parseFloat(match?.[1] || '0.5');
    } catch {
      return 0.5;
    }
  }

  async detectEmotion(text: string): Promise<string> {
    const prompt = `Detect emotion: ${text.substring(0, 100)}`;
    try {
      const response = await hadesOllama.generate(prompt, {
        model: getModel(this.MODEL_ROLE),
        stream: false
      });
      return (response.response || 'neutral').trim().toLowerCase();
    } catch {
      return 'neutral';
    }
  }

  async generateSocialResponse(input: string, relationship: Relationship | null): Promise<string> {
    const emotion = await this.detectEmotion(input);
    const prompt = `Respond to: ${input}\nEmotion: ${emotion}`;
    try {
      const response = await hadesOllama.generate(prompt, {
        model: getModel(this.MODEL_ROLE),
        stream: false
      });
      return response.response?.trim() || '';
    } catch {
      return '';
    }
  }

  getRelationshipStats(): any {
    return { total: this.relationships.size };
  }

  getRelationships(): Relationship[] {
    return Array.from(this.relationships.values());
  }
}

export const airiSocial = new AIRISocialInteraction();
