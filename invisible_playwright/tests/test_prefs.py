import re
import sys

import pytest

from invisible_playwright._fpforge import generate_profile
from invisible_playwright.prefs import (
    _LINUX_GENERIC_FONT_FACTORS,
    _accept_language,
    _font_metrics_for_platform,
    _WIN_LIGHT_COLORS,
    translate_profile_to_prefs,
)


@pytest.mark.unit
def test_translate_includes_gpu_renderer_windows(monkeypatch):
    """On Windows, renderer/vendor are cleared so ANGLE reports native hardware."""
    monkeypatch.setattr(sys, "platform", "win32")
    p = generate_profile(seed=42)
    prefs = translate_profile_to_prefs(p)
    assert prefs["zoom.stealth.webgl.renderer"] == ""
    assert prefs["zoom.stealth.webgl.vendor"] == ""


@pytest.mark.unit
def test_translate_includes_screen():
    p = generate_profile(seed=42)
    prefs = translate_profile_to_prefs(p)
    assert prefs["zoom.stealth.screen.width"] == p.screen.width
    assert prefs["zoom.stealth.screen.height"] == p.screen.height


@pytest.mark.unit
def test_translate_is_deterministic_per_seed():
    a = translate_profile_to_prefs(generate_profile(seed=42))
    b = translate_profile_to_prefs(generate_profile(seed=42))
    assert a == b


@pytest.mark.unit
def test_translate_varies_across_seeds():
    a = translate_profile_to_prefs(generate_profile(seed=1))
    b = translate_profile_to_prefs(generate_profile(seed=2))
    assert a != b


@pytest.mark.unit
def test_translate_has_stealth_baseline_constants():
    p = generate_profile(seed=42)
    prefs = translate_profile_to_prefs(p)
    assert prefs.get("privacy.resistFingerprinting") is False
    assert "media.peerconnection.enabled" in prefs


# ──────────────────────────────────────────────────────────────────────
#  _accept_language (platform-agnostic)
# ──────────────────────────────────────────────────────────────────────


@pytest.mark.unit
def test_accept_language_with_region():
    # AL1
    assert _accept_language("en-US") == "en-US, en"


@pytest.mark.unit
def test_accept_language_no_region():
    # AL2
    assert _accept_language("fr") == "fr"


@pytest.mark.unit
def test_accept_language_underscore_normalized():
    # AL3
    assert _accept_language("pt_BR") == "pt-BR, pt"


# ──────────────────────────────────────────────────────────────────────
#  _font_metrics_for_platform
# ──────────────────────────────────────────────────────────────────────


@pytest.mark.unit
def test_font_metrics_windows_returns_empty(monkeypatch):
    # FM2: Windows never applies width-scale factors.
    monkeypatch.setattr(sys, "platform", "win32")
    assert _font_metrics_for_platform("Arial|1.0,Verdana|0.9,") == ""


@pytest.mark.unit
def test_font_metrics_empty_input_returns_empty():
    # FM3: Empty input always returns "" regardless of platform.
    assert _font_metrics_for_platform("") == ""


# ──────────────────────────────────────────────────────────────────────
#  Platform-specific GPU / MSAA (Windows)
# ──────────────────────────────────────────────────────────────────────


@pytest.mark.unit
def test_gpu_renderer_empty_on_windows(monkeypatch):
    # PG2
    monkeypatch.setattr(sys, "platform", "win32")
    p = generate_profile(seed=42)
    prefs = translate_profile_to_prefs(p)
    assert prefs["zoom.stealth.webgl.renderer"] == ""
    assert prefs["zoom.stealth.webgl.vendor"] == ""


@pytest.mark.unit
def test_msaa_pinned_to_4_on_windows(monkeypatch):
    # PG4: even when profile.webgl.msaa_samples differs, Windows pins to 4.
    monkeypatch.setattr(sys, "platform", "win32")
    p = generate_profile(seed=42, pin={"webgl.msaa_samples": 8})
    prefs = translate_profile_to_prefs(p)
    assert prefs["zoom.stealth.webgl.msaa"] == 4
    assert prefs["webgl.msaa-samples"] == 4
    assert prefs["webgl.msaa-force"] is True


