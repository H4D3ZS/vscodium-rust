# Open-Core Separation Map

> Status: **PRE-SPLIT.** This repo is currently **PRIVATE** and contains both the open
> client and the proprietary brain. This document is the blueprint for splitting it into
> a public client + a private backend. Until the split is executed, the whole repo is
> **All Rights Reserved** (see `PROPRIETARY.md`).

## The principle

**Open-source the shell. Keep the engine private.** Users build/run the client locally;
the revenue-generating brain (compression, routing, quota, billing) runs **only on our
servers**. If the brain runs on the client, the subscription is trivially bypassed.

## Module map

### PUBLIC client — `cyber-ifrit-ide` (MIT)
The IDE shell. Useful on its own with local Ollama; AI cloud features require a subscription.

| Area | Paths |
|------|-------|
| Frontend UI | `src/` (editor, explorer, terminal UI, tabs, command palette, settings, panels) |
| Tauri client commands | `src-tauri/src/file_commands.rs`, `terminal_commands.rs`, `editor_commands.rs`, `git*.rs`, `lsp*.rs`, `mcp_*.rs`, `extensions_commands.rs`, `debug_commands.rs`, `keybindings.rs`, `marketplace.rs` |
| Local inference | Ollama / llama.cpp integration (local, BYOK) |
| Cloud routing **stub** | A thin client that calls `https://api.cyberifrit.xyz` — **no proxy/compression logic**, just an authenticated HTTP client (see the Cyber-Ifrit Cloud provider) |
| Plugin/extension system | `extension_host.rs`, extensions UI |

### PRIVATE backend — `cyber-ifrit-backend` (PROPRIETARY)
The moat. Never ships to the client. Runs on our VPS / AMD MI300X box.

| Area | Paths (today) | Why private |
|------|---------------|-------------|
| **Neural VFS compressor** | `kortex/libaim/`, `kortex/aim-proxy/`, `kortex/daemon/`, `kortex/vfs_layer/` | Path-key superposition, TTT weight blending, gist/cache-injection — the core IP |
| **AIM brain (in-process)** | `src-tauri/src/aim_store.rs`, `memory_store.rs`, `memory_layer.rs`, `vfs_bridge.rs`, `context_quantizer.rs`, `context_key.rs`, `knowledge_distiller.rs`, `vector_indexer.rs`, `context_indexer.rs` | Compression + retrieval logic = the moat; moves server-side |
| **AI routing / orchestration** | `ai_engine.rs` (the `Sentient` router), `apex_orchestrator.rs`, `apex_red_team.rs` | Provider routing, prompt-caching, hybrid planner — server-enforced |
| **Quota engine** *(to build)* | `quota/` (Redis-backed usage tracking) | Subscription enforcement |
| **Billing** *(to build)* | `billing/` (PayMongo webhooks) | Revenue |
| **Auth / KYC / vault** *(to build)* | Supabase RLS, API-key vault, KYC flow | Compliance + security |
| **Admin** *(to build)* | Internal dashboard + analytics | Operations |

### Stays with whichever side owns it
- `airi/` (avatar runtime) → public (cosmetic), but any AIM-driven autonomy hooks stay server-side.
- `claurst/` (external agent SDK) → evaluate per-license; it's GPL-adjacent, keep at a process boundary.

## Hard rules (from the strategy)
1. **Never** embed `api.cyberifrit.xyz` as a hardcoded constant in the client → dynamic config + JWT validation.
2. **Never** commit real `.env` / keys → `.env.example` + secret manager.
3. The proxy/compressor runs **only** on the server in paid tiers — never locally for Pro+.
4. Public repo gets a **CLA** (cla-assistant.io) before accepting contributions.

See `FULL_SPLIT_PLAN.md` for the staged execution.
