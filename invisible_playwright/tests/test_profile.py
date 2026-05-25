"""Unit tests for `_fpforge/profile.py`.

Covers `_validate_pin_key`, `_apply_pins_to_raw`, and `generate_profile`.
Test cases derived via ECP/BVA/error guessing.
"""
from dataclasses import FrozenInstanceError

import pytest

from invisible_playwright._fpforge import generate_profile
from invisible_playwright._fpforge.profile import (
    Profile,
    _PIN_GROUPS,
    _PIN_TO_RAW,
    _apply_pins_to_raw,
    _validate_pin_key,
)


# ─────────────────────────────────────────────────────────────────────
#  _validate_pin_key
# ─────────────────────────────────────────────────────────────────────

@pytest.mark.unit
def test_validate_pin_key_top_level_fonts():
    """VK1 — `fonts` is a known top-level key."""
    _validate_pin_key("fonts")


@pytest.mark.unit
def test_validate_pin_key_top_level_dark_theme():
    """VK2 — `dark_theme` is a known top-level key."""
    _validate_pin_key("dark_theme")


@pytest.mark.unit
def test_validate_pin_key_dotted_screen_width():
    """VK3 — valid dotted path `screen.width`."""
    _validate_pin_key("screen.width")


@pytest.mark.unit
def test_validate_pin_key_dotted_gpu_renderer():
    """VK4 — valid dotted path `gpu.renderer`."""
    _validate_pin_key("gpu.renderer")


@pytest.mark.unit
def test_validate_pin_key_dotted_webgl_msaa_samples():
    """VK5 — valid dotted path `webgl.msaa_samples`."""
    _validate_pin_key("webgl.msaa_samples")


@pytest.mark.unit
def test_validate_pin_key_no_dot_not_top_level_raises():
    """VK6 — bare key not in top-level set raises with hint."""
    with pytest.raises(ValueError, match="group.field"):
        _validate_pin_key("bogus")


@pytest.mark.unit
def test_validate_pin_key_unknown_group_raises():
    """VK7 — unknown group prefix."""
    with pytest.raises(ValueError, match="unknown group"):
        _validate_pin_key("network.port")


@pytest.mark.unit
def test_validate_pin_key_unknown_field_in_valid_group_raises():
    """VK8 — known group, unknown field."""
    with pytest.raises(ValueError, match="unknown field"):
        _validate_pin_key("screen.brightness")


@pytest.mark.unit
def test_validate_pin_key_empty_string_raises():
    """VK9 — empty key fails the dotted-form check."""
    with pytest.raises(ValueError):
        _validate_pin_key("")


@pytest.mark.unit
@pytest.mark.parametrize("group,fields", sorted(_PIN_GROUPS.items()))
def test_validate_pin_key_all_groups_first_field(group, fields):
    """VK10 — every defined group accepts its sorted-first field."""
    first = sorted(fields)[0]
    _validate_pin_key(f"{group}.{first}")


# ─────────────────────────────────────────────────────────────────────
#  _apply_pins_to_raw
# ─────────────────────────────────────────────────────────────────────

def _raw_baseline():
    """A minimal raw dict for pin tests — only the keys we care about."""
    return {
        "screen_w": 1920,
        "screen_h": 1080,
        "webgl_vendor": "Google Inc. (Intel)",
        "webgl_renderer": "ANGLE (Intel)",
        "font_whitelist": "arial,calibri",
        "dark_theme": 0,
    }


@pytest.mark.unit
def test_apply_pins_to_raw_screen_width():
    """AP1 — `screen.width` rewrites `screen_w` in raw."""
    out = _apply_pins_to_raw(_raw_baseline(), {"screen.width": 2560})
    assert out["screen_w"] == 2560


@pytest.mark.unit
def test_apply_pins_to_raw_fonts_list():
    """AP2 — list pin joined into comma-separated whitelist."""
    out = _apply_pins_to_raw(_raw_baseline(), {"fonts": ["Arial", "Verdana"]})
    assert out["font_whitelist"] == "Arial,Verdana"


