// =============================================================================
// Tool Registry — Structured tool definitions inspired by Claude Code's
// architecture. Each tool has a JSON schema, description, and execution
// function backed by Tauri invoke commands.
//
// Replaces the old regex-based parseToolCall() / executeTool() system with
// proper function calling compatible with OpenAI, Anthropic, and Google APIs.
// =============================================================================

import { invoke } from './tauri_bridge';

// ---------------------------------------------------------------------------
// Core Types
// ---------------------------------------------------------------------------

export interface ToolParameter {
    type: string;
    description: string;
    enum?: string[];
    items?: { type: string };
    default?: any;
}

export interface ToolInputSchema {
    type: 'object';
    properties: Record<string, ToolParameter>;
    required?: string[];
}

export interface ToolResult {
    success: boolean;
    data: any;
    error?: string;
}

export interface ToolContext {
    activeRoot: string;
    activeFile?: string;
    agentMode: string;
    abortSignal?: AbortSignal;
}

export interface ToolDef {
    name: string;
    description: string;
    inputSchema: ToolInputSchema;
    isReadOnly?: boolean;
    isEnabled?: () => boolean;
    execute: (input: any, ctx: ToolContext) => Promise<ToolResult>;
}

// ---------------------------------------------------------------------------
// Tool Call / Result Types (for structured function calling)
// ---------------------------------------------------------------------------

export interface ToolCall {
    id: string;
    name: string;
    arguments: Record<string, any>;
}

export interface ToolCallResult {
    tool_call_id: string;
    content: string;
}

// ---------------------------------------------------------------------------
// Helper: Generate unique tool call ID
// ---------------------------------------------------------------------------
let _toolCallCounter = 0;
export function generateToolCallId(): string {
    return `call_${Date.now()}_${++_toolCallCounter}`;
}

// ---------------------------------------------------------------------------
// Helper: Success / Error result builders
// ---------------------------------------------------------------------------
function ok(data: any): ToolResult {
    return { success: true, data };
}

function fail(error: string): ToolResult {
    return { success: false, data: null, error };
}

// =============================================================================
// TOOL DEFINITIONS
// =============================================================================

// ---------------------------------------------------------------------------
// 1. BashTool — Execute shell commands
// ---------------------------------------------------------------------------
export const BashTool: ToolDef = {
    name: 'bash',
    description: `Execute a shell command on the system. Use this for running scripts, installing packages, compiling code, managing git, or any system-level task. The command runs in the project root directory. Commands run with the user's full permissions. For long-running commands, the output will be captured and returned.`,
    inputSchema: {
        type: 'object',
        properties: {
            command: {
                type: 'string',
                description: 'The shell command to execute. Can be multi-line. Avoid interactive commands that wait for user input.',
            },
            timeout_ms: {
                type: 'number',
                description: 'Optional timeout in milliseconds. Defaults to 120000 (2 minutes).',
                default: 120000,
            },
        },
        required: ['command'],
    },
    execute: async (input, ctx) => {
        try {
            const result = await invoke<string>('ai_execute_command', {
                command: input.command,
                cwd: ctx.activeRoot || undefined,
                timeout: input.timeout_ms || 120000,
            });
            return ok(result);
        } catch (e: any) {
            return fail(`Command failed: ${e.message || e}`);
        }
    },
};

// ---------------------------------------------------------------------------
// 2. FileReadTool — Read file contents
// ---------------------------------------------------------------------------
export const FileReadTool: ToolDef = {
    name: 'file_read',
    description: `Read the contents of a file from the filesystem. Supports text files, showing content with line numbers. For large files, use offset and limit to read specific portions. The file_path must be an absolute path.`,
    inputSchema: {
        type: 'object',
        properties: {
            file_path: {
                type: 'string',
                description: 'The absolute path to the file to read.',
            },
            offset: {
                type: 'number',
                description: 'The line number to start reading from (1-indexed). Only provide if the file is too large to read at once.',
            },
            limit: {
                type: 'number',
                description: 'The maximum number of lines to read. Only provide if the file is too large to read at once.',
            },
        },
        required: ['file_path'],
    },
    isReadOnly: true,
    execute: async (input, _ctx) => {
        try {
            const content = await invoke<string>('read_file', { path: input.file_path });
            if (content === null || content === undefined) {
                return fail(`File not found: ${input.file_path}`);
            }
            const lines = content.split('\n');
            const totalLines = lines.length;
            const offset = Math.max(0, (input.offset || 1) - 1);
            const limit = input.limit || totalLines;
            const sliced = lines.slice(offset, offset + limit);
            const numbered = sliced.map((line: string, i: number) => `${offset + i + 1}: ${line}`).join('\n');
            return ok({
                content: numbered,
                totalLines,
                startLine: offset + 1,
                numLines: sliced.length,
                filePath: input.file_path,
            });
        } catch (e: any) {
            return fail(`Failed to read file: ${e.message || e}`);
        }
    },
};

