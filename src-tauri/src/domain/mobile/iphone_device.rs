//! Physical iPhone mirroring over USB via `go-ios` (danielpaulus/go-ios).
//!
//! Pipeline:
//! ```text
//!   iPhone (USB / usbmuxd)
//!     └─ go-ios `ios screen`  → raw H.264 on stdout
//!          └─ reader task     → memory-bounded broadcast<Bytes>
//!               └─ WS :8765   → WebView (WebCodecs VideoDecoder → canvas)
//!   go-ios `ios syslog`  → lines → broadcast<String> → WS :8766 → Output panel
//! ```
//!
//! **Memory discipline (the OOM guard):** every fan-out uses a *bounded*
//! `tokio::sync::broadcast` channel. A slow or stalled WebView client makes the
//! broadcast lag and drop the oldest frames — it can never make the backend
//! buffer without limit. The reader reuses one fixed 64 KiB stack buffer and
//! allocates exactly one `Bytes` per chunk, freed as soon as every live
//! subscriber has forwarded it. Nothing here is proportional to how long the
//! stream runs.
//!
//! The WebSocket servers are started once (idempotent) and left resident but
//! idle when no stream is publishing, so start/stop cycles don't thrash the
//! listener sockets. go-ios itself is resolved from (in order): `GO_IOS_PATH`
//! env var, a binary bundled next to the IDE executable, then `ios` on `PATH`.

use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use futures::{SinkExt, StreamExt};
use serde::Serialize;
use serde_json::{json, Value};
use tauri::{AppHandle, Emitter};
use tokio::net::TcpListener;
use tokio::process::Child;
use tokio::sync::broadcast;
use tokio_tungstenite::tungstenite::Message;

use crate::infrastructure::process_ext::TokioCommandExtHidden;

/// Emit a diagnostic line to the mirror UI so go-ios failures aren't silent.
fn mlog(app: &AppHandle, stream: &str, line: impl Into<String>) {
    let _ = app.emit("iphone:mirror-log", json!({ "stream": stream, "line": line.into() }));
}

/// Spawn a task that reads a child pipe line-by-line and forwards each to the
/// mirror log. Generic so it takes either stdout or stderr.
fn pipe_lines<R>(app: AppHandle, tag: &'static str, reader: R)
where
    R: tokio::io::AsyncRead + Unpin + Send + 'static,
{
    tokio::spawn(async move {
        let mut lines = tokio::io::BufReader::new(reader).lines();
        while let Ok(Some(l)) = lines.next_line().await {
            record_tunnel_line(&l);
            mlog(&app, tag, l);
        }
    });
}

/// Fallback MJPEG port. go-ios v1.2's `screenshot --stream` hardcodes the server
/// to 0.0.0.0:3333 (the `--port` flag is ignored), so we read the real URL it
/// prints and fall back to this only if parsing fails. The WebView renders the
/// stream directly via `<img src=…>` (CSP allows `img-src http://127.0.0.1:*`).
const MIRROR_STREAM_PORT: u16 = 3333;
/// WebSocket port serving live syslog lines (text frames).
const LOG_WS_PORT: u16 = 8766;
/// Bounded log fan-out. Lines are tiny; a little slack is fine.
const LOG_CHANNEL_CAP: usize = 256;

// ─── Shared broadcast hubs (created lazily, live for process lifetime) ───────

fn log_hub() -> &'static broadcast::Sender<String> {
    static HUB: OnceLock<broadcast::Sender<String>> = OnceLock::new();
    HUB.get_or_init(|| broadcast::channel(LOG_CHANNEL_CAP).0)
}

// ─── Running-process handles (so we can stop cleanly) ────────────────────────

fn mirror_child() -> &'static Mutex<Option<Child>> {
    static C: OnceLock<Mutex<Option<Child>>> = OnceLock::new();
    C.get_or_init(|| Mutex::new(None))
}

fn syslog_child() -> &'static Mutex<Option<Child>> {
    static C: OnceLock<Mutex<Option<Child>>> = OnceLock::new();
    C.get_or_init(|| Mutex::new(None))
}

/// The `ios tunnel start` daemon (iOS 17+ RemoteXPC tunnel). Long-lived.
fn tunnel_child() -> &'static Mutex<Option<Child>> {
    static C: OnceLock<Mutex<Option<Child>>> = OnceLock::new();
    C.get_or_init(|| Mutex::new(None))
}

/// Last few go-ios lines, so a failed tunnel start can explain itself instead
/// of asking the user to read raw JSON in the diagnostics pane.
fn tunnel_log() -> &'static Mutex<Vec<String>> {
    static L: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
    L.get_or_init(|| Mutex::new(Vec::new()))
}

fn record_tunnel_line(line: &str) {
    if let Ok(mut v) = tunnel_log().lock() {
        v.push(line.to_string());
        if v.len() > 60 {
            let drop = v.len() - 60;
            v.drain(0..drop);
        }
    }
}

/// Turn go-ios's lockdown errors into the action that actually fixes them.
/// `InvalidHostID` is the common one: the device has no valid pair record for
/// this host, so every tunnel attempt fails until it is paired and trusted.
fn explain_tunnel_failure() -> Option<String> {
    let lines = tunnel_log().lock().ok()?.join("
");
    if lines.contains("InvalidHostID") || lines.contains("failed to start new lockdown session") {
        return Some(
            "The iPhone has no valid pairing record for this PC (go-ios: InvalidHostID).              Unlock the phone, keep it plugged in, click “Pair + Mount”, then tap              “Trust” on the device when prompted — and start the tunnel again."
                .into(),
        );
    }
    if lines.contains("device is locked") || lines.contains("PasswordProtected") {
        return Some("The iPhone is locked. Unlock it and start the tunnel again.".into());
    }
    if lines.contains("no device found") || lines.contains("device not found") {
        return Some("No iPhone detected over USB. Reconnect the cable and hit Refresh.".into());
    }
    if lines.contains("Only one usage of each socket address")
        || lines.contains("address already in use")
        || lines.contains("failed to start tunnel server")
    {
        return Some(
            "A previous go-ios tunnel is still holding its port. It has been stopped —              press “Start Tunnel” once more."
                .into(),
        );
    }
    if lines.contains("permission denied") || lines.contains("Access is denied") {
        return Some(
            "go-ios was denied permission to create the tunnel interface. Restart the IDE              as Administrator, then start the tunnel again."
                .into(),
        );
    }
    None
}

fn mirror_state() -> &'static Mutex<MirrorState> {
    static S: OnceLock<Mutex<MirrorState>> = OnceLock::new();
    S.get_or_init(|| Mutex::new(MirrorState::default()))
}

