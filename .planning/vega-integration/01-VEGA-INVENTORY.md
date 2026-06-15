# Vega Inventory & JS API Contract

> Exact catalog of what `./Vega/` contains and the API our Rust engine must expose.
> This is the **spec** the engine implements. Don't guess — everything here was extracted from source.

## Source Layout (`./Vega/`)
- `platform/com.subgraph.vega.*` — 23 Eclipse RCP plugins (Java). Reference only; we reimplement in Rust.
- `scripts/scanner/` — **the valuable part.** JS modules + prelude libs.
  - `scripts/scanner/prelude/` — `base64.js`, `jquery.js`, `parseuri.js` (helper libs injected into JS ctx)
  - `scripts/scanner/modules/injection/` — 18 active attack modules
  - `scripts/scanner/modules/response/` — 28 passive response-processor modules
- `xml/alerts/` — 85 alert definitions (title/class/severity/impact/remediation/discussion)
- `templates/*.ftl` — FreeMarker report templates (HTML report). Port later to React/MD.

## Module Catalog (46 total)

### Injection modules (active — `category: "Injection Modules"`)
`bash-inject`, `code-injection`, `command-injection`, `cross-domain-policy-audit`,
`format-string-inject`, `header-inject`, `http-trace`, `integer-overflow-inject`,
`local-file-include`, `remote-file-include`, `shell-injection`, `sql-arithmetic-inject`,
`sql-text-injection`, `sql-timing-injection`, `url-injection`, `xml-injection`,
`xpath-injection`, `xss-injection`

### Response modules (passive — `type: "response-processor"`)
`insecure-script-include`, `vauthhttp`, `vautocomplete`, `vdirlist`, `vfileupload`,
`vhttpauth`, `vinfo-1918`, `vinfo-ajax`, `vinfo-blank`, `vinfo-comments`,
`vinfo-cookie-scope`, `vinfo-cookie`, `vinfo-crossdomain`, `vinfo-emails`,
`vinfo-errorpages`, `vinfo-feeds`, `vinfo-headers`, `vinfo-metatags`,
`vinfo-missing-charset`, `vinfo-oracle`, `vinfo-paths`, `vinfo-source`,
`vinfo-unsafe-charset`, `vinfo-wsdl`, `vinfo-xframeoptions`, `vpii-cc`,
`vpii-ssnsin`, `vvcs-users`

## Module Shape

**Injection module** declares `var module = {name, category, differential?}` and implements:
- `initialize(ctx)` — entry point. Gets `ps = ctx.getPathState()`, checks `ps.isParametric()`, then submits altered requests.
- `process(req, res, ctx)` — callback per response. Does differential/fingerprint analysis, raises `ctx.alert(...)`.

**Response module** declares `var module = {name, type:"response-processor"}` and implements:
- `run(request, response, ctx)` — called for every crawled response. Inspects headers/body, raises alerts.

## 🔑 Engine API Contract (what `js_runtime.rs` MUST expose)

These are every `ctx.*` and `ps.*` method the 46 modules call (extracted by grep, with usage counts).
Implementing these = all modules run unchanged.