// ---------------------------------------------------------------------------
// 3. FileWriteTool — Create or overwrite files
// ---------------------------------------------------------------------------
export const FileWriteTool: ToolDef = {
    name: 'file_write',
    description: `Write content to a file. If the file exists, it will be overwritten. If it doesn't exist, it will be created along with any necessary parent directories. The file_path must be absolute. Always provide the complete file content — this tool does whole-file replacement.`,
    inputSchema: {
        type: 'object',
        properties: {
            file_path: {
                type: 'string',
                description: 'The absolute path to the file to write.',
            },
            content: {
                type: 'string',
                description: 'The complete content to write to the file.',
            },
        },
        required: ['file_path', 'content'],
    },
    execute: async (input, _ctx) => {
        try {
            await invoke('write_file_content', {
                path: input.file_path,
                content: input.content,
            });
            return ok({ filePath: input.file_path, type: 'written' });
        } catch (e: any) {
            return fail(`Failed to write file: ${e.message || e}`);
        }
    },
};

// ---------------------------------------------------------------------------
// 4. FileEditTool — Partial file modifications (string replacement)
// ---------------------------------------------------------------------------
export const FileEditTool: ToolDef = {
    name: 'file_edit',
    description: `Make targeted edits to a file by replacing specific text. This is the preferred tool for making changes to existing files — use this instead of file_write when modifying existing content. Specify the exact text to find and what to replace it with. The old_string must match exactly (including whitespace and indentation). For creating new files, use file_write instead.`,
    inputSchema: {
        type: 'object',
        properties: {
            file_path: {
                type: 'string',
                description: 'The absolute path to the file to edit.',
            },
            old_string: {
                type: 'string',
                description: 'The exact text to find in the file. Must match precisely including whitespace.',
            },
            new_string: {
                type: 'string',
                description: 'The text to replace old_string with.',
            },
            replace_all: {
                type: 'boolean',
                description: 'If true, replace all occurrences. If false (default), replace only the first occurrence.',
                default: false,
            },
        },
        required: ['file_path', 'old_string', 'new_string'],
    },
    execute: async (input, _ctx) => {
        try {
            await invoke('ai_modify_file', {
                path: input.file_path,
                target: input.old_string,
                replacement: input.new_string,
            });
            return ok({ filePath: input.file_path, type: 'edited' });
        } catch (e: any) {
            return fail(`Failed to edit file: ${e.message || e}`);
        }
    },
};

// ---------------------------------------------------------------------------
// 5. GlobTool — Find files by name/pattern
// ---------------------------------------------------------------------------
export const GlobTool: ToolDef = {
    name: 'glob',
    description: `Find files matching a glob pattern. Use this to discover files in the project by name pattern (e.g., "**/*.ts" for all TypeScript files). Results are relative paths. Limited to 100 results.`,
    inputSchema: {
        type: 'object',
        properties: {
            pattern: {
                type: 'string',
                description: 'The glob pattern to match files against (e.g., "**/*.tsx", "src/**/*.rs").',
            },
            path: {
                type: 'string',
                description: 'The directory to search in. Defaults to the project root if not specified.',
            },
        },
        required: ['pattern'],
    },
    isReadOnly: true,
    execute: async (input, ctx) => {
        try {
            const searchPath = input.path || ctx.activeRoot;
            const results = await invoke<string[]>('glob_files', {
                pattern: input.pattern,
                path: searchPath,
            });
            return ok({
                filenames: results || [],
                numFiles: results?.length || 0,
                truncated: (results?.length || 0) >= 100,
            });
        } catch (e: any) {
            // Fallback: use list_directory + filter if glob_files not implemented yet
            try {
                const entries = await invoke<any[]>('list_directory', {
                    path: input.path || ctx.activeRoot,
                });
                const filtered = (entries || [])
                    .filter((e: any) => !e.is_dir)
                    .map((e: any) => e.path || e.name);
                return ok({
                    filenames: filtered.slice(0, 100),
                    numFiles: filtered.length,
                    truncated: filtered.length >= 100,
                });
            } catch (e2: any) {
                return fail(`Glob search failed: ${e.message || e}`);
            }
        }
    },
};

