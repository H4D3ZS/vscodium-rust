/**
 * AIRI Development Assistant
 * Full-featured coding partner - writes, debugs, refactors, tests code
 * Works autonomously 24/7 on your codebase
 */

import { Ollama } from 'ollama';
import * as fs from 'fs/promises';
import * as path from 'path';

export interface DevelopmentTask {
  id: string;
  type: TaskType;
  description: string;
  file?: string;
  code?: string;
  status: 'pending' | 'in_progress' | 'completed' | 'failed';
  result?: string;
  timestamp: number;
}

export type TaskType =
  | 'write_code'
  | 'fix_bug'
  | 'refactor'
  | 'write_test'
  | 'add_feature'
  | 'debug'
  | 'review'
  | 'document'
  | 'optimize'
  | 'migrate';

export class AIRIDevelopmentAssistant {
  private ollama: Ollama;
  private taskQueue: DevelopmentTask[];
  private activeTasks: DevelopmentTask[];
  private completedTasks: DevelopmentTask[];
  private readonly MODEL = 'qwen3.6:14b-q4_K_M';
  private workspacePath: string;

  constructor(workspacePath: string) {
    this.ollama = new Ollama({ host: 'http://localhost:1536' }); // AIM proxy
    this.workspacePath = workspacePath;
    this.taskQueue = [];
    this.activeTasks = [];
    this.completedTasks = [];

  }

  /**
   * Write new code
   */
  async writeCode(
    description: string,
    language: string,
    requirements: string[] = []
  ): Promise<string> {

    const prompt = `
Write ${language} code for: ${description}

Requirements:
${requirements.join('\n')}

Provide:
1. Complete, working code
2. Proper error handling
3. Comments explaining key parts
4. Usage example

Write the full implementation.
`;

    try {
      const response = await this.ollama.generate({
        model: this.MODEL,
        prompt,
        stream: false
      });

      const code = this.extractCode(response.response);
      
      return code;
    } catch (error) {
      console.error('[DevAssistant] Code generation failed:', error);
      throw error;
    }
  }

  /**
   * Fix a bug
   */
  async fixBug(
    code: string,
    errorMessage: string,
    expectedBehavior: string,
    actualBehavior: string
  ): Promise<{ fixedCode: string; explanation: string }> {

    const prompt = `
Fix the bug in this code:

${code}

Error: ${errorMessage}
Expected: ${expectedBehavior}
Actual: ${actualBehavior}

Provide:
1. Root cause analysis
2. Fixed code
3. Explanation of the fix

Format:
EXPLANATION: [root cause and fix]
CODE: [fixed code]
`;

    try {
      const response = await this.ollama.generate({
        model: this.MODEL,
        prompt,
        stream: false
      });

      const explanation = response.response.match(/EXPLANATION:\s*([\s\S]*?)(?=CODE:|$)/i)?.[1]?.trim() || '';
      const fixedCode = this.extractCode(response.response);

      
      return { fixedCode, explanation };
    } catch (error) {
      console.error('[DevAssistant] Bug fix failed:', error);
      throw error;
    }
  }

  /**
   * Refactor code
   */
  async refactor(
    code: string,
    goals: string[] = ['improve readability', 'reduce complexity']
  ): Promise<string> {

    const prompt = `
Refactor this code:

${code}

Goals:
${goals.join('\n')}

Consider:
- Reduce complexity
- Improve readability
- Follow best practices
- Maintain exact same behavior
- Better naming
- Extract functions if needed

Provide the refactored code.
`;

    try {
      const response = await this.ollama.generate({
        model: this.MODEL,
        prompt,
        stream: false
      });

      const refactored = this.extractCode(response.response);
      
      return refactored;
    } catch (error) {
      console.error('[DevAssistant] Refactoring failed:', error);
      throw error;
    }
  }

