/**
 * AIRI Autonomous Work System
 * Self-directed task generation and execution
 * Works 24/7, even while you sleep
 */

import type { Ollama } from 'ollama';
import { createSharedOllama } from './shared-ollama';
import * as fs from 'fs/promises';
import * as path from 'path';
import { invoke } from '../tauri_bridge';
import { getToolSchemas } from '../tool_registry';
import { getModel } from './model-config';

export interface AutonomousTask {
  id: string;
  type: TaskType;
  description: string;
  priority: number;
  status: 'pending' | 'in_progress' | 'completed' | 'failed';
  createdAt: number;
  completedAt?: number;
  result?: string;
}

export type TaskType =
  | 'debug'
  | 'implement'
  | 'refactor'
  | 'test'
  | 'document'
  | 'review'
  | 'optimize'
  | 'fix_security'
  | 'clean_code';

export interface WorkspaceState {
  errors: WorkspaceError[];
  todos: TodoItem[];
  recentChanges: FileChange[];
  codeSmells: CodeSmell[];
}

export interface WorkspaceError {
  file: string;
  line: number;
  message: string;
  type: 'syntax' | 'type' | 'runtime' | 'lint';
}

export interface TodoItem {
  file: string;
  line: number;
  text: string;
}

export interface FileChange {
  file: string;
  timestamp: number;
  type: 'create' | 'modify' | 'delete';
}

export interface CodeSmell {
  file: string;
  type: string;
  description: string;
  suggestion: string;
}

export class AIRIAutonomousAgent {
  private ollama: Ollama;
  private workspacePath: string;
  private scanInterval: NodeJS.Timeout | null = null;
  private taskQueue: AutonomousTask[] = [];
  private isWorking: boolean = false;
  private readonly MODEL = 'qwen3.6:14b-q4_K_M';

  constructor(workspacePath: string) {
    this.ollama = createSharedOllama();
    this.workspacePath = workspacePath;

  }

  /**
   * Start autonomous operation
   */
  start(scanIntervalMs: number = 60000): void {

    // Scan workspace periodically
    this.scanInterval = setInterval(() => {
      this.scanAndGenerateTasks();
    }, scanIntervalMs);

    // Start working on tasks
    this.workLoop();
  }

  /**
   * Stop autonomous operation
   */
  stop(): void {
    if (this.scanInterval) {
      clearInterval(this.scanInterval);
      this.scanInterval = null;
    }
  }

  /**
   * Scan workspace and generate tasks
   */
  private async scanAndGenerateTasks(): Promise<void> {
    if (this.isWorking) return; // Don't generate while working


    try {
      const state = await this.analyzeWorkspace();

      // Generate tasks from errors
      for (const error of state.errors) {
        await this.generateTaskFromError(error);
      }

      // Generate tasks from TODOs
      for (const todo of state.todos) {
        await this.generateTaskFromTodo(todo);
      }

      // Generate tasks from code smells
      for (const smell of state.codeSmells) {
        await this.generateTaskFromCodeSmell(smell);
      }

    } catch (error) {
      console.error('[AutonomousAgent] Scan failed:', error);
    }
  }

  /**
   * Analyze workspace state
   */
  private async analyzeWorkspace(): Promise<WorkspaceState> {
    const state: WorkspaceState = {
      errors: [],
      todos: [],
      recentChanges: [],
      codeSmells: []
    };

    // Find all TypeScript/JavaScript/Rust files
    const files = await this.findCodeFiles();

    for (const file of files.slice(0, 50)) { // Limit to 50 files per scan
      try {
        const content = await fs.readFile(file, 'utf-8');

        // Find errors (compile errors would be reported by IDE)
        // For now, look for TODO comments and code smells

        // Extract TODOs
        const todoMatches = content.matchAll(/\/\/\s*TODO[:\s]+(.+)/g);
        for (const match of todoMatches) {
          state.todos.push({
            file,
            line: 0, // Would need line number tracking
            text: match[1]
          });
        }

        // Detect code smells
        const smells = this.detectCodeSmells(content, file);
        state.codeSmells.push(...smells);

      } catch (error) {
        // File read error, skip
      }
    }

    return state;
  }

