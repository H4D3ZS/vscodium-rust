"""Integration tests — multi-module pipelines without a real browser.

These tests verify that the fingerprint sampler, Profile dataclass, prefs
translation and proxy translation compose correctly. They do NOT launch
Firefox. Browser-lifecycle tests live in ``test_e2e.py``.

Scope: Windows, Linux, and platform-agnostic. Platform-specific tests
monkeypatch ``sys.platform`` so the same suite exercises both branches
regardless of the host OS.
"""
from __future__ import annotations

import random
import sys

import pytest

from invisible_playwright._fpforge import generate_profile
from invisible_playwright._proxy import configure_proxy
from invisible_playwright.prefs import (
    _WIN_LIGHT_COLORS,
    translate_profile_to_prefs,
)


# Keys every Profile-derived prefs dict MUST carry. Sourced from
# ``translate_profile_to_prefs`` direct writes (not from _BASELINE) plus
# a couple of baseline keys that callers commonly read.
_REQUIRED_PREFS_KEYS = (
    "zoom.stealth.screen.width",
    "zoom.stealth.screen.height",
    "zoom.stealth.screen.avail_width",
    "zoom.stealth.screen.avail_height",
    "zoom.stealth.screen.dpr",
    "layout.css.devPixelsPerPx",
    "zoom.stealth.hw_concurrency",
    "zoom.stealth.storage.quota_mb",
    "zoom.stealth.audio.sample_rate",
    "zoom.stealth.audio.output_latency_ms",
    "zoom.stealth.audio.max_channel_count",
    "media.av1.enabled",
    "media.encoder.webm.enabled",
    "media.mediasource.webm.enabled",
    "media.mediasource.mp4.enabled",
    "zoom.stealth.font.whitelist",
    "zoom.stealth.font.metrics",
    "ui.systemUsesDarkTheme",
    "intl.accept_languages",
    "general.useragent.locale",
    "intl.locale.requested",
    "zoom.stealth.seed",
    "zoom.stealth.fpp.hw_seed",
    "zoom.stealth.webrtc.host_ip",
    "zoom.stealth.webgl.renderer",
    "zoom.stealth.webgl.vendor",
    "zoom.stealth.webgl.msaa",
    "zoom.stealth.canvas.noise_skip_mask",
    # baseline sanity
    "privacy.resistFingerprinting",
    "media.peerconnection.enabled",
    "general.useragent.override",
)


# ──────────────────────────────────────────────────────────────────────
#  IT1: profile → prefs pipeline yields a complete prefs dict
# ──────────────────────────────────────────────────────────────────────


@pytest.mark.integration
def test_generate_profile_then_translate_has_all_required_keys():
    """IT1 — generate_profile → translate_profile_to_prefs succeeds and the
    returned dict contains every key downstream code (Playwright, the C++
    patches) needs to find."""
    profile = generate_profile(seed=42)
    prefs = translate_profile_to_prefs(profile)

    missing = [k for k in _REQUIRED_PREFS_KEYS if k not in prefs]
    assert not missing, f"prefs dict missing required keys: {missing}"


# ──────────────────────────────────────────────────────────────────────
#  IT2: SOCKS proxy + prefs — mutates prefs in place, returns None
# ──────────────────────────────────────────────────────────────────────


@pytest.mark.integration
def test_socks5_proxy_mutates_prefs_then_pipeline_still_valid():
    """IT2 — configure_proxy writes SOCKS auth keys to the profile-derived
    prefs dict; the result is still a valid prefs dict (all required keys
    intact) and the proxy return is ``None`` so Playwright sees no proxy."""
    profile = generate_profile(seed=42)
    prefs = translate_profile_to_prefs(profile)

    pw_proxy = configure_proxy(
        {
            "server": "socks5://proxy.example.com:1080",
            "username": "alice",
            "password": "s3cret",
        },
        prefs,
    )

    assert pw_proxy is None  # Firefox handles SOCKS internally.
    assert prefs["network.proxy.type"] == 1
    assert prefs["network.proxy.socks"] == "proxy.example.com"
    assert prefs["network.proxy.socks_port"] == 1080
    assert prefs["network.proxy.socks_version"] == 5
    assert prefs["network.proxy.socks_username"] == "alice"
    assert prefs["network.proxy.socks_password"] == "s3cret"
    assert prefs["network.proxy.socks_remote_dns"] is True

    # Profile-derived keys must still be present after proxy mutation.
    for k in _REQUIRED_PREFS_KEYS:
        assert k in prefs, f"proxy mutation dropped required key {k!r}"


