"""Fingerprint surface tests — replicate the checks performed by the canonical
anti-bot detection libraries against an OFFLINE browser session.

Each test asserts the SAME thing the upstream detector would flag. A pass
here means our patched build appears human to that detector; a fail
means a real stealth hole that anti-bot kits would exploit in production.

Detector libraries studied (all FOSS, MIT-licensed):
  - github.com/fingerprintjs/BotD            — 19 detectors, the most
                                                widely deployed client-side
                                                bot detector
  - github.com/abrahamjuliot/creepjs         — headless / stealth / lies
                                                modules
  - github.com/fingerprintjs/fingerprintjs   — canvas / audio / color /
                                                touch consistency
  - github.com/antoinevastel/fpscanner       — UA / platform / oscpu
                                                cross-checks
  - bot.sannysoft.com                        — classic Puppeteer harness

Everything runs against `about:blank` with NO network and NO proxy. The
suite is intended to be part of the release-gate: pre-push hook runs
`pytest -m e2e` and these tests must be green on every release.

Run only this file:
    pytest tests/test_fingerprint_surface.py -m e2e -v
"""
from __future__ import annotations

import re
import sys

import pytest

from invisible_playwright import InvisiblePlaywright
from invisible_playwright.constants import BINARY_ENTRY_REL


# ────────────────────────────────────────────────────────────────────
# Inline PIN — a coherent mid-range Windows desktop. Not user-config:
# these specific values are what the surface tests assert against.
# Keep PIN small (only fields that JS exposes) and stable across runs.
# ────────────────────────────────────────────────────────────────────

PIN = {
    "screen.width": 1920,
    "screen.height": 1080,
    "screen.avail_width": 1920,
    "screen.avail_height": 1040,
    "screen.dpr": 1.0,
    "hardware.concurrency": 8,
    "audio.sample_rate": 48000,
    "audio.max_channel_count": 2,
}


@pytest.fixture(scope="session")
def firefox_binary():
    """Locate the patched Firefox binary. Three lookup paths:
      1. ``INVPW_BINARY_PATH`` env var (for dev iteration against a local build)
      2. Cached binary under ``cache_dir_for_version()`` (post-fetch)
      3. Skip cleanly (no implicit network download)."""
    import os
    env_path = os.environ.get("INVPW_BINARY_PATH")
    if env_path:
        from pathlib import Path
        if Path(env_path).exists():
            return env_path
        pytest.skip(f"INVPW_BINARY_PATH={env_path!r} does not exist")
    if sys.platform not in BINARY_ENTRY_REL:
        pytest.skip(f"unsupported platform: {sys.platform}")
    from invisible_playwright.download import cache_dir_for_version
    entry = cache_dir_for_version() / BINARY_ENTRY_REL[sys.platform]
    if not entry.exists():
        pytest.skip(
            "patched Firefox not cached; run "
            "`python -m invisible_playwright fetch` first, or set "
            "INVPW_BINARY_PATH to a local build"
        )
    return str(entry)


@pytest.fixture(scope="module")
def page(firefox_binary):
    """One headless browser shared across the whole module.
    ~20s startup paid once, then every test runs in ~50ms."""
    with InvisiblePlaywright(
        seed=42,
        pin=PIN,
        binary_path=firefox_binary,
        headless=True,
    ) as browser:
        ctx = browser.new_context()
        p = ctx.new_page()
        p.goto("about:blank", timeout=30_000)
        yield p


def _ev(page, expr):
    return page.evaluate(expr)


# ===========================================================================
# BotD detectors (github.com/fingerprintjs/BotD/tree/main/src/detectors)
# Each detector becomes one pytest. The failure name maps to the BotKind
# constant BotD would emit on the wire.
# ===========================================================================


@pytest.mark.e2e
def test_botd_webdriver_property_is_falsey(page):
    """BotD: navigator.webdriver === true → HeadlessChrome verdict."""
    assert not _ev(page, "navigator.webdriver"), (
        "navigator.webdriver is truthy — instant HeadlessChrome verdict"
    )