  /**
   * Find all code files in workspace
   */
  private async findCodeFiles(): Promise<string[]> {
    const extensions = ['.ts', '.tsx', '.js', '.jsx', '.rs', '.py', '.json'];
    const files: string[] = [];

    async function walk(dir: string): Promise<void> {
      try {
        const entries = await fs.readdir(dir, { withFileTypes: true });

        for (const entry of entries) {
          if (entry.name.startsWith('.') || entry.name === 'node_modules' || entry.name === 'target') {
            continue;
          }

          const fullPath = path.join(dir, entry.name);

          if (entry.isDirectory()) {
            await walk(fullPath);
          } else if (entry.isFile() && extensions.some(ext => entry.name.endsWith(ext))) {
            files.push(fullPath);
          }
        }
      } catch (error) {
        // Ignore inaccessible directories
      }
    }

    await walk(this.workspacePath);
    return files;
  }

  /**
   * Detect code smells
   */
  private detectCodeSmells(content: string, file: string): CodeSmell[] {
    const smells: CodeSmell[] = [];

    // Long functions (>50 lines)
    const lines = content.split('\n');
    let inFunction = false;
    let functionStart = 0;
    let functionLines = 0;

    for (let i = 0; i < lines.length; i++) {
      const line = lines[i];

      if (line.match(/function\s+\w+\s*\(|const\s+\w+\s*=\s*\(|=>\s*{/)) {
        inFunction = true;
        functionStart = i;
        functionLines = 0;
      }

      if (inFunction) {
        functionLines++;

        if (functionLines > 50 && line.includes('}')) {
          smells.push({
            file,
            type: 'long_function',
            description: `Function at line ${functionStart + 1} is ${functionLines} lines long`,
            suggestion: 'Consider breaking into smaller functions'
          });
          inFunction = false;
        }
      }
    }

    // TODO comments
    if (content.includes('TODO') || content.includes('FIXME')) {
      smells.push({
        file,
        type: 'todo_comment',
        description: 'Contains TODO/FIXME comments',
        suggestion: 'Address technical debt'
      });
    }

    const consoleLogs = (content.match(/console\.log/g) || []).length;
    if (consoleLogs > 5) {
      smells.push({
        file,
        type: 'debug_statements',
        description: 'Excessive console.log statements',
        suggestion: 'Remove debug statements before production'
      });
    }

    // any type usage
    if (content.match(/:\s*any\b/)) {
      smells.push({
        file,
        type: 'any_type',
        description: 'Uses "any" type',
        suggestion: 'Use proper TypeScript types'
      });
    }

    return smells;
  }

  /**
   * Generate task from error
   */
  private async generateTaskFromError(error: WorkspaceError): Promise<void> {
    const task: AutonomousTask = {
      id: `task_${Date.now()}_${Math.random()}`,
      type: 'debug',
      description: `Fix error in ${error.file}: ${error.message}`,
      priority: 10, // High priority
      status: 'pending',
      createdAt: Date.now()
    };

    this.taskQueue.push(task);
  }

  /**
   * Generate task from TODO
   */
  private async generateTaskFromTodo(todo: TodoItem): Promise<void> {
    const task: AutonomousTask = {
      id: `task_${Date.now()}_${Math.random()}`,
      type: 'implement',
      description: `Implement TODO in ${todo.file}: ${todo.text}`,
      priority: 5,
      status: 'pending',
      createdAt: Date.now()
    };

    this.taskQueue.push(task);
  }

  /**
   * Generate task from code smell
   */
  private async generateTaskFromCodeSmell(smell: CodeSmell): Promise<void> {
    const task: AutonomousTask = {
      id: `task_${Date.now()}_${Math.random()}`,
      type: 'refactor',
      description: `Fix code smell in ${smell.file}: ${smell.type}`,
      priority: 3,
      status: 'pending',
      createdAt: Date.now()
    };

    this.taskQueue.push(task);
  }

  /**
   * Work loop - process tasks
   */
  private async workLoop(): Promise<void> {
    while (true) {
      await this.processNextTask();

      // Wait before next task
      await new Promise(resolve => setTimeout(resolve, 2000));
    }
  }

  /**
   * Process next task in queue
   */
  private async processNextTask(): Promise<void> {
    if (this.taskQueue.length === 0 || this.isWorking) {
      return;
    }

    // Sort by priority
    this.taskQueue.sort((a, b) => b.priority - a.priority);

    const task = this.taskQueue.shift();
    if (!task) return;

    this.isWorking = true;
    task.status = 'in_progress';


    try {
      const result = await this.executeTask(task);
      task.status = 'completed';
      task.result = result;
      task.completedAt = Date.now();

    } catch (error) {
      task.status = 'failed';
      task.result = String(error);
      console.error(`[AutonomousAgent] ❌ Task failed: ${task.description}`, error);
    } finally {
      this.isWorking = false;
    }
  }

  /**
   * Execute a task
   * Instead of generating passive text blocks, we push the task into the Rust ai_chat engine.
   * This gives the background loop full access to write_file, run_command, and other tools,
   * making it truly autonomous.
   */
  private async executeTask(task: AutonomousTask): Promise<string> {
    const prompt = `BACKGROUND AUTONOMOUS MISSION: [${task.type.toUpperCase()}]\n\n${task.description}\n\nYou are operating in Sentient background mode. You must use your tools (write_to_file, run_command, etc.) to complete this mission. Do not simply explain what to do — ACTUALLY do it. When you are finished, declare MISSION_ACCOMPLISHED.`;

    console.log(`[AutonomousAgent] 🤖 Dispatching task to Sentient Core:`, task.description);

    try {
      const store = (window as any).useStore?.getState();
      const ollamaUrl = store?.ollamaUrl || 'http://127.0.0.1:11434';

      const fullCall = invoke<string>("ai_chat", {
        request: {
          provider: "ollama",
          model: this.MODEL,
          messages: [{
            role: "system",
            content: "You are the AIRI background autonomous agent. Your job is to proactively improve, refactor, and fix the codebase without user intervention. You have full access to tools.",
            tool_calls: null,
            metadata: null
          }, {
            role: "user",
            content: prompt,
            tool_calls: null,
            metadata: null
          }],
          temperature: 0.7,
          autonomous: true,
          root_access: true,
          mode: "Sentient",
          ollama_url: ollamaUrl,
          tools: getToolSchemas(),
        }
      });

      // Let it run for up to 10 minutes (background tasks are allowed to take longer)
      const fullTimeout = new Promise<string>((_, reject) =>
        setTimeout(() => reject(new Error("ai_chat timed out after 600s")), 600_000)
      );

      const result = await Promise.race([fullCall, fullTimeout]);
      return `Mission Accomplished in background loop.\n\nSummary:\n${(result || "").slice(0, 500)}`;
    } catch (err: any) {
      console.error("[AutonomousAgent] ❌ Execution failed:", err);
      throw err;
    }
  }

  /**
   * Get task queue
   */
  getTasks(): AutonomousTask[] {
    return [...this.taskQueue];
  }

  /**
   * Get work status
   */
  getStatus(): string {
    return `
💼 Autonomous Work Status:
  Working: ${this.isWorking}
  Queue: ${this.taskQueue.length} tasks
  Model: ${this.MODEL}
`.trim();
  }
}

// Export factory function (need workspace path)
export function createAutonomousAgent(workspacePath: string): AIRIAutonomousAgent {
  return new AIRIAutonomousAgent(workspacePath);
}