# ──────────────────────────────────────────────────────────────────────
#  IT3: pin overrides propagate end-to-end into the prefs dict
# ──────────────────────────────────────────────────────────────────────


@pytest.mark.integration
def test_pin_screen_width_propagates_through_pipeline():
    """IT3 — a pinned ``screen.width`` shows up in the final prefs dict
    under ``zoom.stealth.screen.width``."""
    profile = generate_profile(seed=42, pin={"screen.width": 2560})
    prefs = translate_profile_to_prefs(profile)

    assert profile.screen.width == 2560
    assert prefs["zoom.stealth.screen.width"] == 2560


@pytest.mark.integration
def test_multiple_pins_all_visible_in_prefs():
    """IT3.b — pinning several unrelated fields at once still routes every
    one through to the prefs dict."""
    pin = {
        "screen.width": 3840,
        "screen.height": 2160,
        "hardware.concurrency": 16,
        "audio.sample_rate": 48000,
    }
    profile = generate_profile(seed=42, pin=pin)
    prefs = translate_profile_to_prefs(profile)

    assert prefs["zoom.stealth.screen.width"] == 3840
    assert prefs["zoom.stealth.screen.height"] == 2160
    assert prefs["zoom.stealth.hw_concurrency"] == 16
    assert prefs["zoom.stealth.audio.sample_rate"] == 48000


# ──────────────────────────────────────────────────────────────────────
#  IT4 / IT5: end-to-end determinism + variation
# ──────────────────────────────────────────────────────────────────────


@pytest.mark.integration
def test_pipeline_deterministic_for_same_seed():
    """IT4 — running the full pipeline twice with the same seed produces
    identical prefs dicts."""
    a = translate_profile_to_prefs(generate_profile(seed=1234))
    b = translate_profile_to_prefs(generate_profile(seed=1234))
    assert a == b


@pytest.mark.integration
def test_pipeline_varies_across_seeds():
    """IT5 — different seeds produce different prefs dicts. Compare the
    full dict, not just a sampled field, to catch regressions where a
    single hot field accidentally becomes seed-invariant."""
    a = translate_profile_to_prefs(generate_profile(seed=1))
    b = translate_profile_to_prefs(generate_profile(seed=2))
    assert a != b


# ──────────────────────────────────────────────────────────────────────
#  IT6: HTTP proxy passthrough does NOT mutate SOCKS prefs
# ──────────────────────────────────────────────────────────────────────


@pytest.mark.integration
def test_http_proxy_returned_unchanged_no_socks_mutations():
    """IT6 — an HTTP proxy is returned to Playwright unchanged and the
    SOCKS prefs are never written. Verifies the two proxy paths don't
    cross-pollute the prefs dict."""
    profile = generate_profile(seed=42)
    prefs = translate_profile_to_prefs(profile)
    proxy_in = {"server": "http://proxy.example.com:8080", "username": "bob"}

    pw_proxy = configure_proxy(proxy_in, prefs)

    assert pw_proxy is proxy_in  # returned unchanged (same object)
    # No SOCKS prefs should have been written.
    assert "network.proxy.type" not in prefs
    assert "network.proxy.socks" not in prefs
    assert "network.proxy.socks_port" not in prefs


# ──────────────────────────────────────────────────────────────────────
#  IT7: profile.fonts reaches prefs as a comma-joined whitelist
# ──────────────────────────────────────────────────────────────────────


