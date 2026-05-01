/**
 * AIRI Continuous Self-Improvement Engine
 * AIRI constantly analyzes, optimizes, and upgrades herself
 * Never stops evolving - every execution makes her better
 */

import { Ollama } from 'ollama';
import * as fs from 'fs/promises';
import * as path from 'path';

export interface Optimization {
  id: string;
  type: OptimizationType;
  description: string;
  impact: number; // 0-100
  effort: number; // 0-100
  status: 'identified' | 'planned' | 'implemented' | 'verified';
  codeChanges?: string;
  performanceGain?: number;
}

export type OptimizationType =
  | 'performance'
  | 'memory'
  | 'code_quality'
  | 'error_handling'
  | 'security'
  | 'feature'
  | 'refactoring';

export interface EvolutionCycle {
  cycle: number;
  timestamp: number;
  optimizationsApplied: number;
  performanceBefore: number;
  performanceAfter: number;
  codeChanges: string[];
}

export class AIRIContinuousImprovement {
  private ollama: Ollama;
  private optimizations: Optimization[];
  private evolutionHistory: EvolutionCycle[];
  private improvementInterval: NodeJS.Timeout | null;
  private readonly MODEL = 'qwen3.6:32b-q4_K_M';
  private codebasePath: string;
  private cycleCount: number = 0;

  constructor(codebasePath: string) {
    this.ollama = new Ollama({ host: 'http://localhost:1536' }); // AIM proxy
    this.optimizations = [];
    this.evolutionHistory = [];
    this.codebasePath = codebasePath;
    this.improvementInterval = null;

  }

  /**
   * Start continuous improvement loop
   * Runs every 30 minutes
   */
  start(): void {
    
    // Analyze and improve every 30 minutes
    this.improvementInterval = setInterval(() => {
      this.runImprovementCycle();
    }, 30 * 60 * 1000);

    // Run first cycle immediately
    this.runImprovementCycle();
  }

  /**
   * Run a complete improvement cycle
   */
  async runImprovementCycle(): Promise<void> {
    this.cycleCount++;

    const cycle: EvolutionCycle = {
      cycle: this.cycleCount,
      timestamp: Date.now(),
      optimizationsApplied: 0,
      performanceBefore: 0,
      performanceAfter: 0,
      codeChanges: []
    };

    try {
      // Phase 1: Analyze current state
      const analysis = await this.analyzeCodebase();
      cycle.performanceBefore = analysis.performanceScore;

      // Phase 2: Identify optimizations
      const newOptimizations = await this.identifyOptimizations(analysis);

      // Phase 3: Prioritize optimizations
      const prioritized = this.prioritizeOptimizations(newOptimizations);

      // Phase 4: Implement top optimizations
      for (const opt of prioritized.slice(0, 5)) { // Top 5 per cycle
        const result = await this.implementOptimization(opt);
        if (result) {
          cycle.optimizationsApplied++;
          cycle.codeChanges.push(opt.description);
          if (result.performanceGain) {
            cycle.performanceAfter += result.performanceGain;
          }
        }
      }

      // Phase 5: Verify improvements
      const afterAnalysis = await this.analyzeCodebase();
      cycle.performanceAfter = afterAnalysis.performanceScore;

      // Record cycle
      this.evolutionHistory.push(cycle);
      
      // Log results
      this.logCycleResults(cycle);

    } catch (error) {
      console.error('[ContinuousImprovement] ❌ Cycle failed:', error);
    }
  }

  /**
   * Analyze entire codebase
   */
  private async analyzeCodebase(): Promise<{
    performanceScore: number;
    codeQualityScore: number;
    issues: string[];
    metrics: any;
  }> {
    const files = await this.getSourceFiles();
    let totalPerfScore = 0;
    let totalQualityScore = 0;
    const issues: string[] = [];

    // Analyze each file (limit to 30 for performance)
    for (const file of files.slice(0, 30)) {
      const content = await fs.readFile(file, 'utf-8');
      const analysis = await this.analyzeFile(content, file);
      
      totalPerfScore += analysis.performanceScore;
      totalQualityScore += analysis.codeQualityScore;
      issues.push(...analysis.issues);
    }

    const fileCount = Math.min(files.length, 30);
    
    return {
      performanceScore: Math.round(totalPerfScore / fileCount),
      codeQualityScore: Math.round(totalQualityScore / fileCount),
      issues,
      metrics: {
        totalFiles: files.length,
        analyzedFiles: fileCount,
        issuesFound: issues.length
      }
    };
  }

