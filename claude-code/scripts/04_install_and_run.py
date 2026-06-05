"""
Script 4: Dynamic pip install and live HTTP request
This is exactly what Claude does during real tasks.
Run with: python3 scripts/04_install_and_run.py
"""
import subprocess, sys

# This is the pattern Claude uses: install on demand, no pre-setup needed
print("Step 1: Installing 'httpx' dynamically...")
subprocess.run(
    [sys.executable, "-m", "pip", "install", "httpx",
     "--break-system-packages", "-q"],
    check=True
)
print("       Installed.")

import httpx

print("\nStep 2: Making a request to PyPI (an allowed domain)...")
r = httpx.get("https://pypi.org/pypi/httpx/json", timeout=10)
data = r.json()
print(f"       httpx latest on PyPI: {data['info']['version']}")
print(f"       HTTP status: {r.status_code}")

print("\nDone! This is how Claude installs packages mid-task without asking you to set anything up.")
