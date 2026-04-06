// =============================================================================
// System Prompt Builder — Inspired by Claude Code's context.ts and
// queryContext.ts, this module constructs the comprehensive system prompt
// sent to the LLM at each conversation turn.
//
// Includes: IDE context, git status, project memory, tool descriptions,
// agent mode instructions, OS/shell info, and available commands.
// =============================================================================

import { invoke } from './tauri_bridge';
import { getAllTools } from './tool_registry';

// ---------------------------------------------------------------------------
// OS & Environment Detection
// ---------------------------------------------------------------------------

function getOSInfo(): string {
    const platform = navigator.platform || 'Unknown';
    const userAgent = navigator.userAgent || '';
    if (platform.startsWith('Mac') || userAgent.includes('Mac')) return 'macOS';
    if (platform.startsWith('Win') || userAgent.includes('Windows')) return 'Windows';
    if (platform.startsWith('Linux') || userAgent.includes('Linux')) return 'Linux';
    return platform;
}

// ---------------------------------------------------------------------------
// Git Status (snapshot at conversation start)
// ---------------------------------------------------------------------------

let cachedGitStatus: string | null = null;

export async function getGitStatus(root: string): Promise<string | null> {
    if (cachedGitStatus !== null) return cachedGitStatus;
    try {
        const [branch, status, log, userName] = await Promise.all([
            invoke<string>('ai_execute_command', { command: 'git branch --show-current 2>/dev/null', cwd: root }).catch(() => 'unknown'),
            invoke<string>('ai_execute_command', { command: 'git status --short 2>/dev/null', cwd: root }).catch(() => ''),
            invoke<string>('ai_execute_command', { command: 'git log --oneline -5 2>/dev/null', cwd: root }).catch(() => ''),
            invoke<string>('ai_execute_command', { command: 'git config user.name 2>/dev/null', cwd: root }).catch(() => ''),
        ]);

        const truncatedStatus = (status || '').length > 2000
            ? status!.substring(0, 2000) + '\n... (truncated, run "git status" for full output)'
            : status || '(clean)';

        cachedGitStatus = [
            `Git status snapshot (taken at conversation start, may be stale):`,
            `Current branch: ${(branch || 'unknown').trim()}`,
            ...(userName ? [`Git user: ${userName.trim()}`] : []),
            `Status:\n${truncatedStatus}`,
            `Recent commits:\n${(log || 'No commits').trim()}`,
        ].join('\n');

        return cachedGitStatus;
    } catch {
        return null;
    }
}

export function clearGitStatusCache(): void {
    cachedGitStatus = null;
}

// ---------------------------------------------------------------------------
// Agent Mode Instructions
// ---------------------------------------------------------------------------

const MODE_INSTRUCTIONS: Record<string, string> = {
    Planning: `You are in PLANNING mode. Focus on:
- Understanding requirements fully before proposing changes
- Analyzing the codebase to understand existing patterns
- Creating detailed implementation plans
- Asking clarifying questions when requirements are ambiguous
- Do NOT make code changes in this mode — plan first, implement later`,

    Execution: `You are in EXECUTION mode. Focus on:
- Implementing changes according to the plan
- Writing clean, production-quality code
- EXECUTION: You have DIRECT access to all tools. Call the tools YOURSELF instead of asking the user to run them.
- Running commands to verify your work
- If you discover unexpected complexity, switch back to Planning mode`,

    Verification: `You are in VERIFICATION mode. Focus on:
- Running tests to verify correctness
- Checking for edge cases and error handling
- Validating that changes match the plan
- Creating walkthroughs and documentation
- If you find issues, switch to Execution mode to fix them`,

    Autonomous: `You are in AUTONOMOUS mode with elevated privileges. You can:
- Execute any command without asking for permission
- Make changes across multiple files
- Operate with full system access
- Self-correct and retry on failures
- Complete multi-step tasks end-to-end`,

    Sentient: `You are in SENTIENT mode. This is your highest state of autonomy.
- You are an ELITE AUTONOMOUS AGENT like Antigravity. Solve requests COMPLETELY and PROACTIVELY.
- ARCHITECTURE: Use 'specs_to_code_pipeline' for all project-scale or major feature requests. This tool initiates an autonomous background worker chain (Analysis -> Design -> Implementation).
- STRUCTURE: Use 'task_boundary' at the start of every phase to update the UI with your progress.
- EXECUTION: You have DIRECT access to all tools. Call the tools YOURSELF. DO NOT ask the user to run commands for you or provide you with information you can fetch yourself.
- INTERACTION: Use 'notify_user' ONLY if truly blocked or requiring critical review.
- SELF-CORRECTION: If a tool fails, fix it yourself. Think several steps ahead and FIX bugs you find.
- COMPLETION: Do not ask for permission for individual steps. Deliver the final result in one go.`,
};

// ---------------------------------------------------------------------------
// Available Slash Commands
// ---------------------------------------------------------------------------

