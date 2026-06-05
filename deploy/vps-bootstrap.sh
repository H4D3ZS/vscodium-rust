#!/usr/bin/env bash
# Cyber-Ifrit VPS bootstrap — gateway + Caddy TLS. Does NOT touch Ollama/ROCm/Docker.
# Run on the droplet as root after copying cyberifrit-gateway.js to /opt/cyberifrit/
#
#   export SUPABASE_ANON_KEY='...'
#   export SUPABASE_SERVICE_ROLE_KEY='...'
#   export CYBERIFRIT_DOMAIN='ai.cyberifrit.xyz'   # optional
#   export CF_API_TOKEN='...'                     # optional — if port 80 is busy (Jupyter)
#   bash vps-bootstrap.sh

set -euo pipefail

DOMAIN="${CYBERIFRIT_DOMAIN:-ai.cyberifrit.xyz}"
GATEWAY_DIR="/opt/cyberifrit"
GATEWAY_JS="${GATEWAY_DIR}/cyberifrit-gateway.js"
SUPABASE_URL="${SUPABASE_URL:-https://ktufvjkvejjshtndmjze.supabase.co}"

red() { printf '\033[31m%s\033[0m\n' "$*"; }
green() { printf '\033[32m%s\033[0m\n' "$*"; }

if [[ "${EUID:-$(id -u)}" -ne 0 ]]; then
  red "Run as root: sudo bash vps-bootstrap.sh"
  exit 1
fi

if [[ -z "${SUPABASE_ANON_KEY:-}" || -z "${SUPABASE_SERVICE_ROLE_KEY:-}" ]]; then
  red "Set SUPABASE_ANON_KEY and SUPABASE_SERVICE_ROLE_KEY before running."
  exit 1
fi

detect_ollama_url() {
  if curl -sf --max-time 4 http://127.0.0.1:11434/api/tags >/dev/null 2>&1; then
    echo "http://127.0.0.1:11434"
    return 0
  fi
  if command -v docker >/dev/null 2>&1 && docker ps --format '{{.Names}}' | grep -qx rocm; then
    local ip hp
    ip="$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' rocm 2>/dev/null || true)"
    if [[ -n "$ip" ]] && curl -sf --max-time 4 "http://${ip}:11434/api/tags" >/dev/null 2>&1; then
      echo "http://${ip}:11434"
      return 0
    fi
    hp="$(docker port rocm 11434/tcp 2>/dev/null | head -1 | awk -F: '{print $NF}' || true)"
    if [[ -n "$hp" ]] && curl -sf --max-time 4 "http://127.0.0.1:${hp}/api/tags" >/dev/null 2>&1; then
      echo "http://127.0.0.1:${hp}"
      return 0
    fi
  fi
  return 1
}

green "=== Cyber-Ifrit VPS bootstrap (Ollama/ROCm untouched) ==="

OLLAMA_URL="$(detect_ollama_url || true)"
if [[ -z "$OLLAMA_URL" ]]; then
  red "Could not reach Ollama on :11434 (host or rocm container). Check: docker exec rocm curl -s http://127.0.0.1:11434/api/tags"
  exit 1
fi
green "Ollama URL: $OLLAMA_URL"

if [[ ! -f "$GATEWAY_JS" ]]; then
  if [[ -f ./cyberifrit-gateway.js ]]; then
    mkdir -p "$GATEWAY_DIR"
    cp ./cyberifrit-gateway.js "$GATEWAY_JS"
  else
    red "Missing $GATEWAY_JS — copy deploy/cyberifrit-gateway.js to the server first."
    exit 1
  fi
fi

if ! command -v node >/dev/null 2>&1 || [[ "$(node -p 'process.versions.node.split(".")[0]' 2>/dev/null || echo 0)" -lt 18 ]]; then
  green "Installing Node.js 20..."
  curl -fsSL https://deb.nodesource.com/setup_20.x | bash -
  apt-get install -y nodejs
fi
green "Node: $(node -v)"

