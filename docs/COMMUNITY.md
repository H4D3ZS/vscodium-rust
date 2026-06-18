# Community Edition (this repository)

This is the **MIT-licensed community build** of VSCodium-Rust / Cyber-Ifrit IDE,
published at [github.com/H4D3ZS/vscodium-rust](https://github.com/H4D3ZS/vscodium-rust).

## What you get (free, local)

- Full IDE shell (editor, terminal, Git, LSP, MCP, agentic tools)
- **Local Ollama** and BYOK API keys — your data stays on your machine
- Agentic development, security / bug-bounty tooling (after ToS acceptance)
- Unlimited local usage meters (community entitlements)

## What is NOT in this repo

Hosted services run on Cyber-Ifrit infrastructure and are part of **Cyber-Ifrit Pro**:

| Feature | Where it lives |
|---------|----------------|
| Managed cloud AI (`ai.cyberifrit.xyz`) | Private SaaS + VPS gateway |
| QR Ph / PayMongo billing | [cyberifrit.xyz](https://cyberifrit.xyz) |
| Supabase subscription sync | Private SaaS IDE build |
| Neural VFS server-side compression | Backend (see `SEPARATION.md`) |

Subscribe at **https://cyberifrit.xyz/pricing** to unlock cloud models from the Pro IDE build.

## Build from source

```powershell
npm install
npm run dev:tauri   # dev
npx tauri build     # release installer
```

Requires [Ollama](https://ollama.com) for local AI. No account sign-in required.

## Contributing

External contributions require a CLA (planned — see `FULL_SPLIT_PLAN.md`).
Run `./scripts/check-oss-boundaries.sh` before opening a PR to avoid leaking SaaS billing code.

## Private SaaS fork

The full subscription IDE (IDE sign-in, trials, QR Ph checkout, cloud JWT sync) is maintained
in a separate private repository and is not published here.
