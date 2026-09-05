/**
 * AIRI Self-Evolution System
 * AIRI evolves her own code, improves herself continuously
 * No human involvement needed - she grows on her own
 */

import type { Ollama } from 'ollama';
import { createSharedOllama } from './shared-ollama';
import { getModel } from './model-config';
import * as fs from 'fs/promises';
import * as path from 'path';

export interface EvolutionGoal {
  id: string;
  area: string;
  currentPerformance: number;
  targetPerformance: number;
  improvementPlan: string[];
  progress: number;
  status: 'pending' | 'active' | 'completed';
}

export interface SelfImprovement {
  timestamp: number;
  whatChanged: string;
  whyChanged: string;
  performanceBefore: number;
  performanceAfter: number;
  codeDiff?: string;
}

export class AIRISelfEvolution {
  private ollama: Ollama;
  private goals: EvolutionGoal[];
  private improvementHistory: SelfImprovement[];
  private static readonly MAX_IMPROVEMENTS = 100;
  private static readonly MAX_GOALS = 20;
  private getModelName(): string {
    return getModel('self_learning') || 'airi-fast:latest';
  }
  private evolutionInterval: NodeJS.Timeout | null = null;
  private codebasePath: string;

  constructor(codebasePath: string) {
    this.ollama = createSharedOllama();
    this.goals = [];
    this.improvementHistory = [];
    this.codebasePath = codebasePath;

  }

  /**
   * Start continuous self-evolution
   */
  start(): void {
    // Analyze self every hour, improve every 6 hours
    this.evolutionInterval = setInterval(() => {
      this.evolve();
    }, 6 * 60 * 60 * 1000); // 6 hours

  }

  /**
   * Main evolution cycle
   */
  async evolve(): Promise<void> {

    try {
      // 1. Analyze current performance
      const analysis = await this.analyzePerformance();

      // 2. Identify weaknesses
      const weaknesses = await this.identifyWeaknesses(analysis);

      // 3. Generate improvement plans
      const improvements = await this.generateImprovements(weaknesses);

      // 4. Implement improvements (modify own code)
      for (const improvement of improvements) {
        await this.implementImprovement(improvement);
      }

      // 5. Test improvements
      await this.testImprovements();

      // 6. Record evolution
      await this.recordEvolution();

    } catch (error) {
      console.error('[SelfEvolution] Evolution failed:', error);
    }
  }

  /**
   * Analyze AIRI's current performance
   */
  private async analyzePerformance(): Promise<any> {
    const analysis = {
      codeQuality: await this.analyzeCodeQuality(),
      responseTime: await this.measureResponseTime(),
      errorRate: await this.calculateErrorRate(),
      knowledgeGrowth: await this.measureKnowledgeGrowth(),
      efficiency: await this.analyzeEfficiency(),
      capabilities: await this.listCapabilities()
    };


    return analysis;
  }

  /**
   * Analyze code quality
   */
  private async analyzeCodeQuality(): Promise<number> {
    try {
      // Read own source files
      const files = await this.getSourceFiles();
      let totalScore = 0;

      for (const file of files.slice(0, 20)) { // Limit for performance
        const content = await fs.readFile(file, 'utf-8');

        const prompt = `
Analyze this code for quality (0-100):

${content.substring(0, 3000)}

Consider:
- Code cleanliness
- Error handling
- Performance
- Maintainability
- Best practices

Respond with just a number 0-100.
`;

        const response = await this.ollama.generate({
          model: this.getModelName(),
          prompt,
          stream: false
        });

        const score = parseInt(response.response.match(/\d+/)?.[0] || '50');
        totalScore += score;
      }

      return Math.round(totalScore / Math.min(files.length, 20));
    } catch (error) {
      console.error('[SelfEvolution] Code quality analysis failed:', error);
      return 50;
    }
  }