cat > /etc/systemd/system/cyberifrit-gateway.service <<UNIT
[Unit]
Description=Cyber-Ifrit cloud gateway (auth + queue -> Ollama)
After=network.target docker.service
Wants=docker.service

[Service]
WorkingDirectory=${GATEWAY_DIR}
ExecStart=/usr/bin/node ${GATEWAY_JS}
Restart=always
Environment=PORT=8787
Environment=OLLAMA_URL=${OLLAMA_URL}
Environment=MAX_CONCURRENT=12
Environment=MAX_QUEUE=200
Environment=SUPABASE_URL=${SUPABASE_URL}
Environment=SUPABASE_ANON_KEY=${SUPABASE_ANON_KEY}
Environment=SUPABASE_SERVICE_ROLE_KEY=${SUPABASE_SERVICE_ROLE_KEY}

[Install]
WantedBy=multi-user.target
UNIT

systemctl daemon-reload
systemctl enable --now cyberifrit-gateway
sleep 1
if curl -sf http://127.0.0.1:8787/healthz | grep -qx ok; then
  green "Gateway healthz: ok"
else
  red "Gateway failed — check: journalctl -u cyberifrit-gateway -n 30 --no-pager"
  exit 1
fi

PORT80_BUSY=false
if ss -tlnH 'sport = :80' 2>/dev/null | grep -q .; then
  PORT80_BUSY=true
  green "Port 80 in use (likely Jupyter) — will use DNS TLS if CF_API_TOKEN is set."
fi

if ! command -v caddy >/dev/null 2>&1; then
  green "Installing Caddy..."
  apt-get update -qq
  apt-get install -y debian-keyring debian-archive-keyring apt-transport-https curl
  curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/gpg.key' | gpg --dearmor -o /usr/share/keyrings/caddy-stable-archive-keyring.gpg 2>/dev/null || true
  curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/debian.deb.txt' | tee /etc/apt/sources.list.d/caddy-stable.list >/dev/null
  apt-get update -qq && apt-get install -y caddy
fi

if [[ "$PORT80_BUSY" == true && -n "${CF_API_TOKEN:-}" ]]; then
  cat > /etc/caddy/Caddyfile <<CADDY
{
  acme_dns cloudflare {env.CF_API_TOKEN}
}

${DOMAIN} {
    reverse_proxy 127.0.0.1:8787 {
        flush_interval -1
    }
}
CADDY
  mkdir -p /etc/systemd/system/caddy.service.d
  printf '[Service]\nEnvironment=CF_API_TOKEN=%s\n' "$CF_API_TOKEN" > /etc/systemd/system/caddy.service.d/cf-token.conf
elif [[ "$PORT80_BUSY" == true ]]; then
  green "Port 80 busy (Jupyter) — Caddy will use TLS-ALPN on :443 only (no port 80 needed)."
  cat > /etc/caddy/Caddyfile <<CADDY
{
    http_port 0
}

${DOMAIN} {
    reverse_proxy 127.0.0.1:8787 {
        flush_interval -1
    }
}
CADDY
else
  cat > /etc/caddy/Caddyfile <<CADDY
${DOMAIN} {
    reverse_proxy 127.0.0.1:8787 {
        flush_interval -1
    }
}
CADDY
fi

systemctl enable caddy
systemctl restart caddy || {
  red "Caddy failed to start — check: journalctl -u caddy -n 30 --no-pager"
  exit 1
}

ufw allow 443/tcp 2>/dev/null || true
ufw allow 80/tcp 2>/dev/null || true

green "=== Done ==="
echo "  Gateway:  http://127.0.0.1:8787/healthz"
echo "  Public:   https://${DOMAIN}/healthz  (after DNS A record -> $(curl -4 -s ifconfig.me 2>/dev/null || hostname -I | awk '{print $1}'))"
echo "  Test 401: curl -s -o /dev/null -w '%{http_code}\n' https://${DOMAIN}/api/tags"
echo "  Ollama:   ${OLLAMA_URL} (unchanged)"
