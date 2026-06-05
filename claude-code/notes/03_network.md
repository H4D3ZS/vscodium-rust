# 3. Network — The Egress Allowlist

Claude's outbound network is NOT open internet.
All requests go through an egress proxy that enforces a domain allowlist.

## Allowed domains (examples)
  pypi.org, files.pythonhosted.org    <- pip installs
  registry.npmjs.org                  <- npm installs
  github.com, raw.githubusercontent.com  <- git, curl from GitHub
  api.anthropic.com                   <- Claude API (for AI artifacts)
  *.adobe.io                          <- Adobe integrations

## Blocked
  Everything else. Arbitrary curl to external IPs fails silently.
  The proxy returns an x-deny-reason header on blocked requests.

## Why this matters for security
Even if malicious code ran inside the sandbox, it could NOT:
  - Exfiltrate data to an attacker's server
  - Download a second-stage payload from a random URL
  - Connect to a C2 (command & control) server

## Web search is separate
Claude's web_search tool is NOT a curl from the sandbox.
It runs as a separate privileged tool outside the container entirely.