# ──────────────────────────────────────────────────────────────────────
#  Canvas noise skip mask (Windows always uses intel path)
# ──────────────────────────────────────────────────────────────────────


@pytest.mark.unit
def test_canvas_noise_mask_windows_uses_intel_path(monkeypatch):
    # CN3: on Windows _renderer_lo is hardcoded to "intel" → mask=15.
    monkeypatch.setattr(sys, "platform", "win32")
    p = generate_profile(
        seed=42,
        pin={"gpu.renderer": "ANGLE (NVIDIA, NVIDIA GeForce RTX 4090 Direct3D11)"},
    )
    prefs = translate_profile_to_prefs(p)
    assert prefs["zoom.stealth.canvas.noise_skip_mask"] == 15


# ──────────────────────────────────────────────────────────────────────
#  WebGL extensions (Windows clears them)
# ──────────────────────────────────────────────────────────────────────


@pytest.mark.unit
def test_webgl_extensions_cleared_on_windows(monkeypatch):
    # WE2
    monkeypatch.setattr(sys, "platform", "win32")
    p = generate_profile(seed=42)
    prefs = translate_profile_to_prefs(p)
    assert prefs["zoom.stealth.webgl.extensions"] == ""
    assert prefs["zoom.stealth.webgl2.extensions"] == ""


# ──────────────────────────────────────────────────────────────────────
#  Timezone (platform-agnostic)
# ──────────────────────────────────────────────────────────────────────


@pytest.mark.unit
def test_timezone_set_propagates_to_both_keys():
    # TZ1
    p = generate_profile(seed=42)
    prefs = translate_profile_to_prefs(p, timezone="America/New_York")
    assert prefs["zoom.stealth.timezone"] == "America/New_York"
    assert prefs["juggler.timezone.override"] == "America/New_York"


@pytest.mark.unit
def test_timezone_empty_omits_both_keys():
    # TZ2
    p = generate_profile(seed=42)
    prefs = translate_profile_to_prefs(p, timezone="")
    assert "zoom.stealth.timezone" not in prefs
    assert "juggler.timezone.override" not in prefs


# ──────────────────────────────────────────────────────────────────────
#  extra_prefs overlay (platform-agnostic)
# ──────────────────────────────────────────────────────────────────────


@pytest.mark.unit
def test_extra_prefs_adds_custom_key():
    # EP1
    p = generate_profile(seed=42)
    prefs = translate_profile_to_prefs(p, extra_prefs={"custom.pref": 42})
    assert prefs["custom.pref"] == 42


@pytest.mark.unit
def test_extra_prefs_none_value_deletes_key():
    # EP2
    p = generate_profile(seed=42)
    prefs = translate_profile_to_prefs(
        p, extra_prefs={"privacy.resistFingerprinting": None}
    )
    assert "privacy.resistFingerprinting" not in prefs


@pytest.mark.unit
def test_extra_prefs_overrides_existing_key():
    # EP3
    p = generate_profile(seed=42)
    prefs = translate_profile_to_prefs(p, extra_prefs={"zoom.stealth.seed": 999})
    assert prefs["zoom.stealth.seed"] == 999


@pytest.mark.unit
def test_extra_prefs_none_is_no_op():
    # EP4
    p = generate_profile(seed=42)
    base = translate_profile_to_prefs(p)
    with_none = translate_profile_to_prefs(p, extra_prefs=None)
    assert base == with_none


@pytest.mark.unit
def test_extra_prefs_empty_dict_is_no_op():
    # EP5
    p = generate_profile(seed=42)
    base = translate_profile_to_prefs(p)
    with_empty = translate_profile_to_prefs(p, extra_prefs={})
    assert base == with_empty


# ──────────────────────────────────────────────────────────────────────
#  System colors / dark theme (platform-agnostic — palette is Win10)
# ──────────────────────────────────────────────────────────────────────