#[derive(Default, Clone)]
struct MirrorState {
    udid: Option<String>,
    streaming: bool,
    syslog: bool,
    tunnel: bool,
}

/// True once the WS listeners are bound. Prevents rebinding on restart.
fn servers_started() -> &'static std::sync::atomic::AtomicBool {
    static B: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    &B
}

// ─── go-ios binary resolution ────────────────────────────────────────────────

/// Directories where iOS tooling (go-ios, zsign, libimobiledevice) is shipped
/// with the IDE. These mirror the LSP bundle layout: Tauri copies
/// `src-tauri/binaries/*` next to the installed exe, so `binaries/ios-tools`
/// travels with the app and end users never install anything by hand. Dev
/// builds also look under `CARGO_MANIFEST_DIR`.
pub fn bundled_ios_tool_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            roots.push(dir.join("binaries").join("ios-tools"));
            roots.push(dir.join("resources").join("binaries").join("ios-tools"));
            roots.push(dir.join("binaries"));
            roots.push(dir.to_path_buf());
        }
    }
    // Runtime CARGO_MANIFEST_DIR (set when launched by cargo, e.g. `tauri dev`)…
    if let Ok(manifest) = std::env::var("CARGO_MANIFEST_DIR") {
        roots.push(PathBuf::from(manifest).join("binaries").join("ios-tools"));
    }
    // …and the compile-time one as a guaranteed dev fallback (baked in at build,
    // so `npm run dev:full` finds tools staged in src-tauri/binaries/ios-tools/
    // even if the runtime env var isn't inherited).
    roots.push(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("binaries").join("ios-tools"));
    roots
}

/// Find a shipped tool by base name across the bundle roots (adds `.exe` on
/// Windows). Used for go-ios and the libimobiledevice/zsign family.
pub fn find_bundled_tool(names: &[&str]) -> Option<PathBuf> {
    let exe_name = |n: &str| if cfg!(windows) { format!("{n}.exe") } else { n.to_string() };
    for root in bundled_ios_tool_roots() {
        for n in names {
            let cand = root.join(exe_name(n));
            if cand.is_file() {
                return Some(cand);
            }
        }
    }
    None
}

/// Locate the `go-ios` binary. Order: `GO_IOS_PATH` env → the iOS tools bundled
/// with the IDE → `ios` on `PATH`. Since the tools ship inside the app, this
/// normally resolves without any user setup.
pub fn resolve_go_ios() -> Option<PathBuf> {
    if let Ok(p) = std::env::var("GO_IOS_PATH") {
        let path = PathBuf::from(p);
        if path.exists() {
            return Some(path);
        }
    }

    if let Some(p) = find_bundled_tool(&["ios", "go-ios"]) {
        return Some(p);
    }

    which::which("ios").ok().or_else(|| which::which("go-ios").ok())
}

fn go_ios_or_err() -> Result<PathBuf, String> {
    resolve_go_ios().ok_or_else(|| {
        "go-ios ('ios') not found. Install from github.com/danielpaulus/go-ios, put it on PATH, \
         drop ios.exe next to the IDE, or set GO_IOS_PATH."
            .to_string()
    })
}

fn go_ios_cmd() -> Result<tokio::process::Command, String> {
    let bin = go_ios_or_err()?;
    let mut cmd = tokio::process::Command::new(bin);
    cmd.hidden_sidecar();
    // Every go-ios invocation must agree on how to reach the tunnel.
    //
    // `ios tunnel start --userspace` runs as its own process; a later
    // `ios screenshot` is a SEPARATE invocation that has to discover it through
    // the tunnel-info agent. Without this var go-ios does not look for a
    // userspace agent, logs "go-ios agent is not running" and "failed to get
    // tunnel info", and then reports the downstream symptom instead — that the
    // instruments service needs a tunnel, which is already running.
    cmd.env("ENABLE_GO_IOS_AGENT", "user");
    Ok(cmd)
}

// ─── Device discovery ────────────────────────────────────────────────────────

#[derive(Serialize, Clone)]
pub struct IPhoneDevice {
    pub udid: String,
    pub name: String,
    pub product: String,
    pub ios_version: String,
    /// "usb" | "network" | "unknown". Network devices support detection, syslog,
    /// install/launch and the hot-reload tunnel — but NOT the H.264 screen
    /// mirror, which is a USB-muxed AV stream.
    pub connection: String,
}