@pytest.mark.integration
def test_profile_fonts_propagate_to_prefs_whitelist():
    """IT7 — every font in ``profile.fonts`` appears in the comma-joined
    ``zoom.stealth.font.whitelist`` pref, in order."""
    profile = generate_profile(seed=42)
    prefs = translate_profile_to_prefs(profile)

    assert profile.fonts, "fixture seed=42 produced empty fonts list"
    whitelist = prefs["zoom.stealth.font.whitelist"]
    assert isinstance(whitelist, str)
    assert whitelist == ",".join(profile.fonts)
    for font in profile.fonts:
        assert font in whitelist


# ──────────────────────────────────────────────────────────────────────
#  IT8: dark_theme controls the Win10 light-palette overlay
# ──────────────────────────────────────────────────────────────────────


@pytest.mark.integration
def test_dark_theme_pipeline_omits_light_palette():
    """IT8.a — dark_theme=True profile → no light-palette colors in prefs."""
    profile = generate_profile(seed=42, pin={"dark_theme": True})
    prefs = translate_profile_to_prefs(profile)

    assert prefs["ui.systemUsesDarkTheme"] == 1
    for key in _WIN_LIGHT_COLORS:
        assert key not in prefs, f"dark theme leaked light color: {key}"


@pytest.mark.integration
def test_light_theme_pipeline_includes_light_palette():
    """IT8.b — dark_theme=False profile → full Win10 light palette is
    overlaid onto the prefs dict."""
    profile = generate_profile(seed=42, pin={"dark_theme": False})
    prefs = translate_profile_to_prefs(profile)

    assert prefs["ui.systemUsesDarkTheme"] == 0
    for key, value in _WIN_LIGHT_COLORS.items():
        assert prefs[key] == value


# ──────────────────────────────────────────────────────────────────────
#  IT9: many seeds all produce valid prefs dicts
# ──────────────────────────────────────────────────────────────────────


@pytest.mark.integration
def test_many_seeds_all_produce_valid_prefs():
    """IT9 — sweep 10 distinct seeds through the full pipeline. Every run
    must succeed and yield a prefs dict containing every required key.
    Catches regressions where a rare CPT branch produces a prefs key
    missing/wrong-typed."""
    rng = random.Random(2026)
    seeds = [rng.randint(1, 2**31 - 1) for _ in range(10)]

    for seed in seeds:
        profile = generate_profile(seed=seed)
        prefs = translate_profile_to_prefs(profile)
        missing = [k for k in _REQUIRED_PREFS_KEYS if k not in prefs]
        assert not missing, f"seed={seed} missing keys: {missing}"


# ──────────────────────────────────────────────────────────────────────
#  IT10 (extra): Windows-specific pipeline — virtual display + SOCKS
#
#  Combines two Windows-specific branches that real callers stack:
#  headless mode (virtual_display=True) and a SOCKS5 proxy. Catches
#  ordering bugs where one branch silently overwrites the other.
# ──────────────────────────────────────────────────────────────────────


@pytest.mark.integration
def test_windows_virtual_display_with_socks_proxy(monkeypatch):
    """IT10 — Windows + virtual_display=True + SOCKS5 proxy: both branches
    land their keys in the prefs dict and don't clobber each other."""
    monkeypatch.setattr(sys, "platform", "win32")
    profile = generate_profile(seed=42)
    prefs = translate_profile_to_prefs(profile, virtual_display=True)
    pw_proxy = configure_proxy(
        {"server": "socks5://127.0.0.1:1080"}, prefs
    )

    assert pw_proxy is None
    assert prefs["security.sandbox.gpu.level"] == 0  # virtual_display branch
    assert prefs["network.proxy.type"] == 1          # SOCKS branch
    assert prefs["network.proxy.socks"] == "127.0.0.1"
    # Windows still has the renderer cleared.
    assert prefs["zoom.stealth.webgl.renderer"] == ""


