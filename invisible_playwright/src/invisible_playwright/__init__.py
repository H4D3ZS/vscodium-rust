"""invisible_playwright — Playwright wrapper for a patched Firefox with stealth profile.

Quickstart:

    from invisible_playwright import InvisiblePlaywright

    with InvisiblePlaywright() as browser:        # random seed
        page = browser.new_page()
        page.goto("https://example.com")

    with InvisiblePlaywright(seed=42) as browser: # deterministic
        ...

    with InvisiblePlaywright(humanize=True) as browser:  # human-like cursor motion
        page = browser.new_page()
        page.click("#submit")   # expanded into a Bezier trajectory
"""
from .launcher import InvisiblePlaywright
from .constants import BINARY_VERSION, FIREFOX_UPSTREAM_VERSION

__version__ = "0.1.0"
__all__ = ["InvisiblePlaywright", "BINARY_VERSION", "FIREFOX_UPSTREAM_VERSION", "__version__"]
