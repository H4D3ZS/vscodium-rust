"""Unit tests for the ``_headless`` virtual-display dispatcher.

The dispatcher (``make_virtual_display``) is the only piece of
``_headless`` we can exercise as a unit test on a single platform:
``_WindowsVirtualDesktop`` actually creates a Win32 desktop on
construction's later ``start()`` call, and ``_LinuxVirtualDisplay`` calls
``Xvfb`` — both belong in integration/E2E coverage. The dispatcher's
job is pure platform routing, which we patch via ``monkeypatch``.

Per scope: Windows-specific + platform-agnostic only. We still cover
the Linux dispatch branch because instantiating ``_LinuxVirtualDisplay``
does no I/O — Xvfb is only spawned in ``start()``, which we never call.
"""
from __future__ import annotations

import sys

import pytest

import invisible_playwright._headless as headless
from invisible_playwright._headless import (
    _LinuxVirtualDisplay,
    _WindowsVirtualDesktop,
    make_virtual_display,
)


@pytest.mark.unit
def test_make_virtual_display_returns_windows_desktop_on_win32(monkeypatch):
    monkeypatch.setattr(headless.sys, "platform", "win32")
    vd = make_virtual_display()
    assert isinstance(vd, _WindowsVirtualDesktop)


@pytest.mark.unit
def test_make_virtual_display_returns_linux_xvfb_on_linux(monkeypatch):
    """``__init__`` of ``_LinuxVirtualDisplay`` does no I/O — only ``start()``
    spawns Xvfb. Exercising the dispatcher here is safe on any host."""
    monkeypatch.setattr(headless.sys, "platform", "linux")
    vd = make_virtual_display()
    assert isinstance(vd, _LinuxVirtualDisplay)


@pytest.mark.unit
def test_make_virtual_display_accepts_linux_variants(monkeypatch):
    """``sys.platform`` can be ``linux2`` on older Pythons / WSL builds.
    The dispatcher uses ``startswith("linux")`` to accept all variants."""
    monkeypatch.setattr(headless.sys, "platform", "linux2")
    assert isinstance(make_virtual_display(), _LinuxVirtualDisplay)


@pytest.mark.unit
def test_make_virtual_display_raises_on_darwin(monkeypatch):
    """macOS is unsupported — the dispatcher must raise with a clear
    message rather than returning a no-op shim. ``InvisiblePlaywright``
    relies on this to bail before launching Firefox on a system where
    the patched binary doesn't exist."""
    monkeypatch.setattr(headless.sys, "platform", "darwin")
    with pytest.raises(RuntimeError, match="Windows and Linux only"):
        make_virtual_display()


@pytest.mark.unit
def test_make_virtual_display_raises_on_unsupported_platform(monkeypatch):
    monkeypatch.setattr(headless.sys, "platform", "freebsd14")
    with pytest.raises(RuntimeError, match="Windows and Linux only"):
        make_virtual_display()


@pytest.mark.unit
def test_make_virtual_display_error_mentions_offending_platform(monkeypatch):
    """Error message should include the actual ``sys.platform`` so the
    user can diagnose why their CI / weird container is being rejected."""
    monkeypatch.setattr(headless.sys, "platform", "sunos5")
    with pytest.raises(RuntimeError, match="sunos5"):
        make_virtual_display()


@pytest.mark.unit
def test_windows_desktop_initial_state_is_clean():
    """Construction must not allocate Win32 resources — only ``start()``
    does. Allows users to instantiate ``InvisiblePlaywright`` without
    pywin32 installed; the import error fires lazily when ``start()`` runs."""
    vd = _WindowsVirtualDesktop()
    assert vd._desktop is None
    assert vd._original_handle == 0


@pytest.mark.unit
@pytest.mark.skipif(sys.platform != "win32", reason="exercises Win32 ctypes")
def test_windows_desktop_stop_is_idempotent_without_start():
    """``stop()`` after never calling ``start()`` must be a no-op, so
    ``__exit__`` from a failed launch can call it unconditionally.

    Skipped off Windows because ``stop()`` unconditionally resolves
    ``ctypes.windll.user32`` at the top of the function — that symbol
    only exists on Windows.  The early-return logic is safe because
    callers only instantiate this class via ``make_virtual_display()``
    which already routes on ``sys.platform == 'win32'``.
    """
    vd = _WindowsVirtualDesktop()
    vd.stop()
    vd.stop()
    assert vd._desktop is None
    assert vd._original_handle == 0


# ──────────────────────────────────────────────────────────────────────
#  _LinuxVirtualDisplay — construction-only smoke tests. ``start()`` is
#  E2E because it spawns Xvfb; ``stop()`` is safe to call when no Xvfb
#  was ever started, so we exercise that path explicitly.
# ──────────────────────────────────────────────────────────────────────


@pytest.mark.unit
def test_linux_virtual_display_initial_state_is_clean():
    """Construction must not spawn Xvfb or mutate the environment — only
    ``start()`` does. Mirrors the Windows construction-state test."""
    vd = _LinuxVirtualDisplay()
    assert vd._proc is None
    assert vd._display is None
    assert vd._saved_env == {}


@pytest.mark.unit
def test_linux_virtual_display_geometry_default():
    """Default geometry is 1920x1080x24 — matches the profile sampler's
    default screen and avoids the Xvfb default of 1280x1024 which the
    fingerprint pipeline never produces."""
    vd = _LinuxVirtualDisplay()
    assert vd._geometry == "1920x1080x24"


@pytest.mark.unit
def test_linux_virtual_display_custom_geometry():
    """Caller-supplied width/height feed straight into the Xvfb geometry
    spec; the depth is always 24 (Firefox/ANGLE assume true-color)."""
    vd = _LinuxVirtualDisplay(width=2560, height=1440)
    assert vd._geometry == "2560x1440x24"


@pytest.mark.unit
def test_linux_virtual_display_stop_without_start_is_safe():
    """``stop()`` before ``start()`` must be a no-op — supports the
    ``__exit__`` path on a launcher that failed before Xvfb was spawned.
    Verifies no AttributeError on env restore (saved_env is empty)."""
    vd = _LinuxVirtualDisplay()
    vd.stop()
    vd.stop()
    assert vd._proc is None
    assert vd._display is None