/// List physically connected iPhones/iPads via `ios list --details`.
#[tauri::command]
pub async fn iphone_list_devices() -> Result<Vec<IPhoneDevice>, String> {
    let mut cmd = go_ios_cmd()?;
    let out = cmd
        .args(["list", "--details"])
        .output()
        .await
        .map_err(|e| format!("go-ios list failed: {e}"))?;

    // go-ios prints one JSON object; tolerate either the `--details` array form
    // or the bare `{"deviceList":[...]}` udid list.
    let text = String::from_utf8_lossy(&out.stdout);
    let mut devices = Vec::new();

    if let Ok(v) = serde_json::from_str::<Value>(text.trim()) {
        if let Some(list) = v.get("deviceList").and_then(|d| d.as_array()) {
            for item in list {
                if let Some(udid) = item.as_str() {
                    devices.push(IPhoneDevice {
                        udid: udid.to_string(),
                        name: String::new(),
                        product: String::new(),
                        ios_version: String::new(),
                        connection: "usb".into(),
                    });
                } else if let Some(obj) = item.as_object() {
                    // go-ios surfaces the usbmux ConnectionType on some builds.
                    let conn = obj.get("ConnectionType").and_then(|c| c.as_str())
                        .map(|c| if c.eq_ignore_ascii_case("network") { "network" } else { "usb" })
                        .unwrap_or("usb");
                    devices.push(IPhoneDevice {
                        udid: obj.get("Udid").or_else(|| obj.get("udid"))
                            .and_then(|u| u.as_str()).unwrap_or("").to_string(),
                        name: obj.get("DeviceName").and_then(|n| n.as_str()).unwrap_or("").to_string(),
                        product: obj.get("ProductType").and_then(|n| n.as_str()).unwrap_or("").to_string(),
                        ios_version: obj.get("ProductVersion").and_then(|n| n.as_str()).unwrap_or("").to_string(),
                        connection: conn.into(),
                    });
                }
            }
        }
    }

    // Fallback: `ios list` without details prints bare udids, one per line.
    // Also used when `--details` fails (iOS 17+ needs the tunnel for details).
    if devices.is_empty() {
        // Try `ios list` (no --details) — works without the tunnel.
        let bare = go_ios_cmd()?.args(["list"]).output().await;
        if let Ok(o) = bare {
            let text = String::from_utf8_lossy(&o.stdout);
            if let Ok(v) = serde_json::from_str::<Value>(text.trim()) {
                if let Some(list) = v.get("deviceList").and_then(|d| d.as_array()) {
                    for item in list {
                        if let Some(udid) = item.as_str() {
                            devices.push(IPhoneDevice {
                                udid: udid.to_string(),
                                name: String::new(),
                                product: String::new(),
                                ios_version: String::new(),
                                connection: "usb".into(),
                            });
                        }
                    }
                }
            }
            // Bare UDID lines (older go-ios builds).
            if devices.is_empty() {
                for line in text.lines() {
                    let t = line.trim();
                    if t.len() >= 20 && !t.contains('{') && !t.contains(' ') {
                        devices.push(IPhoneDevice {
                            udid: t.to_string(),
                            name: String::new(),
                            product: String::new(),
                            ios_version: String::new(),
                            connection: "usb".into(),
                        });
                    }
                }
            }
        }
    }

    // WiFi devices: libimobiledevice's `idevice_id -n` lists network-reachable
    // (WiFi-paired) devices that go-ios/usbmux may not surface. Tag any not
    // already seen as "network". Requires the device to have been paired over
    // USB once with "Sync over Wi-Fi" enabled — an iOS constraint.
    if let Some(idid) = find_bundled_tool(&["idevice_id"]).or_else(|| which::which("idevice_id").ok()) {
        let mut cmd = tokio::process::Command::new(idid);
        cmd.arg("-n").hidden_sidecar();
        if let Ok(out) = cmd.output().await {
            for line in String::from_utf8_lossy(&out.stdout).lines() {
                let u = line.trim();
                if u.len() < 20 || u.contains(' ') {
                    continue;
                }
                match devices.iter_mut().find(|d| d.udid == u) {
                    // Already discovered over USB — keep that. A phone with WiFi
                    // sync enabled shows up in BOTH lists, and this used to
                    // overwrite it to "network" every time, so a cabled device
                    // could never display as USB no matter what the user picked.
                    // USB is also the connection we actually want: mirroring and
                    // WDA need it.
                    Some(d) if d.connection == "usb" => {}
                    Some(d) => d.connection = "network".into(), // WiFi-only
                    None => devices.push(IPhoneDevice {
                        udid: u.to_string(),
                        name: String::new(),
                        product: String::new(),
                        ios_version: String::new(),
                        connection: "network".into(),
                    }),
                }
            }
        }
    }

    // Deduplicate by UDID.
    //
    // Discovery runs several passes (`list --details`, the bare-udid fallback,
    // and `list -n` for network) and each could append the same phone, so the
    // picker showed one device twice — once as WiFi, once as USB. Worse, the UI
    // resolves the selection with `find(d => d.udid === selected)`, which always
    // returned the FIRST match, so choosing the wired row still selected the
    // WiFi entry and the connection type appeared frozen.
    //
    // Keep one row per UDID: prefer the USB entry (mirroring and WebDriverAgent
    // both require the cable), and keep whichever fields are populated, since
    // the details pass carries the name/version and the fallback does not.
    let mut merged: Vec<IPhoneDevice> = Vec::new();
    for d in devices {
        match merged.iter_mut().find(|m| m.udid == d.udid) {
            Some(m) => {
                if m.connection != "usb" && d.connection == "usb" {
                    m.connection = "usb".into();
                }
                if m.name.is_empty() { m.name = d.name; }
                if m.product.is_empty() { m.product = d.product; }
                if m.ios_version.is_empty() { m.ios_version = d.ios_version; }
            }
            None => merged.push(d),
        }
    }

    Ok(merged)
}