@pytest.mark.e2e
def test_botd_app_version_no_headless_token(page):
    """BotD detectAppVersion: /headless|electron|slimerjs/i in appVersion."""
    av = _ev(page, "navigator.appVersion")
    for token in ("headless", "electron", "slimerjs"):
        assert not re.search(token, av, re.I), (
            f"navigator.appVersion contains {token!r}: {av!r}"
        )


@pytest.mark.e2e
def test_botd_user_agent_no_headless_or_selenium_token(page):
    """BotD: /headless|selenium|phantom/i in UA."""
    ua = _ev(page, "navigator.userAgent")
    for token in ("headless", "selenium", "phantom"):
        assert not re.search(token, ua, re.I), (
            f"navigator.userAgent contains {token!r}: {ua!r}"
        )


@pytest.mark.e2e
def test_botd_function_bind_is_function(page):
    """BotD detectFunctionBind: missing Function.prototype.bind = PhantomJS."""
    assert _ev(page, "typeof Function.prototype.bind === 'function'")


@pytest.mark.e2e
def test_botd_product_sub_is_gecko_value(page):
    """BotD detectProductSub: Firefox must return '20100101'; '20030107'
    on a Firefox UA = Chrome-stub leaked under spoof."""
    assert _ev(page, "navigator.productSub") == "20100101", (
        "navigator.productSub must be '20100101' on Firefox"
    )


@pytest.mark.e2e
def test_botd_no_process_global(page):
    """BotD detectProcess: window.process indicates Electron."""
    assert not _ev(page,
        "typeof window.process !== 'undefined' && "
        "window.process.type === 'renderer'"
    )
    assert not _ev(page,
        "typeof window.process !== 'undefined' && "
        "window.process.versions != null && "
        "typeof window.process.versions.electron !== 'undefined'"
    )


@pytest.mark.e2e
def test_botd_eval_length_matches_engine(page):
    """BotD detectEvalLengthInconsistency: `eval.toString().length` must be
    37 on Gecko (33 on Chromium, 39 on IE). Mismatch = engine spoof."""
    assert _ev(page, "eval.toString().length") == 37


@pytest.mark.e2e
def test_botd_languages_array_non_empty(page):
    """BotD detectLanguagesLengthInconsistency: empty navigator.languages
    is the classic HeadlessChrome tell."""
    assert _ev(page, "navigator.languages.length") > 0


@pytest.mark.e2e
def test_botd_plugins_instance_of_PluginArray(page):
    """BotD detectPluginsArray: navigator.plugins must be a real PluginArray."""
    assert _ev(page, "navigator.plugins instanceof PluginArray")


@pytest.mark.e2e
def test_botd_mime_types_consistent_prototype_chain(page):
    """BotD areMimeTypesConsistent: navigator.mimeTypes and each entry
    must have proper prototype chain. Spoofers using plain arrays fail."""
    consistent = _ev(page, """() => {
        if (typeof navigator.mimeTypes === 'undefined' ||
            typeof MimeTypeArray === 'undefined') return false;
        let ok = Object.getPrototypeOf(navigator.mimeTypes) === MimeTypeArray.prototype;
        for (let i = 0; i < navigator.mimeTypes.length; i++) {
            ok = ok && Object.getPrototypeOf(navigator.mimeTypes[i]) === MimeType.prototype;
        }
        return ok;
    }""")
    assert consistent, "navigator.mimeTypes prototype chain inconsistent"


@pytest.mark.e2e
def test_botd_no_distinctive_window_props(page):
    """BotD checkDistinctiveProperties: scan window for automation globals."""
    DISTINCTIVE = [
        "awesomium", "RunPerfTest", "CefSharp", "fmget_targets", "geb",
        "__nightmare", "nightmare", "__phantomas", "callPhantom", "_phantom",
        "wdioElectron", "__webdriverFunc", "_WEBDRIVER_ELEM_CACHE",
        "ChromeDriverw", "domAutomation", "domAutomationController",
    ]
    leaks = [n for n in DISTINCTIVE
             if _ev(page, f"typeof window[{n!r}] !== 'undefined'")]
    assert not leaks, f"Distinctive bot globals leaked: {leaks}"


