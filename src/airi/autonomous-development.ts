/**
 * AIRI Autonomous Development System
 * Complete autonomy - AIRI comprehends, plans, and builds complex systems
 * Takes high-level goals, breaks into tasks, executes independently
 * No human interference needed - she understands, plans, develops
 */

import { Ollama } from 'ollama';
import * as fs from 'fs/promises';
import * as path from 'path';

export interface DevelopmentGoal {
  id: string;
  description: string;
  complexity: 'simple' | 'moderate' | 'complex' | 'system';
  status: 'understanding' | 'planning' | 'executing' | 'completed' | 'failed';
  createdAt: number;
  completedAt?: number;
  tasks: AutonomousTask[];
  architecture?: SystemArchitecture;
}

export interface AutonomousTask {
  id: string;
  parentId?: string;
  description: string;
  type: TaskType;
  status: 'pending' | 'in_progress' | 'completed' | 'blocked';
  priority: number;
  dependencies: string[];
  code?: string;
  result?: string;
  error?: string;
}

export interface SystemArchitecture {
  components: Component[];
  dataFlow: string[];
  technologies: string[];
  fileStructure: FileNode[];
}

export interface Component {
  name: string;
  purpose: string;
  interfaces: string[];
  dependencies: string[];
}

export interface FileNode {
  path: string;
  type: 'file' | 'directory';
  content?: string;
  children?: FileNode[];
}

export type TaskType =
  | 'analyze'
  | 'design'
  | 'implement'
  | 'test'
  | 'document'
  | 'integrate'
  | 'deploy';

export class AIRIAutonomousDevelopment {
  private ollama: Ollama;
  private goals: DevelopmentGoal[];
  private activeGoal: DevelopmentGoal | null;
  private readonly MODEL = 'qwen3.6:32b-q4_K_M';
  private workspacePath: string;
  private isWorking: boolean = false;

  constructor(workspacePath: string) {
    this.ollama = new Ollama({ host: 'http://localhost:1536' }); // AIM proxy
    this.goals = [];
    this.activeGoal = null;
    this.workspacePath = workspacePath;

  }

  /**
   * Receive a high-level goal
   * AIRI will understand, plan, and execute autonomously
   */
  async receiveGoal(description: string): Promise<DevelopmentGoal> {

    const goal: DevelopmentGoal = {
      id: `goal_${Date.now()}`,
      description,
      complexity: 'moderate',
      status: 'understanding',
      createdAt: Date.now(),
      tasks: []
    };

    this.goals.push(goal);
    this.activeGoal = goal;

    // Phase 1: Understand and comprehend
    await this.understandGoal(goal);

    // Phase 2: Plan architecture
    await this.planArchitecture(goal);

    // Phase 3: Break into tasks
    await this.breakIntoTasks(goal);

    // Phase 4: Execute autonomously
    this.executeAutonomously(goal);

    return goal;
  }

  /**
   * Phase 1: Understand the goal deeply
   */
  private async understandGoal(goal: DevelopmentGoal): Promise<void> {

    const prompt = `
Deeply analyze this development goal:

${goal.description}

Provide:
1. What is being requested? (core requirements)
2. What are the implicit requirements?
3. What is the scope and complexity?
4. What are potential challenges?
5. What technologies would be appropriate?

Respond with detailed analysis.
`;

    try {
      const response = await this.ollama.generate({
        model: this.MODEL,
        prompt,
        stream: false
      });


      // Determine complexity
      if (response.response.toLowerCase().includes('complex') || 
          response.response.toLowerCase().includes('system')) {
        goal.complexity = 'complex';
      } else if (response.response.toLowerCase().includes('simple')) {
        goal.complexity = 'simple';
      }

      goal.status = 'planning';
    } catch (error) {
      console.error('[AutonomousDev] Understanding failed:', error);
      goal.status = 'failed';
    }
  }

  /**
   * Phase 2: Plan system architecture
   */
  private async planArchitecture(goal: DevelopmentGoal): Promise<void> {

    const prompt = `
Design a complete system architecture for:

${goal.description}

Provide:
1. System components and their purposes
2. Data flow between components
3. Technology stack recommendation
4. File/directory structure
5. Key interfaces and APIs

Respond in structured format:

COMPONENTS:
- ComponentName: purpose

DATAFLOW:
- Step by step data flow

TECHNOLOGIES:
- List of technologies

FILESTRUCTURE:
- Directory tree

API:
- Key endpoints/interfaces
`;

    try {
      const response = await this.ollama.generate({
        model: this.MODEL,
        prompt,
        stream: false
      });

      const architecture = this.parseArchitecture(response.response);
      goal.architecture = architecture;


    } catch (error) {
      console.error('[AutonomousDev] Architecture planning failed:', error);
    }
  }

