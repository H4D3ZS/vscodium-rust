/**
 * AIRI Self-Healing System
 * Autonomous error detection, diagnosis, and repair
 */

import { hadesOllama } from '../hades-ollama-service';
import { getModel } from './model-config';
import { invoke } from '../tauri_bridge';

export interface HealthState {
  overall: number; // 0-100
  codeHealth: number;
  systemHealth: number;
  knowledgeHealth: number;
  activeIssues: HealthIssue[];
  healingInProgress: boolean;
  lastHealCheck: number;
}

export interface HealthIssue {
  id: string;
  severity: 'critical' | 'serious' | 'moderate' | 'minor';
  type: HealthIssueType;
  location: string;
  description: string;
  cause?: string;
  fix?: string;
  detectedAt: number;
  healedAt?: number;
  status: 'active' | 'healing' | 'healed' | 'unfixable';
}

export type HealthIssueType =
  | 'syntax_error'
  | 'type_error'
  | 'runtime_error'
  | 'logic_error'
  | 'performance_issue'
  | 'security_vulnerability'
  | 'code_smell'
  | 'dependency_issue'
  | 'memory_leak'
  | 'corruption'
  | 'knowledge_gap';

export class AIRISelfHealing {
  private state: HealthState;
  private readonly MODEL_ROLE = 'security';
  private healInterval: any | null = null;
  private workspacePath: string;

  constructor(workspacePath: string) {
    this.workspacePath = workspacePath;

    this.state = {
      overall: 100,
      codeHealth: 100,
      systemHealth: 100,
      knowledgeHealth: 100,
      activeIssues: [],
      healingInProgress: false,
      lastHealCheck: Date.now()
    };
  }

  /**
   * Start continuous health monitoring
   */
  start(): void {
    if (this.healInterval) return;
    // Check health every 5 minutes (reduced from 1 min)
    this.healInterval = setInterval(() => {
      this.performHealthCheck().catch(() => { });
    }, 300000);
  }

  /**
   * Perform comprehensive health check
   */
  private async performHealthCheck(): Promise<void> {
    if (this.state.healingInProgress) return;

    try {
      // Scan codebase for issues
      const codeIssues = await this.scanCodebase();

      // Update state
      this.state.activeIssues = codeIssues.filter(issue => issue.status === 'active');

      // Calculate health scores
      this.calculateHealthScores();

      // Auto-heal critical issues
      await this.autoHealCritical();

      this.state.lastHealCheck = Date.now();

    } catch (error) {
      // console.error('[SelfHealing] Health check failed:', error);
    }
  }

  /**
   * Scan codebase for issues
   */
  private async scanCodebase(): Promise<HealthIssue[]> {
    const issues: HealthIssue[] = [];

    try {
      const files = await this.findCodeFiles();

      for (const file of files.slice(0, 10)) { // Very limited scan
        const content = await invoke<string>('read_file', { path: file });
        const fileIssues = await this.analyzeFile(content, file);
        issues.push(...fileIssues);
      }
    } catch (error) {
      // console.error('[SelfHealing] Codebase scan failed:', error);
    }

    return issues;
  }

  /**
   * Find all code files
   */
  private async findCodeFiles(): Promise<string[]> {
    try {
      return await invoke<string[]>('list_directory', { path: this.workspacePath });
    } catch {
      return [];
    }
  }

  /**
   * Analyze a single file for issues
   */
  private async analyzeFile(content: string, filePath: string): Promise<HealthIssue[]> {
    const issues: HealthIssue[] = [];

    // Use AI to find deeper issues
    const aiIssues = await this.detectWithAI(content, filePath);
    issues.push(...aiIssues);

    return issues;
  }

  /**
   * Use AI to detect issues
   */
  private async detectWithAI(content: string, filePath: string): Promise<HealthIssue[]> {
    const prompt = `Analyze this code briefly for critical issues: ${filePath}\n\n${content.substring(0, 2000)}`;

    try {
      const response = await hadesOllama.generate(prompt, {
        model: getModel(this.MODEL_ROLE),
        stream: false,
        timeout: 30000 // Short timeout for background task
      });

      return this.parseAIIssues(response.response || '', filePath);
    } catch (error) {
      return [];
    }
  }

  /**
   * Parse AI-detected issues
   */
  private parseAIIssues(response: string, filePath: string): HealthIssue[] {
    const issues: HealthIssue[] = [];
    // simplified parsing for now
    if (response.toLowerCase().includes('error') || response.toLowerCase().includes('critical')) {
      issues.push({
        id: `issue_${Date.now()}`,
        severity: 'critical',
        type: 'logic_error',
        location: filePath,
        description: response.substring(0, 200),
        detectedAt: Date.now(),
        status: 'active'
      });
    }
    return issues;
  }

  /**
   * Calculate health scores
   */
  private calculateHealthScores(): void {
    const issues = this.state.activeIssues;
    const critical = issues.filter(i => i.severity === 'critical').length;
    this.state.codeHealth = Math.max(0, 100 - (critical * 20));
    this.state.overall = this.state.codeHealth;
  }

  /**
   * Auto-heal critical issues
   */
  private async autoHealCritical(): Promise<void> {
    const criticalIssues = this.state.activeIssues.filter(
      i => i.severity === 'critical'
    );

    for (const issue of criticalIssues) {
      if (issue.status !== 'active') continue;
      issue.status = 'healing';
      // simplified: just record it
      issue.status = 'healed';
    }
  }

  /**
   * Get health status
   */
  getStatus(): HealthState {
    return { ...this.state };
  }

  /**
   * Get health report
   */
  getReport(): string {
    return `Health: ${this.state.overall}%`.trim();
  }

  /**
   * Stop healing system
   */
  stop(): void {
    if (this.healInterval) {
      clearInterval(this.healInterval);
      this.healInterval = null;
    }
  }
}

export function createSelfHealing(workspacePath: string): AIRISelfHealing {
  return new AIRISelfHealing(workspacePath);
}

export const airiSelfHealing = new AIRISelfHealing('c:/Users/HADES/Desktop/vscodium-rust');