@pytest.mark.e2e
def test_botd_no_distinctive_document_props(page):
    """BotD: document-side automation globals (webdriver/selenium/cdc)."""
    DOC_LEAKS = [
        "__webdriver_evaluate", "__selenium_evaluate",
        "__webdriver_script_function", "__webdriver_script_func",
        "__webdriver_script_fn", "__fxdriver_evaluate",
        "__driver_unwrapped", "__webdriver_unwrapped",
        "__driver_evaluate", "__selenium_unwrapped",
        "__fxdriver_unwrapped",
        "$cdc_asdjflasutopfhvcZLmcf", "$cdc_asdjflasutopfhvcZLmcfl_",
        "$chrome_asyncScriptInfo", "__$webdriverAsyncExecutor",
    ]
    leaks = [n for n in DOC_LEAKS
             if _ev(page, f"typeof document[{n!r}] !== 'undefined'")]
    assert not leaks, f"document carries automation property names: {leaks}"


@pytest.mark.e2e
def test_botd_document_html_attributes_clean(page):
    """BotD detectDocumentAttributes: html element attrs contain 'selenium'
    / 'webdriver' / 'driver' → Selenium verdict."""
    attrs = _ev(page,
        "Array.from(document.documentElement.attributes).map(a => a.name + '=' + a.value)")
    bad = [a for a in attrs if any(t in a.lower()
                                    for t in ("selenium", "webdriver", "driver"))]
    assert not bad, f"HTML attributes contain bot tokens: {bad}"


@pytest.mark.e2e
def test_botd_window_size_nonzero(page):
    """BotD detectWindowSize: headless without window manager → 0x0."""
    ow = _ev(page, "window.outerWidth")
    oh = _ev(page, "window.outerHeight")
    assert ow > 0 and oh > 0, (
        f"outerWidth/Height = {ow}/{oh} — headless without window manager"
    )


@pytest.mark.e2e
def test_botd_webgl_debug_renderer_info_available(page):
    """BotD detectWebGL: WEBGL_debug_renderer_info extension must exist."""
    has_ext = _ev(page, """() => {
        const c = document.createElement('canvas');
        const gl = c.getContext('webgl') || c.getContext('experimental-webgl');
        return !!gl && !!gl.getExtension('WEBGL_debug_renderer_info');
    }""")
    assert has_ext


# ===========================================================================
# sannysoft.com — classic Puppeteer detection harness
# ===========================================================================


@pytest.mark.e2e
def test_sannysoft_chrome_object_consistency(page):
    """Firefox UA + window.chrome present = bot-framework leak."""
    if "Firefox" in _ev(page, "navigator.userAgent"):
        assert not _ev(page, "typeof window.chrome !== 'undefined'")


@pytest.mark.e2e
def test_sannysoft_permissions_query_works(page):
    """navigator.permissions.query() must return a proper PermissionStatus."""
    ok = _ev(page, """async () => {
        if (!navigator.permissions || !navigator.permissions.query) return false;
        try {
            const r = await navigator.permissions.query({name: 'notifications'});
            return r && typeof r.state === 'string';
        } catch (e) { return false; }
    }""")
    assert ok


@pytest.mark.e2e
def test_sannysoft_iframe_chrome_not_leaked(page):
    """iframe.contentWindow.chrome must not leak on Firefox UA."""
    if "Firefox" not in _ev(page, "navigator.userAgent"):
        pytest.skip("Firefox-only invariant")
    leaks = _ev(page, """() => {
        const iframe = document.createElement('iframe');
        iframe.style.display = 'none';
        document.body.appendChild(iframe);
        const is = typeof iframe.contentWindow.chrome !== 'undefined';
        document.body.removeChild(iframe);
        return is;
    }""")
    assert not leaks