@pytest.mark.unit
def test_system_colors_present_when_light_theme():
    # SC1
    p = generate_profile(seed=42, pin={"dark_theme": False})
    prefs = translate_profile_to_prefs(p)
    assert prefs["ui.systemUsesDarkTheme"] == 0
    # Spot-check a few keys from the Win10 light palette.
    for key in _WIN_LIGHT_COLORS:
        assert key in prefs
        assert prefs[key] == _WIN_LIGHT_COLORS[key]


@pytest.mark.unit
def test_system_colors_absent_when_dark_theme():
    # SC2
    p = generate_profile(seed=42, pin={"dark_theme": True})
    prefs = translate_profile_to_prefs(p)
    assert prefs["ui.systemUsesDarkTheme"] == 1
    for key in _WIN_LIGHT_COLORS:
        assert key not in prefs


# ──────────────────────────────────────────────────────────────────────
#  Locale prefs (platform-agnostic)
# ──────────────────────────────────────────────────────────────────────


@pytest.mark.unit
def test_locale_en_us_accept_languages():
    # LC1
    p = generate_profile(seed=42)
    prefs = translate_profile_to_prefs(p, locale="en-US")
    assert prefs["intl.accept_languages"] == "en-US, en"


@pytest.mark.unit
def test_locale_underscore_form_normalized():
    # LC2
    p = generate_profile(seed=42)
    prefs = translate_profile_to_prefs(p, locale="de_DE")
    assert prefs["intl.accept_languages"] == "de-DE, de"
    assert prefs["general.useragent.locale"] == "de-DE"
    assert prefs["intl.locale.requested"] == "de-DE"


@pytest.mark.unit
def test_locale_empty_falls_back_to_en_us():
    # LC3
    p = generate_profile(seed=42)
    prefs = translate_profile_to_prefs(p, locale="")
    assert prefs["intl.accept_languages"] == "en-US, en"


# ──────────────────────────────────────────────────────────────────────
#  Xvfb workarounds (Windows must NOT set Linux-only keys)
# ──────────────────────────────────────────────────────────────────────


@pytest.mark.unit
def test_xvfb_workarounds_absent_on_windows(monkeypatch):
    # XW2
    monkeypatch.setattr(sys, "platform", "win32")
    p = generate_profile(seed=42)
    prefs = translate_profile_to_prefs(p)
    assert "gfx.webrender.all" not in prefs
    assert "gfx.webrender.force-disabled" not in prefs
    assert "webgl.force-enabled" not in prefs


# ──────────────────────────────────────────────────────────────────────
#  Windows virtual-desktop workarounds
# ──────────────────────────────────────────────────────────────────────


@pytest.mark.unit
def test_virtual_display_workaround_applied_on_windows(monkeypatch):
    # VD1
    monkeypatch.setattr(sys, "platform", "win32")
    p = generate_profile(seed=42)
    prefs = translate_profile_to_prefs(p, virtual_display=True)
    assert prefs["security.sandbox.gpu.level"] == 0


@pytest.mark.unit
def test_virtual_display_workaround_absent_when_disabled(monkeypatch):
    # VD2
    monkeypatch.setattr(sys, "platform", "win32")
    p = generate_profile(seed=42)
    prefs = translate_profile_to_prefs(p, virtual_display=False)
    assert "security.sandbox.gpu.level" not in prefs


# ──────────────────────────────────────────────────────────────────────
#  Seed-derived LAN IP (platform-agnostic)
# ──────────────────────────────────────────────────────────────────────


@pytest.mark.unit
def test_lan_ip_matches_192_168_pattern():
    # LI1
    p = generate_profile(seed=42)
    prefs = translate_profile_to_prefs(p)
    ip = prefs["zoom.stealth.webrtc.host_ip"]
    m = re.match(r"^192\.168\.(\d+)\.(\d+)$", ip)
    assert m, f"unexpected LAN IP format: {ip!r}"
    o3, o4 = int(m.group(1)), int(m.group(2))
    assert 1 <= o3 <= 254
    assert 1 <= o4 <= 254


@pytest.mark.unit
def test_lan_ip_deterministic_per_seed():
    # LI2
    a = translate_profile_to_prefs(generate_profile(seed=42))["zoom.stealth.webrtc.host_ip"]
    b = translate_profile_to_prefs(generate_profile(seed=42))["zoom.stealth.webrtc.host_ip"]
    assert a == b