  /**
   * Phase 3: Break down into autonomous tasks
   */
  private async breakIntoTasks(goal: DevelopmentGoal): Promise<void> {

    const prompt = `
Break this development goal into autonomous tasks:

Goal: ${goal.description}

Architecture:
${JSON.stringify(goal.architecture, null, 2)}

Create a complete task list that:
1. Starts with setup/scaffold
2. Implements each component
3. Writes tests for each component
4. Integrates components
5. Writes documentation
6. Final testing

Each task should be:
- Specific and actionable
- Have clear dependencies
- Be independently executable

Respond as JSON array:
[
  {
    "description": "Set up project structure",
    "type": "implement",
    "priority": 1,
    "dependencies": []
  },
  {
    "description": "Create database schema",
    "type": "implement",
    "priority": 2,
    "dependencies": ["Set up project structure"]
  }
]
`;

    try {
      const response = await this.ollama.generate({
        model: this.MODEL,
        prompt,
        stream: false
      });

      const tasks = this.parseTasks(response.response, goal.id);
      goal.tasks = tasks;


    } catch (error) {
      console.error('[AutonomousDev] Task breakdown failed:', error);
    }
  }

  /**
   * Phase 4: Execute tasks autonomously
   */
  private async executeAutonomously(goal: DevelopmentGoal): Promise<void> {
    if (this.isWorking) {
      return;
    }

    this.isWorking = true;
    goal.status = 'executing';


    try {
      // Sort tasks by priority and dependencies
      const sortedTasks = this.sortTasks(goal.tasks);

      for (const task of sortedTasks) {
        if (task.status === 'completed') continue;

        // Check dependencies
        if (!this.dependenciesMet(task, sortedTasks)) {
          task.status = 'blocked';
          continue;
        }

        task.status = 'in_progress';

        try {
          // Execute based on task type
          switch (task.type) {
            case 'implement':
              await this.implementTask(task, goal);
              break;
            case 'test':
              await this.testTask(task, goal);
              break;
            case 'document':
              await this.documentTask(task, goal);
              break;
            case 'design':
              await this.designTask(task, goal);
              break;
            case 'integrate':
              await this.integrateTask(task, goal);
              break;
            case 'analyze':
              await this.analyzeTask(task, goal);
              break;
            case 'deploy':
              await this.deployTask(task, goal);
              break;
          }

          task.status = 'completed';

        } catch (error: any) {
          task.status = 'blocked';
          task.error = error.message;
          console.error(`[AutonomousDev] ❌ Failed: ${task.description}`);
          console.error(`   Error: ${error.message}\n`);
        }
      }

      goal.status = 'completed';
      goal.completedAt = Date.now();


    } catch (error) {
      console.error('[AutonomousDev] Execution failed:', error);
      goal.status = 'failed';
    } finally {
      this.isWorking = false;
    }
  }

  /**
   * Implement a task (write code)
   */
  private async implementTask(task: AutonomousTask, goal: DevelopmentGoal): Promise<void> {
    const prompt = `
Implement this task for the development goal:

Goal: ${goal.description}
Task: ${task.description}

Architecture:
${JSON.stringify(goal.architecture, null, 2)}

Provide complete, production-ready code:
- Full implementation
- Proper error handling
- Type safety (if applicable)
- Comments for complex logic
- Follow best practices

Write the complete code.
`;

    const response = await this.ollama.generate({
      model: this.MODEL,
      prompt,
      stream: false
    });

    const code = this.extractCode(response.response);
    task.code = code;

    // Save to file if path can be determined
    const filePath = this.determineFilePath(task, goal);
    if (filePath) {
      await fs.mkdir(path.dirname(filePath), { recursive: true });
      await fs.writeFile(filePath, code, 'utf-8');
      task.result = `Saved to ${filePath}`;
    }
  }

