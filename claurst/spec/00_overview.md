# Claude Code — Master Architecture Overview

> **Repository:** `X:\Bigger-Projects\Claude-Code`
> **Primary Language:** TypeScript/TSX (~1,902 files, ~800K+ LOC)
> **Secondary Language:** Rust (~47 files, in-progress port)
> **Bundler:** Bun
> **UI Framework:** Custom Ink (React reconciler for terminal)
> **Runtime Target:** Node.js / Bun CLI

---

## 1. What Is Claude Code?

Claude Code is an AI-powered CLI tool and coding assistant. It is a full-featured interactive terminal application that:

- Embeds a Claude AI model as an agentic coding assistant
- Runs in the terminal using a custom React-based TUI (Terminal User Interface)
- Executes tools (file read/write, bash, grep, web search, etc.) with user permission
- Supports multi-agent task delegation, background agents, and swarm mode
- Integrates with IDEs (VS Code, JetBrains) via direct-connect bridge
- Supports remote sessions via WebSocket/SSE transports
- Has a plugin/skills marketplace
- Includes voice input (speech-to-text)
- Features a companion "buddy" system (Tamagotchi-style)
- Syncs sessions to the cloud via the bridge protocol

---

## 2. Repository Structure

```
Claude-Code/
├── src/                          # Main TypeScript/TSX source (34 MB, ~1,902 files)
│   ├── main.tsx                  # PRIMARY ENTRY POINT (4,683 lines)
│   ├── replLauncher.tsx          # REPL mode launcher
│   ├── query.ts                  # Main query/turn execution engine (69KB)
│   ├── QueryEngine.ts            # Query engine class (46KB)
│   ├── Tool.ts                   # Tool base framework (30KB)
│   ├── Task.ts                   # Task definitions
│   ├── commands.ts               # Command registry (25KB)
│   ├── context.ts                # Context management
│   ├── cost-tracker.ts           # Cost tracking (11KB)
│   ├── costHook.ts               # Cost hooks
│   ├── history.ts                # Session history (14KB)
│   ├── dialogLaunchers.tsx       # Dialog launchers (23KB)
│   ├── interactiveHelpers.tsx    # Interactive UI helpers (57KB)
│   ├── projectOnboardingState.ts # Project onboarding state
│   ├── setup.ts                  # Initialization (21KB)
│   ├── tasks.ts                  # Task management
│   ├── tools.ts                  # Tools registry (17KB)
│   ├── ink.ts                    # Ink export shim
│   │
│   ├── assistant/                # Assistant session history
│   ├── bootstrap/                # Bootstrap/state
│   ├── bridge/                   # Bridge protocol (31 files)
│   ├── buddy/                    # Companion pet system (6 files)
│   ├── cli/                      # CLI framework & transports (19 files)
│   ├── commands/                 # 87 slash commands (207 files)
│   ├── components/               # React/Ink UI components (389 files, 32 subdirs)
│   ├── constants/                # Constants & config values (21 files)
│   ├── context/                  # React context providers (9 files)
│   ├── coordinator/              # Coordinator mode logic
│   ├── entrypoints/              # Multiple entry points (8 files)
│   ├── hooks/                    # React hooks (104 files)
│   ├── ink/                      # Custom Ink terminal framework (96 files)
│   ├── keybindings/              # Keyboard shortcut system (14 files)
│   ├── memdir/                   # Memory directory system (8 files)
│   ├── migrations/               # Settings migrations (11 files)
│   ├── moreright/                # useMoreRight hook
│   ├── native-ts/                # Native TypeScript bindings (4 files)
│   ├── outputStyles/             # Output style loader
│   ├── plugins/                  # Plugin system (2 files)
│   ├── query/                    # Query helpers (4 files)
│   ├── remote/                   # Remote session management (4 files)
│   ├── schemas/                  # Zod/JSON schemas
│   ├── screens/                  # Top-level screen layouts (3 files)
│   ├── server/                   # Direct-connect server (3 files)
│   ├── services/                 # Business logic services (130 files)
│   ├── skills/                   # Claude skills/slash commands (20 files)
│   ├── tools/                    # Tool implementations (40+ tools, 184 files)
│   ├── types/                    # TypeScript type definitions
│   ├── utils/                    # Utility functions (~564 files)
│   └── voice/                    # Voice integration
│
├── claude-code-rust/             # Rust port (in-progress, 47 files)
│   ├── Cargo.toml                # Workspace manifest
│   ├── tools/                    # 27 files — tool implementations
│   ├── query/                    # 5 files — query system
│   ├── cli/                      # 3 files — CLI framework
│   ├── api/                      # 2 files — API bindings
│   ├── bridge/                   # 2 files — bridge protocol
│   ├── commands/                 # 2 files — command system
│   ├── core/                     # 2 files — core utilities
│   ├── mcp/                      # 2 files — MCP integration
│   └── tui/                      # 2 files — terminal UI
│
├── public/                       # Static assets
├── README.md                     # Main documentation (27KB)
└── .git/                         # Git metadata
```

