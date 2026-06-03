# AMD-MIX Managed Cloud — Ollama on MI300X behind the subscription

How the **managed cloud** model path works and how to stand it up on the
temporary AMD-MIX VPS. Subscribers get this with **no API key** — the IDE sends
their Supabase **subscription token** automatically; the gateway authorizes it.

```
IDE  ──Bearer <supabase access token>──▶  api.cyberifrit.xyz  ──▶  Ollama (MI300X)
 (provider "cyberifrit")                    auth gateway:           OpenAI-compat
                                            • verify JWT (Supabase)  /v1/chat/completions
                                            • check entitlement
                                              (cloud_models / amd_backend)
                                            • proxy if allowed
```

The IDE side is already wired:
- Provider **`cyberifrit`** → endpoint `https://api.cyberifrit.xyz`
  (`ai_engine.rs::get_endpoint`, override via `CYBERIFRIT_BASE_URL` /
  `api_keys.json:cyberifrit_base_url`).
- Auth: if no explicit `cyberifrit` key is set, the IDE uses the **signed-in
  Supabase access token** (`ai_engine.rs::get_key_for_provider`). So "pay &
  subscribe → managed cloud just works."

---

## 1. Provision + ROCm

On the MI300X VPS (Ubuntu 22.04 + ROCm):

```bash
# ROCm (per AMD's install guide for your kernel), then verify the GPU:
rocminfo | grep -i "Marketing Name"
```

## 2. Ollama with ROCm

```bash
curl -fsSL https://ollama.com/install.sh | sh   # detects ROCm
# Bind to localhost only (the gateway is the public face):
sudo systemctl edit ollama   # add:
#   [Service]
#   Environment="OLLAMA_HOST=127.0.0.1:11434"
sudo systemctl restart ollama

# Pull the models you sell (names the IDE will request):
ollama pull qwen2.5-coder:32b
ollama pull qwen2.5:32b
ollama pull deepseek-r1:32b
```

Ollama exposes an **OpenAI-compatible** API at `/v1/chat/completions` — exactly
what the IDE's `cyberifrit` provider calls.

## 3. Auth gateway (the entitlement check)

Put a tiny gateway in front so only paying subscribers reach the GPU. Terminate
TLS for `api.cyberifrit.xyz`, validate the Supabase JWT + entitlement, then proxy
to local Ollama. Minimal Bun/Node sketch:

```ts
// gateway.ts — run behind nginx/Caddy TLS; listens on 127.0.0.1:8787
const SUPABASE_URL = process.env.SUPABASE_URL!;
const SERVICE_ROLE = process.env.SUPABASE_SERVICE_ROLE_KEY!;
const ANON = process.env.SUPABASE_ANON_KEY!;

Bun.serve({
  port: 8787,
  async fetch(req) {
    const url = new URL(req.url);
    if (!url.pathname.startsWith("/v1/")) return new Response("not found", { status: 404 });

    const jwt = (req.headers.get("authorization") || "").replace(/^Bearer\s+/i, "");
    // 1. Who is this?
    const u = await fetch(`${SUPABASE_URL}/auth/v1/user`, {
      headers: { apikey: ANON, Authorization: `Bearer ${jwt}` },
    }).then((r) => (r.ok ? r.json() : null)).catch(() => null);
    if (!u?.id) return new Response("unauthorized", { status: 401 });

    // 2. Are they entitled? (active sub, paid tier)
    const sub = await fetch(
      `${SUPABASE_URL}/rest/v1/subscriptions?user_id=eq.${u.id}&select=tier,status`,
      { headers: { apikey: SERVICE_ROLE, Authorization: `Bearer ${SERVICE_ROLE}` } },
    ).then((r) => r.json()).then((r) => r?.[0]).catch(() => null);
    const ok = sub && sub.status === "active" &&
      ["pro_developer", "security_researcher", "enterprise"].includes(sub.tier);
    if (!ok) return new Response("subscription required", { status: 402 });

    // 3. Proxy to local Ollama (OpenAI-compat).
    const body = await req.text();
    return fetch(`http://127.0.0.1:11434${url.pathname}`, {
      method: req.method,
      headers: { "Content-Type": "application/json" },
      body,
    });
  },
});
```

nginx (TLS + route only `/v1/`):

```nginx
server {
  server_name api.cyberifrit.xyz;
  listen 443 ssl;  # certs via certbot
  location /v1/ { proxy_pass http://127.0.0.1:8787; proxy_read_timeout 600s; }
}
```

> **Quota:** optionally have the gateway also bump `usage_counters` per request
> (service role) so server-side metering matches the IDE's local count — the
> seam is already in the Supabase schema.

## 4. Use it from the IDE

1. Sign in (Settings → Account) and hold an **active** paid subscription.
2. Model picker → provider **Cyber-Ifrit Cloud** → pick a model name you pulled
   (e.g. `qwen2.5-coder:32b`).
3. The IDE sends your Supabase token as the Bearer; the gateway authorizes and
   proxies to the MI300X. No API key needed.

## Notes

- **BYO key still works** for everything else: set your own
  Anthropic/OpenAI/Interface-AI/etc. key in Settings → Providers & Keys (or
  `api_keys.json`) and select that provider. Managed cloud and BYO key coexist.
- Temporary VPS (until Jun 10): point `CYBERIFRIT_BASE_URL` at the VPS IP/host
  while DNS for `api.cyberifrit.xyz` settles, then switch to the domain.
