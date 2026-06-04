// Cyber-Ifrit Cloud gateway — sits in front of Ollama on the MI300X.
// nginx (TLS, ai.cyberifrit.xyz) -> this (127.0.0.1:8787) -> Ollama (127.0.0.1:11434)
//
// Per request: verify the Supabase JWT + entitlement (active/trialing paid tier),
// then proxy with a fair concurrency queue so 50+ users degrade by latency, never
// by hard rejection (the "no 5-hour wait" promise). Node 18+ (global fetch).

const http = require("http");

const SUPABASE_URL = (process.env.SUPABASE_URL || "").replace(/\/$/, "");
const ANON = process.env.SUPABASE_ANON_KEY || "";
const SERVICE = process.env.SUPABASE_SERVICE_ROLE_KEY || "";
const OLLAMA = process.env.OLLAMA_URL || "http://127.0.0.1:11434";
const PORT = parseInt(process.env.PORT || "8787", 10);
const MAX_CONCURRENT = parseInt(process.env.MAX_CONCURRENT || "12", 10); // in-flight to GPU
const MAX_QUEUE = parseInt(process.env.MAX_QUEUE || "200", 10);          // waiting room
const PAID_TIERS = ["pro_developer", "security_researcher", "enterprise"];
const OK_STATUS = ["active", "trialing", "past_due"]; // past_due = grace

// ── Fair FIFO concurrency queue ──────────────────────────────────────────────
let active = 0;
const waiters = [];
function acquire() {
  return new Promise((resolve, reject) => {
    if (active < MAX_CONCURRENT) { active++; return resolve(); }
    if (waiters.length >= MAX_QUEUE) return reject(new Error("overloaded"));
    waiters.push(resolve);
  });
}
function release() {
  active--;
  const next = waiters.shift();
  if (next) { active++; next(); }
}

// ── Auth (cached) ────────────────────────────────────────────────────────────
const cache = new Map(); // jwt -> { v, t }
async function authorize(jwt) {
  if (!jwt) return { ok: false, code: 401, msg: "no token" };
  const hit = cache.get(jwt);
  if (hit && hit.t > Date.now()) return hit.v;
  let v;
  try {
    const u = await fetch(`${SUPABASE_URL}/auth/v1/user`, {
      headers: { apikey: ANON, Authorization: `Bearer ${jwt}` },
    }).then((r) => (r.ok ? r.json() : null));
    if (!u || !u.id) {
      v = { ok: false, code: 401, msg: "invalid token" };
    } else {
      const sub = await fetch(
        `${SUPABASE_URL}/rest/v1/subscriptions?user_id=eq.${u.id}&select=tier,status`,
        { headers: { apikey: SERVICE, Authorization: `Bearer ${SERVICE}` } }
      ).then((r) => r.json()).then((r) => (Array.isArray(r) ? r[0] : null));
      const ok = sub && OK_STATUS.includes(sub.status) &&
        (PAID_TIERS.includes(sub.tier) || sub.status === "trialing");
      v = ok ? { ok: true, uid: u.id, tier: sub.tier }
             : { ok: false, code: 402, msg: "active subscription or trial required" };
    }
  } catch (e) {
    v = { ok: false, code: 503, msg: "auth backend unavailable" };
  }
  cache.set(jwt, { v, t: Date.now() + (v.ok ? 60000 : 30000) }); // cache 60s ok / 30s deny
  return v;
}

function readBody(req) {
  return new Promise((resolve) => {
    const chunks = [];
    req.on("data", (c) => chunks.push(c));
    req.on("end", () => resolve(Buffer.concat(chunks)));
  });
}

const server = http.createServer(async (req, res) => {
  if (req.url === "/healthz") { res.writeHead(200); return res.end("ok"); }
  if (!req.url.startsWith("/v1/") && !req.url.startsWith("/api/")) {
    res.writeHead(404, { "content-type": "application/json" });
    return res.end(JSON.stringify({ error: "not found" }));
  }
  const jwt = (req.headers["authorization"] || "").replace(/^Bearer\s+/i, "");
  const auth = await authorize(jwt);
  if (!auth.ok) {
    res.writeHead(auth.code, { "content-type": "application/json" });
    return res.end(JSON.stringify({ error: auth.msg }));
  }

  try {
    await acquire();
  } catch {
    res.writeHead(503, { "content-type": "application/json", "retry-after": "5" });
    return res.end(JSON.stringify({ error: "server busy, retry shortly" }));
  }

  try {
    const body = req.method === "POST" ? await readBody(req) : undefined;
    const up = await fetch(`${OLLAMA}${req.url}`, {
      method: req.method,
      headers: { "content-type": "application/json" },
      body,
    });
    res.writeHead(up.status, { "content-type": up.headers.get("content-type") || "application/json" });
    if (up.body) {
      const reader = up.body.getReader();
      // eslint-disable-next-line no-constant-condition
      while (true) {
        const { done, value } = await reader.read();
        if (done) break;
        res.write(Buffer.from(value));
      }
    }
    res.end();
  } catch (e) {
    if (!res.headersSent) res.writeHead(502, { "content-type": "application/json" });
    res.end(JSON.stringify({ error: String(e && e.message || e) }));
  } finally {
    release();
  }
});

server.listen(PORT, "127.0.0.1", () =>
  console.log(`cyberifrit-gateway: 127.0.0.1:${PORT} -> ${OLLAMA} (max ${MAX_CONCURRENT}, queue ${MAX_QUEUE})`)
);
