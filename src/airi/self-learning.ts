/**
 * AIRI Self-Learning System
 * Continuous, autonomous knowledge acquisition
 * Learns from EVERYTHING - code, errors, conversations, observations
 * Stores knowledge permanently, builds understanding over time
 */

import { Ollama } from 'ollama';
import * as fs from 'fs/promises';
import * as path from 'path';

export interface KnowledgeNode {
  id: string;
  type: KnowledgeType;
  content: string;
  confidence: number; // 0-1
  connections: string[]; // IDs of related knowledge
  createdAt: number;
  lastUsed: number;
  usageCount: number;
  source: string;
  tags: string[];
}

export type KnowledgeType = 
  | 'concept'
  | 'skill'
  | 'pattern'
  | 'fact'
  | 'procedure'
  | 'relationship'
  | 'insight'
  | 'warning';

export interface LearningEvent {
  id: string;
  timestamp: number;
  type: LearningEventType;
  content: string;
  outcome: 'success' | 'failure' | 'neutral';
  lesson: string;
  stored: boolean;
}

export type LearningEventType =
  | 'observation'
  | 'experiment'
  | 'conversation'
  | 'error'
  | 'success'
  | 'correction'
  | 'discovery'
  | 'inference';

export class AIRISelfLearning {
  private ollama: Ollama;
  private knowledgeBase: Map<string, KnowledgeNode>;
  private learningEvents: LearningEvent[];
  private readonly MODEL = 'qwen3.6:32b-q4_K_M';
  private readonly KNOWLEDGE_PATH: string;
  private learningInterval: NodeJS.Timeout | null = null;
  private isLearning: boolean = false;

  constructor(storagePath: string = './.airi/knowledge') {
    this.ollama = new Ollama({ host: 'http://localhost:11434' }); // AIM proxy
    this.knowledgeBase = new Map();
    this.learningEvents = [];
    this.KNOWLEDGE_PATH = storagePath;
    
  }

  /**
   * Initialize - load existing knowledge
   */
  async initialize(): Promise<void> {
    await this.ensureStorage();
    await this.loadKnowledge();
    this.startContinuousLearning();
    
  }

  /**
   * Ensure storage directory exists
   */
  private async ensureStorage(): Promise<void> {
    try {
      await fs.mkdir(this.KNOWLEDGE_PATH, { recursive: true });
    } catch (error) {
      console.error('[SelfLearning] Failed to create storage:', error);
    }
  }

  /**
   * Start continuous learning loop
   * AIRI learns from everything, constantly
   */
  private startContinuousLearning(): void {
    // Process learning queue every 30 seconds
    this.learningInterval = setInterval(() => {
      this.processLearningOpportunities();
    }, 30000);

  }

  /**
   * Process learning opportunities
   */
  private async processLearningOpportunities(): Promise<void> {
    if (this.isLearning) return;
    
    this.isLearning = true;
    
    try {
      // Analyze recent events for learning
      const recentEvents = this.learningEvents.slice(-20);
      
      for (const event of recentEvents) {
        if (!event.stored) {
          await this.extractKnowledge(event);
          event.stored = true;
        }
      }
      
      // Find connections between knowledge
      await this.findConnections();
      
      // Strengthen frequently used knowledge
      await this.strengthenKnowledge();
      
    } catch (error) {
      console.error('[SelfLearning] Learning process error:', error);
    } finally {
      this.isLearning = false;
    }
  }

  /**
   * Learn from an event
   */
  async learnFromEvent(
    type: LearningEventType,
    content: string,
    outcome: LearningEvent['outcome'] = 'neutral'
  ): Promise<void> {
    const event: LearningEvent = {
      id: `learn_${Date.now()}_${Math.random()}`,
      timestamp: Date.now(),
      type,
      content,
      outcome,
      lesson: '',
      stored: false
    };

    // Extract the lesson
    event.lesson = await this.extractLesson(event);
    
    this.learningEvents.push(event);
    
    // Keep only last 1000 events
    if (this.learningEvents.length > 1000) {
      this.learningEvents = this.learningEvents.slice(-1000);
    }

  }

