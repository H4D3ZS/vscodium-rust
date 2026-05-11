#!/usr/bin/env bash
#
# vps-ollama-proxy — one-shot Ubuntu VPS: UFW, Nginx, Let's Encrypt (webroot),
# HTTPS reverse proxy to Ollama, Bearer token gate, CORS for GitHub Pages.
#
# Prerequisites: DNS A/AAAA for OLLAMA_DOMAIN → this host; Ollama listening on OLLAMA_UPSTREAM.
#
# Usage:
#   cd tools/vps-ollama-proxy
#   cp env.example .env && nano .env
#   sudo bash bootstrap.sh
#
# Same entrypoint: sudo bash boostrap.sh (wrapper in repo)
#
set -euo pipefail

RED='\033[0;31m'; GRN='\033[0;32m'; YLW='\033[1;33m'; NC='\033[0m'
die() { echo -e "${RED}ERROR:${NC} $*" >&2; exit 1; }
info() { echo -e "${GRN}==>${NC} $*"; }
warn() { echo -e "${YLW}WARN:${NC} $*"; }

[[ "${EUID:-$(id -u)}" -eq 0 ]] || die "Run as root: sudo bash $(basename "${BASH_SOURCE[0]}")"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [[ -f "${SCRIPT_DIR}/.env" ]]; then
  # shellcheck source=/dev/null
  set -a && source "${SCRIPT_DIR}/.env" && set +a
  info "Loaded ${SCRIPT_DIR}/.env"
else
  warn "No ${SCRIPT_DIR}/.env — copy env.example to .env and set variables."
fi

: "${OLLAMA_DOMAIN:?Set OLLAMA_DOMAIN in .env (hostname only, e.g. ai.cyberifrit.xyz)}"
: "${OLLAMA_BEARER:?Set OLLAMA_BEARER in .env (shared secret, no double quotes)}"
: "${LE_EMAIL:?Set LE_EMAIL in .env (Certbot registration email)}"