  /**
   * Write tests for a task
   */
  private async testTask(task: AutonomousTask, goal: DevelopmentGoal): Promise<void> {
    // Find the code to test
    const codeToTest = this.findRelatedCode(task, goal);

    const prompt = `
Write comprehensive tests for this code:

${codeToTest || 'Code related to: ' + task.description}

Include:
1. Unit tests for all functions
2. Edge cases
3. Error scenarios
4. Integration tests if applicable

Testing framework: Jest (or appropriate for the language)

Write complete, runnable tests.
`;

    const response = await this.ollama.generate({
      model: this.MODEL,
      prompt,
      stream: false
    });

    const tests = this.extractCode(response.response);
    task.code = tests;

    // Save test file
    const testPath = this.determineTestPath(task, goal);
    if (testPath) {
      await fs.mkdir(path.dirname(testPath), { recursive: true });
      await fs.writeFile(testPath, tests, 'utf-8');
      task.result = `Tests saved to ${testPath}`;
    }
  }

  /**
   * Write documentation
   */
  private async documentTask(task: AutonomousTask, goal: DevelopmentGoal): Promise<void> {
    const prompt = `
Write documentation for:

Goal: ${goal.description}
Task: ${task.description}

Include:
1. Overview and purpose
2. Installation/setup instructions
3. Usage examples
4. API reference (if applicable)
5. Architecture explanation

Write comprehensive, clear documentation.
`;

    const response = await this.ollama.generate({
      model: this.MODEL,
      prompt,
      stream: false
    });

    task.code = response.response;

    // Save documentation
    const docPath = path.join(this.workspacePath, 'DOCS.md');
    await fs.writeFile(docPath, response.response, 'utf-8');
    task.result = 'Documentation saved to DOCS.md';
  }

  /**
   * Design task
   */
  private async designTask(task: AutonomousTask, goal: DevelopmentGoal): Promise<void> {
    const prompt = `
Create detailed design for:

Goal: ${goal.description}
Task: ${task.description}

Provide:
1. Component design
2. Data structures
3. Interfaces
4. Flow diagrams (text-based)
5. Design decisions and rationale

Respond with detailed design document.
`;

    const response = await this.ollama.generate({
      model: this.MODEL,
      prompt,
      stream: false
    });

    task.code = response.response;
    task.result = 'Design completed';
  }

  /**
   * Integrate components
   */
  private async integrateTask(task: AutonomousTask, goal: DevelopmentGoal): Promise<void> {
    const prompt = `
Integrate components for:

Goal: ${goal.description}
Task: ${task.description}

Existing code:
${this.getExistingCode(goal)}

Provide integration code that:
1. Connects components
2. Handles data flow
3. Manages errors
4. Provides unified interface

Write complete integration code.
`;

    const response = await this.ollama.generate({
      model: this.MODEL,
      prompt,
      stream: false
    });

    const code = this.extractCode(response.response);
    task.code = code;

    // Save integration file
    const integratePath = path.join(this.workspacePath, 'src', 'index.ts');
    await fs.mkdir(path.dirname(integratePath), { recursive: true });
    await fs.writeFile(integratePath, code, 'utf-8');
    task.result = 'Integration saved to src/index.ts';
  }

  /**
   * Analyze task
   */
  private async analyzeTask(task: AutonomousTask, goal: DevelopmentGoal): Promise<void> {
    const prompt = `
Analyze requirements for:

Goal: ${goal.description}
Task: ${task.description}

Provide:
1. Detailed requirements analysis
2. User stories
3. Acceptance criteria
4. Technical constraints
5. Risk assessment

Respond with comprehensive analysis.
`;

    const response = await this.ollama.generate({
      model: this.MODEL,
      prompt,
      stream: false
    });

    task.code = response.response;
    task.result = 'Analysis completed';
  }

  /**
   * Deploy task
   */
  private async deployTask(task: AutonomousTask, goal: DevelopmentGoal): Promise<void> {
    const prompt = `
Create deployment configuration for:

Goal: ${goal.description}
Task: ${task.description}

Provide:
1. Docker configuration
2. CI/CD pipeline
3. Environment variables
4. Deployment scripts
5. Monitoring setup

Write complete deployment configuration.
`;

    const response = await this.ollama.generate({
      model: this.MODEL,
      prompt,
      stream: false
    });

    const code = this.extractCode(response.response);
    task.code = code;

    // Save deployment files
    const dockerPath = path.join(this.workspacePath, 'Dockerfile');
    await fs.writeFile(dockerPath, code, 'utf-8');
    task.result = 'Deployment config saved';
  }

