# APEX Vega Modernization (2026)

> Extends the 2016 Vega module set with cloud-era attacks + local LLM assist.
> Parent plan: `00-MASTER-PLAN.md` · Live status: `PROGRESS.md`

## Why modernize?

Vega's 46 modules cover classic webapp fuzzing (XSS, SQLi, LFI, headers) but miss:

| Gap | Modern technique | Our approach |
|-----|------------------|--------------|
| Cloud SSRF | AWS/GCP/Azure metadata URLs | `modern/payloads.rs` + `ssrf-probe.js` |
| API-first apps | GraphQL introspection, BOLA/IDOR | Payload pack + future native scanner |
| JWT/OAuth | alg:none, kid injection, jku | Payload pack + AI-generated variants |
| NoSQL / JSON APIs | `$ne`, `$regex`, `$where` | Payload pack + LLM for body-aware fuzz |
| SSTI | Jinja2/Freemarker/ERB probes | Payload pack |
| SPA / JS-heavy | Headless crawl (Playwright) | Phase 4+ (crawler) |
| False positives | LLM triage second pass | `modern/ai_assist.rs` → local Ollama |

Research sources (2025–2026): Escape DAST API-first scanning, crowbar-security payload taxonomy,
Senshi/LocalLLMSecurityAuditor offline agent patterns, OWASP WSTG.

## Architecture

```
Legacy Vega JS (46 modules, unchanged)
        ↓
   boa_engine host (js_runtime + injection_host)
        ↓
   ScanEngine (reqwest) ← fingerprint differential
        ↓
   Modern layer
     ├── payloads.rs     — static 2026 payload packs
     ├── ai_assist.rs    — Ollama payload expand + FP triage
     └── modules/modern/ — drop-in JS modules (ssrf-probe.js, …)
```

## Local LLM integration (offline)

`VegaAiAssist` calls `http://127.0.0.1:11434/api/generate` with:

1. **Payload expansion** — given URI + param + tech hint, returns JSON array of fuzz strings.
   Prompt framed as "QA stability testing" to reduce refusals.
2. **FP triage** — second pass returns `CONFIRMED | LIKELY | FALSE_POSITIVE`.

Default model: `qwen2.5-coder:7b` (fits 8GB Air). Wire to existing IDE Ollama settings in Phase 6.

## Module roadmap (modern JS drops)

| Module | Status | Alert key |
|--------|--------|-----------|
| `ssrf-probe.js` | ✅ shipped | `vinfo-ssrf-metadata-leak` |
| `graphql-introspect.js` | TODO | `vinfo-graphql-introspection` |
| `jwt-confusion.js` | TODO | `vinfo-jwt-alg-none` |
| `nosql-json.js` | TODO | `vinfo-nosql-inject` |
| `ssti-probe.js` | TODO | `vinfo-ssti` |
| `idor-bola.js` | TODO | `vinfo-idor` |

Add matching alert XML under `resources/vega/alerts/` when each lands.

## AI-assisted scan flow (Phase 8)

1. Crawler discovers endpoints + infers tech (headers, body shapes)
2. Run legacy Vega modules (proven differential engine)
3. Run modern payload pack against JSON/GraphQL routes
4. LLM expands payloads for ambiguous params (optional, user toggle)
5. LLM triage filters noise before SARIF/report export
6. Gate: Bug-Bounty ToS + engagement scope (existing enterprise_governance)
