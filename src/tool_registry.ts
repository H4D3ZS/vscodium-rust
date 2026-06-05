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
    description?: string;
    enum?: string[];
    items?: any;
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
    description: `NATIVE CORE TOOL: Read the contents of a file from the filesystem. This tool is ALWAYS available on all platforms (Windows/Linux/OSX). Supports text files, showing content with line numbers. For large files, use offset and limit to read specific portions. The file_path must be an absolute path.`,
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
            // Read current content
            const content = await invoke<string>('read_file', { path: input.file_path });
            if (content === null || content === undefined) return fail(`File not found: ${input.file_path}`);

            // Exact string match replacement
            const oldStr: string = input.old_string;
            if (!content.includes(oldStr)) {
                // Try normalized whitespace match as fallback
                return fail(`Could not find exact match for old_string in ${input.file_path}. Ensure the text matches exactly including indentation and whitespace. Read the file first to get the exact text.`);
            }

            const newContent = input.replace_all
                ? content.split(oldStr).join(input.new_string)
                : content.replace(oldStr, input.new_string);

            await invoke('write_file_content', { path: input.file_path, content: newContent });
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
    description: `NATIVE CORE TOOL: Find files matching a glob pattern using high-performance backend indexing. Use this to discover files in the project instantly (e.g., "**/*.ts"). This tool is ALWAYS available on Windows. Results are relative paths.`,
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
    description: `NATIVE CORE TOOL: Search for text patterns in files using ultra-fast ripgrep patterns. Returns matching lines with file paths and line numbers. ALWAYS available and optimized for Windows. Supports regex.`,
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
    description: `Launch the external stealth-Firefox window (visible OS window — user watches live). Agent drives this same instance via browser_navigate / browser_screenshot. NOT an in-IDE iframe.`,
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
            const result = await invoke<any>('spawn_subagent', {
                task: input.task,
                working_directory: input.working_directory || ctx.activeRoot,
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
            const result = await invoke<any>('browser_subagent', {
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
    execute: async (_input, ctx) => {
        try {
            const result = await invoke<any>('git_status', {
                path: ctx.activeRoot,
            });
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
    execute: async (input, ctx) => {
        try {
            if (input.file_path) {
                const result = await invoke<string>('git_diff_file', {
                    path: ctx.activeRoot,
                    file_path: input.file_path,
                });
                return ok(result || 'No changes detected.');
            }

            // Backend git_diff is commit-hash based; use shell command for working-tree diff.
            const cmd = input.staged ? 'git diff --staged' : 'git diff';
            const result = await invoke<string>('ai_execute_command', {
                command: cmd,
                cwd: ctx.activeRoot,
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
    execute: async (input, ctx) => {
        try {
            const files: string[] = Array.isArray(input.files) ? input.files : [];
            if (files.length === 0) {
                return fail('git_add requires at least one file path.');
            }

            if (files.includes('.')) {
                await invoke<string>('ai_execute_command', {
                    command: 'git add .',
                    cwd: ctx.activeRoot,
                });
                return ok('Staged all changes.');
            }

            for (const filePath of files) {
                await invoke('git_stage', {
                    path: ctx.activeRoot,
                    file_path: filePath,
                });
            }
            return ok(`Staged ${files.length} file(s).`);
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
    execute: async (input, ctx) => {
        try {
            await invoke('git_commit', {
                path: ctx.activeRoot,
                message: input.message,
            });
            return ok('Commit created successfully.');
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
    execute: async (input, ctx) => {
        try {
            const history = await invoke<any[]>('get_git_history', { path: ctx.activeRoot });
            const limit = input.limit || 10;
            const result = (history || [])
                .slice(0, limit)
                .map((c: any) => `${c.hash?.slice(0, 7) || 'unknown'} ${c.message || ''} (${c.author || 'unknown'})`)
                .join('\n');
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
                terminal_id: input.terminal_id || undefined,
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
                id: input.terminal_id,
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
                server_name: input.server_name,
                tool_name: input.tool_name,
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
    description: `Execute a predefined skill or workflow. Skills live in .agent/skills/ or Claude-Red/Skills/ (e.g. offensive-sqli, offensive-xss, offensive-ssrf, offensive-active-directory). Auto-loaded in Bug Bounty mode; call explicitly for deep methodology.`,
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
        const name = String(input.skill_name || '').trim();
        const candidates = [
            `${ctx.activeRoot}/.agent/skills/${name}/SKILL.md`,
            `${ctx.activeRoot}/Claude-Red/Skills/${name.replace(/^offensive-/, '')}/SKILL.md`,
        ];
        // Claude-Red nested layout: Skills/{category}/{skill-name}/SKILL.md
        if (name.startsWith('offensive-')) {
            candidates.push(`${ctx.activeRoot}/Claude-Red/Skills/**/${name}/SKILL.md`);
        }
        for (const skillPath of candidates) {
            if (skillPath.includes('**')) continue;
            try {
                const content = await invoke<string>('read_file', { path: skillPath });
                return ok({ skill: name, path: skillPath, instructions: content });
            } catch {
                /* try next */
            }
        }
        // Search Claude-Red manifest path
        try {
            const manifestRaw = await invoke<string>('read_file', {
                path: `${ctx.activeRoot}/Claude-Red/claude-skills.json`,
            });
            const manifest = JSON.parse(manifestRaw);
            const entry = (manifest.skills || []).find((s: any) => s.name === name);
            if (entry?.path) {
                const skillPath = `${ctx.activeRoot}/Claude-Red/${entry.path}`;
                const content = await invoke<string>('read_file', { path: skillPath });
                return ok({ skill: name, path: skillPath, instructions: content });
            }
        } catch {
            /* fall through */
        }
        return fail(`Skill not found: ${name}. Try Claude-Red names like offensive-sqli, offensive-xss, offensive-ssrf.`);
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

// ---------------------------------------------------------------------------
// 24. TaskBoundaryTool — Signal task progress
// ---------------------------------------------------------------------------
export const TaskBoundaryTool: ToolDef = {
    name: 'task_boundary',
    description: `Signal the start or update of a task. Use this to report your progress to the user through the IDE's Progress Updates UI. VERY IMPORTANT: Use this tool to keep the user informed of exactly what you are doing in Sentient mode.`,
    inputSchema: {
        type: 'object',
        properties: {
            TaskName: { type: 'string', description: 'Human readable name of the overarching task' },
            Mode: { type: 'string', description: 'PLANNING, EXECUTION, or VERIFICATION' },
            TaskSummary: { type: 'string', description: 'Concise summary of accomplished work' },
            TaskStatus: { type: 'string', description: 'What you are going to do next' },
            PredictedTaskSize: { type: 'number', description: 'Estimated steps remaining' }
        },
        required: ['TaskName', 'Mode', 'TaskSummary', 'TaskStatus', 'PredictedTaskSize'],
    },
    execute: async (input, _ctx) => {
        try {
            const store = (window as any).useStore?.getState();
            if (store && store.updateAgentTask) {
                const progress = Math.max(10, 100 - (input.PredictedTaskSize * 5));
                store.updateAgentTask({
                    id: 'current-mission',
                    title: input.TaskName,
                    summary: input.TaskSummary,
                    status: 'running',
                    progress: progress,
                    mode: input.Mode,
                    task_status: input.TaskStatus,
                    updatedAt: Date.now()
                });
                store.addAgentStep(`[${input.Mode}] ${input.TaskStatus}`, 'running');
            }
            return ok({ status: 'Task boundary updated successfully in the UI.' });
        } catch (e: any) {
            return fail(`Task boundary failed: ${e.message || e}`);
        }
    },
};

// ---------------------------------------------------------------------------
// 25. ReplaceFileContentTool — Precise multi-line edits
// ---------------------------------------------------------------------------
export const ReplaceFileContentTool: ToolDef = {
    name: 'replace_file_content',
    description: `Precisely edit a file by replacing a specific line range with new content. Preferred over file_edit for complex code modifications.`,
    inputSchema: {
        type: 'object',
        properties: {
            path: { type: 'string', description: 'Absolute or relative path to the file' },
            StartLine: { type: 'number', description: '1-indexed starting line' },
            EndLine: { type: 'number', description: '1-indexed ending line' },
            ReplacementContent: { type: 'string', description: 'The new content to drop in' },
            Instruction: { type: 'string', description: 'Description of changes' }
        },
        required: ['path', 'StartLine', 'EndLine', 'ReplacementContent'],
    },
    execute: async (input, ctx) => {
        try {
            const fullPath = input.path;
            const content = await invoke<string>('read_file', { path: fullPath });
            if (content === null || content === undefined) return fail(`File not found: ${fullPath}`);

            const lines = content.split('\n');
            const start = Math.max(0, input.StartLine - 1);
            const end = Math.min(lines.length, input.EndLine);

            lines.splice(start, end - start, ...input.ReplacementContent.split('\n'));
            const newContent = lines.join('\n');

            await invoke('write_file_content', {
                path: fullPath,
                content: newContent,
            });

            return ok({ filePath: fullPath, type: 'replaced_lines' });
        } catch (e: any) {
            return fail(`Replace failed: ${e.message || e}`);
        }
    },
};

// ---------------------------------------------------------------------------
// 26. MultiReplaceFileContentTool — Multiple edits in one file
// ---------------------------------------------------------------------------
export const MultiReplaceFileContentTool: ToolDef = {
    name: 'multi_replace_file_content',
    description: `Perform multiple non-contiguous line-based replacements in a single file.`,
    inputSchema: {
        type: 'object',
        properties: {
            path: { type: 'string', description: 'Path to the file' },
            Replacements: {
                type: 'array',
                items: {
                    type: 'object',
                    properties: {
                        StartLine: { type: 'number' },
                        EndLine: { type: 'number' },
                        ReplacementContent: { type: 'string' }
                    },
                    required: ['StartLine', 'EndLine', 'ReplacementContent']
                }
            }
        },
        required: ['path', 'Replacements'],
    },
    execute: async (input, ctx) => {
        try {
            const fullPath = input.path;
            const content = await invoke<string>('read_file', { path: fullPath });
            if (content === null || content === undefined) return fail(`File not found: ${fullPath}`);

            let lines = content.split('\n');
            // Sort replacements in reverse order by StartLine to avoid shifting indices
            const sortedReplacements = [...input.Replacements].sort((a, b) => b.StartLine - a.StartLine);

            for (const rep of sortedReplacements) {
                const start = Math.max(0, rep.StartLine - 1);
                const end = Math.min(lines.length, rep.EndLine);
                lines.splice(start, end - start, ...rep.ReplacementContent.split('\n'));
            }

            const newContent = lines.join('\n');
            await invoke('write_file_content', {
                path: fullPath,
                content: newContent,
            });

            return ok({ filePath: fullPath, type: 'multi_replaced', numEdits: input.Replacements.length });
        } catch (e: any) {
            return fail(`Multi-replace failed: ${e.message || e}`);
        }
    },
};

// ---------------------------------------------------------------------------
// 27. SpecsToCodePipelineTool — Autonomous Project Assembly
// ---------------------------------------------------------------------------
export const SpecsToCodePipelineTool: ToolDef = {
    name: 'specs_to_code_pipeline',
    description: `Kick off a comprehensive, multi-phase background process to generate a full project or feature based on technical specifications. 
    This tool performs: Analyzation -> Structure Architecture -> MVC Design -> Backend Implementation -> Frontend Stubbing.
    Use this for large-scale generations instead of manual step-by-step prompting.`,
    inputSchema: {
        type: 'object',
        properties: {
            projectName: { type: 'string', description: 'Descriptive name for the project folder' },
            specs: { type: 'string', description: 'Complete technical specifications in Markdown format' },
            provider: { type: 'string', description: 'Optional LLM provider (e.g., google, ollama:llama3)', default: 'google' }
        },
        required: ['projectName', 'specs'],
    },
    execute: async (input, _ctx) => {
        try {
            // 1. Create the project in the specs DB
            const projectId = await invoke<number>("cmd_specs_create_project", {
                name: input.projectName,
                specs: input.specs,
                provider: input.provider || 'google'
            });

            // 2. Open the specialized tracking UI
            const store = (window as any).useStore?.getState();
            if (store) {
                store.setCurrentSpecProjectId(projectId);
                store.setSpecsWizardStep('status');
                store.setSpecsWizardOpen(true);
            }

            // 3. Trigger initial layout generation
            invoke("cmd_specs_generate_layout", { project_id: projectId }).catch(console.error);

            return ok({
                status: 'Pipeline initialized successfully',
                projectId,
                message: 'Background workers are now assembling the project. The UI has been opened to track progress.'
            });
        } catch (e: any) {
            return fail(`Pipeline start failed: ${e.message || e}`);
        }
    },
};

// ---------------------------------------------------------------------------
// 28. AI Explain Code — Explain what code does in plain English
// ---------------------------------------------------------------------------
export const AIExplainCodeTool: ToolDef = {
    name: 'ai_explain_code',
    description: `Explain what a piece of code does in plain English. Analyzes the code and provides a clear explanation of its functionality, logic flow, and purpose. Use this to understand unfamiliar code or to document how something works.`,
    inputSchema: {
        type: 'object',
        properties: {
            file_path: {
                type: 'string',
                description: 'The absolute path to the file to explain.',
            },
            code_block: {
                type: 'string',
                description: 'Optional specific code block to explain. If not provided, explains the entire file or selected function.',
            },
            start_line: {
                type: 'number',
                description: 'Line to start explanation from.',
            },
            end_line: {
                type: 'number',
                description: 'Line to end explanation at.',
            },
            detail_level: {
                type: 'string',
                description: 'Explanation detail level.',
                enum: ['brief', 'medium', 'detailed'],
                default: 'medium',
            },
        },
        required: ['file_path'],
    },
    isReadOnly: true,
    execute: async (input, _ctx) => {
        try {
            let codeContent = input.code_block || '';
            
            if (!codeContent && input.file_path) {
                const content = await invoke<string>('read_file', { path: input.file_path });
                const lines = content.split('\n');
                const start = Math.max(0, (input.start_line || 1) - 1);
                const end = input.end_line ? Math.min(lines.length, input.end_line) : lines.length;
                codeContent = lines.slice(start, end).join('\n');
            }

            const result = await invoke<string>('ai_explain_code', {
                code: codeContent,
                filePath: input.file_path,
                detailLevel: input.detail_level || 'medium',
            });
            return ok({ explanation: result });
        } catch (e: any) {
            return fail(`Explain code failed: ${e.message || e}`);
        }
    },
};

// ---------------------------------------------------------------------------
// 29. AI Document Code — Generate documentation for code
// ---------------------------------------------------------------------------
export const AIDocumentCodeTool: ToolDef = {
    name: 'ai_document_code',
    description: `Automatically generate documentation for code including JSDoc comments, README files, or inline explanations. Creates professional documentation that explains functions, classes, parameters, and return values.`,
    inputSchema: {
        type: 'object',
        properties: {
            file_path: {
                type: 'string',
                description: 'The absolute path to the file to document.',
            },
            output_format: {
                type: 'string',
                description: 'Format for the documentation.',
                enum: ['jsdoc', 'inline', 'readme', 'markdown'],
                default: 'jsdoc',
            },
            language: {
                type: 'string',
                description: 'Programming language for proper documentation formatting.',
                default: 'typescript',
            },
        },
        required: ['file_path'],
    },
    execute: async (input, _ctx) => {
        try {
            const content = await invoke<string>('read_file', { path: input.file_path });
            const result = await invoke<string>('ai_document_code', {
                code: content,
                filePath: input.file_path,
                format: input.output_format || 'jsdoc',
                language: input.language || 'typescript',
            });
            return ok({ documentation: result });
        } catch (e: any) {
            return fail(`Document code failed: ${e.message || e}`);
        }
    },
};

// ---------------------------------------------------------------------------
// 30. AI Generate Code — Generate code from natural language
// ---------------------------------------------------------------------------
export const AIGenerateCodeTool: ToolDef = {
    name: 'ai_generate_code',
    description: `Generate new code from natural language description. Creates functional code based on your requirements including proper imports, error handling, and best practices. Use this to quickly scaffold new functions, classes, or entire files.`,
    inputSchema: {
        type: 'object',
        properties: {
            prompt: {
                type: 'string',
                description: 'Natural language description of what code to generate.',
            },
            language: {
                type: 'string',
                description: 'Programming language to generate.',
                default: 'typescript',
            },
            framework: {
                type: 'string',
                description: 'Optional framework context (e.g., react, vue, express).',
            },
            file_path: {
                type: 'string',
                description: 'Optional path where the generated code will be saved.',
            },
        },
        required: ['prompt'],
    },
    execute: async (input, _ctx) => {
        try {
            const result = await invoke<string>('ai_generate_code', {
                prompt: input.prompt,
                language: input.language || 'typescript',
                framework: input.framework,
                filePath: input.file_path,
            });
            
            if (input.file_path && result) {
                await invoke('write_file_content', {
                    path: input.file_path,
                    content: result,
                });
            }
            
            return ok({ generated_code: result, saved: !!input.file_path });
        } catch (e: any) {
            return fail(`Generate code failed: ${e.message || e}`);
        }
    },
};

// ---------------------------------------------------------------------------
// 31. AI Refactor — Refactor code for better quality
// ---------------------------------------------------------------------------
export const AIRefactorTool: ToolDef = {
    name: 'ai_refactor_code',
    description: `Refactor code to improve readability, performance, or follow best practices. Can extract functions, rename variables, simplify logic, or apply design patterns. Provides multiple refactoring options and explains each change.`,
    inputSchema: {
        type: 'object',
        properties: {
            file_path: {
                type: 'string',
                description: 'The absolute path to the file to refactor.',
            },
            start_line: {
                type: 'number',
                description: 'Line to start refactoring from.',
            },
            end_line: {
                type: 'number',
                description: 'Line to end refactoring at.',
            },
            refactor_type: {
                type: 'string',
                description: 'Type of refactoring to perform.',
                enum: ['extract_function', 'rename', 'simplify', 'performance', 'security', 'best_practices'],
            },
            target_name: {
                type: 'string',
                description: 'Target name for rename or extraction refactors.',
            },
        },
        required: ['file_path'],
    },
    execute: async (input, _ctx) => {
        try {
            const content = await invoke<string>('read_file', { path: input.file_path });
            const result = await invoke<string>('ai_refactor_code', {
                code: content,
                filePath: input.file_path,
                startLine: input.start_line,
                endLine: input.end_line,
                refactorType: input.refactor_type || 'best_practices',
                targetName: input.target_name,
            });

            if (result && input.file_path) {
                await invoke('write_file_content', {
                    path: input.file_path,
                    content: result,
                });
            }

            return ok({ refactored_code: result });
        } catch (e: any) {
            return fail(`Refactor failed: ${e.message || e}`);
        }
    },
};

// ---------------------------------------------------------------------------
// 32. AI Debug — Analyze and fix bugs
// ---------------------------------------------------------------------------
export const AIDebugTool: ToolDef = {
    name: 'ai_debug_code',
    description: `Analyze code for bugs, errors, and issues. Provides detailed diagnosis of problems along with fixed code. Can identify logic errors, race conditions, memory leaks, and security vulnerabilities.`,
    inputSchema: {
        type: 'object',
        properties: {
            file_path: {
                type: 'string',
                description: 'The absolute path to the file to debug.',
            },
            error_message: {
                type: 'string',
                description: 'Optional error message or stack trace to help diagnose the issue.',
            },
            start_line: {
                type: 'number',
                description: 'Line to start debugging from.',
            },
            end_line: {
                type: 'number',
                description: 'Line to end debugging at.',
            },
        },
        required: ['file_path'],
    },
    execute: async (input, _ctx) => {
        try {
            const content = await invoke<string>('read_file', { path: input.file_path });
            const result = await invoke<any>('ai_debug_code', {
                code: content,
                filePath: input.file_path,
                errorMessage: input.error_message,
                startLine: input.start_line,
                endLine: input.end_line,
            });

            if (result?.fixed_code && input.file_path) {
                await invoke('write_file_content', {
                    path: input.file_path,
                    content: result.fixed_code,
                });
            }

            return ok({
                diagnosis: result?.diagnosis,
                issues: result?.issues,
                fixed_code: result?.fixed_code,
                suggestions: result?.suggestions,
            });
        } catch (e: any) {
            return fail(`Debug failed: ${e.message || e}`);
        }
    },
};

// ---------------------------------------------------------------------------
// 33. Apply from Chat — Apply AI suggestions to code
// ---------------------------------------------------------------------------
export const ApplyFromChatTool: ToolDef = {
    name: 'apply_from_chat',
    description: `Apply code changes that were generated in the AI chat to the actual file. Takes the AI-generated code and writes it to the specified location. This is the core "Apply" feature that makes AI editing seamless.`,
    inputSchema: {
        type: 'object',
        properties: {
            file_path: {
                type: 'string',
                description: 'The absolute path to the file to apply changes to.',
            },
            code: {
                type: 'string',
                description: 'The code to write or replace with.',
            },
            mode: {
                type: 'string',
                description: 'How to apply the code.',
                enum: ['replace', 'insert_after', 'insert_before', 'patch'],
                default: 'replace',
            },
            start_line: {
                type: 'number',
                description: 'For replace/patch modes, the starting line.',
            },
            end_line: {
                type: 'number',
                description: 'For replace mode, the ending line.',
            },
        },
        required: ['file_path', 'code'],
    },
    execute: async (input, _ctx) => {
        try {
            let finalContent = input.code;

            if (input.mode === 'replace' && input.start_line && input.end_line) {
                const content = await invoke<string>('read_file', { path: input.file_path });
                const lines = content.split('\n');
                const start = Math.max(0, input.start_line - 1);
                const end = Math.min(lines.length, input.end_line);
                lines.splice(start, end - start, ...input.code.split('\n'));
                finalContent = lines.join('\n');
            } else if (input.mode === 'insert_after' && input.start_line) {
                const content = await invoke<string>('read_file', { path: input.file_path });
                const lines = content.split('\n');
                const insertIdx = Math.max(0, Math.min(lines.length, input.start_line - 1));
                lines.splice(insertIdx + 1, 0, ...input.code.split('\n'));
                finalContent = lines.join('\n');
            } else if (input.mode === 'insert_before' && input.start_line) {
                const content = await invoke<string>('read_file', { path: input.file_path });
                const lines = content.split('\n');
                const insertIdx = Math.max(0, input.start_line - 1);
                lines.splice(insertIdx, 0, ...input.code.split('\n'));
                finalContent = lines.join('\n');
            }

            await invoke('write_file_content', {
                path: input.file_path,
                content: finalContent,
            });

            return ok({ applied: true, filePath: input.file_path, mode: input.mode });
        } catch (e: any) {
            return fail(`Apply failed: ${e.message || e}`);
        }
    },
};

// ---------------------------------------------------------------------------
// 34. Multi-Cursor AI — AI-powered multi-location edits
// ---------------------------------------------------------------------------
export const MultiCursorAITool: ToolDef = {
    name: 'ai_multi_cursor_edit',
    description: `Perform AI-powered edits at multiple locations simultaneously. Similar to VS Code's multi-cursor but driven by AI — describe a pattern and AI finds all occurrences and edits them consistently.`,
    inputSchema: {
        type: 'object',
        properties: {
            file_path: {
                type: 'string',
                description: 'The absolute path to the file to edit.',
            },
            pattern: {
                type: 'string',
                description: 'Code pattern to find (can be a string or regex).',
            },
            replacement: {
                type: 'string',
                description: 'Replacement code or description of changes.',
            },
            match_scope: {
                type: 'string',
                description: 'Scope of matching.',
                enum: ['exact', 'similar', 'semantic'],
                default: 'exact',
            },
            apply: {
                type: 'boolean',
                description: 'If true, apply changes. If false, just preview.',
                default: true,
            },
        },
        required: ['file_path', 'pattern', 'replacement'],
    },
    execute: async (input, _ctx) => {
        try {
            const content = await invoke<string>('read_file', { path: input.file_path });
            
            const result = await invoke<any>('ai_multi_cursor_edit', {
                code: content,
                filePath: input.file_path,
                pattern: input.pattern,
                replacement: input.replacement,
                matchScope: input.match_scope || 'exact',
                apply: input.apply !== false,
            });

            if (result?.modified_code && input.apply !== false) {
                await invoke('write_file_content', {
                    path: input.file_path,
                    content: result.modified_code,
                });
            }

            return ok({
                matches: result?.matches || [],
                modified: result?.modified_code || content,
                preview_only: input.apply === false,
            });
        } catch (e: any) {
            return fail(`Multi-cursor edit failed: ${e.message || e}`);
        }
    },
};

// ---------------------------------------------------------------------------
// 35. Project Rules — Define project-specific AI behavior
// ---------------------------------------------------------------------------
export const ProjectRulesTool: ToolDef = {
    name: 'project_rules',
    description: `Define or retrieve project-specific rules that the AI should follow. Sets coding standards, conventions, and preferences that the agent uses when generating or editing code.`,
    inputSchema: {
        type: 'object',
        properties: {
            action: {
                type: 'string',
                description: 'Action to perform.',
                enum: ['get', 'set', 'list'],
                default: 'get',
            },
            rules: {
                type: 'string',
                description: 'Rules content to set (for set action).',
            },
            rules_file: {
                type: 'string',
                description: 'Path to rules file (defaults to .hades/rules.md).',
                default: '.hades/rules.md',
            },
        },
        required: ['action'],
    },
    execute: async (input, _ctx) => {
        try {
            if (input.action === 'get') {
                const rulesPath = input.rules_file || '.hades/rules.md';
                try {
                    const content = await invoke<string>('read_file', { path: rulesPath });
                    return ok({ rules: content, path: rulesPath });
                } catch {
                    return ok({ rules: '', path: rulesPath, exists: false });
                }
            } else if (input.action === 'set' && input.rules) {
                const rulesPath = input.rules_file || '.hades/rules.md';
                await invoke('write_file_content', {
                    path: rulesPath,
                    content: input.rules,
                });
                return ok({ saved: true, path: rulesPath });
            } else if (input.action === 'list') {
                return ok({
                    rules_files: [
                        '.hades/rules.md',
                        '.hades/patterns.md',
                        '.hades/decisions.md',
                        'CLAUDE.md',
                        'AGENTS.md',
                    ],
                });
            }
            return fail('Invalid action');
        } catch (e: any) {
            return fail(`Project rules failed: ${e.message || e}`);
        }
    },
};

// ---------------------------------------------------------------------------
// 36. PR AI Review — AI-powered code review
// ---------------------------------------------------------------------------
export const PRAIReviewTool: ToolDef = {
    name: 'ai_pr_review',
    description: `Perform an AI-powered review of a Pull Request or code changes. Analyzes the diff, identifies potential issues, suggests improvements, and provides a comprehensive review report. Similar to GitHub Copilot's PR review.`,
    inputSchema: {
        type: 'object',
        properties: {
            pr_url: {
                type: 'string',
                description: 'URL of the Pull Request to review.',
            },
            diff_content: {
                type: 'string',
                description: 'Alternative: raw diff content to review.',
            },
            focus_areas: {
                type: 'array',
                items: { type: 'string' },
                description: 'Areas to focus on (security, performance, style, etc.).',
            },
        },
    },
    isReadOnly: true,
    execute: async (input, _ctx) => {
        try {
            const result = await invoke<any>('ai_pr_review', {
                prUrl: input.pr_url,
                diffContent: input.diff_content,
                focusAreas: input.focus_areas,
            });
            return ok(result);
        } catch (e: any) {
            return fail(`PR review failed: ${e.message || e}`);
        }
    },
};

// ---------------------------------------------------------------------------
// 37. Context Awareness — Get relevant codebase context
// ---------------------------------------------------------------------------
export const ContextAwarenessTool: ToolDef = {
    name: 'ai_get_context',
    description: `Retrieve relevant context from the codebase for the current task. Uses semantic search to find related files, functions, and patterns that are relevant to what you are working on. This provides the "knowledge" that makes AI coding assistants effective.`,
    inputSchema: {
        type: 'object',
        properties: {
            query: {
                type: 'string',
                description: 'What you are trying to accomplish or find context for.',
            },
            max_files: {
                type: 'number',
                description: 'Maximum number of relevant files to return.',
                default: 5,
            },
            include_types: {
                type: 'array',
                items: { type: 'string' },
                description: 'Types of context to include.',
            },
        },
        required: ['query'],
    },
    isReadOnly: true,
    execute: async (input, _ctx) => {
        try {
            const result = await invoke<any>('ai_get_context', {
                query: input.query,
                maxFiles: input.max_files || 5,
                includeTypes: input.include_types,
            });
            return ok(result);
        } catch (e: any) {
            return fail(`Context retrieval failed: ${e.message || e}`);
        }
    },
};

// ---------------------------------------------------------------------------
// 38. Kortex AIM Spans — compressed-map to exact-source bridge
// ---------------------------------------------------------------------------
export const AimQuerySpansTool: ToolDef = {
    name: 'aim_query_spans',
    description: `Query Kortex AIM VFS for exact source spans relevant to a task. Use this before broad grep/search in large workspaces. It returns bounded file/line windows with hashes so you can verify exact source after using the compressed AIM map.`,
    inputSchema: {
        type: 'object',
        properties: {
            query: {
                type: 'string',
                description: 'Natural language or symbol query describing the code area to locate.',
            },
            limit: {
                type: 'number',
                description: 'Maximum number of spans to return.',
                default: 8,
            },
            max_files: {
                type: 'number',
                description: 'Safety cap for how many candidate files AIM may scan.',
                default: 2000,
            },
        },
        required: ['query'],
    },
    isReadOnly: true,
    execute: async (input, ctx) => {
        try {
            const result = await invoke<any>('aim_query_spans', {
                request: {
                    query: input.query,
                    root: ctx.activeRoot || undefined,
                    limit: input.limit || 8,
                    max_files: input.max_files || 2000,
                },
            });
            return ok(result);
        } catch (e: any) {
            return fail(`AIM span query failed: ${e.message || e}`);
        }
    },
};

export const AimPackContextTool: ToolDef = {
    name: 'aim_pack_context',
    description: `Build a compact, provider-neutral Kortex AIM context packet for a task. This combines the AIM trust envelope with exact source spans, suitable for OpenAI, Anthropic, Gemini, Ollama, Qwen, DeepSeek, and WebUI routing.`,
    inputSchema: {
        type: 'object',
        properties: {
            query: {
                type: 'string',
                description: 'Task, bug, symbol, or feature area to pack context for.',
            },
            limit: {
                type: 'number',
                description: 'Maximum number of exact spans to include.',
                default: 6,
            },
        },
        required: ['query'],
    },
    isReadOnly: true,
    execute: async (input, ctx) => {
        try {
            const [trust, spans] = await Promise.all([
                invoke<any>('aim_trust_manifest', {
                    path: null,
                    root: ctx.activeRoot || null,
                }),
                invoke<any>('aim_query_spans', {
                    request: {
                        query: input.query,
                        root: ctx.activeRoot || undefined,
                        limit: input.limit || 6,
                        max_files: 1000,
                    },
                }),
            ]);

            const lines: string[] = [
                '## Kortex AIM Context Pack',
                `query: ${input.query}`,
                `trust: ${trust.status}; confidence=${Math.round((trust.confidence || 0) * 100)}%; dirty=${trust.git?.dirty_files ?? 0}; sha=${trust.sha256?.slice(0, 16) || 'missing'}`,
                `retrieval: ${spans.source || 'unknown'}; index_hits=${spans.index_hits ?? 0}; scanned=${spans.scanned_files ?? 0}`,
            ];
            if (trust.reasons?.length) {
                lines.push(`notes: ${trust.reasons.join('; ')}`);
            }
            lines.push('', 'Exact spans:');
            for (const span of spans.spans || []) {
                lines.push(
                    `- ${span.file}:${span.line_start}-${span.line_end} score=${span.score} hash=${String(span.hash || '').slice(0, 12)}`
                );
                lines.push('```');
                lines.push(String(span.snippet || '').slice(0, 1800));
                lines.push('```');
            }

            return ok({
                trust,
                spans,
                compact: lines.join('\n'),
            });
        } catch (e: any) {
            return fail(`AIM context pack failed: ${e.message || e}`);
        }
    },
};

// ---------------------------------------------------------------------------
// =============================================================================
// ANTIGRAVITY WORKFLOW TOOLS
// =============================================================================

const AgMarkTaskDoneTool: ToolDef = {
    name: 'ag_mark_task_done',
    description: 'Mark a task as complete in the spec tasks.md file. Call this after successfully implementing a task. Flips `- [ ] TASK-NNN` to `- [x] TASK-NNN`.',
    inputSchema: {
        type: 'object',
        properties: {
            tasks_path: { type: 'string', description: 'Absolute path to the tasks.md file containing the task.' },
            task_id: { type: 'string', description: 'Task ID string, e.g. "TASK-001".' },
        },
        required: ['tasks_path', 'task_id'],
    },
    execute: async ({ tasks_path, task_id }) => {
        try {
            await invoke('ag_mark_task_done', { tasksPath: tasks_path, taskId: task_id });
            return ok(`Task ${task_id} marked as complete in ${tasks_path}`);
        } catch (e: any) {
            return fail(`ag_mark_task_done failed: ${e.message || e}`);
        }
    },
};

const AgPhaseWrapTool: ToolDef = {
    name: 'ag_phase_wrap',
    description: 'After completing a task or phase, call this to write a Phase-Wrap entry to `.hades/state.md`. Summarize what was implemented, tests written, and any decisions made.',
    inputSchema: {
        type: 'object',
        properties: {
            root: { type: 'string', description: 'Project root directory.' },
            task_id: { type: 'string', description: 'Task ID that was just completed.' },
            notes: { type: 'string', description: 'Summary of what was implemented, tests written, and key decisions.' },
        },
        required: ['root', 'task_id', 'notes'],
    },
    execute: async ({ root, task_id, notes }) => {
        try {
            await invoke('ag_phase_wrap', { root, taskId: task_id, notes });
            return ok(`Phase-Wrap written to .hades/state.md for ${task_id}`);
        } catch (e: any) {
            return fail(`ag_phase_wrap failed: ${e.message || e}`);
        }
    },
};

const AgGetNextTaskTool: ToolDef = {
    name: 'ag_get_next_task',
    description: 'Get the next unchecked task from specs/*/tasks.md. Returns task details including ID, description, file reference, and phase.',
    inputSchema: {
        type: 'object',
        properties: {
            root: { type: 'string', description: 'Project root directory.' },
        },
        required: ['root'],
    },
    execute: async ({ root }) => {
        try {
            const task = await invoke<any>('ag_get_next_task', { root });
            if (!task) return ok('No pending tasks found. All tasks complete!');
            return ok(task);
        } catch (e: any) {
            return fail(`ag_get_next_task failed: ${e.message || e}`);
        }
    },
};

const AgListTasksTool: ToolDef = {
    name: 'ag_list_tasks',
    description: 'List all tasks (pending and completed) from all specs/*/tasks.md files in the project.',
    inputSchema: {
        type: 'object',
        properties: {
            root: { type: 'string', description: 'Project root directory.' },
        },
        required: ['root'],
    },
    execute: async ({ root }) => {
        try {
            const tasks = await invoke<any[]>('ag_list_all_tasks', { root });
            const pending = tasks.filter((t: any) => !t.done);
            const done = tasks.filter((t: any) => t.done);
            return ok({
                pending_count: pending.length,
                done_count: done.length,
                pending: pending.slice(0, 20),
                done: done.slice(0, 10),
            });
        } catch (e: any) {
            return fail(`ag_list_tasks failed: ${e.message || e}`);
        }
    },
};

// ---------------------------------------------------------------------------
// =============================================================================
// TOOL REGISTRY
// =============================================================================

const ALL_TOOLS: ToolDef[] = [
    // Core file operations
    BashTool,
    FileReadTool,
    FileWriteTool,
    FileEditTool,
    ReplaceFileContentTool,
    MultiReplaceFileContentTool,
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
    TaskBoundaryTool,
    TodoWriteTool,
    TaskCreateTool,
    TaskUpdateTool,
    SpecsToCodePipelineTool,

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

    // AI Code Tools (Cursor-like features)
    AIExplainCodeTool,
    AIDocumentCodeTool,
    AIGenerateCodeTool,
    AIRefactorTool,
    AIDebugTool,
    ApplyFromChatTool,
    MultiCursorAITool,
    ProjectRulesTool,
    PRAIReviewTool,
    AimQuerySpansTool,
    AimPackContextTool,
    ContextAwarenessTool,

    // Antigravity workflow tools
    AgMarkTaskDoneTool,
    AgPhaseWrapTool,
    AgGetNextTaskTool,
    AgListTasksTool,
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
