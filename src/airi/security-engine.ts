/**
 * AIRI Cybersecurity Engine
 * Red Team / Blue Team / Purple Team operations
 */

import { hadesOllama } from '../hades-ollama-service';
import { getModel } from './model-config';
import { SecurityMode } from './types';

export type { SecurityMode };

export interface SecurityReport {
  timestamp: number;
  mode: SecurityMode;
  findings: SecurityFinding[];
  summary: string;
}

export interface SecurityFinding {
  id: string;
  severity: 'critical' | 'high' | 'medium' | 'low' | 'info';
  type: string;
  description: string;
  location?: string;
  evidence?: string;
  recommendation: string;
  cwe?: string;
  cve?: string;
}

export class AIRISecurityEngine {
  private mode: SecurityMode = 'passive';
  private running = false;
  private readonly MODEL_ROLE = 'security';

  constructor() { }

  start(): void {
    if (this.running) return;
    this.running = true;
  }

  stop(): void {
    this.running = false;
  }

  isRunning(): boolean {
    return this.running;
  }

  setMode(mode: SecurityMode): void {
    this.mode = mode;
  }

  async scanForVulnerabilities(target: {
    code?: string;
    url?: string;
    filePath?: string;
  }): Promise<SecurityReport> {
    if (this.mode !== 'red' && this.mode !== 'purple') {
      throw new Error('Red team mode required for vulnerability scanning');
    }

    const prompt = `Analyze for vulnerabilities: ${target.url || target.filePath || 'code'}\n\n${target.code || ''}`;

    try {
      const response = await hadesOllama.generate(prompt, {
        model: getModel(this.MODEL_ROLE),
        system: this.getSecuritySystemPrompt(),
        stream: false
      });

      const findings = this.parseFindings(response.response || '');

      return {
        timestamp: Date.now(),
        mode: this.mode,
        findings,
        summary: `Vulnerability scan found ${findings.length} issues.`
      };
    } catch (error) {
      // console.error('[Security] Scan failed:', error);
      throw error;
    }
  }

  async monitorForThreats(logs: string[]): Promise<SecurityReport> {
    if (this.mode !== 'blue' && this.mode !== 'purple') {
      throw new Error('Blue team mode required for threat monitoring');
    }

    const prompt = `Analyze logs for threats: ${logs.join('\n')}`;

    try {
      const response = await hadesOllama.generate(prompt, {
        model: getModel(this.MODEL_ROLE),
        system: this.getSecuritySystemPrompt(),
        stream: false
      });

      const findings = this.parseFindings(response.response || '');

      return {
        timestamp: Date.now(),
        mode: this.mode,
        findings,
        summary: `Found ${findings.length} potential threats`
      };
    } catch (error) {
      throw error;
    }
  }

  async checkCodeSecurity(code: string, language: string): Promise<SecurityReport> {
    const prompt = `Analyze ${language} code for security: ${code}`;

    try {
      const response = await hadesOllama.generate(prompt, {
        model: getModel(this.MODEL_ROLE),
        system: this.getSecuritySystemPrompt(),
        stream: false
      });

      const findings = this.parseFindings(response.response || '');

      return {
        timestamp: Date.now(),
        mode: 'passive',
        findings,
        summary: `Found ${findings.length} security issues in code`
      };
    } catch (error) {
      throw error;
    }
  }

  private getSecuritySystemPrompt(): string {
    return "You are an expert cybersecurity analyst.";
  }

  private parseFindings(response: string): SecurityFinding[] {
    const findings: SecurityFinding[] = [];
    // very simple parsing for now
    if (response.toLowerCase().includes('vulnerability') || response.toLowerCase().includes('risk')) {
      findings.push({
        id: `vuln_${Date.now()}`,
        severity: 'high',
        type: 'Generic Vulnerability',
        description: response.substring(0, 200),
        recommendation: 'Review the flagged code immediately.'
      });
    }
    return findings;
  }
}

export const airiSecurity = new AIRISecurityEngine();