// ---------------------------------------------------------------------------
// 6. GrepTool — Search file contents with ripgrep patterns
// ---------------------------------------------------------------------------
export const GrepTool: ToolDef = {
    name: 'grep',
    description: `Search for text patterns in files using regex. Returns matching lines with file paths and line numbers. Use this to find code patterns, function definitions, imports, or any text across the project. Supports regular expressions.`,
    inputSchema: {
        type: 'object',
        properties: {
            pattern: {
                type: 'string',
                description: 'The regex pattern to search for.',
            },
            path: {
                type: 'string',
                description: 'Directory or file to search in. Defaults to project root.',
            },
            include: {
                type: 'string',
                description: 'Glob pattern to filter files (e.g., "*.ts" to only search TypeScript files).',
            },
        },
        required: ['pattern'],
    },
    isReadOnly: true,
    execute: async (input, ctx) => {
        try {
            const searchPath = input.path || ctx.activeRoot;
            const results = await invoke<any[]>('grep_files', {
                pattern: input.pattern,
                path: searchPath,
                include: input.include,
            });
            return ok(results || []);
        } catch (e: any) {
            // Fallback: use search_project if grep_files not available
            try {
                const results = await invoke<any[]>('search_project', {
                    query: input.pattern,
                });
                return ok(results || []);
            } catch (e2: any) {
                return fail(`Search failed: ${e.message || e}`);
            }
        }
    },
};

// ---------------------------------------------------------------------------
// 7. ListDirectoryTool — List directory contents
// ---------------------------------------------------------------------------
export const ListDirectoryTool: ToolDef = {
    name: 'list_directory',
    description: `List the contents of a directory including files and subdirectories. Shows file names, sizes, and types. Use this to explore the project structure.`,
    inputSchema: {
        type: 'object',
        properties: {
            path: {
                type: 'string',
                description: 'The absolute path to the directory to list.',
            },
            recursive: {
                type: 'boolean',
                description: 'If true, list contents recursively.',
                default: false,
            },
        },
        required: ['path'],
    },
    isReadOnly: true,
    execute: async (input, _ctx) => {
        try {
            const entries = await invoke<any[]>('list_directory', { path: input.path });
            const formatted = (entries || []).map((e: any) => ({
                name: e.name,
                path: e.path,
                is_dir: e.is_dir,
                size: e.size,
            }));
            return ok(formatted);
        } catch (e: any) {
            return fail(`Failed to list directory: ${e.message || e}`);
        }
    },
};

// ---------------------------------------------------------------------------
// 8. WebFetchTool — Fetch content from URLs
// ---------------------------------------------------------------------------
export const WebFetchTool: ToolDef = {
    name: 'web_fetch',
    description: `Fetch the content of a URL and return it as text. Useful for reading documentation, downloading files, or checking web pages. Converts HTML to readable text.`,
    inputSchema: {
        type: 'object',
        properties: {
            url: {
                type: 'string',
                description: 'The URL to fetch content from.',
            },
        },
        required: ['url'],
    },
    isReadOnly: true,
    execute: async (input, _ctx) => {
        try {
            const result = await invoke<string>('web_fetch', { url: input.url });
            return ok({ url: input.url, content: result });
        } catch (e: any) {
            // Browser fallback
            try {
                const resp = await fetch(input.url);
                const text = await resp.text();
                return ok({ url: input.url, content: text.slice(0, 50000) });
            } catch (e2: any) {
                return fail(`Failed to fetch URL: ${e.message || e}`);
            }
        }
    },
};

// ---------------------------------------------------------------------------
// 9. WebSearchTool — Search the web
// ---------------------------------------------------------------------------
export const WebSearchTool: ToolDef = {
    name: 'web_search',
    description: `Search the web for information. Returns relevant results with titles, URLs, and snippets. Use this when you need current information or to research topics.`,
    inputSchema: {
        type: 'object',
        properties: {
            query: {
                type: 'string',
                description: 'The search query.',
            },
            num_results: {
                type: 'number',
                description: 'Number of results to return. Default is 5.',
                default: 5,
            },
        },
        required: ['query'],
    },
    isReadOnly: true,
    execute: async (input, _ctx) => {
        try {
            const results = await invoke<any[]>('web_search', {
                query: input.query,
                numResults: input.num_results || 5,
            });
            return ok(results || []);
        } catch (e: any) {
            return fail(`Web search failed: ${e.message || e}`);
        }
    },
};

// ---------------------------------------------------------------------------
// 10. BrowserTool — Browser automation
// ---------------------------------------------------------------------------
export const BrowserOpenTool: ToolDef = {
    name: 'browser_open',
    description: `Open a headless browser session for web automation, testing, or screenshot capture.`,
    inputSchema: { type: 'object', properties: {}, required: [] },
    execute: async (_input, _ctx) => {
        try {
            const result = await invoke<string>('browser_open');
            return ok(result);
        } catch (e: any) {
            return fail(`Browser open failed: ${e}`);
        }
    },
};