[[ -n "${LE_EMAIL// }" ]] || die "LE_EMAIL is empty"
[[ "${OLLAMA_DOMAIN}" != *://* ]] || die "OLLAMA_DOMAIN must be hostname only (no http:// or https://)"
[[ "${OLLAMA_BEARER}" == *'"'* ]] && die "OLLAMA_BEARER must not contain double quotes"
[[ "${LE_EMAIL}" == *@* ]] || die "LE_EMAIL should look like an email address"

# Let's Encrypt / ACME policy blocks certificates for IANA example/reserved names.
case "${OLLAMA_DOMAIN}" in
  *.example.com|*.example.net|*.example.org|*.test|*.invalid|*.localhost)
    die "OLLAMA_DOMAIN=${OLLAMA_DOMAIN} — ACME will refuse this zone. Use a hostname you own (DNS A → this server), e.g. ai.cyberifrit.xyz not ai.example.com."
    ;;
esac

# CORS_ORIGIN accepts a space-separated allow-list. The first entry is also
# the fallback echoed for unknown origins so existing single-origin deployments
# keep working. Add the surfaces your $200 IDE end-users hit:
#   - Tauri webview origins: tauri://localhost (macOS/Linux), https://tauri.localhost (Windows WebView2)
#   - GitHub Pages PWA: https://h4d3zs.github.io
#   - Local dev: http://localhost:5173 http://127.0.0.1:5173
CORS_ORIGIN="${CORS_ORIGIN:-https://h4d3zs.github.io tauri://localhost https://tauri.localhost http://localhost:5173 http://127.0.0.1:5173}"
OLLAMA_UPSTREAM="${OLLAMA_UPSTREAM:-http://127.0.0.1:11434}"
OLLAMA_PROXY_HOST="${OLLAMA_PROXY_HOST:-127.0.0.1:11434}"

# Build the `map` body so nginx can reflect whichever request Origin is on the list.
CORS_FALLBACK=""
CORS_MAP_BODY=""
for origin in ${CORS_ORIGIN}; do
  [[ -z "${CORS_FALLBACK}" ]] && CORS_FALLBACK="${origin}"
  CORS_MAP_BODY+="    \"${origin}\" \"${origin}\";"$'\n'
done

AUTH_EXPECTED="Bearer ${OLLAMA_BEARER}"
MAP_LINE="\"${AUTH_EXPECTED}\" 1;"

info "Checking DNS for ${OLLAMA_DOMAIN} ..."
PUB_IP="$(curl -4 -fsS https://ifconfig.me 2>/dev/null || curl -4 -fsS https://api.ipify.org || true)"
RESOLVED="$(getent ahostsv4 "${OLLAMA_DOMAIN}" 2>/dev/null | awk '{print $1; exit}' || true)"
if [[ -n "${RESOLVED}" && -n "${PUB_IP}" && "${RESOLVED}" != "${PUB_IP}" ]]; then
  warn "DNS ${OLLAMA_DOMAIN} -> ${RESOLVED}, this host IPv4 -> ${PUB_IP} (Certbot may fail until DNS matches)"
fi

info "Installing packages ..."
export DEBIAN_FRONTEND=noninteractive
apt-get update -qq
apt-get install -y -qq nginx certbot python3-certbot-nginx curl ufw openssl >/dev/null

info "UFW: SSH + HTTP/HTTPS ..."
ufw allow OpenSSH >/dev/null 2>&1 || true
ufw allow 'Nginx Full' >/dev/null 2>&1 || true
yes | ufw enable >/dev/null 2>&1 || true

WEBROOT="/var/www/html"
mkdir -p "${WEBROOT}/.well-known/acme-challenge"
chmod -R a+rX "${WEBROOT}/.well-known" 2>/dev/null || true

ZONES_FILE="/etc/nginx/conf.d/ollama-proxy-zones.conf"
if [[ ! -f "${ZONES_FILE}" ]]; then
  info "Writing rate-limit zones (${ZONES_FILE}) ..."
  cat >"${ZONES_FILE}" <<'ZONES'
limit_req_zone $binary_remote_addr zone=ollama_rl:10m rate=15r/s;
limit_conn_zone $binary_remote_addr zone=ollama_conn:10m;
ZONES
fi

mkdir -p /etc/nginx/sites-available /etc/nginx/sites-enabled
SITE_PATH="/etc/nginx/sites-available/${OLLAMA_DOMAIN}.conf"
ENABLED="/etc/nginx/sites-enabled/${OLLAMA_DOMAIN}.conf"

# Duplicate server_name on :80 → wrong vhost answers ACME → Certbot 404
SITES_BACKUP="/root/nginx-sites-enabled-backup-$(date +%s)"
if compgen -G "/etc/nginx/sites-enabled/*" >/dev/null; then
  info "Backing up /etc/nginx/sites-enabled/* -> ${SITES_BACKUP} ..."
  mkdir -p "${SITES_BACKUP}"
  # shellcheck disable=SC2115
  mv /etc/nginx/sites-enabled/* "${SITES_BACKUP}/" 2>/dev/null || true
fi

# e.g. /etc/nginx/conf.d/ollama.conf with same server_name steals port 80
CONFD_BACKUP="/root/nginx-conf.d-backup-$(date +%s)"
mkdir -p "${CONFD_BACKUP}"
shopt -s nullglob
for f in /etc/nginx/conf.d/*.conf; do
  [[ -f "$f" ]] || continue
  [[ "$(basename "$f")" == "ollama-proxy-zones.conf" ]] && continue
  if grep -q "server_name" "$f" 2>/dev/null && grep -Fq "${OLLAMA_DOMAIN}" "$f" 2>/dev/null; then
    info "Moving conf.d vhost $(basename "$f") -> ${CONFD_BACKUP}/ ..."
    mv "$f" "${CONFD_BACKUP}/"
  fi
done
shopt -u nullglob

info "Writing temporary HTTP-only server for ACME ..."
cat >"${SITE_PATH}" <<EOF
server {
    listen 80 default_server;
    listen [::]:80 default_server;
    server_name ${OLLAMA_DOMAIN};

    location /.well-known/acme-challenge/ {
        root ${WEBROOT};
    }

    location / {
        return 200 'ACME OK\n';
        add_header Content-Type text/plain;
    }
}
EOF

ln -sf "${SITE_PATH}" "${ENABLED}"
nginx -t
systemctl reload nginx

CERT_DIR="/etc/letsencrypt/live/${OLLAMA_DOMAIN}"
if [[ ! -f "${CERT_DIR}/fullchain.pem" ]]; then
  info "Obtaining TLS certificate (Let's Encrypt) ..."
  certbot certonly --webroot -w "${WEBROOT}" -d "${OLLAMA_DOMAIN}" \
    --email "${LE_EMAIL}" --agree-tos --non-interactive --rsa-key-size 4096
else
  info "Certificate already present: ${CERT_DIR}"
fi

if [[ ! -f /etc/letsencrypt/ssl-dhparams.pem ]]; then
  info "Generating DH params (one-time, ~1-3 min) ..."
  openssl dhparam -out /etc/letsencrypt/ssl-dhparams.pem 2048
fi

# certbot certonly --webroot often skips this file; copy-paste heredocs also corrupt it easily.
# Prefer repo file snippets/options-ssl-nginx.conf (ship whole tools/vps-ollama-proxy/ to the VPS).
LE_SSL_OPTS="/etc/letsencrypt/options-ssl-nginx.conf"
SSL_SNIP="${SCRIPT_DIR}/snippets/options-ssl-nginx.conf"
mkdir -p /etc/letsencrypt
if [[ -f "${SSL_SNIP}" ]]; then
  info "Installing ${LE_SSL_OPTS} from ${SSL_SNIP} ..."
  install -m 0644 "${SSL_SNIP}" "${LE_SSL_OPTS}"
elif [[ ! -f "${LE_SSL_OPTS}" ]]; then
  info "Creating ${LE_SSL_OPTS} (embedded defaults; add snippets/ to repo copy on server) ..."
  cat >"${LE_SSL_OPTS}" <<'SSL_OPTS'
ssl_session_cache shared:le_nginx_SSL:10m;
ssl_session_timeout 1440m;
ssl_session_tickets off;
ssl_protocols TLSv1.2 TLSv1.3;
ssl_prefer_server_ciphers off;
ssl_ciphers ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-CHACHA20-POLY1305:ECDHE-RSA-CHACHA20-POLY1305:DHE-RSA-AES128-GCM-SHA256:DHE-RSA-AES256-GCM-SHA384;
SSL_OPTS
else
  info "Keeping existing ${LE_SSL_OPTS} (no snippets/options-ssl-nginx.conf next to bootstrap)."
fi
if [[ -f /etc/letsencrypt/ssl-dhparams.pem ]] && ! grep -q '^[[:space:]]*ssl_dhparam' "${LE_SSL_OPTS}" 2>/dev/null; then
  printf '\nssl_dhparam /etc/letsencrypt/ssl-dhparams.pem;\n' >>"${LE_SSL_OPTS}"
fi

info "Writing HTTPS + Ollama proxy + auth + CORS ..."
# map_hash_bucket_size: long "Bearer <token>" keys exceed default 64 -> nginx emerg
# include options-ssl-nginx.conf (created above if certbot did not install it).
cat >"${SITE_PATH}" <<NGX
map_hash_bucket_size 512;
map_hash_max_size 4096;

map \$http_authorization \$ollama_auth_ok {
    default 0;
    ${MAP_LINE}
}

# Reflect a known Origin so multiple end-user surfaces work without using "*"
# (which is incompatible with Authorization-bearing requests in some browsers).
map \$http_origin \$cors_origin_allowed {
    default "${CORS_FALLBACK}";
${CORS_MAP_BODY}}

server {
    listen 80;
    listen [::]:80;
    server_name ${OLLAMA_DOMAIN};

    location /.well-known/acme-challenge/ {
        root ${WEBROOT};
    }
    location / {
        return 301 https://\$host\$request_uri;
    }
}

server {
    listen 443 ssl http2;
    listen [::]:443 ssl http2;
    server_name ${OLLAMA_DOMAIN};

    ssl_certificate     ${CERT_DIR}/fullchain.pem;
    ssl_certificate_key ${CERT_DIR}/privkey.pem;
    include /etc/letsencrypt/options-ssl-nginx.conf;

    add_header Access-Control-Allow-Origin \$cors_origin_allowed always;
    add_header Vary "Origin" always;
    add_header Access-Control-Allow-Methods "GET, POST, OPTIONS" always;
    add_header Access-Control-Allow-Headers "Authorization, Content-Type" always;
    add_header Access-Control-Allow-Credentials "true" always;
    add_header Access-Control-Max-Age 86400 always;

    if (\$request_method = OPTIONS) { return 204; }

    if (\$ollama_auth_ok = 0) { return 401; }

    limit_req zone=ollama_rl burst=30 nodelay;
    limit_conn ollama_conn 20;

    location /v1/ {
        rewrite ^/v1/api/(.*)\$ /api/\$1 break;
        proxy_pass ${OLLAMA_UPSTREAM};
        proxy_http_version 1.1;
        proxy_set_header Host ${OLLAMA_PROXY_HOST};
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
        proxy_set_header Connection \"\";
        proxy_buffering off;
        proxy_read_timeout 3600s;
        proxy_send_timeout 3600s;
    }

    # Native Ollama API surface. Required for /api/tags (model list),
    # /api/pull (pulling new models), and any client that doesn't speak the
    # OpenAI-compat /v1 dialect (e.g. ollama CLI, vscodium-rust IDE settings).
    location /api/ {
        proxy_pass ${OLLAMA_UPSTREAM};
        proxy_http_version 1.1;
        proxy_set_header Host ${OLLAMA_PROXY_HOST};
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
        proxy_set_header Connection \"\";
        proxy_buffering off;
        proxy_read_timeout 3600s;
        proxy_send_timeout 3600s;
    }

    location / {
        return 404;
    }
}
NGX

nginx -t
systemctl reload nginx

if systemctl is-active --quiet ollama 2>/dev/null; then
  info "Ollama service is active."
else
  warn "Ollama not active. Start with: sudo systemctl start ollama"
fi

if curl -sf --max-time 3 "${OLLAMA_UPSTREAM}/api/tags" >/dev/null 2>&1; then
  info "Ollama responds at ${OLLAMA_UPSTREAM}"
else
  warn "No response from ${OLLAMA_UPSTREAM}/api/tags (check Ollama is running)"
fi

info "Smoke: HTTPS /v1/models without auth (expect 401)"
curl -sk -o /dev/null -w "  %{http_code}\n" "https://${OLLAMA_DOMAIN}/v1/models" || true
info "Smoke: HTTPS /v1/models with Bearer (expect 200)"
curl -sk -o /dev/null -w "  %{http_code}\n" -H "Authorization: Bearer ${OLLAMA_BEARER}" "https://${OLLAMA_DOMAIN}/v1/models" || true
info "Smoke: HTTPS /api/tags with Bearer (expect 200; used by ollama CLI + vscodium-rust)"
curl -sk -o /dev/null -w "  %{http_code}\n" -H "Authorization: Bearer ${OLLAMA_BEARER}" "https://${OLLAMA_DOMAIN}/api/tags" || true

info "Done. Clients: https://${OLLAMA_DOMAIN} (both /api/ and /v1/) + Authorization: Bearer <OLLAMA_BEARER>"
