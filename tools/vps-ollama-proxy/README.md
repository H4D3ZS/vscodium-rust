# VPS Ollama HTTPS proxy (Nginx + Certbot + Bearer + CORS)

Ship this folder with the repo so you can reprovision a GPU VPS after snapshots or IP changes.

## Files (safe to commit)

| File | Purpose |
|------|---------|
| `bootstrap.sh` | Full install: packages, UFW, Nginx vhost, Let’s Encrypt, proxy + auth |
| `boostrap.sh` | Typo wrapper: `exec`s `bootstrap.sh` |
| `env.example` | Template — **copy to `.env` on the server** |
| `snippets/options-ssl-nginx.conf` | TLS defaults for `include` (avoids broken manual `tee` heredocs) |
| `.gitignore` | Ignores `.env` when you use local overrides |

## What `bootstrap.sh` does

1. Installs `nginx`, `certbot`, `curl`, `ufw`, `openssl`
2. Opens UFW: SSH + HTTP/HTTPS
3. Writes rate-limit zones to `/etc/nginx/conf.d/ollama-proxy-zones.conf`
4. Backs up conflicting vhosts: all of `/etc/nginx/sites-enabled/*` and any `/etc/nginx/conf.d/*.conf` that mention your `OLLAMA_DOMAIN` + `server_name` (except `ollama-proxy-zones.conf`)
5. Temporary HTTP-only vhost → Certbot **webroot** → certificate
6. Optional DH params file for Certbot’s SSL options
7. Final vhost: HTTPS, Bearer gate on `$http_authorization`, CORS, `/v1/` → Ollama with `/v1/api/*` → `/api/*`

Does **not** install Ollama or pull models.

## Quick start (on the VPS)

```bash
cd /path/to/vscodium-rust/tools/vps-ollama-proxy
cp env.example .env
nano .env   # OLLAMA_DOMAIN, OLLAMA_BEARER, LE_EMAIL

sudo bash bootstrap.sh
# or: sudo bash boostrap.sh
```

Copy the **whole** `vps-ollama-proxy` folder (including `snippets/`) to the VPS so `options-ssl-nginx.conf` installs cleanly. If you only copied `boostrap.sh`, see **Recovery** below.

Required `.env` variables: `OLLAMA_DOMAIN`, `OLLAMA_BEARER`, `LE_EMAIL`.  
Optional: `CORS_ORIGIN`, `OLLAMA_UPSTREAM`, `OLLAMA_PROXY_HOST` (see `env.example`).

`CORS_ORIGIN` is a **space-separated allow-list**. nginx echoes back whichever
origin matches (or the first entry on a miss) so multiple end-user surfaces
share one proxy. Defaults cover the four production targets for paying
vscodium-rust customers:

```
"https://h4d3zs.github.io tauri://localhost https://tauri.localhost http://localhost:5173 http://127.0.0.1:5173"
```

| Origin | Surface |
|--------|---------|
| `tauri://localhost` | macOS / Linux Tauri bundle (older Windows WebView) |
| `https://tauri.localhost` | Windows WebView2 Tauri bundle |
| `https://h4d3zs.github.io` | PWA / stage-web fallback |
| `http://localhost:5173` | `npm run dev` Vite renderer |

> The Tauri build sends all Ollama traffic through Rust IPC, so it never
> trips CORS. The whitelist above only matters for browser surfaces.

DNS: **A** record for the hostname (e.g. `ai`) must point at the server **before** Certbot runs.

**Do not use `ai.example.com`** (or other `*.example.*`) as `OLLAMA_DOMAIN` — Let’s Encrypt returns *policy forbids issuance* for those reserved names. Use your real subdomain (e.g. `ai.cyberifrit.xyz`).

## After success

- Client base URL: `https://<OLLAMA_DOMAIN>/v1`
- Header: `Authorization: Bearer <OLLAMA_BEARER>`

```bash
curl -s -o /dev/null -w "%{http_code}\n" "https://${OLLAMA_DOMAIN}/v1/models"
curl -s -o /dev/null -w "%{http_code}\n" -H "Authorization: Bearer YOUR_TOKEN" "https://${OLLAMA_DOMAIN}/v1/models"
```

Expect `401` then `200` when Ollama is up.

## Point vscodium-rust at this host

1. In the IDE: **Agent settings** (right sidebar → settings / gear, or the Agent settings view) → **Direct Ollama**.
2. **Self-Hosted URL**: `https://ai.cyberifrit.xyz` (no trailing slash).
3. **Ollama bearer**: paste the same value as `OLLAMA_BEARER` from your VPS `.env`, click **Save token** (stored in `api_keys.json` as `ollama`; the Rust backend sends `Authorization: Bearer …` on Ollama HTTP calls). Alternatively set env `OLLAMA_API_KEY` before launching the app.
4. Model picker: choose provider **Ollama** and a model name that exists on the server (e.g. from `/v1/models`).

On **Windows `cmd.exe`**, line continuation uses `^` not `\`. Example:

```bat
curl -sS -o NUL -w "%%{http_code}\n" -H "Authorization: Bearer YOUR_TOKEN" https://ai.cyberifrit.xyz/v1/models
```

## Troubleshooting

| Symptom | Cause / fix |
|---------|-------------|
| Certbot **policy / cannot issue for … example.com** | `OLLAMA_DOMAIN` must be a name you own in DNS (e.g. `ai.cyberifrit.xyz`). `*.example.com` is blocked by ACME policy. |
| Certbot **404** on ACME | Duplicate `server_name` on port 80 — script moves `sites-enabled` and matching `conf.d` files; or manually remove `/etc/nginx/conf.d/ollama.conf` etc., reload nginx, retry |
| **`open() ... options-ssl-nginx.conf failed`** | `certonly --webroot` often leaves certs but not Certbot’s options snippet — latest `bootstrap.sh` creates `/etc/letsencrypt/options-ssl-nginx.conf` if missing, then re-run |
| **`could not build map_hash`** | Long Bearer token — script sets `map_hash_bucket_size 512` |
| **`unexpected EOF` / quote errors** | Old script copies — use repo `bootstrap.sh`; heredocs use escaped `Connection` header |
| **`ssl_dhparam` duplicate** | Do not add `ssl_dhparam` twice — this script relies on Certbot’s `options-ssl-nginx.conf` only |

Re-running the script is safe: existing certs and zone file are reused when present.

## Recovery (garbled `options-ssl-nginx.conf` after bad paste)

From a machine that has the repo:

```bash
scp tools/vps-ollama-proxy/snippets/options-ssl-nginx.conf root@YOUR_VPS:/tmp/options-ssl-nginx.conf
ssh root@YOUR_VPS 'sudo install -m 0644 /tmp/options-ssl-nginx.conf /etc/letsencrypt/options-ssl-nginx.conf'
```

On the VPS, append DH line only if missing:

```bash
grep -q '^[[:space:]]*ssl_dhparam' /etc/letsencrypt/options-ssl-nginx.conf || \
  echo 'ssl_dhparam /etc/letsencrypt/ssl-dhparams.pem;' | sudo tee -a /etc/letsencrypt/options-ssl-nginx.conf
sudo nginx -t && sudo systemctl reload nginx
```

Or re-run `sudo bash boostrap.sh` after syncing the full `tools/vps-ollama-proxy/` directory (bootstrap will overwrite from `snippets/` when present).