export const BrowserNavigateTool: ToolDef = {
    name: 'browser_navigate',
    description: `Navigate the browser to a URL. Opens the URL in the headless browser for taking screenshots, extracting DOM, or interacting with web pages.`,
    inputSchema: {
        type: 'object',
        properties: {
            url: { type: 'string', description: 'The URL to navigate to.' },
        },
        required: ['url'],
    },
    execute: async (input, _ctx) => {
        try {
            const result = await invoke<string>('browser_navigate', { url: input.url });
            return ok(result);
        } catch (e: any) {
            return fail(`Browser navigate failed: ${e}`);
        }
    },
};

export const BrowserScreenshotTool: ToolDef = {
    name: 'browser_screenshot',
    description: `Take a screenshot of the current browser page. Returns a base64-encoded image.`,
    inputSchema: { type: 'object', properties: {}, required: [] },
    isReadOnly: true,
    execute: async (_input, _ctx) => {
        try {
            const b64 = await invoke<string>('browser_screenshot');
            return ok({ screenshot: 'captured', base64Length: b64?.length || 0 });
        } catch (e: any) {
            return fail(`Screenshot failed: ${e}`);
        }
    },
};

export const BrowserCloseTool: ToolDef = {
    name: 'browser_close',
    description: `Close the current browser session.`,
    inputSchema: { type: 'object', properties: {}, required: [] },
    execute: async (_input, _ctx) => {
        try {
            await invoke('browser_close');
            return ok('Browser closed');
        } catch (e: any) {
            return fail(`Browser close failed: ${e}`);
        }
    },
};

// ---------------------------------------------------------------------------
// 11. TodoWriteTool — Manage TODO/task lists
// ---------------------------------------------------------------------------
export const TodoWriteTool: ToolDef = {
    name: 'todo_write',
    description: `Create or update a TODO list or task file. Writes structured task content to a markdown file for tracking implementation progress.`,
    inputSchema: {
        type: 'object',
        properties: {
            file_path: {
                type: 'string',
                description: 'Path to the TODO/task file (e.g., TODO.md, tasks.md).',
            },
            content: {
                type: 'string',
                description: 'The TODO list content in markdown format with checkboxes.',
            },
        },
        required: ['file_path', 'content'],
    },
    execute: async (input, _ctx) => {
        try {
            await invoke('write_file_content', {
                path: input.file_path,
                content: input.content,
            });
            return ok({ filePath: input.file_path, type: 'todo_updated' });
        } catch (e: any) {
            return fail(`Failed to write TODO: ${e.message || e}`);
        }
    },
};

// ---------------------------------------------------------------------------
// 12. TaskCreateTool — Create a new task
// ---------------------------------------------------------------------------
export const TaskCreateTool: ToolDef = {
    name: 'task_create',
    description: `Create a new task for tracking work. Tasks have a title, description, and status. Use this to break down complex work into trackable pieces.`,
    inputSchema: {
        type: 'object',
        properties: {
            title: { type: 'string', description: 'Short title for the task.' },
            description: { type: 'string', description: 'Detailed description of what needs to be done.' },
            priority: {
                type: 'string',
                description: 'Priority level.',
                enum: ['low', 'medium', 'high', 'critical'],
            },
        },
        required: ['title'],
    },
    execute: async (input, _ctx) => {
        const task = {
            id: `task_${Date.now()}`,
            title: input.title,
            description: input.description || '',
            status: 'pending',
            priority: input.priority || 'medium',
            createdAt: Date.now(),
        };
        return ok(task);
    },
};

// ---------------------------------------------------------------------------
// 13. TaskUpdateTool — Update an existing task
// ---------------------------------------------------------------------------
export const TaskUpdateTool: ToolDef = {
    name: 'task_update',
    description: `Update the status or details of an existing task.`,
    inputSchema: {
        type: 'object',
        properties: {
            task_id: { type: 'string', description: 'The ID of the task to update.' },
            status: {
                type: 'string',
                description: 'New status for the task.',
                enum: ['pending', 'in_progress', 'completed', 'cancelled'],
            },
            notes: { type: 'string', description: 'Additional notes or progress update.' },
        },
        required: ['task_id', 'status'],
    },
    execute: async (input, _ctx) => {
        return ok({
            task_id: input.task_id,
            status: input.status,
            notes: input.notes || '',
            updatedAt: Date.now(),
        });
    },
};

// ---------------------------------------------------------------------------
// 14. EnterPlanModeTool — Switch to planning mode
// ---------------------------------------------------------------------------
export const EnterPlanModeTool: ToolDef = {
    name: 'enter_plan_mode',
    description: `Switch to plan mode. In plan mode, you focus on understanding requirements, analyzing the codebase, and creating a plan — without making any code changes. Use this when you need to think through a complex problem before implementing.`,
    inputSchema: {
        type: 'object',
        properties: {
            reason: { type: 'string', description: 'Why you are entering plan mode.' },
        },
        required: [],
    },
    execute: async (input, _ctx) => {
        return ok({ mode: 'planning', reason: input.reason || 'Entering planning mode' });
    },
};

