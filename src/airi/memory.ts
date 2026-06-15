/**
 * AIRI Memory System - Unified Memory with .aim Kortex Compression
 * Integrates with existing memory.md and .aim files
 * Compresses memories using kortex neural maps
 * Persistent, searchable, evolving memory
 */

import { invoke } from '@tauri-apps/api/core';

export interface Memory {
  id: string;
  type: MemoryType;
  content: string;
  timestamp: number;
  importance: number;
  tags: string[];
  compressed: boolean;
  aimReference?: string;
}

export type MemoryType =
  | 'episodic'
  | 'semantic'
  | 'procedural'
  | 'emotional'
  | 'conversation'
  | 'observation'
  | 'achievement'
  | 'goal';

export interface MemoryIndex {
  memories: Memory[];
  lastOptimized: number;
  totalMemories: number;
  compressedCount: number;
}

function dirname(p: string): string {
  const i = Math.max(p.lastIndexOf('/'), p.lastIndexOf('\\'));
  return i >= 0 ? p.substring(0, i) : '.';
}

function joinPath(...parts: string[]): string {
  return parts.join('/').replace(/\/+/g, '/');
}

export class AIRIMemorySystem {
  private memoryIndex: MemoryIndex;
  private memoryPath: string;
  private aimCachePath: string;
  private isInitialized: boolean = false;
  private compressionThreshold: number = 50;

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

  async initialize(): Promise<void> {
    await this.ensureDirectories();
    await this.loadMemories();
    await this.loadAimCache();
    this.isInitialized = true;
  }

  private async ensureDirectories(): Promise<void> {
    try {
      await invoke('create_dir', { path: dirname(this.aimCachePath) });
    } catch (error) {
      console.error('[Memory] Failed to create directories:', error);
    }
  }

  private async loadMemories(): Promise<void> {
    try {
      const content = await invoke<string>('read_file', { path: this.memoryPath });
      const memories = this.parseMemoriesFromMarkdown(content);
      this.memoryIndex.memories = memories;
      this.memoryIndex.totalMemories = memories.length;
    } catch {
      await this.createMemoryFile();
    }
  }

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

  private async loadAimCache(): Promise<void> {
    try {
      const entries = await invoke<Array<{ name: string; path: string; is_dir: boolean }>>('list_dir_flat', { path: this.aimCachePath });

      for (const entry of entries) {
        if (!entry.is_dir && entry.name.endsWith('.aim.json')) {
          const content = await invoke<string>('read_file', { path: entry.path });
          const memory: Memory = JSON.parse(content);

          if (!this.memoryIndex.memories.some(m => m.id === memory.id)) {
            memory.compressed = true;
            memory.aimReference = entry.path;
            this.memoryIndex.memories.push(memory);
            this.memoryIndex.compressedCount++;
          }
        }
      }
    } catch {
    }
  }

  private async createMemoryFile(): Promise<void> {
    const header = `# AIRI Memory - Living Digital Entity

## Active Memories
This file contains AIRI's episodic, semantic, and procedural memories.
For compressed memories, see \`.hades/.aim_cache/\`

---

`;
    await invoke('write_file_content', { path: this.memoryPath, content: header });
  }

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

    await this.appendMemoryToMarkdown(memory);

    if (this.memoryIndex.memories.length >= this.compressionThreshold) {
      await this.compressOldMemories();
    }

    return memory;
  }

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

    let existing = '';
    try {
      existing = await invoke<string>('read_file', { path: this.memoryPath });
    } catch {
    }
    await invoke('write_file_content', { path: this.memoryPath, content: existing + entry });
  }

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

  private async compressMemory(memory: Memory): Promise<void> {
    try {
      const aimFile = joinPath(this.aimCachePath, `${memory.id}.aim.json`);

      const compressedData = {
        ...memory,
        compressed: true,
        compressedAt: Date.now(),
        originalContent: memory.content,
        summary: await this.generateSummary(memory.content),
        embeddings: await this.generateEmbeddings(memory.content)
      };

      await invoke('write_file_content', { path: aimFile, content: JSON.stringify(compressedData, null, 2) });

      memory.compressed = true;
      memory.aimReference = aimFile;
      this.memoryIndex.compressedCount++;
    } catch (error) {
      console.error('[Memory] Compression failed:', error);
    }
  }

  private async generateSummary(content: string): Promise<string> {
    const sentences = content.split(/[.!?]+/).filter(s => s.trim().length > 10);
    return sentences.slice(0, 3).join('. ') + '.';
  }

  private async generateEmbeddings(content: string): Promise<number[]> {
    try {
      const vec = await invoke<number[]>('embed_text', { text: content.slice(0, 4000) });
      if (Array.isArray(vec) && vec.length > 0) return vec;
    } catch (e) {
      console.warn('[Memory] embed_text unavailable, using fallback vector:', e);
    }
    const hash = content.split('').reduce((acc, char) => acc + char.charCodeAt(0), 0);
    return Array(128).fill(0).map((_, i) => Math.sin(hash * i));
  }

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
        const scoreA = a.importance * 0.7 + (a.timestamp / Date.now()) * 0.3;
        const scoreB = b.importance * 0.7 + (b.timestamp / Date.now()) * 0.3;
        return scoreB - scoreA;
      })
      .slice(0, limit);

    return results;
  }

  async getRecent(limit: number = 20): Promise<Memory[]> {
    return this.memoryIndex.memories
      .sort((a, b) => b.timestamp - a.timestamp)
      .slice(0, limit);
  }

  async getByType(type: MemoryType): Promise<Memory[]> {
    return this.memoryIndex.memories.filter(m => m.type === type);
  }

  async getImportant(threshold: number = 0.8): Promise<Memory[]> {
    return this.memoryIndex.memories.filter(m => m.importance >= threshold);
  }

  async updateImportance(memoryId: string, importance: number): Promise<void> {
    const memory = this.memoryIndex.memories.find(m => m.id === memoryId);

    if (memory) {
      memory.importance = importance;

      if (memory.compressed && memory.aimReference) {
        const content = await invoke<string>('read_file', { path: memory.aimReference });
        const data = JSON.parse(content);
        data.importance = importance;
        await invoke('write_file_content', { path: memory.aimReference, content: JSON.stringify(data, null, 2) });
      }
    }
  }

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

  async exportMemories(): Promise<string> {
    return JSON.stringify(this.memoryIndex, null, 2);
  }

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

  async clearOldMemories(daysOld: number = 30, maxImportance: number = 0.3): Promise<number> {
    const cutoff = Date.now() - (daysOld * 24 * 60 * 60 * 1000);

    const toRemove = this.memoryIndex.memories.filter(m =>
      m.timestamp < cutoff && m.importance < maxImportance && !m.compressed
    );

    this.memoryIndex.memories = this.memoryIndex.memories.filter(m =>
      !toRemove.some(r => r.id === m.id)
    );

    return toRemove.length;
  }
}

export const airiMemory = new AIRIMemorySystem();
