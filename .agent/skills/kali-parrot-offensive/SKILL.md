---
name: kali-parrot-offensive
description: Kali Linux and Parrot OS offensive toolkit orchestration for Cyber-Ifrit on Debian-based security distros. Use for red team, adversary emulation, bug bounty, and authorized vigilante/threat-hunt research. Partner target ParrotSec (Palinuro). Always call sec_distro_inventory first.
metadata:
  author: cyber-ifrit
  version: 1.0.0
  partner: ParrotSec
---

# Kali / Parrot Offensive Toolkit

Cyber-Ifrit is designed for **Debian-based security distributions** — especially **Parrot OS** (ParrotSec) and **Kali Linux**. On engagement start, call **`sec_distro_inventory`** to see which native tools are on PATH.

## Authorization (non-negotiable)

- **Red team / bug bounty / own assets / signed ROE only**
- Emulate black-hat TTPs to **defend** — not to attack unauthorized third parties
- "Vigilante" research = threat intel on infrastructure you have **legal permission** to analyze (honeypot, LE coordination, vendor program, your own systems)

## Workflow

1. `sec_distro_inventory` — map distro + available tools
2. Scope lock — exact target URL/host
3. Pick **native tools** from inventory before writing custom scripts
4. `run_command` with real binaries (nmap, nuclei, sqlmap, …)
5. Validate → comprehensive report → Ctrl+Shift+V preview

## Tool categories (sec_distro_inventory)

| Category | Examples |
|----------|----------|
| recon_osint | nmap, amass, subfinder, theharvester, httpx, nuclei |
| web_app | ffuf, gobuster, sqlmap, nikto, dalfox, wpscan, zaproxy |
| network_sniff | tshark, bettercap, responder, mitmproxy, testssl |
| wireless | aircrack-ng, wifite, reaver, kismet |
| exploitation | msfconsole, msfvenom, searchsploit, impacket-* |
| credentials | hashcat, john, hydra (authorized lab accounts) |
| post_exploit_ad | bloodhound, crackmapexec, netexec, evil-winrm |
| reverse_engineering | ghidra, radare2, apktool, jadx, frida |
| parrot_privacy_ops | anonsurf, mat2, onionshare |
| container_cloud | trivy, kube-hunter, prowler |

## Parrot OS specifics

- **anonsurf** — route traffic via Tor when ROE allows anonymous recon
- **Parrot menu** — `/usr/share/parrot` tool layout
- **MATE/rolling** — `sudo apt update && sudo apt install <tool>`
- Partner ecosystem: ParrotSec tooling + Cyber-Ifrit agent orchestration

## Kali Linux specifics

- Metasploit: `msfconsole`, `msfdb init`, `searchsploit`
- Kali menu paths under `/usr/share/kali-menu`
- `apt install kali-linux-default` meta-package for full arsenal

## Adversary emulation phases (MITRE)

| Phase | Tools |
|-------|-------|
| TA0043 Recon | nmap, amass, httpx, theharvester |
| TA0001 Initial Access | nuclei, sqlmap, custom PoC |
| TA0006 Cred Access | hydra (lab), responder, hashcat |
| TA0008 Lateral | crackmapexec, impacket |
| TA0010 Exfil | document only — no real exfil on prod |

## Install missing tools

```bash
sudo apt update
sudo apt install -y nmap nuclei sqlmap ffuf
```

On Parrot/Kali most tools are preinstalled; inventory shows gaps.

## Slash commands

- `/kali <target>` — Kali-class tool orchestration
- `/parrot <target>` — Parrot OS native toolkit + anonsurf when allowed
- `/bugbounty <url>` — web scope + inventory + report

## Related skills

- `.agent/skills/bugbounty-hunter/`
- `.agent/skills/ethical-hacking-methodology/`
- `.agent/skills/red-team-tools/`