---

## 3. High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         USER INTERFACE                           │
│  Terminal (Ink TUI) ←→ React Components ←→ Hooks ←→ Context     │
└────────────────────────────┬────────────────────────────────────┘
                             │
┌────────────────────────────▼────────────────────────────────────┐
│                       MAIN APPLICATION                           │
│  main.tsx → REPL.tsx → PromptInput → MessageList                │
│  Commands (87) ←→ Command Registry ←→ Plugin System             │
└────────────────────────────┬────────────────────────────────────┘
                             │
┌────────────────────────────▼────────────────────────────────────┐
│                       QUERY ENGINE                               │
│  query.ts → QueryEngine.ts → Tool execution → Response handling  │
│  Token budget → Stop hooks → Compact → History                   │
└────────────────────────────┬────────────────────────────────────┘
                             │
┌────────────────────────────▼────────────────────────────────────┐
│                       TOOL SYSTEM (40+ tools)                    │
│  BashTool, FileReadTool, FileEditTool, FileWriteTool             │
│  GlobTool, GrepTool, WebFetchTool, WebSearchTool                 │
│  AgentTool, TaskCreateTool, MCPTool, SkillTool, ...              │
└────────────────────────────┬────────────────────────────────────┘
                             │
┌────────────────────────────▼────────────────────────────────────┐
│                       SERVICES LAYER                             │
│  API Client (claude.ts) → Analytics → SessionMemory             │
│  AutoDream → Compact → RateLimit → MCP servers                  │
└────────────────────────────┬────────────────────────────────────┘
                             │
┌────────────────────────────▼────────────────────────────────────┐
│                       TRANSPORT LAYER                            │
│  CLI (local) / Bridge (remote) / IDE direct-connect             │
│  SSETransport | WebSocketTransport | HybridTransport            │
└─────────────────────────────────────────────────────────────────┘
```

---

## 4. Core Subsystems

### 4.1 Query / Turn Execution (`query.ts`, `QueryEngine.ts`)
The core loop that:
1. Takes user input
2. Builds the API request (system prompt + history + tools)
3. Streams the response from Claude API
4. Handles tool use (executes tools, feeds results back)
5. Manages token budget and context compaction
6. Tracks cost

### 4.2 Tool Framework (`Tool.ts`, `tools/`)
- Base `Tool` abstract class/interface
- Input schema validation (Zod)
- Permission system (each tool declares required permissions)
- 40+ tool implementations
- Sandboxing for dangerous tools

### 4.3 Terminal UI (`ink/`, `components/`)
- Custom React reconciler that renders to terminal
- Layout engine based on Yoga (flexbox for terminal)
- Event system (keyboard, mouse, focus)
- ANSI/CSI/escape sequence processing
- Components: Messages, PromptInput, Spinner, Dialogs, etc.

### 4.4 Commands System (`commands/`, `commands.ts`)
- 87 slash commands (e.g., `/compact`, `/diff`, `/plan`, `/mcp`)
- Plugin-contributed commands
- Command registry with fuzzy matching
- Keybinding integration

### 4.5 Bridge Protocol (`bridge/`)
- Enables remote/cloud-synced sessions
- JWT-authenticated WebSocket/SSE connection to cloud backend servers
- REPL bridge for IDE integration
- Message polling, flush gates, session runners

### 4.6 Multi-Agent System (`tools/AgentTool.ts`, `components/agents/`)
- Spawn sub-agents as isolated Claude instances
- Background task execution
- Coordinator mode (orchestrate multiple agents)
- Swarm mode (parallel worker agents)
- Team system for collaborative agents

### 4.7 Memory System (`memdir/`, `services/SessionMemory/`, `services/autoDream/`)
- Short-term: session history
- Long-term: memdir (markdown files in `~/.claude/memory/`)
- Auto-consolidation: "dream" service consolidates memories during idle
- Memory scanning/relevance scoring for context injection

### 4.8 MCP Integration (`tools/MCPTool.ts`, `components/mcp/`, `entrypoints/mcp.ts`)
- Model Context Protocol server support
- Dynamic tool registration from MCP servers
- Resource management
- Elicitation dialog support

### 4.9 Plugin/Skills System (`plugins/`, `skills/`, `commands/plugin/`)
- Built-in plugins
- Marketplace for community plugins
- Skills: user-invocable slash command macros
- Plugin trust model with approval flow

### 4.10 IDE Integration (`bridge/`, `hooks/useIDEIntegration.tsx`)
- VS Code / JetBrains extensions connect via direct-connect
- Live diff viewing in IDE
- File selection sync (IDE → Claude)
- Status indicator in IDE

---

## 5. Data Flow: A User Turn

```
1. User types in PromptInput
2. Input submitted → useCommandQueue processes
3. If slash command: dispatched to command handler
4. If regular prompt: sent to query.ts runQuery()
5. QueryEngine builds API request:
   - System prompt (from constants/prompts.ts + CLAUDE.md)
   - Message history (from history.ts)
   - Available tools (filtered by permission)
   - Token budget constraints