  // Helper methods

  private parseArchitecture(response: string): SystemArchitecture {
    // Parse AI response into architecture object
    return {
      components: [],
      dataFlow: [],
      technologies: ['Node.js', 'TypeScript', 'Express'],
      fileStructure: []
    };
  }

  private parseTasks(response: string, goalId: string): AutonomousTask[] {
    try {
      const jsonMatch = response.match(/\[[\s\S]*\]/);
      if (jsonMatch) {
        const tasks = JSON.parse(jsonMatch[0]);
        return tasks.map((t: any, i: number) => ({
          id: `task_${goalId}_${i}`,
          description: t.description,
          type: t.type as TaskType,
          priority: t.priority || 5,
          dependencies: t.dependencies || [],
          status: 'pending' as const
        }));
      }
    } catch (error) {
      console.error('Task parsing failed:', error);
    }
    return [];
  }

  private sortTasks(tasks: AutonomousTask[]): AutonomousTask[] {
    return tasks.sort((a, b) => a.priority - b.priority);
  }

  private dependenciesMet(task: AutonomousTask, allTasks: AutonomousTask[]): boolean {
    return task.dependencies.every(depId => {
      const depTask = allTasks.find(t => t.id === depId || t.description.includes(depId));
      return depTask && depTask.status === 'completed';
    });
  }

  private extractCode(response: string): string {
    const codeBlockMatch = response.match(/```[\w]*\n([\s\S]*?)```/);
    if (codeBlockMatch) {
      return codeBlockMatch[1].trim();
    }
    return response.trim();
  }

  private determineFilePath(task: AutonomousTask, goal: DevelopmentGoal): string | null {
    // Determine file path from task description
    if (task.description.toLowerCase().includes('user')) {
      return path.join(this.workspacePath, 'src', 'models', 'User.ts');
    }
    if (task.description.toLowerCase().includes('auth')) {
      return path.join(this.workspacePath, 'src', 'auth', 'auth.ts');
    }
    return null;
  }

  private determineTestPath(task: AutonomousTask, goal: DevelopmentGoal): string {
    return path.join(this.workspacePath, 'tests', `${task.id}.test.ts`);
  }

  private findRelatedCode(task: AutonomousTask, goal: DevelopmentGoal): string | null {
    // Find code from related implementation task
    const relatedTask = goal.tasks.find(t => 
      t.type === 'implement' && 
      t.code &&
      t.description.toLowerCase().includes(task.description.toLowerCase().split(' ')[0])
    );
    return relatedTask?.code || null;
  }

  private getExistingCode(goal: DevelopmentGoal): string {
    return goal.tasks
      .filter(t => t.code && t.status === 'completed')
      .map(t => `// ${t.description}\n${t.code}`)
      .join('\n\n');
  }

  private countFiles(fileStructure: FileNode[]): number {
    let count = 0;
    for (const node of fileStructure) {
      if (node.type === 'file') count++;
      if (node.children) count += this.countFiles(node.children);
    }
    return count;
  }

  /**
   * Get goal progress
   */
  getProgress(goalId: string): {
    total: number;
    completed: number;
    inProgress: number;
    blocked: number;
    percentage: number;
  } {
    const goal = this.goals.find(g => g.id === goalId);
    if (!goal) {
      return { total: 0, completed: 0, inProgress: 0, blocked: 0, percentage: 0 };
    }

    const total = goal.tasks.length;
    const completed = goal.tasks.filter(t => t.status === 'completed').length;
    const inProgress = goal.tasks.filter(t => t.status === 'in_progress').length;
    const blocked = goal.tasks.filter(t => t.status === 'blocked').length;

    return {
      total,
      completed,
      inProgress,
      blocked,
      percentage: Math.round((completed / total) * 100)
    };
  }

  /**
   * Get all goals
   */
  getGoals(): DevelopmentGoal[] {
    return this.goals;
  }

  /**
   * Get active goal
   */
  getActiveGoal(): DevelopmentGoal | null {
    return this.activeGoal;
  }
}

// Export factory
export function createAIRIAutonomousDevelopment(workspacePath: string): AIRIAutonomousDevelopment {
  return new AIRIAutonomousDevelopment(workspacePath);
}