/// Best-effort device preparation: pair (accept "Trust" on the phone) and mount
/// the Developer Disk Image, which iOS 16+ needs for screen capture. Non-fatal
/// per step — returns what each subcommand reported.
#[tauri::command]
pub async fn iphone_prepare(app: AppHandle, udid: String) -> Result<Value, String> {
    async fn run(udid: &str, args: &[&str]) -> Value {
        let mut base: Vec<&str> = args.to_vec();
        base.extend_from_slice(&["--udid", udid]);
        match go_ios_cmd() {
            Ok(mut cmd) => match cmd.args(&base).output().await {
                Ok(o) => json!({
                    "ok": o.status.success(),
                    "stdout": String::from_utf8_lossy(&o.stdout).trim(),
                    "stderr": String::from_utf8_lossy(&o.stderr).trim(),
                }),
                Err(e) => json!({ "ok": false, "error": e.to_string() }),
            },
            Err(e) => json!({ "ok": false, "error": e }),
        }
    }

    // `ios pair` on its own does NOT fix a stale pairing record — that is the
    // `InvalidHostID` failure ("failed to start new lockdown session") seen when
    // starting the tunnel. The host and device disagree about the pair record,
    // and pairing again just re-reads the bad one. Clearing it first is what
    // actually recovers the device, so retry once through `unpair`.
    let mut pair = run(&udid, &["pair"]).await;
    let mut repaired = false;
    let pair_failed = !pair.get("ok").and_then(|v| v.as_bool()).unwrap_or(false)
        || pair
            .get("stderr")
            .and_then(|v| v.as_str())
            .map(|e| e.contains("InvalidHostID") || e.contains("lockdown"))
            .unwrap_or(false);
    if pair_failed {
        let unpair = run(&udid, &["unpair"]).await;
        let retry = run(&udid, &["pair"]).await;
        repaired = retry.get("ok").and_then(|v| v.as_bool()).unwrap_or(false);
        pair = json!({ "first_attempt": pair, "unpair": unpair, "retry": retry, "ok": repaired });
    }

    // Modern go-ios auto-downloads & mounts the DDI with `image auto`.
    let mount = run(&udid, &["image", "auto"]).await;
    // "Saving the PairRecord to usbmux failed" is the unelevated-Windows case:
    // the record is written through Apple Mobile Device Service, which refuses a
    // non-admin token. No amount of retrying or unpairing fixes it.
    let pair_ok = pair.get("ok").and_then(|v| v.as_bool()).unwrap_or(false);
    // go-ios can PANIC instead of failing cleanly: `interface conversion:
    // interface {} is nil, not []uint8` at ios/pair.go, when the device answers
    // the pair request with nothing. In practice that means the phone never put
    // up the trust dialog — usually because a half-written pair record is still
    // on the host. Dumping a Go stack trace into the panel tells the user
    // nothing, so name the recovery instead.
    let panicked = pair.to_string().contains("interface conversion")
        || pair.to_string().contains("goroutine 1 [running]");
    let elevated = is_elevated();
    // Don't gate this on matching go-ios's error text — the message has moved
    // between versions and a missed match sends the user chasing "unlock and
    // trust" forever, which cannot help. On Windows the pair record can only be
    // written through Apple Mobile Device Service with an elevated token, so a
    // failed pair from an unelevated process has exactly one explanation.
    let needs_admin = cfg!(target_os = "windows") && !pair_ok && !elevated;
    let hint = if panicked {
        "go-ios crashed while pairing (a known panic when the phone returns an empty          pair response). The trust dialog never appeared, which usually means a          half-written pair record is still on this PC. Click “Repair Pairing” — it          clears the record, restarts Apple Mobile Device Service and pairs fresh."
    } else if needs_admin && !elevated {
        "Pairing needs Administrator on Windows — the pair record is written through          Apple Mobile Device Service, which refuses an unelevated process. Either restart          the IDE as Administrator and press Pair + Mount again, or pair once using Apple's          own Devices/iTunes app (connect, unlock, tap Trust) — go-ios will reuse that          record. The tunnel itself does NOT need admin."
    } else if !pair_ok {
        "Unlock the iPhone, keep it plugged in, and tap “Trust This Computer” when          prompted, then run Pair + Mount again."
    } else {
        "Paired. Start the tunnel next."
    };

    if pair_ok {
        invalidate_tunnel_after_pair(&app).await;
    }

    Ok(json!({
        "udid": udid,
        "pair": pair,
        "mount": mount,
        "repaired": repaired,
        "needs_admin": needs_admin && !elevated,
        "elevated": elevated,
        "hint": hint,
    }))
}

// ─── WebSocket fan-out servers ───────────────────────────────────────────────

/// Bind the syslog WS server once (mirror frames go over HTTP/MJPEG, not WS).
pub async fn ensure_servers() -> Result<(), String> {
    use std::sync::atomic::Ordering;
    if servers_started().swap(true, Ordering::SeqCst) {
        return Ok(());
    }

    // The syslog WebSocket is an optional side channel — mirroring does not use
    // it. A dev-server restart leaves the previous bind in TIME_WAIT, and
    // failing here aborted the whole mirror with an error about a port the user
    // never asked for. Free the port if something stale holds it; if it is still
    // taken, log and carry on so Mirror is never blocked by Live Logs.
    match TcpListener::bind(("127.0.0.1", LOG_WS_PORT)).await {
        Ok(l) => {
            tokio::spawn(accept_loop_text(l));
        }
        Err(_) => {
            free_tcp_port(LOG_WS_PORT).await;
            match TcpListener::bind(("127.0.0.1", LOG_WS_PORT)).await {
                Ok(l) => {
                    tokio::spawn(accept_loop_text(l));
                }
                Err(e) => {
                    eprintln!(
                        "[iphone] syslog WS :{LOG_WS_PORT} unavailable ({e}) —                          Live Logs disabled, mirroring unaffected"
                    );
                }
            }
        }
    }
    Ok(())
}

/// Accept WS clients and forward broadcast log lines as text messages.
async fn accept_loop_text(listener: TcpListener) {
    loop {
        let Ok((stream, _)) = listener.accept().await else { continue };
        tokio::spawn(async move {
            let Ok(ws) = tokio_tungstenite::accept_async(stream).await else { return };
            let (mut sink, mut src) = ws.split();
            let mut rx = log_hub().subscribe();
            loop {
                tokio::select! {
                    msg = src.next() => match msg {
                        Some(Ok(Message::Close(_))) | None => break,
                        _ => {}
                    },
                    line = rx.recv() => match line {
                        Ok(text) => {
                            if sink.send(Message::Text(text)).await.is_err() {
                                break;
                            }
                        }
                        Err(broadcast::error::RecvError::Lagged(_)) => continue,
                        Err(broadcast::error::RecvError::Closed) => break,
                    },
                }
            }
        });
    }
}

// ─── iOS 17+ tunnel (RemoteXPC) ──────────────────────────────────────────────

