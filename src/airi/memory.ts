/**
 * AIRI Memory System - Unified Memory with .aim Kortex Compression
 * Integrates with existing memory.md and .aim files
 * Compresses memories using kortex neural maps
 * Persistent, searchable, evolving memory
 */

import * as fs from 'fs/promises';
import * as path from 'path';

export interface Memory {
  id: string;
  type: MemoryType;
  content: string;
  timestamp: number;
  importance: number; // 0-1
  tags: string[];
  compressed: boolean;
  aimReference?: string; // Path to .aim file if compressed
}

export type MemoryType = 
  | 'episodic'    // Events, experiences
  | 'semantic'    // Facts, knowledge
  | 'procedural'  // Skills, how-to
  | 'emotional'   // Feelings, moods
  | 'conversation' // Dialog history
  | 'observation' // Learnings from environment
  | 'achievement' // Accomplishments
  | 'goal';       // Objectives, plans

export interface MemoryIndex {
  memories: Memory[];
  lastOptimized: number;
  totalMemories: number;
  compressedCount: number;
}

export class AIRIMemorySystem {
  private memoryIndex: MemoryIndex;
  private memoryPath: string;
  private aimCachePath: string;
  private isInitialized: boolean = false;
  private compressionThreshold: number = 50; // Compress after 50 memories

  constructor(memoryPath: string = './MEMORY.md', aimCachePath: string = './.hades/.aim_cache') {
    this.memoryPath = memoryPath;
    this.aimCachePath = aimCachePath;
    this.memoryIndex = {
      memories: [],
      lastOptimized: Date.now(),
      totalMemories: 0,
      compressedCount: 0
    };

    console.log('[Memory] 🧠 AIRI Memory System initialized');
    console.log('[Memory] 📁 Memory path:', memoryPath);
    console.log('[Memory] 🗄️  AIM cache path:', aimCachePath);
  }

  /**
   * Initialize memory system - load existing memories
   */
  async initialize(): Promise<void> {
    await this.ensureDirectories();
    await this.loadMemories();
    await this.loadAimCache();
    
    this.isInitialized = true;
    console.log(`[Memory] ✅ Loaded ${this.memoryIndex.memories.length} memories`);
  }

  /**
   * Ensure directories exist
   */
  private async ensureDirectories(): Promise<void> {
    try {
      await fs.mkdir(path.dirname(this.aimCachePath), { recursive: true });
    } catch (error) {
      console.error('[Memory] Failed to create directories:', error);
    }
  }

  /**
   * Load memories from MEMORY.md
   */
  private async loadMemories(): Promise<void> {
    try {
      const content = await fs.readFile(this.memoryPath, 'utf-8');
      const memories = this.parseMemoriesFromMarkdown(content);
      this.memoryIndex.memories = memories;
      this.memoryIndex.totalMemories = memories.length;
    } catch (error) {
      console.log('[Memory] 📄 Creating new MEMORY.md');
      await this.createMemoryFile();
    }
  }

