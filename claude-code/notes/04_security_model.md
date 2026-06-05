# 4. Security Model — Defense in Depth

## What the sandbox protects
  1. Host machine — from container escape
  2. External systems — from data exfiltration
  3. Other users — from cross-contamination between sessions

## Layers of defense

  Layer 1: VM / container isolation
    Claude runs inside a VM. Root inside != root on host.
    Kernel namespaces (pid, net, mount, user) isolate the process tree.

  Layer 2: Network egress allowlist
    Outbound traffic is proxied and filtered.
    Blocks exfiltration to arbitrary destinations.

  Layer 3: Read-only bind mounts
    /mnt/skills, /mnt/transcripts, /mnt/user-data/uploads are read-only.
    Claude cannot tamper with your original uploaded files.

  Layer 4: Ephemeral filesystem
    /home/claude resets between tasks.
    No persistent malicious state between sessions.

## What is NOT hardened (and why it doesn't matter)

  No seccomp profile (syscalls not filtered inside container)
    -> Doesn't matter: the VM boundary is the real wall.

  Running as root (no privilege separation inside container)
    -> Doesn't matter: root-in-container != root-on-host.

  No AppArmor/SELinux profiles visible
    -> Isolation is at hypervisor level, not LSM level.

## Common escape techniques and why they're mitigated

  Dirty COW / kernel exploits   -> blocked by VM hypervisor boundary
  Docker socket exposure         -> /var/run/docker.sock is NOT mounted
  Privileged mode               -> not enabled
  Host network mode             -> not used; custom egress proxy instead
