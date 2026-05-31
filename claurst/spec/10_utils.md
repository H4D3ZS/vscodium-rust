# Claude Code — Utilities

This document covers the entire `src/utils/` directory (564 files across 36 subdirectories) plus `src/interactiveHelpers.tsx` and `src/ink.ts`.

## Table of Contents

1. [Core Data Structures](#1-core-data-structures)
2. [Shell & Process Execution](#2-shell--process-execution)
3. [Bash Parsing & Analysis](#3-bash-parsing--analysis)
4. [Configuration & Settings](#4-configuration--settings)
5. [Authentication & Auth Management](#5-authentication--auth-management)
6. [Permissions System](#6-permissions-system)
7. [File & Filesystem Utilities](#7-file--filesystem-utilities)
8. [Git Utilities](#8-git-utilities)
9. [Session Management](#9-session-management)
10. [Message & Content Utilities](#10-message--content-utilities)
11. [Model Utilities](#11-model-utilities)
12. [MCP Utilities](#12-mcp-utilities)
13. [Swarm / Multi-Agent System](#13-swarm--multi-agent-system)
14. [Task & Output Management](#14-task--output-management)
15. [Process User Input](#15-process-user-input)
16. [Hooks System](#16-hooks-system)
17. [Analytics & Telemetry](#17-analytics--telemetry)
18. [Background & Cron Systems](#18-background--cron-systems)
19. [Computer Use](#19-computer-use)
20. [Claude-in-Chrome](#20-claude-in-chrome)
21. [Deep Link System](#21-deep-link-system)
22. [Plugin Utilities](#22-plugin-utilities)
23. [Swarm Backends](#23-swarm-backends)
24. [Skills & Suggestions](#24-skills--suggestions)
25. [Todo System](#25-todo-system)
26. [Teleport System](#26-teleport-system)
27. [Memory Utilities](#27-memory-utilities)
28. [Sandbox Utilities](#28-sandbox-utilities)
29. [Security & Secure Storage](#29-security--secure-storage)
30. [Native Installer](#30-native-installer)
31. [General Utilities (Top-Level)](#31-general-utilities-top-level)
32. [interactiveHelpers.tsx](#32-interactivehelperstsx)
33. [ink.ts (Top-Level Module)](#33-inkts-top-level-module)
34. [utils/shell/ — Shell Provider Abstraction](#34-utilsshell--shell-provider-abstraction)
35. [utils/ultraplan/ — Ultraplan Remote Session Polling](#35-utilsultraplan--ultraplan-remote-session-polling)

---

## 1. Core Data Structures

### `CircularBuffer.ts`
Fixed-size circular buffer that auto-evicts oldest items when full.

```typescript
export class CircularBuffer<T> {
  constructor(private capacity: number)
  add(item: T): void           // Adds item, evicts oldest if full
  addAll(items: T[]): void     // Bulk add
  getRecent(count: number): T[] // N most recent items
  toArray(): T[]               // All items oldest-to-newest
  clear(): void
  length(): number
}
```

### `QueryGuard.ts`
Synchronous state machine for query lifecycle (`idle` / `dispatching` / `running`). Compatible with React's `useSyncExternalStore`.

```typescript
export class QueryGuard {
  reserve(): boolean           // idle → dispatching; returns false if not idle
  cancelReservation(): void    // dispatching → idle
  tryStart(): number | null    // → running; returns generation or null if running
  end(generation: number): boolean  // running → idle; returns false if stale
  forceEnd(): void             // kills any running query
  get isActive(): boolean      // true for dispatching OR running
  get generation(): number
  subscribe: (listener) => () => void  // useSyncExternalStore subscribe
  getSnapshot: () => boolean           // useSyncExternalStore snapshot
}
```

### `signal.ts`
Lightweight event signal primitive (no state storage, just notifications).

```typescript
export type Signal<Args extends unknown[] = []> = {
  subscribe: (listener: (...args: Args) => void) => () => void
  emit: (...args: Args) => void
  clear: () => void
}

export function createSignal<Args extends unknown[] = []>(): Signal<Args>
```
Used ~15× across the codebase for pub/sub of state-change events.

### `memoize.ts`
Advanced memoization utilities beyond lodash.

```typescript
// Write-through cache with TTL and background refresh
export function memoizeWithTTL<Args, Result>(
  f: (...args: Args) => Result,
  cacheLifetimeMs?: number  // default 5 minutes
): MemoizedFunction<Args, Result>

// Async version with in-flight dedup
export function memoizeWithTTLAsync<Args, Result>(
  f: (...args: Args) => Promise<Result>,
  cacheLifetimeMs?: number
): ((...args) => Promise<Result>) & { cache: { clear() } }

// LRU-eviction memoize (prevents unbounded memory growth)
export function memoizeWithLRU<Args, Result>(
  f: (...args: Args) => Result,
  cacheFn: (...args: Args) => string,  // key generator
  maxCacheSize?: number  // default 100
): LRUMemoizedFunction<Args, Result>
```

### `generators.ts`
Async generator utilities.

```typescript
export async function lastX<A>(as: AsyncGenerator<A>): Promise<A>
export async function returnValue<A>(as: AsyncGenerator<unknown, A>): Promise<A>
export async function* all<A>(
  generators: AsyncGenerator<A, void>[],
  concurrencyCap?: number  // default Infinity
): AsyncGenerator<A, void>
export async function toArray<A>(generator: AsyncGenerator<A>): Promise<A[]>
export async function* fromArray<T>(values: T[]): AsyncGenerator<T, void>
```

### `array.ts`
Array utility helpers (deduplicate, group, etc.).

### `set.ts`
Set utility helpers.

### `sequential.ts`
Sequential execution helpers.

### `objectGroupBy.ts`
`Object.groupBy` polyfill/wrapper.

---

## 2. Shell & Process Execution

### `Shell.ts`
Core shell execution engine. The most important utility file.

**Exports:**
```typescript
export type ShellConfig = { provider: ShellProvider }
export type ExecOptions = {
  timeout?: number
  onProgress?: (lastLines, allLines, totalLines, totalBytes, isIncomplete) => void
  preventCwdChanges?: boolean
  shouldUseSandbox?: boolean
  shouldAutoBackground?: boolean
  onStdout?: (data: string) => void  // pipe mode callback
}

export async function findSuitableShell(): Promise<string>
// Priority: CLAUDE_CODE_SHELL env > $SHELL > zsh > bash

export const getShellConfig: typeof getShellConfigImpl  // memoized
export const getPsProvider: () => Promise<ShellProvider>  // memoized

export async function exec(
  command: string,
  abortSignal: AbortSignal,
  shellType: ShellType,
  options?: ExecOptions
): Promise<ShellCommand>

export function setCwd(path: string, relativeTo?: string): void
export type { ExecResult } from './ShellCommand.js'
```

**Key Implementation Details:**
- DEFAULT_TIMEOUT = 30 minutes
- File mode (bash): stdout+stderr both go to a single file fd (atomic O_APPEND)
- Pipe mode (hooks, `onStdout`): data flows through StreamWrapper → TaskOutput in-memory
- CWD tracking: captures `pwd -P` output to temp file after each command
- Sandbox wrapping: calls `SandboxManager.wrapWithSandbox()` when `shouldUseSandbox`
- Sandboxed PowerShell uses `/bin/sh` as outer shell with base64-encoded inner command
- After command: reads cwd file, updates state, notifies hooks, cleans up
- Windows path handling via `posixPathToWindowsPath()`

### `ShellCommand.ts`
Wraps a child process into a `ShellCommand` interface.

**Exports:**
```typescript
export type ExecResult = {
  stdout: string
  stderr: string
  code: number
  interrupted: boolean
  backgroundTaskId?: string
  backgroundedByUser?: boolean
  assistantAutoBackgrounded?: boolean
  outputFilePath?: string    // when stdout was too large for inline
  outputFileSize?: number
  outputTaskId?: string
  preSpawnError?: string
}

export type ShellCommand = {
  background(backgroundTaskId: string): boolean  // runningbg
  result: Promise<ExecResult>
  kill(): void
  status: 'running' | 'backgrounded' | 'completed' | 'killed'
  cleanup(): void  // removes event listeners
  onTimeout?: (callback: (backgroundFn) => void) => void
  taskOutput: TaskOutput
}

export function wrapSpawn(
  childProcess: ChildProcess,
  abortSignal: AbortSignal,
  timeout: number,
  taskOutput: TaskOutput,
  shouldAutoBackground?: boolean,
  maxOutputBytes?: number
): ShellCommand

export function createAbortedCommand(
  backgroundTaskId?: string,
  opts?: { stderr?: string; code?: number }
): ShellCommand

export function createFailedCommand(preSpawnError: string): ShellCommand
```

**Key Details:**
- SIGKILL = 137, SIGTERM = 143
- SIZE_WATCHDOG_INTERVAL_MS = 5,000ms — kills backgrounded processes over disk limit
- Uses `treeKill` to kill entire process trees
- Abort reason `'interrupt'` allows backgrounding instead of killing
- StreamWrapper: thin pipe from ChildProcess stdout/stderr → TaskOutput (pipe mode only)

### `abortController.ts`
Memory-safe AbortController utilities using WeakRef.

```typescript
export function createAbortController(maxListeners?: number): AbortController

export function createChildAbortController(
  parent: AbortController,
  maxListeners?: number
): AbortController
// Child aborts when parent aborts (not vice versa)
// Uses WeakRef — dropped children can be GC'd
// Auto-removes parent listener when child aborts
```

### `combinedAbortSignal.ts`
Creates an AbortSignal that fires when any of multiple signals fires.

### `execFileNoThrow.ts` / `execFileNoThrowPortable.ts`
```typescript
export async function execFileNoThrow(
  file: string, args?: string[], options?: ExecFileOptions
): Promise<{ stdout: string; stderr: string; code: number }>

export function execSyncWithDefaults_DEPRECATED(
  command: string
): string | null
```

### `execSyncWrapper.ts`
Synchronous shell execution wrapper with error handling.

### `genericProcessUtils.ts`
Generic child process utilities (spawn, kill, etc.).

### `gracefulShutdown.ts`
```typescript
export function gracefulShutdown(exitCode?: number): Promise<never>
export function gracefulShutdownSync(exitCode?: number): never
export function registerShutdownHandler(fn: () => void | Promise<void>): void
```

### `process.ts`
Process-level utilities.

### `subprocessEnv.ts`
```typescript
export function subprocessEnv(): NodeJS.ProcessEnv
// Returns filtered process.env suitable for subprocess spawning
```

---

## 3. Bash Parsing & Analysis

### `bash/ParsedCommand.ts`
```typescript
export type ParsedCommand = {
  command: string
  args: string[]
  redirects: Redirect[]
  // ... more fields
}
export function parseBashCommand(input: string): ParsedCommand | null
```

### `bash/ShellSnapshot.ts`
Captures shell environment state (env vars, aliases) for reproduction.

### `bash/ast.ts`
Bash AST node types for the parser.

### `bash/bashParser.ts`
Full bash command parser. Handles pipes, redirections, compound commands.

### `bash/bashPipeCommand.ts`
Represents a pipe-connected command chain.

### `bash/commands.ts`
```typescript
export function extractOutputRedirections(command: string): string[]
export function getCommandName(command: string): string
export function isDestructiveCommand(command: string): boolean
```

### `bash/heredoc.ts`
Heredoc detection and parsing.

### `bash/parser.ts`
Low-level token parser for bash syntax.

### `bash/prefix.ts`
Handles shell variable prefix syntax (`VAR=value cmd`).

### `bash/registry.ts`
Registry of known bash commands and their semantics.

### `bash/shellCompletion.ts`
Shell completion suggestions for partial commands.

### `bash/shellPrefix.ts`
```typescript
export function formatShellPrefixCommand(command: string, envFile: string): string
```

### `bash/shellQuote.ts` / `bash/shellQuoting.ts`
Shell quoting utilities to safely embed strings in shell commands.

### `bash/specs/`
Command-specific completion specs: `alias.ts`, `nohup.ts`, `pyright.ts`, `sleep.ts`, `srun.ts`, `time.ts`, `timeout.ts`.

### `bash/treeSitterAnalysis.ts`
Tree-sitter based deep bash AST analysis for command classification.

---

## 4. Configuration & Settings

### `config.ts`
The central configuration file. Manages global config (per-user), project config (per-project), and session config.

**Key Types:**
```typescript
export type PastedContent = {
  id: number; type: 'text' | 'image'; content: string
  mediaType?: string; filename?: string; dimensions?: ImageDimensions
  sourcePath?: string
}

export interface HistoryEntry {
  display: string
  pastedContents: Record<number, PastedContent>
}

export type ReleaseChannel = 'stable' | 'latest'

export type ProjectConfig = {
  allowedTools: string[]
  mcpContextUris: string[]
  mcpServers?: Record<string, McpServerConfig>
  // + many more fields
}

export type GlobalConfig = {
  // Account info, oauth tokens, model preferences, theme, etc.
}

export type AccountInfo = {
  emailAddress: string
  // + subscription/billing fields
}
```

**Key Exports:**
```typescript
export function getGlobalConfig(): GlobalConfig
export function saveGlobalConfig(update: (current: GlobalConfig) => GlobalConfig): void
export function getProjectConfig(cwd?: string): ProjectConfig
export function saveProjectConfig(update: (current: ProjectConfig) => ProjectConfig, cwd?: string): void
export function checkHasTrustDialogAccepted(): boolean
export function getCustomApiKeyStatus(): 'user' | 'project' | 'env' | 'none'
export function normalizePathForConfigKey(p: string): string
```

### `configConstants.ts`
Config-related constants (file names, paths, defaults).

### `settings/settings.ts`
User settings with multi-source merge (user file, project file, MDM, managed settings).

**Key Exports:**
```typescript
export function getSettings_DEPRECATED(): SettingsJson
export function getSettingsForSource(source: SettingSource): SettingsJson
export function saveSettings(update: (current: SettingsJson) => SettingsJson, source?: EditableSettingSource): void
export function hasAutoModeOptIn(): boolean
export function hasSkipDangerousModePermissionPrompt(): boolean
```

### `settings/types.ts`
`SettingsJson` and `SettingsSchema` (Zod schema) defining all user-configurable settings.

### `settings/constants.ts`
```typescript
export type SettingSource = 'userSettings' | 'projectSettings' | 'sessionSettings' | 'managed' | 'mdm' | 'remoteManagedSettings'
export type EditableSettingSource = 'userSettings' | 'projectSettings' | 'sessionSettings'
export const SETTING_SOURCES: SettingSource[]
```

### `settings/validation.ts`
Settings validation (Zod-based), permission rule filtering, validation tips.

### `settings/settingsCache.ts`
Multi-layer settings cache keyed by source.

### `settings/changeDetector.ts`
Detects when settings change and fires notifications.

### `settings/applySettingsChange.ts`
Applies incremental settings updates.

### `settings/mdm/`
MDM (Mobile Device Management) policy settings:
- `settings.ts`: reads from Windows HKCU registry and macOS plist
- `rawRead.ts`: low-level MDM config reading
- `constants.ts`: MDM-specific constants

### `settings/managedPath.ts`
```typescript
export function getManagedFilePath(): string
export function getManagedSettingsDropInDir(): string
```

### `settings/allErrors.ts`
```typescript
export function getSettingsWithAllErrors(): SettingsWithErrors
```

### `settings/permissionValidation.ts`
Validates permission rule configurations in settings.

### `settings/toolValidationConfig.ts`
Tool-level validation configuration.

### `settings/validateEditTool.ts`
Validates EditTool-specific settings.

### `settings/schemaOutput.ts`
Generates JSON Schema from the settings Zod schema.

### `settings/pluginOnlyPolicy.ts`
Enforces that certain settings can only be configured via plugins.

---

## 5. Authentication & Auth Management

### `auth.ts`
Main authentication orchestration. Handles API key, OAuth, and AWS auth.

**Key Exports:**
```typescript
export function getSubscriptionType(): SubscriptionType | null
export function isClaudeAISubscriber(): boolean
export function isProSubscriber(): boolean
export function isMaxSubscriber(): boolean
export function isTeamPremiumSubscriber(): boolean
export async function getAuthHeaders(): Promise<Record<string, string>>
export async function logout(): Promise<void>
export async function refreshAuth(): Promise<void>
```

### `authPortable.ts`
Cross-platform auth utilities.

```typescript
export function normalizeApiKeyForConfig(key: string): string
export async function maybeRemoveApiKeyFromMacOSKeychainThrows(): Promise<void>
```

### `authFileDescriptor.ts`
Reads API key / OAuth token from file descriptor (for CI/headless use).

```typescript
export function getApiKeyFromFileDescriptor(): string | null
export function getOAuthTokenFromFileDescriptor(): OAuthTokens | null
```

### `aws.ts`
AWS credential management and STS caller identity checking.

```typescript
export async function checkStsCallerIdentity(profile?: string): Promise<AwsCallerIdentity>
export function isValidAwsStsOutput(output: string): boolean
export function clearAwsIniCache(): void
```

### `awsAuthStatusManager.ts`
```typescript
export class AwsAuthStatusManager {
  getStatus(): AwsAuthStatus
  // Manages AWS auth lifecycle with refresh
}
```

### `betas.ts`
Beta feature flag management tied to account subscription.

```typescript
export function clearBetasCaches(): void
export async function getBetaFeatures(): Promise<string[]>
```

### `billing.ts`
Billing-related utilities (subscription type detection, extra-usage checks).

---

## 6. Permissions System

### `permissions/permissions.ts`
Core permission checking engine.

**Key Logic:**
- Integrates `TRANSCRIPT_CLASSIFIER` feature flag for AI-based permission decisions
- Checks permission rules from settings, project config, session config
- Handles `bypassPermissionsKillswitch`
- Manages permission modes (normal, bypass, plan, auto)
- Returns `PermissionDecision` with reason for UI display

**Key Exports:**
```typescript
export async function getPermissionDecision(
  tool: Tool,
  input: AnyObject,
  context: ToolPermissionContext
): Promise<PermissionDecision>

export async function applyPermissionUpdate(
  update: PermissionUpdate,
  destination: PermissionUpdateDestination
): Promise<void>

export async function applyPermissionUpdates(updates: PermissionUpdate[]): Promise<void>
export async function persistPermissionUpdates(updates: PermissionUpdate[]): Promise<void>
export function deletePermissionRuleFromSettings(rule: PermissionRuleFromEditableSettings): void
```

### `permissions/PermissionMode.ts`
```typescript
export type PermissionMode = 'default' | 'acceptEdits' | 'bypassPermissions' | 'plan'
export function permissionModeTitle(mode: PermissionMode): string
```

### `permissions/PermissionResult.ts`
```typescript
export type PermissionDecision =
  | PermissionAskDecision    // { type: 'ask', message, ... }
  | PermissionDenyDecision   // { type: 'deny', reason, ... }
  | { type: 'allow' }
  | { type: 'allowWithSandbox' }
```

### `permissions/PermissionRule.ts`
```typescript
export type PermissionBehavior = 'allow' | 'deny'
export type PermissionRuleValue = string  // e.g., "Bash(rm:*)"
export type PermissionRule = {
  ruleValue: PermissionRuleValue
  behavior: PermissionBehavior
  source: PermissionRuleSource
}
```

### `permissions/PermissionUpdate.ts` / `PermissionUpdateSchema.ts`
Types and logic for updating permission rules in settings.

### `permissions/bashClassifier.ts`
Classifies bash commands for permission decisions (uses patterns and optionally AI).

### `permissions/yoloClassifier.ts`
Classifier for "bypass permissions" mode — auto-approves most commands.

### `permissions/classifierDecision.ts`
AI-based transcript classifier for permission decisions (feature-gated by `TRANSCRIPT_CLASSIFIER`).

### `permissions/classifierShared.ts`
Shared types and utilities for classifiers.

### `permissions/dangerousPatterns.ts`
List of dangerous bash patterns that always require explicit approval.

```typescript
export const DANGEROUS_PATTERNS: RegExp[]
export function hasDangerousPattern(command: string): boolean
```

### `permissions/denialTracking.ts`
Tracks recent permission denials for UI display in `/permissions recent-denials`.

### `permissions/filesystem.ts`
```typescript
export function getClaudeTempDirName(): string
export function validateFilesystemAccess(path: string, mode: 'read' | 'write'): boolean
```

### `permissions/getNextPermissionMode.ts`
State machine transitions for permission mode cycling.

### `permissions/pathValidation.ts`
Validates file paths against workspace directories.

### `permissions/permissionExplainer.ts`
Generates human-readable explanations for permission decisions.

### `permissions/permissionRuleParser.ts`
```typescript
export function permissionRuleValueFromString(s: string): PermissionRuleValue
export function permissionRuleValueToString(v: PermissionRuleValue): string
```

### `permissions/permissionSetup.ts`
Initial permission setup at session start.

### `permissions/permissionsLoader.ts`
Loads and merges permission rules from all sources.

### `permissions/shadowedRuleDetection.ts`
Detects when a permission rule is shadowed/overridden by another rule.

### `permissions/shellRuleMatching.ts`
Matches shell commands against permission rule patterns (glob-style).

### `permissions/autoModeState.ts`
State management for auto-approve mode.

### `permissions/bypassPermissionsKillswitch.ts`
Emergency killswitch for bypass-permissions mode.

---

## 7. File & Filesystem Utilities

### `file.ts`
Core file utilities.

```typescript
export async function pathExists(p: string): Promise<boolean>
export function writeFileSyncAndFlush_DEPRECATED(path: string, content: string): void
export async function safeReadFile(path: string): Promise<string | null>
```

### `fileRead.ts`
```typescript
export function readFileSync(path: string, options?: { encoding?: 'utf8' }): string
// NFC-normalizes content on read (macOS APFS compat)
```

### `fileReadCache.ts`
Content-addressed file read cache with invalidation.

### `fileStateCache.ts`
Caches file stat information to avoid repeated fs.stat calls.

### `fsOperations.ts`
```typescript
export function getFsImplementation(): FsImplementation
// Returns real fs or test mock

export async function readFileRange(
  path: string, start: number, end: number
): Promise<string>

export async function tailFile(
  path: string, maxBytes?: number
): Promise<string>
```

### `fileHistory.ts`
File edit history tracking for `/rewind` and undo operations.

### `fileOperationAnalytics.ts`
Tracks file operation analytics (reads, writes, edits per turn).

### `filePersistence/filePersistence.ts`
Persistence of file state across sessions.

### `filePersistence/outputsScanner.ts`
Scans output files for persistence.

### `readFileInRange.ts`
Reads a specific byte range from a file.

### `readEditContext.ts`
Builds context for file read/edit operations (surrounding lines, etc.).

### `generatedFiles.ts`
Detection of generated/auto-generated files (to skip in certain operations).

### `path.ts`
Path manipulation utilities.

```typescript
export function normalizePathForConfigKey(p: string): string
export function toRelativePath(absolute: string, base?: string): string
export function expandHome(p: string): string
```

### `xdg.ts`
XDG base directory spec utilities.

### `systemDirectories.ts`
Platform-specific system directory paths.

### `cachePaths.ts`
Cache directory paths for various cached data.

### `tempfile.ts`
```typescript
export async function createTempFile(suffix?: string): Promise<string>
export function cleanupTempFiles(): void
```

### `lockfile.ts`
File-based distributed locking.

```typescript
export async function lock(path: string, options?: LockOptions): Promise<() => void>
export function tryLock(path: string): boolean
```

### `cleanupRegistry.ts`
```typescript
export function registerCleanup(fn: () => void | Promise<void>): void
export async function runCleanup(): Promise<void>
```

### `cleanup.ts`
Session cleanup utilities.

### `glob.ts`
Glob pattern matching utilities (wraps glob library).

### `ripgrep.ts`
Ripgrep integration for file content searching.

---

## 8. Git Utilities

### `git.ts`
Primary git utility module.

**Key Exports:**
```typescript
export function findGitRoot(startPath?: string): string | null
export function findCanonicalGitRoot(startPath?: string): string | null
export async function getGitBranch(cwd?: string): Promise<string | null>
export async function getGitStatus(cwd?: string): Promise<string>
export async function getGitDiff(args?: string[]): Promise<string>
export async function isGitRepo(cwd?: string): Promise<boolean>
```

Uses LRU memoization (`memoizeWithLRU`) for root detection.
NFC-normalizes paths for macOS APFS compatibility.

### `git/gitFilesystem.ts`
Low-level git filesystem operations.

```typescript
export function getCachedBranch(cwd: string): string | null
export function getCachedHead(cwd: string): string | null
export function getCachedDefaultBranch(cwd: string): string | null
export function getCachedRemoteUrl(cwd: string): string | null
export function getWorktreeCountFromFs(cwd: string): number
export function isShallowClone(cwd: string): boolean
export function resolveGitDir(cwd: string): string | null
```

### `git/gitConfigParser.ts`
Parses `.git/config` files.

### `git/gitignore.ts`
```typescript
export async function addFileGlobRuleToGitignore(
  pattern: string,
  gitRoot: string
): Promise<void>
export function isIgnoredByGitignore(path: string, cwd: string): boolean
```

### `gitDiff.ts`
Git diff parsing and formatting utilities.

### `gitSettings.ts`
Git configuration utilities for Claude Code settings.

### `ghPrStatus.ts`
GitHub PR status checking via `gh` CLI.

### `github/ghAuthStatus.ts`
GitHub authentication status checking.

### `githubRepoPathMapping.ts`
Maps GitHub repo URLs to local filesystem paths.

### `detectRepository.ts`
Detects repository type (git, mercurial, etc.) and metadata.

### `getWorktreePaths.ts` / `getWorktreePathsPortable.ts`
```typescript
export async function getWorktreePaths(): Promise<string[]>
// Lists all git worktree paths for the current repo
```

### `worktree.ts`
Git worktree management utilities.

### `worktreeModeEnabled.ts`
Feature flag check for worktree mode.

### `commitAttribution.ts`
Adds Claude Code attribution to git commits.

---

## 9. Session Management

### `sessionStorage.ts`
Core session storage — reading/writing JSONL transcript files.

**Key Exports:**
```typescript
export function getTranscriptPathForSession(sessionId: string, cwd?: string): string
export function getAgentTranscriptPath(agentId: AgentId, sessionId: string): string
export async function appendToSession(entry: Entry): Promise<void>
export async function readSession(sessionId: string): Promise<Entry[]>
export async function listSessions(cwd?: string): Promise<SessionSummary[]>
export async function deleteSession(sessionId: string): Promise<void>
export function switchSession(newSessionId: string): void
```

### `sessionStoragePortable.ts`
Portable version of session storage (cross-platform paths).

### `sessionState.ts`
In-memory session state (current session ID, title, etc.).

### `sessionStart.ts`
Session initialization logic.

### `sessionRestore.ts`
Restores a session from its JSONL transcript.

### `sessionActivity.ts`
Tracks session activity for idle detection.

### `sessionEnvVars.ts`
Environment variables injected into the session context.

### `sessionEnvironment.ts`
```typescript
export function getHookEnvFilePath(): string
export function invalidateSessionEnvCache(): void
export async function getSessionEnvironmentVariables(): Promise<Record<string, string>>
```

### `sessionFileAccessHooks.ts`
Hooks for tracking which files were accessed in a session.

### `sessionIngressAuth.ts`
Auth for incoming session connections (bridge, remote).

### `sessionTitle.ts`
```typescript
export async function generateSessionTitle(messages: Message[]): Promise<string>
```

### `sessionUrl.ts`
URL construction for remote session references.

### `listSessionsImpl.ts`
Implementation of session listing with sorting and filtering.

### `agenticSessionSearch.ts`
Search within session transcripts.

### `crossProjectResume.ts`
Handles resuming sessions across different project directories.

### `concurrentSessions.ts`
Manages multiple concurrent Claude Code sessions.

### `cwd.ts`
```typescript
export function pwd(): string              // Current working directory
export function getCwd(): string           // Alias
export function isValidCwd(path: string): boolean
```

---

## 10. Message & Content Utilities

### `messages.ts`
Core message manipulation — the most-used utility in the codebase.

**Key Exports:**
```typescript
export function getContentText(content: ContentBlockParam[]): string
export function normalizeMessages(messages: Message[]): NormalizedMessage[]
export function isUserMessage(msg: Message): msg is UserMessage
export function isAssistantMessage(msg: Message): msg is AssistantMessage
export function extractToolUses(messages: Message[]): ToolUseBlock[]
export function getLastAssistantMessage(messages: Message[]): AssistantMessage | null
export function createSystemMessage(text: string): SystemMessage
export function createUserMessage(content: ContentBlockParam[]): UserMessage
```

### `contentArray.ts`
Utilities for working with `ContentBlockParam[]` arrays.

### `messagePredicates.ts`
Type guards and predicates for message types.

### `messageQueueManager.ts`
Queue management for batched message processing.

### `messages/` (subdirectory)
Message-category-specific utilities:
- `messages/` — subdirectory with specialized message handling utilities

### `attachments.ts`
Manages message attachments (images, files, pastes).

```typescript
export function createAttachmentMessage(content: AttachmentContent): AttachmentMessage
export function getAttachmentMessages(messages: Message[]): AttachmentMessage[]
export type AgentMentionAttachment = { agentId: string; agentName: string }
```

### `collapseBackgroundBashNotifications.ts`
Collapses multiple bash progress notifications into summaries.

### `collapseHookSummaries.ts`
Collapses hook execution summaries.

### `collapseReadSearch.ts`
Collapses file read/search tool uses for compaction.

### `collapseTeammateShutdowns.ts`
Collapses teammate shutdown messages.

### `groupToolUses.ts`
Groups sequential tool use messages for display.

### `controlMessageCompat.ts`
Compatibility layer for control messages across versions.

### `directMemberMessage.ts`
Direct messaging between swarm members.

### `images` utilities:
- `imagePaste.ts` — clipboard image paste handling
- `imageResizer.ts` — resizes/downsamples large images
- `imageStore.ts` — temporary image storage keyed by UUID
- `imageValidation.ts` — validates image format/size
- `imageProcessor` (in FileReadTool) — processes image files for model input

### `pdf.ts` / `pdfUtils.ts`
PDF file handling — extracts text, validates, handles password protection.

### `notebook.ts`
Jupyter notebook utilities (.ipynb parsing, cell editing).

---

## 11. Model Utilities

### `model/model.ts`
Model selection and configuration.

```typescript
export type ModelShortName = string
export type ModelName = string
export type ModelSetting = ModelName | ModelAlias | null

export function getSmallFastModel(): ModelName  // ANTHROPIC_SMALL_FAST_MODEL or haiku
export function isNonCustomOpusModel(model: ModelName): boolean
export function getModelToUse(options?: ModelOptions): ModelName
// Priority: session override > --model flag > ANTHROPIC_MODEL env > settings > default
```

### `model/aliases.ts`
```typescript
export type ModelAlias = 'claude-sonnet' | 'claude-haiku' | 'claude-opus' | ...
export function isModelAlias(s: string): s is ModelAlias
export function resolveModelAlias(alias: ModelAlias): ModelName
```

### `model/modelStrings.ts`
```typescript
export function getModelStrings(): {
  opus40: string; opus41: string; opus45: string; opus46: string
  sonnet45: string; sonnet46: string
  haiku35: string; haiku40: string
  // ... all model IDs, some conditionally included (ANT-only)
}
export function resolveOverriddenModel(model: ModelSetting): ModelName
```

### `model/modelOptions.ts`
Model option types and construction.

### `model/modelCapabilities.ts`
```typescript
export function supportsVision(model: ModelName): boolean
export function supportsExtendedThinking(model: ModelName): boolean
export function supports1MContext(model: ModelName): boolean
export function modelSupportsAdvisor(model: ModelName): boolean
```

### `model/configs.ts`
Per-model configuration (temperature, top_p, max_tokens, etc.).

### `model/providers.ts`
```typescript
export type APIProvider = 'anthropic' | 'bedrock' | 'vertex'
export function getAPIProvider(): APIProvider
```

### `model/bedrock.ts`
AWS Bedrock-specific model ID conversions.

### `model/antModels.ts`
Internal ANT-only model names (dead-code-eliminated in external builds).

### `model/validateModel.ts`
```typescript
export function validateModel(model: string): ModelName
export function isValidAdvisorModel(model: string): boolean
```

### `model/modelAllowlist.ts`
Runtime allowlist of models (from settings/GrowthBook).

### `model/modelSupportOverrides.ts`
Per-model support overrides for edge cases.

### `model/deprecation.ts`
Model deprecation notices and migration paths.

### `model/contextWindowUpgradeCheck.ts`
Checks if 1M context upgrade is available.

### `model/check1mAccess.ts`
Validates 1M context access for current subscription.

### `model/agent.ts`
Agent-specific model selection.

### `modelCost.ts`
```typescript
export function formatModelPricing(model: ModelName): string
export function getOpus46CostTier(): 'standard' | 'premium'
```

### `context.ts`
Context window management.

```typescript
export function has1mContext(): boolean
export function is1mContextDisabled(): boolean
export function modelSupports1M(model: ModelName): boolean
```

---

## 12. MCP Utilities

### `utils/mcp/dateTimeParser.ts`
Parses date/time values from MCP tool inputs.

### `utils/mcp/elicitationValidation.ts`
Validates MCP elicitation schemas.

### `mcpInstructionsDelta.ts`
Diff-based MCP server instructions management.

### `mcpOutputStorage.ts`
Storage for MCP tool output.

### `mcpValidation.ts`
Validates MCP server configurations.

### `mcpWebSocketTransport.ts`
WebSocket transport implementation for MCP servers.

---

## 13. Swarm / Multi-Agent System

### `swarm/constants.ts`
Swarm system constants (max workers, timeouts, etc.).

### `swarm/inProcessRunner.ts`
Runs swarm workers in the same process (no separate terminal).

### `swarm/spawnInProcess.ts`
Spawns in-process swarm workers.

### `swarm/spawnUtils.ts`
Common utilities for spawning swarm workers.

### `swarm/leaderPermissionBridge.ts`
Bridges permission decisions from leader to workers.

### `swarm/permissionSync.ts`
Synchronizes permission state across swarm members.

### `swarm/reconnection.ts`
Handles reconnection logic when swarm workers disconnect.

### `swarm/teamHelpers.ts`
Helper utilities for team coordination.

### `swarm/teammateInit.ts`
Initialization sequence for swarm teammates.

### `swarm/teammateLayoutManager.ts`
Manages terminal layout for swarm teammates.

### `swarm/teammateModel.ts`
Model selection for swarm teammates.

### `swarm/teammatePromptAddendum.ts`
Adds context to teammate prompts.

### `swarm/backends/types.ts`
```typescript
export type PaneBackend = {
  spawn(config: SpawnConfig): Promise<PaneHandle>
  kill(pane: PaneHandle): Promise<void>
  sendInput(pane: PaneHandle, text: string): Promise<void>
  // ...
}
```

### `swarm/backends/TmuxBackend.ts`
Tmux-based pane backend for swarm.

### `swarm/backends/ITermBackend.ts`
iTerm2-based pane backend for swarm.

### `swarm/backends/InProcessBackend.ts`
In-process (no terminal) backend.

### `swarm/backends/PaneBackendExecutor.ts`
Executes commands in pane backends.

### `swarm/backends/detection.ts`
Detects available pane backends (tmux, iTerm2, etc.).

### `swarm/backends/registry.ts`
Registry of available backends.

### `swarm/backends/teammateModeSnapshot.ts`
Snapshots teammate mode state.

### `swarm/backends/it2Setup.ts` / `It2SetupPrompt.tsx`
iTerm2 setup flow for swarm.

### `agentSwarmsEnabled.ts`
```typescript
export function isAgentSwarmsEnabled(): boolean
```

### `inProcessTeammateHelpers.ts`
Helpers for in-process teammate coordination.

### `teammate.ts` / `teammateContext.ts` / `teammateMailbox.ts`
Teammate state, context, and messaging utilities.

### `teamDiscovery.ts`
Discovers available teammates in the session.

### `teamMemoryOps.ts`
Team memory read/write operations.

### `standaloneAgent.ts`
Runs a standalone agent session programmatically.

### `forkedAgent.ts`
Handles forked agent sessions.

---

## 14. Task & Output Management

### `task/TaskOutput.ts`
Single source of truth for a shell command's output.

```typescript
export class TaskOutput {
  readonly taskId: string
  readonly path: string          // Output file path
  readonly stdoutToFile: boolean // true = file mode, false = pipe mode

  constructor(taskId: string, onProgress: ProgressCallback | null, stdoutToFile?: boolean)

  writeStdout(data: string): void   // pipe mode only
  writeStderr(data: string): void   // pipe mode only
  async getStdout(): Promise<string>
  getStderr(): string
  get outputFileRedundant(): boolean
  get outputFileSize(): number
  spillToDisk(): void               // flush in-memory buffer to disk

  clear(): void
  async deleteOutputFile(): Promise<void>

  // Static registry for polling
  static startPolling(taskId: string): void
  static stopPolling(taskId: string): void
}
```

Uses `CircularBuffer<string>(1000)` for recent lines in file mode.
Polling interval: 1000ms for progress callbacks.
Max in-memory: 8MB before spilling to disk.

### `task/diskOutput.ts`
```typescript
export const MAX_TASK_OUTPUT_BYTES: number     // e.g., 5MB
export const MAX_TASK_OUTPUT_BYTES_DISPLAY: string
export class DiskTaskOutput { ... }
export function getTaskOutputPath(taskId: string): string
export function getTaskOutputDir(): string
```

### `task/framework.ts`
Task execution framework — tracks running/completed/failed tasks.

### `task/outputFormatting.ts`
Formats task output for display (truncation, highlighting).

### `task/sdkProgress.ts`
SDK-level progress reporting for tasks.

### `tasks.ts` (top-level)
Task management utilities.

```typescript
export function getActiveTasks(): ActiveTask[]
export function getTaskById(id: string): ActiveTask | null
```

---

## 15. Process User Input

### `processUserInput/processUserInput.ts`
Main entry point for user input processing.

```typescript
export async function processUserInput(
  input: UserInput,
  context: ProcessContext
): Promise<ProcessResult>
```
Handles dispatch to bash, slash command, or text prompt processors.

### `processUserInput/processBashCommand.tsx`
Handles `!command` syntax — executes shell commands inline.

### `processUserInput/processSlashCommand.tsx`
Handles `/command` syntax — finds and dispatches slash commands.

### `processUserInput/processTextPrompt.ts`
Handles regular text prompts — creates user messages for the model.

### `handlePromptSubmit.ts`
Orchestrates prompt submission with hooks, attachments, queue management.

### `slashCommandParsing.ts`
```typescript
export function parseSlashCommand(input: string): { name: string; args: string } | null
```

### `promptShellExecution.ts`
`executeShellCommandsInPrompt()` — expands `!`backtick syntax in prompt commands.

---

## 16. Hooks System

### `hooks.ts`
Core hooks execution engine. Runs user-configured lifecycle hooks.

**Key Exports:**
```typescript
export type HookEvent =
  | 'PreToolUse' | 'PostToolUse'
  | 'UserPromptSubmit' | 'Notification'
  | 'Stop' | 'SubagentStop'

export async function executePreToolUseHooks(
  toolName: string, input: AnyObject, context: HookContext
): Promise<HookResult>

export async function executePostToolUseHooks(
  toolName: string, input: AnyObject, output: AnyObject, context: HookContext
): Promise<HookResult>

export async function executeUserPromptSubmitHooks(
  prompt: string, context: HookContext
): Promise<HookResult>

export function getUserPromptSubmitHookBlockingMessage(result: HookResult): string | null

export async function executeNotificationHooks(
  message: string
): Promise<void>
```

**Implementation:**
- Reads hooks from settings (`PreToolUse`, `PostToolUse`, `UserPromptSubmit`, `Notification`, `Stop`)
- Uses `wrapSpawn()` for execution (pipe mode)
- Injects env vars: `CLAUDE_TOOL_NAME`, `CLAUDE_TOOL_INPUT`, `TOOL_OUTPUT`, etc.
- Respects `shouldAllowManagedHooksOnly()` and `shouldDisableAllHooksIncludingManaged()`
- Plugin options substituted via `substituteUserConfigVariables()`

### `hooks/fileChangedWatcher.ts`
Watches for file changes triggered by hooks.

```typescript
export async function onCwdChangedForHooks(oldCwd: string, newCwd: string): Promise<void>
```

### `hooks/AsyncHookRegistry.ts`
Registry for async hook handlers.

### `hooks/hooksConfigSnapshot.ts`
Cached hooks configuration snapshot.

---

## 17. Analytics & Telemetry

### `telemetry/` subdirectory
- `telemetry/events.ts` — OpenTelemetry event logging
- Related to OTel span/event emission

### `telemetryAttributes.ts`
Standard telemetry attribute names.

### `headlessProfiler.ts` / `profilerBase.ts`
Performance profiling utilities for headless mode.

### `startupProfiler.ts`
```typescript
export function profileCheckpoint(name: string): void
export function getStartupProfile(): StartupProfile
```

### `queryProfiler.ts`
Profiles individual query performance.

### `slowOperations.ts`
```typescript
export function jsonParse<T>(s: string): T
export function jsonStringify(v: unknown): string
export function clone<T>(v: T): T
// These ops are tracked for performance monitoring
```

### `fpsTracker.ts`
Frame rate tracking for UI performance.

### `heatmap.ts`
Records usage heatmap data for UX analytics.

### `unaryLogging.ts`
Unary gRPC event logging.

### `stats.ts` / `statsCache.ts`
Session statistics (token usage, tool calls, etc.).

---

## 18. Background & Cron Systems

### `cron.ts` / `cronScheduler.ts`
Core cron job scheduling.

```typescript
export function scheduleCron(
  id: string,
  schedule: string,
  fn: () => Promise<void>
): void
export function cancelCron(id: string): void
```

### `cronTasks.ts`
Standard cron task definitions (housekeeping, analytics uploads, etc.).

### `cronTasksLock.ts`
Distributed locking for cron tasks.

### `cronJitterConfig.ts`
Adds jitter to cron schedules to prevent thundering herd.

### `background/remote/remoteSession.ts`
Background remote session management.

### `background/remote/preconditions.ts`
Precondition checks for remote sessions.

### `backgroundHousekeeping.ts`
Background cleanup tasks (temp files, old sessions, etc.).

---

## 19. Computer Use

### `computerUse/common.ts`
Common computer use types and utilities.

### `computerUse/executor.ts`
Executes computer use actions (mouse clicks, typing, screenshots).

### `computerUse/gates.ts`
Feature gates for computer use (entitlement checks).

### `computerUse/hostAdapter.ts`
Platform-specific host adapters (macOS, Linux).

### `computerUse/setup.ts` / `computerUse/setupPortable.ts`
Computer use setup and initialization.

### `computerUse/mcpServer.ts`
Exposes computer use capabilities as an MCP server.

### `computerUse/toolRendering.tsx`
React components for rendering computer use tool results.

### `computerUse/wrapper.tsx`
React wrapper for computer use sessions.

### `computerUse/inputLoader.ts`
Loads computer use input configurations.

### `computerUse/swiftLoader.ts`
macOS Swift bridge for computer use.

### `computerUse/drainRunLoop.ts`
Drains the macOS run loop for proper screenshot timing.

### `computerUse/escHotkey.ts`
ESC hotkey handler for computer use sessions.

### `computerUse/computerUseLock.ts`
Prevents concurrent computer use sessions.

### `computerUse/cleanup.ts`
Cleans up computer use resources.

### `computerUse/appNames.ts`
Platform-specific application name detection.

---

## 20. Claude-in-Chrome

### `claudeInChrome/common.ts`
Common types for Claude-in-Chrome integration.

### `claudeInChrome/chromeNativeHost.ts`
Native host messaging protocol for Chrome extension communication.

### `claudeInChrome/mcpServer.ts`
Exposes Chrome tab content as MCP resources.

### `claudeInChrome/prompt.ts`
System prompt extensions for Chrome integration.

### `claudeInChrome/setup.ts` / `setupPortable.ts`
Chrome native host setup.

### `claudeInChrome/toolRendering.tsx`
React rendering for Chrome-specific tool results.

---

## 21. Deep Link System

### `deepLink/parseDeepLink.ts`
```typescript
export function parseDeepLink(url: string): DeepLinkAction | null
// Parses claude://... URLs
```

### `deepLink/protocolHandler.ts`
Registers and handles the `claude://` protocol.

### `deepLink/registerProtocol.ts`
Registers the deep link protocol with the OS.

### `deepLink/banner.ts`
Shows deep link activation banners.

### `deepLink/terminalLauncher.ts`
Launches terminal sessions from deep links.

### `deepLink/terminalPreference.ts`
```typescript
export function updateDeepLinkTerminalPreference(terminal: string): void
export function getPreferredTerminal(): string | null
```

### `desktopDeepLink.ts`
Desktop app deep link integration.

---

## 22. Plugin Utilities

### `plugins/` subdirectory (in utils)
- `pluginOptionsStorage.ts` — stores/retrieves plugin configuration
- `pluginDirectories.ts` — returns plugin installation directories
- `schemas.ts` — plugin manifest Zod schemas

### `dxt/helpers.ts`
DXT (Desktop Extension) package helpers.

### `dxt/zip.ts`
ZIP archive utilities for DXT packages.

---

## 23. Swarm Backends
(Covered in Section 13)

---

## 24. Skills & Suggestions

### `skills/` subdirectory
Skill-specific utilities:
- `skillUsageTracking.ts` — tracks usage frequency per skill

### `suggestions/commandSearch.ts`
```typescript
export function searchCommands(
  query: string,
  commands: Command[]
): SuggestionItem[]
// Uses Fuse.js with memoized index (keyed by commands array identity)
```

### `contextSuggestions.ts`
AI-driven context suggestions for the prompt input.

### `suggestionFiltering.ts`
Filters suggestions by relevance.

---

## 25. Todo System

### `todo/types.ts`
```typescript
export type TodoItem = {
  id: string
  content: string
  status: 'pending' | 'in_progress' | 'completed'
  priority: 'high' | 'medium' | 'low'
}
export type TodoList = TodoItem[]
```

---

## 26. Teleport System

### `teleport/api.ts`
API calls for the Teleport remote execution system.

### `teleport/environmentSelection.ts`
Selects the remote execution environment.

### `teleport/environments.ts`
Available Teleport environments configuration.

### `teleport/gitBundle.ts`
Creates and transfers git bundles for Teleport sessions.

### `teleport.tsx`
```typescript
export async function teleportToRemote(
  options: TeleportOptions,
  root: Root
): Promise<TeleportResult>
// Launches a remote Claude Code session via CCR
```

---

## 27. Memory Utilities

### `memory/types.ts`
```typescript
export type MemoryType = 'project' | 'user' | 'team'
export type MemoryFile = {
  path: string
  type: MemoryType
  content: string
}
```

### `memoryFileDetection.ts`
Detects and loads relevant memory files for the current context.

### `claudemd.ts`
CLAUDE.md file management (reading, writing, external includes).

```typescript
export function getMemoryFiles(cwd?: string): MemoryFile[]
export function getExternalClaudeMdIncludes(cwd?: string): string[]
export function shouldShowClaudeMdExternalIncludesWarning(): boolean
```

### `markdownConfigLoader.ts`
Loads configuration from CLAUDE.md-style markdown files.

---

## 28. Sandbox Utilities

### `sandbox/sandbox-adapter.ts`
```typescript
export class SandboxManager {
  static async wrapWithSandbox(
    command: string,
    shellBin: string,
    options?: SandboxOptions,
    abortSignal?: AbortSignal
  ): Promise<string>
  static cleanupAfterCommand(): void
  static async refreshConfig(): Promise<void>
}
```

Uses Linux bwrap or macOS sandbox-exec depending on platform.

### `sandbox/` (other files)
- Sandbox configuration management
- Platform-specific sandbox profile generation

---

## 29. Security & Secure Storage

### `secureStorage/` subdirectory
Secure credential storage (macOS Keychain, Windows Credential Manager, Linux secret service).

### `crypto.ts`
```typescript
export function generateSecureToken(length?: number): string
export function hashString(s: string): string
```

### `fingerprint.ts`
Device fingerprinting for trust management.

### `hash.ts`
Content hashing utilities.

### `uuid.ts`
```typescript
export function generateUUID(): string
export function generateSessionId(): string
```

### `taggedId.ts`
Type-safe tagged ID types (prevents mixing SessionId, AgentId, etc.).

---

## 30. Native Installer

### `nativeInstaller/` subdirectory
Native binary installation utilities.

### `localInstaller.ts`
Installs Claude Code to local paths.

### `autoUpdater.ts`
Auto-update checking and installation.

---

## 31. General Utilities (Top-Level)

### Environment & Platform
| File | Purpose |
|------|---------|
| `env.ts` | Environment variable constants and getters |
| `envDynamic.ts` | Dynamically-loaded env vars |
| `envUtils.ts` | `isEnvTruthy()`, `isBareMode()`, `isRunningOnHomespace()`, `getClaudeConfigHomeDir()` |
| `envValidation.ts` | Validates required env vars |
| `platform.ts` | `getPlatform(): 'macos' \| 'linux' \| 'windows'` |
| `bundledMode.ts` | `isBundledMode(): boolean` |
| `managedEnv.ts` | Managed environment variable injection from settings |
| `managedEnvConstants.ts` | Constants for managed env vars |

### String & Formatting
| File | Purpose |
|------|---------|
| `stringUtils.ts` | `capitalize()`, `plural()`, `safeJoinLines()`, etc. |
| `format.ts` | `formatDuration()`, `formatBytes()`, etc. |
| `formatBriefTimestamp.ts` | Brief timestamp formatting |
| `truncate.ts` | Text truncation utilities |
| `words.ts` | Word counting and manipulation |
| `sliceAnsi.ts` | Slices ANSI-escaped strings without breaking escape codes |
| `cliHighlight.ts` | Syntax highlighting for CLI output |
| `markdown.ts` | Markdown rendering utilities |
| `hyperlink.ts` | Terminal hyperlink (OSC 8) utilities |
| `intl.ts` | Internationalization utilities |
| `sanitization.ts` | Input sanitization |

### JSON & Data
| File | Purpose |
|------|---------|
| `json.ts` | `safeParseJSON()`, JSON read/write |
| `jsonRead.ts` | `stripBOM()`, safe JSON file reading |
| `yaml.ts` | YAML parsing utilities |
| `xml.ts` | XML parsing utilities |
| `zodToJsonSchema.ts` | Converts Zod schemas to JSON Schema |
| `lazySchema.ts` | Deferred Zod schema validation |
| `frontmatterParser.ts` | Parses YAML frontmatter from markdown |
| `semanticBoolean.ts` | Parses "truthy" strings (`'true'`, `'yes'`, `'1'`) |
| `semanticNumber.ts` | Parses number strings with units |

### HTTP & Networking
| File | Purpose |
|------|---------|
| `http.ts` | Fetch wrappers with timeout and retry |
| `proxy.ts` | HTTP proxy configuration |
| `mtls.ts` | mTLS certificate management |
| `caCerts.ts` | Custom CA certificate loading |
| `caCertsConfig.ts` | CA cert configuration |
| `peerAddress.ts` | Peer address utilities |
| `mcpWebSocketTransport.ts` | MCP WebSocket transport |
| `apiPreconnect.ts` | Pre-connects to API for latency reduction |
| `userAgent.ts` | HTTP User-Agent string construction |

### API & Claude Communication
| File | Purpose |
|------|---------|
| `api.ts` | High-level API utilities |
| `queryContext.ts` | Query context construction |
| `queryHelpers.ts` | Query helper functions |
| `tokenBudget.ts` | Token budget management |
| `tokens.ts` | Token counting utilities |
| `stream.ts` | Streaming response utilities |
| `streamJsonStdoutGuard.ts` | Guards against JSON corruption in stream |
| `streamlinedTransform.ts` | Transforms streaming API responses |
| `thinking.ts` | Extended thinking utilities |
| `sideQuery.ts` | Side-channel query execution |
| `sideQuestion.ts` | Side-question display |
| `sdkEventQueue.ts` | SDK event queue management |

### Session & Context
| File | Purpose |
|------|---------|
| `systemPrompt.ts` | `buildEffectiveSystemPrompt()` — assembles the system prompt from multiple sources |
| `systemPromptType.ts` | `SystemPrompt` type and `asSystemPrompt()` |
| `agentContext.ts` | Agent execution context |
| `agentId.ts` | Agent ID generation |
| `context.ts` | Context window utilities |
| `contextAnalysis.ts` | Analyzes context window usage |
| `analyzeContext.ts` | Deeper context analysis |
| `workloadContext.ts` | Workload context for load balancing |
| `queryContext.ts` | Query context construction |

### Display & UI
| File | Purpose |
|------|---------|
| `displayTags.ts` | UI display tags |
| `logoV2Utils.ts` | Logo V2 utilities |
| `status.tsx` | Status display utilities |
| `statusNoticeDefinitions.tsx` | Status notice type definitions |
| `statusNoticeHelpers.ts` | Status notice helpers |
| `treeify.ts` | Renders tree structures as ASCII |
| `highlightMatch.tsx` | Highlights search matches in text |
| `textHighlighting.ts` | Text highlighting utilities |
| `horizontalScroll.ts` | Horizontal scroll state management |
| `fullscreen.ts` | Full-screen mode utilities |
| `exportRenderer.tsx` | Renders sessions for export |
| `staticRender.tsx` | Static React rendering (headless) |

### Config & Meta
| File | Purpose |
|------|---------|
| `claudeCodeHints.ts` | Context-sensitive hints for users |
| `claudeDesktop.ts` | Claude Desktop app integration |
| `exampleCommands.ts` | Example slash commands for help |
| `releaseNotes.ts` | Release notes loading and display |
| `version.ts` (referenced) | Version string management |
| `undercover.ts` | ANT-only: undercover/internal mode |
| `bundledMode.ts` | Detects bundled (desktop) mode |

### Diagnostics & Debugging
| File | Purpose |
|------|---------|
| `debug.ts` | `logForDebugging()`, `logAntError()` |
| `debugFilter.ts` | Debug output filtering |
| `diagLogs.ts` | `logForDiagnosticsNoPII()` — diagnostic logs without PII |
| `log.ts` | `logError()` — structured error logging |
| `errorLogSink.ts` | Error log collection |
| `warningHandler.ts` | Warning deduplication and display |
| `doctorDiagnostic.ts` | Diagnostic check implementations for `/doctor` |
| `doctorContextWarnings.ts` | Context-specific doctor warnings |
| `diagLogs.ts` | Non-PII diagnostic logging |

### IDE Integration
| File | Purpose |
|------|---------|
| `ide.ts` | VS Code / JetBrains IDE integration |
| `idePathConversion.ts` | Converts paths for IDE display |
| `jetbrains.ts` | JetBrains-specific integration |
| `terminalPanel.ts` | IDE terminal panel integration |

### Miscellaneous
| File | Purpose |
|------|---------|
| `activityManager.ts` | Tracks user activity |
| `argom entSubstitution.ts` | Substitutes arguments in templates |
| `autoModeDenials.ts` | Tracks auto-mode permission denials |
| `autoRunIssue.tsx` | Auto-run issue detection |
| `binaryCheck.ts` | Detects binary files |
| `browser.ts` | Opens URLs in browser |
| `bufferedWriter.ts` | Buffered async file writer |
| `classifierApprovals.ts` | Classifier-based approval management |
| `classifierApprovalsHook.ts` | React hook for classifier approvals |
| `codeIndexing.ts` | Code indexing for search |
| `commandLifecycle.ts` | Command lifecycle event management |
| `completionCache.ts` | Caches shell completions |
| `conversationRecovery.ts` | Recovers from corrupted conversations |
| `effor.ts` | Effort level management |
| `editor.ts` | External editor integration |
| `earlyInput.ts` | Captures input before UI is ready |
| `embeddedTools.ts` | Embedded tool management |
| `extraUsage.ts` | Extra usage tracking |
| `fastMode.ts` | Fast mode (sonnet default) management |
| `idleTimeout.ts` | Idle session timeout management |
| `immediateCommand.ts` | Immediate command execution (no spinner) |
| `keyboardShortcuts.ts` | Keyboard shortcut definitions |
| `mailbox.ts` | Inter-process mailbox messaging |
| `modifiers.ts` | Keyboard modifier key detection |
| `objectGroupBy.ts` | Object.groupBy polyfill |
| `pasteStore.ts` | Paste content storage |
| `planModeV2.ts` | Plan mode state management |
| `plans.ts` | Plan data management |
| `preflightChecks.tsx` | Pre-launch health checks |
| `privacyLevel.ts` | `getEssentialTrafficOnlyReason()` — privacy mode |
| `promptCategory.ts` | Categorizes prompts for analytics |
| `promptEditor.ts` | Multi-line prompt editing |
| `queueProcessor.ts` | Generic queue processing |
| `renderOptions.ts` | `getBaseRenderOptions()` for Ink |
| `screenshotClipboard.ts` | Screenshot clipboard utilities |
| `shellConfig.ts` | Shell configuration management |
| `sleep.ts` | `sleep(ms: number): Promise<void>` |
| `timeouts.ts` | Timeout utilities |
| `tmuxSocket.ts` | Tmux socket communication |
| `toolErrors.ts` | Tool error message formatting |
| `toolPool.ts` | Tool instance pooling |
| `toolResultStorage.ts` | Storage for tool call results |
| `toolSchemaCache.ts` | Caches compiled tool schemas |
| `toolSearch.ts` | Fuzzy search over available tools |
| `transcriptSearch.ts` | Full-text search within session transcripts |
| `user.ts` | User info utilities |
| `userPromptKeywords.ts` | Extracts keywords from user prompts |
| `which.ts` / `findExecutable.ts` | Finds executables in PATH |
| `windowsPaths.ts` | Windows/POSIX path conversion |
| `withResolvers.ts` | `Promise.withResolvers` polyfill |
| `ansiToPng.ts` / `ansiToSvg.ts` | ANSI terminal output to image |
| `asciicast.ts` | asciinema recording format |
| `appleTerminalBackup.ts` / `iTermBackup.ts` | Terminal state backup |
| `attribution.ts` | Message attribution tracking |
| `ink.ts` (in utils) | Ink-related utility helpers |

---

## 32. `interactiveHelpers.tsx`

High-level interactive session initialization utilities. Runs in the main thread before the REPL starts.

**Exports:**
```typescript
export function completeOnboarding(): void
// Sets hasCompletedOnboarding = true in global config

export function showDialog<T = void>(
  root: Root,
  renderer: (done: (result: T) => void) => React.ReactNode
): Promise<T>
// Renders a dialog via Ink and waits for completion

export async function exitWithError(
  root: Root,
  message: string,
  beforeExit?: () => Promise<void>
): Promise<never>
// Renders error message through Ink then exits (avoids console.error being swallowed)

export async function exitWithMessage(
  root: Root,
  message: string,
  options?: { color?: TextProps['color']; exitCode?: number; beforeExit?: () => Promise<void> }
): Promise<never>
```

**Key Imports/Dependencies:**
- `bootstrap/state` — global state management
- `services/analytics/growthbook` — feature flags
- `state/AppState` — application state
- `utils/claudemd` — CLAUDE.md reading
- `utils/config` — global/project config
- `utils/settings/settings` — user settings
- `services/mcpServerApproval` — MCP approval flow
- `keybindings/KeybindingProviderSetup` — keybinding initialization

---

## 33. `ink.ts` (Top-Level Module)

The public API for the Ink terminal rendering framework used throughout Claude Code.

**Purpose:** Wraps the internal `ink/root.ts` with a `ThemeProvider` so every render is automatically themed.

**Key Exports:**
```typescript
// Rendering
export async function render(node: ReactNode, options?: NodeJS.WriteStream | RenderOptions): Promise<Instance>
export async function createRoot(options?: RenderOptions): Promise<Root>

// Re-exports from ink/root
export type { RenderOptions, Instance, Root }

// Theme system
export { color } from './components/design-system/color.js'
export type { Props as BoxProps }
export { default as Box }         // ThemedBox
export type { Props as TextProps }
export { default as Text }        // ThemedText
export { ThemeProvider, usePreviewTheme, useTheme, useThemeSetting }

// Core ink components (re-exported from ink/)
export { Ansi }
export { default as BaseBox }
export { default as Button }
export { default as Link }
export { default as Newline }
export { NoSelect }
export { RawAnsi }
export { default as Spacer }
// ... more components
```

**Key Detail:** The `render()` and `createRoot()` wrappers apply `withTheme()` which wraps the node in `ThemeProvider`. This means every call site automatically has access to themed colors, eliminating the need for each component to mount its own `ThemeProvider`.

---

## 34. `utils/shell/` — Shell Provider Abstraction

The `shell/` subdirectory provides a shell-type-agnostic provider interface so BashTool and PowerShellTool can share execution infrastructure.

### `shell/shellProvider.ts`
Defines the `ShellProvider` interface:

```typescript
export const SHELL_TYPES = ['bash', 'powershell'] as const
export type ShellType = (typeof SHELL_TYPES)[number]
export const DEFAULT_HOOK_SHELL: ShellType = 'bash'

export type ShellProvider = {
  type: ShellType
  shellPath: string
  detached: boolean
  buildExecCommand(command, opts): Promise<{ commandString, cwdFilePath }>
  getSpawnArgs(commandString): string[]
  getEnvironmentOverrides(command): Promise<Record<string, string>>
}
```

### `shell/bashProvider.ts`
Factory: `createBashShellProvider(shellPath, options?): Promise<ShellProvider>`

Key behaviors:
- **Snapshot sourcing**: Lazy-initializes a shell snapshot and sources it on every command for consistent environment. Falls back to login-shell (`-l` flag) when snapshot is missing.
- **extglob disabling**: Injects `shopt -u extglob` / `setopt NO_EXTENDED_GLOB` for security against malicious filename glob expansion after snapshot load.
- **eval wrapping**: Commands are `eval`-wrapped so aliases sourced from snapshot expand correctly.
- **CWD tracking**: Appends `pwd -P >| <cwdFile>` so `getCwd()` stays synchronized.
- **Pipe rearrangement**: When a command contains `|` and needs stdin redirect, calls `rearrangePipeCommand()` to apply redirect to first pipe segment only.
- **Tmux isolation**: Lazily initializes Claude's isolated tmux socket when `hasTmuxToolBeenUsed()` or the command contains `tmux`.
- **Windows compatibility**: Rewrites `2>nul` to `/dev/null` and converts paths via `windowsPathToPosixPath`.
- **`CLAUDE_CODE_SHELL_PREFIX`**: Wraps the assembled command string in a custom prefix for shell environment wrappers.

### `shell/powershellProvider.ts`
Factory: `createPowerShellProvider(shellPath): ShellProvider`

Key behaviors:
- Encodes PS commands as Base64 UTF-16LE (`-EncodedCommand`) to survive shell-quoting layers.
- Non-detached (PowerShell requires synchronous spawning on Windows).
- CWD tracked to a temp file using PS `Set-Content`.
- Uses `$LASTEXITCODE` / `$?` for exit code capture (handles PS 5.1 quirks).

### `shell/outputLimits.ts`
```typescript
export const BASH_MAX_OUTPUT_UPPER_LIMIT = 150_000
export const BASH_MAX_OUTPUT_DEFAULT = 30_000
export function getMaxOutputLength(): number
// Reads BASH_MAX_OUTPUT_LENGTH env var; validated and clamped to [1, 150_000]
```

### `shell/shellToolUtils.ts`
```typescript
export const SHELL_TOOL_NAMES: string[]   // [BASH_TOOL_NAME, POWERSHELL_TOOL_NAME]
export function isPowerShellToolEnabled(): boolean
// Windows-only gate: ant users default ON (opt-out via env=0),
// external users default OFF (opt-in via env=1)
```

### `shell/readOnlyCommandValidation.ts`
Exports comprehensive allow-lists for read-only command validation:
- `GIT_READ_ONLY_COMMANDS` — All safe git subcommands with permitted flags and callbacks
- `GH_READ_ONLY_COMMANDS` — Ant-only GitHub CLI read commands
- `EXTERNAL_READONLY_COMMANDS` — Cross-shell safe commands (ls, cat, grep, etc.)
- `containsVulnerableUncPath(cmd)` — Detects Windows UNC paths that could leak credentials
- `FlagArgType` — `'none' | 'number' | 'string' | 'char' | '{}' | 'EOF'`
- `ExternalCommandConfig` — `{ safeFlags, additionalCommandIsDangerousCallback?, respectsDoubleDash? }`

### `shell/specPrefix.ts`
Fig-spec-driven command prefix extraction:
```typescript
export const DEPTH_RULES: Record<string, number>
// Overrides: rg→2, gcloud→4, kubectl→3, docker→3, aws→4, etc.

export async function buildPrefix(
  command: string,
  args: string[],
  spec: CommandSpec | null
): Promise<string>
// e.g. "git -C /repo status --short" → "git status"
```

### `shell/prefix.ts`
LLM-based (Haiku) command prefix extraction factory:
```typescript
export type CommandPrefixResult = { commandPrefix: string | null }
export type PrefixExtractorConfig = {
  toolName: string
  policySpec: string
  eventName: string
  querySource: QuerySource
  preCheck?: (command: string) => CommandPrefixResult | null
}

export function createCommandPrefixExtractor(config: PrefixExtractorConfig)
// Returns memoized (LRU 200) async function. Calls Haiku with policySpec.
// Rejects: command_injection_detected, bare shell names, non-prefix responses.

export function createSubcommandPrefixExtractor(getPrefix, splitCommand)
// Extracts prefix for each subcommand in a compound command (e.g. a && b)
```

### `shell/powershellDetection.ts`
Detects available PowerShell executables (`pwsh` vs `powershell.exe`) on Windows.

---

## 35. `utils/ultraplan/` — Ultraplan Remote Session Polling

The `ultraplan/` subdirectory supports the `/ultraplan` command, which launches a plan-mode CCR session and waits for the user to approve a plan in the browser.

### `ultraplan/ccrSession.ts`

Types:
```typescript
export type PollFailReason =
  | 'terminated' | 'timeout_pending' | 'timeout_no_plan'
  | 'extract_marker_missing' | 'network_or_unknown' | 'stopped'

export class UltraplanPollError extends Error {
  reason: PollFailReason
  rejectCount: number
}

export const ULTRAPLAN_TELEPORT_SENTINEL = '__ULTRAPLAN_TELEPORT_LOCAL__'
// Browser embeds this in the rejection feedback when user clicks "teleport back to terminal"

export type ScanResult =
  | { kind: 'approved'; plan: string }
  | { kind: 'teleport'; plan: string }  // user wants local execution
  | { kind: 'rejected'; id: string }
  | { kind: 'pending' }
  | { kind: 'terminated'; subtype: string }
  | { kind: 'unchanged' }

export type UltraplanPhase = 'running' | 'needs_input' | 'plan_ready'
```

`ExitPlanModeScanner` — pure stateful classifier over the CCR event stream:
```typescript
export class ExitPlanModeScanner {
  get rejectCount(): number
  get hasPendingPlan(): boolean   // ExitPlanMode emitted, no tool_result yet
  everSeenPending: boolean
  ingest(newEvents: SDKMessage[]): ScanResult
  // Tracks ExitPlanMode tool_use IDs and their corresponding tool_results
  // Precedence: approved > terminated > rejected > pending > unchanged
}
```

`pollForApprovedExitPlanMode` — main polling loop:
```typescript
export type PollResult = {
  plan: string
  rejectCount: number
  executionTarget: 'local' | 'remote'
}

export async function pollForApprovedExitPlanMode(
  sessionId: string,
  timeoutMs: number,
  onPhaseChange?: (phase: UltraplanPhase) => void,
  shouldStop?: () => boolean
): Promise<PollResult>
```

Polls `pollRemoteSessionEvents()` every 3 seconds. `MAX_CONSECUTIVE_FAILURES = 5` before giving up on transient network errors. Plan text is extracted from `tool_result.content` via:
- `"## Approved Plan:\n"` or `"## Approved Plan (edited by user):\n"` markers (remote execution)
- `ULTRAPLAN_TELEPORT_SENTINEL + "\n"` marker (local execution)

### `ultraplan/keyword.ts`

Detects `ultraplan` / `ultrareview` keyword triggers in user input, with smart exclusion:

```typescript
export function findUltraplanTriggerPositions(text: string): TriggerPosition[]
export function findUltrareviewTriggerPositions(text: string): TriggerPosition[]
export function hasUltraplanKeyword(text: string): boolean
export function hasUltrareviewKeyword(text: string): boolean
export function replaceUltraplanKeyword(text: string): string
// Replaces first triggerable "ultraplan" with "plan" for the forwarded CCR prompt
```

Does NOT trigger when the keyword is inside paired delimiters (backticks, quotes, brackets, braces), in a path/identifier context, followed by `?`, or in a slash command. Apostrophe-aware: `"let's ultraplan"` still triggers.