@pytest.mark.unit
def test_apply_pins_to_raw_fonts_tuple():
    """AP3 — tuple pin is also accepted."""
    out = _apply_pins_to_raw(_raw_baseline(), {"fonts": ("Arial",)})
    assert out["font_whitelist"] == "Arial"


@pytest.mark.unit
def test_apply_pins_to_raw_fonts_string_raises():
    """AP4 — bare string is not a list/tuple, must raise."""
    with pytest.raises(TypeError, match="list/tuple"):
        _apply_pins_to_raw(_raw_baseline(), {"fonts": "Arial"})


@pytest.mark.unit
def test_apply_pins_to_raw_fonts_int_raises():
    """AP5 — int is also rejected."""
    with pytest.raises(TypeError):
        _apply_pins_to_raw(_raw_baseline(), {"fonts": 42})


@pytest.mark.unit
def test_apply_pins_to_raw_multiple_pins():
    """AP6 — multiple pins all land in raw."""
    pin = {"gpu.vendor": "X", "gpu.renderer": "Y"}
    out = _apply_pins_to_raw(_raw_baseline(), pin)
    assert out["webgl_vendor"] == "X"
    assert out["webgl_renderer"] == "Y"


@pytest.mark.unit
def test_apply_pins_to_raw_returns_copy_not_mutation():
    """AP7 — input dict is not mutated."""
    raw = _raw_baseline()
    snapshot = dict(raw)
    _apply_pins_to_raw(raw, {"screen.width": 9999})
    assert raw == snapshot


@pytest.mark.unit
def test_apply_pins_to_raw_unknown_key_silent():
    """AP8 — key not in `_PIN_TO_RAW` (and not 'fonts') is ignored.

    Validation happens upstream in `generate_profile`; the inner helper
    guards defensively but does not raise.
    """
    raw = _raw_baseline()
    out = _apply_pins_to_raw(raw, {"some.unknown": 123})
    # No change to known fields
    assert out["screen_w"] == raw["screen_w"]
    # No new key added
    assert "some.unknown" not in out


# ─────────────────────────────────────────────────────────────────────
#  generate_profile
# ─────────────────────────────────────────────────────────────────────

@pytest.mark.unit
def test_generate_profile_happy_path():
    """GP1 — returns a fully populated Profile."""
    p = generate_profile(seed=42)
    assert isinstance(p, Profile)
    assert p.seed == 42
    assert p.gpu.vendor
    assert p.gpu.renderer
    assert p.gpu.class_tier in _PIN_GROUPS["gpu"].union({"low_end", "mid_range",
        "high_end", "integrated_old", "integrated_modern", "workstation"})
    assert p.screen.width > 0
    assert p.screen.height > 0
    assert p.hardware.concurrency > 0
    assert p.audio.sample_rate > 0


@pytest.mark.unit
def test_generate_profile_deterministic():
    """GP2 — same seed → identical Profile (equality on frozen dataclass)."""
    a = generate_profile(seed=42)
    b = generate_profile(seed=42)
    assert a == b


@pytest.mark.unit
def test_generate_profile_seed_float_coerced():
    """GP3 — float seed is coerced to int (truncated)."""
    a = generate_profile(seed=42.7)
    b = generate_profile(seed=42)
    assert a == b


@pytest.mark.unit
def test_generate_profile_seed_string_coerced():
    """GP4 — numeric string seed works via int() coercion."""
    a = generate_profile(seed="42")
    b = generate_profile(seed=42)
    assert a == b


@pytest.mark.unit
def test_generate_profile_no_pin_samples_freely():
    """GP5 — no pin: every field is sampler-derived (sanity: 2 seeds differ)."""
    a = generate_profile(seed=1)
    b = generate_profile(seed=2)
    assert a != b


@pytest.mark.unit
def test_generate_profile_pin_overrides_screen_width():
    """GP6 — pinned width visible on the Profile dataclass."""
    p = generate_profile(seed=42, pin={"screen.width": 9999})
    assert p.screen.width == 9999


@pytest.mark.unit
def test_generate_profile_pin_visible_in_prefs_dict():
    """GP7 — pinned values flow through to to_prefs_dict()."""
    p = generate_profile(seed=42, pin={"screen.width": 9999})
    assert p.to_prefs_dict()["screen_w"] == 9999


