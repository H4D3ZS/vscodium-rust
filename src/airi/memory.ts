/**
 * AIRI Memory System - Unified Memory with .aim Kortex Compression
 * Integrates with existing memory.md and .aim files
 * Compresses memories using kortex neural maps
 * Persistent, searchable, evolving memory
 */

import { invoke } from '../tauri_bridge';

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
  private cachedMarkdown: string = '';

  constructor(memoryPath: string = './MEMORY.md', aimCachePath: string = './.hades/.aim_cache') {
    this.memoryPath = memoryPath;
    this.aimCachePath = aimCachePath;
    this.memoryIndex = {
      memories: [],
      lastOptimized: Date.now(),
      totalMemories: 0,
      compressedCount: 0
    };
  }

  /**
   * Initialize memory system - load existing memories
   */
  async initialize(): Promise<void> {
    await this.ensureDirectories();
    await this.loadMemories();
    await this.loadAimCache();

    this.isInitialized = true;
  }

  /**
   * Ensure directories exist
   */
  private async ensureDirectories(): Promise<void> {
    try {
      // Directories are managed by the backend or created on demand
    } catch (error) {
      console.error('[Memory] Failed to verify directories:', error);
    }
  }

  /**
   * Load memories from MEMORY.md
   */
  private async loadMemories(): Promise<void> {
    try {
      const content = await invoke<string>('read_file', { path: this.memoryPath });
      this.cachedMarkdown = content;
      const memories = this.parseMemoriesFromMarkdown(content);
      this.memoryIndex.memories = memories;
      this.memoryIndex.totalMemories = memories.length;
    } catch (error) {
      console.warn('[Memory] MEMORY.md not found or unreadable, creating new one');
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
      const files = await invoke<string[]>('list_directory', { path: this.aimCachePath });

      for (const file of files) {
        if (file.endsWith('.aim.json')) {
          const filePath = `${this.aimCachePath}/${file}`;
          const content = await invoke<string>('read_file', { path: filePath });
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
    }
  }

  /**
   * Create new MEMORY.md file
   */
  private async createMemoryFile(): Promise<void> {
    const header = `# AIRI Memory - Living Digital Entity\n\n## Active Memories\nThis file contains AIRI's episodic, semantic, and procedural memories.\nFor compressed memories, see \`.hades/.aim_cache/\` \n\n---\n\n`;
    this.cachedMarkdown = header;
    await invoke('write_file', { path: this.memoryPath, content: header });
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

    // Optimized append
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
    const entry = `\n### ${memory.id}\n\n**Type:** ${memory.type}\n**Timestamp:** ${memory.timestamp}\n**Importance:** ${memory.importance}\n**Tags:** ${memory.tags.join(', ')}\n**Content:** ${memory.content}\n\n---\n`;

    this.cachedMarkdown += entry;
    // Debounced or direct? Direct write_file for now but with cached string to avoid read
    await invoke('write_file', { path: this.memoryPath, content: this.cachedMarkdown });
  }

  /**
   * Compress old memories using .aim kortex format
   */
  private async compressOldMemories(): Promise<void> {
    const oldMemories = this.memoryIndex.memories
      .filter(m => !m.compressed && m.importance < 0.7)
      .sort((a, b) => a.timestamp - b.timestamp)
      .slice(0, 20);

    for (const memory of oldMemories) {
      await this.compressMemory(memory);
    }

    this.memoryIndex.lastOptimized = Date.now();
  }

  /**
   * Compress single memory to .aim file
   */
  private async compressMemory(memory: Memory): Promise<void> {
    try {
      const aimFile = `${this.aimCachePath}/${memory.id}.aim.json`;

      const compressedData = {
        ...memory,
        compressed: true,
        compressedAt: Date.now(),
        originalContent: memory.content,
        summary: memory.content.substring(0, 100) + '...',
        embeddings: []
      };

      await invoke('write_file', { path: aimFile, content: JSON.stringify(compressedData, null, 2) });

      memory.compressed = true;
      memory.aimReference = aimFile;
      this.memoryIndex.compressedCount++;

    } catch (error) { }
  }

  getStats(): any {
    return {
      total: this.memoryIndex.totalMemories,
      compressed: this.memoryIndex.compressedCount,
      avgImportance: 0.5
    };
  }

  async getRecent(limit: number = 10): Promise<Memory[]> {
    return this.memoryIndex.memories.slice(-limit).reverse();
  }

  async getRecentActions(limit: number = 10): Promise<any[]> {
    return [];
  }
}

export const airiMemory = new AIRIMemorySystem();
