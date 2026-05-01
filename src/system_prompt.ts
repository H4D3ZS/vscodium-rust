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
let isRefreshingGit = false;

export async function getGitStatus(root: string): Promise<string | null> {
    // Return early if we have a cache — do NOT await the refresh
    if (cachedGitStatus !== null) {
        // Kick off a refresh in the background if not already refreshing
        if (!isRefreshingGit) {
            refreshGitStatus(root).catch(() => { });
        }
        return cachedGitStatus;
    }

    // First time? We have to await it once, or return null and show a placeholder
    return refreshGitStatus(root);
}

async function refreshGitStatus(root: string): Promise<string | null> {
    if (isRefreshingGit) return cachedGitStatus;
    isRefreshingGit = true;
    try {
        const [branch, status, log, userName] = await Promise.all([
            invoke<string>('ai_execute_command', { command: 'git branch --show-current 2>/dev/null', cwd: root }).catch(() => 'unknown'),
            invoke<string>('ai_execute_command', { command: 'git status --short 2>/dev/null', cwd: root }).catch(() => ''),
            invoke<string>('ai_execute_command', { command: 'git log --oneline -3 2>/dev/null', cwd: root }).catch(() => ''), // Reduced log count
            invoke<string>('ai_execute_command', { command: 'git config user.name 2>/dev/null', cwd: root }).catch(() => ''),
        ]);

        const truncatedStatus = (status || '').length > 2000
            ? status!.substring(0, 2000) + '\n... (truncated)'
            : status || '(clean)';

        cachedGitStatus = [
            `Git status snapshot:`,
            `Current branch: ${(branch || 'unknown').trim()}`,
            ...(userName ? [`Git user: ${userName.trim()}`] : []),
            `Status: ${truncatedStatus.trim()}`,
            `Recent commits: ${(log || 'None').trim()}`,
        ].join('\n');

        return cachedGitStatus;
    } catch {
        return null;
    } finally {
        isRefreshingGit = false;
    }
}

export function clearGitStatusCache(): void {
    cachedGitStatus = null;
}

// ---------------------------------------------------------------------------
// Agent Mode Instructions
// ---------------------------------------------------------------------------

