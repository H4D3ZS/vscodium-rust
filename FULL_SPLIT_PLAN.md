# Full Open-Core Split — Staged Plan

Goal: extract the proprietary brain into a private `cyber-ifrit-backend` running on our
VPS/AMD MI300X, leave a clean MIT client (`cyber-ifrit-ide`) that calls it over HTTPS with
a subscription JWT. This is a multi-week effort; do it in stages, keep `main` shippable.

## Stage 0 — Done / in progress (this session)
- ✅ Private SaaS repo (`vscodium-rust-ide-saas`) — full tree, brain included (safe, private).
- ✅ Open-core docs: `SEPARATION.md`, `PROPRIETARY.md`, `LICENSE-CLIENT-MIT.txt`.
- ✅ **Cyber-Ifrit Cloud provider** wired in the client (configurable endpoint + JWT) — the
  seam the brain will live behind. Off until the VPS endpoint is live.

## Stage 1 — Stand up the backend service (week 1)
1. New **private** repo `cyber-ifrit-backend`. Layout:
   ```
   src/aim-proxy/   # Rust/Axum MITM router (from kortex/aim-proxy)
   src/kortex/      # Neural VFS compressor (from kortex/libaim, daemon, vfs_layer)
   src/quota/       # Redis-backed usage tracking (new)
   src/billing/     # PayMongo webhook handlers (new)
   src/admin/       # internal dashboard + analytics (new)
   .env.example     # never commit real keys
   deploy/          # Docker, systemd, nginx
   ```
2. Move `kortex/` (libaim, aim-proxy, daemon, vfs_layer) **as-is** into the backend repo.
3. Wrap it in an **OpenAI-compatible HTTP API** (`/v1/chat/completions`, `/v1/models`):
   request → JWT validate → quota check → AIM compress/inject → forward to AMD inference
   (vLLM/Ollama on MI300X) → stream back. This is what the client's Cyber-Ifrit Cloud
   provider already expects.
4. Deploy to the $5–8 VPS behind `api.cyberifrit.xyz` (nginx + TLS). MI300X box as the
   inference upstream.

## Stage 2 — Subscription enforcement (week 1–2)
- **Supabase**: users, subscriptions, RLS. JWT issuance on login.
- **Quota engine**: Redis counters per user/tier (requests/mo, indexing GB). Reject/throttle
  past the tier cap. The 90%-token-saving Neural VFS makes the unit economics work.
- **PayMongo**: checkout + webhooks → flip subscription status in Supabase.
- **KYC** for Researcher/Enterprise (authorized-use compliance).

## Stage 3 — Thin the client (week 2)
- Remove the in-process brain modules from the client's `src-tauri/src/` (see SEPARATION.md):
  `aim_store`, `memory_store`, `memory_layer`, `vfs_bridge`, `context_quantizer`,
  `context_key`, `knowledge_distiller`, `vector_indexer`, `context_indexer`, and the
  routing in `ai_engine.rs` → replace with calls to the Cyber-Ifrit Cloud provider.
- Keep local-Ollama BYOK working (Community tier) so the open client is useful standalone.
- For Pro+ tiers, AI **only** routes through `api.cyberifrit.xyz` (no local proxy → no bypass).

## Stage 4 — Publish the open client (week 2–3)
- New **public** repo `cyber-ifrit-ide`. Push only the SEPARATION.md "PUBLIC" set.
- Apply `LICENSE-CLIENT-MIT.txt` as `LICENSE`.
- Add `CLA.md` + cla-assistant.io before accepting PRs.
- README: client is MIT; AI/Neural-VFS/billing are proprietary hosted services.

## Critical mistakes to avoid
| Mistake | Fix |
|---|---|
| Hardcode `api.cyberifrit.xyz` in client | Dynamic config + JWT validation (already configurable in the provider) |
| Commit real `.env` | `.env.example` + secret manager |
| `aim-proxy` runs locally for Pro | Proxy runs ONLY on the VPS |
| No CLA on public repo | cla-assistant.io before first external PR |
| Backend code in the public repo | Strict folder split + a CI check that fails if brain paths appear |

## CI guard (add to public client repo)
A pipeline step that greps the tree for proprietary markers (`aim_store`, `libaim`,
`path-key`, `PayMongo`, `SUPABASE_`, `api.cyberifrit.xyz` literal) and fails the build if
any are found — prevents accidental moat leakage on every push.
