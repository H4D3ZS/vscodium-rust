# VSCodium-Rust | Agentic & Sovereign IDE

**A friend of AI engineers** — local models, agent loops, and PyTorch ML Studio in one native IDE.
Train on your GPU, iterate with agents on your code, and keep weights + data on your machine.

A high-performance implementation of the VS Code architecture, rewritten using **Rust**, **Tauri**, and **TypeScript**.

VSCodium-Rust is a **full-scale, ultra-lightweight agentic development environment** designed for **data sovereignty, ML experimentation, and parallel-mind engineering.**

![VSCodium-Rust Agentic View](pics/1.png)

---

## Friend of AI engineers

We built this IDE for people who ship models *and* ship software:

| Pillar | What you get |
|--------|----------------|
| **Local-first** | Ollama + optional cloud keys — your data stays yours |
| **PyTorch ML Studio** | Setup → data → train → dashboard → export (TorchStudio-inspired) |
| **Agent-native** | Autonomous loop, shadow VFS verify, Hermes skills, MCP |
| **Research-ready** | Security modes, browser automation, reverse-engineering hooks |

See [docs/FRIEND_OF_AI_ENGINEERS.md](docs/FRIEND_OF_AI_ENGINEERS.md) for the full manifesto.

---

## ☁️ Open-Core / Cloud AI

The **IDE client** is open (MIT, when published as `Community AI-ide`) and fully usable
on its own with **local Ollama** — your data never leaves your machine.

**Cloud AI routing, Neural VFS compression, and subscription management are proprietary
services** hosted on Community AI infrastructure (AMD MI300X backend) — they are **not**
part of the open client. To use cloud AI features, subscribe at
**https://example.invalid**. See `SEPARATION.md`, `PROPRIETARY.md`, and
`FULL_SPLIT_PLAN.md` for the architecture.

> This repository is currently **private** and **All Rights Reserved** (`PROPRIETARY.md`)
> — it still contains the proprietary brain pending the open-core split.

---

## 🚀 Key Evolutionary Features

### 🧠 1. Claude Code Integrated (42+ Tools)
We have achieved 100% feature parity with Claude Code's agentic architecture. The built-in **Antigravity Agent** utilizes 42+ specialized tools with standard JSON schemas, allowing it to:
- **Analyze & Plan:** High-fidelity project research and roadmap generation.
- **Execute:** Atomic file edits, partial modifications, and full-volume writes.
- **Git & Terminal Mastery:** Native backend PTY terminals and Git integration for automated commits, diffs, and staging.

### 🌐 2. Parallel Mind Architecture (Multi-Agent)
The only IDE that supports **True Asynchronous Sub-Agent Orchestration**. Delegate complex tasks to specialized background agents:
- **Research while Implementing:** Spawn a browser sub-agent to find documentation while you implement the feature.
- **Multi-Tasking:** Run planning, roadmap, development, and reverse engineering tasks simultaneously in a parallelized backend (`tokio` + `Arc<Self>`).
- **Live Progress:** Real-time tracking of all background thoughts and tasks in the Agent Sidebar.

### 🏠 3. Absolute Data Sovereignty (Ollama First)
VSCodium-Rust is designed for developers who demand **freedom from corporate filters**:
- **Self-Hosted Brain:** Connect to local models via **Ollama** or custom providers with 100% private, offline tool-calling.
- **JSON Schema Parity:** All IDE tools are exposed via standard formats, ensuring any tool-calling model (Llama 3, Mistral) can be fully agentic within your workspace.
- **Pay Only for What You Use:** Bring your own API keys for hosted models (Anthropic, OpenAI, Gemini) and eliminate redundant subscriptions.

### 📊 5. PyTorch ML Studio (Friend of ML engineers)
In-IDE machine learning — inspired by [TorchStudio](https://www.torchstudio.ai/), wired for local sovereignty:

- **Full pipeline:** CSV/image datasets → train → live loss/acc charts → confusion matrix → ONNX/TorchScript export
- **Experiment tooling:** Optuna HPO, LR finder, grad check, multi-run comparison, checkpoint resume
- **Model hub:** torchvision, timm, and HuggingFace gallery with one-click weight load
- **Architecture graph:** Layer SVG visualization in the Model tab

Open from the activity bar (PyTorch beaker) or Settings → PyTorch ML Studio.

### 📊 6. Visual Lab & Data Flow Builder
A powerful, **Rust-backend-driven** visualization engine for complex data structures:
- **Instant JSON/SQL Visualization:** Toggle a visual graph view for any JSON file or SQL schema directly from the editor.
- **Cross-Format Support:** Intelligent parsers for JSON (hierarchical), SQL (ERDs), and MongoDB/BSON documents.
- **AI Flow Builder:** Describe your desired architecture in natural language and have the built-in engine generate a complete interactive diagram.
- **High Performance:** Optimized for 60fps interaction even with 100+ nodes using lightweight rendering and native layout calculations.

![Visual Lab Flow](pics/flow_visualizer.png)

---

## 🛠️ For Cybersecurity & Reverse Engineering
Built by a researcher for researchers. VSCodium-Rust is an elite tool for **Security Audits and Malware Analysis**:
- **Integrated Reverse Engineering:** Native **Model Context Protocol (MCP)** support for integration with tools like IDA Pro.
- **Isolated PTY Terminals:** Full control over process spawning and network isolation.
- **Simulator Mastery:** Integrated professional-grade emulators for **iOS (v26.3.1)** and Android directly in workspace panels. The iPhone-emulator integration ships in-repo; the heavy `acheron` hypervisor binary + iOS `.ipsw` firmware are user-provided — see **[docs/IPHONE_EMULATOR.md](docs/IPHONE_EMULATOR.md)** for how to obtain/build them.

---

## 📁 Project Architecture
- **Frontend:** TypeScript/Vite application designed for 100% visual parity with VS Code layout metrics.
- **Backend:** Rust (Tauri) handling IPC, file I/O, Git operations, and the **Agentic Dispatcher**.
- **Agent Orchestrator:** Modular, category-based tool handler with enforced path security (`validate_path`).

---

## ☁️ Cloud AI & Neural VFS

The Community AI IDE client is open-source under the MIT License. 
Cloud AI routing, Neural VFS compression, and subscription management are **proprietary services** hosted on Community AI infrastructure. 
To use AI features, you must subscribe to Community AI Cloud at https://example.invalid

---

## 📝 License & Credits
- **Credits:** Standing on the shoulders of giants: Zed Industries (GPUI), VSCodium Team, and Palinuro.
- **License:** MIT

---

**VSCodium-Rust is for AI engineers who demand speed, privacy, local GPUs, and full architectural sovereignty.**

[![Buy Me A Coffee](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://buymeacoffee.com/H4D3ZS)
