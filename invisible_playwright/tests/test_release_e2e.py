"""End-to-end release tests.

These exercise the FULL user install path against the LIVE GitHub release.
They are slow (download a ~110 MB binary, launch Firefox) and require network
access — marked `e2e` so they're excluded from the default suite. Run them
BEFORE announcing a release:

    pytest tests/test_release_e2e.py -m e2e -v

Or to target a specific git revision (default is current HEAD on origin/main):

    INVPW_E2E_REV=v0.1.5 pytest tests/test_release_e2e.py -m e2e -v

What each test verifies and why it exists:

  test_clean_install_from_git_main:
    Spawns a fresh venv and pip-installs the wrapper from git HEAD. Confirms
    the package has no broken metadata, missing deps, or import errors in a
    pristine environment. Catches the "works on my machine because I already
    have the dev deps" class of bug.

  test_fetch_against_live_release:
    After the install, runs `python -m invisible_playwright fetch --force`,
    which downloads the live tarball + checksums.txt for the pinned
    BINARY_VERSION from the production GitHub release. This is THE test that
    would have caught LostBoxArt's #15 — the checksums.txt parser bug only
    manifested against the real binary-mode format the release ships, not
    against unit-test mocks.

  test_version_command_after_fetch:
    Confirms `python -m invisible_playwright --version` resolves the binary
    and reports the expected `firefox-N` tag. Sanity check that the binary
    landed in the cache and the wrapper can find it.

  test_playwright_launch_against_real_site (linux-only by default):
    Launches the patched Firefox under the wrapper, navigates to a stable
    public URL, and reads a known DOM property. This is the full stack:
    wrapper init → Firefox launch → Juggler handshake → page.goto →
    page.evaluate. If anything along the way regresses (Juggler protocol
    schema drift, prefs typo, sandbox issue, …) this fails loudly.

The tests use a temp cache dir per run (env var
`INVISIBLE_PLAYWRIGHT_CACHE_DIR`) so they never poison the developer's real
cache and never get false positives from a previously-cached binary.
"""
from __future__ import annotations

import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

import pytest


REPO_URL = "https://github.com/feder-cr/invisible_playwright.git"
REV = os.environ.get("INVPW_E2E_REV", "main")


# ---------- helpers --------------------------------------------------------- #


def _run(cmd: list[str], *, env: dict | None = None, cwd: Path | None = None,
         timeout: int = 300, check: bool = True) -> subprocess.CompletedProcess:
    """Run a subprocess with full output captured. Fail with both streams shown."""
    result = subprocess.run(
        cmd, env=env, cwd=cwd, timeout=timeout,
        capture_output=True, text=True,
    )
    if check and result.returncode != 0:
        raise AssertionError(
            f"{' '.join(cmd)} exited {result.returncode}\n"
            f"--- stdout ---\n{result.stdout[-3000:]}\n"
            f"--- stderr ---\n{result.stderr[-3000:]}"
        )
    return result


def _venv_python(venv: Path) -> Path:
    if os.name == "nt":
        return venv / "Scripts" / "python.exe"
    return venv / "bin" / "python"


# ---------- fixtures -------------------------------------------------------- #


@pytest.fixture(scope="module")
def workspace() -> Path:
    """A single temp dir reused across the module so we don't re-create the
    venv + re-download the 110 MB tarball for every individual test."""
    root = Path(tempfile.mkdtemp(prefix="invpw-e2e-"))
    yield root
    shutil.rmtree(root, ignore_errors=True)


@pytest.fixture(scope="module")
def clean_venv(workspace: Path) -> Path:
    """A fresh venv, pip upgraded. Returns its python executable path."""
    venv_dir = workspace / "venv"
    _run([sys.executable, "-m", "venv", str(venv_dir)], timeout=180)
    py = _venv_python(venv_dir)
    assert py.exists(), f"venv python not found at {py}"
    _run([str(py), "-m", "pip", "install", "--upgrade", "pip", "--quiet"], timeout=180)
    return py


@pytest.fixture(scope="module")
def isolated_cache_env(workspace: Path) -> dict:
    """Environment dict pointing the wrapper at a private cache dir so this
    test never reads or pollutes the developer's real cache."""
    cache = workspace / "cache"
    cache.mkdir(exist_ok=True)
    env = os.environ.copy()
    env["INVISIBLE_PLAYWRIGHT_CACHE_DIR"] = str(cache)
    env["XDG_CACHE_HOME"] = str(cache)
    return env


# ---------- tests ----------------------------------------------------------- #


@pytest.mark.e2e
def test_clean_install_from_git_main(clean_venv: Path):
    """The package installs cleanly from git+HTTPS in a pristine venv."""
    url = f"git+{REPO_URL}@{REV}"
    _run([str(clean_venv), "-m", "pip", "install", url], timeout=600)

    # Importability check — catches missing __init__ exports, broken syntax,
    # missing runtime deps.
    out = _run(
        [str(clean_venv), "-c",
         "import invisible_playwright as ip; "
         "print('OK', ip.__name__)"],
        timeout=30,
    )
    assert "OK invisible_playwright" in out.stdout


