# Claude Code — Services, Context, State & Screens

This document exhaustively covers all files in `src/services/`, `src/context/`, `src/bootstrap/state.ts`, `src/coordinator/`, `src/server/`, and `src/screens/`. Every exported symbol is listed with its full signature, key logic, configuration, and dependencies.

---

## Table of Contents

1. [bootstrap/state.ts](#bootstrapstatets)
2. [coordinator/coordinatorMode.ts](#coordinatorcoordinatormodetse)
3. [server/types.ts](#servertypests)
4. [server/createDirectConnectSession.ts](#servercreatedirectconnectsessionts)
5. [server/directConnectManager.ts](#serverdirectconnectmanagerts)
6. [services/analytics/config.ts](#servicesanalyticsconfigts)
7. [services/analytics/growthbook.ts](#servicesanalyticsgrowthbookts)
8. [services/analytics/metadata.ts](#servicesanalyticsmetadatats)
9. [services/analytics/index.ts](#servicesanalyticsindexts)
10. [services/analytics/sink.ts](#servicesanalyticssinkt)
11. [services/analytics/sinkKillswitch.ts](#servicesanalyticssinkKillswitchts)
12. [services/analytics/datadog.ts](#servicesanalyticsdatadogts)
13. [services/analytics/firstPartyEventLogger.ts](#servicesanalyticsfirstpartyeventloggerts)
14. [services/analytics/firstPartyEventLoggingExporter.ts](#servicesanalyticsfirstpartyeventloggingexporterts)
15. [services/api/bootstrap.ts](#servicesapibootstrapts)
16. [services/api/client.ts](#servicesapiclientts)
17. [services/api/claude.ts](#servicesapicludets)
18. [services/api/dumpPrompts.ts](#servicesapidumppromptsts)
19. [services/api/emptyUsage.ts](#servicesapiemptyusagets)
20. [services/api/errorUtils.ts](#servicesapierrorutilsts)
21. [services/api/errors.ts](#servicesapierrorsts)
22. [services/api/filesApi.ts](#servicesapifilesapits)
23. [services/api/firstTokenDate.ts](#servicesapifirsttokendatets)
24. [services/api/grove.ts](#servicesapigrovet)
25. [services/api/logging.ts](#servicesapiloggingts)
26. [services/api/metricsOptOut.ts](#servicesapimetricsoptoutts)
27. [services/api/overageCreditGrant.ts](#servicesapiovaragecreditgrantts)
28. [services/api/promptCacheBreakDetection.ts](#servicesapipromptcachebreakdetectionts)
29. [services/api/referral.ts](#servicesapireferralts)
30. [services/api/sessionIngress.ts](#servicesapisessioningressts)
31. [services/api/ultrareviewQuota.ts](#servicesapiultrareviewquotats)
32. [services/api/usage.ts](#servicesapiusagets)
33. [services/api/withRetry.ts](#servicesapiwithretryts)
34. [services/AgentSummary/agentSummary.ts](#servicesagentsummaryagentsummaryts)
35. [services/autoDream/autoDream.ts](#servicesautodreamautodreamts)
36. [services/autoDream/config.ts](#servicesautodreamconfigts)
37. [services/autoDream/consolidationLock.ts](#servicesautodreamconsolidationlockts)
38. [services/autoDream/consolidationPrompt.ts](#servicesautodreamconsolidationpromptt)
39. [services/awaySummary.ts](#servicesawaysummaryts)
40. [services/claudeAiLimits.ts](#servicesclaudeailimitsts)
41. [services/claudeAiLimitsHook.ts](#servicesclaudeailimitshookts)
42. [services/compact/apiMicrocompact.ts](#servicescompactapimicrocompactts)
43. [services/compact/autoCompact.ts](#servicescompactautocompactts)
44. [services/compact/compact.ts](#servicescompactcompactts)
45. [services/compact/compactWarningHook.ts](#servicescompactcompactwarninghookts)
46. [services/compact/compactWarningState.ts](#servicescompactcompactwarningstatets)
47. [services/compact/grouping.ts](#servicescompactgroupingts)
48. [services/compact/microCompact.ts](#servicescompactmicrocompactts)
49. [services/compact/postCompactCleanup.ts](#servicescompactpostcompactcleanuptes)
50. [services/compact/prompt.ts](#servicescompactpromptts)
51. [services/compact/sessionMemoryCompact.ts](#servicescompactsessionmemorycompactts)
52. [services/compact/timeBasedMCConfig.ts](#servicescompacttimebasedmcconfigts)
53. [services/diagnosticTracking.ts](#servicesdiagnostictrackingtss)
54. [services/internalLogging.ts](#servicesinternalloggingts)
55. [services/MagicDocs/magicDocs.ts](#servicesmagicdocsmagicdocsts)
56. [services/MagicDocs/prompts.ts](#servicesmagicdocspromptsts)
57. [services/mcpServerApproval.tsx](#servicesmcpserverapprovala)
58. [services/mockRateLimits.ts](#servicesmockratelimitsts)
59. [services/MCP (mcp/)](#services-mcp)
60. [services/notifier.ts](#servicesnotifierts)
61. [services/preventSleep.ts](#servicespreventsleepts)
62. [services/PromptSuggestion/promptSuggestion.ts](#servicespromptsuggestionpromptsuggestsionts)
63. [services/PromptSuggestion/speculation.ts](#servicespromptsuggestionspeculationts)
64. [services/rateLimitMocking.ts](#servicesratelimitmockingts)
65. [services/rateLimitMessages.ts](#servicesratelimitmessagests)
66. [services/SessionMemory/prompts.ts](#servicessessionmemorypromptsts)
67. [services/SessionMemory/sessionMemory.ts](#servicessessionmemorysessionmemoryts)
68. [services/SessionMemory/sessionMemoryUtils.ts](#servicessessionmemorysessionmemoryutilsts)
69. [services/tokenEstimation.ts](#servicestokenestimationts)
70. [services/vcr.ts](#servicesvcrts)
71. [services/voice.ts](#servicesvoicets)
72. [services/voiceKeyterms.ts](#servicesvoicekeytermsss)
73. [services/voiceStreamSTT.ts](#servicesvoicestreamsttts)
74. [context/QueuedMessageContext.tsx](#contextqueuedmessagecontexttsx)
75. [context/fpsMetrics.tsx](#contextfpsmetricstsx)
76. [context/mailbox.tsx](#contextmailboxtsx)
77. [context/modalContext.tsx](#contextmodalcontexttsx)
78. [context/notifications.tsx](#contextnotificationstsx)
79. [context/overlayContext.tsx](#contextoverlaycontexttsx)
80. [context/promptOverlayContext.tsx](#contextpromptoverlaycontexttsx)
81. [context/stats.tsx](#contextstatstsx)
82. [context/voice.tsx](#contextvoicetsx)
83. [screens/Doctor.tsx](#screensdoctortsx)
84. [screens/REPL.tsx](#screensrepltsx)
85. [screens/ResumeConversation.tsx](#screensresumeconversationtsx)

---

## bootstrap/state.ts

**Path:** `src/bootstrap/state.ts`

**Purpose:** The single global session state singleton for the entire Claude Code process. Acts as the authoritative source of truth for all per-session metrics, model configuration, telemetry handles, and feature flags. Designed as a strict leaf in the import DAG — imports nothing from `src/utils/` except via explicit safe indirection.

**Key Types Exported:**

```typescript
export type ChannelEntry =
  | { kind: 'plugin'; name: string; marketplace: string; dev?: boolean }
  | { kind: 'server'; name: string; dev?: boolean }

export type AttributedCounter = {
  add(value: number, additionalAttributes?: Attributes): void
}
```

**`State` Type (internal, not exported directly):** Contains ~80 fields including:
- `originalCwd: string` — resolved cwd at process start (NFC-normalized, symlinks resolved)
- `projectRoot: string` — stable identity root (set at startup, never changed by mid-session EnterWorktreeTool)
- `totalCostUSD: number`, `totalAPIDuration: number`, `totalAPIDurationWithoutRetries: number`
- `totalToolDuration: number`, `turnHookDurationMs: number`, `turnToolDurationMs: number`
- `totalLinesAdded: number`, `totalLinesRemoved: number`
- `cwd: string` — current working directory (mutable, changes with shell.ts setCwd)
- `modelUsage: { [modelName: string]: ModelUsage }` — per-model usage tracking
- `mainLoopModelOverride: ModelSetting | undefined`, `initialMainLoopModel: ModelSetting`
- `sessionId: SessionId` — UUID regenerated on `clearConversation`
- `parentSessionId: SessionId | undefined` — previous session (for lineage tracking)
- `isInteractive: boolean`, `kairosActive: boolean`, `strictToolResultPairing: boolean`
- `sdkAgentProgressSummariesEnabled: boolean`, `userMsgOptIn: boolean`
- `clientType: string` (default `'cli'`), `sessionSource: string | undefined`
- `meter: Meter | null`, `sessionCounter`, `locCounter`, `prCounter`, `commitCounter`, `costCounter`, `tokenCounter`, `codeEditToolDecisionCounter`, `activeTimeCounter` — OTel metrics
- `sessionId: SessionId` (randomUUID at init), `parentSessionId: SessionId | undefined`
- `loggerProvider: LoggerProvider | null`, `eventLogger: ReturnType<typeof logs.getLogger> | null`
- `meterProvider: MeterProvider | null`, `tracerProvider: BasicTracerProvider | null`
- `agentColorMap: Map<string, AgentColorName>`, `agentColorIndex: number`
- `lastAPIRequest`, `lastAPIRequestMessages`, `lastClassifierRequests`, `cachedClaudeMdContent`
- `inMemoryErrorLog: Array<{ error: string; timestamp: string }>`
- `inlinePlugins: string[]`, `chromeFlagOverride: boolean | undefined`
- `sessionBypassPermissionsMode: boolean`, `scheduledTasksEnabled: boolean`
- `sessionCronTasks: SessionCronTask[]`, `sessionCreatedTeams: Set<string>`
- `sessionTrustAccepted: boolean`, `sessionPersistenceDisabled: boolean`
- `hasExitedPlanMode: boolean`, `needsPlanModeExitAttachment: boolean`, `needsAutoModeExitAttachment: boolean`
- `initJsonSchema: Record<string, unknown> | null`, `registeredHooks: Partial<Record<HookEvent, RegisteredHookMatcher[]>> | null`
- `planSlugCache: Map<string, string>` — sessionId → wordSlug
- `teleportedSessionInfo: { isTeleported, hasLoggedFirstMessage, sessionId } | null`
- `invokedSkills: Map<string, { skillName, skillPath, content, invokedAt, agentId }>` — keyed by `"${agentId ?? ''}:${skillName}"`
- `slowOperations: Array<{ operation, durationMs, timestamp }>` — ant-only dev bar
- `sdkBetas: string[] | undefined`, `mainThreadAgentType: string | undefined`
- `isRemoteMode: boolean`, `directConnectServerUrl: string | undefined`
- `systemPromptSectionCache: Map<string, string | null>`, `lastEmittedDate: string | null`
- `additionalDirectoriesForClaudeMd: string[]`, `allowedChannels: ChannelEntry[]`, `hasDevChannels: boolean`
- `sessionProjectDir: string | null` — transcript directory override
- `promptCache1hAllowlist: string[] | null`, `promptCache1hEligible: boolean | null`
- `afkModeHeaderLatched: boolean | null`, `fastModeHeaderLatched: boolean | null`
- `cacheEditingHeaderLatched: boolean | null`, `thinkingClearLatched: boolean | null`
- `promptId: string | null`, `lastMainRequestId: string | undefined`
- `lastApiCompletionTimestamp: number | null`, `pendingPostCompaction: boolean`

**Exported Functions (getters/setters/mutators):**

```typescript
export function getSessionId(): SessionId
export function regenerateSessionId(options?: { setCurrentAsParent?: boolean }): SessionId
export function getParentSessionId(): SessionId | undefined
export function switchSession(sessionId: SessionId, projectDir?: string | null): void
export const onSessionSwitch: Signal<[id: SessionId]>['subscribe']
export function getSessionProjectDir(): string | null
export function getOriginalCwd(): string
export function getProjectRoot(): string
export function setOriginalCwd(cwd: string): void
export function setProjectRoot(cwd: string): void  // --worktree startup only
export function getCwdState(): string
export function setCwdState(cwd: string): void
export function getDirectConnectServerUrl(): string | undefined
export function setDirectConnectServerUrl(url: string): void
export function addToTotalDurationState(duration: number, durationWithoutRetries: number): void
export function resetTotalDurationStateAndCost_FOR_TESTS_ONLY(): void
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
export function getStatsStore(): { observe(name: string, value: number): void } | null
export function setStatsStore(store: { observe(name: string, value: number): void } | null): void
export function updateLastInteractionTime(immediate?: boolean): void
export function flushInteractionTime(): void
export function addToTotalLinesChanged(added: number, removed: number): void
export function getTotalLinesAdded(): number
export function getTotalLinesRemoved(): number
export function getTotalInputTokens(): number
export function getTotalOutputTokens(): number
export function getTotalCacheReadInputTokens(): number
export function getTotalCacheCreationInputTokens(): number
export function getTotalWebSearchRequests(): number
export function getTurnOutputTokens(): number
export function getCurrentTurnTokenBudget(): number | null
export function snapshotOutputTokensForTurn(budget: number | null): void
export function getBudgetContinuationCount(): number
export function incrementBudgetContinuationCount(): void
export function setHasUnknownModelCost(): void
export function hasUnknownModelCost(): boolean
export function getLastMainRequestId(): string | undefined
export function setLastMainRequestId(requestId: string): void
export function getLastApiCompletionTimestamp(): number | null
export function setLastApiCompletionTimestamp(timestamp: number): void
export function markPostCompaction(): void
export function consumePostCompaction(): boolean
export function getLastInteractionTime(): number
export function markScrollActivity(): void
export function getIsScrollDraining(): boolean
export function waitForScrollIdle(): Promise<void>
export function getModelUsage(): { [modelName: string]: ModelUsage }
export function getUsageForModel(model: string): ModelUsage | undefined
export function getMainLoopModelOverride(): ModelSetting | undefined
export function getInitialMainLoopModel(): ModelSetting
export function setMainLoopModelOverride(model: ModelSetting | undefined): void
// ... and many more setters for isInteractive, clientType, sessionSource, telemetry counters, etc.
```

**Key Logic:**
- `STATE` is a module-level singleton initialized via `getInitialState()` on import
- `updateLastInteractionTime(immediate?)`: deferred by default (batches keypresses into single Date.now() per Ink render); pass `immediate=true` for post-render useEffect callbacks
- `flushInteractionTime()`: called by Ink before each render cycle
- Scroll drain: `markScrollActivity()` sets a debounce flag (`scrollDraining`) for 150ms; background intervals call `getIsScrollDraining()` to yield; `waitForScrollIdle()` polls with 150ms intervals
- `switchSession()` atomically updates `sessionId + sessionProjectDir`; emits `sessionSwitched` signal
- `regenerateSessionId()` can optionally set current as parent (used for plan mode → implementation lineage)
- `markPostCompaction()` / `consumePostCompaction()`: one-shot latch, auto-resets after first consumption

**Configuration:**
- `SCROLL_DRAIN_IDLE_MS = 150`
- `RESERVOIR_SIZE` (histogram sampling) = 1024 (in stats.tsx)

**Dependencies:** `@anthropic-ai/sdk`, `@opentelemetry/api`, `@opentelemetry/sdk-*`, `src/utils/crypto.js`, `src/utils/signal.js`, `src/utils/settings/settingsCache.js`, `src/types/ids.js`

---

## coordinator/coordinatorMode.ts

**Path:** `src/coordinator/coordinatorMode.ts`

**Purpose:** Implements multi-worker "coordinator mode" where Claude Code orchestrates multiple parallel subagents. Provides the system prompt, user context injection, mode detection, and session-resume alignment logic.

**Exports:**

```typescript
export function isCoordinatorMode(): boolean
export function matchSessionMode(
  sessionMode: 'coordinator' | 'normal' | undefined
): string | undefined
export function getCoordinatorUserContext(
  mcpClients: ReadonlyArray<{ name: string }>,
  scratchpadDir?: string
): { [k: string]: string }
export function getCoordinatorSystemPrompt(): string
```

**Key Logic:**
- `isCoordinatorMode()`: reads `CLAUDE_CODE_COORDINATOR_MODE` env var; only active when `feature('COORDINATOR_MODE')` bundle flag is set
- `matchSessionMode()`: when resuming a session, aligns the current coordinator mode with the stored session mode. Flips `process.env.CLAUDE_CODE_COORDINATOR_MODE` in-place (since `isCoordinatorMode()` reads it live). Returns a user-visible warning message if mode was switched, `undefined` if no change needed. Logs `tengu_coordinator_mode_switched` analytics event
- `getCoordinatorUserContext()`: returns `{ workerToolsContext: string }` with worker tool list, MCP server names, and scratchpad directory (if gate `tengu_scratch` enabled). In `CLAUDE_CODE_SIMPLE` mode, limits worker tools to Bash/Read/Edit
- `getCoordinatorSystemPrompt()`: returns a multi-section system prompt (1500+ chars) describing coordinator role, available tools (Agent, SendMessage, TaskStop), task workflow phases (Research → Synthesis → Implementation → Verification), concurrency strategy, worker prompt writing guidelines, and full example session

**Internal Constants:**
```typescript
const INTERNAL_WORKER_TOOLS = new Set([
  TEAM_CREATE_TOOL_NAME,
  TEAM_DELETE_TOOL_NAME,
  SEND_MESSAGE_TOOL_NAME,
  SYNTHETIC_OUTPUT_TOOL_NAME,
])
```

**Configuration:**
- `COORDINATOR_MODE` bundle feature flag
- `CLAUDE_CODE_COORDINATOR_MODE` env var
- `CLAUDE_CODE_SIMPLE` env var — restricts worker tool set to Bash/Read/Edit
- GrowthBook gate `tengu_scratch` — enables scratchpad directory context

**Dependencies:** `bun:bundle`, `constants/tools.js`, `services/analytics/growthbook.js`, `services/analytics/index.js`, various tool name constants, `utils/envUtils.js`

---

## server/types.ts

**Path:** `src/server/types.ts`

**Purpose:** Shared type definitions for the Claude Code server (direct-connect mode). Provides the Zod validation schema for session creation responses.

**Exports:**

```typescript
export const connectResponseSchema: () => ZodObject<{
  session_id: ZodString
  ws_url: ZodString
  work_dir: ZodString.optional()
}>

export type ServerConfig = {
  port: number
  host?: string
  authToken?: string
}

export type SessionState = 'starting' | 'running' | 'detached' | 'stopping' | 'stopped'

export type SessionInfo = {
  sessionId: string
  state: SessionState
  wsUrl: string
  workDir?: string
  createdAt: number
  lastActivity: number
}

export type SessionIndexEntry = {
  sessionId: string
  createdAt: number
  workDir?: string
}

export type SessionIndex = Record<string, SessionIndexEntry>
```

**Key Logic:** `connectResponseSchema()` is a factory function (not a cached value) to allow Zod to be lazy-loaded. Used by `createDirectConnectSession.ts` to validate the `POST /sessions` response body.

**Dependencies:** `zod`

---

## server/createDirectConnectSession.ts

**Path:** `src/server/createDirectConnectSession.ts`

**Purpose:** Creates a session on a remote direct-connect Claude Code server. Posts to `/sessions`, validates response, returns a `DirectConnectConfig` ready for use by the REPL or headless runner.

**Exports:**

```typescript
export class DirectConnectError extends Error {
  constructor(message: string)
  name: 'DirectConnectError'
}

export async function createDirectConnectSession(opts: {
  serverUrl: string
  authToken?: string
  cwd: string
  dangerouslySkipPermissions?: boolean
}): Promise<{
  config: DirectConnectConfig
  workDir?: string
}>
```

**Key Logic:**
- POSTs `{ cwd, dangerously_skip_permissions? }` as JSON to `${serverUrl}/sessions`
- Sends `Authorization: Bearer ${authToken}` if provided
- Validates response JSON via `connectResponseSchema().safeParse()`
- Returns `{ config: { serverUrl, sessionId, wsUrl, authToken }, workDir }`
- Throws `DirectConnectError` on fetch failure, non-OK HTTP status, or response parse failure

**Dependencies:** `server/types.js`, `server/directConnectManager.js`, `utils/errors.js`, `utils/slowOperations.js`

---

## server/directConnectManager.ts

**Path:** `src/server/directConnectManager.ts`

**Purpose:** WebSocket client for communicating with a remote direct-connect Claude Code server. Handles message routing, permission request/response, interrupt signals, and connection lifecycle.

**Exports:**

```typescript
export type DirectConnectConfig = {
  serverUrl: string
  sessionId: string
  wsUrl: string
  authToken?: string
}

export type DirectConnectCallbacks = {
  onMessage: (message: SDKMessage) => void
  onPermissionRequest: (request: SDKControlPermissionRequest, requestId: string) => void
  onConnected?: () => void
  onDisconnected?: () => void
  onError?: (error: Error) => void
}

export class DirectConnectSessionManager {
  constructor(config: DirectConnectConfig, callbacks: DirectConnectCallbacks)
  connect(): void
  sendMessage(content: RemoteMessageContent): boolean
  respondToPermissionRequest(requestId: string, result: RemotePermissionResponse): void
  sendInterrupt(): void
  disconnect(): void
  isConnected(): boolean
}
```

**Key Logic:**
- `connect()`: opens WebSocket with `Authorization: Bearer` header (Bun WebSocket headers override); sets up `open`, `message`, `close`, `error` listeners
- Message parsing: splits NDJSON lines, parses each line, dispatches:
  - `control_request` with subtype `can_use_tool` → `onPermissionRequest()`
  - unrecognized control subtypes → auto-sends error response (prevents server hang)
  - Filtered out: `control_response`, `keep_alive`, `control_cancel_request`, `streamlined_text`, `streamlined_tool_use_summary`, system messages with subtype `post_turn_summary`
  - All others → `onMessage()`
- `sendMessage()`: formats as `SDKUserMessage` (`{ type: 'user', message: { role: 'user', content }, parent_tool_use_id: null, session_id: '' }`)
- `respondToPermissionRequest()`: formats as `SDKControlResponse` with `behavior` and either `updatedInput` (allow) or `message` (deny)
- `sendInterrupt()`: sends `{ type: 'control_request', request_id: crypto.randomUUID(), request: { subtype: 'interrupt' } }`

**Dependencies:** `entrypoints/agentSdkTypes.js`, `entrypoints/sdk/controlTypes.js`, `remote/RemoteSessionManager.js`, `utils/debug.js`, `utils/slowOperations.js`, `utils/teleport/api.js`

---

## services/analytics/config.ts

**Path:** `src/services/analytics/config.ts`

**Purpose:** Shared analytics configuration — common logic for disabling analytics across all backends.

**Exports:**

```typescript
export function isAnalyticsDisabled(): boolean
export function isFeedbackSurveyDisabled(): boolean
```

**Key Logic:**
- `isAnalyticsDisabled()`: returns `true` when `NODE_ENV === 'test'`, `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX`, `CLAUDE_CODE_USE_FOUNDRY` truthy, or `isTelemetryDisabled()` is true
- `isFeedbackSurveyDisabled()`: returns `true` when `NODE_ENV === 'test'` or `isTelemetryDisabled()` — does NOT gate on 3P providers (Bedrock/Vertex/Foundry) since the survey is local UI with no transcript data; enterprise captures via OTEL

**Dependencies:** `utils/envUtils.js`, `utils/privacyLevel.js`

---

## services/analytics/growthbook.ts

**Path:** `src/services/analytics/growthbook.ts`

**Purpose:** GrowthBook feature flag and dynamic config client. Provides cached and blocking access to feature gates, handles remote eval with disk persistence, manages refresh lifecycle, and exposes override APIs for development/testing.

**Key Types:**

```typescript
export type GrowthBookUserAttributes = {
  user_id?: string
  org_id?: string
  user_type?: string
  // ... other Statsig-compatible attributes
}
```

**Exports:**

```typescript
export function onGrowthBookRefresh(listener: () => void): () => void
export function hasGrowthBookEnvOverride(feature: string): boolean
export function getAllGrowthBookFeatures(): Record<string, unknown>
export function getGrowthBookConfigOverrides(): Record<string, unknown>
export function setGrowthBookConfigOverride(feature: string, value: unknown): void
export function clearGrowthBookConfigOverrides(): void
export function getApiBaseUrlHost(): string | undefined
export function initializeGrowthBook(): Promise<GrowthBook | null>
export function getFeatureValue_DEPRECATED<T>(feature: string, defaultValue: T): Promise<T>
export function getFeatureValue_CACHED_MAY_BE_STALE<T>(feature: string, defaultValue: T): T
export function getFeatureValue_CACHED_WITH_REFRESH<T>(feature: string, defaultValue: T): T  // deprecated
export function checkStatsigFeatureGate_CACHED_MAY_BE_STALE(gate: string): boolean
export function checkSecurityRestrictionGate(gate: string): Promise<boolean>
export function checkGate_CACHED_OR_BLOCKING(gate: string): Promise<boolean>
export function getDynamicConfig_CACHED_MAY_BE_STALE<T>(config: string, defaultValue: T): T
export function refreshGrowthBookAfterAuthChange(): void
```

**Key Logic:**
- **Initialization (`initializeGrowthBook()`):** Memoized singleton. Loads disk-cached features from `~/.claude/cachedGrowthBookFeatures`. Applies `CLAUDE_INTERNAL_FC_OVERRIDES` env var overrides (JSON). Connects to GrowthBook remote with 5000ms timeout, then sets up periodic refresh. Returns `null` when analytics disabled or in API key mode without user_id
- **Remote eval workaround:** GrowthBook's remote eval returns `{ value }` but client expects `{ defaultValue }`. The code transforms `{ value: V }` → `{ defaultValue: V }` before storing in `remoteEvalFeatureValues` Map. Synced to disk via `syncRemoteEvalToDisk()`
- **Caching tiers:**
  - `_DEPRECATED` functions: block on `initializeGrowthBook()` Promise
  - `_CACHED_MAY_BE_STALE`: returns synchronously from in-memory cache (may be stale after refresh)
  - `_CACHED_OR_BLOCKING`: awaits init, then returns cached value; used for security gates only
- **Security gates (`checkSecurityRestrictionGate()`):** awaits init, checks gate value, blocks if not initialized. Used for enterprise policy enforcement
- **Refresh listeners:** `onGrowthBookRefresh()` registers a listener called after each GrowthBook refresh cycle; returns unsubscribe function
- **Overrides:** `setGrowthBookConfigOverride()` / `clearGrowthBookConfigOverrides()` in-process override map; `CLAUDE_INTERNAL_FC_OVERRIDES` JSON env var for process-level overrides

**Configuration:**
- Disk cache: `~/.claude/cachedGrowthBookFeatures`
- Init timeout: 5000ms
- `CLAUDE_INTERNAL_FC_OVERRIDES` env var: JSON override map

**Dependencies:** `growthbook` SDK, `services/analytics/config.js`, `utils/auth.js`, `utils/config.js`

---

## services/analytics/metadata.ts

**Path:** `src/services/analytics/metadata.ts`

**Purpose:** Event metadata enrichment for analytics. Provides types and utilities for building structured `EventMetadata` objects with environment context, process metrics, and safe telemetry extraction from tool inputs.

**Constants:**
- `TOOL_INPUT_STRING_TRUNCATE_AT = 512` — strings longer than this get truncated
- `TOOL_INPUT_STRING_TRUNCATE_TO = 128` — truncated target length
- `TOOL_INPUT_MAX_JSON_CHARS = 4096` — JSON input cap before discarding
- `MAX_FILE_EXTENSION_LENGTH = 10` — max chars for file extensions

**Exports:**

```typescript
export type AnalyticsMetadata_I_VERIFIED_THIS_IS_NOT_CODE_OR_FILEPATHS = never  // marker type

export function sanitizeToolNameForAnalytics(toolName: string): never  // returns marker type

export function isToolDetailsLoggingEnabled(): boolean  // gated on OTEL_LOG_TOOL_DETAILS env

export function isAnalyticsToolDetailsLoggingEnabled(
  mcpServerType: string | undefined,
  mcpServerBaseUrl: string | undefined
): boolean

export function mcpToolDetailsForAnalytics(
  toolName: string,
  mcpServerType: string | undefined,
  mcpServerBaseUrl: string | undefined
): { mcpServerName?: never; mcpToolName?: never }

export function extractMcpToolDetails(
  toolName: string
): { serverName: string; mcpToolName: string } | undefined

export function extractSkillName(
  toolName: string,
  input: unknown
): never | undefined  // returns marker type or undefined

export function extractToolInputForTelemetry(input: unknown): string | undefined

export function getFileExtensionForAnalytics(filePath: string): never | undefined

export function getFileExtensionsFromBashCommand(
  command: string,
  simulatedSedEditFilePath?: string
): never | undefined

export type EnvContext = {
  userType: string
  isCI: boolean
  platform: string
  // ... other context fields
}

export type ProcessMetrics = {
  heapUsedMB: number
  heapTotalMB: number
  rssMB: number
  externalMB: number
}

export type EventMetadata = {
  // enriched event payload type
}

export type EnrichMetadataOptions = {
  includeProcessMetrics?: boolean
  // ...
}

export async function getEventMetadata(options?: EnrichMetadataOptions): Promise<EventMetadata>
export async function buildEnvContext(): Promise<EnvContext>  // memoized
```

**Key Logic:**
- `BUILTIN_MCP_SERVER_NAMES` set is gated behind `CHICAGO_MCP` feature flag — determines which MCP servers are considered "builtin"
- `extractToolInputForTelemetry()`: JSON-serializes input, truncates strings over `TOOL_INPUT_STRING_TRUNCATE_AT` to `TOOL_INPUT_STRING_TRUNCATE_TO`, caps total at `TOOL_INPUT_MAX_JSON_CHARS`
- `getFileExtensionsFromBashCommand()`: parses bash command to extract file extensions using regex patterns; handles `sed -i` specially via `simulatedSedEditFilePath`
- `buildEnvContext()` is memoized — called once per process and cached
- Agent identification classifies turns as: teammate (subagent of another), subagent (spawned by coordinator), standalone

**Dependencies:** `services/analytics/growthbook.js`, `utils/envUtils.js`, `utils/platform.js`

---

## services/analytics/index.ts

**Path:** `src/services/analytics/index.ts`

**Purpose:** The main analytics entry point — a no-dependency module that provides a queuing facade for all event logging. Events are queued until a sink is attached, preventing startup ordering issues.

**Design:** Explicitly has NO dependencies to avoid import cycles. Events are queued in `eventQueue` until `attachAnalyticsSink()` drains them via `queueMicrotask`.

**Exports:**

```typescript
export type AnalyticsMetadata_I_VERIFIED_THIS_IS_NOT_CODE_OR_FILEPATHS = never  // marker type
export type AnalyticsMetadata_I_VERIFIED_THIS_IS_PII_TAGGED = never  // PII-tagged marker type

export function stripProtoFields<V>(
  metadata: Record<string, V>
): Record<string, V>

export type AnalyticsSink = {
  logEvent: (eventName: string, metadata: LogEventMetadata) => void
  logEventAsync: (eventName: string, metadata: LogEventMetadata) => Promise<void>
}

export function attachAnalyticsSink(newSink: AnalyticsSink): void
export function logEvent(eventName: string, metadata: LogEventMetadata): void
export async function logEventAsync(eventName: string, metadata: LogEventMetadata): Promise<void>
export function _resetForTesting(): void
```

**Types (internal):**
```typescript
type LogEventMetadata = { [key: string]: boolean | number | undefined }
type QueuedEvent = { eventName: string; metadata: LogEventMetadata; async: boolean }
```

**Key Logic:**
- `attachAnalyticsSink()`: idempotent (no-op if sink already set). Drains queue via `queueMicrotask` to avoid blocking startup. For ant users (`USER_TYPE === 'ant'`), logs `analytics_sink_attached` with `queued_event_count`
- `stripProtoFields()`: removes keys starting with `_PROTO_` from event metadata (for non-1P destinations). Returns same reference if no `_PROTO_` keys present
- `_PROTO_*` keys route to PII-tagged BigQuery columns — stripped before Datadog but preserved for firstPartyEventLoggingExporter
- Metadata type is intentionally restricted to `boolean | number | undefined` — no strings unless explicitly cast with the marker type

---

## services/analytics/sink.ts

**Path:** `src/services/analytics/sink.ts`

**Purpose:** The analytics sink implementation that routes events to Datadog and first-party event logging backends. Handles sampling, metadata enrichment, and per-sink kill switches.

**Exports:**

```typescript
export function createAnalyticsSink(options?: {
  isInteractive?: boolean
}): AnalyticsSink
```

**Key Logic:**
- Routes events to two sinks: Datadog (via `logDatadogEvent`) and first-party event logging (via `log1PEvent`)
- Checks `isSinkKilled('datadog')` and `isSinkKilled('firstParty')` before dispatching to each sink
- Applies sampling based on `tengu_event_sampling_config` dynamic config — adds `sample_rate` to metadata when sampled
- Strips `_PROTO_*` keys before Datadog fanout (`stripProtoFields`)
- Metadata is enriched with `getEventMetadata()` before dispatch

---

## services/analytics/sinkKillswitch.ts

**Path:** `src/services/analytics/sinkKillswitch.ts`

**Purpose:** Per-sink analytics kill switch, controlled by a GrowthBook JSON config.

**Exports:**

```typescript
export type SinkName = 'datadog' | 'firstParty'

export function isSinkKilled(sink: SinkName): boolean
```

**Key Logic:**
- Config name: `tengu_frond_boric` (mangled/obfuscated name)
- Shape: `{ datadog?: boolean, firstParty?: boolean }` — `true` stops dispatch to that sink
- Default `{}` (nothing killed). Fail-open: missing/malformed config = sink stays on
- Must NOT be called from `isGrowthBookEnabled()` — would cause recursion; call at per-event dispatch sites instead

**Dependencies:** `services/analytics/growthbook.js`

---

## services/analytics/datadog.ts

**Path:** `src/services/analytics/datadog.ts`

**Purpose:** Datadog metrics and event logging integration for Claude Code analytics.

**Key Logic:**
- Sends events via `@datadog/datadog-ci` or direct HTTP to Datadog API
- Disabled when `isAnalyticsDisabled()` returns true
- Event namespace: `tengu_*` prefix for all Claude Code events
- Tags include version, platform, user type, session metadata

---

## services/analytics/firstPartyEventLogger.ts

**Path:** `src/services/analytics/firstPartyEventLogger.ts`

**Purpose:** First-party event logging integration — routes events to the internal event logging system.

**Key Logic:**
- Gated by `is1PEventLoggingEnabled()` which checks GrowthBook feature `tengu_fpel` and `isAnalyticsDisabled()`
- Enriches events with `EventMetadata` from `getEventMetadata()`
- Handles proto field hoisting from `_PROTO_*` keys

---

## services/analytics/firstPartyEventLoggingExporter.ts

**Path:** `src/services/analytics/firstPartyEventLoggingExporter.ts`

**Purpose:** OpenTelemetry log exporter that routes logs to the first-party event logging pipeline.

**Key Logic:**
- Implements OTel `LogRecordExporter` interface
- Hoists `_PROTO_*` keys to top-level proto fields in the BQ destination
- Calls `stripProtoFields()` after hoisting as defensive cleanup
- Only sends to first-party pipeline (not Datadog)

---

## services/api/bootstrap.ts

**Path:** `src/services/api/bootstrap.ts`

**Purpose:** Bootstraps the API client with session-specific configuration at startup. Wires together auth, proxy, and telemetry settings.

**Key Logic:**
- Configures `ANTHROPIC_API_URL` base URL, auth headers, proxy settings
- Calls `initializeGrowthBook()` early in startup
- Sets up OTel span management
- Handles `CLAUDE_CODE_SKIP_BEDROCK_TLS` for Bedrock TLS verification skip

---

## services/api/client.ts

**Path:** `src/services/api/client.ts`

**Purpose:** Provides the configured SDK client instance and helper utilities for making API calls.

**Exports:**

```typescript
export function getClient(): Anthropic
export function getClientForModel(model: string): Anthropic
```

**Key Logic:**
- Client is created lazily and cached
- Applies base URL overrides from `ANTHROPIC_BASE_URL` env
- Configures mTLS via `getMtlsConfig()`
- Routes through proxy if `HTTPS_PROXY` / `HTTP_PROXY` set

---

## services/api/claude.ts

**Path:** `src/services/api/claude.ts`

**Purpose:** Main API call layer for Claude completions. Handles prompt caching, extra body parameters, task budget configuration, metadata, and user message formatting.

**Exports:**

```typescript
export function getExtraBodyParams(betaHeaders?: string[]): JsonObject
export function getPromptCachingEnabled(model: string): boolean
export function getCacheControl(opts: {
  scope?: string
  querySource?: QuerySource
}): { type: 'ephemeral' | 'persistent'; ttl?: number; scope?: string }
export function configureTaskBudgetParams(
  taskBudget: number,
  outputConfig: OutputConfig,
  betas: string[]
): void
export function getAPIMetadata(): { user_id: string }
export async function verifyApiKey(
  apiKey: string,
  isNonInteractiveSession: boolean
): Promise<boolean>
export function userMessageToMessageParam(
  message: UserMessage,
  addCache: boolean,
  enablePromptCaching: boolean,
  querySource?: QuerySource
): MessageParam
```

**Internal functions (not exported):**
- `configureEffortParams()`: sets thinking budget based on effort level
- `should1hCacheTTL()`: checks if 1h TTL is applicable for current user/model

**Key Logic:**
- **Prompt caching:** `getPromptCachingEnabled()` checks model allowlist. `getCacheControl()` returns `{ type: 'ephemeral' }` normally, `{ type: 'ephemeral', ttl: 3600 }` when 1h TTL gate passes
- **1h TTL gate:** `should1hCacheTTL()` checks `tengu_prompt_cache_1h_config` GrowthBook allowlist (session-stable, latched in `STATE.promptCache1hAllowlist`) and `STATE.promptCache1hEligible` (also latched to prevent mid-session overage flips)
- **Anti-distillation:** `tengu_anti_distill_fake_tool_injection` GrowthBook gate — injects fake tools into API calls as a training data quality signal
- **Extra body params:** `getExtraBodyParams()` assembles beta headers, model-specific params, and context management config

**Dependencies:** `@anthropic-ai/sdk`, `bootstrap/state.js`, `services/analytics/growthbook.js`, `services/compact/apiMicrocompact.js`

---

## services/api/dumpPrompts.ts

**Path:** `src/services/api/dumpPrompts.ts`

**Purpose:** Debug utility for ant users — dumps API request/response JSONL logs to disk for inspection. Used for prompt debugging and sharing bug reports.

**Exports:**

```typescript
export function getLastApiRequests(): Array<{ timestamp: string; request: unknown }>
export function clearApiRequestCache(): void
export function clearDumpState(agentIdOrSessionId: string): void
export function clearAllDumpState(): void
export function addApiRequestToCache(requestData: unknown): void
export function getDumpPromptsPath(agentIdOrSessionId?: string): string
export function createDumpPromptsFetch(
  agentIdOrSessionId: string
): ClientOptions['fetch']
```

**Key Logic:**
- Ant-only (no-op for non-ant users)
- `MAX_CACHED_REQUESTS = 5` — in-memory ring buffer of recent requests
- Deferred writes via `setImmediate` to avoid blocking the request path
- JSONL format with records of types: `init`, `system_update`, `message`, `response`
- Per-session state tracking with fingerprint-based change detection (avoids re-writing unchanged system prompts)
- Path: `~/.claude/dump-prompts/<agentIdOrSessionId>.jsonl`

---

## services/api/emptyUsage.ts

**Path:** `src/services/api/emptyUsage.ts`

**Purpose:** Provides a zero-value `Usage` object for cases where usage data is unavailable.

**Exports:**

```typescript
export const EMPTY_USAGE: Usage
export type NonNullableUsage = {
  input_tokens: number
  output_tokens: number
  cache_read_input_tokens: number
  cache_creation_input_tokens: number
}
```

---

## services/api/errorUtils.ts

**Path:** `src/services/api/errorUtils.ts`

**Purpose:** Utilities for classifying and handling API errors.

**Exports:**

```typescript
export function isRateLimitError(error: unknown): boolean
export function isOverloadedError(error: unknown): boolean
export function isAuthError(error: unknown): boolean
export function isConnectionError(error: unknown): boolean
export function getRetryAfterMs(error: unknown): number | undefined
```

---

## services/api/errors.ts

**Path:** `src/services/api/errors.ts`

**Purpose:** Defines all API error message constants and classification functions for user-facing error handling.

**Exports:**

```typescript
export const API_ERROR_MESSAGE_PREFIX = 'API Error'
export function startsWithApiErrorPrefix(text: string): boolean

export const PROMPT_TOO_LONG_ERROR_MESSAGE: string
export function isPromptTooLongMessage(msg: string): boolean
export function parsePromptTooLongTokenCounts(
  rawMessage: string
): { actualTokens: number; limitTokens: number } | null
export function getPromptTooLongTokenGap(msg: string): number | undefined

export function isMediaSizeError(raw: unknown): boolean
export function isMediaSizeErrorMessage(msg: string): boolean

export const CREDIT_BALANCE_TOO_LOW_ERROR_MESSAGE: string
export const INVALID_API_KEY_ERROR_MESSAGE: string
export const INVALID_API_KEY_ERROR_MESSAGE_EXTERNAL: string
export const TOKEN_REVOKED_ERROR_MESSAGE: string
export const CCR_AUTH_ERROR_MESSAGE: string
export const REPEATED_529_ERROR_MESSAGE: string
export const CUSTOM_OFF_SWITCH_MESSAGE: string
export const API_TIMEOUT_ERROR_MESSAGE: string
export const OAUTH_ORG_NOT_ALLOWED_ERROR_MESSAGE: string

export function getPdfTooLargeErrorMessage(): string
export function getPdfPasswordProtectedErrorMessage(): string
export function getPdfInvalidErrorMessage(): string
export function getImageTooLargeErrorMessage(): string
export function getRequestTooLargeErrorMessage(): string
export function getTokenRevokedErrorMessage(): string
export function getOauthOrgNotAllowedErrorMessage(): string
```

---

## services/api/filesApi.ts

**Path:** `src/services/api/filesApi.ts`

**Purpose:** Files API client — downloads, uploads, lists, and manages files in Files API (beta).

**Constants:**
- `FILES_API_BETA_HEADER = 'files-api-2025-04-14,oauth-2025-04-20'`
- `MAX_FILE_SIZE_BYTES = 500 * 1024 * 1024` (500MB)
- `DEFAULT_CONCURRENCY = 5`
- `MAX_RETRIES = 3`
- `BASE_DELAY_MS = 500`

**Exports:**

```typescript
export type File = {
  fileId: string
  relativePath: string
  mimeType?: string
}

export type FilesApiConfig = {
  apiKey?: string
  baseUrl?: string
  sessionId?: string
}

export type DownloadResult = {
  fileId: string
  relativePath: string
  success: boolean
  error?: string
  savedPath?: string
}

export type UploadResult = {
  fileId: string
  relativePath: string
  success: boolean
  error?: string
  remoteFileId?: string
}

export type FileMetadata = {
  id: string
  filename: string
  created_at: number
  purpose: string
  size: number
}

export class UploadNonRetriableError extends Error {}

export function parseFileSpecs(fileSpecs: string[]): File[]
export async function downloadFile(fileId: string, config: FilesApiConfig): Promise<Buffer>
export function buildDownloadPath(
  basePath: string,
  sessionId: string,
  relativePath: string
): string | null
export async function downloadAndSaveFile(
  attachment: File,
  config: FilesApiConfig
): Promise<DownloadResult>
export async function downloadSessionFiles(
  files: File[],
  config: FilesApiConfig,
  concurrency?: number
): Promise<DownloadResult[]>
export async function uploadFile(
  filePath: string,
  relativePath: string,
  config: FilesApiConfig,
  opts?: { retries?: number }
): Promise<UploadResult>
export async function uploadSessionFiles(
  files: File[],
  config: FilesApiConfig,
  concurrency?: number
): Promise<UploadResult[]>
export async function listFilesCreatedAfter(
  afterCreatedAt: number,
  config: FilesApiConfig
): Promise<FileMetadata[]>
```

**Key Logic:**
- `buildDownloadPath()`: path traversal guard — if `relativePath` contains `..` components or resolves outside `basePath/sessionId`, returns `null`
- Download/upload use exponential backoff: `BASE_DELAY_MS * 2^attempt` with jitter
- `uploadSessionFiles()` / `downloadSessionFiles()`: parallel with configurable concurrency (default 5)
- `listFilesCreatedAfter()`: paginated using cursor, collects all pages

---

## services/api/firstTokenDate.ts

**Path:** `src/services/api/firstTokenDate.ts`

**Purpose:** Fetches and stores the date when the user first made a Claude Code API call.

**Exports:**

```typescript
export async function fetchAndStoreClaudeCodeFirstTokenDate(): Promise<void>
```

**Key Logic:**
- Fetches `/api/organization/claude_code_first_token_date`
- Stores in `claudeCodeFirstTokenDate` config field
- Idempotent — no-ops if already stored

---

## services/api/grove.ts

**Path:** `src/services/api/grove.ts`

**Purpose:** Grove is a consumer Terms/Privacy Policy notification feature. Manages fetching, caching, and determining whether to show the Grove notice to users.

**Constants:**
- `GROVE_CACHE_EXPIRATION_MS = 24 * 60 * 60 * 1000` (24 hours)

**Exports:**

```typescript
export type AccountSettings = {
  groveEnabled: boolean
  groveNoticeViewed: boolean
  // ...
}

export type GroveConfig = {
  enabled: boolean
  forceShow: boolean
  // ...
}

export type ApiResult<T> = { success: true; data: T } | { success: false; error: string }

export async function getGroveSettings(): Promise<ApiResult<AccountSettings>>  // memoized 24h
export async function markGroveNoticeViewed(): Promise<void>
export async function updateGroveSettings(groveEnabled: boolean): Promise<void>
export async function isQualifiedForGrove(): Promise<boolean>
export async function getGroveNoticeConfig(): Promise<ApiResult<GroveConfig>>  // memoized 24h
export function calculateShouldShowGrove(
  settingsResult: ApiResult<AccountSettings>,
  configResult: ApiResult<GroveConfig>,
  showIfAlreadyViewed: boolean
): boolean
export async function checkGroveForNonInteractive(): Promise<void>
```

**Key Logic:**
- Cache-first with background refresh: returns cached data immediately, refreshes in background after expiry
- `calculateShouldShowGrove()`: checks config enabled, settings not viewed, and user qualification
- `checkGroveForNonInteractive()`: called in non-interactive mode to log grove status without showing UI

---

## services/api/logging.ts

**Path:** `src/services/api/logging.ts`

**Purpose:** API query/success/error logging with gateway detection and OTel span management.

**Exports:**

```typescript
export type GlobalCacheStrategy = 'tool_based' | 'system_prompt' | 'none'

export function logAPIQuery(opts: {
  model: string
  querySource: QuerySource
  // ... other fields
}): void

export function logAPIError(opts: {
  error: unknown
  model: string
  querySource: QuerySource
  // ... other fields
}): void

export function logAPISuccessAndDuration(opts: {
  model: string
  usage: Usage
  querySource: QuerySource
  durationMs: number
  // ... other fields
}): void
```

**Key Logic:**
- Gateway detection: identifies litellm, helicone, portkey, cloudflare-ai-gateway, kong, braintrust, databricks from `ANTHROPIC_BASE_URL`
- Events: `tengu_api_query`, `tengu_api_error`, `tengu_api_success`
- OTel spans created/ended around API calls
- Teleport session tracking: logs extra fields when session is teleported
- Re-exports `EMPTY_USAGE` and `NonNullableUsage`

---

## services/api/metricsOptOut.ts

**Path:** `src/services/api/metricsOptOut.ts`

**Purpose:** Checks whether metrics collection is enabled for the current organization. Implements two-tier caching.

**Constants:**
- `CACHE_TTL_MS = 60 * 60 * 1000` (1 hour in-memory)
- `DISK_CACHE_TTL_MS = 24 * 60 * 60 * 1000` (24 hours on disk)
- Endpoint: `api/claude_code/organizations/metrics_enabled`

**Exports:**

```typescript
export async function checkMetricsEnabled(): Promise<MetricsStatus>
export function _clearMetricsEnabledCacheForTesting(): void
```

**Key Logic:**
- Two-tier cache: in-memory (1h TTL) → disk (24h TTL) → network
- Requires `profile` OAuth scope; returns `enabled: true` if unauthenticated or scope missing
- `MetricsStatus`: `{ enabled: boolean; source: 'cache' | 'network' | 'default' }`

---

## services/api/overageCreditGrant.ts

**Path:** `src/services/api/overageCreditGrant.ts`

**Purpose:** Manages overage credit grant information for subscribed users who exceed their plan limits.

**Constants:**
- `CACHE_TTL_MS = 60 * 60 * 1000` (1 hour)

**Exports:**

```typescript
export type OverageCreditGrantInfo = {
  hasGrant: boolean
  amount?: number
  currency?: string
  expiresAt?: string
}

export type OverageCreditGrantCacheEntry = {
  data: OverageCreditGrantInfo
  fetchedAt: number
  orgId: string
}

export function getCachedOverageCreditGrant(): OverageCreditGrantInfo | null
export function invalidateOverageCreditGrantCache(): void
export async function refreshOverageCreditGrantCache(): Promise<void>
export function formatGrantAmount(info: OverageCreditGrantInfo): string | null
```

**Key Logic:**
- Per-org cache in `overageCreditGrantCache` Map (keyed by org ID)
- `formatGrantAmount()`: formats amount as currency string (e.g., "$5.00") or `null` if no grant

---

## services/api/promptCacheBreakDetection.ts

**Path:** `src/services/api/promptCacheBreakDetection.ts`

**Purpose:** Detects unexpected prompt cache breaks that indicate server-side cache eviction. Writes diff files to disk for debugging.

**Constants:**
- `CACHE_TTL_1HOUR_MS = 3_600_000` (1 hour)
- `MIN_CACHE_MISS_TOKENS = 2_000` — minimum to consider a break significant
- 95% threshold — cache reads must drop to ≤5% of expected to count as a break
- `MAX_TRACKED_SOURCES = 10`

**Exports:**

```typescript
export type PromptStateSnapshot = {
  messages: MessageParam[]
  systemPrompt: string
  tools: unknown[]
  timestamp: number
  querySource: QuerySource
}

export const CACHE_TTL_1HOUR_MS: number

export function recordPromptState(snapshot: PromptStateSnapshot): void
export async function checkResponseForCacheBreak(
  querySource: QuerySource,
  cacheReadTokens: number,
  cacheCreationTokens: number,
  messages: MessageParam[],
  agentId?: string,
  requestId?: string
): Promise<void>
export function notifyCacheDeletion(querySource: QuerySource, agentId?: string): void
export function notifyCompaction(querySource: QuerySource, agentId?: string): void
export function cleanupAgentTracking(agentId: string): void
export function resetPromptCacheBreakDetection(): void
```

**Key Logic:**
- 2-phase detection: `recordPromptState()` before call, `checkResponseForCacheBreak()` after
- Per-source tracking Map: keyed by `querySource` (or `agent:${agentId}`)
- Tracked source prefixes: `repl_main_thread`, `sdk`, `agent:custom`, `agent:default`, `agent:builtin`
- Writes diff files to `~/.claude/tmp/cache-break-*.diff` when a break is detected
- Events: `tengu_prompt_cache_break`
- `notifyCacheDeletion()` / `notifyCompaction()`: suppress false positives after intentional cache clearing

---

## services/api/referral.ts

**Path:** `src/services/api/referral.ts`

**Purpose:** Manages referral program eligibility, redemptions, and guest passes for Claude Code subscribers.

**Constants:**
- `CACHE_EXPIRATION_MS = 24 * 60 * 60 * 1000` (24 hours)

**Exports:**

```typescript
export async function fetchReferralEligibility(
  campaign?: string
): Promise<ReferralEligibilityResponse>
export async function fetchReferralRedemptions(
  campaign?: string
): Promise<ReferralRedemptionsResponse>
export function checkCachedPassesEligibility(): {
  eligible: boolean
  needsRefresh: boolean
  hasCache: boolean
}
export function formatCreditAmount(reward: ReferrerReward): string
export function getCachedReferrerReward(): ReferrerRewardInfo | null
export function getCachedRemainingPasses(): number | null
export async function fetchAndStorePassesEligibility(): Promise<ReferralEligibilityResponse | null>
export async function getCachedOrFetchPassesEligibility(): Promise<ReferralEligibilityResponse | null>
export async function prefetchPassesEligibility(): Promise<void>
```

**Key Logic:**
- Max-subscription only — returns `null` / ineligible for non-max subscribers
- In-flight deduplication: multiple calls to `getCachedOrFetchPassesEligibility()` share one pending Promise
- 24h cache TTL

---

## services/api/sessionIngress.ts

**Path:** `src/services/api/sessionIngress.ts`

**Purpose:** Manages session log ingress — append-log with optimistic concurrency for multi-writer scenarios (e.g., continued sessions from different machines).

**Constants:**
- `MAX_RETRIES = 10`
- `BASE_DELAY_MS = 500` — exponential backoff base

**Exports:**

```typescript
export async function appendSessionLog(
  sessionId: string,
  entry: SessionLogEntry,
  url: string
): Promise<boolean>
export async function getSessionLogs(
  sessionId: string,
  url: string
): Promise<Entry[] | null>
export async function getSessionLogsViaOAuth(
  sessionId: string,
  accessToken: string,
  orgUUID: string
): Promise<Entry[] | null>
export async function getTeleportEvents(
  sessionId: string,
  accessToken: string,
  orgUUID: string
): Promise<Entry[] | null>
export function clearSession(sessionId: string): void
export function clearAllSessions(): void
```

**Key Logic:**
- Optimistic concurrency: `Last-Uuid` header on append; 409 response adopts server's last UUID
- Sequential wrappers per session prevent out-of-order appends
- `getTeleportEvents()`: paginated (max 100 pages, 1000 events/page)
- `clearSession()` / `clearAllSessions()`: clears in-memory sequential wrapper state

---

## services/api/ultrareviewQuota.ts

**Path:** `src/services/api/ultrareviewQuota.ts`

**Purpose:** Fetches ultrareview (deep code review) quota information for subscribed users.

**Exports:**

```typescript
export type UltrareviewQuotaResponse = {
  used: number
  limit: number
  resetsAt: string
}

export async function fetchUltrareviewQuota(): Promise<UltrareviewQuotaResponse | null>
```

**Key Logic:**
- Endpoint: `/v1/ultrareview/quota`
- Subscriber-only; returns `null` for non-subscribers or on error
- 5 second timeout

---

## services/api/usage.ts

**Path:** `src/services/api/usage.ts`

**Purpose:** Fetches usage statistics for the current user/organization from the Claude Code API.

**Exports:**

```typescript
export async function fetchUsage(): Promise<UsageStats | null>
export type UsageStats = {
  // usage breakdown fields
}
```

---

## services/api/withRetry.ts

**Path:** `src/services/api/withRetry.ts`

**Purpose:** Generic retry wrapper for API calls with exponential backoff.

**Exports:**

```typescript
export async function withRetry<T>(
  fn: () => Promise<T>,
  opts?: {
    maxRetries?: number
    baseDelayMs?: number
    shouldRetry?: (error: unknown) => boolean
  }
): Promise<T>
```

---

## services/AgentSummary/agentSummary.ts

**Path:** `src/services/AgentSummary/agentSummary.ts`

**Purpose:** Periodic background summarization of agent conversations to compress context while preserving key information.

**Key Logic:**
- Runs on a 30-second background timer
- Generates summaries using the main Claude model
- Compressed summaries are injected back as system messages
- Used by subagents and teammates to manage long-running conversations

---

## services/autoDream/autoDream.ts

**Path:** `src/services/autoDream/autoDream.ts`

**Purpose:** Background memory consolidation system. Periodically scans session transcripts and uses a forked agent to consolidate learnings into persistent memory files.

**Constants:**
- `SESSION_SCAN_INTERVAL_MS = 10 * 60 * 1000` (10 minutes)
- `DEFAULTS = { minHours: 24, minSessions: 5 }` — minimum time and sessions before consolidation

**Exports:**

```typescript
export function initAutoDream(): () => void  // returns stop/cleanup function
```

**Key Logic:**
- Gate order: time check (minHours) → session count check (minSessions) → consolidation lock
- GrowthBook config `tengu_onyx_plover` controls `{ minHours, minSessions, enabled }`
- `initAutoDream()` registers as a post-sampling hook; returns cleanup function
- Uses `SESSION_SCAN_INTERVAL_MS` for polling
- Spawns a forked agent using `buildConsolidationPrompt()`

---

## services/autoDream/config.ts

**Path:** `src/services/autoDream/config.ts`

**Purpose:** Configuration gate for the autoDream memory consolidation feature.

**Exports:**

```typescript
export function isAutoDreamEnabled(): boolean
```

**Key Logic:** User setting takes precedence over GrowthBook gate `tengu_onyx_plover`. Checks `userSettings.autoDream` first, then GrowthBook.

---

## services/autoDream/consolidationLock.ts

**Path:** `src/services/autoDream/consolidationLock.ts`

**Purpose:** File-based mutex lock for the memory consolidation process to prevent concurrent consolidations across sessions/processes.

**Constants:**
- `LOCK_FILE = '.consolidate-lock'` — in memory directory
- `HOLDER_STALE_MS = 60 * 60 * 1000` (1 hour) — stale lock threshold

**Exports:**

```typescript
export async function readLastConsolidatedAt(): Promise<number>
export async function tryAcquireConsolidationLock(): Promise<number | null>
export async function rollbackConsolidationLock(priorMtime: number): Promise<void>
export async function listSessionsTouchedSince(sinceMs: number): Promise<string[]>
export async function recordConsolidation(): Promise<void>
```

**Key Logic:**
- Lock file mtime = `lastConsolidatedAt` timestamp (dual-purpose: both locking and timestamp)
- PID-based ownership — stale locks (PID dead or >1h old) are overwritten
- `tryAcquireConsolidationLock()`: returns prior mtime on success, `null` if already held
- `rollbackConsolidationLock()`: restores mtime to `priorMtime` on consolidation failure

---

## services/autoDream/consolidationPrompt.ts

**Path:** `src/services/autoDream/consolidationPrompt.ts`

**Purpose:** Builds the system prompt for the memory consolidation agent.

**Exports:**

```typescript
export function buildConsolidationPrompt(
  memoryRoot: string,
  transcriptDir: string,
  extra: string
): string
```

**Key Logic:** Returns a 4-phase prompt:
1. Orient — read existing memory files to understand current state
2. Gather recent signal — read session transcripts since last consolidation
3. Consolidate — merge new learnings into memory files
4. Prune and index — remove stale entries, update index file

---

## services/awaySummary.ts

**Path:** `src/services/awaySummary.ts`

**Purpose:** Generates "away summaries" — brief catch-up summaries shown when the user returns to a long-running session after being away.

**Key Logic:**
- Triggered when `lastInteractionTime` gap exceeds threshold
- Uses Claude to generate a brief (1-3 sentence) summary of what happened while away
- Displayed as a system message above the prompt

---

## services/claudeAiLimits.ts

**Path:** `src/services/claudeAiLimits.ts`

**Purpose:** Fetches and manages rate limit information for Claude.ai-authenticated users.

**Exports:**

```typescript
export async function fetchClaudeAiLimits(): Promise<ClaudeAiLimits | null>
export type ClaudeAiLimits = {
  // rate limit fields
}
```

---

## services/claudeAiLimitsHook.ts

**Path:** `src/services/claudeAiLimitsHook.ts`

**Purpose:** React hook for accessing Claude.ai rate limit data with automatic refresh.

**Exports:**

```typescript
export function useClaudeAiLimits(): ClaudeAiLimits | null
```

---

## services/compact/apiMicrocompact.ts

**Path:** `src/services/compact/apiMicrocompact.ts`

**Purpose:** API-native context management strategies using server-side `cache_edits` feature. Configures context window editing without full client-side rewriting.

**Constants:**
- `DEFAULT_MAX_INPUT_TOKENS = 180_000`
- `DEFAULT_TARGET_INPUT_TOKENS = 40_000`

**Type: `ContextEditStrategy`:**
```typescript
export type ContextEditStrategy =
  | {
      type: 'clear_tool_uses_20250919'
      trigger?: { type: 'input_tokens'; value: number }
      keep?: { type: 'tool_uses'; value: number }
      clear_tool_inputs?: boolean | string[]
      exclude_tools?: string[]
      clear_at_least?: { type: 'input_tokens'; value: number }
    }
  | {
      type: 'clear_thinking_20251015'
      keep: { type: 'thinking_turns'; value: number } | 'all'
    }

export type ContextManagementConfig = {
  edits: ContextEditStrategy[]
}
```

**Exports:**

```typescript
export function getAPIContextManagement(options?: {
  hasThinking?: boolean
  isRedactThinkingActive?: boolean
  clearAllThinking?: boolean
}): ContextManagementConfig | undefined
```

**Key Logic:**
- Tool clearing strategies are ant-only, gated by `USE_API_CLEAR_TOOL_RESULTS` and `USE_API_CLEAR_TOOL_USES` env vars
- `TOOLS_CLEARABLE_RESULTS`: shell tools, Glob, Grep, FileRead, WebFetch, WebSearch
- `TOOLS_CLEARABLE_USES`: FileEdit, FileWrite, NotebookEdit
- Thinking clearing: when `hasThinking && !isRedactThinkingActive`, adds `clear_thinking_20251015`
- When `clearAllThinking` (>1h idle = confirmed cache miss): keeps only last 1 thinking turn

---

## services/compact/autoCompact.ts

**Path:** `src/services/compact/autoCompact.ts`

**Purpose:** Automatic context compaction — triggers full conversation summarization when context window usage exceeds threshold.

**Key Logic:**
- Monitors token usage against configurable threshold (default 90% of context window)
- When triggered, calls `compact()` to summarize the conversation
- Posts a `CompactBoundaryMessage` in the conversation to mark compaction point
- Resets token tracking after compaction

---

## services/compact/compact.ts

**Path:** `src/services/compact/compact.ts`

**Purpose:** Full conversation compaction — replaces conversation history with an LLM-generated summary.

**Key Logic:**
- Uses the detailed analysis instruction prompts from `prompt.ts`
- Optionally performs partial compaction (keeps recent messages)
- The `NO_TOOLS_PREAMBLE` constant is a critical instruction preventing the compaction model from calling tools during summarization
- Writes compact summary as a synthetic `<compact_summary>` tagged message

---

## services/compact/compactWarningHook.ts

**Path:** `src/services/compact/compactWarningHook.ts`

**Purpose:** React hook for accessing the compact warning suppression state.

**Exports:**

```typescript
export function useCompactWarningSuppression(): boolean
```

---

## services/compact/compactWarningState.ts

**Path:** `src/services/compact/compactWarningState.ts`

**Purpose:** Store and actions for suppressing the "compact recommended" warning after microcompact runs.

**Exports:**

```typescript
export const compactWarningStore: Store<boolean>
export function suppressCompactWarning(): void
export function clearCompactWarningSuppression(): void
```

---

## services/compact/grouping.ts

**Path:** `src/services/compact/grouping.ts`

**Purpose:** Groups conversation messages by API round (each assistant message with its preceding user message).

**Exports:**

```typescript
export function groupMessagesByApiRound(messages: Message[]): Message[][]
```

**Key Logic:** Groups by assistant `message.id` boundary — each group contains one API round (user + assistant + tool results).

---

## services/compact/microCompact.ts

**Path:** `src/services/compact/microCompact.ts`

**Purpose:** Microcompaction — lightweight context reduction by clearing tool result content without full conversation summarization. Two paths: cached microcompact (via API `cache_edits`) and time-based microcompact (direct content mutation when cache is cold).

**Constants (exported):**
```typescript
export const TIME_BASED_MC_CLEARED_MESSAGE = '[Old tool result content cleared]'
```

**Compactable tool sets:**
```typescript
const COMPACTABLE_TOOLS = new Set([
  FILE_READ_TOOL_NAME, SHELL_TOOL_NAMES..., GREP_TOOL_NAME, GLOB_TOOL_NAME,
  WEB_SEARCH_TOOL_NAME, WEB_FETCH_TOOL_NAME, FILE_EDIT_TOOL_NAME, FILE_WRITE_TOOL_NAME
])
```

**Exports:**

```typescript
export function consumePendingCacheEdits():
  import('./cachedMicrocompact.js').CacheEditsBlock | null

export function getPinnedCacheEdits():
  import('./cachedMicrocompact.js').PinnedCacheEdits[]

export function pinCacheEdits(
  userMessageIndex: number,
  block: import('./cachedMicrocompact.js').CacheEditsBlock
): void

export function markToolsSentToAPIState(): void

export function resetMicrocompactState(): void

export function estimateMessageTokens(messages: Message[]): number

export type PendingCacheEdits = {
  trigger: 'auto'
  deletedToolIds: string[]
  baselineCacheDeletedTokens: number
}

export type MicrocompactResult = {
  messages: Message[]
  compactionInfo?: {
    pendingCacheEdits?: PendingCacheEdits
  }
}

export function evaluateTimeBasedTrigger(
  messages: Message[],
  querySource: QuerySource | undefined
): { gapMinutes: number; config: TimeBasedMCConfig } | null

export async function microcompactMessages(
  messages: Message[],
  toolUseContext?: ToolUseContext,
  querySource?: QuerySource
): Promise<MicrocompactResult>
```

**Key Logic:**
- `microcompactMessages()` dispatch order:
  1. Time-based trigger check: if gap since last assistant > threshold → `maybeTimeBasedMicrocompact()` (short-circuits)
  2. Cached MC path: if `CACHED_MICROCOMPACT` feature enabled, model supported, and main thread source → `cachedMicrocompactPath()`
  3. Otherwise: return messages unchanged (legacy path removed)
- **Cached MC path:** registers tool results grouped by user message; calls `getToolResultsToDelete()`; queues `CacheEditsBlock` as `pendingCacheEdits`; does NOT mutate message content
- **Time-based MC path:** directly mutates tool result `content` to `TIME_BASED_MC_CLEARED_MESSAGE`; resets cached MC state; notifies cache break detection
- `estimateMessageTokens()`: rough estimation with 4/3 padding factor; images/documents = 2000 tokens
- `isMainThreadSource()`: prefix-matches `repl_main_thread` (handles output style variants like `repl_main_thread:outputStyle:custom`)
- Events: `tengu_cached_microcompact`, `tengu_time_based_microcompact`

---

## services/compact/postCompactCleanup.ts

**Path:** `src/services/compact/postCompactCleanup.ts`

**Purpose:** Runs cleanup tasks after any compaction (auto or manual `/compact`).

**Exports:**

```typescript
export function runPostCompactCleanup(querySource?: QuerySource): void
```

**Key Logic:** Clears: microcompact state, context collapse state, system prompt sections, classifier approvals, speculative checks, beta tracing state, session messages cache.

---

## services/compact/prompt.ts

**Path:** `src/services/compact/prompt.ts`

**Purpose:** Prompt constants used for compact summarization.

**Key Exports:**
- `NO_TOOLS_PREAMBLE`: Critical instruction string prepended to compact calls — prevents the model from invoking any tools during summarization
- `DETAILED_ANALYSIS_INSTRUCTION_BASE`: Instruction for full compaction
- `DETAILED_ANALYSIS_INSTRUCTION_PARTIAL`: Instruction for partial compaction (keeps recent messages)

---

## services/compact/sessionMemoryCompact.ts

**Path:** `src/services/compact/sessionMemoryCompact.ts`

**Purpose:** Session memory compaction — a lighter alternative to full compact that summarizes older session memory segments while preserving recent tool context.

**Constants:**
```typescript
export const DEFAULT_SM_COMPACT_CONFIG: SessionMemoryCompactConfig = {
  minTokens: 10000,
  minTextBlockMessages: 5,
  maxTokens: 40000
}
```

**Exports:**

```typescript
export type SessionMemoryCompactConfig = {
  minTokens: number
  minTextBlockMessages: number
  maxTokens: number
}

export function setSessionMemoryCompactConfig(config: SessionMemoryCompactConfig): void
export function getSessionMemoryCompactConfig(): SessionMemoryCompactConfig
export function resetSessionMemoryCompactConfig(): void
export function hasTextBlocks(message: Message): boolean
export function adjustIndexToPreserveAPIInvariants(
  messages: Message[],
  startIndex: number
): number
export function calculateMessagesToKeepIndex(
  messages: Message[],
  lastSummarizedIndex: number
): number
export function shouldUseSessionMemoryCompaction(): boolean
export async function trySessionMemoryCompaction(
  messages: Message[],
  agentId?: string,
  autoCompactThreshold?: number
): Promise<CompactionResult | null>
```

**Key Logic:**
- GrowthBook config `tengu_sm_compact_config` overrides defaults
- Guarded by `ENABLE_CLAUDE_CODE_SM_COMPACT` / `DISABLE_CLAUDE_CODE_SM_COMPACT` env vars
- AND requires both GrowthBook gates `tengu_session_memory` AND `tengu_sm_compact` to be enabled
- `adjustIndexToPreserveAPIInvariants()`: ensures cut point doesn't leave orphaned tool_use without tool_result
- `calculateMessagesToKeepIndex()`: binary search-based index calculation respecting min/max token bounds

---

## services/compact/timeBasedMCConfig.ts

**Path:** `src/services/compact/timeBasedMCConfig.ts`

**Purpose:** Configuration for time-based microcompact (triggers on idle gap).

**Exports:**

```typescript
export type TimeBasedMCConfig = {
  enabled: boolean
  gapThresholdMinutes: number
  keepRecent: number
}

export function getTimeBasedMCConfig(): TimeBasedMCConfig
```

**Key Logic:**
- GrowthBook config: `tengu_slate_heron`
- Defaults: `{ enabled: false, gapThresholdMinutes: 60, keepRecent: 5 }`

---

## services/diagnosticTracking.ts

**Path:** `src/services/diagnosticTracking.ts`

**Purpose:** Tracks file diagnostics (lint errors, type errors) before and after file edits to detect regressions introduced by Claude Code.

**Constants:**
- `MAX_DIAGNOSTICS_SUMMARY_CHARS = 4000`

**Exports:**

```typescript
export interface Diagnostic {
  severity: 'error' | 'warning' | 'information' | 'hint'
  message: string
  source?: string
  range: { start: { line: number; character: number }; end: { line: number; character: number } }
}

export interface DiagnosticFile {
  uri: string
  diagnostics: Diagnostic[]
}

export class DiagnosticTrackingService {
  static getInstance(): DiagnosticTrackingService
  initialize(mcpClient: unknown): void
  shutdown(): Promise<void>
  reset(): void
  ensureFileOpened(fileUri: string): Promise<void>
  beforeFileEdited(filePath: string): Promise<void>
}
```

**Key Logic:**
- Singleton via `getInstance()`
- `beforeFileEdited()`: captures current diagnostics for a file before any edits, so post-edit diagnostics can be compared
- `initialize()`: connects to LSP MCP client for diagnostic data
- `shutdown()`: flushes any pending diagnostic comparisons

---

## services/internalLogging.ts

**Path:** `src/services/internalLogging.ts`

**Purpose:** Internal logging utilities for internal (ant) users — logs K8s namespace, container ID, and tool permission context for debugging.

**Exports:**

```typescript
export async function getContainerId(): Promise<string | null>  // memoized
export async function logPermissionContextForAnts(
  toolPermissionContext: unknown,
  moment: string
): Promise<void>
```

**Key Logic:**
- K8s namespace: reads from `/var/run/secrets/kubernetes.io/serviceaccount/namespace`
- Container ID: extracted from `/proc/self/mountinfo` (first overlay/device entry)
- Both are memoized — container identity doesn't change mid-session
- Only logs when `USER_TYPE === 'ant'`

---

## services/MagicDocs/magicDocs.ts

**Path:** `src/services/MagicDocs/magicDocs.ts`

**Purpose:** Auto-maintained markdown documentation files that stay in sync with code changes made by Claude Code.

**Key Logic:**
- Monitors file edits and regenerates associated markdown docs
- Uses Claude to understand the semantic meaning of changes
- Prompts defined in `prompts.ts`

---

## services/MagicDocs/prompts.ts

**Path:** `src/services/MagicDocs/prompts.ts`

**Purpose:** System prompt and instruction templates for magic docs generation.

---

## services/mcpServerApproval.tsx

**Path:** `src/services/mcpServerApproval.tsx`

**Purpose:** Shows MCP server approval dialogs for pending project servers at startup, reusing the existing Ink root instance.

**Exports:**

```typescript
export async function handleMcpjsonServerApprovals(root: Root): Promise<void>
```

**Key Logic:**
- Queries `getMcpConfigsByScope('project')` for project-scoped MCP servers
- Filters to servers with status `'pending'` via `getProjectMcpServerStatus()`
- Single pending server: renders `MCPServerApprovalDialog`
- Multiple pending servers: renders `MCPServerMultiselectDialog`
- Awaits user decision via Promise/resolve pattern before returning

---

## services/mockRateLimits.ts

**Path:** `src/services/mockRateLimits.ts`

**Purpose:** Development/testing utility for simulating various rate limit scenarios without hitting actual API limits. Ant-only.

**Exports:**

```typescript
export type MockHeaderKey =
  | 'x-ratelimit-requests-remaining'
  | 'x-ratelimit-tokens-remaining'
  // ... other rate limit header names

export type MockScenario =
  | 'primary_hard_limit'
  | 'secondary_hard_limit'
  | 'approaching_limit'
  | 'fast_mode_rate_limit'
  | 'burst_limit'
  // ... 20+ scenarios total

export function setMockHeader(key: MockHeaderKey, value?: string): void
export function addExceededLimit(type: string, hoursFromNow: number): void
export function setMockEarlyWarning(
  claimAbbrev: string,
  utilization: number,
  hoursFromNow?: number
): void
export function clearMockEarlyWarning(): void
export function setMockRateLimitScenario(scenario: MockScenario): void
export function getMockHeaderless429Message(): string | null
export function getMockHeaders(): MockHeaders | null
export function getMockStatus(): string
export function clearMockHeaders(): void
export function applyMockHeaders(headers: Headers): Headers
export function shouldProcessMockLimits(): boolean
export function getCurrentMockScenario(): MockScenario | null
export function getScenarioDescription(scenario: MockScenario): string
export function setMockSubscriptionType(type: SubscriptionType | null): void
export function getMockSubscriptionType(): SubscriptionType | null
export function shouldUseMockSubscription(): boolean
export function setMockBillingAccess(hasAccess: boolean | null): void
export function isMockFastModeRateLimitScenario(): boolean
export function checkMockFastModeRateLimit(isFastModeActive?: boolean): MockHeaders | null
```

---

## services/notifier.ts

**Path:** `src/services/notifier.ts`

**Purpose:** System-level desktop notifications for long-running operations.

**Key Logic:**
- Sends macOS/Linux notifications when task completes (user is away)
- Uses `node-notifier` or native OS notification APIs
- Gated on user preference and focus state

---

## services/preventSleep.ts

**Path:** `src/services/preventSleep.ts`

**Purpose:** Prevents system sleep while Claude Code tasks are running.

**Key Logic:**
- Uses platform-specific APIs (caffeinate on macOS, systemd-inhibit on Linux)
- Returns a cleanup function to re-enable sleep
- Only active during tool execution phases

---

## services/PromptSuggestion/promptSuggestion.ts

**Path:** `src/services/PromptSuggestion/promptSuggestion.ts`

**Purpose:** Generates prompt suggestions based on current codebase context for the prompt input autocomplete.

**Key Logic:**
- Analyzes recent git changes, open files, and task patterns
- Returns ranked suggestion list
- Caches suggestions per-context hash to avoid redundant generation

---

## services/PromptSuggestion/speculation.ts

**Path:** `src/services/PromptSuggestion/speculation.ts`

**Purpose:** Speculative pre-execution — starts running likely next commands before user confirms, then either applies or discards the result.

**Key Logic:**
- Monitors user typing patterns to predict next action
- Pre-warms common tool executions (file reads, searches)
- Cancels speculative execution if prediction was wrong

---

## services/rateLimitMocking.ts

**Path:** `src/services/rateLimitMocking.ts`

**Purpose:** Facade layer for rate limit mock application and error checking.

**Exports:**

```typescript
export function processRateLimitHeaders(headers: Headers): Headers
export function shouldProcessRateLimits(isSubscriber: boolean): boolean
export function checkMockRateLimitError(
  currentModel: string,
  isFastModeActive?: boolean
): APIError | null
export function isMockRateLimitError(error: unknown): boolean
export { shouldProcessMockLimits }  // re-exported from mockRateLimits.ts
```

---

## services/rateLimitMessages.ts

**Path:** `src/services/rateLimitMessages.ts`

**Purpose:** Human-readable rate limit message formatting and display logic.

**Key Logic:**
- Formats rate limit headers into user-friendly messages
- Handles early warning, hard limit, and reset time display
- Localizes timestamps to user's timezone

---

## services/SessionMemory/prompts.ts

**Path:** `src/services/SessionMemory/prompts.ts`

**Purpose:** Prompt templates for session memory operations (summarization, retrieval, consolidation).

---

## services/SessionMemory/sessionMemory.ts

**Path:** `src/services/SessionMemory/sessionMemory.ts`

**Purpose:** Manages persistent session memory — stores and retrieves relevant context snippets across sessions.

**Key Logic:**
- GrowthBook gate: `tengu_session_memory`
- Stores memory in `~/.claude/sessions/<sessionId>/memory.jsonl`
- Retrieves relevant memories using semantic similarity
- Integrates with conversation context as system prompt additions

---

## services/SessionMemory/sessionMemoryUtils.ts

**Path:** `src/services/SessionMemory/sessionMemoryUtils.ts`

**Purpose:** Utility functions for session memory operations (formatting, filtering, path resolution).

---

## services/tokenEstimation.ts

**Path:** `src/services/tokenEstimation.ts`

**Purpose:** Rough token count estimation without calling the API tokenizer.

**Exports:**

```typescript
export function roughTokenCountEstimation(text: string): number
```

**Key Logic:** Approximates token count as `text.length / 4` (roughly 4 chars per token for English/code). Used for pre-flight estimates in compaction decisions.

---

## services/vcr.ts

**Path:** `src/services/vcr.ts`

**Purpose:** VCR (Video Cassette Recorder) test fixture system — records and replays API interactions for deterministic testing.

**Exports:**

```typescript
export async function withVCR<T>(
  messages: unknown[],
  f: () => Promise<T>
): Promise<T>

export async function withFixture<T>(
  input: unknown,
  fixtureName: string,
  f: () => Promise<T>
): Promise<T>

export async function withStreamingVCR<T>(
  messages: unknown[],
  f: () => Promise<T>
): Promise<T>
```

**Key Logic:**
- SHA1-hashes input to create fixture filenames (deterministic, content-addressed)
- `FORCE_VCR` env var: ants can force VCR mode outside of test environment
- CI guard: fails if fixture is missing and `VCR_RECORD` is not set (prevents silent misses)
- Fixture storage: `src/test-fixtures/vcr/` directory

---

## services/voice.ts

**Path:** `src/services/voice.ts`

**Purpose:** Audio recording service for push-to-talk voice input. Supports native audio (cpal via NAPI) on macOS/Linux/Windows with SoX and arecord fallbacks on Linux.

**Constants:**
- `RECORDING_SAMPLE_RATE = 16000`
- `RECORDING_CHANNELS = 1`
- `SILENCE_DURATION_SECS = '2.0'` — SoX silence detection
- `SILENCE_THRESHOLD = '3%'`

**Exports:**

```typescript
export type RecordingAvailability = {
  available: boolean
  reason: string | null
}

export async function checkVoiceDependencies(): Promise<{
  available: boolean
  missing: string[]
  installCommand: string | null
}>

export async function requestMicrophonePermission(): Promise<boolean>

export async function checkRecordingAvailability(): Promise<RecordingAvailability>

export async function startRecording(
  onData: (chunk: Buffer) => void,
  onEnd: () => void,
  options?: { silenceDetection?: boolean }
): Promise<boolean>

export function stopRecording(): void

export function _resetArecordProbeForTesting(): void
export function _resetAlsaCardsForTesting(): void
```

**Key Logic:**
- **Backend selection priority:** native (cpal via NAPI) → arecord (ALSA, Linux only) → SoX rec
- **Native module:** `audio-capture-napi` is lazy-loaded on first voice keypress (dlopen blocks event loop ~1s warm, ~8s cold)
- **arecord probe:** memoized async probe that verifies device open succeeds (not just binary existence); 150ms race timer
- **Linux ALSA guard:** checks `/proc/asound/cards` before using native cpal to avoid spurious stderr
- **WSL handling:** distinguishes WSL1 (no audio), Win10 WSL2 (no audio), Win11 WSLg (PulseAudio works)
- SoX arguments: raw PCM 16kHz/16-bit/mono with `--buffer 1024` for small chunk flushing and silence detection
- Push-to-talk mode: `silenceDetection: false` ignores native module's silence-triggered `onEnd`

---

## services/voiceKeyterms.ts

**Path:** `src/services/voiceKeyterms.ts`

**Purpose:** Generates domain-specific vocabulary hints (Deepgram "keywords") for improved STT accuracy in the voice_stream endpoint.

**Constants:**
- `MAX_KEYTERMS = 50`
- `GLOBAL_KEYTERMS`: hardcoded list including `'MCP'`, `'symlink'`, `'grep'`, `'regex'`, `'localhost'`, `'codebase'`, `'TypeScript'`, `'JSON'`, `'OAuth'`, `'webhook'`, `'gRPC'`, `'dotfiles'`, `'subagent'`, `'worktree'`

**Exports:**

```typescript
export function splitIdentifier(name: string): string[]

export async function getVoiceKeyterms(
  recentFiles?: ReadonlySet<string>
): Promise<string[]>
```

**Key Logic:**
- `splitIdentifier()`: splits camelCase/PascalCase/kebab-case/snake_case/path identifiers into words; discards fragments ≤2 chars
- `getVoiceKeyterms()`: combines global terms + project root basename + git branch words + recent file name words
- Project root basename kept whole (not split) to match full project name phrases
- Git branch words split via `splitIdentifier()`; recent files split by filename stem
- Capped at `MAX_KEYTERMS = 50`

---

## services/voiceStreamSTT.ts

**Path:** `src/services/voiceStreamSTT.ts`

**Purpose:** voice_stream WebSocket STT client. Connects to `wss://api.anthropic.com/api/ws/speech_to_text/voice_stream` using OAuth credentials for push-to-talk transcription.

**Constants:**
- `VOICE_STREAM_PATH = '/api/ws/speech_to_text/voice_stream'`
- `KEEPALIVE_INTERVAL_MS = 8_000`
- `FINALIZE_TIMEOUTS_MS = { safety: 5_000, noData: 1_500 }` (exported for tests)
- Wire messages: `KEEPALIVE_MSG = '{"type":"KeepAlive"}'`, `CLOSE_STREAM_MSG = '{"type":"CloseStream"}'`

**Exports:**

```typescript
export const FINALIZE_TIMEOUTS_MS: { safety: number; noData: number }

export type VoiceStreamCallbacks = {
  onTranscript: (text: string, isFinal: boolean) => void
  onError: (error: string, opts?: { fatal?: boolean }) => void
  onClose: () => void
  onReady: (connection: VoiceStreamConnection) => void
}

export type FinalizeSource =
  | 'post_closestream_endpoint'
  | 'no_data_timeout'
  | 'safety_timeout'
  | 'ws_close'
  | 'ws_already_closed'

export type VoiceStreamConnection = {
  send: (audioChunk: Buffer) => void
  finalize: () => Promise<FinalizeSource>
  close: () => void
  isConnected: () => boolean
}

export function isVoiceStreamAvailable(): boolean

export async function connectVoiceStream(
  callbacks: VoiceStreamCallbacks,
  options?: { language?: string; keyterms?: string[] }
): Promise<VoiceStreamConnection | null>
```

**Key Logic:**
- Only available for OAuth-authenticated users (OAuth tokens); gates on `isOAuthAuthEnabled()` and valid access token
- Routes to `api.anthropic.com` (not `claude.ai`) to avoid Cloudflare TLS fingerprinting challenges
- `VOICE_STREAM_BASE_URL` env var allows override for testing
- URL params: `encoding=linear16`, `sample_rate=16000`, `channels=1`, `endpointing_ms=300`, `utterance_end_ms=1000`
- GrowthBook gate `tengu_cobalt_frost`: enables Nova 3 STT provider via `use_conversation_engine=true&stt_provider=deepgram-nova3`
- Keyterms appended as repeated `keyterms=` query params
- keepalive interval: 8s
- `finalize()` sends `CloseStream`, races `noData` (1.5s) vs `safety` (5s) timers vs `TranscriptEndpoint` message
- Wire message types: `TranscriptText`, `TranscriptEndpoint`, `TranscriptError`, `error`
- Ant-only build (behind `feature('VOICE_MODE')` gate)

---

## context/QueuedMessageContext.tsx

**Path:** `src/context/QueuedMessageContext.tsx`

**Purpose:** React context for the queued message system — tracks messages waiting to be sent to Claude when the current turn completes.

**Exports:**
```typescript
export const QueuedMessageContext: React.Context<QueuedMessage[]>
export function useQueuedMessages(): QueuedMessage[]
export function QueuedMessageProvider(props: { children: React.ReactNode }): JSX.Element
```

---

## context/fpsMetrics.tsx

**Path:** `src/context/fpsMetrics.tsx`

**Purpose:** FPS (frames per second) measurement context for Ink terminal rendering performance monitoring.

**Exports:**
```typescript
export function FpsMetricsProvider(props: { children: React.ReactNode }): JSX.Element
export function useFpsMetrics(): { fps: number; frameCount: number }
```

---

## context/mailbox.tsx

**Path:** `src/context/mailbox.tsx`

**Purpose:** Provides inter-agent messaging context — the "mailbox" for receiving messages from other agents/workers.

**Exports:**
```typescript
export type MailboxMessage = { from: string; content: string; timestamp: number }
export const MailboxContext: React.Context<MailboxMessage[]>
export function MailboxProvider(props: { children: React.ReactNode }): JSX.Element
export function useMailbox(): MailboxMessage[]
```

---

## context/modalContext.tsx

**Path:** `src/context/modalContext.tsx`

**Purpose:** Context for managing modal dialog state — tracks which modal is currently open and provides open/close actions.

**Exports:**
```typescript
export type ModalState = { isOpen: boolean; content: React.ReactNode | null }
export const ModalContext: React.Context<ModalState>
export function ModalProvider(props: { children: React.ReactNode }): JSX.Element
export function useModal(): { open: (content: React.ReactNode) => void; close: () => void }
```

---

## context/notifications.tsx

**Path:** `src/context/notifications.tsx`

**Purpose:** Notification queue management — displays toast-style notifications in the status line with priority ordering, deduplication, fold/merge, and timeout handling.

**Types:**

```typescript
type Priority = 'low' | 'medium' | 'high' | 'immediate'

type BaseNotification = {
  key: string
  invalidates?: string[]
  priority: Priority
  timeoutMs?: number
  fold?: (accumulator: Notification, incoming: Notification) => Notification
}

type TextNotification = BaseNotification & { text: string; color?: keyof Theme }
type JSXNotification = BaseNotification & { jsx: React.ReactNode }
export type Notification = TextNotification | JSXNotification
```

**Exports:**

```typescript
const DEFAULT_TIMEOUT_MS = 8000

export function useNotifications(): {
  addNotification: (content: Notification) => void
  removeNotification: (key: string) => void
}
```

**Key Logic:**
- Notification state lives in `AppState` (`notifications.current` + `notifications.queue`)
- `immediate` priority: bypasses queue, shows immediately, re-queues current (non-immediate) notification
- `fold` function: merges notifications with the same key (accumulator pattern)
- Deduplication: only one notification per key in queue + current
- `invalidates[]`: removes named notifications from queue and clears current if matching
- `DEFAULT_TIMEOUT_MS = 8000` (8 seconds); auto-advance to next queued after timeout
- Module-level `currentTimeoutId` tracks the active auto-dismiss timer

---

## context/overlayContext.tsx

**Path:** `src/context/overlayContext.tsx`

**Purpose:** Overlay tracking for Escape key coordination. Tracks which overlays (dialogs, selects) are currently open so the cancel handler doesn't misinterpret Escape presses.

**Constants:**
```typescript
const NON_MODAL_OVERLAYS = new Set(['autocomplete'])
```

**Exports:**

```typescript
export function useRegisterOverlay(id: string, enabled?: boolean): void
export function useIsOverlayActive(): boolean
export function useIsModalOverlayActive(): boolean
```

**Key Logic:**
- State stored in `AppState.activeOverlays: Set<string>`
- `useRegisterOverlay()`: registers on mount (useEffect), unregisters on unmount via cleanup
- On overlay close: triggers `instances.get(process.stdout)?.invalidatePrevFrame()` (via useLayoutEffect) to force full-damage diff — prevents ghost cells from tall overlays (e.g. FuzzyPicker)
- `useIsOverlayActive()`: `activeOverlays.size > 0`
- `useIsModalOverlayActive()`: any overlay in set that is NOT in `NON_MODAL_OVERLAYS`

---

## context/promptOverlayContext.tsx

**Path:** `src/context/promptOverlayContext.tsx`

**Purpose:** Context for prompt-level overlay management — tracks overlays that affect prompt input focus and behavior.

**Exports:**
```typescript
export function useRegisterPromptOverlay(id: string, enabled?: boolean): void
export function useIsPromptOverlayActive(): boolean
```

---

## context/stats.tsx

**Path:** `src/context/stats.tsx`

**Purpose:** In-process performance metrics store with reservoir sampling for histograms. Persists metrics to project config on process exit.

**Constants:**
- `RESERVOIR_SIZE = 1024` — reservoir sampling capacity for histograms

**Types:**

```typescript
export type StatsStore = {
  increment(name: string, value?: number): void
  set(name: string, value: number): void
  observe(name: string, value: number): void
  add(name: string, value: string): void
  getAll(): Record<string, number>
}
```

**Exports:**

```typescript
export function createStatsStore(): StatsStore
export const StatsContext: React.Context<StatsStore | null>
export function StatsProvider(props: { store?: StatsStore; children: React.ReactNode }): JSX.Element
export function useStats(): StatsStore
export function useCounter(name: string): (value?: number) => void
export function useGauge(name: string): (value: number) => void
export function useTimer(name: string): (value: number) => void
// ... additional hook exports
```

**Key Logic:**
- `createStatsStore()`: creates a stats store with three internal data structures:
  - `metrics: Map<string, number>` — counters and gauges
  - `histograms: Map<string, Histogram>` — reservoir-sampled distributions
  - `sets: Map<string, Set<string>>` — unique value sets (reported as `.size`)
- `observe()`: histogram using reservoir sampling (Algorithm R) with `RESERVOIR_SIZE = 1024`
- `getAll()`: returns flat `Record<string, number>` with histogram percentiles (`_p50`, `_p95`, `_p99`), min, max, avg, count
- `StatsProvider`: flushes metrics to `lastSessionMetrics` in project config on process `'exit'` event
- `useCounter()` / `useGauge()` / `useTimer()`: memoized hooks returning bound store methods

---

## context/voice.tsx

**Path:** `src/context/voice.tsx`

**Purpose:** Voice mode context — provides voice recording state, transcript, and connection status to the UI.

**Exports:**

```typescript
export type VoiceContextValue = {
  isRecording: boolean
  transcript: string
  isConnecting: boolean
  error: string | null
  startRecording: () => void
  stopRecording: () => void
}

export const VoiceContext: React.Context<VoiceContextValue>
export function VoiceProvider(props: { children: React.ReactNode }): JSX.Element
export function useVoice(): VoiceContextValue
```

---

## screens/Doctor.tsx

**Path:** `src/screens/Doctor.tsx`

**Purpose:** The `/doctor` command UI screen — displays system health diagnostics including version info, environment checks, MCP server status, sandbox status, keybinding warnings, and available updates.

**Props:**
```typescript
type Props = {
  onDone: () => void
}
```

**Internal Types:**
```typescript
type AgentInfo = {
  name: string
  version: string
  // ...
}

type VersionLockInfo = {
  locked: boolean
  version?: string
  // ...
}
```

**Key Logic:**
- Uses `getDoctorDiagnostic()` for environment checks
- Calls `checkContextWarnings()` for context-related issues
- Loads dist tags with `getNpmDistTags()` and `getGcsDistTags()` (wrapped in Suspense)
- Renders sub-sections: `SandboxDoctorSection`, `ValidationErrorsList`, `KeybindingWarnings`, `McpParsingWarnings`
- Displays version comparison: current vs latest npm/GCS versions
- Shows agent info, version lock status, and update channels

---

## screens/REPL.tsx

**Path:** `src/screens/REPL.tsx`

**Purpose:** The main interactive REPL screen — the primary conversational UI that orchestrates the entire Claude Code interactive session.

**Key Logic:**
- Manages the full conversation lifecycle: user input → API query → tool execution → response display
- Handles all interactive features: conversation history, compaction, clear, file editing, permissions
- Routes slash commands to command handlers
- Manages modal dialogs (permissions, MCP approvals, etc.)
- Integrates with all context providers: notifications, overlays, voice, stats
- The largest component in the codebase, responsible for the overall user interaction loop

---

## screens/ResumeConversation.tsx

**Path:** `src/screens/ResumeConversation.tsx`

**Purpose:** UI screen for the session resume flow — shows a list of recent sessions with previews and allows the user to select one to resume.

**Key Logic:**
- Loads session index from disk
- Renders `SessionPreview` components for each session
- Handles keyboard navigation (arrow keys, Enter to select)
- Filters sessions by project root or shows all
- Passes selected session ID back to caller via callback

---

## Cross-Cutting Architecture Notes

### State Management Architecture

The codebase uses three distinct state layers:

1. **`bootstrap/state.ts`** — Global mutable singleton for session-scoped state (costs, model, telemetry). Intentionally a DAG leaf — imports nothing from services.

2. **React `AppState`** — UI state via Zustand-like store (`state/AppState.ts`). Accessed via `useAppState()` selectors and `useSetAppState()`.

3. **React Contexts** — Domain-specific contexts (notifications, overlays, stats, voice) for scoped subtree state.

### Analytics Architecture

Events flow through a multi-layer pipeline:

```
logEvent() → index.ts queue → sink.ts dispatch
  → [sampling check] → [isSinkKilled check]
  → datadog.ts (strips _PROTO_ keys)
  → firstPartyEventLogger.ts (hoists _PROTO_ keys to proto fields)
```

All analytics is disabled when: `NODE_ENV === test`, 3P providers (Bedrock/Vertex/Foundry), or `isTelemetryDisabled()`.

### Compact/Microcompact Architecture

Context compression has multiple layers:

1. **`apiMicrocompact.ts`** — API-native server-side editing (cache_edits) — no client-side mutation
2. **`microCompact.ts`** — Client-side tool result clearing (cached path + time-based path)
3. **`autoCompact.ts`** — Full conversation summarization trigger (threshold-based)
4. **`compact.ts`** — Full summarization implementation (LLM call)
5. **`sessionMemoryCompact.ts`** — Lighter session memory segment summarization

### GrowthBook Feature Flag Naming Convention

All feature flags use obfuscated/mangled names with `tengu_` prefix to prevent scraping:
- Feature flags: `tengu_prompt_cache_1h_config`, `tengu_session_memory`, `tengu_sm_compact`, etc.
- Kill switches: `tengu_frond_boric` (analytics sink killswitch)
- Voice: `tengu_cobalt_frost` (Nova 3 STT)
- AutoDream: `tengu_onyx_plover`
- Coordinator mode: checked via `feature('COORDINATOR_MODE')` bundle flag