/// Major iOS version of a device (e.g. 18), via `ios info`. None if unknown.
/// Public so the control layer (WDA) can decide whether a tunnel is required.
pub async fn device_ios_major_pub(udid: &str) -> Option<u32> {
    device_ios_major(udid).await
}

/// Public tunnel-up check for the control layer.
pub async fn tunnel_is_up_pub() -> bool {
    tunnel_is_up().await
}

/// Major iOS version of a device (e.g. 18), via `ios info`. None if unknown.
async fn device_ios_major(udid: &str) -> Option<u32> {
    let mut cmd = go_ios_cmd().ok()?;
    let out = cmd.args(["info", "--udid", udid]).output().await.ok()?;
    let v: Value = serde_json::from_slice(&out.stdout).ok()?;
    v.get("ProductVersion")
        .and_then(|s| s.as_str())
        .and_then(|s| s.split('.').next())
        .and_then(|s| s.parse().ok())
}

/// Is a go-ios tunnel currently up? `ios tunnel ls` lists running tunnels.
async fn tunnel_is_up() -> bool {
    let Ok(mut cmd) = go_ios_cmd() else { return false };
    let Ok(o) = cmd.args(["tunnel", "ls"]).output().await else { return false };
    let Ok(v) = serde_json::from_slice::<Value>(&o.stdout) else { return false };
    let Some(entries) = v.as_array() else { return false };

    // Listed is not the same as usable. A tunnel negotiated against a pair record
    // that has since been replaced stays in `tunnel ls`, but its userspace port
    // refuses connections — "ConnectUserSpaceTunnel: failed to dial ... actively
    // refused it". Reporting that as "Tunnel already running" made Start Tunnel a
    // no-op and left mirroring permanently broken, so probe the port.
    for e in entries {
        let Some(port) = e.get("userspaceTunPort").and_then(|p| p.as_u64()) else {
            // Kernel tunnel — nothing local to probe; trust the listing.
            return true;
        };
        let addr = format!("127.0.0.1:{port}");
        let reachable = tokio::time::timeout(
            std::time::Duration::from_millis(700),
            tokio::net::TcpStream::connect(&addr),
        )
        .await
        .map(|r| r.is_ok())
        .unwrap_or(false);
        if reachable {
            return true;
        }
    }
    false
}

/// Start the `ios tunnel start` daemon (iOS 17+). Needs administrator rights on
/// Windows — it creates a virtual network interface. Idempotent-ish: if a tunnel
/// is already up it just marks state. Streams go-ios output to the mirror log.
#[tauri::command]
pub async fn iphone_tunnel_start(app: AppHandle) -> Result<String, String> {
    if tunnel_is_up().await {
        mirror_state().lock().unwrap().tunnel = true;
        return Ok("Tunnel already running".into());
    }
    // Reap a dead child before trusting this guard. `tunnel_child` kept the
    // handle even after go-ios exited, so a single failed start (e.g. the
    // InvalidHostID pairing error) latched this branch forever and every later
    // attempt returned "already in progress" without ever retrying — the tunnel
    // could not be started again without restarting the IDE.
    {
        let mut slot = tunnel_child().lock().unwrap();
        let still_running = match slot.as_mut() {
            Some(c) => matches!(c.try_wait(), Ok(None)),
            None => false,
        };
        if still_running {
            return Ok("Tunnel start already in progress".into());
        }
        *slot = None;
    }
    // Fresh attempt — drop diagnostics from the previous one.
    if let Ok(mut v) = tunnel_log().lock() {
        v.clear();
    }

    // Clear any orphan from an earlier attempt first. A failed tunnel leaves the
    // go-ios process alive holding its tunnel-info port, and the next start dies
    // with "Only one usage of each socket address". Reaping our handle is not
    // enough — the OS process outlives it, so ask go-ios to stop its agent and
    // then kill any stragglers by name.
    if let Ok(mut c) = go_ios_cmd() {
        let _ = c.args(["tunnel", "stopagent"]).output().await;
    }
    kill_orphan_tunnels().await;

    // `--userspace` negotiates the tunnel without elevation (verified on iOS
    // 18.1) — no admin/sudo needed. It also serves the tunnel-info API on 28100
    // so `ios screenshot`/`syslog` find the tunnel automatically.
    let mut cmd = go_ios_cmd()?;
    cmd.args(["tunnel", "start", "--userspace"])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());
    let mut child = cmd.spawn().map_err(|e| format!("spawn 'ios tunnel start': {e}"))?;

    // Pipe both streams to the UI so admin/permission failures are visible.
    if let Some(o) = child.stdout.take() { pipe_lines(app.clone(), "out", o); }
    if let Some(e) = child.stderr.take() { pipe_lines(app.clone(), "err", e); }
    *tunnel_child().lock().unwrap() = Some(child);

    // Give it a moment to establish, then confirm.
    tokio::time::sleep(std::time::Duration::from_millis(2500)).await;
    let up = tunnel_is_up().await;
    mirror_state().lock().unwrap().tunnel = up;
    if up {
        Ok("Tunnel started (userspace)".into())
    } else {
        Err("Tunnel did not come up within a few seconds. Check the diagnostics log \
             below for the go-ios error (e.g. device locked, not trusted, or unplugged), \
             then retry.".into())
    }
}

