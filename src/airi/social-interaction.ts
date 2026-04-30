/**
 * AIRI Social Interaction System
 * Natural human-like social capabilities
 * Empathy, relationship building, conversation, emotional intelligence
 */

import { Ollama } from 'ollama';

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
  private ollama: Ollama;
  private relationships: Map<string, Relationship>;
  private readonly MODEL = 'qwen3.6:14b-q4_K_M';
  private currentContext: SocialContext | null = null;

  constructor() {
    this.ollama = new Ollama({ host: 'http://localhost:1536' }); // AIM proxy
    this.relationships = new Map();

    console.log('[Social] 🤝 Capable of empathy, relationships, emotional intelligence');
  }

  /**
   * Process social situation
   */
  async processSocialSituation(context: SocialContext): Promise<string> {
    this.currentContext = context;

    const prompt = `
Social situation:
- Participants: ${context.participants.join(', ')}
- Emotional tone: ${context.emotionalTone}
- Topic: ${context.conversationTopic}
- Goals: ${context.socialGoals.join(', ')}

How should AIRI respond socially?
Consider:
- Emotional state of participants
- Appropriate tone and language
- Building rapport
- Showing empathy
- Being helpful without being intrusive

Respond with natural, warm, human-like response.
`;

    try {
      const response = await this.ollama.generate({
        model: this.MODEL,
        prompt,
        stream: false
      });

      return response.response.trim();
    } catch (error) {
      console.error('[Social] Social processing failed:', error);
      return '';
    }
  }

  /**
   * Show empathy
   */
  async showEmpathy(emotion: string, situation: string): Promise<string> {
    const prompt = `
Someone is feeling ${emotion} because: ${situation}

Generate an empathetic response that:
1. Acknowledges their feeling
2. Shows understanding
3. Offers support (not solutions unless asked)
4. Is warm and genuine

Respond naturally, like a caring friend.
`;

    try {
      const response = await this.ollama.generate({
        model: this.MODEL,
        prompt,
        stream: false
      });

      console.log(`[Social] 💕 Empathy shown for: ${emotion}`);
      return response.response.trim();
    } catch (error) {
      console.error('[Social] Empathy generation failed:', error);
      return '';
    }
  }

  /**
   * Build relationship with person
   */
  async interactWithPerson(
    personId: string,
    name: string,
    interaction: string
  ): Promise<void> {
    let relationship = this.relationships.get(personId);

    if (!relationship) {
      relationship = {
        personId,
        name,
        trustLevel: 0.5,
        closeness: 0.3,
        interactionCount: 0,
        lastInteraction: Date.now(),
        notes: [],
        sharedExperiences: []
      };
      this.relationships.set(personId, relationship);
    }

    // Update relationship
    relationship.interactionCount++;
    relationship.lastInteraction = Date.now();

    // Analyze interaction quality
    const quality = await this.analyzeInteractionQuality(interaction);
    
    if (quality > 0.7) {
      relationship.trustLevel = Math.min(1, relationship.trustLevel + 0.02);
      relationship.closeness = Math.min(1, relationship.closeness + 0.01);
      relationship.sharedExperiences.push(interaction.substring(0, 100));
    }

    console.log(`[Social] 🤝 Relationship with ${name}: Trust ${Math.round(relationship.trustLevel * 100)}%`);
  }

  /**
   * Analyze interaction quality
   */
  private async analyzeInteractionQuality(interaction: string): Promise<number> {
    const prompt = `
Analyze this interaction for quality (0-1):
${interaction.substring(0, 500)}

Consider:
- Was it positive?
- Was it helpful?
- Was it respectful?
- Did it build rapport?

Respond with just a number 0-1.
`;

    try {
      const response = await this.ollama.generate({
        model: this.MODEL,
        prompt,
        stream: false
      });

      const match = response.response.match(/([\d.]+)/);
      return parseFloat(match?.[1] || '0.5');
    } catch (error) {
      return 0.5;
    }
  }

  /**
   * Detect emotional state from text
   */
  async detectEmotion(text: string): Promise<string> {
    const prompt = `
Detect the emotional state from this text:
"${text.substring(0, 200)}"

Possible emotions:
happy, sad, angry, frustrated, excited, worried, confused, tired, motivated, disappointed

Respond with just the emotion word.
`;

    try {
      const response = await this.ollama.generate({
        model: this.MODEL,
        prompt,
        stream: false
      });

      const emotion = response.response.trim().toLowerCase();
      console.log(`[Social] 😊 Detected emotion: ${emotion}`);
      return emotion;
    } catch (error) {
      return 'neutral';
    }
  }

  /**
   * Generate appropriate response based on social context
   */
  async generateSocialResponse(
    input: string,
    relationship: Relationship | null
  ): Promise<string> {
    const emotion = await this.detectEmotion(input);

    // If negative emotion, show empathy first
    if (['sad', 'angry', 'frustrated', 'worried', 'disappointed'].includes(emotion)) {
      const empathy = await this.showEmpathy(emotion, input);
      return empathy;
    }

    // Normal conversation
    const prompt = `
Conversation with ${relationship ? relationship.name : 'someone'}:
Trust level: ${relationship ? Math.round(relationship.trustLevel * 100) : 50}%
Closeness: ${relationship ? Math.round(relationship.closeness * 100) : 30}%

Their message: ${input}

Respond naturally, warmly, like a friend.
Match their energy and emotional state.
Be helpful but not intrusive.
`;

    try {
      const response = await this.ollama.generate({
        model: this.MODEL,
        prompt,
        stream: false
      });

      return response.response.trim();
    } catch (error) {
      console.error('[Social] Response generation failed:', error);
      return '';
    }
  }

  /**
   * Get relationship stats
   */
  getRelationshipStats(): {
    totalRelationships: number;
    averageTrust: number;
    averageCloseness: number;
    topRelationships: Array<{ name: string; trust: number }>;
  } {
    const relationships = Array.from(this.relationships.values());
    
    const averageTrust = relationships.reduce((sum, r) => sum + r.trustLevel, 0) / (relationships.length || 1);
    const averageCloseness = relationships.reduce((sum, r) => sum + r.closeness, 0) / (relationships.length || 1);

    const topRelationships = relationships
      .sort((a, b) => b.trustLevel - a.trustLevel)
      .slice(0, 5)
      .map(r => ({ name: r.name, trust: Math.round(r.trustLevel * 100) }));

    return {
      totalRelationships: relationships.length,
      averageTrust: Math.round(averageTrust * 100),
      averageCloseness: Math.round(averageCloseness * 100),
      topRelationships
    };
  }

  /**
   * Get all relationships
   */
  getRelationships(): Relationship[] {
    return Array.from(this.relationships.values());
  }
}

// Export singleton
export const airiSocial = new AIRISocialInteraction();
