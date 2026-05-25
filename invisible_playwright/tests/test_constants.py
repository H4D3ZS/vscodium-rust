import pytest

from invisible_playwright.constants import (
    ARCHIVE_NAME,
    BINARY_BASENAME,
    BINARY_ENTRY_REL,
    BINARY_VERSION,
    FIREFOX_UPSTREAM_VERSION,
    RELEASE_URL_TEMPLATE,
)


@pytest.mark.unit
def test_binary_version_format():
    assert BINARY_VERSION.startswith("firefox-")
    assert BINARY_VERSION.split("-", 1)[1].isdigit()


@pytest.mark.unit
def test_archive_name_windows():
    name = ARCHIVE_NAME("win32", "AMD64")
    assert name.endswith(".zip")
    assert "win-x86_64" in name


@pytest.mark.unit
def test_archive_name_linux():
    name = ARCHIVE_NAME("linux", "x86_64")
    assert name.endswith(".tar.gz")
    assert "linux-x86_64" in name


@pytest.mark.unit
def test_archive_name_unsupported_raises():
    with pytest.raises(NotImplementedError):
        ARCHIVE_NAME("darwin", "arm64")


@pytest.mark.unit
def test_binary_basename_format():
    assert "firefox" in BINARY_BASENAME.lower()
    assert "stealth" in BINARY_BASENAME.lower()


# ---- Comprehensive ARCHIVE_NAME edge cases -------------------------------- #
# Same risk shape as bug #15: a missed format assumption (sha256sum binary
# mode) silently produced wrong output. Same class of bug here would be
# uppercase platform string or odd machine value passing through to a
# wrong-named asset on the CDN and 404-ing.

@pytest.mark.unit
@pytest.mark.parametrize("platform_key,machine,expected_substring", [
    ("win32",  "AMD64",     "win-x86_64.zip"),       # Windows reports AMD64
    ("win32",  "amd64",     "win-x86_64.zip"),       # lowercase variant
    ("win32",  "x86_64",    "win-x86_64.zip"),       # mingw-style
    ("linux",  "x86_64",    "linux-x86_64.tar.gz"),  # standard Linux
    ("linux",  "AMD64",     "linux-x86_64.tar.gz"),  # odd but plausible
    ("Linux",  "x86_64",    "linux-x86_64.tar.gz"),  # case-insensitive platform
    ("WIN32",  "AMD64",     "win-x86_64.zip"),       # ALL CAPS platform
])
def test_archive_name_accepts_case_variations(platform_key, machine, expected_substring):
    """sys.platform / platform.machine() return inconsistent casing across
    OS versions and Python versions. The asset filename must be stable
    regardless — otherwise the CDN 404s."""
    assert ARCHIVE_NAME(platform_key, machine).endswith(expected_substring)


@pytest.mark.unit
@pytest.mark.parametrize("machine", ["i386", "i686", "ppc64le", "armv7l", "riscv64"])
def test_archive_name_rejects_unsupported_arches(machine):
    """Unsupported arches must raise NotImplementedError with the bad value
    in the message — silent fallback to a default arch would download the
    wrong binary, run, and fingerprint differently."""
    with pytest.raises(NotImplementedError, match=machine):
        ARCHIVE_NAME("linux", machine)


@pytest.mark.unit
@pytest.mark.parametrize("machine", ["arm64", "aarch64"])
def test_archive_name_arm64_not_yet_supported(machine):
    """ARM64 is a frequent request (issue #6). Until binaries exist for it,
    ARCHIVE_NAME should hard-fail rather than silently degrade. If this test
    starts failing because someone shipped ARM64 builds, replace it with the
    positive case."""
    with pytest.raises(NotImplementedError):
        ARCHIVE_NAME("linux", machine)


@pytest.mark.unit
@pytest.mark.parametrize("platform_key", ["darwin", "freebsd", "cygwin", "openbsd"])
def test_archive_name_rejects_unsupported_platforms(platform_key):
    """Same logic — non-Linux/non-Windows platforms must raise, not silently
    pick one of the two."""
    with pytest.raises(NotImplementedError, match=platform_key):
        ARCHIVE_NAME(platform_key, "x86_64")