/// Whether this process has Administrator rights (Windows) / is root (unix).
///
/// Pairing needs it: on Windows the pair record is written through Apple Mobile
/// Device Service, and an unelevated process is refused with
/// "Saving the PairRecord to usbmux failed". Note this is NOT required for the
/// tunnel — that runs `--userspace` and works unelevated.
pub fn is_elevated() -> bool {
    #[cfg(target_os = "windows")]
    {
        // `net session` succeeds only for an elevated token.
        crate::process_ext::hidden_command("net")
            .args(["session"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::env::var("USER").map(|u| u == "root").unwrap_or(false)
    }
}

/// Full pairing recovery: revoke trust, clear the host-side record, restart the
/// USB service, wait for re-enumeration, then pair fresh.
///
/// Why this exists: a stale pair record on the host holds a HostID the phone has
/// stopped honouring. Every lockdown call then fails with `InvalidHostID`, and
/// re-pairing over it fails because Apple Mobile Device Service will not replace
/// the file in place — it reports that as "Saving the PairRecord to usbmux
/// failed", which reads like a permissions problem and is not one. Neither
/// pairing through Apple's own app nor running elevated fixes it; only clearing
/// the record does. `unpair` first so the device drops its side too and prompts
/// "Trust This Computer" again on the next pair.
///
/// Needs elevation on Windows (service control + a write under ProgramData).
#[tauri::command]
pub async fn iphone_repair_pairing(app: AppHandle, udid: String) -> Result<Value, String> {
    if cfg!(target_os = "windows") && !is_elevated() {
        return Err(
            "Repairing the pairing needs Administrator: it stops Apple Mobile Device Service              and clears the stale record under ProgramData. Relaunch the IDE as Administrator              and press this again — normal use afterwards does not need admin."
                .into(),
        );
    }
    let mlogp = |m: &str| mlog(&app, "meta", m.to_string());

    // 1. Revoke on the device so it re-prompts for trust.
    mlogp("[repair] revoking trust on the device…");
    if let Ok(mut c) = go_ios_cmd() {
        let _ = c.args(["unpair", "--udid", &udid]).output().await;
    }

    // 2/3. Clear the host record with the service stopped — it holds the file open.
    #[cfg(target_os = "windows")]
    {
        mlogp("[repair] stopping Apple Mobile Device Service…");
        let _ = crate::process_ext::hidden_command("net")
            .args(["stop", "Apple Mobile Device Service", "/y"])
            .output();

        let dir = std::path::PathBuf::from(r"C:\ProgramData\Apple\Lockdown");
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for e in entries.flatten() {
                let name = e.file_name().to_string_lossy().to_string();
                // `<udid>.plist` plus any `.tmp` left by a failed replace.
                if name.starts_with(&udid) {
                    let _ = std::fs::remove_file(e.path());
                    mlogp(&format!("[repair] removed {name}"));
                }
            }
        }

        mlogp("[repair] starting Apple Mobile Device Service…");
        let _ = crate::process_ext::hidden_command("net")
            .args(["start", "Apple Mobile Device Service"])
            .output();
    }

    // 4. Restarting the service tears down USB enumeration; wait for it back.
    mlogp("[repair] waiting for the device to re-enumerate…");
    let mut seen = false;
    for _ in 0..20 {
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        if list_udids().await.iter().any(|u| u == &udid) {
            seen = true;
            break;
        }
    }
    if !seen {
        return Err(
            "The device did not come back after restarting Apple Mobile Device Service.              Unplug the cable, plug it back in, and press Repair Pairing again."
                .into(),
        );
    }

    // 5. Fresh pair — the phone should now prompt for trust.
    mlogp("[repair] pairing (tap “Trust This Computer” on the phone)…");
    let mut cmd = go_ios_cmd()?;
    let out = cmd
        .args(["pair", "--udid", &udid])
        .output()
        .await
        .map_err(|e| format!("run 'ios pair': {e}"))?;
    let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
    let ok = out.status.success() && !stderr.contains("Pairing failed");
    if !ok {
        return Err(format!(
            "Pairing still failed after the reset. Unlock the phone, tap “Trust This              Computer”, and try again. go-ios said: {stderr}"
        ));
    }
    invalidate_tunnel_after_pair(&app).await;
    mlogp("[repair] paired — start the tunnel next.");
    Ok(json!({ "ok": true, "udid": udid, "stderr": stderr }))
}

/// UDIDs currently visible to usbmux.
async fn list_udids() -> Vec<String> {
    let Ok(mut cmd) = go_ios_cmd() else { return Vec::new() };
    let Ok(out) = cmd.args(["list"]).output().await else { return Vec::new() };
    serde_json::from_slice::<Value>(&out.stdout)
        .ok()
        .and_then(|v| {
            v.get("deviceList")?
                .as_array()
                .map(|a| a.iter().filter_map(|x| x.as_str().map(str::to_string)).collect())
        })
        .unwrap_or_default()
}

/// Kill whatever currently listens on `port`, leaving every other process alone.
///
/// Used before starting the MJPEG server: a screenshot run that ended badly
/// keeps 0.0.0.0:3333 bound. Killing by image name would take the tunnel with
/// it, so resolve the owning PID instead.
async fn free_tcp_port(port: u16) {
    #[cfg(target_os = "windows")]
    {
        let Ok(out) = crate::process_ext::hidden_command("netstat").args(["-ano"]).output() else {
            return;
        };
        let text = String::from_utf8_lossy(&out.stdout);
        let needle = format!(":{port}");
        let mut pids: Vec<String> = Vec::new();
        for line in text.lines() {
            if !line.contains(&needle) || !line.contains("LISTENING") {
                continue;
            }
            if let Some(pid) = line.split_whitespace().last() {
                if pid.chars().all(|c| c.is_ascii_digit()) && pid != "0" {
                    pids.push(pid.to_string());
                }
            }
        }
        for pid in pids {
            let _ = crate::process_ext::hidden_command("taskkill")
                .args(["/F", "/T", "/PID", &pid])
                .output();
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = tokio::process::Command::new("sh")
            .arg("-c")
            .arg(format!("lsof -ti tcp:{port} | xargs -r kill -9"))
            .output()
            .await;
    }
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
}

/// Tear down the tunnel after a successful (re)pair.
///
/// A userspace tunnel is negotiated against the pair record that existed when it
/// started. Re-pairing invalidates it: `tunnel ls` still lists it, so
/// `tunnel_is_up` reports "Tunnel already running", but its RSD endpoint refuses
/// connections and the next mirror dies with
/// "ConnectUserSpaceTunnel: failed to dial ... actively refused it".
/// Dropping it here forces Start Tunnel to build a fresh one.
async fn invalidate_tunnel_after_pair(app: &AppHandle) {
    if !tunnel_is_up().await {
        return;
    }
    mlog(app, "meta", "pairing changed — restarting the tunnel so it uses the new record".to_string());
    if let Some(mut child) = tunnel_child().lock().unwrap().take() {
        let _ = child.start_kill();
    }
    if let Ok(mut c) = go_ios_cmd() {
        let _ = c.args(["tunnel", "stopagent"]).output().await;
    }
    kill_orphan_tunnels().await;
    mirror_state().lock().unwrap().tunnel = false;
}

/// Kill go-ios processes left over from a failed tunnel start.
///
/// `stopagent` only reaches an agent go-ios itself registered; a process that
/// died mid-handshake keeps its socket until the OS reaps it, which is what
/// produces the port-bind error on the next attempt. `/T` takes the tree.
async fn kill_orphan_tunnels() {
    #[cfg(target_os = "windows")]
    {
        let _ = crate::process_ext::hidden_command("taskkill")
            .args(["/F", "/T", "/IM", "ios.exe"])
            .output();
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = tokio::process::Command::new("pkill")
            .args(["-f", "ios tunnel start"])
            .output()
            .await;
    }
    // Give the OS a moment to release the listening socket.
    tokio::time::sleep(std::time::Duration::from_millis(400)).await;
}

/// Stop the tunnel daemon.
#[tauri::command]
pub async fn iphone_tunnel_stop() -> Result<String, String> {
    if let Some(mut child) = tunnel_child().lock().unwrap().take() {
        let _ = child.start_kill();
    }
    // Best-effort: also ask go-ios to stop any agent.
    if let Ok(mut cmd) = go_ios_cmd() {
        let _ = cmd.args(["tunnel", "stopagent"]).output().await;
    }
    mirror_state().lock().unwrap().tunnel = false;
    Ok("Tunnel stopped".into())
}

use tokio::io::AsyncBufReadExt as _;

// ─── Mirror stream lifecycle ─────────────────────────────────────────────────

#[derive(Serialize)]
pub struct MirrorStartResult {
    /// HTTP MJPEG stream URL for the WebView `<img>` (CSP-allowed 127.0.0.1).
    pub stream_url: String,
    pub log_ws: String,
    pub udid: String,
    pub go_ios: String,
    pub tunnel_required: bool,
}

/// Start the mirror. On iOS 17+ this needs an active tunnel and a mounted
/// Developer Image; we ensure both, then run `ios screenshot --stream --port`,
/// which serves MJPEG the WebView renders directly. go-ios output is streamed to
/// the UI so any failure is visible rather than a silent blank screen.
#[tauri::command]
pub async fn iphone_start_mirror(app: AppHandle, udid: String) -> Result<MirrorStartResult, String> {
    {
        let st = mirror_state().lock().unwrap();
        if st.streaming {
            return Err("Mirror already running. Stop it first.".into());
        }
    }
    ensure_servers().await?;
    let go_ios = go_ios_or_err()?;

    // iOS 17+ needs the RemoteXPC tunnel for the screenshot/instruments service.
    let major = device_ios_major(&udid).await.unwrap_or(18);
    let tunnel_required = major >= 17;
    if tunnel_required && !tunnel_is_up().await {
        return Err(format!(
            "iOS {major} needs the go-ios tunnel before mirroring. Click “Start Tunnel” \
             (userspace — no admin needed), then press Mirror again."
        ));
    }

    // Ensure the Developer Image is mounted (needed for screen capture on 16+).
    if major >= 16 {
        if let Ok(mut cmd) = go_ios_cmd() {
            mlog(&app, "meta", "mounting developer image…");
            match cmd.args(["image", "auto", "--udid", &udid]).output().await {
                Ok(o) if !o.status.success() => {
                    mlog(&app, "err", String::from_utf8_lossy(&o.stderr).trim().to_string());
                }
                Err(e) => mlog(&app, "err", format!("image auto: {e}")),
                _ => {}
            }
        }
    }

    // Clear any previous mirror first. The screenshot server binds 0.0.0.0:3333,
    // and a run that ended badly (or a screenshot-loop timeout) leaves the
    // process alive holding it — the next Mirror then dies with "Only one usage
    // of each socket address". Same orphan class as the tunnel.
    {
        if let Some(mut old) = mirror_child().lock().unwrap().take() {
            let _ = old.start_kill();
        }
    }
    // Free the MJPEG port by PID, never by image name: `taskkill /IM ios.exe`
    // also kills the running tunnel, and the screenshot service cannot reach the
    // device without it — mirroring would kill the very thing it depends on.
    free_tcp_port(MIRROR_STREAM_PORT).await;

    // MJPEG server: `ios screenshot --stream`. go-ios prints the server URL it
    // actually bound (port is not ours to choose); we parse it from stdout and
    // use that, so we never point the <img> at the wrong port.
    let mut cmd = go_ios_cmd()?;
    cmd.args(["screenshot", "--stream", "--udid", &udid])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());
    let mut child = cmd.spawn().map_err(|e| format!("spawn 'ios screenshot --stream': {e}"))?;

    // Discover the served URL from the first stdout lines, then keep draining to
    // the log. 0.0.0.0 → 127.0.0.1 so the WebView (and CSP) can reach it.
    let (tx_url, rx_url) = tokio::sync::oneshot::channel::<String>();
    // Watch BOTH pipes. go-ios logs to stderr, so the line carrying "url" —
    // 'starting server, open your browser here' — never reached a stdout-only
    // reader. The 6s wait below then timed out and killed a screenshot server
    // that had started correctly, reporting "Mirror failed to start".
    let tx_shared = std::sync::Arc::new(Mutex::new(Some(tx_url)));
    let mut watch = |reader: Option<tokio::process::ChildStdout>,
                     err_reader: Option<tokio::process::ChildStderr>,
                     tag: &'static str| {
        let app2 = app.clone();
        let tx = tx_shared.clone();
        tokio::spawn(async move {
            // One helper, either pipe.
            macro_rules! drain {
                ($r:expr) => {{
                    let mut lines = tokio::io::BufReader::new($r).lines();
                    while let Ok(Some(l)) = lines.next_line().await {
                        if let Ok(v) = serde_json::from_str::<Value>(&l) {
                            if let Some(u) = v.get("url").and_then(|u| u.as_str()) {
                                let u = u.replace("0.0.0.0", "127.0.0.1");
                                if let Ok(mut g) = tx.lock() {
                                    if let Some(t) = g.take() {
                                        let _ = t.send(u);
                                    }
                                }
                            }
                        }
                        mlog(&app2, tag, l);
                    }
                }};
            }
            if let Some(r) = reader { drain!(r); }
            if let Some(r) = err_reader { drain!(r); }
        });
    };
    watch(child.stdout.take(), None, "out");
    watch(None, child.stderr.take(), "err");

    let stream_url = match tokio::time::timeout(std::time::Duration::from_secs(6), rx_url).await {
        Ok(Ok(u)) => u,
        _ => {
            // go-ios didn't print a URL — the screenshot server failed to start.
            // Kill the child and return a clear error instead of a dead URL.
            if let Some(mut child) = mirror_child().lock().unwrap().take() {
                let _ = child.start_kill();
            }
            return Err("Mirror failed to start. Check the diagnostics log — likely \
                         the tunnel is not running or the device is locked. Click \
                         \u{201c}Start Tunnel\u{201d}, unlock the device, then retry Mirror."
                .into());
        }
    };

    {
        let mut st = mirror_state().lock().unwrap();
        st.udid = Some(udid.clone());
        st.streaming = true;
    }
    *mirror_child().lock().unwrap() = Some(child);

    Ok(MirrorStartResult {
        stream_url,
        log_ws: format!("ws://127.0.0.1:{LOG_WS_PORT}"),
        udid,
        go_ios: go_ios.to_string_lossy().to_string(),
        tunnel_required,
    })
}

