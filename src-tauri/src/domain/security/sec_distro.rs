//! Security-distribution detection (Kali, Parrot OS, Debian) and offensive-tool
//! inventory for authorized red-team / bug-bounty engagements.

use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecDistro {
    Kali,
    Parrot,
    Debian,
    Unknown,
}

impl SecDistro {
    pub fn as_str(self) -> &'static str {
        match self {
            SecDistro::Kali => "kali",
            SecDistro::Parrot => "parrot",
            SecDistro::Debian => "debian",
            SecDistro::Unknown => "unknown",
        }
    }
}

/// Detect whether we're on a security-focused GNU/Linux distro.
pub fn detect_sec_distro() -> SecDistro {
    if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
        let lower = content.to_lowercase();
        if lower.contains("id=kali") || lower.contains("id_like=debian") && lower.contains("kali") {
            return SecDistro::Kali;
        }
        if lower.contains("id=parrot") || lower.contains("parrot os") || lower.contains("parrotsec") {
            return SecDistro::Parrot;
        }
        if lower.contains("id=debian") {
            return SecDistro::Debian;
        }
    }
    if Path::new("/usr/share/kali-menu").exists() {
        return SecDistro::Kali;
    }
    if Path::new("/usr/share/parrot").exists() || Path::new("/etc/anonsurf").exists() {
        return SecDistro::Parrot;
    }
    SecDistro::Unknown
}

/// Curated tools shipped on Kali / Parrot (and common on Debian security installs).
/// Grouped by engagement phase — agent should call `sec_distro_inventory` first.
pub fn tool_catalog() -> BTreeMap<&'static str, &'static [&'static str]> {
    let mut m = BTreeMap::new();
    m.insert(
        "recon_osint",
        &[
            "nmap", "masscan", "rustscan", "amass", "subfinder", "assetfinder", "theharvester",
            "dnsenum", "fierce", "dig", "host", "whois", "recon-ng", "spiderfoot", "maltego",
            "shodan", "httpx", "httprobe", "whatweb", "wafw00f",
        ][..],
    );
    m.insert(
        "web_app",
        &[
            "ffuf", "gobuster", "dirb", "feroxbuster", "nikto", "nuclei", "sqlmap", "wpscan",
            "commix", "dalfox", "xsstrike", "wfuzz", "arjun", "paramspider", "crlfuzz",
            "zaproxy", "curl", "wget",
        ][..],
    );
    m.insert(
        "network_sniff",
        &[
            "wireshark", "tshark", "tcpdump", "ettercap", "bettercap", "responder", "mitmproxy",
            "sslscan", "testssl", "openssl",
        ][..],
    );
    m.insert(
        "wireless",
        &["aircrack-ng", "airodump-ng", "aireplay-ng", "wifite", "reaver", "bully", "kismet"][..],
    );
    m.insert(
        "exploitation",
        &[
            "msfconsole", "msfvenom", "searchsploit", "msfdb", "armitage", "beef-xss",
            "commix", "impacket-psexec", "impacket-wmiexec", "impacket-secretsdump",
        ][..],
    );
    m.insert(
        "credentials",
        &[
            "hashcat", "john", "hydra", "medusa", "ncrack", "patator", "cewl", "crunch",
            "hash-identifier",
        ][..],
    );
    m.insert(
        "post_exploit_ad",
        &[
            "bloodhound-python", "bloodhound", "crackmapexec", "netexec", "evil-winrm",
            "psexec.py", "wmiexec.py", "secretsdump.py", "ldapdomaindump", "kerbrute",
        ][..],
    );
    m.insert(
        "reverse_engineering",
        &[
            "ghidra", "radare2", "r2", "gdb", "objdump", "strings", "binwalk", "volatility3",
            "volatility", "apktool", "jadx", "frida", "lief",
        ][..],
    );
    m.insert(
        "parrot_privacy_ops",
        &["anonsurf", "parrot-updater", "cryptsetup", "mat2", "onionshare"][..],
    );
    m.insert(
        "container_cloud",
        &["docker", "kubectl", "helm", "trivy", "kube-hunter", "prowler"][..],
    );
    m
}

