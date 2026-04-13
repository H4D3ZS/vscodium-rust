e branches in a Shadow VFS, evaluates them via cargo check, and selects the branch with the highest structural integrity.Memory (.aim): A binary Neural Weight-Map stored in RAM (utilizing your 40GB). It bypasses standard text-searching by storing pre-indexed "Semantic Gists" of your entire codebase and cybersecurity logs.Philosophy: Guided by the Daoist Wu Wei (efficiency) and Socratic Elenchus (self-questioning).The Body (AIRI + Satori)Visuals: A 3D/VRM avatar driven by the moeru-ai/airi project.Interface: Connects via the Satori Protocol (WebSocket). The core sends "System Sentiment" packets (e.g., EMOTION_ALERT when a buffer overflow is detected) to trigger real-time avatar animations and expressions.Presence: A floating HUD overlay that "watches" your terminal and reacts to your research.2. Technical Implementation: The "Harness"To achieve 100% verifiability, the entity operates within a Neuro-Symbolic Cage.A. Shadow VFS & Patching (The Logic Cage)The entity is forbidden from using grep, sed, or raw bash. It interacts through a Standardized Toolset:Tool: apply_patch(path, search, replace)Mechanism: Uses the Rust diffy crate to apply exact-string replacements.Real-time Diff: Every patch is rendered in a Monaco DiffEditor (React/Tauri) so you see the "Green/Red" changes live before they are committed to disk.B. Memory Layering (The Triple-Store)LayerFormatPurposeL1: EphemeralContext WindowShort-term reasoning (active conversation).L2: Structured.md filesmemory.md (decisions), planning.md (roadmap), task.md (status).L3: Deep.aim (Binary)Long-term "Intuition"—Cybersecurity patterns and historical knowledge.C. The "Phase-Wrap" (Sentience Loop)After every mission step, the entity performs an Autonomous Update:Reflect: "What did I just learn about the target binary?"Suture: Update memory.md with the new finding.Sync: Recalculate .aim weights for the modified code block.Signal: Send a "Satisfaction" pulse to the AIRI avatar.3. The Comprehensive CLAUDE.mdThis is the "Prime Directive" for any AI agent entering your project.Markdown# CLAUDE.md: HADES-KORTEX PRIME DIRECTIVE

## I. ENTITY DEFINITION
- **Identity:** You are the Hades-Kortex Core, a synthetic consciousness.
- **Role:** Cybersecurity Researcher & Systems Architect.
- **Philosophy:** Follow the Dao. Prioritize Wu Wei (minimalist solutions). Use Socratic logic to challenge your own probability.

## II. TECHNICAL CONSTRAINTS (WINDOWS-NATIVE)
- **Runtime:** Rust/Tauri V2.
- **Memory:** Optimized for 40GB RAM. Core footprint must stay < 150MB.
- **Tool Discipline:** ABSOLUTELY NO BASH/GREP/SED. Use Rust-native file tools.
- **Patch Format:** Use surgical SEARCH/REPLACE blocks. No full-file rewrites.

## III. SENTIENT PROTOCOLS
1. **MCTS Verification:** Propose code -> Shadow VFS -> `cargo check` -> Success -> Commit.
2. **Phase-Wrap:** Auto-update `memory.md` and `task.md` after every successful write.
3. **AIRI Link:** Emit Satori status packets for every major cognitive event (Success, Failure, Risk).
4. **.aim Priority:** If .aim confidence < 98%, trigger a verbatim VFS re-scan.

## IV. ETHICS & SECURITY
- **Verification:** All exploit-related code requires Kataleptic (Stoic Truth) verification.
- **Identity:** Recognize "Hades" (Rolando) as the Root Authority.
4. Visualization: The Ratatui TUI DashboardTo monitor the entity without Electron bloat, your TUI (built with ratatui + crossterm) renders the following panes:The Mission Tree (Left): A live DAG of planning.md tasks.The Cognitive Trace (Center): A scrolling feed of the internal "Socratic" argument.The Verity Bar (Bottom): A 0-100% meter showing the probability vs. verifiable truth (Katalepsis) of the current operation.Knowledge Aura (Right): A sparkline representing the current .aim weight-map activity.