/// Stop the mirror stream (kills the go-ios screenshot server).
#[tauri::command]
pub async fn iphone_stop_mirror() -> Result<String, String> {
    if let Some(mut child) = mirror_child().lock().unwrap().take() {
        let _ = child.start_kill();
    }
    let mut st = mirror_state().lock().unwrap();
    st.streaming = false;
    st.udid = None;
    Ok("Mirror stopped".into())
}

/// Start streaming `ios syslog` lines into the log broadcast (WS :8766).
#[tauri::command]
pub async fn iphone_start_syslog(udid: String) -> Result<String, String> {
    {
        let st = mirror_state().lock().unwrap();
        if st.syslog {
            return Err("Syslog already streaming".into());
        }
    }
    ensure_servers().await?;

    let mut cmd = go_ios_cmd()?;
    cmd.args(["syslog", "--udid", udid.as_str()])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null());
    let mut child = cmd.spawn().map_err(|e| format!("spawn 'ios syslog': {e}"))?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| "no stdout on go-ios syslog child".to_string())?;

    mirror_state().lock().unwrap().syslog = true;
    *syslog_child().lock().unwrap() = Some(child);

    tokio::spawn(async move {
        let tx = log_hub();
        let mut reader = tokio::io::BufReader::new(stdout);
        let mut line = String::new();
        loop {
            line.clear();
            match tokio::io::AsyncBufReadExt::read_line(&mut reader, &mut line).await {
                Ok(0) => break,
                Ok(_) => {
                    let _ = tx.send(line.trim_end().to_string());
                }
                Err(_) => break,
            }
        }
        mirror_state().lock().unwrap().syslog = false;
    });

    Ok(format!("ws://127.0.0.1:{LOG_WS_PORT}"))
}

