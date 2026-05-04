/**
 * AIRI Development Assistant
 * Full-featured coding partner
 */

import { hadesOllama } from '../hades-ollama-service';
import { getModel } from './model-config';
import { invoke } from '../tauri_bridge';

export interface DevelopmentTask {
  id: string;
  type: TaskType;
  description: string;
  file?: string;
  status: 'pending' | 'in_progress' | 'completed' | 'failed';
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
  private taskQueue: DevelopmentTask[] = [];
  private activeTasks: DevelopmentTask[] = [];
  private readonly MODEL_ROLE = 'code_gen';
  private workspacePath: string;

  constructor(workspacePath: string) {
    this.workspacePath = workspacePath;
  }

  async writeCode(description: string, language: string): Promise<string> {
    const prompt = `Write ${language} code: ${description}`;
    try {
      const response = await hadesOllama.generate(prompt, {
        model: getModel(this.MODEL_ROLE),
        stream: false
      });
      return this.extractCode(response.response || '');
    } catch (error) {
      throw error;
    }
  }

  async fixBug(code: string, error: string): Promise<any> {
    const prompt = `Fix error: ${error}\nCode: ${code}`;
    try {
      const response = await hadesOllama.generate(prompt, {
        model: getModel(this.MODEL_ROLE),
        stream: false
      });
      return { fixedCode: this.extractCode(response.response || ''), explanation: 'Generated fix' };
    } catch (error) {
      throw error;
    }
  }

  private extractCode(response: string): string {
    const codeBlockMatch = response.match(/```[\w]*\n([\s\S]*?)```/);
    return codeBlockMatch ? codeBlockMatch[1].trim() : response.trim();
  }

  async saveToFile(filePath: string, code: string): Promise<void> {
    await invoke('write_file', { path: filePath, content: code });
  }

  async readFile(filePath: string): Promise<string> {
    return await invoke<string>('read_file', { path: filePath });
  }

  getStats(): any {
    return { queued: this.taskQueue.length };
  }
}

export function createAIRIDevelopmentAssistant(workspacePath: string): AIRIDevelopmentAssistant {
  return new AIRIDevelopmentAssistant(workspacePath);
}

export const airiDevelopmentAssistant = new AIRIDevelopmentAssistant('c:/Users/HADES/Desktop/vscodium-rust');
