# Claude Code — Components: Core & Messages

This document covers every component in `src/components/` (top-level files) and `src/components/messages/` (including `UserToolResultMessage/` sub-directory).

---

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Top-Level Components](#top-level-components)
3. [messages/ Subdirectory](#messages-subdirectory)
4. [messages/UserToolResultMessage/ Subdirectory](#messagesusertooltoolresultmessage-subdirectory)

---

## Architecture Overview

All components are compiled with the **React Compiler** (`react/compiler-runtime`). The `_c(N)` cache allocator and `Symbol.for("react.memo_cache_sentinel")` guard pattern appear in virtually every component — this is automatic memoization, not hand-written.

The UI framework is **Ink** (terminal React renderer). Common Ink primitives used across all components:

- `Box`, `Text` — layout and text
- `useInput`, `useTheme`, `useTerminalFocus`, `useAnimationFrame`, `useTheme` — Ink hooks
- `Ansi`, `RawAnsi`, `NoSelect`, `Link` — special rendering nodes
- `ScrollBox`, `ScrollBoxHandle` — scrollable region

Feature flags are evaluated at compile time via `feature('FLAG_NAME')` from `bun:bundle`. Dead code is eliminated in external builds.

Global state is accessed via `useAppState`, `useSetAppState`, `useAppStateStore` from `src/state/AppState.js`.

---

## Top-Level Components

---

### App.tsx

**Purpose:** Top-level React provider wrapper that nests all global context providers.

**Exports:** `App`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `getFpsMetrics` | `() => FpsMetrics \| undefined` | yes | FPS metrics supplier for FpsMetricsProvider |
| `stats` | `StatsStore` | no | Stats store for StatsProvider |
| `initialState` | `AppState` | yes | Initial AppState passed to AppStateProvider |
| `children` | `React.ReactNode` | yes | Content rendered inside providers |

**Provider nesting:** `FpsMetricsProvider > StatsProvider > AppStateProvider`

---

### AgentProgressLine.tsx

**Purpose:** Renders a single line in the coordinator agent progress tree, showing type/name label, status text, tool use count, and token count.

**Exports:** `AgentProgressLine`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `agentType` | `string` | yes | Type identifier of the sub-agent |
| `description` | `string` | no | Human-readable description |
| `name` | `string` | no | Agent name |
| `descriptionColor` | `string` | no | Color for description text |
| `taskDescription` | `string` | no | Current task description |
| `toolUseCount` | `number` | yes | Number of tool uses |
| `tokens` | `number \| null` | yes | Token count (null = not yet known) |
| `color` | `string` | no | Color for type/name label |
| `isLast` | `boolean` | yes | Whether this is the last item (controls `└─` vs `├─`) |
| `isResolved` | `boolean` | yes | Whether task is complete |
| `isError` | `boolean` | yes | Whether task errored |
| `isAsync` | `boolean` | no | Whether agent runs asynchronously |
| `shouldAnimate` | `boolean` | yes | Whether to animate the spinner |
| `lastToolInfo` | `object` | no | Info about most recent tool use |
| `hideType` | `boolean` | no | Suppress type label rendering |

**Key behavior:** Renders tree connector characters `└─` (last) or `├─` (non-last). Shows tool use count suffix and token count.

---

### ApproveApiKey.tsx

**Purpose:** Dialog asking user to approve or reject a custom (user-supplied) API key found in the environment.

**Exports:** `ApproveApiKey`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `customApiKeyTruncated` | `string` | yes | Truncated API key for display |
| `onDone` | `(approved: boolean) => void` | yes | Called with approval result |

**Key behavior:** Saves to `globalConfig.customApiKeyResponses.approved` or `.rejected` depending on selection.

---

### AutoModeOptInDialog.tsx

**Purpose:** Dialog presented to users to opt in or out of auto mode (full agentic mode). Contains legally-reviewed description text.

**Exports:** `AUTO_MODE_DESCRIPTION` (const string), `AutoModeOptInDialog`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `onAccept` | `() => void` | yes | Called when user accepts |
| `onDecline` | `() => void` | yes | Called when user declines |
| `declineExits` | `boolean` | no | Whether declining exits the app |

**Key behavior:** Three options: accept-default (sets `defaultMode:'auto'`), accept, decline. Logs `tengu_auto_mode_opt_in_dialog_shown`, `tengu_auto_mode_opt_in_dialog_accept`, `tengu_auto_mode_opt_in_dialog_accept_default`, `tengu_auto_mode_opt_in_dialog_decline`.

---

### AutoUpdater.tsx

**Purpose:** npm-based auto-updater. Polls GCS every 30 minutes for newer versions and installs them.

**Exports:** `AutoUpdater`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `isUpdating` | `boolean` | yes | Whether an update is in progress |
| `onChangeIsUpdating` | `(v: boolean) => void` | yes | Toggle update in-progress state |
| `onAutoUpdaterResult` | `(r: AutoUpdaterResult) => void` | yes | Called when update completes/fails |
| `autoUpdaterResult` | `AutoUpdaterResult \| null` | yes | Current result state |
| `showSuccessMessage` | `boolean` | yes | Whether to display success notification |
| `verbose` | `boolean` | yes | Verbose output mode |

**Key state:** `versions: { global?, latest? }`, `hasLocalInstall: boolean`

**Key behavior:** Checks `maxVersion` kill switch, reads `installationType` to decide whether to run, polls on 30-minute interval.

---

### AutoUpdaterWrapper.tsx

**Purpose:** Routes auto-update logic to the appropriate updater (package manager, native installer, or npm).

**Exports:** `AutoUpdaterWrapper`

**Props:** Same as `AutoUpdater`.

**Key state:** `useNativeInstaller: boolean | null`, `isPackageManager: boolean | null`

**Key behavior:** Renders `PackageManagerAutoUpdater`, `NativeAutoUpdater`, or `AutoUpdater` depending on how Claude Code was installed.

---

### AwsAuthStatusBox.tsx

**Purpose:** Displays a bordered box with "Cloud Authentication" status when AWS authentication is in progress or has errored.

**Exports:** `AwsAuthStatusBox`

**Props:** None (reads from `AwsAuthStatusManager` singleton).

**Key state:** `status: AwsAuthStatus` (subscribed via `useEffect`)

**Key behavior:** Returns `null` unless `isAuthenticating` or there is an error. Renders a styled bordered box.

---

### BaseTextInput.tsx

**Purpose:** Low-level text input component shared by `TextInput` and `VimTextInput`. Handles cursor, paste, and highlight rendering.

**Exports:** `BaseTextInput`

**Props:** `BaseTextInputProps & { inputState: BaseInputState; children?: ReactNode; terminalFocus: boolean; highlights?: TextHighlight[]; invert?: (text: string) => string; hidePlaceholderText?: boolean }`

**Key behavior:** Uses `useDeclaredCursor` (for terminal cursor placement), `usePasteHandler` (clipboard paste), `renderPlaceholder` utility.

---

### BashModeProgress.tsx

**Purpose:** Renders the in-progress bash command UI: the input message and streaming progress output.

**Exports:** `BashModeProgress`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `input` | `string` | yes | The bash command being run |
| `progress` | `ShellProgress \| null` | yes | Current shell progress |
| `verbose` | `boolean` | yes | Verbose mode |

**Key behavior:** Renders `UserBashInputMessage` + `ShellProgressMessage` (or falls back to `BashTool.renderToolUseProgressMessage`).

---

### BridgeDialog.tsx

**Purpose:** Dialog for setting up the remote bridge (REPL bridge). Shows a QR code and branch name for mobile/remote access.

**Exports:** `BridgeDialog`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `onDone` | `() => void` | yes | Called when dialog closes |

**Key state:** `showQR: boolean`, `qrText: string`, `branchName: string`

**Key behavior:** Uses `qrcode` library to render QR code. Reads bridge state from AppState.

---

### BypassPermissionsModeDialog.tsx

**Purpose:** Confirmation dialog shown when the `--dangerously-skip-permissions` flag is used. Requires explicit acceptance.

**Exports:** `BypassPermissionsModeDialog`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `onAccept` | `() => void` | yes | Called when user accepts danger mode |

**Key behavior:** Escape handler calls `gracefulShutdownSync(0)`. Decline calls `gracefulShutdownSync(1)`. Logs `tengu_bypass_permissions_mode_dialog_shown` and `tengu_bypass_permissions_mode_dialog_accept`.

---

### ChannelDowngradeDialog.tsx

**Purpose:** Dialog shown when the installed version would be a downgrade relative to the user's current version on a different release channel.

**Exports:** `ChannelDowngradeChoice` (type: `'downgrade' | 'stay' | 'cancel'`), `ChannelDowngradeDialog`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `currentVersion` | `string` | yes | Current installed version |
| `onChoice` | `(choice: ChannelDowngradeChoice) => void` | yes | Called with user's choice |

---

### ClickableImageRef.tsx

**Purpose:** Renders an image reference (by `imageId`) as a clickable hyperlink in terminals that support OSC 8. Falls back to styled text in non-supporting terminals.

**Exports:** `ClickableImageRef`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `imageId` | `number` | yes | Internal image ID |
| `backgroundColor` | `keyof Theme` | no | Background color key |
| `isSelected` | `boolean` | no | Whether this ref is currently selected |

**Key behavior:** Uses `pathToFileURL` + `supportsHyperlinks()` to produce OSC 8 links.

---

### ClaudeInChromeOnboarding.tsx

**Purpose:** Onboarding flow for the Claude in Chrome browser extension. Shows installation status and saves acceptance to global config.

**Exports:** `ClaudeInChromeOnboarding`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `onDone` | `() => void` | yes | Called when onboarding completes |

**Key state:** `isExtensionInstalled: boolean`

**Key behavior:** Logs `tengu_claude_in_chrome_onboarding_shown`, saves config flag.

---

### ClaudeMdExternalIncludesDialog.tsx

**Purpose:** Dialog warning the user about external files included in CLAUDE.md (`@path` directives). User must approve to allow them.

**Exports:** `ClaudeMdExternalIncludesDialog`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `onDone` | `() => void` | yes | Called when dialog closes |
| `isStandaloneDialog` | `boolean` | no | Whether shown standalone vs embedded |
| `externalIncludes` | `ExternalClaudeMdInclude[]` | no | List of detected external includes |

**Key behavior:** Saves `hasClaudeMdExternalIncludesApproved` and `hasClaudeMdExternalIncludesWarningShown` to project config.

---

### CompactSummary.tsx

**Purpose:** Renders a visual separator/summary card marking a compacted conversation boundary.

**Exports:** `CompactSummary`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `message` | `NormalizedUserMessage` | yes | The compact summary attachment message |
| `screen` | `Screen` | yes | Current screen context |

**Key behavior:** Shows metadata: `messagesSummarized`, direction, `userContext`.

---

### ConfigurableShortcutHint.tsx

**Purpose:** Renders a keyboard shortcut hint using the user-configured binding for an action, falling back to a literal string if no binding is set.

**Exports:** `ConfigurableShortcutHint`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `action` | `KeybindingAction` | yes | Keybinding action name |
| `context` | `KeybindingContextName` | yes | Keybinding context name |
| `fallback` | `string` | yes | Literal fallback if no binding |
| `description` | `string` | yes | Human description of action |
| `parens` | `boolean` | no | Whether to wrap in parentheses |
| `bold` | `boolean` | no | Whether to bold the hint |

**Key behavior:** Calls `useShortcutDisplay` to resolve binding, delegates to `KeyboardShortcutHint`.

---

### ConsoleOAuthFlow.tsx

**Purpose:** Full OAuth login flow for claude.ai/console authentication. Manages a state machine for the multi-step auth process.

**Exports:** `ConsoleOAuthFlow`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `onDone` | `() => void` | yes | Called when auth completes or is cancelled |
| `startingMessage` | `string` | no | Introductory message text |
| `mode` | `'login' \| 'setup-token'` | no | Whether doing login or token setup |
| `forceLoginMethod` | `'claudeai' \| 'console'` | no | Override default login provider |

**Key state:** `OAuthStatus` — union of states: `idle`, `platform_setup`, `ready_to_start`, `waiting_for_login`, `creating_api_key`, `about_to_retry`, `success`, `error`.

---

### ContextSuggestions.tsx

**Purpose:** Renders a list of context-saving suggestions (e.g., "remove X tokens by adding a .gitignore pattern").

**Exports:** `ContextSuggestions`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `suggestions` | `ContextSuggestion[]` | yes | List of suggestions to display |

**Key behavior:** Each suggestion shows a title, severity icon, and token savings.

---

### ContextVisualization.tsx

**Purpose:** Visualizes context window usage with a collapse-status indicator (when `CONTEXT_COLLAPSE` feature flag is active).

**Exports:** `ContextVisualization`

**Key behavior:** Contains internal `CollapseStatus` sub-component gated by `feature('CONTEXT_COLLAPSE')`.

---

### CoordinatorAgentStatus.tsx

**Purpose:** Shows the coordinator/swarm agent progress panel in the sidebar. Reads agent task state from AppState.

**Exports:** `getVisibleAgentTasks(tasks): Task[]`, `CoordinatorTaskPanel`

**Props:** None (reads from AppState).

---

### CostThresholdDialog.tsx

**Purpose:** Dialog warning the user that they have spent $5 on API calls.

**Exports:** `CostThresholdDialog`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `onDone` | `() => void` | yes | Called when user acknowledges |

---

### CtrlOToExpand.tsx

**Purpose:** Renders a dim hint "ctrl+o to expand" for collapsible content. Provides a React context to prevent nested hints.

**Exports:** `SubAgentProvider`, `CtrlOToExpand`, `ctrlOToExpand(): string`

**Key behavior:** `SubAgentContext` is `React.createContext(false)` — prevents double-rendering the hint inside nested sub-agent output. `ctrlOToExpand()` returns a `chalk.dim` string for use in non-React contexts.

---

### DesktopHandoff.tsx

**Purpose:** Manages the "open in Claude Desktop" handoff flow. Checks if Desktop is installed, downloads it if not, then opens it.

**Exports:** `getDownloadUrl(): string`, `DesktopHandoff`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `onDone` | `(result?: string, options?: { display?: CommandResultDisplay }) => void` | yes | Called when handoff completes |

**Key state:** `DesktopHandoffState = 'checking' | 'prompt-download' | 'flushing' | 'opening' | 'success' | 'error'`

---

### DevBar.tsx

**Purpose:** Internal developer bar showing slow operations (for dev/ant builds only).

**Exports:** `DevBar`

**Props:** None.

**Key state:** `slowOps` — polled every 500ms; displays last 3 slow operations.

**Key behavior:** Only renders for dev or ant builds.

---

### DiagnosticsDisplay.tsx

**Purpose:** Renders diagnostic issues found in files (TypeScript errors, lint warnings, etc.).

**Exports:** `DiagnosticsDisplay`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `attachment` | `DiagnosticsAttachment` | yes | Attachment containing file diagnostic data |
| `verbose` | `boolean` | yes | Show per-file detail vs summary |

**Key behavior:** In non-verbose mode shows "Found **N** new diagnostic issue(s) in N file(s)" + `CtrlOToExpand`. In verbose mode shows per-file breakdown.

---

### EffortCallout.tsx

**Purpose:** Renders a callout when a non-default effort level is active (e.g., "max thinking" mode).

**Exports:** `EffortCallout`

**Key behavior:** Feature-gated. Shows the effort symbol and description.

---

### EffortIndicator.ts

**Purpose:** Utility module (non-component) providing effort-related display helpers.

**Exports:**

| Export | Signature | Description |
|---|---|---|
| `getEffortNotificationText` | `(effortValue: any, model: string): string \| undefined` | Returns notification text for effort level changes |
| `effortLevelToSymbol` | `(level: EffortLevel): string` | Maps effort level to display symbol |

---

### ExitFlow.tsx

**Purpose:** Orchestrates the exit flow — showing worktree cleanup dialog if in a worktree.

**Exports:** `ExitFlow`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `onDone` | `(message?: string) => void` | yes | Called when exit flow completes |
| `onCancel` | `() => void` | no | Called when user cancels exit |
| `showWorktree` | `boolean` | yes | Whether to show worktree cleanup UI |

**Key behavior:** Shows `WorktreeExitDialog` when `showWorktree` is true; otherwise null.

---

### ExportDialog.tsx

**Purpose:** Dialog for exporting conversation content to clipboard or a file.

**Exports:** `ExportDialog`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `content` | `string` | yes | Content to export |
| `defaultFilename` | `string` | yes | Suggested filename |
| `onDone` | `(result: { success: boolean; message: string }) => void` | yes | Called with result |

**Key state:** `ExportOption = 'clipboard' | 'file'`

**Key behavior:** Shows filename `TextInput` when "file" is selected.

---

### FallbackToolUseErrorMessage.tsx

**Purpose:** Renders an error message for tool use results when no tool-specific renderer is available.

**Exports:** `FallbackToolUseErrorMessage`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `result` | `ToolResultBlockParam['content']` | yes | The error result content |
| `verbose` | `boolean` | yes | Verbose mode |

**Key behavior:** `MAX_RENDERED_LINES = 10`. Strips underline ANSI, removes sandbox violation/error XML tags. Shows "+N lines (ctrl+o to see all)" hint when truncated.

---

### FallbackToolUseRejectedMessage.tsx

**Purpose:** Renders the "Interrupted · What should Claude do instead?" message for rejected tool uses when no tool-specific renderer exists.

**Exports:** `FallbackToolUseRejectedMessage`

**Props:** None.

**Key behavior:** Wraps `InterruptedByUser` in `MessageResponse` with height=1.

---

### FastIcon.tsx

**Purpose:** Renders the lightning bolt icon (⚡) for fast mode, with appropriate color and dim state for cooldown.

**Exports:** `FastIcon`, `getFastIconString(applyColor?: boolean, cooldown?: boolean): string`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `cooldown` | `boolean` | no | Whether fast mode is in cooldown (dims icon) |

---

### Feedback.tsx

**Purpose:** Full feedback submission form. Collects a description, optional transcript, optionally queries Haiku for AI-assisted categorization, then opens a GitHub issue.

**Exports:** `redactSensitiveInfo(text: string): string`, `Feedback`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `abortSignal` | `AbortSignal` | yes | For cancellable async operations |
| `messages` | `Message[]` | yes | Conversation messages |
| `initialDescription` | `string` | no | Pre-filled feedback text |
| `onDone` | `(result: string, options?: { display?: CommandResultDisplay }) => void` | yes | Called when done |
| `backgroundTasks` | `{ [taskId: string]: { type: string; identity?: { agentId: string }; messages?: Message[] } }` | no | Background task messages |

**Key state:** `Step = 'userInput' | 'consent' | 'submitting' | 'done'`

**Constants:** `GITHUB_URL_LIMIT = 7250`, `GITHUB_ISSUES_REPO_URL` (build-target specific)

**Key behavior:** `redactSensitiveInfo` strips API keys (`sk-ant-...`) before submission using regex.

---

### FileEditToolDiff.tsx

**Purpose:** Renders a diff for a file edit (via `FileEditTool`). Loads diff data asynchronously using React `Suspense`.

**Exports:** `FileEditToolDiff`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `file_path` | `string` | yes | Absolute path to the file being edited |
| `edits` | `FileEdit[]` | yes | List of edits to apply |

**Key behavior:** Uses `useState` to initialize a promise-based data loader, wraps `DiffBody` in `<Suspense>`.

---

### FileEditToolUpdatedMessage.tsx

**Purpose:** Displays a summary of a file edit (lines added/removed) with a structured diff.

**Exports:** `FileEditToolUpdatedMessage`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `filePath` | `string` | yes | Path to edited file |
| `structuredPatch` | `StructuredPatchHunk[]` | yes | Diff hunks |
| `firstLine` | `string \| null` | yes | First line of file (for shebang detection) |
| `fileContent` | `string` | no | Full file content for syntax context |
| `style` | `'condensed'` | no | Compact display mode |
| `verbose` | `boolean` | yes | Verbose mode |
| `previewHint` | `string` | no | Optional hint text below diff |

---

### FileEditToolUseRejectedMessage.tsx

**Purpose:** Shows a "User rejected write/update to file" message with a preview of the rejected diff or content.

**Exports:** `FileEditToolUseRejectedMessage`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `file_path` | `string` | yes | Target file path |
| `operation` | `'write' \| 'update'` | yes | Operation type |
| `patch` | `StructuredPatchHunk[]` | no | Diff for update operations |
| `firstLine` | `string \| null` | yes | First line of file |
| `fileContent` | `string` | no | Full file content |
| `content` | `string` | no | New file content for write operations |
| `style` | `'condensed'` | no | Compact display mode |
| `verbose` | `boolean` | yes | Verbose mode |

**Constants:** `MAX_LINES_TO_RENDER = 10`

---

### FilePathLink.tsx

**Purpose:** Renders an absolute file path as an OSC 8 hyperlink for terminal emulators (e.g., iTerm2) that support them.

**Exports:** `FilePathLink`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `filePath` | `string` | yes | Absolute file path |
| `children` | `React.ReactNode` | no | Display text (defaults to filePath) |

**Key behavior:** Uses `pathToFileURL` to convert the path to a `file://` URL, wraps in Ink `Link`.

---

### FullscreenLayout.tsx

**Purpose:** The main layout container for fullscreen mode. Manages a scrollable region, fixed bottom slot, overlays, modal pane, and floating content.

**Exports:** `ScrollChromeContext` (React context), `FullscreenLayout`

**`ScrollChromeContext`:** `{ setStickyPrompt: (p: StickyPrompt | null) => void }` — allows VirtualMessageList's `StickyTracker` to update sticky prompt state without prop-drilling.

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `scrollable` | `ReactNode` | yes | Content that scrolls (messages, tool output) |
| `bottom` | `ReactNode` | yes | Content pinned to bottom (spinner, prompt, permissions) |
| `overlay` | `ReactNode` | no | Content rendered inside ScrollBox after messages (permission requests) |
| `bottomFloat` | `ReactNode` | no | Absolute-positioned floating content over scrollback (companion speech bubble) |
| `modal` | `ReactNode` | no | Slash-command dialog in absolute bottom-anchored pane. Provides ModalContext. Fullscreen only. |
| `modalScrollRef` | `React.RefObject<ScrollBoxHandle \| null>` | no | Ref for Tabs to attach scroll-owning ScrollBox |
| `scrollRef` | `RefObject<ScrollBoxHandle \| null>` | no | Ref for keyboard scrolling |
| `dividerYRef` | `RefObject<number \| null>` | no | Y-position of unseen-divider (for scroll pill) |
| `hidePill` | `boolean` | no | Force-hide the scroll pill |

**Constants:** `MODAL_TRANSCRIPT_PEEK = 2` rows.

---

### GlobalSearchDialog.tsx

**Purpose:** Full-text ripgrep search dialog (ctrl+shift+f). Debounced search with a file preview pane.

**Exports:** `GlobalSearchDialog`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `onDone` | `() => void` | yes | Close callback |
| `onInsert` | `(text: string) => void` | yes | Called to insert a result into the prompt |

**Key state:** `matches: Match[]`, `truncated: boolean`, `isSearching: boolean`

**Constants:** `VISIBLE_RESULTS = 12`, `DEBOUNCE_MS = 100`, `PREVIEW_CONTEXT_LINES = 4`, `MAX_MATCHES_PER_FILE = 10`, `MAX_TOTAL_MATCHES = 500`

**Key behavior:** Uses `useRegisterOverlay("global-search")`. Previews on right column when `columns >= 140`. Calls `ripGrepStream` for search.

---

### HighlightedCode.tsx

**Purpose:** Renders syntax-highlighted source code using the native Rust `ColorFile` module. Falls back to `HighlightedCodeFallback` when unavailable.

**Exports:** `HighlightedCode` (memo-wrapped)

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `code` | `string` | yes | Source code string |
| `filePath` | `string` | yes | File path for language detection |
| `width` | `number` | no | Render width (defaults to 80) |
| `dim` | `boolean` | no | Whether to dim the output |

**Key state:** `measuredWidth: number` (from `measureElement`)

**Constants:** `DEFAULT_WIDTH = 80`

**Key behavior:** Respects `settings.syntaxHighlightingDisabled`. Uses `expectColorFile()` from the Rust `colorDiff` module.

---

### HistorySearchDialog.tsx

**Purpose:** Fuzzy-searchable history browser (ctrl+r). Loads all timestamped history entries async, allows fuzzy matching and preview.

**Exports:** `HistorySearchDialog`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `initialQuery` | `string` | no | Pre-filled search query |
| `onSelect` | `(entry: HistoryEntry) => void` | yes | Called when user selects an entry |
| `onCancel` | `() => void` | yes | Called when user cancels |

**Key state:** `items: Item[] | null`, `query: string`

**Constants:** `PREVIEW_ROWS = 6`, `AGE_WIDTH = 8`

**Key behavior:** Uses `useRegisterOverlay('history-search')`. Loads from `getTimestampedHistory()` async generator. Uses `FuzzyPicker` for display.

---

### IdeAutoConnectDialog.tsx

**Purpose:** First-run dialog asking whether to auto-connect to the IDE on startup.

**Exports:** `IdeAutoConnectDialog`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `onComplete` | `() => void` | yes | Called when user makes a selection |

**Key behavior:** Saves `globalConfig.autoConnectIde` and `globalConfig.hasIdeAutoConnectDialogBeenShown = true`.

---

### IdeOnboardingDialog.tsx

**Purpose:** Onboarding dialog for IDE integration (VS Code, JetBrains etc). Shows installation status and instructions.

**Exports:** `IdeOnboardingDialog`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `onDone` | `() => void` | yes | Called when dialog closes |
| `installationStatus` | `IDEExtensionInstallationStatus \| null` | yes | Current extension install status |

**Key behavior:** Marks dialog as shown immediately on mount via `markDialogAsShown()`. Responds to `confirm:yes` and `confirm:no` keybindings.

---

### IdeStatusIndicator.tsx

**Purpose:** Shows current IDE selection state in the status line — active file or selected lines count.

**Exports:** `IdeStatusIndicator`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `ideSelection` | `IDESelection \| undefined` | yes | Current IDE selection state |
| `mcpClients` | `MCPServerConnection[]` | no | MCP clients for connection status |

**Key behavior:** Uses `useIdeConnectionStatus`. Returns null unless connected with a selection. Shows `⧉ N lines selected` or `⧉ In filename.ts`.

---

### IdleReturnDialog.tsx

**Purpose:** Dialog shown when user returns after an idle period, offering options to continue, clear, dismiss, or never show again.

**Exports:** `IdleReturnDialog`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `idleMinutes` | `number` | yes | Minutes the user was away |
| `totalInputTokens` | `number` | yes | Current context size in tokens |
| `onDone` | `(action: IdleReturnAction) => void` | yes | Called with user's choice |

**Types:** `IdleReturnAction = 'continue' | 'clear' | 'dismiss' | 'never'`

---

### InterruptedByUser.tsx

**Purpose:** Renders the "Interrupted · What should Claude do instead?" text. In ant builds shows a different message.

**Exports:** `InterruptedByUser`

**Props:** None.

---

### InvalidConfigDialog.tsx

**Purpose:** Dialog shown when the Claude config file contains invalid JSON. User can exit or reset the config.

**Exports:** `InvalidConfigDialog` (named, not default), `InvalidConfigHandlerProps`, `InvalidConfigDialogProps`

**`InvalidConfigDialogProps`:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `filePath` | `string` | yes | Path to invalid config file |
| `errorDescription` | `string` | yes | Error description to display |
| `onExit` | `() => void` | yes | Exit without fixing |
| `onReset` | `() => void` | yes | Reset config to defaults |

**Key behavior:** Also exports standalone renderer via `render()` for use outside the React tree.

---

### InvalidSettingsDialog.tsx

**Purpose:** Dialog shown when settings files have validation errors. User can continue (skipping invalid files) or exit.

**Exports:** `InvalidSettingsDialog`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `settingsErrors` | `ValidationError[]` | yes | List of validation errors |
| `onContinue` | `() => void` | yes | Continue despite errors |
| `onExit` | `() => void` | yes | Exit to fix errors |

---

### KeybindingWarnings.tsx

**Purpose:** Displays keybinding validation warnings/errors. Only shown when keybinding customization is enabled (ant + feature gate).

**Exports:** `KeybindingWarnings`

**Props:** None.

**Key behavior:** Calls `isKeybindingCustomizationEnabled()`. Groups warnings by severity (error/warning). Shows file path via `getKeybindingsPath()`.

---

### LanguagePicker.tsx

**Purpose:** Text input for selecting the preferred response language.

**Exports:** `LanguagePicker`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `initialLanguage` | `string \| undefined` | yes | Current language setting |
| `onComplete` | `(language: string \| undefined) => void` | yes | Called with selected language |
| `onCancel` | `() => void` | yes | Called when cancelled |

**Key state:** `language: string | undefined`, `cursorOffset: number`

---

### LogSelector.tsx

**Purpose:** Full-featured session log browser with fuzzy search, tag filtering, agentic search, and session preview.

**Exports:** `LogSelectorProps`, `LogSelector`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `logs` | `LogOption[]` | yes | All available log entries |
| `maxHeight` | `number` | no | Max display height |
| `forceWidth` | `number` | no | Override terminal width |
| `onCancel` | `() => void` | no | Cancel callback |
| `onSelect` | `(log: LogOption) => void` | yes | Selection callback |
| `onLogsChanged` | `() => void` | no | Called when log list changes |
| `onLoadMore` | `(count: number) => void` | no | Load more entries |
| `initialSearchQuery` | `string` | no | Pre-filled query |
| `showAllProjects` | `boolean` | no | Show logs from all projects |
| `onToggleAllProjects` | `() => void` | no | Toggle all-projects mode |
| `onAgenticSearch` | `(query: string, logs: LogOption[], signal?: AbortSignal) => Promise<LogOption[]>` | no | AI-powered search callback |

**Internal types:** `AgenticSearchState`, `LogTreeNode`

---

### MarkdownTable.tsx

**Purpose:** Renders a Markdown table token with ANSI-aware column width calculation and wrapping. Switches to vertical (key-value) format for wide content.

**Exports:** `MarkdownTable`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `token` | `Tokens.Table` | yes | Parsed marked.js table token |
| `highlight` | `CliHighlight \| null` | yes | Syntax highlight context |
| `forceWidth` | `number` | no | Override terminal width (for testing) |

**Constants:** `SAFETY_MARGIN = 4`, `MIN_COLUMN_WIDTH = 3`, `MAX_ROW_LINES = 4`

---

### Markdown.tsx

**Purpose:** Renders Markdown text with `marked` (GFM mode). Includes an LRU token cache and a fast path for plain text.

**Exports:** `Markdown`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `children` | `string` | yes | Markdown source text |
| `dimColor` | `boolean` | no | Whether to dim the output |

**Key behavior:** Module-level LRU token cache with max 500 entries. Fast path skips `marked.lexer` for plain text (no Markdown syntax). Uses marked GFM dialect.

---

### MemoryUsageIndicator.tsx

**Purpose:** Shows high/critical heap memory warning with `/heapdump` link. Ant-internal build only — returns `null` in external builds.

**Exports:** `MemoryUsageIndicator`

**Props:** None.

**Key behavior:** Returns null in external builds (build-time constant). Uses `useMemoryUsage()` hook (10s polling). Shows warning/error color based on status.

---

### Message.tsx

**Purpose:** Central message dispatcher. Routes each message/content block to its appropriate rendering component.

**Exports:** `hasThinkingContent(message): boolean`, `Message`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `message` | `NormalizedMessage` | yes | Normalized message to render |
| `lookups` | `ReturnType<typeof buildMessageLookups>` | yes | Precomputed message lookups |
| `containerWidth` | `number` | no | Available width |
| `addMargin` | `boolean` | yes | Whether to add top margin |
| `tools` | `Tools` | yes | Available tools |
| `commands` | `Command[]` | yes | Available commands |
| `verbose` | `boolean` | yes | Verbose mode |
| `inProgressToolUseIDs` | `Set<string>` | yes | IDs of in-flight tool uses |
| `progressMessagesForMessage` | `Message[]` | yes | Progress messages for this message |
| `shouldAnimate` | `boolean` | yes | Whether to animate spinners |
| `shouldShowDot` | `boolean` | yes | Whether to show dot prefix |
| `style` | `object` | no | Style overrides |
| `width` | `number` | no | Explicit width |
| `isTranscriptMode` | `boolean` | yes | Transcript/history mode |
| `isStatic` | `boolean` | yes | Prevent re-renders |
| `onOpenRateLimitOptions` | `() => void` | no | Opens rate limit options |
| `isActiveCollapsedGroup` | `boolean` | no | Whether this is the active collapsed group |
| `isUserContinuation` | `boolean` | no | Whether prev message is also user |
| `lastThinkingBlockId` | `string \| null` | yes | ID of last thinking block |
| `latestBashOutputUUID` | `string \| null` | yes | UUID of latest bash output |

---

### MessageModel.tsx

**Purpose:** Displays the model identifier for an assistant message in transcript mode.

**Exports:** `MessageModel`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `message` | `NormalizedMessage` | yes | Message to check |
| `isTranscriptMode` | `boolean` | yes | Only shows in transcript mode |

**Key behavior:** Only renders for assistant messages with a `model` field and text content blocks.

---

### MessageResponse.tsx

**Purpose:** Wraps assistant response content with the `⎿` prefix character. Uses `MessageResponseContext` to prevent double-prefix nesting.

**Exports:** `MessageResponse`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `children` | `ReactNode` | yes | Response content |
| `height` | `number` | no | Explicit height (skips Ratchet wrapper) |

**Key behavior:** Renders `⎿` via `NoSelect`. Uses `MessageResponseContext` to avoid nested prefixes. Wraps in `Ratchet` unless `height` is specified.

---

### MessageRow.tsx

**Purpose:** Renders a single message row with optional OffscreenFreeze, model indicator, and timestamp.

**Exports:** `hasContentAfterIndex(messages, index, tools, streamingToolUseIDs): boolean`, `Props`, `MessageRow`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `message` | `RenderableMessage` | yes | Message to render |
| `isUserContinuation` | `boolean` | yes | Whether prev message is user |
| `hasContentAfter` | `boolean` | yes | Whether non-skippable content follows |
| `tools` | `Tools` | yes | Available tools |
| `commands` | `Command[]` | yes | Available commands |
| `verbose` | `boolean` | yes | Verbose mode |
| `inProgressToolUseIDs` | `Set<string>` | yes | In-flight tool IDs |
| `streamingToolUseIDs` | `Set<string>` | yes | Streaming tool IDs |
| `screen` | `Screen` | yes | Current screen context |
| `canAnimate` | `boolean` | yes | Allow animation |
| `onOpenRateLimitOptions` | `() => void` | no | Rate limit options callback |
| `lastThinkingBlockId` | `string \| null` | yes | ID of last thinking block |
| `latestBashOutputUUID` | `string \| null` | yes | UUID of latest bash output |
| `columns` | `number` | yes | Terminal column count |
| `isLoading` | `boolean` | yes | Whether a query is in flight |
| `lookups` | `ReturnType<typeof buildMessageLookups>` | yes | Precomputed lookups |

---

### MessageSelector.tsx

**Purpose:** Allows the user to select a historical message to rewind/restore to.

**Exports:** `MessageSelector`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `messages` | `Message[]` | yes | All conversation messages |
| `onPreRestore` | `() => void` | yes | Pre-restore hook |
| `onRestoreMessage` | `(message: UserMessage) => Promise<void>` | yes | Restore conversation to message |
| `onRestoreCode` | `(message: UserMessage) => Promise<void>` | yes | Restore code to message |
| `onSummarize` | `(message: UserMessage, feedback?: string, direction?: PartialCompactDirection) => Promise<void>` | yes | Summarize up to message |
| `onClose` | `() => void` | yes | Close callback |
| `preselectedMessage` | `UserMessage` | no | Skip pick-list, go direct to confirm |

**Types:** `RestoreOption = 'both' | 'conversation' | 'code' | 'summarize' | 'summarize_up_to' | 'nevermind'`

**Constants:** `MAX_VISIBLE_MESSAGES = 7`

---

### MessageTimestamp.tsx

**Purpose:** Shows the formatted timestamp for an assistant message in transcript mode.

**Exports:** `MessageTimestamp`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `message` | `NormalizedMessage` | yes | Message with timestamp |
| `isTranscriptMode` | `boolean` | yes | Only shows in transcript mode |

**Key behavior:** Only renders for assistant messages with text content. Formats as `HH:MM AM/PM`.

---

### Messages.tsx

**Purpose:** Top-level conversation view component. Normalizes messages, collapses read/search groups, builds lookups, and drives the virtual scroll list or static list.

**Exports:** `shouldRenderStatically(screen: Screen): boolean`, `Messages`

**Key behavior:**
- Contains `LogoHeader = React.memo(...)` with blit-optimization note (must render before messages for correct scroll behavior).
- Filters out null-rendering attachments before the 200-message render cap.
- Builds `buildMessageLookups` for efficient tool use result matching.
- Uses `VirtualMessageList` with `JumpHandle` for transcript mode, static list for REPL mode.

---

### ModelPicker.tsx

**Purpose:** Model selection picker with effort level toggle. Supports fast mode awareness and session-scoped vs global settings.

**Exports:** `Props`, `ModelPicker`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `initial` | `string \| null` | yes | Initial model (null = no preference) |
| `sessionModel` | `ModelSetting` | no | Current session model |
| `onSelect` | `(model: string \| null, effort: EffortLevel \| undefined) => void` | yes | Selection callback |
| `onCancel` | `() => void` | no | Cancel callback |
| `isStandaloneCommand` | `boolean` | no | In standalone command context |
| `showFastModeNotice` | `boolean` | no | Show fast mode info |
| `headerText` | `string` | no | Override dim header line |
| `skipSettingsWrite` | `boolean` | no | Skip writing effort to userSettings (for project-scoped installs) |

**Constants:** `NO_PREFERENCE = '__NO_PREFERENCE__'`

---

### NativeAutoUpdater.tsx

**Purpose:** Auto-updater for native installer builds. Calls `installLatest()` from `nativeInstaller` when a newer version is available.

**Exports:** `NativeAutoUpdater`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `isUpdating` | `boolean` | yes | In-progress flag |
| `onChangeIsUpdating` | `(v: boolean) => void` | yes | Toggle in-progress |
| `onAutoUpdaterResult` | `(r: AutoUpdaterResult) => void` | yes | Result callback |
| `autoUpdaterResult` | `AutoUpdaterResult \| null` | yes | Current result |
| `showSuccessMessage` | `boolean` | yes | Show success notification |
| `verbose` | `boolean` | yes | Verbose mode |

**Key state:** `versions: { current?: string | null; latest?: string | null }` (polled by interval)

---

### NotebookEditToolUseRejectedMessage.tsx

**Purpose:** Shows "User rejected replace/insert/delete cell in notebook_path" with code preview.

**Exports:** `NotebookEditToolUseRejectedMessage`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `notebook_path` | `string` | yes | Path to the notebook |
| `cell_id` | `string \| undefined` | yes | Target cell ID |
| `new_source` | `string` | yes | New cell source |
| `cell_type` | `'code' \| 'markdown'` | no | Cell type |
| `edit_mode` | `'replace' \| 'insert' \| 'delete'` | no | Edit mode (default: 'replace') |
| `verbose` | `boolean` | yes | Verbose mode |

---

### OffscreenFreeze.tsx

**Purpose:** Performance optimization that freezes children when they scroll above the terminal viewport into scrollback.

**Exports:** `OffscreenFreeze`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `children` | `React.ReactNode` | yes | Content to potentially freeze |

**Key behavior:** Uses `'use no memo'` directive to opt out of React Compiler. Uses `useTerminalViewport` to detect visibility. Caches the last visible render in `useRef`. When `inVirtualList` is true, freeze is bypassed (virtual list clips inside viewport).

---

### Onboarding.tsx

**Purpose:** Multi-step first-run onboarding flow: preflight checks, theme selection, OAuth, API key approval, security review, terminal setup.

**Exports:** `Onboarding`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `onDone` | `() => void` | yes | Called when all steps complete |

**Key state:** `currentStepIndex: number`, `skipOAuth: boolean`, `oauthEnabled: boolean`, `theme`

**Steps:** `StepId = 'preflight' | 'theme' | 'oauth' | 'api-key' | 'security' | 'terminal-setup'`

---

### OutputStylePicker.tsx

**Purpose:** Picker for selecting the active output style (default, concise, detailed, custom styles from `.claude/output-styles/`).

**Exports:** `OutputStylePickerProps`, `OutputStylePicker`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `initialStyle` | `OutputStyle` | yes | Current style |
| `onComplete` | `(style: OutputStyle) => void` | yes | Selection callback |
| `onCancel` | `() => void` | yes | Cancel callback |
| `isStandaloneCommand` | `boolean` | no | In standalone command context |

**Key state:** `styleOptions: OptionWithDescription[]`, `isLoading: boolean`

---

### PackageManagerAutoUpdater.tsx

**Purpose:** Notifies users about available updates when Claude Code was installed via a package manager (brew, pip, etc.).

**Exports:** `PackageManagerAutoUpdater`

**Props:** Same as `NativeAutoUpdater`.

**Key state:** `updateAvailable: boolean`, `packageManager: PackageManager | "unknown"`

**Key behavior:** Only shows notification text — does not auto-install. Uses `MACRO.VERSION` (build-time constant).

---

### PrBadge.tsx

**Purpose:** Renders a PR number badge with review state coloring.

**Exports:** `PrBadge`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `number` | `number` | yes | PR number |
| `url` | `string` | yes | PR URL |
| `reviewState` | `PrReviewState` | no | Review status |
| `bold` | `boolean` | no | Bold style |

---

### PressEnterToContinue.tsx

**Purpose:** Simple "Press **Enter** to continue…" prompt with permission color styling.

**Exports:** `PressEnterToContinue`

**Props:** None.

---

### QuickOpenDialog.tsx

**Purpose:** Quick-open fuzzy file finder (ctrl+shift+p). Shows file results with a syntax-highlighted preview.

**Exports:** `QuickOpenDialog`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `onDone` | `() => void` | yes | Close callback |
| `onInsert` | `(text: string) => void` | yes | Insert path into prompt |

**Key state:** `results`, `query`, `focusedPath`, `preview`

**Constants:** `VISIBLE_RESULTS = 8`, `PREVIEW_LINES = 20`

---

### RemoteCallout.tsx

**Purpose:** One-time callout dialog prompting users to enable Remote Control (bridge). Saves `remoteDialogSeen = true` on mount.

**Exports:** `RemoteCallout`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `onDone` | `(selection: RemoteCalloutSelection) => void` | yes | Called with 'enable' or 'dismiss' |

**Types:** `RemoteCalloutSelection = 'enable' | 'dismiss'`

---

### RemoteEnvironmentDialog.tsx

**Purpose:** Picker for selecting a remote Teleport environment (claude.ai/code environments).

**Exports:** `RemoteEnvironmentDialog`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `onDone` | `(message?: string) => void` | yes | Close callback |

**Key state:** `loadingState: 'loading' | 'updating' | null`, `environments: EnvironmentResource[]`, `selectedEnvironment`, `selectedEnvironmentSource`, `error`

---

### ResumeTask.tsx

**Purpose:** Lists remote Claude Code sessions (from Sessions API) for resuming. Filters by current git repository.

**Exports:** `ResumeTask`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `onSelect` | `(session: CodeSession) => void` | yes | Session selection callback |
| `onCancel` | `() => void` | yes | Cancel callback |
| `isEmbedded` | `boolean` | no | Whether shown inside another component |

**Key state:** `sessions: CodeSession[]`, `currentRepo: string | null`, `loading: boolean`, `loadErrorType: LoadErrorType | null`, `retrying: boolean`, `focusedIndex: number`

---

### SandboxViolationExpandedView.tsx

**Purpose:** Shows recent sandboxing violations (last 10). Subscribed to `SandboxManager.getSandboxViolationStore()`.

**Exports:** `SandboxViolationExpandedView`

**Props:** None.

**Key state:** `violations: SandboxViolationEvent[]`, `totalCount: number`

**Key behavior:** Returns null when sandboxing is disabled or on Linux.

---

### ScrollKeybindingHandler.tsx

**Purpose:** Keyboard handler for scrolling — j/k/arrows, page up/down, g/G, ctrl+u/d/b/f. Includes scroll acceleration for smooth wheel events.

**Exports:** `ScrollKeybindingHandler`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `scrollRef` | `RefObject<ScrollBoxHandle \| null>` | yes | The scroll box to control |
| `isActive` | `boolean` | yes | Whether key bindings are active |
| `onScroll` | `(sticky: boolean, handle: ScrollBoxHandle) => void` | no | Post-scroll callback |
| `isModal` | `boolean` | no | Enable modal pager keys (g/G, ctrl+u/d/b/f) |

**Constants:** `WHEEL_ACCEL_WINDOW_MS = 40`, `WHEEL_ACCEL_STEP = 0.3`, `WHEEL_ACCEL_MAX = 6`

---

### SearchBox.tsx

**Purpose:** Renders a styled search input box with cursor display and placeholder.

**Exports:** `SearchBox`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `query` | `string` | yes | Current query text |
| `placeholder` | `string` | no | Placeholder text (default: "Search…") |
| `isFocused` | `boolean` | yes | Whether input is focused |
| `isTerminalFocused` | `boolean` | yes | Whether the terminal has focus |
| `prefix` | `string` | no | Icon prefix (default: "⌕") |
| `width` | `number \| string` | no | Width override |
| `cursorOffset` | `number` | no | Cursor position (defaults to query.length) |
| `borderless` | `boolean` | no | Remove border |

---

### SentryErrorBoundary.ts

**Purpose:** React error boundary that silently swallows rendering errors (renders `null`).

**Exports:** `SentryErrorBoundary`

**Props:** `{ children: React.ReactNode }`

**Key behavior:** Class component; catches render errors via `getDerivedStateFromError` and renders null.

---

### SessionBackgroundHint.tsx

**Purpose:** Shows a hint and handles the Ctrl+B double-press pattern for backgrounding the current session.

**Exports:** `SessionBackgroundHint`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `onBackgroundSession` | `() => void` | yes | Called to background the session |
| `isLoading` | `boolean` | yes | Whether a query is in progress |

**Key state:** `showSessionHint: boolean`

**Key behavior:** Only activates when `isLoading` and no foreground bash/agent tasks. Uses `useDoublePress` — first press shows hint, second within 800ms executes.

---

### SessionPreview.tsx

**Purpose:** Renders a read-only preview of a historical session log using the full `Messages` component.

**Exports:** `SessionPreview`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `log` | `LogOption` | yes | Log entry to preview |
| `onExit` | `() => void` | yes | Close callback |
| `onSelect` | `(log: LogOption) => void` | yes | Open this session fully |

**Key state:** `fullLog: LogOption | null` (loaded async from `loadFullLog` for lite logs)

---

### ShowInIDEPrompt.tsx

**Purpose:** Shows an "Opened changes in {IDE}" confirmation pane with Yes/No options when a diff was opened in the IDE.

**Exports:** `ShowInIDEPrompt`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `filePath` | `string` | yes | File being reviewed |
| `input` | `A` (generic) | yes | Tool input arguments |
| `onChange` | `(option: PermissionOption, args: A, feedback?: string) => void` | yes | Selection callback |
| `options` | `PermissionOptionWithLabel[]` | yes | Accept/reject options |
| `ideName` | `string` | yes | IDE display name (e.g., "VS Code") |
| `symlinkTarget` | `string \| null` | no | Symlink target if applicable |
| `rejectFeedback` | `string` | yes | Text for reject confirmation |
| `acceptFeedback` | `string` | yes | Text for accept confirmation |
| `setFocusedOption` | `(value: string) => void` | yes | Focus control callback |
| `onInputModeToggle` | `(value: string) => void` | yes | Toggle input mode |
| `focusedOption` | `string` | yes | Currently focused option |
| `yesInputMode` | `boolean` | yes | Whether in yes-input mode |
| `noInputMode` | `boolean` | yes | Whether in no-input mode |

---

### SkillImprovementSurvey.tsx

**Purpose:** Renders a post-skill-execution survey asking if the skill improvement was helpful.

**Exports:** `SkillImprovementSurvey`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `isOpen` | `boolean` | yes | Whether survey is visible |
| `skillName` | `string` | yes | Name of the skill |
| `updates` | `SkillUpdate[]` | yes | Skill update descriptions |
| `handleSelect` | `(selected: FeedbackSurveyResponse) => void` | yes | Response callback |
| `inputValue` | `string` | yes | Current text input value |
| `setInputValue` | `(value: string) => void` | yes | Text input setter |

---

### Spinner.tsx

**Purpose:** Re-exports the main animated Spinner component and its `SpinnerMode` type.

**Exports:** `SpinnerMode` (re-export from `./Spinner/index.js`), `Spinner`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `mode` | `SpinnerMode` | yes | Spinner animation mode |
| `loadingStartTimeRef` | `RefObject<number>` | yes | When loading started |
| `totalPausedMsRef` | `RefObject<number>` | yes | Accumulated paused milliseconds |
| `pauseStartTimeRef` | `RefObject<number>` | yes | When last pause began |
| `spinnerTip` | `string` | no | Tip text below spinner |
| `responseLengthRef` | `RefObject<number>` | yes | Streaming response length |
| `overrideColor` | `string` | no | Override spinner color |
| `overrideShimmerColor` | `string` | no | Override shimmer color |

---

### Stats.tsx

**Purpose:** Full-screen usage statistics viewer with date range tabs, token breakdown, model heatmap, and ASCII chart.

**Exports:** `Stats`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `onClose` | `(result?: string, options?: { display?: CommandResultDisplay }) => void` | yes | Close callback |

**Types:** `StatsResult = { type: 'success'; data: ClaudeCodeStats } | { type: 'error'; message: string } | { type: 'empty' }`

**Constants:** `DATE_RANGE_LABELS: Record<StatsDateRange, string>` (`7d`, `30d`, `90d`)

---

### StatusLine.tsx

**Purpose:** Bottom status line showing model, permission mode, context usage, worktree, and session info.

**Exports:** `statusLineShouldDisplay(settings): boolean`, `StatusLine`

**Key behavior:** Reads model, permission mode, context usage, worktree status, and session info from AppState/settings.

---

### StatusNotices.tsx

**Purpose:** Renders active startup notices (e.g., deprecation warnings, MCP errors). Uses React `use()` for async memory file loading.

**Exports:** `StatusNotices`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `agentDefinitions` | `AgentDefinitionsResult` | no | Agent definitions for notice context |

**Key behavior:** Calls `getActiveNotices(context)` where context includes `config`, `agentDefinitions`, `memoryFiles`. Returns null if no active notices.

---

### StructuredDiff.tsx

**Purpose:** Renders a single diff hunk with syntax highlighting via the Rust `ColorDiff` NAPI module. Caches rendered output at module level (WeakMap) to survive remounts.

**Exports:** `StructuredDiff` (memo-wrapped)

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `patch` | `StructuredPatchHunk` | yes | Single diff hunk |
| `dim` | `boolean` | yes | Whether to dim the output |
| `filePath` | `string` | yes | File path for language detection |
| `firstLine` | `string \| null` | yes | First line for shebang detection |
| `fileContent` | `string` | no | Full file content for syntax context |
| `width` | `number` | yes | Render width |
| `skipHighlighting` | `boolean` | no | Skip syntax highlighting |

**Key behavior:** Module-level `RENDER_CACHE = new WeakMap<StructuredPatchHunk, Map<string, CachedRender>>()`. Two `RawAnsi` columns (gutter + content) replace N `DiffLine` rows. Gutter width = marker(1) + space + max_digits + space.

---

### StructuredDiffList.tsx

**Purpose:** Renders a list of diff hunks (from `StructuredDiff`) separated by ellipsis markers.

**Exports:** `StructuredDiffList`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `hunks` | `StructuredPatchHunk[]` | yes | Diff hunks to render |
| `dim` | `boolean` | yes | Dim mode |
| `width` | `number` | yes | Render width |
| `filePath` | `string` | yes | For language detection |
| `firstLine` | `string \| null` | yes | For shebang detection |
| `fileContent` | `string` | no | Full file for syntax context |

**Key behavior:** Uses `intersperse` utility to inject `<NoSelect>...</NoSelect>` separators between hunks.

---

### TagTabs.tsx

**Purpose:** Renders a horizontal tab bar for session log tag filtering with overflow handling and "← N" / "→ (tab to cycle)" hints.

**Exports:** `TagTabs`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `tabs` | `string[]` | yes | Tab labels |
| `selectedIndex` | `number` | yes | Currently selected tab index |
| `availableWidth` | `number` | yes | Display width constraint |
| `showAllProjects` | `boolean` | no | Whether all-projects mode is on |

**Constants:** `ALL_TAB_LABEL = 'All'`, `TAB_PADDING = 2`, `MAX_OVERFLOW_DIGITS = 2`

---

### TextInput.tsx

**Purpose:** Full-featured text input with voice recording waveform cursor animation, clipboard paste hint, and vim/normal mode routing.

**Exports:** `Props` (= `BaseTextInputProps & { highlights?: TextHighlight[] }`), `TextInput` (default export)

**Key behavior:**
- Uses `feature('VOICE_MODE')` for conditional voice recording waveform cursor.
- Smoothed waveform uses exponential moving average (`SMOOTH = 0.7`).
- `LEVEL_BOOST = 1.8` to amplify audio levels to full bar range.
- Delegates to `BaseTextInput` with `useTextInput` hook for state.

**Constants:** `BARS = ' ▁▂▃▄▅▆▇█'`, `CURSOR_WAVEFORM_WIDTH = 1`, `SMOOTH = 0.7`, `LEVEL_BOOST = 1.8`, `SILENCE_THRESHOLD = 0.15`

---

### ThemePicker.tsx

**Purpose:** Theme selection UI with live preview of a syntax-highlighted diff snippet.

**Exports:** `ThemePickerProps`, `ThemePicker`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `onThemeSelect` | `(setting: ThemeSetting) => void` | yes | Called when theme is selected |
| `showIntroText` | `boolean` | no | Show introductory text (default: false) |
| `helpText` | `string` | no | Help text below selector |
| `showHelpTextBelow` | `boolean` | no | Position of help text |
| `hideEscToCancel` | `boolean` | no | Hide Esc hint |
| `skipExitHandling` | `boolean` | no | Skip exit handling (for onboarding) |
| `onCancel` | `() => void` | no | Custom cancel handler |

---

### ThinkingToggle.tsx

**Purpose:** Picker to enable/disable extended thinking. Shows a confirmation prompt when toggling mid-conversation.

**Exports:** `Props`, `ThinkingToggle`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `currentValue` | `boolean` | yes | Current thinking enabled state |
| `onSelect` | `(enabled: boolean) => void` | yes | Selection callback |
| `onCancel` | `() => void` | no | Cancel callback |
| `isMidConversation` | `boolean` | no | Whether mid-conversation (affects confirmation) |

**Key state:** `confirmationPending: boolean | null`

---

### TokenWarning.tsx

**Purpose:** Displays context window usage warning. When `CONTEXT_COLLAPSE` is enabled, shows live collapse progress.

**Exports:** `TokenWarning`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `tokenUsage` | `number` | yes | Current token count |
| `model` | `string` | yes | Active model (for context window size) |

**Key behavior:** Contains internal `CollapseLabel` sub-component that subscribes to the collapse stats store via `useSyncExternalStore`.

---

### ToolUseLoader.tsx

**Purpose:** Animated tool use status dot (●). Blinks while unresolved, shows green for success, red for error.

**Exports:** `ToolUseLoader`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `isError` | `boolean` | yes | Whether the tool use errored |
| `isUnresolved` | `boolean` | yes | Whether still in progress |
| `shouldAnimate` | `boolean` | yes | Whether to animate blink |

**Key behavior:** Uses `useBlink`. Color: undefined (dim) when unresolved, 'error' on error, 'success' on success. Note: sensitive to dim+bold ANSI reset interaction (see chalk issue #290).

---

### ValidationErrorsList.tsx

**Purpose:** Renders a tree-formatted list of settings validation errors using dot-notation paths.

**Exports:** `ValidationErrorsList`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `errors` | `ValidationError[]` | yes | Validation errors to display |

**Key behavior:** Uses `lodash-es/setWith` to build nested tree from dot-notation paths, then `treeify` util to render. Formats array indices with their values for readability.

---

### VimTextInput.tsx

**Purpose:** Vim-mode text input. Wraps `BaseTextInput` with vim state management from `useVimInput`.

**Exports:** `Props` (= `VimTextInputProps & { highlights?: TextHighlight[] }`), `VimTextInput` (default export)

**Key behavior:** Passes full vim input props through `useVimInput`. Invert function applies `chalk.inverse` when terminal has focus.

---

### VirtualMessageList.tsx

**Purpose:** Virtualized scrollable message list with search highlighting, jump navigation, and sticky prompt support.

**Exports:** `StickyPrompt` (type), `JumpHandle` (imperative handle type), `VirtualMessageList`

**`JumpHandle` interface:**

| Method | Description |
|---|---|
| `jumpToIndex(index: number)` | Scroll to specific message index |
| `setSearchQuery(query: string)` | Set text search query |
| `nextMatch()` | Jump to next search match |
| `prevMatch()` | Jump to previous search match |
| `setAnchor(index: number)` | Set scroll anchor |
| `warmSearchIndex()` | Pre-warm search index |
| `disarmSearch()` | Clear search state |

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `messages` | `RenderableMessage[]` | yes | Messages to render |
| `scrollRef` | `RefObject<ScrollBoxHandle>` | yes | Scroll box ref |
| `columns` | `number` | yes | Terminal width |
| `itemKey` | `(msg: RenderableMessage) => string` | yes | Unique key function |
| `renderItem` | `(msg: RenderableMessage, index: number) => ReactNode` | yes | Item renderer |
| `onItemClick` | `(index: number) => void` | no | Click handler |

---

### WorkflowMultiselectDialog.tsx

**Purpose:** Multi-select dialog for choosing GitHub Actions workflows to install with the GitHub App integration.

**Exports:** `WorkflowMultiselectDialog`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `onSubmit` | `(selectedWorkflows: Workflow[]) => void` | yes | Called with selected workflows |
| `defaultSelections` | `Workflow[]` | yes | Initially selected workflows |

**Workflows:** `claude` (@Claude Code tag), `claude-review` (automated PR review)

---

### WorktreeExitDialog.tsx

**Purpose:** Dialog shown when exiting from a git worktree session. Asks whether to keep the worktree, clean it up, or eject commits.

**Exports:** `WorktreeExitDialog`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `onDone` | `(result?: string, options?: { display?: CommandResultDisplay }) => void` | yes | Close with result |
| `onCancel` | `() => void` | no | Cancel callback |

**Key state:** `status: 'loading' | 'asking' | 'keeping' | 'removing' | 'done'`, `changes: string[]`, `commitCount: number`, `resultMessage: string | undefined`

**Key behavior:** Reads git status and worktree commit count. Uses `cleanupWorktree`/`keepWorktree`/`killTmuxSession`. Lazy-requires `sessionStorage` to avoid circular import.

---

### messageActions.tsx

**Purpose:** Provides contexts and types for keyboard-driven message navigation in the virtual list.

**Exports:**

| Export | Type/Signature | Description |
|---|---|---|
| `NavigableType` | type | Union of navigable message types |
| `NavigableOf<T>` | generic type | Navigable message wrapper |
| `NavigableMessage` | type | Union of all navigable message variants |
| `isNavigableMessage(msg)` | `(msg: NormalizedMessage) => boolean` | Type guard |
| `PRIMARY_INPUT` | `Map<NavigableType, string>` | Maps type to primary keyboard action |
| `InVirtualListContext` | `React.Context<boolean>` | Whether inside virtual list |
| `MessageActionsSelectedContext` | `React.Context<boolean>` | Whether this message is selected |
| `MessageActionsNav` | type | Navigation state type |
| `MessageActionsState` | type | Full message actions state type |
| `useSelectedMessageBg` | hook | Returns background color for selected message |

---

## messages/ Subdirectory

---

### messages/AdvisorMessage.tsx

**Purpose:** Renders advisor (internal assistant) server tool use blocks with status indicator and optional JSON input.

**Exports:** `AdvisorMessage`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `block` | `AdvisorBlock` | yes | The advisor content block |
| `addMargin` | `boolean` | yes | Top margin |
| `resolvedToolUseIDs` | `Set<string>` | yes | Completed tool IDs |
| `erroredToolUseIDs` | `Set<string>` | yes | Errored tool IDs |
| `shouldAnimate` | `boolean` | yes | Animate loader |
| `verbose` | `boolean` | yes | Verbose mode |
| `advisorModel` | `string` | no | Advisor model name |

---

### messages/AssistantRedactedThinkingMessage.tsx

**Purpose:** Renders a placeholder for redacted thinking blocks ("✻ Thinking…").

**Exports:** `AssistantRedactedThinkingMessage`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `addMargin` | `boolean` | no | Top margin (default: false) |

---

### messages/AssistantTextMessage.tsx

**Purpose:** Renders assistant text response blocks. Handles many special API error string constants (rate limit messages, overload strings, etc.).

**Exports:** `AssistantTextMessage`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `param` | `TextBlockParam` | yes | The text content block |
| `addMargin` | `boolean` | yes | Top margin |
| `shouldShowDot` | `boolean` | yes | Show response dot prefix |
| `verbose` | `boolean` | yes | Verbose mode |
| `width` | `number` | no | Explicit width |
| `onOpenRateLimitOptions` | `() => void` | no | Opens rate limit upgrade options |

---

### messages/AssistantThinkingMessage.tsx

**Purpose:** Renders a thinking block — either as "∴ Thinking (ctrl+o to expand)" summary or full content in transcript/verbose mode.

**Exports:** `AssistantThinkingMessage`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `param` | `ThinkingBlock \| ThinkingBlockParam \| { type: 'thinking'; thinking: string }` | yes | Thinking content |
| `addMargin` | `boolean` | no | Top margin (default: false) |
| `isTranscriptMode` | `boolean` | yes | Show full thinking in transcript mode |
| `verbose` | `boolean` | yes | Show full thinking in verbose mode |
| `hideInTranscript` | `boolean` | no | Hide this block in transcript mode |

---

### messages/AssistantToolUseMessage.tsx

**Purpose:** Renders a tool use invocation block. Routes to the tool's own `renderToolUse` method or fallback.

**Exports:** `AssistantToolUseMessage`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `param` | `ToolUseBlockParam` | yes | Tool use content block |
| `addMargin` | `boolean` | yes | Top margin |
| `tools` | `Tools` | yes | Available tools |
| `commands` | `Command[]` | yes | Available commands |
| `verbose` | `boolean` | yes | Verbose mode |
| `inProgressToolUseIDs` | `Set<string>` | yes | In-flight tool IDs |
| `progressMessagesForMessage` | `Message[]` | yes | Progress messages |
| `shouldAnimate` | `boolean` | yes | Animate spinner |
| `shouldShowDot` | `boolean` | yes | Show dot prefix |
| `inProgressToolCallCount` | `number` | no | Count of in-flight calls |
| `lookups` | `ReturnType<typeof buildMessageLookups>` | yes | Message lookups |
| `isTranscriptMode` | `boolean` | no | Transcript mode |

---

### messages/AttachmentMessage.tsx

**Purpose:** Routes attachment messages to their specific renderers based on `attachment.type`. The `switch` default branch asserts `NullRenderingAttachmentType` via TypeScript to enforce exhaustiveness.

**Exports:** `AttachmentMessage`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `addMargin` | `boolean` | yes | Top margin |
| `attachment` | `Attachment` | yes | Attachment to render |
| `verbose` | `boolean` | yes | Verbose mode |
| `isTranscriptMode` | `boolean` | no | Transcript mode |

**Key behavior:** Feature-gated `EXPERIMENTAL_SKILL_SEARCH` for demo env detection. Handles `teammate_mailbox` attachment type specially (filters idle notifications before count). Uses `tryRenderPlanApprovalMessage` for plan-related attachments.

---

### messages/CollapsedReadSearchContent.tsx

**Purpose:** Renders the collapsed "Read N files, searched M patterns" summary line for collapsed read/search tool groups.

**Exports:** `CollapsedReadSearchContent`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `message` | `CollapsedReadSearchGroup` | yes | The collapsed group |
| `inProgressToolUseIDs` | `Set<string>` | yes | In-flight tool IDs |
| `shouldAnimate` | `boolean` | yes | Animate loader |
| `verbose` | `boolean` | yes | Verbose mode |
| `tools` | `Tools` | yes | Available tools |
| `lookups` | `ReturnType<typeof buildMessageLookups>` | yes | Message lookups |
| `isActiveGroup` | `boolean` | no | Whether this is the currently active group |

**Constants:** `MIN_HINT_DISPLAY_MS = 700`

**Key behavior:** Feature-gated team memory (`TEAMMEM`) counts. Internal `VerboseToolUse` sub-component for verbose mode.

---

### messages/CompactBoundaryMessage.tsx

**Purpose:** Renders the "✻ Conversation compacted (ctrl+o for history)" marker at compact boundaries.

**Exports:** `CompactBoundaryMessage`

**Props:** None.

---

### messages/GroupedToolUseContent.tsx

**Purpose:** Renders a grouped tool use by delegating to `tool.renderGroupedToolUse`.

**Exports:** `GroupedToolUseContent`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `message` | `GroupedToolUseMessage` | yes | The grouped tool use message |
| `tools` | `Tools` | yes | Available tools |
| `lookups` | `ReturnType<typeof buildMessageLookups>` | yes | Message lookups |
| `inProgressToolUseIDs` | `Set<string>` | yes | In-flight tool IDs |
| `shouldAnimate` | `boolean` | yes | Animate spinner |

---

### messages/HighlightedThinkingText.tsx

**Purpose:** Renders thinking/prompt text with optional KAIROS brief layout mode. Supports "You {timestamp}" header in brief layout. Applies rainbow color to "ultrathink" trigger sequences.

**Exports:** `HighlightedThinkingText`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `text` | `string` | yes | The text to render |
| `useBriefLayout` | `boolean` | no | Use brief (compact) layout |
| `timestamp` | `string` | no | ISO timestamp for brief header |

**Key behavior:** Uses `findThinkingTriggerPositions` and `getRainbowColor` for ultrathink highlighting. Uses `QueuedMessageContext` for queued state styling.

---

### messages/HookProgressMessage.tsx

**Purpose:** Renders hook execution progress for PreToolUse/PostToolUse hooks.

**Exports:** `HookProgressMessage`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `hookEvent` | `HookEvent` | yes | The hook event |
| `lookups` | `ReturnType<typeof buildMessageLookups>` | yes | Message lookups |
| `toolUseID` | `string` | yes | Associated tool use ID |
| `verbose` | `boolean` | yes | Verbose mode |
| `isTranscriptMode` | `boolean` | no | Transcript mode |

---

### messages/nullRenderingAttachments.ts

**Purpose:** Defines which attachment types render as `null` (no visible output) and should be filtered before the 200-message render budget.

**Exports:**

| Export | Type | Description |
|---|---|---|
| `NullRenderingAttachmentType` | type | Union of 29 null-rendering attachment type strings |
| `isNullRenderingAttachment(msg)` | `(msg: Message \| NormalizedMessage) => boolean` | Returns true if message is a null-rendering attachment |

**Null-rendering types (29 total):** `hook_success`, `hook_additional_context`, `hook_cancelled`, `command_permissions`, `agent_mention`, `budget_usd`, `critical_system_reminder`, `edited_image_file`, `edited_text_file`, `opened_file_in_ide`, `output_style`, `plan_mode`, `plan_mode_exit`, `plan_mode_reentry`, `structured_output`, `team_context`, `todo_reminder`, `context_efficiency`, `deferred_tools_delta`, `mcp_instructions_delta`, `companion_intro`, `token_usage`, `ultrathink_effort`, `max_turns_reached`, `task_reminder`, `auto_mode`, `auto_mode_exit`, `output_token_usage`, `pen_mode_enter`, `pen_mode_exit`, `verify_plan_reminder`, `current_session_memory`, `compaction_reminder`, `date_change`

**Note:** TypeScript enforces sync: `AttachmentMessage`'s `switch` default branch asserts `attachment.type satisfies NullRenderingAttachmentType`. Tracked as CC-724.

---

### messages/PlanApprovalMessage.tsx

**Purpose:** Renders plan approval request/response display and handles plan-related attachments.

**Exports:** `PlanApprovalRequestDisplay`, `tryRenderPlanApprovalMessage`, plus other plan approval message components.

---

### messages/RateLimitMessage.tsx

**Purpose:** Renders rate limit error messages with optional upsell text.

**Exports:** `getUpsellMessage(params: UpsellParams): string | null`, `RateLimitMessage`

**Key behavior:** `getUpsellMessage` returns upgrade-prompt text based on subscription type and rate limit reason.

---

### messages/ShutdownMessage.tsx

**Purpose:** Renders swarm agent shutdown-related messages.

**Exports:** `ShutdownRequestDisplay`, `ShutdownRejectedDisplay`

**`ShutdownRequestDisplay` Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `request` | `ShutdownRequestMessage` | yes | Shutdown request data |

**Key behavior:** Shows warning-colored bordered box with `from` and `reason`. `ShutdownRejectedDisplay` shows subtle (grey) bordered box.

---

### messages/SystemAPIErrorMessage.tsx

**Purpose:** Displays API error messages with a countdown timer before retry. Hidden for first 3 retries.

**Exports:** `SystemAPIErrorMessage`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `message` | `SystemAPIErrorMessage` | yes | The error message |
| `verbose` | `boolean` | yes | Verbose mode |

**Key state:** `countdownMs: number`

**Constants:** `MAX_API_ERROR_CHARS = 1000`

**Key behavior:** `hidden = retryAttempt < 4`. Uses `useInterval` to tick countdown toward `retryInMs`. Shows `retryAttempt / maxRetries` progress.

---

### messages/SystemTextMessage.tsx

**Purpose:** Renders all system message subtypes: turn duration, stop hook summary, bridge status, thinking, memory saved, and generic text.

**Exports:** `SystemTextMessage`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `message` | `SystemMessage` | yes | System message to render |
| `addMargin` | `boolean` | yes | Top margin |
| `verbose` | `boolean` | yes | Verbose mode |
| `isTranscriptMode` | `boolean` | no | Transcript mode |

**Key behavior:** Routes to `TurnDurationMessage`, `StopHookSummaryMessage`, etc. based on `message.subtype`. Feature-gated `TEAMMEM` for memory saved display. Uses `TURN_COMPLETION_VERBS` for random completion verb selection.

---

### messages/TaskAssignmentMessage.tsx

**Purpose:** Renders a task assignment from the coordinator to a sub-agent.

**Exports:** `TaskAssignmentDisplay`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `assignment` | `TaskAssignmentMessage` | yes | Task assignment data |

**Key behavior:** Renders cyan-bordered box with task ID, assigned-by, subject, and optional description.

---

### messages/UserAgentNotificationMessage.tsx

**Purpose:** Renders agent task-completion notifications extracted from XML `<task-notification>` tag. Shows colored bullet with summary and optional detail line.

**Exports:** `UserAgentNotificationMessage`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `addMargin` | `boolean` | yes | Top margin |
| `param` | `TextBlockParam` | yes | Text block containing `<task-notification>` XML |

**Key behavior:** Extracts `<summary>` and `<status>` inner tags. `getStatusColor(status)` maps: `'completed'` → `'success'`, `'failed'` → `'error'`, `'killed'` → `'warning'`, default → `'text'`. Renders `BLACK_CIRCLE` in status color followed by summary text. Returns null if no summary.

---

### messages/UserBashInputMessage.tsx

**Purpose:** Renders the bash command input (extracted from `<bash-input>` XML tag) with bash border styling.

**Exports:** `UserBashInputMessage`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `addMargin` | `boolean` | yes | Top margin |
| `param` | `TextBlockParam` | yes | Text block containing bash input tag |

**Key behavior:** Extracts `<bash-input>` tag. Returns null if no input. Renders `! {command}` with `bashMessageBackgroundColor`.

---

### messages/UserBashOutputMessage.tsx

**Purpose:** Renders bash tool output (stdout + stderr) by delegating to `BashToolResultMessage`.

**Exports:** `UserBashOutputMessage`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `content` | `string` | yes | Raw bash output XML |
| `verbose` | `boolean` | yes | Verbose mode |

**Key behavior:** Extracts `<bash-stdout>` and `<bash-stderr>` tags (handles `<persisted-output>` within stdout). Delegates to `BashToolResultMessage`.

---

### messages/UserChannelMessage.tsx

**Purpose:** Renders messages received over the bridge/channel connection (e.g., from Slack plugin). Parses `<channel source="..." user="..." chat_id="...">content</channel>` XML format.

**Exports:** `UserChannelMessage`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `addMargin` | `boolean` | yes | Top margin |
| `param` | `TextBlockParam` | yes | Text block containing `<channel>` XML |

**Constants:** `TRUNCATE_AT = 60` — body is whitespace-collapsed and truncated to 60 characters for display.

**Key behavior:** Regex parses `source`, optional `user` attribute, and body content. Plugin-provided server names (e.g., `plugin:slack-channel:slack`) are stripped to the leaf after the last `:`. Shows `CHANNEL_ARROW` prefix, server name, optional user attribution, and truncated message body. Returns null if regex doesn't match.

---

### messages/UserCommandMessage.tsx

**Purpose:** Renders a user slash command (e.g., `/help`, `/compact`) extracted from XML `<command-message>` tag. Supports both regular command display and `skill-format` display.

**Exports:** `UserCommandMessage`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `addMargin` | `boolean` | yes | Top margin |
| `param` | `TextBlockParam` | yes | Text block containing `<command-message>` XML |

**Key behavior:** Extracts `<command-message>` and `<command-args>` tags. If `<skill-format>true</skill-format>` is present, renders as `Skill(commandName)` with subtle pointer icon. Otherwise renders the command name with `figures.pointer`. Returns null if no command message found.

---

### messages/UserImageMessage.tsx

**Purpose:** Renders an image attachment in user messages. Shows as a clickable OSC 8 hyperlink if image is stored and terminal supports hyperlinks.

**Exports:** `UserImageMessage`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `imageId` | `number` | no | ID for stored image lookup via `getStoredImagePath` |
| `addMargin` | `boolean` | no | Top margin (for images starting a new turn without preceding text) |

**Key behavior:** Label is `[Image #N]` or `[Image]`. If `imageId` is provided and `getStoredImagePath(imageId)` returns a path and `supportsHyperlinks()` is true, wraps content in an Ink `Link` with a `file://` URL. When `addMargin` is true, wraps in `Box marginTop={1}`; otherwise uses `MessageResponse` styling to appear connected to the message above.

---

### messages/UserLocalCommandOutputMessage.tsx

**Purpose:** Renders local command output from `<local-command-stdout>` and `<local-command-stderr>` tags. Used for displaying output from locally-executed hooks/commands.

**Exports:** `UserLocalCommandOutputMessage`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `content` | `string` | yes | Raw XML containing stdout/stderr tags |

**Key behavior:** Extracts `<local-command-stdout>` and `<local-command-stderr>` tags. If neither is present, renders `NO_CONTENT_MESSAGE` dimmed in a `MessageResponse`. Each non-empty trimmed stream is rendered via internal `IndentedContent` component. `IndentedContent` checks if content already starts with `DIAMOND_OPEN` or `DIAMOND_FILLED` prefix (avoids double-prefixing); otherwise renders indented with `Markdown` or plain text depending on content type.

---

### messages/UserMemoryInputMessage.tsx

**Purpose:** Renders a user-triggered memory save notification. Displays the memory content with a distinctive `#` prefix and a randomly-chosen acknowledgment phrase.

**Exports:** `UserMemoryInputMessage`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `addMargin` | `boolean` | yes | Top margin |
| `text` | `string` | yes | Raw text containing `<user-memory-input>` XML tag |

**Key behavior:** Extracts `<user-memory-input>` tag content. Returns null if not found. Renders the memory content with `#` prefix in `remember` color on `memoryBackgroundColor` background. Shows a randomized acknowledgment (`'Got it.'`, `'Good to know.'`, `'Noted.'`) from `lodash-es/sample` in a `MessageResponse` with `height={1}`.

---

### messages/UserPlanMessage.tsx

**Purpose:** Renders a plan content block with structured formatting.

**Exports:** `UserPlanMessage`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `addMargin` | `boolean` | yes | Top margin |
| `planContent` | `string` | yes | Plan content text |

---

### messages/UserPromptMessage.tsx

**Purpose:** Renders a user text prompt with truncation for long inputs. Feature-gated brief layout for KAIROS.

**Exports:** `UserPromptMessage`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `addMargin` | `boolean` | yes | Top margin |
| `param` | `TextBlockParam` | yes | Text content block |
| `isTranscriptMode` | `boolean` | no | Transcript mode |
| `timestamp` | `string` | no | ISO timestamp |

**Constants:** `MAX_DISPLAY_CHARS = 10000`, `TRUNCATE_HEAD_CHARS = 2500`, `TRUNCATE_TAIL_CHARS = 2500`

**Key behavior:** Feature-gated `KAIROS` / `KAIROS_BRIEF` for `isBriefOnly` layout. Truncates to `HEAD...TAIL` with "+N chars omitted" notice.

---

### messages/UserResourceUpdateMessage.tsx

**Purpose:** Renders MCP resource and polling update notifications, showing what resources changed and why.

**Exports:** `UserResourceUpdateMessage`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `addMargin` | `boolean` | yes | Top margin |
| `param` | `TextBlockParam` | yes | Text block containing `<mcp-resource-update>` / `<mcp-polling-update>` XML |

**Internal types:**

```ts
type ParsedUpdate = {
  kind: 'resource' | 'polling';
  server: string;
  target: string;  // URI for resource updates, tool name for polling
  reason?: string;
}
```

**Key behavior:** `parseUpdates(text)` uses two regexes: `<mcp-resource-update server="..." uri="...">` and `<mcp-polling-update type="..." server="..." tool="...">`. `formatUri(uri)` strips `file://` prefix and shows filename only; truncates other URIs to 39 chars + ellipsis if over 40. Shows `REFRESH_ARROW` icon, server name, target, and optional reason per update. Returns null if no updates found.

---

### messages/UserTeammateMessage.tsx

**Purpose:** Renders messages from teammate (sub-agent/coordinator) delivered via the mailbox XML protocol. Dispatches to specialized renderers for plans, shutdowns, and task assignments.

**Exports:** `TeammateMessageContent`, `UserTeammateMessage`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `addMargin` | `boolean` | yes | Top margin |
| `param` | `TextBlockParam` | yes | Text block containing `<teammate-message>` XML tags |
| `isTranscriptMode` | `boolean` | no | Transcript display mode |

**Internal type:**

```ts
type ParsedMessage = {
  teammateId: string;
  content: string;
  color?: string;
  summary?: string;
}
```

**Key behavior:** `TEAMMATE_MSG_REGEX` matches `<teammate-message teammate_id="..." color="..." summary="...">content</teammate-message>`. Pre-filters messages: removes `isShutdownApproved` lifecycle messages and `teammate_terminated` JSON payloads (avoids blank line artifacts). For each remaining message, tries in order: `tryRenderPlanApprovalMessage`, `tryRenderShutdownMessage`, `tryRenderTaskAssignmentMessage`. Falls back to generic rendering with teammate ID as colored label (using `toInkColor(color)`) and content. Special `'leader'` teammateId displays as `'leader'`.

---

### messages/UserTextMessage.tsx

**Purpose:** Master router for user text messages. Extracts XML tags and dispatches to specific sub-renderers.

**Exports:** `UserTextMessage`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `addMargin` | `boolean` | yes | Top margin |
| `param` | `TextBlockParam` | yes | Text content block |
| `verbose` | `boolean` | yes | Verbose mode |
| `planContent` | `string` | no | Plan content extracted separately |
| `isTranscriptMode` | `boolean` | no | Transcript mode |
| `timestamp` | `string` | no | ISO timestamp |

**Key behavior:** Dispatches to: `UserPlanMessage` (planContent), `UserBashInputMessage` (`<bash-input>`), `UserBashOutputMessage` (`<bash-stdout>`/`<bash-stderr>`), `UserCommandMessage` (`<command-message>`), `UserLocalCommandOutputMessage` (`<local-command-stdout>`), `UserMemoryInputMessage` (`<tick>`), `UserTeammateMessage` (`<teammate-message>`), `UserAgentNotificationMessage` (`<task-notification>`), `InterruptedByUser` (INTERRUPT_MESSAGE), `UserResourceUpdateMessage`, `UserPromptMessage` (fallthrough).

---

### messages/teamMemCollapsed.tsx

**Purpose:** Renders team memory operation counts (reads/searches/writes) for the collapsed read/search summary. Loaded lazily when `feature('TEAMMEM')` is true.

**Exports:** `checkHasTeamMemOps(message: CollapsedReadSearchGroup): boolean`, `TeamMemCountParts`

**`TeamMemCountParts` Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `message` | `CollapsedReadSearchGroup` | yes | The collapsed group |
| `isActiveGroup` | `boolean` | yes | Whether currently active |
| `hasPrecedingParts` | `boolean` | yes | Whether other counts precede this |

**Key behavior:** Plain function (not React component) to prevent React Compiler from memoizing property accesses. Renders comma-separated "Recalled N", "Searched N", "Saved N" parts.

---

### messages/teamMemSaved.ts

**Purpose:** Utility for extracting team memory save info from a system memory-saved message.

**Exports:** `teamMemSavedPart(message: SystemMemorySavedMessage): { segment: string; count: number } | null`

---

## messages/UserToolResultMessage/ Subdirectory

---

### UserToolResultMessage/UserToolResultMessage.tsx

**Purpose:** Routes tool result blocks to specific sub-renderers based on result status.

**Exports:** `UserToolResultMessage`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `param` | `ToolResultBlockParam` | yes | Tool result content |
| `message` | `NormalizedUserMessage` | yes | Parent message |
| `lookups` | `ReturnType<typeof buildMessageLookups>` | yes | Message lookups |
| `progressMessagesForMessage` | `Message[]` | yes | Progress messages |
| `style` | `'condensed'` | no | Condensed display mode |
| `tools` | `Tools` | yes | Available tools |
| `verbose` | `boolean` | yes | Verbose mode |
| `width` | `number` | yes | Render width |
| `isTranscriptMode` | `boolean` | no | Transcript mode |

**Key behavior:** Routes to:
- `UserToolCanceledMessage` — when result text matches `CANCEL_MESSAGE`
- `UserToolRejectMessage` — when result text matches `REJECT_MESSAGE` or `INTERRUPT`
- `UserToolErrorMessage` — when `is_error: true`
- `UserToolSuccessMessage` — otherwise

---

### UserToolResultMessage/UserToolCanceledMessage.tsx

**Purpose:** No-props component. Renders the "Interrupted · What should Claude do instead?" message for explicitly canceled tool uses.

**Exports:** `UserToolCanceledMessage`

**Props:** None.

**Key behavior:** Wraps `InterruptedByUser` in a `MessageResponse` with `height={1}`. Fully static — memoized as a module-level constant by React Compiler.

---

### UserToolResultMessage/UserToolErrorMessage.tsx

**Purpose:** Renders error results for tool uses. Detects interrupt, plan rejection, classifier denial, and custom reject-with-reason prefix before falling back to the tool's own error renderer.

**Exports:** `UserToolErrorMessage`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `progressMessagesForMessage` | `ProgressMessage[]` | yes | Progress messages for this tool use |
| `tool` | `Tool` | no | Tool definition (undefined for old conversations) |
| `tools` | `Tools` | yes | All available tools |
| `param` | `ToolResultBlockParam` | yes | Tool result block |
| `verbose` | `boolean` | yes | Verbose mode |
| `isTranscriptMode` | `boolean` | no | Transcript mode |

**Key behavior:** Routes in priority order:
1. `INTERRUPT_MESSAGE_FOR_TOOL_USE` → `InterruptedByUser` in `MessageResponse`
2. `PLAN_REJECTION_PREFIX` → `RejectedPlanMessage` with plan content extracted
3. `isClassifierDenial(param.content)` or `REJECT_MESSAGE_WITH_REASON_PREFIX` → `RejectedToolUseMessage`
4. If tool defines `renderToolUseErrorMessage` → delegates to tool renderer
5. Fallback → `FallbackToolUseErrorMessage`

---

### UserToolResultMessage/UserToolRejectMessage.tsx

**Purpose:** Renders rejection messages for tool uses that were explicitly rejected by user permission rules. Routes to tool-specific renderer or generic fallback.

**Exports:** `UserToolRejectMessage`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `input` | `Record<string, unknown>` | yes | Tool input parameters |
| `progressMessagesForMessage` | `ProgressMessage[]` | yes | Progress messages |
| `style` | `'condensed'` | no | Condensed display mode |
| `tool` | `Tool` | no | Tool definition |
| `tools` | `Tools` | yes | All available tools |
| `lookups` | `ReturnType<typeof buildMessageLookups>` | yes | Message lookups |
| `verbose` | `boolean` | yes | Verbose mode |
| `isTranscriptMode` | `boolean` | no | Transcript mode |

**Key behavior:** If tool has no `renderToolUseRejectedMessage`, falls back to `FallbackToolUseRejectedMessage`. Otherwise calls `tool.renderToolUseRejectedMessage(input, columns, theme, verbose, isTranscriptMode, filterToolProgressMessages(...))`.

---

### UserToolResultMessage/UserToolSuccessMessage.tsx

**Purpose:** Renders successful tool result output. Validates against the tool's output schema before rendering, and supports classifier approval banners and KAIROS brief mode.

**Exports:** `UserToolSuccessMessage`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `message` | `NormalizedUserMessage` | yes | Parent message |
| `lookups` | `ReturnType<typeof buildMessageLookups>` | yes | Message lookups |
| `toolUseID` | `string` | yes | Tool use ID for result lookup |
| `progressMessagesForMessage` | `ProgressMessage[]` | yes | Progress messages |
| `style` | `'condensed'` | no | Condensed display mode |
| `tool` | `Tool` | no | Tool definition |
| `tools` | `Tools` | yes | All available tools |
| `verbose` | `boolean` | yes | Verbose mode |
| `width` | `number \| string` | yes | Render width |
| `isTranscriptMode` | `boolean` | no | Transcript mode |

**Key behavior:**
- Feature-gated `KAIROS` / `KAIROS_BRIEF` reads `isBriefOnly` from `useAppState` (same pattern as `UserPromptMessage` to avoid subscriptions in non-KAIROS builds).
- Captures `classifierRule` and `yoloReason` via `useState` lazy initializer from `getClassifierApproval(toolUseID)` / `getYoloClassifierApproval(toolUseID)`. Deletes from map on effect to prevent linear memory growth.
- Validates `message.toolUseResult` against `tool.outputSchema?.safeParse(...)` — returns null if parse fails (handles corrupt/old-format resumed transcripts; issue anthropics/claude-code#39817).
- Wraps tool's `renderToolResultMessage` in `SentryErrorBoundary`.
- Renders `HookProgressMessage` items from filtered progress messages.

---

### UserToolResultMessage/RejectedPlanMessage.tsx

**Purpose:** Renders a user-rejected plan approval. Shows the plan content in a styled box.

**Exports:** `RejectedPlanMessage`

**Props:**

| Prop | Type | Required | Description |
|---|---|---|---|
| `plan` | `string` | yes | Plan content text |

**Key behavior:** Renders "User rejected Claude's plan:" label in subtle color above a round-bordered `planMode`-colored box containing the plan rendered via `Markdown`. Uses `overflow="hidden"` for correct Windows Terminal rendering.

---

### UserToolResultMessage/RejectedToolUseMessage.tsx

**Purpose:** No-props component. Renders the generic "Tool use rejected" dimmed message.

**Exports:** `RejectedToolUseMessage`

**Props:** None.

**Key behavior:** Fully static — renders dimmed "Tool use rejected" text in a `MessageResponse` with `height={1}`. Memoized as a module-level constant by React Compiler.

---

### UserToolResultMessage/utils.tsx

**Purpose:** Shared React hook for resolving a `Tool` definition from a tool use ID via message lookups.

**Exports:** `useGetToolFromMessages(toolUseID: string, tools: Tools, lookups: ReturnType<typeof buildMessageLookups>): { tool: Tool; toolUse: ToolUseBlockParam } | null`

**Key behavior:** Memoized hook. Looks up `toolUseByToolUseID.get(toolUseID)`, then resolves via `findToolByName(tools, toolUse.name)`. Returns null if either lookup fails.