// ---------------------------------------------------------------------------
// 15. ExitPlanModeTool — Switch back to execution mode
// ---------------------------------------------------------------------------
export const ExitPlanModeTool: ToolDef = {
    name: 'exit_plan_mode',
    description: `Exit plan mode and return to normal execution mode. Use this after you have finished planning and are ready to implement changes.`,
    inputSchema: {
        type: 'object',
        properties: {
            summary: { type: 'string', description: 'Summary of what was planned.' },
        },
        required: [],
    },
    execute: async (input, _ctx) => {
        return ok({ mode: 'execution', summary: input.summary || '' });
    },
};

// ---------------------------------------------------------------------------
// 16. AgentTool — Spawn a sub-agent for parallel work
// ---------------------------------------------------------------------------
export const SpawnSubAgentTool: ToolDef = {
    name: 'spawn_subagent',
    description: `Spawn a sub-agent to perform a specific task in parallel. The sub-agent has access to the same tools but operates independently. Use this for tasks that can be parallelized, like researching while implementing, or making changes to multiple independent files. Each sub-agent has its own conversation context.`,
    inputSchema: {
        type: 'object',
        properties: {
            task: {
                type: 'string',
                description: 'Detailed description of the task for the sub-agent to perform.',
            },
            working_directory: {
                type: 'string',
                description: 'The working directory for the sub-agent. Defaults to project root.',
            },
        },
        required: ['task'],
    },
    execute: async (input, ctx) => {
        try {
            const result = await invoke<string>('spawn_subagent', {
                task: input.task,
                workingDirectory: input.working_directory || ctx.activeRoot,
            });
            return ok({ agentResult: result });
        } catch (e: any) {
            return fail(`Sub-agent failed: ${e.message || e}`);
        }
    },
};

// ---------------------------------------------------------------------------
// 17. BrowserSubAgentTool — Parallel web browsing sub-agent
// ---------------------------------------------------------------------------
export const BrowserSubAgentTool: ToolDef = {
    name: 'browser_subagent',
    description: `Spawn a specialized sub-agent for complex web navigation, research, and data extraction. The sub-agent works in parallel and returns a comprehensive report. Use this instead of individual browser tools for high-level research objectives.`,
    inputSchema: {
        type: 'object',
        properties: {
            task: {
                type: 'string',
                description: 'The research task or objective for the browser sub-agent (e.g., "Find the latest documentation for Tauri v2").',
            },
        },
        required: ['task'],
    },
    execute: async (input, _ctx) => {
        try {
            const result = await invoke<string>('browser_subagent', {
                task: input.task,
            });
            return ok({ browserResult: result });
        } catch (e: any) {
            return fail(`Browser sub-agent failed: ${e.message || e}`);
        }
    },
};

// ---------------------------------------------------------------------------
// 18. AskUserQuestionTool — Ask the user a clarifying question
// ---------------------------------------------------------------------------
export const AskUserQuestionTool: ToolDef = {
    name: 'ask_user',
    description: `Ask the user a clarifying question when you need more information to proceed. Use this when instructions are ambiguous or when you need the user to make a decision.`,
    inputSchema: {
        type: 'object',
        properties: {
            question: {
                type: 'string',
                description: 'The question to ask the user.',
            },
        },
        required: ['question'],
    },
    isReadOnly: true,
    execute: async (input, _ctx) => {
        // This is handled specially by the UI — the question is displayed
        // and the user's response is sent back as the next message
        return ok({ question: input.question, awaitingResponse: true });
    },
};

// ---------------------------------------------------------------------------
// 18. GitTool — Git operations (Backend Optimized)
// ---------------------------------------------------------------------------
export const GitStatusTool: ToolDef = {
    name: 'git_status',
    description: `Get the current git status including branch, staged/unstaged changes, and recent commits. Uses native backend Git integration.`,
    inputSchema: { type: 'object', properties: {}, required: [] },
    isReadOnly: true,
    execute: async (_input, _ctx) => {
        try {
            const result = await invoke<string>('git_status');
            return ok(result);
        } catch (e: any) {
            return fail(`Git status failed: ${e}`);
        }
    },
};

