"""Unit tests for invisible_playwright._fpforge._sampler.

Covers classify_gpu (decision-table over GPU strings), _screen_tier,
derive_font_prefs / derive_font_whitelist, and the public Forge / sample
entry points.
"""
import random

import pytest

from invisible_playwright._fpforge import _sampler
from invisible_playwright._fpforge._sampler import (
    Forge,
    _LOCKED,
    _screen_tier,
    classify_gpu,
    derive_font_prefs,
    derive_font_whitelist,
    sample,
)


# ── classify_gpu ────────────────────────────────────────────────────────
#
# Decision-table tests against every branch of the classifier. Inputs use
# the ANGLE renderer string format that Firefox actually exposes.

def _gpu(renderer):
    return {"renderer": renderer, "vendor": "Google Inc."}


@pytest.mark.unit
@pytest.mark.parametrize("renderer", [
    "ANGLE (Intel, Intel(R) HD Graphics 3000 Direct3D11 vs_5_0 ps_5_0)",
    "ANGLE (Intel, Intel(R) HD Graphics 4000 Direct3D11 vs_5_0 ps_5_0)",
    "ANGLE (Intel, Intel(R) HD Graphics 2500 Direct3D11 vs_5_0 ps_5_0)",
])
def test_classify_gpu_intel_hd_old_buckets(renderer):
    """CG1-CG3 [DT]: HD 2500/3000/4000 → integrated_old."""
    assert classify_gpu(_gpu(renderer)) == "integrated_old"


@pytest.mark.unit
@pytest.mark.parametrize("renderer", [
    "ANGLE (Intel, Intel(R) HD Graphics 530 Direct3D11)",
    "ANGLE (Intel, Intel(R) UHD Graphics 630 Direct3D11)",
    "ANGLE (Intel, Intel(R) Iris Xe Graphics Direct3D11)",
    "ANGLE (Intel, Intel(R) Arc A750 Direct3D11)",
])
def test_classify_gpu_intel_modern(renderer):
    """CG4-CG7 [DT]: modern Intel HD/UHD/Iris/Arc → integrated_modern."""
    assert classify_gpu(_gpu(renderer)) == "integrated_modern"


@pytest.mark.unit
@pytest.mark.parametrize("renderer", [
    "ANGLE (AMD, AMD Radeon Graphics Direct3D11)",
    "ANGLE (AMD, AMD Radeon Vega 8 Direct3D11)",
])
def test_classify_gpu_amd_integrated(renderer):
    """CG8-CG9 [DT]: AMD APU graphics → integrated_modern."""
    assert classify_gpu(_gpu(renderer)) == "integrated_modern"


@pytest.mark.unit
@pytest.mark.parametrize("renderer", [
    "ANGLE (NVIDIA, NVIDIA GeForce 8800 GTX Direct3D11)",
    "ANGLE (NVIDIA, NVIDIA GeForce GTX 480 Direct3D11)",
    "ANGLE (NVIDIA, NVIDIA GeForce GTX 980 Direct3D11)",
    "ANGLE (NVIDIA, NVIDIA GeForce GT 1030 Direct3D11)",
])
def test_classify_gpu_nvidia_vintage_buckets(renderer):
    """CG10-CG13 [DT]: vintage GeForce buckets → low_end."""
    assert classify_gpu(_gpu(renderer)) == "low_end"


@pytest.mark.unit
def test_classify_gpu_nvidia_modern_geforce_falls_to_low_end():
    """CG14 [DT]: GeForce GTX 1060 — sanitized vintage → low_end via fallback."""
    assert classify_gpu(_gpu(
        "ANGLE (NVIDIA, NVIDIA GeForce GTX 1060 Direct3D11)"
    )) == "low_end"


@pytest.mark.unit
def test_classify_gpu_nvidia_quadro_k_matches_vintage_pattern():
    """CG15 [DT]: Quadro K2200 → low_end (matches vintage Quadro K pattern)."""
    assert classify_gpu(_gpu(
        "ANGLE (NVIDIA, NVIDIA Quadro K2200 Direct3D11)"
    )) == "low_end"


