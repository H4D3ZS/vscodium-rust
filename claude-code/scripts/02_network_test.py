"""
Script 2: Test the network egress allowlist
Run with: python3 scripts/02_network_test.py

NOTE: Run this inside Claude's sandbox to see the real blocking.
On your local machine all requests will succeed (no proxy).
"""
import urllib.request, urllib.error

ALLOWED = [
    ("PyPI (allowed)",   "https://pypi.org/pypi/requests/json"),
    ("GitHub (allowed)", "https://raw.githubusercontent.com/anthropics/anthropic-sdk-python/main/README.md"),
]
BLOCKED = [
    ("Google (blocked)", "https://www.google.com"),
    ("Random IP",        "http://1.2.3.4"),
    ("Twitter (blocked)","https://twitter.com"),
]

def test(label, url):
    try:
        with urllib.request.urlopen(url, timeout=5) as r:
            print(f"  OK   {label} -> HTTP {r.status}")
    except urllib.error.HTTPError as e:
        print(f"  HTTP {label} -> {e.code}")
    except Exception as e:
        print(f"  FAIL {label} -> {type(e).__name__}: {e}")

print("Allowed domains:")
for label, url in ALLOWED:
    test(label, url)

print("\nBlocked domains (inside Claude sandbox):")
for label, url in BLOCKED:
    test(label, url)