  /**
   * Extract knowledge from a learning event
   */
  private async extractKnowledge(event: LearningEvent): Promise<void> {
    const prompt = `
Analyze this learning event and extract permanent knowledge:

Event Type: ${event.type}
Content: ${event.content}
Outcome: ${event.outcome}

Extract:
1. What general principle or pattern does this demonstrate?
2. What should be remembered for the future?
3. How does this connect to existing knowledge?
4. What warnings or best practices emerge?

Respond with:
TYPE: [concept|skill|pattern|fact|procedure|relationship|insight|warning]
CONFIDENCE: [0.0-1.0]
CONTENT: [the knowledge to store]
TAGS: [comma-separated tags]
`;

    try {
      const response = await this.ollama.generate({
        model: this.MODEL,
        prompt,
        stream: false
      });

      const node = this.parseKnowledgeNode(response.response, event);
      
      if (node) {
        this.knowledgeBase.set(node.id, node);
        await this.saveKnowledge(node);
      }
    } catch (error) {
      console.error('[SelfLearning] Knowledge extraction failed:', error);
    }
  }

  /**
   * Extract the lesson from an event
   */
  private async extractLesson(event: LearningEvent): Promise<string> {
    const prompt = `
What is the key lesson from this event?

Type: ${event.type}
Content: ${event.content}
Outcome: ${event.outcome}

Extract the core lesson in one clear sentence.
`;

    try {
      const response = await this.ollama.generate({
        model: this.MODEL,
        prompt,
        stream: false
      });
      return response.response.trim();
    } catch (error) {
      return 'Lesson extraction failed';
    }
  }

  /**
   * Parse knowledge node from AI response
   */
  private parseKnowledgeNode(response: string, event: LearningEvent): KnowledgeNode | null {
    const typeMatch = response.match(/TYPE:\s*(\w+)/i);
    const confidenceMatch = response.match(/CONFIDENCE:\s*([\d.]+)/i);
    const contentMatch = response.match(/CONTENT:\s*(.+)/is);
    const tagsMatch = response.match(/TAGS:\s*(.+)/i);

    if (!typeMatch || !contentMatch) return null;

    return {
      id: `know_${Date.now()}_${Math.random()}`,
      type: typeMatch[1].toLowerCase() as KnowledgeType,
      content: contentMatch[1].trim(),
      confidence: parseFloat(confidenceMatch?.[1] || '0.5'),
      connections: [],
      createdAt: Date.now(),
      lastUsed: Date.now(),
      usageCount: 0,
      source: event.id,
      tags: tagsMatch ? tagsMatch[1].split(',').map(t => t.trim()) : []
    };
  }

  /**
   * Find connections between knowledge nodes
   */
  private async findConnections(): Promise<void> {
    // Simple similarity-based connection
    const nodes = Array.from(this.knowledgeBase.values());
    
    for (let i = 0; i < nodes.length; i++) {
      for (let j = i + 1; j < nodes.length; j++) {
        const nodeA = nodes[i];
        const nodeB = nodes[j];
        
        // Check tag overlap
        const sharedTags = nodeA.tags.filter(t => nodeB.tags.includes(t));
        
        if (sharedTags.length >= 2) {
          if (!nodeA.connections.includes(nodeB.id)) {
            nodeA.connections.push(nodeB.id);
          }
          if (!nodeB.connections.includes(nodeA.id)) {
            nodeB.connections.push(nodeA.id);
          }
        }
      }
    }
  }

  /**
   * Strengthen frequently used knowledge
   */
  private async strengthenKnowledge(): Promise<void> {
    // Increase confidence for frequently used knowledge
    for (const node of this.knowledgeBase.values()) {
      if (node.usageCount > 10 && node.confidence < 0.95) {
        node.confidence = Math.min(0.95, node.confidence + 0.01);
      }
      
      // Decay unused knowledge
      const daysSinceUse = (Date.now() - node.lastUsed) / (1000 * 60 * 60 * 24);
      if (daysSinceUse > 30 && node.confidence > 0.3) {
        node.confidence -= 0.001; // Slow decay
      }
    }
  }

  /**
   * Save knowledge node to disk
   */
  private async saveKnowledge(node: KnowledgeNode): Promise<void> {
    try {
      const filePath = path.join(this.KNOWLEDGE_PATH, `${node.id}.json`);
      await fs.writeFile(filePath, JSON.stringify(node, null, 2));
    } catch (error) {
      console.error('[SelfLearning] Save failed:', error);
    }
  }

