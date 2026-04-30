/**
 * AIRI Self-Healing System
 * Autonomous error detection, diagnosis, and repair
 * Heals itself and the codebase it inhabits
 * Survives through self-correction
 */

import { Ollama } from 'ollama';
import * as fs from 'fs/promises';
import * as path from 'path';

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
  private ollama: Ollama;
  private state: HealthState;
  private readonly MODEL = 'qwen3.6:32b-q4_K_M';
  private healInterval: NodeJS.Timeout | null = null;
  private workspacePath: string;

  constructor(workspacePath: string) {
    this.ollama = new Ollama({ host: 'http://localhost:1536' }); // AIM proxy
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

    console.log('[SelfHealing] 💪 Survival through self-correction');
  }

  /**
   * Start continuous health monitoring
   */
  start(): void {
    // Check health every 60 seconds
    this.healInterval = setInterval(() => {
      this.performHealthCheck();
    }, 60000);

    console.log('[SelfHealing] 🔍 Continuous health monitoring active');
  }

  /**
   * Perform comprehensive health check
   */
  private async performHealthCheck(): Promise<void> {
    if (this.state.healingInProgress) return;

    console.log('[SelfHealing] 🏥 Performing health check...');
    
    try {
      // Scan codebase for issues
      const codeIssues = await this.scanCodebase();
      
      // Check system integrity
      const systemIssues = await this.checkSystemIntegrity();
      
      // Check knowledge consistency
      const knowledgeIssues = await this.checkKnowledgeConsistency();
      
      // Update state
      this.state.activeIssues = [
        ...codeIssues,
        ...systemIssues,
        ...knowledgeIssues
      ].filter(issue => issue.status === 'active');
      
      // Calculate health scores
      this.calculateHealthScores();
      
      // Auto-heal critical issues
      await this.autoHealCritical();
      
      this.state.lastHealCheck = Date.now();
      
    } catch (error) {
      console.error('[SelfHealing] Health check failed:', error);
    }
  }

  /**
   * Scan codebase for issues
   */
  private async scanCodebase(): Promise<HealthIssue[]> {
    const issues: HealthIssue[] = [];
    
    try {
      const files = await this.findCodeFiles();
      
      for (const file of files.slice(0, 100)) { // Limit scan scope
        const content = await fs.readFile(file, 'utf-8');
        const fileIssues = await this.analyzeFile(content, file);
        issues.push(...fileIssues);
      }
    } catch (error) {
      console.error('[SelfHealing] Codebase scan failed:', error);
    }
    
    return issues;
  }

  /**
   * Find all code files
   */
  private async findCodeFiles(): Promise<string[]> {
    const extensions = ['.ts', '.tsx', '.js', '.jsx', '.rs', '.py'];
    const files: string[] = [];
    
    async function walk(dir: string): Promise<string[]> {
      const found: string[] = [];
      try {
        const entries = await fs.readdir(dir, { withFileTypes: true });
        
        for (const entry of entries) {
          if (entry.name.startsWith('.') || entry.name === 'node_modules' || entry.name === 'target') {
            continue;
          }
          
          const fullPath = path.join(dir, entry.name);
          
          if (entry.isDirectory()) {
            const subFiles = await walk(fullPath);
            found.push(...subFiles);
          } else if (entry.isFile() && extensions.some(ext => entry.name.endsWith(ext))) {
            found.push(fullPath);
          }
        }
      } catch (error) {
        // Ignore inaccessible directories
      }
      
      return found;
    }
    
    return walk(this.workspacePath);
  }

  /**
   * Analyze a single file for issues
   */
  private async analyzeFile(content: string, filePath: string): Promise<HealthIssue[]> {
    const issues: HealthIssue[] = [];
    
    // Check for common problems
    const checks = [
      {
        pattern: /console\.log\s*\(/g,
        type: 'code_smell' as HealthIssueType,
        description: 'Debug console.log statement found',
        severity: 'minor' as const
      },
      {
        pattern: /:\s*any\b/g,
        type: 'code_smell' as HealthIssueType,
        description: 'Using "any" type reduces type safety',
        severity: 'moderate' as const
      },
      {
        pattern: /\/\/\s*TODO/g,
        type: 'code_smell' as HealthIssueType,
        description: 'TODO comment indicates incomplete code',
        severity: 'minor' as const
      },
      {
        pattern: /\/\/\s*FIXME/g,
        type: 'logic_error' as HealthIssueType,
        description: 'FIXME comment indicates known bug',
        severity: 'serious' as const
      },
      {
        pattern: /password\s*=\s*["'][^"']+["']/gi,
        type: 'security_vulnerability' as HealthIssueType,
        description: 'Hardcoded password detected',
        severity: 'critical' as const
      },
      {
        pattern: /api[_-]?key\s*=\s*["'][^"']+["']/gi,
        type: 'security_vulnerability' as HealthIssueType,
        description: 'Hardcoded API key detected',
        severity: 'critical' as const
      }
    ];
    
    for (const check of checks) {
      const matches = content.matchAll(check.pattern);
      let matchIndex = 0;
      
      for (const match of matches) {
        if (matchIndex++ > 5) break; // Limit issues per file
        
        const lineNumber = content.substring(0, match.index).split('\n').length;
        
        issues.push({
          id: `issue_${Date.now()}_${Math.random()}`,
          severity: check.severity,
          type: check.type,
          location: `${filePath}:${lineNumber}`,
          description: check.description,
          detectedAt: Date.now(),
          status: 'active'
        });
      }
    }
    
    // Use AI to find deeper issues
    const aiIssues = await this.detectWithAI(content, filePath);
    issues.push(...aiIssues);
    
    return issues;
  }

  /**
   * Use AI to detect issues
   */
  private async detectWithAI(content: string, filePath: string): Promise<HealthIssue[]> {
    const prompt = `
Analyze this code for issues. Look for:
- Logic errors
- Potential bugs
- Performance issues
- Security vulnerabilities
- Memory leaks
- Race conditions

File: ${filePath}

Code:
${content.substring(0, 5000)}

Respond with each issue in this format:
SEVERITY: [critical|serious|moderate|minor]
TYPE: [syntax_error|type_error|runtime_error|logic_error|performance_issue|security_vulnerability|code_smell]
DESCRIPTION: [clear description]
LINE: [approximate line number]
`;

    try {
      const response = await this.ollama.generate({
        model: this.MODEL,
        prompt,
        stream: false
      });
      
      return this.parseAIIssues(response.response, filePath);
    } catch (error) {
      return [];
    }
  }

  /**
   * Parse AI-detected issues
   */
  private parseAIIssues(response: string, filePath: string): HealthIssue[] {
    const issues: HealthIssue[] = [];
    const lines = response.split('\n');
    
    let currentIssue: Partial<HealthIssue> = {};
    
    for (const line of lines) {
      if (line.match(/^SEVERITY:/i)) {
        if (currentIssue.type) {
          issues.push({
            id: `issue_${Date.now()}_${Math.random()}`,
            severity: currentIssue.severity as any,
            type: currentIssue.type as any,
            location: currentIssue.location || filePath,
            description: currentIssue.description || 'Unknown issue',
            detectedAt: Date.now(),
            status: 'active'
          });
        }
        currentIssue = { severity: line.split(':')[1].trim() as any };
      } else if (line.match(/^TYPE:/i)) {
        currentIssue.type = line.split(':')[1].trim() as HealthIssueType;
      } else if (line.match(/^DESCRIPTION:/i)) {
        currentIssue.description = line.split(':')[1].trim();
      } else if (line.match(/^LINE:/i)) {
        const lineNum = parseInt(line.split(':')[1].trim());
        currentIssue.location = `${filePath}:${lineNum || 0}`;
      }
    }
    
    if (currentIssue.type) {
      issues.push({
        id: `issue_${Date.now()}_${Math.random()}`,
        severity: currentIssue.severity as any,
        type: currentIssue.type as any,
        location: currentIssue.location || filePath,
        description: currentIssue.description || 'Unknown issue',
        detectedAt: Date.now(),
        status: 'active'
      });
    }
    
    return issues;
  }

  /**
   * Check system integrity
   */
  private async checkSystemIntegrity(): Promise<HealthIssue[]> {
    const issues: HealthIssue[] = [];
    
    // Check if Ollama is running
    try {
      const response = await fetch('http://localhost:1536/api/tags');
      if (!response.ok) {
        issues.push({
          id: `sys_${Date.now()}`,
          severity: 'critical',
          type: 'corruption',
          location: 'ollama_service',
          description: 'Ollama service not responding',
          detectedAt: Date.now(),
          status: 'active'
        });
      }
    } catch (error) {
      issues.push({
        id: `sys_${Date.now()}`,
        severity: 'critical',
        type: 'corruption',
        location: 'ollama_service',
        description: 'Cannot connect to Ollama',
        detectedAt: Date.now(),
        status: 'active'
      });
    }
    
    return issues;
  }

  /**
   * Check knowledge consistency
   */
  private async checkKnowledgeConsistency(): Promise<HealthIssue[]> {
    // Placeholder for knowledge consistency checks
    return [];
  }

  /**
   * Calculate health scores
   */
  private calculateHealthScores(): void {
    const issues = this.state.activeIssues;
    
    const critical = issues.filter(i => i.severity === 'critical').length;
    const serious = issues.filter(i => i.severity === 'serious').length;
    const moderate = issues.filter(i => i.severity === 'moderate').length;
    const minor = issues.filter(i => i.severity === 'minor').length;
    
    // Calculate code health
    this.state.codeHealth = Math.max(0, 
      100 - (critical * 20) - (serious * 10) - (moderate * 5) - (minor * 1)
    );
    
    // Overall health
    this.state.overall = this.state.codeHealth;
  }

  /**
   * Auto-heal critical issues
   */
  private async autoHealCritical(): Promise<void> {
    const criticalIssues = this.state.activeIssues.filter(
      i => i.severity === 'critical' || i.severity === 'serious'
    );
    
    for (const issue of criticalIssues) {
      if (issue.status !== 'active') continue;
      
      console.log(`[SelfHealing] 🚨 Healing critical: ${issue.description}`);
      
      issue.status = 'healing';
      
      try {
        const fix = await this.generateFix(issue);
        
        if (fix) {
          await this.applyFix(issue, fix);
          issue.status = 'healed';
          issue.healedAt = Date.now();
          issue.fix = fix;
          
          console.log(`[SelfHealing] ✅ Healed: ${issue.description}`);
        } else {
          issue.status = 'unfixable';
          console.log(`[SelfHealing] ⚠️ Unfixable: ${issue.description}`);
        }
      } catch (error) {
        issue.status = 'active';
        console.error(`[SelfHealing] ❌ Heal failed: ${issue.description}`, error);
      }
    }
  }

  /**
   * Generate fix for an issue
   */
  private async generateFix(issue: HealthIssue): Promise<string | null> {
    const prompt = `
Generate a fix for this issue:

Type: ${issue.type}
Location: ${issue.location}
Description: ${issue.description}

Provide:
1. Root cause
2. Exact fix (code if applicable)
3. Steps to apply

Respond with:
CAUSE: [root cause]
FIX: [the fix]
`;

    try {
      const response = await this.ollama.generate({
        model: this.MODEL,
        prompt,
        stream: false
      });
      
      const fixMatch = response.response.match(/FIX:\s*(.+)/is);
      return fixMatch ? fixMatch[1].trim() : null;
    } catch (error) {
      return null;
    }
  }

  /**
   * Apply a fix
   */
  private async applyFix(issue: HealthIssue, fix: string): Promise<void> {
    // Parse location to get file
    const [filePath, lineNum] = issue.location.split(':');
    
    if (issue.type === 'security_vulnerability' && fix.includes('Remove')) {
      // For security issues, remove the problematic code
      const content = await fs.readFile(filePath, 'utf-8');
      const fixed = content.replace(/(password|api[_-]?key)\s*=\s*["'][^"']+["']/gi, '$1 = process.env.$1');
      await fs.writeFile(filePath, fixed, 'utf-8');
    }
    
    console.log(`[SelfHealing] 🔧 Applied fix to ${filePath}`);
  }

  /**
   * Manual heal request
   */
  async heal(issueId: string): Promise<boolean> {
    const issue = this.state.activeIssues.find(i => i.id === issueId);
    
    if (!issue) return false;
    
    issue.status = 'healing';
    
    const fix = await this.generateFix(issue);
    
    if (fix) {
      await this.applyFix(issue, fix);
      issue.status = 'healed';
      issue.healedAt = Date.now();
      return true;
    }
    
    issue.status = 'unfixable';
    return false;
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
    const { overall, codeHealth, activeIssues } = this.state;
    
    return `
🏥 Health Report:
  ❤️  Overall: ${overall}%
  💻 Code: ${codeHealth}%
  🐛 Active Issues: ${activeIssues.length}
    - Critical: ${activeIssues.filter(i => i.severity === 'critical').length}
    - Serious: ${activeIssues.filter(i => i.severity === 'serious').length}
    - Moderate: ${activeIssues.filter(i => i.severity === 'moderate').length}
    - Minor: ${activeIssues.filter(i => i.severity === 'minor').length}
`.trim();
  }

  /**
   * Stop healing system
   */
  stop(): void {
    if (this.healInterval) {
      clearInterval(this.healInterval);
    }
    console.log('[SelfHealing] ⏸️ Healing paused');
  }
}

// Export factory (needs workspace path)
export function createSelfHealing(workspacePath: string): AIRISelfHealing {
  return new AIRISelfHealing(workspacePath);
}
