#!/usr/bin/env python3
"""Browser sidecar for VSCodium-Rust.

Drives invisible_playwright (a patched, stealth Firefox that beats bot/
fingerprint detection) through a line-delimited JSON protocol:

  stdin  : one JSON request per line  -> {"action": "...", "args": {...}}
  stdout : one JSON response per line  -> {"ok": true, "result": {...}}
                                        or {"ok": false, "error": "..."}

ALL diagnostics (Playwright logs, Firefox download progress) go to stderr so
stdout stays a clean response channel the Rust side can parse line-by-line.

This is the browser engine behind the agent's web-pentest tools. It runs a real
browser: JS executes, the DOM is the rendered DOM, screenshots are real pixels,
and `http` sends crafted requests through the browser's network stack.
"""
import sys
import json
import base64
import traceback

_ip = None
_browser = None
_page = None
_last_headers = {}
_last_status = None


def log(*a):
    print(*a, file=sys.stderr, flush=True)


def respond(obj):
    sys.stdout.write(json.dumps(obj) + "\n")
    sys.stdout.flush()


def ensure_browser(args):
    global _ip, _browser, _page
    if _page is not None:
        return
    from invisible_playwright import InvisiblePlaywright
    # headless=False (default) = a REAL visible Firefox window — this is the
    # user-facing browser. The agent drives this same instance, so what the agent
    # does is visible. Pass headless=True only for pure background automation.
    kwargs = {
        "humanize": bool(args.get("humanize", True)),
        "headless": bool(args.get("headless", False)),
    }
    if args.get("seed") is not None:
        kwargs["seed"] = args["seed"]
    if args.get("proxy"):
        kwargs["proxy"] = args["proxy"]
    log("[sidecar] launching stealth Firefox (visible window; first run downloads it)...")
    _ip = InvisiblePlaywright(**kwargs)
    _browser = _ip.__enter__()
    _page = _browser.new_page()

    # Some navigations (redirects, prerender, cached docs) make `page.goto`
    # return None even though a real main-frame document response arrived. A
    # standing response listener captures the latest main-frame navigation
    # response so `navigate` always has a status + headers to audit, even when
    # goto's own return is null. Last navigation response wins (final URL).
    def _on_response(resp):
        global _last_status, _last_headers
        try:
            req = resp.request
            if req.is_navigation_request() and resp.frame == _page.main_frame:
                _last_status = resp.status
                _last_headers = dict(resp.headers)
        except Exception:
            pass

    _page.on("response", _on_response)
    log("[sidecar] browser ready")


def do(action, args):
    global _page, _ip, _browser, _last_headers, _last_status

    if action == "open":
        ensure_browser(args)
        return {"status": "ready"}

    if action == "close":
        try:
            if _ip is not None:
                _ip.__exit__(None, None, None)
        finally:
            _ip = _browser = _page = None
        return {"status": "closed"}

    ensure_browser(args)

    if action == "navigate":
        url = args["url"]
        # Clear stale audit data; the response listener (or goto below)
        # repopulates it for THIS navigation.
        _last_status = None
        _last_headers = {}
        resp = _page.goto(
            url,
            wait_until=args.get("wait_until", "domcontentloaded"),
            timeout=int(args.get("timeout", 45000)),
        )
        # Prefer goto's direct response; fall back to whatever the standing
        # response listener captured (handles goto -> None on the main frame).
        if resp is not None:
            _last_status = resp.status
            _last_headers = dict(resp.headers)
        return {
            "url": _page.url,
            "status": _last_status,
            "headers": _last_headers,
            "title": _page.title(),
        }

    if action == "content":
        has_body = _page.query_selector("body") is not None
        return {
            "url": _page.url,
            "title": _page.title(),
            "html": _page.content(),
            "text": (_page.inner_text("body")[:8000] if has_body else ""),
            "status": _last_status,
            "headers": _last_headers,
        }

    if action == "screenshot":
        png = _page.screenshot(full_page=bool(args.get("full_page", False)))
        return {"screenshot": base64.b64encode(png).decode()}

    if action == "click":
        _page.click(args["selector"], timeout=int(args.get("timeout", 8000)))
        return {"clicked": args["selector"], "url": _page.url}

    if action == "fill":
        _page.fill(args["selector"], args.get("text", ""), timeout=int(args.get("timeout", 8000)))
        return {"filled": args["selector"]}

    if action == "press":
        _page.keyboard.press(args.get("key", "Enter"))
        return {"pressed": args.get("key", "Enter"), "url": _page.url}

    if action == "eval":
        val = _page.evaluate(args["script"])
        try:
            json.dumps(val)
        except Exception:
            val = str(val)
        return {"value": val}

    if action == "links":
        links = _page.eval_on_selector_all(
            "a",
            "els => els.map(e => ({text:(e.innerText||'').trim().slice(0,120), href:e.href}))",
        )
        return {"links": links[:300]}

    if action == "forms":
        forms = _page.eval_on_selector_all(
            "form",
            """els => els.map(f => ({
                action: f.action,
                method: (f.getAttribute('method') || 'get').toLowerCase(),
                inputs: Array.from(f.querySelectorAll('input,textarea,select')).map(i => ({
                    name: i.name, type: (i.type || i.tagName.toLowerCase()), id: i.id
                }))
            }))""",
        )
        return {"forms": forms}

    if action == "cookies":
        return {"cookies": _page.context.cookies()}

    if action == "http":
        # Send a crafted request through the browser's network context (carries
        # the stealth profile + cookies). Useful for authenticated probing.
        method = str(args.get("method", "GET")).upper()
        url = args["url"]
        kw = {}
        if args.get("headers"):
            kw["headers"] = args["headers"]
        if args.get("data") is not None:
            kw["data"] = args["data"]
        r = _page.request.fetch(url, method=method, **kw)
        return {"status": r.status, "headers": dict(r.headers), "body": r.text()[:12000]}

    raise ValueError(f"unknown action: {action}")


def main():
    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        try:
            req = json.loads(line)
        except Exception as e:
            respond({"ok": False, "error": f"bad json: {e}"})
            continue
        try:
            result = do(req.get("action", ""), req.get("args", {}) or {})
            respond({"ok": True, "result": result})
        except Exception as e:
            log("[sidecar] error:", traceback.format_exc())
            respond({"ok": False, "error": f"{type(e).__name__}: {e}"})


if __name__ == "__main__":
    main()