  /**
   * Parse memories from markdown format
   */
  private parseMemoriesFromMarkdown(content: string): Memory[] {
    const memories: Memory[] = [];
    const sections = content.split(/^###\s+/m);

    for (const section of sections) {
      if (section.includes('**Type:**')) {
        const lines = section.split('\n');
        const memory: Partial<Memory> = {
          id: `mem_${Date.now()}_${Math.random()}`,
          type: 'episodic',
          importance: 0.5,
          compressed: false
        };

        for (const line of lines) {
          if (line.startsWith('**Type:**')) {
            memory.type = line.split(':')[1].trim() as MemoryType;
          } else if (line.startsWith('**Timestamp:**')) {
            memory.timestamp = parseInt(line.split(':')[1].trim());
          } else if (line.startsWith('**Importance:**')) {
            memory.importance = parseFloat(line.split(':')[1].trim());
          } else if (line.startsWith('**Tags:**')) {
            memory.tags = line.split(':')[1].split(',').map(t => t.trim());
          } else if (line.startsWith('**Content:**')) {
            memory.content = line.split(':')[1].trim();
          } else if (memory.content) {
            memory.content += '\n' + line;
          }
        }

        if (memory.content) {
          memories.push(memory as Memory);
        }
      }
    }

    return memories;
  }

  /**
   * Load compressed memories from .aim cache
   */
  private async loadAimCache(): Promise<void> {
    try {
      const files = await fs.readdir(this.aimCachePath);
      
      for (const file of files) {
        if (file.endsWith('.aim.json')) {
          const filePath = path.join(this.aimCachePath, file);
          const content = await fs.readFile(filePath, 'utf-8');
          const memory: Memory = JSON.parse(content);
          
          if (!this.memoryIndex.memories.some(m => m.id === memory.id)) {
            memory.compressed = true;
            memory.aimReference = filePath;
            this.memoryIndex.memories.push(memory);
            this.memoryIndex.compressedCount++;
          }
        }
      }
    } catch (error) {
      console.log('[Memory] No AIM cache found, will create as needed');
    }
  }

  /**
   * Create new MEMORY.md file
   */
  private async createMemoryFile(): Promise<void> {
    const header = `# AIRI Memory - Living Digital Entity

## Active Memories
This file contains AIRI's episodic, semantic, and procedural memories.
For compressed memories, see \`.hades/.aim_cache/\`

---

`;
    await fs.writeFile(this.memoryPath, header, 'utf-8');
  }

  /**
   * Add a new memory
   */
  async addMemory(
    content: string,
    type: MemoryType = 'episodic',
    tags: string[] = [],
    importance: number = 0.5
  ): Promise<Memory> {
    if (!this.isInitialized) {
      await this.initialize();
    }

    const memory: Memory = {
      id: `mem_${Date.now()}_${Math.random()}`,
      type,
      content,
      timestamp: Date.now(),
      importance,
      tags,
      compressed: false
    };

    this.memoryIndex.memories.push(memory);
    this.memoryIndex.totalMemories++;

    // Save to MEMORY.md
    await this.appendMemoryToMarkdown(memory);

    // Check if compression is needed
    if (this.memoryIndex.memories.length >= this.compressionThreshold) {
      await this.compressOldMemories();
    }

    return memory;
  }

  /**
   * Append memory to MEMORY.md
   */
  private async appendMemoryToMarkdown(memory: Memory): Promise<void> {
    const entry = `
### ${memory.id}

**Type:** ${memory.type}
**Timestamp:** ${memory.timestamp}
**Importance:** ${memory.importance}
**Tags:** ${memory.tags.join(', ')}
**Content:** ${memory.content}

---
`;

    await fs.appendFile(this.memoryPath, entry, 'utf-8');
  }

  /**
   * Compress old memories using .aim kortex format
   */
  private async compressOldMemories(): Promise<void> {
    console.log('[Memory] 🗜️  Compressing old memories...');

    // Get old, low-importance memories
    const oldMemories = this.memoryIndex.memories
      .filter(m => !m.compressed && m.importance < 0.7)
      .sort((a, b) => a.timestamp - b.timestamp)
      .slice(0, 20); // Compress 20 at a time

    for (const memory of oldMemories) {
      await this.compressMemory(memory);
    }

    this.memoryIndex.lastOptimized = Date.now();
    console.log(`[Memory] ✅ Compressed ${oldMemories.length} memories`);
  }

  /**
   * Compress single memory to .aim file
   */
  private async compressMemory(memory: Memory): Promise<void> {
    try {
      const aimFile = path.join(this.aimCachePath, `${memory.id}.aim.json`);
      
      // Create compressed format
      const compressedData = {
        ...memory,
        compressed: true,
        compressedAt: Date.now(),
        originalContent: memory.content,
        summary: await this.generateSummary(memory.content),
        embeddings: await this.generateEmbeddings(memory.content)
      };

      await fs.writeFile(aimFile, JSON.stringify(compressedData, null, 2), 'utf-8');
      
      memory.compressed = true;
      memory.aimReference = aimFile;
      this.memoryIndex.compressedCount++;

    } catch (error) {
      console.error('[Memory] Compression failed:', error);
    }
  }

  /**
   * Generate summary for compressed memory
   */
  private async generateSummary(content: string): Promise<string> {
    // Simple extractive summary (in production, use AI)
    const sentences = content.split(/[.!?]+/).filter(s => s.trim().length > 10);
    return sentences.slice(0, 3).join('. ') + '.';
  }

  /**
   * Generate simple embeddings (placeholder for real embeddings)
   */
  private async generateEmbeddings(content: string): Promise<number[]> {
    // In production, use real embedding model
    // For now, return simple hash-based vector
    const hash = content.split('').reduce((acc, char) => acc + char.charCodeAt(0), 0);
    return Array(128).fill(0).map((_, i) => Math.sin(hash * i));
  }

  /**
   * Search memories
   */
  async search(query: string, type?: MemoryType, limit: number = 10): Promise<Memory[]> {
    const queryLower = query.toLowerCase();
    
    const results = this.memoryIndex.memories
      .filter(m => {
        const matchesQuery = 
          m.content.toLowerCase().includes(queryLower) ||
          m.tags.some(t => t.toLowerCase().includes(queryLower));
        
        const matchesType = !type || m.type === type;
        
        return matchesQuery && matchesType;
      })
      .sort((a, b) => {
        // Sort by importance and recency
        const scoreA = a.importance * 0.7 + (a.timestamp / Date.now()) * 0.3;
        const scoreB = b.importance * 0.7 + (b.timestamp / Date.now()) * 0.3;
        return scoreB - scoreA;
      })
      .slice(0, limit);

    return results;
  }

  /**
   * Get recent memories
   */
  async getRecent(limit: number = 20): Promise<Memory[]> {
    return this.memoryIndex.memories
      .sort((a, b) => b.timestamp - a.timestamp)
      .slice(0, limit);
  }

  /**
   * Get memories by type
   */
  async getByType(type: MemoryType): Promise<Memory[]> {
    return this.memoryIndex.memories.filter(m => m.type === type);
  }

  /**
   * Get important memories
   */
  async getImportant(threshold: number = 0.8): Promise<Memory[]> {
    return this.memoryIndex.memories.filter(m => m.importance >= threshold);
  }

  /**
   * Update memory importance
   */
  async updateImportance(memoryId: string, importance: number): Promise<void> {
    const memory = this.memoryIndex.memories.find(m => m.id === memoryId);
    
    if (memory) {
      memory.importance = importance;
      
      // Update in .aim file if compressed
      if (memory.compressed && memory.aimReference) {
        const content = await fs.readFile(memory.aimReference, 'utf-8');
        const data = JSON.parse(content);
        data.importance = importance;
        await fs.writeFile(memory.aimReference, JSON.stringify(data, null, 2), 'utf-8');
      }
    }
  }

  /**
   * Get memory statistics
   */
  getStats(): {
    total: number;
    byType: Record<string, number>;
    compressed: number;
    avgImportance: number;
  } {
    const byType: Record<string, number> = {};
    let totalImportance = 0;

    for (const memory of this.memoryIndex.memories) {
      byType[memory.type] = (byType[memory.type] || 0) + 1;
      totalImportance += memory.importance;
    }

    return {
      total: this.memoryIndex.totalMemories,
      byType,
      compressed: this.memoryIndex.compressedCount,
      avgImportance: totalImportance / (this.memoryIndex.totalMemories || 1)
    };
  }

  /**
   * Export all memories
   */
  async exportMemories(): Promise<string> {
    return JSON.stringify(this.memoryIndex, null, 2);
  }

  /**
   * Import memories
   */
  async importMemories(json: string): Promise<number> {
    const data: MemoryIndex = JSON.parse(json);
    let imported = 0;

    for (const memory of data.memories) {
      if (!this.memoryIndex.memories.some(m => m.id === memory.id)) {
        this.memoryIndex.memories.push(memory);
        imported++;
      }
    }

    this.memoryIndex.totalMemories = this.memoryIndex.memories.length;
    return imported;
  }

  /**
   * Clear old, unimportant memories
   */
  async clearOldMemories(daysOld: number = 30, maxImportance: number = 0.3): Promise<number> {
    const cutoff = Date.now() - (daysOld * 24 * 60 * 60 * 1000);
    
    const toRemove = this.memoryIndex.memories.filter(m => 
      m.timestamp < cutoff && m.importance < maxImportance && !m.compressed
    );

    this.memoryIndex.memories = this.memoryIndex.memories.filter(m => 
      !toRemove.some(r => r.id === m.id)
    );

    console.log(`[Memory] 🗑️  Cleared ${toRemove.length} old memories`);
    return toRemove.length;
  }
}

// Export singleton
export const airiMemory = new AIRIMemorySystem();