@pytest.mark.unit
def test_generate_profile_invalid_pin_raises():
    """GP8 — bad pin key surfaces ValueError from validation."""
    with pytest.raises(ValueError):
        generate_profile(seed=42, pin={"bogus": 1})


@pytest.mark.unit
def test_generate_profile_empty_pin_equals_no_pin():
    """GP9 — empty pin dict is a no-op."""
    a = generate_profile(seed=42, pin={})
    b = generate_profile(seed=42)
    assert a == b


@pytest.mark.unit
def test_generate_profile_is_frozen():
    """GP10 — Profile dataclass is immutable."""
    p = generate_profile(seed=42)
    with pytest.raises(FrozenInstanceError):
        p.seed = 99  # type: ignore[misc]


@pytest.mark.unit
def test_generate_profile_fonts_is_list_of_strings():
    """GP11 — fonts is a non-empty list of stripped strings."""
    p = generate_profile(seed=42)
    assert isinstance(p.fonts, list)
    assert len(p.fonts) > 0
    assert all(isinstance(f, str) and f.strip() == f for f in p.fonts)


@pytest.mark.unit
def test_generate_profile_to_prefs_dict_flat_and_matches_raw():
    """GP12 — to_prefs_dict() returns a flat dict containing core sampler keys."""
    p = generate_profile(seed=42)
    d = p.to_prefs_dict()
    assert isinstance(d, dict)
    for key in ("screen_w", "screen_h", "webgl_vendor", "webgl_renderer",
                "hw_concurrency", "stealth_seed"):
        assert key in d


@pytest.mark.unit
def test_generate_profile_seed_zero():
    """GP13 — seed=0 is a valid lowest-value boundary."""
    p = generate_profile(seed=0)
    assert p.seed == 0


@pytest.mark.unit
def test_generate_profile_seed_max_int31():
    """GP14 — seed at int31 upper bound works."""
    seed = (1 << 31) - 1
    p = generate_profile(seed=seed)
    assert p.seed == seed


@pytest.mark.unit
def test_generate_profile_dark_theme_is_bool():
    """GP15 — dark_theme is coerced to bool on the dataclass."""
    p = generate_profile(seed=42)
    assert isinstance(p.dark_theme, bool)


# ─────────────────────────────────────────────────────────────────────
#  Additional pin coverage (recheck pass)
# ─────────────────────────────────────────────────────────────────────

@pytest.mark.unit
def test_generate_profile_pin_dark_theme_true():
    """Pinning dark_theme=True flows through coercion to bool."""
    p = generate_profile(seed=42, pin={"dark_theme": True})
    assert p.dark_theme is True


@pytest.mark.unit
def test_generate_profile_pin_dark_theme_false():
    p = generate_profile(seed=42, pin={"dark_theme": False})
    assert p.dark_theme is False


@pytest.mark.unit
def test_generate_profile_pin_fonts_list_visible_on_profile():
    """fonts pin: list → joined raw string → split back to list on Profile."""
    p = generate_profile(seed=42, pin={"fonts": ["Arial", "Verdana"]})
    assert p.fonts == ["Arial", "Verdana"]


@pytest.mark.unit
def test_generate_profile_pin_gpu_renderer_propagates():
    p = generate_profile(seed=42, pin={"gpu.renderer": "FORCED_RENDERER"})
    assert p.gpu.renderer == "FORCED_RENDERER"
    assert p.to_prefs_dict()["webgl_renderer"] == "FORCED_RENDERER"


@pytest.mark.unit
def test_generate_profile_pin_to_raw_keymap_complete():
    """Every dotted pin key (besides 'fonts') has a `_PIN_TO_RAW` mapping.

    Guards against silently-ignored pins if someone adds a key to `_PIN_GROUPS`
    but forgets the raw-key mapping.
    """
    dotted = {f"{group}.{field}" for group, fields in _PIN_GROUPS.items()
              for field in fields}
    # 'dark_theme' is top-level and present in _PIN_TO_RAW; 'fonts' is handled
    # specially and intentionally absent.
    missing = dotted - set(_PIN_TO_RAW.keys())
    assert missing == set(), f"pin keys without raw mapping: {sorted(missing)}"
