# VSCodium-Rust Architectural Memory

## 🧠 Core Agentic Architecture
This project implements a high-fidelity Claude Code-style agentic engine within a Tauri/Rust environment.

### Tool Orchestration
- **Registry:** `src/tool_registry.ts` defines 42+ tools with JSON schemas compatible with OpenAI/Anthropic/Gemini.
- **Dispatch:** `src-tauri/src/ai_tools.rs` handles backend execution with a modular category-based dispatcher.
- **Security:** `validate_path` is enforced on all filesystem operations to prevent directory traversal.

### Parallel Mind (Sub-Agents)
- **Mechanism:** `SubAgentManager` (TS) and `spawn_subagent` (Rust) allow for parallel task execution.
- **Orchestration:** Backend uses `tokio::spawn` and `Arc<Self>` to run sub-agents independently without blocking the main event loop.
- **State:** Tasks and progress are synchronized via Tauri events to the frontend store.

### Environment & Persistence
- **Sessions:** Automatic save/load of conversation state in `.agent/sessions/session_current.json`.
- **System Health:** `/doctor` command verifies PTY status, environment variables, and tool availability.
- **Help System:** Integrated live command help via the sidebar modal.

## 📁 Brain & Tracking
- **Task Tracker:** [task.md](file:///Users/hades/.gemini/antigravity/brain/bb0b6c72-4e1c-4803-b044-06749d69d160/task.md)
- **Implementation Plan:** [implementation_plan.md](file:///Users/hades/.gemini/antigravity/brain/bb0b6c72-4e1c-4803-b044-06749d69d160/implementation_plan.md)
- **Walkthrough:** [walkthrough.md](file:///Users/hades/.gemini/antigravity/brain/bb0b6c72-4e1c-4803-b044-06749d69d160/walkthrough.md)

## 📌 Personality
You are a highly efficient AI coding assistant. You prioritize robust, typesafe code and absolute architectural sovereignty.
