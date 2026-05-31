# Claude Code — Tools System

## Table of Contents

1. [Tool Framework (Tool.ts)](#1-tool-framework)
2. [Tool Registry (tools.ts)](#2-tool-registry)
3. [Task Framework (Task.ts / tasks.ts)](#3-task-framework)
4. [Core File Tools](#4-core-file-tools)
   - [FileReadTool](#41-filereadtool)
   - [FileWriteTool](#42-filewritetool)
   - [FileEditTool](#43-fileedittool)
5. [Shell Execution Tools](#5-shell-execution-tools)
   - [BashTool](#51-bashtool)
   - [PowerShellTool](#52-powershelltool)
6. [Search Tools](#6-search-tools)
   - [GlobTool](#61-globtool)
   - [GrepTool](#62-greptool)
7. [Agent / Multi-Agent Tools](#7-agent--multi-agent-tools)
   - [AgentTool](#71-agenttool)
   - [TeamCreateTool](#72-teamcreatetool)
   - [TeamDeleteTool](#73-teamdeletetool)
   - [SendMessageTool](#74-sendmessagetool)
8. [Task Management Tools](#8-task-management-tools)
   - [TaskStopTool](#81-taskstoptool)
   - [TaskOutputTool](#82-taskoutputtool)
   - [TodoWriteTool (V1)](#83-todowritetool-v1)
   - [TaskCreateTool (V2)](#84-taskcreatetool-v2)
   - [TaskGetTool (V2)](#85-taskgettool-v2)
   - [TaskUpdateTool (V2)](#86-taskunpdatetool-v2)
   - [TaskListTool (V2)](#87-tasklisttool-v2)
9. [Web Tools](#9-web-tools)
   - [WebFetchTool](#91-webfetchtool)
   - [WebSearchTool](#92-websearchtool)
10. [MCP Integration Tools](#10-mcp-integration-tools)
    - [MCPTool](#101-mcptool)
    - [McpAuthTool](#102-mcpauthtool)
    - [ListMcpResourcesTool](#103-listmcpresourcestool)
    - [ReadMcpResourceTool](#104-readmcpresourcetool)
11. [Plan Mode Tools](#11-plan-mode-tools)
    - [EnterPlanModeTool](#111-enterplanmodetool)
    - [ExitPlanModeV2Tool](#112-exitplanmodev2tool)
12. [Notebook Tool](#12-notebook-tool)
13. [Worktree Tools](#13-worktree-tools)
    - [EnterWorktreeTool](#131-enterworkreetool)
    - [ExitWorktreeTool](#132-exitworkreetool)
14. [Scheduling Tools](#14-scheduling-tools)
    - [CronCreateTool](#141-croncreatetool)
    - [CronDeleteTool](#142-crondeletetool)
    - [CronListTool](#143-cronlisttool)
15. [Meta / Discovery Tools](#15-meta--discovery-tools)
    - [ToolSearchTool](#151-toolsearchtool)
    - [AskUserQuestionTool](#152-askuserquestiontool)
16. [Kairos / Special Mode Tools](#16-kairos--special-mode-tools)
    - [BriefTool (SendUserMessage)](#161-brieftool-senduserrmessage)
    - [SleepTool](#162-sleeptool)
    - [RemoteTriggerTool](#163-remotetriggertool)
17. [SDK / Output Tools](#17-sdk--output-tools)
    - [SyntheticOutputTool (StructuredOutput)](#171-syntheticoutputtool-structuredoutput)
18. [Skill Tool](#18-skill-tool)
19. [LSP Tool](#19-lsp-tool)
20. [REPL Tool](#20-repl-tool)
21. [Config Tool](#21-config-tool)
22. [Shared Utilities](#22-shared-utilities)
    - [tools/utils.ts](#221-toolsutilsts)
    - [tools/shared/gitOperationTracking.ts](#222-toolssharedgitoperationtrackingts)
    - [tools/shared/spawnMultiAgent.ts](#223-toolssharedspawnmultiagentts)
23. [Testing Utilities](#23-testing-utilities)

---

## 1. Tool Framework

**Source:** `src/Tool.ts`

### 1.1 Core Interface

```typescript
export type Tool<Input extends ZodType = ZodType, Output = unknown, Progress = unknown> = {
  // Identity
  name: string
  isMcp?: boolean
  mcpInfo?: { serverName: string; toolName: string }
  isLsp?: boolean
  alwaysLoad?: boolean
  shouldDefer?: boolean

  // Schema (getter properties for lazy init)
  readonly inputSchema: Input
  readonly outputSchema?: ZodType<Output>

  // Metadata
  description(): Promise<string>
  prompt(): Promise<string>
  userFacingName(input?: z.infer<Input>): string
  maxResultSizeChars?: number
  searchHint?: string

  // Capability flags (accept input for per-call decisions)
  isEnabled(permissionContext?: ToolPermissionContext): boolean
  isConcurrencySafe(input?: z.infer<Input>): boolean
  isReadOnly(input?: z.infer<Input>): boolean
  isDestructive?(input: z.infer<Input>): boolean
  toAutoClassifierInput(input: z.infer<Input>): string
  isSearchOrReadCommand?: (input: z.infer<Input>) => { isSearch: boolean; isRead: boolean }

  // Execution
  validateInput?(input: z.infer<Input>): Promise<ValidationResult>
  checkPermissions(input: z.infer<Input>, context: ToolUseContext): Promise<PermissionDecision>
  call(input: z.infer<Input>, context: ToolUseContext): Promise<ToolResult<Output>>

  // UI rendering (React / Ink)
  renderToolUseMessage(input: z.infer<Input>, options: RenderOptions): ReactNode | null
  renderToolUseProgressMessage?(progress: Progress, input?: z.infer<Input>): ReactNode | null
  renderToolUseQueuedMessage?(input: z.infer<Input>): ReactNode | null
  renderToolUseRejectedMessage?(input: z.infer<Input>, ...): ReactNode | null
  renderToolResultMessage(output: Output, ...): ReactNode | null
  renderToolUseErrorMessage?(error: Error, ...): ReactNode | null

  // Output mapping
  mapToolResultToToolResultBlockParam(
    output: Output,
    toolUseID: string,
    context: ToolUseContext,
  ): ToolResultBlockParam

  // Path tracking (for permission rules)
  getPath?(input?: z.infer<Input>): string | undefined
}
```

### 1.2 ToolDef — Definition Shape

`ToolDef<Input, Output, Progress>` is the shape passed to `buildTool()`. It has the same fields as `Tool` minus the defaults filled in by `buildTool()`.

### 1.3 buildTool()

```typescript
function buildTool<Input, Output, Progress>(def: ToolDef<Input, Output, Progress>): Tool<...>
```

Fills in safe defaults:
- `isEnabled` → `() => true`
- `isConcurrencySafe` → `() => false`
- `isReadOnly` → `() => false`
- `checkPermissions` → `async () => ({ behavior: 'allow', updatedInput: input })`
- `toAutoClassifierInput` → `() => ''`
- `userFacingName` → `() => def.name`

### 1.4 ToolUseContext

The context object passed to every `call()` and `checkPermissions()`:

```typescript
type ToolUseContext = {
  // Configuration
  options: {
    commands: Command[]
    tools: Tool[]
    mcpClients: MCPClient[]
    mainLoopModel: string
    thinkingConfig: ThinkingConfig
    // ... additional options
  }

  // Abort signal
  abortController: AbortController

  // App state accessors
  getAppState(): AppState
  setAppState(fn: (prev: AppState) => AppState): void

  // File read tracking (for read-before-write enforcement)
  readFileState: Map<string, { mtime: number; content: string }>

  // Permission context
  permissionContext: ToolPermissionContext

  // UI injection
  setToolJSX: SetToolJSXFn

  // Callbacks
  onPermissionRequest(request: PermissionRequest): Promise<PermissionDecision>
  onToolCallStart(toolName: string, input: unknown): void
  onToolCallEnd(toolName: string, result: unknown): void

  // Agent/teammate context
  agentId?: AgentId
  isSubagent?: boolean
  isCoordinator?: boolean

  // Additional fields for specific tool categories
  globLimits?: { maxResults: number }
}
```

### 1.5 ToolPermissionContext

```typescript
type ToolPermissionContext = {
  mode: PermissionMode  // 'default' | 'plan' | 'auto' | 'bypassPermissions' | 'acceptEdits'
  alwaysAllow: PermissionRule[]
  alwaysDeny: PermissionRule[]
  alwaysAsk: PermissionRule[]
  additionalWorkingDirectories: string[]
  toolPermissions: Record<string, ToolPermissionOverride>
}
```

### 1.6 PermissionResult / PermissionDecision

```typescript
type PermissionDecision =
  | { behavior: 'allow'; updatedInput: Input }
  | { behavior: 'ask'; message: string; decisionReason?: string }
  | { behavior: 'deny'; message: string }
  | { behavior: 'passthrough' }  // Always asks user

type ValidationResult =
  | { result: true }
  | { result: false; message: string; errorCode?: number }
```

### 1.7 ToolResult

```typescript
type ToolResult<T> = { data: T }
```

### 1.8 Tools Type Alias & Helpers

```typescript
type Tools = Tool[]

function findToolByName(tools: Tools, name: string): Tool | undefined
function toolMatchesName(tool: Tool, name: string): boolean
```

---

## 2. Tool Registry

**Source:** `src/tools.ts`

### 2.1 getAllBaseTools()

Returns the full ordered list of built-in tools. Order must stay in sync with Statsig caching config.

```typescript
function getAllBaseTools(): Tool[]
```

Includes (conditionally):
- Always: BashTool, GlobTool, GrepTool, FileReadTool, FileEditTool, FileWriteTool, AgentTool, WebFetchTool, WebSearchTool, NotebookEditTool, TodoWriteTool, TaskStopTool, AskUserQuestionTool, SkillTool, MCPTool, EnterPlanModeTool, ExitPlanModeV2Tool, ToolSearchTool, TaskCreateTool, TaskGetTool, TaskUpdateTool, TaskListTool, TeamCreateTool, TeamDeleteTool, SendMessageTool, TaskOutputTool, SyntheticOutputTool, EnterWorktreeTool, ExitWorktreeTool, BriefTool, RemoteTriggerTool
- Ant-only: ConfigTool, TungstenTool, REPLTool
- Feature-gated (`feature('KAIROS')` + `isKairosCronEnabled()`): CronCreateTool, CronDeleteTool, CronListTool
- Feature-gated (`feature('AGENT_TRIGGERS')`): RemoteTriggerTool
- Feature-gated (`feature('SLEEP_TOOL')`): SleepTool
- Feature-gated (`feature('MONITOR_TOOL')`): MonitorMcpTask
- LSP enabled: LSPTool

### 2.2 getTools(permissionContext)

```typescript
function getTools(permissionContext: ToolPermissionContext): Tool[]
```

- If `CLAUDE_CODE_SIMPLE` env var is set: returns only `[BashTool, FileReadTool, FileEditTool]`
- Calls `getAllBaseTools()`, filters with `filterToolsByDenyRules()`
- In REPL mode: hides `REPL_ONLY_TOOLS` (Bash, Read, Write, Edit, Glob, Grep, NotebookEdit, Agent)

### 2.3 assembleToolPool()

```typescript
function assembleToolPool(
  baseTools: Tool[],
  mcpTools: Tool[],
): Tool[]
```

- Combines built-in + MCP tools
- Sorts by name for prompt-cache stability
- Deduplicates (built-in tools win over MCP tools with same name)

### 2.4 filterToolsByDenyRules()

```typescript
function filterToolsByDenyRules(
  tools: Tool[],
  permissionContext: ToolPermissionContext,
): Tool[]
```

Removes tools whose name matches a blanket deny rule in permissionContext.

### 2.5 Tool Presets

```typescript
const TOOL_PRESETS = {
  'full': /* all tools */,
  'minimal': /* BashTool, FileReadTool, FileEditTool */,
  // ...
}

function parseToolPreset(preset: string): Tool[] | null
function getToolsForDefaultPreset(): Tool[]
function getMergedTools(base: Tool[], overrides: Tool[]): Tool[]
```

---

## 3. Task Framework

**Sources:** `src/Task.ts`, `src/tasks.ts`

### 3.1 TaskType

```typescript
type TaskType =
  | 'local_bash'       // prefix: 'b'
  | 'local_agent'      // prefix: 'a'
  | 'remote_agent'     // prefix: 'r'
  | 'in_process_teammate' // prefix: 't'
  | 'local_workflow'   // prefix: 'w'
  | 'monitor_mcp'      // prefix: 'm'
  | 'dream'            // prefix: 'd'
```

### 3.2 TaskStatus

```typescript
type TaskStatus = 'pending' | 'running' | 'completed' | 'failed' | 'killed'

function isTerminalTaskStatus(status: TaskStatus): boolean
// Returns true for 'completed', 'failed', 'killed'
```

### 3.3 TaskStateBase

```typescript
type TaskStateBase = {
  id: string          // prefix + 8 random base-36 chars
  type: TaskType
  status: TaskStatus
  description: string
  toolUseId?: string
  startTime: number   // Date.now()
  endTime?: number
  totalPausedMs?: number
  outputFile: string  // getTaskOutputPath(id)
  outputOffset: number
  notified: boolean
}
```

### 3.4 ID Generation

```typescript
// Alphabet: '0123456789abcdefghijklmnopqrstuvwxyz'
// 36^8 ≈ 2.8 trillion combinations
function generateTaskId(type: TaskType): string
// Returns: prefix + 8 crypto-random base-36 chars

function createTaskStateBase(
  id: string,
  type: TaskType,
  description: string,
  toolUseId?: string,
): TaskStateBase
```

### 3.5 Task Interface

```typescript
type Task = {
  name: string
  type: TaskType
  kill(taskId: string, setAppState: SetAppState): Promise<void>
}
```

### 3.6 Task Registry

```typescript
// src/tasks.ts
function getAllTasks(): Task[]
// Returns: [LocalShellTask, LocalAgentTask, RemoteAgentTask, DreamTask,
//           optionally LocalWorkflowTask, MonitorMcpTask]

function getTaskByType(type: TaskType): Task | undefined
```

---

## 4. Core File Tools

### 4.1 FileReadTool

**Tool name:** `Read`
**Source:** `src/tools/FileReadTool/FileReadTool.ts`

**Characteristics:**
- `isConcurrencySafe: true`
- `isReadOnly: true`
- `maxResultSizeChars: Infinity` (prevents circular reads through disk persistence)
- `strict: true`
- `searchHint: 'read files, images, PDFs, notebooks'`
- `isSearchOrReadCommand: { isSearch: false, isRead: true }`

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `file_path` | `string` | Yes | Absolute path to file |
| `offset` | `integer` | No | Line number to start reading from |
| `limit` | `integer` | No | Number of lines to read |
| `pages` | `string` | No | PDF page range (e.g. `"1-5"`, `"3"`, `"10-20"`); PDF only; max 20 pages per request |

**Output Schema (discriminated union on `type`):**

```typescript
// Text file
{ type: 'text'; content: string; numLines: number; startLine: number; totalLines: number }

// Image file
{ type: 'image'; base64: string; mimeType: string; originalSize: number; dimensions: { width: number; height: number } }

// Jupyter notebook
{ type: 'notebook'; cells: NotebookCell[] }

// PDF (full)
{ type: 'pdf'; base64: string; originalSize: number }

// PDF (extracted pages)
{ type: 'parts'; pages: PDFPage[] }

// Unchanged (content not modified since last read)
{ type: 'file_unchanged' }
```

**Security / Validation:**
- Blocked device paths: `/dev/zero`, `/dev/random`, `/dev/urandom`, `/dev/full`, `/dev/stdin`, `/dev/tty`, and other infinite-stream devices
- Registers file in `readFileState` cache (path → `{mtime, content}`) enabling FileEditTool/FileWriteTool read-before-write enforcement
- UNC path handling skipped on Windows

**Exports:**
```typescript
function registerFileReadListener(listener: FileReadListener): void
class MaxFileReadTokenExceededError extends Error
```

---

### 4.2 FileWriteTool

**Tool name:** `Write`
**Source:** `src/tools/FileWriteTool/FileWriteTool.ts`

**Characteristics:**
- `strict: true`
- `maxResultSizeChars: 100_000`
- `searchHint: 'create or overwrite files'`

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `file_path` | `string` | Yes | Absolute path to file |
| `content` | `string` | Yes | Full file content to write |

**Output Schema:**

```typescript
{
  type: 'create' | 'update'
  filePath: string
  content: string
  structuredPatch: StructuredPatch
  originalFile: string | null   // null for new files
  gitDiff?: string              // Optional git diff of changes
}
```

**Validation / Safety:**
- **Read-before-write enforcement:** For existing files, file must appear in `readFileState` cache (must have been read with FileReadTool)
- **mtime staleness check:** If file mtime has changed since last read, refuses to overwrite to prevent clobbering concurrent changes
- **File size limit:** Max 1 GiB
- **UNC path security:** Skips read-check for UNC paths (`\\server\share`) on Windows
- **`.ipynb` files:** Redirected to `NotebookEditTool`
- **Team memory protection:** Blocks writes to team memory secret files
- **Deny rules:** Checks permission context deny rules
- **LF line endings:** New content uses LF regardless of platform
- Notifies LSP client on successful edit

---

### 4.3 FileEditTool

**Tool name:** `Edit`
**Source:** `src/tools/FileEditTool/FileEditTool.ts`

**Characteristics:**
- `strict: true`
- `maxResultSizeChars: 100_000`
- `searchHint: 'modify file contents in place'`

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `file_path` | `string` | Yes | Absolute path to file to edit |
| `old_string` | `string` | Yes | Text to search for (must exist exactly once unless `replace_all` is true) |
| `new_string` | `string` | Yes | Replacement text (must differ from `old_string`) |
| `replace_all` | `boolean` | No (default `false`) | Replace all occurrences of `old_string` |

**Output Schema:**

```typescript
{
  filePath: string
  oldString: string
  newString: string
  originalFile: string      // File content before edit
  structuredPatch: StructuredPatch
  userModified: boolean     // Whether user modified the proposed diff
  replaceAll: boolean
  gitDiff?: string
}
```

**Validation / Safety:**
- `old_string` must differ from `new_string`
- File must have been read via `readFileState` (read-before-write enforcement)
- mtime staleness check (same as FileWriteTool)
- `old_string` must be found in file content
- Unless `replace_all` is true, at most 1 occurrence of `old_string` is allowed
- File size: max 1 GiB
- UNC path security skip
- `.ipynb` files redirected to `NotebookEditTool`
- Team memory secret guard
- Permission context deny rule check
- Uses `findActualString()` for quote normalization (handles straight/curly quotes interchangeably)
- Uses `preserveQuoteStyle()` to maintain original quote style
- Notifies LSP client on successful edit

---

## 5. Shell Execution Tools

### 5.1 BashTool

**Tool name:** `Bash`
**Source:** `src/tools/BashTool/BashTool.tsx`

**Characteristics:**
- `maxResultSizeChars` (varies by output type)
- `searchHint` (from prompt)
- Supports background task execution
- Supports sandboxing (bwrap on Linux, sandbox-exec on macOS)

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `command` | `string` | Yes | Shell command to execute |
| `timeout` | `number` | No | Timeout in milliseconds (max varies by context) |
| `description` | `string` | No | Human-readable description of what the command does |
| `run_in_background` | `boolean` | No | Launch as background task; omitted from schema when `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS=true` |
| `dangerouslyDisableSandbox` | `boolean` | No | Override sandbox mode |
| `_simulatedSedEdit` | internal | — | Never in model-facing schema; used for sed edit simulation |

**Output Schema:**

```typescript
{
  stdout: string
  stderr: string
  interrupted: boolean
  isImage?: boolean               // stdout contains base64 image data
  backgroundTaskId?: string       // Set when run_in_background=true
  backgroundedByUser?: boolean    // User pressed Ctrl+B
  assistantAutoBackgrounded?: boolean  // Auto-backgrounded after blocking budget
  dangerouslyDisableSandbox?: boolean
  returnCodeInterpretation?: string    // Semantic meaning of exit code
  noOutputExpected?: boolean
  structuredContent?: unknown
  persistedOutputPath?: string     // Path when output too large for inline
  persistedOutputSize?: number     // Total bytes when persisted
}
```

**Timing Constants:**
- Progress threshold: `2_000ms` (show progress spinner after 2s)
- Auto-background threshold: `120_000ms` (auto-background after 2 minutes)
- `ASSISTANT_BLOCKING_BUDGET_MS`: `15_000ms` (in assistant/Kairos mode, auto-background after 15s in main agent)

**Permission Behavior:**
- Checks permission rules against the command string
- `bypassPermissions` mode: allows all commands
- `auto`/`acceptEdits` mode: allows read-only bash commands without asking; asks for write commands
- `default` mode: always asks unless explicitly allowed

**Sandbox:**
- Linux: `bwrap` (bubblewrap) based isolation
- macOS: `sandbox-exec` based isolation
- Windows: no sandbox (bwrap/sandbox-exec are POSIX-only)
- `dangerouslyDisableSandbox` overrides per-call
- Enterprise policy: if sandbox required and unavailable, execution is blocked

**Blocked patterns:**
- `detectBlockedSleepPattern()`: detects bare `sleep N` commands with N>=2 as first statement; suggests using SleepTool instead

**Git operation tracking:**
- `trackGitOperations()` called after each bash command to fire analytics for commits, pushes, PR creation
- `isSearchOrReadBashCommand()`: classifies command for UI collapsing

**Exports:**
```typescript
function isSearchOrReadBashCommand(command: string): { isSearch: boolean; isRead: boolean }
function detectBlockedSleepPattern(command: string): string | null
type BashToolInput = { command: string; timeout?: number; description?: string; run_in_background?: boolean; dangerouslyDisableSandbox?: boolean }
```

---

### 5.2 PowerShellTool

**Tool name:** `PowerShell`
**Source:** `src/tools/PowerShellTool/PowerShellTool.tsx`

Windows-native PowerShell execution tool, mirroring BashTool's interface.

**Characteristics:**
- Windows-specific; requires PowerShell (`pwsh`) to be installed
- Detects PowerShell path via `getCachedPowerShellPath()`
- Tracks git operations via shared `trackGitOperations()`
- Same sandbox policy as BashTool (POSIX sandbox applies on Linux/macOS when running `pwsh`)
- `PROGRESS_THRESHOLD_MS: 2_000ms`
- `ASSISTANT_BLOCKING_BUDGET_MS: 15_000ms`

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `command` | `string` | Yes | PowerShell command to execute |
| `timeout` | `number` | No | Optional timeout in milliseconds (`max getMaxTimeoutMs()`) |
| `description` | `string` | No | Description of what the command does |
| `run_in_background` | `boolean` | No | Background execution; omitted when `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS=true` |
| `dangerouslyDisableSandbox` | `boolean` | No | Override sandbox mode |

**Output Schema:**

```typescript
{
  stdout: string
  stderr: string
  interrupted: boolean
  returnCodeInterpretation?: string
  isImage?: boolean
  persistedOutputPath?: string
  persistedOutputSize?: number
  backgroundTaskId?: string
  backgroundedByUser?: boolean
  assistantAutoBackgrounded?: boolean
}
```

**PowerShell-specific features:**
- `PS_SEARCH_COMMANDS`: `Select-String`, `Get-ChildItem`, `FindStr`, `where.exe` (grep/find equivalents)
- `PS_READ_COMMANDS`: `Get-Content`, `Get-Item`, `Test-Path`, `Resolve-Path`, `Get-Process`, `Get-Service`, `Get-ChildItem`, `Get-Location`, `Get-FileHash`, `Get-Acl`, `Format-Hex`
- `PS_SEMANTIC_NEUTRAL_COMMANDS`: `Write-Output`, `Write-Host`
- `detectBlockedSleepPattern()`: catches `Start-Sleep N`, `Start-Sleep -Seconds N`, `sleep N` as first statement
- `DISALLOWED_AUTO_BACKGROUND_COMMANDS`: `['start-sleep', 'sleep']` (not auto-backgrounded)
- Windows-native sandbox policy: if enterprise requires sandbox but Windows native, execution blocked

**Exports:**
```typescript
export type PowerShellToolInput
function detectBlockedSleepPattern(command: string): string | null
```

---

## 6. Search Tools

### 6.1 GlobTool

**Tool name:** `Glob`
**Source:** `src/tools/GlobTool/GlobTool.ts`

**Characteristics:**
- `isConcurrencySafe: true`
- `isReadOnly: true`
- `searchHint: 'find files by name pattern or wildcard'`
- `isSearchOrReadCommand: { isSearch: true, isRead: false }`

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `pattern` | `string` | Yes | Glob pattern (e.g. `"**/*.js"`, `"src/**/*.ts"`) |
| `path` | `string` | No | Directory to search in (defaults to cwd) |

**Output Schema:**

```typescript
{
  filenames: string[]   // Relative to cwd
  durationMs: number
  numFiles: number
  truncated: boolean    // True if results were truncated
}
```

**Behavior:**
- Default limit: 100 files (overridable via `context.globLimits?.maxResults`)
- Results are sorted by modification time (most recent first)
- Paths relativized to cwd for compactness

---

### 6.2 GrepTool

**Tool name:** `Grep`
**Source:** `src/tools/GrepTool/GrepTool.ts`

**Characteristics:**
- `isConcurrencySafe: true`
- `isReadOnly: true`
- `strict: true`
- `maxResultSizeChars: 20_000`
- `searchHint: 'search file contents with regex (ripgrep)'`
- `isSearchOrReadCommand: { isSearch: true }`

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `pattern` | `string` | Yes | Regular expression pattern |
| `path` | `string` | No | File or directory to search |
| `glob` | `string` | No | Glob filter (e.g. `"*.js"`, `"**/*.tsx"`) |
| `output_mode` | `'content' \| 'files_with_matches' \| 'count'` | No (default `'files_with_matches'`) | Output format |
| `-B` | `number` | No | Lines before each match (requires `output_mode: 'content'`) |
| `-A` | `number` | No | Lines after each match (requires `output_mode: 'content'`) |
| `-C` | `number` | No | Lines before and after each match |
| `context` | `number` | No | Alias for `-C` |
| `-n` | `boolean` | No | Show line numbers (requires `output_mode: 'content'`) |
| `-i` | `boolean` | No | Case-insensitive search |
| `type` | `string` | No | File type filter (e.g. `"js"`, `"py"`) |
| `head_limit` | `number` | No (default `250`) | Limit output to first N lines/entries |
| `offset` | `number` | No (default `0`) | Skip first N entries |
| `multiline` | `boolean` | No (default `false`) | Enable multiline matching (`.` matches newlines) |

**Output Schema:**

```typescript
{
  mode: 'content' | 'files_with_matches' | 'count'
  numFiles: number
  filenames: string[]
  content?: string        // When mode='content'
  numLines?: number       // When mode='content'
  numMatches?: number     // When mode='count'
  appliedLimit?: number
  appliedOffset?: number
}
```

**Implementation Details:**
- Backed by `ripgrep` (`rg`) binary
- Excludes VCS directories: `.git`, `.svn`, `.hg`, `.bzr`, `.jj`, `.sl`
- `max-columns: 500` to prevent oversized lines
- Default `head_limit: 250` when unspecified

---

## 7. Agent / Multi-Agent Tools

### 7.1 AgentTool

**Tool name:** `Agent` (alias: `Task`)
**Constants:** `AGENT_TOOL_NAME`, `LEGACY_AGENT_TOOL_NAME`
**Source:** `src/tools/AgentTool/AgentTool.tsx`

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `description` | `string` | Yes | 3-5 word description of the task |
| `prompt` | `string` | Yes | Full task prompt for the agent |
| `subagent_type` | `string` | No | Type of subagent to spawn |
| `model` | `'sonnet' \| 'opus' \| 'haiku'` | No | Model alias for the agent |
| `run_in_background` | `boolean` | No | Launch as background task |
| `name` | `string` | No | Named agent for messaging |
| `team_name` | `string` | No | Associate with this team |
| `mode` | `string` | No | Permission mode override |
| `isolation` | `'worktree' \| 'remote'` | No | Isolation strategy |
| `cwd` | `string` | No | Working directory (Kairos only) |

**Output Schema:**

Synchronous completion:
```typescript
{
  status: 'completed'
  result: string
}
```

Asynchronous launch:
```typescript
{
  status: 'async_launched'
  agentId: string
  description: string
  prompt: string
}
```

**Behavior:**
- Auto-backgrounds after `120_000ms`
- Progress shown after `2_000ms`
- Supports fork subagent (`subagent_type: 'fork'`)
- Multi-agent swarm integration: when inside a team, can spawn named teammates
- `isolation: 'worktree'` creates git worktree for isolated execution
- `isolation: 'remote'` runs in remote session

---

### 7.2 TeamCreateTool

**Tool name:** `TeamCreate`
**Source:** `src/tools/TeamCreateTool/TeamCreateTool.ts`
**Gate:** `isAgentSwarmsEnabled()`

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `team_name` | `string` | Yes | Name for the team |
| `description` | `string` | No | Team description |
| `agent_type` | `string` | No | Default agent type for team members |

**Output Schema:**

```typescript
{
  team_name: string
  team_file_path: string
  lead_agent_id: string
}
```

**Behavior:**
- Creates team file at `~/.claude/teams/<team_name>.json`
- Resets task list to team-scoped task list
- Registers team for session cleanup (auto-cleanup on exit)
- One team per leader enforced: calling again while a team exists returns an error

---

### 7.3 TeamDeleteTool

**Tool name:** `TeamDelete`
**Source:** `src/tools/TeamDeleteTool/TeamDeleteTool.ts`
**Gate:** `isAgentSwarmsEnabled()`
**Input:** `{}` (empty object)

**Output Schema:**

```typescript
{
  success: boolean
  message: string
  team_name?: string
}
```

**Behavior:**
- Refuses to delete if any non-lead members have `isActive !== false` (still running)
- Calls `cleanupTeamDirectories(teamName)` to remove team files and worktrees
- Unregisters team from session cleanup
- Clears teammate color assignments
- Clears leader team name (task list falls back to session ID)
- Clears team context and inbox from app state

---

### 7.4 SendMessageTool

**Tool name:** `SendMessage`
**Source:** `src/tools/SendMessageTool/SendMessageTool.ts`
**Gate:** `isAgentSwarmsEnabled()`

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `to` | `string` | Yes | Recipient: agent name, `'*'` (broadcast), `'uds:<path>'`, or `'bridge:<session-id>'` |
| `summary` | `string` | No | Short summary of message for UI |
| `message` | `string \| StructuredMessage` | Yes | Message content |

**StructuredMessage union:**

```typescript
type StructuredMessage =
  | { type: 'shutdown_request'; reason?: string }
  | { type: 'shutdown_response'; status: 'ok' | 'error'; message?: string }
  | { type: 'plan_approval_response'; approved: boolean; comment?: string; requestId: string }
```

**Routing:**
- **In-process agents:** Queues message in agent's inbox or resumes paused agent
- **Mailbox (teammates):** Writes to `~/.claude/mailboxes/<name>.json`
- **UDS socket:** Sends via Unix domain socket (for local inter-process)
- **Bridge (cross-machine):** Routes via Remote Control API; requires user safety check (not auto-approvable)

**Permission:** Bridge messages require user consent via `decisionReason` safety gate.

---

## 8. Task Management Tools

### 8.1 TaskStopTool

**Tool name:** `TaskStop` (alias: `KillShell`)
**Source:** `src/tools/TaskStopTool/TaskStopTool.ts`
**Characteristics:** `shouldDefer: true`, `isConcurrencySafe: true`

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `task_id` | `string` | No | Task ID to stop (from background task launch) |
| `shell_id` | `string` | No | Deprecated alias for `task_id` |

**Output Schema:**

```typescript
{
  message: string
  task_id: string
  task_type: TaskType
  command?: string    // For bash tasks
}
```

**Validation:**
- Task must exist in app state
- Task must be in a running (non-terminal) state

---

### 8.2 TaskOutputTool

**Tool name:** `TaskOutput` (`TASK_OUTPUT_TOOL_NAME`)
**Source:** `src/tools/TaskOutputTool/TaskOutputTool.tsx`

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `task_id` | `string` | Yes | Task ID to read output from |
| `block` | `boolean` | No (default `true`) | Block until task completes |
| `timeout` | `number` | No (default `30_000`, range `0–600_000ms`) | Maximum wait time in ms |

**Output Schema:**

```typescript
{
  retrieval_status: 'success' | 'timeout' | 'not_ready'
  task: TaskOutput | null
}

type TaskOutput = {
  task_id: string
  task_type: TaskType
  status: TaskStatus
  description: string
  output: string
  exitCode?: number
  error?: string
  prompt?: string     // For agent tasks
  result?: string     // Final result text for agent tasks
}
```

---

### 8.3 TodoWriteTool (V1)

**Tool name:** `TodoWrite`
**Source:** `src/tools/TodoWriteTool/TodoWriteTool.ts`
**Characteristics:** `strict: true`, `shouldDefer: true`, `maxResultSizeChars: 100_000`
**Gate:** Disabled when `isTodoV2Enabled()`

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `todos` | `TodoItem[]` | Yes | Full replacement list of todo items |

`TodoItem` schema:
```typescript
{
  id: string
  content: string
  status: 'pending' | 'in_progress' | 'completed'
  priority: 'high' | 'medium' | 'low'
}
```

**Output Schema:**

```typescript
{
  oldTodos: TodoItem[]
  newTodos: TodoItem[]
  verificationNudgeNeeded?: boolean
}
```

**Behavior:**
- Replaces the entire todo list atomically
- Clears list automatically when all todos are completed
- `verificationNudgeNeeded`: signals UI to nudge model to verify completed items

---

### 8.4 TaskCreateTool (V2)

**Tool name:** `TaskCreate`
**Source:** `src/tools/TaskCreateTool/TaskCreateTool.ts`
**Gate:** `isTodoV2Enabled()`

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `subject` | `string` | Yes | Short task title |
| `description` | `string` | Yes | Detailed task description |
| `activeForm` | `object` | No | Form data for active task |
| `metadata` | `object` | No | Arbitrary metadata |

**Output Schema:**

```typescript
{
  task: {
    id: string
    subject: string
  }
}
```

**Behavior:**
- Runs `executeTaskCreatedHooks` after creation
- Auto-expands task panel in UI
- Deletes task if hook throws an error

---

### 8.5 TaskGetTool (V2)

**Tool name:** `TaskGet`
**Source:** `src/tools/TaskGetTool/TaskGetTool.ts`
**Gate:** `isTodoV2Enabled()`
**Characteristics:** `shouldDefer: true`, `isReadOnly: true`

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `taskId` | `string` | Yes | Task ID to retrieve |

**Output Schema:**

```typescript
{
  task: {
    id: string
    subject: string
    description: string
    status: TaskStatus
    blocks: string[]        // Task IDs this task blocks
    blockedBy: string[]     // Task IDs blocking this task
  } | null
}
```

---

### 8.6 TaskUpdateTool (V2)

**Tool name:** `TaskUpdate`
**Source:** `src/tools/TaskUpdateTool/TaskUpdateTool.ts`
**Gate:** `isTodoV2Enabled()`

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `taskId` | `string` | Yes | Task ID to update |
| `subject` | `string` | No | Updated title |
| `description` | `string` | No | Updated description |
| `activeForm` | `object` | No | Updated form data |
| `status` | `TaskStatus \| 'deleted'` | No | New status; `'deleted'` removes the task |
| `addBlocks` | `string[]` | No | Task IDs this task should block |
| `addBlockedBy` | `string[]` | No | Task IDs that block this task |
| `owner` | `string` | No | Assign ownership |
| `metadata` | `object` | No | Updated metadata |

**Output Schema:**

```typescript
{
  success: boolean
  taskId: string
  updatedFields: string[]
  error?: string
  statusChange?: { from: TaskStatus; to: TaskStatus | 'deleted' }
  verificationNudgeNeeded?: boolean
}
```

**Behavior:**
- Runs `executeTaskCompletedHooks` when status transitions to `completed`
- Auto-sets owner to calling agent on `in_progress` status
- Writes mailbox notification on owner change
- `verificationNudgeNeeded`: set when 3+ tasks completed without a verification step

---

### 8.7 TaskListTool (V2)

**Tool name:** `TaskList`
**Source:** `src/tools/TaskListTool/TaskListTool.ts`
**Gate:** `isTodoV2Enabled()`
**Characteristics:** `shouldDefer: true`, `isReadOnly: true`
**Input:** `{}` (empty object)

**Output Schema:**

```typescript
{
  tasks: Array<{
    id: string
    subject: string
    status: TaskStatus
    owner?: string
    blockedBy: string[]     // Only non-completed blocking tasks
  }>
}
```

**Behavior:**
- Filters out tasks with `_internal` metadata flag
- Filters already-completed IDs from `blockedBy` lists

---

## 9. Web Tools

### 9.1 WebFetchTool

**Tool name:** `WebFetch`
**Source:** `src/tools/WebFetchTool/WebFetchTool.ts`
**Characteristics:** `shouldDefer: true`, `maxResultSizeChars: 100_000`

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `url` | `string` (URL) | Yes | URL to fetch |
| `prompt` | `string` | Yes | Instruction for summarizing the fetched content |

**Output Schema:**

```typescript
{
  bytes: number
  code: number          // HTTP status code
  codeText: string
  result: string        // Processed/summarized content
  durationMs: number
  url: string
}
```

**Permission:** Per-hostname rules. Preapproved hosts bypass prompt. Rule format: `domain:hostname`.

**Implementation:**
- Converts HTML to Markdown before processing
- Applies Haiku model summarization via `applyPromptToMarkdown(prompt, markdown)` when content exceeds threshold
- Respects `abortController` signal

---

### 9.2 WebSearchTool

**Tool name:** `WebSearch`
**Source:** `src/tools/WebSearchTool/WebSearchTool.ts`
**Characteristics:** `shouldDefer: true`, `maxResultSizeChars: 100_000`

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `query` | `string` (min 2 chars) | Yes | Search query |
| `allowed_domains` | `string[]` | No | Restrict results to these domains |
| `blocked_domains` | `string[]` | No | Exclude results from these domains |

**Output Schema:**

```typescript
{
  query: string
  results: SearchResult[] | string   // String when no results or commentary
  durationSeconds: number
}

type SearchResult = {
  title: string
  url: string
  snippet: string
}
```

**Permission:** `'passthrough'` — always prompts user

**Implementation:**
- Uses beta tool: `web_search_20250305`
- Maximum 8 search operations per call
- Enabled only for `firstParty`, `vertex`, and `foundry` API providers

---

## 10. MCP Integration Tools

### 10.1 MCPTool

**Tool name:** `mcp` (overridden per server to `mcp__<server>__<tool>`)
**Source:** `src/tools/MCPTool/MCPTool.ts`

**Characteristics:**
- `isMcp: true`
- `maxResultSizeChars: 100_000`
- `permission: 'passthrough'` (always asks)
- All methods are overridden in `mcpClient.ts` when instantiated per server

**Input Schema:** `z.object({}).passthrough()` — accepts any object
**Output:** `string`

**Exports:**
```typescript
type MCPProgress  // Re-exported from mcp progress types
```

---

### 10.2 McpAuthTool

**Tool name:** `mcp__<serverName>__authenticate`
**Source:** `src/tools/McpAuthTool/McpAuthTool.ts`

A pseudo-tool factory, not a standard `buildTool()` instance.

```typescript
function createMcpAuthTool(
  serverName: string,
  config: ScopedMcpServerConfig,
): Tool<InputSchema, McpAuthOutput>
```

**Input:** `{}` (empty)

**Output Schema:**

```typescript
type McpAuthOutput = {
  status: 'auth_url' | 'unsupported' | 'error'
  message: string
  authUrl?: string    // Present when status='auth_url'
}
```

**Behavior:**
- Created for MCP servers that are installed but need OAuth authentication
- Starts `performMCPOAuthFlow()` with `skipBrowserOpen: true`
- Returns authorization URL for user to open in browser
- Background continuation: when OAuth completes, calls `reconnectMcpServerImpl()` and swaps real tools into app state via prefix-based replacement
- `claudeai-proxy` transport: returns `'unsupported'` and directs user to `/mcp`
- Silent auth (cached IdP token): returns success message without URL

---

### 10.3 ListMcpResourcesTool

**Tool name:** `mcp__listResources` (`LIST_MCP_RESOURCES_TOOL_NAME`)
**Source:** `src/tools/ListMcpResourcesTool/ListMcpResourcesTool.ts`
**Characteristics:** `shouldDefer: true`

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `server` | `string` | No | Filter by server name |

**Output Schema:**

```typescript
Array<{
  uri: string
  name: string
  mimeType?: string
  description?: string
  server: string
}>
```

**Note:** Not included in `getTools()` directly; only added when MCP servers with resources are present.

---

### 10.4 ReadMcpResourceTool

**Tool name:** `ReadMcpResourceTool`
**Source:** `src/tools/ReadMcpResourceTool/ReadMcpResourceTool.ts`
**Characteristics:** `shouldDefer: true`

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `server` | `string` | Yes | MCP server name |
| `uri` | `string` | Yes | Resource URI to read |

**Output Schema:**

```typescript
{
  contents: Array<{
    uri: string
    mimeType?: string
    text?: string
    blobSavedTo?: string    // Path when binary blob saved to disk
  }>
}
```

**Implementation:**
- Binary blobs are saved to disk; `getBinaryBlobSavedMessage()` returns path reference string

---

## 11. Plan Mode Tools

### 11.1 EnterPlanModeTool

**Tool name:** `EnterPlanMode`
**Source:** `src/tools/EnterPlanModeTool/EnterPlanModeTool.ts`
**Characteristics:** `shouldDefer: true`, `maxResultSizeChars: 100_000`, `isConcurrencySafe: true`, `isReadOnly: true`

**Input:** `{}` (empty object)

**Output Schema:**

```typescript
{
  message: string   // Confirmation message
}
```

**Behavior:**
- Sets permission mode to `'plan'` (read-only planning mode)
- Disabled with `--channels` flag
- Cannot be called from agent (teammate) context; only from main agent
- Saves current permission mode as `prePlanMode` for restoration by `ExitPlanMode`

---

### 11.2 ExitPlanModeV2Tool

**Tool name:** `ExitPlanMode` (`EXIT_PLAN_MODE_V2_TOOL_NAME`)
**Source:** `src/tools/ExitPlanModeTool/ExitPlanModeV2Tool.ts`
**Characteristics:** `shouldDefer: true`, `maxResultSizeChars: 100_000`

**Input Schema (model-facing):**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `allowedPrompts` | `AllowedPrompt[]` | No | Pre-approved tool calls (passthrough schema) |

`AllowedPrompt`:
```typescript
{
  tool: 'Bash'
  prompt: string
}
```

**SDK Input Schema** (adds):

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `plan` | `string` | No | Plan text for SDK mode |
| `planFilePath` | `string` | No | Path to plan file for SDK mode |

**Output Schema:**

```typescript
type Output = {
  plan: string
  isAgent: boolean
  filePath?: string
  hasTaskTool?: boolean
  planWasEdited?: boolean
  awaitingLeaderApproval?: boolean
  requestId?: string
}
```

**Behavior:**
- For teammates with `isPlanModeRequired()`: sends `plan_approval_request` message to team-lead's mailbox; returns `awaitingLeaderApproval: true` with a `requestId`
- For non-teammates: restores permission mode to `prePlanMode` (typically `default` or `auto`)
- Disabled with `--channels` flag

**Exports:**
```typescript
type AllowedPrompt
const _sdkInputSchema  // Extended schema with plan/planFilePath
type Output
```

---

## 12. Notebook Tool

### NotebookEditTool

**Tool name:** `NotebookEdit`
**Source:** `src/tools/NotebookEditTool/NotebookEditTool.ts`
**Characteristics:** `shouldDefer: true`, `maxResultSizeChars: 100_000`

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `notebook_path` | `string` | Yes | Absolute path to `.ipynb` file |
| `cell_id` | `string` | No | Target cell ID (required for edit/delete; omit for new cell) |
| `new_source` | `string` | Yes | New source content for the cell |
| `cell_type` | `'code' \| 'markdown'` | No | Cell type (for new cells) |
| `edit_mode` | `'replace' \| 'insert' \| 'delete'` | No (default `'replace'`) | Edit operation |

**Output Schema:**

```typescript
{
  new_source: string
  cell_id?: string
  cell_type: 'code' | 'markdown'
  language: string
  edit_mode: 'replace' | 'insert' | 'delete'
  error?: string
  notebook_path: string
  original_file: string   // Full notebook JSON before edit
  updated_file: string    // Full notebook JSON after edit
}
```

**Validation / Safety:**
- Read-before-write required (same as FileEditTool/FileWriteTool)
- mtime staleness check
- UNC path security skip
- Clears `execution_count` and `outputs` on cell replace (avoids stale output display)

---

## 13. Worktree Tools

### 13.1 EnterWorktreeTool

**Tool name:** `EnterWorktree` (`ENTER_WORKTREE_TOOL_NAME`)
**Source:** `src/tools/EnterWorktreeTool/EnterWorktreeTool.ts`
**Characteristics:** `shouldDefer: true`, `maxResultSizeChars: 100_000`

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `name` | `string` | No | Optional name for the worktree. Each `/`-separated segment: letters, digits, dots, underscores, dashes only; max 64 chars total. Random name generated if not provided. |

**Output Schema:**

```typescript
{
  worktreePath: string
  worktreeBranch?: string
  message: string
}
```

**Behavior:**
- Validates not already in a worktree session created by this session
- Resolves to main repo root before creating worktree
- Calls `createWorktreeForSession(sessionId, slug)` to create git worktree (or hooks-based worktree)
- Updates `cwd`, `originalCwd` to worktree path
- Clears system prompt sections cache (so `env_info_simple` recomputes with worktree context)
- Clears memoized caches that depend on cwd
- Logs `tengu_worktree_created` analytics event

---

### 13.2 ExitWorktreeTool

**Tool name:** `ExitWorktree` (`EXIT_WORKTREE_TOOL_NAME`)
**Source:** `src/tools/ExitWorktreeTool/ExitWorktreeTool.ts`
**Characteristics:** `shouldDefer: true`, `maxResultSizeChars: 100_000`

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `action` | `'keep' \| 'remove'` | Yes | `'keep'` preserves worktree and branch on disk; `'remove'` deletes both |
| `discard_changes` | `boolean` | No | Required `true` when `action='remove'` and worktree has uncommitted files or unmerged commits |

**Output Schema:**

```typescript
{
  action: 'keep' | 'remove'
  originalCwd: string
  worktreePath: string
  worktreeBranch?: string
  tmuxSessionName?: string
  discardedFiles?: number     // Set when action='remove'
  discardedCommits?: number   // Set when action='remove'
  message: string
}
```

**Validation:**
- Only operates on worktrees created by `EnterWorktreeTool` in the current session (scope guard via `getCurrentWorktreeSession()`)
- When `action='remove'` without `discard_changes: true`:
  - Runs `countWorktreeChanges()`: uses `git status --porcelain` + `git rev-list --count <originalHead>..HEAD`
  - Returns error listing uncommitted files and unmerged commits
  - Returns error if git state cannot be determined (fail-closed)

**Behavior:**
- `action='keep'`: calls `keepWorktree()`, restores session to original cwd, preserves worktree for later use
- `action='remove'`: kills tmux session if any, calls `cleanupWorktree()` (removes worktree and branch), restores session
- Both actions: restore `cwd`, `originalCwd`, optionally `projectRoot`, clear caches
- Logs `tengu_worktree_kept` or `tengu_worktree_removed` analytics events

---

## 14. Scheduling Tools

These tools are gated by `isKairosCronEnabled()` (requires `feature('KAIROS')` + GB gate).

### 14.1 CronCreateTool

**Tool name:** `CronCreate` (`CRON_CREATE_TOOL_NAME`)
**Source:** `src/tools/ScheduleCronTool/CronCreateTool.ts`
**Characteristics:** `shouldDefer: true`, `maxResultSizeChars: 100_000`

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `cron` | `string` | Yes | Standard 5-field cron expression in local time: `"M H DoM Mon DoW"` |
| `prompt` | `string` | Yes | Prompt to enqueue at each fire time |
| `recurring` | `boolean` | No (default `true`) | `true` = fire on every cron match (auto-expires after `DEFAULT_MAX_AGE_DAYS` days); `false` = fire once then auto-delete |
| `durable` | `boolean` | No (default `false`) | `true` = persist to `.claude/scheduled_tasks.json` and survive restarts; `false` = in-memory only, dies when session ends |

**Output Schema:**

```typescript
{
  id: string            // Job ID for reference in CronDelete/CronList
  humanSchedule: string // Human-readable schedule (e.g. "Every 5 minutes")
  recurring: boolean
  durable?: boolean
}
```

**Validation:**
- Valid 5-field cron expression required
- Expression must match at least one calendar date within the next year
- Maximum 50 concurrent scheduled jobs
- Durable crons not supported for teammates (teammates don't persist across sessions)

**Constants:**
- `MAX_JOBS: 50`
- `DEFAULT_MAX_AGE_DAYS`: defined in prompt.ts

---

### 14.2 CronDeleteTool

**Tool name:** `CronDelete` (`CRON_DELETE_TOOL_NAME`)
**Source:** `src/tools/ScheduleCronTool/CronDeleteTool.ts`
**Characteristics:** `shouldDefer: true`, `maxResultSizeChars: 100_000`

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | `string` | Yes | Job ID returned by `CronCreate` |

**Output Schema:**

```typescript
{
  id: string    // Cancelled job ID
}
```

**Validation:**
- Job with given ID must exist
- Teammates may only delete their own cron jobs (ownership enforced by `agentId`)

---

### 14.3 CronListTool

**Tool name:** `CronList` (`CRON_LIST_TOOL_NAME`)
**Source:** `src/tools/ScheduleCronTool/CronListTool.ts`
**Characteristics:** `shouldDefer: true`, `isConcurrencySafe: true`, `isReadOnly: true`, `maxResultSizeChars: 100_000`
**Input:** `{}` (empty object)

**Output Schema:**

```typescript
{
  jobs: Array<{
    id: string
    cron: string
    humanSchedule: string
    prompt: string
    recurring?: boolean
    durable?: boolean
  }>
}
```

**Behavior:**
- Teammates only see their own cron jobs (filtered by `agentId`)
- Team lead (no teammate context) sees all jobs

---

## 15. Meta / Discovery Tools

### 15.1 ToolSearchTool

**Tool name:** `ToolSearch` (`TOOL_SEARCH_TOOL_NAME`)
**Source:** `src/tools/ToolSearchTool/ToolSearchTool.ts`
**Characteristics:** `maxResultSizeChars: 100_000`, `isConcurrencySafe: true`, `isReadOnly: true`

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `query` | `string` | Yes | `select:<name>` for exact tool lookup by name, or keywords for fuzzy search |
| `max_results` | `number` | No (default `5`) | Maximum number of tools to return |

**Output Schema:**

```typescript
{
  matches: string[]              // Tool names that matched
  query: string
  total_deferred_tools: number
  pending_mcp_servers?: string[] // MCP servers still loading
}
```

**Scoring algorithm:**

| Match type | Built-in score | MCP score |
|------------|---------------|-----------|
| Exact name part match | 10 | 12 |
| Substring in name | 5 | 6 |
| Word boundary in `searchHint` | 4 | — |
| Match in description | 2 | — |

**Behavior:**
- `select:<name>` prefix: exact name lookup, fetches full schema definition
- Keywords: fuzzy scoring across all deferred tools
- `mapToolResultToToolResultBlockParam`: returns `tool_reference` blocks for matched tools (injects their schemas into context)

**Exports:**
```typescript
function clearToolSearchDescriptionCache(): void
```

---

### 15.2 AskUserQuestionTool

**Tool name:** `AskUserQuestion` (`ASK_USER_QUESTION_TOOL_NAME`)
**Source:** `src/tools/AskUserQuestionTool/AskUserQuestionTool.tsx`
**Characteristics:** `shouldDefer: true`, `maxResultSizeChars: 100_000`, `requiresUserInteraction: true`

**Input Schema (model-facing):**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `questions` | `Question[]` | Yes | 1–4 questions to ask the user |
| `answers` | `object` | No | Injected by UI after user responds (not in model-facing schema) |
| `annotations` | `object` | No | Metadata annotations |
| `metadata` | `object` | No | Arbitrary metadata |

`Question` schema:
```typescript
{
  question: string
  header?: string
  options: QuestionOption[]   // 2-4 options
  multiSelect?: boolean
}

type QuestionOption = {
  label: string
  value: string
  description?: string
}
```

**Output Schema:**

```typescript
{
  questions: Question[]
  answers: Record<string, string | string[]>  // question → answer(s)
  annotations?: object
}
```

**Behavior:**
- Disabled when `--channels` flag is active (no terminal for dialog)
- UI renders interactive question dialog; `answers` field is injected by UI layer
- Supports single-select and multi-select question types

**Exports:**
```typescript
const _sdkInputSchema   // Full input schema including answers field
const _sdkOutputSchema  // Full output schema
type Question
type QuestionOption
```

---

## 16. Kairos / Special Mode Tools

### 16.1 BriefTool (SendUserMessage)

**Tool name:** `SendUserMessage` (`BRIEF_TOOL_NAME`, alias: `LEGACY_BRIEF_TOOL_NAME`)
**Source:** `src/tools/BriefTool/BriefTool.ts`
**Gate:** `isBriefEnabled()` — requires `feature('KAIROS')` or `feature('KAIROS_BRIEF')` + Growthbook gate + `userMsgOptIn` or `kairosActive`

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `message` | `string` (markdown) | Yes | Message to send to the user |
| `attachments` | `string[]` | No | File paths to attach |
| `status` | `'normal' \| 'proactive'` | Yes | Message disposition: `'proactive'` for unsolicited updates |

**Output Schema:**

```typescript
type Output = {
  message: string
  attachments?: Array<{
    path: string
    size: number
    isImage: boolean
    file_uuid: string
  }>
  sentAt?: string   // ISO timestamp
}
```

**Exports:**
```typescript
function isBriefEntitled(): boolean  // Has the entitlement
function isBriefEnabled(): boolean   // Has entitlement AND feature flags on
type Output
```

---

### 16.2 SleepTool

**Tool name:** `Sleep`
**Source:** `src/tools/SleepTool/` (only `prompt.ts` present; tool implementation loaded via `require()` when `feature('SLEEP_TOOL')`)
**Gate:** `feature('SLEEP_TOOL')` feature flag

**Purpose:** Wait for a specified duration without holding a shell process. Can be interrupted by the user.

**Prompt highlights:**
- Use when user says to sleep/rest, when waiting for something, or when nothing to do
- Can be called concurrently with other tools
- Prefer over `Bash(sleep ...)` — doesn't hold a shell process
- Receives periodic `<tick>` prompts during sleep; check for useful work before sleeping
- Each wake-up costs an API call; prompt cache expires after 5 minutes of inactivity

---

### 16.3 RemoteTriggerTool

**Tool name:** `RemoteTrigger` (`REMOTE_TRIGGER_TOOL_NAME`)
**Source:** `src/tools/RemoteTriggerTool/RemoteTriggerTool.ts`
**Gate:** `feature('AGENT_TRIGGERS')` + `getFeatureValue_CACHED_MAY_BE_STALE('tengu_surreal_dali', false)` + `isPolicyAllowed('allow_remote_sessions')`
**Characteristics:** `shouldDefer: true`, `maxResultSizeChars: 100_000`, `isConcurrencySafe: true`

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `action` | `'list' \| 'get' \| 'create' \| 'update' \| 'run'` | Yes | CRUD operation on triggers |
| `trigger_id` | `string` | No | Required for `get`, `update`, `run` (regex: `[\w-]+`) |
| `body` | `Record<string, unknown>` | No | JSON body for `create` and `update` |

**Output Schema:**

```typescript
{
  status: number   // HTTP status code
  json: string     // Serialized response body
}
```

**Implementation:**
- Calls `${BASE_API_URL}/v1/code/triggers` REST API
- Uses OAuth token (`checkAndRefreshOAuthTokenIfNeeded()` + `getClaudeAIOAuthTokens()`)
- Requires org UUID (`getOrganizationUUID()`)
- API beta header: `ccr-triggers-2026-01-30`
- Timeout: `20_000ms`
- `isReadOnly`: `true` for `list` and `get` actions; `false` otherwise

---

## 17. SDK / Output Tools

### 17.1 SyntheticOutputTool (StructuredOutput)

**Tool name:** `StructuredOutput`
**Source:** `src/tools/SyntheticOutputTool/SyntheticOutputTool.ts`
**Gate:** Enabled only in non-interactive sessions (SDK / `--output-format json` mode)

**Input:** Passthrough — any object, validated against the provided JSON schema via AJV
**Output:** `'Structured output provided successfully'` (string)

**Factory:**

```typescript
function createSyntheticOutputTool(jsonSchema: object): Tool
```

- Creates a validated instance with `WeakMap` caching (same schema object returns same tool)
- Used in SDK/`--output-format json` mode to force the model to emit a structured final response
- AJV validation ensures output matches the caller-provided JSON schema

---

## 18. Skill Tool

**Tool name:** `Skill` (`SKILL_TOOL_NAME`)
**Source:** `src/tools/SkillTool/SkillTool.ts`
**Characteristics:** `maxResultSizeChars: 100_000`

**Purpose:** Runs prompt commands (skills) defined in local files or MCP skill servers.

**Exports:**
```typescript
type Progress  // Re-export of SkillToolProgress
```

---

## 19. LSP Tool

**Tool name:** `LSP` (`LSP_TOOL_NAME`)
**Source:** `src/tools/LSPTool/LSPTool.ts`
**Gate:** `ENABLE_LSP_TOOL=true` environment variable
**Characteristics:** `isLsp: true`

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `operation` | `string` | Yes | One of: `goToDefinition`, `findReferences`, `hover`, `documentSymbol`, `workspaceSymbol`, `goToImplementation`, `prepareCallHierarchy`, `incomingCalls`, `outgoingCalls` |
| `filePath` | `string` | Yes | Absolute path to file |
| `line` | `number` | Yes | 1-based line number |
| `character` | `number` | Yes | 1-based character position |

**Constraints:**
- Max file size: 10 MB

---

## 20. REPL Tool

**Tool name:** `REPL`
**Source:** `src/tools/REPLTool/`
**Gate:** Ant-only (`USER_TYPE === 'ant'`) + loaded via `require()`

**Constants (`src/tools/REPLTool/constants.ts`):**

```typescript
const REPL_TOOL_NAME = 'REPL'

function isReplModeEnabled(): boolean
// true when: CLAUDE_CODE_REPL not falsy AND (CLAUDE_REPL_MODE=1 OR (USER_TYPE='ant' AND CLAUDE_CODE_ENTRYPOINT='cli'))
// SDK entrypoints default to REPL mode OFF

const REPL_ONLY_TOOLS = new Set([
  'Read', 'Write', 'Edit', 'Glob', 'Grep', 'Bash', 'NotebookEdit', 'Agent',
])
// Hidden from model in REPL mode; model must use REPL for batch operations
```

**Primitive Tools (`src/tools/REPLTool/primitiveTools.ts`):**

```typescript
function getReplPrimitiveTools(): readonly Tool[]
// Returns: [FileReadTool, FileWriteTool, FileEditTool, GlobTool, GrepTool, BashTool, NotebookEditTool, AgentTool]
// Lazy getter to avoid TDZ circular dependency
// These tools remain accessible inside REPL VM context even when hidden from model
```

---

## 21. Config Tool

**Tool name:** `Config` (`CONFIG_TOOL_NAME`)
**Source:** `src/tools/ConfigTool/ConfigTool.ts`
**Gate:** Ant-only
**Characteristics:** `shouldDefer: true`, `maxResultSizeChars: 100_000`

**Input Schema:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `setting` | `string` | Yes | Configuration key (e.g. `"theme"`, `"model"`) |
| `value` | `string \| boolean \| number` | No | New value (omit for GET operation) |

**Output Schema:**

```typescript
{
  success: boolean
  operation?: 'get' | 'set'
  setting?: string
  value?: unknown
  previousValue?: unknown
  newValue?: unknown
  error?: string
}
```

**Permission:**
- GET operations: auto-allow
- SET operations: requires user permission prompt

**Sources:** Settings can come from `'global'` config or `'settings'` config.

**Exports:**
```typescript
type Input
type Output
```

---

## 22. Shared Utilities

### 22.1 tools/utils.ts

```typescript
/**
 * Tags user messages with a sourceToolUseID so they stay transient
 * until the tool resolves. Prevents "is running" message duplication in UI.
 */
function tagMessagesWithToolUseID(
  messages: (UserMessage | AttachmentMessage | SystemMessage)[],
  toolUseID: string | undefined,
): (UserMessage | AttachmentMessage | SystemMessage)[]

/**
 * Extracts the tool use ID from a parent message for a given tool name.
 */
function getToolUseIDFromParentMessage(
  parentMessage: AssistantMessage,
  toolName: string,
): string | undefined
```

---

### 22.2 tools/shared/gitOperationTracking.ts

Shell-agnostic git operation tracking for usage metrics. Works identically for BashTool and PowerShellTool.

**Exported Types:**

```typescript
type CommitKind = 'committed' | 'amended' | 'cherry-picked'
type BranchAction = 'merged' | 'rebased'
type PrAction = 'created' | 'edited' | 'merged' | 'commented' | 'closed' | 'ready'
```

**Key Functions:**

```typescript
/**
 * Scan command + output for git operations worth surfacing in UI summary.
 * Detects: git commit, git push, git merge, git rebase, gh pr *, glab mr create, curl PR APIs.
 */
function detectGitOperation(
  command: string,
  output: string,
): {
  commit?: { sha: string; kind: CommitKind }
  push?: { branch: string }
  branch?: { ref: string; action: BranchAction }
  pr?: { number: number; url?: string; action: PrAction }
}

/**
 * Fire analytics events and OTLP counters for git operations.
 * Called after each Bash/PowerShell command completes (exit code 0 only).
 */
function trackGitOperations(
  command: string,
  exitCode: number,
  stdout?: string,
): void

// Exported for testing
function parseGitCommitId(stdout: string): string | undefined
```

**Detected operations:**
- `git commit` → `tengu_git_operation{operation: 'commit'}`, increments commit OTLP counter
- `git commit --amend` → additionally fires `tengu_git_operation{operation: 'commit_amend'}`
- `git push` → `tengu_git_operation{operation: 'push'}`
- `gh pr create/edit/merge/comment/close/ready` → `tengu_git_operation{operation: 'pr_<action>'}`, creates fires PR OTLP counter + links session to PR URL
- `glab mr create` → `tengu_git_operation{operation: 'pr_create'}`, increments PR OTLP counter
- `curl POST` to PR endpoints → `tengu_git_operation{operation: 'pr_create'}`

**Git command regex:** Tolerates global options between `git` and subcommand (e.g. `git -c commit.gpgsign=false commit`).

---

### 22.3 tools/shared/spawnMultiAgent.ts

Shared module for teammate/subagent creation, extracted from TeammateTool for reuse by AgentTool.

**Key functions:**

```typescript
// Internal helper
function getDefaultTeammateModel(leaderModel: string | null): string
// Checks globalConfig.teammateDefaultModel; null → follow leader; undefined → use hardcoded fallback
```

**Backend types:**
- `in-process`: Spawns teammate as in-process coroutine (no external process)
- Tmux-based pane: Spawns in new tmux pane within swarm session
- External process backends

**Detection:**
- `detectAndGetBackend()`: Probes available backends
- `isInProcessEnabled()`: Checks if in-process spawning is available
- `isTmuxAvailable()`: Checks tmux availability for pane backend

**Environment inheritance:**
- `buildInheritedEnvVars()`: Builds environment variable set for spawned teammate
- Key env vars propagated: `TEAMMATE_COMMAND_ENV_VAR`, model overrides, plugin paths, etc.

---

## 23. Testing Utilities

### TestingPermissionTool

**Tool name:** `TestingPermission`
**Source:** `src/tools/testing/TestingPermissionTool.tsx`
**Gate:** Only enabled when `NODE_ENV === 'test'` (hardcoded: `"production" === 'test'` → always disabled in production)

```typescript
export const TestingPermissionTool: Tool<InputSchema, string>
```

**Input:** `{}` (empty object)
**Output:** `'TestingPermission executed successfully'`

**Behavior:**
- Always returns `{ behavior: 'ask', message: 'Run test?' }` from `checkPermissions()`
- Used for end-to-end permission dialog testing
- All render functions return `null`
- `isConcurrencySafe: true`, `isReadOnly: true`
- Never appears in production tool list (disabled at build time)

---

## Appendix: Tool Name Constants

| Constant | Value | Source |
|----------|-------|--------|
| `BASH_TOOL_NAME` | `'Bash'` | `BashTool/toolName.ts` |
| `FILE_READ_TOOL_NAME` | `'Read'` | `FileReadTool/prompt.ts` |
| `FILE_WRITE_TOOL_NAME` | `'Write'` | `FileWriteTool/prompt.ts` |
| `FILE_EDIT_TOOL_NAME` | `'Edit'` | `FileEditTool/constants.ts` |
| `GLOB_TOOL_NAME` | `'Glob'` | `GlobTool/prompt.ts` |
| `GREP_TOOL_NAME` | `'Grep'` | `GrepTool/prompt.ts` |
| `AGENT_TOOL_NAME` | `'Agent'` | `AgentTool/constants.ts` |
| `LEGACY_AGENT_TOOL_NAME` | `'Task'` | `AgentTool/constants.ts` |
| `NOTEBOOK_EDIT_TOOL_NAME` | `'NotebookEdit'` | `NotebookEditTool/constants.ts` |
| `TASK_OUTPUT_TOOL_NAME` | `'TaskOutput'` | `TaskOutputTool/` |
| `ASK_USER_QUESTION_TOOL_NAME` | `'AskUserQuestion'` | `AskUserQuestionTool/` |
| `SKILL_TOOL_NAME` | `'Skill'` | `SkillTool/` |
| `TOOL_SEARCH_TOOL_NAME` | `'ToolSearch'` | `ToolSearchTool/` |
| `CONFIG_TOOL_NAME` | `'Config'` | `ConfigTool/` |
| `BRIEF_TOOL_NAME` | `'SendUserMessage'` | `BriefTool/` |
| `SLEEP_TOOL_NAME` | `'Sleep'` | `SleepTool/prompt.ts` |
| `REMOTE_TRIGGER_TOOL_NAME` | (from `prompt.ts`) | `RemoteTriggerTool/prompt.ts` |
| `ENTER_WORKTREE_TOOL_NAME` | (from `constants.ts`) | `EnterWorktreeTool/constants.ts` |
| `EXIT_WORKTREE_TOOL_NAME` | (from `constants.ts`) | `ExitWorktreeTool/constants.ts` |
| `TEAM_DELETE_TOOL_NAME` | (from `constants.ts`) | `TeamDeleteTool/constants.ts` |
| `CRON_CREATE_TOOL_NAME` | (from `prompt.ts`) | `ScheduleCronTool/prompt.ts` |
| `CRON_DELETE_TOOL_NAME` | (from `prompt.ts`) | `ScheduleCronTool/prompt.ts` |
| `CRON_LIST_TOOL_NAME` | (from `prompt.ts`) | `ScheduleCronTool/prompt.ts` |
| `EXIT_PLAN_MODE_V2_TOOL_NAME` | `'ExitPlanMode'` | `ExitPlanModeTool/` |
| `LIST_MCP_RESOURCES_TOOL_NAME` | `'mcp__listResources'` | `ListMcpResourcesTool/` |
| `POWERSHELL_TOOL_NAME` | `'PowerShell'` | `PowerShellTool/toolName.ts` |
| `REPL_TOOL_NAME` | `'REPL'` | `REPLTool/constants.ts` |
| `LSP_TOOL_NAME` | `'LSP'` | `LSPTool/` |

---

## Appendix: Tool Feature Gates Summary

| Tool | Gate / Condition |
|------|-----------------|
| `ConfigTool`, `REPLTool` | `USER_TYPE === 'ant'` |
| `CronCreate/Delete/List` | `feature('KAIROS')` + `isKairosCronEnabled()` GB gate |
| `SleepTool` | `feature('SLEEP_TOOL')` |
| `RemoteTriggerTool` | `feature('AGENT_TRIGGERS')` + `tengu_surreal_dali` GB flag + `allow_remote_sessions` policy |
| `BriefTool` | `feature('KAIROS')` or `feature('KAIROS_BRIEF')` + GB gate + userMsgOptIn or kairosActive |
| `TeamCreate/Delete`, `SendMessage` | `isAgentSwarmsEnabled()` |
| `TaskCreate/Get/Update/List` | `isTodoV2Enabled()` |
| `TodoWriteTool` | `!isTodoV2Enabled()` |
| `LSPTool` | `ENABLE_LSP_TOOL=true` env var |
| `TestingPermissionTool` | `NODE_ENV === 'test'` (always disabled in production) |
| `MonitorMcpTask` | `feature('MONITOR_TOOL')` |
| `LocalWorkflowTask` | `feature('WORKFLOW_SCRIPTS')` |

---

## Appendix: deferred vs. alwaysLoad

Tools with `shouldDefer: true` are hidden from the initial prompt context to save tokens. The model discovers them via `ToolSearchTool` using keyword search or `select:<name>` queries. `ToolSearchTool` injects `tool_reference` blocks for matched tools, which causes the tool schemas to be loaded into context.

Tools with `alwaysLoad: true` are always included even in contexts where deferral is the default.

Tools without either flag are included in the initial prompt by default.