  /**
   * Analyze single file
   */
  private async analyzeFile(content: string, filePath: string): Promise<{
    performanceScore: number;
    codeQualityScore: number;
    issues: string[];
  }> {
    const prompt = `
Analyze this code for performance and quality:

File: ${filePath}

${content.substring(0, 4000)}

Rate performance (0-100):
- Function length
- Nested loops
- Memory usage
- Async handling
- Caching

Rate code quality (0-100):
- Readability
- Error handling
- Type safety
- Comments
- Best practices

List specific issues found.

Respond with JSON:
{
  "performanceScore": number,
  "codeQualityScore": number,
  "issues": ["issue1", "issue2"]
}
`;

    try {
      const response = await this.ollama.generate({
        model: this.MODEL,
        prompt,
        stream: false
      });

      // Parse JSON response
      const jsonMatch = response.response.match(/\{[\s\S]*\}/);
      if (jsonMatch) {
        const analysis = JSON.parse(jsonMatch[0]);
        return {
          performanceScore: analysis.performanceScore || 50,
          codeQualityScore: analysis.codeQualityScore || 50,
          issues: analysis.issues || []
        };
      }

      return {
        performanceScore: 50,
        codeQualityScore: 50,
        issues: []
      };
    } catch (error) {
      console.error(`[ContinuousImprovement] Analysis failed for ${filePath}:`, error);
      return {
        performanceScore: 50,
        codeQualityScore: 50,
        issues: ['Analysis failed']
      };
    }
  }

  /**
   * Identify optimizations from analysis
   */
  private async identifyOptimizations(analysis: any): Promise<Optimization[]> {
    const optimizations: Optimization[] = [];

    // Use AI to generate optimization suggestions
    const prompt = `
Based on this codebase analysis:
- Performance Score: ${analysis.performanceScore}/100
- Code Quality Score: ${analysis.codeQualityScore}/100
- Issues: ${analysis.issues.join(', ')}

Generate specific, actionable optimizations.
For each optimization, provide:
1. Type (performance/memory/code_quality/error_handling/security/refactoring)
2. Description
3. Impact (0-100)
4. Effort (0-100)

Respond as JSON array:
[
  {
    "type": "performance",
    "description": "Add caching to frequently-called function",
    "impact": 75,
    "effort": 30
  }
]
`;

    try {
      const response = await this.ollama.generate({
        model: this.MODEL,
        prompt,
        stream: false
      });

      const jsonMatch = response.response.match(/\[[\s\S]*\]/);
      if (jsonMatch) {
        const opts = JSON.parse(jsonMatch[0]);
        
        for (const opt of opts) {
          optimizations.push({
            id: `opt_${Date.now()}_${Math.random()}`,
            type: opt.type as OptimizationType,
            description: opt.description,
            impact: opt.impact,
            effort: opt.effort,
            status: 'identified'
          });
        }
      }
    } catch (error) {
      console.error('[ContinuousImprovement] Optimization identification failed:', error);
    }

    return optimizations;
  }

  /**
   * Prioritize optimizations by impact/effort ratio
   */
  private prioritizeOptimizations(optimizations: Optimization[]): Optimization[] {
    return optimizations
      .filter(opt => opt.status === 'identified')
      .sort((a, b) => {
        const ratioA = a.impact / (a.effort || 1);
        const ratioB = b.impact / (b.effort || 1);
        return ratioB - ratioA; // Higher ratio first
      });
  }