@pytest.mark.unit
def test_classify_gpu_amd_radeon_high_end_boundary():
    """CG16 [DT]: AMD Radeon RX 5700 XT (n=5700) → high_end."""
    assert classify_gpu(_gpu(
        "ANGLE (AMD, AMD Radeon RX 5700 XT Direct3D11)"
    )) == "high_end"


@pytest.mark.unit
@pytest.mark.parametrize("renderer", [
    "ANGLE (AMD, AMD Radeon RX 5500 Direct3D11)",
    "ANGLE (AMD, AMD Radeon RX 580 Direct3D11)",
])
def test_classify_gpu_amd_radeon_mid_range(renderer):
    """CG17-CG18 [DT]: RX 5500 / RX 580 → mid_range."""
    assert classify_gpu(_gpu(renderer)) == "mid_range"


@pytest.mark.unit
def test_classify_gpu_amd_radeon_below_mid_range():
    """CG19 [DT]: RX 480 (n=480, not in mid_range buckets) → low_end."""
    assert classify_gpu(_gpu(
        "ANGLE (AMD, AMD Radeon RX 480 Direct3D11)"
    )) == "low_end"


@pytest.mark.unit
def test_classify_gpu_amd_firepro_falls_through_to_fallback():
    """CG20 [DT]: AMD FirePro W7100 — workstation regex requires
    'Radeon' prefix, FirePro alone doesn't match → falls through to
    mid_range fallback. (Plan claimed workstation; actual code path
    only routes Radeon-Pro-prefixed cards into the workstation bucket.)
    """
    assert classify_gpu(_gpu(
        "ANGLE (AMD, AMD FirePro W7100 Direct3D11)"
    )) == "mid_range"


@pytest.mark.unit
def test_classify_gpu_amd_radeon_pro_workstation():
    """CG21 [DT]: AMD Radeon Pro WX 7100 → workstation."""
    assert classify_gpu(_gpu(
        "ANGLE (AMD, AMD Radeon Pro WX 7100 Direct3D11)"
    )) == "workstation"


@pytest.mark.unit
def test_classify_gpu_unknown_renderer_falls_back_to_mid_range():
    """CG22 [DT]: completely unknown vendor/renderer → mid_range fallback."""
    assert classify_gpu(_gpu(
        "ANGLE (Unknown, Something Else Direct3D11)"
    )) == "mid_range"


@pytest.mark.unit
def test_classify_gpu_empty_renderer_falls_back_to_mid_range():
    """CG23 [BVA]: empty renderer string → mid_range fallback."""
    assert classify_gpu({"renderer": "", "vendor": ""}) == "mid_range"


@pytest.mark.unit
@pytest.mark.parametrize("renderer", [
    "ANGLE (AMD, AMD Radeon RX 5699 Direct3D11)",   # CG24: just below 5700
    "ANGLE (AMD, AMD Radeon RX 5601 Direct3D11)",   # CG25: just above 5600
    "ANGLE (AMD, AMD Radeon RX 579 Direct3D11)",    # CG26: just below 580
    "ANGLE (AMD, AMD Radeon RX 591 Direct3D11)",    # CG27: just above 590
])
def test_classify_gpu_amd_radeon_boundary_values_outside_mid_range(renderer):
    """CG24-CG27 [BVA]: AMD Radeon numbers just outside mid_range buckets → low_end."""
    assert classify_gpu(_gpu(renderer)) == "low_end"


@pytest.mark.unit
def test_classify_gpu_missing_renderer_key_uses_empty_default():
    """CG28 [ERR]: dict without 'renderer' key → mid_range fallback (r='')."""
    assert classify_gpu({"vendor": "X"}) == "mid_range"


# ── _screen_tier ────────────────────────────────────────────────────────

@pytest.mark.unit
@pytest.mark.parametrize("w,h,expected", [
    (1920, 1080, "1080p"),         # ST1 [ECP]
    (2560, 1440, "1440p"),         # ST2 [ECP]
    (3840, 2160, "2160p"),         # ST3 [ECP]
    (3440, 1440, "ultrawide"),     # ST4 [ECP] aspect 2.39 > 2.1
    (1921, 1080, "1440p"),         # ST5 [BVA] just above 1920
    (2561, 1440, "2160p"),         # ST6 [BVA] just above 2560
    (3841, 2160, "ultrawide"),     # ST7 [BVA] just above 3840
    (1280, 720,  "1080p"),         # ST8 [BVA] below 1920
])
def test_screen_tier_classification(w, h, expected):
    assert _screen_tier({"screen": {"w": w, "h": h}}) == expected


