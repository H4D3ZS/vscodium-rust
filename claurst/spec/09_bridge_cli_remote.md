# Claude Code — Bridge Protocol, CLI Framework & Remote Systems

## Table of Contents

1. [Bridge System Overview](#1-bridge-system-overview)
2. [Bridge Types & Core Data Structures](#2-bridge-types--core-data-structures)
3. [Bridge API Client](#3-bridge-api-client)
4. [Bridge Configuration & Auth](#4-bridge-configuration--auth)
5. [Bridge Entitlement & Feature Gating](#5-bridge-entitlement--feature-gating)
6. [Session Lifecycle: Standalone Bridge (bridgeMain.ts)](#6-session-lifecycle-standalone-bridge-bridgemaints)
7. [REPL Bridge (replBridge.ts / initReplBridge.ts)](#7-repl-bridge-replbridgets--initreplbridgets)
8. [Env-Less Bridge Core (remoteBridgeCore.ts)](#8-env-less-bridge-core-remotebridgecorts)
9. [Transport Layer](#9-transport-layer)
10. [Message Protocol (bridgeMessaging.ts)](#10-message-protocol-bridgemessagingts)
11. [JWT Authentication (jwtUtils.ts)](#11-jwt-authentication-jwtutilsts)
12. [Session ID Compatibility (sessionIdCompat.ts)](#12-session-id-compatibility-sessionidcompatts)
13. [Work Secrets & CCR v2 Registration (workSecret.ts)](#13-work-secrets--ccr-v2-registration-worksecretsts)
14. [Bridge Pointer: Crash Recovery (bridgePointer.ts)](#14-bridge-pointer-crash-recovery-bridgepointerts)
15. [Permission Callbacks (bridgePermissionCallbacks.ts)](#15-permission-callbacks-bridgepermissioncallbacksts)
16. [Inbound Messages & Attachments](#16-inbound-messages--attachments)
17. [Session Runner (sessionRunner.ts)](#17-session-runner-sessionrunnerts)
18. [Bridge Debug & Fault Injection (bridgeDebug.ts)](#18-bridge-debug--fault-injection-bridgedebugets)
19. [Bridge Utilities](#19-bridge-utilities)
20. [CLI Framework](#20-cli-framework)
21. [CLI Transports](#21-cli-transports)
22. [Remote Session System](#22-remote-session-system)
23. [replLauncher.tsx](#23-repplaunchertsx)
24. [Configuration Defaults & GrowthBook Flags](#24-configuration-defaults--growthbook-flags)
25. [Complete API Endpoint Reference](#25-complete-api-endpoint-reference)
26. [WebSocket & SSE Protocol Reference](#26-websocket--sse-protocol-reference)

---

## 1. Bridge System Overview

The "Bridge" (Remote Control) system allows a local Claude Code CLI session to be driven from the claude.ai web application. It creates a bidirectional communication channel between the running CLI process and the cloud backend (CCR — Cloud Code Runner).

### Architecture: Two Bridge Variants

**V1 — Environment-Based Bridge (env-based)**
- Uses the Environments API (`/v1/environments/bridge`).
- Bridge registers as an "environment", polls for "work" (session dispatches).
- Session transport: WebSocket (v1) or SSE+CCRClient (CCR v2) via `HybridTransport` or `SSETransport`.
- `initBridgeCore()` in `replBridge.ts` handles the REPL side.
- `runBridgeLoop()` in `bridgeMain.ts` handles the standalone `claude remote-control` side.

**V2 — Environment-Less Bridge (env-less)**
- No Environments API layer whatsoever.
- Direct flow: POST `/v1/code/sessions` → POST `/v1/code/sessions/{id}/bridge` → `createV2ReplTransport()`.
- Only for REPL sessions; daemon/print stay on env-based.
- Gated by `tengu_bridge_repl_v2` GrowthBook flag.

### Two Deployment Modes

**REPL Bridge (always-on / `/remote-control`)**
- Initialized by `initReplBridge()`, called from `useReplBridge` hook or `print.ts`.
- Runs inside the existing REPL process; messages from claude.ai are injected as user input.
- Lives in `bridge/replBridge.ts`, `bridge/initReplBridge.ts`, `bridge/remoteBridgeCore.ts`.

**Standalone Bridge (`claude remote-control`)**
- Spawns child claude processes per session.
- Main loop in `bridge/bridgeMain.ts` → `runBridgeLoop()`.
- Supports multi-session spawn modes: `single-session`, `worktree`, `same-dir`.

---

## 2. Bridge Types & Core Data Structures

**File:** `bridge/types.ts`

### Constants

```typescript
DEFAULT_SESSION_TIMEOUT_MS = 24 * 60 * 60 * 1000   // 24 hours
BRIDGE_LOGIN_INSTRUCTION: string  // "Remote Control is only available with claude.ai subscriptions..."
BRIDGE_LOGIN_ERROR: string        // Full error printed when not authenticated
REMOTE_CONTROL_DISCONNECTED_MSG = 'Remote Control disconnected.'
```

### WorkData

```typescript
type WorkData = {
  type: 'session' | 'healthcheck'
  id: string  // session ID
}
```

### WorkResponse

The response from polling for work (`GET .../work/poll`):

```typescript
type WorkResponse = {
  id: string               // work item ID
  type: 'work'
  environment_id: string
  state: string
  data: WorkData
  secret: string           // base64url-encoded JSON WorkSecret
  created_at: string
}
```

### WorkSecret

Decoded from `WorkResponse.secret` (base64url JSON):

```typescript
type WorkSecret = {
  version: number                          // Must be 1
  session_ingress_token: string            // JWT for session-ingress API calls
  api_base_url: string
  sources: Array<{
    type: string
    git_info?: { type: string; repo: string; ref?: string; token?: string }
  }>
  auth: Array<{ type: string; token: string }>
  claude_code_args?: Record<string, string> | null
  mcp_config?: unknown | null
  environment_variables?: Record<string, string> | null
  use_code_sessions?: boolean              // Server-driven CCR v2 selector
}
```

### BridgeConfig

Configuration object passed to the main loop and API client:

```typescript
type BridgeConfig = {
  dir: string                    // Working directory
  machineName: string            // Hostname
  branch: string                 // Git branch
  gitRepoUrl: string | null
  maxSessions: number            // Capacity for multi-session mode
  spawnMode: SpawnMode           // 'single-session' | 'worktree' | 'same-dir'
  verbose: boolean
  sandbox: boolean
  bridgeId: string               // Client-generated UUID identifying this bridge instance
  workerType: string             // Sent as metadata.worker_type (e.g. 'claude_code')
  environmentId: string          // Client-generated UUID for idempotent registration
  reuseEnvironmentId?: string    // Backend-issued ID to reuse on re-register
  apiBaseUrl: string
  sessionIngressUrl: string      // May differ from apiBaseUrl in local dev
  debugFile?: string
  sessionTimeoutMs?: number
}
```

### SpawnMode

```typescript
type SpawnMode = 'single-session' | 'worktree' | 'same-dir'
```

- `single-session`: One session, bridge tears down when it ends.
- `worktree`: Persistent server, each session gets an isolated git worktree.
- `same-dir`: Persistent server, sessions share cwd (may conflict).

### BridgeWorkerType

```typescript
type BridgeWorkerType = 'claude_code' | 'claude_code_assistant'
```

### SessionActivity

```typescript
type SessionActivityType = 'tool_start' | 'text' | 'result' | 'error'

type SessionActivity = {
  type: SessionActivityType
  summary: string   // e.g. "Editing src/foo.ts", "Reading package.json"
  timestamp: number
}
```

### SessionHandle

Interface returned by `SessionSpawner.spawn()`:

```typescript
type SessionHandle = {
  sessionId: string
  done: Promise<SessionDoneStatus>       // 'completed' | 'failed' | 'interrupted'
  kill(): void
  forceKill(): void
  activities: SessionActivity[]          // Ring buffer of last ~10 activities
  currentActivity: SessionActivity | null
  accessToken: string                    // session_ingress_token
  lastStderr: string[]                   // Ring buffer of last stderr lines
  writeStdin(data: string): void
  updateAccessToken(token: string): void
}
```

### SessionSpawnOpts

```typescript
type SessionSpawnOpts = {
  sessionId: string
  sdkUrl: string
  accessToken: string
  useCcrV2?: boolean            // Spawn child with CCR v2 env vars
  workerEpoch?: number          // Required when useCcrV2=true
  onFirstUserMessage?: (text: string) => void
}
```

### BridgeApiClient Interface

```typescript
type BridgeApiClient = {
  registerBridgeEnvironment(config: BridgeConfig): Promise<{
    environment_id: string
    environment_secret: string
  }>
  pollForWork(
    environmentId: string,
    environmentSecret: string,
    signal?: AbortSignal,
    reclaimOlderThanMs?: number,
  ): Promise<WorkResponse | null>
  acknowledgeWork(environmentId, workId, sessionToken): Promise<void>
  stopWork(environmentId, workId, force): Promise<void>
  deregisterEnvironment(environmentId): Promise<void>
  sendPermissionResponseEvent(sessionId, event, sessionToken): Promise<void>
  archiveSession(sessionId): Promise<void>
  reconnectSession(environmentId, sessionId): Promise<void>
  heartbeatWork(environmentId, workId, sessionToken): Promise<{
    lease_extended: boolean
    state: string
  }>
}
```

### PermissionResponseEvent

```typescript
type PermissionResponseEvent = {
  type: 'control_response'
  response: {
    subtype: 'success'
    request_id: string
    response: Record<string, unknown>
  }
}
```

### BridgeLogger Interface

Full interface for the bridge UI/logging system (defined in `types.ts`):

```typescript
type BridgeLogger = {
  printBanner(config, environmentId): void
  logSessionStart(sessionId, prompt): void
  logSessionComplete(sessionId, durationMs): void
  logSessionFailed(sessionId, error): void
  logStatus(message): void
  logVerbose(message): void
  logError(message): void
  logReconnected(disconnectedMs): void
  updateIdleStatus(): void
  updateReconnectingStatus(delayStr, elapsedStr): void
  updateSessionStatus(sessionId, elapsed, activity, trail): void
  clearStatus(): void
  setRepoInfo(repoName, branch): void
  setDebugLogPath(path): void
  setAttached(sessionId): void
  updateFailedStatus(error): void
  toggleQr(): void
  updateSessionCount(active, max, mode): void
  setSpawnModeDisplay(mode): void
  addSession(sessionId, url): void
  updateSessionActivity(sessionId, activity): void
  setSessionTitle(sessionId, title): void
  removeSession(sessionId): void
  refreshDisplay(): void
}
```

---

## 3. Bridge API Client

**File:** `bridge/bridgeApi.ts`

### Exports

#### `validateBridgeId(id: string, label: string): string`

Validates that a server-provided ID is safe for URL path interpolation. Uses pattern `/^[a-zA-Z0-9_-]+$/`. Throws on unsafe characters to prevent path traversal attacks.

#### `class BridgeFatalError extends Error`

Non-retryable bridge errors. Carries:
- `status: number` — HTTP status code
- `errorType: string | undefined` — server-provided error type (e.g. `"environment_expired"`)

#### `createBridgeApiClient(deps: BridgeApiDeps): BridgeApiClient`

Factory for the HTTP client. Dependencies:

```typescript
type BridgeApiDeps = {
  baseUrl: string
  getAccessToken: () => string | undefined
  runnerVersion: string
  onDebug?: (msg: string) => void
  onAuth401?: (staleAccessToken: string) => Promise<boolean>
  getTrustedDeviceToken?: () => string | undefined
}
```

**Request Headers** (on all API calls):
```
Authorization: Bearer <token>
Content-Type: application/json
anthropic-version: 2023-06-01
anthropic-beta: environments-2025-11-01
x-environment-runner-version: <runnerVersion>
X-Trusted-Device-Token: <token>  (optional, when tengu_sessions_elevated_auth_enforcement)
```

**OAuth 401 Retry:** On 401, calls `onAuth401(staleToken)`. If token refresh succeeds, retries the request once. If the retry also returns 401, throws `BridgeFatalError`.

**Poll endpoint** (`pollForWork`): Uses `environmentSecret` (not OAuth token) as Bearer auth. Logs empty polls every 1st time and then every 100th consecutive empty poll.

#### `isExpiredErrorType(errorType: string | undefined): boolean`

Returns true if the error type string contains `'expired'` or `'lifetime'`.

#### `isSuppressible403(err: BridgeFatalError): boolean`

Returns true for 403 errors involving `external_poll_sessions` or `environments:manage` scope — these are permission errors for non-critical operations that should not be surfaced to users.

### Error Status Handling

| HTTP Status | Behavior |
|-------------|----------|
| 200, 204 | Success |
| 401 | `BridgeFatalError` with login instruction |
| 403 (expired errorType) | `BridgeFatalError`: "session has expired" |
| 403 (other) | `BridgeFatalError`: access denied / org permissions |
| 404 | `BridgeFatalError`: not found |
| 410 | `BridgeFatalError` with `errorType='environment_expired'` |
| 429 | Plain `Error`: rate limited |
| Other | Plain `Error` with status code |

---

## 4. Bridge Configuration & Auth

**File:** `bridge/bridgeConfig.ts`

Consolidates auth/URL resolution. Two layers: dev overrides (ant-only) and production OAuth.

### Exports

#### `getBridgeTokenOverride(): string | undefined`

Returns `process.env.CLAUDE_BRIDGE_OAUTH_TOKEN` if `process.env.USER_TYPE === 'ant'`, else `undefined`.

#### `getBridgeBaseUrlOverride(): string | undefined`

Returns `process.env.CLAUDE_BRIDGE_BASE_URL` if `process.env.USER_TYPE === 'ant'`, else `undefined`.

#### `getBridgeAccessToken(): string | undefined`

Dev override first, then `getClaudeAIOAuthTokens()?.accessToken`.

#### `getBridgeBaseUrl(): string`

Dev override first, then `getOauthConfig().BASE_API_URL`.

---

## 5. Bridge Entitlement & Feature Gating

**File:** `bridge/bridgeEnabled.ts`

### Exports

#### `isBridgeEnabled(): boolean`

Synchronous. Requires:
1. Build flag `feature('BRIDGE_MODE')` must be true
2. `isClaudeAISubscriber()` — excludes Bedrock/Vertex/API key users
3. GrowthBook flag `tengu_ccr_bridge` (cached, may be stale)

#### `isBridgeEnabledBlocking(): Promise<boolean>`

Like `isBridgeEnabled()` but awaits GrowthBook server fetch if disk cache says false. Use at entitlement gates to avoid unfair denials from stale cache.

#### `getBridgeDisabledReason(): Promise<string | null>`

Returns a user-facing reason string if bridge is unavailable, or `null` if enabled. Checks:
1. Not a claude.ai subscriber
2. Missing `user:profile` scope (setup-token / env-var OAuth tokens)
3. Missing `organizationUuid` in OAuth account info
4. `tengu_ccr_bridge` gate off

#### `isEnvLessBridgeEnabled(): boolean`

Gates `tengu_bridge_repl_v2` — the V2 (env-less) REPL bridge path. Cached, may be stale.

#### `isCseShimEnabled(): boolean`

Kill-switch for `cse_*` → `session_*` retag shim. Reads `tengu_bridge_repl_v2_cse_shim_enabled` (default `true`). When false, `toCompatSessionId()` is a no-op.

#### `checkBridgeMinVersion(): string | null`

Returns error string if CLI version is below `tengu_bridge_min_version` config (default `'0.0.0'`).

#### `getCcrAutoConnectDefault(): boolean`

Returns `true` when `feature('CCR_AUTO_CONNECT')` and `tengu_cobalt_harbor` gate are both enabled. Used as default for `remoteControlAtStartup` config.

#### `isCcrMirrorEnabled(): boolean`

Returns `true` when `feature('CCR_MIRROR')` and either `CLAUDE_CODE_CCR_MIRROR` env var is truthy or `tengu_ccr_mirror` gate is enabled.

---

## 6. Session Lifecycle: Standalone Bridge (bridgeMain.ts)

**File:** `bridge/bridgeMain.ts`

This is the main loop for `claude remote-control` (standalone mode).

### BackoffConfig

```typescript
type BackoffConfig = {
  connInitialMs: number       // Default: 2,000ms
  connCapMs: number           // Default: 120,000ms (2 min)
  connGiveUpMs: number        // Default: 600,000ms (10 min)
  generalInitialMs: number    // Default: 500ms
  generalCapMs: number        // Default: 30,000ms
  generalGiveUpMs: number     // Default: 600,000ms (10 min)
  shutdownGraceMs?: number    // SIGTERM→SIGKILL grace. Default: 30s
  stopWorkBaseDelayMs?: number // stopWork retry base. Default: 1000ms
}
```

### Constants

```typescript
STATUS_UPDATE_INTERVAL_MS = 1_000    // Live display refresh rate
SPAWN_SESSIONS_DEFAULT = 32          // Default max sessions
```

### `runBridgeLoop(config, environmentId, environmentSecret, api, spawner, logger, signal, backoffConfig?, initialSessionId?, getAccessToken?): Promise<void>`

Main exported function. Manages:
- `activeSessions: Map<string, SessionHandle>` — currently running sessions
- `sessionStartTimes: Map<string, number>` — for elapsed time display
- `sessionWorkIds: Map<string, string>` — work ID per session
- `sessionCompatIds: Map<string, string>` — `session_*` ID per session (stable per-session)
- `sessionIngressTokens: Map<string, string>` — JWT per session for heartbeat auth
- `sessionTimers: Map<string, ReturnType<setTimeout>>` — per-session timeout timers
- `completedWorkIds: Set<string>` — prevents double-stop
- `sessionWorktrees: Map<string, {...}>` — worktree cleanup state
- `timedOutSessions: Set<string>` — sessions killed by timeout watchdog
- `titledSessions: Set<string>` — sessions already titled (suppresses auto-title)
- `capacityWake: CapacityWake` — signals early wake from at-capacity sleep

**Heartbeat Logic:**

`heartbeatActiveWorkItems()` iterates all `activeSessions`, calls `api.heartbeatWork()` with each session's ingress token. On `BridgeFatalError` 401/403 (JWT expired), calls `api.reconnectSession()` to trigger server re-dispatch. Returns:
- `'ok'` — at least one heartbeat succeeded
- `'auth_failed'` — one or more sessions had expired JWTs (re-queued via reconnect)
- `'fatal'` — 404/410 errors (environment expired)
- `'failed'` — all heartbeats failed for other reasons

**Token Refresh (Proactive):**

`createTokenRefreshScheduler()` fires 5 minutes before each session's JWT expires.
- **V1 sessions**: calls `handle.updateAccessToken(oauthToken)` to inject the new OAuth token directly to the child process stdin.
- **V2 sessions** (`v2Sessions` set): calls `api.reconnectSession()` to trigger server re-dispatch with a fresh JWT (V2 children validate the JWT's `session_id` claim, so OAuth tokens cannot be used directly).

**Session Done Handler:**

`onSessionDone()` cleans up all maps, fires `capacityWake.wake()`, then:
- `status = 'completed'`: logs completion
- `status = 'failed'`: logs failure with stderr (unless it was shutdown)
- `status = 'interrupted'`: logs verbose only

For non-interrupted sessions with a `workId`, calls `stopWorkWithRetry()` (3 retries with 1s/2s/4s backoff). For worktree sessions, calls `removeAgentWorktree()`.

**Status Display:**

Tick every 1s via `setInterval`. Calls `logger.updateSessionStatus()` for each active session showing: elapsed time, current activity, last 5 tool activities as a trail.

### Session Spawn: V1 vs V2

When work arrives:
1. Decodes `WorkSecret` from `work.secret`.
2. If `workSecret.use_code_sessions === true`: uses CCR v2 path (`buildCCRv2SdkUrl`, `registerWorker()`, `useCcrV2=true`).
3. Otherwise: uses V1 path (`buildSdkUrl()`, `useCcrV2=false`).

### SpawnMode: Worktree

When `config.spawnMode === 'worktree'`:
- Calls `createAgentWorktree()` to create an isolated git worktree.
- Passes the worktree path as the session's working directory.
- On session completion, calls `removeAgentWorktree()`.

### Graceful Shutdown Sequence

1. Abort `loopSignal`.
2. Stop all status update timers.
3. For each active session: call `handle.kill()`, await `handle.done`.
4. Await all pending cleanups (stopWork + worktree removal).
5. Call `api.deregisterEnvironment()`.
6. If session(s) were not fatal-exited, show resume hint.

---

## 7. REPL Bridge (replBridge.ts / initReplBridge.ts)

### ReplBridgeHandle (`bridge/replBridge.ts`)

```typescript
type ReplBridgeHandle = {
  bridgeSessionId: string
  environmentId: string
  sessionIngressUrl: string
  writeMessages(messages: Message[]): void
  writeSdkMessages(messages: SDKMessage[]): void
  sendControlRequest(request: SDKControlRequest): void
  sendControlResponse(response: SDKControlResponse): void
  sendControlCancelRequest(requestId: string): void
  sendResult(): void
  teardown(): Promise<void>
}
```

### BridgeState

```typescript
type BridgeState = 'ready' | 'connected' | 'reconnecting' | 'failed'
```

### BridgeCoreParams

The explicit-parameter interface to `initBridgeCore()` (enabling daemon/non-REPL callers):

```typescript
type BridgeCoreParams = {
  dir: string
  machineName: string
  branch: string
  gitRepoUrl: string | null
  title: string
  baseUrl: string
  sessionIngressUrl: string
  workerType: string
  sessionId: string             // REPL's own session ID
  getAccessToken: () => string | undefined
  onAuth401?: (staleAccessToken: string) => Promise<boolean>
  toSDKMessages: (messages: Message[]) => SDKMessage[]
  initialHistoryCap: number
  pollConfig?: PollIntervalConfig
  // ... plus InitBridgeOptions callbacks
}
```

### InitBridgeOptions (`bridge/initReplBridge.ts`)

```typescript
type InitBridgeOptions = {
  onInboundMessage?: (msg: SDKMessage) => void | Promise<void>
  onPermissionResponse?: (response: SDKControlResponse) => void
  onInterrupt?: () => void
  onSetModel?: (model: string | undefined) => void
  onSetMaxThinkingTokens?: (maxTokens: number | null) => void
  onSetPermissionMode?: (mode: PermissionMode) => { ok: true } | { ok: false; error: string }
  onStateChange?: (state: BridgeState, detail?: string) => void
  initialMessages?: Message[]
  initialName?: string
  getMessages?: () => Message[]
  previouslyFlushedUUIDs?: Set<string>
  perpetual?: boolean
}
```

### replBridgeHandle.ts

Global pointer to the active REPL bridge handle:

```typescript
setReplBridgeHandle(h: ReplBridgeHandle | null): void
getReplBridgeHandle(): ReplBridgeHandle | null
getSelfBridgeCompatId(): string | undefined  // Returns session_* compat ID
```

---

## 8. Env-Less Bridge Core (remoteBridgeCore.ts)

**File:** `bridge/remoteBridgeCore.ts`

Direct-connect bridge that bypasses the Environments API entirely.

### Connection Flow

1. `POST /v1/code/sessions` with `{ title, bridge: {} }` → `session.id` (`cse_*`)
2. `POST /v1/code/sessions/{id}/bridge` → `{ worker_jwt, expires_in, api_base_url, worker_epoch }`
3. `createV2ReplTransport(worker_jwt, worker_epoch)` — SSE + CCRClient
4. `createTokenRefreshScheduler(scheduleFromExpiresIn)` — proactive `/bridge` re-call
5. On 401 SSE: rebuild transport with fresh `/bridge` credentials (same seq-num)

### EnvLessBridgeParams

```typescript
type EnvLessBridgeParams = {
  baseUrl: string
  orgUUID: string
  title: string
  getAccessToken: () => string | undefined
  onAuth401?: (staleAccessToken: string) => Promise<boolean>
  toSDKMessages: (messages: Message[]) => SDKMessage[]
  initialHistoryCap: number
  initialMessages?: Message[]
  onInboundMessage?: (msg: SDKMessage) => void | Promise<void>
  onUserMessage?: (text: string, sessionId: string) => boolean
  onPermissionResponse?: (response: SDKControlResponse) => void
  onInterrupt?: () => void
  onSetModel?: (model: string | undefined) => void
  onSetMaxThinkingTokens?: (maxTokens: number | null) => void
  onSetPermissionMode?: (mode: PermissionMode) => { ok: true } | { ok: false; error: string }
  onStateChange?: (state: BridgeState, detail?: string) => void
  perpetual?: boolean
}
```

### Connect Cause Telemetry

```typescript
type ConnectCause = 'initial' | 'proactive_refresh' | 'auth_401_recovery'
```

Sent with `tengu_bridge_repl_v2_ws_connected` analytics event.

---

## 9. Transport Layer

### ReplBridgeTransport Interface (`bridge/replBridgeTransport.ts`)

Abstracts over V1 (HybridTransport) and V2 (SSETransport+CCRClient):

```typescript
type ReplBridgeTransport = {
  write(message: StdoutMessage): Promise<void>
  writeBatch(messages: StdoutMessage[]): Promise<void>
  close(): void
  isConnectedStatus(): boolean
  getStateLabel(): string
  setOnData(callback: (data: string) => void): void
  setOnClose(callback: (closeCode?: number) => void): void
  setOnConnect(callback: () => void): void
  connect(): void
  getLastSequenceNum(): number    // V1 always returns 0; V2 returns SSE seq
  readonly droppedBatchCount: number  // V1 only; V2 always 0
  reportState(state: SessionState): void      // V2 only; V1 no-op
  reportMetadata(metadata: Record<string, unknown>): void  // V2 only
  reportDelivery(eventId: string, status: 'processing' | 'processed'): void  // V2 only
  flush(): Promise<void>          // V2 only; V1 resolves immediately
}
```

### `createV1ReplTransport(hybrid: HybridTransport): ReplBridgeTransport`

Thin no-op wrapper that delegates to `HybridTransport`. V1-specific behaviors:
- `getLastSequenceNum()` always returns 0
- `reportState()`, `reportMetadata()`, `reportDelivery()`, `flush()` are all no-ops

### `createV2ReplTransport(opts): Promise<ReplBridgeTransport>`

Options:
```typescript
{
  sessionUrl: string           // /v1/code/sessions/{id}
  ingressToken: string
  sessionId: string
  initialSequenceNum?: number  // SSE resume cursor
  epoch?: number               // If from /bridge, server already bumped epoch
  heartbeatIntervalMs?: number // Default: 20s
  heartbeatJitterFraction?: number
  outboundOnly?: boolean       // Skip SSE read stream (mirror mode)
  getAuthToken?: () => string | undefined  // Per-instance auth (multi-session safe)
}
```

**Close Codes used internally:**
- `4090` — epoch superseded (epoch mismatch from CCRClient)
- `4091` — CCR initialize() failure
- `4092` — SSE reconnect-budget exhaustion (mapped from `undefined`)

**Delivery ACK behavior:** Both `'received'` and `'processed'` are fired immediately on SSE event receipt to prevent phantom prompt flooding on restarts. This is a fix for the issue where `reconnectSession` re-queues prompts that haven't been ACK'd as `'processed'`.

### HybridTransport (`cli/transports/HybridTransport.ts`)

Extends `WebSocketTransport`. WebSocket for reads, HTTP POST for writes.

**Configuration:**
```typescript
BATCH_FLUSH_INTERVAL_MS = 100     // Accumulates stream_events before POST
POST_TIMEOUT_MS = 15_000           // Per-attempt timeout
CLOSE_GRACE_MS = 3000              // Grace period for queued writes on close
```

**Write flow:**
```
write(stream_event) ─┐
                     │ (100ms timer)
write(other) ──────► SerialBatchEventUploader.enqueue()
writeBatch() ───────┘     │
                          ▼ serial, batched, retries indefinitely
                     postOnce()  (single HTTP POST)
```

- `maxBatchSize`: 500
- `maxQueueSize`: 100,000
- `baseDelayMs`: 500, `maxDelayMs`: 8000, `jitterMs`: 1000

**Post URL:** Converts WebSocket URL to HTTP(S) POST endpoint (`convertWsUrlToPostUrl()`).

### WebSocketTransport (`cli/transports/WebSocketTransport.ts`)

Base WebSocket transport.

**Configuration:**
```typescript
DEFAULT_MAX_BUFFER_SIZE = 1000
DEFAULT_BASE_RECONNECT_DELAY = 1000
DEFAULT_MAX_RECONNECT_DELAY = 30_000
DEFAULT_RECONNECT_GIVE_UP_MS = 600_000   // 10 minutes
DEFAULT_PING_INTERVAL = 10_000
DEFAULT_KEEPALIVE_INTERVAL = 300_000     // 5 minutes
SLEEP_DETECTION_THRESHOLD_MS = 60_000   // 2× max reconnect delay
KEEP_ALIVE_FRAME = '{"type":"keep_alive"}\n'
```

**Permanent Close Codes** (no retry):
- `1002` — protocol error (session reaped)
- `4001` — session expired/not found
- `4003` — unauthorized

**Sleep detection:** If gap between reconnection attempts exceeds `60s`, resets the reconnection budget and retries (machine likely slept).

**States:** `'idle' | 'connected' | 'reconnecting' | 'closing' | 'closed'`

### SSETransport (`cli/transports/SSETransport.ts`)

Server-Sent Events transport.

**Configuration:**
```typescript
RECONNECT_BASE_DELAY_MS = 1000
RECONNECT_MAX_DELAY_MS = 30_000
RECONNECT_GIVE_UP_MS = 600_000    // 10 minutes
LIVENESS_TIMEOUT_MS = 45_000      // Server keepalives every 15s
PERMANENT_HTTP_CODES = {401, 403, 404}
POST_MAX_RETRIES = 10
POST_BASE_DELAY_MS = 500
POST_MAX_DELAY_MS = 8000
```

**SSE Frame Parsing:**

```typescript
type SSEFrame = {
  event?: string
  id?: string       // Used as sequence number for Last-Event-ID
  data?: string
}
```

Frames are double-newline delimited. Leading space after `:` is stripped per SSE spec. Comments (`:keepalive`) are ignored. Sequence numbers are tracked via `id` field.

**Exported for testing:** `parseSSEFrames(buffer: string): { frames: SSEFrame[]; remaining: string }`

**Sequence number carryover:** On reconnect, sends `Last-Event-ID` or `from_sequence_num` query param so the server resumes from where the old stream left off.

### SerialBatchEventUploader (`cli/transports/SerialBatchEventUploader.ts`)

```typescript
type SerialBatchEventUploaderConfig<T> = {
  maxBatchSize: number         // Max items per POST
  maxBatchBytes?: number       // Max serialized bytes per POST
  maxQueueSize: number         // Max pending items before enqueue() blocks
  send: (batch: T[]) => Promise<void>  // The actual HTTP call
  baseDelayMs: number
  maxDelayMs: number
  jitterMs: number
  maxConsecutiveFailures?: number   // After N failures, drop batch and advance
  onBatchDropped?: (batchSize, failures) => void
}
```

**`class RetryableError extends Error`**: Throw from `config.send()` to override exponential backoff with server-supplied `retryAfterMs` (e.g., for 429 responses).

**Backpressure:** `enqueue()` blocks when `maxQueueSize` is reached.

### WorkerStateUploader (`cli/transports/WorkerStateUploader.ts`)

Coalescing uploader for `PUT /worker` (session state + metadata).

- At most 1 in-flight PUT + 1 pending patch (never grows beyond 2 slots).
- Coalescing rules:
  - Top-level keys: last value wins.
  - `external_metadata` / `internal_metadata`: RFC 7396 merge (null values preserved for server-side delete).

### CCRClient (`cli/transports/ccrClient.ts`)

The CCR v2 write-side client.

**Configuration:**
```typescript
DEFAULT_HEARTBEAT_INTERVAL_MS = 20_000    // Server TTL: 60s
STREAM_EVENT_FLUSH_INTERVAL_MS = 100      // text_delta coalescing window
MAX_CONSECUTIVE_AUTH_FAILURES = 10        // ~200s at 20s heartbeat
```

**`class CCRInitError extends Error`**: Carries `reason: CCRInitFailReason`:
- `'no_auth_headers'`
- `'missing_epoch'`
- `'worker_register_failed'`

**Epoch mismatch (409 response)**: Triggers `onEpochMismatch()` callback, which closes CCRClient and SSETransport and fires `onCloseCb(4090)`.

**text_delta coalescing:** `stream_event` messages with `content_block_delta` / `text_delta` are accumulated in a per-message-ID buffer for `100ms`. Each emitted event is a complete self-contained snapshot of the text so far.

### Transport Selection (`cli/transports/transportUtils.ts`)

```typescript
getTransportForUrl(url, headers, sessionId, refreshHeaders): Transport
```

Priority:
1. `SSETransport` — when `CLAUDE_CODE_USE_CCR_V2` env is truthy
2. `HybridTransport` — when URL is `ws(s)://` AND `CLAUDE_CODE_POST_FOR_SESSION_INGRESS_V2` env is truthy
3. `WebSocketTransport` — default for `ws(s)://`
4. Throws for unsupported protocols

---

## 10. Message Protocol (bridgeMessaging.ts)

**File:** `bridge/bridgeMessaging.ts`

Pure functions shared by both V1 (`initBridgeCore`) and V2 (`initEnvLessBridgeCore`).

### Type Guards

#### `isSDKMessage(value: unknown): value is SDKMessage`
Checks for non-null object with string `type` field.

#### `isSDKControlResponse(value: unknown): value is SDKControlResponse`
Checks `type === 'control_response'` and has `'response'` field.

#### `isSDKControlRequest(value: unknown): value is SDKControlRequest`
Checks `type === 'control_request'`, has `'request_id'` and `'request'` fields.

#### `isEligibleBridgeMessage(m: Message): boolean`

Returns true for messages that should be forwarded to the bridge transport:
- `type === 'user'` (non-virtual)
- `type === 'assistant'` (non-virtual)
- `type === 'system'` with `subtype === 'local_command'`

#### `extractTitleText(m: Message): string | undefined`

Extracts title-worthy text from a user message. Filters out:
- Non-user messages
- `isMeta` messages
- Tool result messages
- Compact summary messages
- Non-human origins (`origin.kind !== 'human'`)
- Pure display-tag content (stripped via `stripDisplayTagsAllowEmpty`)

### `handleIngressMessage(data, recentPostedUUIDs, recentInboundUUIDs, onInboundMessage, onPermissionResponse?, onControlRequest?): void`

Parses and routes an ingress WebSocket message:
1. Parses JSON, normalizes control message keys.
2. Checks for `control_response` → calls `onPermissionResponse`.
3. Checks for `control_request` → calls `onControlRequest`.
4. Validates it's an `SDKMessage`.
5. Echo dedup: skips if UUID in `recentPostedUUIDs`.
6. Re-delivery dedup: skips if UUID in `recentInboundUUIDs`.
7. Only forwards `type === 'user'` messages to `onInboundMessage`.
8. All other message types are logged and ignored.

### Server Control Request Handling

`handleServerControlRequest(request, handlers): void`

Processes server-sent `control_request` messages. Must respond promptly (server kills WS after ~10-14s timeout).

**Supported subtypes:**

| Subtype | Behavior |
|---------|----------|
| `initialize` | Responds with `{ commands: [], output_style: 'normal', available_output_styles: ['normal'], models: [], account: {}, pid: process.pid }` |
| `set_model` | Calls `onSetModel(request.request.model)`, responds success |
| `set_max_thinking_tokens` | Calls `onSetMaxThinkingTokens(maxTokens)`, responds success |
| `set_permission_mode` | Calls `onSetPermissionMode(mode)`, responds success or error |
| `interrupt` | Calls `onInterrupt()`, responds success |
| unknown | Responds with error: "REPL bridge does not handle control_request subtype: ..." |

**Outbound-only mode**: All mutable requests respond with error `'This session is outbound-only...'`. `initialize` still responds success.

**Response envelope:**
```json
{
  "type": "control_response",
  "response": {
    "subtype": "success" | "error",
    "request_id": "<id>",
    ...
  },
  "session_id": "<sessionId>"
}
```

### `makeResultMessage(sessionId: string): SDKResultSuccess`

Builds a minimal result message for session archival:
```json
{
  "type": "result",
  "subtype": "success",
  "duration_ms": 0,
  "duration_api_ms": 0,
  "is_error": false,
  "num_turns": 0,
  "result": "",
  "stop_reason": null,
  "total_cost_usd": 0,
  "usage": {...},
  "modelUsage": {},
  "permission_denials": [],
  "session_id": "<sessionId>",
  "uuid": "<randomUUID>"
}
```

### `class BoundedUUIDSet`

FIFO-bounded ring buffer for UUID deduplication. O(capacity) memory.

```typescript
class BoundedUUIDSet {
  constructor(capacity: number)
  add(uuid: string): void   // Evicts oldest when at capacity
  has(uuid: string): boolean
  clear(): void
}
```

Used for:
- `recentPostedUUIDs` — echo suppression (messages we sent reflected back)
- `recentInboundUUIDs` — re-delivery dedup (server replays history after transport swap)

---

## 11. JWT Authentication (jwtUtils.ts)

**File:** `bridge/jwtUtils.ts`

### `decodeJwtPayload(token: string): unknown | null`

Decodes JWT payload segment without signature verification. Strips `sk-ant-si-` prefix if present. Returns parsed JSON or `null` on malformed input.

### `decodeJwtExpiry(token: string): number | null`

Extracts the `exp` Unix seconds claim from a JWT without verifying the signature. Returns `null` if unparseable.

### `createTokenRefreshScheduler(opts): { schedule, scheduleFromExpiresIn, cancel, cancelAll }`

**Options:**
```typescript
{
  getAccessToken: () => string | undefined | Promise<string | undefined>
  onRefresh: (sessionId: string, oauthToken: string) => void
  label: string
  refreshBufferMs?: number   // Default: TOKEN_REFRESH_BUFFER_MS = 5 min
}
```

**Constants:**
```typescript
TOKEN_REFRESH_BUFFER_MS = 5 * 60 * 1000      // 5 minutes before expiry
FALLBACK_REFRESH_INTERVAL_MS = 30 * 60 * 1000 // 30 minutes (fallback)
MAX_REFRESH_FAILURES = 3
REFRESH_RETRY_DELAY_MS = 60_000               // 1 minute
```

**Methods:**

`schedule(sessionId, token)`: Decodes `exp` from JWT, schedules refresh `(exp × 1000 - now - refreshBufferMs)` ms from now. If token has no decodable `exp` (e.g., OAuth token), preserves existing timer.

`scheduleFromExpiresIn(sessionId, expiresInSeconds)`: Schedules refresh using explicit TTL. Clamp to 30s floor: `max(expiresInSeconds × 1000 - refreshBufferMs, 30_000)`.

`cancel(sessionId)`: Clears timer, bumps generation to invalidate in-flight refreshes.

`cancelAll()`: Clears all timers and failure counters.

**Generation tracking:** Each session has a monotonic generation counter. `doRefresh()` checks that the generation hasn't changed before scheduling follow-up timers. This prevents orphaned timers when a session is cancelled while a refresh is in flight.

**Follow-up refresh:** After each successful refresh, schedules `FALLBACK_REFRESH_INTERVAL_MS` (30 min) follow-up to handle long-running sessions that outlast the first refresh window.

---

## 12. Session ID Compatibility (sessionIdCompat.ts)

**File:** `bridge/sessionIdCompat.ts`

Handles the V2 compat layer's `cse_*` ↔ `session_*` ID translation.

**Problem:** CCR V2 infra uses `cse_*` prefix internally; the compat gateway and client-facing API (`/v1/sessions`) expect `session_*`. Same UUID, different prefix.

### Exports

#### `setCseShimGate(gate: () => boolean): void`

Registers the GrowthBook gate `isCseShimEnabled`. Called from bridge init code that already imports `bridgeEnabled.ts`. The SDK bundle never calls this, so the shim defaults to active.

#### `toCompatSessionId(id: string): string`

Re-tags `cse_*` → `session_*` for compat API calls (`/v1/sessions/{id}`, `/archive`, `/events`). No-op for IDs that aren't `cse_*`. No-op when shim gate is off.

```
"cse_abc123" → "session_abc123"
"session_abc123" → "session_abc123"  (no-op)
```

#### `toInfraSessionId(id: string): string`

Inverse: re-tags `session_*` → `cse_*` for infrastructure calls (`/bridge/reconnect`). No-op for IDs that aren't `session_*`.

```
"session_abc123" → "cse_abc123"
"cse_abc123" → "cse_abc123"  (no-op)
```

---

## 13. Work Secrets & CCR v2 Registration (workSecret.ts)

**File:** `bridge/workSecret.ts`

### `decodeWorkSecret(secret: string): WorkSecret`

Decodes base64url-encoded work secret JSON. Validates:
- Must be a version-1 secret.
- `session_ingress_token` must be a non-empty string.
- `api_base_url` must be a string.

### `buildSdkUrl(apiBaseUrl: string, sessionId: string): string`

Builds V1 WebSocket URL:
- Localhost: `ws://host/v2/session_ingress/ws/{sessionId}` (direct to session-ingress)
- Production: `wss://host/v1/session_ingress/ws/{sessionId}` (Envoy rewrites `/v1/` → `/v2/`)

### `sameSessionId(a: string, b: string): boolean`

Compares two session IDs regardless of prefix (`cse_` vs `session_`). Compares the UUID body (everything after the last `_`). Requires body length ≥ 4 to avoid false matches on malformed IDs.

### `buildCCRv2SdkUrl(apiBaseUrl: string, sessionId: string): string`

Builds V2 HTTP(S) session URL: `{apiBaseUrl}/v1/code/sessions/{sessionId}`

### `registerWorker(sessionUrl: string, accessToken: string): Promise<number>`

`POST {sessionUrl}/worker/register` to register as the CCR worker. Returns `worker_epoch`. The epoch is serialized as `int64` which may be returned as a string by protojson — handles both `string` and `number` forms.

---

## 14. Bridge Pointer: Crash Recovery (bridgePointer.ts)

**File:** `bridge/bridgePointer.ts`

### Purpose

Crash-recovery pointer written after session creation, refreshed periodically, cleared on clean shutdown. On next startup, `claude remote-control` detects stale pointers and offers to resume.

### Constants

```typescript
BRIDGE_POINTER_TTL_MS = 4 * 60 * 60 * 1000   // 4 hours (matches Redis BRIDGE_LAST_POLL_TTL)
MAX_WORKTREE_FANOUT = 50
```

### BridgePointer Schema

```typescript
type BridgePointer = {
  sessionId: string
  environmentId: string
  source: 'standalone' | 'repl'
}
```

**File location:** `{projectsDir}/{sanitizedDir}/bridge-pointer.json`

### Exports

#### `getBridgePointerPath(dir: string): string`

Returns the absolute path to the bridge pointer file for a given working directory.

#### `writeBridgePointer(dir, pointer): Promise<void>`

Writes pointer atomically. Also used to refresh mtime (same-content writes). Best-effort — logs and swallows errors.

#### `readBridgePointer(dir): Promise<(BridgePointer & { ageMs: number }) | null>`

Reads the pointer. Returns `null` if:
- File does not exist
- JSON is malformed
- Schema validation fails
- `mtime` is more than 4 hours ago (stale)

Stale/invalid pointers are automatically deleted.

#### `readBridgePointerAcrossWorktrees(dir): Promise<{ pointer, dir } | null>`

Worktree-aware read for `--continue`. Fast-path checks the given dir first. If not found, fans out to git worktree siblings (via `getWorktreePathsPortable()`) in parallel (capped at 50). Returns the freshest pointer and its directory.

#### `clearBridgePointer(dir): Promise<void>`

Deletes the pointer file. Idempotent (ENOENT is expected on clean shutdown).

---

## 15. Permission Callbacks (bridgePermissionCallbacks.ts)

**File:** `bridge/bridgePermissionCallbacks.ts`

### Types

```typescript
type BridgePermissionResponse = {
  behavior: 'allow' | 'deny'
  updatedInput?: Record<string, unknown>
  updatedPermissions?: PermissionUpdate[]
  message?: string
}

type BridgePermissionCallbacks = {
  sendRequest(
    requestId, toolName, input, toolUseId, description,
    permissionSuggestions?, blockedPath?
  ): void
  sendResponse(requestId, response: BridgePermissionResponse): void
  cancelRequest(requestId): void
  onResponse(
    requestId,
    handler: (response: BridgePermissionResponse) => void
  ): () => void  // returns unsubscribe function
}
```

### `isBridgePermissionResponse(value: unknown): value is BridgePermissionResponse`

Type predicate. Checks that `value.behavior === 'allow' || value.behavior === 'deny'`.

---

## 16. Inbound Messages & Attachments

### inboundAttachments.ts

Resolves `file_uuid` attachments from inbound bridge user messages.

**Flow:**
1. Web composer uploads via `/api/{org}/upload` (cookie-auth).
2. Bridge receives `file_attachments: [{ file_uuid, file_name }]` on the user message.
3. `resolveInboundAttachments()` fetches each file via `GET /api/oauth/files/{uuid}/content`.
4. Downloads to `~/.claude/uploads/{sessionId}/{uuid-prefix}-{sanitizedName}`.
5. Returns `@"path"` prefix string to prepend to message content.

**`DOWNLOAD_TIMEOUT_MS = 30_000`**

**File path sanitization:** `sanitizeFileName()` strips path components and replaces non-alphanumeric chars except `._-` with `_`.

**Prefix format:** 8 chars from `file_uuid` (or random UUID), e.g. `@"abc12345-filename.pdf" `.

**Exports:**
```typescript
extractInboundAttachments(msg: unknown): InboundAttachment[]
resolveInboundAttachments(attachments: InboundAttachment[]): Promise<string>
prependPathRefs(
  content: string | Array<ContentBlockParam>,
  prefix: string,
): string | Array<ContentBlockParam>
resolveAndPrepend(
  msg: unknown,
  content: string | Array<ContentBlockParam>,
): Promise<string | Array<ContentBlockParam>>
```

`prependPathRefs()` targets the **last** text block in a content array (because `processUserInputBase` reads the last block).

### inboundMessages.ts

**`extractInboundMessageFields(msg: SDKMessage): { content, uuid } | undefined`**

Extracts content and UUID from a user message. Normalizes image blocks:
- Converts camelCase `mediaType` → `media_type` (mobile app compatibility fix for `mobile-apps#5825`)
- Detects missing `media_type` via `detectImageFormatFromBase64()`

**`normalizeImageBlocks(blocks: ContentBlockParam[]): ContentBlockParam[]`**

Fast-path: returns original reference if no malformed blocks. Only allocates on the fix-needed path.

---

## 17. Session Runner (sessionRunner.ts)

**File:** `bridge/sessionRunner.ts`

Spawns child Claude CLI processes for bridge sessions.

### Constants

```typescript
MAX_ACTIVITIES = 10      // Ring buffer size for activity history
MAX_STDERR_LINES = 10    // Ring buffer size for stderr
```

### `safeFilenameId(id: string): string`

Sanitizes session IDs for use in file names. Replaces non-alphanumeric chars (except `_-`) with underscores.

### PermissionRequest

Message emitted by child CLI on stdout when it needs permission:

```typescript
type PermissionRequest = {
  type: 'control_request'
  request_id: string
  request: {
    subtype: 'can_use_tool'
    tool_name: string
    input: Record<string, unknown>
    tool_use_id: string
  }
}
```

### SessionSpawnerDeps

```typescript
type SessionSpawnerDeps = {
  execPath: string
  scriptArgs: string[]        // Empty for compiled binaries; [process.argv[1]] for npm
  env: NodeJS.ProcessEnv
  verbose: boolean
  sandbox: boolean
  debugFile?: string
  permissionMode?: string
  onDebug: (msg: string) => void
  onActivity?: (sessionId, activity) => void
  onPermissionRequest?: (sessionId, request, accessToken) => void
}
```

### Tool Activity Verbs

```typescript
const TOOL_VERBS = {
  Read: 'Reading', Write: 'Writing', Edit: 'Editing', MultiEdit: 'Editing',
  Bash: 'Running', Glob: 'Searching', Grep: 'Searching',
  WebFetch: 'Fetching', WebSearch: 'Searching', Task: 'Running task',
  FileReadTool: 'Reading', FileWriteTool: 'Writing', FileEditTool: 'Editing',
  GlobTool: 'Searching', GrepTool: 'Searching', BashTool: 'Running',
  NotebookEditTool: 'Editing notebook', LSP: 'LSP',
}
```

---

## 18. Bridge Debug & Fault Injection (bridgeDebug.ts)

**File:** `bridge/bridgeDebug.ts`

Ant-only fault injection for testing bridge recovery paths. Zero overhead in external builds.

### BridgeFault

```typescript
type BridgeFault = {
  method: 'pollForWork' | 'registerBridgeEnvironment' | 'reconnectSession' | 'heartbeatWork'
  kind: 'fatal' | 'transient'
  status: number
  errorType?: string
  count: number          // Decremented on consume; removed at 0
}
```

- **fatal**: Throws `BridgeFatalError` — triggers environment teardown.
- **transient**: Throws a plain `Error` (mimics 5xx/network) — triggers retry/backoff.

### BridgeDebugHandle

```typescript
type BridgeDebugHandle = {
  fireClose: (code: number) => void       // Invoke transport permanent-close handler
  forceReconnect: () => void              // Call reconnectEnvironmentWithSession()
  injectFault: (fault: BridgeFault) => void
  wakePollLoop: () => void                // Abort at-capacity sleep immediately
  describe: () => string                  // "envId=... sessionId=..."
}
```

### Exports

```typescript
registerBridgeDebugHandle(h: BridgeDebugHandle): void
clearBridgeDebugHandle(): void
getBridgeDebugHandle(): BridgeDebugHandle | null
injectBridgeFault(fault: BridgeFault): void
wrapApiForFaultInjection(api: BridgeApiClient): BridgeApiClient
```

`wrapApiForFaultInjection()` wraps `pollForWork`, `registerBridgeEnvironment`, `reconnectSession`, and `heartbeatWork` with fault queue checks. All other methods pass through unchanged.

---

## 19. Bridge Utilities

### debugUtils.ts

```typescript
redactSecrets(s: string): string
```

Redacts sensitive field values matching: `session_ingress_token`, `environment_secret`, `access_token`, `secret`, `token`. Values shorter than 16 chars → `[REDACTED]`. Longer: `first8chars...last4chars`.

```typescript
debugTruncate(s: string): string          // 2000 char limit, collapses newlines
debugBody(data: unknown): string          // Serialize + redact + truncate
describeAxiosError(err: unknown): string  // Extracts server message from axios errors
extractHttpStatus(err: unknown): number | undefined
extractErrorDetail(data: unknown): string | undefined  // Checks data.message, data.error.message
logBridgeSkip(reason, debugMsg?, v2?): void  // Logs analytics + debug for bridge skip
```

### bridgeStatusUtil.ts

```typescript
type StatusState = 'idle' | 'attached' | 'titled' | 'reconnecting' | 'failed'
TOOL_DISPLAY_EXPIRY_MS = 30_000   // How long a tool activity stays visible
SHIMMER_INTERVAL_MS = 150         // Shimmer animation tick

timestamp(): string                // "HH:MM:SS"
formatDuration(ms): string         // re-exported from utils/format.ts
truncatePrompt(s, width): string   // re-exported
abbreviateActivity(summary): string  // Truncates to 30 chars

buildBridgeConnectUrl(environmentId, ingressUrl?): string
  // → "{baseUrl}/code?bridge={environmentId}"

buildBridgeSessionUrl(sessionId, environmentId, ingressUrl?): string
  // → "{remoteSessionUrl}?bridge={environmentId}"

computeGlimmerIndex(tick, messageWidth): number
computeShimmerSegments(text, glimmerIndex): { before, shimmer, after }

getBridgeStatus({ error, connected, sessionActive, reconnecting }): BridgeStatusInfo
  // Returns { label, color } for UI rendering

buildIdleFooterText(url): string
buildActiveFooterText(url): string
FAILED_FOOTER_TEXT = 'Something went wrong, please try again'

wrapWithOsc8Link(text, url): string   // OSC 8 terminal hyperlink
```

### capacityWake.ts

```typescript
type CapacitySignal = { signal: AbortSignal; cleanup: () => void }
type CapacityWake = {
  signal(): CapacitySignal  // Merged abort: outer loop OR capacity wake
  wake(): void              // Abort current sleep, arm fresh controller
}

createCapacityWake(outerSignal: AbortSignal): CapacityWake
```

Shared primitive for both `replBridge.ts` and `bridgeMain.ts` to sleep while at-capacity and wake early when a session ends or the outer signal aborts.

### flushGate.ts

```typescript
class FlushGate<T> {
  get active(): boolean
  get pendingCount(): number
  start(): void                  // Mark flush in-progress; enqueue() queues items
  end(): T[]                     // End flush, return queued items
  enqueue(...items: T[]): boolean  // Queue if active, else returns false
  drop(): number                 // Discard all (permanent close); returns count dropped
  deactivate(): void             // Clear active without dropping (transport replacement)
}
```

Used during initial history flush to queue new messages that arrive concurrently, preventing server-side interleaving.

### pollConfig.ts / pollConfigDefaults.ts

`getPollIntervalConfig(): PollIntervalConfig` — reads from GrowthBook `tengu_bridge_poll_interval_config` with 5-minute refresh. Falls back to defaults on schema violation.

```typescript
type PollIntervalConfig = {
  poll_interval_ms_not_at_capacity: number        // Default: 2000ms
  poll_interval_ms_at_capacity: number            // Default: 600,000ms (10 min)
  non_exclusive_heartbeat_interval_ms: number     // Default: 0 (disabled)
  multisession_poll_interval_ms_not_at_capacity: number  // Default: 2000ms
  multisession_poll_interval_ms_partial_capacity: number // Default: 2000ms
  multisession_poll_interval_ms_at_capacity: number      // Default: 600,000ms
  reclaim_older_than_ms: number                   // Default: 5000ms
  session_keepalive_interval_v2_ms: number        // Default: 120,000ms
}
```

**Validation rules:**
- `poll_interval_ms_*` and `multisession_*` fields: min 100ms.
- `non_exclusive_heartbeat_interval_ms`: min 0 (0 = disabled).
- At-capacity intervals: 0 (disabled) or ≥100ms (1-99 rejected to prevent confusion with seconds).
- Object-level: at least one at-capacity liveness mechanism must be enabled (heartbeat > 0 OR at-capacity poll > 0).

### envLessBridgeConfig.ts

`getEnvLessBridgeConfig(): Promise<EnvLessBridgeConfig>` — reads from GrowthBook `tengu_bridge_repl_v2_config`. Defaults:

```typescript
DEFAULT_ENV_LESS_BRIDGE_CONFIG = {
  init_retry_max_attempts: 3,
  init_retry_base_delay_ms: 500,
  init_retry_jitter_fraction: 0.25,
  init_retry_max_delay_ms: 4000,
  http_timeout_ms: 10_000,
  uuid_dedup_buffer_size: 2000,
  heartbeat_interval_ms: 20_000,
  heartbeat_jitter_fraction: 0.1,
  token_refresh_buffer_ms: 300_000,
  teardown_archive_timeout_ms: 1500,
  connect_timeout_ms: 15_000,
  min_version: '0.0.0',
  should_show_app_upgrade_message: false,
}
```

Validation ranges:
- `init_retry_max_attempts`: 1–10
- `http_timeout_ms`: min 2000ms
- `heartbeat_interval_ms`: 5000–30,000ms
- `heartbeat_jitter_fraction`: 0–0.5
- `token_refresh_buffer_ms`: 30,000–1,800,000ms
- `teardown_archive_timeout_ms`: 500–2000ms
- `connect_timeout_ms`: 5,000–60,000ms

### trustedDevice.ts

Manages the trusted device token for ELEVATED security tier bridge sessions.

```typescript
getTrustedDeviceToken(): string | undefined
clearTrustedDeviceTokenCache(): void
clearTrustedDeviceToken(): void
enrollTrustedDevice(): Promise<void>
```

**Gate:** `tengu_sessions_elevated_auth_enforcement`. When gate is off, `getTrustedDeviceToken()` returns `undefined` unconditionally.

**Storage:** macOS keychain via `getSecureStorage()` (memoized — keychain spawn is ~40ms).

**Token precedence:** `CLAUDE_TRUSTED_DEVICE_TOKEN` env var > keychain.

**Enrollment:** `POST /api/auth/trusted_devices` with display name `"Claude Code on {hostname()} · {platform}"`. Must be called within 10 minutes of login. Best-effort — never throws. On success, persists to keychain and clears memo cache.

### codeSessionApi.ts

Thin HTTP wrappers for CCR V2 code-session API.

```typescript
createCodeSession(
  baseUrl, accessToken, title, timeoutMs, tags?
): Promise<string | null>
// POST /v1/code/sessions → session.id (cse_*)
// Body: { title, bridge: {}, tags?: [...] }

type RemoteCredentials = {
  worker_jwt: string
  api_base_url: string
  expires_in: number         // Seconds
  worker_epoch: number
}

fetchRemoteCredentials(
  sessionId, baseUrl, accessToken, timeoutMs, trustedDeviceToken?
): Promise<RemoteCredentials | null>
// POST /v1/code/sessions/{id}/bridge
```

`worker_epoch` is parsed defensively (protojson may return int64 as string).

### sessionRunner.ts (full createSessionSpawner)

The `createSessionSpawner()` function (referenced from `bridgeMain.ts`) creates a `SessionSpawner` that:
1. Calls `spawn(child_process)` with the Claude binary and appropriate flags.
2. Parses child stdout as NDJSON, routing `control_request` messages to permission callbacks.
3. Tracks `SessionActivity` in a ring buffer of size 10.
4. Tracks last 10 stderr lines.
5. Watches child exit to resolve `handle.done` with `'completed'` (exit 0) or `'failed'` (exit != 0) or `'interrupted'` (SIGTERM/SIGKILL).

---

## 20. CLI Framework

### cli/exit.ts

```typescript
cliError(msg?: string): never   // stderr + process.exit(1)
cliOk(msg?: string): never      // stdout + process.exit(0)
```

Centralized CLI exit helpers. `cliError` uses `console.error`; `cliOk` uses `process.stdout.write`. The `never` return type allows TypeScript to narrow control flow at call sites.

### cli/ndjsonSafeStringify.ts

```typescript
ndjsonSafeStringify(value: unknown): string
```

JSON serializer that escapes `U+2028` (LINE SEPARATOR) and `U+2029` (PARAGRAPH SEPARATOR) as `\u2028`/`\u2029`. These are valid line terminators in JavaScript (ECMA-262 §11.3) and would break line-splitting NDJSON receivers. The escaped form is still valid JSON.

### cli/handlers/auth.ts

```typescript
installOAuthTokens(tokens: OAuthTokens): Promise<void>
authLogin({ email?, sso?, console?, claudeai? }): Promise<void>
authStatus({ json?, text? }): Promise<void>
authLogout(): Promise<void>
```

**`installOAuthTokens()`** performs post-token acquisition:
1. `performLogout({ clearOnboarding: false })` — clears old state
2. Fetches OAuth profile or falls back to `tokenAccount`
3. Calls `storeOAuthAccountInfo()`
4. `saveOAuthTokensIfNeeded()` + `clearOAuthTokenCache()`
5. `fetchAndStoreUserRoles()` (best-effort)
6. For claude.ai auth: `fetchAndStoreClaudeCodeFirstTokenDate()` (best-effort)
7. For Console auth: `createAndStoreApiKey()` (required — throws if fails)
8. `clearAuthRelatedCaches()`

**`authLogin()`** fast path: When `CLAUDE_CODE_OAUTH_REFRESH_TOKEN` env var is set, exchanges directly via `refreshOAuthToken()`, skipping browser OAuth flow. Requires `CLAUDE_CODE_OAUTH_SCOPES` env var (space-separated scopes).

**`authStatus()`** JSON output fields:
```json
{
  "loggedIn": boolean,
  "authMethod": "none" | "claude.ai" | "api_key_helper" | "oauth_token" | "api_key" | "third_party",
  "apiProvider": string,
  "apiKeySource": string,       // Present when apiKey
  "email": string | null,       // Present when claude.ai
  "orgId": string | null,
  "orgName": string | null,
  "subscriptionType": string | null
}
```

### cli/handlers/agents.ts

```typescript
agentsHandler(): Promise<void>
```

Lists configured agents grouped by source. Output format: `agentType · model · memory` per agent. Shows shadowed agents (overridden by a higher-priority source) with `(shadowed by {source})` prefix.

### cli/handlers/autoMode.ts

```typescript
autoModeDefaultsHandler(): void      // Dumps default auto mode rules as JSON
autoModeConfigHandler(): void        // Dumps effective config (user settings OR defaults)
autoModeCritiqueHandler({ model? }): Promise<void>  // AI critique of user rules
```

**Auto mode rule categories:** `allow`, `soft_deny`, `environment`.

**Critique uses `sideQuery()`** with a dedicated system prompt that asks Claude to evaluate rules for clarity, completeness, conflicts, and actionability.

### cli/handlers/mcp.tsx (partial)

```typescript
mcpServeHandler({ debug?, verbose? }): Promise<void>
mcpRemoveHandler(name, { scope? }): Promise<void>
// ... plus add, get, list, reset, import, desktop-import handlers
```

### cli/handlers/plugins.ts (partial)

Handlers for `claude plugin *` and marketplace commands:
- `installPlugin`, `uninstallPlugin`, `enablePlugin`, `disablePlugin`
- `listPlugins`, `marketplaceSearch`, `addMarketplace`, `removeMarketplace`

### cli/print.ts

The main SDK `-p` (print mode) handler. Orchestrates:
- `StructuredIO` / `RemoteIO` for I/O
- Tool pool assembly
- Message queue management
- Session state notifications
- Optional bridge enablement via `enableRemoteControl`

### cli/update.ts

```typescript
update(): Promise<void>
```

Handles `claude update`. Checks current version, detects install type, selects updater (npm global, native binary, local). Shows warnings for multiple installations.

---

## 21. CLI Transports

See [Section 9 — Transport Layer](#9-transport-layer) for the transport hierarchy.

### cli/remoteIO.ts — RemoteIO class

```typescript
class RemoteIO extends StructuredIO {
  constructor(streamUrl: string, initialPrompt?, replayUserMessages?)
}
```

Bidirectional streaming for SDK mode. Extends `StructuredIO`.

**Constructor behavior:**
1. Creates `PassThrough` input stream.
2. Reads `CLAUDE_CODE_SESSION_ACCESS_TOKEN` for initial auth headers.
3. Reads `CLAUDE_CODE_ENVIRONMENT_RUNNER_VERSION` for `x-environment-runner-version` header.
4. Creates `refreshHeaders` closure that re-reads the token dynamically on reconnects.
5. Calls `getTransportForUrl()` to get transport (WS, Hybrid, or SSE).

**Keep-alive:** When `session_keepalive_interval_v2_ms > 0` and transport is SSE/v2, sends silent `{type:'keep_alive'}` frames at that interval to prevent upstream proxy idle timeouts.

**State/metadata listeners:** Wires `setSessionStateChangedListener` and `setSessionMetadataChangedListener` to propagate `SessionState` changes to the CCRClient via `reportState()` and `reportMetadata()`.

**Command lifecycle:** Wires `setCommandLifecycleListener` to fire `reportDelivery('processing')` on command start and `reportDelivery('processed')` on command end.

### cli/structuredIO.ts — StructuredIO class

```typescript
class StructuredIO {
  constructor(inputStream: Readable, replayUserMessages?)
  // Handles SDK control message parsing (control_request/control_response)
  // Permission handling via can_use_tool protocol
  // Elicitation dialog support
  // Hook system integration
}

const SANDBOX_NETWORK_ACCESS_TOOL_NAME = 'SandboxNetworkAccess'
```

`StructuredIO` is the base class providing:
- NDJSON message deserialization from stdin
- `can_use_tool` permission request handling
- `control_response` dispatching
- Elicitation dialog flow (`SDKControlElicitationResponseSchema`)
- Hook execution before permission decisions

---

## 22. Remote Session System

### remote/SessionsWebSocket.ts — SessionsWebSocket

WebSocket client for viewing sessions via `/v1/sessions/ws/{id}/subscribe`.

**Connection Protocol:**
1. Connect to `wss://api.anthropic.com/v1/sessions/ws/{sessionId}/subscribe?organization_uuid={orgUuid}`
2. Send auth message:
   ```json
   { "type": "auth", "credential": { "type": "oauth", "token": "<accessToken>" } }
   ```
3. Receive `SessionsMessage` stream.

**Configuration:**
```typescript
RECONNECT_DELAY_MS = 2000
MAX_RECONNECT_ATTEMPTS = 5
PING_INTERVAL_MS = 30000
MAX_SESSION_NOT_FOUND_RETRIES = 3   // 4001 can be transient during compaction
PERMANENT_CLOSE_CODES = { 4003 }    // unauthorized — stops reconnecting
```

**Note:** `4001` (session not found) is handled separately with up to 3 limited retries because compaction can briefly cause false "not found" responses.

**Callbacks:**
```typescript
type SessionsWebSocketCallbacks = {
  onMessage: (message: SessionsMessage) => void
  onClose?: () => void          // Permanent close only
  onError?: (error: Error) => void
  onConnected?: () => void
  onReconnecting?: () => void   // Transient drop with reconnect scheduled
}
```

**Message types accepted:** Any object with a string `type` field (open-ended to avoid dropping new server message types).

### remote/RemoteSessionManager.ts — RemoteSessionManager

Coordinates WebSocket subscription + HTTP POST + permission flow.

```typescript
type RemoteSessionConfig = {
  sessionId: string
  getAccessToken: () => string
  orgUuid: string
  hasInitialPrompt?: boolean
  viewerOnly?: boolean    // Pure viewer: no interrupt, no title update, no 60s reconnect timeout
}

type RemotePermissionResponse =
  | { behavior: 'allow'; updatedInput: Record<string, unknown> }
  | { behavior: 'deny'; message: string }
```

**Callbacks:**
```typescript
type RemoteSessionCallbacks = {
  onMessage: (message: SDKMessage) => void
  onPermissionRequest: (request, requestId) => void
  onPermissionCancelled?: (requestId, toolUseId) => void
  onConnected?: () => void
  onDisconnected?: () => void
  onReconnecting?: () => void
  onError?: (error: Error) => void
}
```

### remote/remotePermissionBridge.ts

```typescript
createSyntheticAssistantMessage(
  request: SDKControlPermissionRequest,
  requestId: string,
): AssistantMessage
```

Creates a synthetic `AssistantMessage` wrapping a remote `tool_use` for the permission dialog. Uses a fake message ID `remote-{requestId}` and empty usage stats.

```typescript
createToolStub(toolName: string): Tool
```

Creates a minimal `Tool` stub for tools unknown to the local CLI (e.g., MCP tools running on CCR). Routes to `FallbackPermissionRequest`. The stub's `renderToolUseMessage()` shows up to 3 input key-value pairs.

### remote/sdkMessageAdapter.ts

Converts `SDKMessage` from CCR to REPL `Message` types.

```typescript
type ConvertedMessage =
  | { type: 'message'; message: Message }
  | { type: 'stream_event'; event: StreamEvent }
  | { type: 'ignored' }

type ConvertOptions = {
  convertToolResults?: boolean          // For direct-connect mode
  convertUserTextMessages?: boolean     // For historical event conversion
}

convertSDKMessage(msg: SDKMessage, opts?: ConvertOptions): ConvertedMessage
isSessionEndMessage(msg: SDKMessage): boolean     // msg.type === 'result'
isSuccessResult(msg: SDKResultMessage): boolean   // msg.subtype === 'success'
getResultText(msg: SDKResultMessage): string | null
```

**Conversion rules:**

| SDKMessage type | Converted to |
|-----------------|-------------|
| `assistant` | `AssistantMessage` |
| `user` (tool_result, convertToolResults=true) | `UserMessage` |
| `user` (text, convertUserTextMessages=true) | `UserMessage` |
| `user` (other) | `ignored` |
| `stream_event` | `StreamEvent` |
| `result` (success) | `ignored` |
| `result` (error) | `SystemMessage` (warning) |
| `system` (init) | `SystemMessage` (info): "Remote session initialized (model: ...)" |
| `system` (status: compacting) | `SystemMessage` (info): "Compacting conversation…" |
| `system` (compact_boundary) | `SystemMessage` (compact_boundary) |
| `tool_progress` | `SystemMessage` (info): "Tool {name} running for {n}s…" |
| `auth_status`, `tool_use_summary`, `rate_limit_event` | `ignored` |
| unknown | `ignored` (logged) |

---

## 23. replLauncher.tsx

**File:** `src/replLauncher.tsx`

```typescript
type AppWrapperProps = {
  getFpsMetrics: () => FpsMetrics | undefined
  stats?: StatsStore
  initialState: AppState
}

launchRepl(
  root: Root,
  appProps: AppWrapperProps,
  replProps: REPLProps,
  renderAndRun: (root: Root, element: React.ReactNode) => Promise<void>,
): Promise<void>
```

Lazy-loads `App` and `REPL` components and renders them wrapped in the React tree. Used by `main.tsx` to defer loading of the heavy UI component tree (App.js + REPL.js) until it's actually needed for interactive mode.

---

## 24. Configuration Defaults & GrowthBook Flags

### GrowthBook Feature Flags (Bridge)

| Flag | Type | Default | Purpose |
|------|------|---------|---------|
| `tengu_ccr_bridge` | boolean | `false` | Master gate: enables Remote Control |
| `tengu_bridge_repl_v2` | boolean | `false` | Enables env-less (V2) REPL bridge |
| `tengu_bridge_repl_v2_cse_shim_enabled` | boolean | `true` | `cse_*` → `session_*` retag shim |
| `tengu_bridge_min_version` | DynamicConfig `{minVersion}` | `'0.0.0'` | Min CLI version for V1 bridge |
| `tengu_bridge_repl_v2_config` | DynamicConfig (EnvLessBridgeConfig) | See defaults | V2 bridge timing config |
| `tengu_bridge_poll_interval_config` | DynamicConfig (PollIntervalConfig) | See defaults | Poll intervals |
| `tengu_ccr_bridge_multi_session` | boolean | N/A | Enables multi-session spawn modes |
| `tengu_sessions_elevated_auth_enforcement` | boolean | `false` | Enables trusted device requirement |
| `tengu_cobalt_harbor` | boolean | `false` | Auto-connect CCR on startup |
| `tengu_ccr_mirror` | boolean | `false` | CCR mirror mode |

### Build Flags (bun:bundle features)

| Flag | Purpose |
|------|---------|
| `BRIDGE_MODE` | Enables bridge-related code paths |
| `CCR_AUTO_CONNECT` | Enables `getCcrAutoConnectDefault()` |
| `CCR_MIRROR` | Enables `isCcrMirrorEnabled()` |
| `BASH_CLASSIFIER` | Enables bash classifier reason serialization |
| `TRANSCRIPT_CLASSIFIER` | Enables transcript classifier reason serialization |

### Environment Variables (Bridge)

| Variable | Purpose |
|----------|---------|
| `CLAUDE_BRIDGE_OAUTH_TOKEN` | Ant-only: override OAuth token for bridge |
| `CLAUDE_BRIDGE_BASE_URL` | Ant-only: override API base URL |
| `CLAUDE_TRUSTED_DEVICE_TOKEN` | Override trusted device token from env |
| `CLAUDE_CODE_USE_CCR_V2` | Use SSETransport + CCRClient for all sessions |
| `CLAUDE_CODE_POST_FOR_SESSION_INGRESS_V2` | Use HybridTransport (WS reads + POST writes) |
| `CLAUDE_CODE_CCR_MIRROR` | Enable CCR mirror mode |
| `CLAUDE_CODE_SESSION_ACCESS_TOKEN` | Session ingress auth token |
| `CLAUDE_CODE_ENVIRONMENT_RUNNER_VERSION` | Sent as `x-environment-runner-version` header |
| `CLAUDE_CODE_OAUTH_REFRESH_TOKEN` | Fast-path login via token exchange |
| `CLAUDE_CODE_OAUTH_SCOPES` | Required when `CLAUDE_CODE_OAUTH_REFRESH_TOKEN` is set |

---

## 25. Complete API Endpoint Reference

### Environments API (`/v1/environments/bridge`)

All environment API calls require:
- `anthropic-version: 2023-06-01`
- `anthropic-beta: environments-2025-11-01`
- `x-environment-runner-version: <version>`
- `Authorization: Bearer <token>`
- Optional: `X-Trusted-Device-Token: <token>`

#### POST `/v1/environments/bridge`

Register a bridge environment.

**Request:**
```json
{
  "machine_name": "hostname",
  "directory": "/path/to/dir",
  "branch": "main",
  "git_repo_url": "https://github.com/owner/repo",
  "max_sessions": 4,
  "metadata": { "worker_type": "claude_code" },
  "environment_id": "<backend-issued-id>"   // Optional: for re-registration
}
```

**Response:**
```json
{
  "environment_id": "env_abc123",
  "environment_secret": "secret_xyz"
}
```

**Timeout:** 15s

#### GET `/v1/environments/{environmentId}/work/poll`

Poll for new work. Auth: `environmentSecret`.

**Query params:**
- `reclaim_older_than_ms` (optional): Reclaim unacknowledged work older than this

**Response:** `WorkResponse | null` (null = no work available)

**Timeout:** 10s

#### POST `/v1/environments/{environmentId}/work/{workId}/ack`

Acknowledge a work item. Auth: `sessionToken` (session ingress JWT).

**Timeout:** 10s

#### POST `/v1/environments/{environmentId}/work/{workId}/heartbeat`

Send heartbeat. Auth: `sessionToken` (session ingress JWT).

**Response:**
```json
{
  "lease_extended": true,
  "state": "running",
  "last_heartbeat": "2025-03-31T...",
  "ttl_seconds": 300
}
```

**Timeout:** 10s

#### POST `/v1/environments/{environmentId}/work/{workId}/stop`

Stop a work item. Auth: OAuth token.

**Request:** `{ "force": true|false }`

**Timeout:** 10s

#### DELETE `/v1/environments/bridge/{environmentId}`

Deregister environment. Auth: OAuth token.

**Timeout:** 10s

#### POST `/v1/environments/{environmentId}/bridge/reconnect`

Force re-dispatch a session. Auth: OAuth token.

**Request:** `{ "session_id": "cse_abc123" }`

**Timeout:** 10s

### Sessions API (`/v1/sessions`)

All calls require `anthropic-beta: ccr-byoc-2025-07-29` and `x-organization-uuid: <orgUuid>`.

#### POST `/v1/sessions`

Create a session.

**Request:**
```json
{
  "title": "My Session",
  "events": [{ "type": "event", "data": <SDKMessage> }],
  "session_context": {
    "sources": [{ "type": "git_repository", "url": "...", "revision": "main" }],
    "outcomes": [...],
    "model": "claude-opus-4"
  },
  "environment_id": "env_abc123",
  "source": "remote-control",
  "permission_mode": "auto"
}
```

**Response:** `{ "id": "session_abc123" }`

#### GET `/v1/sessions/{sessionId}`

Fetch session metadata. Returns `{ environment_id?, title? }`.

#### PATCH `/v1/sessions/{sessionId}`

Update session title. **Request:** `{ "title": "New Title" }`

#### POST `/v1/sessions/{sessionId}/archive`

Archive a session. Returns `409` if already archived (idempotent).

#### POST `/v1/sessions/{sessionId}/events`

Send events to a session (used for permission responses). Auth: session ingress token.

**Request:**
```json
{
  "events": [
    {
      "type": "control_response",
      "response": {
        "subtype": "success",
        "request_id": "req_abc",
        "response": { "behavior": "allow" }
      }
    }
  ]
}
```

### CCR V2 Code Sessions API

#### POST `/v1/code/sessions`

**Request:**
```json
{
  "title": "Bridge Session",
  "bridge": {},
  "tags": ["optional", "tags"]
}
```

**Response:** `{ "session": { "id": "cse_abc123" } }`

#### POST `/v1/code/sessions/{sessionId}/bridge`

Register as bridge worker and get JWT.

**Optional header:** `X-Trusted-Device-Token`

**Response:**
```json
{
  "worker_jwt": "sk-ant-si-...",
  "api_base_url": "https://...",
  "expires_in": 18000,
  "worker_epoch": "42"
}
```

**Note:** Each call bumps `worker_epoch` — this call IS the registration.

#### POST `/v1/code/sessions/{sessionId}/worker/register`

Register as CCR worker (V1 CCR v2 path). Returns `{ "worker_epoch": "42" }`.

#### GET `/v1/code/sessions/{sessionId}/worker/events/stream`

SSE stream for receiving inbound events. Sends `Last-Event-ID` or `from_sequence_num` for resumption.

**SSE frame format:**
```
event: sdk_event
id: 42
data: {"event_id":"evt_abc","payload":{...}}

:keepalive

```

#### POST `/v1/code/sessions/{sessionId}/worker/events`

Post events to the session. Auth: worker JWT.

#### PUT `/v1/code/sessions/{sessionId}/worker`

Update worker state.

**Request:**
```json
{
  "worker_status": "running" | "requires_action" | "completed",
  "external_metadata": { "key": "value" },
  "internal_metadata": { "key": "value" }
}
```

**Metadata merge:** RFC 7396 — keys added/overwritten, `null` values = server-side delete.

#### POST `/v1/code/sessions/{sessionId}/worker/events/{eventId}/delivery`

Report event delivery status. **Request:** `{ "status": "received" | "processing" | "processed" }`

### Auth / Trusted Device

#### POST `/api/auth/trusted_devices`

Enroll device for elevated security sessions.

**Request:**
```json
{ "display_name": "Claude Code on hostname · darwin" }
```

**Response:**
```json
{ "device_token": "...", "device_id": "..." }
```

**Gate:** Must be called within 10 minutes of login.

### File Attachments

#### GET `/api/oauth/files/{fileUuid}/content`

Download attachment content. Auth: OAuth token. Returns binary file data.

### Sessions WebSocket (Remote Viewer)

#### WS `/v1/sessions/ws/{sessionId}/subscribe?organization_uuid={orgUuid}`

Connect as viewer. After connect, send:
```json
{ "type": "auth", "credential": { "type": "oauth", "token": "<accessToken>" } }
```

---

## 26. WebSocket & SSE Protocol Reference

### Session-Ingress WebSocket Protocol

**URL:** `wss://api.anthropic.com/v1/session_ingress/ws/{sessionId}`

**V1 read/write:** Messages in both directions are NDJSON (`StdoutMessage` / `StdinMessage`).

**Permanent close codes (client stops retrying):**
- `1002` — protocol error
- `4001` — session expired/not found
- `4003` — unauthorized

**Keep-alive frame:** `{"type":"keep_alive"}` — sent by client at `DEFAULT_KEEPALIVE_INTERVAL` (5 min).

**Ping/pong:** Client sends ping every 10s, expects pong within 10s. Connection recycled on pong timeout.

### SSE Event Format

```
event: sdk_event
id: <sequence_number>
data: <json_payload>

:keepalive

```

- `id` field: monotonic sequence number used for resume (`Last-Event-ID`)
- `event` field: event type (`sdk_event`, `keep_alive`, etc.)
- `data` field: JSON object with `event_id` and `payload`

### Control Message Protocol

**SDKControlRequest** (server → client):
```json
{
  "type": "control_request",
  "request_id": "req_abc",
  "request": {
    "subtype": "initialize" | "set_model" | "set_max_thinking_tokens" | "set_permission_mode" | "interrupt" | "can_use_tool",
    ...subtype-specific fields...
  }
}
```

**SDKControlResponse** (client → server):
```json
{
  "type": "control_response",
  "session_id": "session_abc",
  "response": {
    "subtype": "success" | "error",
    "request_id": "req_abc",
    ...response payload...
  }
}
```

**SDKControlCancelRequest** (server → client):
```json
{
  "type": "control_cancel_request",
  "request_id": "req_abc"
}
```

### Permission Request Flow

1. Child CLI emits `control_request` with `subtype: 'can_use_tool'` on stdout.
2. Bridge receives it via `onPermissionRequest` callback.
3. Bridge forwards to server via `POST /v1/sessions/{id}/events` (permission response event).
4. Claude.ai displays approval dialog.
5. User approves/denies.
6. Server sends `control_response` back through the WebSocket.
7. Bridge calls `onPermissionResponse` callback.
8. Child CLI receives the decision on stdin and proceeds.

**Permission response payload:**
```json
{
  "behavior": "allow" | "deny",
  "updatedInput": {...},          // Optional: modified tool input
  "updatedPermissions": [...],    // Optional: new permission rules
  "message": "..."                // Optional: deny message
}
```

### NDJSON Safety

All NDJSON-format messages (used in child process stdio) must escape `U+2028` and `U+2029` as `\u2028`/`\u2029` (see `ndjsonSafeStringify`). These are valid JSON but are JavaScript line terminators that can break line-splitting receivers.

---

*Document generated from source analysis of Claude Code codebase, 2026-03-31.*