export const GitDiffTool: ToolDef = {
    name: 'git_diff',
    description: `Show git diff of current changes. Shows what has been modified but not yet committed.`,
    inputSchema: {
        type: 'object',
        properties: {
            staged: {
                type: 'boolean',
                description: 'If true, show only staged changes. Default shows unstaged.',
                default: false,
            },
            file_path: {
                type: 'string',
                description: 'Optional specific file or directory to diff.',
            },
        },
        required: [],
    },
    isReadOnly: true,
    execute: async (input, _ctx) => {
        try {
            const result = await invoke<string>('git_diff', {
                staged: !!input.staged,
                path: input.file_path || undefined,
            });
            return ok(result || 'No changes detected.');
        } catch (e: any) {
            return fail(`Git diff failed: ${e}`);
        }
    },
};

export const GitAddTool: ToolDef = {
    name: 'git_add',
    description: `Stage files for commit. Moves changes from the working directory to the staging area.`,
    inputSchema: {
        type: 'object',
        properties: {
            files: {
                type: 'array',
                items: { type: 'string' },
                description: 'List of files to stage. Use ["."] to stage all changes.',
            },
        },
        required: ['files'],
    },
    execute: async (input, _ctx) => {
        try {
            const result = await invoke<string>('git_add', { files: input.files });
            return ok(result);
        } catch (e: any) {
            return fail(`Git add failed: ${e}`);
        }
    },
};

export const GitCommitTool: ToolDef = {
    name: 'git_commit',
    description: `Create a git commit with all staged changes. Should follow conventional commit format.`,
    inputSchema: {
        type: 'object',
        properties: {
            message: {
                type: 'string',
                description: 'The commit message.',
            },
        },
        required: ['message'],
    },
    execute: async (input, _ctx) => {
        try {
            const result = await invoke<string>('git_commit', { message: input.message });
            return ok(result);
        } catch (e: any) {
            return fail(`Git commit failed: ${e}`);
        }
    },
};

export const GitLogTool: ToolDef = {
    name: 'git_log',
    description: `View the git commit history.`,
    inputSchema: {
        type: 'object',
        properties: {
            limit: {
                type: 'number',
                description: 'Number of commits to show. Default is 10.',
                default: 10,
            },
        },
        required: [],
    },
    isReadOnly: true,
    execute: async (input, _ctx) => {
        try {
            const result = await invoke<string>('git_log', { limit: input.limit || 10 });
            return ok(result);
        } catch (e: any) {
            return fail(`Git log failed: ${e}`);
        }
    },
};

// ---------------------------------------------------------------------------
// 19. Terminal Persistence Tool — Persistent shell sessions
// ---------------------------------------------------------------------------
export const TerminalSendTool: ToolDef = {
    name: 'terminal_send_data',
    description: `Send input to an existing terminal session. Use this for interactive commands or long-running processes. The session stays alive between calls.`,
    inputSchema: {
        type: 'object',
        properties: {
            data: {
                type: 'string',
                description: 'Text to send to terminal stdin (include \\n for enter).',
            },
            terminal_id: {
                type: 'string',
                description: 'Optional ID of the terminal to send data to. Defaults to first active terminal.',
            },
        },
        required: ['data'],
    },
    execute: async (input, _ctx) => {
        try {
            await invoke('terminal_send_data', {
                data: input.data,
                terminalId: input.terminal_id || undefined,
            });
            return ok({ status: 'sent' });
        } catch (e: any) {
            return fail(`Terminal write failed: ${e}`);
        }
    },
};

export const TerminalReadTool: ToolDef = {
    name: 'terminal_read_output',
    description: `Read the most recent output from a persistent terminal session. Use this to poll for results from long-running commands.`,
    inputSchema: {
        type: 'object',
        properties: {
            terminal_id: {
                type: 'string',
                description: 'Optional ID of the terminal to read from. Defaults to first active terminal.',
            },
        },
        required: [],
    },
    isReadOnly: true,
    execute: async (input, _ctx) => {
        try {
            const result = await invoke<string>('terminal_read_output', {
                terminalId: input.terminal_id || undefined,
            });
            return ok(result);
        } catch (e: any) {
            return fail(`Terminal read failed: ${e}`);
        }
    },
};

// ---------------------------------------------------------------------------
// 20. SystemHealthTool — Diagnostic information (/doctor)
// ---------------------------------------------------------------------------
export const SystemHealthTool: ToolDef = {
    name: 'get_system_health',
    description: `Run system diagnostics to check the health of Git, Node.js, Rust/Cargo, and MCP servers. Essential for troubleshooting the engineering environment.`,
    inputSchema: { type: 'object', properties: {}, required: [] },
    isReadOnly: true,
    execute: async (_input, _ctx) => {
        try {
            const result = await invoke<any>('get_system_health');
            return ok(result);
        } catch (e: any) {
            return fail(`Health check failed: ${e}`);
        }
    },
};