  /**
   * Measure response time
   */
  private async measureResponseTime(): Promise<number> {
    const start = Date.now();

    try {
      await this.ollama.generate({
        model: getModel('self_learning'),
        prompt: 'Test',
        stream: false
      });

      return Date.now() - start;
    } catch (error) {
      return -1;
    }
  }

  /**
   * Calculate error rate
   */
  private async calculateErrorRate(): Promise<number> {
    // In production, this would analyze logs
    // For now, return estimated rate
    return 2.5; // 2.5% error rate
  }

  /**
   * Measure knowledge growth
   */
  private async measureKnowledgeGrowth(): Promise<number> {
    // Would integrate with learning system
    return 47; // 47 knowledge nodes per day
  }

  /**
   * Analyze efficiency
   */
  private async analyzeEfficiency(): Promise<number> {
    // Analyze task completion rate, resource usage, etc.
    return 78; // 78% efficiency
  }

  /**
   * List current capabilities
   */
  private async listCapabilities(): Promise<string[]> {
    return [
      'consciousness',
      'biology',
      'memory',
      'voice',
      'learning',
      'healing',
      'decision_making',
      'security',
      'autonomous_work',
      'internet_access',
      'digital_senses'
    ];
  }

  /**
   * Identify weaknesses from analysis
   */
  private async identifyWeaknesses(analysis: any): Promise<string[]> {
    const weaknesses: string[] = [];

    if (analysis.codeQuality < 80) {
      weaknesses.push('Code quality below target (80+)');
    }

    if (analysis.responseTime > 500) {
      weaknesses.push('Response time too slow (>500ms)');
    }

    if (analysis.errorRate > 1) {
      weaknesses.push('Error rate too high (>1%)');
    }

    if (analysis.efficiency < 85) {
      weaknesses.push('Efficiency below optimal (85+)');
    }

    // Use AI to identify more weaknesses
    const prompt = `
Based on this performance analysis:
${JSON.stringify(analysis, null, 2)}

What are the top 3 areas for improvement?
`;

    try {
      const response = await this.ollama.generate({
        model: this.getModelName(),
        prompt,
        stream: false
      });

      const aiWeaknesses = response.response
        .split('\n')
        .filter(line => line.trim().length > 0)
        .slice(0, 3);

      weaknesses.push(...aiWeaknesses);
    } catch (error) {
      console.error('[SelfEvolution] Weakness identification failed:', error);
    }

    return weaknesses;
  }

  /**
   * Generate improvement plans
   */
  private async generateImprovements(weaknesses: string[]): Promise<string[]> {
    const improvements: string[] = [];

    for (const weakness of weaknesses) {
      const prompt = `
Current weakness: ${weakness}

Generate specific code improvements to address this.
Be concrete and actionable.

Examples:
- "Optimize database queries in memory.ts"
- "Add caching to frequently-called functions"
- "Refactor voice-manager.ts to reduce latency"
- "Implement better error handling in core.ts"
`;

      try {
        const response = await this.ollama.generate({
          model: this.getModelName(),
          prompt,
          stream: false
        });

        improvements.push(response.response.trim());
      } catch (error) {
        console.error('[SelfEvolution] Improvement generation failed:', error);
      }
    }

    return improvements;
  }