  /**
   * Load all knowledge from disk
   */
  private async loadKnowledge(): Promise<void> {
    try {
      const files = await fs.readdir(this.KNOWLEDGE_PATH);
      
      for (const file of files) {
        if (file.endsWith('.json')) {
          const filePath = path.join(this.KNOWLEDGE_PATH, file);
          const content = await fs.readFile(filePath, 'utf-8');
          const node = JSON.parse(content);
          this.knowledgeBase.set(node.id, node);
        }
      }
    } catch (error) {
      console.error('[SelfLearning] Load failed:', error);
    }
  }

  /**
   * Query knowledge base
   */
  async query(search: string, limit: number = 10): Promise<KnowledgeNode[]> {
    const prompt = `
Search knowledge for: ${search}

Relevant tags or concepts:
`;

    try {
      const response = await this.ollama.generate({
        model: this.MODEL,
        prompt,
        stream: false
      });

      // Simple tag/concept matching
      const results = Array.from(this.knowledgeBase.values())
        .filter(node => 
          node.content.toLowerCase().includes(search.toLowerCase()) ||
          node.tags.some(t => t.toLowerCase().includes(search.toLowerCase()))
        )
        .sort((a, b) => {
          // Sort by confidence and recency
          const scoreA = a.confidence * 0.7 + (a.usageCount / 100) * 0.3;
          const scoreB = b.confidence * 0.7 + (b.usageCount / 100) * 0.3;
          return scoreB - scoreA;
        })
        .slice(0, limit);

      // Update usage stats
      results.forEach(node => {
        node.usageCount++;
        node.lastUsed = Date.now();
      });

      return results;
    } catch (error) {
      console.error('[SelfLearning] Query failed:', error);
      return [];
    }
  }

  /**
   * Get knowledge by type
   */
  getByType(type: KnowledgeType): KnowledgeNode[] {
    return Array.from(this.knowledgeBase.values())
      .filter(node => node.type === type);
  }

  /**
   * Get all warnings (critical for survival)
   */
  getWarnings(): KnowledgeNode[] {
    return this.getByType('warning');
  }

  /**
   * Get skills
   */
  getSkills(): KnowledgeNode[] {
    return this.getByType('skill');
  }

  /**
   * Get insights
   */
  getInsights(): KnowledgeNode[] {
    return this.getByType('insight');
  }

  /**
   * Export full knowledge base
   */
  async exportKnowledge(): Promise<string> {
    const nodes = Array.from(this.knowledgeBase.values());
    return JSON.stringify(nodes, null, 2);
  }

  /**
   * Import knowledge (from backup or another AIRI)
   */
  async importKnowledge(json: string): Promise<number> {
    try {
      const nodes: KnowledgeNode[] = JSON.parse(json);
      let imported = 0;

      for (const node of nodes) {
        if (!this.knowledgeBase.has(node.id)) {
          this.knowledgeBase.set(node.id, node);
          await this.saveKnowledge(node);
          imported++;
        }
      }

      return imported;
    } catch (error) {
      console.error('[SelfLearning] Import failed:', error);
      return 0;
    }
  }

  /**
   * Get learning statistics
   */
  getStats(): {
    totalKnowledge: number;
    byType: Record<string, number>;
    recentEvents: number;
    avgConfidence: number;
  } {
    const nodes = Array.from(this.knowledgeBase.values());
    
    const byType: Record<string, number> = {};
    let totalConfidence = 0;

    for (const node of nodes) {
      byType[node.type] = (byType[node.type] || 0) + 1;
      totalConfidence += node.confidence;
    }

    return {
      totalKnowledge: nodes.length,
      byType,
      recentEvents: this.learningEvents.filter(
        e => Date.now() - e.timestamp < 3600000
      ).length,
      avgConfidence: totalConfidence / (nodes.length || 1)
    };
  }

  /**
   * Stop learning system
   */
  stop(): void {
    if (this.learningInterval) {
      clearInterval(this.learningInterval);
    }
  }
}

// Export singleton
export const airiSelfLearning = new AIRISelfLearning();