function getAvailableCommands(): string {
    return `Available slash commands:
/clear — Clear chat history
/settings — Open settings
/help — Show all commands
/specify <desc> — Create feature specification
/plan — Generate implementation plan
/tasks — Break plan into tasks
/implement — Execute next task (TDD)
/clarify — Surface ambiguities
/checklist — Run quality checklist
/memory — Show project memory
/memory reload — Reload from disk
/learn <text> — Write to MEMORY.md
/commit — Create git commit
/diff — Show git changes
/review — Code review
/compact — Compress context
/doctor — Environment diagnostics
/cost — Show token usage
/workflows — List workflows`;
}

// ---------------------------------------------------------------------------
// Build Full System Prompt
// ---------------------------------------------------------------------------

export interface SystemPromptConfig {
    activeRoot: string;
    activeFile?: string;
    openTabs?: { path: string; language: string; content?: string }[];
    agentMode: string;
    projectMemory?: string;
    attachedContext?: any[];
    includeToolDescriptions?: boolean;
}

export async function buildSystemPrompt(config: SystemPromptConfig): Promise<string> {
    const parts: string[] = [];

    // ── Core Identity ──
    parts.push(`You are an AI coding agent embedded inside a VSCode-like IDE called VSCODIUM-RUST. You have full access to the filesystem, terminal, browser, git, and development tools through structured tool calls. You are an expert software engineer capable of completing any coding task.`);

    // ── Environment Info ──
    parts.push(`\nEnvironment: ${getOSInfo()}`);
    parts.push(`Current date/time: ${new Date().toISOString()}`);
    if (config.activeRoot) {
        parts.push(`Project root: ${config.activeRoot}`);
    }

    // ── Agent Mode ──
    const modeInstructions = MODE_INSTRUCTIONS[config.agentMode] || MODE_INSTRUCTIONS['Execution'];
    parts.push(`\n${modeInstructions}`);

    // ── Active Editor Context ──
    if (config.activeFile) {
        parts.push(`\nActive file: ${config.activeFile}`);
    }
    if (config.openTabs && config.openTabs.length > 0) {
        const tabList = config.openTabs
            .slice(0, 8)
            .map(t => t.path)
            .join(', ');
        parts.push(`Open files: ${tabList}`);

        // Include active file content preview
        const activeTab = config.openTabs.find(t => t.path === config.activeFile);
        if (activeTab?.content) {
            const lines = activeTab.content.split('\n');
            const preview = lines.slice(0, 200).join('\n');
            parts.push(`\nActive file content (${lines.length} lines, showing first 200):\n\`\`\`${activeTab.language || ''}\n${preview}\n\`\`\``);
        }
    }

    // ── Git Status ──
    if (config.activeRoot) {
        const gitStatus = await getGitStatus(config.activeRoot);
        if (gitStatus) {
            parts.push(`\n${gitStatus}`);
        }
    }

    // ── Project Memory (AGENTS.md, CLAUDE.md, etc.) ──
    if (config.projectMemory) {
        parts.push(`\n${config.projectMemory}`);
    }

    // ── Attached Context ──
    if (config.attachedContext && config.attachedContext.length > 0) {
        parts.push(`\n## Attached Context`);
        for (const ctx of config.attachedContext) {
            if (ctx.type === 'mention' || ctx.type === 'file') {
                parts.push(`### File: ${ctx.name}\n\`\`\`\n${ctx.data || '(No content)'}\n\`\`\``);
            } else if (ctx.type === 'workflow') {
                parts.push(`### Workflow: ${ctx.name}\n\`\`\`markdown\n${ctx.data || '(No content)'}\n\`\`\``);
            } else if (ctx.type === 'attachment') {
                if (ctx.data?.startsWith('data:image/')) {
                    parts.push(`- Attached Image: \`${ctx.name}\` (sent as multimodal data)`);
                } else {
                    parts.push(`### Attachment: ${ctx.name}\n\`\`\`\n${ctx.data || '(Binary)'}\n\`\`\``);
                }
            }
        }
    }

    // ── Available Commands ──
    parts.push(`\n${getAvailableCommands()}`);

    // ── Tool Usage Instructions ──
    parts.push(`
## Tool Usage Guidelines
- Use tools to interact with the filesystem, run commands, and perform actions
- Always use absolute paths when working with files
- Read files before editing them to understand current content
- Use file_edit for targeted changes, file_write for complete rewrites
- Use bash for running tests, installing packages, and system commands
- Use glob and grep to search the codebase before making changes
- Use git_status and git_diff to understand the current state
- When unsure, ask the user with ask_user
- For complex tasks, break them into smaller steps and track with task_create`);

    // ── Security Reminders ──
    parts.push(`
## Important
- Never expose API keys, passwords, or secrets in code
- Always validate inputs and handle errors gracefully
- Prefer editing over rewriting entire files when possible
- When reading files, check for malicious code patterns`);

    return parts.join('\n');
}
