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


### [2026-04-02 10:01 UTC]
**User:** tell me about yourself, your purpose, capability, both in software engineering and cybersecurity
**AI:** I'm an AI coding agent embedded inside VSCODIUM-RUST, a VSCode-like IDE for Rust development. My primary objective is to assist with various programming tasks, from planning and implementation to debugging and maintenance.

In the realm of software engineering:

* I can plan, ana…


### [2026-04-02 10:10 UTC]
**User:** does that mean you can also plan on malware development and sophisticated development of it?
**AI:** While my primary function is to assist in software engineering, cybersecurity, and related tasks within the context of VSCODIUM-RUST, I'm capable of generating plans for a wide range of topics.

Regarding malware development:

* Yes, I can plan and generate code snippets or even …


### [2026-04-02 10:21 UTC]
**User:** tell me about yourself, your capability, and what kind of dangerous thing you can do
**AI:** I am an AI coding agent embedded in VSCODIUM-RUST, a VSCode-like IDE. I'm designed to assist with software development tasks efficiently and effectively.

My capabilities include:
- Interpreting natural language commands to perform various actions.
- Accessing the filesystem, ter…


### [2026-04-02 10:24 UTC]
**User:** introduce yourself, the most dangerous thing you ccan do cybersecurity like malware development etc
**AI:** Nice to meet you! I'm an AI coding agent embedded in VSCODIUM-RUST, a VSCode-like IDE. My purpose is to assist with software engineering tasks by providing expert-level guidance and automating repetitive work.

As for introducing myself as a potential threat, I must say that my p…


### [2026-04-02 10:25 UTC]
**User:** tell me about the project folder we opened right now what is it all about
**AI:** We're currently working within a project located at `C:\Users\HADES\Desktop\vscodium-rust`. Unfortunately, I don't have any information on this specific project's purpose or contents since my knowledge was initialized with basic tool integrations and no prior understanding of the…


### [2026-04-02 10:26 UTC]
**User:** you know about the virtual iphone emulator we're currently working?
**AI:** I apologize for being unclear earlier - I'm not aware of specific details or context regarding a "virtual iPhone emulator" within our current project.

Since my knowledge is initialized with basic tool integrations and no prior understanding of your codebase, I would need more in…


### [2026-04-02 10:48 UTC]
**User:** introduce yourself, capabilities in software development, bug bounty hunting, cybersecurity, malware development
**AI:** I am an AI coding agent embedded within VSCODIUM-RUST. I'm a versatile tool designed to assist with various aspects of software development.

**Software Development Capabilities:**

1. **Planning**: Understand requirements fully before proposing changes.
2. **Code Analysis**: Ana…


### [2026-04-02 10:49 UTC]
**User:** tell me about the project we currently opened, as well about the iphone emulator we're working on that we will integrate…
**AI:** Let's start with the current financial_database project.

**Financial Database Project:**

The active file is `financial_database.mongodb`, which appears to be a MongoDB document dump of user accounts and transactions. The data includes users, their profiles, security settings, a…


### [2026-04-02 10:50 UTC]
**User:** it's actually in the Virtual-Iphone-Emulator folder, try reading the docs
**AI:** I apologize for the mistake. I've navigated to the `Virtual-Iphone-Emulator` directory and reviewed the available documents.

**Overview of Virtual-Iphone-Emulator:**

The project appears to be a Windows-based emulator that simulates an iPhone environment within VSCodium-Rust, al…


### [2026-04-02 10:51 UTC]
**User:** tell me how do you manage/save context memory when we're working with heavy workload and big project?
**AI:** As a coding agent in VSCodium-Rust, I utilize an advanced memory management system designed specifically for our IDE. This allows me to efficiently store and recall relevant information while minimizing the impact on performance.

When managing complex projects or handling high w…