# ──────────────────────────────────────────────────────────────────────
#  IT11 (extra): Linux-specific pipeline — Xvfb workarounds + GPU spoof
#  + SOCKS5 proxy. The Linux equivalent of IT10. Verifies that the three
#  Linux-only branches (renderer spoof, Xvfb webrender disable, MSAA
#  from profile) coexist with proxy mutation in the same prefs dict.
# ──────────────────────────────────────────────────────────────────────


@pytest.mark.integration
def test_linux_xvfb_workarounds_with_socks_proxy(monkeypatch):
    """IT11 — Linux + SOCKS5 proxy: Xvfb workarounds applied, GPU renderer
    spoofed from profile, SOCKS keys written. virtual_display is a Windows-
    only concept so we omit it here; passing ``virtual_display=True`` on
    Linux must NOT set ``security.sandbox.gpu.level`` (covered by VD3)."""
    monkeypatch.setattr(sys, "platform", "linux")
    profile = generate_profile(seed=42)
    prefs = translate_profile_to_prefs(profile, virtual_display=True)
    pw_proxy = configure_proxy(
        {"server": "socks5://127.0.0.1:1080"}, prefs
    )

    assert pw_proxy is None
    # Xvfb workarounds present.
    assert prefs["gfx.webrender.all"] is False
    assert prefs["gfx.webrender.force-disabled"] is True
    assert prefs["webgl.force-enabled"] is True
    # Windows-only sandbox key absent on Linux even with virtual_display=True.
    assert "security.sandbox.gpu.level" not in prefs
    # GPU renderer is spoofed from the profile (not cleared like on Windows).
    assert prefs["zoom.stealth.webgl.renderer"] == profile.gpu.renderer
    assert prefs["zoom.stealth.webgl.renderer"]  # non-empty
    # SOCKS branch wrote its keys without clobbering the Linux prefs above.
    assert prefs["network.proxy.type"] == 1
    assert prefs["network.proxy.socks"] == "127.0.0.1"


# ──────────────────────────────────────────────────────────────────────
#  IT12 (extra): Linux pipeline carries profile MSAA end-to-end. Windows
#  pins MSAA to 4 regardless of the profile; Linux must let the sampled
#  value through. Guards the platform branch in ``translate_profile_to_prefs``.
# ──────────────────────────────────────────────────────────────────────


@pytest.mark.integration
def test_linux_msaa_pin_propagates_through_pipeline(monkeypatch):
    """IT12 — pinning MSAA on Linux survives the prefs translation; on
    Windows the same pin is overwritten to 4 (covered by the unit tests)."""
    monkeypatch.setattr(sys, "platform", "linux")
    profile = generate_profile(seed=42, pin={"webgl.msaa_samples": 8})
    prefs = translate_profile_to_prefs(profile)

    assert prefs["zoom.stealth.webgl.msaa"] == 8
    assert prefs["webgl.msaa-samples"] == 8
    assert prefs["webgl.msaa-force"] is True


# ──────────────────────────────────────────────────────────────────────
#  IT13 (extra): Linux font metrics receive the GTK/DejaVu compensation
#  block. End-to-end check that ``_LINUX_GENERIC_FONT_FACTORS`` is
#  prepended to the per-font metrics string sampled from the profile.
# ──────────────────────────────────────────────────────────────────────


@pytest.mark.integration
def test_linux_font_metrics_include_generic_factors(monkeypatch):
    """IT13 — on Linux the font metrics pref starts with the generic
    width-scale factors (GTK/DejaVu compensation) so glyph widths match
    Windows. Without this, Linux sessions leak via metric drift."""
    from invisible_playwright.prefs import _LINUX_GENERIC_FONT_FACTORS

    monkeypatch.setattr(sys, "platform", "linux")
    profile = generate_profile(seed=42)
    prefs = translate_profile_to_prefs(profile)

    metrics = prefs["zoom.stealth.font.metrics"]
    assert metrics.startswith(_LINUX_GENERIC_FONT_FACTORS)