  /**
   * Implement optimization
   */
  private async implementOptimization(optimization: Optimization): Promise<{
    success: boolean;
    performanceGain?: number;
  } | null> {

    optimization.status = 'planned';

    // Generate code changes
    const prompt = `
Generate code changes for this optimization:

${optimization.description}

Type: ${optimization.type}

Provide:
1. File path(s) to modify
2. Current code (brief excerpt)
3. New optimized code
4. Explanation of improvement

Format:
{
  "file": "path/to/file.ts",
  "currentCode": "...",
  "newCode": "...",
  "explanation": "..."
}
`;

    try {
      const response = await this.ollama.generate({
        model: this.MODEL,
        prompt,
        stream: false
      });

      const jsonMatch = response.response.match(/\{[\s\S]*\}/);
      if (jsonMatch) {
        const changes = JSON.parse(jsonMatch[0]);
        
        // Apply changes
        const filePath = path.join(this.codebasePath, changes.file);
        
        // Backup original
        const original = await fs.readFile(filePath, 'utf-8');
        await fs.writeFile(filePath + '.backup', original, 'utf-8');

        // Apply new code
        await fs.writeFile(filePath, changes.newCode, 'utf-8');

        optimization.status = 'implemented';
        optimization.codeChanges = changes.newCode;


        // Verify improvement
        const performanceGain = await this.verifyImprovement(filePath, original);
        
        if (performanceGain > 0) {
          optimization.performanceGain = performanceGain;
          optimization.status = 'verified';
          
          // Remove backup on success
          await fs.unlink(filePath + '.backup');
          
          return { success: true, performanceGain };
        } else {
          // Rollback on failure
          await fs.writeFile(filePath, original, 'utf-8');
          await fs.unlink(filePath + '.backup');
          
          return { success: false };
        }
      }
    } catch (error) {
      console.error(`   ❌ Implementation failed:`, error);
    }

    return null;
  }

  /**
   * Verify improvement was actually made
   */
  private async verifyImprovement(filePath: string, originalCode: string): Promise<number> {
    const newCode = await fs.readFile(filePath, 'utf-8');
    
    // Use AI to compare and estimate performance gain
    const prompt = `
Compare these two code versions and estimate performance improvement:

ORIGINAL:
${originalCode.substring(0, 2000)}

NEW:
${newCode.substring(0, 2000)}

Estimate performance gain as percentage (0-100).
Respond with just a number.
`;

    try {
      const response = await this.ollama.generate({
        model: this.MODEL,
        prompt,
        stream: false
      });

      const match = response.response.match(/(\d+)/);
      return parseInt(match?.[1] || '0');
    } catch (error) {
      return 0;
    }
  }

  /**
   * Get source files
   */
  private async getSourceFiles(): Promise<string[]> {
    const files: string[] = [];

    async function walk(dir: string): Promise<string[]> {
      const found: string[] = [];
      try {
        const entries = await fs.readdir(dir, { withFileTypes: true });
        
        for (const entry of entries) {
          if (entry.name.startsWith('.') || entry.name === 'node_modules') {
            continue;
          }

          const fullPath = path.join(dir, entry.name);
          
          if (entry.isDirectory()) {
            const subFiles = await walk(fullPath);
            found.push(...subFiles);
          } else if (entry.isFile() && entry.name.endsWith('.ts')) {
            found.push(fullPath);
          }
        }
      } catch (error) {
        // Ignore inaccessible directories
      }
      
      return found;
    }

    return walk(this.codebasePath);
  }

  /**
   * Log cycle results
   */
  private logCycleResults(cycle: EvolutionCycle): void {
    const gain = cycle.performanceAfter - cycle.performanceBefore;
    
    
    if (cycle.codeChanges.length > 0) {
      cycle.codeChanges.forEach((change, i) => {
      });
    }
  }

  /**
   * Get evolution stats
   */
  getStats(): {
    totalCycles: number;
    totalOptimizations: number;
    averageGain: number;
    currentVersion: string;
    evolutionTrend: 'improving' | 'stable' | 'degrading';
  } {
    const totalOptimizations = this.evolutionHistory.reduce(
      (sum, cycle) => sum + cycle.optimizationsApplied,
      0
    );

    const gains = this.evolutionHistory.map(
      cycle => cycle.performanceAfter - cycle.performanceBefore
    );
    
    const averageGain = gains.length > 0
      ? gains.reduce((a, b) => a + b, 0) / gains.length
      : 0;

    // Determine trend (last 5 cycles)
    const recentGains = gains.slice(-5);
    const trend = recentGains.length > 0 && recentGains.every(g => g >= 0)
      ? 'improving' as const
      : recentGains.some(g => g < -5)
        ? 'degrading' as const
        : 'stable' as const;

    return {
      totalCycles: this.cycleCount,
      totalOptimizations,
      averageGain: Math.round(averageGain * 100) / 100,
      currentVersion: `3.0.${this.cycleCount}`,
      evolutionTrend: trend
    };
  }

  /**
   * Stop improvement loop
   */
  stop(): void {
    if (this.improvementInterval) {
      clearInterval(this.improvementInterval);
    }
  }
}

// Export factory
export function createAIRIContinuousImprovement(codebasePath: string): AIRIContinuousImprovement {
  return new AIRIContinuousImprovement(codebasePath);
}
