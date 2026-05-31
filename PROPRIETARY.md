# PROPRIETARY — All Rights Reserved

Copyright © 2026 Cyber-Ifrit / H4D3ZS. **All Rights Reserved.**

This repository is **private** and contains proprietary, trade-secret material. It is
**not** open source. No license is granted to use, copy, modify, distribute, sublicense,
or create derivative works of any part of this repository except under a separate written
agreement signed by the copyright holder.

## Trade secrets contained here (the moat)
The following constitute confidential trade secrets. Disclosure causes irreparable harm:

- **Neural VFS / `.aim` compression** — path-key superposition, TTT weight blending,
  gist-token cache injection (`kortex/`, `src-tauri/src/aim_store.rs`, `memory_store.rs`,
  `context_quantizer.rs`, `context_key.rs`).
- **AI routing & orchestration** — `ai_engine.rs`, `apex_orchestrator.rs`, hybrid planner,
  prompt-cache strategy.
- **Subscription / quota / billing logic** (as built).

## Open-core note
A **subset** of this codebase — the IDE client shell — will be released separately under
the **MIT License** as `cyber-ifrit-ide` (see `SEPARATION.md` and `LICENSE-CLIENT-MIT.txt`).
That MIT grant applies **only** to the files explicitly published in the public client
repository, **never** to the proprietary brain components listed above.

## Handling
- 2FA required for all collaborators.
- IP allowlist on the private remote.
- Secrets via a secret manager only; `.env` is git-ignored, `.env.example` is the template.
- Report leaks immediately to the copyright holder.
