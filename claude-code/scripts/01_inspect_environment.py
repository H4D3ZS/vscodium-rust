"""
Script 1: Inspect the sandbox environment
Run with: python3 scripts/01_inspect_environment.py
Works on your local machine too — compare the output!
"""
import platform, os, subprocess, sys, shutil

print("=" * 50)
print("SYSTEM")
print("=" * 50)
print(f"OS:       {platform.system()} {platform.release()}")
print(f"Machine:  {platform.machine()}")
print(f"Python:   {sys.version}")
print(f"User:     {os.getenv('USER', os.getenv('USERNAME', 'unknown'))}  uid={os.getuid() if hasattr(os,'getuid') else 'N/A'}")
print(f"Hostname: {platform.node()}")

print("\n" + "=" * 50)
print("DISK & MEMORY")
print("=" * 50)
total, used, free = shutil.disk_usage("/")
print(f"Disk:  total={total/1e9:.0f}GB  used={used/1e9:.0f}GB  free={free/1e9:.0f}GB")
try:
    with open("/proc/meminfo") as f:
        for line in f:
            if line.startswith(("MemTotal", "MemAvailable")):
                print("RAM:", line.strip())
except FileNotFoundError:
    print("RAM info not available (not Linux)")

print("\n" + "=" * 50)
print("TOOLS AVAILABLE")
print("=" * 50)
for tool in ["python3", "node", "npm", "pip3", "git", "curl", "ffmpeg", "wget"]:
    path = shutil.which(tool)
    print(f"  {tool:12} -> {path or 'NOT FOUND'}")

print("\n" + "=" * 50)
print("FILESYSTEM MOUNTS (Linux only)")
print("=" * 50)
try:
    result = subprocess.run(["mount"], capture_output=True, text=True)
    for line in result.stdout.splitlines():
        if "/mnt" in line or "home" in line:
            print(" ", line)
except Exception as e:
    print(f"  (skipped: {e})")