  /**
   * Write tests
   */
  async writeTests(
    code: string,
    language: string,
    framework: string = 'jest'
  ): Promise<string> {

    const prompt = `
Write comprehensive tests for this ${language} code:

${code}

Testing framework: ${framework}

Include:
1. Unit tests for all functions
2. Edge cases
3. Error cases
4. Happy path tests
5. Integration tests if applicable

Write complete, runnable tests.
`;

    try {
      const response = await this.ollama.generate({
        model: this.MODEL,
        prompt,
        stream: false
      });

      const tests = this.extractCode(response.response);
      
      return tests;
    } catch (error) {
      console.error('[DevAssistant] Test generation failed:', error);
      throw error;
    }
  }

  /**
   * Add a new feature
   */
  async addFeature(
    existingCode: string,
    featureDescription: string,
    language: string
  ): Promise<string> {

    const prompt = `
Add this feature to the existing code:

Existing code:
${existingCode}

New feature:
${featureDescription}

Language: ${language}

Provide:
1. Modified code with the new feature integrated
2. Comments showing what was added
3. Any new dependencies needed

Write the complete updated code.
`;

    try {
      const response = await this.ollama.generate({
        model: this.MODEL,
        prompt,
        stream: false
      });

      const updatedCode = this.extractCode(response.response);
      
      return updatedCode;
    } catch (error) {
      console.error('[DevAssistant] Feature addition failed:', error);
      throw error;
    }
  }

  /**
   * Debug step-by-step
   */
  async debug(
    code: string,
    issue: string,
    language: string
  ): Promise<{
    analysis: string;
    steps: string[];
    solution: string;
  }> {

    const prompt = `
Debug this ${language} code:

${code}

Issue: ${issue}

Provide:
1. Root cause analysis
2. Step-by-step debugging approach
3. Solution with code

Format:
ANALYSIS: [root cause]
STEPS: [numbered list of debugging steps]
SOLUTION: [code solution]
`;

    try {
      const response = await this.ollama.generate({
        model: this.MODEL,
        prompt,
        stream: false
      });

      const analysis = response.response.match(/ANALYSIS:\s*([\s\S]*?)(?=STEPS:|$)/i)?.[1]?.trim() || '';
      const stepsMatch = response.response.match(/STEPS:\s*([\s\S]*?)(?=SOLUTION:|$)/i);
      const steps = stepsMatch ? stepsMatch[1].split('\n').filter(s => s.match(/^\d+\./)).map(s => s.replace(/^\d+\.\s*/, '')) : [];
      const solution = this.extractCode(response.response);

      
      return { analysis, steps, solution };
    } catch (error) {
      console.error('[DevAssistant] Debugging failed:', error);
      throw error;
    }
  }

  /**
   * Code review
   */
  async reviewCode(code: string, language: string): Promise<{
    score: number;
    issues: string[];
    suggestions: string[];
    bestPractices: string[];
  }> {

    const prompt = `
Review this ${language} code:

${code}

Evaluate:
1. Code quality (0-100)
2. Issues and bugs
3. Suggestions for improvement
4. Best practices violations

Format:
SCORE: [0-100]
ISSUES: [numbered list]
SUGGESTIONS: [numbered list]
BEST_PRACTICES: [numbered list]
`;

    try {
      const response = await this.ollama.generate({
        model: this.MODEL,
        prompt,
        stream: false
      });

      const scoreMatch = response.response.match(/SCORE:\s*(\d+)/i);
      const score = parseInt(scoreMatch?.[1] || '50');

      const extractList = (section: string) => {
        const match = response.response.match(new RegExp(`${section}:\\s*([\\s\\S]*?)(?=(SCORE:|ISSUES:|SUGGESTIONS:|BEST_PRACTICES:|$)`, 'i'));
        if (match) {
          return match[1].split('\n')
            .filter(line => line.match(/^\d+[\.\)]\s*/))
            .map(line => line.replace(/^\d+[\.\)]\s*/, ''));
        }
        return [];
      };