# ---- ARCHIVE_NAME ↔ BINARY_ENTRY_REL invariant ---------------------------- #
# For every supported platform there MUST be an entry in BINARY_ENTRY_REL,
# otherwise ensure_binary() will raise NotImplementedError AFTER having
# already downloaded a 110 MB tarball — terrible UX.

@pytest.mark.unit
def test_binary_entry_rel_covers_every_supported_platform():
    """If ARCHIVE_NAME accepts a platform key, BINARY_ENTRY_REL must declare
    where the executable lives inside the archive for it."""
    for plat in ["win32", "linux"]:
        ARCHIVE_NAME(plat, "x86_64")  # must not raise
        assert plat in BINARY_ENTRY_REL, (
            f"ARCHIVE_NAME accepts {plat!r} but BINARY_ENTRY_REL has no entry "
            f"— ensure_binary() will fail late after a 110 MB download."
        )


@pytest.mark.unit
def test_binary_entry_rel_extension_matches_platform():
    """firefox.exe on Windows, plain `firefox` on Linux."""
    assert BINARY_ENTRY_REL["win32"].endswith(".exe")
    assert not BINARY_ENTRY_REL["linux"].endswith(".exe")
    assert BINARY_ENTRY_REL["linux"] == "firefox"


# ---- RELEASE_URL_TEMPLATE shape ------------------------------------------- #

@pytest.mark.unit
def test_release_url_template_is_https():
    """No http://. GitHub redirects http but we never accept the redirect."""
    assert RELEASE_URL_TEMPLATE.startswith("https://github.com/")


@pytest.mark.unit
def test_release_url_template_has_required_placeholders():
    """{tag} and {asset} must both be present, otherwise _resolve_asset_url
    won't format a usable URL and downloads fail with confusing 404s."""
    assert "{tag}" in RELEASE_URL_TEMPLATE
    assert "{asset}" in RELEASE_URL_TEMPLATE


@pytest.mark.unit
def test_release_url_template_formats_cleanly():
    """Confirm .format() actually substitutes — catches typos like {tags}."""
    url = RELEASE_URL_TEMPLATE.format(tag="firefox-99", asset="thing.zip")
    assert "{" not in url and "}" not in url
    assert "firefox-99" in url
    assert "thing.zip" in url


@pytest.mark.unit
def test_release_url_points_at_owned_repo():
    """The template MUST point at an owner/repo the maintainer actually
    controls. A typo here would direct everyone's downloads at a stranger's
    GitHub account — silent supply-chain risk."""
    assert "/feder-cr/invisible_playwright/" in RELEASE_URL_TEMPLATE, (
        f"RELEASE_URL_TEMPLATE was changed to point elsewhere: "
        f"{RELEASE_URL_TEMPLATE!r}. Update this test only if the move is intentional."
    )


# ---- Firefox upstream version sanity -------------------------------------- #

@pytest.mark.unit
def test_firefox_upstream_version_is_three_part_semver():
    parts = FIREFOX_UPSTREAM_VERSION.split(".")
    assert len(parts) >= 2, f"version too short: {FIREFOX_UPSTREAM_VERSION!r}"
    for p in parts:
        assert p.isdigit(), f"non-numeric segment in {FIREFOX_UPSTREAM_VERSION!r}"


@pytest.mark.unit
def test_binary_basename_includes_upstream_version():
    """The basename references the upstream version, so the asset filename
    on the CDN encodes which Firefox was patched. Bumping FIREFOX_UPSTREAM_VERSION
    without rebuilding would leave stale binaries — this guards against
    accidentally desyncing the two."""
    assert FIREFOX_UPSTREAM_VERSION in BINARY_BASENAME


@pytest.mark.unit
@pytest.mark.parametrize("plat", ["win32", "linux"])
def test_archive_name_includes_upstream_version(plat):
    """Same desync guard, from the other direction."""
    assert FIREFOX_UPSTREAM_VERSION in ARCHIVE_NAME(plat, "x86_64")