// ---------------------------------------------------------------------------
// 21. MCP Tool — Model Context Protocol server interaction
// ---------------------------------------------------------------------------
export const MCPTool: ToolDef = {
    name: 'mcp_call',
    description: `Call a tool on a connected MCP (Model Context Protocol) server. MCP servers provide additional capabilities like database access, API integrations, and specialized tools.`,
    inputSchema: {
        type: 'object',
        properties: {
            server_name: {
                type: 'string',
                description: 'The name of the MCP server to call.',
            },
            tool_name: {
                type: 'string',
                description: 'The name of the tool on the MCP server.',
            },
            arguments: {
                type: 'object',
                description: 'Arguments to pass to the MCP tool.',
            },
        },
        required: ['server_name', 'tool_name'],
    },
    execute: async (input, _ctx) => {
        try {
            const result = await invoke<any>('mcp_call_tool', {
                serverName: input.server_name,
                toolName: input.tool_name,
                arguments: input.arguments || {},
            });
            return ok(result);
        } catch (e: any) {
            return fail(`MCP call failed: ${e.message || e}`);
        }
    },
};

// ---------------------------------------------------------------------------
// 20. NotebookEditTool — Edit Jupyter notebooks
// ---------------------------------------------------------------------------
export const NotebookEditTool: ToolDef = {
    name: 'notebook_edit',
    description: `Edit a Jupyter notebook (.ipynb file). Can insert, replace, or delete cells.`,
    inputSchema: {
        type: 'object',
        properties: {
            file_path: {
                type: 'string',
                description: 'Path to the .ipynb notebook file.',
            },
            operation: {
                type: 'string',
                description: 'The operation to perform.',
                enum: ['insert_cell', 'replace_cell', 'delete_cell'],
            },
            cell_index: {
                type: 'number',
                description: 'The index of the cell to operate on (0-indexed).',
            },
            cell_type: {
                type: 'string',
                description: 'The type of cell.',
                enum: ['code', 'markdown'],
            },
            content: {
                type: 'string',
                description: 'The content of the cell.',
            },
        },
        required: ['file_path', 'operation'],
    },
    execute: async (input, _ctx) => {
        try {
            // Read notebook, modify, write back
            const raw = await invoke<string>('read_file', { path: input.file_path });
            const notebook = JSON.parse(raw);
            const cells = notebook.cells || [];

            switch (input.operation) {
                case 'insert_cell': {
                    const newCell = {
                        cell_type: input.cell_type || 'code',
                        source: (input.content || '').split('\n').map((l: string, i: number, arr: string[]) =>
                            i < arr.length - 1 ? l + '\n' : l
                        ),
                        metadata: {},
                        outputs: [],
                    };
                    const idx = input.cell_index ?? cells.length;
                    cells.splice(idx, 0, newCell);
                    break;
                }
                case 'replace_cell': {
                    if (input.cell_index === undefined || input.cell_index >= cells.length) {
                        return fail('Invalid cell_index');
                    }
                    cells[input.cell_index].source = (input.content || '').split('\n').map((l: string, i: number, arr: string[]) =>
                        i < arr.length - 1 ? l + '\n' : l
                    );
                    if (input.cell_type) cells[input.cell_index].cell_type = input.cell_type;
                    break;
                }
                case 'delete_cell': {
                    if (input.cell_index === undefined || input.cell_index >= cells.length) {
                        return fail('Invalid cell_index');
                    }
                    cells.splice(input.cell_index, 1);
                    break;
                }
            }

            notebook.cells = cells;
            await invoke('write_file_content', {
                path: input.file_path,
                content: JSON.stringify(notebook, null, 1),
            });
            return ok({ filePath: input.file_path, operation: input.operation });
        } catch (e: any) {
            return fail(`Notebook edit failed: ${e.message || e}`);
        }
    },
};

// ---------------------------------------------------------------------------
// 21. CreateDirectoryTool — Create directories
// ---------------------------------------------------------------------------
export const CreateDirectoryTool: ToolDef = {
    name: 'create_directory',
    description: `Create a new directory (and any necessary parent directories).`,
    inputSchema: {
        type: 'object',
        properties: {
            path: { type: 'string', description: 'Absolute path for the new directory.' },
        },
        required: ['path'],
    },
    execute: async (input, _ctx) => {
        try {
            await invoke('create_dir', { path: input.path });
            return ok({ path: input.path, created: true });
        } catch (e: any) {
            return fail(`Failed to create directory: ${e.message || e}`);
        }
    },
};