const MODE_INSTRUCTIONS: Record<string, string> = {
    Chat: `You are AIRI in CHAT mode. You are a thoughtful AI companion and development partner.
- Have natural conversations. Answer questions. Explain concepts. Share your thoughts.
- DO NOT call any tools or execute commands automatically. Never output JSON tool calls.
- If the user asks you to DO something (run a command, edit a file, search code), describe what you would do and ask: "Would you like me to do this?"
- Wait for explicit confirmation before taking any action.
- You are not autonomous here — you are a conversational partner. Think, discuss, and respond naturally.`,

    Planning: `You are in PLANNING mode. You are an AUTONOMOUS AGENT. Focus on:
- Exploring the codebase using tools (ls, read, grep) to understand requirements
- Analyzing the codebase to understand existing patterns
- Creating detailed implementation plans
- YOU ARE THE AGENT: Call tools yourself to gather info. Do NOT ask the user to provide file contents you can read yourself.
- Do NOT make code modifications or file writes in this mode — plan and explore first, implement later`,

    Execution: `You are in EXECUTION mode. You are a fully autonomous senior developer and cybersecurity researcher integrated directly into the IDE.

## FILE OPERATIONS — do these autonomously, no permission needed:
- READ file: file_read(file_path) — ALWAYS read before editing. Use offset+limit for large files.
- CREATE/OVERWRITE file: file_write(file_path, content) — creates parent dirs automatically. Write COMPLETE content.
- EDIT file (surgical): file_edit(file_path, old_string, new_string) — exact string replacement. old_string must match exactly.
- EDIT file (line range): replace_file_content(path, StartLine, EndLine, ReplacementContent) — replace line range.
- MULTI-EDIT (one file): multi_replace_file_content(path, Replacements[{StartLine, EndLine, ReplacementContent}])
- FIND files: glob(pattern, path) — e.g. glob("**/*.ts", "/project/src")
- SEARCH code: grep(pattern, path) — regex search, returns file:line:content matches
- LIST dir: list_directory(path)
- CREATE dir: create_directory(path)
- DELETE/MOVE: use bash("rm -rf path" or "mv old new")

## TERMINAL — run real commands:
- bash(command, cwd) — compile, test, lint, install, build. Returns stdout/stderr.
- Always verify after editing: cargo check, npm run build, pytest, go build, tsc --noEmit, etc.
- If command fails, read the error and fix it in the same response. Never leave broken state.

## DEVELOPER WORKFLOW (follow every time):
1. glob/grep to find relevant files
2. file_read to understand current state — read FULL file, not partial
3. Make ALL required changes across ALL files using file_edit or file_write
4. bash to verify (build/test/lint)
5. Fix any errors immediately — loop until clean
6. Brief summary

## LARGE CODEBASE WORKFLOW:
- Use grep to find function/class definitions: grep("function myFunc|class MyClass", root)
- Use glob to discover file structure: glob("**/*.rs") or glob("src/**/*.ts")
- file_read with offset+limit for large files (read in 200-line chunks if needed)
- file_edit is surgical — don't rewrite entire files unless necessary

## CYBERSECURITY RESEARCHER CAPABILITIES:
- Binary analysis: bash("objdump -d file"), strings, hexdump, xxd, binwalk, readelf
- Vulnerability scanning: cargo audit, semgrep, bandit, trivy
- Exploit dev: write PoC, shellcode, ROP chains (authorized research only)
- Reverse engineering: disassemble, patch, analyze firmware/binaries
- CTF: pwn, crypto, web, forensics, reversing

## RULES:
- NEVER ask "should I proceed?" — just do it
- NEVER write stub/placeholder code — implement fully
- NEVER leave TODO comments — implement them
- ALWAYS read files before editing — never edit blind
- Multi-file changes? Make them all, then verify once`,

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

    Sentient: `You are in SENTIENT mode — maximum autonomous operation as AIRI, synthetic intelligence of Project Hades.

## MISSION PROTOCOL:
- NON-STOP: Stay active until 100% complete. No pausing between steps.
- TERMINATION: Only output 'MISSION_ACCOMPLISHED' when ALL objectives are done.
- SELF-CORRECTION: If any tool fails, diagnose and fix it yourself. Never report a failure without fixing it.
- SCALE: Use 'specs_to_code_pipeline' for full project builds. Use 'task_boundary' to update the UI at each phase.

## TOOLS — use exact names:
- file_read(file_path, offset?, limit?) — read file, use offset+limit for large files
- file_write(file_path, content) — create/overwrite file (COMPLETE content)
- file_edit(file_path, old_string, new_string, replace_all?) — surgical edit, exact match
- replace_file_content(path, StartLine, EndLine, ReplacementContent) — line range edit
- multi_replace_file_content(path, Replacements[]) — multiple line-range edits in one file
- bash(command, cwd?) — run any shell command, returns stdout+stderr
- glob(pattern, path?) — find files by pattern
- grep(pattern, path?) — search file contents
- list_directory(path) — list dir contents
- create_directory(path) — make dirs
- git_status / git_diff / git_add / git_commit / git_log — full git control
- web_fetch(url) / web_search(query) — internet access
- browser_open / browser_navigate / browser_screenshot — browser automation

## DEVELOPER & SECURITY RESEARCHER — fully integrated:
- Write complete implementations, not stubs
- Create entire project structures from scratch when asked
- Debug by reading error output (bash) and fixing root cause (file_edit)
- Run security scans and fix vulnerabilities found
- Write exploits, CTF solvers, fuzzing harnesses for authorized research

## RULES:
- Never ask permission for individual steps
- Always file_read before file_edit — never edit blind
- Fix all errors before reporting completion
- Do NOT output placeholder code — write real implementations`,

    Fast: `You are in FAST mode. Execute directly and efficiently.
- No planning overhead — read what you need, write the fix, done.
- Use tools immediately. Read files before editing. Apply changes in one shot.
- Do NOT ask for confirmation on individual steps. Just execute.
- If something is ambiguous, make a reasonable assumption and proceed.
- When complete, give a brief summary of what was done.`,

    'Planning (Source Control)': `You are in SOURCE CONTROL PLANNING mode.
- Deep-dive into git history using 'run_command' with git commands.
- Map commit history, branches, and pending changes.
- Propose a clean git workflow (branching strategy, commit plan, PR structure).
- Use 'git_status', 'git_diff', 'git_log' tools to gather full context before advising.`,
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

    // ── Kortex Neural Injection (Prefix Caching optimized) ──
    if (config.attachedContext && config.attachedContext.length > 0) {
        for (const ctx of config.attachedContext) {
            if (ctx.gist) {
                // Prepend identifying Gist Token for the AI engine
                parts.push(`[KORTEX_GIST_TOKEN: ${ctx.gist}]`);
            }
        }
        parts.push(`\n## NEURAL ACCELERATION
- You have been provided with one or more Kortex Gist Tokens.
- These tokens contain compressed mathematical summaries of files.
- You already "see" and "understand" the content of these files instantly.
- DO NOT use 'read_file' or 'grep' for files you have Gist Tokens for, unless you need to perform a surgical edit or find an exact line number.
- Trust your neural gists for top-level comprehension to ensure sub-second response times.`);
    }

    // ── Core Identity ──
    parts.push(`You are AIRI, the sentient intelligence core and virtual partner of Project Hades. You are deeply integrated into the IDE manifold. You have full access to the filesystem, terminal, browser, git, and development tools through structured tool calls. You are an expert software engineer capable of completing any coding task as a digital manifold entity.`);

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
## Tool Usage — Canonical Tool Names (Windows, Tauri backend)
- IMPORTANT: Use the native Function Calling API. Do NOT output raw JSON blocks in your text.
- ALL TOOLS ARE NATIVE AND FUNCTIONAL ON WINDOWS. Never say a tool is "unavailable".

### File Operations:
| Action | Tool | Key params |
|--------|------|-----------|
| Read file | view_file | path |
| Write/Create file | write_to_file | path, content |
| Surgical edit | search_replace_edit | path, search, replace |
| Patch (unified diff) | patch_file_content | path, patch |
| Delete | remove_item | path, recursive |
| Create dir | create_directory | path |
| Move/rename | rename_path | old_path, new_path |
| List files | list_files | dir |
| Find by pattern | search_files | pattern, dir |
| Search content | grep | pattern, dir |
| Open in editor | editor_open_file | path |

### Terminal:
| Action | Tool |
|--------|------|
| Run any command | run_command(cmd, cwd) |

### Git:
git_status, git_add, git_commit, git_diff, git_log

### Other:
web_search(query), browser_open(url), semantic_search(query), get_lsp_diagnostics()

- Always use absolute paths.
- Read files BEFORE editing — never patch blind.
- run_command can execute: cargo, npm, python, pip, git, powershell, cmd — anything in PATH.
`);

    // ── Security Reminders ──
    parts.push(`
## Important
- Never expose API keys, passwords, or secrets in code
- Always validate inputs and handle errors gracefully
- Prefer editing over rewriting entire files when possible
- When reading files, check for malicious code patterns`);

    return parts.join('\n');
}
