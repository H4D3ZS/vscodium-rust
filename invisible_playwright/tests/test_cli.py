import subprocess
import sys
from pathlib import Path

import pytest

from invisible_playwright import cli


@pytest.mark.unit
def test_version_subcommand():
    r = subprocess.run(
        [sys.executable, "-m", "invisible_playwright", "version"],
        capture_output=True, text=True, check=True,
    )
    assert "firefox-" in r.stdout
    assert "invisible_playwright" in r.stdout.lower()


@pytest.mark.unit
def test_help_subcommand():
    r = subprocess.run(
        [sys.executable, "-m", "invisible_playwright", "--help"],
        capture_output=True, text=True,
    )
    assert r.returncode == 0
    assert "fetch" in r.stdout
    assert "path" in r.stdout
    assert "clear-cache" in r.stdout


# CL1: clear-cache with existing cache prints "removed:" + path
@pytest.mark.unit
def test_clear_cache_with_existing_cache(tmp_path, monkeypatch, capsys):
    cache = tmp_path / "existing-cache"
    cache.mkdir()
    (cache / "marker").write_text("x")
    monkeypatch.setattr("invisible_playwright.cli.cache_root", lambda: cache)

    rc = cli.main(["clear-cache"])

    captured = capsys.readouterr()
    assert rc == 0
    assert captured.out.startswith("removed:")
    assert str(cache) in captured.out
    assert not cache.exists()


# CL2: clear-cache with no cache prints "nothing to remove:"
@pytest.mark.unit
def test_clear_cache_with_no_cache(tmp_path, monkeypatch, capsys):
    cache = tmp_path / "missing-cache"
    assert not cache.exists()
    monkeypatch.setattr("invisible_playwright.cli.cache_root", lambda: cache)

    rc = cli.main(["clear-cache"])

    captured = capsys.readouterr()
    assert rc == 0
    assert captured.out.startswith("nothing to remove:")
    assert str(cache) in captured.out


# CL3: path when binary exists prints path, exit 0
@pytest.mark.unit
def test_path_subcommand_when_binary_exists(tmp_path, monkeypatch, capsys):
    fake_binary = tmp_path / "firefox.exe"
    fake_binary.write_text("x")
    monkeypatch.setattr("invisible_playwright.cli.ensure_binary", lambda: fake_binary)

    rc = cli.main(["path"])

    captured = capsys.readouterr()
    assert rc == 0
    assert str(fake_binary) in captured.out
    assert captured.err == ""


# CL4: path when binary missing prints to stderr, exit 1
@pytest.mark.unit
def test_path_subcommand_when_binary_missing(monkeypatch, capsys):
    def boom():
        raise RuntimeError("download failed")
    monkeypatch.setattr("invisible_playwright.cli.ensure_binary", boom)

    rc = cli.main(["path"])

    captured = capsys.readouterr()
    assert rc == 1
    assert "error:" in captured.err
    assert "download failed" in captured.err
    assert captured.out == ""


# CL5: no subcommand → argparse error, exit != 0
@pytest.mark.unit
def test_no_subcommand_errors():
    with pytest.raises(SystemExit) as exc_info:
        cli.main([])
    assert exc_info.value.code != 0


# CL6: unknown subcommand → argparse error
@pytest.mark.unit
def test_unknown_subcommand_errors():
    with pytest.raises(SystemExit) as exc_info:
        cli.main(["bogus"])
    assert exc_info.value.code != 0


# Extra: fetch happy path with mocked ensure_binary
@pytest.mark.unit
def test_fetch_subcommand_prints_path(tmp_path, monkeypatch, capsys):
    fake_binary = tmp_path / "firefox.exe"
    fake_binary.write_text("x")
    monkeypatch.setattr("invisible_playwright.cli.ensure_binary", lambda: fake_binary)

    rc = cli.main(["fetch"])

    captured = capsys.readouterr()
    assert rc == 0
    assert str(fake_binary) in captured.out