@pytest.mark.unit
def test_screen_tier_empty_context_defaults_to_1080p():
    """ST9 [ERR]: empty ctx → defaults w=1920, h=1080 → 1080p."""
    assert _screen_tier({}) == "1080p"


@pytest.mark.unit
def test_screen_tier_4200x2000_is_ultrawide_via_width_branch():
    """ST10 [BVA]: w=4200,h=2000 — ratio 2.1 is NOT >2.1 (strict), but
    w>3840 also routes to the final ultrawide branch."""
    assert _screen_tier({"screen": {"w": 4200, "h": 2000}}) == "ultrawide"


# ── derive_font_prefs / derive_font_whitelist ───────────────────────────

@pytest.mark.unit
def test_derive_font_prefs_returns_whitelist_and_metrics_keys():
    """FP1 [HAPPY]: result has the two expected string keys."""
    out = derive_font_prefs("integrated_modern", random.Random(42))
    assert set(out.keys()) == {"whitelist", "metrics"}
    assert isinstance(out["whitelist"], str)
    assert isinstance(out["metrics"], str)


@pytest.mark.unit
def test_derive_font_prefs_core_fonts_always_present():
    """FP2 [ECP]: every core font name appears in whitelist regardless of class."""
    out = derive_font_prefs("integrated_old", random.Random(0))
    names = set(out["whitelist"].split(","))
    for entry in _sampler._FONT_CORE:
        assert entry["name"] in names


@pytest.mark.unit
def test_derive_font_prefs_deterministic_per_seed():
    """FP3 [ECP]: same gpu_class + same rng seed → identical result."""
    a = derive_font_prefs("workstation", random.Random(7))
    b = derive_font_prefs("workstation", random.Random(7))
    assert a == b


@pytest.mark.unit
def test_derive_font_prefs_unknown_class_falls_back_to_integrated_modern():
    """FP4 [ECP]: gpu_class missing from CPT → uses integrated_modern row."""
    fallback = derive_font_prefs("nonexistent", random.Random(123))
    expected = derive_font_prefs("integrated_modern", random.Random(123))
    assert fallback == expected


@pytest.mark.unit
def test_derive_font_prefs_metrics_and_whitelist_are_coherent():
    """FP5 [ECP]: every name in whitelist has a metrics entry and vice versa."""
    out = derive_font_prefs("mid_range", random.Random(99))
    wl_names = out["whitelist"].split(",")
    metrics_names = [s.split("|", 1)[0] for s in out["metrics"].split(",")]
    assert wl_names == metrics_names


@pytest.mark.unit
def test_derive_font_prefs_whitelist_alphabetically_sorted():
    """FP6 [ECP]: whitelist names are sorted (ordering invariant for stable dedup)."""
    out = derive_font_prefs("high_end", random.Random(5))
    names = out["whitelist"].split(",")
    assert names == sorted(names)


@pytest.mark.unit
def test_derive_font_whitelist_legacy_shim_matches_dict_form():
    """FW1 [HAPPY]: legacy shim returns same string as dict['whitelist']."""
    rng_a = random.Random(11)
    rng_b = random.Random(11)
    assert derive_font_whitelist("low_end", rng_a) == \
        derive_font_prefs("low_end", rng_b)["whitelist"]


# ── Forge / sample ──────────────────────────────────────────────────────

# Keys the Forge.sample bundle must always contain. Builds on _LOCKED +
# every Bayesian-sampled field exposed in the return dict.
_EXPECTED_KEYS = {
    "stealth_seed",
    *_LOCKED.keys(),
    "webgl_renderer", "webgl_vendor", "gpu_class",
    "intra_tier", "screen_tier",
    "screen_w", "screen_h", "screen_avail_w", "screen_avail_h", "dpr",
    "hw_concurrency", "msaa_samples",
    "audio_sample_rate", "audio_output_latency_ms", "audio_max_channel_count",
    "av1_enabled", "webm_encoder_enabled",
    "mediasource_webm", "mediasource_mp4", "webspeech_synth",
    "storage_quota_mb", "dark_theme",
    "font_whitelist", "font_metrics",
}