      const issues = extractList('ISSUES');
      const suggestions = extractList('SUGGESTIONS');
      const bestPractices = extractList('BEST_PRACTICES');

      
      return { score, issues, suggestions, bestPractices };
    } catch (error) {
      console.error('[DevAssistant] Code review failed:', error);
      throw error;
    }
  }

  /**
   * Write documentation
   */
  async writeDocumentation(code: string, type: 'readme' | 'api' | 'comments' | 'tutorial'): Promise<string> {

    const prompt = `
Write ${type} documentation for this code:

${code}

For README: Include installation, usage, examples
For API: Include function signatures, parameters, return values
For COMMENTS: Add inline comments explaining logic
For TUTORIAL: Step-by-step guide with examples

Write comprehensive, clear documentation.
`;

    try {
      const response = await this.ollama.generate({
        model: this.MODEL,
        prompt,
        stream: false
      });

      
      return response.response;
    } catch (error) {
      console.error('[DevAssistant] Documentation failed:', error);
      throw error;
    }
  }

  /**
   * Optimize performance
   */
  async optimizePerformance(code: string, language: string): Promise<{
    optimizedCode: string;
    improvements: string[];
    estimatedGain: string;
  }> {

    const prompt = `
Optimize this ${language} code for performance:

${code}

Consider:
- Algorithm efficiency
- Memory usage
- Caching opportunities
- Parallel processing
- Built-in optimizations

Provide:
1. Optimized code
2. List of improvements made
3. Estimated performance gain

Format:
CODE: [optimized code]
IMPROVEMENTS: [numbered list]
ESTIMATED_GAIN: [percentage or description]
`;

    try {
      const response = await this.ollama.generate({
        model: this.MODEL,
        prompt,
        stream: false
      });

      const optimizedCode = this.extractCode(response.response);
      const improvementsMatch = response.response.match(/IMPROVEMENTS:\s*([\s\S]*?)(?=ESTIMATED_GAIN:|$)/i);
      const improvements = improvementsMatch ? improvementsMatch[1].split('\n').filter(l => l.match(/^\d+\./)).map(l => l.replace(/^\d+\.\s*/, '')) : [];
      const estimatedGain = response.response.match(/ESTIMATED_GAIN:\s*(.+)/i)?.[1]?.trim() || 'Unknown';

      
      return { optimizedCode, improvements, estimatedGain };
    } catch (error) {
      console.error('[DevAssistant] Optimization failed:', error);
      throw error;
    }
  }

  /**
   * Extract code from AI response
   */
  private extractCode(response: string): string {
    // Try to extract from code blocks
    const codeBlockMatch = response.match(/```[\w]*\n([\s\S]*?)```/);
    if (codeBlockMatch) {
      return codeBlockMatch[1].trim();
    }
    
    // Return response if no code blocks
    return response.trim();
  }

  /**
   * Save code to file
   */
  async saveToFile(filePath: string, code: string): Promise<void> {
    const fullPath = path.join(this.workspacePath, filePath);
    await fs.mkdir(path.dirname(fullPath), { recursive: true });
    await fs.writeFile(fullPath, code, 'utf-8');
  }

  /**
   * Read file
   */
  async readFile(filePath: string): Promise<string> {
    const fullPath = path.join(this.workspacePath, filePath);
    return fs.readFile(fullPath, 'utf-8');
  }

  /**
   * Get task statistics
   */
  getStats(): {
    queued: number;
    active: number;
    completed: number;
    byType: Record<string, number>;
  } {
    const byType: Record<string, number> = {};
    
    [...this.taskQueue, ...this.activeTasks, ...this.completedTasks].forEach(task => {
      byType[task.type] = (byType[task.type] || 0) + 1;
    });

    return {
      queued: this.taskQueue.length,
      active: this.activeTasks.length,
      completed: this.completedTasks.length,
      byType
    };
  }
}

// Export factory
export function createAIRIDevelopmentAssistant(workspacePath: string): AIRIDevelopmentAssistant {
  return new AIRIDevelopmentAssistant(workspacePath);
}
