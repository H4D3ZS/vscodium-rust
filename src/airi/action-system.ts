/**
 * AIRI Action System
 * AIRI's "motor control" - ways to interact with and change the world
 * File operations, system control, network actions, API calls, etc.
 */

import * as fs from 'fs/promises';
import * as path from 'path';
import { exec } from 'child_process';
import { promisify } from 'util';

const execAsync = promisify(exec);

export interface Action {
  id: string;
  type: ActionType;
  description: string;
  status: 'pending' | 'executing' | 'completed' | 'failed';
  timestamp: number;
  result?: any;
  error?: string;
}

export type ActionType =
  | 'file_read'
  | 'file_write'
  | 'file_delete'
  | 'file_create'
  | 'directory_create'
  | 'command_execute'
  | 'api_call'
  | 'network_request'
  | 'git_operation'
  | 'process_spawn'
  | 'system_control';

export class AIRIActionSystem {
  private actionHistory: Action[];
  private allowedPaths: string[];
  private blockedCommands: string[];
  private maxActionsPerMinute: number;
  private actionCount: number;
  private resetTimer: NodeJS.Timeout | null;

  constructor(allowedPaths: string[] = [process.cwd()]) {
    this.actionHistory = [];
    this.allowedPaths = allowedPaths;
    this.blockedCommands = [
      'rm -rf /',
      'format c:',
      'del /f /q *',
      'shutdown -s -t 0'
    ];
    this.maxActionsPerMinute = 100;
    this.actionCount = 0;
    this.resetTimer = null;

    this.startActionCounter();

    console.log('[Action] ✋ AIRI Action System initialized');
    console.log('[Action] 🎯 Can interact with filesystem, system, network');
  }

  /**
   * Start action counter (rate limiting)
   */
  private startActionCounter(): void {
    this.resetTimer = setInterval(() => {
      this.actionCount = 0;
    }, 60000); // Reset every minute
  }

  /**
   * Execute action with safety checks
   */
  async execute(action: Action): Promise<any> {
    // Rate limiting
    if (this.actionCount >= this.maxActionsPerMinute) {
      throw new Error('Action rate limit exceeded');
    }

    // Safety checks
    if (!this.isActionSafe(action)) {
      throw new Error('Action blocked for safety');
    }

    action.status = 'executing';
    action.timestamp = Date.now();
    this.actionCount++;

    try {
      let result: any;

      switch (action.type) {
        case 'file_read':
          result = await this.readFile(action.description);
          break;
        case 'file_write':
          result = await this.writeFile(action.description, action.result);
          break;
        case 'file_create':
          result = await this.createFile(action.description);
          break;
        case 'file_delete':
          result = await this.deleteFile(action.description);
          break;
        case 'directory_create':
          result = await this.createDirectory(action.description);
          break;
        case 'command_execute':
          result = await this.executeCommand(action.description);
          break;
        case 'api_call':
          result = await this.makeAPICall(action.description);
          break;
        case 'network_request':
          result = await this.makeNetworkRequest(action.description);
          break;
        case 'git_operation':
          result = await this.gitOperation(action.description);
          break;
        default:
          throw new Error(`Unknown action type: ${action.type}`);
      }

      action.status = 'completed';
      action.result = result;
      this.actionHistory.push(action);

      console.log(`[Action] ✅ ${action.type}: ${action.description.substring(0, 50)}...`);

      return result;
    } catch (error: any) {
      action.status = 'failed';
      action.error = error.message;
      this.actionHistory.push(action);

      console.error(`[Action] ❌ ${action.type}: ${error.message}`);

      throw error;
    }
  }

  /**
   * Check if action is safe
   */
  private isActionSafe(action: Action): boolean {
    // Check blocked commands
    if (action.type === 'command_execute') {
      for (const blocked of this.blockedCommands) {
        if (action.description.includes(blocked)) {
          console.warn(`[Action] 🚫 Blocked dangerous command: ${action.description}`);
          return false;
        }
      }
    }

    // Check path restrictions
    if (['file_read', 'file_write', 'file_delete', 'file_create'].includes(action.type)) {
      const actionPath = path.resolve(action.description);
      const isAllowed = this.allowedPaths.some(allowed => 
        actionPath.startsWith(path.resolve(allowed))
      );

      if (!isAllowed) {
        console.warn(`[Action] 🚫 Path not allowed: ${action.description}`);
        return false;
      }
    }

    return true;
  }

  /**
   * Read file
   */
  private async readFile(filePath: string): Promise<string> {
    return fs.readFile(filePath, 'utf-8');
  }

  /**
   * Write file
   */
  private async writeFile(filePath: string, content: string): Promise<void> {
    await fs.writeFile(filePath, content, 'utf-8');
  }

  /**
   * Create file
   */
  private async createFile(filePath: string): Promise<void> {
    await fs.mkdir(path.dirname(filePath), { recursive: true });
    await fs.writeFile(filePath, '', 'utf-8');
  }

  /**
   * Delete file
   */
  private async deleteFile(filePath: string): Promise<void> {
    await fs.unlink(filePath);
  }

  /**
   * Create directory
   */
  private async createDirectory(dirPath: string): Promise<void> {
    await fs.mkdir(dirPath, { recursive: true });
  }

  /**
   * Execute command
   */
  private async executeCommand(command: string): Promise<{ stdout: string; stderr: string }> {
    const { stdout, stderr } = await execAsync(command);
    return { stdout, stderr };
  }

  /**
   * Make API call
   */
  private async makeAPICall(url: string): Promise<any> {
    const response = await fetch(url);
    return response.json();
  }

  /**
   * Make network request
   */
  private async makeNetworkRequest(url: string): Promise<string> {
    const response = await fetch(url);
    return response.text();
  }

  /**
   * Git operation
   */
  private async gitOperation(command: string): Promise<{ stdout: string; stderr: string }> {
    const gitCommand = `git ${command}`;
    return this.executeCommand(gitCommand);
  }

  /**
   * Get action history
   */
  getHistory(limit: number = 50): Action[] {
    return this.actionHistory.slice(-limit);
  }

  /**
   * Get action stats
   */
  getStats(): {
    total: number;
    completed: number;
    failed: number;
    perType: Record<string, number>;
    actionsThisMinute: number;
  } {
    const perType: Record<string, number> = {};
    let completed = 0;
    let failed = 0;

    for (const action of this.actionHistory) {
      perType[action.type] = (perType[action.type] || 0) + 1;
      if (action.status === 'completed') completed++;
      if (action.status === 'failed') failed++;
    }

    return {
      total: this.actionHistory.length,
      completed,
      failed,
      perType,
      actionsThisMinute: this.actionCount
    };
  }

  /**
   * Stop action system
   */
  stop(): void {
    if (this.resetTimer) {
      clearInterval(this.resetTimer);
    }
    console.log('[Action] ⏸️ Action system paused');
  }
}

// Export factory
export function createAIRIActionSystem(allowedPaths?: string[]): AIRIActionSystem {
  return new AIRIActionSystem(allowedPaths);
}