@pytest.mark.unit
def test_forge_sample_returns_dict():
    """FS1 [HAPPY]: sample(42) returns a non-empty dict."""
    out = sample(42)
    assert isinstance(out, dict) and out


@pytest.mark.unit
def test_forge_sample_has_every_expected_key():
    """FS2 [ECP]: every locked + sampled key is present in the bundle."""
    out = sample(42)
    missing = _EXPECTED_KEYS - set(out.keys())
    assert not missing, f"missing keys: {missing}"


@pytest.mark.unit
def test_forge_sample_field_types():
    """FS3 [ECP]: int/float/bool fields have the right Python types."""
    out = sample(42)
    assert isinstance(out["screen_w"], int)
    assert isinstance(out["screen_h"], int)
    assert isinstance(out["dpr"], float)
    assert isinstance(out["hw_concurrency"], int)
    assert isinstance(out["webdriver"], bool)
    assert isinstance(out["av1_enabled"], bool)
    assert isinstance(out["max_touch_points"], int)


@pytest.mark.unit
def test_forge_sample_deterministic_per_seed():
    """FS4 [ECP]: same seed → identical bundle."""
    assert sample(42) == sample(42)


@pytest.mark.unit
def test_forge_sample_varies_across_seeds():
    """FS5 [ECP]: distinct seeds → at least one varying field across N seeds."""
    bundles = [sample(s) for s in range(8)]
    renderers = {b["webgl_renderer"] for b in bundles}
    assert len(renderers) > 1


@pytest.mark.unit
def test_forge_sample_locked_identity_fields_match_locked_table():
    """FS6 [ECP]: every field in _LOCKED is echoed verbatim in the bundle."""
    out = sample(42)
    for k, v in _LOCKED.items():
        assert out[k] == v


@pytest.mark.unit
def test_forge_constructor_equivalent_to_sample_helper():
    """FS7 [ECP]: Forge(seed).sample() == sample(seed)."""
    assert Forge(42).sample() == sample(42)


@pytest.mark.unit
def test_forge_sample_avail_h_defaults_to_h_minus_40_when_missing(monkeypatch):
    """FS8 [ECP]: when a screen entry has no 'ah' key, screen_avail_h
    defaults to screen_h - 40. Real CPT data always provides 'ah', so
    we monkeypatch the network to return a synthetic bundle."""
    fake_bundle = {
        "gpu": {"renderer": "ANGLE (Intel, Intel(R) UHD Graphics 630 Direct3D11)",
                "vendor": "Google Inc."},
        "gpu_class": "integrated_modern",
        "intra_tier": "standard",
        "screen": {"w": 1920, "h": 1080, "dpr": 1.0},  # no aw, no ah
        "screen_tier": "1080p",
        "hw_concurrency": 8,
        "msaa_samples": 4,
        "codec": {"av1_enabled": True, "webm_encoder_enabled": True,
                  "mediasource_webm": True, "mediasource_mp4": True,
                  "webspeech_synth": True},
        "storage_quota_mb": 256000,
        "audio": {"rate": 48000, "latency": 20, "channels": 2},
        "dark_theme": 0,
    }
    monkeypatch.setattr(_sampler._NETWORK, "sample", lambda _rng: fake_bundle)
    out = Forge(42).sample()
    assert out["screen_avail_w"] == 1920    # falls back to w
    assert out["screen_avail_h"] == 1080 - 40


@pytest.mark.unit
def test_forge_sample_includes_font_keys():
    """FS9 [ECP]: font_whitelist + font_metrics present and non-empty."""
    out = sample(42)
    assert out["font_whitelist"]
    assert out["font_metrics"]
    assert "," in out["font_whitelist"]    # at least the core fonts joined


@pytest.mark.unit
def test_forge_seed_coercion_to_int():
    """FS extra: Forge(seed) coerces seed to int (e.g. float 42.7 → 42)."""
    f = Forge(42.7)
    assert f.seed == 42
