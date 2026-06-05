"""
Script 3: Walk the important filesystem paths
Run with: python3 scripts/03_filesystem_tour.py
"""
import os

PATHS = {
    "/home/claude":                  "Claude scratchpad (resets each task)",
    "/mnt/user-data/uploads":        "Your uploaded files (read-only for Claude)",
    "/mnt/user-data/outputs":        "Claude deliverables (your downloads)",
    "/mnt/skills/public":            "Skill guides for file creation (read-only)",
    "/mnt/transcripts":              "Conversation logs (read-only)",
    "/var/run/docker.sock":          "Docker socket (should be ABSENT for security)",
}

print(f"{'STATUS':<8} {'WRITABLE':<10} PATH")
print("-" * 70)
for path, desc in PATHS.items():
    exists   = os.path.exists(path)
    writable = os.access(path, os.W_OK) if exists else False
    status   = "EXISTS" if exists else "ABSENT"
    perm     = "yes" if writable else "no" if exists else "-"
    print(f"{status:<8} {perm:<10} {path}")
    print(f"         {'':10} └─ {desc}")
    if exists and os.path.isdir(path):
        try:
            items = os.listdir(path)[:4]
            print(f"         {'':10}    contents: {items}")
        except PermissionError:
            print(f"         {'':10}    contents: (permission denied)")
    print()