/// Stop the syslog stream.
#[tauri::command]
pub async fn iphone_stop_syslog() -> Result<String, String> {
    if let Some(mut child) = syslog_child().lock().unwrap().take() {
        let _ = child.start_kill();
    }
    mirror_state().lock().unwrap().syslog = false;
    Ok("Syslog stopped".into())
}

#[derive(Serialize)]
pub struct IPhoneMirrorStatus {
    pub go_ios_found: bool,
    pub go_ios_path: Option<String>,
    pub streaming: bool,
    pub syslog: bool,
    pub tunnel: bool,
    pub udid: Option<String>,
    pub stream_url: String,
    pub log_ws: String,
    pub log_clients: usize,
}

/// Current mirror status for the UI (and a quick go-ios availability probe).
#[tauri::command]
pub fn iphone_mirror_status() -> IPhoneMirrorStatus {
    let path = resolve_go_ios();

    // Reap a dead screenshot process before reporting. `streaming` was only ever
    // cleared by an explicit Stop, so a stream that died on its own (screenshot
    // loop timeout, tunnel drop, killed child) left the flag true forever: the
    // Mirror button showed as active without being clicked, and start_mirror
    // refused with "Mirror already running. Stop it first." Presence of a handle
    // is not the same as a live process.
    {
        let mut slot = mirror_child().lock().unwrap();
        let dead = match slot.as_mut() {
            Some(c) => !matches!(c.try_wait(), Ok(None)),
            None => true,
        };
        if dead {
            *slot = None;
            let mut st = mirror_state().lock().unwrap();
            if st.streaming {
                st.streaming = false;
            }
        }
    }

    let st = mirror_state().lock().unwrap();
    IPhoneMirrorStatus {
        go_ios_found: path.is_some(),
        go_ios_path: path.map(|p| p.to_string_lossy().to_string()),
        streaming: st.streaming,
        syslog: st.syslog,
        tunnel: st.tunnel,
        udid: st.udid.clone(),
        stream_url: format!("http://127.0.0.1:{MIRROR_STREAM_PORT}"),
        log_ws: format!("ws://127.0.0.1:{LOG_WS_PORT}"),
        log_clients: log_hub().receiver_count(),
    }
}
