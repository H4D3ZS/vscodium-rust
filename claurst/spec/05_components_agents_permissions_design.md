# Claude Code — Components: Agents, Permissions, Design System & Feature Modules

This document provides an exhaustive spec for all components in the `src/components/` subdirectories covering agents management, permission UIs, the design system primitives, and the many feature-specific modules (MCP, memory, tasks, teams, diff, grove, hooks, HelpV2, TrustDialog, ManagedSettingsSecurityDialog, ClaudeCodeHint, HighlightedCode, LogoV2, DesktopUpsell, FeedbackSurvey, LspRecommendation, Passes, Spinner, PromptInput, CustomSelect, Settings, sandbox, shell, skills, ui, wizard).

---

## Table of Contents

1. [agents/](#1-agents)
2. [permissions/](#2-permissions)
3. [design-system/](#3-design-system)
4. [wizard/](#4-wizard)
5. [mcp/](#5-mcp)
6. [memory/](#6-memory)
7. [tasks/](#7-tasks)
8. [teams/](#8-teams)
9. [diff/](#9-diff)
10. [grove/](#10-grove)
11. [hooks/](#11-hooks)
12. [HelpV2/](#12-helpv2)
13. [TrustDialog/](#13-trustdialog)
14. [ManagedSettingsSecurityDialog/](#14-managedsettingssecuritydialog)
15. [ClaudeCodeHint/](#15-claudecodehint)
16. [HighlightedCode/](#16-highlightedcode)
17. [LogoV2/](#17-logov2)
18. [DesktopUpsell/](#18-desktopupsell)
19. [FeedbackSurvey/](#19-feedbacksurvey)
20. [LspRecommendation/](#20-lsprecommendation)
21. [Passes/](#21-passes)
22. [Spinner/](#22-spinner)
23. [PromptInput/](#23-promptinput)
24. [CustomSelect/](#24-customselect)
25. [Settings/](#25-settings)
26. [sandbox/](#26-sandbox)
27. [shell/](#27-shell)
28. [skills/](#28-skills)
29. [ui/](#29-ui)

---

## 1. agents/

The agents subsystem provides a full UI for creating, viewing, editing, deleting, and listing custom Claude sub-agents (stored as Markdown files with YAML front matter in `.claude/agents/`).

### 1.1 AgentDetail

**File:** `agents/AgentDetail.tsx`

**Purpose:** Renders a read-only detail view of a single `AgentDefinition`, showing all agent metadata (file path, description, tools, model, permission mode, memory, hooks, skills, color swatch, and system prompt for non-built-in agents).

**Props Interface:**
```typescript
type Props = {
  agent: AgentDefinition;      // The agent to display
  tools: Tools;                // All available tools (for resolving tool names)
  allAgents?: AgentDefinition[]; // All agents (unused in current render, passed for context)
  onBack: () => void;          // Called when user presses Esc or Enter
}
```

**Key Behaviors:**
- Resolves tool list via `resolveAgentTools(agent, tools, false)` — shows "All tools", named valid tools, and warns about unrecognized tool names with a warning symbol.
- Computes `backgroundColor` from `getAgentColor(agent.agentType)` for the color swatch preview.
- Binds `confirm:no` keybinding in Confirmation context to call `onBack`.
- Binds `return` keypress to `onBack`.
- Renders system prompt via `<Markdown>` only for non-built-in agents (`!isBuiltInAgent(agent)`).
- Displays model via `getAgentModelDisplay(agent.model)`.
- Displays memory via `getMemoryScopeDisplay(agent.memory)`.
- Displays skills count inline when > 10 entries.

**Exports:** `AgentDetail`

---

### 1.2 AgentEditor

**File:** `agents/AgentEditor.tsx`

**Purpose:** Full-screen editor for modifying an existing `AgentDefinition`. Supports editing the agent type (name), system prompt, description (whenToUse), tools, model, color, memory, and effort. Persists changes by calling `updateAgentFile`. Provides a step-by-step editing flow using a sub-state machine.

**Props Interface:** (large component; reconstructed from source)
```typescript
type Props = {
  agent: AgentDefinition;          // The agent to edit
  tools: Tools;                    // All available tools
  existingAgents: AgentDefinition[]; // For duplicate-name validation
  onComplete: (message: string) => void; // Called after successful save
  onCancel: () => void;            // Called when user cancels
}
```

**Key Behaviors:**
- Multi-step flow: main edit menu → individual field editors (each with their own step).
- Edit menu shows: Edit system prompt, Edit description, Select tools, Select model, Choose color, (conditional) Choose memory, Back.
- Each field editor reuses the same sub-components used in wizard steps (ToolSelector, ModelSelector, ColorPicker, etc.).
- On save calls `updateAgentFile(agent, ...)` and reloads agent state in app state.
- Validates agent type via `validateAgentType()` before allowing name changes.
- Uses React compiler memoization throughout.

**Exports:** `AgentEditor`

---

### 1.3 AgentNavigationFooter

**File:** `agents/AgentNavigationFooter.tsx`

**Purpose:** Renders a dimmed footer line with keyboard navigation instructions. Integrates with Ctrl+C/D exit state to show "Press X again to exit" during double-tap detection.

**Props Interface:**
```typescript
type Props = {
  instructions?: string;
  // Default: "Press ↑↓ to navigate · Enter to select · Esc to go back"
}
```

**Key Behaviors:**
- Calls `useExitOnCtrlCDWithKeybindings()` to detect pending exit.
- When `exitState.pending` is true, overrides instruction text with `"Press ${exitState.keyName} again to exit"`.
- Renders with `marginLeft={2}` and `dimColor`.

**Exports:** `AgentNavigationFooter`

---

### 1.4 AgentsList

**File:** `agents/AgentsList.tsx`

**Purpose:** Displays a list of agents for a given source (all, built-in, userSettings, projectSettings, etc.), grouped by source category. Supports keyboard navigation, optional "Create new agent" entry, and shows override warnings when an agent is shadowed by another scope.

**Props Interface:**
```typescript
type Props = {
  source: SettingSource | 'all' | 'built-in' | 'plugin';
  agents: ResolvedAgent[];           // Agents resolved with override info
  onBack: () => void;
  onSelect: (agent: AgentDefinition) => void;
  onCreateNew?: () => void;          // If provided, shows "Create new agent" option at top
  changes?: string[];                // Recent change messages to display
}
```

**Key Behaviors:**
- State: `selectedAgent` (currently highlighted agent), `isCreateNewSelected` (boolean, focuses "Create new agent").
- Auto-selects first agent or "Create new" if nothing selected.
- `handleKeyDown`: Up/Down arrows navigate the list (wraps around). Enter activates selected item.
- Groups agents by source using `AGENT_SOURCE_GROUPS`.
- Built-in agents are rendered separately with `renderBuiltInAgentsSection()` (dimmed, no pointer).
- Per-agent display: name, optional model display (middot-separated), memory label, override shadow warning.
- Shadowed agents shown dimmed with a warning symbol (`figures.warning`) and "shadowed by X" text.
- Changes list shown at top with success color.

**Exports:** `AgentsList`

---

### 1.5 AgentsMenu

**File:** `agents/AgentsMenu.tsx`

**Purpose:** Top-level orchestrator for the agents management UI. Implements a state machine with modes: `list-agents`, `create-agent`, `agent-menu`, `view-agent`, `edit-agent`, `delete-confirm`. Reads/writes app state for agent definitions.

**Props Interface:**
```typescript
type Props = {
  tools: Tools;
  onExit: (result?: string, options?: { display?: CommandResultDisplay }) => void;
}
```

**Key Behaviors:**
- `modeState` union type discriminated by `mode` field.
- Reads `agentDefinitions`, `mcpTools`, `toolPermissionContext` from app state.
- Merges tools via `useMergedTools()`.
- Groups agents into 8 buckets by source (built-in, userSettings, projectSettings, policySettings, localSettings, flagSettings, plugin, all).
- Resolves display-time overrides via `resolveAgentOverrides()`.
- `handleAgentCreated`: adds change message, returns to list-agents/all view.
- `handleAgentDeleted`: calls `deleteAgentFromFile()`, updates app state, adds change message.
- In `agent-menu` mode: renders a `<Select>` with options View / Edit (if editable) / Delete (if editable) / Back. Editability requires `source` not in `['built-in', 'plugin', 'flagSettings']`.
- In `delete-confirm` mode: shows a confirmation `<Dialog>` with Yes/No options.
- On exit: formats change summary or "Agents dialog dismissed" system message.

**Exports:** `AgentsMenu`

---

### 1.6 ColorPicker

**File:** `agents/ColorPicker.tsx`

**Purpose:** Interactive color selector for agent type color. Displays all `AGENT_COLORS` plus an "Automatic color" option. Shows a live preview of the color applied to the agent name.

**Props Interface:**
```typescript
type Props = {
  agentName: string;                           // For preview display
  currentColor?: AgentColorName | 'automatic';  // Pre-selected color
  onConfirm: (color: AgentColorName | undefined) => void; // undefined = automatic
}
```

**Key Behaviors:**
- `COLOR_OPTIONS = ['automatic', ...AGENT_COLORS]`.
- State: `selectedIndex` initialized from `currentColor` or 0.
- Up/Down arrow keys navigate; Enter confirms. `'automatic'` selection calls `onConfirm(undefined)`.
- Preview box shows agent name with background color or inverse styling.
- Uses `AGENT_COLOR_TO_THEME_COLOR` for actual terminal color values.

**Exports:** `ColorPicker`

---

### 1.7 ModelSelector

**File:** `agents/ModelSelector.tsx`

**Purpose:** Renders a `<Select>` component for choosing an agent model from the standard list (`getAgentModelOptions()`). Injects the current model as a custom option if it is a full model ID not in the standard alias list.

**Props Interface:**
```typescript
interface ModelSelectorProps {
  initialModel?: string;                     // Current model value
  onComplete: (model?: string) => void;      // Called with selected model
  onCancel?: () => void;                     // If absent, cancel calls onComplete(undefined)
}
```

**Key Behaviors:**
- If `initialModel` is not in the standard options list, prepends `{ value: initialModel, label: initialModel, description: "Current model (custom ID)" }`.
- Default value: `initialModel ?? 'sonnet'`.
- Cancel: calls `onCancel()` if provided, otherwise `onComplete(undefined)`.
- Renders a description line: "Model determines the agent's reasoning capabilities and speed."

**Exports:** `ModelSelector`

---

### 1.8 ToolSelector

**File:** `agents/ToolSelector.tsx`

**Purpose:** Multi-select UI for choosing which tools an agent can access. Groups tools into buckets: Read-only, Edit, Execution, MCP (per-server), and Other. Supports "All tools" wildcard selection.

**Props Interface:**
```typescript
type Props = {
  tools: Tools;                                        // All available tools
  initialTools: string[] | undefined;                  // Pre-selected tool names (undefined = all)
  onComplete: (selectedTools: string[] | undefined) => void; // undefined = all tools
  onCancel?: () => void;
}

type ToolBucket = {
  name: string;
  toolNames: Set<string>;
  isMcp?: boolean;
};

type ToolBuckets = {
  READ_ONLY: ToolBucket;  // GlobTool, GrepTool, FileReadTool, WebFetchTool, etc.
  EDIT: ToolBucket;       // FileEditTool, FileWriteTool, NotebookEditTool
  EXECUTION: ToolBucket;  // BashTool, (TungstenTool for internal builds)
  MCP: ToolBucket;        // Dynamic, populated per MCP server
  OTHER: ToolBucket;      // Uncategorized catch-all
};
```

**Key Behaviors:**
- MCP tools grouped dynamically by server name via `getMcpServerBuckets()`.
- Tools not matching any bucket go into OTHER.
- AGENT_TOOL_NAME excluded from selectable tools.
- Toggle selection per individual tool; "Select all" / "Deselect all" shortcuts within buckets.
- Up/Down navigation; Space/Enter toggle; Tab/Shift+Tab move between buckets.
- On submit: if no tools selected and wildcard not set, passes empty array. Pressing a confirm shortcut with no selection can also pass `undefined` (all tools).

**Exports:** `ToolSelector`

---

### 1.9 agentFileUtils.ts

**File:** `agents/agentFileUtils.ts`

**Purpose:** File system utility functions for reading and writing agent definition Markdown files.

**Exports:**

```typescript
// Formats agent fields into a Markdown file with YAML front matter
function formatAgentAsMarkdown(
  agentType: string,
  whenToUse: string,
  tools: string[] | undefined,
  systemPrompt: string,
  color?: string,
  model?: string,
  memory?: AgentMemoryScope,
  effort?: EffortValue,
): string

// Returns the absolute directory for an agent based on source
function getAgentDirectoryPath(location: SettingSource): string  // (private)

// Returns relative directory path for display
function getRelativeAgentDirectoryPath(location: SettingSource): string  // (private)

// Path for a NEW agent file (uses agentType as filename)
function getNewAgentFilePath(agent: { source: SettingSource; agentType: string }): string

// Path for an EXISTING agent (uses actual filename if different from agentType)
function getActualAgentFilePath(agent: AgentDefinition): string

// Relative path for new agent display
function getNewRelativeAgentFilePath(agent: { source: SettingSource | 'built-in'; agentType: string }): string

// Relative path for existing agent display (handles built-in/plugin/flagSettings)
function getActualRelativeAgentFilePath(agent: AgentDefinition): string

// Ensures directory exists then writes agent file
async function saveAgentToFile(
  source: SettingSource | 'built-in',
  agentType: string,
  whenToUse: string,
  tools: string[] | undefined,
  systemPrompt: string,
  checkExists?: boolean,     // default true — throws EEXIST if file already exists
  color?: string,
  model?: string,
  memory?: AgentMemoryScope,
  effort?: EffortValue,
): Promise<void>

// Overwrites an existing agent's file
async function updateAgentFile(
  agent: AgentDefinition,
  newWhenToUse: string,
  newTools: string[] | undefined,
  newSystemPrompt: string,
  newColor?: string,
  newModel?: string,
  newMemory?: AgentMemoryScope,
  newEffort?: EffortValue,
): Promise<void>

// Removes the agent's file (ignores ENOENT)
async function deleteAgentFromFile(agent: AgentDefinition): Promise<void>
```

**Key Behaviors:**
- All writes use `writeFileAndFlush` which calls `handle.datasync()` after writing for durability.
- `formatAgentAsMarkdown` escapes backslashes, double quotes, and newlines in `whenToUse` for YAML double-quoted strings.
- Tools field omitted entirely when `undefined` or `['*']` (means all tools allowed).

---

### 1.10 generateAgent.ts

**File:** `agents/generateAgent.ts`

**Purpose:** Uses an LLM call (`queryModelWithoutStreaming`) to auto-generate an agent configuration from a natural language user prompt. Returns `identifier`, `whenToUse`, and `systemPrompt` as structured JSON.

**Exports:**
```typescript
type GeneratedAgent = {
  identifier: string
  whenToUse: string
  systemPrompt: string
}

async function generateAgent(
  userPrompt: string,
  model: ModelName,
  existingIdentifiers: string[],  // Blocked identifiers included in prompt
  abortSignal: AbortSignal,
): Promise<GeneratedAgent>
```

**Key Behaviors:**
- Uses a detailed system prompt (`AGENT_CREATION_SYSTEM_PROMPT`) instructing Claude on how to design agent personas, write system prompts, create identifiers (lowercase, 2-4 words, hyphens only, no "helper"/"assistant").
- When `isAutoMemoryEnabled()`, appends `AGENT_MEMORY_INSTRUCTIONS` to the system prompt.
- Prepends user context via `prependUserContext()`.
- Parses JSON from response — falls back to regex extraction if direct parse fails.
- Fires `tengu_agent_definition_generated` analytics event.
- Throws on invalid/missing fields.

---

### 1.11 types.ts

**File:** `agents/types.ts`

**Purpose:** Shared type definitions for the agents UI state machine.

**Exports:**
```typescript
const AGENT_PATHS = {
  FOLDER_NAME: '.claude',
  AGENTS_DIR: 'agents',
} as const

type ModeState =
  | { mode: 'main-menu' }
  | { mode: 'list-agents'; source: SettingSource | 'all' | 'built-in' }
  | { mode: 'agent-menu'; agent: AgentDefinition; previousMode: ModeState }
  | { mode: 'view-agent'; agent: AgentDefinition; previousMode: ModeState }
  | { mode: 'create-agent' }
  | { mode: 'edit-agent'; agent: AgentDefinition; previousMode: ModeState }
  | { mode: 'delete-confirm'; agent: AgentDefinition; previousMode: ModeState }

type AgentValidationResult = {
  isValid: boolean
  warnings: string[]
  errors: string[]
}
```

---

### 1.12 utils.ts

**File:** `agents/utils.ts`

**Exports:**
```typescript
function getAgentSourceDisplayName(
  source: SettingSource | 'all' | 'built-in' | 'plugin'
): string
// Returns: 'Agents' | 'Built-in agents' | 'Plugin agents' | capitalize(getSettingSourceName(source))
```

---

### 1.13 validateAgent.ts

**File:** `agents/validateAgent.ts`

**Exports:**
```typescript
type AgentValidationResult = {
  isValid: boolean
  errors: string[]
  warnings: string[]
}

function validateAgentType(agentType: string): string | null
// Returns error message or null if valid.
// Rules: required, /^[a-zA-Z0-9][a-zA-Z0-9-]*[a-zA-Z0-9]$/, length 3-50 chars.

function validateAgent(
  agent: Omit<CustomAgentDefinition, 'location'>,
  availableTools: Tools,
  existingAgents: AgentDefinition[],
): AgentValidationResult
// Validates: agentType, whenToUse (min 10 chars / max 5000), tools array,
// system prompt (min 20 chars / max 10000 chars warning threshold).
// Checks for duplicate agentType across sources.
// Uses resolveAgentTools to check for invalid tool names.
```

---

### 1.14 new-agent-creation/CreateAgentWizard

**File:** `agents/new-agent-creation/CreateAgentWizard.tsx`

**Purpose:** Assembles the multi-step new-agent creation wizard by composing step components inside `<WizardProvider>`. Conditionally includes `MemoryStep` based on feature flag.

**Props Interface:**
```typescript
type Props = {
  tools: Tools;
  existingAgents: AgentDefinition[];
  onComplete: (message: string) => void;
  onCancel: () => void;
}
```

**Wizard Step Order (0-indexed):**
0. `LocationStep` — project vs personal scope
1. `MethodStep` — generate with Claude vs manual
2. `GenerateStep` — natural language prompt → LLM generation (skipped for manual)
3. `TypeStep` — agent identifier / name
4. `PromptStep` — system prompt text
5. `DescriptionStep` — whenToUse description
6. `ToolsStep` — tool selection
7. `ModelStep` — model selection
8. `ColorStep` — color selection
9. `MemoryStep` — memory scope (conditional on `isAutoMemoryEnabled()`)
10. `ConfirmStepWrapper` — review and save

**Key Behaviors:**
- WizardProvider title: "Create new agent", `showStepCounter: false`.
- `onComplete` of WizardProvider is a no-op (actual completion handled by `ConfirmStepWrapper`).
- Passes `onCancel` through to WizardProvider.

---

### 1.15 Wizard Steps

#### LocationStep
Offers two options: "Project (.claude/agents/)" (`projectSettings`) or "Personal (~/.claude/agents/)" (`userSettings`). Updates `wizardData.location` and calls `goNext()`.

#### MethodStep
Offers "Generate with Claude (recommended)" or "Manual configuration". For generate: sets `wizardData.method = 'generate'`, `wasGenerated: true`, `goNext()`. For manual: sets `method: 'manual'`, `wasGenerated: false`, skips to step 3 via `goToStep(3)`.

#### GenerateStep
- Text input for natural language description of the agent.
- On submit: calls `generateAgent(prompt, model, existingIdentifiers, abortSignal)`.
- Shows animated `<Spinner>` during generation.
- On success: populates `wizardData.agentType`, `systemPrompt`, `whenToUse`, `generatedAgent`.
- Navigates directly to step 6 (ToolsStep) after generation (skips manual name/prompt/description steps).
- Esc during generation cancels via abort signal.
- Supports external editor (`chat:externalEditor` keybinding).

#### TypeStep
Props: `{ existingAgents: AgentDefinition[] }`. Text input for agent identifier. Validates via `validateAgentType()`. Updates `wizardData.agentType`.

#### PromptStep
Large text input for system prompt. Min 20 chars enforced. Supports external editor. Updates `wizardData.systemPrompt`.

#### DescriptionStep
Text input for `whenToUse`. Required, min 1 char. Supports external editor. Updates `wizardData.whenToUse`.

#### ToolsStep
Props: `{ tools: Tools }`. Wraps `<ToolSelector>` in `WizardDialogLayout`. Updates `wizardData.selectedTools`.

#### ModelStep
Wraps `<ModelSelector>` in `WizardDialogLayout`. Updates `wizardData.selectedModel`.

#### ColorStep
Wraps `<ColorPicker>` in `WizardDialogLayout`. On confirm: builds `wizardData.finalAgent` object with all accumulated data, updates `wizardData.selectedColor`.

#### MemoryStep
Conditional step (only shown when `isAutoMemoryEnabled()`). Offers memory scope options ordered by recommended scope (project-first for project agents, user-first for personal agents). Includes "None" option. Updates `wizardData.selectedMemory`.

#### ConfirmStep / ConfirmStepWrapper
Props: `{ tools: Tools; existingAgents: AgentDefinition[]; onComplete: (message: string) => void }`. Shows a summary of all selected options, validates via `validateAgent()`, calls `saveAgentToFile()`, then reloads agents in app state and calls `onComplete(message)`.

---

## 2. permissions/

The permissions subsystem renders interactive dialogs asking users to approve, reject, or configure rules for tool uses. Each tool has its own permission request component.

### 2.1 Core Types & PermissionRequest

**File:** `permissions/PermissionRequest.tsx`

**Purpose:** Top-level dispatcher that picks the correct permission request component for each tool type and renders it. Also handles notifications and idle detection.

**Key Types:**
```typescript
type PermissionRequestProps<Input extends AnyObject = AnyObject> = {
  toolUseConfirm: ToolUseConfirm<Input>;
  toolUseContext: ToolUseContext;
  onDone(): void;
  onReject(): void;
  verbose: boolean;
  workerBadge: WorkerBadgeProps | undefined;
  setStickyFooter?: (jsx: React.ReactNode | null) => void;
  // Registers JSX in a sticky footer below scrollable area (fullscreen only).
}

type ToolUseConfirm<Input extends AnyObject = AnyObject> = {
  assistantMessage: AssistantMessage;
  tool: Tool<Input>;
  description: string;
  input: z.infer<Input>;
  toolUseContext: ToolUseContext;
  toolUseID: string;
  permissionResult: PermissionDecision;
  permissionPromptStartTimeMs: number;
  classifierCheckInProgress?: boolean;
  classifierAutoApproved?: boolean;
  classifierMatchedRule?: string;
  workerBadge?: WorkerBadgeProps;
  onUserInteraction(): void;
  onAbort(): void;
  onDismissCheckmark?(): void;
  onAllow(updatedInput, permissionUpdates: PermissionUpdate[], feedback?, contentBlocks?): void;
  onReject(feedback?, contentBlocks?): void;
  recheckPermission(): Promise<void>;
}
```

**Tool→Component Mapping:**
| Tool | Component |
|------|-----------|
| FileEditTool | FileEditPermissionRequest |
| FileWriteTool | FileWritePermissionRequest |
| BashTool | BashPermissionRequest |
| PowerShellTool | PowerShellPermissionRequest |
| WebFetchTool | WebFetchPermissionRequest |
| NotebookEditTool | NotebookEditPermissionRequest |
| ExitPlanModeV2Tool | ExitPlanModePermissionRequest |
| EnterPlanModeTool | EnterPlanModePermissionRequest |
| SkillTool | SkillPermissionRequest |
| AskUserQuestionTool | AskUserQuestionPermissionRequest |
| GlobTool / GrepTool / FileReadTool | FilesystemPermissionRequest |
| ReviewArtifactTool (feature flag) | ReviewArtifactPermissionRequest |
| WorkflowTool (feature flag) | WorkflowPermissionRequest |
| MonitorTool (feature flag) | MonitorPermissionRequest |
| default | FallbackPermissionRequest |

**Exports:** `PermissionRequest`, `PermissionRequestProps`, `ToolUseConfirm`

---

### 2.2 PermissionDialog

**File:** `permissions/PermissionDialog.tsx`

**Purpose:** The shared visual container for all tool permission requests. Renders a title bar (with optional worker badge), subtitle, and children, wrapped in a styled border box.

**Props Interface:**
```typescript
type Props = {
  title: string;
  subtitle?: React.ReactNode;
  color?: keyof Theme;          // default: 'permission'
  titleColor?: keyof Theme;
  innerPaddingX?: number;       // default: 1
  workerBadge?: WorkerBadgeProps;
  titleRight?: React.ReactNode; // Right-aligned content in title row
  children: React.ReactNode;
}
```

**Key Behaviors:**
- Renders `<PermissionRequestTitle>` with the title, subtitle, optional color override, and workerBadge.
- Title row uses `justifyContent="space-between"` to place `titleRight` on the right.
- Children rendered in inner `Box` with `paddingX={innerPaddingX}`.

**Exports:** `PermissionDialog`

---

### 2.3 BashPermissionRequest

**File:** `permissions/BashPermissionRequest/BashPermissionRequest.tsx`

**Purpose:** Permission dialog for `BashTool`. Handles classifier-based auto-approval animation, sed edit detection (redirects to `SedEditPermissionRequest`), sandbox detection, and a rich set of options.

**Key Behaviors:**
- `ClassifierCheckingSubtitle`: separate sub-component that renders an animated shimmer "Attempting to auto-approve…" text at 20fps, isolated to prevent full-dialog re-renders.
- Checks `classifierCheckInProgress` prop — shows shimmer subtitle while classifier runs.
- If command matches a `sed` edit pattern, delegates to `SedEditPermissionRequest`.
- If `shouldUseSandbox()`, shows sandbox-specific options.
- Options computed by `bashToolUseOptions()`.
- Logs permission decision via `usePermissionRequestLogging()`.
- Supports destructive command warning display.

**Exports:** `BashPermissionRequest`

---

### 2.4 FilePermissionDialog

**File:** `permissions/FilePermissionDialog/FilePermissionDialog.tsx`

**Purpose:** Generic reusable dialog for file-operation permissions (used by FileEdit, FileWrite, NotebookEdit permission requests). Handles path display, symlink detection, IDE diff integration, and option rendering.

**Props Interface:**
```typescript
type FilePermissionDialogProps<T extends ToolInput = ToolInput> = {
  toolUseConfirm: ToolUseConfirm;
  toolUseContext: ToolUseContext;
  onDone: () => void;
  onReject: () => void;
  title: string;
  subtitle?: React.ReactNode;
  question?: string | React.ReactNode;   // default: 'Do you want to proceed?'
  content?: React.ReactNode;
  completionType?: CompletionType;       // default: 'tool_use_single'
  languageName?: string;                 // Overrides path-derived language name
  path: string | null;
  parseInput: (input: unknown) => T;
  operationType?: FileOperationType;     // default: 'write'
  ideDiffSupport?: IDEDiffSupport<T>;
  workerBadge: WorkerBadgeProps | undefined;
}
```

**Key Behaviors:**
- Language name derived async from `getLanguageName(path)` if not overridden.
- Checks for symlink target when `operationType !== 'read'`.
- Shows `<ShowInIDEPrompt>` when IDE diff is available.
- Logs via `usePermissionRequestLogging()`.

**Exports:** `FilePermissionDialog`, `FilePermissionDialogProps`

---

### 2.5 WorkerBadge

**File:** `permissions/WorkerBadge.tsx`

**Purpose:** Colored badge showing which swarm worker is requesting a permission.

**Props Interface:**
```typescript
type WorkerBadgeProps = {
  name: string;    // Worker name (shown as @name)
  color: string;   // Raw color string (converted to Ink color via toInkColor)
}
```

**Rendering:** `● @{name}` with the bullet in the worker's color.

**Exports:** `WorkerBadge`, `WorkerBadgeProps`

---

### 2.6 Other Permission Request Components

Each follows the `PermissionRequestProps` interface and renders inside `<PermissionDialog>` or `<FilePermissionDialog>`.

#### FileEditPermissionRequest
Renders a diff of the proposed file edit. Uses `FilePermissionDialog` with diff content component. `operationType: 'write'`.

#### FileWritePermissionRequest
Renders the full proposed file content. Uses `FilePermissionDialog`. `operationType: 'write'`.

#### FilesystemPermissionRequest
For GlobTool / GrepTool / FileReadTool. Shows a read-only access dialog. `operationType: 'read'`.

#### NotebookEditPermissionRequest
For Jupyter notebook edits. Renders a notebook cell diff. Uses `FilePermissionDialog` with `languageName` derived from cell type.

#### WebFetchPermissionRequest
Shows the URL being fetched and fetch options.

#### PowerShellPermissionRequest
Similar to BashPermissionRequest but for PowerShell commands. Uses `powershellToolUseOptions()`.

#### SedEditPermissionRequest
Rendered by BashPermissionRequest when sed-edit pattern is detected. Shows the file diff that the sed command would produce.

#### EnterPlanModePermissionRequest
Simple confirmation to enter plan mode.

#### ExitPlanModePermissionRequest
Full plan review with sticky footer for response options. Uses `setStickyFooter` for keeping response visible while scrolling.

#### SkillPermissionRequest
For executing a skill/command.

#### AskUserQuestionPermissionRequest
Renders a multi-question form with navigation. Subcomponents:
- `PreviewBox` — shows rendered preview of a question
- `PreviewQuestionView` — full question preview
- `QuestionNavigationBar` — prev/next navigation
- `QuestionView` — single question input
- `SubmitQuestionsView` — confirmation before submitting
- `use-multiple-choice-state.ts` — hook managing checkbox/radio state

#### ComputerUseApproval
For computer-use actions.

#### FallbackPermissionRequest
Generic fallback for unrecognized tool types. Shows tool name and description.

#### SandboxPermissionRequest
Shown when a command is being run in sandbox mode.

---

### 2.7 PermissionDecisionDebugInfo

**File:** `permissions/PermissionDecisionDebugInfo.tsx`

**Purpose:** Renders debug information about the permission decision (decision reason, rule details, classifier results) when verbose mode is enabled.

---

### 2.8 PermissionExplanation

**File:** `permissions/PermissionExplanation.tsx`

**Purpose:** Renders an explanation of why a permission rule is being requested. `usePermissionExplainerUI` hook manages the expand/collapse state of the explainer section.

**Exports:** `PermissionExplainerContent`, `usePermissionExplainerUI`

---

### 2.9 PermissionPrompt

**File:** `permissions/PermissionPrompt.tsx`

**Purpose:** Wraps permission request UI with the fullscreen layout management and sticky footer support for plan mode responses.

---

### 2.10 PermissionRequestTitle

**File:** `permissions/PermissionRequestTitle.tsx`

**Purpose:** Renders the colored title bar for permission dialogs. Shows title text in permission color with optional worker badge below.

**Props Interface:**
```typescript
type Props = {
  title: string;
  subtitle?: React.ReactNode;
  color?: keyof Theme;
  workerBadge?: WorkerBadgeProps;
}
```

---

### 2.11 PermissionRuleExplanation

**File:** `permissions/PermissionRuleExplanation.tsx`

**Purpose:** Shows the permission rule suggestions (e.g., "Allow bash: git *", "Deny bash: rm -rf") that would be created if the user clicks "Always allow" or "Always deny".

---

### 2.12 WorkerPendingPermission

**File:** `permissions/WorkerPendingPermission.tsx`

**Purpose:** Rendered in the swarm coordinator view to show a pending permission from a worker agent.

---

### 2.13 hooks.ts

**File:** `permissions/hooks.ts`

**Exports:**
```typescript
type UnaryEvent = {
  completion_type: CompletionType;
  language_name: string | Promise<string>;
}

// Logs permission request start/end/result analytics. Called once per dialog.
function usePermissionRequestLogging(
  toolUseConfirm: ToolUseConfirm,
  unaryEvent: UnaryEvent,
): void
```

**Key Behaviors:**
- Fires `tengu_permission_request_start` on mount.
- Fires `tengu_permission_request_end` with result info on unmount.
- Converts `permissionResult` to a structured log string.

---

### 2.14 shellPermissionHelpers.tsx

**File:** `permissions/shellPermissionHelpers.tsx`

**Purpose:** Shared JSX helpers and option builders used by `BashPermissionRequest` and `PowerShellPermissionRequest` (e.g., building the approve/deny/always-allow option list).

---

### 2.15 useShellPermissionFeedback.ts

**File:** `permissions/useShellPermissionFeedback.ts`

**Purpose:** Hook managing the feedback text input that appears when user selects "No (feedback)" in shell permission dialogs.

---

### 2.16 utils.ts

**File:** `permissions/utils.ts`

**Exports:**
```typescript
function logUnaryPermissionEvent(
  toolUseConfirm: ToolUseConfirm,
  unaryEvent: UnaryEvent,
  result: string,
): void
```

---

### 2.17 rules/ subdirectory

Permission rule management UI for the `/permissions` command screen.

| File | Purpose |
|------|---------|
| `AddPermissionRules.tsx` | Form to add new allow/deny rules |
| `AddWorkspaceDirectory.tsx` | Form to add trusted workspace directories |
| `PermissionRuleDescription.tsx` | Renders a human-readable description of a rule |
| `PermissionRuleInput.tsx` | Text input + validation for rule patterns |
| `PermissionRuleList.tsx` | Shows existing rules with delete option |
| `RecentDenialsTab.tsx` | Tab showing recently denied tool uses (can convert to rules) |
| `RemoveWorkspaceDirectory.tsx` | Confirmation dialog for removing workspace directory |
| `WorkspaceTab.tsx` | Tab showing trusted workspace directories |

---

## 3. design-system/

Reusable primitive components that form the visual foundation of all terminal UI.

### 3.1 Byline

**File:** `design-system/Byline.tsx`

**Purpose:** Joins children with a middot separator (` · `) for inline metadata display. Automatically filters null/undefined/false children.

**Props Interface:**
```typescript
type Props = {
  children: React.ReactNode;
}
```

**Key Behaviors:**
- Uses `Children.toArray()` which filters falsy nodes.
- Returns `null` if no valid children.
- Renders separators only between adjacent valid elements.

**Exports:** `Byline`

---

### 3.2 Dialog

**File:** `design-system/Dialog.tsx`

**Purpose:** Confirm/cancel dialog container. Registers `confirm:no` and Ctrl+C/D keybindings. Shows title, optional subtitle, children, and a keyboard hint footer.

**Props Interface:**
```typescript
type DialogProps = {
  title: React.ReactNode;
  subtitle?: React.ReactNode;
  children: React.ReactNode;
  onCancel: () => void;
  color?: keyof Theme;        // default: 'permission'
  hideInputGuide?: boolean;
  hideBorder?: boolean;
  inputGuide?: (exitState: ExitState) => React.ReactNode;  // Custom footer
  isCancelActive?: boolean;   // default: true — controls keybinding activation
}
```

**Key Behaviors:**
- `isCancelActive=false`: disables the `confirm:no` and exit keybindings (useful when embedded TextInput needs Esc).
- Default input guide: "Enter to confirm · Esc to cancel" (or "Press X again to exit" when exit pending).
- Wraps content in `<Pane>` unless `hideBorder`.
- Title rendered in bold with `color`.

**Exports:** `Dialog`

---

### 3.3 Divider

**File:** `design-system/Divider.tsx`

**Props Interface:**
```typescript
type DividerProps = {
  width?: number;        // Defaults to terminal width
  color?: keyof Theme;   // If absent, uses dimColor
  char?: string;         // default: '─' (U+2500)
  padding?: number;      // Subtracted from width, default: 0
  title?: string;        // Centered title (may contain ANSI codes)
}
```

**Key Behaviors:**
- Without title: `char.repeat(effectiveWidth)` in a `<Text>`.
- With title: left fill + space + title + space + right fill. Left/right fill split evenly (left gets floor half).
- Terminal width queried via `useTerminalSize()`.

**Exports:** `Divider`

---

### 3.4 FuzzyPicker

**File:** `design-system/FuzzyPicker.tsx`

**Purpose:** Full-featured fuzzy search picker with optional preview panel. Supports up/down/down-to-top list directions, Tab/Shift+Tab secondary actions, preview on right or bottom, match count label.

**Props Interface:**
```typescript
type PickerAction<T> = {
  action: string;          // Label for byline hint
  handler: (item: T) => void;
}

type Props<T> = {
  title: string;
  placeholder?: string;               // default: 'Type to search…'
  initialQuery?: string;
  items: readonly T[];
  getKey: (item: T) => string;
  renderItem: (item: T, isFocused: boolean) => React.ReactNode;
  renderPreview?: (item: T) => React.ReactNode;
  previewPosition?: 'bottom' | 'right';  // default: 'bottom'
  visibleCount?: number;              // default: 8
  direction?: 'down' | 'up';         // default: 'down'
  onQueryChange: (query: string) => void;
  onSelect: (item: T) => void;
  onTab?: PickerAction<T>;
  onShiftTab?: PickerAction<T>;
  onFocus?: (item: T | undefined) => void;
  onCancel: () => void;
  emptyMessage?: string | ((query: string) => string);
  matchLabel?: string;
  selectAction?: string;
  extraHints?: React.ReactNode;
}
```

**Key Behaviors:**
- Constants: `DEFAULT_VISIBLE=8`, `CHROME_ROWS=10`, `MIN_VISIBLE=2`.
- Auto-adjusts visible count based on terminal height.
- Fires `onFocus` when focused item changes.
- `direction='up'`: items[0] at bottom (atuin-style); arrows match screen direction.

**Exports:** `FuzzyPicker`

---

### 3.5 KeyboardShortcutHint

**File:** `design-system/KeyboardShortcutHint.tsx`

**Purpose:** Renders a keyboard shortcut hint like "ctrl+o to expand" or "(tab to toggle)".

**Props Interface:**
```typescript
type Props = {
  shortcut: string;    // e.g., "ctrl+o", "Enter", "↑↓"
  action: string;      // e.g., "expand", "select", "navigate"
  parens?: boolean;    // default: false — wraps in parentheses
  bold?: boolean;      // default: false — renders shortcut in bold
}
```

**Exports:** `KeyboardShortcutHint`

---

### 3.6 ListItem

**File:** `design-system/ListItem.tsx`

**Purpose:** Standard list item for selection UIs with pointer (❯), checkmark (✓), scroll hint arrows, focus/selection colors, and disabled state.

**Props Interface:**
```typescript
type ListItemProps = {
  isFocused: boolean;
  isSelected?: boolean;      // default: false — shows ✓
  children: ReactNode;
  description?: string;      // Shown below main content
  showScrollDown?: boolean;  // Shows ↓ instead of pointer (scroll hint)
  showScrollUp?: boolean;    // Shows ↑ instead of pointer (scroll hint)
  styled?: boolean;          // default: true — auto-colors children by state
  disabled?: boolean;        // default: false — dimmed, no indicators
  declareCursor?: boolean;   // default: true — declares terminal cursor position
}
```

**Key Behaviors:**
- When focused and not selected: pointer (❯) in suggestion color.
- When selected and not focused: checkmark (✓) in suggestion color.
- When disabled: no indicator, dimmed text.
- When `styled=false`: children rendered as-is for custom styling.

**Exports:** `ListItem`

---

### 3.7 LoadingState

**File:** `design-system/LoadingState.tsx`

**Purpose:** Spinner + message for async loading states.

**Props Interface:**
```typescript
type LoadingStateProps = {
  message: string;
  bold?: boolean;         // default: false
  dimColor?: boolean;     // default: false
  subtitle?: string;      // Optional secondary line below
}
```

**Exports:** `LoadingState`

---

### 3.8 Pane

**File:** `design-system/Pane.tsx`

**Purpose:** A terminal region bounded by a colored top divider line, used by all slash-command screens.

**Props Interface:**
```typescript
type PaneProps = {
  children: React.ReactNode;
  color?: keyof Theme;    // Theme color for top divider
}
```

**Key Behaviors:**
- When rendered inside a modal (`useIsInsideModal()`): skips the Divider (the modal frame serves as border), renders with `paddingX={1}` and `flexShrink={0}`.
- Normal rendering: `paddingTop={1}` + `<Divider color={color}>` + `<Box paddingX={2}>children</Box>`.

**Exports:** `Pane`

---

### 3.9 ProgressBar

**File:** `design-system/ProgressBar.tsx`

**Purpose:** Horizontal text-art progress bar using Unicode block characters.

**Props Interface:**
```typescript
type Props = {
  ratio: number;           // [0, 1]
  width: number;           // Number of character columns
  fillColor?: keyof Theme;
  emptyColor?: keyof Theme;
}
```

**Key Behaviors:**
- Uses 9 block characters: `[' ', '▏', '▎', '▍', '▌', '▋', '▊', '▉', '█']`.
- Clamps ratio to [0, 1].
- Whole filled cells: `Math.floor(ratio * width)`.
- Partial cell: `Math.floor(remainder * BLOCKS.length)`.
- `fillColor` applied as text color; `emptyColor` as background.

**Exports:** `ProgressBar`

---

### 3.10 Ratchet

**File:** `design-system/Ratchet.tsx`

**Purpose:** Prevents layout bounce/shrink by maintaining minimum height equal to the maximum seen height. Used for content that can grow but should not shrink when re-rendered.

**Props Interface:**
```typescript
type Props = {
  children: React.ReactNode;
  lock?: 'always' | 'offscreen';  // default: 'always'
}
```

**Key Behaviors:**
- `lock='always'`: always enforces minHeight (inner Box max seen height, capped at terminal rows).
- `lock='offscreen'`: only enforces minHeight when the element is not visible in terminal viewport.
- Uses `useTerminalViewport()` for visibility detection.
- `useLayoutEffect` measures inner content height after every render.

**Exports:** `Ratchet`

---

### 3.11 StatusIcon

**File:** `design-system/StatusIcon.tsx`

**Purpose:** Renders a colored status indicator icon.

**Props Interface:**
```typescript
type Status = 'success' | 'error' | 'warning' | 'info' | 'pending' | 'loading'
type Props = {
  status: Status;
  withSpace?: boolean;   // default: false — adds trailing space
}
```

**Status Config:**
| Status | Icon | Color |
|--------|------|-------|
| success | ✓ (figures.tick) | success (green) |
| error | ✗ (figures.cross) | error (red) |
| warning | ⚠ (figures.warning) | warning (yellow) |
| info | ℹ (figures.info) | suggestion (blue) |
| pending | ○ (figures.circle) | dimColor |
| loading | … | dimColor |

**Exports:** `StatusIcon`

---

### 3.12 Tabs

**File:** `design-system/Tabs.tsx`

**Purpose:** Tab container with keyboard navigation. Supports controlled and uncontrolled modes, fixed content height, optional banner, and content-initiated tab switching.

**Props Interface:**
```typescript
type TabsProps = {
  children: Array<React.ReactElement<TabProps>>;
  title?: string;
  color?: keyof Theme;
  defaultTab?: string;
  hidden?: boolean;
  useFullWidth?: boolean;
  selectedTab?: string;                // Controlled mode
  onTabChange?: (tabId: string) => void; // Controlled mode
  banner?: React.ReactNode;
  disableNavigation?: boolean;
  initialHeaderFocused?: boolean;      // default: true
  contentHeight?: number;              // Fixed height for all tabs
  navFromContent?: boolean;            // Allow Tab/←/→ from content
}

type TabsContextValue = {
  selectedTab: string | undefined;
  width: number | undefined;
  headerFocused: boolean;
  focusHeader: () => void;
  blurHeader: () => void;
  registerOptIn: () => () => void;
}
```

**Key Behaviors:**
- Tab component reads `TabsContext` to know if it is the selected tab.
- Left/Right arrows or Tab key switches tabs when header focused.
- `navFromContent=true` allows content area to trigger tab switches.

**Exports:** `Tabs`, `TabsContext`, `TabsContextValue`

---

### 3.13 ThemeProvider

**File:** `design-system/ThemeProvider.tsx`

**Purpose:** Provides theme state (dark/light/auto) to the component tree. Resolves `'auto'` by detecting system theme via OSC 11 terminal queries.

**Props Interface:**
```typescript
type Props = {
  children: React.ReactNode;
  initialState?: ThemeSetting;
  onThemeSave?: (setting: ThemeSetting) => void;
}

type ThemeContextValue = {
  themeSetting: ThemeSetting;              // Saved preference (may be 'auto')
  setThemeSetting: (s: ThemeSetting) => void;
  setPreviewTheme: (s: ThemeSetting) => void;
  savePreview: () => void;
  cancelPreview: () => void;
  currentTheme: ThemeName;                 // Resolved theme, never 'auto'
}
```

**Key Behaviors:**
- `previewTheme` takes priority over `themeSetting` during theme picker interactions.
- Seeds system theme from `$COLORFGBG` env var; OSC 11 watcher corrects it.
- Default theme (outside provider, for tests): `'dark'`.

**Exports:** `ThemeProvider`, `ThemeContext`, `useTheme` (hook)

---

### 3.14 ThemedBox

**File:** `design-system/ThemedBox.tsx`

**Purpose:** Theme-aware Box component that resolves theme key colors (`keyof Theme`) in all border/background color props to raw terminal colors before passing to the underlying Ink `Box`.

**Props Type:** `BaseStylesWithoutColors & ThemedColorProps & EventHandlerProps`

```typescript
type ThemedColorProps = {
  borderColor?: keyof Theme | Color;
  borderTopColor?: keyof Theme | Color;
  borderBottomColor?: keyof Theme | Color;
  borderLeftColor?: keyof Theme | Color;
  borderRightColor?: keyof Theme | Color;
  backgroundColor?: keyof Theme | Color;
}
```

**Key Behaviors:**
- `resolveColor()`: passes through raw colors (`#`, `rgb(`, `ansi256(`, `ansi:`), looks up theme keys.

**Exports:** `ThemedBox` (default export), `Props`

---

### 3.15 ThemedText

**File:** `design-system/ThemedText.tsx`

**Purpose:** Theme-aware Text component that resolves `keyof Theme` color values and supports `TextHoverColorContext` for cascade coloring.

**Props Interface:**
```typescript
type Props = {
  color?: keyof Theme | Color;
  backgroundColor?: keyof Theme;
  dimColor?: boolean;          // Uses theme's inactive color (compatible with bold)
  bold?: boolean;
  italic?: boolean;
  underline?: boolean;
  strikethrough?: boolean;
  inverse?: boolean;
  wrap?: Styles['textWrap'];
  children?: ReactNode;
}
```

**Exports:** `ThemedText`, `Props`, `TextHoverColorContext`

---

### 3.16 color.ts

**File:** `design-system/color.ts`

**Purpose:** Curried theme-aware colorization function.

**Exports:**
```typescript
function color(
  c: keyof Theme | Color | undefined,
  theme: ThemeName,
  type: ColorType = 'foreground',
): (text: string) => string
```

**Key Behaviors:**
- Raw color values bypass theme lookup.
- Theme key values are looked up in `getTheme(theme)` and passed to `colorize()`.

---

## 4. wizard/

A generic multi-step wizard framework used by agent creation and potentially other flows.

### 4.1 WizardProvider

**File:** `wizard/WizardProvider.tsx`

**Purpose:** Context provider managing wizard state: current step, data accumulation, navigation history, and completion.

**Props Interface:**
```typescript
type WizardProviderProps<T> = {
  steps: WizardStepComponent<T>[];
  initialData?: Partial<T>;
  onComplete: (data: T) => void;
  onCancel: () => void;
  children?: React.ReactNode;
  title?: string;
  showStepCounter?: boolean;    // default: true
}
```

**WizardContextValue:**
```typescript
type WizardContextValue<T> = {
  currentStepIndex: number;
  totalSteps: number;
  wizardData: Partial<T>;
  updateWizardData: (partial: Partial<T>) => void;
  goNext: () => void;
  goBack: () => void;
  goToStep: (index: number) => void;
  cancel: () => void;
  title: string | undefined;
  showStepCounter: boolean;
}
```

**Key Behaviors:**
- Maintains `navigationHistory` stack for `goBack()`.
- `goNext()` at last step sets `isCompleted=true`, triggering `onComplete(wizardData)` in an effect.
- Calls `useExitOnCtrlCDWithKeybindings()` to register Ctrl+C/D at wizard level.

**Exports:** `WizardProvider`, `WizardContext`

---

### 4.2 WizardDialogLayout

**File:** `wizard/WizardDialogLayout.tsx`

**Purpose:** Standard layout for wizard steps — wraps content in a `<Dialog>` with step counter in title, navigation footer.

**Props Interface:**
```typescript
type Props = {
  title?: string;              // Overrides provider title
  color?: keyof Theme;         // default: 'suggestion'
  children: ReactNode;
  subtitle?: string;
  footerText?: ReactNode;
}
```

**Key Behaviors:**
- Title format: `"${title} (${currentStep + 1}/${totalSteps})"` when `showStepCounter` is true.
- `isCancelActive={false}` on the inner Dialog (wizard handles its own cancel).
- Renders `<WizardNavigationFooter instructions={footerText}>` below Dialog.

**Exports:** `WizardDialogLayout`

---

### 4.3 WizardNavigationFooter

**File:** `wizard/WizardNavigationFooter.tsx`

**Purpose:** Renders keyboard hints at the bottom of wizard dialogs.

**Props Interface:**
```typescript
type Props = {
  instructions?: ReactNode;
}
```

---

### 4.4 useWizard

**File:** `wizard/useWizard.ts`

**Purpose:** Hook to access `WizardContext` from within a step component. Throws if used outside a `WizardProvider`.

**Exports:**
```typescript
function useWizard<T extends Record<string, unknown> = Record<string, unknown>>(): WizardContextValue<T>
```

---

### 4.5 index.ts

Re-exports `WizardProvider`, `useWizard`, and wizard-related types.

---

## 5. mcp/

Model Context Protocol server management UI.

### Files Overview

| File | Purpose |
|------|---------|
| `CapabilitiesSection.tsx` | Renders server capabilities list (tools, resources, prompts) |
| `ElicitationDialog.tsx` | Dialog for MCP elicitation (server requesting structured user input) |
| `MCPAgentServerMenu.tsx` | Menu for managing MCP agent-type servers |
| `MCPListPanel.tsx` | Panel showing all connected MCP servers with status |
| `MCPReconnect.tsx` | UI for reconnecting to a disconnected server |
| `MCPRemoteServerMenu.tsx` | Menu for remote MCP server configuration |
| `MCPSettings.tsx` | Top-level MCP settings screen |
| `MCPStdioServerMenu.tsx` | Menu for stdio-type MCP servers |
| `MCPToolDetailView.tsx` | Detail view of a single MCP tool |
| `MCPToolListView.tsx` | List of tools provided by an MCP server |
| `McpParsingWarnings.tsx` | Shows YAML/JSON parsing warnings from MCP config |
| `index.ts` | Re-exports |
| `utils/reconnectHelpers.tsx` | Helper functions for server reconnect logic |

**Key Types (from MCPSettings):**
```typescript
// Server status display combines name, transport type, connection state, tool count
```

---

## 6. memory/

Agent memory file management UI.

### Files

| File | Purpose |
|------|---------|
| `MemoryFileSelector.tsx` | File picker for selecting which memory file to view/edit |
| `MemoryUpdateNotification.tsx` | Toast-style notification shown when agent updates its memory |

**MemoryFileSelector Props:**
```typescript
type Props = {
  onSelect: (filePath: string) => void;
  onCancel: () => void;
}
```

---

## 7. tasks/

Background task and remote session monitoring UI.

### Files Overview

| File | Purpose |
|------|---------|
| `AsyncAgentDetailDialog.tsx` | Detail dialog for an async/queued agent task |
| `BackgroundTask.tsx` | Single background task row in the task list |
| `BackgroundTaskStatus.tsx` | Status badge for a background task |
| `BackgroundTasksDialog.tsx` | Full dialog listing all background tasks |
| `DreamDetailDialog.tsx` | Detail for a "dream" (autoDream background consolidation) task |
| `InProcessTeammateDetailDialog.tsx` | Detail for an in-process swarm teammate |
| `RemoteSessionDetailDialog.tsx` | Detail for a remote Claude Code session |
| `RemoteSessionProgress.tsx` | Progress display for remote session activity |
| `ShellDetailDialog.tsx` | Detail for a shell background task |
| `ShellProgress.tsx` | Progress line for shell commands |
| `renderToolActivity.tsx` | Renders current tool activity for a running task |
| `taskStatusUtils.tsx` | Utility functions for task status display |

---

## 8. teams/

Swarm/multi-agent team status UI.

### Files

| File | Purpose |
|------|---------|
| `TeamStatus.tsx` | Shows the status of all active team members (swarm workers) |
| `TeamsDialog.tsx` | Dialog for managing team composition and viewing worker details |

---

## 9. diff/

File diff viewing UI.

### Files

| File | Purpose |
|------|---------|
| `DiffDetailView.tsx` | Full-screen diff detail with scroll support |
| `DiffDialog.tsx` | Dialog wrapping `DiffDetailView` |
| `DiffFileList.tsx` | List of changed files with summary stats |

---

## 10. grove/

Grove (shared project workspace) integration.

### Files

| File | Purpose |
|------|---------|
| `Grove.tsx` | Main Grove integration UI component |

---

## 11. hooks/ (components/hooks/)

These are **component-level** hook subdirectory hooks, not the top-level `src/hooks/`. They are part of the hooks command settings UI.

### Files

| File | Purpose |
|------|---------|
| `HooksConfigMenu.tsx` | Configuration menu for Claude hooks (pre/post tool hooks) |
| `PromptDialog.tsx` | Dialog for entering a hook prompt/command |
| `SelectEventMode.tsx` | Select hook event type (PreToolUse, PostToolUse, etc.) |
| `SelectHookMode.tsx` | Select hook execution mode (allow, block, prompt) |
| `SelectMatcherMode.tsx` | Select how to match tools (all, specific, pattern) |
| `ViewHookMode.tsx` | Read-only view of an existing hook configuration |

---

## 12. HelpV2/

Second-generation help UI, shown by `/help`.

### Files

| File | Purpose |
|------|---------|
| `Commands.tsx` | Renders the commands reference tab |
| `General.tsx` | Renders the general help/tips tab |
| `HelpV2.tsx` | Top-level help screen with Tabs (General, Commands) |

---

## 13. TrustDialog/

Trust confirmation dialogs for files and workspace directories.

### Files

| File | Purpose |
|------|---------|
| `TrustDialog.tsx` | Dialog asking user to confirm trust for a directory or file |
| `utils.ts` | Trust decision helpers |

---

## 14. ManagedSettingsSecurityDialog/

Security warning for managed/policy settings.

### Files

| File | Purpose |
|------|---------|
| `ManagedSettingsSecurityDialog.tsx` | Warning dialog when policy settings override user preferences |
| `utils.ts` | Utilities for detecting managed setting conflicts |

---

## 15. ClaudeCodeHint/

Plugin/command hint menu.

### Files

| File | Purpose |
|------|---------|
| `PluginHintMenu.tsx` | Shows plugin-provided hints in a menu format |

---

## 16. HighlightedCode/

Syntax-highlighted code rendering.

### Files

| File | Purpose |
|------|---------|
| `Fallback.tsx` | Non-highlighted code block fallback |

The main `HighlightedCode.tsx` is in the parent `components/` directory (not in this subdirectory); this subdirectory provides only the fallback.

---

## 17. LogoV2/

Animated logo/welcome screen and feed system.

### Files

| File | Purpose |
|------|---------|
| `AnimatedAsterisk.tsx` | Spinning asterisk logo animation |
| `AnimatedClawd.tsx` | Animated "Clawd" mascot version |
| `ChannelsNotice.tsx` | Notice about available channels |
| `Clawd.tsx` | Static Clawd mascot |
| `CondensedLogo.tsx` | Compact logo for limited space contexts |
| `EmergencyTip.tsx` | Urgent tip overlay |
| `Feed.tsx` | Scrollable feed of announcements/tips |
| `FeedColumn.tsx` | Column layout for feed items |
| `GuestPassesUpsell.tsx` | Upsell for guest passes feature |
| `LogoV2.tsx` | Main logo component (animated asterisk + welcome) |
| `Opus1mMergeNotice.tsx` | Notice about Opus 1M model merge |
| `OverageCreditUpsell.tsx` | Upsell for overage credit purchase |
| `VoiceModeNotice.tsx` | Notice about voice mode availability |
| `WelcomeV2.tsx` | Full welcome screen with logo and feed |
| `feedConfigs.tsx` | Configuration data for feed content |

---

## 18. DesktopUpsell/

Desktop app upsell during startup.

### Files

| File | Purpose |
|------|---------|
| `DesktopUpsellStartup.tsx` | Full-screen upsell shown at startup for desktop app |

---

## 19. FeedbackSurvey/

In-app feedback and survey system.

### Files

| File | Purpose |
|------|---------|
| `FeedbackSurvey.tsx` | Main survey container |
| `FeedbackSurveyView.tsx` | Renders a single survey question |
| `TranscriptSharePrompt.tsx` | Asks user whether to share conversation transcript |
| `submitTranscriptShare.ts` | API call to submit transcript share |
| `useDebouncedDigitInput.ts` | Hook for numeric rating inputs (debounced) |
| `useFeedbackSurvey.tsx` | Main hook managing survey display lifecycle |
| `useMemorySurvey.tsx` | Hook for memory-specific survey prompts |
| `usePostCompactSurvey.tsx` | Hook for post-compact operation survey |
| `useSurveyState.tsx` | Core survey state management hook |

---

## 20. LspRecommendation/

LSP/IDE integration recommendation UI.

### Files

| File | Purpose |
|------|---------|
| `LspRecommendationMenu.tsx` | Menu suggesting LSP/IDE plugin installation |

---

## 21. Passes/

Guest passes system UI.

### Files

| File | Purpose |
|------|---------|
| `Passes.tsx` | Displays and manages guest passes for Claude |

---

## 22. Spinner/

Animated spinner components.

### Files

| File | Purpose |
|------|---------|
| `FlashingChar.tsx` | Character that flashes on/off |
| `GlimmerMessage.tsx` | Full shimmer-animated message text |
| `ShimmerChar.tsx` | Individual character with shimmer animation |
| `SpinnerAnimationRow.tsx` | Single row of spinner animation |
| `SpinnerGlyph.tsx` | The animated spinner glyph itself |
| `TeammateSpinnerLine.tsx` | Spinner line for teammate/worker status |
| `TeammateSpinnerTree.tsx` | Tree of spinner lines for all teammates |
| `index.ts` | Re-exports `Spinner` as the main export |
| `teammateSelectHint.ts` | Returns keyboard hint for teammate selection |
| `useShimmerAnimation.ts` | Hook generating shimmer animation indices |
| `useStalledAnimation.ts` | Hook detecting when animation has stalled |
| `utils.ts` | Spinner utility functions |

**SpinnerGlyph:** Animated terminal spinner using Unicode braille or other characters, driven by a clock tick interval.

**useShimmerAnimation:** Returns `[ref, glimmerIndex]`. The `glimmerIndex` represents the leading edge of a shimmer animation sweep across the text.

---

## 23. PromptInput/

The main prompt input area at the bottom of the REPL screen.

### Files

| File | Purpose |
|------|---------|
| `HistorySearchInput.tsx` | Ctrl+R history search input overlay |
| `IssueFlagBanner.tsx` | Banner for flagged issues |
| `Notifications.tsx` | In-prompt notification bubbles |
| `PromptInput.tsx` | Top-level prompt input orchestrator |
| `PromptInputFooter.tsx` | Footer area (model name, token count, etc.) |
| `PromptInputFooterLeftSide.tsx` | Left side of footer (model indicator) |
| `PromptInputFooterSuggestions.tsx` | File/command suggestions dropdown |
| `PromptInputHelpMenu.tsx` | Quick help menu (shown on ?) |
| `PromptInputModeIndicator.tsx` | Shows current input mode (normal, plan, etc.) |
| `PromptInputQueuedCommands.tsx` | Shows queued commands waiting to run |
| `PromptInputStashNotice.tsx` | Shows stashed input notification |
| `SandboxPromptFooterHint.tsx` | Hint for sandbox mode in footer |
| `ShimmeredInput.tsx` | Text input with shimmer loading state |
| `VoiceIndicator.tsx` | Voice mode activity indicator |
| `inputModes.ts` | Input mode type definitions and transitions |
| `inputPaste.ts` | Paste handling logic |
| `useMaybeTruncateInput.ts` | Hook for truncating long inputs in display |
| `usePromptInputPlaceholder.ts` | Hook generating placeholder text |
| `useShowFastIconHint.ts` | Hook controlling fast-mode icon hint visibility |
| `useSwarmBanner.ts` | Hook for swarm status banner in prompt |
| `utils.ts` | Input utility functions |

---

## 24. CustomSelect/

Accessible terminal select/dropdown components.

### Files

| File | Purpose |
|------|---------|
| `SelectMulti.tsx` | Multi-select dropdown |
| `index.ts` | Re-exports |
| `option-map.ts` | Option data structure and utilities |
| `select-input-option.tsx` | Option rendering in input mode |
| `select-option.tsx` | Single option rendering |
| `select.tsx` | Main single-select component |
| `use-multi-select-state.ts` | State hook for multi-select |
| `use-select-input.ts` | Input handling hook for select |
| `use-select-navigation.ts` | Keyboard navigation hook |
| `use-select-state.ts` | State hook for single select |

**Select Props (core):**
```typescript
type SelectProps<T extends string = string> = {
  options: Array<{ value: T; label: string; description?: string; disabled?: boolean }>;
  defaultValue?: T;
  onChange: (value: T) => void;
  onCancel?: () => void;
  isDisabled?: boolean;
}
```

**SelectMulti Props:**
```typescript
type SelectMultiProps<T extends string = string> = {
  options: Array<{ value: T; label: string }>;
  defaultValues?: T[];
  onChange: (values: T[]) => void;
  onCancel?: () => void;
}
```

---

## 25. Settings/

Settings slash-command screen (`/config`).

### Files

| File | Purpose |
|------|---------|
| `Config.tsx` | Configuration settings tab (API key, model, etc.) |
| `Settings.tsx` | Top-level settings screen with tabs |
| `Status.tsx` | Status tab showing connection and auth state |
| `Usage.tsx` | Usage statistics tab |

---

## 26. sandbox/

Sandbox configuration UI (shown by `/sandbox`).

### Files

| File | Purpose |
|------|---------|
| `SandboxConfigTab.tsx` | Sandbox enable/disable and mode selection |
| `SandboxDependenciesTab.tsx` | Shows dependencies inside sandbox |
| `SandboxDoctorSection.tsx` | Diagnostic section for sandbox health |
| `SandboxOverridesTab.tsx` | Per-project sandbox overrides |
| `SandboxSettings.tsx` | Top-level sandbox settings screen with tabs |

---

## 27. shell/

Shell output display components.

### Files

| File | Purpose |
|------|---------|
| `ExpandShellOutputContext.tsx` | Context provider for controlling shell output expansion state |
| `OutputLine.tsx` | Single line of shell output with ANSI support |
| `ShellProgressMessage.tsx` | In-progress shell command display with live output |
| `ShellTimeDisplay.tsx` | Shows elapsed/total time for a shell command |

---

## 28. skills/

Skills (slash commands from plugins) UI.

### Files

| File | Purpose |
|------|---------|
| `SkillsMenu.tsx` | Menu listing available skills with search |

---

## 29. ui/

General-purpose UI primitives not in design-system.

### Files

| File | Purpose |
|------|---------|
| `OrderedList.tsx` | Numbered list container |
| `OrderedListItem.tsx` | Single item in an ordered list |
| `TreeSelect.tsx` | Tree/hierarchical select component |

**TreeSelect** supports nested option trees where nodes can be expanded/collapsed. Used for hierarchical tool or directory selection.

---

## Cross-Cutting Patterns

### React Compiler Memoization
All components use `import { c as _c } from "react/compiler-runtime"` and the `$` cache array pattern for React Compiler auto-memoization. This is a build-time transformation — the source TypeScript uses standard React patterns.

### Theme Integration
- All color props accept `keyof Theme` (semantic tokens like `'permission'`, `'suggestion'`, `'success'`, `'error'`, `'warning'`, `'inactive'`) or raw CSS color strings.
- `ThemedBox` and `ThemedText` resolve theme tokens to raw colors at render time via `useTheme()`.

### Keybinding System
- Components use `useKeybinding(action, handler, { context, isActive })` from `../../keybindings/useKeybinding.js`.
- Contexts: `'Confirmation'`, `'Settings'`, `'Chat'`, `'Application'`.
- Standard actions: `'confirm:no'` (Esc/N), `'app:exit'` (Ctrl+C), `'app:interrupt'` (Ctrl+D).

### Permission Flow
1. Tool use triggers permission check → `PermissionResult` with behavior `'ask'` / `'allow'` / `'deny'` / `'passthrough'`.
2. `'ask'` behavior → `ToolUseConfirm` object created → `PermissionRequest` component rendered.
3. User selects option → `onAllow(updatedInput, permissionUpdates)` or `onReject(feedback)` called.
4. Permission updates stored in settings for future auto-approval.

### Wizard Pattern
1. `WizardProvider` holds step array and accumulated data.
2. Each step calls `useWizard()` to get navigation functions.
3. Steps call `updateWizardData(partial)` then `goNext()` / `goToStep(n)`.
4. Final step calls external `onComplete(message)` directly (not via wizard's `onComplete`).

### Agent File Format
```markdown
---
name: agent-identifier
description: "When to use this agent..."
tools: BashTool, FileEditTool
model: sonnet
effort: 3
color: blue
memory: project
---

System prompt content here...
```
