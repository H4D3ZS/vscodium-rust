# VSCodium-Rust | Agentic & Sovereign IDE

A groundbreaking, high-performance implementation of the VS Code architecture, rewritten from the ground up using **Rust**, **Tauri**, and **TypeScript**. 

VSCodium-Rust is more than an editor — it is a **full-scale, ultra-lightweight agentic development environment** designed for **Data Sovereignty, Speed, and "Parallel Mind" Engineering.**

![VSCodium-Rust Agentic View](pics/1.png)

---

## ☁️ Open-Core / Cloud AI

The **IDE client** is open (MIT, when published as `cyber-ifrit-ide`) and fully usable
on its own with **local Ollama** — your data never leaves your machine.

**Cloud AI routing, Neural VFS compression, and subscription management are proprietary
services** hosted on Cyber-Ifrit infrastructure (AMD MI300X backend) — they are **not**
part of the open client. To use cloud AI features, subscribe at
**https://cyberifrit.xyz/pricing**. See `SEPARATION.md`, `PROPRIETARY.md`, and
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

### 📊 5. Visual Lab & Data Flow Builder
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

The Cyber-Ifrit IDE client is open-source under the MIT License. 
Cloud AI routing, Neural VFS compression, and subscription management are **proprietary services** hosted on Cyber-Ifrit infrastructure. 
To use AI features, you must subscribe to Cyber-Ifrit Cloud at https://cyberifrit.xyz/pricing

---

## 📝 License & Credits
- **Credits:** Standing on the shoulders of giants: Zed Industries (GPUI), VSCodium Team, and Palinuro.
- **License:** MIT

---

**VSCodium-Rust is for any developer who demands speed, privacy, and full architectural sovereignty.**

[![Buy Me A Coffee](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://buymeacoffee.com/H4D3ZS)