### `ctx` (ModuleContext) — scan/request orchestration
| Method | Uses | Purpose |
|---|---|---|
| `ctx.alert(type, req, res, opts)` | 70 | Raise a finding. `opts={output,key,resource,detectiontype,...}` |
| `ctx.getSavedResponse(i)` | 55 | Retrieve stored response by index |
| `ctx.submitAlteredRequest(cb, payload, append, index)` | 43 | Fuzz: send req with param mutated by `payload` |
| `ctx.isFingerprintMatch(i, j)` | 39 | Compare two saved responses' fingerprints (differential core) |
| `ctx.getSavedRequest(i)` | 37 | Retrieve stored request by index |
| `ctx.responseChecks(i)` | 31 | Run passive response modules on saved response i |
| `ctx.addStringHighlight(s)` | 31 | Mark substring in response for UI |
| `ctx.getPathState()` | 29 | Get the PathState (`ps`) |
| `ctx.setModuleFailed()` | 12 | Abort this module |
| `ctx.incrementResponseCount()` | 12 | Bump + return count of received responses |
| `ctx.hasModuleFailed()` | 12 | Guard |
| `ctx.addRequestResponse(req, res)` | 12 | Save a req/res pair for later differential |
| `ctx.error(req, res, msg)` | 11 | Log an error against this scan step |
| `ctx.submitMultipleAlteredRequests(cb, payloads[], append)` | 9 | Batch fuzz |
| `ctx.addRegexCaseInsensitiveHighlight(re)` | 8 | UI highlight by regex |
| `ctx.submitRequest(req, cb, index)` | 6 | Send a prebuilt request |
| `ctx.getCurrentIndex()` | 6 | Index of current response in callback |
| `ctx.allResponsesReceived()` | 6 | True when all submitted reqs are back |
| `ctx.setIntegerProperty(k, v)` | 4 | Per-scan scratch storage |
| `ctx.alertExists(key)` | 4 | Dedupe guard by alert key |
| `ctx.isValidInternetDomainName(s)` | 2 | Helper |
| `ctx.internetDomainName(s)` | 2 | Helper |
| `ctx.contentChecks(req, res)` | 2 | Run content checks (XSS tag detection) |
| `ctx.getOrigResponse()` | 1 | The unmodified baseline response |
| `ctx.getIntegerProperty(k)` | 1 | Read scratch |

### `ps` (PathState) — the target path + its fuzzable parameters
| Method | Uses | Purpose |
|---|---|---|
| `ps.getFuzzableParameter()` | 20 | The param being fuzzed (`.name`) |
| `ps.getPath()` | 17 | Path object (`.getUri()`, `.isPostTarget()`) |
| `ps.isParametric()` | 16 | Does this path have params to fuzz? |
| `ps.decrementFuzzCounter()` | 6 | Concurrency/budget bookkeeping |
| `ps.incrementFuzzCounter()` | 5 | " |
| `ps.createAlteredRequest(payload, append)` | 4 | Build a mutated request |
| `ps.path` (property) | 3 | |
| `ps.registerXssRequest(req, xid)` | 2 | Track XSS probe by id |
| `ps.length` | 2 | |
| `ps.getPathFingerprint()` | 2 | Baseline fingerprint |
| `ps.createXssTag(payload?, xid)` | 2 | Build unique XSS marker |
| `ps.createRequest()` | 2 | |
| `ps.createRawRequest()` | 2 | |
| `ps.allocateXssId()` | 2 | Unique id for XSS correlation |
| `ps.isRootPath()` | 1 | |
| `ps.getResponse()` | 1 | |

### Request/Response objects (passed to modules)
- `request.requestLine.uri`, `req.addHeader(name, val)`
- `response.hasHeader(name)`, `response.getFirstHeader(name)` → `{name, value}`, `response.bodyAsString`, `response.fetchFail`

## Alert Definition Model (`xml/alerts/*.xml`, 85 files)
```xml
<alert>
  <title>...</title>
  <class>Environment|Injection|...</class>
  <severity>Info|Low|Medium|High|Critical</severity>
  <impact>...</impact>            (1..n)
  <remediation>...</remediation>
  <discussion>...</discussion>
</alert>
```
Filename (minus `.xml`) = the alert `type` key passed to `ctx.alert("vfileupload", ...)`.
→ `alerts.rs` parses these into `HashMap<String, AlertDefinition>`.

## Crawler (reference: `platform/com.subgraph.vega.crawler`)
7 Java classes: `WebCrawler`, `CrawlerTask`, `RequestConsumer`, `HttpResponseProcessor`,
`CrawlerPauseLock`, `TaskCounter`, `WebCrawlerFactory`. Producer/consumer model: a task queue of URIs,
N consumer workers fetch + parse links (jsoup) + enqueue new in-scope URIs. We reimplement with
`tokio` tasks + `reqwest` + an HTML link extractor (`scraper` crate or regex for v1).

## Proxy (reference: `platform/com.subgraph.vega.http.proxy`, 22 classes)
Intercepting MITM proxy. Records transactions, allows edit/replay, feeds scanner. v1 can be a simple
forward proxy that logs; TLS interception is a stretch goal.
