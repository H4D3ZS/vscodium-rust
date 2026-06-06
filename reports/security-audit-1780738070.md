# Security Audit Report

- **Scope:** `.`
- **Depth:** deep
- **Files scanned:** 10308
- **Total findings:** 400

## Summary by severity

| Severity | Count |
|----------|------:|
| CRITICAL | 31 |
| HIGH | 89 |
| MEDIUM | 140 |
| LOW | 110 |
| INFO | 30 |

## Dependency posture

- `package.json` present — Run `npm audit` / `pnpm audit` to check npm dependencies for known CVEs.

## Findings

### SEC-023 — Hardcoded secret: private_key_block [Hardcoded Credentials]
- **Severity:** CRITICAL  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\k8s-manifest-generator\sub-skills\implementation-playbook.md:214`
- **Evidence:** `----…----  (27 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-062 — Hardcoded secret: github_token [Hardcoded Credentials]
- **Severity:** CRITICAL  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\kilo-docs\pages\kiloclaw\development-tools\github.md:58`
- **Evidence:** `ghp_…xxxx  (40 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-075 — Hardcoded secret: aws_access_key_id [Hardcoded Credentials]
- **Severity:** CRITICAL  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\src-rust\crates\core\src\team_memory_sync.rs:599`
- **Evidence:** `AKIA…MPLE  (20 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-076 — Hardcoded secret: private_key_block [Hardcoded Credentials]
- **Severity:** CRITICAL  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\src-rust\crates\core\src\team_memory_sync.rs:609`
- **Evidence:** `----…----  (31 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-118 — Hardcoded secret: stripe_secret_key [Hardcoded Credentials]
- **Severity:** CRITICAL  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.env:20`
- **Evidence:** `sk_t…vb8f  (32 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-123 — Hardcoded secret: stripe_secret_key [Hardcoded Credentials]
- **Severity:** CRITICAL  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-account.mjs:13`
- **Evidence:** `sk_t…vb8f  (32 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-126 — Hardcoded secret: stripe_secret_key [Hardcoded Credentials]
- **Severity:** CRITICAL  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-analytics.mjs:13`
- **Evidence:** `sk_t…vb8f  (32 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-129 — Hardcoded secret: stripe_secret_key [Hardcoded Credentials]
- **Severity:** CRITICAL  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-apikeys.mjs:13`
- **Evidence:** `sk_t…vb8f  (32 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-132 — Hardcoded secret: stripe_secret_key [Hardcoded Credentials]
- **Severity:** CRITICAL  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-audit.mjs:13`
- **Evidence:** `sk_t…vb8f  (32 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-135 — Hardcoded secret: stripe_secret_key [Hardcoded Credentials]
- **Severity:** CRITICAL  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-backup.mjs:13`
- **Evidence:** `sk_t…vb8f  (32 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-138 — Hardcoded secret: stripe_secret_key [Hardcoded Credentials]
- **Severity:** CRITICAL  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-billing.mjs:13`
- **Evidence:** `sk_t…vb8f  (32 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-141 — Hardcoded secret: stripe_secret_key [Hardcoded Credentials]
- **Severity:** CRITICAL  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-cert-issue.mjs:13`
- **Evidence:** `sk_t…vb8f  (32 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-144 — Hardcoded secret: stripe_secret_key [Hardcoded Credentials]
- **Severity:** CRITICAL  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-certificates.mjs:13`
- **Evidence:** `sk_t…vb8f  (32 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-147 — Hardcoded secret: stripe_secret_key [Hardcoded Credentials]
- **Severity:** CRITICAL  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-content.mjs:13`
- **Evidence:** `sk_t…vb8f  (32 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-150 — Hardcoded secret: stripe_secret_key [Hardcoded Credentials]
- **Severity:** CRITICAL  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-infra.mjs:13`
- **Evidence:** `sk_t…vb8f  (32 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-153 — Hardcoded secret: stripe_secret_key [Hardcoded Credentials]
- **Severity:** CRITICAL  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-leads.mjs:13`
- **Evidence:** `sk_t…vb8f  (32 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-156 — Hardcoded secret: stripe_secret_key [Hardcoded Credentials]
- **Severity:** CRITICAL  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-media.mjs:13`
- **Evidence:** `sk_t…vb8f  (32 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-159 — Hardcoded secret: stripe_secret_key [Hardcoded Credentials]
- **Severity:** CRITICAL  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-models.mjs:13`
- **Evidence:** `sk_t…vb8f  (32 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-162 — Hardcoded secret: stripe_secret_key [Hardcoded Credentials]
- **Severity:** CRITICAL  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-notifications.mjs:13`
- **Evidence:** `sk_t…vb8f  (32 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-165 — Hardcoded secret: stripe_secret_key [Hardcoded Credentials]
- **Severity:** CRITICAL  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-payments.mjs:13`
- **Evidence:** `sk_t…vb8f  (32 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-168 — Hardcoded secret: stripe_secret_key [Hardcoded Credentials]
- **Severity:** CRITICAL  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-settings.mjs:13`
- **Evidence:** `sk_t…vb8f  (32 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-171 — Hardcoded secret: stripe_secret_key [Hardcoded Credentials]
- **Severity:** CRITICAL  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-users.mjs:13`
- **Evidence:** `sk_t…vb8f  (32 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-174 — Hardcoded secret: stripe_secret_key [Hardcoded Credentials]
- **Severity:** CRITICAL  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\altcha-challenge.mjs:76`
- **Evidence:** `sk_t…vb8f  (32 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-177 — Hardcoded secret: stripe_secret_key [Hardcoded Credentials]
- **Severity:** CRITICAL  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\api-v1-authorize-key.mjs:16`
- **Evidence:** `sk_t…vb8f  (32 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-180 — Hardcoded secret: stripe_secret_key [Hardcoded Credentials]
- **Severity:** CRITICAL  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\api-v1-chat.mjs:13`
- **Evidence:** `sk_t…vb8f  (32 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-183 — Hardcoded secret: stripe_secret_key [Hardcoded Credentials]
- **Severity:** CRITICAL  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\api-v1-cheatsheets.mjs:164`
- **Evidence:** `sk_t…vb8f  (32 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-186 — Hardcoded secret: stripe_secret_key [Hardcoded Credentials]
- **Severity:** CRITICAL  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\api-v1-security.mjs:99`
- **Evidence:** `sk_t…vb8f  (32 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-189 — Hardcoded secret: stripe_secret_key [Hardcoded Credentials]
- **Severity:** CRITICAL  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\captcha-config.mjs:16`
- **Evidence:** `sk_t…vb8f  (32 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-192 — Hardcoded secret: stripe_secret_key [Hardcoded Credentials]
- **Severity:** CRITICAL  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\cloud-authorize.mjs:13`
- **Evidence:** `sk_t…vb8f  (32 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-195 — Hardcoded secret: stripe_secret_key [Hardcoded Credentials]
- **Severity:** CRITICAL  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\create-checkout.mjs:13`
- **Evidence:** `sk_t…vb8f  (32 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-198 — Hardcoded secret: stripe_secret_key [Hardcoded Credentials]
- **Severity:** CRITICAL  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\create-subscription.mjs:13`
- **Evidence:** `sk_t…vb8f  (32 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-011 — Hardcoded secret: postgres_url [Hardcoded Credentials]
- **Severity:** HIGH  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\environment-setup-guide\sub-skills\step-3-create-docker-composeyml.md:13`
- **Evidence:** `post…d@db  (33 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-021 — Hardcoded secret: postgres_url [Hardcoded Credentials]
- **Severity:** HIGH  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\javascript-typescript-typescript-scaffold\sub-skills\7-configure-development-tools.md:7`
- **Evidence:** `post…host  (32 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-025 — Hardcoded secret: postgres_url [Hardcoded Credentials]
- **Severity:** HIGH  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\prisma-expert\sub-skills\connection-management.md:43`
- **Evidence:** `post…host  (27 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-026 — Hardcoded secret: postgres_url [Hardcoded Credentials]
- **Severity:** HIGH  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\python-development-python-scaffold\sub-skills\7-configure-development-tools.md:15`
- **Evidence:** `post…host  (32 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-030 — Hardcoded secret: postgres_url [Hardcoded Credentials]
- **Severity:** HIGH  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\apps\server\.env:1`
- **Evidence:** `post…N@db  (52 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-059 — Hardcoded secret: postgres_url [Hardcoded Credentials]
- **Severity:** HIGH  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\services\telegram-bot\.env:1`
- **Evidence:** `post…host  (36 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-060 — Hardcoded secret: postgres_url [Hardcoded Credentials]
- **Severity:** HIGH  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\services\telegram-bot\README.md:32`
- **Evidence:** `post…host  (36 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-061 — Hardcoded secret: postgres_url [Hardcoded Credentials]
- **Severity:** HIGH  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\services\telegram-bot\README.md:53`
- **Evidence:** `post…host  (36 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-071 — Hardcoded secret: postgres_url [Hardcoded Credentials]
- **Severity:** HIGH  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\kilo-vscode\webview-ui\src\stories\marketplace.stories.tsx:132`
- **Evidence:** `post…host  (32 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-072 — Hardcoded secret: postgres_url [Hardcoded Credentials]
- **Severity:** HIGH  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\kilo-vscode\webview-ui\src\stories\marketplace.stories.tsx:140`
- **Evidence:** `post…host  (32 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-073 — Hardcoded secret: slack_token [Hardcoded Credentials]
- **Severity:** HIGH  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\kilo-vscode\webview-ui\src\stories\marketplace.stories.tsx:166`
- **Evidence:** `xoxb…xxxx  (17 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-081 — Hardcoded secret: postgres_url [Hardcoded Credentials]
- **Severity:** HIGH  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\.tmp\plugins\plugins\cloudflare\skills\cloudflare\references\hyperdrive\configuration.md:11`
- **Evidence:** `post…host  (25 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-082 — Hardcoded secret: mysql_url [Hardcoded Credentials]
- **Severity:** HIGH  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\.tmp\plugins\plugins\cloudflare\skills\cloudflare\references\hyperdrive\configuration.md:27`
- **Evidence:** `mysq…host  (22 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-083 — Hardcoded secret: postgres_url [Hardcoded Credentials]
- **Severity:** HIGH  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\.tmp\plugins\plugins\cloudflare\skills\cloudflare\references\hyperdrive\configuration.md:40`
- **Evidence:** `post…host  (30 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-084 — Hardcoded secret: postgres_url [Hardcoded Credentials]
- **Severity:** HIGH  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\.tmp\plugins\plugins\cloudflare\skills\cloudflare\references\hyperdrive\configuration.md:138`
- **Evidence:** `post…host  (30 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-085 — Hardcoded secret: postgres_url [Hardcoded Credentials]
- **Severity:** HIGH  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\.tmp\plugins\plugins\cloudflare\skills\cloudflare\references\hyperdrive\configuration.md:148`
- **Evidence:** `post…mote  (27 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-086 — Hardcoded secret: mysql_url [Hardcoded Credentials]
- **Severity:** HIGH  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\.tmp\plugins\plugins\cloudflare\skills\cloudflare\references\hyperdrive\configuration.md:151`
- **Evidence:** `mysq…mote  (24 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-087 — Hardcoded secret: postgres_url [Hardcoded Credentials]
- **Severity:** HIGH  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\.tmp\plugins\plugins\cloudflare\skills\cloudflare\references\hyperdrive\README.md:25`
- **Evidence:** `post…host  (25 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-088 — Hardcoded secret: postgres_url [Hardcoded Credentials]
- **Severity:** HIGH  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\.tmp\plugins\plugins\cloudflare\skills\wrangler\SKILL.md:495`
- **Evidence:** `post…host  (25 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-091 — Hardcoded secret: postgres_url [Hardcoded Credentials]
- **Severity:** HIGH  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\.tmp\plugins\plugins\render\skills\render-deploy\references\configuration-guide.md:333`
- **Evidence:** `post….com  (51 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-106 — Hardcoded secret: openai_api_key [Hardcoded Credentials]
- **Severity:** HIGH  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\rules\default.rules:17`
- **Evidence:** `sk-f…0918  (35 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-107 — Hardcoded secret: openai_api_key [Hardcoded Credentials]
- **Severity:** HIGH  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\rules\default.rules:18`
- **Evidence:** `sk-f…0918  (35 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-108 — Hardcoded secret: postgres_url [Hardcoded Credentials]
- **Severity:** HIGH  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\vendor_imports\skills\skills\.curated\cloudflare-deploy\references\hyperdrive\configuration.md:11`
- **Evidence:** `post…host  (25 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-109 — Hardcoded secret: mysql_url [Hardcoded Credentials]
- **Severity:** HIGH  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\vendor_imports\skills\skills\.curated\cloudflare-deploy\references\hyperdrive\configuration.md:27`
- **Evidence:** `mysq…host  (22 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-110 — Hardcoded secret: postgres_url [Hardcoded Credentials]
- **Severity:** HIGH  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\vendor_imports\skills\skills\.curated\cloudflare-deploy\references\hyperdrive\configuration.md:40`
- **Evidence:** `post…host  (30 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-111 — Hardcoded secret: postgres_url [Hardcoded Credentials]
- **Severity:** HIGH  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\vendor_imports\skills\skills\.curated\cloudflare-deploy\references\hyperdrive\configuration.md:138`
- **Evidence:** `post…host  (30 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-112 — Hardcoded secret: postgres_url [Hardcoded Credentials]
- **Severity:** HIGH  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\vendor_imports\skills\skills\.curated\cloudflare-deploy\references\hyperdrive\configuration.md:148`
- **Evidence:** `post…mote  (27 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-113 — Hardcoded secret: mysql_url [Hardcoded Credentials]
- **Severity:** HIGH  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\vendor_imports\skills\skills\.curated\cloudflare-deploy\references\hyperdrive\configuration.md:151`
- **Evidence:** `mysq…mote  (24 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-114 — Hardcoded secret: postgres_url [Hardcoded Credentials]
- **Severity:** HIGH  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\vendor_imports\skills\skills\.curated\cloudflare-deploy\references\hyperdrive\README.md:25`
- **Evidence:** `post…host  (25 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-115 — Hardcoded secret: postgres_url [Hardcoded Credentials]
- **Severity:** HIGH  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\vendor_imports\skills\skills\.curated\render-deploy\references\configuration-guide.md:333`
- **Evidence:** `post….com  (51 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-210 — Dangerous Eval [Dangerous Eval]
- **Severity:** HIGH  ·  **CWE:** CWE-95
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\loki-mode\benchmarks\results\2026-01-05-00-49-17\humaneval-solutions\160.py:29`
- **Evidence:** `return eval(expression)`
- **Remediation:** Avoid eval; parse/allow-list input instead of executing it.

### SEC-212 — Dangerous Eval [Dangerous Eval]
- **Severity:** HIGH  ·  **CWE:** CWE-95
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\loki-mode\benchmarks\results\humaneval-loki-solutions\160.py:34`
- **Evidence:** `return eval(expression)`
- **Remediation:** Avoid eval; parse/allow-list input instead of executing it.

### SEC-216 — Dangerous Eval [Dangerous Eval]
- **Severity:** HIGH  ·  **CWE:** CWE-95
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\playwright-skill\lib\helpers.js:174`
- **Evidence:** `return await page.$$eval(selector, elements =>`
- **Remediation:** Avoid eval; parse/allow-list input instead of executing it.

### SEC-217 — OS Command Injection [OS Command Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-78
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\typescript-expert\scripts\ts_diagnostic.py:16`
- **Evidence:** `result = subprocess.run(cmd, shell=True, capture_output=True, text=True)`
- **Remediation:** Never pass untrusted input to a shell; use argument arrays / parameterized APIs.

### SEC-218 — Dangerous Eval [Dangerous Eval]
- **Severity:** HIGH  ·  **CWE:** CWE-95
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\vulnerability-scanner\scripts\security_scan.py:63`
- **Evidence:** `(r'eval\s*\(', "eval() usage", "critical", "Code Injection risk"),`
- **Remediation:** Avoid eval; parse/allow-list input instead of executing it.

### SEC-219 — OS Command Injection [OS Command Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-78
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\vulnerability-scanner\scripts\security_scan.py:66`
- **Evidence:** `(r'child_process\.exec\s*\(', "child_process.exec", "high", "Command Injection risk"),`
- **Remediation:** Never pass untrusted input to a shell; use argument arrays / parameterized APIs.

### SEC-221 — Dangerous Eval [Dangerous Eval]
- **Severity:** HIGH  ·  **CWE:** CWE-95
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\apps\server\src\services\billing\flux-meter.ts:95`
- **Evidence:** `const raw = await redis.eval(`
- **Remediation:** Avoid eval; parse/allow-list input instead of executing it.

### SEC-247 — Dangerous Eval [Dangerous Eval]
- **Severity:** HIGH  ·  **CWE:** CWE-95
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\services\minecraft\src\cognitive\conscious\js-planner.ts:740`
- **Evidence:** `void new Function(`return (async () => (\n${script}\n))()`)`
- **Remediation:** Avoid eval; parse/allow-list input instead of executing it.

### SEC-267 — Dangerous Eval [Dangerous Eval]
- **Severity:** HIGH  ·  **CWE:** CWE-95
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\services\twitter-services\src\core\services\tweet.ts:120`
- **Evidence:** `const isAlreadyLiked = await page.$eval(`
- **Remediation:** Avoid eval; parse/allow-list input instead of executing it.

### SEC-306 — OS Command Injection [OS Command Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-78
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\kilo-vscode\src\util\process.ts:28`
- **Evidence:** `/** Promisified `child_process.execFile` with `windowsHide: true` forced on. */`
- **Remediation:** Never pass untrusted input to a shell; use argument arrays / parameterized APIs.

### SEC-309 — Dangerous Eval [Dangerous Eval]
- **Severity:** HIGH  ·  **CWE:** CWE-95
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\opencode\src\cli\cmd\debug\agent.ts:102`
- **Evidence:** `return new Function(`return (${trimmed})`)()`
- **Remediation:** Avoid eval; parse/allow-list input instead of executing it.

### SEC-311 — SQL Injection [SQL Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-89
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\src-rust\crates\api\src\lib.rs:543`
- **Evidence:** `format!("No API key for the selected model. {}", hint)`
- **Remediation:** Use parameterized queries / prepared statements, never string concatenation.

### SEC-312 — SQL Injection [SQL Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-89
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\src-rust\crates\api\src\lib.rs:646`
- **Evidence:** `format!("No API key for the selected model. {}", hint)`
- **Remediation:** Use parameterized queries / prepared statements, never string concatenation.

### SEC-313 — SQL Injection [SQL Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-89
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\src-rust\crates\commands\src\lib.rs:2180`
- **Evidence:** `return CommandResult::Error(format!("Failed to update settings: {}", e));`
- **Remediation:** Use parameterized queries / prepared statements, never string concatenation.

### SEC-314 — Dangerous Eval [Dangerous Eval]
- **Severity:** HIGH  ·  **CWE:** CWE-95
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\src-rust\crates\commands\src\lib.rs:5112`
- **Evidence:** `pub async fn eval(js: &str) -> anyhow::Result<String> {`
- **Remediation:** Avoid eval; parse/allow-list input instead of executing it.

### SEC-315 — Dangerous Eval [Dangerous Eval]
- **Severity:** HIGH  ·  **CWE:** CWE-95
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\src-rust\crates\commands\src\lib.rs:5279`
- **Evidence:** `match chrome_cdp::eval(rest).await {`
- **Remediation:** Avoid eval; parse/allow-list input instead of executing it.

### SEC-316 — SQL Injection [SQL Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-89
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\src-rust\crates\core\src\attachments.rs:118`
- **Evidence:** `parts.push(format!("selection: L{}-L{}", start, end));`
- **Remediation:** Use parameterized queries / prepared statements, never string concatenation.

### SEC-317 — SQL Injection [SQL Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-89
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\src-rust\crates\core\src\snapshot.rs:86`
- **Evidence:** `errors.push(format!("Failed to delete {}: {}", path, e));`
- **Remediation:** Use parameterized queries / prepared statements, never string concatenation.

### SEC-318 — SQL Injection [SQL Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-89
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\src-rust\crates\plugins\src\lib.rs:575`
- **Evidence:** `msg.push_str(&format!("\nUpdated: {}", diff.updated.join(", ")));`
- **Remediation:** Use parameterized queries / prepared statements, never string concatenation.

### SEC-321 — SQL Injection [SQL Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-89
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\src-rust\crates\tools\src\cron.rs:421`
- **Evidence:** `ToolResult::success(format!("Deleted cron task '{}'.", params.id))`
- **Remediation:** Use parameterized queries / prepared statements, never string concatenation.

### SEC-322 — SQL Injection [SQL Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-89
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\src-rust\crates\tools\src\notebook_edit.rs:291`
- **Evidence:** `Ok(format!("Inserted {} cell '{}' at position {}", cell_type, new_id, insert_at))`
- **Remediation:** Use parameterized queries / prepared statements, never string concatenation.

### SEC-323 — SQL Injection [SQL Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-89
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\src-rust\crates\tools\src\notebook_edit.rs:303`
- **Evidence:** `Ok(format!("Deleted cell '{}' (was at index {})", cell_id, idx))`
- **Remediation:** Use parameterized queries / prepared statements, never string concatenation.

### SEC-324 — SQL Injection [SQL Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-89
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\src-rust\crates\tui\src\app.rs:2869`
- **Evidence:** `self.status_message = Some(format!("Deleted branch: {}", branch_id));`
- **Remediation:** Use parameterized queries / prepared statements, never string concatenation.

### SEC-325 — SQL Injection [SQL Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-89
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\src-rust\crates\tui\src\session_branching.rs:337`
- **Evidence:** `Line::from(format!("Delete branch '{}'?", branch_name)),`
- **Remediation:** Use parameterized queries / prepared statements, never string concatenation.

### SEC-328 — Dangerous Eval [Dangerous Eval]
- **Severity:** HIGH  ·  **CWE:** CWE-95
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\geometry-of-consolidation\modal_app\app.py:88`
- **Evidence:** `# Separate image with vLLM for LLM downstream eval (E7). Kept separate from`
- **Remediation:** Avoid eval; parse/allow-list input instead of executing it.

### SEC-329 — Dangerous Eval [Dangerous Eval]
- **Severity:** HIGH  ·  **CWE:** CWE-95
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kilocode\packages\kilo-jetbrains\.intellijPlatform\ides\IU-2026.1\plugins\javascript-plugin\jsLanguageServicesImpl\external\lib.es5.d.ts:33`
- **Evidence:** `declare function eval(x: string): any;`
- **Remediation:** Avoid eval; parse/allow-list input instead of executing it.

### SEC-330 — OS Command Injection [OS Command Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-78
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kilocode\packages\kilo-jetbrains\.intellijPlatform\ides\IU-2026.1\plugins\javascript-plugin\jsLanguageServicesImpl\external\_typingsInstaller.js:175`
- **Evidence:** `const stdout = (0, import_child_process.execSync)(command, { ...options, encoding: "utf-8" });`
- **Remediation:** Never pass untrusted input to a shell; use argument arrays / parameterized APIs.

### SEC-331 — Disabled TLS Verification [Disabled TLS Verification]
- **Severity:** HIGH  ·  **CWE:** CWE-295
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kilocode\packages\kilo-jetbrains\.intellijPlatform\ides\IU-2026.1\plugins\karma\js_reporter\karma-intellij\lib\intellijRunner.js:42`
- **Evidence:** `rejectUnauthorized: false`
- **Remediation:** Never disable certificate validation.

### SEC-333 — OS Command Injection [OS Command Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-78
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kilocode\packages\kilo-vscode\src\util\process.ts:28`
- **Evidence:** `/** Promisified `child_process.execFile` with `windowsHide: true` forced on. */`
- **Remediation:** Never pass untrusted input to a shell; use argument arrays / parameterized APIs.

### SEC-334 — Dangerous Eval [Dangerous Eval]
- **Severity:** HIGH  ·  **CWE:** CWE-95
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kilocode\packages\opencode\src\cli\cmd\debug\agent.ts:110`
- **Evidence:** `return new Function(`return (${trimmed})`)()`
- **Remediation:** Avoid eval; parse/allow-list input instead of executing it.

### SEC-335 — Dangerous Eval [Dangerous Eval]
- **Severity:** HIGH  ·  **CWE:** CWE-95
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kilocode\packages\opencode\test\cli\bin-kilo.test.ts:8`
- **Evidence:** `expect(() => new Function(code)).not.toThrow()`
- **Remediation:** Avoid eval; parse/allow-list input instead of executing it.

### SEC-357 — Dangerous Eval [Dangerous Eval]
- **Severity:** HIGH  ·  **CWE:** CWE-95
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\harness\src\lib.rs:243`
- **Evidence:** `if content.contains("eval(") || content.contains("dangerouslySetInnerHTML") {`
- **Remediation:** Avoid eval; parse/allow-list input instead of executing it.

### SEC-363 — Dangerous Eval [Dangerous Eval]
- **Severity:** HIGH  ·  **CWE:** CWE-95
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\llama.cpp\tests\test-backend-ops.cpp:1280`
- **Evidence:** `test_status_t eval(ggml_backend_t backend1,`
- **Remediation:** Avoid eval; parse/allow-list input instead of executing it.

### SEC-364 — Dangerous Eval [Dangerous Eval]
- **Severity:** HIGH  ·  **CWE:** CWE-95
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\llama.cpp\tests\test-backend-ops.cpp:9215`
- **Evidence:** `test_status_t status = test->eval(backend, backend_cpu, op_names_filter, output_printer);`
- **Remediation:** Avoid eval; parse/allow-list input instead of executing it.

### SEC-365 — Dangerous Eval [Dangerous Eval]
- **Severity:** HIGH  ·  **CWE:** CWE-95
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\llama.cpp\tools\completion\completion.cpp:670`
- **Evidence:** `// try to reuse a matching prefix from the loaded session instead of re-eval (via n_past)`
- **Remediation:** Avoid eval; parse/allow-list input instead of executing it.

### SEC-366 — OS Command Injection [OS Command Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-78
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\llama.cpp\tools\server\bench\bench.py:228`
- **Evidence:** `k6_completed = subprocess.run(args, shell=True, stdout=sys.stdout, stderr=sys.stderr)`
- **Remediation:** Never pass untrusted input to a shell; use argument arrays / parameterized APIs.

### SEC-367 — SQL Injection [SQL Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-89
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\mcp-servers\browser-agent\.venv\Lib\site-packages\adodbapi\test\adodbapitest.py:158`
- **Evidence:** `crsr.execute("SELECT fldData,fld2 FROM xx_%s ORDER BY fldData" % config.tmp)`
- **Remediation:** Use parameterized queries / prepared statements, never string concatenation.

### SEC-368 — SQL Injection [SQL Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-89
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\mcp-servers\browser-agent\.venv\Lib\site-packages\adodbapi\test\adodbapitest.py:207`
- **Evidence:** `crsr.execute("INSERT INTO xx_%s (fldId) VALUES (1)" % config.tmp)`
- **Remediation:** Use parameterized queries / prepared statements, never string concatenation.

### SEC-369 — SQL Injection [SQL Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-89
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\mcp-servers\browser-agent\.venv\Lib\site-packages\adodbapi\test\adodbapitest.py:209`
- **Evidence:** `crsr.execute("SELECT fldId,fldData FROM xx_%s" % config.tmp)`
- **Remediation:** Use parameterized queries / prepared statements, never string concatenation.

### SEC-370 — SQL Injection [SQL Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-89
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\mcp-servers\browser-agent\.venv\Lib\site-packages\adodbapi\test\adodbapitest.py:488`
- **Evidence:** `crsr.execute("INSERT INTO xx_%s (fldData) VALUES (%i)" % (config.tmp, i))`
- **Remediation:** Use parameterized queries / prepared statements, never string concatenation.

### SEC-371 — SQL Injection [SQL Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-89
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\mcp-servers\browser-agent\.venv\Lib\site-packages\adodbapi\test\adodbapitest.py:494`
- **Evidence:** `crsr.execute("SELECT fldData FROM xx_%s" % config.tmp)`
- **Remediation:** Use parameterized queries / prepared statements, never string concatenation.

### SEC-372 — SQL Injection [SQL Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-89
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\mcp-servers\browser-agent\.venv\Lib\site-packages\adodbapi\test\adodbapitest.py:529`
- **Evidence:** `crsr.execute("SELECT fldData FROM xx_%s" % config.tmp)`
- **Remediation:** Use parameterized queries / prepared statements, never string concatenation.

### SEC-373 — SQL Injection [SQL Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-89
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\mcp-servers\browser-agent\.venv\Lib\site-packages\adodbapi\test\adodbapitest.py:550`
- **Evidence:** `crsr.execute("SELECT fldData FROM xx_%s" % config.tmp)`
- **Remediation:** Use parameterized queries / prepared statements, never string concatenation.

### SEC-374 — SQL Injection [SQL Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-89
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\mcp-servers\browser-agent\.venv\Lib\site-packages\adodbapi\test\adodbapitest.py:558`
- **Evidence:** `crsr.execute("SELECT fldData FROM xx_%s" % config.tmp)`
- **Remediation:** Use parameterized queries / prepared statements, never string concatenation.

### SEC-375 — SQL Injection [SQL Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-89
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\mcp-servers\browser-agent\.venv\Lib\site-packages\adodbapi\test\adodbapitest.py:569`
- **Evidence:** `crsr.execute("DELETE FROM xx_%s WHERE fldData >= 5" % config.tmp)`
- **Remediation:** Use parameterized queries / prepared statements, never string concatenation.

### SEC-376 — SQL Injection [SQL Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-89
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\mcp-servers\browser-agent\.venv\Lib\site-packages\adodbapi\test\adodbapitest.py:579`
- **Evidence:** `crsr.execute("SELECT fldData FROM xx_%s" % config.tmp)`
- **Remediation:** Use parameterized queries / prepared statements, never string concatenation.

### SEC-377 — SQL Injection [SQL Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-89
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\mcp-servers\browser-agent\.venv\Lib\site-packages\adodbapi\test\adodbapitest.py:591`
- **Evidence:** `crsr.execute("SELECT fldData FROM xx_%s" % config.tmp)`
- **Remediation:** Use parameterized queries / prepared statements, never string concatenation.

### SEC-378 — SQL Injection [SQL Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-89
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\mcp-servers\browser-agent\.venv\Lib\site-packages\adodbapi\test\adodbapitest.py:663`
- **Evidence:** `crsr.execute("select fldThree,fldFour,fldTwo from xx_%s" % config.tmp)`
- **Remediation:** Use parameterized queries / prepared statements, never string concatenation.

### SEC-379 — SQL Injection [SQL Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-89
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\mcp-servers\browser-agent\.venv\Lib\site-packages\adodbapi\test\adodbapitest.py:732`
- **Evidence:** `crsr.execute("SELECT fldData FROM xx_%s WHERE fldID=20" % config.tmp)`
- **Remediation:** Use parameterized queries / prepared statements, never string concatenation.

### SEC-380 — SQL Injection [SQL Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-89
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\mcp-servers\browser-agent\.venv\Lib\site-packages\adodbapi\test\adodbapitest.py:778`
- **Evidence:** `crsr.execute("SELECT fldData FROM xx_%s WHERE fldID=30" % config.tmp)`
- **Remediation:** Use parameterized queries / prepared statements, never string concatenation.

### SEC-381 — SQL Injection [SQL Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-89
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\mcp-servers\browser-agent\.venv\Lib\site-packages\adodbapi\test\adodbapitest.py:826`
- **Evidence:** `crsr.execute("SELECT fldData FROM xx_%s WHERE fldID=30" % config.tmp)`
- **Remediation:** Use parameterized queries / prepared statements, never string concatenation.

### SEC-382 — SQL Injection [SQL Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-89
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\mcp-servers\browser-agent\.venv\Lib\site-packages\adodbapi\test\adodbapitest.py:899`
- **Evidence:** `crsr.execute("SELECT fldData FROM xx_%s WHERE fldID=30" % config.tmp)`
- **Remediation:** Use parameterized queries / prepared statements, never string concatenation.

### SEC-383 — SQL Injection [SQL Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-89
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\mcp-servers\browser-agent\.venv\Lib\site-packages\adodbapi\test\adodbapitest.py:910`
- **Evidence:** `crsr.execute("INSERT INTO xx_%s (fldData) VALUES(100)" % config.tmp)`
- **Remediation:** Use parameterized queries / prepared statements, never string concatenation.

### SEC-384 — SQL Injection [SQL Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-89
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\mcp-servers\browser-agent\.venv\Lib\site-packages\adodbapi\test\adodbapitest.py:921`
- **Evidence:** `crsr.execute("SELECT fldData from xx_%s" % config.tmp)`
- **Remediation:** Use parameterized queries / prepared statements, never string concatenation.

### SEC-385 — SQL Injection [SQL Injection]
- **Severity:** HIGH  ·  **CWE:** CWE-89
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\mcp-servers\browser-agent\.venv\Lib\site-packages\adodbapi\test\adodbapitest.py:935`
- **Evidence:** `crsr.execute("INSERT INTO xx_%s (fldData) VALUES(100)" % config.tmp)`
- **Remediation:** Use parameterized queries / prepared statements, never string concatenation.

### SEC-386 — Unsafe Deserialization [Unsafe Deserialization]
- **Severity:** HIGH  ·  **CWE:** CWE-502
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\mcp-servers\browser-agent\.venv\Lib\site-packages\anyio\to_interpreter.py:130`
- **Evidence:** `res = pickle.loads(res)`
- **Remediation:** Use safe loaders / allow-lists / signed payloads.

### SEC-387 — Unsafe Deserialization [Unsafe Deserialization]
- **Severity:** HIGH  ·  **CWE:** CWE-502
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\mcp-servers\browser-agent\.venv\Lib\site-packages\anyio\to_process.py:94`
- **Evidence:** `retval = pickle.loads(pickled_response)`
- **Remediation:** Use safe loaders / allow-lists / signed payloads.

### SEC-388 — Dangerous Eval [Dangerous Eval]
- **Severity:** HIGH  ·  **CWE:** CWE-95
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\mcp-servers\browser-agent\.venv\Lib\site-packages\attr\_make.py:227`
- **Evidence:** `eval(bytecode, globs, locs)`
- **Remediation:** Avoid eval; parse/allow-list input instead of executing it.

### SEC-389 — Dangerous Eval [Dangerous Eval]
- **Severity:** HIGH  ·  **CWE:** CWE-95
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\mcp-servers\browser-agent\.venv\Lib\site-packages\cffi\recompiler.py:78`
- **Evidence:** `flags = eval(self.flags, G_FLAGS)`
- **Remediation:** Avoid eval; parse/allow-list input instead of executing it.

### SEC-029 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\.agents\skills\vueuse-functions\references\useJwt.md:21`
- **Evidence:** `eyJh…UwCc  (131 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-116 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.env:7`
- **Evidence:** `eyJh…7ujQ  (208 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-117 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.env:9`
- **Evidence:** `eyJh…vGTw  (219 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-119 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.env:29`
- **Evidence:** `eyJh…7ujQ  (208 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-120 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.env.development:13`
- **Evidence:** `eyJh…7ujQ  (208 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-121 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-account.mjs:4`
- **Evidence:** `eyJh…7ujQ  (208 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-122 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-account.mjs:5`
- **Evidence:** `eyJh…vGTw  (219 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-124 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-analytics.mjs:4`
- **Evidence:** `eyJh…7ujQ  (208 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-125 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-analytics.mjs:5`
- **Evidence:** `eyJh…vGTw  (219 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-127 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-apikeys.mjs:4`
- **Evidence:** `eyJh…7ujQ  (208 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-128 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-apikeys.mjs:5`
- **Evidence:** `eyJh…vGTw  (219 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-130 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-audit.mjs:4`
- **Evidence:** `eyJh…7ujQ  (208 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-131 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-audit.mjs:5`
- **Evidence:** `eyJh…vGTw  (219 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-133 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-backup.mjs:4`
- **Evidence:** `eyJh…7ujQ  (208 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-134 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-backup.mjs:5`
- **Evidence:** `eyJh…vGTw  (219 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-136 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-billing.mjs:4`
- **Evidence:** `eyJh…7ujQ  (208 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-137 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-billing.mjs:5`
- **Evidence:** `eyJh…vGTw  (219 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-139 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-cert-issue.mjs:4`
- **Evidence:** `eyJh…7ujQ  (208 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-140 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-cert-issue.mjs:5`
- **Evidence:** `eyJh…vGTw  (219 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-142 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-certificates.mjs:4`
- **Evidence:** `eyJh…7ujQ  (208 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-143 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-certificates.mjs:5`
- **Evidence:** `eyJh…vGTw  (219 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-145 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-content.mjs:4`
- **Evidence:** `eyJh…7ujQ  (208 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-146 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-content.mjs:5`
- **Evidence:** `eyJh…vGTw  (219 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-148 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-infra.mjs:4`
- **Evidence:** `eyJh…7ujQ  (208 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-149 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-infra.mjs:5`
- **Evidence:** `eyJh…vGTw  (219 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-151 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-leads.mjs:4`
- **Evidence:** `eyJh…7ujQ  (208 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-152 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-leads.mjs:5`
- **Evidence:** `eyJh…vGTw  (219 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-154 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-media.mjs:4`
- **Evidence:** `eyJh…7ujQ  (208 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-155 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-media.mjs:5`
- **Evidence:** `eyJh…vGTw  (219 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-157 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-models.mjs:4`
- **Evidence:** `eyJh…7ujQ  (208 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-158 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-models.mjs:5`
- **Evidence:** `eyJh…vGTw  (219 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-160 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-notifications.mjs:4`
- **Evidence:** `eyJh…7ujQ  (208 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-161 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-notifications.mjs:5`
- **Evidence:** `eyJh…vGTw  (219 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-163 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-payments.mjs:4`
- **Evidence:** `eyJh…7ujQ  (208 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-164 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-payments.mjs:5`
- **Evidence:** `eyJh…vGTw  (219 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-166 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-settings.mjs:4`
- **Evidence:** `eyJh…7ujQ  (208 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-167 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-settings.mjs:5`
- **Evidence:** `eyJh…vGTw  (219 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-169 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-users.mjs:4`
- **Evidence:** `eyJh…7ujQ  (208 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-170 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\admin-users.mjs:5`
- **Evidence:** `eyJh…vGTw  (219 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-172 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\altcha-challenge.mjs:67`
- **Evidence:** `eyJh…7ujQ  (208 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-173 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\altcha-challenge.mjs:68`
- **Evidence:** `eyJh…vGTw  (219 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-175 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\api-v1-authorize-key.mjs:7`
- **Evidence:** `eyJh…7ujQ  (208 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-176 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\api-v1-authorize-key.mjs:8`
- **Evidence:** `eyJh…vGTw  (219 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-178 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\api-v1-chat.mjs:4`
- **Evidence:** `eyJh…7ujQ  (208 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-179 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\api-v1-chat.mjs:5`
- **Evidence:** `eyJh…vGTw  (219 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-181 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\api-v1-cheatsheets.mjs:155`
- **Evidence:** `eyJh…7ujQ  (208 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-182 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\api-v1-cheatsheets.mjs:156`
- **Evidence:** `eyJh…vGTw  (219 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-184 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\api-v1-security.mjs:90`
- **Evidence:** `eyJh…7ujQ  (208 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-185 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\api-v1-security.mjs:91`
- **Evidence:** `eyJh…vGTw  (219 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-187 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\captcha-config.mjs:7`
- **Evidence:** `eyJh…7ujQ  (208 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-188 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\captcha-config.mjs:8`
- **Evidence:** `eyJh…vGTw  (219 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-190 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\cloud-authorize.mjs:4`
- **Evidence:** `eyJh…7ujQ  (208 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-191 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\cloud-authorize.mjs:5`
- **Evidence:** `eyJh…vGTw  (219 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-193 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\create-checkout.mjs:4`
- **Evidence:** `eyJh…7ujQ  (208 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-194 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\create-checkout.mjs:5`
- **Evidence:** `eyJh…vGTw  (219 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-196 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\create-subscription.mjs:4`
- **Evidence:** `eyJh…7ujQ  (208 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-197 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\create-subscription.mjs:5`
- **Evidence:** `eyJh…vGTw  (219 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-199 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\healthz.mjs:4`
- **Evidence:** `eyJh…7ujQ  (208 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-200 — Hardcoded secret: jwt_token [Hardcoded Credentials]
- **Severity:** MEDIUM  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\Cyber-Ifrit-Portfolio\.netlify\dev-bundles\healthz.mjs:5`
- **Evidence:** `eyJh…vGTw  (219 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-201 — Insecure Randomness [Insecure Randomness]
- **Severity:** MEDIUM  ·  **CWE:** CWE-330
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\agent-d3js-skill\assets\interactive-template.jsx:212`
- **Evidence:** `x: Math.random() * 100,`
- **Remediation:** Use a CSPRNG (crypto.randomBytes, secrets, getrandom) for security tokens.

### SEC-202 — Insecure Randomness [Insecure Randomness]
- **Severity:** MEDIUM  ·  **CWE:** CWE-330
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\agent-d3js-skill\assets\interactive-template.jsx:213`
- **Evidence:** `y: Math.random() * 100,`
- **Remediation:** Use a CSPRNG (crypto.randomBytes, secrets, getrandom) for security tokens.

### SEC-203 — Insecure Randomness [Insecure Randomness]
- **Severity:** MEDIUM  ·  **CWE:** CWE-330
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\agent-d3js-skill\assets\interactive-template.jsx:214`
- **Evidence:** `size: Math.random() * 30 + 5,`
- **Remediation:** Use a CSPRNG (crypto.randomBytes, secrets, getrandom) for security tokens.

### SEC-204 — Insecure Randomness [Insecure Randomness]
- **Severity:** MEDIUM  ·  **CWE:** CWE-330
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\agent-d3js-skill\assets\interactive-template.jsx:215`
- **Evidence:** `category: ['A', 'B', 'C', 'D'][Math.floor(Math.random() * 4)]`
- **Remediation:** Use a CSPRNG (crypto.randomBytes, secrets, getrandom) for security tokens.

### SEC-205 — Cross-Site Scripting [Cross-Site Scripting]
- **Severity:** MEDIUM  ·  **CWE:** CWE-79
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\loki-mode\autonomy\run.sh:758`
- **Evidence:** `document.getElementById('agents-grid').innerHTML = agents.length`
- **Remediation:** Escape/encode output; prefer textContent or a sanitizer (DOMPurify).

### SEC-206 — Cross-Site Scripting [Cross-Site Scripting]
- **Severity:** MEDIUM  ·  **CWE:** CWE-79
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\loki-mode\autonomy\run.sh:771`
- **Evidence:** `document.getElementById('pending-tasks').innerHTML = pending.length ? pending.map(renderTask).join('') : '<div class="empty">No pending tasks</div>';`
- **Remediation:** Escape/encode output; prefer textContent or a sanitizer (DOMPurify).

### SEC-207 — Cross-Site Scripting [Cross-Site Scripting]
- **Severity:** MEDIUM  ·  **CWE:** CWE-79
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\loki-mode\autonomy\run.sh:772`
- **Evidence:** `document.getElementById('progress-tasks').innerHTML = progress.length ? progress.map(renderTask).join('') : '<div class="empty">No tasks in progress</div>';`
- **Remediation:** Escape/encode output; prefer textContent or a sanitizer (DOMPurify).

### SEC-208 — Cross-Site Scripting [Cross-Site Scripting]
- **Severity:** MEDIUM  ·  **CWE:** CWE-79
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\loki-mode\autonomy\run.sh:773`
- **Evidence:** `document.getElementById('completed-tasks').innerHTML = completed.length ? completed.slice(-10).reverse().map(renderTask).join('') : '<div class="empty">No compl`
- **Remediation:** Escape/encode output; prefer textContent or a sanitizer (DOMPurify).

### SEC-209 — Cross-Site Scripting [Cross-Site Scripting]
- **Severity:** MEDIUM  ·  **CWE:** CWE-79
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\loki-mode\autonomy\run.sh:774`
- **Evidence:** `document.getElementById('failed-tasks').innerHTML = failed.length ? failed.map(renderTask).join('') : '<div class="empty">No failed tasks</div>';`
- **Remediation:** Escape/encode output; prefer textContent or a sanitizer (DOMPurify).

### SEC-211 — Weak Cryptographic Hash [Weak Cryptographic Hash]
- **Severity:** MEDIUM  ·  **CWE:** CWE-327
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\loki-mode\benchmarks\results\2026-01-05-00-49-17\humaneval-solutions\162.py:11`
- **Evidence:** `return hashlib.md5(text.encode()).hexdigest()`
- **Remediation:** Use SHA-256+; bcrypt/scrypt/argon2 for passwords.

### SEC-213 — Weak Cryptographic Hash [Weak Cryptographic Hash]
- **Severity:** MEDIUM  ·  **CWE:** CWE-327
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\loki-mode\benchmarks\results\humaneval-loki-solutions\162.py:16`
- **Evidence:** `return hashlib.md5(text.encode()).hexdigest()`
- **Remediation:** Use SHA-256+; bcrypt/scrypt/argon2 for passwords.

### SEC-214 — Weak Cryptographic Hash [Weak Cryptographic Hash]
- **Severity:** MEDIUM  ·  **CWE:** CWE-327
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\loki-mode\tests\test-task-queue.sh:360`
- **Evidence:** `idempotency_key = hashlib.md5(json.dumps(new_task['payload'], sort_keys=True).encode()).hexdigest()`
- **Remediation:** Use SHA-256+; bcrypt/scrypt/argon2 for passwords.

### SEC-215 — Insecure Randomness [Insecure Randomness]
- **Severity:** MEDIUM  ·  **CWE:** CWE-330
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\notebooklm\scripts\browser_utils.py:88`
- **Evidence:** `if random.random() < 0.05:`
- **Remediation:** Use a CSPRNG (crypto.randomBytes, secrets, getrandom) for security tokens.

### SEC-220 — Cross-Site Scripting [Cross-Site Scripting]
- **Severity:** MEDIUM  ·  **CWE:** CWE-79
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\vulnerability-scanner\scripts\security_scan.py:70`
- **Evidence:** `(r'dangerouslySetInnerHTML', "dangerouslySetInnerHTML", "high", "XSS risk"),`
- **Remediation:** Escape/encode output; prefer textContent or a sanitizer (DOMPurify).

### SEC-222 — Insecure Randomness [Insecure Randomness]
- **Severity:** MEDIUM  ·  **CWE:** CWE-330
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\apps\stage-tamagotchi\src\main\services\airi\plugins\index.test.ts:302`
- **Evidence:** `id: payload.id ?? Math.random().toString(36).slice(2, 10),`
- **Remediation:** Use a CSPRNG (crypto.randomBytes, secrets, getrandom) for security tokens.

### SEC-223 — Insecure Randomness [Insecure Randomness]
- **Severity:** MEDIUM  ·  **CWE:** CWE-330
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\apps\stage-tamagotchi\src\main\services\airi\plugins\index.test.ts:1133`
- **Evidence:** `id: payload.id ?? Math.random().toString(36).slice(2, 10),`
- **Remediation:** Use a CSPRNG (crypto.randomBytes, secrets, getrandom) for security tokens.

### SEC-224 — Insecure Randomness [Insecure Randomness]
- **Severity:** MEDIUM  ·  **CWE:** CWE-330
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\apps\stage-tamagotchi\src\main\services\airi\plugins\kits\gamelet\index.ts:117`
- **Evidence:** `const random = globalThis.crypto?.randomUUID?.() ?? Math.random().toString(36).slice(2, 12)`
- **Remediation:** Use a CSPRNG (crypto.randomBytes, secrets, getrandom) for security tokens.

### SEC-225 — Insecure Randomness [Insecure Randomness]
- **Severity:** MEDIUM  ·  **CWE:** CWE-330
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\apps\stage-tamagotchi\src\main\services\airi\widgets\artistry-bridge.ts:80`
- **Evidence:** `return `${widgetId}:${Date.now()}:${Math.random().toString(36).slice(2, 8)}``
- **Remediation:** Use a CSPRNG (crypto.randomBytes, secrets, getrandom) for security tokens.

### SEC-226 — Insecure Randomness [Insecure Randomness]
- **Severity:** MEDIUM  ·  **CWE:** CWE-330
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\apps\stage-tamagotchi\src\main\services\airi\widgets\providers\comfyui.ts:65`
- **Evidence:** `const jobId = request.extra?.internalJobId || Math.random().toString(36).slice(2)`
- **Remediation:** Use a CSPRNG (crypto.randomBytes, secrets, getrandom) for security tokens.

### SEC-227 — Insecure Randomness [Insecure Randomness]
- **Severity:** MEDIUM  ·  **CWE:** CWE-330
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\apps\stage-tamagotchi\src\main\services\airi\widgets\providers\comfyui.ts:331`
- **Evidence:** `node.inputs.seed = Math.floor(Math.random() * 1e15)`
- **Remediation:** Use a CSPRNG (crypto.randomBytes, secrets, getrandom) for security tokens.

### SEC-228 — Insecure Randomness [Insecure Randomness]
- **Severity:** MEDIUM  ·  **CWE:** CWE-330
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\apps\stage-tamagotchi\src\main\services\airi\widgets\providers\replicate.ts:122`
- **Evidence:** `const jobId = request.extra?.internalJobId || Math.random().toString(36).slice(2)`
- **Remediation:** Use a CSPRNG (crypto.randomBytes, secrets, getrandom) for security tokens.

### SEC-229 — Insecure Randomness [Insecure Randomness]
- **Severity:** MEDIUM  ·  **CWE:** CWE-330
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\apps\stage-tamagotchi\src\main\windows\shared\referenced-window.ts:67`
- **Evidence:** `const id = payload.id ?? Math.random().toString(36).slice(2, 10)`
- **Remediation:** Use a CSPRNG (crypto.randomBytes, secrets, getrandom) for security tokens.

### SEC-230 — Insecure Randomness [Insecure Randomness]
- **Severity:** MEDIUM  ·  **CWE:** CWE-330
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\apps\stage-tamagotchi\src\main\windows\widgets\index.ts:353`
- **Evidence:** `const id = options?.id ?? Math.random().toString(36).slice(2, 10)`
- **Remediation:** Use a CSPRNG (crypto.randomBytes, secrets, getrandom) for security tokens.

### SEC-231 — Insecure Randomness [Insecure Randomness]
- **Severity:** MEDIUM  ·  **CWE:** CWE-330
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\apps\stage-tamagotchi\src\renderer\bridges\stage-three-runtime-trace.ts:28`
- **Evidence:** `const instanceId = Math.random().toString(36).slice(2, 10)`
- **Remediation:** Use a CSPRNG (crypto.randomBytes, secrets, getrandom) for security tokens.

### SEC-232 — Insecure Randomness [Insecure Randomness]
- **Severity:** MEDIUM  ·  **CWE:** CWE-330
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\apps\stage-tamagotchi\src\renderer\pages\settings\modules\mcp-config.ts:27`
- **Evidence:** `return `mcp-${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 8)}``
- **Remediation:** Use a CSPRNG (crypto.randomBytes, secrets, getrandom) for security tokens.

### SEC-233 — Insecure Randomness [Insecure Randomness]
- **Severity:** MEDIUM  ·  **CWE:** CWE-330
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\apps\stage-tamagotchi\src\renderer\stores\chat-sync.ts:75`
- **Evidence:** `return `${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 10)}``
- **Remediation:** Use a CSPRNG (crypto.randomBytes, secrets, getrandom) for security tokens.

### SEC-234 — Insecure Randomness [Insecure Randomness]
- **Severity:** MEDIUM  ·  **CWE:** CWE-330
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\apps\stage-tamagotchi\src\renderer\stores\chat-sync.ts:161`
- **Evidence:** `const instanceId = `${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 10)}``
- **Remediation:** Use a CSPRNG (crypto.randomBytes, secrets, getrandom) for security tokens.

### SEC-235 — Insecure Randomness [Insecure Randomness]
- **Severity:** MEDIUM  ·  **CWE:** CWE-330
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\pipelines-audio\src\processors\tts-chunker.ts:488`
- **Evidence:** `segmentId: `${meta.streamId}:${Date.now()}:${Math.random().toString(36).slice(2, 8)}`,`
- **Remediation:** Use a CSPRNG (crypto.randomBytes, secrets, getrandom) for security tokens.

### SEC-236 — Insecure Randomness [Insecure Randomness]
- **Severity:** MEDIUM  ·  **CWE:** CWE-330
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\pipelines-audio\src\speech-pipeline.ts:60`
- **Evidence:** `return `${prefix}-${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 8)}``
- **Remediation:** Use a CSPRNG (crypto.randomBytes, secrets, getrandom) for security tokens.

### SEC-237 — Insecure Randomness [Insecure Randomness]
- **Severity:** MEDIUM  ·  **CWE:** CWE-330
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\server-sdk\src\client.ts:83`
- **Evidence:** `return `${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 8)}``
- **Remediation:** Use a CSPRNG (crypto.randomBytes, secrets, getrandom) for security tokens.

### SEC-238 — Insecure Randomness [Insecure Randomness]
- **Severity:** MEDIUM  ·  **CWE:** CWE-330
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\server-sdk\src\client.ts:87`
- **Evidence:** `return `${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 10)}``
- **Remediation:** Use a CSPRNG (crypto.randomBytes, secrets, getrandom) for security tokens.

### SEC-239 — Insecure Randomness [Insecure Randomness]
- **Severity:** MEDIUM  ·  **CWE:** CWE-330
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\stage-ui\src\composables\use-lamp-flicker-animation.ts:20`
- **Evidence:** `flickerDuration.value = `${(5.8 + Math.random() * 1.8).toFixed(2)}s``
- **Remediation:** Use a CSPRNG (crypto.randomBytes, secrets, getrandom) for security tokens.

### SEC-240 — Insecure Randomness [Insecure Randomness]
- **Severity:** MEDIUM  ·  **CWE:** CWE-330
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\stage-ui\src\composables\use-lamp-flicker-animation.ts:23`
- **Evidence:** `flickerDelay.value = `${(-Math.random() * 5.4).toFixed(2)}s``
- **Remediation:** Use a CSPRNG (crypto.randomBytes, secrets, getrandom) for security tokens.

### SEC-241 — Insecure Randomness [Insecure Randomness]
- **Severity:** MEDIUM  ·  **CWE:** CWE-330
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\stage-ui\src\libs\chat-sync\ws-client.ts:130`
- **Evidence:** `return Math.floor(exp * 0.5 + Math.random() * exp * 0.5)`
- **Remediation:** Use a CSPRNG (crypto.randomBytes, secrets, getrandom) for security tokens.

### SEC-242 — Insecure Randomness [Insecure Randomness]
- **Severity:** MEDIUM  ·  **CWE:** CWE-330
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\stage-ui\src\stores\perf-tracer-bridge.ts:38`
- **Evidence:** `const instanceId = Math.random().toString(36).slice(2, 10)`
- **Remediation:** Use a CSPRNG (crypto.randomBytes, secrets, getrandom) for security tokens.

### SEC-243 — Insecure Randomness [Insecure Randomness]
- **Severity:** MEDIUM  ·  **CWE:** CWE-330
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\stage-ui\src\tools\debug.ts:11`
- **Evidence:** `resolve(Math.random().toString())`
- **Remediation:** Use a CSPRNG (crypto.randomBytes, secrets, getrandom) for security tokens.

### SEC-244 — Insecure Randomness [Insecure Randomness]
- **Severity:** MEDIUM  ·  **CWE:** CWE-330
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\stage-ui-live2d\src\composables\live2d\motion-manager.ts:257`
- **Evidence:** `blinkState.delayMs = minDelay + Math.random() * (maxDelay - minDelay)`
- **Remediation:** Use a CSPRNG (crypto.randomBytes, secrets, getrandom) for security tokens.

### SEC-245 — Insecure Randomness [Insecure Randomness]
- **Severity:** MEDIUM  ·  **CWE:** CWE-330
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\stage-ui-live2d\src\utils\eye-motions.ts:25`
- **Evidence:** `const r = Math.random()`
- **Remediation:** Use a CSPRNG (crypto.randomBytes, secrets, getrandom) for security tokens.

### SEC-246 — Insecure Randomness [Insecure Randomness]
- **Severity:** MEDIUM  ·  **CWE:** CWE-330
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\stage-ui-live2d\src\utils\eye-motions.ts:28`
- **Evidence:** `return EYE_SACCADE_INT_P[i][1] + Math.random() * EYE_SACCADE_INT_STEP`
- **Remediation:** Use a CSPRNG (crypto.randomBytes, secrets, getrandom) for security tokens.

### SEC-248 — Cross-Site Scripting [Cross-Site Scripting]
- **Severity:** MEDIUM  ·  **CWE:** CWE-79
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\services\minecraft\src\debug\web\app.js:254`
- **Evidence:** `this.elements.queueList.innerHTML = queue.map((item, idx) => ``
- **Remediation:** Escape/encode output; prefer textContent or a sanitizer (DOMPurify).

### SEC-249 — Cross-Site Scripting [Cross-Site Scripting]
- **Severity:** MEDIUM  ·  **CWE:** CWE-79
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\services\minecraft\src\debug\web\app.js:263`
- **Evidence:** `this.elements.queueList.innerHTML = '<div class="empty-state">Queue empty</div>'`
- **Remediation:** Escape/encode output; prefer textContent or a sanitizer (DOMPurify).

### SEC-250 — Cross-Site Scripting [Cross-Site Scripting]
- **Severity:** MEDIUM  ·  **CWE:** CWE-79
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\services\minecraft\src\debug\web\app.js:268`
- **Evidence:** `this.elements.processingContent.innerHTML = ``
- **Remediation:** Escape/encode output; prefer textContent or a sanitizer (DOMPurify).

### SEC-251 — Cross-Site Scripting [Cross-Site Scripting]
- **Severity:** MEDIUM  ·  **CWE:** CWE-79
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\services\minecraft\src\debug\web\app.js:278`
- **Evidence:** `this.elements.processingContent.innerHTML = '<span style="color: var(--text-muted);">Idle</span>'`
- **Remediation:** Escape/encode output; prefer textContent or a sanitizer (DOMPurify).

### SEC-252 — Cross-Site Scripting [Cross-Site Scripting]
- **Severity:** MEDIUM  ·  **CWE:** CWE-79
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\services\minecraft\src\debug\web\app.js:492`
- **Evidence:** `select.innerHTML = `<option value="">Live (current session)</option>${files.map(f => `<option value="${f}">${f}</option>`).join('')}``
- **Remediation:** Escape/encode output; prefer textContent or a sanitizer (DOMPurify).

### SEC-253 — Cross-Site Scripting [Cross-Site Scripting]
- **Severity:** MEDIUM  ·  **CWE:** CWE-79
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\services\minecraft\src\debug\web\app.js:545`
- **Evidence:** `this.elements.container.innerHTML = filtered.map((log) => {`
- **Remediation:** Escape/encode output; prefer textContent or a sanitizer (DOMPurify).

### SEC-254 — Cross-Site Scripting [Cross-Site Scripting]
- **Severity:** MEDIUM  ·  **CWE:** CWE-79
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\services\minecraft\src\debug\web\app.js:690`
- **Evidence:** `this.elements.container.innerHTML = html + typing`
- **Remediation:** Escape/encode output; prefer textContent or a sanitizer (DOMPurify).

### SEC-255 — Cross-Site Scripting [Cross-Site Scripting]
- **Severity:** MEDIUM  ·  **CWE:** CWE-79
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\services\minecraft\src\debug\web\app.js:963`
- **Evidence:** `this.elements.grid.innerHTML = '<div class="empty-state">Loading tools...</div>'`
- **Remediation:** Escape/encode output; prefer textContent or a sanitizer (DOMPurify).

### SEC-256 — Cross-Site Scripting [Cross-Site Scripting]
- **Severity:** MEDIUM  ·  **CWE:** CWE-79
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\services\minecraft\src\debug\web\app.js:975`
- **Evidence:** `this.elements.grid.innerHTML = '<div class="empty-state">No tools match filter</div>'`
- **Remediation:** Escape/encode output; prefer textContent or a sanitizer (DOMPurify).

### SEC-257 — Cross-Site Scripting [Cross-Site Scripting]
- **Severity:** MEDIUM  ·  **CWE:** CWE-79
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\services\minecraft\src\debug\web\app.js:982`
- **Evidence:** `this.elements.grid.innerHTML = filtered.map(tool => this.renderCard(tool)).join('')`
- **Remediation:** Escape/encode output; prefer textContent or a sanitizer (DOMPurify).

### SEC-258 — Cross-Site Scripting [Cross-Site Scripting]
- **Severity:** MEDIUM  ·  **CWE:** CWE-79
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\services\minecraft\src\debug\web\app.js:1176`
- **Evidence:** `resultEl.innerHTML = ``
- **Remediation:** Escape/encode output; prefer textContent or a sanitizer (DOMPurify).

### SEC-259 — Cross-Site Scripting [Cross-Site Scripting]
- **Severity:** MEDIUM  ·  **CWE:** CWE-79
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\services\minecraft\src\debug\web\app.js:1293`
- **Evidence:** `this.elements.varsList.innerHTML = '<div class="empty-state">No variables loaded</div>'`
- **Remediation:** Escape/encode output; prefer textContent or a sanitizer (DOMPurify).

### SEC-260 — Cross-Site Scripting [Cross-Site Scripting]
- **Severity:** MEDIUM  ·  **CWE:** CWE-79
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\services\minecraft\src\debug\web\app.js:1305`
- **Evidence:** `this.elements.varsList.innerHTML = '<div class="empty-state">No variables match filter</div>'`
- **Remediation:** Escape/encode output; prefer textContent or a sanitizer (DOMPurify).

### SEC-261 — Cross-Site Scripting [Cross-Site Scripting]
- **Severity:** MEDIUM  ·  **CWE:** CWE-79
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\services\minecraft\src\debug\web\app.js:1309`
- **Evidence:** `this.elements.varsList.innerHTML = filtered.map(v => ``
- **Remediation:** Escape/encode output; prefer textContent or a sanitizer (DOMPurify).

### SEC-262 — Cross-Site Scripting [Cross-Site Scripting]
- **Severity:** MEDIUM  ·  **CWE:** CWE-79
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\services\minecraft\src\debug\web\app.js:1323`
- **Evidence:** `this.elements.resultList.innerHTML = '<div class="empty-state">No REPL executions yet</div>'`
- **Remediation:** Escape/encode output; prefer textContent or a sanitizer (DOMPurify).

### SEC-263 — Cross-Site Scripting [Cross-Site Scripting]
- **Severity:** MEDIUM  ·  **CWE:** CWE-79
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\services\minecraft\src\debug\web\app.js:1327`
- **Evidence:** `this.elements.resultList.innerHTML = this.results.map((result) => {`
- **Remediation:** Escape/encode output; prefer textContent or a sanitizer (DOMPurify).

### SEC-264 — Cross-Site Scripting [Cross-Site Scripting]
- **Severity:** MEDIUM  ·  **CWE:** CWE-79
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\services\minecraft\src\debug\web\app.js:1447`
- **Evidence:** `this.elements.list.innerHTML = '<div class="empty-state">No events</div>'`
- **Remediation:** Escape/encode output; prefer textContent or a sanitizer (DOMPurify).

### SEC-265 — Cross-Site Scripting [Cross-Site Scripting]
- **Severity:** MEDIUM  ·  **CWE:** CWE-79
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\services\minecraft\src\debug\web\app.js:1451`
- **Evidence:** `this.elements.list.innerHTML = recent.map(e => this.renderEvent(e)).join('')`
- **Remediation:** Escape/encode output; prefer textContent or a sanitizer (DOMPurify).

### SEC-266 — Cross-Site Scripting [Cross-Site Scripting]
- **Severity:** MEDIUM  ·  **CWE:** CWE-79
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\services\minecraft\src\debug\web\app.js:1529`
- **Evidence:** `this.elements.detailContent.innerHTML = this.renderEventTree(tree)`
- **Remediation:** Escape/encode output; prefer textContent or a sanitizer (DOMPurify).

### SEC-268 — Cross-Site Scripting [Cross-Site Scripting]
- **Severity:** MEDIUM  ·  **CWE:** CWE-79
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\app\src\addons\serialize.test.ts:17`
- **Evidence:** `document.body.innerHTML = ""`
- **Remediation:** Escape/encode output; prefer textContent or a sanitizer (DOMPurify).

### SEC-269 — Cross-Site Scripting [Cross-Site Scripting]
- **Severity:** MEDIUM  ·  **CWE:** CWE-79
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\app\src\components\file-tree.tsx:99`
- **Evidence:** `image.innerHTML = (icon as SVGElement).outerHTML + (text as HTMLSpanElement).outerHTML`
- **Remediation:** Escape/encode output; prefer textContent or a sanitizer (DOMPurify).

### SEC-270 — Cross-Site Scripting [Cross-Site Scripting]
- **Severity:** MEDIUM  ·  **CWE:** CWE-79
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\app\src\components\prompt-input.tsx:487`
- **Evidence:** `editorRef.innerHTML = ""`
- **Remediation:** Escape/encode output; prefer textContent or a sanitizer (DOMPurify).

### SEC-271 — Cross-Site Scripting [Cross-Site Scripting]
- **Severity:** MEDIUM  ·  **CWE:** CWE-79
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\app\src\pages\session\helpers.test.ts:66`
- **Evidence:** `document.body.innerHTML = `<div id="terminal-wrapper-one"><div data-component="terminal"><textarea></textarea></div></div>``
- **Remediation:** Escape/encode output; prefer textContent or a sanitizer (DOMPurify).

### SEC-272 — Cross-Site Scripting [Cross-Site Scripting]
- **Severity:** MEDIUM  ·  **CWE:** CWE-79
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\app\src\pages\session\helpers.test.ts:75`
- **Evidence:** `document.body.innerHTML = `<div id="terminal-wrapper-two"><div data-component="terminal" tabindex="0"></div></div>``
- **Remediation:** Escape/encode output; prefer textContent or a sanitizer (DOMPurify).

### SEC-326 — Weak Cryptographic Hash [Weak Cryptographic Hash]
- **Severity:** MEDIUM  ·  **CWE:** CWE-327
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\ds4\ds4_server.c:4545`
- **Evidence:** `* The filename is SHA1(token ids), not SHA1(text).  The text field is only for`
- **Remediation:** Use SHA-256+; bcrypt/scrypt/argon2 for passwords.

### SEC-327 — Weak Cryptographic Hash [Weak Cryptographic Hash]
- **Severity:** MEDIUM  ·  **CWE:** CWE-327
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\geometry-of-consolidation\data\build_drm.py:134`
- **Evidence:** `ids.append(f"drm::{hashlib.md5(key.encode()).hexdigest()[:8]}::{j}")`
- **Remediation:** Use SHA-256+; bcrypt/scrypt/argon2 for passwords.

### SEC-358 — Weak Cryptographic Hash [Weak Cryptographic Hash]
- **Severity:** MEDIUM  ·  **CWE:** CWE-327
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\llama.cpp\examples\gguf-hash\deps\sha1\sha1.c:282`
- **Evidence:** `void SHA1(`
- **Remediation:** Use SHA-256+; bcrypt/scrypt/argon2 for passwords.

### SEC-359 — Weak Cryptographic Hash [Weak Cryptographic Hash]
- **Severity:** MEDIUM  ·  **CWE:** CWE-327
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\llama.cpp\examples\gguf-hash\gguf-hash.cpp:385`
- **Evidence:** `SHA1( result, (const char *)raw_data, n_bytes);`
- **Remediation:** Use SHA-256+; bcrypt/scrypt/argon2 for passwords.

### SEC-360 — Weak Cryptographic Hash [Weak Cryptographic Hash]
- **Severity:** MEDIUM  ·  **CWE:** CWE-327
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\llama.cpp\gguf-py\gguf\scripts\gguf_hash.py:31`
- **Evidence:** `sha1 = hashlib.sha1()`
- **Remediation:** Use SHA-256+; bcrypt/scrypt/argon2 for passwords.

### SEC-361 — Weak Cryptographic Hash [Weak Cryptographic Hash]
- **Severity:** MEDIUM  ·  **CWE:** CWE-327
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\llama.cpp\gguf-py\gguf\scripts\gguf_hash.py:33`
- **Evidence:** `uuidv5_sha1 = hashlib.sha1()`
- **Remediation:** Use SHA-256+; bcrypt/scrypt/argon2 for passwords.

### SEC-362 — Weak Cryptographic Hash [Weak Cryptographic Hash]
- **Severity:** MEDIUM  ·  **CWE:** CWE-327
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\llama.cpp\gguf-py\gguf\scripts\gguf_hash.py:68`
- **Evidence:** `sha1_layer = hashlib.sha1()`
- **Remediation:** Use SHA-256+; bcrypt/scrypt/argon2 for passwords.

### SEC-390 — Weak Cryptographic Hash [Weak Cryptographic Hash]
- **Severity:** MEDIUM  ·  **CWE:** CWE-327
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\mcp-servers\browser-agent\.venv\Lib\site-packages\cryptography\hazmat\backends\openssl\backend.py:114`
- **Evidence:** `# signatures, e.g. RSA PKCS#1 v1.5 SHA1 (sha1WithRSAEncryption).`
- **Remediation:** Use SHA-256+; bcrypt/scrypt/argon2 for passwords.

### SEC-391 — Weak Cryptographic Hash [Weak Cryptographic Hash]
- **Severity:** MEDIUM  ·  **CWE:** CWE-327
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\mcp-servers\browser-agent\.venv\Lib\site-packages\cryptography\hazmat\primitives\hashes.py:101`
- **Evidence:** `class SHA1(HashAlgorithm):`
- **Remediation:** Use SHA-256+; bcrypt/scrypt/argon2 for passwords.

### SEC-392 — Weak Cryptographic Hash [Weak Cryptographic Hash]
- **Severity:** MEDIUM  ·  **CWE:** CWE-327
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\mcp-servers\browser-agent\.venv\Lib\site-packages\cryptography\hazmat\primitives\hashes.py:203`
- **Evidence:** `class MD5(HashAlgorithm):`
- **Remediation:** Use SHA-256+; bcrypt/scrypt/argon2 for passwords.

### SEC-393 — Weak Cryptographic Hash [Weak Cryptographic Hash]
- **Severity:** MEDIUM  ·  **CWE:** CWE-327
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\mcp-servers\browser-agent\.venv\Lib\site-packages\cryptography\hazmat\primitives\serialization\ssh.py:1009`
- **Evidence:** `hash_alg = hashes.SHA1()`
- **Remediation:** Use SHA-256+; bcrypt/scrypt/argon2 for passwords.

### SEC-394 — Weak Cryptographic Hash [Weak Cryptographic Hash]
- **Severity:** MEDIUM  ·  **CWE:** CWE-327
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\mcp-servers\browser-agent\.venv\Lib\site-packages\cryptography\hazmat\_oid.py:128`
- **Evidence:** `SignatureAlgorithmOID.RSA_WITH_MD5: hashes.MD5(),`
- **Remediation:** Use SHA-256+; bcrypt/scrypt/argon2 for passwords.

### SEC-395 — Weak Cryptographic Hash [Weak Cryptographic Hash]
- **Severity:** MEDIUM  ·  **CWE:** CWE-327
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\mcp-servers\browser-agent\.venv\Lib\site-packages\cryptography\hazmat\_oid.py:129`
- **Evidence:** `SignatureAlgorithmOID.RSA_WITH_SHA1: hashes.SHA1(),`
- **Remediation:** Use SHA-256+; bcrypt/scrypt/argon2 for passwords.

### SEC-396 — Weak Cryptographic Hash [Weak Cryptographic Hash]
- **Severity:** MEDIUM  ·  **CWE:** CWE-327
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\mcp-servers\browser-agent\.venv\Lib\site-packages\cryptography\hazmat\_oid.py:130`
- **Evidence:** `SignatureAlgorithmOID._RSA_WITH_SHA1: hashes.SHA1(),`
- **Remediation:** Use SHA-256+; bcrypt/scrypt/argon2 for passwords.

### SEC-397 — Weak Cryptographic Hash [Weak Cryptographic Hash]
- **Severity:** MEDIUM  ·  **CWE:** CWE-327
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\mcp-servers\browser-agent\.venv\Lib\site-packages\cryptography\hazmat\_oid.py:139`
- **Evidence:** `SignatureAlgorithmOID.ECDSA_WITH_SHA1: hashes.SHA1(),`
- **Remediation:** Use SHA-256+; bcrypt/scrypt/argon2 for passwords.

### SEC-398 — Weak Cryptographic Hash [Weak Cryptographic Hash]
- **Severity:** MEDIUM  ·  **CWE:** CWE-327
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\mcp-servers\browser-agent\.venv\Lib\site-packages\cryptography\hazmat\_oid.py:148`
- **Evidence:** `SignatureAlgorithmOID.DSA_WITH_SHA1: hashes.SHA1(),`
- **Remediation:** Use SHA-256+; bcrypt/scrypt/argon2 for passwords.

### SEC-399 — Weak Cryptographic Hash [Weak Cryptographic Hash]
- **Severity:** MEDIUM  ·  **CWE:** CWE-327
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\mcp-servers\browser-agent\.venv\Lib\site-packages\cryptography\x509\extensions.py:72`
- **Evidence:** `return hashlib.sha1(data).digest()`
- **Remediation:** Use SHA-256+; bcrypt/scrypt/argon2 for passwords.

### SEC-400 — Weak Cryptographic Hash [Weak Cryptographic Hash]
- **Severity:** MEDIUM  ·  **CWE:** CWE-327
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\mcp-servers\browser-agent\.venv\Lib\site-packages\httpx\_auth.py:309`
- **Evidence:** `return hashlib.sha1(s).hexdigest()[:16].encode()`
- **Remediation:** Use SHA-256+; bcrypt/scrypt/argon2 for passwords.

### SEC-001 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\agent-security-review\sub-skills\1-secrets-management.md:6`
- **Evidence:** `Pass…123"  (24 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-002 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\api-documentation-generator\sub-skills\example-1-rest-api-endpoint-documentation.md:77`
- **Evidence:** `pass…23!'  (26 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-003 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\api-fuzzing-bug-bounty\sub-skills\graphql-sqlnosql-injection.md:7`
- **Evidence:** `pass…ord"  (20 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-004 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\api-fuzzing-bug-bounty\sub-skills\rate-limit-bypass-batching.md:4`
- **Evidence:** `pass…ord"  (19 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-005 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\api-fuzzing-bug-bounty\sub-skills\rate-limit-bypass-batching.md:5`
- **Evidence:** `pass…ord"  (19 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-006 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\api-fuzzing-bug-bounty\sub-skills\rate-limit-bypass-batching.md:6`
- **Evidence:** `pass…ord"  (19 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-007 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\bun-development\sub-skills\55-password-hashing.md:5`
- **Evidence:** `pass…ret"  (25 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-008 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\deployment-validation-config-validate\sub-skills\4-configuration-testing.md:20`
- **Evidence:** `pass…123'  (25 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-009 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\deployment-validation-config-validate\sub-skills\4-configuration-testing.md:33`
- **Evidence:** `pass…123'  (25 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-010 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\e2e-testing-patterns\sub-skills\implementation-playbook.md:161`
- **Evidence:** `pass…!@#'  (22 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-012 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\javascript-testing-patterns\sub-skills\implementation-playbook.md:516`
- **Evidence:** `pass…123'  (23 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-013 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\javascript-testing-patterns\sub-skills\implementation-playbook.md:536`
- **Evidence:** `pass…123'  (23 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-014 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\javascript-testing-patterns\sub-skills\implementation-playbook.md:551`
- **Evidence:** `pass…123'  (23 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-015 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\javascript-testing-patterns\sub-skills\implementation-playbook.md:572`
- **Evidence:** `pass…123'  (23 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-016 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\javascript-testing-patterns\sub-skills\implementation-playbook.md:609`
- **Evidence:** `pass…123'  (23 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-017 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\javascript-testing-patterns\sub-skills\implementation-playbook.md:616`
- **Evidence:** `pass…123'  (23 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-018 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\javascript-testing-patterns\sub-skills\implementation-playbook.md:650`
- **Evidence:** `pass…ord'  (25 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-019 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\javascript-testing-patterns\sub-skills\implementation-playbook.md:680`
- **Evidence:** `pass…ord'  (27 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-020 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\javascript-testing-patterns\sub-skills\implementation-playbook.md:692`
- **Evidence:** `pass…ord'  (27 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-022 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\k8s-manifest-generator\sub-skills\implementation-playbook.md:206`
- **Evidence:** `PASS…eme"  (20 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-024 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\playwright-skill\lib\helpers.js:208`
- **Evidence:** `pass…me="  (23 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-027 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\sql-injection-testing\sub-skills\example-3-login-bypass.md:18`
- **Evidence:** `pass…ing'  (19 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-028 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\.agent\skills\sql-injection-testing\sub-skills\phase-3-authentication-bypass.md:27`
- **Evidence:** `pass…ing'  (19 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-031 — Hardcoded secret: generic_api_key [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\stage-ui\src\libs\providers\providers\aihubmix\index.ts:31`
- **Evidence:** `apiK…hema  (28 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-032 — Hardcoded secret: generic_api_key [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\stage-ui\src\libs\providers\providers\amazon-bedrock\index.ts:197`
- **Evidence:** `apiK…hema  (33 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-033 — Hardcoded secret: generic_api_key [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\stage-ui\src\libs\providers\providers\anthropic\index.ts:52`
- **Evidence:** `apiK…hema  (29 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-034 — Hardcoded secret: generic_api_key [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\stage-ui\src\libs\providers\providers\ark-shared.ts:69`
- **Evidence:** `apiK…hema  (31 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-035 — Hardcoded secret: generic_api_key [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\stage-ui\src\libs\providers\providers\azure-ai-foundry\index.ts:28`
- **Evidence:** `apiK…hema  (34 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-036 — Hardcoded secret: generic_api_key [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\stage-ui\src\libs\providers\providers\azure-openai\index.ts:177`
- **Evidence:** `apiK…hema  (31 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-037 — Hardcoded secret: generic_api_key [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\stage-ui\src\libs\providers\providers\cerebras-ai\index.ts:30`
- **Evidence:** `apiK…hema  (28 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-038 — Hardcoded secret: generic_api_key [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\stage-ui\src\libs\providers\providers\comet-api\index.ts:30`
- **Evidence:** `apiK…hema  (28 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-039 — Hardcoded secret: generic_api_key [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\stage-ui\src\libs\providers\providers\deepseek\index.ts:31`
- **Evidence:** `apiK…hema  (28 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-040 — Hardcoded secret: generic_api_key [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\stage-ui\src\libs\providers\providers\featherless-ai\index.ts:29`
- **Evidence:** `apiK…hema  (31 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-041 — Hardcoded secret: generic_api_key [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\stage-ui\src\libs\providers\providers\fireworks-ai\index.ts:30`
- **Evidence:** `apiK…hema  (29 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-042 — Hardcoded secret: generic_api_key [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\stage-ui\src\libs\providers\providers\google-generative-ai\index.ts:31`
- **Evidence:** `apiK…hema  (36 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-043 — Hardcoded secret: generic_api_key [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\stage-ui\src\libs\providers\providers\lm-studio\index.ts:32`
- **Evidence:** `apiK…hema  (28 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-044 — Hardcoded secret: generic_api_key [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\stage-ui\src\libs\providers\providers\minimax\index.ts:94`
- **Evidence:** `apiK…hema  (29 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-045 — Hardcoded secret: generic_api_key [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\stage-ui\src\libs\providers\providers\minimax\index.ts:134`
- **Evidence:** `apiK…hema  (33 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-046 — Hardcoded secret: generic_api_key [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\stage-ui\src\libs\providers\providers\modelscope\index.ts:30`
- **Evidence:** `apiK…hema  (30 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-047 — Hardcoded secret: generic_api_key [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\stage-ui\src\libs\providers\providers\moonshot-ai\index.ts:29`
- **Evidence:** `apiK…hema  (28 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-048 — Hardcoded secret: generic_api_key [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\stage-ui\src\libs\providers\providers\openai\index.ts:30`
- **Evidence:** `apiK…hema  (36 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-049 — Hardcoded secret: generic_api_key [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\stage-ui\src\libs\providers\providers\openai-compatible\index.ts:31`
- **Evidence:** `apiK…hema  (36 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-050 — Hardcoded secret: generic_api_key [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\stage-ui\src\libs\providers\providers\openrouter-ai\index.ts:35`
- **Evidence:** `apiK…hema  (30 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-051 — Hardcoded secret: generic_api_key [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\stage-ui\src\libs\providers\providers\perplexity-ai\index.ts:32`
- **Evidence:** `apiK…hema  (30 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-052 — Hardcoded secret: generic_api_key [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\stage-ui\src\libs\providers\providers\together-ai\index.ts:30`
- **Evidence:** `apiK…hema  (28 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-053 — Hardcoded secret: generic_api_key [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\stage-ui\src\stores\modules\artistry.ts:48`
- **Evidence:** `ApiK…eset  (35 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-054 — Hardcoded secret: generic_api_key [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\stage-ui\src\stores\modules\artistry.ts:63`
- **Evidence:** `ApiK…eset  (35 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-055 — Hardcoded secret: generic_api_key [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\stage-ui\src\stores\modules\twitter.ts:11`
- **Evidence:** `apiK…eset  (35 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-056 — Hardcoded secret: generic_api_key [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\packages\stage-ui\src\stores\modules\twitter.ts:13`
- **Evidence:** `acce…eset  (40 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-057 — Hardcoded secret: generic_api_key [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\services\computer-use-mcp\src\e2e\provider-bootstrap.ts:74`
- **Evidence:** `apiK…iKey  (34 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-058 — Hardcoded secret: generic_api_key [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\airi\services\computer-use-mcp\src\e2e\provider-bootstrap.ts:86`
- **Evidence:** `apiK…iKey  (38 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-063 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\kilo-jetbrains\backend\bin\test\ai\kilocode\backend\KiloBackendHttpClientsTest.kt:15`
- **Evidence:** `pwd …123"  (17 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-064 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\kilo-jetbrains\backend\bin\test\ai\kilocode\backend\KiloBackendHttpClientsTest.kt:72`
- **Evidence:** `pwd …pwd"  (17 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-065 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\kilo-jetbrains\backend\bin\test\ai\kilocode\backend\testing\MockCliServer.kt:28`
- **Evidence:** `pass…ord"  (26 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-066 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\kilo-jetbrains\backend\src\test\kotlin\ai\kilocode\backend\KiloBackendHttpClientsTest.kt:15`
- **Evidence:** `pwd …123"  (17 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-067 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\kilo-jetbrains\backend\src\test\kotlin\ai\kilocode\backend\KiloBackendHttpClientsTest.kt:72`
- **Evidence:** `pwd …pwd"  (17 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-068 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\kilo-jetbrains\backend\src\test\kotlin\ai\kilocode\backend\testing\MockCliServer.kt:28`
- **Evidence:** `pass…ord"  (26 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-069 — Hardcoded secret: generic_api_key [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\kilo-telemetry\src\client.ts:5`
- **Evidence:** `API_…B6z"  (59 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-070 — Hardcoded secret: generic_api_key [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\kilo-vscode\webview-ui\src\components\settings\CustomProviderDialog.tsx:286`
- **Evidence:** `apiK…rKey  (32 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-074 — Hardcoded secret: generic_api_key [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\opencode\test\provider\gitlab-duo.test.ts:156`
- **Evidence:** `apiK…ken"  (30 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-077 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\.tmp\plugins\plugins\base44\skills\base44-sdk\references\auth.md:160`
- **Evidence:** `pass…123"  (29 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-078 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\.tmp\plugins\plugins\base44\skills\base44-sdk\references\auth.md:451`
- **Evidence:** `Pass…123"  (32 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-079 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\.tmp\plugins\plugins\base44\skills\base44-sdk\references\auth.md:478`
- **Evidence:** `Pass…123"  (26 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-080 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\.tmp\plugins\plugins\base44\skills\base44-sdk\references\auth.md:479`
- **Evidence:** `Pass…456"  (32 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-089 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\.tmp\plugins\plugins\nvidia\skills\physical-ai-infrastructure-setup-and-resilient-scaling\components\inference-nim-operator\scripts\install.sh:41`
- **Evidence:** `pass…EY}"  (25 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-090 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\.tmp\plugins\plugins\nvidia\skills\physical-ai-infrastructure-setup-and-resilient-scaling\components\inference-nim-operator\scripts\install.sh:96`
- **Evidence:** `pass…EY}"  (25 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-092 — Hardcoded secret: generic_api_key [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\.tmp\plugins\plugins\twilio-developer-kit\skills\twilio-agent-connect\SKILL.md:213`
- **Evidence:** `API_…xxxx  (42 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-093 — Hardcoded secret: generic_api_key [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\.tmp\plugins\plugins\twilio-developer-kit\skills\twilio-iam-auth-setup\SKILL.md:217`
- **Evidence:** `API_…xxxx  (42 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-094 — Hardcoded secret: generic_api_key [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\.tmp\plugins\plugins\twilio-developer-kit\skills\twilio-iam-auth-setup\SKILL.md:222`
- **Evidence:** `AUTH…oken  (31 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-095 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\.tmp\plugins\plugins\zoom\skills\general\use-cases\embed-meetings.md:85`
- **Evidence:** `pass…ord'  (20 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-096 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\.tmp\plugins\plugins\zoom\skills\general\use-cases\embed-meetings.md:110`
- **Evidence:** `pass…ord'  (20 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-097 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\.tmp\plugins\plugins\zoom\skills\meeting-sdk\linux\linux.md:62`
- **Evidence:** `pass…123"  (23 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-098 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\.tmp\plugins\plugins\zoom\skills\meeting-sdk\react-native\examples\join-meeting-pattern.md:11`
- **Evidence:** `pass…ord'  (28 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-099 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\.tmp\plugins\plugins\zoom\skills\meeting-sdk\web\references\web.md:248`
- **Evidence:** `pass…ord'  (28 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-100 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\.tmp\plugins\plugins\zoom\skills\meeting-sdk\web\SKILL.md:215`
- **Evidence:** `pass…ord'  (28 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-101 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\.tmp\plugins\plugins\zoom\skills\meeting-sdk\web\SKILL.md:262`
- **Evidence:** `pass…ord'  (28 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-102 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\.tmp\plugins\plugins\zoom\skills\meeting-sdk\web\troubleshooting\common-issues.md:137`
- **Evidence:** `pass…123'  (22 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-103 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\.tmp\plugins\plugins\zoom\skills\meeting-sdk\web\troubleshooting\common-issues.md:142`
- **Evidence:** `pass…123'  (22 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-104 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\.tmp\plugins\plugins\zoom\skills\video-sdk\linux\linux.md:174`
- **Evidence:** `Pass…ord"  (21 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-105 — Hardcoded secret: generic_password [Hardcoded Credentials]
- **Severity:** LOW  ·  **CWE:** CWE-798
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\codex\.tmp\plugins\plugins\zoom\skills\video-sdk\linux\troubleshooting\common-issues.md:37`
- **Evidence:** `Pass…ord"  (21 chars)`
- **Remediation:** Move the secret to an environment variable / secret manager and rotate it.

### SEC-298 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\desktop\src-tauri\src\os\windows.rs:165`
- **Evidence:** `let status = unsafe {`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-299 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\desktop\src-tauri\src\os\windows.rs:186`
- **Evidence:** `let status = unsafe {`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-300 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\desktop\src-tauri\src\os\windows.rs:202`
- **Evidence:** `let words = unsafe {`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-302 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\desktop\src-tauri\src\window_customizer.rs:23`
- **Evidence:** `unsafe {`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-303 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\desktop\src-tauri\src\window_customizer.rs:34`
- **Evidence:** `unsafe {`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-305 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\kilo-ui\src\components\diff.tsx:156`
- **Evidence:** ``@layer unsafe { @media (pointer: fine) { [data-separator='line-info-basic'][data-expand-index] [data-separator-wrapper] { grid-template-columns: 34px auto; } }`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-319 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\src-rust\crates\query\src\coordinator.rs:205`
- **Evidence:** `unsafe {`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-320 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\src-rust\crates\query\src\coordinator.rs:211`
- **Evidence:** `unsafe {`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-332 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kilocode\packages\kilo-ui\src\components\diff.tsx:267`
- **Evidence:** ``@layer unsafe { @media (pointer: fine) { [data-separator='line-info-basic'][data-expand-index] [data-separator-wrapper] { grid-template-columns: 34px auto; } }`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-336 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\hades-bridge\src\ffi.rs:216`
- **Evidence:** `unsafe {`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-337 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\hades-bridge\src\ffi.rs:230`
- **Evidence:** `unsafe {`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-338 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\hades-bridge\src\ffi.rs:244`
- **Evidence:** `unsafe {`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-339 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\hades-bridge\src\ffi.rs:261`
- **Evidence:** `unsafe {`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-340 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\hades-bridge\src\ffi.rs:356`
- **Evidence:** `unsafe {`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-341 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\hades-bridge\src\ffi.rs:382`
- **Evidence:** `unsafe {`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-342 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\hades-bridge\src\ffi.rs:401`
- **Evidence:** `unsafe {`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-343 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\hades-bridge\src\tensor.rs:136`
- **Evidence:** `unsafe {`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-344 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\hades-bridge\src\tensor.rs:155`
- **Evidence:** `unsafe {`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-345 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\hades-bridge\src\tensor.rs:183`
- **Evidence:** `unsafe {`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-346 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\hades-bridge\src\tensor.rs:225`
- **Evidence:** `unsafe { std::slice::from_raw_parts(self.data.as_ptr() as *const f32, self.nelements) }`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-347 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\hades-bridge\src\tensor.rs:231`
- **Evidence:** `unsafe { std::slice::from_raw_parts_mut(self.data.as_mut_ptr() as *mut f32, self.nelements) }`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-348 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\hades-bridge\src\tensor.rs:244`
- **Evidence:** `unsafe {`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-349 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\hades-bridge\src\tensor.rs:261`
- **Evidence:** `let wrapper = unsafe { GgmlTensorWrapper::wrap_raw(ptr, 4, 4) };`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-350 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\hades-kernel\src\jit_decompression\inflation.rs:93`
- **Evidence:** `let ptr = unsafe { self.ptr.as_ptr().add(offset) };`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-351 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\hades-kernel\src\jit_decompression\inflation.rs:174`
- **Evidence:** `unsafe {`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-352 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\hades-kernel\src\jit_decompression\kv_cache.rs:309`
- **Evidence:** `let injector = unsafe { &*(injector_ptr as *const KVCacheInjector) };`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-353 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\hades-kernel\src\jit_decompression\kv_cache.rs:312`
- **Evidence:** `let gist_slice = unsafe { std::slice::from_raw_parts(gist_ptr, 1536) };`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-354 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\hades-kernel\src\memory.rs:29`
- **Evidence:** `let mmap = unsafe { Mmap::map(&file) }`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-355 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\hades-kernel\src\memory.rs:100`
- **Evidence:** `let mmap = unsafe { Mmap::map(&file) }`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-356 — Rust unsafe block [Rust unsafe block]
- **Severity:** LOW  ·  **CWE:** CWE-119
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\kortex\hades-kernel\src\memory.rs:108`
- **Evidence:** `let tensor_ptr = unsafe {`
- **Remediation:** Audit unsafe blocks for memory-safety invariants; minimize their scope.

### SEC-273 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\desktop\src-tauri\src\cli.rs:115`
- **Evidence:** `.expect("Failed to get current binary")`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-274 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\desktop\src-tauri\src\cli.rs:117`
- **Evidence:** `.expect("Failed to get parent dir")`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-275 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\desktop\src-tauri\src\cli.rs:374`
- **Evidence:** `.expect("Failed to resolve app local data dir");`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-276 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\desktop\src-tauri\src\cli.rs:488`
- **Evidence:** `BufReader::new(child.stdout().take().unwrap()),`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-277 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\desktop\src-tauri\src\cli.rs:494`
- **Evidence:** `BufReader::new(child.stderr().take().unwrap()),`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-278 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\desktop\src-tauri\src\cli.rs:572`
- **Evidence:** `.expect("Failed to spawn opencode");`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-279 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\desktop\src-tauri\src\lib.rs:82`
- **Evidence:** `.expect("Failed to acquire mutex lock")`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-280 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\desktop\src-tauri\src\lib.rs:346`
- **Evidence:** `.expect("failed to resolve app log dir");`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-281 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\desktop\src-tauri\src\lib.rs:363`
- **Evidence:** `.expect("error while running tauri application")`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-282 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\desktop\src-tauri\src\lib.rs:405`
- **Evidence:** `.expect("Failed to export typescript bindings");`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-283 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\desktop\src-tauri\src\lib.rs:456`
- **Evidence:** `path = %opencode_db_path().expect("failed to get db path").display(),`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-284 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\desktop\src-tauri\src\lib.rs:468`
- **Evidence:** `&& let Some(done_tx) = done_tx.lock().unwrap().take()`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-285 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\desktop\src-tauri\src\lib.rs:510`
- **Evidence:** `let loading_window = LoadingWindow::create(&app).expect("Failed to create loading window");`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-286 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\desktop\src-tauri\src\lib.rs:518`
- **Evidence:** `MainWindow::create(&app).expect("Failed to create main window");`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-287 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\desktop\src-tauri\src\lib.rs:558`
- **Evidence:** `.expect("Failed to bind to find free port")`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-288 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\desktop\src-tauri\src\lib.rs:560`
- **Evidence:** `.expect("Failed to get local address")`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-289 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\desktop\src-tauri\src\linux_windowing.rs:239`
- **Evidence:** `let decision = select_backend(&env, false).expect("missing decision");`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-290 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\desktop\src-tauri\src\linux_windowing.rs:254`
- **Evidence:** `let decision = select_backend(&env, true).expect("missing decision");`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-291 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\desktop\src-tauri\src\linux_windowing.rs:266`
- **Evidence:** `let decision = select_backend(&env, true).expect("missing decision");`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-292 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\desktop\src-tauri\src\linux_windowing.rs:278`
- **Evidence:** `let decision = select_backend(&env, false).expect("missing decision");`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-293 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\desktop\src-tauri\src\linux_windowing.rs:291`
- **Evidence:** `let decision = select_backend(&env, false).expect("missing decision");`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-294 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\desktop\src-tauri\src\linux_windowing.rs:302`
- **Evidence:** `let decision = select_backend(&env, false).expect("missing decision");`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-295 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\desktop\src-tauri\src\logging.rs:13`
- **Evidence:** `std::fs::create_dir_all(log_dir).expect("failed to create log directory");`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-296 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\desktop\src-tauri\src\logging.rs:23`
- **Evidence:** `.expect("logging already initialized");`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-297 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\desktop\src-tauri\src\logging.rs:25`
- **Evidence:** `let file = File::create(&log_path).expect("failed to create log file");`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-301 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\desktop\src-tauri\src\windows.rs:164`
- **Evidence:** `.data_directory(_app.path().config_dir().expect("Failed to get config dir").join(_app.config().product_name.clone().unwrap()))`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-304 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\desktop\src-tauri\src\window_customizer.rs:41`
- **Evidence:** `Retained::retain(_webview.inner().cast()).unwrap();`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-307 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\opencode\src\bus\index.ts:103`
- **Evidence:** `return Stream.unwrap(`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-308 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\opencode\src\bus\index.ts:114`
- **Evidence:** `return Stream.unwrap(`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

### SEC-310 — Panic-prone unwrap/expect [Panic-prone unwrap/expect]
- **Severity:** INFO  ·  **CWE:** CWE-248
- **Location:** `\\?\C:\Users\HADES\Desktop\vscodium-rust\claurst\kilocode\packages\opencode\src\control-plane\adaptors\worktree.ts:6`
- **Evidence:** `name: WorkspaceInfo.shape.name.unwrap(),`
- **Remediation:** Handle errors with ? / match instead of unwrap/expect on hot paths.