@pytest.mark.unit
def test_lan_ip_seed_zero_has_no_zero_octets():
    # LI3: code adds +1 so neither dynamic octet should ever be 0.
    p = generate_profile(seed=0)
    prefs = translate_profile_to_prefs(p)
    ip = prefs["zoom.stealth.webrtc.host_ip"]
    octets = ip.split(".")
    assert octets[0] == "192"
    assert octets[1] == "168"
    assert int(octets[2]) >= 1
    assert int(octets[3]) >= 1


# ──────────────────────────────────────────────────────────────────────
#  Linux-specific tests — exercise the branches that only fire when
#  ``sys.platform.startswith("linux")``. Patched via ``monkeypatch`` so
#  these run on any host CI environment.
# ──────────────────────────────────────────────────────────────────────


@pytest.mark.unit
def test_font_metrics_linux_prepends_generic_factors(monkeypatch):
    # FM1: Linux prepends the GTK/DejaVu compensation block to the
    # per-font metrics string sampled from the profile.
    monkeypatch.setattr(sys, "platform", "linux")
    out = _font_metrics_for_platform("Arial|1.0,Verdana|0.9,")
    assert out.startswith(_LINUX_GENERIC_FONT_FACTORS)
    assert out.endswith("Arial|1.0,Verdana|0.9,")


@pytest.mark.unit
def test_font_metrics_linux_empty_input_returns_empty(monkeypatch):
    # FM1b: even on Linux, empty profile metrics short-circuits before
    # the prepend so we never emit a metrics pref containing only the
    # generic block (which would surface as a tampering signal).
    monkeypatch.setattr(sys, "platform", "linux")
    assert _font_metrics_for_platform("") == ""


@pytest.mark.unit
def test_font_metrics_linux2_variant_uses_linux_branch(monkeypatch):
    # FM1c: ``sys.platform`` can be ``linux2`` on older Pythons / odd
    # WSL builds. ``startswith("linux")`` accepts both.
    monkeypatch.setattr(sys, "platform", "linux2")
    out = _font_metrics_for_platform("Verdana|0.9,")
    assert out.startswith(_LINUX_GENERIC_FONT_FACTORS)


@pytest.mark.unit
def test_gpu_renderer_set_from_profile_on_linux(monkeypatch):
    # PG1: on Linux we spoof to the profile's Windows-ANGLE renderer
    # string so cross-platform sessions present a consistent Windows GPU.
    monkeypatch.setattr(sys, "platform", "linux")
    p = generate_profile(seed=42)
    prefs = translate_profile_to_prefs(p)
    assert prefs["zoom.stealth.webgl.renderer"] == p.gpu.renderer
    assert prefs["zoom.stealth.webgl.vendor"] == p.gpu.vendor
    assert prefs["zoom.stealth.webgl.renderer"]  # non-empty


@pytest.mark.unit
def test_msaa_from_profile_on_linux(monkeypatch):
    # PG3: on Linux, MSAA comes from the profile's sampled value rather
    # than being pinned to 4 (which is the Windows ANGLE default).
    monkeypatch.setattr(sys, "platform", "linux")
    p = generate_profile(seed=42, pin={"webgl.msaa_samples": 8})
    prefs = translate_profile_to_prefs(p)
    assert prefs["zoom.stealth.webgl.msaa"] == 8
    assert prefs["webgl.msaa-samples"] == 8
    assert prefs["webgl.msaa-force"] is True


@pytest.mark.unit
def test_msaa_zero_disables_force_on_linux(monkeypatch):
    # PG3b: MSAA=0 means "no MSAA" so ``webgl.msaa-force`` must be False.
    # Verifies the ``> 0`` guard on the force flag.
    monkeypatch.setattr(sys, "platform", "linux")
    p = generate_profile(seed=42, pin={"webgl.msaa_samples": 0})
    prefs = translate_profile_to_prefs(p)
    assert prefs["zoom.stealth.webgl.msaa"] == 0
    assert prefs["webgl.msaa-force"] is False


