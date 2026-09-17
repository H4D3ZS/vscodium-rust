use std::net::{SocketAddr, TcpStream};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

static SUPERVISOR_RUNNING: AtomicBool = AtomicBool::new(false);

pub fn is_port_open(port: u16) -> bool {
    let addr_str = format!("127.0.0.1:{}", port);
    if let Ok(addr) = addr_str.parse::<SocketAddr>() {
        TcpStream::connect_timeout(&addr, Duration::from_millis(250)).is_ok()
    } else {
        false
    }
}

fn find_workspace_root() -> PathBuf {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            if parent.join("frida_touch_server.py").exists() {
                return parent.to_path_buf();
            }
            if let Some(grand) = parent.parent() {
                if grand.join("frida_touch_server.py").exists() {
                    return grand.to_path_buf();
                }
                if let Some(g2) = grand.parent() {
                    if g2.join("frida_touch_server.py").exists() {
                        return g2.to_path_buf();
                    }
                }
            }
        }
    }
    if let Ok(cwd) = std::env::current_dir() {
        if cwd.join("frida_touch_server.py").exists() {
            return cwd;
        }
    }
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

pub fn start_auto_daemon_supervisor(udid: String) {
    if SUPERVISOR_RUNNING.swap(true, Ordering::SeqCst) {
        return;
    }

    std::thread::spawn(move || {
        let ws = find_workspace_root();
        let go_ios = vscode_rust_app::domain::sentinel::jbops::go_ios_path();

        loop {
            // 1. Frida Touch Server (Port 8105)
            if !is_port_open(8105) {
                let script = ws.join("frida_touch_server.py");
                if script.exists() {
                    let mut cmd = Command::new("python");
                    cmd.args(["-u", script.to_string_lossy().as_ref()]);
                    cmd.current_dir(&ws);
                    cmd.stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());
                    #[cfg(windows)]
                    {
                        use std::os::windows::process::CommandExt;
                        cmd.creation_flags(0x08000000);
                    }
                    let _ = cmd.spawn();
                }
            }

            // 2. go-ios Userspace Tunnel (Port 60105)
            if !is_port_open(60105) {
                if let Some(ref bin) = go_ios {
                    let mut cmd = Command::new(bin);
                    let mut args = vec!["tunnel", "start", "--userspace"];
                    if !udid.is_empty() && udid != "auto" {
                        args.push("--udid");
                        args.push(&udid);
                    }
                    cmd.args(&args);
                    cmd.current_dir(&ws);
                    cmd.stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());
                    #[cfg(windows)]
                    {
                        use std::os::windows::process::CommandExt;
                        cmd.creation_flags(0x08000000);
                    }
                    let _ = cmd.spawn();
                }
            }

            // 3. Apple TSS Proxy (Port 816)
            if !is_port_open(816) {
                let script = ws.join("tss_proxy.py");
                if script.exists() {
                    let mut cmd = Command::new("python");
                    cmd.args(["-u", script.to_string_lossy().as_ref()]);
                    cmd.current_dir(&ws);
                    cmd.stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());
                    #[cfg(windows)]
                    {
                        use std::os::windows::process::CommandExt;
                        cmd.creation_flags(0x08000000);
                    }
                    let _ = cmd.spawn();
                }
            }

            std::thread::sleep(Duration::from_secs(4));
        }
    });
}