6. Stream response from the Claude API (services/api/claude.ts)
7. For each content block:
   - text → render AssistantTextMessage
   - thinking → render AssistantThinkingMessage
   - tool_use → execute tool, show permission dialog if needed
8. Tool results fed back into next API request
9. Loop until stop condition (no more tool use, stop hook, budget exceeded)
10. Final response rendered, history updated, cost tracked
```

---

## 6. Key Files by Importance

| Rank | File | Size | Role |
|------|------|------|------|
| 1 | `src/main.tsx` | 4,683 lines | Primary entry point, app initialization |
| 2 | `src/query.ts` | 69KB | Main query execution loop |
| 3 | `src/QueryEngine.ts` | 46KB | Query engine class |
| 4 | `src/interactiveHelpers.tsx` | 57KB | Interactive UI helpers |
| 5 | `src/Tool.ts` | 30KB | Tool base framework |
| 6 | `src/commands.ts` | 25KB | Command registry |
| 7 | `src/dialogLaunchers.tsx` | 23KB | Dialog launch system |
| 8 | `src/setup.ts` | 21KB | Initialization |
| 9 | `src/tools.ts` | 17KB | Tools registry |
| 10 | `src/history.ts` | 14KB | Session history |

---

## 7. Permission Model

Claude Code uses a layered permission system:

1. **Automatic** — Read-only operations, info queries
2. **Ask Once** — Prompt user, remember for session
3. **Ask Always** — Prompt user every time
4. **Deny** — Block completely

Permission rules are stored in settings (global `~/.claude/settings.json`, project `.claude/settings.json`) and can be configured with patterns.

Permission categories:
- `Bash` — Shell command execution
- `FileRead` — Reading files/directories
- `FileEdit` — Editing existing files
- `FileWrite` — Creating new files
- `WebFetch` — HTTP requests
- `MCP` — MCP tool calls
- `Sandbox` — Sandboxed execution

---

## 8. Settings System

Layered settings (in priority order):
1. **Managed** — Enterprise/managed settings (read-only)
2. **Local project** — `.claude/settings.local.json` (gitignored)
3. **Project** — `.claude/settings.json` (shared)
4. **Global** — `~/.claude/settings.json`

Settings include: model selection, permission rules, API key, theme, keybindings, MCP server configurations, beta features.

---

## 9. Model Support

Based on migration files, the model evolution:
- `claude-3-sonnet` → `claude-sonnet-1m` → `claude-sonnet-4-5` → `claude-sonnet-4-6`
- `claude-3-opus` → `claude-opus-1m` → `claude-opus` → (various)
- `claude-3-5-haiku` → (current)
- `claude-haiku-4-5` (current haiku)

Current defaults (as of source): `claude-sonnet-4-6` and `claude-opus-4-6`

---

## 10. Analytics & Telemetry

- **First-party logging** — Session events to the backend (`services/analytics/`)
- **Datadog** — Performance metrics
- **Growthbook** — Feature flags / A/B testing
- **Opt-out** — `services/api/metricsOptOut.ts` handles user opt-out

---

## 11. Spec Document Index

| File | Contents |
|------|----------|
| `00_overview.md` | This file — master architecture overview |
| `01_core_entry_query.md` | Entry points, query system, history, cost tracking |
| `02_commands.md` | All 87 slash commands |
| `03_tools.md` | All 40+ tool implementations |
| `04_components_core_messages.md` | Top-level components and message components |
| `05_components_agents_permissions_design.md` | Agents, permissions, design system, feature modules |
| `06_services_context_state.md` | Services, context providers, state, screens, server |
| `07_hooks.md` | All React hooks |
| `08_ink_terminal.md` | Ink terminal rendering framework |
| `09_bridge_cli_remote.md` | Bridge protocol, CLI framework, remote sessions |
| `10_utils.md` | All utility functions (~564 files) |
| `11_special_systems.md` | Buddy, memory, keybindings, skills, voice, plugins |
| `12_constants_types.md` | All constants, types, and configuration |
| `13_rust_codebase.md` | Rust port/rewrite |
| `INDEX.md` | Quick-reference index |

---

*Generated from source analysis of the Claude Code codebase. ~1,902 TypeScript/TSX files, ~800K+ lines of code.*