fn which(cmd: &str) -> bool {
    #[cfg(unix)]
    {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(format!("command -v {} >/dev/null 2>&1", cmd))
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }
    #[cfg(windows)]
    {
        std::process::Command::new("where")
            .arg(cmd)
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }
}

/// Probe PATH for curated offensive tools; returns structured JSON for the agent.
pub fn inventory_json(category: Option<&str>) -> Value {
    let distro = detect_sec_distro();
    let catalog = tool_catalog();
    let mut available: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut missing: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut total_avail = 0u32;
    let mut total_checked = 0u32;

    for (cat, tools) in &catalog {
        if let Some(filter) = category {
            if *cat != filter {
                continue;
            }
        }
        let mut a = Vec::new();
        let mut m = Vec::new();
        for t in *tools {
            total_checked += 1;
            if which(t) {
                a.push(t.to_string());
                total_avail += 1;
            } else {
                m.push(t.to_string());
            }
        }
        if !a.is_empty() {
            available.insert(cat.to_string(), a);
        }
        if !m.is_empty() {
            missing.insert(cat.to_string(), m);
        }
    }

    json!({
        "distro": distro.as_str(),
        "distro_label": match distro {
            SecDistro::Kali => "Kali Linux",
            SecDistro::Parrot => "Parrot OS (ParrotSec)",
            SecDistro::Debian => "Debian GNU/Linux",
            SecDistro::Unknown => "Unknown / non-security distro",
        },
        "partner_note": "Cyber-Ifrit targets Parrot OS / Kali-class Debian security distros. Prefer native tools from inventory before reinventing with curl/python.",
        "tools_available": available,
        "tools_missing": missing,
        "summary": format!("{}/{} curated offensive tools on PATH", total_avail, total_checked),
        "usage": "Run sec_distro_inventory at engagement start. Use listed binaries via run_command against IN-SCOPE targets only. Install missing tools with apt (Parrot/Kali/Debian).",
        "categories": catalog.keys().collect::<Vec<_>>(),
    })
}

/// Adversary-emulation playbook for prompts (authorized scope only).
pub const ADVERSARY_EMULATION_PLAYBOOK: &str = r#"ADVERSARY EMULATION PLAYBOOK (Red Team / authorized bug bounty — NOT unauthorized targets):

MINDSET: Think like a skilled adversary (black-hat TTPs) to defend like a red teamer. Every technique is for assets you OWN or are explicitly authorized to test.

START EVERY ENGAGEMENT:
1. sec_distro_inventory — discover Kali/Parrot/Debian tools on PATH
2. Scope lock — exact URL/host; no localhost pivot on external web tests
3. Pick native tools from inventory before writing custom scripts

PHASE → TOOL MAPPING (use what's installed):
- Recon/OSINT: nmap, amass, subfinder, theharvester, httpx, nuclei
- Web: ffuf, gobuster, sqlmap, nikto, dalfox, wpscan, zaproxy
- Network: tshark, bettercap, responder, mitmproxy, testssl
- Exploit validation: searchsploit, msfconsole (lab), custom PoC via run_command
- Creds: hashcat, john, hydra (authorized brute on owned lab accounts only)
- AD/Post: bloodhound, crackmapexec/netexec, impacket-* (internal pentest scope)
- Mobile/RE: apktool, jadx, frida, ghidra, radare2
- IDA/Ghidra MCP (agent): install from IDE MCP Store → Security — ida-pro-mcp (decompile/xrefs/patch) or pyghidra-mcp; requires local licenses + uv
- Parrot ops: anonsurf for anonymity on authorized external tests (when ROE allows)

VIGILANTE / GREY-HAT RESEARCH (still requires authorization):
- Threat-hunt criminal infrastructure you have legal permission to analyze (LE, vendor, own honeypot)
- Document TTPs for blue-team detection — never operate on third-party systems without ROE

REPORTING: reports/<slug>/PENTEST-REPORT-*.md with kill chain + FIND-NNN sections."#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_has_recon() {
        assert!(tool_catalog().contains_key("recon_osint"));
    }

    #[test]
    fn inventory_returns_json() {
        let v = inventory_json(None);
        assert!(v.get("distro").is_some());
        assert!(v.get("tools_available").is_some());
    }
}
