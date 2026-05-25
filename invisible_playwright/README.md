# invisible_playwright

[![tests](https://github.com/feder-cr/invisible_playwright/actions/workflows/tests.yml/badge.svg)](https://github.com/feder-cr/invisible_playwright/actions/workflows/tests.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Python 3.11+](https://img.shields.io/badge/python-3.11+-blue.svg)](https://www.python.org/downloads/)
[![Firefox 150.0.1](https://img.shields.io/badge/firefox-150.0.1-orange.svg)](https://www.mozilla.org/firefox/)
[![GitHub release](https://img.shields.io/github/v/release/feder-cr/invisible_playwright.svg)](https://github.com/feder-cr/invisible_playwright/releases)
[![GitHub stars](https://img.shields.io/github/stars/feder-cr/invisible_playwright.svg?style=social)](https://github.com/feder-cr/invisible_playwright/stargazers)
[![browser launches](https://img.shields.io/github/downloads/feder-cr/invisible_firefox/usage-counter/total?label=browser%20launches&color=blue)](https://github.com/feder-cr/invisible_firefox/releases/tag/usage-counter)

[![LinkedIn](https://img.shields.io/badge/LinkedIn-Federico%20Elia-0A66C2?logo=linkedin&logoColor=white)](https://it.linkedin.com/in/federico-elia-5199951b6)

A patched Firefox **100% Playwright-compatible** that passes the hardest browser-fingerprint detectors in the wild.


## Results

### Google reCAPTCHA v3 - **0.90 / 1.0**

Top-tier score. Google classifies the session as "very likely a human". Most anti-detect stacks plateau around 0.3-0.7.

![reCAPTCHA score 0.90](docs/screenshots/recaptcha_score.png)

### Fingerprint Pro - **bot: not detected, VPN: false, tampering: false, dev tools: not detected**

FingerprintJS Pro's full Smart Signals battery flips every flag to "Not detected". Browser correctly identified as Firefox 150 on Windows 10. Confidence score 0.9.

![FingerprintPro not detected](docs/screenshots/fingerprintpro.png)

### CreepJS - **0 lies**, fingerprint is internally coherent

No contradictions between headless hints, spoofed values, and real rendering output. That "0 lies" is what kills most anti-detect browsers: one inconsistency (e.g. Chrome UA + Firefox WebGL) and the trust score collapses.

![CreepJS 0 lies](docs/screenshots/creepjs.png)

### BrowserLeaks WebRTC - **no public IP leak**

WebRTC srflx address is the proxy egress IP; host candidates are private LAN. The real public IP never leaks via STUN, even on pages that configure their own ICE servers. Stock Firefox exposes an mDNS hostname (e.g. `abc-1234.local`) as a host ICE candidate, which is itself a stable per-session signal detectors fingerprint. invisible_playwright replaces host candidates with synthetic private-LAN IPs that match the spoofed network, removing the mDNS tell.

![WebRTC no leaks](docs/screenshots/webrtc.png)

### bot.sannysoft.com - **all checks pass**

Every row green: WebDriver not present, Chrome-only properties absent, plugin/mime/languages arrays coherent, permissions API correct, iframe/source window checks pass.

![Sannysoft all green](docs/screenshots/sannysoft.png)

---

## Why it's powerful

**Most anti-detect browsers patch Chromium at the JavaScript level** - they override `navigator`, `WebGLRenderingContext.getParameter`, canvas APIs, and so on via injected scripts. This has two fatal problems:

1. **JS patches are detectable.** Anti-bots enumerate native function `.toString()`, check descriptor configurability, compare property enumeration order, watch for prototype mutations. Every patch leaves a fingerprint of its own. CreepJS has an entire battery of "lies detectors" built around this.
2. **Chromium itself is now suspect.** Residential-proxy bot traffic is overwhelmingly Chromium-based, so detectors weight anything Chromium-shaped as risky by default. Chromium-based forks inherit Chrome's open-source layers (BoringSSL, Blink, V8, ANGLE) cleanly, but they still cannot fully match Chrome in practice: Chrome ships closed-source components on top (Widevine, proprietary codecs, Google Update / Safe Browsing endpoints) that flip detectable JS feature flags and network signals, and forks lag Chrome's release cadence by days to weeks, leaving telltale version-specific behaviours that detectors lock onto.

**invisible_playwright patches Firefox at the C++ level.** The spoofed values come back out through the normal Gecko paths - there is no JS shim, no override, no `Object.defineProperty`. **From the page's point of view, the browser is just telling the truth.** Anti-bot lie-detectors have nothing to latch onto.

invisible_playwright spoofs **all the layers that matter, together, coherently** — Navigator, screen, GPU/WebGL, Canvas, fonts, audio, WebRTC, timezone, DevTools detection, SOCKS5 auth, and the rest. See [feder-cr/invisible_firefox](https://github.com/feder-cr/invisible_firefox) for the full per-layer breakdown of which C++ files are patched and why.

Everything is driven by preferences - no hardcoded values in the binary. You change one pref, you change the spoofed value.

---

## How it compares

Commercial anti-detect browsers (Multilogin Mimic, GoLogin Orbita, AdsPower, Dolphin Anty) ship patched Chromium and apply most spoofing at the JavaScript layer. A few (Kameleo, Multilogin Stealthfox) also offer Firefox-based profiles, but the spoofing pattern is the same: runtime overrides on top of an unmodified rendering engine. That's the ceiling - and it's a low one.

| | invisible_playwright | Multilogin / GoLogin | AdsPower / Dolphin | Kameleo |
|---|---|---|---|---|
| Engine | Firefox (open source) | Chromium fork | Chromium fork | Chromium |
| Patch depth | C++ source | JS overrides | JS overrides | JS overrides |
| `.toString()` clean | ✅ Native Gecko path | ❌ Detectable shims | ❌ Detectable shims | ❌ Detectable shims |
| Canvas / WebGL | ✅ C++ level | ⚠️ JS override | ⚠️ JS override | ⚠️ JS override |
| SOCKS5 auth | ✅ Patched | ⚠️ Varies | ⚠️ Varies | ❌ |
| Self-hosted | ✅ | ❌ SaaS | ❌ SaaS | ❌ Cloud |
| reCAPTCHA v3 score | **0.90** | ~0.3-0.6 | ~0.3-0.5 | ~0.3-0.5 |
| FP Pro - bot detected | ✅ Not detected | ❌ Detected | ❌ Detected | ❌ Detected |
| FP Pro - tampering | ✅ Not detected | ❌ Detected | ❌ Detected | ❌ Detected |
| FP Pro - VPN flag | ✅ false | ❌ true | ❌ true | ❌ true |
| CreepJS lies | ✅ 0 | ❌ multiple | ❌ multiple | ❌ multiple |

Competitor scores reflect our own testing on Windows 10 against the same five detection suites used above; results may vary with their evolving builds.

---

## Install

```bash
pip install git+https://github.com/feder-cr/invisible_playwright.git
python -m invisible_playwright fetch      # one-time ~100 MB download, SHA256-verified
```

Supported platforms: **Windows x86_64**, **Linux x86_64**.

---

## Usage
### Random fingerprint per session
**100% Playwright-compatible** - sync and async, all methods, zero API changes. If you already use Playwright, switching is two lines:

```diff
- from playwright.sync_api import sync_playwright
- with sync_playwright() as p:
-     browser = p.firefox.launch()
+ from invisible_playwright import InvisiblePlaywright
+ with InvisiblePlaywright() as browser:
```

Every session gets a unique, coherent fingerprint drawn from real-world Firefox telemetry (GPU / audio / fonts / ~400 other fields) and Bezier-curve mouse motion baked into the browser itself.

**Sync**
```python
from invisible_playwright import InvisiblePlaywright

with InvisiblePlaywright(proxy={"server": "socks5://...", "username": "u", "password": "p"}) as browser:
    page = browser.new_page()
    page.goto("https://example.com")
    page.click("#submit")   # mouse arcs to the button on a Bezier curve
```

**Async**
```python
from invisible_playwright.async_api import InvisiblePlaywright

async with InvisiblePlaywright(proxy={"server": "socks5://...", "username": "u", "password": "p"}) as browser:
    page = await browser.new_page()
    await page.goto("https://example.com")
    await page.click("#submit")
```

The `browser` object is a `playwright.sync_api.Browser` / `playwright.async_api.Browser` - every Playwright method works as-is.

---

### Random fingerprint per session

```python
from invisible_playwright import InvisiblePlaywright

with InvisiblePlaywright() as browser:
    page = browser.new_page()
    page.goto("https://creepjs-api.web.app")
```

Every call samples a new coherent profile. Log the seed to reproduce interesting runs:

```python
sf = InvisiblePlaywright()
with sf as browser:
    print("seed =", sf.seed)
    # ...
```

### Reproducible fingerprint

```python
with InvisiblePlaywright(seed=42) as browser:
    ...   # same GPU, same canvas hash, same audio context, every run
```

### Proxies

```python
proxy = {
    "server": "socks5://gate.example.com:1080",
    "username": "user",
    "password": "pass",
}
with InvisiblePlaywright(proxy=proxy) as browser:
    ...
```

Schemes supported: `socks5`, `socks4`, `http`, `https`. Auth works on all of them (SOCKS5 via patched `nsProtocolProxyService.cpp`, HTTP/HTTPS via Playwright). DNS is routed through the proxy by default, no local leak.

### Pinning specific fingerprint fields

By default everything comes from `seed`. To force specific values while the rest stays seed-derived:

```python
with InvisiblePlaywright(
    seed=42,
    pin={
        "gpu.renderer": "ANGLE (NVIDIA, NVIDIA GeForce RTX 4090 Direct3D11)",
        "gpu.vendor":   "Google Inc. (NVIDIA)",
        "screen.width":  2560,
        "screen.height": 1440,
        "hardware.concurrency": 16,
    },
) as browser:
    ...
```

Full list of pinnable keys, how pinning interacts with the Bayesian sampler, and common patterns are in **[docs/pinning.md](docs/pinning.md)**.

---

## CLI

```bash
invisible_playwright fetch          # download the binary if missing
invisible_playwright path           # print the absolute path to the cached binary
invisible_playwright version        # wrapper and binary versions
invisible_playwright clear-cache    # remove all cached binaries
```

## Related projects

invisible_playwright takes a different angle than the major Firefox-hardening projects but stands on their shoulders:

- **[arkenfox/user.js](https://github.com/arkenfox/user.js)** - the canonical Firefox configuration for privacy/security hardening via prefs. Reading arkenfox is how you understand which `user.js` knobs matter; invisible_playwright goes further by patching the C++ source where prefs alone are insufficient (Canvas noise, WebGL parameter overrides, font whitelisting, WebRTC IP swap, DevTools detection bypass).
- **[LibreWolf](https://librewolf.net)** - a Firefox fork bundled with sensible privacy defaults. Same audience, different distribution model: LibreWolf ships a configured Firefox binary, invisible_playwright ships source patches + a wrapper for automation.
- **[Camoufox](https://github.com/daijro/camoufox)** - the most well-known open-source anti-detect Firefox project. We share design goals on the fingerprint-spoofing side; the implementation approach differs (Camoufox patches a wider surface and ships its own fingerprint database, while invisible_playwright sticks closer to vanilla and drives spoofing from a Bayesian sampler).

---

## License

MIT - see [LICENSE](LICENSE). The patched Firefox binary is distributed under the MPL-2.0 (Firefox upstream license). The C++ patches against mozilla-central that produce that binary are at [feder-cr/invisible_firefox](https://github.com/feder-cr/invisible_firefox).
