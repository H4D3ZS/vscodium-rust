# Claude Code — Core Entry Points & Query System

## Table of Contents

1. [entrypoints/cli.tsx — Bootstrap Dispatcher](#entrypointsclisx--bootstrap-dispatcher)
2. [main.tsx — Full CLI Entry Point](#maintsx--full-cli-entry-point)
3. [replLauncher.tsx — REPL UI Launcher](#repplaunchertsx--repl-ui-launcher)
4. [entrypoints/init.ts — Initialization & Telemetry](#entrypointsinits--initialization--telemetry)
5. [entrypoints/mcp.ts — MCP Server Entrypoint](#entrypointsmcpts--mcp-server-entrypoint)
6. [entrypoints/agentSdkTypes.ts — Agent SDK Public API](#entrypointsagentsdktypests--agent-sdk-public-api)
7. [entrypoints/sandboxTypes.ts — Sandbox Configuration Types](#entrypointssandboxtypests--sandbox-configuration-types)
8. [entrypoints/sdk/coreSchemas.ts — SDK Core Zod Schemas](#entrypointssdkcoreshematss--sdk-core-zod-schemas)
9. [entrypoints/sdk/coreTypes.ts — SDK Core TypeScript Types](#entrypointssdkcoretypests--sdk-core-typescript-types)
10. [entrypoints/sdk/controlSchemas.ts — SDK Control Protocol Schemas](#entrypointssdkcontrolschematss--sdk-control-protocol-schemas)
11. [query.ts — Core Async Query Loop](#queryts--core-async-query-loop)
12. [QueryEngine.ts — Stateful Query Engine (SDK/Headless)](#queryenginets--stateful-query-engine-sdkheadless)
13. [query/config.ts — Query Configuration Snapshot](#queryconfigts--query-configuration-snapshot)
14. [query/deps.ts — Query Dependency Injection](#querydepsts--query-dependency-injection)
15. [query/stopHooks.ts — Stop Hook Orchestration](#querystophooksts--stop-hook-orchestration)
16. [query/tokenBudget.ts — Token Budget Tracking](#querytokenbudgetts--token-budget-tracking)
17. [context.ts — System & User Context Providers](#contextts--system--user-context-providers)
18. [history.ts — Prompt History Management](#historyts--prompt-history-management)
19. [cost-tracker.ts — Session Cost Tracking](#cost-trackerts--session-cost-tracking)
20. [costHook.ts — React Cost Summary Hook](#costhookts--react-cost-summary-hook)
21. [projectOnboardingState.ts — Project Onboarding State](#projectonboardingstatets--project-onboarding-state)
22. [bootstrap/state.ts — Global Session State](#bootstrapstatets--global-session-state)
23. [assistant/sessionHistory.ts — Remote Session History Pagination](#assistantsessionhistoryts--remote-session-history-pagination)

---

## entrypoints/cli.tsx — Bootstrap Dispatcher

### Purpose

The very first module executed when a user runs `claude`. Acts as a lightweight bootstrap dispatcher that checks process arguments for known fast-paths **before** loading any heavy modules. Each fast-path dynamically imports only what it needs. The full CLI (`main.tsx`) is only loaded when no fast-path matches.

### Key Flow

```
process.argv parsing
  ├── --version / -v → print MACRO.VERSION, exit (zero imports)
  ├── --dump-system-prompt → dump rendered system prompt (ant-only, DUMP_SYSTEM_PROMPT feature)
  ├── --claude-in-chrome-mcp → runClaudeInChromeMcpServer()
  ├── --chrome-native-host → runChromeNativeHost()
  ├── --computer-use-mcp → runComputerUseMcpServer() (CHICAGO_MCP feature)
  ├── --daemon-worker=<kind> → runDaemonWorker() (DAEMON feature)
  ├── remote-control|rc|remote|sync|bridge → bridgeMain() (BRIDGE_MODE feature)
  ├── daemon → daemonMain() (DAEMON feature)
  ├── ps|logs|attach|kill|--bg|--background → bg handlers (BG_SESSIONS feature)
  ├── new|list|reply → templatesMain() (TEMPLATES feature)
  ├── environment-runner → environmentRunnerMain() (BYOC_ENVIRONMENT_RUNNER feature)
  ├── self-hosted-runner → selfHostedRunnerMain() (SELF_HOSTED_RUNNER feature)
  ├── --worktree + --tmux → execIntoTmuxWorktree()
  └── (default) → startCapturingEarlyInput() → import main.tsx → cliMain()
```

### Top-Level Side Effects (at module load time)

| Side Effect | Purpose |
|---|---|
| `process.env.COREPACK_ENABLE_AUTO_PIN = '0'` | Prevent yarnpkg from being added to package.json |
| `process.env.NODE_OPTIONS += '--max-old-space-size=8192'` | CCR environment (16GB containers) heap size |
| `ABLATION_BASELINE` flag | Sets multiple `CLAUDE_CODE_*` env vars for harness-science L0 ablation |

### Exports

```typescript
// No named exports — module executes main() IIFE at bottom
async function main(): Promise<void>  // internal, not exported
```

### Feature Flags Checked

- `DUMP_SYSTEM_PROMPT` — ant-only system prompt dump
- `CHICAGO_MCP` — computer-use MCP server
- `DAEMON` — daemon worker and supervisor
- `BRIDGE_MODE` — remote control bridge
- `BG_SESSIONS` — background session management
- `TEMPLATES` — template job commands
- `BYOC_ENVIRONMENT_RUNNER` — BYOC headless runner
- `SELF_HOSTED_RUNNER` — self-hosted runner
- `ABLATION_BASELINE` — harness-science baseline

### Dependencies

- `bun:bundle` (`feature`)
- `../utils/startupProfiler.js`
- `../utils/config.js` (enableConfigs)
- Various fast-path modules loaded dynamically

---

## main.tsx — Full CLI Entry Point

### Purpose

The main CLI module. Loaded only after `entrypoints/cli.tsx` determines no fast-path matches. Defines the Commander.js command tree, handles all CLI flags, orchestrates startup (migrations, trust dialog, MCP config, tool loading), and launches either the interactive REPL or the headless/print (`-p`) path.

### Exported Functions

```typescript
export async function main(): Promise<void>
export function startDeferredPrefetches(): void
```

#### `main()`

The primary entry point for the full CLI. Responsibilities in order:

1. **Security**: Sets `process.env.NoDefaultCurrentDirectoryInExePath = '1'` (Windows PATH attack prevention)
2. **Warning handler**: `initializeWarningHandler()`
3. **Signal handlers**: SIGINT (skip in print mode), exit cursor reset
4. **Early arg processing**:
   - `cc://` / `cc+unix://` URL rewriting (DIRECT_CONNECT feature)
   - `--handle-uri` deep link handling (LODESTONE feature)
   - `claude assistant [sessionId]` rewriting (KAIROS feature)
   - `claude ssh <host>` rewriting (SSH_REMOTE feature)
5. **Settings flag parsing**: `eagerLoadSettings()` runs before `init()`
6. **Commander.js setup**: Defines the complete command tree (see CLI flags below)
7. **`init()`** call: Validates configs, sets up network, telemetry loading promise
8. **Migration run**: `runMigrations()` at version `CURRENT_MIGRATION_VERSION = 11`
9. **Trust check**: Shows trust dialog if not previously accepted
10. **Telemetry init**: `initializeTelemetryAfterTrust()`
11. **Session setup**: model, permissions, MCP servers, tools, agents
12. **Launch**: Either `showSetupScreens()` + `launchRepl()`, or headless `runHeadless()`

#### `startDeferredPrefetches()`

Called after first REPL render to avoid blocking the initial paint. Skipped when:
- `CLAUDE_CODE_EXIT_AFTER_FIRST_RENDER=1`
- `--bare` mode (isBareMode())

Prefetches (fire-and-forget):
- `initUser()`
- `getUserContext()`
- `prefetchSystemContextIfSafe()`
- `getRelevantTips()`
- AWS/GCP credentials (if Bedrock/Vertex enabled)
- `countFilesRoundedRg()` (3 second timeout)
- `initializeAnalyticsGates()`
- `prefetchOfficialMcpUrls()`
- `refreshModelCapabilities()`
- `settingsChangeDetector.initialize()`
- `skillChangeDetector.initialize()` (non-bare only)

### Constants

| Constant | Value | Purpose |
|---|---|---|
| `CURRENT_MIGRATION_VERSION` | `11` | Version gate for running config migrations |

### CLI Flags (Commander.js)

| Flag | Type | Description |
|---|---|---|
| `-p, --print <prompt>` | `string` | Non-interactive/headless mode |
| `--model <model>` | `string` | Model override |
| `--fallback-model <model>` | `string` | Fallback model for retry |
| `--permission-mode <mode>` | enum | Permission mode for tool execution |
| `--dangerously-skip-permissions` | `boolean` | Bypass all permission checks |
| `--verbose` | `boolean` | Verbose output |
| `--debug` | `boolean` | Debug mode |
| `--mcp-config <file>` | `string` | MCP config file path |
| `--add-dir <dir>` | `string[]` | Additional directories for CLAUDE.md |
| `--resume [sessionId]` | `string?` | Resume previous session |
| `--bare` | `boolean` | Simple/stripped mode (no UI extras) |
| `--settings <path|json>` | `string` | Flag-layer settings override |
| `--setting-sources <sources>` | `string` | Allowed settings sources |
| `--output-format <format>` | string | Output format for headless mode |
| `--max-turns <n>` | `number` | Max turns in headless mode |
| `--max-budget-usd <n>` | `number` | Cost budget limit |
| `--task-budget <n>` | `number` | API task budget |
| `--no-streaming` | `boolean` | Disable streaming |
| `--worktree <branch>` | `string` | Git worktree mode |
| `--agent <type>` | `string` | Main thread agent type |
| `--tmux` / `--tmux=classic` | `boolean` | Use tmux |
| `--plugin-dir <dir>` | `string[]` | Session-only plugin directories |
| `--input-format <format>` | string | Input format |
| `--sdk-betas <betas>` | `string` | Comma-separated SDK beta headers |
| `--allowedTools <tools>` | `string` | Tool allowlist (CLI override) |
| `--disallowedTools <tools>` | `string` | Tool denylist (CLI override) |

### Migration Functions

Executed in `runMigrations()` when `globalConfig.migrationVersion !== 11`:

| Function | Purpose |
|---|---|
| `migrateAutoUpdatesToSettings()` | Move auto-update config |
| `migrateBypassPermissionsAcceptedToSettings()` | Move bypass flag |
| `migrateEnableAllProjectMcpServersToSettings()` | Move MCP enable setting |
| `resetProToOpusDefault()` | Reset Pro model to Opus |
| `migrateSonnet1mToSonnet45()` | Rename model string |
| `migrateLegacyOpusToCurrent()` | Upgrade legacy Opus |
| `migrateSonnet45ToSonnet46()` | Rename to Sonnet 4.6 |
| `migrateOpusToOpus1m()` | Rename to Opus 1m |
| `migrateReplBridgeEnabledToRemoteControlAtStartup()` | Rename bridge flag |
| `resetAutoModeOptInForDefaultOffer()` | Reset auto mode opt-in (TRANSCRIPT_CLASSIFIER) |
| `migrateFennecToOpus()` | ant-only Fennec model migration |

### Top-Level Side Effects

```typescript
profileCheckpoint('main_tsx_entry')    // startup profiling
startMdmRawRead()                       // parallel MDM subprocess (plutil/reg query)
startKeychainPrefetch()                 // parallel macOS keychain reads
```

### Key Internal Functions

```typescript
function logManagedSettings(): void
function isBeingDebugged(): boolean
function logSessionTelemetry(): void
function getCertEnvVarTelemetry(): Record<string, boolean>
async function logStartupTelemetry(): Promise<void>
function runMigrations(): void
function prefetchSystemContextIfSafe(): void
function loadSettingsFromFlag(settingsFile: string): void
function loadSettingSourcesFromFlag(settingSourcesArg: string): void
function eagerLoadSettings(): void
function initializeEntrypoint(isNonInteractive: boolean): void
```

### Pending State Types (feature-gated)

```typescript
type PendingConnect = {
  url: string | undefined
  authToken: string | undefined
  dangerouslySkipPermissions: boolean
}

type PendingAssistantChat = {
  sessionId?: string
  discover: boolean
}

type PendingSSH = {
  host: string | undefined
  cwd: string | undefined
  permissionMode: string | undefined
  dangerouslySkipPermissions: boolean
  local: boolean
  extraCliArgs: string[]
}
```

### Feature Flags

`DIRECT_CONNECT`, `KAIROS`, `SSH_REMOTE`, `LODESTONE`, `COORDINATOR_MODE`, `TRANSCRIPT_CLASSIFIER`, `BREAK_CACHE_COMMAND`, `HISTORY_SNIP`, `DAEMON`, `BG_SESSIONS`, `TEMPLATES`

---

## replLauncher.tsx — REPL UI Launcher

### Purpose

Thin async launcher that dynamically imports `App` and `REPL` components (avoiding circular dependencies) and renders them into the Ink root. Exists as a separate file so that `App` and `REPL` are loaded lazily.

### Exports

```typescript
export async function launchRepl(
  root: Root,
  appProps: AppWrapperProps,
  replProps: REPLProps,
  renderAndRun: (root: Root, element: React.ReactNode) => Promise<void>,
): Promise<void>
```

### Types

```typescript
type AppWrapperProps = {
  getFpsMetrics: () => FpsMetrics | undefined
  stats?: StatsStore
  initialState: AppState
}
```

### Implementation

Dynamically imports `./components/App.js` and `./screens/REPL.js`, then calls `renderAndRun(root, <App {...appProps}><REPL {...replProps} /></App>)`.

### Dependencies

- `react`
- `./context/stats.js` (type only)
- `./ink.js` (type only)
- `./screens/REPL.js` (type only)
- `./state/AppStateStore.js` (type only)
- `./utils/fpsTracker.js` (type only)

---

## entrypoints/init.ts — Initialization & Telemetry

### Purpose

Handles all early initialization tasks that must complete before the CLI can safely make API calls or show a UI. Memoized so it runs exactly once per process. Telemetry initialization is deferred until after trust is established.

### Exports

```typescript
export const init: () => Promise<void>  // memoized with lodash-es/memoize
export function initializeTelemetryAfterTrust(): void
```

#### `init()` — Memoized Async Initialization

Runs once. Sequence:

1. `enableConfigs()` — validate and activate config system
2. `applySafeConfigEnvironmentVariables()` — apply env vars that are safe before trust
3. `applyExtraCACertsFromConfig()` — inject `NODE_EXTRA_CA_CERTS` before first TLS connection
4. `setupGracefulShutdown()` — register flush/cleanup on exit
5. `initialize1PEventLogging()` + GrowthBook refresh listener (deferred)
6. `populateOAuthAccountInfoIfNeeded()` (fire-and-forget)
7. `initJetBrainsDetection()` (fire-and-forget)
8. `detectCurrentRepository()` (fire-and-forget)
9. `initializeRemoteManagedSettingsLoadingPromise()` (if eligible)
10. `initializePolicyLimitsLoadingPromise()` (if eligible)
11. `recordFirstStartTime()`
12. `configureGlobalMTLS()`
13. `configureGlobalAgents()` (proxy)
14. `preconnectAnthropicApi()` — overlap TCP+TLS with action handler work
15. Upstream proxy initialization (CLAUDE_CODE_REMOTE only)
16. `setShellIfWindows()` — configure git-bash on Windows
17. `registerCleanup(shutdownLspServerManager)`
18. `registerCleanup(cleanupSessionTeams)` (lazy import)
19. `ensureScratchpadDir()` (if scratchpad enabled)

**Error handling**: `ConfigParseError` → shows `InvalidConfigDialog` (interactive) or `stderr` (non-interactive). Other errors re-throw.

#### `initializeTelemetryAfterTrust()`

Called once after trust is established. For remote-settings-eligible users: waits for settings to load, then calls `applyConfigEnvironmentVariables()` before initializing. For SDK/headless with beta tracing: initializes eagerly first.

Internal: `doInitializeTelemetry()` → `setMeterState()` → `initializeTelemetry()` (lazy-loaded OpenTelemetry, ~400KB)

`AttributedCounter` factory: wraps OpenTelemetry `Counter` to always merge `getTelemetryAttributes()` with any additional attributes on each `add()` call.

### Dependencies

- `../bootstrap/state.js`
- `../utils/config.js`
- `../services/lsp/manager.js`
- `../services/oauth/client.js`
- `../services/policyLimits/index.js`
- `../services/remoteManagedSettings/index.js`
- `../utils/apiPreconnect.js`
- `../utils/caCertsConfig.js`
- `../utils/cleanupRegistry.js`
- `../utils/gracefulShutdown.js`
- `../utils/managedEnv.js`
- `../utils/mtls.js`
- `../utils/proxy.js`
- `../utils/telemetry/betaSessionTracing.js`
- `../utils/telemetryAttributes.js`
- `../utils/windowsPaths.js`

---

## entrypoints/mcp.ts — MCP Server Entrypoint

### Purpose

Starts Claude Code as an MCP (Model Context Protocol) server, exposing Claude's built-in tools over the `stdio` transport. Server name: `claude/tengu`.

### Exports

```typescript
export async function startMCPServer(
  cwd: string,
  debug: boolean,
  verbose: boolean,
): Promise<void>
```

### Implementation Details

- **Transport**: `StdioServerTransport` (stdin/stdout)
- **Capabilities**: `{ tools: {} }`
- **File state cache**: LRU with limit 100 files / 25 MB
- **Commands exposed**: Only `review` command (via `MCP_COMMANDS`)
- **Tool exposure**: All tools from `getTools(toolPermissionContext)` with empty permission context

#### `ListTools` Handler

For each tool:
1. Calls `tool.prompt(...)` to get the description
2. Converts `tool.inputSchema` to JSON Schema via `zodToJsonSchema()`
3. Converts `tool.outputSchema` (if present) to JSON Schema — only included if root type is `object` (not `anyOf`/`oneOf`)

#### `CallTool` Handler

1. Gets tools via `getTools(emptyPermissionContext)`
2. Finds tool by name; throws if not found
3. Calls `tool.isEnabled()`, `tool.validateInput()`, then `tool.call()`
4. Builds a `ToolUseContext` with:
   - `isNonInteractiveSession: true`
   - `thinkingConfig: { type: 'disabled' }`
   - `mcpClients: []`
5. Returns `{ content: [{ type: 'text', text: result }] }` or `{ isError: true, content: [...] }`

### Constants

| Constant | Value |
|---|---|
| `READ_FILE_STATE_CACHE_SIZE` | `100` |
| MCP server name | `'claude/tengu'` |
| MCP server version | `MACRO.VERSION` |

---

## entrypoints/agentSdkTypes.ts — Agent SDK Public API

### Purpose

The main entrypoint for Claude Code Agent SDK types. Re-exports all public SDK types and declares stub functions that throw `'not implemented'` — actual implementations are provided by the real SDK runtime (the CLI process). This file is the type-only interface for SDK consumers.

### Exports

```typescript
// Control protocol (alpha)
export type { SDKControlRequest, SDKControlResponse } from './sdk/controlTypes.js'

// Core types (common serializable)
export * from './sdk/coreTypes.js'

// Runtime types (callbacks, interfaces)
export * from './sdk/runtimeTypes.js'

// Settings types
export type { Settings } from './sdk/settingsTypes.generated.js'

// Tool types
export * from './sdk/toolTypes.js'
```

### Exported Functions (stubs)

```typescript
export function tool<Schema extends AnyZodRawShape>(
  _name: string,
  _description: string,
  _inputSchema: Schema,
  _handler: (args: InferShape<Schema>, extra: unknown) => Promise<CallToolResult>,
  _extras?: { annotations?: ToolAnnotations; searchHint?: string; alwaysLoad?: boolean },
): SdkMcpToolDefinition<Schema>

export function createSdkMcpServer(
  _options: CreateSdkMcpServerOptions,
): McpSdkServerConfigWithInstance

export class AbortError extends Error {}

// V1 API
export function query(_params: {
  prompt: string | AsyncIterable<SDKUserMessage>
  options?: InternalOptions | Options
}): InternalQuery | Query

// V2 API (alpha/unstable)
export function unstable_v2_createSession(_options: SDKSessionOptions): SDKSession
export function unstable_v2_resumeSession(_sessionId: string, _options: SDKSessionOptions): SDKSession
export async function unstable_v2_prompt(_message: string, _options: SDKSessionOptions): Promise<SDKResultMessage>

// Session management
export async function getSessionMessages(_sessionId: string, _options?: GetSessionMessagesOptions): Promise<SessionMessage[]>
export async function listSessions(_options?: ListSessionsOptions): Promise<SDKSessionInfo[]>
export async function getSessionInfo(_sessionId: string, _options?: GetSessionInfoOptions): Promise<SDKSessionInfo | undefined>
export async function renameSession(_sessionId: string, _title: string, _options?: SessionMutationOptions): Promise<void>
export async function tagSession(_sessionId: string, _tag: string | null, _options?: SessionMutationOptions): Promise<void>
```

### Re-exported Constants

```typescript
export const HOOK_EVENTS = [
  'PreToolUse', 'PostToolUse', 'PostToolUseFailure', 'Notification',
  'UserPromptSubmit', 'SessionStart', 'SessionEnd', 'Stop', 'StopFailure',
  'SubagentStart', 'SubagentStop', 'PreCompact', 'PostCompact',
  'PermissionRequest', 'PermissionDenied', 'Setup', 'TeammateIdle',
  'TaskCreated', 'TaskCompleted', 'Elicitation', 'ElicitationResult',
  'ConfigChange', 'WorktreeCreate', 'WorktreeRemove', 'InstructionsLoaded',
  'CwdChanged', 'FileChanged',
] as const

export const EXIT_REASONS = [
  'clear', 'resume', 'logout', 'prompt_input_exit', 'other',
  'bypass_permissions_disabled',
] as const
```

---

## entrypoints/sandboxTypes.ts — Sandbox Configuration Types

### Purpose

Single source of truth for sandbox configuration types. Both the SDK and settings validation import from here.

### Exports

#### Schemas (Zod, via `lazySchema`)

```typescript
export const SandboxNetworkConfigSchema   // optional object
export const SandboxFilesystemConfigSchema  // optional object
export const SandboxSettingsSchema          // passthrough object
```

#### Inferred TypeScript Types

```typescript
export type SandboxSettings         // from SandboxSettingsSchema
export type SandboxNetworkConfig    // NonNullable<...>
export type SandboxFilesystemConfig // NonNullable<...>
export type SandboxIgnoreViolations // NonNullable<SandboxSettings['ignoreViolations']>
```

### Schema Detail

#### `SandboxNetworkConfigSchema`

| Field | Type | Description |
|---|---|---|
| `allowedDomains` | `string[]?` | Allowed outbound domains |
| `allowManagedDomainsOnly` | `boolean?` | Managed-settings-only domain enforcement |
| `allowUnixSockets` | `string[]?` | macOS-only unix socket paths |
| `allowAllUnixSockets` | `boolean?` | Disable unix socket blocking |
| `allowLocalBinding` | `boolean?` | Allow local port binding |
| `httpProxyPort` | `number?` | HTTP proxy port |
| `socksProxyPort` | `number?` | SOCKS proxy port |

#### `SandboxFilesystemConfigSchema`

| Field | Type | Description |
|---|---|---|
| `allowWrite` | `string[]?` | Additional write-allowed paths |
| `denyWrite` | `string[]?` | Additional write-denied paths |
| `denyRead` | `string[]?` | Additional read-denied paths |
| `allowRead` | `string[]?` | Paths to re-allow within denyRead regions |
| `allowManagedReadPathsOnly` | `boolean?` | Managed-settings-only read path enforcement |

#### `SandboxSettingsSchema`

| Field | Type | Description |
|---|---|---|
| `enabled` | `boolean?` | Enable sandboxing |
| `failIfUnavailable` | `boolean?` | Hard-fail if sandbox unavailable |
| `autoAllowBashIfSandboxed` | `boolean?` | Auto-allow Bash when sandboxed |
| `allowUnsandboxedCommands` | `boolean?` | Allow `dangerouslyDisableSandbox` param |
| `network` | `SandboxNetworkConfig?` | Network config |
| `filesystem` | `SandboxFilesystemConfig?` | Filesystem config |
| `ignoreViolations` | `Record<string, string[]>?` | Per-rule violation ignoring |
| `enableWeakerNestedSandbox` | `boolean?` | Weaker nested sandbox |
| `enableWeakerNetworkIsolation` | `boolean?` | macOS: allow com.apple.trustd.agent |
| `excludedCommands` | `string[]?` | Commands to exclude from sandboxing |
| `ripgrep` | `{ command: string; args?: string[] }?` | Custom ripgrep config |

Note: Schema uses `.passthrough()` — undocumented `enabledPlatforms` field is accepted.

---

## entrypoints/sdk/coreSchemas.ts — SDK Core Zod Schemas

### Purpose

Single source of truth for SDK data type schemas. TypeScript types are code-generated from these schemas. Uses `lazySchema()` wrapper for all schemas (deferred evaluation).

### Schema Groups

#### Usage & Model

```typescript
export const ModelUsageSchema  // inputTokens, outputTokens, cacheRead/Write, webSearch, costUSD, contextWindow, maxOutputTokens
```

#### Output Format

```typescript
export const OutputFormatTypeSchema   // z.literal('json_schema')
export const BaseOutputFormatSchema   // { type: OutputFormatTypeSchema }
export const JsonSchemaOutputFormatSchema  // { type: 'json_schema', schema: Record<string, unknown> }
export const OutputFormatSchema       // = JsonSchemaOutputFormatSchema
```

#### Config

```typescript
export const ApiKeySourceSchema       // 'user' | 'project' | 'org' | 'temporary' | 'oauth'
export const ConfigScopeSchema        // 'local' | 'user' | 'project'
export const SdkBetaSchema            // z.literal('context-1m-2025-08-07')
```

#### Thinking Config

```typescript
export const ThinkingAdaptiveSchema  // { type: 'adaptive' }  — Claude decides (Opus 4.6+)
export const ThinkingEnabledSchema   // { type: 'enabled', budgetTokens?: number }  — fixed budget
export const ThinkingDisabledSchema  // { type: 'disabled' }
export const ThinkingConfigSchema    // union of above three
```

#### MCP Server Config

```typescript
export const McpStdioServerConfigSchema   // { type?: 'stdio', command, args?, env? }
export const McpSSEServerConfigSchema     // { type: 'sse', url, headers? }
export const McpHttpServerConfigSchema    // { type: 'http', url, headers? }
export const McpSdkServerConfigSchema     // { type: 'sdk', name }
export const McpServerConfigForProcessTransportSchema  // union of stdio|sse|http|sdk
export const McpClaudeAIProxyServerConfigSchema  // { type: 'claudeai-proxy', url, id }
export const McpServerStatusConfigSchema  // union of process transport + claudeai-proxy
export const McpSetServersResultSchema    // { added, removed, errors }
```

#### MCP Server Status

```typescript
export const McpServerStatusSchema  // { name, status, serverInfo?, error?, config?, scope?, tools?, capabilities? }
// status enum: 'connected' | 'failed' | 'needs-auth' | 'pending' | 'disabled'
```

#### Permission Types

```typescript
export const PermissionUpdateDestinationSchema  // 'userSettings' | 'projectSettings' | 'localSettings' | 'session' | 'cliArg'
export const PermissionBehaviorSchema          // 'allow' | 'deny' | 'ask'
export const PermissionRuleValueSchema         // { toolName, ruleContent? }
export const PermissionUpdateSchema            // discriminated union: addRules | replaceRules | removeRules | setMode | addDirectories | removeDirectories
export const PermissionDecisionClassificationSchema  // 'user_temporary' | 'user_permanent' | 'user_reject'
export const PermissionResultSchema            // { behavior: 'allow', updatedInput?, ... } | { behavior: 'deny', message, ... }
export const PermissionModeSchema              // 'default' | 'acceptEdits' | 'bypassPermissions' | 'plan' | 'dontAsk'
```

#### Hook Types

```typescript
export const HOOK_EVENTS  // const array of 28 event names (same as coreTypes.ts)
export const HookEventSchema  // z.enum(HOOK_EVENTS)
export const BaseHookInputSchema  // { session_id, transcript_path, cwd, permission_mode?, agent_id? }
export const HookInputSchema     // full hook input (extends base)
```

#### Message Types

All `SDKMessage*` schemas define the full message taxonomy emitted by `query()`:

| Schema | Description |
|---|---|
| `SDKMessageSchema` | Root union of all message types |
| `SDKUserMessageSchema` | User turn message |
| `SDKStreamlinedTextMessageSchema` | Optimized text-only message |
| `SDKStreamlinedToolUseSummaryMessageSchema` | Tool use summary (optimized) |
| `SDKPostTurnSummaryMessageSchema` | End-of-turn summary |

---

## entrypoints/sdk/coreTypes.ts — SDK Core TypeScript Types

### Purpose

TypeScript type declarations for the SDK, code-generated from `coreSchemas.ts`. Not edited manually.

### Exports

```typescript
// Re-exports sandbox types
export type { SandboxFilesystemConfig, SandboxIgnoreViolations, SandboxNetworkConfig, SandboxSettings }
  from '../sandboxTypes.js'

// All generated types
export * from './coreTypes.generated.js'

// Utility types not expressible as Zod schemas
export type { NonNullableUsage } from './sdkUtilityTypes.js'
```

### Const Arrays (runtime)

```typescript
export const HOOK_EVENTS = [...] as const  // 28 hook event names
export const EXIT_REASONS = ['clear', 'resume', 'logout', 'prompt_input_exit', 'other', 'bypass_permissions_disabled'] as const
```

---

## entrypoints/sdk/controlSchemas.ts — SDK Control Protocol Schemas

### Purpose

Zod schemas for the SDK control protocol — the bidirectional communication channel between SDK implementations (Python SDK, desktop apps) and the CLI process. Uses `SDKControlRequest` / `SDKControlResponse` message wrappers over stdin/stdout.

### Control Request Schemas

Each request has a `subtype` discriminator:

| Schema | `subtype` | Description |
|---|---|---|
| `SDKControlInitializeRequestSchema` | `'initialize'` | Initialize SDK session (hooks, MCP, agents, jsonSchema) |
| `SDKControlInterruptRequestSchema` | `'interrupt'` | Interrupt current turn |
| `SDKControlPermissionRequestSchema` | `'can_use_tool'` | Request tool permission |
| `SDKControlSetPermissionModeRequestSchema` | `'set_permission_mode'` | Change permission mode |
| `SDKControlSetModelRequestSchema` | `'set_model'` | Change active model |
| `SDKControlSetMaxThinkingTokensRequestSchema` | `'set_max_thinking_tokens'` | Set thinking budget |
| `SDKControlMcpStatusRequestSchema` | `'mcp_status'` | Get MCP server status |
| `SDKControlGetContextUsageRequestSchema` | `'get_context_usage'` | Get context window breakdown |
| `SDKControlRewindFilesRequestSchema` | `'rewind_files'` | Rewind file changes since a user message |
| `SDKControlCancelAsyncMessageRequestSchema` | `'cancel_async_message'` | Drop queued async message |
| `SDKControlSeedReadStateRequestSchema` | `'seed_read_state'` | Seed readFileState cache |
| `SDKHookCallbackRequestSchema` | `'hook_callback'` | Deliver hook callback |
| `SDKControlMcpMessageRequestSchema` | `'mcp_message'` | Send JSON-RPC to MCP server |
| `SDKControlMcpSetServersRequestSchema` | `'mcp_set_servers'` | Replace dynamic MCP servers |
| `SDKControlReloadPluginsRequestSchema` | `'reload_plugins'` | Reload plugins from disk |
| `SDKControlMcpReconnectRequestSchema` | `'mcp_reconnect'` | Reconnect failed MCP server |
| `SDKControlMcpToggleRequestSchema` | `'mcp_toggle'` | Enable/disable MCP server |
| `SDKControlStopTaskRequestSchema` | `'stop_task'` | Stop a running task |
| `SDKControlApplyFlagSettingsRequestSchema` | `'apply_flag_settings'` | Merge flag settings layer |
| `SDKControlGetSettingsRequestSchema` | `'get_settings'` | Get effective + per-source settings |
| `SDKControlElicitationRequestSchema` | `'elicitation'` | MCP elicitation request |

### Control Response Schemas

```typescript
export const SDKControlInitializeResponseSchema  // commands, agents, output_style, models, account, pid?, fast_mode_state?
export const SDKControlMcpStatusResponseSchema   // { mcpServers: McpServerStatus[] }
export const SDKControlGetContextUsageResponseSchema  // detailed context breakdown
export const SDKControlRewindFilesResponseSchema      // { canRewind, error?, filesChanged?, insertions?, deletions? }
export const SDKControlCancelAsyncMessageResponseSchema  // { cancelled: boolean }
export const SDKControlMcpSetServersResponseSchema    // { added, removed, errors }
export const SDKControlReloadPluginsResponseSchema    // { commands, agents, plugins, mcpServers, error_count }
export const SDKControlGetSettingsResponseSchema      // { effective, sources, applied? }
export const SDKControlElicitationResponseSchema      // { action: 'accept'|'decline'|'cancel', content? }
```

### Wire Message Wrappers

```typescript
// Outer request envelope
export const SDKControlRequestSchema = z.object({
  type: z.literal('control_request'),
  request_id: z.string(),
  request: SDKControlRequestInnerSchema(),  // union of all request types
})

// Response envelope
export const SDKControlResponseSchema = z.object({
  type: z.literal('control_response'),
  response: z.union([ControlResponseSchema(), ControlErrorResponseSchema()]),
})

// Cancel (for long-running requests)
export const SDKControlCancelRequestSchema = z.object({
  type: z.literal('control_cancel_request'),
  request_id: z.string(),
})
```

### Aggregate Message Types

```typescript
// Messages written to stdout by CLI
export const StdoutMessageSchema = z.union([
  SDKMessageSchema(),
  SDKStreamlinedTextMessageSchema(),
  SDKStreamlinedToolUseSummaryMessageSchema(),
  SDKPostTurnSummaryMessageSchema(),
  SDKControlResponseSchema(),
  SDKControlRequestSchema(),
  SDKControlCancelRequestSchema(),
  SDKKeepAliveMessageSchema(),
])

// Messages read from stdin by CLI
export const StdinMessageSchema = z.union([
  SDKUserMessageSchema(),
  SDKControlRequestSchema(),
  SDKControlResponseSchema(),
  SDKKeepAliveMessageSchema(),
  SDKUpdateEnvironmentVariablesMessageSchema(),
])
```

### Hook Callback Matcher

```typescript
export const SDKHookCallbackMatcherSchema = z.object({
  matcher: z.string().optional(),
  hookCallbackIds: z.array(z.string()),
  timeout: z.number().optional(),
})
```

### Context Usage Response Detail

The `SDKControlGetContextUsageResponseSchema` response includes:

| Field | Type |
|---|---|
| `categories` | `Array<{ name, tokens, color, isDeferred? }>` |
| `totalTokens` | `number` |
| `maxTokens` | `number` |
| `rawMaxTokens` | `number` |
| `percentage` | `number` |
| `gridRows` | `Array<Array<ContextGridSquare>>` |
| `model` | `string` |
| `memoryFiles` | `Array<{ path, type, tokens }>` |
| `mcpTools` | `Array<{ name, serverName, tokens, isLoaded? }>` |
| `deferredBuiltinTools` | `Array<{ name, tokens, isLoaded }>?` |
| `systemTools` | `Array<{ name, tokens }>?` |
| `systemPromptSections` | `Array<{ name, tokens }>?` |
| `agents` | `Array<{ agentType, source, tokens }>` |
| `slashCommands` | `{ totalCommands, includedCommands, tokens }?` |
| `skills` | `{ totalSkills, includedSkills, tokens, skillFrontmatter[] }?` |
| `autoCompactThreshold` | `number?` |
| `isAutoCompactEnabled` | `boolean` |
| `messageBreakdown` | detailed message token breakdown `?` |
| `apiUsage` | `{ input_tokens, output_tokens, cache_creation_input_tokens, cache_read_input_tokens }` or `null` |

---

## query.ts — Core Async Query Loop

### Purpose

The core agentic query loop. Drives the back-and-forth between the user's prompt, the Claude API, and tool execution. Implemented as an `AsyncGenerator` that yields `StreamEvent | RequestStartEvent | Message | TombstoneMessage | ToolUseSummaryMessage` and returns a `Terminal` value.

### Exports

```typescript
export type QueryParams = {
  messages: Message[]
  systemPrompt: SystemPrompt
  userContext: { [k: string]: string }
  systemContext: { [k: string]: string }
  canUseTool: CanUseToolFn
  toolUseContext: ToolUseContext
  fallbackModel?: string
  querySource: QuerySource
  maxOutputTokensOverride?: number
  maxTurns?: number
  skipCacheWrite?: boolean
  taskBudget?: { total: number }
  deps?: QueryDeps
}

export async function* query(
  params: QueryParams,
): AsyncGenerator<
  StreamEvent | RequestStartEvent | Message | TombstoneMessage | ToolUseSummaryMessage,
  Terminal
>
```

### Internal State Type

```typescript
type State = {
  messages: Message[]
  toolUseContext: ToolUseContext
  autoCompactTracking: AutoCompactTrackingState | undefined
  maxOutputTokensRecoveryCount: number
  hasAttemptedReactiveCompact: boolean
  maxOutputTokensOverride: number | undefined
  pendingToolUseSummary: Promise<ToolUseSummaryMessage | null> | undefined
  stopHookActive: boolean | undefined
  turnCount: number
  transition: Continue | undefined  // Why the previous iteration continued
}
```

### Constants

| Constant | Value | Description |
|---|---|---|
| `MAX_OUTPUT_TOKENS_RECOVERY_LIMIT` | `3` | Max recovery retries for max_output_tokens errors |

### Query Loop Architecture

```
query(params)
  └── queryLoop(params, consumedCommandUuids)
        ├── snapshot config (buildQueryConfig())
        ├── start memory prefetch (startRelevantMemoryPrefetch)
        └── while (true):
              1. yield { type: 'stream_request_start' }
              2. build queryTracking (chainId/depth)
              3. get messages after compact boundary
              4. apply tool result budget (applyToolResultBudget)
              5. snip compact if needed (HISTORY_SNIP feature)
              6. microcompact (deps.microcompact)
              7. context collapse (CONTEXT_COLLAPSE feature)
              8. build fullSystemPrompt
              9. autocompact (deps.autocompact) → maybe yield compact boundary messages
             10. check blocking token limit (if not compacted and not reactive compact)
             11. call model (deps.callModel) → stream assistant messages
             12. execute tools (runTools or StreamingToolExecutor)
             13. yield messages, tool results
             14. handleStopHooks
             15. check for continuation conditions:
                  - stop hooks blocked → continue with blocking errors
                  - maxTurns exceeded → return 'max_turns'
                  - no tool use → check tokenBudget → return 'end_turn'
                  - tool use → continue loop
```

### Recovery Paths

The query loop includes several error recovery paths:

| Error | Recovery |
|---|---|
| `max_output_tokens` | Retry up to `MAX_OUTPUT_TOKENS_RECOVERY_LIMIT` times, incrementing budget |
| Prompt too long | Reactive compact (REACTIVE_COMPACT feature) or return `blocking_limit` |
| Streaming fallback | Tombstone orphaned messages, create fresh `StreamingToolExecutor` |
| FallbackTriggeredError | Switch to fallback model, retry |
| Context collapse overflow | Drain staged collapses via CONTEXT_COLLAPSE feature |

### Feature Flags

`HISTORY_SNIP`, `CONTEXT_COLLAPSE`, `REACTIVE_COMPACT`, `CACHED_MICROCOMPACT`, `TOKEN_BUDGET`, `BG_SESSIONS`

### Tool Execution Integration

- **Streaming tool execution** (gated on `config.gates.streamingToolExecution`): Uses `StreamingToolExecutor` class
- **Sequential tool execution**: Uses `runTools()` from `services/tools/toolOrchestration.js`
- Tool results are yielded back into the loop as `UserMessage` objects

### Key Behaviors

- **`persistReplacements`**: Tool result content replacement is persisted for `agent:*` and `repl_main_thread*` query sources
- **`backfillObservableInput`**: Adds observable fields to tool input before yielding (e.g., expanded file paths) — only when NEW fields are added, not overwrites
- **Tombstoning**: Orphaned messages from failed streaming fallback are tombstoned via `{ type: 'tombstone', message }` events
- **Query chain tracking**: Each iteration increments `queryTracking.depth`; first iteration creates a new `chainId` UUID

---

## QueryEngine.ts — Stateful Query Engine (SDK/Headless)

### Purpose

Owns the complete query lifecycle and session state for a conversation. Designed for the SDK/headless (`-p`) path. One instance per conversation; each `submitMessage()` call starts a new turn while preserving all state (messages, file cache, usage, permission denials).

### Exports

```typescript
export type QueryEngineConfig = {
  cwd: string
  tools: Tools
  commands: Command[]
  mcpClients: MCPServerConnection[]
  agents: AgentDefinition[]
  canUseTool: CanUseToolFn
  getAppState: () => AppState
  setAppState: (f: (prev: AppState) => AppState) => void
  initialMessages?: Message[]
  readFileCache: FileStateCache
  customSystemPrompt?: string
  appendSystemPrompt?: string
  userSpecifiedModel?: string
  fallbackModel?: string
  thinkingConfig?: ThinkingConfig
  maxTurns?: number
  maxBudgetUsd?: number
  taskBudget?: { total: number }
  jsonSchema?: Record<string, unknown>
  verbose?: boolean
  replayUserMessages?: boolean
  handleElicitation?: ToolUseContext['handleElicitation']
  includePartialMessages?: boolean
  setSDKStatus?: (status: SDKStatus) => void
  abortController?: AbortController
  orphanedPermission?: OrphanedPermission
  snipReplay?: (yieldedSystemMsg: Message, store: Message[]) => { messages: Message[]; executed: boolean } | undefined
}

export class QueryEngine {
  constructor(config: QueryEngineConfig)
  async *submitMessage(
    prompt: string | ContentBlockParam[],
    options?: { uuid?: string; isMeta?: boolean },
  ): AsyncGenerator<SDKMessage, void, unknown>
  abort(): void
  getMessages(): Message[]
  getTotalUsage(): NonNullableUsage
  getPermissionDenials(): SDKPermissionDenial[]
}
```

### Internal State

```typescript
private config: QueryEngineConfig
private mutableMessages: Message[]
private abortController: AbortController
private permissionDenials: SDKPermissionDenial[]
private totalUsage: NonNullableUsage
private hasHandledOrphanedPermission: boolean
private readFileState: FileStateCache
private discoveredSkillNames: Set<string>  // cleared each submitMessage()
private loadedNestedMemoryPaths: Set<string>  // grows across turns
```

### `submitMessage()` Flow

1. Clear `discoveredSkillNames`
2. Set CWD via `setCwd(cwd)`
3. Wrap `canUseTool` to track permission denials
4. Resolve initial model and thinking config
5. `fetchSystemPromptParts()` — build system prompt, user context, system context
6. Build `systemPrompt` from `customSystemPrompt | defaultSystemPrompt` + `memoryMechanicsPrompt?` + `appendSystemPrompt?`
7. Register structured output enforcement (if `jsonSchema` + synthetic output tool)
8. Build initial `processUserInputContext`
9. Handle orphaned permission (once per lifetime)
10. Call `processUserInput()` for the user's prompt
11. Push new messages to `mutableMessages`
12. Persist to transcript (before API call — ensures `--resume` works even if killed mid-flight)
13. Replay user messages if `replayUserMessages: true`
14. Update `ToolPermissionContext.alwaysAllowRules.command` from `processUserInput` result
15. Re-build `processUserInputContext` with updated messages and model
16. Stream from `query()` generator — convert to `SDKMessage`, yield
17. Track usage via `accumulateUsage()` / `updateUsage()`
18. Handle local command output, compact boundaries, snip boundaries

### Transcript Persistence

In `submitMessage()`, the user's messages are written to the transcript **before** entering the API query loop. Timing variants:
- **`--bare` / `isBareMode()`**: Fire-and-forget (saves ~4ms on SSD)
- **`CLAUDE_CODE_EAGER_FLUSH` or `CLAUDE_CODE_IS_COWORK`**: Awaited + flushed
- **Default**: Awaited

### `ProcessUserInputContext` Internals

First build (before slash command processing):
- `setMessages`: writes back to `mutableMessages`
- `isNonInteractiveSession: true`

Second build (after slash command processing):
- `setMessages`: no-op (slash commands already committed)

---

## query/config.ts — Query Configuration Snapshot

### Purpose

Captures immutable configuration values at query entry. Separated from per-iteration state to make future `step()` extraction (pure reducer pattern) tractable. Intentionally excludes `feature()` gates (those are build-time tree-shaking boundaries).

### Exports

```typescript
export type QueryConfig = {
  sessionId: SessionId
  gates: {
    streamingToolExecution: boolean  // Statsig: 'tengu_streaming_tool_execution2'
    emitToolUseSummaries: boolean    // env: CLAUDE_CODE_EMIT_TOOL_USE_SUMMARIES
    isAnt: boolean                   // env: USER_TYPE === 'ant'
    fastModeEnabled: boolean         // env: !CLAUDE_CODE_DISABLE_FAST_MODE
  }
}

export function buildQueryConfig(): QueryConfig
```

### Gate Details

| Gate | Source | Key |
|---|---|---|
| `streamingToolExecution` | Statsig (cached, may be stale) | `tengu_streaming_tool_execution2` |
| `emitToolUseSummaries` | Environment variable | `CLAUDE_CODE_EMIT_TOOL_USE_SUMMARIES` |
| `isAnt` | Environment variable | `USER_TYPE === 'ant'` |
| `fastModeEnabled` | Environment variable | `!CLAUDE_CODE_DISABLE_FAST_MODE` |

---

## query/deps.ts — Query Dependency Injection

### Purpose

I/O dependencies for `query()`, passed via `QueryParams.deps`. Enables test injection of fakes without `spyOn`-per-module boilerplate.

### Exports

```typescript
export type QueryDeps = {
  callModel: typeof queryModelWithStreaming       // API streaming call
  microcompact: typeof microcompactMessages       // microcompaction
  autocompact: typeof autoCompactIfNeeded         // autocompaction
  uuid: () => string                              // UUID generation
}

export function productionDeps(): QueryDeps
```

### Production Dependencies

| Dep | Implementation |
|---|---|
| `callModel` | `queryModelWithStreaming` from `services/api/claude.js` |
| `microcompact` | `microcompactMessages` from `services/compact/microCompact.js` |
| `autocompact` | `autoCompactIfNeeded` from `services/compact/autoCompact.js` |
| `uuid` | `randomUUID` from Node.js `crypto` module |

---

## query/stopHooks.ts — Stop Hook Orchestration

### Purpose

Orchestrates all end-of-turn hooks: `Stop`, `SubagentStop`, `TeammateIdle`, `TaskCompleted`. Also handles background side-effects (prompt suggestion, memory extraction, auto-dream, computer-use cleanup).

### Exports

```typescript
export async function* handleStopHooks(
  messagesForQuery: Message[],
  assistantMessages: AssistantMessage[],
  systemPrompt: SystemPrompt,
  userContext: { [k: string]: string },
  systemContext: { [k: string]: string },
  toolUseContext: ToolUseContext,
  querySource: QuerySource,
  stopHookActive?: boolean,
): AsyncGenerator<
  StreamEvent | RequestStartEvent | Message | TombstoneMessage | ToolUseSummaryMessage,
  StopHookResult
>

type StopHookResult = {
  blockingErrors: Message[]
  preventContinuation: boolean
}
```

### Execution Order

1. **`saveCacheSafeParams()`** — snapshot context for prompt suggestion / btw queries (main thread and SDK only)
2. **Template job classification** (TEMPLATES feature, main thread only, non-subagent): `classifyAndWriteState()` — max 60s timeout
3. **Background side-effects** (non-bare mode):
   - `executePromptSuggestion()` (fire-and-forget, unless `CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION=false`)
   - `executeExtractMemories()` (EXTRACT_MEMORIES feature, main thread only)
   - `executeAutoDream()` (non-subagent)
4. **Computer-use cleanup** (CHICAGO_MCP feature, main thread only): `cleanupComputerUseAfterTurn()`
5. **`executeStopHooks()`** — runs Stop/SubagentStop hooks in parallel, yields progress messages
6. **Summary message** if any hooks ran
7. **Notification** if hook errors occurred
8. **Teammate hooks** (if `isTeammate()`):
   - `executeTaskCompletedHooks()` for in-progress tasks owned by this agent
   - `executeTeammateIdleHooks()`

### Hook Result Types

| Result Field | Effect |
|---|---|
| `blockingError` | Creates `UserMessage` with `isMeta: true`, added to `blockingErrors` |
| `preventContinuation` | Sets flag, yields `hook_stopped_continuation` attachment |
| Abort signal | Yields `UserInterruptionMessage`, returns `{ blockingErrors: [], preventContinuation: true }` |

### Telemetry Events

- `tengu_pre_stop_hooks_cancelled` — hook aborted mid-execution
- `tengu_stop_hook_error` — exception in hook execution

---

## query/tokenBudget.ts — Token Budget Tracking

### Purpose

Tracks token budget utilization across query loop iterations to decide whether to continue or stop based on the `+500k auto-continue` feature (distinct from the API `task_budget`).

### Exports

```typescript
export type BudgetTracker = {
  continuationCount: number
  lastDeltaTokens: number
  lastGlobalTurnTokens: number
  startedAt: number
}

export function createBudgetTracker(): BudgetTracker

export type TokenBudgetDecision =
  | { action: 'continue'; nudgeMessage: string; continuationCount: number; pct: number; turnTokens: number; budget: number }
  | { action: 'stop'; completionEvent: { continuationCount: number; pct: number; turnTokens: number; budget: number; diminishingReturns: boolean; durationMs: number } | null }

export function checkTokenBudget(
  tracker: BudgetTracker,
  agentId: string | undefined,
  budget: number | null,
  globalTurnTokens: number,
): TokenBudgetDecision
```

### Constants

| Constant | Value | Description |
|---|---|---|
| `COMPLETION_THRESHOLD` | `0.9` | 90% of budget consumed → stop |
| `DIMINISHING_THRESHOLD` | `500` | Tokens delta < 500 across two checks → diminishing returns |

### Algorithm

```
checkTokenBudget(tracker, agentId, budget, globalTurnTokens):
  if agentId or budget null or budget <= 0:
    return { action: 'stop', completionEvent: null }

  pct = round(turnTokens / budget * 100)
  deltaSinceLast = globalTurnTokens - tracker.lastGlobalTurnTokens

  isDiminishing = (
    continuationCount >= 3 AND
    deltaSinceLast < 500 AND
    lastDeltaTokens < 500
  )

  if !isDiminishing AND turnTokens < budget * 0.9:
    → return { action: 'continue', nudgeMessage: ... }

  if isDiminishing OR continuationCount > 0:
    → return { action: 'stop', completionEvent: { ..., diminishingReturns: isDiminishing } }

  → return { action: 'stop', completionEvent: null }
```

---

## context.ts — System & User Context Providers

### Purpose

Provides memoized context functions that build the `systemContext` and `userContext` dictionaries prepended to each conversation. Both are cached for the duration of the conversation. Includes the git status, CLAUDE.md content, and current date.

### Exports

```typescript
export function getSystemPromptInjection(): string | null
export function setSystemPromptInjection(value: string | null): void

export const getGitStatus: () => Promise<string | null>  // memoized
export const getSystemContext: () => Promise<{ [k: string]: string }>  // memoized
export const getUserContext: () => Promise<{ [k: string]: string }>    // memoized
```

### Constants

| Constant | Value | Description |
|---|---|---|
| `MAX_STATUS_CHARS` | `2000` | Max characters for `git status --short` output |

### `getGitStatus()` — Memoized

Returns `null` in test environment. Otherwise:
1. Checks `getIsGit()` — returns `null` if not a git repo
2. Runs in parallel: `getBranch()`, `getDefaultBranch()`, `git status --short`, `git log --oneline -n 5`, `git config user.name`
3. Truncates status at `MAX_STATUS_CHARS` (appends truncation notice)
4. Returns formatted string with branch, main branch, git user, status, and recent commits

**Format**:
```
This is the git status at the start of the conversation. Note that this status is a snapshot in time, and will not update during the conversation.

Current branch: <branch>

Main branch (you will usually use this for PRs): <mainBranch>

Git user: <userName>

Status:
<status or "(clean)">

Recent commits:
<log>
```

### `getSystemContext()` — Memoized

```typescript
{
  gitStatus?: string,    // from getGitStatus() — skipped for CCR or when git disabled
  cacheBreaker?: string, // "[CACHE_BREAKER: injection]" — BREAK_CACHE_COMMAND feature only
}
```

Skips git status when:
- `CLAUDE_CODE_REMOTE=true` (CCR environment)
- `shouldIncludeGitInstructions()` returns false

### `getUserContext()` — Memoized

```typescript
{
  claudeMd?: string,    // Combined CLAUDE.md content
  currentDate: string,  // "Today's date is YYYY-MM-DD."
}
```

CLAUDE.md loading is disabled when:
- `CLAUDE_CODE_DISABLE_CLAUDE_MDS=true`
- `isBareMode()` AND no `--add-dir` directories

Side effect: calls `setCachedClaudeMdContent()` to cache content for auto-mode classifier.

### `setSystemPromptInjection()` Side Effect

When injection changes, clears both `getUserContext.cache` and `getSystemContext.cache` (from lodash memoize) to force rebuild.

---

## history.ts — Prompt History Management

### Purpose

Manages the persistent prompt history (up-arrow navigation and Ctrl+R fuzzy search). History is stored as JSONL in `~/.claude/history.jsonl`. Handles pasted content with both inline (≤ 1024 bytes) and external hash-based storage. Entries are scoped to project root.

### Exports

```typescript
// Pasted content helpers
export function getPastedTextRefNumLines(text: string): number
export function formatPastedTextRef(id: number, numLines: number): string
export function formatImageRef(id: number): string
export function parseReferences(input: string): Array<{ id: number; match: string; index: number }>
export function expandPastedTextRefs(input: string, pastedContents: Record<number, PastedContent>): string

// History reading
export async function* makeHistoryReader(): AsyncGenerator<HistoryEntry>
export async function* getTimestampedHistory(): AsyncGenerator<TimestampedHistoryEntry>
export async function* getHistory(): AsyncGenerator<HistoryEntry>

// History writing
export function addToHistory(command: HistoryEntry | string): void
export function clearPendingHistoryEntries(): void
export function removeLastFromHistory(): void

// Types
export type TimestampedHistoryEntry = {
  display: string
  timestamp: number
  resolve: () => Promise<HistoryEntry>
}
```

### Constants

| Constant | Value | Description |
|---|---|---|
| `MAX_HISTORY_ITEMS` | `100` | Max entries returned from `getHistory()` |
| `MAX_PASTED_CONTENT_LENGTH` | `1024` | Threshold for inline vs hash-based storage |

### Internal Types

```typescript
type LogEntry = {
  display: string
  pastedContents: Record<number, StoredPastedContent>
  timestamp: number
  project: string
  sessionId?: string
}

type StoredPastedContent = {
  id: number
  type: 'text' | 'image'
  content?: string          // inline (≤ MAX_PASTED_CONTENT_LENGTH)
  contentHash?: string      // hash ref for large pastes
  mediaType?: string
  filename?: string
}
```

### History File

**Path**: `join(getClaudeConfigHomeDir(), 'history.jsonl')`

**Locking**: Uses file-based lockfile with:
- `stale: 10000` ms
- Retries: 3, min timeout 50ms

### `addToHistory()` Behavior

1. Skips if `CLAUDE_CODE_SKIP_PROMPT_HISTORY=true` (tmux subprocess sessions)
2. Registers cleanup hook on first call (flushes pending entries on process exit)
3. Calls `addToPromptHistory()` async (fire-and-forget)

### `getHistory()` Ordering

Current session entries first (newest-first), then other session entries (also newest-first). Same `MAX_HISTORY_ITEMS` window. This prevents concurrent sessions from interleaving up-arrow history.

### `removeLastFromHistory()` — Undo Last Entry

Fast path: if entry is still in `pendingEntries`, splices it out.
Slow path: if already flushed, adds timestamp to `skippedTimestamps` set (consulted during reads).
One-shot: clears `lastAddedEntry` after use.

### Paste Reference Pattern

Reference format: `\[(Pasted text|Image|\.\.\.Truncated text) #(\d+)(?: \+\d+ lines)?(\.)*\]`

Examples:
- `[Pasted text #1]` — zero-line paste
- `[Pasted text #1 +10 lines]` — multi-line paste (counts `\n` occurrences)
- `[Image #2]` — image paste

---

## cost-tracker.ts — Session Cost Tracking

### Purpose

Manages session cost/usage tracking, persistence to project config, and formatted display. Delegates all state to `bootstrap/state.ts`.

### Exports

```typescript
// Re-exports from bootstrap/state.ts
export { getTotalCostUSD as getTotalCost }
export { getTotalDuration }
export { getTotalAPIDuration }
export { getTotalAPIDurationWithoutRetries }
export { addToTotalLinesChanged }
export { getTotalLinesAdded, getTotalLinesRemoved }
export { getTotalInputTokens, getTotalOutputTokens }
export { getTotalCacheReadInputTokens, getTotalCacheCreationInputTokens }
export { getTotalWebSearchRequests }
export { hasUnknownModelCost }
export { resetStateForTests, resetCostState }
export { setHasUnknownModelCost }
export { getModelUsage, getUsageForModel }
export { formatCost }  // internal function, also exported

// New functions
export function getStoredSessionCosts(sessionId: string): StoredCostState | undefined
export function restoreCostStateForSession(sessionId: string): boolean
export function saveCurrentSessionCosts(fpsMetrics?: FpsMetrics): void
export function addToTotalSessionCost(cost: number, usage: Usage, model: string): number
export function formatTotalCost(): string
```

### Internal Types

```typescript
type StoredCostState = {
  totalCostUSD: number
  totalAPIDuration: number
  totalAPIDurationWithoutRetries: number
  totalToolDuration: number
  totalLinesAdded: number
  totalLinesRemoved: number
  lastDuration: number | undefined
  modelUsage: { [modelName: string]: ModelUsage } | undefined
}
```

### `addToTotalSessionCost()` Detail

1. Calls `addToTotalModelUsage()` — aggregates per-model token counts
2. Calls `addToTotalCostState()` in bootstrap/state.ts
3. Records to OpenTelemetry counters (`getCostCounter()`, `getTokenCounter()`)
4. Processes advisor usage from `getAdvisorUsage(usage)` — recursively calls itself for each advisor model
5. Returns total cost (including advisor costs)

### `saveCurrentSessionCosts()` — Persisted Fields

Writes to project config:
- `lastCost`, `lastAPIDuration`, `lastAPIDurationWithoutRetries`, `lastToolDuration`, `lastDuration`
- `lastLinesAdded`, `lastLinesRemoved`
- `lastTotalInputTokens`, `lastTotalOutputTokens`
- `lastTotalCacheCreationInputTokens`, `lastTotalCacheReadInputTokens`
- `lastTotalWebSearchRequests`
- `lastFpsAverage`, `lastFpsLow1Pct` (from `fpsMetrics`)
- `lastModelUsage` (per-model: input/output/cache tokens, web searches, cost)
- `lastSessionId`

### `formatTotalCost()` — Display Format

```
Total cost:            $X.XXXX
Total duration (API):  Xs
Total duration (wall): Xs
Total code changes:    N lines added, N lines removed
Usage by model:
   claude-sonnet-4-6:  N input, N output, N cache read, N cache write ($X.XXXX)
```

### `formatCost()` Helper

```typescript
function formatCost(cost: number, maxDecimalPlaces: number = 4): string
// Returns "$X.XX" if cost > 0.5, else "$X.XXXX" (or fewer decimal places)
```

---

## costHook.ts — React Cost Summary Hook

### Purpose

React hook that registers a `process.exit` listener to print cost summary and save session costs when the process exits.

### Exports

```typescript
export function useCostSummary(getFpsMetrics?: () => FpsMetrics | undefined): void
```

### Behavior

- Runs once on mount (empty dependency array)
- On `process.exit`:
  1. If `hasConsoleBillingAccess()`: writes `formatTotalCost()` to stdout
  2. Calls `saveCurrentSessionCosts(getFpsMetrics?.())`
- Cleans up the exit listener on unmount

---

## projectOnboardingState.ts — Project Onboarding State

### Purpose

Tracks and controls the display of the project onboarding checklist (shown when a user first opens a new project).

### Exports

```typescript
export type Step = {
  key: string
  text: string
  isComplete: boolean
  isCompletable: boolean
  isEnabled: boolean
}

export function getSteps(): Step[]
export function isProjectOnboardingComplete(): boolean
export function maybeMarkProjectOnboardingComplete(): void
export const shouldShowProjectOnboarding: () => boolean  // memoized
export function incrementProjectOnboardingSeenCount(): void
```

### Steps

| Key | Condition to enable | Completion check |
|---|---|---|
| `'workspace'` | `isDirEmpty(getCwd())` | Never auto-completes (user must act) |
| `'claudemd'` | `!isDirEmpty(getCwd())` | `existsSync(join(getCwd(), 'CLAUDE.md'))` |

### `shouldShowProjectOnboarding()` — Memoized

Returns `false` if any of:
- `projectConfig.hasCompletedProjectOnboarding === true`
- `projectConfig.projectOnboardingSeenCount >= 4`
- `process.env.IS_DEMO` is set
- `isProjectOnboardingComplete()` returns true

### `maybeMarkProjectOnboardingComplete()` Behavior

Short-circuits on `hasCompletedProjectOnboarding: true` in cached config (avoids filesystem hit on every prompt submit). Saves to project config when complete.

---

## bootstrap/state.ts — Global Session State

### Purpose

The single module-level state singleton for a Claude Code process. **DO NOT ADD MORE STATE HERE** (documented with triple comment emphasis). Contains all session-scoped values including costs, tokens, model configuration, telemetry, agent state, and session identity.

### State Structure

The `State` type is a large flat object. Key groupings:

#### Session Identity

| Field | Type | Description |
|---|---|---|
| `sessionId` | `SessionId` | UUID (randomUUID at init) |
| `parentSessionId` | `SessionId?` | Parent session for lineage tracking |
| `originalCwd` | `string` | CWD at startup (NFC-normalized) |
| `projectRoot` | `string` | Stable project root (set by --worktree; not updated mid-session) |
| `cwd` | `string` | Current working directory (updated by setCwd) |
| `sessionProjectDir` | `string | null` | Dir containing session JSONL; null = derive from originalCwd |

#### Cost & Token Counters

| Field | Type |
|---|---|
| `totalCostUSD` | `number` |
| `totalAPIDuration` | `number` |
| `totalAPIDurationWithoutRetries` | `number` |
| `totalToolDuration` | `number` |
| `totalLinesAdded`, `totalLinesRemoved` | `number` |
| `modelUsage` | `{ [modelName: string]: ModelUsage }` |

#### Per-Turn Counters (reset each turn)

| Field | Type |
|---|---|
| `turnHookDurationMs` | `number` |
| `turnToolDurationMs` | `number` |
| `turnClassifierDurationMs` | `number` |
| `turnToolCount` | `number` |
| `turnHookCount` | `number` |
| `turnClassifierCount` | `number` |

#### Model Configuration

| Field | Type | Description |
|---|---|---|
| `mainLoopModelOverride` | `ModelSetting?` | Set by --model flag |
| `initialMainLoopModel` | `ModelSetting` | Set at startup |
| `modelStrings` | `ModelStrings | null` | Loaded model string definitions |

#### Telemetry / OpenTelemetry

| Field | Type |
|---|---|
| `meter` | `Meter | null` |
| `meterProvider` | `MeterProvider | null` |
| `loggerProvider` | `LoggerProvider | null` |
| `tracerProvider` | `BasicTracerProvider | null` |
| `eventLogger` | `ReturnType<typeof logs.getLogger> | null` |
| `sessionCounter` | `AttributedCounter | null` |
| `locCounter`, `prCounter`, `commitCounter` | `AttributedCounter | null` |
| `costCounter`, `tokenCounter` | `AttributedCounter | null` |
| `codeEditToolDecisionCounter`, `activeTimeCounter` | `AttributedCounter | null` |
| `statsStore` | `{ observe(name, value): void } | null` |

#### Session Flags

| Field | Default | Description |
|---|---|---|
| `isInteractive` | `false` | Interactive REPL mode |
| `kairosActive` | `false` | Assistant (KAIROS) mode active |
| `strictToolResultPairing` | `false` | HFI mode — throws on mismatch |
| `sessionBypassPermissionsMode` | `false` | Not persisted |
| `sessionPersistenceDisabled` | `false` | Disable transcript write |
| `sessionTrustAccepted` | `false` | Session-only trust (home dir) |
| `hasExitedPlanMode` | `false` | For re-entry guidance |
| `scheduledTasksEnabled` | `false` | Set by cron scheduler |
| `isRemoteMode` | `false` | --remote flag |

#### Prompt Cache Latches

All start as `null` (not yet triggered), flip to `true` once, and stay `true`:

| Field | Purpose |
|---|---|
| `afkModeHeaderLatched` | Sticky AFK_MODE_BETA_HEADER |
| `fastModeHeaderLatched` | Sticky FAST_MODE_BETA_HEADER |
| `cacheEditingHeaderLatched` | Sticky cache-editing beta header |
| `thinkingClearLatched` | Clear thinking after >1h idle |

### Exported Functions (selected)

#### Session Identity

```typescript
export function getSessionId(): SessionId
export function regenerateSessionId(options?: { setCurrentAsParent?: boolean }): SessionId
export function getParentSessionId(): SessionId | undefined
export function switchSession(sessionId: SessionId, projectDir?: string | null): void
export function getSessionProjectDir(): string | null
export const onSessionSwitch: (listener: (id: SessionId) => void) => () => void
export function getOriginalCwd(): string
export function setOriginalCwd(cwd: string): void
export function getProjectRoot(): string
export function setProjectRoot(cwd: string): void
export function getCwdState(): string
export function setCwdState(cwd: string): void
```

#### Cost / Duration Tracking

```typescript
export function addToTotalDurationState(duration: number, durationWithoutRetries: number): void
export function addToTotalCostState(cost: number, modelUsage: ModelUsage, model: string): void
export function getTotalCostUSD(): number
export function getTotalAPIDuration(): number
export function getTotalDuration(): number
export function getTotalAPIDurationWithoutRetries(): number
export function getTotalToolDuration(): number
export function addToToolDuration(duration: number): void
export function getTurnHookDurationMs(): number
export function addToTurnHookDuration(duration: number): void
export function resetTurnHookDuration(): void
export function getTurnHookCount(): number
export function getTurnToolDurationMs(): number
export function resetTurnToolDuration(): void
export function getTurnToolCount(): number
export function getTurnClassifierDurationMs(): number
export function addToTurnClassifierDuration(duration: number): void
export function resetTurnClassifierDuration(): void
export function getTurnClassifierCount(): number
```

#### Token Accounting

```typescript
export function getTotalInputTokens(): number   // sumBy modelUsage.inputTokens
export function getTotalOutputTokens(): number
export function getTotalCacheReadInputTokens(): number
export function getTotalCacheCreationInputTokens(): number
export function getTotalWebSearchRequests(): number
export function getModelUsage(): { [modelName: string]: ModelUsage }
export function getUsageForModel(model: string): ModelUsage | undefined
```

#### Turn Token Budget

```typescript
export function getTurnOutputTokens(): number   // current turn output tokens
export function getCurrentTurnTokenBudget(): number | null
export function snapshotOutputTokensForTurn(budget: number | null): void
export function getBudgetContinuationCount(): number
export function incrementBudgetContinuationCount(): void
```

#### Post-Compaction Tracking

```typescript
export function markPostCompaction(): void       // sets pendingPostCompaction=true
export function consumePostCompaction(): boolean // returns true once after compaction, resets
```

#### Cost State Persistence

```typescript
export function resetCostState(): void
export function setCostStateForRestore({ totalCostUSD, totalAPIDuration, ... }): void
export function resetStateForTests(): void
```

#### Scroll Drain

```typescript
export function markScrollActivity(): void
export function getIsScrollDraining(): boolean
export async function waitForScrollIdle(): Promise<void>
```

#### Model

```typescript
export function getMainLoopModelOverride(): ModelSetting | undefined
export function getInitialMainLoopModel(): ModelSetting
export function setMainLoopModelOverride(model: ModelSetting | undefined): void
export function setInitialMainLoopModel(model: ModelSetting): void
export function getSdkBetas(): string[] | undefined
export function setSdkBetas(betas: string[] | undefined): void
```

#### Telemetry Setters

```typescript
export function setMeter(meter: Meter, createAttributedCounter: ...): void
export function getSessionCounter(): AttributedCounter | null
export function setStatsStore(store: ...): void
export function getStatsStore(): ...
export function updateLastInteractionTime(immediate?: boolean): void
export function flushInteractionTime(): void
```

#### Misc Session State

```typescript
export function setIsInteractive(v: boolean): void
export function getIsNonInteractiveSession(): boolean
export function setKairosActive(v: boolean): void
export function isSessionPersistenceDisabled(): boolean
export function setSessionPersistenceDisabled(v: boolean): void
export function setMainThreadAgentType(type: string | undefined): void
export function setIsRemoteMode(v: boolean): void
export function setClientType(type: string): void
export function setSessionSource(source: string | undefined): void
export function setInlinePlugins(dirs: string[]): void
export function getAdditionalDirectoriesForClaudeMd(): string[]
export function setAdditionalDirectoriesForClaudeMd(dirs: string[]): void
export function setAllowedChannels(channels: ChannelEntry[]): void
export function setAllowedSettingSources(sources: SettingSource[]): void
export function setSdkBetas(betas: string[] | undefined): void
export function setCachedClaudeMdContent(content: string | null): void
export function getLastMainRequestId(): string | undefined
export function setLastMainRequestId(requestId: string): void
export function setTeleportedSessionInfo(info: ...): void
```

### Important Design Notes

- **Singleton**: `STATE` is a module-level constant, initialized once via `getInitialState()`
- **`projectRoot` vs `originalCwd`**: `projectRoot` is set at startup (including by `--worktree`) and **never** updated by `EnterWorktreeTool`. `originalCwd` is for file operations; `projectRoot` is for session identity (history, skills)
- **`sessionProjectDir`**: Always reset on `switchSession()` and `regenerateSessionId()`. `null` means "derive from `originalCwd`"
- **Scroll drain**: Module-level (not in `STATE`) — ephemeral hot-path flag with 150ms debounce
- **Bootstrap isolation**: This module must remain a leaf in the import DAG (cannot import from `src/utils/` directly — uses path aliases)

### `AttributedCounter` Type

```typescript
export type AttributedCounter = {
  add(value: number, additionalAttributes?: Attributes): void
}
```

### `ChannelEntry` Type

```typescript
export type ChannelEntry =
  | { kind: 'plugin'; name: string; marketplace: string; dev?: boolean }
  | { kind: 'server'; name: string; dev?: boolean }
```

---

## assistant/sessionHistory.ts — Remote Session History Pagination

### Purpose

Fetches conversation event history from the Claude API for remote (CCR/BYOC) sessions. Used by the assistant/teleport feature to replay conversation history when resuming a remote session.

### Exports

```typescript
export const HISTORY_PAGE_SIZE = 100

export type HistoryPage = {
  events: SDKMessage[]      // chronological within page
  firstId: string | null    // oldest event ID → before_id cursor for next-older page
  hasMore: boolean          // true = older events exist
}

export type HistoryAuthCtx = {
  baseUrl: string
  headers: Record<string, string>
}

export async function createHistoryAuthCtx(sessionId: string): Promise<HistoryAuthCtx>
export async function fetchLatestEvents(ctx: HistoryAuthCtx, limit?: number): Promise<HistoryPage | null>
export async function fetchOlderEvents(ctx: HistoryAuthCtx, beforeId: string, limit?: number): Promise<HistoryPage | null>
```

### API Endpoints

Base URL: `${getOauthConfig().BASE_API_URL}/v1/sessions/${sessionId}/events`

| Function | Query Params | Description |
|---|---|---|
| `fetchLatestEvents` | `{ limit, anchor_to_latest: true }` | Newest `limit` events, chronological |
| `fetchOlderEvents` | `{ limit, before_id: beforeId }` | Events before cursor |

### Authentication

Requires OAuth access token + organization UUID. Headers:
- Standard OAuth headers (`getOAuthHeaders(accessToken)`)
- `anthropic-beta: ccr-byoc-2025-07-29`
- `x-organization-uuid: orgUUID`

### Request Configuration

- **Timeout**: 15,000ms per request
- **Status validation**: `validateStatus: () => true` (manual status check)
- **Error handling**: Returns `null` on network error or non-200 status; logs HTTP status to debug

### Response Type

```typescript
type SessionEventsResponse = {
  data: SDKMessage[]
  has_more: boolean
  first_id: string | null
  last_id: string | null
}
```

---

## Cross-Cutting Concerns

### Startup Profiling

Throughout these files, `profileCheckpoint(label)` calls mark timing milestones for startup performance analysis. Key checkpoints:

| Checkpoint | Location |
|---|---|
| `main_tsx_entry` | First line of main.tsx (before imports) |
| `main_tsx_imports_loaded` | After all imports loaded |
| `cli_entry` | cli.tsx after profile import |
| `cli_before_main_import` | Before `import('../main.js')` |
| `cli_after_main_import` | After main.ts loaded |
| `cli_after_main_complete` | After `main()` returns |
| `init_function_start` / `init_function_end` | init.ts boundaries |
| `main_function_start` | Entry to `main()` |
| `query_fn_entry` | Each query loop iteration |
| `query_api_streaming_start` | Before first API streaming call |

### Feature Flags System

Feature flags (`feature('FLAG_NAME')`) from `bun:bundle` are **build-time** dead-code elimination gates, not runtime toggles. The bun bundler evaluates these at build time and removes unreachable code branches from external builds.

Runtime gates use:
- `checkStatsigFeatureGate_CACHED_MAY_BE_STALE()` — Statsig experiment gates
- `isEnvTruthy(process.env.*)` — environment variable gates
- GrowthBook feature values — for more complex multi-variant experiments

### Query Source Values

The `querySource: QuerySource` parameter in `query()` and `handleStopHooks()` categorizes the origin of each query:

| Value | Description |
|---|---|
| `'repl_main_thread'` | Interactive REPL, main user turn |
| `'sdk'` | SDK/headless path |
| `'agent:*'` | Subagent (AgentTool) |
| `'compact'` | Compaction fork |
| `'session_memory'` | Session memory fork |

Values starting with `'agent:'` or `'repl_main_thread'` enable transcript persistence of content replacements.

### Dependency Injection Pattern

`QueryDeps` (in `query/deps.ts`) is the primary example of DI in this codebase. Rather than `jest.spyOn` across 6-8 files, tests can pass `deps` to `QueryParams` directly. The pattern is intentionally narrow (4 deps) with the note that it can be expanded.