@pytest.mark.unit
def test_canvas_noise_mask_intel_on_linux(monkeypatch):
    # CN1: Intel renderer → 1/16 noise (mask=15). Pinning the renderer
    # exercises the live ``_renderer_lo`` branch on Linux (where the
    # value is read from the profile rather than hardcoded as on Windows).
    monkeypatch.setattr(sys, "platform", "linux")
    p = generate_profile(
        seed=42,
        pin={
            "gpu.renderer": "ANGLE (Intel, Intel(R) UHD Graphics 630 Direct3D11 vs_5_0 ps_5_0, D3D11)",
            "gpu.vendor": "Google Inc. (Intel)",
        },
    )
    prefs = translate_profile_to_prefs(p)
    assert prefs["zoom.stealth.canvas.noise_skip_mask"] == 15


@pytest.mark.unit
def test_canvas_noise_mask_nvidia_on_linux(monkeypatch):
    # CN2: NVIDIA/AMD renderer → 1/8 noise (mask=7). The "intel" substring
    # check must NOT match here.
    monkeypatch.setattr(sys, "platform", "linux")
    p = generate_profile(
        seed=42,
        pin={
            "gpu.renderer": "ANGLE (NVIDIA, NVIDIA GeForce RTX 4090 Direct3D11 vs_5_0 ps_5_0, D3D11)",
            "gpu.vendor": "Google Inc. (NVIDIA)",
        },
    )
    prefs = translate_profile_to_prefs(p)
    assert prefs["zoom.stealth.canvas.noise_skip_mask"] == 7


@pytest.mark.unit
def test_webgl_extensions_preserved_on_linux(monkeypatch):
    # WE1: on Linux the curated WebGL1/2 extension lists from _BASELINE
    # remain in the prefs dict so the patched binary publishes them
    # instead of native Mesa's set.
    monkeypatch.setattr(sys, "platform", "linux")
    p = generate_profile(seed=42)
    prefs = translate_profile_to_prefs(p)
    assert prefs["zoom.stealth.webgl.extensions"]
    assert prefs["zoom.stealth.webgl2.extensions"]
    # Spot-check a canonical Windows ANGLE extension is in the list.
    assert "ANGLE_instanced_arrays" in prefs["zoom.stealth.webgl.extensions"]
    assert "OVR_multiview2" in prefs["zoom.stealth.webgl2.extensions"]


@pytest.mark.unit
def test_xvfb_workarounds_applied_on_linux(monkeypatch):
    # XW1: Linux Firefox under Xvfb can't run WebRender, so we force the
    # software path. These are added via ``setdefault`` so callers can
    # still override them via ``extra_prefs``.
    monkeypatch.setattr(sys, "platform", "linux")
    p = generate_profile(seed=42)
    prefs = translate_profile_to_prefs(p)
    assert prefs["gfx.webrender.all"] is False
    assert prefs["gfx.webrender.force-disabled"] is True
    assert prefs["webgl.force-enabled"] is True


@pytest.mark.unit
def test_xvfb_workarounds_caller_can_override(monkeypatch):
    # XW1b: the workarounds are added with ``setdefault``, so a user-
    # supplied ``extra_prefs`` value wins. Verifies the override path
    # doesn't get clobbered by the platform branch.
    monkeypatch.setattr(sys, "platform", "linux")
    p = generate_profile(seed=42)
    prefs = translate_profile_to_prefs(
        p, extra_prefs={"webgl.force-enabled": False}
    )
    assert prefs["webgl.force-enabled"] is False


@pytest.mark.unit
def test_virtual_display_no_op_on_linux(monkeypatch):
    # VD3: ``virtual_display`` is a Windows-only concept (CreateDesktop
    # alt-desktop GPU sandbox workaround). Even when True, Linux must
    # not pick up ``security.sandbox.gpu.level``.
    monkeypatch.setattr(sys, "platform", "linux")
    p = generate_profile(seed=42)
    prefs = translate_profile_to_prefs(p, virtual_display=True)
    assert "security.sandbox.gpu.level" not in prefs
