# VPS setup — cloud inference behind `ai.cyberifrit.xyz` (manual)

You run these. The goal is to expose the **already-working** Ollama on the
MI300X to signed-in paid users, through an auth+queue gateway, over TLS — **with
two pieces only**: the gateway and (optionally) the AIM proxy.

> ⛔ **DO NOT touch Ollama / ROCm.** It runs flawlessly on a base deploy. No
> `HOME=`, no `OLLAMA_NUM_PARALLEL`, no service restarts, no `pkill llama`. If
> Ollama answers on `127.0.0.1:11434`, leave it exactly as it is.

Sanity check Ollama (read-only — don't restart it):

```bash
curl -s http://127.0.0.1:11434/api/tags | head -c 300; echo
```

---

## 1. Auth + concurrency gateway (required)

This is `deploy/cyberifrit-gateway.js` from the repo. It verifies the Supabase
JWT + entitlement (active/trialing paid tier), then proxies to Ollama through a
fair FIFO queue (no rate-limit; 50+ users degrade by latency, never rejection).

```
nginx/Caddy (TLS, ai.cyberifrit.xyz) ──▶ gateway 127.0.0.1:8787 ──▶ Ollama 127.0.0.1:11434
```

Install Node 18+, copy the gateway, run it under systemd:

```bash
sudo mkdir -p /opt/cyberifrit && sudo cp cyberifrit-gateway.js /opt/cyberifrit/
sudo tee /etc/systemd/system/cyberifrit-gateway.service >/dev/null <<'UNIT'
[Unit]
Description=Cyber-Ifrit cloud gateway (auth + queue -> Ollama)
After=network.target
[Service]
WorkingDirectory=/opt/cyberifrit
ExecStart=/usr/bin/node /opt/cyberifrit/cyberifrit-gateway.js
Restart=always
Environment=PORT=8787
Environment=OLLAMA_URL=http://127.0.0.1:11434
Environment=MAX_CONCURRENT=12
Environment=MAX_QUEUE=200
Environment=SUPABASE_URL=https://ktufvjkvejjshtndmjze.supabase.co
Environment=SUPABASE_ANON_KEY=__anon_key__
Environment=SUPABASE_SERVICE_ROLE_KEY=__service_role_key__
[Install]
WantedBy=multi-user.target
UNIT
sudo systemctl daemon-reload && sudo systemctl enable --now cyberifrit-gateway
curl -s 127.0.0.1:8787/healthz   # -> ok
```

Fill the two Supabase keys (same values as the website). The gateway binds
**127.0.0.1 only** — it must sit behind the reverse proxy, never exposed raw.

## 2. TLS reverse proxy

**Caddy** (simplest — auto-TLS):

```bash
sudo apt install -y caddy
sudo tee /etc/caddy/Caddyfile >/dev/null <<'CADDY'
ai.cyberifrit.xyz {
    reverse_proxy 127.0.0.1:8787 {
        flush_interval -1          # stream tokens immediately (SSE/NDJSON)
    }
}
CADDY
sudo systemctl restart caddy
```

(nginx alternative: `proxy_pass http://127.0.0.1:8787;` with
`proxy_buffering off;` and `proxy_read_timeout 600s;` for long generations.)

## 3. DNS

Point `ai.cyberifrit.xyz` at the droplet's public IP.

- In Cloudflare add: `A  ai  <DROPLET_IP>` — **DNS only (grey cloud)**.
- Why grey: proxied Cloudflare Free has a **100s** response cap and buffers
  streams; model generations exceed that. DNS-only lets Caddy terminate TLS and
  stream uncapped. (You still keep Cloudflare's WAF on the website itself.)

Open the firewall for TLS only:

```bash
sudo ufw allow 443/tcp && sudo ufw allow 80/tcp   # 80 = ACME challenge
```

## 4. (Optional) AIM proxy — `.aim` context injection

Only if you want `.aim` memory injection in front of Ollama. It's cross-platform
(rustls, no system OpenSSL) and binds `:1536`, forwarding to Ollama `:11434`.

```bash
# build once (from the kortex workspace):
cargo build --release --bin aim-proxy
# run with the Linux memory path:
AIM_PATH=/opt/cyberifrit/.aim/memory.aim ./aim-proxy   # listens :1536 -> :11434
```

If you use it, point the gateway at it instead of Ollama:
`Environment=OLLAMA_URL=http://127.0.0.1:1536`. Otherwise skip this entirely.

## 5. Wire the IDE + admin

- **IDE**: set the cloud Ollama base to `https://ai.cyberifrit.xyz`. Signed-in
  users send their Supabase JWT as `Authorization: Bearer`; the gateway enforces
  the paid/trial entitlement.
- **Admin → Settings**: set **Inference health URL** to
  `https://ai.cyberifrit.xyz/healthz` so the Infrastructure card shows up/down +
  latency.

## 6. Verify

```bash
# unauthenticated -> 401
curl -s -o /dev/null -w '%{http_code}\n' https://ai.cyberifrit.xyz/api/tags
# with a signed-in user's JWT -> 200 + model list
curl -s https://ai.cyberifrit.xyz/api/tags -H "Authorization: Bearer $JWT" | head -c 200
```

- [ ] `healthz` = ok through HTTPS
- [ ] no token → 401, free/expired user → 402, paid/trial → 200
- [ ] tokens stream (no 100s cut-off)
- [ ] Ollama/ROCm config **untouched**