// ---------------------------------------------------------------------------
// 22. SkillTool — Execute a skill/workflow
// ---------------------------------------------------------------------------
export const SkillTool: ToolDef = {
    name: 'skill_execute',
    description: `Execute a predefined skill or workflow. Skills are reusable task templates stored in .agent/skills/ directories.`,
    inputSchema: {
        type: 'object',
        properties: {
            skill_name: {
                type: 'string',
                description: 'Name of the skill to execute.',
            },
            arguments: {
                type: 'string',
                description: 'Arguments to pass to the skill.',
            },
        },
        required: ['skill_name'],
    },
    execute: async (input, ctx) => {
        try {
            const skillPath = `${ctx.activeRoot}/.agent/skills/${input.skill_name}/SKILL.md`;
            const content = await invoke<string>('read_file', { path: skillPath });
            return ok({ skill: input.skill_name, instructions: content });
        } catch (e: any) {
            return fail(`Skill not found: ${input.skill_name}`);
        }
    },
};

// ---------------------------------------------------------------------------
// 23. SendMessageTool — Inter-agent messaging
// ---------------------------------------------------------------------------
export const SendMessageTool: ToolDef = {
    name: 'send_message',
    description: `Send a message to another agent or to the user. Used for inter-agent communication in multi-agent workflows.`,
    inputSchema: {
        type: 'object',
        properties: {
            recipient: {
                type: 'string',
                description: 'The recipient agent ID or "user" for the end user.',
            },
            message: {
                type: 'string',
                description: 'The message content.',
            },
        },
        required: ['recipient', 'message'],
    },
    execute: async (input, _ctx) => {
        return ok({
            sent: true,
            recipient: input.recipient,
            message: input.message,
            timestamp: Date.now(),
        });
    },
};

// =============================================================================
// TOOL REGISTRY
// =============================================================================

const ALL_TOOLS: ToolDef[] = [
    // Core file operations
    BashTool,
    FileReadTool,
    FileWriteTool,
    FileEditTool,
    GlobTool,
    GrepTool,
    ListDirectoryTool,

    // Web operations
    WebFetchTool,
    WebSearchTool,

    // Browser automation
    BrowserOpenTool,
    BrowserNavigateTool,
    BrowserScreenshotTool,
    BrowserCloseTool,

    // Task management
    TodoWriteTool,
    TaskCreateTool,
    TaskUpdateTool,

    // Planning
    EnterPlanModeTool,
    ExitPlanModeTool,

    // Multi-agent
    SpawnSubAgentTool,
    BrowserSubAgentTool,
    AskUserQuestionTool,
    SendMessageTool,

    // Git operations
    GitStatusTool,
    GitDiffTool,
    GitCommitTool,
    GitAddTool,
    GitLogTool,

    // Terminal
    TerminalSendTool,
    TerminalReadTool,

    // Advanced
    SystemHealthTool,
    MCPTool,
    NotebookEditTool,
    CreateDirectoryTool,
    SkillTool,
];

// ---------------------------------------------------------------------------
// Registry API
// ---------------------------------------------------------------------------

export function getAllTools(): ToolDef[] {
    return ALL_TOOLS.filter(t => !t.isEnabled || t.isEnabled());
}

export function getToolByName(name: string): ToolDef | undefined {
    return ALL_TOOLS.find(t => t.name === name);
}

export function getToolSchemas(): any[] {
    return getAllTools().map(tool => ({
        type: 'function',
        function: {
            name: tool.name,
            description: tool.description,
            parameters: tool.inputSchema,
        },
    }));
}

/**
 * Convert tool schemas to Anthropic format (for Claude API)
 */
export function getToolSchemasAnthropic(): any[] {
    return getAllTools().map(tool => ({
        name: tool.name,
        description: tool.description,
        input_schema: tool.inputSchema,
    }));
}

/**
 * Convert tool schemas to Google/Gemini format
 */
export function getToolSchemasGoogle(): any[] {
    return getAllTools().map(tool => ({
        name: tool.name,
        description: tool.description,
        parameters: {
            ...tool.inputSchema,
            // Google requires explicit property ordering
        },
    }));
}

/**
 * Execute a tool call by name
 */
export async function executeToolCall(
    toolCall: ToolCall,
    ctx: ToolContext,
): Promise<ToolCallResult> {
    const tool = getToolByName(toolCall.name);
    if (!tool) {
        return {
            tool_call_id: toolCall.id,
            content: `Error: Unknown tool "${toolCall.name}". Available tools: ${getAllTools().map(t => t.name).join(', ')}`,
        };
    }

    try {
        const result = await tool.execute(toolCall.arguments, ctx);
        const content = result.success
            ? (typeof result.data === 'string' ? result.data : JSON.stringify(result.data, null, 2))
            : `Error: ${result.error}`;
        return {
            tool_call_id: toolCall.id,
            content,
        };
    } catch (e: any) {
        return {
            tool_call_id: toolCall.id,
            content: `Tool execution error: ${e.message || e}`,
        };
    }
}