@pytest.mark.e2e
def test_sannysoft_iframe_languages_not_empty(page):
    """Iframe-scope navigator.languages must have ≥1 entry."""
    n = _ev(page, """() => {
        const f = document.createElement('iframe');
        f.style.display = 'none';
        document.body.appendChild(f);
        const len = f.contentWindow.navigator.languages.length;
        document.body.removeChild(f);
        return len;
    }""")
    assert n > 0


# ===========================================================================
# FingerprintJS — fingerprint surface coherence
# ===========================================================================


@pytest.mark.e2e
def test_fpjs_canvas_2d_context_returns_valid(page):
    ok = _ev(page, """() => {
        const c = document.createElement('canvas');
        c.width = 100; c.height = 100;
        const ctx = c.getContext('2d');
        if (!ctx) return false;
        ctx.fillText('test', 10, 10);
        const data = c.toDataURL();
        return data.length > 100 && data.startsWith('data:image/png;base64');
    }""")
    assert ok


@pytest.mark.e2e
def test_fpjs_audio_context_works(page):
    ok = _ev(page, """async () => {
        try {
            const ctx = new (window.OfflineAudioContext ||
                              window.webkitOfflineAudioContext)(1, 5000, 44100);
            const osc = ctx.createOscillator();
            osc.connect(ctx.destination);
            osc.start(0);
            const buf = await ctx.startRendering();
            return buf && buf.length > 0;
        } catch (e) { return false; }
    }""")
    assert ok


@pytest.mark.e2e
def test_fpjs_color_gamut_query_works(page):
    """matchMedia('(color-gamut: ...)') must match at least srgb."""
    ok = _ev(page, """matchMedia('(color-gamut: srgb)').matches ||
                      matchMedia('(color-gamut: p3)').matches ||
                      matchMedia('(color-gamut: rec2020)').matches""")
    assert ok


@pytest.mark.e2e
def test_fpjs_screen_color_depth_realistic(page):
    """Atypical color depths are headless-distinctive."""
    cd = _ev(page, "screen.colorDepth")
    assert cd in (24, 30, 32)


# ===========================================================================
# PIN-locked surfaces (the values declared in PIN above)
# ===========================================================================


@pytest.mark.e2e
def test_pin_screen_width_lands_in_screen_object(page):
    assert _ev(page, "screen.width") == PIN["screen.width"]


@pytest.mark.e2e
def test_pin_screen_height_lands_in_screen_object(page):
    assert _ev(page, "screen.height") == PIN["screen.height"]


@pytest.mark.e2e
def test_pin_hardware_concurrency_lands_in_navigator(page):
    assert (_ev(page, "navigator.hardwareConcurrency")
            == PIN["hardware.concurrency"])


@pytest.mark.e2e
def test_pin_audio_sample_rate_lands_in_AudioContext(page):
    assert _ev(page,
        "(new (window.AudioContext||window.webkitAudioContext)()).sampleRate"
    ) == PIN["audio.sample_rate"]


@pytest.mark.e2e
def test_pin_audio_max_channels_lands_in_destination(page):
    assert _ev(page,
        "(new (window.AudioContext||window.webkitAudioContext)())"
        ".destination.maxChannelCount"
    ) == PIN["audio.max_channel_count"]


# ===========================================================================
# fpscanner-style cross-checks
# ===========================================================================


@pytest.mark.e2e
def test_fpscanner_ua_vs_platform_consistent(page):
    """UA OS substring must agree with navigator.platform OS substring."""
    ua = _ev(page, "navigator.userAgent")
    platform = _ev(page, "navigator.platform")
    if "Windows" in ua:
        assert "Win" in platform, f"UA Win but platform={platform!r}"
    elif "Mac" in ua:
        assert "Mac" in platform
    elif "Linux" in ua:
        assert "Linux" in platform or "X11" in platform


@pytest.mark.e2e
def test_fpscanner_no_userAgentData_on_firefox(page):
    """navigator.userAgentData is Chromium-only. Presence on Firefox UA = bot."""
    if "Firefox" in _ev(page, "navigator.userAgent"):
        assert not _ev(page, "'userAgentData' in navigator")
