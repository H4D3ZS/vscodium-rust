# 1. The Environment

Claude runs inside a Linux VM (Ubuntu 24, x86_64), spun up fresh per task.
It runs as root inside the container — but that root has NO host privileges.

## Verified live specs
- Kernel:  Linux 6.18
- Python:  3.12.3
- Node.js: 22.x
- RAM:     ~3.9 GB
- Disk:    ~252 GB
- Tools:   git, curl, wget, ffmpeg, pip, npm

## Why root inside a container is fine
Normally running as root is dangerous. Inside a Docker/VM sandbox:
  - The container IS the security boundary
  - root inside container != root on host
  - Even if code breaks out of Python, it still hits the container wall
  - Network egress is restricted to an allowlist (no arbitrary internet)

This is the "defense by containment" model.