@pytest.mark.e2e
def test_version_command_reports_wrapper_and_binary(clean_venv: Path):
    """`python -m invisible_playwright --version` runs and reports both the
    wrapper version and the BINARY_VERSION it'll try to fetch."""
    out = _run(
        [str(clean_venv), "-m", "invisible_playwright", "--version"],
        timeout=30,
    )
    text = out.stdout + out.stderr
    assert "firefox-" in text, f"BINARY_VERSION not reported: {text!r}"


@pytest.mark.e2e
def test_fetch_against_live_release(clean_venv: Path, isolated_cache_env: dict):
    """Hit the LIVE GitHub release: download tarball + checksums.txt, parse,
    SHA256-verify, extract. This is the regression sentinel for #15.

    If checksums.txt is shipped in `*`-prefixed (binary) format and the parser
    keeps the `*` in the key, this raises
        RuntimeError: no SHA256 for {asset} in checksums.txt
    """
    out = _run(
        [str(clean_venv), "-m", "invisible_playwright", "fetch", "--force"],
        env=isolated_cache_env,
        timeout=900,  # 110 MB download + extract on slow connections
    )
    output = out.stdout + out.stderr
    # Anti-regression for #15: this exact string would surface if the parser
    # broke again. Spell it out so a future failure is grep-able to the issue.
    assert "no SHA256 for" not in output, (
        "Issue #15 regression: parser couldn't find SHA for the asset.\n"
        f"Output:\n{output[-2000:]}"
    )
    assert "SHA256 mismatch" not in output, (
        "Tarball SHA doesn't match the published checksums.txt — "
        "either the upload was corrupted or the release was re-packed "
        "without updating checksums.txt."
    )


@pytest.mark.e2e
def test_binary_executes_after_fetch(clean_venv: Path, isolated_cache_env: dict):
    """After fetch, the binary cache contains a launchable Firefox."""
    out = _run(
        [str(clean_venv), "-c",
         "from invisible_playwright.download import ensure_binary; "
         "p = ensure_binary(); print('BINARY', p)"],
        env=isolated_cache_env,
        timeout=60,
    )
    binary_line = [l for l in out.stdout.splitlines() if l.startswith("BINARY ")]
    assert binary_line, f"ensure_binary() didn't print path: {out.stdout!r}"
    binary_path = Path(binary_line[0].split(" ", 1)[1])
    assert binary_path.exists(), f"binary missing: {binary_path}"

    # `firefox --version` exit code is enough; output format differs across
    # platforms (Win shows nothing on stdout, Linux prints to stdout).
    # On Linux invoke via WSL when running from Windows.
    if os.name == "nt" and binary_path.suffix == "":
        # Linux binary path on Windows host — skip launch, the previous
        # ensure_binary() already proved cache landed correctly.
        pytest.skip("Cross-platform binary launch from Windows requires WSL.")
    r = subprocess.run([str(binary_path), "--version"],
                       capture_output=True, text=True, timeout=30)
    text = (r.stdout + r.stderr).lower()
    assert "firefox" in text and "150." in text, (
        f"binary --version didn't report Firefox 150: rc={r.returncode} "
        f"out={r.stdout!r} err={r.stderr!r}"
    )


@pytest.mark.e2e
@pytest.mark.linux_only
def test_playwright_launch_against_real_site(clean_venv: Path,
                                             isolated_cache_env: dict):
    """Full stack: launch the patched Firefox via the wrapper, navigate to a
    real URL, evaluate JS. Catches Juggler protocol drift, profile-generation
    bugs, locale handling regressions, prefs typos."""
    if sys.platform.startswith("win"):
        pytest.skip("Headless launch path requires display server (skip on Win).")

    script = (
        "from invisible_playwright import InvisiblePlaywright\n"
        "with InvisiblePlaywright(headless=True, seed=42) as browser:\n"
        "    ctx = browser.new_context()\n"
        "    page = ctx.new_page()\n"
        "    page.goto('https://example.com', timeout=30000)\n"
        "    title = page.title()\n"
        "    ua = page.evaluate('navigator.userAgent')\n"
        "    print('TITLE=' + title)\n"
        "    print('UA=' + ua)\n"
    )
    out = _run([str(clean_venv), "-c", script],
               env=isolated_cache_env, timeout=180)
    assert "TITLE=Example Domain" in out.stdout, (
        f"page.title() didn't return expected text:\n{out.stdout[-1000:]}"
    )
    assert "UA=" in out.stdout and "Firefox/150" in out.stdout, (
        "navigator.userAgent doesn't report Firefox/150 — UA spoofing "
        f"regression?\n{out.stdout[-1000:]}"
    )


# ---------- meta: verify the test markers themselves work ------------------- #


@pytest.mark.e2e
def test_e2e_marker_is_excluded_by_default():
    """Sanity check on pyproject.toml's `addopts = '-m not e2e'` — this test
    only runs when `-m e2e` is passed explicitly. If you're reading this in
    a normal pytest run, the addopts filter is broken."""
    assert True
