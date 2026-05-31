# Claude Code — React Hooks

This document covers every hook in `src/hooks/`, `src/hooks/toolPermission/`, and `src/hooks/notifs/`. For each hook the entry covers: purpose, parameters/props, return value, key logic and side effects, and dependencies.

---

## Table of Contents

1. [Core / Utility Hooks](#core--utility-hooks)
2. [Input & Text Editing Hooks](#input--text-editing-hooks)
3. [Permission & Tool-Use Hooks](#permission--tool-use-hooks)
4. [Swarm / Teammate Hooks](#swarm--teammate-hooks)
5. [IDE Integration Hooks](#ide-integration-hooks)
6. [Remote & Session Hooks](#remote--session-hooks)
7. [Plugin & Suggestion Hooks](#plugin--suggestion-hooks)
8. [Notification Hooks (`notifs/`)](#notification-hooks-notifs)
9. [Tool Permission Subsystem (`toolPermission/`)](#tool-permission-subsystem-toolpermission)
10. [Non-Hook Utilities in `hooks/`](#non-hook-utilities-in-hooks)

---

## Core / Utility Hooks

### `useAfterFirstRender`

**File:** `hooks/useAfterFirstRender.ts`

**Purpose:** ANT-internal startup-time measurement hook. After the first render it writes startup time to stderr and calls `process.exit(0)` if the `CLAUDE_CODE_EXIT_AFTER_FIRST_RENDER` environment variable is set.

**Parameters:** none

**Return Value:** `void`

**Key Logic:**
- Reads env var `CLAUDE_CODE_EXIT_AFTER_FIRST_RENDER`.
- Uses a `useEffect` on `[]` to fire after first commit.
- Computes elapsed ms from `MACRO.STARTUP_TIMESTAMP`, writes to `process.stderr`, then exits.

**Dependencies:** `useEffect` (React)

---

### `useApiKeyVerification`

**File:** `hooks/useApiKeyVerification.ts`

**Purpose:** Manages the full lifecycle of API key verification — loading, valid, invalid, missing, or error — and exposes a `reverify` callback. Guards against running `apiKeyHelper` scripts before the trust dialog is dismissed to prevent RCE.

**Parameters:** none

**Return Value:** `ApiKeyVerificationResult` — `{ status: 'loading'|'valid'|'invalid'|'missing'|'error', reverify: () => void, errorMessage?: string }`

**Key Logic:**
- Subscribes to `AppState` for `trustDialogAccepted` and `apiKeyVerificationStatus`.
- Uses `useEffect` to run the verification on mount and whenever `reverify` is called.
- Skips the `apiKeyHelper` process before the trust dialog is shown.
- Returns a stable `reverify` callback via `useCallback`.

**Dependencies:** `useAppState`, `useSetAppState`, `useCallback`, `useEffect`

---

### `useBlink`

**File:** `hooks/useBlink.ts`

**Purpose:** Returns a blinking boolean flag synchronized with an animation-frame clock. Pauses when the terminal is blurred or the component is offscreen (OffscreenFreeze).

**Parameters:**
- `enabled: boolean` — when false, always returns `true` (cursor always visible).
- `intervalMs?: number` — blink period in milliseconds (default: 530).

**Return Value:** `[ref: RefObject<unknown>, isVisible: boolean]`

**Key Logic:**
- Uses `useAnimationFrame` (Ink hook) to read the shared clock counter.
- Divides counter by `intervalMs / frameMs` and toggles on even/odd.
- Returns same ref from Ink's `useOffscreenFreeze` to pause when not in viewport.

**Dependencies:** `useAnimationFrame` (ink), `useTerminalFocus` (ink), `useRef`, `useMemo`

---

### `useCommandQueue`

**File:** `hooks/useCommandQueue.ts`

**Purpose:** Exposes the current unified command queue as a reactive array. Any component can subscribe to observe queued commands without managing external store subscriptions manually.

**Parameters:** none

**Return Value:** `readonly QueuedCommand[]`

**Key Logic:**
- Wraps `useSyncExternalStore` over `messageQueueManager`'s subscribe/getSnapshot pair.
- Re-renders only when the queue reference changes (not on every push that doesn't change length).

**Dependencies:** `useSyncExternalStore` (React), `messageQueueManager`

---

### `useCopyOnSelect`

**File:** `hooks/useCopyOnSelect.ts`

**Purpose:** Automatically copies selected text to the clipboard when the user releases the mouse (mouseup) or double/triple-clicks. Also exports `useSelectionBgColor` for theming selected text.

**Parameters:**
- `selection: SelectionState` — current ink selection state.
- `isActive: boolean` — only active when true.
- `onCopied?: () => void` — callback fired after clipboard write.

**Return Value:** `void`

**Key Logic:**
- Subscribes to ink mouse events via `useEffect`.
- On mouseup: if selection is non-empty and `isActive`, calls `navigator.clipboard.writeText`.
- `useSelectionBgColor()` reads AppState theme to return the correct highlight color.

**Dependencies:** `useEffect`, `useAppState`, `useCallback`

---

### `useDoublePress`

**File:** `hooks/useDoublePress.ts`

**Purpose:** Returns a callback that implements double-press detection within an 800 ms window. Used for Ctrl+C/D to exit and double-Escape to clear input.

**Parameters:**
- `setPending: (show: boolean) => void` — called with `true` after first press, `false` after timeout.
- `onDoublePress: () => void` — called when the second press occurs within the window.
- `onFirstPress?: () => void` — optional side effect on first press.

**Return Value:** `() => void` — the wrapped press handler

**Key Logic:**
- Tracks `lastPressTime` in a ref.
- On call: if elapsed < 800 ms, calls `onDoublePress` and resets; otherwise calls `setPending(true)`, sets a 800 ms timer to call `setPending(false)`, and optionally calls `onFirstPress`.

**Dependencies:** `useRef`, `useCallback`

---

### `useElapsedTime`

**File:** `hooks/useElapsedTime.ts`

**Purpose:** Computes a human-readable elapsed time string (e.g. `"1m 23s"`) that updates while a task is running and freezes once it ends.

**Parameters:**
- `startTime: number` — Unix timestamp (ms) when timing started.
- `isRunning: boolean` — when false, elapsed is frozen.
- `ms?: number` — update interval in ms (default: 1000).
- `pausedMs?: number` — accumulated paused time to subtract.
- `endTime?: number` — if provided, freezes at this timestamp.

**Return Value:** `string` — formatted elapsed time like `"5s"`, `"1m 23s"`, `"2h 5m"`.

**Key Logic:**
- Uses `useSyncExternalStore` over a timer-based external clock.
- Clock updates every `ms` via `setInterval`; each subscriber gets a stable snapshot until it ticks.
- Formats the delta using `formatDuration`.

**Dependencies:** `useSyncExternalStore`, `useRef`

---

### `useExitOnCtrlCD`

**File:** `hooks/useExitOnCtrlCD.ts`

**Purpose:** Implements double-press Ctrl+C / Ctrl+D to exit. Returns pending state so callers can show a "Press again to exit" hint.

**Parameters:**
- `useKeybindingsHook: (bindings: ...) => void` — injectable hook for binding.
- `onInterrupt?: () => void` — called on first Ctrl+C press.
- `onExit?: () => void` — called on second press.
- `isActive?: boolean` — enables/disables the handler.

**Return Value:** `ExitState` — `{ pending: boolean, keyName: string | null }`

**Key Logic:**
- Uses `useDoublePress` internally for both Ctrl+C and Ctrl+D.
- Sets `pending = true` after first press; false after timeout or second press.
- `keyName` tracks which key was pressed ('Ctrl-C' or 'Ctrl-D').

**Dependencies:** `useDoublePress`, `useState`, `useCallback`

---

### `useExitOnCtrlCDWithKeybindings`

**File:** `hooks/useExitOnCtrlCDWithKeybindings.ts`

**Purpose:** Convenience wrapper that wires `useExitOnCtrlCD` to the keybinding system.

**Parameters:**
- `onExit?: () => void`
- `onInterrupt?: () => void`
- `isActive?: boolean`

**Return Value:** `ExitState`

**Key Logic:** Passes `useKeybindings` as the hook parameter to `useExitOnCtrlCD`.

**Dependencies:** `useExitOnCtrlCD`, `useKeybindings`

---

### `useMemoryUsage`

**File:** `hooks/useMemoryUsage.ts`

**Purpose:** Polls Node.js `process.memoryUsage().heapUsed` every 10 seconds and returns a status when memory usage is high or critical.

**Parameters:** none

**Return Value:** `MemoryUsageInfo | null` — `null` for normal; `{ heapUsed: number, status: 'high' | 'critical' }` when heap > 1.5 GB (high) or > 2.5 GB (critical).

**Key Logic:**
- Uses `useInterval` (usehooks-ts) with 10 000 ms period.
- Thresholds: `HIGH_HEAP_MB = 1536`, `CRITICAL_HEAP_MB = 2560`.
- Returns `null` when below thresholds.

**Dependencies:** `useInterval`, `useState`, `useEffect`

---

### `useMinDisplayTime`

**File:** `hooks/useMinDisplayTime.ts`

**Purpose:** Prevents UI flicker by guaranteeing each distinct value stays visible for at least `minMs` milliseconds before switching.

**Parameters:**
- `value: T` — the value to display.
- `minMs: number` — minimum display duration.

**Return Value:** `T` — the "stable" displayed value, may lag behind `value`.

**Key Logic:**
- Uses `useRef` to track the current stable value and the timestamp it was set.
- On `value` change: if `Date.now() - lastChanged >= minMs`, updates immediately; otherwise schedules a `setTimeout` to update after the remainder.

**Dependencies:** `useState`, `useRef`, `useEffect`

---

### `useNotifyAfterTimeout`

**File:** `hooks/useNotifyAfterTimeout.ts`

**Purpose:** Sends a desktop (OS-level) notification after 6 seconds of user inactivity — used to alert the user when Claude has been working unattended.

**Parameters:**
- `message: string` — the notification body text.
- `notificationType: string` — identifies the event type for analytics.

**Return Value:** `void`

**Key Logic:**
- Waits 6 000 ms after mount using `setTimeout`.
- Checks terminal focus state; only fires if the terminal is not focused.
- Calls `sendDesktopNotification` from a native module.

**Dependencies:** `useEffect`, `useRef`

---

### `useTimeout`

**File:** `hooks/useTimeout.ts`

**Purpose:** Returns a boolean that becomes `true` after `delay` ms. Resets when `resetTrigger` changes.

**Parameters:**
- `delay: number` — ms to wait.
- `resetTrigger?: number` — changing this value resets the timer.

**Return Value:** `boolean` — `false` until the delay elapses, then `true`.

**Key Logic:** Simple `useState` + `useEffect` with `setTimeout`. Cleanup clears the timeout on re-run or unmount.

**Dependencies:** `useState`, `useEffect`

---

### `useSettings`

**File:** `hooks/useSettings.ts`

**Purpose:** Reads the current settings from global AppState. Reactive — re-renders when settings change (e.g. file-watcher triggers).

**Parameters:** none

**Return Value:** `ReadonlySettings`

**Key Logic:** Returns `useAppState(s => s.settings)`.

**Dependencies:** `useAppState`

---

### `useSettingsChange`

**File:** `hooks/useSettingsChange.ts`

**Purpose:** Subscribes to the settings change detector and calls `onChange` with the new settings and the change source whenever the settings file is modified on disk.

**Parameters:**
- `onChange: (source: string, settings: Settings) => void`

**Return Value:** `void`

**Key Logic:**
- Uses `useEffect` to subscribe to `settingsChangeDetector.subscribe(onChange)`.
- Returns the unsubscribe function as the cleanup.

**Dependencies:** `useEffect`, `settingsChangeDetector`

---

### `useDeferredHookMessages`

**File:** `hooks/useDeferredHookMessages.ts`

**Purpose:** Injects `SessionStart` hook messages into the message list asynchronously on mount, avoiding blocking the first render.

**Parameters:**
- `pendingHookMessages: Message[]` — messages generated by session-start hooks.
- `setMessages: SetMessages` — the message list updater.

**Return Value:** `() => Promise<void>` — a stable async callback to trigger injection.

**Key Logic:**
- Defers via `setTimeout(0)` to let the first render complete before injecting hook messages.
- Uses `useRef` to avoid stale closure issues.

**Dependencies:** `useRef`, `useCallback`

---

### `useDiffData`

**File:** `hooks/useDiffData.ts`

**Purpose:** Fetches current git diff statistics and hunks on mount (used by the `/diff` command view).

**Parameters:** none

**Return Value:** `DiffData` — `{ stats: DiffStats, files: string[], hunks: DiffHunk[], loading: boolean }`

**Key Logic:**
- On mount, calls `getGitDiff()` which runs `git diff` in the cwd.
- Sets `loading: true` until the async fetch completes.

**Dependencies:** `useState`, `useEffect`

---

### `useFileHistorySnapshotInit`

**File:** `hooks/useFileHistorySnapshotInit.ts`

**Purpose:** One-time initialization of the file history state from snapshot data stored in the conversation log, restoring file timestamps across `/resume`.

**Parameters:**
- `initialFileHistorySnapshots: FileHistorySnapshot[]`
- `fileHistoryState: FileHistoryState`
- `onUpdateState: (state: FileHistoryState) => void`

**Return Value:** `void`

**Key Logic:**
- Uses `useEffect` with `[]` dep to run only once.
- Merges `initialFileHistorySnapshots` into `fileHistoryState` without overwriting newer entries.

**Dependencies:** `useEffect`

---

### `useInputBuffer`

**File:** `hooks/useInputBuffer.ts`

**Purpose:** Provides a debounced undo buffer for text input, enabling "undo last paste" or "undo last edit" functionality.

**Parameters:**
- `maxBufferSize: number` — maximum number of entries to keep.
- `debounceMs: number` — how long to wait before committing current value.

**Return Value:** `UseInputBufferResult` — `{ pushToBuffer, undo, canUndo, clearBuffer }`

**Key Logic:**
- Maintains a `string[]` undo stack in a ref.
- `pushToBuffer` is debounced: multiple rapid changes collapse into one buffer entry.
- `undo` pops the stack and calls `onChange` with the previous value.

**Dependencies:** `useRef`, `useCallback`, `useEffect`

---

### `useLogMessages`

**File:** `hooks/useLogMessages.ts`

**Purpose:** Incrementally records messages to the conversation transcript file (`.jsonl`) after each render. Avoids re-writing the full transcript on every update.

**Parameters:**
- `messages: readonly Message[]` — the current message list.
- `ignore?: boolean` — when true, skips recording.

**Return Value:** `void`

**Key Logic:**
- Tracks `lastProcessedIndex` in a ref to process only new messages.
- Handles edge cases: compaction (transcript size shrinks), first render, head-pointer rewind.
- Calls `recordTranscript(messages, from, to)` for new messages only.
- Deduplicates compact-summary boundaries.

**Dependencies:** `useEffect`, `useRef`

---

### `useMainLoopModel`

**File:** `hooks/useMainLoopModel.ts`

**Purpose:** Returns the resolved model name for the current session. Re-evaluates when GrowthBook flags are refreshed so model alias resolution stays current mid-session.

**Parameters:** none

**Return Value:** `ModelName`

**Key Logic:**
- Reads `settings.model` from AppState.
- Subscribes to `onGrowthBookRefresh` via `useEffect`; on each refresh, forces a re-render by incrementing a counter state.
- Calls `resolveModelAlias(model)` to translate user-facing aliases (e.g. `opus`) to concrete model IDs.

**Dependencies:** `useAppState`, `useEffect`, `useState`

---

### `useManagePlugins`

**File:** `hooks/useManagePlugins.ts`

**Purpose:** Loads the plugin list on mount and wires up plugin lifecycle management: delisting enforcement, MCP/LSP plugin counting, and refresh-needed notifications.

**Parameters:**
- `{ enabled?: boolean }`

**Return Value:** `void`

**Key Logic:**
- On mount (if enabled): calls `loadPlugins()` and writes results to AppState.
- Enforces delisted plugin removal by reading `delistedPlugins` from settings.
- Counts active MCP and LSP plugins and writes totals to AppState for /doctor diagnostics.
- Does NOT auto-refresh; refresh is triggered explicitly via `/reload-plugins`.

**Dependencies:** `useEffect`, `useSetAppState`, `useAppState`

---

### `useMergedClients`

**File:** `hooks/useMergedClients.ts`

**Purpose:** Deduplicates two MCP client lists (initial from settings + dynamically loaded) by server name.

**Parameters:**
- `initialClients: MCPServerConnection[]`
- `mcpClients: MCPServerConnection[]`

**Return Value:** `MCPServerConnection[]`

**Key Logic:** Uses `lodash.uniqBy([...initialClients, ...mcpClients], 'name')`. The `useMemo` dependency is the combined list length and name set.

**Dependencies:** `useMemo`, `lodash.uniqBy`

---

### `useMergedCommands`

**File:** `hooks/useMergedCommands.ts`

**Purpose:** Deduplicates command lists from initial load and MCP-sourced commands by command name.

**Parameters:**
- `initialCommands: Command[]`
- `mcpCommands: Command[]`

**Return Value:** `Command[]`

**Key Logic:** `useMemo` over `uniqBy([...initialCommands, ...mcpCommands], getCommandName)`.

**Dependencies:** `useMemo`

---

### `useMergedTools`

**File:** `hooks/useMergedTools.ts`

**Purpose:** Assembles the full tool pool for a session by combining built-in tools, MCP tools, and applying permission-context filtering.

**Parameters:**
- `initialTools: Tool[]`
- `mcpTools: Tool[]`
- `toolPermissionContext: ToolPermissionContext`

**Return Value:** `Tools` (the assembled tool set)

**Key Logic:**
- Calls `assembleToolPool(initialTools, mcpTools)` to build the combined list.
- Then calls `mergeAndFilterTools(pool, toolPermissionContext)` to remove disabled tools.

**Dependencies:** `useMemo`

---

### `useSkillsChange`

**File:** `hooks/useSkillsChange.ts`

**Purpose:** Keeps the command list fresh when skill files change on disk or when GrowthBook flags are refreshed.

**Parameters:**
- `cwd: string | undefined` — the current working directory for scanning skills.
- `onCommandsChange: (commands: Command[]) => void` — callback to update the command list.

**Return Value:** `void`

**Key Logic:**
- Subscribes to `skillChangeDetector.subscribe(handleChange)` — fires on skill file writes.
- On file change: calls `clearCommandsCache()` + `getCommands(cwd)` and calls `onCommandsChange`.
- Subscribes to `onGrowthBookRefresh(handleGrowthBookRefresh)` — on GB flag refresh, calls `clearCommandMemoizationCaches()` + `getCommands(cwd)` to re-evaluate feature-gated commands.

**Dependencies:** `useEffect`, `useCallback`

---

### `useUpdateNotification`

**File:** `hooks/useUpdateNotification.ts`

**Purpose:** Returns the new semantic version string when an auto-update has been downloaded, for display in the status bar. Returns `null` if no new version or the version hasn't changed since last notification.

**Parameters:**
- `updatedVersion: string | null | undefined` — the downloaded version (from auto-updater).
- `initialVersion?: string` — baseline version (default: `MACRO.VERSION`).

**Return Value:** `string | null`

**Key Logic:**
- Parses both versions with `semver` to extract `major.minor.patch`.
- Uses `useState` to track the last-notified semver.
- If the new semver differs from `lastNotifiedSemver`, sets state and returns the new value (triggers notification display). Otherwise returns `null`.

**Dependencies:** `useState`, `semver`

---

## Input & Text Editing Hooks

### `useTextInput`

**File:** `hooks/useTextInput.ts`

**Purpose:** Full readline-style text input handler. Manages cursor position, multiline editing, kill ring (Ctrl+K/U/W), yank (Ctrl+Y / Meta+Y), history navigation (Up/Down arrows), and ghost text rendering.

**Parameters:** `UseTextInputProps` including:
- `value: string` — current text value (controlled).
- `onChange: (value: string) => void`
- `onSubmit?: (value: string) => void`
- `onExit?: () => void`
- `onHistoryUp / onHistoryDown / onHistoryReset / onClearInput`
- `focus?: boolean`
- `mask?: string` — masks all chars with this string.
- `multiline?: boolean`
- `cursorChar: string`
- `columns: number` — terminal width for wrapping.
- `externalOffset: number` — cursor offset controlled externally.
- `onOffsetChange: (offset: number) => void`
- `inputFilter?: (input: string, key: Key) => string`
- `inlineGhostText?: InlineGhostText`
- `disableCursorMovementForUpDownKeys?: boolean`
- `disableEscapeDoublePress?: boolean`
- `maxVisibleLines?: number`

**Return Value:** `TextInputState` — `{ onInput, renderedValue, offset, setOffset, cursorLine, cursorColumn, viewportCharOffset, viewportCharEnd }`

**Key Logic:**
- `Cursor` class from `utils/Cursor.js` manages the text buffer and position arithmetic.
- Maps keypresses to cursor mutations: Ctrl+A (home), Ctrl+E (end), Ctrl+F/B (forward/back), Ctrl+N/P (next/prev line), Meta+F/B (word navigation).
- Kill ring: Ctrl+K (kill to end), Ctrl+U (kill to start), Ctrl+W (kill word). Successive kills append to ring.
- Yank: Ctrl+Y inserts last kill; Meta+Y cycles through the ring.
- Double-press Ctrl+C clears or exits (via `useDoublePress`).
- Double-press Escape clears input with "Esc again to clear" hint.
- SSH-coalesced Enter detection: `text\r` form triggers submit.
- Handles raw `\x7f` DEL characters for SSH/tmux compatibility.
- Inline ghost text rendered at cursor position when `inlineGhostText.insertPosition === offset`.

**Dependencies:** `useDoublePress`, `useNotifications`, `Cursor` class, `useCallback`

---

### `useVimInput`

**File:** `hooks/useVimInput.ts`

**Purpose:** Extends `useTextInput` with a full Vim normal/insert mode state machine, including operators (d, c, y), motions, dot-repeat, find (f/F/t/T), text objects (iw, aw, etc.), and yank register.

**Parameters:** `UseVimInputProps` — same as `UseTextInputProps` plus:
- `onModeChange?: (mode: VimMode) => void`
- `onUndo?: () => void`

**Return Value:** `VimInputState` — extends `TextInputState` with `{ mode: VimMode, setMode }`

**Key Logic:**
- Delegates INSERT mode keypresses to `useTextInput` after running `inputFilter`.
- In NORMAL mode, dispatches keypresses through `transition(state.command, input, ctx)` from `vim/transitions.ts`.
- Manages `vimStateRef` (current mode + pending command accumulator) and `persistentRef` (register, lastFind, lastChange for dot-repeat).
- Escape in INSERT: `switchToNormalMode()` moves cursor left by one.
- Arrow keys in NORMAL: mapped to h/j/k/l motions.
- `?` in NORMAL idle: enters `/` search by writing `?` to the input.
- `setModeExternal` allows callers to programmatically switch modes (used by `/vim` command).

**Dependencies:** `useTextInput`, `useState`, `useRef`, `useCallback`, vim operators/transitions

---

### `useSearchInput`

**File:** `hooks/useSearchInput.ts`

**Purpose:** Full readline-style text input for search boxes (history search, global search). Includes kill ring, yank, and word navigation.

**Parameters:** `UseSearchInputOptions` — `{ initialValue?, onKeyDown?, placeholder? }`

**Return Value:** `{ query: string, setQuery, cursorOffset: number, handleKeyDown: (key: Key, input: string) => void }`

**Key Logic:**
- Implements the same Ctrl key mapping as `useTextInput` but as a standalone reducer without React state for the cursor offset.
- Used in `HistorySearchInput` and `GlobalSearchDialog`.

**Dependencies:** `useState`, `useCallback`, `useRef`, kill ring utilities

---

### `useArrowKeyHistory`

**File:** `hooks/useArrowKeyHistory.tsx`

**Purpose:** Arrow-key navigation through input history with lazy chunked loading, mode-based filtering, and draft preservation.

**Parameters:**
- `onSetInput: (value: string) => void`
- `currentInput: string`
- `pastedContents: string[]` — paste-detected content to exclude from history matching.
- `setCursorOffset?: (offset: number) => void`
- `currentMode?: PromptInputMode`

**Return Value:** `{ handleHistoryUp, handleHistoryDown, handleHistoryReset }`

**Key Logic:**
- Loads history lazily in chunks of 50 from `getHistory()`.
- Navigates using an index pointer; on first Up saves the current draft.
- Filters entries by mode (e.g., bash mode only returns bash history).
- Shows a "Search history: Ctrl+R" hint notification on first use.
- Resets index when `currentInput` changes externally.

**Dependencies:** `useState`, `useRef`, `useCallback`, `useNotifications`

---

### `useHistorySearch`

**File:** `hooks/useHistorySearch.ts`

**Purpose:** Implements `Ctrl+R` backward incremental history search with query matching and keyboard navigation.

**Parameters:**
- `onSetInput: (value: string) => void`
- `currentInput: string`
- plus keybinding options.

**Return Value:** `{ historyQuery, setHistoryQuery, historyMatch, historyFailedMatch, handleKeyDown }`

**Key Logic:**
- Registers `history:search` keybinding (Ctrl+R) to activate search mode.
- In search mode, registers `historySearch:*` bindings (Enter to confirm, Escape to cancel, up/down to cycle matches).
- Filters history entries by substring match against `historyQuery`.
- `historyFailedMatch: boolean` — true when query has text but no match.

**Dependencies:** `useKeybinding`, `useKeybindings`, `useState`, `useCallback`, `useEffect`

---

### `useTypeahead`

**File:** `hooks/useTypeahead.tsx`

**Purpose:** The primary typeahead/autocomplete engine for the prompt input. Handles `@file`, `/command`, `#channel`, and directory suggestions using debounced fuzzy matching, shell completion, and MCP resources.

**Parameters:** Large props object including:
- `inputValue: string`, `cursorOffset: number`
- `commands: Command[]`, `agents: AgentDefinition[]`
- `mcpResources: MCPResource[]`
- `isLoading: boolean`
- `onSelect: (value: string) => void`
- `onToggleVisible: (show: boolean) => void`

**Return Value:** `{ suggestions, selectedIndex, handleKeyDown, isSuggesting, suggestionType, ... }`

**Key Logic:**
- Detects suggestion context from input: `@token` triggers file/resource/agent suggestions; `/` triggers command suggestions; `#channel` triggers Slack channel suggestions (if Slack MCP present).
- File suggestions use `generateUnifiedSuggestions` (nucleo + Fuse.js ranked).
- Command suggestions use `generateCommandSuggestions` with argument hint generation.
- Shell completions use `getShellCompletions` for bash/zsh completions.
- Path completions use `getPathCompletions` / `getDirectoryCompletions`.
- Registers as an overlay via `useRegisterOverlay` so escape/enter/arrow keys are captured.
- Uses `useDebounceCallback` (usehooks-ts) to rate-limit file lookups.
- Tracks keyboard navigation state (`selectedIndex`) internally.
- Session resume suggestions for `/resume` queries via `searchSessionsByCustomTitle`.

**Dependencies:** `useInput` (ink, backward-compat bridge), `useRegisterOverlay`, `useKeybindings`, `useDebounceCallback`, `useState`, `useRef`, `useMemo`, `useCallback`, `useEffect`, `generateUnifiedSuggestions`, `generateCommandSuggestions`, `getShellCompletions`

---

### `usePasteHandler`

**File:** `hooks/usePasteHandler.ts`

**Purpose:** Handles bracketed paste mode detection, large paste chunking, image file path detection, and macOS clipboard image fallback.

**Parameters:**
- `{ onPaste: (text: string) => void, onInput: (text: string, key: Key) => void, onImagePaste?: (base64: string, ...) => void }`

**Return Value:** `{ wrappedOnInput, pasteState, isPasting }`

**Key Logic:**
- Detects bracketed paste via `\x1b[?2004h` / `\x1b[200~` / `\x1b[201~` escape sequences.
- Splits large pastes (>1000 chars) into multiple `onPaste` calls to avoid blocking the event loop.
- Detects image file paths in paste content (extensions `.png`, `.jpg`, `.gif`, etc.) and triggers `onImagePaste`.
- On macOS: falls back to `pbpaste` for image clipboard content.

**Dependencies:** `useState`, `useRef`, `useCallback`, `useEffect`

---

### `useVoice`

**File:** `hooks/useVoice.ts`

**Purpose:** Hold-to-talk voice recording using the `voice_stream` STT endpoint. Auto-repeat key events extend the recording; releasing the key after `RELEASE_TIMEOUT_MS` stops it.

**Parameters:** `{ onTranscript: (text: string) => void, enabled: boolean }`

**Return Value:** `{ state: 'idle'|'recording'|'processing', handleKeyEvent: (fallbackMs?: number) => void }`

**Key Logic:**
- Calls `connectVoiceStream()` to open a WebSocket to `voice_stream` STT.
- Maps user locale to BCP-47 language codes for Deepgram (20+ languages mapped).
- Auto-repeat detection: key events arriving within 120 ms are considered "held".
- Modifier combos (Ctrl+Space etc.) use 2 000 ms `FIRST_PRESS_FALLBACK_MS`.
- Requires 5 rapid keydowns (HOLD_THRESHOLD) for bare-char bindings to activate, 2 for warmup feedback.
- Uses `useTerminalFocus` to pause recording when terminal loses focus.
- Fetches voice keyterms for improved domain recognition.

**Dependencies:** `useState`, `useRef`, `useCallback`, `useEffect`, `useTerminalFocus`, `connectVoiceStream`, `getVoiceKeyterms`

---

### `useVoiceEnabled`

**File:** `hooks/useVoiceEnabled.ts`

**Purpose:** Combines user intent (`settings.voiceEnabled`), OAuth auth check, and GrowthBook kill-switch into a single boolean indicating whether voice mode is available.

**Parameters:** none

**Return Value:** `boolean`

**Key Logic:**
- `userIntent` from `useAppState(s => s.settings.voiceEnabled === true)`.
- `authed` memoized on `authVersion` — avoids expensive `hasVoiceAuth()` call on every render.
- `isVoiceGrowthBookEnabled()` not memoized (cheap cached lookup, so mid-session kill-switch takes effect).

**Dependencies:** `useAppState`, `useMemo`

---

### `useVoiceIntegration`

**File:** `hooks/useVoiceIntegration.tsx`

**Purpose:** Orchestrates the full voice-mode integration: reading keybindings, detecting held keys, activating `useVoice`, suppressing full-width space input during recording, and showing status notifications.

**Parameters:**
- `{ onTranscript: (text: string) => void, isModalOverlayActive: boolean }`

**Return Value:** `{ voiceState: VoiceState, handleVoiceKeyEvent }`

**Key Logic:**
- Reads `voice:activate` keybinding from keybinding context (default: spacebar).
- Detects held key by counting rapid key events (HOLD_THRESHOLD=5 for bare chars, 1 for modifier combos).
- Shows warmup notification after WARMUP_THRESHOLD=2 events.
- Uses `useInput` (ink) as a backward-compat bridge until REPL wires `handleKeyDown` to `<Box onKeyDown>`.
- Guards on `useIsModalOverlayActive()` — does not activate voice while a modal is open.
- Dead-code elimination: conditionally requires `useVoice` only if `feature('VOICE_MODE')` is set; otherwise uses a no-op stub.
- Calls `normalizeFullWidthSpace` to handle full-width space (Japanese IME) by passing it to the transcript instead of activating voice.

**Dependencies:** `useVoice`, `useVoiceEnabled`, `useInput` (ink), `useOptionalKeybindingContext`, `useIsModalOverlayActive`, `useNotifications`, `useState`, `useRef`, `useMemo`, `useCallback`, `useEffect`

---

### `useVirtualScroll`

**File:** `hooks/useVirtualScroll.ts`

**Purpose:** React-level virtualization for `MessageRow` items inside a `ScrollBox`. Mounts only items in the viewport plus overscan, using spacer boxes to maintain scroll height.

**Parameters:**
- `scrollRef: RefObject<ScrollBoxHandle | null>` — reference to the ScrollBox.
- `itemKeys: readonly string[]` — stable keys for each item.
- `columns: number` — terminal width; triggers height cache rescaling on change.

**Return Value:** `VirtualScrollResult`:
- `range: [startIndex, endIndex)` — half-open slice to render.
- `topSpacer: number` — rows before the first rendered item.
- `bottomSpacer: number` — rows after last rendered item.
- `measureRef: (key) => ref` — attach to each item root `Box` for height measurement.
- `spacerRef: RefObject<DOMElement>` — attach to top spacer for drift-free origin tracking.
- `offsets: ArrayLike<number>` — cumulative y-offsets per item.
- `getItemTop: (index) => number` — reads live Yoga `computedTop`.
- `getItemElement: (index) => DOMElement | null`
- `getItemHeight: (index) => number | undefined`
- `scrollToIndex: (i) => void`

**Key Logic:**
- `DEFAULT_ESTIMATE = 3` rows for unmeasured items; `OVERSCAN_ROWS = 80`; `COLD_START_COUNT = 30`.
- `SCROLL_QUANTUM = 40` rows — scrollTop is quantized so React re-renders only when the mounted range needs to shift, not on every wheel tick.
- `SLIDE_STEP = 25` — caps new mounts per commit to bound reconcile time.
- `PESSIMISTIC_HEIGHT = 1` for coverage back-walk (guarantees viewport coverage).
- `MAX_MOUNTED_ITEMS = 300` cap.
- On column change: scales all cached heights by `oldCols/newCols` instead of clearing.
- Uses `useSyncExternalStore` over the ScrollBox's scroll-top external store.
- Uses `useLayoutEffect` to measure Yoga heights after each commit.
- Sticky-scroll: when pinned to the bottom, always renders the last N items.

**Dependencies:** `useRef`, `useMemo`, `useDeferredValue`, `useLayoutEffect`, `useSyncExternalStore`, `ScrollBox` handle

---

## Permission & Tool-Use Hooks

### `useCanUseTool`

**File:** `hooks/useCanUseTool.tsx`

**Purpose:** Core permission gate for tool execution. Called for every tool use attempt; routes through the appropriate handler (coordinator, interactive, swarm-worker) and resolves to a `PermissionDecision`.

**Parameters:**
- `setToolUseConfirmQueue: SetState<ToolUseConfirm[]>`
- `setToolPermissionContext: (ctx: ToolPermissionContext) => void`

**Return Value:** `CanUseToolFn` — `async (tool, input, toolUseContext, assistantMessage, toolUseID) => PermissionDecision`

**Key Logic:**
1. Calls `hasPermissionsToUseTool` to get the initial decision (`allow`, `deny`, or `ask`).
2. If `allow` or `deny`: logs and returns immediately.
3. If `ask`:
   a. If swarm worker: delegates to `handleSwarmWorkerPermission` (forwards to leader via mailbox).
   b. If coordinator worker: delegates to `handleCoordinatorPermission` (awaits hooks + classifier, then falls through).
   c. Otherwise: delegates to `handleInteractivePermission` (shows dialog, races hooks/classifier/bridge/channel).
4. Creates a `PermissionContext` object with the full set of callbacks (logDecision, persistPermissions, tryClassifier, runHooks, etc.).

**Dependencies:** `useCallback`, `useAppState`, `useSetAppState`, `createPermissionContext`, `createPermissionQueueOps`, `handleCoordinatorPermission`, `handleInteractivePermission`, `handleSwarmWorkerPermission`

---

### `CancelRequestHandler` (exported as `useCancelRequest` module)

**File:** `hooks/useCancelRequest.ts`

**Purpose:** React component (renders `null`) that registers three keybinding handlers for cancellation:
1. `chat:cancel` (Escape) — cancels running task or pops queued command.
2. `app:interrupt` (Ctrl+C) — cancels running task; in teammate view, also kills all agents and exits.
3. `chat:killAgents` (Ctrl+X Ctrl+K) — two-press pattern to stop all background agents.

**Parameters:** `CancelRequestHandlerProps`:
- `setToolUseConfirmQueue`, `onCancel`, `onAgentsKilled`
- `isMessageSelectorVisible`, `screen`
- `abortSignal?: AbortSignal`
- `popCommandFromQueue?`, `vimMode`, `isLocalJSXCommand`, `isSearchingHistory`, `isHelpOpen`
- `inputMode?, inputValue?, streamMode?`

**Return Value:** `null`

**Key Logic:**
- `handleCancel`: Priority 1 — abort signal if task running. Priority 2 — pop command if queue non-empty. Fallback — call `onCancel`.
- `handleInterrupt`: if in teammate view, kills all agents + exits. Then calls `handleCancel`.
- `handleKillAgents`: first press shows "Press again" hint; second press within 3 000 ms (`KILL_AGENTS_CONFIRM_WINDOW_MS`) kills all `local_agent` tasks, emits SDK events, enqueues aggregate notification.
- `isEscapeActive` / `isCtrlCActive` guards: skip if overlay, vim INSERT, transcript, history-search, help, etc.
- `chat:killAgents` always registered to prevent Ctrl+K (chord prefix) passing to readline.

**Dependencies:** `useKeybinding`, `useAppState`, `useSetAppState`, `useCommandQueue`, `useNotifications`, `useIsOverlayActive`, `useCallback`, `useRef`, `killAllRunningAgentTasks`, `emitTaskTerminatedSdk`

---

### `useSwarmPermissionPoller`

**File:** `hooks/useSwarmPermissionPoller.ts`

**Purpose:** Polls every 500 ms for permission responses from the swarm leader when running as a worker agent. When a response arrives, it invokes the registered callback (`onAllow` or `onReject`).

**Parameters:** none

**Return Value:** `void`

**Key Logic:**
- Only active when `isSwarmWorker()` returns `true`.
- Uses `useInterval` (usehooks-ts) with `POLL_INTERVAL_MS = 500`.
- For each `requestId` in `pendingCallbacks`, calls `pollForResponse(requestId, agentName, teamName)`.
- On response: calls `processResponse(response)` which invokes the callback, then calls `removeWorkerResponse`.
- Module-level `pendingCallbacks: Map<string, PermissionResponseCallback>` and `pendingSandboxCallbacks: Map<...>`.
- Exported helper functions: `registerPermissionCallback`, `unregisterPermissionCallback`, `hasPermissionCallback`, `clearAllPendingCallbacks`, `processMailboxPermissionResponse`, `registerSandboxPermissionCallback`, `hasSandboxPermissionCallback`, `processSandboxPermissionResponse`.

**Dependencies:** `useCallback`, `useEffect`, `useRef`, `useInterval`, `permissionSync`

---

## Swarm / Teammate Hooks

### `useSwarmInitialization`

**File:** `hooks/useSwarmInitialization.ts`

**Purpose:** Initializes swarm features (teammate context and hooks) on mount. Handles both resumed sessions (teamName/agentName in transcript) and fresh spawns (environment variables).

**Parameters:**
- `setAppState: SetAppState`
- `initialMessages: Message[] | undefined`
- `{ enabled?: boolean }`

**Return Value:** `void`

**Key Logic:**
- Checks `isAgentSwarmsEnabled()` before doing anything.
- Resumed session path: reads `teamName`/`agentName` from `initialMessages[0]`, calls `initializeTeammateContextFromSession`, reads team file to get `agentId`, calls `initializeTeammateHooks`.
- Fresh spawn path: calls `getDynamicTeamContext()` to read env vars, then calls `initializeTeammateHooks`.

**Dependencies:** `useEffect`

---

### `useTeammateViewAutoExit`

**File:** `hooks/useTeammateViewAutoExit.ts`

**Purpose:** Auto-exits teammate viewing mode when the viewed teammate is killed, fails, encounters an error, or is evicted from the task map.

**Parameters:** none

**Return Value:** `void`

**Key Logic:**
- Selects only `viewingAgentTaskId` and the viewed task from AppState (avoids re-rendering on unrelated streaming updates).
- Narrows the task to `InProcessTeammateTask` type.
- Exits if task evicted, status is `killed`, `failed`, or error is present.
- Does NOT exit if status is `running`, `completed`, or `pending`.

**Dependencies:** `useEffect`, `useAppState`, `useSetAppState`, `exitTeammateView`

---

### `useBackgroundTaskNavigation`

**File:** `hooks/useBackgroundTaskNavigation.ts`

**Purpose:** Manages keyboard navigation of the background task list. Shift+Up/Down moves selection; Enter enters the view; `f` opens the full transcript; `k` kills the task; Escape exits.

**Parameters:** `options?: { isActive?: boolean }`

**Return Value:** `{ handleKeyDown: (key: Key, input: string) => void }`

**Key Logic:**
- Reads the list of background tasks from AppState.
- `selectedIndex` is clamped to `[0, tasks.length - 1]` whenever the list changes.
- Enter: sets `viewingAgentTaskId` in AppState to the selected task's ID.
- `f`: sets `showFullTranscript = true` in AppState.
- `k`: calls `killTask(task.id)`.
- Escape: calls `exitTeammateView(setAppState)`.

**Dependencies:** `useAppState`, `useSetAppState`, `useState`, `useEffect`, `useCallback`

---

### `useInboxPoller`

**File:** `hooks/useInboxPoller.ts`

**Purpose:** Polls the team lead's inbox every 1 second (or on demand when idle) and routes messages. Handles permission requests/responses, sandbox permissions, plan approvals, shutdown handling, team permission updates, mode-set requests, and regular messages.

**Parameters:**
- `{ enabled: boolean, isLoading: boolean, focusedInputDialog: string | null, onSubmitMessage: (msg) => void }`

**Return Value:** `void`

**Key Logic:**
- Uses `useInterval` with 1 000 ms period; no-ops if `!enabled`.
- Reads messages from the team lead's mailbox directory.
- Dispatches by message type:
  - `permission_request` → adds to `toolUseConfirmQueue`.
  - `permission_response` → calls `processMailboxPermissionResponse`.
  - `sandbox_permission_request` → calls sandbox permission handler.
  - `sandbox_permission_response` → calls `processSandboxPermissionResponse`.
  - `plan_approval` → routes to plan approval handler.
  - `shutdown` → gracefully exits.
  - `team_permission_update` → updates AppState permission context.
  - `mode_set` → changes permission mode.
  - Regular messages → calls `onSubmitMessage` when idle.
- Delivers pending messages only when `!isLoading` and no focused input dialog.

**Dependencies:** `useInterval`, `useEffect`, `useRef`, `useAppState`, `useSetAppState`, `processMailboxPermissionResponse`, `processSandboxPermissionResponse`

---

### `useTaskListWatcher`

**File:** `hooks/useTaskListWatcher.ts`

**Purpose:** Watches a task list directory and automatically picks up open, unowned tasks to work on (tasks mode). Claims tasks atomically to prevent race conditions.

**Parameters:**
- `{ taskListId?: string, isLoading: boolean, onSubmitTask: (prompt: string) => boolean }`

**Return Value:** `void`

**Key Logic:**
- Calls `ensureTasksDir` on mount, then `watch(tasksDir, debouncedCheck)` with DEBOUNCE_MS=1000.
- Uses stable refs for `isLoading` and `onSubmitTask` to avoid Bun PathWatcherManager deadlock (oven-sh/bun#27469) by not recreating the watcher on every turn.
- `checkForTasks`: lists tasks, finds `status=pending`, `owner=undefined`, all `blockedBy` completed; calls `claimTask(taskListId, task.id, agentId)`.
- Formats task as `"Complete all open tasks. Start with task #N: ...\n\nDescription"`.
- Additional `useEffect` on `isLoading` to trigger check when going idle.

**Dependencies:** `fs.watch`, `useEffect`, `useRef`

---

### `useTasksV2`

**File:** `hooks/useTasksV2.ts`

**Purpose:** Exposes the current task list for the persistent TodoV2 UI. All consumers share a single `TasksV2Store` (singleton file-watcher) to avoid watcher churn.

**Parameters:** none

**Return Value:** `Task[] | undefined` — `undefined` when hidden (all completed for >5 s, or empty).

**Key Logic:**
- `TasksV2Store` class: manages `fs.watch`, `onTasksUpdated` subscription, debounced fetch (DEBOUNCE_MS=50), hide timer (HIDE_DELAY_MS=5000), fallback poll (FALLBACK_POLL_MS=5000).
- `getSnapshot` returns `undefined` when `#hidden = true`.
- `useSyncExternalStore` subscription; store starts on first subscriber, stops on last unsubscribe.
- Only active when `isTodoV2Enabled()` and (no team context, or is team lead).
- `useTasksV2WithCollapseEffect`: same as `useTasksV2` plus collapses the expanded task view in AppState when the list becomes hidden.

**Dependencies:** `useSyncExternalStore`, `useEffect`, `useAppState`, `useSetAppState`, `fs.watch`

---

### `useSessionBackgrounding`

**File:** `hooks/useSessionBackgrounding.ts`

**Purpose:** Manages Ctrl+B backgrounding and foregrounding of the current session. When a task is foregrounded, it syncs that task's messages to the main message list.

**Parameters:** (large props including setMessages, setIsLoading, tools, etc.)

**Return Value:** `{ handleBackgroundSession: () => void }`

**Key Logic:**
- On `handleBackgroundSession`: if a task is running, backgrounds it (writes to AppState background tasks); otherwise foregrounds the first background task.
- Foreground: injects the backgrounded task's messages into the main list via `setMessages`, resumes the task's streams.

**Dependencies:** `useCallback`, `useAppState`, `useSetAppState`

---

### `useScheduledTasks`

**File:** `hooks/useScheduledTasks.ts`

**Purpose:** Mounts the cron scheduler in the REPL. Fired tasks are enqueued via `enqueuePendingNotification` at `later` priority; teammate-scoped crons are injected directly into that teammate's message stream.

**Parameters:**
- `{ isLoading: boolean, assistantMode?: boolean, setMessages: Dispatch<SetStateAction<Message[]>> }`

**Return Value:** `void`

**Key Logic:**
- Gated on `isKairosCronEnabled()` at effect time.
- Uses `isLoadingRef` to avoid stale closures on `isLoading`.
- `onFireTask` callback: if task has `agentId`, finds the teammate and calls `injectUserMessageToTeammate`; otherwise creates a `ScheduledTaskFireMessage` and enqueues the prompt.
- `createCronScheduler` is the shared scheduler core (used also by `print.ts` for headless mode).
- `isKilled` callback polls `isKairosCronEnabled()` each tick as a mid-session killswitch.

**Dependencies:** `useEffect`, `useRef`, `useAppStateStore`, `useSetAppState`, `createCronScheduler`

---

## IDE Integration Hooks

### `useIDEIntegration`

**File:** `hooks/useIDEIntegration.tsx`

**Purpose:** Manages IDE auto-connection on startup. Detects running IDEs, sets up dynamic MCP config for the found IDE server, and shows the IDE onboarding dialog if needed.

**Parameters:**
- `{ autoConnectIdeFlag, ideToInstallExtension, setDynamicMcpConfig, setShowIdeOnboarding, setIDEInstallationState }`

**Return Value:** `void`

**Key Logic:**
- Calls `detectIDEs()` on mount to find running IDE extension servers.
- If found: calls `setDynamicMcpConfig` to add the IDE's MCP server config.
- Checks `settings.ideHintShownCount` to decide whether to show onboarding.
- Handles the `ideToInstallExtension` CLI flag for direct IDE extension install flows.

**Dependencies:** `useEffect`, `useRef`, `useAppState`

---

### `useIdeAtMentioned`

**File:** `hooks/useIdeAtMentioned.ts`

**Purpose:** Listens for `at_mentioned` MCP notifications from the IDE extension and calls a callback with file/line context so Claude can reference the file.

**Parameters:**
- `mcpClients: MCPServerConnection[]`
- `onAtMentioned: (filePath: string, lineNumber?: number) => void`

**Return Value:** `void`

**Key Logic:**
- Uses `getConnectedIdeClient(mcpClients)` to find the IDE client.
- Registers a notification handler for the `at_mentioned` method via `ideClient.client.setNotificationHandler`.
- Passes parsed `filePath` and `lineNumber` to `onAtMentioned`.

**Dependencies:** `useEffect`

---

### `useIdeConnectionStatus`

**File:** `hooks/useIdeConnectionStatus.ts`

**Purpose:** Returns the current IDE connection status (`connected`, `disconnected`, `pending`, or `null`) and the IDE name.

**Parameters:**
- `mcpClients?: MCPServerConnection[]`

**Return Value:** `{ status: IDEConnectionStatus | null, ideName: string | null }`

**Key Logic:**
- Uses `useMemo` over `mcpClients` to find the IDE client by checking `isIdeClient(client)`.
- Maps client connection state to the status enum.

**Dependencies:** `useMemo`

---

### `useIdeLogging`

**File:** `hooks/useIdeLogging.ts`

**Purpose:** Registers a `log_event` MCP notification handler on the IDE client to forward IDE telemetry events to the analytics system.

**Parameters:**
- `mcpClients: MCPServerConnection[]`

**Return Value:** `void`

**Key Logic:**
- Calls `getConnectedIdeClient` to find the IDE client.
- Registers a Zod-validated handler: `{ method: 'log_event', params: { eventName, eventData } }`.
- Calls `logEvent('tengu_ide_${eventName}', eventData)`.

**Dependencies:** `useEffect`, `zod`

---

### `useIdeSelection`

**File:** `hooks/useIdeSelection.ts`

**Purpose:** Listens for `selection_changed` MCP notifications from the IDE and delivers them as `IDESelection` objects to the REPL.

**Parameters:**
- `mcpClients: MCPServerConnection[]`
- `onSelect: (selection: IDESelection) => void`

**Return Value:** `void`

**Key Logic:**
- Finds IDE client, registers notification handler for `selection_changed`.
- Converts the raw notification payload to `IDESelection` format `{ filePath, text, lineStart, lineEnd, lineCount }`.

**Dependencies:** `useEffect`

---

### `useDiffInIDE`

**File:** `hooks/useDiffInIDE.ts`

**Purpose:** Opens a file diff in the connected IDE via MCP RPC. Handles user save/close/reject responses to finalize or revert the edit.

**Parameters:**
- `{ onChange, toolUseContext, filePath, edits, editMode }`

**Return Value:** `{ closeTabInIDE, showingDiffInIDE, ideName, hasError }`

**Key Logic:**
- Calls `ideClient.client.request('show_diff', { filePath, edits, ... })` to open the diff in the IDE.
- Waits for the IDE to respond with `saved`, `closed`, or `rejected`.
- On `saved`: calls `onChange` to apply the edit.
- On `rejected`: calls the abort controller.
- Returns `closeTabInIDE()` so the permission dialog can close the tab programmatically.

**Dependencies:** `useState`, `useRef`, `useCallback`, `useEffect`

---

## Remote & Session Hooks

### `useDirectConnect`

**File:** `hooks/useDirectConnect.ts`

**Purpose:** Manages a WebSocket connection to a DirectConnect server (local server mode). Routes inbound messages and permission requests between the server and the REPL.

**Parameters:**
- `{ config, setMessages, setIsLoading, setToolUseConfirmQueue, tools }`

**Return Value:** `UseDirectConnectResult` — `{ isConnected, send }`

**Key Logic:**
- Uses `directConnectManager` to manage the WebSocket lifecycle.
- Translates inbound SDK messages to `Message[]` format.
- Handles tool permission requests by pushing to `setToolUseConfirmQueue`.
- Reconnects on disconnect with exponential backoff.

**Dependencies:** `useEffect`, `useRef`, `useState`

---

### `useSSHSession`

**File:** `hooks/useSSHSession.ts`

**Purpose:** Wires an SSH session manager to the REPL. Handles reconnection, graceful shutdown on disconnect, and transcript message injection.

**Parameters:**
- `{ session, setMessages, setIsLoading, setToolUseConfirmQueue, tools }`

**Return Value:** `UseSSHSessionResult` — `{ isConnected, disconnect }`

**Key Logic:**
- Subscribes to SSH session events: `message`, `connect`, `disconnect`, `error`.
- On disconnect: injects a system message explaining the disconnect and showing reconnect options.
- On reconnect: injects a system message confirming reconnection.
- Handles graceful shutdown: drains pending messages before closing.

**Dependencies:** `useEffect`, `useRef`, `useState`

---

### `useRemoteSession`

**File:** `hooks/useRemoteSession.ts`

**Purpose:** Full CCR (Claude Code Remote) WebSocket session management. Handles bidirectional message conversion, streaming tool uses, permission request/response flow, response timeout detection, session title updates, and subagent task counting.

**Parameters:** Large props object including:
- `config: AppConfig`
- `setMessages: SetMessages`
- `setIsLoading`, `setToolUseConfirmQueue`
- `tools: Tool[]`
- `onSessionTitleUpdate?: (title: string) => void`

**Return Value:** `UseRemoteSessionResult` — `{ isConnected, sendMessage, sessionId, ... }`

**Key Logic:**
- Connects to CCR WebSocket on mount, reconnects on disconnect.
- Converts `SDKMessage` types to internal `Message` format on inbound.
- Converts outbound messages to SDK format for CCR consumption.
- Manages streaming tool uses: accumulates `input_json_delta` chunks, fires permission dialog on `tool_use` completion.
- Response timeout: sets a flag after 30 s of no response from the model.
- Session title: subscribes to `session_title_update` events and calls `onSessionTitleUpdate`.
- Subagent task counting: tracks `subagent_start` / `subagent_end` events to count running agents.

**Dependencies:** `useEffect`, `useState`, `useRef`, `useCallback`, `SessionsWebSocket`

---

### `useAssistantHistory`

**File:** `hooks/useAssistantHistory.ts`

**Purpose:** Lazy-loads older messages from a remote session's history as the user scrolls up in viewer-only mode.

**Parameters:**
- `{ config, setMessages, scrollRef, onPrepend }`

**Return Value:** `{ maybeLoadOlder: () => Promise<void> }`

**Key Logic:**
- On scroll-up event, calls `loadSessionHistory(sessionId, pageToken)`.
- Prepends loaded messages to the message list.
- Chains viewport fill: if the loaded messages don't fill the viewport, immediately loads another page.
- Scroll anchoring: saves the current scroll position before prepending and restores it after.
- Shows a sentinel message at the top ("Beginning of conversation") once all pages are exhausted.

**Dependencies:** `useCallback`, `useRef`, `useEffect`

---

### `useMailboxBridge`

**File:** `hooks/useMailboxBridge.ts`

**Purpose:** Bridges the mailbox message context to the REPL's submit function. Polls the mailbox on revision change when idle.

**Parameters:**
- `{ isLoading: boolean, onSubmitMessage: (msg) => void }`

**Return Value:** `void`

**Key Logic:**
- Subscribes to `mailboxRevision` changes in AppState.
- When revision bumps and `!isLoading`: calls `pollMailbox()` and submits any pending messages via `onSubmitMessage`.

**Dependencies:** `useEffect`, `useRef`, `useAppState`

---

### `useReplBridge`

**File:** `hooks/useReplBridge.tsx`

**Purpose:** Full REPL bridge session management — the main hook wiring the REPL to the Claude API. Manages the full query execution loop, permission flow, streaming message assembly, compact operations, and bridge connectivity.

**Parameters:** Large props including tools, messages, setMessages, config, and many callbacks.

**Return Value:** Large result including `{ onQuery, isLoading, abortController, toolUseConfirmQueue, ... }`

**Key Logic:** (file is >75k tokens; key points from reading the first 80 lines and the summary)
- Manages `abortController` lifecycle — creates a new one per query, aborts on cancel.
- Calls the streaming Claude API via `query.ts`.
- Assembles streaming `AssistantMessage` from delta events.
- Routes `tool_use` to `canUseTool` for permission gating.
- Handles compact boundary detection and auto-compact triggers.
- Integrates bridge callbacks for CCR permission relaying.
- Manages local session history recording.

**Dependencies:** `useState`, `useRef`, `useCallback`, `useEffect`, `useMemo`, `useCanUseTool`, `useLogMessages`, `query`, `compact`

---

### `useTeleportResume`

**File:** `hooks/useTeleportResume.tsx`

**Purpose:** Manages the async lifecycle of teleporting into a remote Code Session: loading state, error state, selected session tracking, and the `resumeSession` callback.

**Parameters:**
- `source: TeleportSource` — `'cliArg' | 'localCommand'` (for analytics).

**Return Value:** `{ resumeSession, isResuming, error, selectedSession, clearError }`

**Key Logic:**
- `resumeSession(session)`: sets `isResuming = true`, logs `tengu_teleport_resume_session`, calls `teleportResumeCodeSession(session.id)`, sets `teleportedSessionInfo` for reliability logging.
- On error: wraps in `TeleportResumeError` with `isOperationError` flag for UI differentiation.
- Uses React Compiler (`_c`) memoization.

**Dependencies:** `useState`, `useCallback`, `teleportResumeCodeSession`, `setTeleportedSessionInfo`

---

## Plugin & Suggestion Hooks

### `usePromptSuggestion`

**File:** `hooks/usePromptSuggestion.ts`

**Purpose:** Manages AI prompt completion suggestions: fetches a suggestion for the current input, tracks accept/ignore/submit outcomes, and logs telemetry.

**Parameters:**
- `{ inputValue: string, isAssistantResponding: boolean }`

**Return Value:** `{ suggestion: string | null, markAccepted, markShown, logOutcomeAtSubmission }`

**Key Logic:**
- Calls `generatePromptSuggestion(inputValue)` debounced after 300 ms of no typing.
- `markAccepted()`: records that the user pressed Tab to accept.
- `markShown()`: records when the ghost-text suggestion becomes visible.
- `logOutcomeAtSubmission()`: called on submit; logs `accept` (Tab was pressed), `ignore` (shown but not accepted), or `no_suggestion` outcome.

**Dependencies:** `useState`, `useRef`, `useCallback`, `useEffect`, `generatePromptSuggestion`

---

### `usePromptsFromClaudeInChrome`

**File:** `hooks/usePromptsFromClaudeInChrome.tsx`

**Purpose:** Listens for prompts sent from the Claude in Chrome extension via MCP notifications. Also syncs the current permission mode to the extension.

**Parameters:**
- `mcpClients: MCPServerConnection[]`
- `toolPermissionMode: PermissionMode`

**Return Value:** `void`

**Key Logic:**
- Finds the Chrome extension MCP client.
- Registers a notification handler for `prompt_from_chrome`.
- Submits received prompts to the command queue.
- On `toolPermissionMode` change: sends a `mode_changed` notification back to the extension.

**Dependencies:** `useEffect`, `useRef`

---

### `usePrStatus`

**File:** `hooks/usePrStatus.ts`

**Purpose:** Polls `gh pr status` every 60 seconds to detect review state changes on the current branch's PR.

**Parameters:**
- `isLoading: boolean`
- `enabled?: boolean`

**Return Value:** `PrStatusState` — `{ reviewStatus: 'approved'|'changes_requested'|'pending'|null, prUrl: string | null }`

**Key Logic:**
- Uses `useInterval` with 60 000 ms period; skips poll when `isLoading`.
- Stops polling after 60 minutes of idle time (no new turns).
- Permanently disables if a fetch takes >4 seconds (likely no `gh` binary or no PR).
- Runs `gh pr status --json reviewDecision,url` in a subprocess.

**Dependencies:** `useInterval`, `useState`, `useRef`

---

### `useClaudeCodeHintRecommendation`

**File:** `hooks/useClaudeCodeHintRecommendation.tsx`

**Purpose:** Surfaces plugin install prompts from `<claude-code-hint />` tags parsed from Claude's responses. Show-once semantics per plugin per session.

**Parameters:** none

**Return Value:** `{ recommendation: PluginRecommendation | null, handleResponse: (accepted: boolean) => void }`

**Key Logic:**
- Monitors `messages` for assistant messages containing `<claude-code-hint plugin="name" />` XML.
- Uses `usePluginRecommendationBase` state machine to gate display.
- `handleResponse(true)`: installs the plugin; `false`: dismisses.

**Dependencies:** `usePluginRecommendationBase`, `useAppState`, `useEffect`

---

### `useLspPluginRecommendation`

**File:** `hooks/useLspPluginRecommendation.tsx`

**Purpose:** Recommends an LSP plugin when the user edits a file whose extension matches a supported language and the LSP binary is present.

**Parameters:** none

**Return Value:** `{ recommendation: PluginRecommendation | null, handleResponse: (accepted: boolean) => void }`

**Key Logic:**
- Watches `messages` for `FileEdit` / `FileWrite` tool result messages.
- Extracts the file extension, checks if there's a matching LSP plugin.
- Uses `usePluginRecommendationBase` to avoid showing while another recommendation is active.
- Show-once per session (tracked in AppState).

**Dependencies:** `usePluginRecommendationBase`, `useAppState`, `useEffect`

---

### `usePluginRecommendationBase`

**File:** `hooks/usePluginRecommendationBase.tsx`

**Purpose:** Shared state machine for plugin recommendations. Guards against showing a recommendation while in remote mode, while another is already showing, or while a check is in-flight.

**Parameters:** generic `T`

**Return Value:** `{ recommendation: T | null, clearRecommendation, tryResolve: (candidate: T | null) => void }`

**Key Logic:**
- `tryResolve(candidate)`: if remote mode or already showing, no-op; otherwise sets `recommendation`.
- `clearRecommendation()`: clears the current recommendation.
- Guards `inFlight` ref to prevent concurrent checks.

**Dependencies:** `useState`, `useRef`, `useCallback`

---

### `useOfficialMarketplaceNotification`

**File:** `hooks/useOfficialMarketplaceNotification.tsx`

**Purpose:** Handles official marketplace auto-install on first launch and shows success/failure startup notifications.

**Parameters:** none

**Return Value:** `void`

**Key Logic:**
- On mount: checks `settings.autoInstallOfficialMarketplace` flag.
- If set and not yet installed: calls `installOfficialMarketplace()`.
- Shows a `startup` priority notification on success or failure.

**Dependencies:** `useEffect`, `useRef`, `useNotifications`

---

### `useChromeExtensionNotification`

**File:** `hooks/useChromeExtensionNotification.tsx`

**Purpose:** Shows startup notifications about the Chrome extension status: requires subscription, not installed, or default-enabled.

**Parameters:** none

**Return Value:** `void`

**Key Logic:**
- Uses `useStartupNotification` (notifs/) internally.
- Checks `chrome_extension_status` from settings/config.
- Returns appropriate notification text for each status.

**Dependencies:** `useStartupNotification`

---

### `useClipboardImageHint`

**File:** `hooks/useClipboardImageHint.ts`

**Purpose:** Shows a notification when the terminal gains focus and the clipboard contains an image. Debounced with 1 000 ms delay and a 30-second cooldown.

**Parameters:**
- `isFocused: boolean`
- `enabled: boolean`

**Return Value:** `void`

**Key Logic:**
- Watches `isFocused` changes.
- On focus gain: checks clipboard via `navigator.clipboard.read()` for `image/*` MIME.
- If image found: calls `addNotification({ key: 'clipboard-image', text: 'Image in clipboard · Ctrl+V to attach' })`.
- Cooldown: stores last-shown timestamp to avoid notification spam.

**Dependencies:** `useEffect`, `useRef`, `useNotifications`

---

### `useManagePlugins`

**File:** `hooks/useManagePlugins.ts` — (same as above; see Core section for full details)

---

### `useIssueFlagBanner`

**File:** `hooks/useIssueFlagBanner.ts`

**Purpose:** ANT-internal: shows an issue-flag banner after a session has friction signals (e.g. repeated tool retries) and has been active for at least 30 minutes with 3+ submits.

**Parameters:**
- `messages: readonly Message[]`
- `submitCount: number`

**Return Value:** `boolean` — whether to show the banner.

**Key Logic:**
- Counts tool errors, retries, and refusals in recent messages.
- Only enabled for ANT internal users (checks `isAntInternal()`).
- 30-minute cooldown stored in localStorage.

**Dependencies:** `useMemo`, `useRef`

---

### `useQueueProcessor`

**File:** `hooks/useQueueProcessor.ts`

**Purpose:** Processes queued commands from the unified command queue when no active query is running and no blocking UI is shown.

**Parameters:**
- `{ executeQueuedInput: (cmd: QueuedCommand) => void, hasActiveLocalJsxUI: boolean, queryGuard: () => boolean }`

**Return Value:** `void`

**Key Logic:**
- Uses two `useSyncExternalStore` subscriptions: one for the command queue, one for the "is loading" state.
- When the queue is non-empty, `!isLoading`, `!hasActiveLocalJsxUI`, and `queryGuard()` returns true: dequeues and executes the next command.
- Uses `useEffect` to trigger processing on state changes.

**Dependencies:** `useSyncExternalStore`, `useEffect`, `messageQueueManager`

---

### `useTurnDiffs`

**File:** `hooks/useTurnDiffs.ts`

**Purpose:** Extracts per-turn file diffs from the message list for display in the `/diff` view. Uses incremental processing — only new messages are scanned on each render.

**Parameters:**
- `messages: Message[]`

**Return Value:** `TurnDiff[]` — reverse-chronological list of turns that modified files.

**Key Logic:**
- `TurnDiff`: `{ turnIndex, userPromptPreview, timestamp, files: Map<string, TurnFileDiff>, stats }`.
- Detects turn boundaries from user messages that are not tool results and not `isMeta`.
- Collects `FileEdit`/`FileWrite` tool results within each turn.
- New-file hunks: generates synthetic `+line` hunks from `content`.
- Accumulated across edits to the same file in a turn.
- Cache ref holds `completedTurns` + `currentTurn` + `lastProcessedIndex` for O(n_new) processing.

**Dependencies:** `useMemo`, `useRef`

---

### `useSkillImprovementSurvey`

**File:** `hooks/useSkillImprovementSurvey.ts`

**Purpose:** Manages the skill improvement survey dialog. Triggered by `AppState.skillImprovementSurvey`, applies improvements to the skills file on accept.

**Parameters:**
- `setMessages: SetMessages`

**Return Value:** `{ isOpen: boolean, suggestion: SkillSuggestion | null, handleSelect: (selection) => void }`

**Key Logic:**
- Reads `AppState.skillImprovementSurvey` to get the pending survey.
- `handleSelect('apply')`: calls `applySkillImprovement(suggestion)` and injects a system message.
- `handleSelect('dismiss')`: clears the survey from AppState.
- On any selection: clears `AppState.skillImprovementSurvey`.

**Dependencies:** `useAppState`, `useSetAppState`, `useCallback`

---

### `useGlobalKeybindings`

**File:** `hooks/useGlobalKeybindings.tsx`

**Purpose:** React component (renders `null`) that registers global keybinding handlers for Ctrl+T (toggle todos), Ctrl+O (toggle transcript/messages), Ctrl+E (toggle show-all), and Escape/Ctrl+C (exit transcript).

**Parameters:** Props including `screen`, `isLoading`, `isSearchingHistory`, `isHelpOpen`, etc.

**Return Value:** `null`

**Key Logic:**
- Registers `view:toggleTasks`, `view:toggleTranscript`, `view:toggleShowAll` bindings.
- KAIROS feature flag gates: `view:toggleTasks` only active when `isTodoV2Enabled()`.
- `view:toggleTranscript` sets `expandedView = 'messages'` or `'none'` in AppState.
- `view:toggleShowAll` sets `showAllMessages` in AppState.

**Dependencies:** `useKeybinding`, `useAppState`, `useSetAppState`

---

### `CommandKeybindingHandlers`

**File:** `hooks/useCommandKeybindings.tsx`

**Purpose:** Registers all `command:*` keybinding actions as slash command submitters — e.g., `command:compact`, `command:memory`, `command:config`, etc.

**Parameters:** `{ onSubmit: (cmd: string) => void, isActive?: boolean }`

**Return Value:** `null`

**Key Logic:**
- Calls `useKeybindings` with a map of `command:X` → `() => onSubmit('/X')` entries.
- Derives the slash command name from the keybinding action name.

**Dependencies:** `useKeybindings`

---

### `useAwaySummary`

**File:** `hooks/useAwaySummary.ts`

**Purpose:** Appends a "while you were away" summary message after the terminal has been blurred for 5 minutes, when no turn is in progress.

**Parameters:**
- `messages: readonly Message[]`
- `setMessages: SetMessages`
- `isLoading: boolean`

**Return Value:** `void`

**Key Logic:**
- Gated on `feature('AWAY_SUMMARY')` bundle flag and `tengu_sedge_lantern` GrowthBook flag.
- Subscribes to `subscribeTerminalFocus` for blur/focus events.
- On blur: starts a `BLUR_DELAY_MS = 5 * 60_000` timer.
- Timer fire: if still loading, sets `pendingRef = true` (deferred); otherwise calls `generate()`.
- `generate()`: calls `generateAwaySummary(messages, signal)` → appends `createAwaySummaryMessage(text)`.
- On focus: clears timer, aborts in-flight generation, clears `pendingRef`.
- Second `useEffect` on `isLoading`: if `!isLoading` and `pendingRef` and still blurred, fires `generate()`.
- `hasSummarySinceLastUserTurn()`: walks backward to prevent duplicate summaries.

**Dependencies:** `useEffect`, `useRef`, `useCallback`, `subscribeTerminalFocus`, `generateAwaySummary`

---

### `useAfterFirstRender` / `renderPlaceholder`

See entry under "Core / Utility Hooks" for `useAfterFirstRender`.

---

## Notification Hooks (`notifs/`)

All hooks in `notifs/` use `useNotifications()` from `context/notifications.js` to push entries to the status bar notification queue. Most are gated on `!getIsRemoteMode()`.

---

### `useStartupNotification`

**File:** `hooks/notifs/useStartupNotification.ts`

**Purpose:** Base primitive for fire-once-on-mount notifications. Encapsulates the remote-mode gate and once-per-session ref guard used by most other `notifs/` hooks.

**Parameters:**
- `compute: () => Result | Promise<Result>` — returns `null` to skip, `Notification` for one, `Notification[]` for many.

**Return Value:** `void`

**Key Logic:**
- `hasRunRef` prevents re-firing on re-render.
- Runs `compute` inside `Promise.resolve().then(...)` to allow async.
- Catches errors via `logError`.
- Skips entirely in remote mode.

**Dependencies:** `useEffect`, `useRef`, `useNotifications`

---

### `useAutoModeUnavailableNotification`

**File:** `hooks/notifs/useAutoModeUnavailableNotification.ts`

**Purpose:** Shows a one-shot warning when the Shift+Tab mode carousel wraps past where "auto mode" would have been, explaining why auto mode is unavailable.

**Parameters:** none

**Key Logic:**
- Detects the wrap: `mode === 'default' && prevMode !== 'default' && prevMode !== 'auto' && !isAutoModeAvailable && hasAutoModeOptIn()`.
- Calls `getAutoModeUnavailableReason()` to get the specific reason (circuit-breaker, org-allowlist, settings).
- `shownRef` prevents showing more than once per session.
- Gated on `feature('TRANSCRIPT_CLASSIFIER')`.

---

### `useCanSwitchToExistingSubscription`

**File:** `hooks/notifs/useCanSwitchToExistingSubscription.tsx`

**Purpose:** Shows up to 3 times (MAX_SHOW_COUNT=3) across sessions a notification prompting users who have a Claude Pro/Max subscription but are logged in via API key to run `/login`.

**Key Logic:**
- Reads `globalConfig.subscriptionNoticeCount`; returns null if >= 3.
- Calls `getOauthProfileFromApiKey()` to check for Pro/Max subscription.
- Increments `subscriptionNoticeCount` in global config on each show.
- Renders a JSX notification with `color="suggestion"`.

---

### `useDeprecationWarningNotification`

**File:** `hooks/notifs/useDeprecationWarningNotification.tsx`

**Purpose:** Shows a `color="warning"` notification when the active model is deprecated.

**Parameters:** `model: string`

**Key Logic:**
- Calls `getModelDeprecationWarning(model)` on each `model` change.
- Uses `lastWarningRef` to avoid re-adding the same notification on re-render.
- Resets tracking if model changes to non-deprecated.

---

### `useFastModeNotification`

**File:** `hooks/notifs/useFastModeNotification.tsx`

**Purpose:** Shows real-time notifications for fast mode state changes: cooldown started/expired, org-level enable/disable, and overage rejection.

**Key Logic:**
- Subscribes to `onCooldownTriggered`, `onCooldownExpired`, `onFastModeOverageRejection`, `onOrgFastModeChanged` event emitters.
- On org-disabled while fast mode is active: disables fast mode in AppState.
- Shows immediate-priority notifications with `color="fastMode"` or `color="warning"`.

---

### `useIDEStatusIndicator`

**File:** `hooks/notifs/useIDEStatusIndicator.tsx`

**Purpose:** Shows IDE connection status in the notification area: hint to install extension, JetBrains info, install error, or current selection preview.

**Parameters:** `{ ideInstallationStatus, ideSelection, mcpClients }`

**Key Logic:**
- Uses `useIdeConnectionStatus(mcpClients)` to get current status.
- Shows "install extension" hint up to MAX_IDE_HINT_SHOW_COUNT=5 times (tracked in globalConfig).
- Shows JetBrains info notification (different flow than VS Code).
- Shows selection preview as a persistent notification when file/text is selected.

---

### `useInstallMessages`

**File:** `hooks/notifs/useInstallMessages.tsx`

**Purpose:** Shows startup notifications for native installer issues (PATH not configured, alias not set, install errors).

**Key Logic:**
- Calls `checkInstall()` to get installation messages.
- Maps message types to priorities: `error`/`userActionRequired` → `high`; `path`/`alias` → `medium`; others → `low`.
- Colors: `error` → `color="error"`; others → `color="warning"`.

---

### `useLspInitializationNotification`

**File:** `hooks/notifs/useLspInitializationNotification.tsx`

**Purpose:** Polls LSP server status every 5 000 ms and shows notifications when the LSP manager or individual servers fail to initialize.

**Key Logic:**
- Gated on `ENABLE_LSP_TOOL` env var.
- Polls `getInitializationStatus()` and `getLspServerManager()`.
- De-duplicates errors using `notifiedErrorsRef` set.
- Adds errors to `appState.plugins.errors` for `/doctor` display.

---

### `useMcpConnectivityStatus`

**File:** `hooks/notifs/useMcpConnectivityStatus.tsx`

**Purpose:** Shows notifications when MCP servers fail to connect or need authentication.

**Parameters:** `{ mcpClients?: MCPServerConnection[] }`

**Key Logic:**
- Filters `mcpClients` by connection state: `failed`, `needs_auth`.
- Separately tracks `claudeai` clients (connector) vs local clients (server).
- Shows JSX notifications with counts and `· /mcp` navigation hint.

---

### `useModelMigrationNotifications`

**File:** `hooks/notifs/useModelMigrationNotifications.tsx`

**Purpose:** Shows one-time notifications immediately after automatic model migrations (e.g. Sonnet 4.5 → 4.6, Opus Pro → Opus 4.6).

**Key Logic:**
- Uses `useStartupNotification` with a `MIGRATIONS` array of check functions.
- Each check reads a timestamp field from `globalConfig` and returns a notification if the timestamp is within the last 3 seconds (i.e., this is the launch that triggered the migration).

---

### `useNpmDeprecationNotification`

**File:** `hooks/notifs/useNpmDeprecationNotification.tsx`

**Purpose:** Shows a 15-second warning notification when Claude Code is running via an npm install (deprecated) rather than the native installer.

**Key Logic:**
- Skips if `isInBundledMode()` or `DISABLE_INSTALLATION_CHECKS` env var is set.
- Calls `getCurrentInstallationType()`; skips for `'development'` installs.

---

### `usePluginAutoupdateNotification`

**File:** `hooks/notifs/usePluginAutoupdateNotification.tsx`

**Purpose:** Subscribes to `onPluginsAutoUpdated` and shows a notification prompting the user to run `/reload-plugins` when plugins have been auto-updated in the background.

**Key Logic:**
- `useState([])` for `updatedPlugins` list.
- `onPluginsAutoUpdated` subscription fires with updated plugin IDs.
- Extracts plugin names (strips `@marketplace` suffix) and shows JSX notification with `color="success"`.
- 10 000 ms timeout.

---

### `usePluginInstallationStatus`

**File:** `hooks/notifs/usePluginInstallationStatus.tsx`

**Purpose:** Shows a notification when one or more plugins fail to install (from AppState `plugins.installationStatus`).

**Key Logic:**
- Reads `installationStatus.marketplaces` and `installationStatus.plugins` from AppState.
- Filters by `status === 'failed'`, memoizes counts.
- Shows `"N plugins failed to install · /plugin for details"` with `priority: 'medium'`.

---

### `useRateLimitWarningNotification`

**File:** `hooks/notifs/useRateLimitWarningNotification.tsx`

**Purpose:** Shows rate limit warnings: (1) immediate notification when entering overage mode; (2) warning notification when approaching usage limits.

**Parameters:** `model: string`

**Key Logic:**
- `useClaudeAiLimits()` for reactive limit data.
- `getRateLimitWarning(limits, model)` — string describing approaching limit.
- `getUsingOverageText(limits)` — string for overage mode.
- Overage notification shown once per overage entry (tracked via `hasShownOverageNotification` state).
- Team/enterprise: skips overage notification unless user has billing access.
- Warning notification shown only when text changes (deduped via `shownWarningRef`).

---

### `useSettingsErrors`

**File:** `hooks/notifs/useSettingsErrors.tsx`

**Purpose:** Watches for settings validation errors (from `getSettingsWithAllErrors`) and shows/removes a warning notification in the status bar.

**Return Value:** `ValidationError[]` — the current list of errors (also used for /doctor display).

**Key Logic:**
- Initial state populated synchronously from `getSettingsWithAllErrors()`.
- `useSettingsChange` subscription: re-reads errors on file change.
- Shows `"Found N settings issues · /doctor for details"` with 60 000 ms timeout.
- Removes notification when errors clear.

---

### `useTeammateShutdownNotification` / `useTeammateLifecycleNotification`

**File:** `hooks/notifs/useTeammateShutdownNotification.ts`

**Purpose:** Fires batched spawn/shutdown notifications when in-process teammates start or complete. Uses fold() to combine `"1 agent spawned"` + `"1 agent spawned"` into `"2 agents spawned"`.

**Key Logic:**
- Reads `tasks` from AppState.
- Tracks seen running/completed IDs in `seenRunningRef` / `seenCompletedRef`.
- `makeSpawnNotif(count)` / `makeShutdownNotif(count)` with 5 000 ms timeout and fold function.
- Exported as `useTeammateLifecycleNotification`.

---

## Tool Permission Subsystem (`toolPermission/`)

### `PermissionContext.ts`

**File:** `hooks/toolPermission/PermissionContext.ts`

**Purpose:** Factory for creating a `PermissionContext` object — the shared context passed to all three permission handlers. Contains all callbacks needed to approve, deny, log, queue, and persist permission decisions.

**Exported Functions:**
- `createPermissionContext(tool, input, toolUseContext, assistantMessage, toolUseID, setToolPermissionContext, queueOps?) → PermissionContext`
- `createPermissionQueueOps(setToolUseConfirmQueue) → PermissionQueueOps` — bridges React state setter to the generic queue interface.
- `createResolveOnce<T>(resolve) → ResolveOnce<T>` — atomic check-and-mark-as-resolved guard for races.

**PermissionContext methods:**
- `logDecision(args, opts?)` — delegates to `logPermissionDecision`.
- `logCancelled()` — logs `tengu_tool_use_cancelled` event.
- `persistPermissions(updates)` — calls `persistPermissionUpdates` and updates AppState.
- `resolveIfAborted(resolve)` — short-circuits if abort signal is fired.
- `cancelAndAbort(feedback?, isAbort?, contentBlocks?)` — builds a deny decision and aborts the controller if appropriate.
- `tryClassifier(pendingCheck, updatedInput)` — awaits classifier auto-approval (bash only; `BASH_CLASSIFIER` feature flag).
- `runHooks(permissionMode, suggestions, updatedInput?, startTimeMs?)` — executes PermissionRequest hooks sequentially.
- `buildAllow(updatedInput, opts?)` → `PermissionAllowDecision`
- `buildDeny(message, reason)` → `PermissionDenyDecision`
- `handleUserAllow(updatedInput, permissionUpdates, feedback?, startTimeMs?, contentBlocks?, decisionReason?)` — persists updates, logs, returns allow decision.
- `handleHookAllow(finalInput, permissionUpdates, startTimeMs?)` — same for hook-sourced allows.
- `pushToQueue(item)`, `removeFromQueue()`, `updateQueueItem(patch)` — queue management via `queueOps`.

---

### `permissionLogging.ts`

**File:** `hooks/toolPermission/permissionLogging.ts`

**Purpose:** Centralized analytics and telemetry logging for all tool permission decisions. Fans out to Statsig (logEvent), OTel telemetry, code-edit metrics, and the `toolUseContext.toolDecisions` map.

**Exported Functions:**
- `logPermissionDecision(ctx, args, startTimeMs?)` — main entry point.
- `isCodeEditingTool(toolName)` — checks if tool is Edit/Write/NotebookEdit.
- `buildCodeEditToolAttributes(tool, input, decision, source)` — builds OTel attributes including language from file path.

**Analytics Events:**
- `tengu_tool_use_granted_in_config` — auto-approved by settings allowlist.
- `tengu_tool_use_granted_in_prompt_permanent` / `_temporary` — user approved.
- `tengu_tool_use_granted_by_permission_hook` — hook approved.
- `tengu_tool_use_granted_by_classifier` — classifier approved.
- `tengu_tool_use_rejected_in_prompt` — any rejection.
- `tengu_tool_use_denied_in_config` — denied by settings denylist.

---

### `handlers/coordinatorHandler.ts`

**File:** `hooks/toolPermission/handlers/coordinatorHandler.ts`

**Purpose:** Handles the coordinator-worker permission flow: runs hooks then classifier (both awaited sequentially) before falling through to the interactive dialog.

**Exported:** `handleCoordinatorPermission(params) → Promise<PermissionDecision | null>`

**Parameters:** `CoordinatorPermissionParams` — `{ ctx, pendingClassifierCheck?, updatedInput, suggestions, permissionMode }`

**Logic:**
1. `await ctx.runHooks(...)` — if hooks return a decision, return it.
2. If `BASH_CLASSIFIER` flag: `await ctx.tryClassifier?.(...)` — if classifier returns, return it.
3. Return `null` → caller falls through to `handleInteractivePermission`.
4. On unexpected error: logs and returns null (graceful fallback to dialog).

---

### `handlers/interactiveHandler.ts`

**File:** `hooks/toolPermission/handlers/interactiveHandler.ts`

**Purpose:** Handles the interactive (main-agent) permission flow. Sets up the `ToolUseConfirm` queue entry with all callbacks and races user interaction against background automated checks (hooks, classifier, bridge, channel).

**Exported:** `handleInteractivePermission(params, resolve) → void` (synchronous setup)

**Key Logic:**
- Creates a `PermissionConfirm` queue entry with callbacks: `onAbort`, `onAllow`, `onReject`, `recheckPermission`, `onUserInteraction`, `onDismissCheckmark`.
- `createResolveOnce` guard ensures only the first resolution wins.
- `userInteracted` flag: prevents classifier from auto-approving after user interaction (200 ms grace period).
- **Race 1 — User**: `onAllow`/`onReject`/`onAbort` callbacks.
- **Race 2 — Hooks**: async `ctx.runHooks(...)` — if win, removes from queue, resolves.
- **Race 3 — Classifier**: `executeAsyncClassifierCheck(...)` — on allow, shows checkmark UI for 3 s (focused) or 1 s (blurred), then removes from queue.
- **Race 4 — Bridge (CCR)**: sends `permission_request` to CCR; subscribes to CCR response; on win, logs, resolves.
- **Race 5 — Channel**: sends structured `permission_request` to all active channel MCP servers (Telegram, iMessage); subscribes to response; on win, resolves.
- Checkmark dismissal: `onDismissCheckmark` allows user to press Escape during the checkmark window.
- Abort: if abort signal fires mid-dialog, `claim()` races to resolve with cancel.

---

### `handlers/swarmWorkerHandler.ts`

**File:** `hooks/toolPermission/handlers/swarmWorkerHandler.ts`

**Purpose:** Handles the swarm-worker permission flow: tries classifier auto-approval, then forwards the request to the team leader via mailbox. Awaits the leader's response.

**Exported:** `handleSwarmWorkerPermission(params) → Promise<PermissionDecision | null>`

**Logic:**
1. Returns `null` if not `isAgentSwarmsEnabled()` or not `isSwarmWorker()`.
2. If `BASH_CLASSIFIER`: tries `ctx.tryClassifier?.(...)`.
3. Creates a `Promise<PermissionDecision>` that resolves when the leader responds.
4. Registers `onAllow`/`onReject` callbacks via `registerPermissionCallback`.
5. Calls `sendPermissionRequestViaMailbox(request)` to notify the leader.
6. Sets `AppState.pendingWorkerRequest` for visual indicator.
7. On abort: resolves with `cancelAndAbort`.
8. On error: returns `null` (fallback to local UI handling).

---

## Non-Hook Utilities in `hooks/`

These files live in `hooks/` but are not React hooks.

### `fileSuggestions.ts`

**File:** `hooks/fileSuggestions.ts`

**Exports:**
- `generateFileSuggestions(query, cwd, ...) → Promise<SuggestionItem[]>` — main entry point. Manages a `FileIndex` singleton (native Rust/nucleo), fetches tracked files via `git ls-files`, falls back to `ripgrep`. Returns up to 15 scored matches.
- `startBackgroundCacheRefresh(cwd)` — queues a background refresh of untracked files.
- `clearFileSuggestionCaches()` — called on `/clear` to reset the index.
- `applyFileSuggestion(suggestion, inputValue, cursorOffset) → string` — replaces the `@token` in the input with the chosen file path.
- `findLongestCommonPrefix(suggestions) → string` — used for Tab-autocomplete of common prefix.
- `onIndexBuildComplete(callback)` — notifies when background index is built.

**Key Design:**
- Path signature (mtime + size) invalidates cache without full rebuild.
- `.ignore` / `.rgignore` file support.
- Directory name extraction for `@dir/` completions.

---

### `unifiedSuggestions.ts`

**File:** `hooks/unifiedSuggestions.ts`

**Exports:**
- `generateUnifiedSuggestions(query, mcpResources, agents, showOnEmpty) → Promise<SuggestionItem[]>` — merges file suggestions (nucleo), MCP resource suggestions (Fuse.js), and agent suggestions into a ranked list of up to 15 items.

**Key Design:**
- File suggestions use nucleo score (0–1 float).
- MCP resource suggestions use Fuse.js score (lower = better, inverted for sorting).
- Agent suggestions always appended at lower priority.

---

### `renderPlaceholder.ts`

**File:** `hooks/renderPlaceholder.ts`

**Exports:**
- `renderPlaceholder(placeholder, hidePlaceholderText?, cursorChar?) → string` — pure function that renders placeholder text with a cursor character appended. When `hidePlaceholderText = true` (voice recording mode), returns only the cursor.
