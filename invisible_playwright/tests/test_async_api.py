"""Constructor-parity tests for the async ``InvisiblePlaywright``.

The async API mirrors the sync launcher (same prefs pipeline, same
profile generation, same proxy handling). The only async-specific
surface is ``__aenter__`` / ``__aexit__`` and an awaitable ``new_page``
patch — both require a real Firefox binary to exercise meaningfully and
are covered by the sync E2E tests via parity arguments.

What we test here without launching a browser: the constructor builds
the same eager Profile, clamps the seed identically, and surfaces pin
validation errors at construction time. These guards keep the async
class from silently drifting away from the sync class as features land.
"""
from __future__ import annotations

import pytest

from invisible_playwright.async_api import InvisiblePlaywright as AsyncIP
from invisible_playwright.launcher import InvisiblePlaywright as SyncIP


@pytest.mark.unit
def test_async_explicit_seed_is_stored():
    ip = AsyncIP(seed=42)
    assert ip.seed == 42


@pytest.mark.unit
def test_async_random_seed_is_positive_int31():
    """Same int31 contract as sync: the C++ side rejects ``seed <= 0`` and
    a 32-bit value risks the high bit looking negative."""
    ip = AsyncIP()
    assert isinstance(ip.seed, int)
    assert 0 < ip.seed < 2**31


@pytest.mark.unit
def test_async_random_seed_varies_across_instances():
    seeds = {AsyncIP().seed for _ in range(5)}
    assert len(seeds) > 1


@pytest.mark.unit
def test_async_profile_built_eagerly_in_constructor():
    """Pin validation must fire before ``__aenter__`` — otherwise a user
    only learns their pin is wrong when the browser launch starts."""
    ip = AsyncIP(seed=42)
    assert ip._profile is not None
    assert ip._profile.seed == 42


@pytest.mark.unit
def test_async_invalid_pin_raises_in_constructor():
    with pytest.raises(ValueError):
        AsyncIP(seed=42, pin={"not_a_real_field": 1})


@pytest.mark.unit
def test_async_and_sync_share_seed_for_same_input():
    """Same seed → identical Profile across the two APIs. Both lean on
    ``generate_profile(seed)``; if they diverge it means one of them
    started doing extra sampling."""
    seed = 12345
    a = AsyncIP(seed=seed)
    s = SyncIP(seed=seed)
    assert a._profile == s._profile


@pytest.mark.unit
def test_async_seed_coerced_from_float():
    """``int(seed)`` truncation — matches sync clamping behaviour."""
    ip = AsyncIP(seed=42.9)
    assert ip.seed == 42


@pytest.mark.unit
def test_async_default_context_kwargs_match_sync():
    """The two ``_default_context_kwargs`` implementations must produce
    the same dict for the same inputs. Guards against the async copy
    drifting away when sync adds new keys."""
    a = AsyncIP(seed=42, timezone="America/New_York", locale="de-DE")
    s = SyncIP(seed=42, timezone="America/New_York", locale="de-DE")
    assert a._default_context_kwargs() == s._default_context_kwargs()
