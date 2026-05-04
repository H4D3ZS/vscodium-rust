/**
 * AIRI Autonomous Agent
 * Proactive task execution, background scans, and self-directed work
 */

import { hadesOllama } from '../hades-ollama-service';
import { getModel } from './model-config';
import { airiConsciousness } from './consciousness';
import { airiBiology } from './biology';
import { airiSecurity } from './security-engine';
import { airiSelfLearning } from './self-learning';
import { invoke } from '../tauri_bridge';

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

export interface AgentState {
  isActive: boolean;
  currentTask?: AutonomousTask;
  tasks: AutonomousTask[];
  findings: string[];
  lastScan: number;
}

export class AIRIAutonomousAgent {
  private workspacePath: string;
  private scanInterval: any | null = null;
  private taskQueue: AutonomousTask[] = [];
  private state: AgentState;
  private isWorking: boolean = false;
  private readonly MODEL_ROLE = 'consciousness';

  constructor(workspacePath: string) {
    this.workspacePath = workspacePath;
    this.state = {
      isActive: false,
      tasks: [],
      findings: [],
      lastScan: Date.now()
    };
  }

  /**
   * Start autonomous operation
   */
  start(scanIntervalMs: number = 60000): void {
    if (this.state.isActive) return;
    this.state.isActive = true;

    // Scan workspace periodically
    this.scanInterval = setInterval(() => {
      this.scanAndGenerateTasks().catch(() => { });
    }, scanIntervalMs);

    // Start working on tasks
    this.workLoop().catch(() => { });
  }

  /**
   * Stop autonomous operation
   */
  stop(): void {
    if (this.scanInterval) {
      clearInterval(this.scanInterval);
      this.scanInterval = null;
    }
    this.state.isActive = false;
  }

  /**
   * Scan workspace and generate tasks
   */
  private async scanAndGenerateTasks(): Promise<void> {
    if (this.isWorking) return;

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
      // console.error('[AutonomousAgent] Scan failed:', error);
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

    try {
      const files = await this.findCodeFiles();

      for (const file of files.slice(0, 20)) { // Reduced limit for performance
        try {
          const content = await invoke<string>('read_file', { path: file });

          // Extract TODOs
          const todoMatches = content.matchAll(/\/\/\s*TODO[:\s]+(.+)/g);
          for (const match of todoMatches) {
            state.todos.push({
              file,
              line: 0,
              text: match[1]
            });
          }

          // Detect code smells
          const smells = this.detectCodeSmells(content, file);
          state.codeSmells.push(...smells);

        } catch (error) {
          // skip
        }
      }
    } catch {
      // ignore
    }

    return state;
  }

  /**
   * Find all code files in workspace
   */
  private async findCodeFiles(): Promise<string[]> {
    try {
      return await invoke<string[]>('list_directory', { path: this.workspacePath });
    } catch {
      return [];
    }
  }

  /**
   * Detect code smells
   */
  private detectCodeSmells(content: string, file: string): CodeSmell[] {
    const smells: CodeSmell[] = [];

    // Long functions (>100 lines)
    const lines = content.split('\n');
    if (lines.length > 500) {
      smells.push({
        file,
        type: 'massive_file',
        description: `File has ${lines.length} lines.`,
        suggestion: 'Consider refactoring into smaller modules'
      });
    }

    if (content.includes('TODO') || content.includes('FIXME')) {
      smells.push({
        file,
        type: 'todo_comment',
        description: 'Contains TODO/FIXME comments',
        suggestion: 'Address technical debt'
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
      priority: 10,
      status: 'pending',
      createdAt: Date.now()
    };

    if (!this.taskQueue.some(t => t.description === task.description)) {
      this.taskQueue.push(task);
    }
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

    if (!this.taskQueue.some(t => t.description === task.description)) {
      this.taskQueue.push(task);
    }
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

    if (!this.taskQueue.some(t => t.description === task.description)) {
      this.taskQueue.push(task);
    }
  }

  /**
   * Work loop - process tasks
   */
  private async workLoop(): Promise<void> {
    while (this.state.isActive) {
      await this.processNextTask();
      // Reduced task frequency to save resources (5 minutes between tasks)
      await new Promise(resolve => setTimeout(resolve, 300000));
    }
  }

  /**
   * Process next task in queue
   */
  private async processNextTask(): Promise<void> {
    if (this.taskQueue.length === 0 || this.isWorking) {
      return;
    }

    this.taskQueue.sort((a, b) => b.priority - a.priority);

    const task = this.taskQueue.shift();
    if (!task) return;

    this.isWorking = true;
    task.status = 'in_progress';
    this.state.currentTask = task;

    try {
      const result = await this.executeTask(task);
      task.status = 'completed';
      task.result = result;
      task.completedAt = Date.now();

      airiSelfLearning.learnFromEvent('success', `Completed autonomous task: ${task.description}`, 'success').catch(() => { });

    } catch (error) {
      task.status = 'failed';
      task.result = String(error);
    } finally {
      this.isWorking = false;
      this.state.currentTask = undefined;
    }
  }

  /**
   * Execute a task
   */
  private async executeTask(task: AutonomousTask): Promise<string> {
    const prompt = `Task: ${task.type}\nFile/Detail: ${task.description}. Provide a summary fix approach.`;

    const response = await hadesOllama.generate(prompt, {
      model: getModel(this.MODEL_ROLE),
      stream: false
    });

    return (response.response || '').trim();
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
  Model: ${getModel(this.MODEL_ROLE)}
`.trim();
  }
}

export function createAutonomousAgent(workspacePath: string): AIRIAutonomousAgent {
  return new AIRIAutonomousAgent(workspacePath);
}

export const airiAutonomousAgent = new AIRIAutonomousAgent('c:/Users/HADES/Desktop/vscodium-rust');