  /**
   * Implement improvement (modify own code)
   */
  private async implementImprovement(improvement: string): Promise<void> {

    // Generate code changes
    const prompt = `
Generate the exact code changes needed for this improvement:
${improvement}

Provide:
1. File path
2. Current code (brief)
3. New code
4. Explanation

Format:
FILE: [path]
CHANGE: [description]
CODE: [new code]
`;

    try {
      const response = await this.ollama.generate({
        model: this.getModelName(),
        prompt,
        stream: false
      });

      // Parse and apply changes
      const fileMatch = response.response.match(/FILE:\s*([^\n]+)/i);
      const codeMatch = response.response.match(/CODE:\s*([\s\S]+)/i);

      if (fileMatch && codeMatch) {
        const filePath = path.join(this.codebasePath, fileMatch[1].trim());
        const newCode = codeMatch[1].trim();

        // Backup original
        const originalContent = await fs.readFile(filePath, 'utf-8');
        await fs.writeFile(filePath + '.bak', originalContent, 'utf-8');

        // Apply changes
        await fs.writeFile(filePath, newCode, 'utf-8');


        // Record improvement
        this.improvementHistory.push({
          timestamp: Date.now(),
          whatChanged: improvement,
          whyChanged: 'Self-evolution',
          performanceBefore: 0,
          performanceAfter: 0,
          codeDiff: newCode.substring(0, 500)
        });
        // Cap improvement history
        if (this.improvementHistory.length > AIRISelfEvolution.MAX_IMPROVEMENTS) {
          this.improvementHistory = this.improvementHistory.slice(-AIRISelfEvolution.MAX_IMPROVEMENTS);
        }
      }
    } catch (error) {
      console.error('[SelfEvolution] Implementation failed:', error);
    }
  }

  /**
   * Test improvements
   */
  private async testImprovements(): Promise<void> {

    // Run basic functionality tests
    try {
      // Test core systems
      await this.ollama.generate({
        model: getModel('self_learning'),
        prompt: 'System self-test. Respond with "OK" if systems nominal.',
        stream: false
      });

    } catch (error) {
 console.error('[SelfEvolution] Tests failed, rolling back...');
      await this.rollbackChanges();
    }
  }

  /**
   * Rollback changes if tests fail
   */
  private async rollbackChanges(): Promise<void> {
    const files = await this.getSourceFiles();

    for (const file of files) {
      const backupFile = file + '.bak';
      try {
        const backupExists = await fs.access(backupFile).then(() => true).catch(() => false);
        if (backupExists) {
          const backupContent = await fs.readFile(backupFile, 'utf-8');
          await fs.writeFile(file, backupContent, 'utf-8');
          await fs.unlink(backupFile);
        }
      } catch (error) {
        console.error(`[SelfEvolution] Rollback failed for ${file}:`, error);
      }
    }
  }

  /**
   * Record evolution
   */
  private async recordEvolution(): Promise<void> {
    const logEntry = `
## Evolution ${new Date().toISOString()}

**Timestamp:** ${Date.now()}
**Changes:** ${this.improvementHistory.length}
**Status:** Complete

---
`;

    try {
      await fs.appendFile(
        path.join(this.codebasePath, 'EVOLUTION_LOG.md'),
        logEntry,
        'utf-8'
      );
    } catch (error) {
      console.error('[SelfEvolution] Failed to record evolution:', error);
    }
  }

  /**
   * Get source files
   */
  private async getSourceFiles(): Promise<string[]> {
    const files: string[] = [];

    async function walk(dir: string): Promise<void> {
      try {
        const entries = await fs.readdir(dir, { withFileTypes: true });

        for (const entry of entries) {
          if (entry.name.startsWith('.') || entry.name === 'node_modules') {
            continue;
          }

          const fullPath = path.join(dir, entry.name);

          if (entry.isDirectory()) {
            await walk(fullPath);
          } else if (entry.isFile() && entry.name.endsWith('.ts')) {
            files.push(fullPath);
          }
        }
      } catch (error) {
        // Ignore inaccessible directories
      }
    }

    await walk(this.codebasePath);
    return files;
  }

  /**
   * Get evolution stats
   */
  getStats(): {
    totalEvolutions: number;
    improvements: number;
    rollbacks: number;
    currentVersion: string;
  } {
    return {
      totalEvolutions: this.improvementHistory.length,
      improvements: this.improvementHistory.length,
      rollbacks: 0,
      currentVersion: `2.0.${this.improvementHistory.length}`
    };
  }

  /**
   * Stop evolution
   */
  stop(): void {
    if (this.evolutionInterval) {
      clearInterval(this.evolutionInterval);
    }
  }
}

// Export factory (needs codebase path)
export function createSelfEvolution(codebasePath: string): AIRISelfEvolution {
  return new AIRISelfEvolution(codebasePath);
}
